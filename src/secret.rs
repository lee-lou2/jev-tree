//! Server-managed secrets: encryption, hashing, token issuance.
//!
//! Three rules:
//! - DB dumps must not yield usable keys: secrets at rest are AES-256-GCM
//!   encrypted with a key derived (Argon2id) from material that lives
//!   outside the DB (`JEV_TREE_SECRET_KEY` file/env, else per-machine file).
//! - Session tokens are opaque: only their SHA-256 hash is stored.
//! - Runtime never reads key material from env: env seeds the DB once at
//!   boot, then the DB is the single source of truth.

use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit, Nonce};
use argon2::Argon2;
use base64::{
    engine::general_purpose::{STANDARD as B64, URL_SAFE_NO_PAD as B64URL},
    Engine,
};
use rand::Rng;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Mutex, OnceLock};

fn fill_random(buf: &mut [u8]) {
    rand::rng().fill_bytes(buf);
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Raw key material. Priority: explicit file > explicit env > per-machine
/// file (auto-created next to the DB). Never stored in the database.
fn key_material(db_path: &str) -> Vec<u8> {
    if let Ok(path) = std::env::var("JEV_TREE_SECRET_KEY_FILE") {
        if let Ok(raw) = std::fs::read(&path) {
            if !raw.is_empty() {
                return raw;
            }
        }
    }
    if let Ok(raw) = std::env::var("JEV_TREE_SECRET_KEY") {
        if !raw.trim().is_empty() {
            return raw.trim().as_bytes().to_vec();
        }
    }
    let sibling = std::path::Path::new(db_path)
        .parent()
        .map(|p| p.join(".jev-tree.key"))
        .unwrap_or_else(|| std::path::PathBuf::from(".jev-tree.key"));
    if let Ok(raw) = std::fs::read(&sibling) {
        if !raw.is_empty() {
            return raw;
        }
    }
    let mut buf = vec![0u8; 32];
    fill_random(&mut buf);
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    options.mode(0o600);
    match options.open(&sibling) {
        Ok(mut file) => {
            use std::io::Write;
            let _ = file.write_all(&buf);
            buf
        }
        Err(_) => match std::fs::read(&sibling) {
            Ok(raw) if !raw.is_empty() => raw,
            _ => buf,
        },
    }
}

/// Derive the AES-256 key with Argon2id (per-machine salt persisted beside
/// the key file so the same material yields the same key across restarts).
fn aes_key(db_path: &str) -> [u8; 32] {
    static KEYS: OnceLock<Mutex<HashMap<String, [u8; 32]>>> = OnceLock::new();
    let cache = KEYS.get_or_init(|| Mutex::new(HashMap::new()));
    let mut guard = cache.lock().unwrap_or_else(|e| e.into_inner());
    *guard
        .entry(db_path.to_string())
        .or_insert_with(|| derive_aes_key(db_path))
}

fn derive_aes_key(db_path: &str) -> [u8; 32] {
    let material = key_material(db_path);
    let salt_path = std::path::Path::new(db_path)
        .parent()
        .map(|p| p.join(".jev-tree.salt"))
        .unwrap_or_else(|| std::path::PathBuf::from(".jev-tree.salt"));
    // Atomic first-write: concurrent processes must converge on one salt.
    let salt_bytes: Vec<u8> = match std::fs::read(&salt_path) {
        Ok(raw) if raw.len() >= 16 => raw,
        _ => {
            let mut buf = vec![0u8; 16];
            fill_random(&mut buf);
            match std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&salt_path)
            {
                Ok(mut file) => {
                    use std::io::Write;
                    let _ = file.write_all(&buf);
                    buf
                }
                // Lost the race: read the winner's salt.
                Err(_) => match std::fs::read(&salt_path) {
                    Ok(raw) if raw.len() >= 16 => raw,
                    _ => buf,
                },
            }
        }
    };
    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(&material, &salt_bytes, &mut key)
        .expect("argon2 must run");
    key
}

fn db_path() -> String {
    std::env::var("JEV_TREE_DB").unwrap_or_else(|_| "data/demo.db".into())
}

/// Encrypt a secret for storage. Format: `v2:<base64(nonce‖ciphertext)>`.
/// Nonce is random per value (AES-GCM), so equal plaintexts differ at rest.
pub fn encrypt(plaintext: &str) -> String {
    let cipher = Aes256Gcm::new_from_slice(&aes_key(&db_path())).expect("key must be 32B");
    let mut nonce_bytes = [0u8; 12];
    fill_random(&mut nonce_bytes);
    let nonce = Nonce::from(nonce_bytes);
    let mut out = nonce_bytes.to_vec();
    out.extend(
        cipher
            .encrypt(&nonce, plaintext.as_bytes())
            .expect("encryption must succeed"),
    );
    format!("v2:{}", B64.encode(out))
}

