use crate::models::{Answer, Judgment, Node, Question};
use reqwest::{Client, header};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        Arc, Mutex, RwLock,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JevError {
    #[error("TypeSafe transport failed")]
    Transport(#[from] reqwest::Error),
    #[error("TypeSafe returned HTTP {0}")]
    Status(u16),
    #[error("TypeSafe returned invalid response: {0}")]
    Invalid(String),
    #[error("Jev API key is required")]
    Unconfigured,
}

/// Resolved Jev connection: key/base/model may come from DB settings.
#[derive(Clone, Debug, Default)]
pub struct JevConfig {
    pub key: Option<String>,
    pub base_url: String,
    pub model: String,
    /// OpenAI-compatible base used to route search/ingest when a token and model are set.
    /// Never written into [`Self::normalized`]; System One stays on TypeSafe.
    pub llm_base_url: String,
    pub llm_token: Option<String>,
    pub llm_model: String,
}

impl JevConfig {
    /// Layer DB settings on top of defaults. The DB is the only source;
    /// env only seeds empty slots at boot (see `seed_settings_from_env`).
    pub fn with_db(&mut self, settings: &crate::models::AppSettings) {
        if !settings.jev_api_key.trim().is_empty() {
            self.key = Some(settings.jev_api_key.clone());
        }
        self.llm_base_url = settings
            .llm_base_url
            .trim()
            .trim_end_matches('/')
            .to_string();
        self.llm_token = Some(settings.llm_token.clone()).filter(|token| !token.trim().is_empty());
        self.llm_model = settings.llm_model.trim().to_string();
    }

    pub fn normalized(&self) -> (Option<String>, String, String) {
        (
            self.key.clone().filter(|k| !k.trim().is_empty()),
            if self.base_url.trim().is_empty() {
                "https://api.typesafe.ai".into()
            } else {
                self.base_url.trim().trim_end_matches('/').to_string()
            },
            if self.model.trim().is_empty() {
                "jev-latest".into()
            } else {
                self.model.trim().to_string()
            },
        )
    }
}

#[derive(Clone)]
pub struct JevClient {
    client: Client,
    config: Arc<RwLock<JevConfig>>,
    /// Test-only preferred Choice ids. Empty in production. Compiled in for the
    /// `eval` example's mock router; never reachable over HTTP.
    script: Arc<Mutex<Vec<String>>>,
    /// Test-only leaf for [`Self::review`]. `None` means "use the model".
    llm_script: Arc<Mutex<Option<String>>>,
    /// Moves on every settings change or script, so cached results made under
    /// other settings are never served.
    generation: Arc<AtomicU64>,
}

impl JevClient {
    pub fn with_config(config: JevConfig) -> Result<Self, reqwest::Error> {
        let client = Client::builder()
            .timeout(Duration::from_secs(20))
            .connect_timeout(Duration::from_secs(5))
            .pool_idle_timeout(Duration::from_secs(60))
            .redirect(reqwest::redirect::Policy::none())
            .user_agent(concat!("jev-tree/", env!("CARGO_PKG_VERSION")))
            .build()?;
        Ok(Self {
            client,
            config: Arc::new(RwLock::new(config)),
            script: Arc::new(Mutex::new(Vec::new())),
            llm_script: Arc::new(Mutex::new(None)),
            generation: Arc::new(AtomicU64::new(0)),
        })
    }

    pub fn script_choices(&self, ids: Vec<String>) {
        if let Ok(mut guard) = self.script.lock() {
            *guard = ids;
        }
        self.generation.fetch_add(1, Ordering::AcqRel);
    }

    /// Changes whenever the settings (or a test script) change.
    pub fn generation(&self) -> u64 {
        self.generation.load(Ordering::Acquire)
    }

    /// Pin the router leaf in tests. `__none__` abstains. `None` clears the pin.
    pub fn script_llm(&self, leaf: Option<String>) {
        if let Ok(mut guard) = self.llm_script.lock() {
            *guard = leaf;
        }
        self.generation.fetch_add(1, Ordering::AcqRel);
    }

    pub fn choice_scripted(&self) -> bool {
        self.script
            .lock()
            .map(|guard| !guard.is_empty())
            .unwrap_or(false)
    }

    /// True when search/ingest should route with the configured LLM (or a test pin).
    pub fn llm_routing(&self) -> bool {
        if self
            .llm_script
            .lock()
            .map(|guard| guard.is_some())
            .unwrap_or(false)
        {
            return true;
        }
        let Ok(config) = self.config.read() else {
            return false;
        };
        config.llm_token.is_some()
            && !config.llm_model.trim().is_empty()
            && !config.llm_base_url.trim().is_empty()
    }

    /// Hot-swap key/base/model after a settings change (no restart).
    pub fn reconfigure(&self, config: JevConfig) {
        if let Ok(mut guard) = self.config.write() {
            *guard = config;
        }
        self.generation.fetch_add(1, Ordering::AcqRel);
    }

    fn snapshot(&self) -> (Option<String>, String, String) {
        self.config.read().map(|c| c.normalized()).unwrap_or((
            None,
            "https://api.typesafe.ai".into(),
            "jev-latest".into(),
        ))
    }

    pub fn key_set(&self) -> bool {
        self.snapshot().0.is_some()
    }

    /// Test pin for [`Self::review`]. Production leaves this empty.
    pub fn llm_pinned(&self) -> bool {
        self.llm_script
            .lock()
            .map(|guard| guard.is_some())
            .unwrap_or(false)
    }

    pub fn label(&self) -> &'static str {
        if self.key_set() {
            "jev"
        } else {
            "unconfigured"
        }
    }

    /// The LLM settles a route Jev was unsure of: the one-shot prompt over the
    /// menu Jev chose from, in tree order. The model needs that whole menu, not
    /// Jev's favourites: shown only Jev's top candidates, best first, it agrees
    /// with Jev's mistakes (93% on the golden queries against 97% with the whole
    /// menu). `abstain` offers `__none__` (a forest or `root` walk); a
    /// `start_node` walk must land in the menu.
    pub async fn review(
        &self,
        query: &str,
        background: &str,
        nodes: &[Node],
        menu: &[String],
        abstain: bool,
    ) -> Result<LlmRoute, JevError> {
        if let Ok(guard) = self.llm_script.lock()
            && let Some(leaf) = guard.as_ref()
        {
            return Ok(LlmRoute {
                leaf: (leaf != "__none__" && !leaf.trim().is_empty()).then(|| leaf.clone()),
                steps: vec![],
                meter: LlmMeter::default(),
            });
        }
        let creds = self.llm_creds()?;
        let (lines, options, allowed) = llm_menu(nodes, menu, abstain);
        let prompt = one_shot_prompt(abstain, &allowed, query, background, &lines);
        let mut meter = LlmMeter::default();
        let picked = self
            .complete_id(
                &creds.base,
                &creds.token,
                &creds.model,
                &prompt,
                &options,
                &mut meter,
            )
            .await?;
        let choice_name = options
            .iter()
            .find(|(id, _)| id == &picked)
            .map(|(_, name)| name.clone())
            .unwrap_or_default();
        let leaf = (picked != "__none__").then_some(picked);
        Ok(LlmRoute {
            steps: vec![LlmRouteStep {
                depth: 0,
                parent_name: "llm review".into(),
                candidates: options,
                choice_id: leaf.clone(),
                choice_name,
            }],
            leaf,
            meter,
        })
    }

    fn llm_creds(&self) -> Result<LlmCreds, JevError> {
        let config = self
            .config
            .read()
            .map_err(|_| JevError::Invalid("llm config unavailable".into()))?;
        let creds = LlmCreds {
            base: config.llm_base_url.clone(),
            token: config.llm_token.clone().unwrap_or_default(),
            model: config.llm_model.clone(),
        };
        if creds.token.trim().is_empty()
            || creds.model.trim().is_empty()
            || creds.base.trim().is_empty()
        {
            return Err(JevError::Invalid("LLM route is not configured".into()));
        }
        Ok(creds)
    }

    async fn complete_id(
        &self,
        base: &str,
        token: &str,
        model: &str,
        user: &str,
        choices: &[(String, String)],
        meter: &mut LlmMeter,
    ) -> Result<String, JevError> {
        let content = self.chat(base, token, model, user, meter).await?;
        if let Some(id) = parse_final_choice(&content, choices) {
            return Ok(id);
        }
        let retry = format!(
            "{user}\n\n이전 답변에서 id를 찾을 수 없었습니다. \
             설명·근거·마크다운 없이, 허용된 id 하나만 한 줄로 다시 쓰세요."
        );
        let content = self.chat(base, token, model, &retry, meter).await?;
        parse_final_choice(&content, choices)
            .ok_or_else(|| JevError::Invalid("LLM route did not return an allowed id".into()))
    }

    async fn chat(
        &self,
        base: &str,
        token: &str,
        model: &str,
        user: &str,
        meter: &mut LlmMeter,
    ) -> Result<String, JevError> {
        let url = chat_completions_url(base);
        let messages = json!([
            {"role": "system", "content": "분류기입니다. 마지막 줄에는 허용된 id 하나만 씁니다."},
            {"role": "user", "content": user},
        ]);
        let mut payload = json!({
            "model": model,
            "reasoning_effort": "low",
            // A reasoning model bills its thinking against max_tokens, and the
            // menu can name every node in scope. 900 was too small: the reply
            // budget went to deliberation and `content` came back empty, which
            // is what the "LLM returned no content" and "did not return an
            // allowed id" warnings were. The answer is one id, so this is pure
            // thinking headroom — cheap insurance against a truncated reply.
            "max_tokens": ROUTE_MAX_TOKENS,
            "messages": messages,
        });
        let first = match self.post_chat(&url, token, &payload, meter).await {
            Err(JevError::Status(400)) => {
                // The endpoint may reject the thinking knob or a ceiling above
                // its own. Shrink the budget so an oversized ask still routes.
                plain_chat_payload(&mut payload);
                shrink_max_tokens(&mut payload);
                self.post_chat(&url, token, &payload, meter).await?
            }
            other => other?,
        };
        if !first.is_empty() {
            return Ok(first);
        }
        // Some models spend the reply on reasoning and leave content empty.
        // Ask once more in plain text, then give up so Jev's pick stands.
        plain_chat_payload(&mut payload);
        let second = self.post_chat(&url, token, &payload, meter).await?;
        if second.is_empty() {
            return Err(JevError::Invalid("LLM returned no content".into()));
        }
        Ok(second)
    }

    async fn post_chat(
        &self,
        url: &str,
        token: &str,
        payload: &Value,
        meter: &mut LlmMeter,
    ) -> Result<String, JevError> {
        // 90s, not the 20s the client defaults to: the route call now names every
        // node in scope, and a reasoning model deliberates over that menu for a
        // while. The route step's own budget (`llm_route_budget`) is the real
        // ceiling and cancels this future first; this timeout only stops a hung
        // socket from pinning the connection.
        let response = self
            .client
            .post(url)
            .timeout(Duration::from_secs(90))
            .bearer_auth(token)
            .header(header::CONTENT_TYPE, "application/json")
            .json(payload)
            .send()
            .await?;
        let status = response.status();
        if !status.is_success() {
            return Err(JevError::Status(status.as_u16()));
        }
        let body: Value = response
            .json()
            .await
            .map_err(|error| JevError::Invalid(format!("llm parse failed: {error}")))?;
        meter.calls += 1;
        let tokens = |key: &str| body.pointer(key).and_then(Value::as_u64).unwrap_or(0);
        meter.usage.input_tokens += tokens("/usage/prompt_tokens");
        meter.usage.output_tokens += tokens("/usage/completion_tokens");
        Ok(assistant_text(&body))
    }

    /// Probe `{base}/v1/models` then `{base}/models`, unless `base` already ends
    /// in `/v1` (then `{base}/models` only). Does not fall back to the Jev key.
    /// Returns `(models, used_base)` on success.
    pub async fn list_models(
        &self,
        base_url: Option<&str>,
        token: Option<&str>,
    ) -> Result<(Vec<crate::models::ModelInfo>, String), JevError> {
        let base = base_url
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().trim_end_matches('/').to_string())
            .ok_or_else(|| JevError::Invalid("LLM base URL is required".into()))?;
        let token = token
            .filter(|s| !s.trim().is_empty())
            .map(str::to_string)
            .ok_or_else(|| JevError::Invalid("LLM token is required".into()))?;
        let mut last_status = None;
        let mut body: Option<Value> = None;
        for url in model_list_urls(&base) {
            let response = self.client.get(&url).bearer_auth(&token).send().await?;
            let status = response.status();
            if status.is_success() {
                body = Some(
                    response
                        .json()
                        .await
                        .map_err(|e| JevError::Invalid(format!("model list parse failed: {e}")))?,
                );
                break;
            }
            last_status = Some(status.as_u16());
            if status.as_u16() != 404 && status.as_u16() != 405 {
                return Err(JevError::Status(status.as_u16()));
            }
        }
        let body = body.ok_or(JevError::Status(last_status.unwrap_or(404)))?;
        Ok((parse_model_list(&body), base))
    }

    pub async fn evaluate(
        &self,
        state: Value,
        questions: BTreeMap<String, Question>,
    ) -> Result<Judgment, JevError> {
        if let Ok(script) = self.script.lock()
            && !script.is_empty()
        {
            return Ok(scripted(&questions, &script));
        }
        // A pinned router is a test. Rank with the same fixed scores so the
        // rest of the run does not need a live key.
        if self.llm_pinned() {
            return Ok(scripted(&questions, &[]));
        }
        let (key, base_url, model) = self.snapshot();
        let Some(key) = key else {
            return Err(JevError::Unconfigured);
        };
        let body = json!({"state": state, "model": model, "questions": questions});
        let mut attempt = 0;
        loop {
            let response = self
                .client
                .post(format!("{base_url}/v1/systemone"))
                .bearer_auth(&key)
                .header(header::CONTENT_TYPE, "application/json")
                .json(&body)
                .send()
                .await;
            match response {
                Ok(response) => {
                    let status = response.status();
                    if status.is_success() {
                        let result: Judgment = response.json().await?;
                        validate(&result, &questions)?;
                        return Ok(result);
                    }
                    let retryable = status.as_u16() == 408
                        || status.as_u16() == 429
                        || status.is_server_error();
                    if !retryable || attempt >= 2 {
                        return Err(JevError::Status(status.as_u16()));
                    }
                }
                Err(error) => {
                    if attempt >= 2 {
                        return Err(JevError::Transport(error));
                    }
                }
            }
            tokio::time::sleep(Duration::from_millis(250 * (1 << attempt))).await;
            attempt += 1;
        }
    }
}

