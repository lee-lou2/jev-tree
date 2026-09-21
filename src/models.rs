use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub examples: Vec<String>,
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub category_id: String,
    pub question: String,
    pub answer: String,
    pub kind: String,
    pub status: String,
    #[serde(default = "one")]
    pub version: i64,
}
fn one() -> i64 {
    1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankedItem {
    pub item: Item,
    pub relevance: f64,
    pub directness: f64,
    pub confidence: Option<f64>,
    pub role: String,
    pub score: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathPart {
    pub id: String,
    pub name: String,
    pub probability: f64,
    pub confidence: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub id: Option<String>,
    pub name: String,
    pub probability: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceStep {
    pub depth: usize,
    pub node_id: Option<String>,
    pub node_name: String,
    pub path: Vec<PathPart>,
    pub candidates: Vec<Candidate>,
    pub choice_id: Option<String>,
    pub choice_name: String,
    pub confidence: Option<f64>,
    pub reason: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalBeam {
    pub node_id: Option<String>,
    pub path: Vec<PathPart>,
    pub score: f64,
    pub alive: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trace {
    pub run_id: String,
    pub mode: String,
    pub steps: Vec<TraceStep>,
    pub leaf_id: Option<String>,
    pub score: f64,
    pub final_beams: Vec<FinalBeam>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub query: String,
    pub items: Vec<RankedItem>,
    pub leaf_id: Option<String>,
    pub path: Vec<PathPart>,
    pub trace: Trace,
    pub abstained: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestResult {
    pub question: String,
    pub answer: String,
    pub kind: String,
    pub category_id: Option<String>,
    pub path: Vec<PathPart>,
    pub duplicate_ids: Vec<String>,
    pub confidence: Option<f64>,
    pub trace: Trace,
    /// Stored row, draft or published. Use it to publish or discard a draft.
    pub item_id: Option<String>,
    pub version: i64,
    /// Set only once the row is `active`.
    pub published_id: Option<String>,
    pub updated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunRequest {
    #[serde(default = "default_mode")]
    pub mode: String,
    #[serde(default)]
    pub query: String,
    #[serde(default)]
    pub question: String,
    #[serde(default)]
    pub answer: String,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub context: Vec<Value>,
    pub start_node: Option<String>,
    /// Slash-separated node ids from the global forest, e.g. `products` or
    /// `products/products_stock`. Scopes descent to that subtree. Settings
    /// and keys ignore this; they stay project-wide.
    pub root: Option<String>,
    #[serde(default = "default_beam")]
    pub beam_width: usize,
    #[serde(default = "default_limit")]
    pub limit: usize,
    #[serde(default)]
    pub auto_publish: bool,
    pub item_id: Option<String>,
    pub expected_version: Option<i64>,
}
fn default_mode() -> String {
    "search".into()
}
fn default_beam() -> usize {
    3
}
fn default_limit() -> usize {
    8
}
impl Default for RunRequest {
    fn default() -> Self {
        Self {
            mode: default_mode(),
            query: String::new(),
            question: String::new(),
            answer: String::new(),
            source: String::new(),
            context: Vec::new(),
            start_node: None,
            root: None,
            beam_width: default_beam(),
            limit: default_limit(),
            auto_publish: false,
            item_id: None,
            expected_version: None,
        }
    }
}
impl RunRequest {
    pub fn text(&self) -> String {
        if self.mode == "ingest" {
            let question = self.question.trim();
            let answer = self.answer.trim();
            if !question.is_empty() {
                return if answer.is_empty() {
                    question.to_string()
                } else {
                    format!("Q: {question}\nA: {answer}")
                };
            }
        }
        let query = self.query.trim();
        if query.is_empty() {
            self.source.trim().to_string()
        } else {
            query.to_string()
        }
    }
    pub fn validate(&self) -> Result<(), String> {
        if !["search", "ingest"].contains(&self.mode.as_str()) {
            return Err("mode must be search or ingest".into());
        }
        let text = self.text();
        if text.is_empty() || text.chars().count() > 8000 {
            return Err("query must contain 1..8000 characters".into());
        }
        if !(1..=5).contains(&self.beam_width) || !(1..=20).contains(&self.limit) {
            return Err("beam_width/limit out of range".into());
        }
        if self.context.len() > 32 {
            return Err("at most 32 context turns are allowed".into());
        }
        if self.item_id.is_some() != self.expected_version.is_some() {
            return Err("item_id and expected_version must be supplied together".into());
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Server settings + sessions (stored in SQLite, changed via UI, never env)
// ---------------------------------------------------------------------------

/// All server-managed settings. Secrets are stored encrypted (AES-256-GCM) so a
/// DB dump alone is not enough to use them.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppSettings {
    /// Project name shown in the header/brand area.
    #[serde(default)]
    pub site_name: String,
    /// Optional subtitle shown under the brand.
    #[serde(default)]
    pub site_description: String,
    /// Optional logo image (data URL, PNG/JPEG/SVG/WebP, <= 500 KB).
    #[serde(default)]
    pub site_logo: String,
    /// Jev API key (TypeSafe System One). Required before any run.
    #[serde(default)]
    pub jev_api_key: String,
    /// Optional override of the TypeSafe base URL.
    #[serde(default)]
    pub llm_base_url: String,
    /// Optional override token used for the model list probe.
    #[serde(default)]
    pub llm_token: String,
    /// Optional pinned model id. Empty = `jev-latest`.
    #[serde(default)]
    pub llm_model: String,
}

impl AppSettings {
    /// Keys that are secrets: never returned in full to the UI.
    /// `has_server_key` is filled in by the caller (separate settings key).
    pub fn masked(&self, has_server_key: bool) -> serde_json::Value {
        serde_json::json!({
            "site_name": self.site_name,
            "site_description": self.site_description,
            "site_logo": self.site_logo,
            "has_server_key": has_server_key,
            "jev_api_key_set": !self.jev_api_key.is_empty(),
            "jev_api_key_hint": mask_secret(&self.jev_api_key),
            "llm_base_url": self.llm_base_url,
            "llm_token_set": !self.llm_token.is_empty(),
            "llm_token_hint": mask_secret(&self.llm_token),
            "llm_model": self.llm_model,
        })
    }
}

/// Hint for a stored secret: `****` plus the last four characters.
/// Counts characters, never bytes — a non-ASCII secret must not panic here.
fn mask_secret(value: &str) -> String {
    let chars: Vec<char> = value.chars().collect();
    if chars.is_empty() {
        return String::new();
    }
    if chars.len() <= 8 {
        return "****".into();
    }
    let tail: String = chars[chars.len() - 4..].iter().collect();
    format!("****{tail}")
}

/// Request body for PATCH /api/settings (all fields optional).
#[derive(Debug, Deserialize, Default)]
pub struct SettingsPatch {
    pub site_name: Option<String>,
    pub site_description: Option<String>,
    /// `null` clears the logo; omitted leaves it unchanged.
    pub site_logo: Option<Option<String>>,
    pub jev_api_key: Option<String>,
    pub llm_base_url: Option<String>,
    pub llm_token: Option<String>,
    pub llm_model: Option<String>,
    /// Rotate the login password. Empty string clears it (open mode).
    pub server_key: Option<String>,
}

/// One model entry from `GET /v1/models`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub release_date: String,
}

/// First-run / login payload.
#[derive(Debug, Deserialize)]
pub struct SetupRequest {
    /// Create the login password. Omitted = generate one server-side.
    pub server_key: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    #[serde(default)]
    pub server_key: String,
}

/// Model list probe with explicit credentials (unsaved preview).
#[derive(Debug, Deserialize)]
pub struct ModelsProbe {
    pub base_url: Option<String>,
    pub token: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Question {
    #[serde(rename = "type")]
    pub kind: String,
    pub instructions: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criteria: Option<Value>,
}
#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Answer {
    Choice {
        choice: String,
        probabilities: BTreeMap<String, f64>,
        confidence: f64,
    },
    Noul {
        noul: f64,
    },
    Score {
        score: f64,
        #[serde(default)]
        confidence: f64,
    },
}
#[derive(Debug, Default, Deserialize, Clone)]
pub struct Usage {
    #[serde(default, rename = "input_tokens")]
    pub _input_tokens: u64,
    #[serde(default, rename = "output_tokens")]
    pub _output_tokens: u64,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Judgment {
    pub answers: BTreeMap<String, Answer>,
    #[serde(default, rename = "model")]
    pub _model: String,
    #[serde(default, rename = "usage")]
    pub _usage: Usage,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_secret_is_char_safe() {
        assert_eq!(mask_secret(""), "");
        assert_eq!(mask_secret("short"), "****");
        assert_eq!(mask_secret("sk-abcdefghijklmn"), "****klmn");
        // Multi-byte secrets must not panic on a byte boundary.
        assert_eq!(mask_secret("한글로된아주긴비밀키입니다"), "****키입니다");
    }

    #[test]
    fn masked_settings_never_leak_plaintext() {
        let settings = AppSettings {
            jev_api_key: "아주긴비밀키값입니다".into(),
            llm_token: "sk-0123456789".into(),
            ..Default::default()
        };
        let masked = settings.masked(true);
        let rendered = masked.to_string();
        assert!(!rendered.contains("아주긴비밀키"));
        assert!(!rendered.contains("sk-0123456789"));
        assert_eq!(masked["jev_api_key_set"], true);
        assert_eq!(masked["llm_token_hint"], "****6789");
    }
}
