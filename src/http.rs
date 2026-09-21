use crate::engine::{depth, execute};
use crate::error::Error;
use crate::models::*;
use crate::{AppResult, AppState};
use axum::{
    extract::{Query, State},
    response::{
        sse::{Event, KeepAlive, Sse},
        IntoResponse, Json, Response,
    },
    routing::{get, post},
    Router,
};
use futures_core::Stream;
use serde_json::{json, Value};
use std::{convert::Infallible, env};
use tokio::sync::mpsc;
use tower_http::services::ServeDir;

const SESSION_TTL_SECS: i64 = 30 * 24 * 3600;

#[derive(Clone, Default)]
struct Session {
    token: String,
    authed: bool,
    via_api_key: bool,
}

async fn session(
    State(state): State<AppState>,
    req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<Response, Error> {
    let needs_auth = state.store.has_server_key().unwrap_or(false);
    if !needs_auth {
        let mut req = req;
        req.extensions_mut().insert(Session::default());
        return Ok(next.run(req).await);
    }
    let token = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .unwrap_or("")
        .to_string();
    // Consumer API keys (per-integration, revocable) also authenticate.
    // They never slide/expire like login sessions.
    if !token.is_empty() && state.store.api_key_valid(&token).unwrap_or(false) {
        let mut req = req;
        req.extensions_mut().insert(Session {
            token,
            authed: true,
            via_api_key: true,
        });
        return Ok(next.run(req).await);
    }
    if token.is_empty() || !state.store.session_valid(&token).unwrap_or(false) {
        return Err(Error::Unauthorized("login required".into()));
    }
    // Sliding expiry + opportunistic sweep of dead sessions.
    let _ = state.store.session_create(&token, SESSION_TTL_SECS);
    let _ = state.store.session_sweep();
    let mut req = req;
    req.extensions_mut().insert(Session {
        token,
        authed: true,
        via_api_key: false,
    });
    Ok(next.run(req).await)
}

fn require_session(session: &Session, needs_auth: bool) -> Result<(), Error> {
    if needs_auth && !session.authed {
        return Err(Error::Unauthorized("login required".into()));
    }
    Ok(())
}

pub fn router(state: AppState, static_dir: String) -> Router {
    let open = Router::new()
        .route("/setup/status", get(setup_status))
        .route("/setup/server-key", post(setup_server_key))
        .route("/auth/login", post(login))
        .route("/site", get(site_info))
        .with_state(state.clone());
    let guarded = Router::new()
        .route("/auth/logout", post(logout))
        .route("/health", get(health))
        .route("/seed/stats", get(seed_stats))
        .route("/seed", axum::routing::post(reset_seed))
        .route("/tree", get(tree))
        .route("/items", get(list_items))
        .route("/items/{id}", axum::routing::delete(archive_item))
        .route("/settings", get(get_settings))
        .route("/settings", axum::routing::patch(patch_settings))
        .route("/llm/models", post(llm_models))
        .route("/keys", post(issue_key))
        .route("/keys", get(list_keys))
        .route("/keys/{id}", axum::routing::delete(revoke_key))
        .route("/run", post(run_once))
        .route("/run/stream", post(run_stream))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), session));
    let spec_file = format!("{}/openapi.json", static_dir.trim_end_matches('/'));
    let docs_file = format!("{}/docs.html", static_dir.trim_end_matches('/'));
    let index_file = format!("{}/index.html", static_dir.trim_end_matches('/'));
    let api = open
        .route(
            "/openapi.json",
            get(move || async move { openapi_spec_handler(&spec_file).await }),
        )
        .merge(guarded);
    Router::new()
        .route(
            "/",
            get(move || async move { index_file_handler(&index_file).await }),
        )
        .route(
            "/docs",
            get(move || async move { html_file_handler(&docs_file).await }),
        )
        .nest("/api", api)
        .nest_service("/static", ServeDir::new(static_dir.clone()))
        .fallback_service(ServeDir::new(static_dir).append_index_html_on_directories(true))
}

async fn index_file_handler(path: &str) -> impl IntoResponse {
    html_file_handler(path).await
}

async fn html_file_handler(path: &str) -> impl IntoResponse {
    let body = std::fs::read(path).unwrap_or_default();
    (
        [(axum::http::header::CACHE_CONTROL, "no-store")],
        axum::response::Html(body),
    )
}