fn scripted(questions: &BTreeMap<String, Question>, preferred: &[String]) -> Judgment {
    let mut answers = BTreeMap::new();
    for (id, question) in questions {
        let answer = match question.kind.as_str() {
            "choice" => {
                let criteria = question
                    .criteria
                    .as_ref()
                    .and_then(Value::as_object)
                    .cloned()
                    .unwrap_or_default();
                // `~id` scripts an unsure pick: the same choice at 0.5, which is
                // how tests reach the paths a doubtful route takes.
                let (choice, chosen) = preferred
                    .iter()
                    .find_map(|cid| match cid.strip_prefix('~') {
                        Some(id) if criteria.contains_key(id) => Some((id.to_string(), 0.5)),
                        None if criteria.contains_key(cid) => Some((cid.clone(), 0.95)),
                        _ => None,
                    })
                    .or_else(|| criteria.keys().next().map(|id| (id.clone(), 0.95)))
                    .unwrap_or_default();
                let mut probabilities = BTreeMap::new();
                let rest = if criteria.len() > 1 {
                    (1.0 - chosen) / (criteria.len() - 1) as f64
                } else {
                    0.0
                };
                for key in criteria.keys() {
                    probabilities.insert(key.clone(), if *key == choice { chosen } else { rest });
                }
                Answer::Choice {
                    choice,
                    probabilities,
                    confidence: 0.9,
                }
            }
            "noul" => Answer::Noul { noul: 0.8 },
            _ => Answer::Score {
                score: 1.6,
                confidence: 0.9,
            },
        };
        answers.insert(id.clone(), answer);
    }
    Judgment {
        answers,
        _model: "script".into(),
        usage: Default::default(),
    }
}

