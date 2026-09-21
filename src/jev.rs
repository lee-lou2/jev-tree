use crate::models::{Answer, Judgment, Node, Question};
use reqwest::{header, Client};
use serde_json::{json, Value};
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
}

/// Resolved Jev connection: key/base/model may come from DB settings.
#[derive(Clone, Debug, Default)]
pub struct JevConfig {
    pub key: Option<String>,
    pub base_url: String,
    pub model: String,
}

impl JevConfig {
    /// Layer DB settings on top of defaults. The DB is the only source;
    /// env only seeds empty slots at boot (see `seed_settings_from_env`).
    pub fn with_db(&mut self, settings: &crate::models::AppSettings) {
        if !settings.jev_api_key.trim().is_empty() {
            self.key = Some(settings.jev_api_key.clone());
        }
        // llm.base_url / llm.token / llm.model are for model listing only.
        // Search always posts to TypeSafe System One.
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
    nodes: Arc<RwLock<Vec<Node>>>,
    /// Test-only preferred Choice ids. Empty in production.
    script: Arc<Mutex<Vec<String>>>,
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
            .user_agent("jev-tree/0.2")
            .build()?;
        Ok(Self {
            client,
            config: std::sync::Arc::new(std::sync::RwLock::new(config)),
            nodes,
            script: Arc::new(Mutex::new(Vec::new())),
        })
    }

    pub fn script_choices(&self, ids: Vec<String>) {
        if let Ok(mut guard) = self.script.lock() {
            *guard = ids;
        }
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

    pub fn label(&self) -> &'static str {
        if self.snapshot().0.is_some() {
            "jev"
        } else {
            "heuristic"
        }
    }

    /// Probe `{base}/models` (OpenAI) or `{base}/v1/models` (TypeSafe) with the
    /// given credentials. Does not fall back to the Jev key.
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
        if let Ok(script) = self.script.lock() {
            if !script.is_empty() {
                return Ok(scripted(&questions, &script));
            }
        }
        let (key, base_url, model) = self.snapshot();
        let Some(key) = key else {
            return Ok(self.heuristic(&state, &questions));
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
                        if let Ok(nodes) = self.nodes.read() {
                            if let Some(node) = nodes.iter().find(|n| n.id == *cid) {
                                contents.push_str(&format!(" {} {}", node.name, node.description));
                            }
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

/// How far the best child must stand out from its siblings before the heuristic descends.
///
/// Any value above zero is enough to catch a query that matches nothing, which is the
/// case that matters most. Swept over 150 sampled seed questions: 0.05/0.10 -> 105 exact
/// leaves, 0.20 -> 104, 0.35 -> 98 (too eager to stop at broad root descriptions).
/// 0.20 sits in the flat part of that curve with the most headroom against a single
/// accidental bigram, which is worth 0.35 on its own.
const TERMINAL_MARGIN: f64 = 0.20;

/// Score the terminal option (`__stop__` / `__none__`) without reading its wording.
///
/// It carries a fixed English sentence, so a lexical score would tie the outcome to the
/// query's alphabet. An absolute threshold fails too: overlap magnitude depends on how
/// wordy a taxonomy happens to be. What travels across seeds and languages is whether one
/// child *stands out* from its siblings. If they all look equally (un)related, staying put
/// is the honest answer.
fn terminal_bar(children: &BTreeMap<String, f64>) -> f64 {
    if children.len() < 2 {
        // Nothing to choose between. Descend unless the child shows no evidence at all;
        // on an exact tie the terminal sorts first and wins.
        return 0.0;
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
}
