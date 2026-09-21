//! Hierarchical knowledge service: Jev-driven tree descent over SQLite.
//!
//! The HTTP surface lives in [`http`]; classification and ranking live in [`engine`].

pub mod engine;
pub mod error;
pub mod http;
pub mod jev;
pub mod models;
pub mod secret;
pub mod store;

use crate::error::Error;
use crate::jev::{JevClient, JevConfig};
use crate::models::Node;
use crate::store::Store;
use axum::Json;
use std::{
    env,
    sync::{Arc, RwLock},
};

pub type AppResult<T> = Result<Json<T>, Error>;

#[derive(Clone)]
pub struct AppState {
    pub store: Store,
    pub nodes: Arc<RwLock<Vec<Node>>>,
    pub jev: JevClient,
}

impl AppState {
    pub fn taxonomy(&self) -> Result<Vec<Node>, Error> {
        self.nodes
            .read()
            .map(|guard| guard.clone())
            .map_err(|_| Error::Poisoned)
    }

    pub fn reload_taxonomy(&self) -> Result<(), Error> {
        let loaded = self.store.load_taxonomy()?;
        let mut guard = self.nodes.write().map_err(|_| Error::Poisoned)?;
        *guard = loaded;
        Ok(())
    }
}

/// One-way boot seeding: env values fill EMPTY DB slots only.
pub fn seed_settings_from_env(store: &Store) {
    let seeded = |key: &str| {
        store
            .setting_get(key)
            .map(|v| v.unwrap_or_default())
            .unwrap_or_default()
    };
    let seed_secret = |slot: &str, value: String| {
        if !value.trim().is_empty() && seeded(slot).is_empty() {
            let _ = store.setting_set(slot, &crate::secret::encrypt(value.trim()));
        }
    };
    let seed_plain = |slot: &str, value: String| {
        if !value.trim().is_empty() && seeded(slot).is_empty() {
            let _ = store.setting_set(slot, value.trim());
        }
    };
    if let Ok(v) = env::var("JEV_TREE_INIT_API_KEY") {
        seed_secret("jev.api_key", v);
    } else if let Ok(v) = env::var("TYPESAFE_API_KEY") {
        seed_secret("jev.api_key", v);
    }
    if let Ok(v) = env::var("JEV_TREE_INIT_BASE_URL") {
        seed_plain("llm.base_url", v.trim().trim_end_matches('/').to_string());
    }
    if let Ok(v) = env::var("JEV_TREE_INIT_TOKEN") {
        seed_secret("llm.token", v);
    }
    if let Ok(v) = env::var("JEV_TREE_INIT_MODEL") {
        seed_plain("llm.model", v);
    }
    if let Ok(v) = env::var("JEV_TREE_INIT_SERVER_KEY") {
        let v = v.trim().to_string();
        if v.chars().count() >= crate::secret::PASSWORD_MIN_CHARS
            && seeded("auth.server_key_hash").is_empty()
        {
            let _ = store.setting_set("auth.server_key_hash", &crate::secret::hash_server_key(&v));
        }
    }
}

/// A bind address that only local processes can reach.
pub fn is_loopback_bind(bind: &str) -> bool {
    let bind = bind.trim();
    bind.eq_ignore_ascii_case("localhost")
        || bind
            .parse::<std::net::IpAddr>()
            .map(|ip| ip.is_loopback())
            .unwrap_or(false)
}

/// Open mode (no login password) lets any caller claim the password, reset the seed, and
/// read settings. That is fine on loopback and unsafe anywhere else, so refuse to start
/// unless the operator opts in explicitly.
pub fn refuse_open_public_bind(bind: &str, has_server_key: bool) -> Option<String> {
    let opted_in = env::var("JEV_TREE_ALLOW_OPEN")
        .map(|v| v == "1")
        .unwrap_or(false);
    if has_server_key || opted_in || is_loopback_bind(bind) {
        return None;
    }
    Some(format!(
        "refusing to bind {bind} without a login password: open mode lets anyone claim the \
         password and reset the database. Set JEV_TREE_INIT_SERVER_KEY (6+ characters), or \
         set JEV_TREE_ALLOW_OPEN=1 to accept the risk."
    ))
}

pub fn load_nodes(seed: &str) -> Result<Vec<Node>, Box<dyn std::error::Error + Send + Sync>> {
    #[derive(serde::Deserialize)]
    struct SeedFile {
        nodes: Vec<Node>,
    }
    Ok(serde_json::from_str::<SeedFile>(&std::fs::read_to_string(seed)?)?.nodes)
}

pub fn build_state(
    db: &str,
    seed: &str,
) -> Result<AppState, Box<dyn std::error::Error + Send + Sync>> {
    let store = Store::open(db, seed)?;
    seed_settings_from_env(&store);
    let nodes = Arc::new(RwLock::new(store.load_taxonomy()?));
    let mut jev_config = JevConfig::default();
    if let Ok(settings) = store.load_settings() {
        jev_config.with_db(&settings);
    }
    Ok(AppState {
        jev: JevClient::with_config(jev_config, nodes.clone())?,
        store,
        nodes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loopback_binds_are_recognised() {
        for bind in ["127.0.0.1", "::1", "localhost", " 127.0.0.1 "] {
            assert!(is_loopback_bind(bind), "{bind}");
        }
        for bind in ["0.0.0.0", "::", "10.0.0.5", ""] {
            assert!(!is_loopback_bind(bind), "{bind}");
        }
    }

    #[test]
    fn open_mode_refuses_a_public_bind() {
        assert!(refuse_open_public_bind("0.0.0.0", false).is_some());
        assert!(refuse_open_public_bind("0.0.0.0", true).is_none());
        assert!(refuse_open_public_bind("127.0.0.1", false).is_none());
    }
}
