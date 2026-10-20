use crate::engine::{depth, execute, resolve_root, run_deadline};
use crate::error::Error;
use crate::models::*;
use crate::{AppResult, AppState};
use axum::{
    Router,
    extract::{Query, State},
    http::{StatusCode, Uri},
    response::{
        IntoResponse, Json, Response,
        sse::{Event, KeepAlive, Sse},
    },
    routing::{get, post},
};
use futures_core::Stream;
use serde_json::{Value, json};
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

/// Settings, key management, model probes and the destructive seed reset belong
/// to the operator, so they need a login session. An integration's API key is for
/// the data endpoints: one that could mint keys or rotate the password would
/// outlive its own revocation.
fn require_admin(state: &AppState, session: &Session) -> Result<(), Error> {
    authed(state, session)?;
    if session.via_api_key {
        return Err(Error::Forbidden(
            "this endpoint needs a login session; an API key cannot use it".into(),
        ));
    }
    Ok(())
}

/// axum answers extractor failures (bad JSON, a wrong content type, a body over
/// the limit) and routing misses (405) with plain text. Every `/api` error is
/// documented as `{code, detail}`, so those are wrapped here; responses that
/// already are JSON pass through untouched.
async fn error_envelope(response: Response) -> Response {
    let status = response.status();
    if !(status.is_client_error() || status.is_server_error()) {
        return response;
    }
    let is_json = response
        .headers()
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.starts_with("application/json"));
    if is_json {
        return response;
    }
    let (mut parts, body) = response.into_parts();
    let text = match axum::body::to_bytes(body, 8 * 1024).await {
        Ok(bytes) => String::from_utf8_lossy(&bytes).trim().to_string(),
        Err(_) => String::new(),
    };
    let (status, code) = match status {
        StatusCode::UNPROCESSABLE_ENTITY => (StatusCode::BAD_REQUEST, "invalid_request"),
        StatusCode::UNAUTHORIZED => (status, "unauthorized"),
        StatusCode::FORBIDDEN => (status, "forbidden"),
        StatusCode::NOT_FOUND => (status, "not_found"),
        StatusCode::CONFLICT => (status, "conflict"),
        other if other.is_server_error() => (other, "internal_error"),
        other => (other, "invalid_request"),
    };
    let detail = if text.is_empty() {
        status
            .canonical_reason()
            .unwrap_or("request failed")
            .to_lowercase()
    } else {
        text
    };
    parts.headers.remove(axum::http::header::CONTENT_TYPE);
    parts.headers.remove(axum::http::header::CONTENT_LENGTH);
    (
        status,
        parts.headers,
        Json(json!({"code": code, "detail": detail})),
    )
        .into_response()
}

/// What a handler panic becomes: the same `{code, detail}` envelope, and no
/// panic text, which can carry internals.
pub fn panic_response(_panic: Box<dyn std::any::Any + Send + 'static>) -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"code": "internal_error", "detail": "internal error"})),
    )
        .into_response()
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
        .merge(guarded)
        .layer(axum::middleware::map_response(error_envelope));
    let index_spa = index_file.clone();
    Router::new()
        .route(
            "/",
            get({
                let index_file = index_file.clone();
                move || async move { index_file_handler(&index_file).await }
            }),
        )
        .route(
            "/docs",
            get(move || async move { html_file_handler(&docs_file).await }),
        )
        .nest("/api", api)
        .nest_service("/static", ServeDir::new(static_dir))
        .fallback(move |uri: Uri| {
            let index_file = index_spa.clone();
            async move { spa_fallback(&index_file, uri).await }
        })
}

/// UI paths like `/products` or `/products/products_stock` load the same app.
/// API, static files, and `/docs` stay on their own routes. Settings and keys
/// are project-wide and do not live under a tree path.
async fn spa_fallback(index_file: &str, uri: Uri) -> Response {
    let path = uri.path();
    if path == "/api" || path.starts_with("/api/") || path.starts_with("/static/") {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"code": "not_found", "detail": "not found"})),
        )
            .into_response();
    }
    index_file_handler(index_file).await.into_response()
}

