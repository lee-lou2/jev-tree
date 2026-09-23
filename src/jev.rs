use crate::models::{Answer, Judgment, Node, Question};
use reqwest::{Client, header};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Arc, Mutex, RwLock},
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
    config: std::sync::Arc<std::sync::RwLock<JevConfig>>,
    /// Read by the lexical heuristic, which only the unit tests call.
    #[cfg(test)]
    nodes: Arc<RwLock<Vec<Node>>>,
    /// Test-only preferred Choice ids. Empty in production.
    script: Arc<Mutex<Vec<String>>>,
    /// Test-only leaf for [`Self::route`]. `None` means "use the model".
    llm_script: Arc<Mutex<Option<String>>>,
}

impl JevClient {
    pub fn with_config(
        config: JevConfig,
        nodes: Arc<RwLock<Vec<Node>>>,
    ) -> Result<Self, reqwest::Error> {
        let client = Client::builder()
            .timeout(Duration::from_secs(20))
            .connect_timeout(Duration::from_secs(5))
            .pool_idle_timeout(Duration::from_secs(60))
            .redirect(reqwest::redirect::Policy::none())
            .user_agent("jev-tree/0.4")
            .build()?;
        #[cfg(not(test))]
        let _nodes = nodes;
        Ok(Self {
            client,
            config: std::sync::Arc::new(std::sync::RwLock::new(config)),
            #[cfg(test)]
            nodes,
            script: Arc::new(Mutex::new(Vec::new())),
            llm_script: Arc::new(Mutex::new(None)),
        })
    }

    pub fn script_choices(&self, ids: Vec<String>) {
        if let Ok(mut guard) = self.script.lock() {
            *guard = ids;
        }
    }