async fn health(State(state): State<AppState>) -> AppResult<Value> {
    let (categories, items) = state.store.stats()?;
    Ok(Json(
        json!({"evaluator":state.jev.label(),"categories":categories,"items":items,"runtime":"rust-axum"}),
    ))
}

async fn seed_stats(State(state): State<AppState>) -> AppResult<Value> {
    let (nodes, items) = state.store.stats()?;
    let taxonomy = state.taxonomy()?;
    let max_depth = taxonomy
        .iter()
        .map(|node| depth(&node.id, &taxonomy))
        .max()
        .unwrap_or(0);
    Ok(Json(
        json!({"nodes":nodes,"items":items,"max_depth":max_depth,"runtime":"rust-axum"}),
    ))
}

async fn tree(State(state): State<AppState>) -> AppResult<Value> {
    let taxonomy = state.taxonomy()?;
    Ok(Json(state.store.tree(&taxonomy)?))
}

// ---------------------------------------------------------------------------
// Setup / auth / settings — server-managed, stored in SQLite
// ---------------------------------------------------------------------------

/// Public first-run state. `site` is included so the login screen can brand
/// itself before authentication.
async fn setup_status(State(state): State<AppState>) -> AppResult<Value> {
    let settings = state.store.load_settings()?;
    Ok(Json(json!({
        "has_server_key": state.store.has_server_key()?,
        "jev_api_key_set": !settings.jev_api_key.is_empty(),
        "evaluator": state.jev.label(),
        "site": {
            "site_name": settings.site_name,
            "site_description": settings.site_description,
            "site_logo": settings.site_logo,
        },
    })))
}

/// Machine-readable API contract (static/openapi.json). Served under /api so
/// agents can fetch it straight from a running server. Human companion:
/// docs/api.md in the repo.
async fn openapi_spec_handler(path: &str) -> impl IntoResponse {
    let body = std::fs::read(path).unwrap_or_else(|_| b"{}".to_vec());
    (
        [(axum::http::header::CONTENT_TYPE, "application/json")],
        body,
    )
}

/// Public branding (name/description/logo) for header + login screen.
async fn site_info(State(state): State<AppState>) -> AppResult<Value> {
    let settings = state.store.load_settings()?;
    Ok(Json(json!({
        "site_name": settings.site_name,
        "site_description": settings.site_description,
        "site_logo": settings.site_logo,
    })))
}

/// One-time login-password bootstrap. Fails when a password already exists.
/// Omit `server_key` to generate one (`jev_tree_sk_…`) server-side.
async fn setup_server_key(
    State(state): State<AppState>,
    Json(body): Json<SetupRequest>,
) -> Result<Json<Value>, Error> {
    if state.store.has_server_key()? {
        return Err(Error::BadRequest("a password is already set".into()));
    }
    let key = body
        .server_key
        .filter(|k| !k.trim().is_empty())
        .unwrap_or_else(crate::secret::new_server_key);
    if key.chars().count() < crate::secret::PASSWORD_MIN_CHARS {
        return Err(Error::BadRequest(format!(
            "password must be at least {} characters",
            crate::secret::PASSWORD_MIN_CHARS
        )));
    }
    state.store.setting_set(
        "auth.server_key_hash",
        &crate::secret::hash_server_key(&key),
    )?;
    let token = crate::secret::new_token();
    state.store.session_create(&token, SESSION_TTL_SECS)?;
    Ok(Json(json!({"server_key": key, "token": token})))
}

async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<Value>, Error> {
    let stored = state.store.setting_get("auth.server_key_hash")?;
    match stored {
        // Open mode: no login password configured, issue a session right away.
        None => {
            let token = crate::secret::new_token();
            state.store.session_create(&token, SESSION_TTL_SECS)?;
            Ok(Json(json!({"token": token, "open_mode": true})))
        }
        Some(hash) if hash.trim().is_empty() => {
            let token = crate::secret::new_token();
            state.store.session_create(&token, SESSION_TTL_SECS)?;
            Ok(Json(json!({"token": token, "open_mode": true})))
        }
        Some(hash) => {
            if !crate::secret::verify_server_key(body.server_key.trim(), &hash) {
                return Err(Error::Unauthorized("invalid password".into()));
            }
            let token = crate::secret::new_token();
            state.store.session_create(&token, SESSION_TTL_SECS)?;
            Ok(Json(json!({"token": token})))
        }
    }
}