/// Decrypt a value produced by [`encrypt`]. `v1:` (legacy XOR) values are
/// still readable and transparently upgraded by the caller re-saving.
pub fn decrypt(stored: &str) -> Result<String, String> {
    if let Some(encoded) = stored.strip_prefix("v2:") {
        let raw = B64.decode(encoded).map_err(|e| e.to_string())?;
        if raw.len() < 12 {
            return Err("ciphertext too short".into());
        }
        let cipher = Aes256Gcm::new_from_slice(&aes_key(&db_path())).expect("key must be 32B");
        let (nonce_bytes, ciphertext) = raw.split_at(12);
        let nonce = Nonce::try_from(nonce_bytes).map_err(|_| "bad nonce".to_string())?;
        let plain = cipher
            .decrypt(&nonce, ciphertext)
            .map_err(|_| "decryption failed (wrong key or tampered value)".to_string())?;
        return String::from_utf8(plain).map_err(|e| e.to_string());
    }
    if let Some(encoded) = stored.strip_prefix("v1:") {
        // Legacy XOR envelope: same resolution as v2 but kept readable so
        // existing rows keep working until re-saved.
        let raw = B64.decode(encoded).map_err(|e| e.to_string())?;
        let material = key_material(&db_path());
        let plain: Vec<u8> = raw
            .iter()
            .enumerate()
            .map(|(i, b)| b ^ material[i % material.len()])
            .collect();
        return String::from_utf8(plain).map_err(|e| e.to_string());
    }
    // Pre-envelope plain values pass through for backward compatibility.
    Ok(stored.to_string())
}

pub fn sha256_hex(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    hex(&hasher.finalize())
}

/// Salted server-key hash: `v1:<salt-b64>:<sha256-hex(salt+key)>`.
pub fn hash_server_key(key: &str) -> String {
    let mut salt = [0u8; 16];
    fill_random(&mut salt);
    let salt_b64 = B64.encode(salt);
    let mut hasher = Sha256::new();
    hasher.update(salt);
    hasher.update(key.as_bytes());
    format!("v1:{salt_b64}:{}", hex(&hasher.finalize()))
}

pub fn verify_server_key(key: &str, stored: &str) -> bool {
    let parts: Vec<&str> = stored.split(':').collect();
    if parts.len() != 3 || parts[0] != "v1" {
        return false;
    }
    let Ok(salt) = B64.decode(parts[1]) else {
        return false;
    };
    let mut hasher = Sha256::new();
    hasher.update(salt);
    hasher.update(key.as_bytes());
    hex(&hasher.finalize()) == parts[2]
}

/// Issue an opaque session token (returned once, never stored raw).
pub fn new_token() -> String {
    let mut buf = [0u8; 32];
    fill_random(&mut buf);
    B64.encode(buf)
}

/// Minimum length for a custom login password (character count, not bytes).
pub const PASSWORD_MIN_CHARS: usize = 6;

/// Generate a login password: `jev_tree_sk_<43 base64url chars>`
/// (192-bit entropy). Older `jt-XXXX-XXXX-XXXX` values keep working —
/// verification hashes, not formats.
const SERVER_KEY_PREFIX: &str = "jev_tree_sk_";

pub fn new_server_key() -> String {
    let mut buf = [0u8; 32];
    fill_random(&mut buf);
    format!("{SERVER_KEY_PREFIX}{}", B64URL.encode(buf))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_key_roundtrip() {
        let key = new_server_key();
        assert!(key.starts_with(SERVER_KEY_PREFIX));
        assert!(key.chars().count() >= PASSWORD_MIN_CHARS);
        assert!(key.len() > 40);
        let stored = hash_server_key(&key);
        assert!(verify_server_key(&key, &stored));
        assert!(!verify_server_key("wrong", &stored));
    }

    #[test]
    fn legacy_short_key_still_verifies() {
        // Format changed; stored hashes verify any string, so existing
        // `jt-…` keys are not locked out by the rotation.
        let stored = hash_server_key("jt-ABCD-1234-EFGH");
        assert!(verify_server_key("jt-ABCD-1234-EFGH", &stored));
    }

    #[test]
    fn v2_encrypt_roundtrip_and_nonce_varies() {
        // Isolate from the developer machine's real salt/key files.
        let dir = std::env::temp_dir().join(format!("jev-tree-sectest-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("test.db").to_string_lossy().to_string();
        std::env::set_var("JEV_TREE_DB", &path);
        let k1 = aes_key(&path);
        let k2 = aes_key(&path);
        assert_eq!(k1, k2, "aes_key must be stable for the same db path");
        let a = encrypt("hello-jevisopen");
        assert!(a.starts_with("v2:"));
        assert_eq!(decrypt(&a).unwrap(), "hello-jevisopen");
        // Same plaintext -> different ciphertext (random nonce).
        assert_ne!(a, encrypt("hello-jevisopen"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn token_is_opaque_and_unique() {
        assert_ne!(new_token(), new_token());
        let h1 = sha256_hex("abc");
        assert_eq!(h1.len(), 64);
        assert_ne!(h1, sha256_hex("abd"));
    }
}