    /// Pin the router leaf in tests. `__none__` abstains. `None` clears the pin.
    pub fn script_llm(&self, leaf: Option<String>) {
        if let Ok(mut guard) = self.llm_script.lock() {
            *guard = leaf;
        }
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

    /// Test pin for [`Self::route`]. Production leaves this empty.
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

    /// Pick the node a request should be stored under and retrieved from.
    ///
    /// Two calls when an LLM is configured and the walk is the whole forest: the
    /// root topic, then one node inside that subtree (a lexical shortlist plus
    /// each candidate's parent). A `restrict` node skips the root call. A failed
    /// call is the caller's signal to use the Jev beam. `__none__` abstains.
    /// Item ranking does not use this model. `note` fires before each model call
    /// and again when that call chooses.
    pub async fn route<F>(
        &self,
        query: &str,
        background: &str,
        nodes: &[Node],
        restrict: Option<&str>,
        mut note: F,
    ) -> Result<LlmRoute, JevError>
    where
        F: FnMut(RouteNote),
    {
        if let Ok(guard) = self.llm_script.lock()
            && let Some(leaf) = guard.as_ref()
        {
            return Ok(LlmRoute {
                leaf: if leaf == "__none__" || leaf.trim().is_empty() {
                    None
                } else {
                    Some(leaf.clone())
                },
                steps: vec![],
            });
        }
        let mut step = Stepper {
            depth: 0,
            note: &mut note,
            steps: Vec::new(),
        };
        let (base, token, model) = {
            let config = self
                .config
                .read()
                .map_err(|_| JevError::Invalid("llm config unavailable".into()))?;
            (
                config.llm_base_url.clone(),
                config.llm_token.clone().unwrap_or_default(),
                config.llm_model.clone(),
            )
        };
        if token.trim().is_empty() || model.trim().is_empty() || base.trim().is_empty() {
            return Err(JevError::Invalid("LLM route is not configured".into()));
        }
        let by_id = node_index(nodes);
        if let Some(start) = restrict {
            if !by_id.contains_key(start) {
                return Err(JevError::Invalid(format!("restrict not in scope: {start}")));
            }
            if children_of(nodes, Some(start)).is_empty() {
                return Ok(LlmRoute {
                    leaf: Some(start.to_string()),
                    steps: step.steps,
                });
            }
            let leaf = self
                .route_within(
                    &LlmCreds { base, token, model },
                    query,
                    background,
                    nodes,
                    start,
                    &mut step,
                )
                .await?;
            return Ok(LlmRoute {
                leaf,
                steps: step.steps,
            });
        }
        let roots = children_of(nodes, None);
        if roots.is_empty() {
            return Ok(LlmRoute {
                leaf: None,
                steps: step.steps,
            });
        }
        if roots.len() == 1 && children_of(nodes, Some(roots[0].as_str())).is_empty() {
            return Ok(LlmRoute {
                leaf: Some(roots[0].clone()),
                steps: step.steps,
            });
        }
        let mut allowed = roots.to_vec();
        allowed.push("__none__".into());
        let mut lines = Vec::new();
        for id in &roots {
            let node = &by_id[id.as_str()];
            let kids = children_of(nodes, Some(id))
                .iter()
                .take(8)
                .map(|cid| by_id[cid.as_str()].name.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            lines.push(format!(
                "{id}\t{}\t{}\t아래: {kids}",
                node.name,
                clip(&node.description, 180),
            ));
        }
        lines.push("__none__\t없음\t어느 주제에도 속하지 않는다.\t아래:".into());
        let mut options: Vec<(String, String)> = roots
            .iter()
            .map(|id| (id.clone(), by_id[id.as_str()].name.clone()))
            .collect();
        options.push(("__none__".into(), "no matching topic".into()));
        step.ask("the knowledge root");
        let root = self
            .complete_id(
                &base,
                &token,
                &model,
                &format!(
                    "요청의 최상위 주제를 하나 고르세요. 각 설명과 하위 주제를 읽고, 형제 주제와의 차이를 기준으로 고르세요. 어느 주제에도 속하지 않으면 __none__.\n허용된 id: {}\n\n{}\n\n{}\n\n마지막 줄에 id 하나만.",
                    allowed.join(", "),
                    request_block(query, background),
                    lines.join("\n")
                ),
                &allowed,
            )
            .await?;
        if root == "__none__" {
            step.chose(
                "the knowledge root",
                options,
                None,
                "no matching topic".into(),
            );
            return Ok(LlmRoute {
                leaf: None,
                steps: step.steps,
            });
        }
        let root_name = by_id
            .get(root.as_str())
            .map(|node| node.name.clone())
            .unwrap_or_else(|| root.clone());
        step.chose("the knowledge root", options, Some(root.clone()), root_name);
        let leaf = self
            .route_within(
                &LlmCreds { base, token, model },
                query,
                background,
                nodes,
                &root,
                &mut step,
            )
            .await?;
        Ok(LlmRoute {
            leaf,
            steps: step.steps,
        })
    }

    async fn route_within<F>(
        &self,
        creds: &LlmCreds,
        query: &str,
        background: &str,
        nodes: &[Node],
        root: &str,
        step: &mut Stepper<'_, F>,
    ) -> Result<Option<String>, JevError>
    where
        F: FnMut(RouteNote),
    {
        let by_id = node_index(nodes);
        let root_name = by_id
            .get(root)
            .map(|node| node.name.clone())
            .unwrap_or_else(|| root.to_string());
        let short = llm_shortlist(query, nodes, root, 20);
        if short.is_empty() {
            return Ok(Some(root.to_string()));
        }
        if short.len() == 1 {
            return Ok(Some(short[0].clone()));
        }
        let options: Vec<(String, String)> = short
            .iter()
            .map(|id| {
                let name = by_id
                    .get(id.as_str())
                    .map(|node| node.name.clone())
                    .unwrap_or_else(|| id.clone());
                (id.clone(), name)
            })
            .collect();
        let lines = options
            .iter()
            .filter_map(|(id, _)| by_id.get(id.as_str()).map(|node| (id, node)))
            .map(|(id, node)| {
                format!(
                    "{id}\t{}\t{}",
                    ancestor_names(nodes, id).join(" / "),
                    clip(&node.description, 180)
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        let prompt = format!(
            "요청을 가장 정확히 보관할 노드 하나를 고르세요. 요청이 더 구체적인 조건을 말하면 그 하위 노드를 고르고, 주제만 묻고 하위 사례를 특정하지 않으면 그 주제 노드에 머물세요.\n허용된 id: {}\n\n{}\n\n{lines}\n\n마지막 줄에 id 하나만.",
            short.join(", "),
            request_block(query, background)
        );
        step.ask(&root_name);
        let picked = self
            .complete_id(&creds.base, &creds.token, &creds.model, &prompt, &short)
            .await?;
        let picked_name = by_id
            .get(picked.as_str())
            .map(|node| node.name.clone())
            .unwrap_or_else(|| picked.clone());
        if picked == root {
            step.chose(&root_name, options, None, "stay here".into());
        } else {
            step.chose(&root_name, options, Some(picked.clone()), picked_name);
        }
        Ok(Some(picked))
    }

    async fn complete_id(
        &self,
        base: &str,
        token: &str,
        model: &str,
        user: &str,
        allowed: &[String],
    ) -> Result<String, JevError> {
        let content = self.chat(base, token, model, user).await?;
        if let Some(id) = parse_final_id(&content, allowed) {
            return Ok(id);
        }
        let retry =
            format!("{user}\n\n마지막 줄에 허용된 id 하나만 다시 쓰세요. 설명은 쓰지 마세요.");
        let content = self.chat(base, token, model, &retry).await?;
        parse_final_id(&content, allowed)
            .ok_or_else(|| JevError::Invalid("LLM route did not return an allowed id".into()))
    }

    async fn chat(
        &self,
        base: &str,
        token: &str,
        model: &str,
        user: &str,
    ) -> Result<String, JevError> {
        let url = chat_completions_url(base);
        let messages = json!([
            {"role": "system", "content": "분류기입니다. 마지막 줄에는 허용된 id 하나만 씁니다."},
            {"role": "user", "content": user},
        ]);
        let mut payload = json!({
            "model": model,
            "reasoning_effort": "low",
            "max_tokens": 900,
            "messages": messages,
        });
        let first = match self.post_chat(&url, token, &payload).await {
            Err(JevError::Status(400)) => {
                plain_chat_payload(&mut payload);
                self.post_chat(&url, token, &payload).await?
            }
            other => other?,
        };
        if !first.is_empty() {
            return Ok(first);
        }
        // Some models spend the reply on reasoning and leave content empty.
        // Ask once more in plain text, then give up so the Jev beam can run.
        plain_chat_payload(&mut payload);
        let second = self.post_chat(&url, token, &payload).await?;
        if second.is_empty() {
            return Err(JevError::Invalid("LLM returned no content".into()));
        }
        Ok(second)
    }

    async fn post_chat(&self, url: &str, token: &str, payload: &Value) -> Result<String, JevError> {
        let response = self
            .client
            .post(url)
            .timeout(Duration::from_secs(45))
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

    #[cfg(test)]
    fn heuristic(&self, state: &Value, questions: &BTreeMap<String, Question>) -> Judgment {
        let text = state
            .get("current_request")
            .or_else(|| state.get("document"))
            .or_else(|| state.get("conversation"))
            .and_then(Value::as_str)
            .unwrap_or("");
        let query_tokens = tokens(text);
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
                    let mut children = BTreeMap::new();
                    let mut terminals = Vec::new();
                    for (cid, desc) in &criteria {
                        if cid.starts_with("__") {
                            terminals.push(cid.clone());
                            continue;
                        }
                        // Score only this choice's label/description. Descendant names
                        // must not steal a shallower sibling match.
                        let mut contents = desc.as_str().unwrap_or("").to_string();
                        if let Ok(nodes) = self.nodes.read()
                            && let Some(node) = nodes.iter().find(|n| n.id == *cid)
                        {
                            contents.push_str(&format!(" {} {}", node.name, node.description));
                        }
                        children.insert(cid.clone(), overlap_score(&query_tokens, text, &contents));
                    }
                    let bar = terminal_bar(&children);
                    let mut weights: BTreeMap<String, f64> = children
                        .into_iter()
                        .map(|(cid, overlap)| (cid, overlap.exp()))
                        .collect();
                    for cid in terminals {
                        weights.insert(cid, bar.exp());
                    }
                    let total: f64 = weights.values().sum();
                    let probabilities: BTreeMap<String, f64> = weights
                        .into_iter()
                        .map(|(k, v)| (k, v / total.max(1e-9)))
                        .collect();
                    let choice = probabilities
                        .iter()
                        .max_by(|a, b| a.1.total_cmp(b.1))
                        .map(|(id, _)| id.clone())
                        .unwrap_or_default();
                    Answer::Choice {
                        choice,
                        probabilities,
                        confidence: 0.6,
                    }
                }
                kind => {
                    let item_id = id
                        .strip_prefix("use_")
                        .or_else(|| id.strip_prefix("direct_"))
                        .unwrap_or(id);
                    let item_text = state
                        .get("items")
                        .and_then(Value::as_array)
                        .and_then(|items| {
                            items
                                .iter()
                                .find(|i| i.get("id").and_then(Value::as_str) == Some(item_id))
                        })
                        .map(|item| {
                            format!(
                                "{} {}",
                                item.get("question").and_then(Value::as_str).unwrap_or(""),
                                item.get("answer").and_then(Value::as_str).unwrap_or("")
                            )
                        })
                        .unwrap_or_default();
                    let overlap = overlap_score(&query_tokens, text, &item_text);
                    if kind == "noul" {
                        Answer::Noul {
                            noul: (0.25 + 0.18 * overlap).min(0.96),
                        }
                    } else {
                        Answer::Score {
                            score: (0.4 + 0.4 * overlap).min(2.0),
                            confidence: 0.6,
                        }
                    }
                }
            };
            answers.insert(id.clone(), answer);
        }
        Judgment {
            answers,
            _model: "heuristic".into(),
            _usage: Default::default(),
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
                let choice = preferred
                    .iter()
                    .find(|cid| criteria.contains_key(*cid))
                    .cloned()
                    .or_else(|| criteria.keys().next().cloned())
                    .unwrap_or_default();
                let mut probabilities = BTreeMap::new();
                let rest = if criteria.len() > 1 {
                    0.05 / (criteria.len() - 1) as f64
                } else {
                    0.0
                };
                for key in criteria.keys() {
                    probabilities.insert(key.clone(), if *key == choice { 0.95 } else { rest });
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
        _usage: Default::default(),
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

/// One routing beat the UI can draw while the model call is in flight.
#[derive(Clone, Debug)]
pub struct RouteNote {
    pub depth: usize,
    pub asking: bool,
    pub parent_name: String,
    pub choice_id: Option<String>,
    pub choice_name: String,
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
}

struct Stepper<'a, F> {
    depth: usize,
    note: &'a mut F,
    steps: Vec<LlmRouteStep>,
}

impl<F: FnMut(RouteNote)> Stepper<'_, F> {
    fn ask(&mut self, parent: &str) {
        (self.note)(RouteNote {
            depth: self.depth,
            asking: true,
            parent_name: parent.to_string(),
            choice_id: None,
            choice_name: String::new(),
        });
    }

    fn chose(
        &mut self,
        parent: &str,
        candidates: Vec<(String, String)>,
        choice_id: Option<String>,
        choice_name: String,
    ) {
        self.steps.push(LlmRouteStep {
            depth: self.depth,
            parent_name: parent.to_string(),
            candidates,
            choice_id: choice_id.clone(),
            choice_name: choice_name.clone(),
        });
        (self.note)(RouteNote {
            depth: self.depth,
            asking: false,
            parent_name: parent.to_string(),
            choice_id,
            choice_name,
        });
        self.depth += 1;
    }
}

struct LlmCreds {
    base: String,
    token: String,
    model: String,
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

fn children_of(nodes: &[Node], parent: Option<&str>) -> Vec<String> {
    let mut ids: Vec<String> = nodes
        .iter()
        .filter(|node| node.parent_id.as_deref() == parent)
        .map(|node| node.id.clone())
        .collect();
    ids.sort();
    ids
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

fn subtree_ids(nodes: &[Node], root: &str) -> Vec<String> {
    let mut ids = vec![root.to_string()];
    let mut cursor = 0;
    while cursor < ids.len() {
        let parent = ids[cursor].clone();
        for child in children_of(nodes, Some(parent.as_str())) {
            if !ids.iter().any(|id| id == &child) {
                ids.push(child);
            }
        }
        cursor += 1;
    }
    ids
}

/// Top lexical matches inside `root`'s subtree, plus each match's parent when it
/// is still inside that subtree. Parents stay visible so a specific child cannot
/// hide the topic the request actually named.
pub fn llm_shortlist(query: &str, nodes: &[Node], root: &str, k: usize) -> Vec<String> {
    let inside = subtree_ids(nodes, root);
    let query_tokens = tokens(query);
    let mut scored: Vec<(String, f64)> = inside
        .iter()
        .map(|id| {
            let names = ancestor_names(nodes, id).join(" ");
            let description = node_index(nodes)
                .get(id.as_str())
                .map(|node| node.description.clone())
                .unwrap_or_default();
            let text = format!("{names} {description}");
            (id.clone(), overlap_score(&query_tokens, query, &text))
        })
        .collect();
    scored.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for (id, _) in scored.into_iter().take(k.max(1)) {
        let parent = node_index(nodes)
            .get(id.as_str())
            .and_then(|node| node.parent_id.clone());
        if seen.insert(id.clone()) {
            out.push(id);
        }
        if let Some(parent) = parent
            && inside.iter().any(|id| id == &parent)
            && seen.insert(parent.clone())
        {
            out.push(parent);
        }
    }
    out
}

fn plain_chat_payload(payload: &mut Value) {
    if let Some(map) = payload.as_object_mut() {
        map.remove("reasoning_effort");
        map.insert("temperature".into(), json!(0));
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

/// How far the best child must stand out from its siblings before the heuristic descends.
///
/// Any value above zero is enough to catch a query that matches nothing, which is the
/// case that matters most. Swept over 150 sampled seed questions: 0.05/0.10 -> 105 exact
/// leaves, 0.20 -> 104, 0.35 -> 98 (too eager to stop at broad root descriptions).
/// 0.20 sits in the flat part of that curve with the most headroom against a single
/// accidental bigram, which is worth 0.35 on its own.
#[cfg(test)]
const TERMINAL_MARGIN: f64 = 0.20;

/// Score the terminal option (`__stop__` / `__none__`) without reading its wording.
///
/// It carries a fixed English sentence, so a lexical score would tie the outcome to the
/// query's alphabet. An absolute threshold fails too: overlap magnitude depends on how
/// wordy a taxonomy happens to be. What travels across seeds and languages is whether one
/// child *stands out* from its siblings. If they all look equally (un)related, staying put
/// is the honest answer.
#[cfg(test)]
fn terminal_bar(children: &BTreeMap<String, f64>) -> f64 {
    if children.len() < 2 {
        let only = children.values().copied().next().unwrap_or(0.0);
        // No sibling to compare. Stay put unless this child has some evidence.
        // A tie used to fall through to the child, so a single root could never abstain.
        return if only > 0.0 { 0.0 } else { 1.0 };
    }
    let mut values: Vec<f64> = children.values().copied().collect();
    values.sort_by(|a, b| a.total_cmp(b));
    values.pop();
    values.iter().sum::<f64>() / values.len() as f64 + TERMINAL_MARGIN
}

pub fn tokens(text: &str) -> BTreeSet<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| t.chars().count() > 1)
        .map(str::to_string)
        .collect()
}

pub fn overlap_score(query_tokens: &BTreeSet<String>, query: &str, contents: &str) -> f64 {
    let content_tokens = tokens(contents);
    let mut score = query_tokens.intersection(&content_tokens).count() as f64;
    let compact_query: String = query
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase();
    let compact_contents: String = contents
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase();
    let qchars: Vec<char> = compact_query.chars().collect();
    let mut seen = BTreeSet::new();
    for n in 2..=4 {
        if qchars.len() < n {
            continue;
        }
        for i in 0..=qchars.len() - n {
            let gram: String = qchars[i..i + n].iter().collect();
            if compact_contents.contains(&gram) && seen.insert(gram) {
                score += 0.35;
            }
        }
    }
    score
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

    #[test]
    fn terminal_choice_is_language_neutral() {
        // The same "nothing fits" query must behave the same in either alphabet: the
        // terminal option is scored against sibling spread, never on its English wording.
        let nodes = Arc::new(RwLock::new(Vec::new()));
        let client = JevClient::with_config(JevConfig::default(), nodes).unwrap();
        let mut questions = BTreeMap::new();
        questions.insert(
            "q".to_string(),
            Question {
                kind: "choice".into(),
                instructions: String::new(),
                criteria: Some(json!({
                    "orders": "주문: 배송과 주문 상태",
                    "account": "계정: 로그인과 비밀번호",
                    "__none__": "No root topic fits; choose no matching topic.",
                })),
            },
        );
        for query in ["quantum chromodynamics lattice", "안드로메다 은하까지 거리"] {
            let judgment = client.heuristic(&json!({ "current_request": query }), &questions);
            match judgment.answers.get("q") {
                Some(Answer::Choice { choice, .. }) => {
                    assert_eq!(choice, "__none__", "{query}")
                }
                other => panic!("unexpected answer: {other:?}"),
            }
        }
    }

    #[test]
    fn single_child_with_no_overlap_abstains() {
        let nodes = Arc::new(RwLock::new(Vec::new()));
        let client = JevClient::with_config(JevConfig::default(), nodes).unwrap();
        let mut questions = BTreeMap::new();
        questions.insert(
            "q".to_string(),
            Question {
                kind: "choice".into(),
                instructions: String::new(),
                criteria: Some(json!({
                    "products": "상품: 재고와 품질",
                    "__none__": "No root topic fits; choose no matching topic.",
                })),
            },
        );
        let miss = client.heuristic(
            &json!({ "current_request": "안드로메다 은하까지의 거리" }),
            &questions,
        );
        match miss.answers.get("q") {
            Some(Answer::Choice { choice, .. }) => assert_eq!(choice, "__none__"),
            other => panic!("unexpected answer: {other:?}"),
        }
        let hit = client.heuristic(&json!({ "current_request": "재고 없는 상품" }), &questions);
        match hit.answers.get("q") {
            Some(Answer::Choice { choice, .. }) => assert_eq!(choice, "products"),
            other => panic!("unexpected answer: {other:?}"),
        }
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
    fn shortlist_keeps_the_parent_of_a_specific_match() {
        let nodes = vec![
            Node {
                id: "orders".into(),
                name: "주문".into(),
                description: "주문과 배송".into(),
                examples: vec![],
                parent_id: None,
            },
            Node {
                id: "orders_tracking".into(),
                name: "배송 추적".into(),
                description: "운송장과 통관 지연".into(),
                examples: vec![],
                parent_id: Some("orders".into()),
            },
            Node {
                id: "account".into(),
                name: "계정".into(),
                description: "로그인과 비밀번호".into(),
                examples: vec![],
                parent_id: None,
            },
        ];
        let short = llm_shortlist("통관에서 운송장이 안 보여요", &nodes, "orders", 5);
        assert_eq!(short.first().map(String::as_str), Some("orders_tracking"));
        assert!(short.iter().any(|id| id == "orders"));
        assert!(short.iter().all(|id| id != "account"));
    }
}