async fn logout(
    State(state): State<AppState>,
    session: axum::Extension<Session>,
) -> AppResult<Value> {
    if !session.token.is_empty() && !session.via_api_key {
        let _ = state.store.session_revoke(&session.token);
    }
    Ok(Json(json!({"ok": true})))
}

fn authed(state: &AppState, session: &Session) -> Result<(), Error> {
    require_session(session, state.store.has_server_key().unwrap_or(false))
}

async fn get_settings(
    State(state): State<AppState>,
    session: axum::Extension<Session>,
) -> Result<Json<Value>, Error> {
    authed(&state, &session)?;
    let settings = state.store.load_settings()?;
    Ok(Json(settings.masked(state.store.has_server_key()?)))
}

/// Validate + persist settings. Secrets are encrypted at rest.
/// Empty-string secrets clear the stored value; `site_logo: null` clears it.
async fn patch_settings(
    State(state): State<AppState>,
    session: axum::Extension<Session>,
    Json(patch): Json<SettingsPatch>,
) -> Result<Json<Value>, Error> {
    authed(&state, &session)?;
    if let Some(name) = patch.site_name {
        let name = name.trim().to_string();
        if name.chars().count() > 60 {
            return Err(Error::BadRequest(
                "site name must be 60 characters or fewer".into(),
            ));
        }
        state.store.setting_set("site.name", &name)?;
    }
    if let Some(description) = patch.site_description {
        let description = description.trim().to_string();
        if description.chars().count() > 200 {
            return Err(Error::BadRequest(
                "description must be 200 characters or fewer".into(),
            ));
        }
        state.store.setting_set("site.description", &description)?;
    }
    if let Some(logo) = patch.site_logo {
        match logo {
            None => state.store.setting_set("site.logo", "")?,
            Some(data_url) => {
                let data_url = data_url.trim().to_string();
                validate_logo(&data_url)?;
                state.store.setting_set("site.logo", &data_url)?;
            }
        }
    }
    if let Some(key) = patch.jev_api_key {
        let key = key.trim().to_string();
        if key.is_empty() {
            state.store.setting_set("jev.api_key", "")?;
        } else {
            state
                .store
                .setting_set("jev.api_key", &crate::secret::encrypt(&key))?;
        }
    }
    if let Some(base_url) = patch.llm_base_url {
        let base_url = normalize_base_url(&base_url)?;
        state.store.setting_set("llm.base_url", &base_url)?;
    }
    if let Some(token) = patch.llm_token {
        let token = token.trim().to_string();
        if token.is_empty() {
            state.store.setting_set("llm.token", "")?;
        } else {
            state
                .store
                .setting_set("llm.token", &crate::secret::encrypt(&token))?;
        }
    }
    if let Some(model) = patch.llm_model {
        let model = model.trim().to_string();
        if model.chars().count() > 120 {
            return Err(Error::BadRequest("model name is too long".into()));
        }
        state.store.setting_set("llm.model", &model)?;
    }
    if let Some(server_key) = patch.server_key {
        if server_key.trim().is_empty() {
            state.store.setting_set("auth.server_key_hash", "")?;
        } else {
            if server_key.trim().chars().count() < crate::secret::PASSWORD_MIN_CHARS {
                return Err(Error::BadRequest(format!(
                    "password must be at least {} characters",
                    crate::secret::PASSWORD_MIN_CHARS
                )));
            }
            state.store.setting_set(
                "auth.server_key_hash",
                &crate::secret::hash_server_key(server_key.trim()),
            )?;
        }
    }
    // Hot-swap the Jev connection so settings apply without restart.
    // DB is the only source: no env reads at request time.
    let mut config = crate::jev::JevConfig::default();
    config.with_db(&state.store.load_settings()?);
    state.jev.reconfigure(config);
    let settings = state.store.load_settings()?;
    Ok(Json(json!({
        "ok": true,
        "evaluator": state.jev.label(),
        "settings": settings.masked(state.store.has_server_key()?),})))
}

