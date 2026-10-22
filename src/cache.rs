//! Recent search results, so a question many people ask is routed and ranked
//! once, not once per person.
//!
//! The key is everything a result depends on: the request, the store's
//! generation (every write to the taxonomy or an item moves it) and the
//! evaluator settings' generation (every settings change moves it). A stale
//! result cannot come back; entries also expire after [`TTL`].

use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Results kept at once; the oldest goes first.
const CAPACITY: usize = 1024;

/// How long a result may be served again.
const TTL: Duration = Duration::from_secs(600);

type Key = [u8; 32];

#[derive(Clone)]
pub struct ResultCache {
    inner: Option<Arc<Mutex<Entries>>>,
}

#[derive(Default)]
struct Entries {
    map: HashMap<Key, (Instant, Value)>,
    order: VecDeque<Key>,
}

impl Default for ResultCache {
    fn default() -> Self {
        Self {
            inner: Some(Arc::default()),
        }
    }
}

impl ResultCache {
    /// A cache that never keeps anything: the eval runner measures every run.
    pub fn disabled() -> Self {
        Self { inner: None }
    }

    pub fn key(parts: &[&str]) -> Key {
        let mut hasher = Sha256::new();
        for part in parts {
            hasher.update(part.len().to_le_bytes());
            hasher.update(part.as_bytes());
        }
        hasher.finalize().into()
    }

    pub fn get(&self, key: &Key) -> Option<Value> {
        let inner = self.inner.as_ref()?;
        let mut entries = inner.lock().ok()?;
        match entries.map.get(key) {
            Some((at, value)) if at.elapsed() < TTL => Some(value.clone()),
            Some(_) => {
                entries.map.remove(key);
                None
            }
            None => None,
        }
    }

    pub fn put(&self, key: Key, value: Value) {
        let Some(inner) = self.inner.as_ref() else {
            return;
        };
        let Ok(mut entries) = inner.lock() else {
            return;
        };
        if entries.map.insert(key, (Instant::now(), value)).is_none() {
            entries.order.push_back(key);
        }
        while entries.map.len() > CAPACITY {
            let Some(oldest) = entries.order.pop_front() else {
                break;
            };
            entries.map.remove(&oldest);
        }
        // Keys of entries that expired or were replaced linger in `order`;
        // drop them once they outnumber the live ones.
        if entries.order.len() > 2 * CAPACITY {
            let Entries { map, order } = &mut *entries;
            order.retain(|key| map.contains_key(key));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn a_result_comes_back_under_its_own_key_only() {
        let cache = ResultCache::default();
        let key = ResultCache::key(&["search", "환불은 언제 돼요?", "7"]);
        cache.put(key, json!({"leaf_id": "refund"}));
        assert_eq!(cache.get(&key).unwrap()["leaf_id"], "refund");
        let moved = ResultCache::key(&["search", "환불은 언제 돼요?", "8"]);
        assert!(
            cache.get(&moved).is_none(),
            "another generation is another key"
        );
    }

    #[test]
    fn parts_do_not_run_into_each_other() {
        assert_ne!(
            ResultCache::key(&["ab", "c"]),
            ResultCache::key(&["a", "bc"])
        );
    }

    #[test]
    fn the_oldest_result_goes_first() {
        let cache = ResultCache::default();
        let first = ResultCache::key(&["0"]);
        for i in 0..=CAPACITY {
            cache.put(ResultCache::key(&[&i.to_string()]), json!(i));
        }
        assert!(cache.get(&first).is_none());
        assert!(
            cache
                .get(&ResultCache::key(&[&CAPACITY.to_string()]))
                .is_some()
        );
    }

    #[test]
    fn a_disabled_cache_keeps_nothing() {
        let cache = ResultCache::disabled();
        let key = ResultCache::key(&["x"]);
        cache.put(key, json!(1));
        assert!(cache.get(&key).is_none());
    }
}