/// Candidate model-list URLs. A base that already ends with `/v1` is used as-is;
/// otherwise we try `{base}/v1/models` then `{base}/models` (OpenAI-compatible).
fn model_list_urls(base: &str) -> Vec<String> {
    let base = base.trim_end_matches('/');
    if base.ends_with("/v1") {
        vec![format!("{base}/models")]
    } else {
        vec![format!("{base}/v1/models"), format!("{base}/models")]
    }
}

fn model_name(entry: &Value) -> Option<String> {
    entry
        .get("id")
        .or_else(|| entry.get("name"))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn parse_model_list(body: &Value) -> Vec<crate::models::ModelInfo> {
    let rows = body
        .get("data")
        .or_else(|| body.get("models"))
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut seen = BTreeSet::new();
    let mut models = Vec::new();
    for entry in rows {
        let Some(name) = model_name(&entry) else {
            continue;
        };
        if !seen.insert(name.clone()) {
            continue;
        }
        models.push(crate::models::ModelInfo {
            name,
            description: entry
                .get("description")
                .or_else(|| entry.get("owned_by"))
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            release_date: entry
                .get("release_date")
                .and_then(Value::as_str)
                .map(str::to_string)
                .or_else(|| {
                    entry.get("created").and_then(|v| match v {
                        Value::Number(n) => Some(n.to_string()),
                        Value::String(s) => Some(s.clone()),
                        _ => None,
                    })
                })
                .unwrap_or_default(),
        });
    }
    models
}

/// One LLM routing call and the options it was shown. Probabilities are absent:
/// the model returns an id, not a distribution.
#[derive(Clone, Debug)]
pub struct LlmRouteStep {
    pub depth: usize,
    pub parent_name: String,
    pub candidates: Vec<(String, String)>,
    pub choice_id: Option<String>,
    pub choice_name: String,
}

#[derive(Clone, Debug)]
pub struct LlmRoute {
    pub leaf: Option<String>,
    pub steps: Vec<LlmRouteStep>,
    /// What the route cost: chat completions and the tokens they reported.
    pub meter: LlmMeter,
}

/// Chat completions one route made and the tokens the endpoint reported.
#[derive(Clone, Copy, Debug, Default)]
pub struct LlmMeter {
    pub calls: usize,
    pub usage: crate::models::Usage,
}

struct LlmCreds {
    base: String,
    token: String,
    model: String,
}

/// The one-shot routing prompt: every node in `lines`, pick one. Shared by the
/// LLM router and the review of a doubtful Jev pick, so both read the same rules.
fn one_shot_prompt(
    abstain: bool,
    allowed: &[String],
    query: &str,
    background: &str,
    lines: &[String],
) -> String {
    let head = if abstain {
        "요청을 가장 정확히 다루는 노드 하나를 고르세요. \
         자식이 남아 있으면 더 구체적인 자식 노드를 고르고, 주제만 묻고 하위 사례를 특정하지 않으면 그 노드에 머무세요. \
         어느 노드에도 속하지 않으면 __none__."
    } else {
        "요청을 가장 정확히 다루는 노드 하나를 고르세요. \
         자식이 남아 있으면 더 구체적인 자식 노드를 고르고, 주제만 묻고 하위 사례를 특정하지 않으면 그 노드에 머무세요. \
         범위를 벗어나는 주제에는 시작 노드 자신을 고르세요."
    };
    format!(
        "{head}\n노드 설명이 겹치면 더 구체적인 쪽을 고르세요. 경로는 위치를 알려 줄 뿐이고, \
판단은 현재 요청과 각 설명으로 하세요.\n허용된 id: {}\n\n{}\n\n{}\n\n마지막 줄에 id 하나만.",
        allowed.join(", "),
        request_block(query, background),
        lines.join("\n")
    )
}

fn chat_completions_url(base: &str) -> String {
    let base = base.trim().trim_end_matches('/');
    if base.ends_with("/v1") {
        format!("{base}/chat/completions")
    } else {
        format!("{base}/v1/chat/completions")
    }
}

fn clip(text: &str, max_chars: usize) -> String {
    text.chars().take(max_chars).collect()
}

fn request_block(query: &str, background: &str) -> String {
    let background = background.trim();
    if background.is_empty() || background == query.trim() {
        format!("요청: {query}")
    } else {
        format!("요청: {query}\n배경: {}", clip(background, 1500))
    }
}

fn node_index(nodes: &[Node]) -> BTreeMap<&str, &Node> {
    nodes.iter().map(|node| (node.id.as_str(), node)).collect()
}

fn ancestor_names(nodes: &[Node], id: &str) -> Vec<String> {
    let by_id = node_index(nodes);
    let mut chain = Vec::new();
    let mut cursor = Some(id);
    let mut guard = 0;
    while let Some(current) = cursor {
        guard += 1;
        if guard > nodes.len() + 1 {
            break;
        }
        let Some(node) = by_id.get(current) else {
            break;
        };
        chain.push(node.name.clone());
        cursor = node.parent_id.as_deref();
    }
    chain.reverse();
    chain
}

/// One row per node in scope, in tree order, plus `__none__` when the walk may
/// abstain. Returns the menu text, the (id, name) options for the trace, and the
/// allowed ids the model may answer with.
pub fn llm_menu(
    nodes: &[Node],
    scope: &[String],
    abstain: bool,
) -> (Vec<String>, Vec<(String, String)>, Vec<String>) {
    let by_id = node_index(nodes);
    let mut menu = Vec::with_capacity(scope.len() + 1);
    let mut options = Vec::with_capacity(scope.len() + 1);
    let mut allowed = Vec::with_capacity(scope.len() + 1);
    for id in scope {
        let Some(node) = by_id.get(id.as_str()) else {
            continue;
        };
        menu.push(format!(
            "{id}\t{}\t{}",
            ancestor_names(nodes, id).join(" / "),
            clip(&node.description, 180)
        ));
        options.push((id.clone(), node.name.clone()));
        allowed.push(id.clone());
    }
    if abstain {
        menu.push("__none__\t없음\t어느 주제에도 속하지 않는다.".into());
        options.push(("__none__".into(), "no matching topic".into()));
        allowed.push("__none__".into());
    }
    (menu, options, allowed)
}

/// Output budget for one routing reply. A reasoning model bills its thinking
/// against this, so it has to cover deliberating over the menu as well as the
/// one id that comes out the far end. The reply itself is tiny — the room is
/// for the thinking, and a ceiling that cuts it off leaves `content` empty.
const ROUTE_MAX_TOKENS: u32 = 16384;

/// Ceiling for endpoints whose own cap is below [`ROUTE_MAX_TOKENS`]. Used only
/// after a 400, so the common path keeps the full budget.
const ROUTE_MAX_TOKENS_FALLBACK: u32 = 4096;

fn plain_chat_payload(payload: &mut Value) {
    if let Some(map) = payload.as_object_mut() {
        map.remove("reasoning_effort");
        map.insert("temperature".into(), json!(0));
    }
}

fn shrink_max_tokens(payload: &mut Value) {
    if let Some(map) = payload.as_object_mut() {
        map.insert("max_tokens".into(), json!(ROUTE_MAX_TOKENS_FALLBACK));
    }
}

/// Text the router can parse. Prefer the assistant content. If that is empty,
/// use a reasoning field or a legacy `choices[0].text`.
fn assistant_text(body: &Value) -> String {
    let message = body.pointer("/choices/0/message");
    if let Some(text) = message_text(message.and_then(|m| m.get("content"))) {
        return text;
    }
    for key in ["reasoning_content", "reasoning"] {
        if let Some(text) = message_text(message.and_then(|m| m.get(key))) {
            return text;
        }
    }
    message_text(body.pointer("/choices/0/text")).unwrap_or_default()
}

fn message_text(value: Option<&Value>) -> Option<String> {
    let value = value?;
    let text = match value {
        Value::String(text) => text.clone(),
        Value::Array(parts) => parts
            .iter()
            .filter_map(|part| {
                part.as_str()
                    .map(str::to_string)
                    .or_else(|| part.get("text").and_then(Value::as_str).map(str::to_string))
            })
            .collect::<Vec<_>>()
            .join("\n"),
        _ => return None,
    };
    let text = text.trim().to_string();
    if text.is_empty() { None } else { Some(text) }
}

/// Last line must be one allowed id. Scanning the whole reply is unsafe:
/// `orders` is a substring of `orders_tracking`.
pub fn parse_final_id(content: &str, allowed: &[String]) -> Option<String> {
    let last = content.lines().rev().find(|line| !line.trim().is_empty())?;
    let mut last = last.trim().trim_matches(|c| "`\"'.".contains(c));
    for prefix in ["ID:", "id:", "ID：", "id："] {
        if let Some(rest) = last.strip_prefix(prefix) {
            last = rest.trim();
        }
    }
    if allowed.iter().any(|id| id == last) {
        return Some(last.to_string());
    }
    let hits: Vec<&String> = allowed
        .iter()
        .filter(|id| {
            last.split(|c: char| !c.is_ascii_alphanumeric() && c != '_' && c != '-')
                .any(|token| token == id.as_str())
        })
        .collect();
    if hits.len() == 1 {
        return Some(hits[0].clone());
    }
    None
}

/// The node the model named. [`parse_final_id`] first, then the whole reply:
/// a reasoning model often leaves its pick in the middle of a sentence, and a
/// weak one answers with the node's name instead of its id. Ambiguous replies
/// (two ids, two names) return `None` so the caller can retry rather than guess.
pub fn parse_final_choice(content: &str, choices: &[(String, String)]) -> Option<String> {
    let allowed: Vec<String> = choices.iter().map(|(id, _)| id.clone()).collect();
    if let Some(id) = parse_final_id(content, &allowed) {
        return Some(id);
    }
    // Whole-reply scan, still whole-token so `orders` never matches inside
    // `orders_tracking`. The last mention wins: that is the model's conclusion.
    let mut found: Option<String> = None;
    let mut ambiguous = false;
    for line in content.lines() {
        let hits: Vec<&String> = allowed
            .iter()
            .filter(|id| {
                line.split(|c: char| !c.is_ascii_alphanumeric() && c != '_' && c != '-')
                    .any(|token| token == id.as_str())
            })
            .collect();
        match hits.len() {
            0 => {}
            1 => {
                found = Some(hits[0].clone());
                ambiguous = false;
            }
            _ => ambiguous = true,
        }
    }
    if let Some(id) = found
        && !ambiguous
    {
        return Some(id);
    }
    // Last resort: the node name. Only an unambiguous name counts.
    let last = content.lines().rev().find(|line| !line.trim().is_empty())?;
    let mut named: Option<&String> = None;
    for (id, name) in choices {
        let name = name.trim();
        if name.is_empty() || !last.contains(name) {
            continue;
        }
        if named.is_some() {
            return None;
        }
        named = Some(id);
    }
    named.cloned()
}

fn validate(result: &Judgment, questions: &BTreeMap<String, Question>) -> Result<(), JevError> {
    for (id, question) in questions {
        let answer = result
            .answers
            .get(id)
            .ok_or_else(|| JevError::Invalid(format!("missing answer {id}")))?;
        match (question.kind.as_str(), answer) {
            (
                "choice",
                Answer::Choice {
                    choice,
                    probabilities,
                    ..
                },
            ) => {
                let criteria = question
                    .criteria
                    .as_ref()
                    .and_then(Value::as_object)
                    .ok_or_else(|| JevError::Invalid("choice criteria missing".into()))?;
                if !criteria.contains_key(choice)
                    || criteria.keys().any(|id| !probabilities.contains_key(id))
                    || probabilities
                        .values()
                        .any(|p| !p.is_finite() || !(0.0..=1.0).contains(p))
                {
                    return Err(JevError::Invalid(format!("invalid choice {id}")));
                }
            }
            ("noul", Answer::Noul { noul }) if noul.is_finite() && (0.0..=1.0).contains(noul) => (),
            ("score", Answer::Score { score, .. })
                if score.is_finite() && (0.0..=2.0).contains(score) => {}
            _ => return Err(JevError::Invalid(format!("wrong answer type/range {id}"))),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// DB settings are the source of truth; a config built from env is overwritten by them.
    #[test]
    fn db_settings_override_the_config() {
        let mut config = JevConfig {
            key: Some("env-key".into()),
            base_url: "https://api.typesafe.ai".into(),
            model: "jev-latest".into(),
            ..JevConfig::default()
        };
        config.with_db(&crate::models::AppSettings {
            jev_api_key: "db-key".into(),
            llm_base_url: "https://api.openai.com/v1".into(),
            llm_model: "gpt-4o-mini".into(),
            ..Default::default()
        });
        let (key, base, model) = config.normalized();
        assert_eq!(key.as_deref(), Some("db-key"));
        assert_eq!(base, "https://api.typesafe.ai");
        assert_eq!(model, "jev-latest");
        assert_eq!(config.llm_model, "gpt-4o-mini");
        assert_eq!(config.llm_base_url, "https://api.openai.com/v1");
    }

    #[test]
    fn urls_for_openai_style_v1_base() {
        assert_eq!(
            model_list_urls("https://api.openai.com/v1"),
            vec!["https://api.openai.com/v1/models".to_string()]
        );
    }

    #[test]
    fn urls_for_bare_host_try_v1_then_root() {
        assert_eq!(
            model_list_urls("https://proxy.example"),
            vec![
                "https://proxy.example/v1/models".to_string(),
                "https://proxy.example/models".to_string(),
            ]
        );
    }

    #[test]
    fn parse_openai_data_id() {
        let models = parse_model_list(&json!({
            "object": "list",
            "data": [
                {"id": "gpt-4o-mini", "owned_by": "openai"},
                {"id": "gpt-4o", "owned_by": "openai"},
                {"id": "gpt-4o"}
            ]
        }));
        assert_eq!(
            models.iter().map(|m| m.name.as_str()).collect::<Vec<_>>(),
            vec!["gpt-4o-mini", "gpt-4o"]
        );
        assert_eq!(models[0].description, "openai");
    }

    #[test]
    fn parse_typesafe_models_name() {
        let models = parse_model_list(&json!({
            "models": [{"name": "jev-latest", "description": "alias", "release_date": "2026-01-01"}]
        }));
        assert_eq!(models[0].name, "jev-latest");
        assert_eq!(models[0].description, "alias");
        assert_eq!(models[0].release_date, "2026-01-01");
    }

    #[test]
    fn parse_empty_or_unknown_shape_is_empty() {
        assert!(parse_model_list(&json!({"object": "list"})).is_empty());
        assert!(parse_model_list(&json!([])).is_empty());
    }

    #[test]
    fn assistant_text_reads_content_then_reasoning() {
        let content = json!({"choices":[{"message":{"content":" orders_tracking "}}]});
        assert_eq!(assistant_text(&content), "orders_tracking");
        let parts =
            json!({"choices":[{"message":{"content":[{"text":"note"},{"text":"orders"}]}}]});
        assert_eq!(assistant_text(&parts), "note\norders");
        let reasoning =
            json!({"choices":[{"message":{"content":null,"reasoning_content":"account"}}]});
        assert_eq!(assistant_text(&reasoning), "account");
        let empty = json!({"choices":[{"message":{"content":""}}]});
        assert_eq!(assistant_text(&empty), "");
    }

    #[test]
    fn parse_final_id_uses_the_last_line_only() {
        let allowed = vec!["orders".into(), "orders_tracking".into(), "__none__".into()];
        assert_eq!(
            parse_final_id("orders is mentioned\norders_tracking", &allowed).as_deref(),
            Some("orders_tracking")
        );
        assert_eq!(
            parse_final_id("reasoning\nID: __none__", &allowed).as_deref(),
            Some("__none__")
        );
        // `orders` is contained in `orders_tracking`, so a line that only names the
        // longer id must not resolve to the prefix.
        assert_eq!(
            parse_final_id("pick orders_tracking", &allowed).as_deref(),
            Some("orders_tracking")
        );
        assert!(parse_final_id("orders or orders_tracking", &allowed).is_none());
    }

    #[test]
    fn parse_final_choice_survives_reasoning_prose_and_names() {
        let choices = vec![
            ("orders".to_string(), "주문".to_string()),
            ("orders_tracking".to_string(), "배송 추적".to_string()),
            ("account".to_string(), "계정".to_string()),
        ];
        // The pick sits mid-sentence — the reasoning models' usual shape.
        assert_eq!(
            parse_final_choice("orders_tracking이 더 정확합니다.\n그 이유는...", &choices)
                .as_deref(),
            Some("orders_tracking")
        );
        // The last unambiguous mention wins.
        assert_eq!(
            parse_final_choice("orders일까?\n아니다, orders_tracking.", &choices).as_deref(),
            Some("orders_tracking")
        );
        // A weak model answers with the node name, not the id.
        assert_eq!(
            parse_final_choice("배송 추적", &choices).as_deref(),
            Some("orders_tracking")
        );
        // Two names on the answer line is still a guess we refuse to make.
        assert!(parse_final_choice("주문 또는 배송 추적", &choices).is_none());
    }

    fn menu_fixture() -> Vec<Node> {
        vec![
            Node {
                id: "it".into(),
                name: "IT".into(),
                description: "기기 문제".into(),
                examples: vec![],
                parent_id: None,
            },
            Node {
                id: "it_net".into(),
                name: "네트워크".into(),
                description: "연결 문제".into(),
                examples: vec![],
                parent_id: Some("it".into()),
            },
            Node {
                id: "it_net_wifi".into(),
                name: "와이파이".into(),
                description: "무선 연결".into(),
                examples: vec![],
                parent_id: Some("it_net".into()),
            },
            Node {
                id: "it_net_wifi_drop".into(),
                name: "끊김".into(),
                description: "주기적 단절".into(),
                examples: vec![],
                parent_id: Some("it_net_wifi".into()),
            },
        ]
    }

    #[test]
    fn one_shot_menu_names_the_whole_path_and_caps_at_180() {
        let nodes = menu_fixture();
        let long = "가".repeat(400);
        let mut nodes = nodes;
        nodes[3].description = long;
        let scope: Vec<String> = nodes.iter().map(|n| n.id.clone()).collect();
        let (menu, options, allowed) = llm_menu(&nodes, &scope, false);
        assert_eq!(menu.len(), 4);
        // 심층 노드는 조상 경로를 함께 보여 준다. 라우터가 위치만 보고 헷갈리지 않게.
        assert!(
            menu[3].starts_with("it_net_wifi_drop\tIT / 네트워크 / 와이파이 / 끊김\t"),
            "{}",
            menu[3]
        );
        // 설명은 180자에서 잘린다. 메뉴가 트리 크기만큼 커지지 않게.
        assert_eq!(
            menu[3].chars().count(),
            "it_net_wifi_drop\tIT / 네트워크 / 와이파이 / 끊김\t"
                .chars()
                .count()
                + 180
        );
        assert_eq!(options.len(), 4);
        assert_eq!(allowed.len(), 4);
        assert!(
            !allowed.iter().any(|id| id == "__none__"),
            "start_node 아래에서는 보류할 수 없다"
        );
    }

    #[test]
    fn one_shot_menu_offers_none_only_for_the_whole_forest() {
        let nodes = menu_fixture();
        let scope: Vec<String> = nodes.iter().map(|n| n.id.clone()).collect();
        let (menu, options, allowed) = llm_menu(&nodes, &scope, true);
        assert_eq!(
            menu.last().unwrap(),
            "__none__\t없음\t어느 주제에도 속하지 않는다."
        );
        assert_eq!(
            options.last().unwrap(),
            &("__none__".to_string(), "no matching topic".to_string())
        );
        assert!(allowed.iter().any(|id| id == "__none__"));
    }

    #[test]
    fn one_shot_menu_skips_ids_that_left_the_scope() {
        let nodes = menu_fixture();
        let scope = vec!["it_net_wifi_drop".to_string(), "없는노드".to_string()];
        let (menu, options, allowed) = llm_menu(&nodes, &scope, false);
        assert_eq!(menu.len(), 1);
        assert_eq!(options.len(), 1);
        assert_eq!(allowed, vec!["it_net_wifi_drop".to_string()]);
    }
}
