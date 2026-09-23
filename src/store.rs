use crate::models::{AppSettings, Item, Node};
use rusqlite::{Connection, OptionalExtension, params};
use serde::Deserialize;
use serde_json::Value;
use std::{
    path::Path,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct Store {
    inner: Arc<Mutex<Connection>>,
}

#[derive(Deserialize)]
struct Seed {
    nodes: Vec<Node>,
    items: Vec<SeedItem>,
}
#[derive(Deserialize)]
struct SeedItem {
    category_id: String,
    kind: String,
    question: String,
    answer: String,
}

impl Store {
    pub fn open(
        path: &str,
        seed_path: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(parent) = Path::new(path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        let connection = Connection::open(path)?;
        connection.busy_timeout(std::time::Duration::from_secs(5))?;
        connection.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;
            CREATE TABLE IF NOT EXISTS taxonomy_nodes(id TEXT PRIMARY KEY,parent_id TEXT,name TEXT NOT NULL,description TEXT NOT NULL DEFAULT '',examples_json TEXT NOT NULL DEFAULT '[]');
            CREATE INDEX IF NOT EXISTS idx_taxonomy_parent ON taxonomy_nodes(parent_id);
            CREATE TABLE IF NOT EXISTS knowledge_items(id TEXT PRIMARY KEY,category_id TEXT NOT NULL,question TEXT NOT NULL,answer TEXT NOT NULL,kind TEXT NOT NULL DEFAULT 'qa',tags_json TEXT NOT NULL DEFAULT '[]',status TEXT NOT NULL DEFAULT 'active',version INTEGER NOT NULL DEFAULT 1,metadata_json TEXT NOT NULL DEFAULT '{}');
            CREATE INDEX IF NOT EXISTS idx_knowledge_category ON knowledge_items(category_id,status);
            CREATE TABLE IF NOT EXISTS app_settings(key TEXT PRIMARY KEY,value TEXT NOT NULL DEFAULT '',updated_at TEXT NOT NULL DEFAULT '');
            CREATE TABLE IF NOT EXISTS sessions(token_hash TEXT PRIMARY KEY,created_at TEXT NOT NULL DEFAULT '',expires_at TEXT NOT NULL DEFAULT '');
            CREATE TABLE IF NOT EXISTS api_keys(id INTEGER PRIMARY KEY AUTOINCREMENT,name TEXT NOT NULL UNIQUE,key_hash TEXT NOT NULL,key_prefix TEXT NOT NULL DEFAULT '',created_at TEXT NOT NULL DEFAULT '',revoked INTEGER NOT NULL DEFAULT 0);")?;
        Self::migrate(&connection)?;
        let count: i64 =
            connection.query_row("SELECT COUNT(*) FROM taxonomy_nodes", [], |row| row.get(0))?;
        let store = Self {
            inner: Arc::new(Mutex::new(connection)),
        };
        if count == 0 {
            store.reset(seed_path)?;
        }
        Ok(store)
    }

    fn migrate(connection: &Connection) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 0.2.2 dropped the FTS mirror: it was written on every upsert and never queried.
        connection.execute_batch("DROP TABLE IF EXISTS knowledge_fts;")?;
        let mut has_version = false;
        let mut stmt = connection.prepare("PRAGMA table_info(knowledge_items)")?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(1))?;
        for name in rows {
            if name? == "version" {
                has_version = true;
                break;
            }
        }
        if !has_version {
            connection.execute(
                "ALTER TABLE knowledge_items ADD COLUMN version INTEGER NOT NULL DEFAULT 1",
                [],
            )?;
        }
        Ok(())
    }

    fn lock(&self) -> Result<std::sync::MutexGuard<'_, Connection>, crate::error::Error> {
        self.inner.lock().map_err(|_| crate::error::Error::Poisoned)
    }

    pub fn reset(&self, seed_path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let seed: Seed = serde_json::from_str(&std::fs::read_to_string(seed_path)?)?;
        validate_taxonomy(&seed.nodes).map_err(|detail| format!("invalid taxonomy: {detail}"))?;
        let mut connection = self.inner.lock().map_err(|_| "store lock poisoned")?;
        let transaction =
            connection.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        transaction.execute_batch("DELETE FROM knowledge_items; DELETE FROM taxonomy_nodes;")?;
        for node in seed.nodes {
            transaction.execute("INSERT INTO taxonomy_nodes(id,parent_id,name,description,examples_json) VALUES(?,?,?,?,?)", params![node.id,node.parent_id,node.name,node.description,serde_json::to_string(&node.examples)?])?;
        }
        for (index, item) in seed.items.into_iter().enumerate() {
            transaction.execute("INSERT INTO knowledge_items(id,category_id,question,answer,kind,version) VALUES(?,?,?,?,?,1)", params![(index + 1).to_string(),item.category_id,item.question,item.answer,item.kind])?;
        }
        transaction.commit()?;
        Ok(())
    }

    pub fn load_taxonomy(&self) -> Result<Vec<Node>, crate::error::Error> {
        let connection = self.lock()?;
        let mut statement = connection
            .prepare("SELECT id,parent_id,name,description,examples_json FROM taxonomy_nodes")?;
        let rows = statement.query_map([], |row| {
            let examples_json: String = row.get(4)?;
            let examples = serde_json::from_str(&examples_json).unwrap_or_default();
            Ok(Node {
                id: row.get(0)?,
                parent_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                examples,
            })
        })?;
        let mut nodes = Vec::new();
        for row in rows {
            nodes.push(row?);
        }
        Ok(nodes)
    }

    fn descendant_ids(nodes: &[Node], root: &str) -> Vec<String> {
        let mut ids = vec![root.to_string()];
        let mut cursor = 0;
        while cursor < ids.len() {
            let parent = ids[cursor].clone();
            ids.extend(
                nodes
                    .iter()
                    .filter(|node| node.parent_id.as_deref() == Some(parent.as_str()))
                    .map(|node| node.id.clone()),
            );
            cursor += 1;
        }
        ids
    }

    pub fn stats(&self) -> Result<(i64, i64), crate::error::Error> {
        let connection = self.lock()?;
        Ok((
            connection.query_row("SELECT COUNT(*) FROM taxonomy_nodes", [], |row| row.get(0))?,
            connection.query_row(
                "SELECT COUNT(*) FROM knowledge_items WHERE status='active'",
                [],
                |row| row.get(0),
            )?,
        ))
    }

    pub fn count_for(&self, category_id: &str) -> Result<i64, crate::error::Error> {
        let connection = self.lock()?;
        Ok(connection.query_row(
            "SELECT COUNT(*) FROM knowledge_items WHERE status='active' AND category_id=?",
            [category_id],
            |row| row.get(0),
        )?)
    }

    /// Active rows in these categories, in id order. Ranking is Jev's job;
    /// this does not drop rows that fail a lexical overlap check.
    pub fn active_in(&self, categories: &[String]) -> Result<Vec<Item>, crate::error::Error> {
        if categories.is_empty() {
            return Ok(Vec::new());
        }
        let connection = self.lock()?;
        let placeholders = vec!["?"; categories.len()].join(",");
        let mut statement = connection.prepare(&format!(
            "SELECT id,category_id,question,answer,kind,status,version FROM knowledge_items \
             WHERE status='active' AND category_id IN ({placeholders}) ORDER BY CAST(id AS INTEGER), id"
        ))?;
        let items = statement
            .query_map(rusqlite::params_from_iter(categories), item_from_row)?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(items)
    }

    /// Filter and paginate in SQL. `status` is `active`, `draft`, or `all`.
    pub fn list_items(
        &self,
        offset: usize,
        limit: usize,
        query: Option<&str>,
        category: Option<&str>,
        include_descendants: bool,
        status: &str,
    ) -> Result<(Vec<Item>, i64), crate::error::Error> {
        let allowed = match category {
            Some(root) if include_descendants => {
                Some(Self::descendant_ids(&self.load_taxonomy()?, root))
            }
            Some(root) => Some(vec![root.to_string()]),
            None => None,
        };
        let connection = self.lock()?;
        let mut clauses: Vec<String> = Vec::new();
        let mut args: Vec<String> = Vec::new();
        if status == "all" {
            clauses.push("status IN ('active','draft')".into());
        } else {
            clauses.push("status=?".into());
            args.push(status.to_string());
        }
        if let Some(ids) = &allowed {
            if ids.is_empty() {
                return Ok((Vec::new(), 0));
            }
            clauses.push(format!(
                "category_id IN ({})",
                vec!["?"; ids.len()].join(",")
            ));
            args.extend(ids.iter().cloned());
        }
        let needle = query.unwrap_or("").trim().to_lowercase();
        if !needle.is_empty() {
            clauses.push("(lower(question) LIKE ? OR lower(answer) LIKE ?)".into());
            args.push(format!("%{needle}%"));
            args.push(format!("%{needle}%"));
        }
        let where_sql = clauses.join(" AND ");
        let total: i64 = connection.query_row(
            &format!("SELECT COUNT(*) FROM knowledge_items WHERE {where_sql}"),
            rusqlite::params_from_iter(args.iter()),
            |row| row.get(0),
        )?;
        let mut statement = connection.prepare(&format!(
            "SELECT id,category_id,question,answer,kind,status,version FROM knowledge_items \
             WHERE {where_sql} ORDER BY CAST(id AS INTEGER), id LIMIT ? OFFSET ?"
        ))?;
        let mut page_args = args;
        page_args.push(limit.to_string());
        page_args.push(offset.to_string());
        let page = statement
            .query_map(rusqlite::params_from_iter(page_args.iter()), item_from_row)?
            .collect::<Result<Vec<_>, _>>()?;
        Ok((page, total))
    }

    /// Retire an item. Archived rows leave search, listings, and upsert matching.
    pub fn archive_item(&self, id: &str) -> Result<bool, crate::error::Error> {
        let connection = self.lock()?;
        Ok(connection.execute(
            "UPDATE knowledge_items SET status='archived' WHERE id=? AND status IN ('active','draft')",
            [id],
        )? > 0)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn upsert(
        &self,
        category: &str,
        question: &str,
        answer: &str,
        kind: &str,
        item_id: Option<&str>,
        expected_version: Option<i64>,
        publish: bool,
    ) -> Result<(Item, bool), crate::error::Error> {
        let status = if publish { "active" } else { "draft" };
        let mut connection = self.lock()?;
        let transaction =
            connection.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        let existing: Option<(String, i64)> = if let Some(id) = item_id {
            transaction.query_row("SELECT id,version FROM knowledge_items WHERE id=? AND status IN ('active','draft')", [id], |row| Ok((row.get(0)?,row.get(1)?))).optional()?
        } else {
            transaction.query_row("SELECT id,version FROM knowledge_items WHERE category_id=? AND trim(question)=trim(?) AND status IN ('active','draft') LIMIT 1", params![category,question], |row| Ok((row.get(0)?,row.get(1)?))).optional()?
        };
        let updated = existing.is_some();
        let id = if let Some((id, version)) = existing {
            if let Some(expected) = expected_version
                && expected != version
            {
                return Err(crate::error::Error::Conflict(format!(
                    "item version changed; current version is {version}"
                )));
            }
            transaction.execute("UPDATE knowledge_items SET category_id=?,question=?,answer=?,kind=?,status=?,version=version+1 WHERE id=?", params![category,question,answer,kind,status,id])?;
            id
        } else {
            if item_id.is_some() {
                return Err(crate::error::Error::NotFound("item not found".into()));
            }
            let next: i64 = transaction.query_row(
                "SELECT COALESCE(MAX(CAST(id AS INTEGER)),0)+1 FROM knowledge_items",
                [],
                |row| row.get(0),
            )?;
            let id = next.to_string();
            transaction.execute("INSERT INTO knowledge_items(id,category_id,question,answer,kind,status,version) VALUES(?,?,?,?,?,?,1)", params![id,category,question,answer,kind,status])?;
            id
        };
        transaction.commit()?;
        drop(connection);
        let item = self
            .item_by_id(&id)?
            .ok_or(crate::error::Error::NotFound("item not found".into()))?;
        Ok((item, updated))
    }

    fn item_by_id(&self, id: &str) -> Result<Option<Item>, crate::error::Error> {
        let connection = self.lock()?;
        Ok(connection.query_row("SELECT id,category_id,question,answer,kind,status,version FROM knowledge_items WHERE id=?", [id], item_from_row).optional()?)
    }

    // -- app settings (key/value; secrets encrypted by the caller) ------------

    pub fn setting_get(&self, key: &str) -> Result<Option<String>, crate::error::Error> {
        let connection = self.lock()?;
        Ok(connection
            .query_row("SELECT value FROM app_settings WHERE key=?", [key], |row| {
                row.get(0)
            })
            .optional()?)
    }

    pub fn setting_set(&self, key: &str, value: &str) -> Result<(), crate::error::Error> {
        let connection = self.lock()?;
        connection.execute(
            "INSERT INTO app_settings(key,value,updated_at) VALUES(?,?,datetime('now'))\
             ON CONFLICT(key) DO UPDATE SET value=excluded.value,updated_at=excluded.updated_at",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn load_settings(&self) -> Result<AppSettings, crate::error::Error> {
        let get = |key: &str| self.setting_get(key).map(|v| v.unwrap_or_default());
        let decrypted = |key: &str| {
            let raw = get(key)?;
            if raw.is_empty() {
                Ok(String::new())
            } else {
                crate::secret::decrypt(&raw).map_err(|e| {
                    crate::error::Error::Internal(format!("secret decode failed: {e}"))
                })
            }
        };
        Ok(AppSettings {
            site_name: get("site.name")?,
            site_description: get("site.description")?,
            site_logo: get("site.logo")?,
            jev_api_key: decrypted("jev.api_key")?,
            llm_base_url: get("llm.base_url")?,
            llm_token: decrypted("llm.token")?,
            llm_model: get("llm.model")?,
        })
    }

    pub fn has_server_key(&self) -> Result<bool, crate::error::Error> {
        Ok(self
            .setting_get("auth.server_key_hash")?
            .map(|v| !v.trim().is_empty())
            .unwrap_or(false))
    }

    // -- sessions (opaque bearer tokens, only SHA-256 hashes stored) ----------

    pub fn session_create(&self, token: &str, ttl_secs: i64) -> Result<(), crate::error::Error> {
        let connection = self.lock()?;
        // Upsert: re-issuing the same token (sliding refresh) extends expiry
        // instead of hitting the PRIMARY KEY.
        connection.execute(
            "INSERT INTO sessions(token_hash,created_at,expires_at)\
             VALUES(?,datetime('now'),datetime('now',?))\
             ON CONFLICT(token_hash) DO UPDATE SET expires_at=datetime('now',?)",
            params![
                crate::secret::sha256_hex(token),
                format!("+{ttl_secs} seconds"),
                format!("+{ttl_secs} seconds"),
            ],
        )?;
        Ok(())
    }

    pub fn session_valid(&self, token: &str) -> Result<bool, crate::error::Error> {
        let connection = self.lock()?;
        Ok(connection.query_row(
            "SELECT COUNT(*) FROM sessions WHERE token_hash=? AND expires_at>datetime('now')",
            [crate::secret::sha256_hex(token)],
            |row| row.get::<_, i64>(0),
        )? > 0)
    }

    pub fn session_revoke(&self, token: &str) -> Result<(), crate::error::Error> {
        let connection = self.lock()?;
        connection.execute(
            "DELETE FROM sessions WHERE token_hash=?",
            [crate::secret::sha256_hex(token)],
        )?;
        Ok(())
    }

    pub fn session_sweep(&self) -> Result<(), crate::error::Error> {
        let connection = self.lock()?;
        connection.execute("DELETE FROM sessions WHERE expires_at<=datetime('now')", [])?;
        Ok(())
    }

    // -- consumer API keys (OpenAPI operators; NOT the Jev model key) -------

    pub fn api_key_issue(&self, name: &str, key: &str) -> Result<(), crate::error::Error> {
        let connection = self.lock()?;
        let existed = connection
            .query_row(
                "SELECT COUNT(*) FROM api_keys WHERE name=?",
                [name],
                |row| row.get::<_, i64>(0),
            )
            .unwrap_or(0);
        if existed > 0 {
            return Err(crate::error::Error::BadRequest(
                "a key with that name already exists".into(),
            ));
        }
        connection.execute(
            "INSERT INTO api_keys(name,key_hash,key_prefix,created_at)\
             VALUES(?,?,?,datetime('now'))",
            params![
                name,
                crate::secret::sha256_hex(key),
                key.chars().take(12).collect::<String>(),
            ],
        )?;
        Ok(())
    }

    pub fn api_key_id(&self, name: &str) -> Result<Option<i64>, crate::error::Error> {
        let connection = self.lock()?;
        Ok(connection
            .query_row("SELECT id FROM api_keys WHERE name=?", [name], |row| {
                row.get(0)
            })
            .optional()?)
    }

    pub fn api_key_list(&self) -> Result<Value, crate::error::Error> {
        let connection = self.lock()?;
        let mut stmt = connection
            .prepare("SELECT id,name,key_prefix,created_at,revoked FROM api_keys ORDER BY id")?;
        let rows = stmt.query_map([], |row| {
            Ok(json::json!({
                "id": row.get::<_, i64>(0)?,
                "name": row.get::<_, String>(1)?,
                "prefix": row.get::<_, String>(2)?,
                "created_at": row.get::<_, String>(3)?,
                "revoked": row.get::<_, i64>(4)? != 0,
            }))
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(Value::Array(out))
    }

    pub fn api_key_revoke(&self, id: i64) -> Result<bool, crate::error::Error> {
        let connection = self.lock()?;
        let n = connection.execute("UPDATE api_keys SET revoked=1 WHERE id=?", [id])?;
        Ok(n > 0)
    }

    pub fn api_key_valid(&self, key: &str) -> Result<bool, crate::error::Error> {
        let connection = self.lock()?;
        Ok(connection.query_row(
            "SELECT COUNT(*) FROM api_keys WHERE key_hash=? AND revoked=0",
            [crate::secret::sha256_hex(key)],
            |row| row.get::<_, i64>(0),
        )? > 0)
    }

    pub fn item_counts(
        &self,
    ) -> Result<std::collections::HashMap<String, i64>, crate::error::Error> {
        let connection = self.lock()?;
        let mut statement = connection.prepare(
            "SELECT category_id, COUNT(*) FROM knowledge_items WHERE status='active' GROUP BY category_id",
        )?;
        let mut counts = std::collections::HashMap::new();
        for row in statement.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })? {
            let (id, count) = row?;
            counts.insert(id, count);
        }
        Ok(counts)
    }

    pub fn tree(&self, nodes: &[Node], parent: Option<&str>) -> Result<Value, crate::error::Error> {
        let counts = self.item_counts()?;
        fn build(
            counts: &std::collections::HashMap<String, i64>,
            nodes: &[Node],
            parent: Option<&str>,
        ) -> Result<Vec<Value>, crate::error::Error> {
            nodes.iter().filter(|node| node.parent_id.as_deref() == parent).map(|node| Ok(json::json!({"id":node.id,"name":node.name,"description":node.description,"examples":node.examples,"parent_id":node.parent_id,"item_count":counts.get(&node.id).copied().unwrap_or(0),"children":build(counts,nodes,Some(&node.id))?}))).collect()
        }
        Ok(json::json!({"tree": build(&counts, nodes, parent)?}))
    }
}

/// Seed taxonomy gate: unique ids, existing parents, no cycles, no sibling name dup.
pub fn validate_taxonomy(nodes: &[Node]) -> Result<(), String> {
    use std::collections::{HashMap, HashSet};
    let mut ids = HashSet::new();
    for node in nodes {
        if node.id.trim().is_empty() {
            return Err("empty node id".into());
        }
        if !ids.insert(node.id.clone()) {
            return Err(format!("duplicate node id: {}", node.id));
        }
        if node.name.trim().is_empty() {
            return Err(format!("empty name: {}", node.id));
        }
        if node.description.trim().is_empty() {
            return Err(format!("empty description: {}", node.id));
        }
    }
    let by_id: HashMap<&str, &Node> = nodes.iter().map(|node| (node.id.as_str(), node)).collect();
    for node in nodes {
        if let Some(parent) = node.parent_id.as_deref() {
            if !by_id.contains_key(parent) {
                return Err(format!("missing parent {parent} for {}", node.id));
            }
            let mut cursor = parent;
            let mut seen = HashSet::from([node.id.as_str()]);
            while let Some(current) = by_id.get(cursor) {
                let Some(next) = current.parent_id.as_deref() else {
                    break;
                };
                if !seen.insert(next) {
                    return Err(format!("taxonomy cycle at {}", node.id));
                }
                cursor = next;
            }
        }
    }
    let mut siblings: HashMap<Option<&str>, HashSet<&str>> = HashMap::new();
    for node in nodes {
        let entry = siblings.entry(node.parent_id.as_deref()).or_default();
        if !entry.insert(node.name.as_str()) {
            return Err(format!("duplicate sibling name: {}", node.name));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Node;

    fn node(id: &str, name: &str, parent: Option<&str>) -> Node {
        Node {
            id: id.into(),
            name: name.into(),
            description: format!("{name} description"),
            examples: Vec::new(),
            parent_id: parent.map(str::to_string),
        }
    }

    #[test]
    fn taxonomy_rejects_duplicates_cycles_and_missing_parents() {
        assert!(
            validate_taxonomy(&[
                node("root", "Root", None),
                node("child", "Child", Some("root")),
            ])
            .is_ok()
        );
        assert!(
            validate_taxonomy(&[node("root", "Root", None), node("root", "Other", None),]).is_err()
        );
        assert!(validate_taxonomy(&[node("orphan", "Orphan", Some("missing"))]).is_err());
        assert!(
            validate_taxonomy(&[node("a", "A", Some("b")), node("b", "B", Some("a")),]).is_err()
        );
        assert!(
            validate_taxonomy(&[
                node("root", "Root", None),
                node("x", "Same", Some("root")),
                node("y", "Same", Some("root")),
            ])
            .is_err()
        );
    }
}

fn item_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Item> {
    Ok(Item {
        id: row.get(0)?,
        category_id: row.get(1)?,
        question: row.get(2)?,
        answer: row.get(3)?,
        kind: row.get(4)?,
        status: row.get(5)?,
        version: row.get(6)?,
    })
}

mod json {
    pub use serde_json::json;
}