/// Probe `{base}/v1/models` with explicit or saved credentials (no persistence).
async fn llm_models(
    State(state): State<AppState>,
    session: axum::Extension<Session>,
    Json(probe): Json<ModelsProbe>,
) -> Result<Json<Value>, Error> {
    authed(&state, &session)?;
    let settings = state.store.load_settings()?;
    let base = probe
        .base_url
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
        .or_else(|| {
            if settings.llm_base_url.trim().is_empty() {
                None
            } else {
                Some(settings.llm_base_url.clone())
            }
        });
    let token = probe.token.filter(|s| !s.trim().is_empty()).or_else(|| {
        if settings.llm_token.is_empty() {
            None
        } else {
            Some(settings.llm_token.clone())
        }
    });
    if base.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true) {
        return Err(Error::BadRequest("LLM base URL is required".into()));
    }
    if token.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true) {
        return Err(Error::BadRequest("LLM token is required".into()));
    }
    match state
        .jev
        .list_models(base.as_deref(), token.as_deref())
        .await
    {
        Ok((models, used_base)) => Ok(Json(json!({
            "models": models,
            "base_url": used_base,
        }))),
        Err(e) => Err(Error::Upstream(e)),
    }
}

// ---------------------------------------------------------------------------
// API keys for integrations (distinct from the Jev model key and the login password).
//
// Why a separate key table: the login password is one shared secret for people.
// CI, agents, and partners get their own revocable API keys instead.
// Rows store sha256(key) + metadata only; plaintext shows once at issue.
// ---------------------------------------------------------------------------

/// Issue a consumer API key. Body: `{"name": "ci-bot"}` (required,
/// ≤60 chars, unique). Returns the plaintext key ONCE — store it client-side.
async fn issue_key(
    State(state): State<AppState>,
    session: axum::Extension<Session>,
    Json(body): Json<serde_json::Value>,
) -> Result<Json<Value>, Error> {
    authed(&state, &session)?;
    let name = body
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim()
        .to_string();
    if name.is_empty() {
        return Err(Error::BadRequest("key name is required".into()));
    }
    if name.chars().count() > 60 {
        return Err(Error::BadRequest(
            "key name must be 60 characters or fewer".into(),
        ));
    }
    let key = crate::secret::new_server_key(); // same jev_tree_sk_ format
    state.store.api_key_issue(&name, &key)?;
    Ok(Json(json!({
        "id": state.store.api_key_id(&name)?.unwrap_or_default(),
        "name": name,
        "key": key, // plaintext exactly once
        "warning": "this key is shown once; store it somewhere safe",
    })))
}

/// List consumer keys (metadata + masked prefix only, never plaintext).
async fn list_keys(
    State(state): State<AppState>,
    session: axum::Extension<Session>,
) -> Result<Json<Value>, Error> {
    authed(&state, &session)?;
    Ok(Json(json!({"keys": state.store.api_key_list()?})))
}

/// Revoke a consumer key by id. Existing sessions minted from it are NOT
/// retroactively killed (tokens live 30d); rotate promptly after a leak.
async fn revoke_key(
    State(state): State<AppState>,
    session: axum::Extension<Session>,
    axum::extract::Path(id): axum::extract::Path<i64>,
) -> Result<Json<Value>, Error> {
    authed(&state, &session)?;
    if !state.store.api_key_revoke(id)? {
        return Err(Error::NotFound("key not found".into()));
    }
    Ok(Json(json!({"ok": true, "revoked": id})))
}

fn normalize_base_url(raw: &str) -> Result<String, Error> {
    let trimmed = raw.trim().trim_end_matches('/').to_string();
    if trimmed.is_empty() {
        return Ok(String::new());
    }
    if !(trimmed.starts_with("http://") || trimmed.starts_with("https://")) {
        return Err(Error::BadRequest(
            "base URL must start with http(s)://".into(),
        ));
    }
    if trimmed.chars().count() > 200 {
        return Err(Error::BadRequest("base URL is too long".into()));
    }
    Ok(trimmed)
}