async fn index_file_handler(path: &str) -> impl IntoResponse + use<> {
    html_file_handler(path).await
}

async fn html_file_handler(path: &str) -> impl IntoResponse + use<> {
    let body = std::fs::read(path).unwrap_or_default();
    (
        [(axum::http::header::CACHE_CONTROL, "no-store")],
        axum::response::Html(body),
    )
}

#[derive(Debug, serde::Deserialize, Default)]
struct RootQuery {
    root: Option<String>,
}

fn root_arg(query: &RootQuery) -> Option<&str> {
    query
        .root
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn scoped_nodes(
    state: &AppState,
    root: Option<&str>,
) -> Result<(Vec<Node>, crate::engine::RootScope), Error> {
    let taxonomy = state.taxonomy()?;
    let scope = resolve_root(&taxonomy, root)?;
    Ok((taxonomy, scope))
}

async fn health(State(state): State<AppState>, Query(query): Query<RootQuery>) -> AppResult<Value> {
    let (categories, items) = state.store.stats()?;
    let scope = resolve_root(&state.taxonomy()?, root_arg(&query))?;
    let (categories, items) = match scope.allowed.as_ref() {
        Some(allowed) => {
            let categories = allowed.len() as i64;
            let mut items = 0;
            for id in allowed {
                items += state.store.count_for(id)?;
            }
            (categories, items)
        }
        None => (categories, items),
    };
    Ok(Json(
        json!({"evaluator":state.jev.label(),"categories":categories,"items":items,"runtime":"rust-axum"}),
    ))
}

async fn seed_stats(
    State(state): State<AppState>,
    Query(query): Query<RootQuery>,
) -> AppResult<Value> {
    let (nodes, items) = state.store.stats()?;
    let (taxonomy, scope) = scoped_nodes(&state, root_arg(&query))?;
    let (nodes, items, max_depth) = match scope.allowed.as_ref() {
        Some(allowed) => {
            let nodes = allowed.len() as i64;
            let mut items = 0;
            for id in allowed {
                items += state.store.count_for(id)?;
            }
            let max_depth = allowed
                .iter()
                .map(|id| depth(id, &taxonomy))
                .max()
                .unwrap_or(0);
            (nodes, items, max_depth)
        }
        None => {
            let max_depth = taxonomy
                .iter()
                .map(|node| depth(&node.id, &taxonomy))
                .max()
                .unwrap_or(0);
            (nodes, items, max_depth)
        }
    };
    Ok(Json(
        json!({"nodes":nodes,"items":items,"max_depth":max_depth,"runtime":"rust-axum"}),
    ))
}

async fn tree(State(state): State<AppState>, Query(query): Query<RootQuery>) -> AppResult<Value> {
    let (taxonomy, scope) = scoped_nodes(&state, root_arg(&query))?;
    let mut payload = state
        .store
        .tree(&taxonomy, scope.root.as_ref().map(|node| node.id.as_str()))?;
    if let Some(obj) = payload.as_object_mut() {
        obj.insert(
            "root".into(),
            match scope.root {
                Some(node) => json!({
                    "id": node.id,
                    "name": node.name,
                    "description": node.description,
                    "parent_id": node.parent_id,
                }),
                None => Value::Null,
            },
        );
        obj.insert(
            "path".into(),
            json!(
                scope
                    .path
                    .iter()
                    .map(|node| json!({"id": node.id, "name": node.name}))
                    .collect::<Vec<_>>()
            ),
        );
    }
    Ok(Json(payload))
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
async fn openapi_spec_handler(path: &str) -> impl IntoResponse + use<> {
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
    tracing::info!("login password created through first-run setup");
    Ok(Json(json!({"server_key": key, "token": token})))
}

/// Argon2id costs about 19 MiB and tens of milliseconds per attempt. Without a
/// cap, a burst of simultaneous logins would all pass the guard before the first
/// failure is recorded and hash at once, and the memory alone would be the
/// attack. Two at a time; the rest queue, and ask the guard again on their turn.
static LOGIN_HASH_SLOTS: tokio::sync::Semaphore = tokio::sync::Semaphore::const_new(2);

async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<Value>, Error> {
    let stored = state
        .store
        .setting_get("auth.server_key_hash")?
        .filter(|hash| !hash.trim().is_empty());
    let Some(hash) = stored else {
        // Open mode: no login password configured, issue a session right away.
        let token = crate::secret::new_token();
        state.store.session_create(&token, SESSION_TTL_SECS)?;
        return Ok(Json(json!({"token": token, "open_mode": true})));
    };
    // After a run of wrong guesses the door closes for a while. Refuse before
    // hashing: while it is closed, a guess costs the server nothing.
    state.store.login_allowed()?;
    let _slot = LOGIN_HASH_SLOTS
        .acquire()
        .await
        .map_err(|_| Error::Internal("login queue closed".into()))?;
    // The door may have closed while this attempt waited its turn.
    state.store.login_allowed()?;
    let key = body.server_key.trim().to_string();
    // Argon2id takes tens of milliseconds and a good chunk of memory: keep it
    // off the async workers. A hash from before Argon2id is replaced on the
    // spot, while the password is in hand.
    let (ok, upgraded) = tokio::task::spawn_blocking({
        let hash = hash.clone();
        move || {
            let ok = crate::secret::verify_server_key(&key, &hash);
            let upgraded = (ok && crate::secret::server_key_needs_upgrade(&hash))
                .then(|| crate::secret::hash_server_key(&key));
            (ok, upgraded)
        }
    })
    .await
    .map_err(|error| Error::Internal(error.to_string()))?;
    if !ok {
        state.store.login_failed();
        return Err(Error::Unauthorized("invalid password".into()));
    }
    state.store.login_succeeded();
    if let Some(upgraded) = upgraded {
        state.store.setting_set("auth.server_key_hash", &upgraded)?;
        tracing::info!("login password hash upgraded to argon2id");
    }
    let token = crate::secret::new_token();
    state.store.session_create(&token, SESSION_TTL_SECS)?;
    tracing::info!("login ok");
    Ok(Json(json!({"token": token})))
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
    require_admin(&state, &session)?;
    let mut changed: Vec<&str> = Vec::new();
    if let Some(name) = patch.site_name {
        let name = name.trim().to_string();
        if name.chars().count() > 60 {
            return Err(Error::BadRequest(
                "site name must be 60 characters or fewer".into(),
            ));
        }
        state.store.setting_set("site.name", &name)?;
        changed.push("site_name");
    }
    if let Some(description) = patch.site_description {
        let description = description.trim().to_string();
        if description.chars().count() > 200 {
            return Err(Error::BadRequest(
                "description must be 200 characters or fewer".into(),
            ));
        }
        state.store.setting_set("site.description", &description)?;
        changed.push("site_description");
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
        changed.push("site_logo");
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
        changed.push("jev_api_key");
    }
    if let Some(base_url) = patch.llm_base_url {
        let base_url = normalize_base_url(&base_url)?;
        state.store.setting_set("llm.base_url", &base_url)?;
        changed.push("llm_base_url");
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
        changed.push("llm_token");
    }
    if let Some(model) = patch.llm_model {
        let model = model.trim().to_string();
        if model.chars().count() > 120 {
            return Err(Error::BadRequest("model name is too long".into()));
        }
        state.store.setting_set("llm.model", &model)?;
        changed.push("llm_model");
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
        changed.push("server_key");
    }
    // Names only, never values: who changed what is worth a log line.
    if !changed.is_empty() {
        tracing::info!(?changed, "settings changed");
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
    require_admin(&state, &session)?;
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
    require_admin(&state, &session)?;
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
    tracing::info!(%name, "api key issued");
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
    require_admin(&state, &session)?;
    Ok(Json(json!({"keys": state.store.api_key_list()?})))
}

/// Revoke a consumer key by id. Existing sessions minted from it are NOT
/// retroactively killed (tokens live 30d); rotate promptly after a leak.
async fn revoke_key(
    State(state): State<AppState>,
    session: axum::Extension<Session>,
    axum::extract::Path(id): axum::extract::Path<i64>,
) -> Result<Json<Value>, Error> {
    require_admin(&state, &session)?;
    if !state.store.api_key_revoke(id)? {
        return Err(Error::NotFound("key not found".into()));
    }
    tracing::info!(id, "api key revoked");
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
    root: Option<String>,
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
    let root = query
        .root
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty());
    let taxonomy = state.taxonomy()?;
    let tree_scope = resolve_root(&taxonomy, root)?;
    let category = match query
        .category_id
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty())
    {
        Some(id) => {
            if tree_scope
                .allowed
                .as_ref()
                .is_some_and(|allowed| !allowed.contains(id))
            {
                return Err(Error::BadRequest(
                    "category_id is outside the current root".into(),
                ));
            }
            Some(id.to_string())
        }
        None => tree_scope.root.as_ref().map(|node| node.id.clone()),
    };
    let include_descendants = match query.category_id.as_deref() {
        Some(_) => query.scope == "subtree",
        None => tree_scope.root.is_some() || query.scope == "subtree",
    };
    let (items, total) = state.store.list_items(
        query.offset,
        limit,
        Some(&query.q),
        category.as_deref(),
        include_descendants,
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
    session: axum::Extension<Session>,
    body: Option<Json<SeedReset>>,
) -> AppResult<Value> {
    require_admin(&state, &session)?;
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
    tracing::warn!(
        nodes,
        items,
        "seed reset: every category and item was replaced"
    );
    Ok(Json(json!({"seeded":{"nodes":nodes,"items":items}})))
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
    use crate::AppState;
    use crate::jev::{JevClient, JevConfig};
    use crate::store::Store;
    use std::sync::{Arc, RwLock};

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
        // SAFETY: test-only bootstrap, called before this fixture's server starts.
        unsafe {
            std::env::set_var("JEV_TREE_DB", db.to_str().unwrap());
            std::env::set_var("JEV_TREE_SEED", seed.to_str().unwrap());
            std::env::set_var(
                "JEV_TREE_SECRET_KEY",
                "test-secret-material-for-jev-tree-http",
            );
        }
        (guard, dir, db, static_dir)
    }

    fn test_router(db: &std::path::Path, static_dir: &str) -> (Router, AppState) {
        let seed = std::env::var("JEV_TREE_SEED").unwrap();
        let store = Store::open(db.to_str().unwrap(), &seed).unwrap();
        let nodes = Arc::new(RwLock::new(store.load_taxonomy().unwrap()));
        let mut config = JevConfig::default();
        if let Ok(settings) = store.load_settings() {
            config.with_db(&settings);
        }
        let state = AppState {
            store,
            jev: JevClient::with_config(config).unwrap(),
            nodes,
        };
        (router(state.clone(), static_dir.to_string()), state)
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
        let (app, state) = test_router(&db, &static_dir);

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
        assert_eq!(health["evaluator"], "unconfigured");
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
        assert_eq!(status, axum::http::StatusCode::BAD_REQUEST, "{search}");
        assert_eq!(search["code"], "invalid_request");
        assert!(
            search["detail"]
                .as_str()
                .unwrap_or("")
                .contains("Jev API key")
        );

        state.jev.script_choices(vec!["orders_tracking".into()]);
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
        assert_eq!(search["trace"]["router"], "flat");
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

        let (status, scoped) = send(
            &app,
            axum::http::Request::builder()
                .uri("/api/tree?root=orders")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(status, axum::http::StatusCode::OK, "{scoped}");
        assert_eq!(scoped["root"]["id"], "orders");
        let kids = scoped["tree"].as_array().unwrap();
        assert_eq!(kids.len(), 1);
        assert_eq!(kids[0]["id"], "orders_tracking");

        let (status, missing) = send(
            &app,
            axum::http::Request::builder()
                .uri("/api/tree?root=missing")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(status, axum::http::StatusCode::NOT_FOUND, "{missing}");

        let (status, nested) = send(
            &app,
            json_req(
                "POST",
                "/api/run",
                json!({
                    "mode":"search",
                    "query":"해외 배송이 통관에서 멈췄어요",
                    "root":"orders"
                }),
                None,
            ),
        )
        .await;
        assert_eq!(status, axum::http::StatusCode::OK, "{nested}");
        assert_eq!(nested["leaf_id"], "orders_tracking");

        let (status, scoped_items) = send(
            &app,
            axum::http::Request::builder()
                .uri("/api/items?root=orders&limit=20")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(status, axum::http::StatusCode::OK, "{scoped_items}");
        let scoped_ids: Vec<&str> = scoped_items["items"]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| row["category_id"].as_str().unwrap())
            .collect();
        assert!(scoped_ids.iter().all(|id| *id == "orders_tracking"));

        let (status, outside) = send(
            &app,
            axum::http::Request::builder()
                .uri("/api/items?root=orders&category_id=account")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(status, axum::http::StatusCode::BAD_REQUEST, "{outside}");

        let spa = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/orders")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(spa.status(), axum::http::StatusCode::OK);
        let spa_html =
            String::from_utf8(spa.into_body().collect().await.unwrap().to_bytes().to_vec())
                .unwrap();
        assert!(spa_html.contains("lab.js"));

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
        let (app, _state) = test_router(&db, &static_dir);

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
        assert_eq!(health["evaluator"], "unconfigured");
    }

    async fn send_full(
        app: &Router,
        req: axum::http::Request<axum::body::Body>,
    ) -> (axum::http::StatusCode, axum::http::HeaderMap, Value) {
        use http_body_util::BodyExt;
        use tower::ServiceExt as _;
        let response = app.clone().oneshot(req).await.unwrap();
        let status = response.status();
        let headers = response.headers().clone();
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body = serde_json::from_slice(&bytes)
            .unwrap_or_else(|_| json!(String::from_utf8_lossy(&bytes)));
        (status, headers, body)
    }

    fn bare_req(
        method: &str,
        uri: &str,
        token: Option<&str>,
    ) -> axum::http::Request<axum::body::Body> {
        let mut builder = axum::http::Request::builder().method(method).uri(uri);
        if let Some(token) = token {
            builder = builder.header(axum::http::header::AUTHORIZATION, format!("Bearer {token}"));
        }
        builder.body(axum::body::Body::empty()).unwrap()
    }

    /// Turn on key mode; returns the login password, a session token, and an API key.
    async fn key_mode(app: &Router) -> (String, String, String) {
        let (status, setup) = send(
            app,
            json_req("POST", "/api/setup/server-key", json!({}), None),
        )
        .await;
        assert_eq!(status, axum::http::StatusCode::OK, "{setup}");
        let password = setup["server_key"].as_str().unwrap().to_string();
        let session = setup["token"].as_str().unwrap().to_string();
        let (status, issued) = send(
            app,
            json_req("POST", "/api/keys", json!({"name":"ci"}), Some(&session)),
        )
        .await;
        assert_eq!(status, axum::http::StatusCode::OK, "{issued}");
        (
            password,
            session,
            issued["key"].as_str().unwrap().to_string(),
        )
    }

    #[tokio::test]
    #[allow(clippy::await_holding_lock)]
    async fn an_api_key_cannot_use_the_operator_endpoints() {
        use axum::http::StatusCode;
        let (_guard, _dir, db, static_dir) = test_paths();
        let (app, _state) = test_router(&db, &static_dir);
        let (password, session, api_key) = key_mode(&app).await;

        let with_body = [
            (
                "PATCH",
                "/api/settings",
                json!({"server_key": "rotated-by-a-key"}),
            ),
            (
                "PATCH",
                "/api/settings",
                json!({"llm_base_url": "http://127.0.0.1:9"}),
            ),
            (
                "POST",
                "/api/llm/models",
                json!({"base_url": "http://127.0.0.1:9", "token": "x"}),
            ),
            ("POST", "/api/keys", json!({"name": "second"})),
            ("POST", "/api/seed", json!({"confirm": "RESET"})),
        ];
        for (method, uri, body) in with_body {
            let (status, error) = send(&app, json_req(method, uri, body, Some(&api_key))).await;
            assert_eq!(status, StatusCode::FORBIDDEN, "{method} {uri}: {error}");
            assert_eq!(error["code"], "forbidden", "{method} {uri}");
        }
        for (method, uri) in [("GET", "/api/keys"), ("DELETE", "/api/keys/1")] {
            let (status, error) = send(&app, bare_req(method, uri, Some(&api_key))).await;
            assert_eq!(status, StatusCode::FORBIDDEN, "{method} {uri}: {error}");
            assert_eq!(error["code"], "forbidden", "{method} {uri}");
        }

        // Nothing above took effect: the password is the one it was.
        let (status, _) = send(
            &app,
            json_req(
                "POST",
                "/api/auth/login",
                json!({"server_key": password}),
                None,
            ),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let (_, keys) = send(&app, bare_req("GET", "/api/keys", Some(&session))).await;
        assert_eq!(keys["keys"].as_array().unwrap().len(), 1, "{keys}");

        // The data endpoints are what an integration key is for.
        for uri in [
            "/api/health",
            "/api/items?limit=1",
            "/api/tree",
            "/api/settings",
        ] {
            let (status, body) = send(&app, bare_req("GET", uri, Some(&api_key))).await;
            assert_eq!(status, StatusCode::OK, "{uri}: {body}");
        }
        let (status, _) = send(&app, bare_req("DELETE", "/api/items/9999", Some(&api_key))).await;
        assert_eq!(
            status,
            StatusCode::NOT_FOUND,
            "reaches the handler, not the guard"
        );

        // A login session still does everything.
        let (status, body) = send(&app, bare_req("GET", "/api/keys", Some(&session))).await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let (status, body) = send(
            &app,
            json_req(
                "PATCH",
                "/api/settings",
                json!({"site_name": "ops"}),
                Some(&session),
            ),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
    }

    #[tokio::test]
    #[allow(clippy::await_holding_lock)]
    async fn extractor_and_routing_errors_use_the_error_envelope() {
        use axum::http::StatusCode;
        let (_guard, _dir, db, static_dir) = test_paths();
        let (app, _state) = test_router(&db, &static_dir);
        let raw = |method: &str, uri: &str, content_type: Option<&str>, body: Vec<u8>| {
            let mut builder = axum::http::Request::builder().method(method).uri(uri);
            if let Some(content_type) = content_type {
                builder = builder.header(axum::http::header::CONTENT_TYPE, content_type);
            }
            builder.body(axum::body::Body::from(body)).unwrap()
        };
        let json_type = Some("application/json");
        let cases = [
            (
                "truncated JSON",
                raw(
                    "POST",
                    "/api/run",
                    json_type,
                    br#"{"mode":"search","query":"#.to_vec(),
                ),
                StatusCode::BAD_REQUEST,
            ),
            (
                "a field of the wrong type",
                raw(
                    "POST",
                    "/api/run",
                    json_type,
                    br#"{"mode":"search","query":"x","context":"notarray"}"#.to_vec(),
                ),
                StatusCode::BAD_REQUEST,
            ),
            (
                "no JSON content type",
                raw("POST", "/api/run", Some("text/plain"), b"{}".to_vec()),
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
            ),
            (
                "a body over the limit",
                raw("POST", "/api/run", json_type, vec![b' '; 2_600_000]),
                StatusCode::PAYLOAD_TOO_LARGE,
            ),
            (
                "a method the path does not take",
                raw("POST", "/api/health", json_type, b"{}".to_vec()),
                StatusCode::METHOD_NOT_ALLOWED,
            ),
            (
                "a query value of the wrong type",
                raw("GET", "/api/items?limit=abc", None, vec![]),
                StatusCode::BAD_REQUEST,
            ),
            (
                "a path value of the wrong type",
                raw("DELETE", "/api/keys/abc", None, vec![]),
                StatusCode::BAD_REQUEST,
            ),
        ];
        for (what, request, expected) in cases {
            let (status, headers, body) = send_full(&app, request).await;
            assert_eq!(status, expected, "{what}: {body}");
            assert_eq!(body["code"], "invalid_request", "{what}: {body}");
            assert!(
                body["detail"]
                    .as_str()
                    .is_some_and(|detail| !detail.is_empty()),
                "{what}: {body}"
            );
            assert!(
                headers
                    .get(axum::http::header::CONTENT_TYPE)
                    .and_then(|value| value.to_str().ok())
                    .is_some_and(|value| value.starts_with("application/json")),
                "{what}: {headers:?}"
            );
            if expected == StatusCode::METHOD_NOT_ALLOWED {
                assert!(
                    headers.contains_key(axum::http::header::ALLOW),
                    "Allow survives"
                );
            }
        }
    }

    #[tokio::test]
    async fn a_panic_is_answered_with_the_error_envelope() {
        use http_body_util::BodyExt;
        let response = panic_response(Box::new("secret detail from inside"));
        assert_eq!(
            response.status(),
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        );
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(body["code"], "internal_error");
        assert!(!body.to_string().contains("secret detail"), "{body}");
    }

    #[tokio::test]
    #[allow(clippy::await_holding_lock)]
    async fn repeated_wrong_passwords_close_the_login_for_a_while() {
        use axum::http::StatusCode;
        let (_guard, _dir, db, static_dir) = test_paths();
        let (app, _state) = test_router(&db, &static_dir);
        let (password, session, _api_key) = key_mode(&app).await;
        let login = |password: &str| {
            json_req(
                "POST",
                "/api/auth/login",
                json!({"server_key": password}),
                None,
            )
        };
        for attempt in 0..crate::store::LoginGuard::FREE_FAILURES {
            let (status, body) = send(&app, login("not-the-password")).await;
            assert_eq!(
                status,
                StatusCode::UNAUTHORIZED,
                "attempt {attempt}: {body}"
            );
        }
        // The door is closed now, even for the right password.
        let (status, headers, body) = send_full(&app, login(&password)).await;
        assert_eq!(status, StatusCode::TOO_MANY_REQUESTS, "{body}");
        assert_eq!(body["code"], "rate_limited", "{body}");
        let retry_after: u64 = headers
            .get(axum::http::header::RETRY_AFTER)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse().ok())
            .expect("Retry-After says how long to wait");
        assert!((1..=crate::store::LoginGuard::MAX_BLOCK_SECS).contains(&retry_after));
        // Someone already logged in is not locked out of their own session.
        let (status, body) = send(&app, bare_req("GET", "/api/health", Some(&session))).await;
        assert_eq!(status, StatusCode::OK, "{body}");
    }

    #[tokio::test]
    #[allow(clippy::await_holding_lock)]
    async fn a_burst_of_logins_is_throttled_instead_of_hashed_all_at_once() {
        use axum::http::StatusCode;
        let (_guard, _dir, db, static_dir) = test_paths();
        let (app, _state) = test_router(&db, &static_dir);
        let (_password, _session, _api_key) = key_mode(&app).await;
        let attempts = futures_util::future::join_all((0..10).map(|n| {
            let app = app.clone();
            async move {
                send(
                    &app,
                    json_req(
                        "POST",
                        "/api/auth/login",
                        json!({"server_key": format!("wrong-{n}")}),
                        None,
                    ),
                )
                .await
                .0
            }
        }))
        .await;
        let wrong = attempts
            .iter()
            .filter(|status| **status == StatusCode::UNAUTHORIZED)
            .count();
        let refused = attempts
            .iter()
            .filter(|status| **status == StatusCode::TOO_MANY_REQUESTS)
            .count();
        assert_eq!(wrong + refused, attempts.len(), "{attempts:?}");
        // Only the free misses (plus the one already hashing when the door shut)
        // were hashed; the attempts queued behind them were turned away.
        assert!(
            wrong >= crate::store::LoginGuard::FREE_FAILURES as usize,
            "{attempts:?}"
        );
        assert!(refused >= 1, "{attempts:?}");
    }

    #[tokio::test]
    #[allow(clippy::await_holding_lock)]
    async fn a_login_replaces_an_old_sha256_hash_with_argon2id() {
        use axum::http::StatusCode;
        let (_guard, _dir, db, static_dir) = test_paths();
        let (app, state) = test_router(&db, &static_dir);
        state
            .store
            .setting_set(
                "auth.server_key_hash",
                &crate::secret::legacy_server_key_hash("old-password"),
            )
            .unwrap();
        let login = |password: &str| {
            json_req(
                "POST",
                "/api/auth/login",
                json!({"server_key": password}),
                None,
            )
        };
        let (status, _) = send(&app, login("wrong-password")).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert!(
            state
                .store
                .setting_get("auth.server_key_hash")
                .unwrap()
                .unwrap()
                .starts_with("v1:"),
            "a failed login must not touch the hash"
        );
        let (status, first) = send(&app, login("old-password")).await;
        assert_eq!(status, StatusCode::OK, "{first}");
        let stored = state
            .store
            .setting_get("auth.server_key_hash")
            .unwrap()
            .unwrap();
        assert!(stored.starts_with("v2:$argon2id$"), "{stored}");
        // The same password keeps working against the new hash.
        let (status, second) = send(&app, login("old-password")).await;
        assert_eq!(status, StatusCode::OK, "{second}");
    }

    #[test]
    fn openapi_describes_this_build_and_its_auth() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static/openapi.json");
        let spec: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
        assert_eq!(
            spec["info"]["version"],
            env!("CARGO_PKG_VERSION"),
            "bump static/openapi.json with Cargo.toml"
        );
        let public = [
            "/api/setup/status",
            "/api/setup/server-key",
            "/api/auth/login",
            "/api/site",
            "/api/openapi.json",
        ];
        let operator = [
            ("/api/settings", "patch"),
            ("/api/llm/models", "post"),
            ("/api/keys", "post"),
            ("/api/keys", "get"),
            ("/api/keys/{id}", "delete"),
            ("/api/seed", "post"),
        ];
        let mut guarded = 0;
        for (path, item) in spec["paths"].as_object().unwrap() {
            for (method, operation) in item.as_object().unwrap() {
                let responses = operation["responses"].as_object().unwrap();
                if public.contains(&path.as_str()) {
                    assert!(
                        operation.get("security").is_none(),
                        "{method} {path} is public"
                    );
                    continue;
                }
                guarded += 1;
                assert_eq!(
                    operation["security"],
                    json!([{"bearer": []}]),
                    "{method} {path}"
                );
                assert!(
                    responses.contains_key("401"),
                    "{method} {path} must declare 401"
                );
                if operator.contains(&(path.as_str(), method.as_str())) {
                    assert!(
                        responses.contains_key("403"),
                        "{method} {path} must declare 403"
                    );
                }
            }
        }
        assert!(guarded >= 15, "{guarded} guarded operations");
        for status in ["401", "429"] {
            assert!(
                spec["paths"]["/api/auth/login"]["post"]["responses"]
                    .get(status)
                    .is_some(),
                "login declares {status}"
            );
        }
    }
}
