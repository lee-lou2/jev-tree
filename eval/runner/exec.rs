//! Runs golden cases in-process through `jev_tree::engine::execute`, the same code
//! path `POST /api/run` uses (request validation and the run deadline included).
//! Every search shares one database; every ingest scenario gets a fresh copy.

use crate::data::{Case, Kb, Resolved, Variant};
use crate::score::{self, Score};
use jev_tree::AppState;
use jev_tree::jev::{JevClient, JevConfig};
use jev_tree::models::RunRequest;
use jev_tree::store::Store;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tokio::sync::{Semaphore, mpsc};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Router {
    Jev,
    JevLlm,
}

impl Router {
    pub fn name(self) -> &'static str {
        match self {
            Router::Jev => "jev",
            Router::JevLlm => "jev_llm",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mock {
    Off,
    /// Scripted evaluator that walks straight to the labelled node. Tests the harness.
    Oracle,
    /// Scripted evaluator that always says "nothing fits".
    Abstain,
}

impl Mock {
    pub fn name(self) -> &'static str {
        match self {
            Mock::Off => "off",
            Mock::Oracle => "oracle",
            Mock::Abstain => "abstain",
        }
    }
}

/// Model credentials. Never written to results.
#[derive(Clone, Default)]
pub struct Credentials {
    pub jev_key: String,
    pub jev_base: String,
    pub jev_model: String,
    pub llm_base: String,
    pub llm_token: String,
    pub llm_model: String,
    pub source: String,
}

fn env_first(names: &[&str]) -> String {
    names
        .iter()
        .filter_map(|name| std::env::var(name).ok())
        .map(|value| value.trim().to_string())
        .find(|value| !value.is_empty())
        .unwrap_or_default()
}

impl Credentials {
    pub fn from_env() -> Credentials {
        Credentials {
            jev_key: env_first(&[
                "JEV_EVAL_JEV_KEY",
                "JEV_TREE_INIT_API_KEY",
                "TYPESAFE_API_KEY",
            ]),
            jev_base: env_first(&["JEV_EVAL_JEV_BASE_URL"]),
            jev_model: env_first(&["JEV_EVAL_JEV_MODEL"]),
            llm_base: env_first(&["JEV_EVAL_LLM_BASE_URL", "JEV_TREE_INIT_BASE_URL"])
                .trim_end_matches('/')
                .to_string(),
            llm_token: env_first(&["JEV_EVAL_LLM_TOKEN", "JEV_TREE_INIT_TOKEN"]),
            llm_model: env_first(&["JEV_EVAL_LLM_MODEL", "JEV_TREE_INIT_MODEL"]),
            source: "env".into(),
        }
    }