fn validate_logo(data_url: &str) -> Result<(), Error> {
    if data_url.is_empty() {
        return Ok(());
    }
    let (meta, payload) = data_url
        .split_once(",")
        .ok_or_else(|| Error::BadRequest("invalid image data URL".into()))?;
    let meta = meta.to_ascii_lowercase();
    if !meta.starts_with("data:image/") || !meta.contains(";base64") {
        return Err(Error::BadRequest(
            "logo must be a PNG, JPEG, SVG, or WebP data URL".into(),
        ));
    }
    let kind = meta
        .trim_start_matches("data:image/")
        .split(';')
        .next()
        .unwrap_or("");
    if !["png", "jpeg", "jpg", "svg+xml", "webp"].contains(&kind) {
        return Err(Error::BadRequest(
            "logo must be a PNG, JPEG, SVG, or WebP data URL".into(),
        ));
    }
    // ~500 KB cap on the encoded payload.
    if payload.len() > 680_000 {
        return Err(Error::BadRequest("logo must be 500 KB or smaller".into()));
    }
    Ok(())
}

#[derive(Debug, serde::Deserialize)]
struct ItemsQuery {
    #[serde(default)]
    offset: usize,
    #[serde(default = "default_item_limit")]
    limit: usize,
    #[serde(default)]
    q: String,
    category_id: Option<String>,
    #[serde(default = "default_items_scope")]
    scope: String,
    #[serde(default = "default_items_status")]
    status: String,
}
fn default_items_scope() -> String {
    "exact".into()
}
fn default_items_status() -> String {
    "active".into()
}
fn default_item_limit() -> usize {
    50
}

async fn list_items(
    State(state): State<AppState>,
    Query(query): Query<ItemsQuery>,
) -> Result<Json<Value>, Error> {
    let limit = query.limit.clamp(1, 100);
    if query.scope != "exact" && query.scope != "subtree" {
        return Err(Error::BadRequest("scope must be exact|subtree".into()));
    }
    if !["active", "draft", "all"].contains(&query.status.as_str()) {
        return Err(Error::BadRequest("status must be active|draft|all".into()));
    }
    let (items, total) = state.store.list_items(
        query.offset,
        limit,
        Some(&query.q),
        query.category_id.as_deref(),
        query.scope == "subtree",
        &query.status,
    )?;
    Ok(Json(
        json!({"items":items,"total":total,"offset":query.offset,"limit":limit,"scope":query.scope,"status":query.status}),
    ))
}

/// Retire an item. Discards a draft, or takes a published answer out of service.
async fn archive_item(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<Value>, Error> {
    if !state.store.archive_item(&id)? {
        return Err(Error::NotFound("item not found".into()));
    }
    Ok(Json(json!({"ok": true, "archived": id})))
}

#[derive(Debug, serde::Deserialize, Default)]
struct SeedReset {
    #[serde(default)]
    confirm: String,
}

async fn reset_seed(
    State(state): State<AppState>,
    body: Option<Json<SeedReset>>,
) -> AppResult<Value> {
    let confirm = body.map(|Json(body)| body.confirm).unwrap_or_default();
    if confirm != "RESET" {
        return Err(Error::BadRequest(
            "seed reset requires {\"confirm\":\"RESET\"}; this deletes every category and item"
                .into(),
        ));
    }
    state
        .store
        .reset(&env::var("JEV_TREE_SEED").unwrap_or_else(|_| "data/seed.json".into()))
        .map_err(|error| Error::Internal(error.to_string()))?;
    state.reload_taxonomy()?;
    let (nodes, items) = state.store.stats()?;
    Ok(Json(json!({"seeded":{"nodes":nodes,"items":items}})))
}

/// Upper bound on a single descent so a degraded evaluator cannot pin a connection open.
fn run_deadline() -> std::time::Duration {
    let seconds = env::var("JEV_TREE_RUN_TIMEOUT_SECS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(120);
    std::time::Duration::from_secs(seconds)
}

async fn run_once(
    State(state): State<AppState>,
    Json(request): Json<RunRequest>,
) -> Result<Response, Error> {
    request.validate().map_err(Error::BadRequest)?;
    let result = tokio::time::timeout(run_deadline(), execute(state, request, None))
        .await
        .map_err(|_| Error::Timeout)??;
    Ok(Json(result).into_response())
}

async fn run_stream(
    State(state): State<AppState>,
    Json(request): Json<RunRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, Error> {
    request.validate().map_err(Error::BadRequest)?;
    let (tx, mut rx) = mpsc::channel::<Value>(64);
    tokio::spawn(async move {
        let result =
            tokio::time::timeout(run_deadline(), execute(state, request, Some(tx.clone())))
                .await
                .unwrap_or(Err(Error::Timeout));
        if let Err(error) = result {
            let _ = tx
                .send(json!({"type":"error","code":error.code(),"message":error.public_message()}))
                .await;
        }
    });
    let stream = async_stream::stream! {
        while let Some(value) = rx.recv().await {
            yield Ok(Event::default().json_data(value).unwrap_or_else(|_| Event::default().data("{}")));
        }
    };
    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jev::{JevClient, JevConfig};
    use crate::store::Store;
    use crate::AppState;
    use std::sync::{Arc, RwLock};

    #[test]
    fn jev_config_db_overrides_env() {
        let mut config = JevConfig {
            key: Some("env-key".into()),
            base_url: "https://api.typesafe.ai".into(),
            model: "jev-latest".into(),
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
    }
    #[test]
    fn path_score_is_geometric_mean() {
        assert!((crate::engine::path_score(&[0.8, 0.8]) - 0.8).abs() < 1e-9);
        assert!(crate::engine::path_score(&[0.9, 0.1]) < crate::engine::path_score(&[0.8, 0.8]));
    }
    #[test]
    fn normalize_marked_qa() {
        let (q, a) = crate::engine::normalize("Q: 카드 환불은 언제 되나요? A: 3~5영업일입니다.");
        assert_eq!(q, "카드 환불은 언제 되나요?");
        assert_eq!(a, "3~5영업일입니다.");
    }
    #[test]
    fn request_rejects_unsupported_mode() {
        let request: RunRequest =
            serde_json::from_value(json!({"mode":"classify","query":"x"})).unwrap();
        assert!(request.validate().is_err());
    }
    #[test]
    fn request_defaults_to_search() {
        let request: RunRequest = serde_json::from_value(json!({"query":"x"})).unwrap();
        assert_eq!(request.mode, "search");
        assert_eq!(request.beam_width, 3);
        assert!(request.validate().is_ok());
    }
    #[test]
    fn ingest_uses_question_and_answer_fields() {
        let request: RunRequest = serde_json::from_value(json!({
            "mode":"ingest",
            "question":"카드 환불은 언제 되나요?",
            "answer":"3~5영업일입니다."
        }))
        .unwrap();
        assert!(request.validate().is_ok());
        assert!(request.text().contains("카드 환불"));
        assert!(request.text().contains("3~5영업일"));
    }

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        LOCK.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn test_paths() -> (
        std::sync::MutexGuard<'static, ()>,
        tempfile::TempDir,
        std::path::PathBuf,
        String,
    ) {
        let guard = env_lock();
        let dir = tempfile::tempdir().unwrap();
        let db = dir.path().join("t.db");
        let seed =
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/mini-seed.json");
        let static_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("static")
            .canonicalize()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        std::env::set_var("JEV_TREE_DB", db.to_str().unwrap());
        std::env::set_var("JEV_TREE_SEED", seed.to_str().unwrap());
        std::env::set_var(
            "JEV_TREE_SECRET_KEY",
            "test-secret-material-for-jev-tree-http",
        );
        (guard, dir, db, static_dir)
    }

    fn test_router(db: &std::path::Path, static_dir: &str) -> Router {
        let seed = std::env::var("JEV_TREE_SEED").unwrap();
        let store = Store::open(db.to_str().unwrap(), &seed).unwrap();
        let nodes = Arc::new(RwLock::new(store.load_taxonomy().unwrap()));
        let mut config = JevConfig::default();
        if let Ok(settings) = store.load_settings() {
            config.with_db(&settings);
        }
        router(
            AppState {
                store,
                jev: JevClient::with_config(config, nodes.clone()).unwrap(),
                nodes,
            },
            static_dir.to_string(),
        )
    }

    async fn send(
        app: &Router,
        req: axum::http::Request<axum::body::Body>,
    ) -> (axum::http::StatusCode, Value) {
        use http_body_util::BodyExt;
        use tower::ServiceExt as _;
        let response = app.clone().oneshot(req).await.unwrap();
        let status = response.status();
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body = if bytes.is_empty() {
            json!(null)
        } else {
            serde_json::from_slice(&bytes)
                .unwrap_or_else(|_| json!(String::from_utf8_lossy(&bytes)))
        };
        (status, body)
    }

    fn json_req(
        method: &str,
        uri: &str,
        body: Value,
        token: Option<&str>,
    ) -> axum::http::Request<axum::body::Body> {
        let mut builder = axum::http::Request::builder()
            .method(method)
            .uri(uri)
            .header(axum::http::header::CONTENT_TYPE, "application/json");
        if let Some(token) = token {
            builder = builder.header(axum::http::header::AUTHORIZATION, format!("Bearer {token}"));
        }
        builder
            .body(axum::body::Body::from(body.to_string()))
            .unwrap()
    }

    #[tokio::test]
    #[allow(clippy::await_holding_lock)]
    async fn openapi_docs_and_open_mode_contract() {
        use http_body_util::BodyExt;
        use tower::ServiceExt as _;
        let (_guard, _dir, db, static_dir) = test_paths();
        let app = test_router(&db, &static_dir);

        let (status, spec) = send(
            &app,
            axum::http::Request::builder()
                .uri("/api/openapi.json")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(status, axum::http::StatusCode::OK);
        assert!(spec["openapi"].as_str().unwrap().starts_with("3."));
        assert!(spec["paths"].get("/api/run").is_some());
        assert!(
            spec["paths"]["/api/run"]["post"]["responses"]["200"]["content"]["application/json"]
                ["schema"]
                .get("content")
                .is_none()
        );

        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/docs")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let html = String::from_utf8(
            response
                .into_body()
                .collect()
                .await
                .unwrap()
                .to_bytes()
                .to_vec(),
        )
        .unwrap();
        assert!(html.contains("/api/openapi.json"));
        assert!(
            !html.contains("//cdn.") && !html.contains("//unpkg."),
            "/docs must not depend on a CDN"
        );

        let (status, health) = send(
            &app,
            axum::http::Request::builder()
                .uri("/api/health")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(status, axum::http::StatusCode::OK);
        assert_eq!(health["evaluator"], "heuristic");
        assert_eq!(health["categories"], 3);
        assert_eq!(health["items"], 2);

        let (status, stats) = send(
            &app,
            axum::http::Request::builder()
                .uri("/api/seed/stats")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(status, axum::http::StatusCode::OK, "{stats}");
        assert_eq!(stats["nodes"], 3);
        assert_eq!(stats["items"], 2);
        assert!(stats.get("version").is_none());
        assert!(stats.get("synthetic").is_none());

        let (status, search) = send(
            &app,
            json_req(
                "POST",
                "/api/run",
                json!({"mode":"search","query":"해외 배송이 통관에서 멈췄어요"}),
                None,
            ),
        )
        .await;
        assert_eq!(status, axum::http::StatusCode::OK, "{search}");
        assert_eq!(search["leaf_id"], "orders_tracking");
        assert!(search["items"].as_array().unwrap().iter().any(|row| {
            row["item"]["answer"]
                .as_str()
                .unwrap_or("")
                .contains("통관")
        }));

        let (status, classify) = send(
            &app,
            json_req(
                "POST",
                "/api/run",
                json!({"mode":"classify","query":"x"}),
                None,
            ),
        )
        .await;
        assert_eq!(status, axum::http::StatusCode::BAD_REQUEST);
        assert_eq!(classify["code"], "invalid_request");

        let (status, draft) = send(
            &app,
            json_req(
                "POST",
                "/api/run",
                json!({
                    "mode":"ingest",
                    "question":"통관이 왜 멈추나요?",
                    "answer":"개인통관번호를 확인하세요."
                }),
                None,
            ),
        )
        .await;
        assert_eq!(status, axum::http::StatusCode::OK, "{draft}");
        assert!(draft["published_id"].is_null());
        assert_eq!(draft["category_id"], "orders_tracking");

        let (status, saved) = send(
            &app,
            json_req(
                "POST",
                "/api/run",
                json!({
                    "mode":"ingest",
                    "question":"통관이 왜 멈추나요?",
                    "answer":"개인통관번호를 확인하세요.",
                    "auto_publish":true
                }),
                None,
            ),
        )
        .await;
        assert_eq!(status, axum::http::StatusCode::OK, "{saved}");
        assert!(!saved["published_id"].as_str().unwrap().is_empty());
        assert_eq!(saved["updated"], true);

        let (status, denied) = send(&app, json_req("POST", "/api/seed", json!({}), None)).await;
        assert_eq!(status, axum::http::StatusCode::BAD_REQUEST, "{denied}");

        let (status, seeded) = send(
            &app,
            json_req("POST", "/api/seed", json!({"confirm":"RESET"}), None),
        )
        .await;
        assert_eq!(status, axum::http::StatusCode::OK, "{seeded}");
        assert_eq!(seeded["seeded"]["nodes"], 3);

        let spec_paths = spec["paths"]
            .as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect::<std::collections::BTreeSet<_>>();
        let expected = [
            "/api/auth/login",
            "/api/auth/logout",
            "/api/health",
            "/api/items",
            "/api/items/{id}",
            "/api/keys",
            "/api/keys/{id}",
            "/api/llm/models",
            "/api/openapi.json",
            "/api/run",
            "/api/run/stream",
            "/api/seed",
            "/api/seed/stats",
            "/api/settings",
            "/api/setup/server-key",
            "/api/setup/status",
            "/api/site",
            "/api/tree",
        ]
        .into_iter()
        .map(str::to_string)
        .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(spec_paths, expected);
    }

    #[tokio::test]
    #[allow(clippy::await_holding_lock)]
    async fn key_mode_session_and_api_key() {
        let (_guard, _dir, db, static_dir) = test_paths();
        let app = test_router(&db, &static_dir);

        let (status, setup) = send(
            &app,
            json_req("POST", "/api/setup/server-key", json!({}), None),
        )
        .await;
        assert_eq!(status, axum::http::StatusCode::OK, "{setup}");
        let server_key = setup["server_key"].as_str().unwrap().to_string();
        let token = setup["token"].as_str().unwrap().to_string();
        assert!(server_key.starts_with("jev_tree_sk_"));

        let (status, too_short) = send(
            &app,
            json_req(
                "PATCH",
                "/api/settings",
                json!({"server_key":"12345"}),
                Some(&token),
            ),
        )
        .await;
        assert_eq!(status, axum::http::StatusCode::BAD_REQUEST, "{too_short}");
        assert_eq!(too_short["code"], "invalid_request");

        let (status, denied) = send(
            &app,
            axum::http::Request::builder()
                .uri("/api/health")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(status, axum::http::StatusCode::UNAUTHORIZED);
        assert_eq!(denied["code"], "unauthorized");

        let (status, health) = send(
            &app,
            axum::http::Request::builder()
                .uri("/api/health")
                .header(axum::http::header::AUTHORIZATION, format!("Bearer {token}"))
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(status, axum::http::StatusCode::OK, "{health}");

        let (status, _bad) = send(
            &app,
            json_req(
                "POST",
                "/api/auth/login",
                json!({"server_key":"wrong-key-that-is-long-enough"}),
                None,
            ),
        )
        .await;
        assert_eq!(status, axum::http::StatusCode::UNAUTHORIZED);

        let (status, login) = send(
            &app,
            json_req(
                "POST",
                "/api/auth/login",
                json!({"server_key":server_key}),
                None,
            ),
        )
        .await;
        assert_eq!(status, axum::http::StatusCode::OK, "{login}");
        let session = login["token"].as_str().unwrap();

        let (status, rotated) = send(
            &app,
            json_req(
                "PATCH",
                "/api/settings",
                json!({"server_key":"abcdef"}),
                Some(session),
            ),
        )
        .await;
        assert_eq!(status, axum::http::StatusCode::OK, "{rotated}");
        let (status, short_login) = send(
            &app,
            json_req(
                "POST",
                "/api/auth/login",
                json!({"server_key":"abcdef"}),
                None,
            ),
        )
        .await;
        assert_eq!(status, axum::http::StatusCode::OK, "{short_login}");

        let (status, issued) = send(
            &app,
            json_req("POST", "/api/keys", json!({"name":"ci"}), Some(session)),
        )
        .await;
        assert_eq!(status, axum::http::StatusCode::OK, "{issued}");
        let api_key = issued["key"].as_str().unwrap();

        let (status, health) = send(
            &app,
            axum::http::Request::builder()
                .uri("/api/health")
                .header(
                    axum::http::header::AUTHORIZATION,
                    format!("Bearer {api_key}"),
                )
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(status, axum::http::StatusCode::OK, "{health}");
        assert_eq!(health["evaluator"], "heuristic");
    }
}