    /// Read the Jev key and LLM settings a local server saved through the UI.
    /// Opens the database read-only and needs the `.jev-tree.key`/`.jev-tree.salt`
    /// files beside it (or `JEV_TREE_SECRET_KEY*`). Call before any thread starts:
    /// decryption reads `JEV_TREE_DB`.
    pub fn from_db(path: &Path) -> Result<Credentials, String> {
        let dir = path.parent().unwrap_or(Path::new("."));
        let has_material = std::env::var("JEV_TREE_SECRET_KEY_FILE").is_ok()
            || std::env::var("JEV_TREE_SECRET_KEY").is_ok()
            || dir.join(".jev-tree.key").exists();
        if !path.exists() || !has_material || !dir.join(".jev-tree.salt").exists() {
            return Err(format!(
                "{}: database or its .jev-tree.key/.jev-tree.salt not found",
                path.display()
            ));
        }
        let connection = rusqlite::Connection::open_with_flags(
            path,
            rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .map_err(|e| format!("{}: {e}", path.display()))?;
        let get = |key: &str| -> String {
            connection
                .query_row("SELECT value FROM app_settings WHERE key=?", [key], |row| {
                    row.get::<_, String>(0)
                })
                .unwrap_or_default()
        };
        // SAFETY: called from `main` before the runtime and any other thread start.
        unsafe {
            std::env::set_var("JEV_TREE_DB", path);
        }
        let secret = |key: &str| -> Result<String, String> {
            let raw = get(key);
            if raw.is_empty() {
                Ok(String::new())
            } else {
                jev_tree::secret::decrypt(&raw).map_err(|e| format!("{key}: {e}"))
            }
        };
        let mut creds = Credentials::from_env();
        let jev_key = secret("jev.api_key")?;
        if !jev_key.is_empty() {
            creds.jev_key = jev_key;
        }
        let base = get("llm.base_url");
        let token = secret("llm.token")?;
        let model = get("llm.model");
        if !base.is_empty() && !token.is_empty() && !model.is_empty() {
            creds.llm_base = base.trim_end_matches('/').to_string();
            creds.llm_token = token;
            creds.llm_model = model;
        }
        creds.source = format!("db:{}", path.display());
        Ok(creds)
    }

    pub fn llm_ready(&self) -> bool {
        !self.llm_base.is_empty() && !self.llm_token.is_empty() && !self.llm_model.is_empty()
    }

    pub fn jev_config(&self, router: Router) -> JevConfig {
        let llm = router == Router::JevLlm;
        JevConfig {
            key: Some(self.jev_key.clone()).filter(|key| !key.is_empty()),
            base_url: self.jev_base.clone(),
            model: self.jev_model.clone(),
            llm_base_url: if llm {
                self.llm_base.clone()
            } else {
                String::new()
            },
            llm_token: if llm {
                Some(self.llm_token.clone()).filter(|token| !token.is_empty())
            } else {
                None
            },
            llm_model: if llm {
                self.llm_model.clone()
            } else {
                String::new()
            },
        }
    }

    /// Model names for the run record. No secrets.
    pub fn describe(&self, router: Router) -> Value {
        let (_, base, model) = self.jev_config(router).normalized();
        json!({
            "jev_model": model,
            "jev_base_url": base,
            "llm_model": if router == Router::JevLlm { Some(self.llm_model.clone()) } else { None },
            "llm_base_url": if router == Router::JevLlm { Some(self.llm_base.clone()) } else { None },
            "credentials": self.source,
        })
    }
}

pub struct Settings {
    pub router: Router,
    pub mock: Mock,
    pub creds: Credentials,
    pub concurrency: usize,
    pub timeout: Duration,
    pub beam_width: Option<usize>,
    pub limit: Option<usize>,
    pub verify_draft_search: bool,
    pub repeat: usize,
}

// ---------------------------------------------------------------------------
// Records
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Failure {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ranked {
    pub id: String,
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    pub score: f64,
    pub relevance: f64,
    pub directness: f64,
    pub role: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchOut {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Failure>,
    pub leaf: Option<String>,
    pub path: Vec<String>,
    pub router: String,
    pub abstained: bool,
    pub items: Vec<Ranked>,
    pub trace_score: Option<f64>,
    /// Jev beam rounds (one evaluator call each).
    pub beam_rounds: usize,
    /// LLM routing calls.
    pub llm_calls: usize,
    /// Active items ranked in the landed subtree.
    pub pool: Option<usize>,
    pub ms: u64,
    pub route_ms: Option<u64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IngestStep {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Failure>,
    pub category: Option<String>,
    pub path: Vec<String>,
    pub router: String,
    pub item_id: Option<String>,
    pub version: i64,
    pub published_id: Option<String>,
    pub updated: bool,
    pub duplicate_ids: Vec<String>,
    pub beam_rounds: usize,
    pub llm_calls: usize,
    pub ms: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IngestOut {
    pub draft: IngestStep,
    pub rows_before: i64,
    pub rows_after_draft: i64,
    pub draft_status: Option<String>,
    pub draft_active: Option<bool>,
    pub draft_search_leak: Option<bool>,
    pub publish: Option<IngestStep>,
    pub probe: Option<SearchOut>,
    pub ms: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContractOut {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Failure>,
    pub ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub case_id: String,
    pub kind: String,
    pub router: String,
    pub variant: String,
    pub repeat: usize,
    pub facets: BTreeMap<String, String>,
    pub text: String,
    pub expected: Resolved,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<SearchOut>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingest: Option<IngestOut>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract: Option<ContractOut>,
    pub score: Score,
}

pub fn facets(kb: &Kb, case: &Case, resolved: &Resolved) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    let expect = match case.kind.as_str() {
        "contract" => "error",
        "ingest" if resolved.node.is_none() => "reject",
        "ingest" => "file",
        _ => match &resolved.node {
            None => "oos",
            Some(_) if resolved.dropped => "dropped",
            Some(id) if kb.manifest.empty_leaves.contains(id) => "empty_leaf",
            Some(_) if !resolved.answerable => "no_answer",
            Some(_) if resolved.level == "internal" => "internal",
            Some(_) => "leaf",
        },
    };
    map.insert("expect".into(), expect.into());
    map.insert("domain".into(), resolved.domain.clone());
    map.insert("turns".into(), case.turns().into());
    map.insert("entry".into(), case.entry().into());
    map.insert("level".into(), resolved.level.clone());
    map.insert(
        "depth".into(),
        if resolved.depth == 0 {
            "-".into()
        } else {
            format!("d{}", resolved.depth)
        },
    );
    map.insert(
        "pool".into(),
        crate::data::pool_bucket(resolved.pool).to_string(),
    );
    map.insert("split".into(), case.split());
    let tag = |value: &Option<String>| value.clone().unwrap_or_else(|| "-".into());
    map.insert("style".into(), tag(&case.tags.style));
    map.insert("ctx".into(), tag(&case.tags.ctx));
    map.insert("diff".into(), tag(&case.tags.diff));
    map
}

// ---------------------------------------------------------------------------
// Engine calls
// ---------------------------------------------------------------------------

struct Events {
    beam_rounds: usize,
    llm_calls: usize,
    pool: Option<usize>,
    descent_done: Option<Instant>,
}

async fn collect(mut rx: mpsc::Receiver<Value>) -> Events {
    let mut events = Events {
        beam_rounds: 0,
        llm_calls: 0,
        pool: None,
        descent_done: None,
    };
    while let Some(value) = rx.recv().await {
        match value["type"].as_str() {
            Some("descent_step") if value["status"] == "asking" => {
                // Beam rounds carry a run id; LLM routing notes do not.
                if value.get("run_id").is_some() {
                    events.beam_rounds += 1;
                } else {
                    events.llm_calls += 1;
                }
            }
            Some("retrieval_completed") => {
                events.pool = value["count"].as_u64().map(|n| n as usize);
            }
            Some("descent_done") => events.descent_done = Some(Instant::now()),
            _ => {}
        }
    }
    events
}

fn failure(error: &jev_tree::error::Error) -> Failure {
    Failure {
        code: error.code().into(),
        message: format!("{} ({error})", error.public_message()),
    }
}

/// One `execute` call with the HTTP layer's validation and deadline.
async fn call(
    state: &AppState,
    request: RunRequest,
    timeout: Duration,
) -> (Result<Value, Failure>, Events, u64, Option<u64>) {
    let started = Instant::now();
    if let Err(message) = request.validate() {
        let events = Events {
            beam_rounds: 0,
            llm_calls: 0,
            pool: None,
            descent_done: None,
        };
        return (
            Err(Failure {
                code: "invalid_request".into(),
                message,
            }),
            events,
            0,
            None,
        );
    }
    let (tx, rx) = mpsc::channel(4096);
    let collector = tokio::spawn(collect(rx));
    let outcome = tokio::time::timeout(
        timeout,
        jev_tree::engine::execute(state.clone(), request, Some(tx)),
    )
    .await;
    let events = collector.await.unwrap_or(Events {
        beam_rounds: 0,
        llm_calls: 0,
        pool: None,
        descent_done: None,
    });
    let ms = started.elapsed().as_millis() as u64;
    let route_ms = events
        .descent_done
        .map(|done| done.duration_since(started).as_millis() as u64);
    let result = match outcome {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(error)) => Err(failure(&error)),
        Err(_) => Err(Failure {
            code: "timeout".into(),
            message: format!("run passed {}s", timeout.as_secs()),
        }),
    };
    (result, events, ms, route_ms)
}

fn path_ids(value: &Value) -> Vec<String> {
    value
        .as_array()
        .map(|parts| {
            parts
                .iter()
                .filter_map(|part| part["id"].as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

async fn search(
    state: &AppState,
    variant: &Variant,
    request: RunRequest,
    timeout: Duration,
) -> SearchOut {
    let (result, events, ms, route_ms) = call(state, request, timeout).await;
    let mut out = SearchOut {
        beam_rounds: events.beam_rounds,
        llm_calls: events.llm_calls,
        pool: events.pool,
        ms,
        route_ms,
        ..Default::default()
    };
    match result {
        Err(error) => out.error = Some(error),
        Ok(value) => {
            out.leaf = value["leaf_id"].as_str().map(str::to_string);
            out.path = path_ids(&value["path"]);
            out.router = value["trace"]["router"].as_str().unwrap_or("").to_string();
            out.abstained = value["abstained"].as_bool().unwrap_or(false);
            out.trace_score = value["trace"]["score"].as_f64();
            out.items = value["items"]
                .as_array()
                .map(|rows| {
                    rows.iter()
                        .map(|row| {
                            let id = row["item"]["id"].as_str().unwrap_or("").to_string();
                            Ranked {
                                key: variant.id_to_ref.get(&id).cloned(),
                                id,
                                score: row["score"].as_f64().unwrap_or(0.0),
                                relevance: row["relevance"].as_f64().unwrap_or(0.0),
                                directness: row["directness"].as_f64().unwrap_or(0.0),
                                role: row["role"].as_str().unwrap_or("").to_string(),
                            }
                        })
                        .collect()
                })
                .unwrap_or_default();
        }
    }
    out
}

async fn ingest_step(state: &AppState, request: RunRequest, timeout: Duration) -> IngestStep {
    let (result, events, ms, _) = call(state, request, timeout).await;
    let mut out = IngestStep {
        beam_rounds: events.beam_rounds,
        llm_calls: events.llm_calls,
        ms,
        ..Default::default()
    };
    match result {
        Err(error) => out.error = Some(error),
        Ok(value) => {
            out.category = value["category_id"].as_str().map(str::to_string);
            out.path = path_ids(&value["path"]);
            out.router = value["trace"]["router"].as_str().unwrap_or("").to_string();
            out.item_id = value["item_id"].as_str().map(str::to_string);
            out.version = value["version"].as_i64().unwrap_or(0);
            out.published_id = value["published_id"].as_str().map(str::to_string);
            out.updated = value["updated"].as_bool().unwrap_or(false);
            out.duplicate_ids = value["duplicate_ids"]
                .as_array()
                .map(|ids| {
                    ids.iter()
                        .filter_map(|id| id.as_str().map(str::to_string))
                        .collect()
                })
                .unwrap_or_default();
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Evaluator clients
// ---------------------------------------------------------------------------

fn oracle_script(variant: &Variant, node: Option<&str>) -> Vec<String> {
    match node {
        None => vec!["__none__".into()],
        Some(id) => {
            let mut script = variant.chain(id);
            if !variant.is_leaf(id) {
                script.push("__stop__".into());
            }
            script
        }
    }
}

struct Engine {
    store: Store,
    nodes: Arc<RwLock<Vec<jev_tree::models::Node>>>,
    shared: JevClient,
}

impl Engine {
    fn open(db: &Path, seed: &Path, settings: &Settings) -> Result<Engine, String> {
        let store = Store::open(&db.to_string_lossy(), &seed.to_string_lossy())
            .map_err(|e| format!("open {}: {e}", db.display()))?;
        let nodes = Arc::new(RwLock::new(
            store.load_taxonomy().map_err(|e| e.to_string())?,
        ));
        let shared = JevClient::with_config(settings.creds.jev_config(settings.router))
            .map_err(|e| e.to_string())?;
        Ok(Engine {
            store,
            nodes,
            shared,
        })
    }

    /// Real runs share one client (connection pool). Mock runs script a fresh one.
    fn state(
        &self,
        settings: &Settings,
        variant: &Variant,
        node: Option<&str>,
        kind: &str,
    ) -> Result<AppState, String> {
        let jev = match settings.mock {
            Mock::Off => self.shared.clone(),
            mock => {
                let client =
                    JevClient::with_config(JevConfig::default()).map_err(|e| e.to_string())?;
                if kind == "contract" {
                    // Pass the key gate; contract errors happen before any evaluator call.
                    client.script_choices(vec!["__none__".into()]);
                } else if mock == Mock::Abstain {
                    client.script_choices(vec!["__none__".into(), "__stop__".into()]);
                } else if settings.router == Router::JevLlm {
                    client.script_llm(Some(node.unwrap_or("__none__").to_string()));
                } else {
                    client.script_choices(oracle_script(variant, node));
                }
                client
            }
        };
        Ok(AppState {
            store: self.store.clone(),
            nodes: self.nodes.clone(),
            jev,
        })
    }
}

fn row_count(store: &Store) -> i64 {
    store
        .list_items(0, 1, None, None, false, "all")
        .map(|(_, total)| total)
        .unwrap_or(-1)
}

/// The seed loader numbers rows in file order; confirm before trusting ref -> id.
fn check_ids(store: &Store, variant: &Variant) -> Result<(), String> {
    let mut offset = 0;
    let mut seen = 0;
    loop {
        let (page, total) = store
            .list_items(offset, 100, None, None, false, "active")
            .map_err(|e| e.to_string())?;
        for item in &page {
            let expected = variant.id_to_ref.get(&item.id);
            let actual = variant
                .items
                .iter()
                .find(|candidate| candidate.question == item.question)
                .map(|candidate| &candidate.key);
            if expected.is_none() || expected != actual {
                return Err(format!(
                    "item id {} does not map to the expected ref; is the fixture seeded in order?",
                    item.id
                ));
            }
        }
        seen += page.len();
        offset += 100;
        if page.is_empty() || seen as i64 >= total {
            break;
        }
    }
    if seen != variant.items.len() {
        return Err(format!(
            "seeded {seen} items, variant has {}",
            variant.items.len()
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Case runners
// ---------------------------------------------------------------------------

fn base_request(case: &Case, resolved: &Resolved, settings: &Settings) -> RunRequest {
    RunRequest {
        context: case.input.context.clone(),
        root: resolved.root_path.clone(),
        start_node: resolved.start_node.clone(),
        beam_width: settings.beam_width.or(case.input.beam_width).unwrap_or(3),
        limit: settings.limit.or(case.input.limit).unwrap_or(8),
        ..Default::default()
    }
}

async fn run_search(
    engine: &Engine,
    variant: &Variant,
    case: &Case,
    resolved: &Resolved,
    settings: &Settings,
) -> Result<SearchOut, String> {
    let state = engine.state(settings, variant, resolved.node.as_deref(), "search")?;
    let request = RunRequest {
        mode: "search".into(),
        query: case.input.query.clone(),
        ..base_request(case, resolved, settings)
    };
    Ok(search(&state, variant, request, settings.timeout).await)
}

async fn run_contract(
    engine: &Engine,
    variant: &Variant,
    case: &Case,
    resolved: &Resolved,
    settings: &Settings,
) -> Result<ContractOut, String> {
    let state = engine.state(settings, variant, None, "contract")?;
    let started = Instant::now();
    let mode = if case.input.question.is_empty() {
        "search"
    } else {
        "ingest"
    };
    let mut body = json!({
        "mode": mode,
        "query": case.input.query,
        "question": case.input.question,
        "answer": case.input.answer,
        "context": case.input.context,
        "root": resolved.root_path,
        "start_node": resolved.start_node,
    });
    if let (Some(Value::Object(raw)), Some(target)) = (&case.input.raw, body.as_object_mut()) {
        for (key, value) in raw {
            target.insert(key.clone(), value.clone());
        }
    }
    let error = match serde_json::from_value::<RunRequest>(body) {
        Err(error) => Some(Failure {
            code: "invalid_request".into(),
            message: error.to_string(),
        }),
        Ok(request) => call(&state, request, settings.timeout).await.0.err(),
    };
    Ok(ContractOut {
        error,
        ms: started.elapsed().as_millis() as u64,
    })
}

async fn run_ingest(
    seed: &Path,
    db: &Path,
    variant: &Variant,
    case: &Case,
    resolved: &Resolved,
    settings: &Settings,
) -> Result<IngestOut, String> {
    let started = Instant::now();
    let engine = Engine::open(db, seed, settings)?;
    let state = engine.state(settings, variant, resolved.node.as_deref(), "ingest")?;
    let mut out = IngestOut {
        rows_before: row_count(&engine.store),
        ..Default::default()
    };
    let draft_request = RunRequest {
        mode: "ingest".into(),
        question: case.input.question.clone(),
        answer: case.input.answer.clone(),
        auto_publish: false,
        ..base_request(case, resolved, settings)
    };
    out.draft = ingest_step(&state, draft_request.clone(), settings.timeout).await;
    out.rows_after_draft = row_count(&engine.store);
    if let (Some(id), Some(category)) = (out.draft.item_id.clone(), out.draft.category.clone()) {
        let active = engine
            .store
            .active_in(std::slice::from_ref(&category))
            .map_err(|e| e.to_string())?;
        out.draft_active = Some(active.iter().any(|item| item.id == id));
        let (drafts, _) = engine
            .store
            .list_items(0, 100, None, Some(&category), false, "draft")
            .map_err(|e| e.to_string())?;
        out.draft_status = Some(if drafts.iter().any(|item| item.id == id) {
            "draft".into()
        } else if out.draft_active == Some(true) {
            "active".into()
        } else {
            "missing".into()
        });
        if settings.verify_draft_search
            && let Some(probe) = &case.probe
        {
            let request = RunRequest {
                mode: "search".into(),
                query: probe.query.clone(),
                context: probe.context.clone(),
                ..base_request(case, resolved, settings)
            };
            let before = search(&state, variant, request, settings.timeout).await;
            out.draft_search_leak = Some(before.items.iter().any(|item| item.id == id));
        }
        let publish_request = RunRequest {
            item_id: Some(id.clone()),
            expected_version: Some(out.draft.version),
            auto_publish: true,
            ..draft_request
        };
        out.publish = Some(ingest_step(&state, publish_request, settings.timeout).await);
        if let Some(probe) = &case.probe {
            let request = RunRequest {
                mode: "search".into(),
                query: probe.query.clone(),
                context: probe.context.clone(),
                ..base_request(case, resolved, settings)
            };
            out.probe = Some(search(&state, variant, request, settings.timeout).await);
        }
    }
    out.ms = started.elapsed().as_millis() as u64;
    Ok(out)
}

// ---------------------------------------------------------------------------
// One cell = one router x one variant
// ---------------------------------------------------------------------------

pub struct CellInput {
    pub kb: Arc<Kb>,
    pub variant: Arc<Variant>,
    pub settings: Arc<Settings>,
    pub cases: Vec<(Case, Resolved)>,
    pub label: String,
}

pub async fn run_cell(input: CellInput) -> Result<Vec<Record>, String> {
    let CellInput {
        kb,
        variant,
        settings,
        cases,
        label,
    } = input;
    let workdir = tempfile::Builder::new()
        .prefix("jev-eval-")
        .tempdir()
        .map_err(|e| e.to_string())?;
    let seed = workdir.path().join("seed.json");
    std::fs::write(
        &seed,
        serde_json::to_vec(&variant.seed_json()).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let engine = Arc::new(Engine::open(
        &workdir.path().join("search.db"),
        &seed,
        &settings,
    )?);
    check_ids(&engine.store, &variant)?;
    let seed: Arc<PathBuf> = Arc::new(seed);
    let dir: Arc<PathBuf> = Arc::new(workdir.path().to_path_buf());
    let total = cases.len() * settings.repeat.max(1);
    let done = Arc::new(AtomicUsize::new(0));
    let gate = Arc::new(Semaphore::new(settings.concurrency.max(1)));
    let mut tasks = tokio::task::JoinSet::new();
    for repeat in 0..settings.repeat.max(1) {
        for (case, resolved) in &cases {
            let (case, resolved) = (case.clone(), resolved.clone());
            let (kb, variant, settings, engine) = (
                kb.clone(),
                variant.clone(),
                settings.clone(),
                engine.clone(),
            );
            let (seed, dir, done, gate, label) = (
                seed.clone(),
                dir.clone(),
                done.clone(),
                gate.clone(),
                label.clone(),
            );
            tasks.spawn(async move {
                let _permit = gate.acquire_owned().await.map_err(|e| e.to_string())?;
                let mut record = Record {
                    case_id: case.id.clone(),
                    kind: case.kind.clone(),
                    router: settings.router.name().into(),
                    variant: variant.name.clone(),
                    repeat,
                    facets: facets(&kb, &case, &resolved),
                    text: if case.kind == "ingest" {
                        case.input.question.clone()
                    } else {
                        case.input.query.clone()
                    },
                    expected: resolved.clone(),
                    search: None,
                    ingest: None,
                    contract: None,
                    score: Score::default(),
                };
                match case.kind.as_str() {
                    "search" => {
                        record.search =
                            Some(run_search(&engine, &variant, &case, &resolved, &settings).await?)
                    }
                    "ingest" => {
                        let db = dir.join(format!("ingest-{}-{repeat}.db", case.id));
                        record.ingest = Some(
                            run_ingest(&seed, &db, &variant, &case, &resolved, &settings).await?,
                        );
                    }
                    _ => {
                        record.contract = Some(
                            run_contract(&engine, &variant, &case, &resolved, &settings).await?,
                        )
                    }
                }
                record.score = score::score(&kb, &variant, &case, &record);
                let finished = done.fetch_add(1, Ordering::SeqCst) + 1;
                eprintln!(
                    "[{label}] {finished}/{total} {} {}",
                    case.id,
                    score::one_line(&record)
                );
                Ok::<Record, String>(record)
            });
        }
    }
    let mut records = Vec::new();
    while let Some(joined) = tasks.join_next().await {
        match joined {
            Ok(Ok(record)) => records.push(record),
            Ok(Err(error)) => return Err(error),
            Err(error) => return Err(format!("case task panicked: {error}")),
        }
    }
    records.sort_by(|a, b| a.case_id.cmp(&b.case_id).then(a.repeat.cmp(&b.repeat)));
    Ok(records)
}
