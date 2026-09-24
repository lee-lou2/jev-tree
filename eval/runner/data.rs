//! Golden dataset: the knowledge-base fixture, its structural variants, the golden
//! cases, and the checks that keep them consistent.

use jev_tree::models::Node;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

pub const CASE_TYPES: &[&str] = &["search", "ingest", "contract"];
pub const STYLES: &[&str] = &[
    "canonical",
    "paraphrase",
    "colloquial",
    "keyword",
    "long",
    "typo",
    "english",
    "mixed",
    "negation",
    "multi_intent",
    "vague",
    "condition",
    "injection",
    "oos_near",
    "oos_far",
];
pub const CTXS: &[&str] = &[
    "coref", "ellipsis", "refine", "slot", "shift", "distract", "correct", "disambig", "long",
];
pub const DIFFS: &[&str] = &["easy", "medium", "hard"];

// ---------------------------------------------------------------------------
// Knowledge base fixture
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KbNode {
    pub id: String,
    pub parent_id: Option<String>,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub examples: Vec<String>,
    /// `core` (kept in `standard`) or `fine` (only in `deep`). Intermediate nodes only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KbItem {
    /// Stable reference used by golden cases: `<category_id>#<suffix>`.
    #[serde(rename = "ref")]
    pub key: String,
    pub category_id: String,
    pub kind: String,
    pub question: String,
    pub answer: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VariantSpec {
    pub keep_tiers: Vec<String>,
    pub keep_roots: bool,
    /// Keep only the first N qa items of every leaf (the "few items" variant).
    #[serde(default)]
    pub leaf_items: Option<usize>,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Manifest {
    pub name: String,
    #[serde(default)]
    pub title: String,
    pub version: String,
    pub domains: BTreeMap<String, String>,
    pub variants: BTreeMap<String, VariantSpec>,
    #[serde(default)]
    pub default_variant: String,
    #[serde(default)]
    pub empty_leaves: Vec<String>,
    #[serde(default)]
    pub dense_leaves: Vec<String>,
}

pub struct Kb {
    pub manifest: Manifest,
    pub nodes: Vec<KbNode>,
    pub items: Vec<KbItem>,
    pub item_src: Vec<String>,
    pub hash: String,
    /// Lines that did not parse. Reported by `validate`; the rest still loads.
    pub load_errors: Vec<String>,
    by_id: HashMap<String, usize>,
    children: HashMap<Option<String>, Vec<String>>,
}

fn read_text(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))
}

fn jsonl_files(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    for entry in entries {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.is_dir() {
            jsonl_files(&path, out)?;
        } else if path.extension().is_some_and(|ext| ext == "jsonl") {
            out.push(path);
        }
    }
    out.sort();
    Ok(())
}

fn display_path(path: &Path) -> String {
    let text = path.display().to_string();
    match text.find("eval/") {
        Some(at) => text[at..].to_string(),
        None => text,
    }
}

impl Kb {
    pub fn load(dir: &Path) -> Result<Kb, String> {
        let mut hasher = Sha256::new();
        let manifest_text = read_text(&dir.join("manifest.json"))?;
        hasher.update(manifest_text.as_bytes());
        let manifest: Manifest =
            serde_json::from_str(&manifest_text).map_err(|e| format!("manifest.json: {e}"))?;
        let nodes_text = read_text(&dir.join("nodes.json"))?;
        hasher.update(nodes_text.as_bytes());
        #[derive(Deserialize)]
        struct NodesFile {
            nodes: Vec<KbNode>,
        }
        let nodes = serde_json::from_str::<NodesFile>(&nodes_text)
            .map_err(|e| format!("nodes.json: {e}"))?
            .nodes;
        let mut files = Vec::new();
        let items_dir = dir.join("items");
        if items_dir.exists() {
            jsonl_files(&items_dir, &mut files)?;
        }
        let mut items = Vec::new();
        let mut item_src = Vec::new();
        let mut load_errors = Vec::new();
        for file in files {
            let text = read_text(&file)?;
            hasher.update(text.as_bytes());
            for (index, line) in text.lines().enumerate() {
                if line.trim().is_empty() {
                    continue;
                }
                let at = format!("{}:{}", display_path(&file), index + 1);
                match serde_json::from_str::<KbItem>(line) {
                    Ok(item) => {
                        items.push(item);
                        item_src.push(at);
                    }
                    Err(error) => load_errors.push(format!("{at}: {error}")),
                }
            }
        }
        let by_id = nodes
            .iter()
            .enumerate()
            .map(|(index, node)| (node.id.clone(), index))
            .collect();
        let mut children: HashMap<Option<String>, Vec<String>> = HashMap::new();
        for node in &nodes {
            children
                .entry(node.parent_id.clone())
                .or_default()
                .push(node.id.clone());
        }
        let digest = hasher.finalize();
        let hash = digest.iter().take(6).map(|b| format!("{b:02x}")).collect();
        Ok(Kb {
            manifest,
            nodes,
            items,
            item_src,
            hash,
            load_errors,
            by_id,
            children,
        })
    }

    pub fn node(&self, id: &str) -> Option<&KbNode> {
        self.by_id.get(id).map(|index| &self.nodes[*index])
    }

    pub fn has(&self, id: &str) -> bool {
        self.by_id.contains_key(id)
    }

    pub fn kids(&self, id: Option<&str>) -> &[String] {
        self.children
            .get(&id.map(str::to_string))
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn is_leaf(&self, id: &str) -> bool {
        self.kids(Some(id)).is_empty()
    }

    /// Master ancestors, top-level first, including `id`.
    pub fn chain(&self, id: &str) -> Vec<String> {
        let mut chain = Vec::new();
        let mut cursor = Some(id.to_string());
        while let Some(current) = cursor {
            if chain.len() > self.nodes.len() {
                break;
            }
            cursor = self.node(&current).and_then(|node| node.parent_id.clone());
            chain.push(current);
        }
        chain.reverse();
        chain
    }

    /// Top-level domain of a node in the master tree.
    pub fn domain(&self, id: &str) -> String {
        self.chain(id).first().cloned().unwrap_or_default()
    }

    pub fn is_ancestor_or_self(&self, ancestor: &str, id: &str) -> bool {
        self.chain(id).iter().any(|node| node == ancestor)
    }

    pub fn item(&self, key: &str) -> Option<&KbItem> {
        self.items.iter().find(|item| item.key == key)
    }
}

// ---------------------------------------------------------------------------
// Structural variants (same items, different depth)
// ---------------------------------------------------------------------------

pub struct Variant {
    pub name: String,
    /// Items were thinned out on purpose, so a missing answer means "abstain".
    pub thinned: bool,
    pub nodes: Vec<Node>,
    pub items: Vec<KbItem>,
    parent: HashMap<String, Option<String>>,
    children: HashMap<Option<String>, Vec<String>>,
    pub ref_to_id: HashMap<String, String>,
    pub id_to_ref: HashMap<String, String>,
    direct_items: HashMap<String, usize>,
}

impl Variant {
    pub fn build(kb: &Kb, name: &str) -> Result<Variant, String> {
        let spec = kb
            .manifest
            .variants
            .get(name)
            .ok_or_else(|| format!("unknown variant: {name}"))?;
        let keep = |node: &KbNode| -> bool {
            if kb.is_leaf(&node.id) {
                true
            } else if node.parent_id.is_none() {
                spec.keep_roots
            } else {
                node.tier
                    .as_deref()
                    .is_some_and(|tier| spec.keep_tiers.iter().any(|keep| keep == tier))
            }
        };
        let kept: HashSet<&str> = kb
            .nodes
            .iter()
            .filter(|node| keep(node))
            .map(|node| node.id.as_str())
            .collect();
        let mut nodes = Vec::new();
        for node in &kb.nodes {
            if !kept.contains(node.id.as_str()) {
                continue;
            }
            let mut parent = node.parent_id.clone();
            while let Some(id) = parent.clone() {
                if kept.contains(id.as_str()) {
                    break;
                }
                parent = kb.node(&id).and_then(|up| up.parent_id.clone());
            }
            nodes.push(Node {
                id: node.id.clone(),
                name: node.name.clone(),
                description: node.description.clone(),
                examples: node.examples.clone(),
                parent_id: parent,
            });
        }
        jev_tree::store::validate_taxonomy(&nodes)
            .map_err(|detail| format!("variant {name}: {detail}"))?;
        let mut per_leaf: HashMap<&str, usize> = HashMap::new();
        let items: Vec<KbItem> = kb
            .items
            .iter()
            .filter(|item| kept.contains(item.category_id.as_str()))
            .filter(|item| match spec.leaf_items {
                Some(cap) if kb.is_leaf(&item.category_id) => {
                    let seen = per_leaf.entry(item.category_id.as_str()).or_default();
                    *seen += 1;
                    *seen <= cap
                }
                _ => true,
            })
            .cloned()
            .collect();
        let mut parent = HashMap::new();
        let mut children: HashMap<Option<String>, Vec<String>> = HashMap::new();
        for node in &nodes {
            parent.insert(node.id.clone(), node.parent_id.clone());
            children
                .entry(node.parent_id.clone())
                .or_default()
                .push(node.id.clone());
        }
        let mut ref_to_id = HashMap::new();
        let mut id_to_ref = HashMap::new();
        let mut direct_items: HashMap<String, usize> = HashMap::new();
        for (index, item) in items.iter().enumerate() {
            // `Store::reset` numbers seed rows from 1 in file order.
            let id = (index + 1).to_string();
            ref_to_id.insert(item.key.clone(), id.clone());
            id_to_ref.insert(id, item.key.clone());
            *direct_items.entry(item.category_id.clone()).or_default() += 1;
        }
        Ok(Variant {
            name: name.to_string(),
            thinned: spec.leaf_items.is_some(),
            nodes,
            items,
            parent,
            children,
            ref_to_id,
            id_to_ref,
            direct_items,
        })
    }

    pub fn has(&self, id: &str) -> bool {
        self.parent.contains_key(id)
    }

    pub fn kids(&self, id: Option<&str>) -> &[String] {
        self.children
            .get(&id.map(str::to_string))
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn is_leaf(&self, id: &str) -> bool {
        self.kids(Some(id)).is_empty()
    }

    /// Ancestors in this variant, top-level first, including `id`.
    pub fn chain(&self, id: &str) -> Vec<String> {
        let mut chain = Vec::new();
        let mut cursor = Some(id.to_string());
        while let Some(current) = cursor {
            if chain.len() > self.nodes.len() || !self.has(&current) {
                break;
            }
            cursor = self.parent.get(&current).cloned().flatten();
            chain.push(current);
        }
        chain.reverse();
        chain
    }

    pub fn depth(&self, id: &str) -> usize {
        self.chain(id).len()
    }

    /// `root` value for `POST /api/run`: slash-separated ids from the forest down.
    pub fn root_path(&self, id: &str) -> String {
        self.chain(id).join("/")
    }

    pub fn subtree(&self, id: &str) -> Vec<String> {
        let mut out = vec![id.to_string()];
        let mut cursor = 0;
        while cursor < out.len() {
            let next: Vec<String> = self.kids(Some(&out[cursor])).to_vec();
            out.extend(next);
            cursor += 1;
        }
        out
    }

    /// Active items Jev ranks when search lands on `id`.
    pub fn pool(&self, id: &str) -> usize {
        self.subtree(id)
            .iter()
            .map(|node| self.direct_items.get(node).copied().unwrap_or(0))
            .sum()
    }

    pub fn max_depth(&self) -> usize {
        self.nodes
            .iter()
            .map(|node| self.depth(&node.id))
            .max()
            .unwrap_or(0)
    }

    pub fn seed_json(&self) -> Value {
        json!({
            "nodes": self.nodes,
            "items": self.items.iter().map(|item| json!({
                "ref": item.key,
                "category_id": item.category_id,
                "kind": item.kind,
                "question": item.question,
                "answer": item.answer,
            })).collect::<Vec<_>>(),
        })
    }
}

// ---------------------------------------------------------------------------
// Golden cases
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CaseInput {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub query: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub question: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub answer: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub context: Vec<Value>,
    /// Virtual root by node id. The runner turns it into the variant's path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_node: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_node: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub beam_width: Option<usize>,
    /// Contract cases only: raw request fields laid over the built request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw: Option<Value>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Expect {
    /// Expected category. `null` means "outside the tree" (or outside `root_node`).
    #[serde(default)]
    pub node: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub also_ok: Vec<String>,
    /// Graded relevance by item ref: 2 = answers it, 1 = useful but partial.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub items: BTreeMap<String, u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answerable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub duplicates: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updates: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Probe {
    pub query: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub context: Vec<Value>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ctx: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diff: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Case {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub split: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<String>>,
    pub input: CaseInput,
    #[serde(default)]
    pub expect: Expect,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probe: Option<Probe>,
    #[serde(default)]
    pub tags: Tags,
    #[serde(default)]
    pub note: String,
    #[serde(skip)]
    pub source: String,
}

impl Case {
    pub fn answerable(&self) -> bool {
        self.expect
            .answerable
            .unwrap_or(!self.expect.items.is_empty())
    }

    /// `dev` or `test`. Unless a case pins it, about 30% land in `test` by a hash of the id.
    pub fn split(&self) -> String {
        if let Some(split) = &self.split {
            return split.clone();
        }
        let digest = Sha256::digest(self.id.as_bytes());
        if digest[0] % 10 < 3 {
            "test".into()
        } else {
            "dev".into()
        }
    }

    pub fn turns(&self) -> &'static str {
        if self.input.context.is_empty() {
            "single"
        } else {
            "multi"
        }
    }

    pub fn entry(&self) -> &'static str {
        if self.input.start_node.is_some() {
            "start"
        } else if self.input.root_node.is_some() {
            "root"
        } else {
            "forest"
        }
    }
}

/// Golden cases from every `*.jsonl` under `dir`, plus the lines that did not parse.
pub fn load_cases(dir: &Path) -> Result<(Vec<Case>, Vec<String>), String> {
    let mut files = Vec::new();
    jsonl_files(dir, &mut files)?;
    let mut cases = Vec::new();
    let mut errors = Vec::new();
    for file in files {
        let text = read_text(&file)?;
        for (index, line) in text.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            let at = format!("{}:{}", display_path(&file), index + 1);
            match serde_json::from_str::<Case>(line) {
                Ok(mut case) => {
                    case.source = at;
                    cases.push(case);
                }
                Err(error) => errors.push(format!("{at}: {error}")),
            }
        }
    }
    Ok((cases, errors))
}

pub fn cases_hash(dir: &Path) -> String {
    let mut files = Vec::new();
    if jsonl_files(dir, &mut files).is_err() {
        return "unknown".into();
    }
    let mut hasher = Sha256::new();
    for file in files {
        if let Ok(text) = fs::read_to_string(&file) {
            hasher.update(text.as_bytes());
        }
    }
    hasher
        .finalize()
        .iter()
        .take(6)
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// A case bound to one variant: node ids that exist there, item ids, and derived facets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolved {
    pub root_path: Option<String>,
    pub start_node: Option<String>,
    pub node: Option<String>,
    pub accept: Vec<String>,
    /// Item id -> grade.
    pub grades: BTreeMap<String, u8>,
    pub answerable: bool,
    pub domain: String,
    pub level: String,
    pub depth: usize,
    pub pool: usize,
    pub duplicate_ids: Vec<String>,
    pub update_id: Option<String>,
    /// The variant removed every answer item, so the case now expects an abstain.
    #[serde(default)]
    pub dropped: bool,
    /// Contract cases: the error code the request must fail with.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

pub fn pool_bucket(pool: usize) -> &'static str {
    match pool {
        0 => "0",
        1 => "1",
        2..=5 => "2-5",
        6..=32 => "6-32",
        33..=100 => "33-100",
        _ => "101+",
    }
}

/// Bind a case to a variant, or say why it does not apply there.
pub fn resolve(kb: &Kb, variant: &Variant, case: &Case) -> Result<Resolved, String> {
    if let Some(allowed) = &case.variants
        && !allowed.iter().any(|name| name == &variant.name)
    {
        return Err("variant not listed".into());
    }
    let root_path = match &case.input.root_node {
        Some(id) if !variant.has(id) => return Err(format!("root_node {id} absent")),
        Some(id) => Some(variant.root_path(id)),
        None => None,
    };
    if let Some(id) = &case.input.start_node
        && !variant.has(id)
    {
        return Err(format!("start_node {id} absent"));
    }
    if let Some(id) = &case.expect.node
        && !variant.has(id)
    {
        return Err(format!("node {id} absent"));
    }
    let node = case.expect.node.clone();
    let mut accept: Vec<String> = node.iter().cloned().collect();
    for id in &case.expect.also_ok {
        if variant.has(id) && !accept.contains(id) {
            accept.push(id.clone());
        }
    }
    let mut grades = BTreeMap::new();
    for (key, grade) in &case.expect.items {
        if let Some(id) = variant.ref_to_id.get(key) {
            grades.insert(id.clone(), *grade);
        }
    }
    let mut answerable = case.answerable();
    let mut dropped = false;
    if case.kind == "search" && answerable && !grades.values().any(|grade| *grade == 2) {
        if !variant.thinned {
            return Err("no grade-2 item left".into());
        }
        // The leaf is still there but its answer was thinned out: saying "I don't know"
        // is now the right outcome.
        answerable = false;
        dropped = true;
        grades.clear();
    }
    let mut duplicate_ids = Vec::new();
    for key in &case.expect.duplicates {
        match variant.ref_to_id.get(key) {
            Some(id) => duplicate_ids.push(id.clone()),
            None => return Err(format!("duplicate {key} absent")),
        }
    }
    let update_id = match &case.expect.updates {
        Some(key) => match variant.ref_to_id.get(key) {
            Some(id) => Some(id.clone()),
            None => return Err(format!("updates {key} absent")),
        },
        None => None,
    };
    let (level, depth, pool) = match &node {
        Some(id) if variant.is_leaf(id) => ("leaf", variant.depth(id), variant.pool(id)),
        Some(id) => ("internal", variant.depth(id), variant.pool(id)),
        None => ("none", 0, 0),
    };
    Ok(Resolved {
        root_path,
        start_node: case.input.start_node.clone(),
        node: node.clone(),
        accept,
        grades,
        answerable,
        domain: node
            .as_deref()
            .map(|id| kb.domain(id))
            .unwrap_or_else(|| "oos".into()),
        level: level.into(),
        depth,
        pool,
        duplicate_ids,
        update_id,
        dropped,
        error: case.expect.error.clone(),
    })
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

#[derive(Default)]
pub struct Findings {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl Findings {
    fn error(&mut self, text: String) {
        self.errors.push(text);
    }
    fn warn(&mut self, text: String) {
        self.warnings.push(text);
    }
}

fn is_snake(id: &str) -> bool {
    !id.is_empty()
        && id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

pub fn validate_kb(kb: &Kb, out: &mut Findings) {
    for error in &kb.load_errors {
        out.error(error.clone());
    }
    let mut seen = HashSet::new();
    for node in &kb.nodes {
        if !seen.insert(node.id.as_str()) {
            out.error(format!("node {}: duplicate id", node.id));
        }
        if !is_snake(&node.id) {
            out.error(format!("node {}: id must be snake_case", node.id));
        }
        if let Some(parent) = &node.parent_id {
            if !kb.has(parent) {
                out.error(format!("node {}: missing parent {parent}", node.id));
            } else if !node.id.starts_with(&format!("{parent}_")) {
                out.warn(format!(
                    "node {}: id does not extend parent {parent}",
                    node.id
                ));
            }
        }
        let leaf = kb.is_leaf(&node.id);
        let root = node.parent_id.is_none();
        match (&node.tier, leaf || root) {
            (Some(tier), true) => out.error(format!(
                "node {}: tier {tier} is only for intermediate nodes",
                node.id
            )),
            (None, false) => out.error(format!("node {}: intermediate node needs a tier", node.id)),
            (Some(tier), false) if tier != "core" && tier != "fine" => {
                out.error(format!("node {}: tier must be core or fine", node.id))
            }
            _ => {}
        }
        if node.description.trim().is_empty() {
            out.error(format!("node {}: empty description", node.id));
        }
        if node.description.chars().count() > 180 {
            out.warn(format!(
                "node {}: description over 180 characters is clipped by the LLM router",
                node.id
            ));
        }
        if node.examples.is_empty() {
            out.warn(format!("node {}: no examples", node.id));
        }
    }
    let mut names = HashMap::new();
    for node in &kb.nodes {
        if let Some(other) = names.insert(node.name.as_str(), node.id.as_str()) {
            out.error(format!(
                "node {}: name '{}' also used by {other}; flat variants need unique names",
                node.id, node.name
            ));
        }
    }
    for id in kb
        .manifest
        .empty_leaves
        .iter()
        .chain(&kb.manifest.dense_leaves)
    {
        if !kb.has(id) || !kb.is_leaf(id) {
            out.error(format!("manifest: {id} is not a leaf"));
        }
    }
    for root in kb.kids(None) {
        if !kb.manifest.domains.contains_key(root) {
            out.error(format!("manifest: domain code missing for {root}"));
        }
    }
    // Items
    let mut keys = HashSet::new();
    let mut questions: HashMap<String, String> = HashMap::new();
    let mut per_category: HashMap<&str, (usize, usize)> = HashMap::new();
    for (item, at) in kb.items.iter().zip(&kb.item_src) {
        if !keys.insert(item.key.as_str()) {
            out.error(format!("{at}: duplicate ref {}", item.key));
        }
        if !kb.has(&item.category_id) {
            out.error(format!("{at}: unknown category {}", item.category_id));
            continue;
        }
        let prefix = format!("{}#", item.category_id);
        if !item.key.starts_with(&prefix) || item.key.len() == prefix.len() {
            out.error(format!("{at}: ref must look like {prefix}<suffix>"));
        }
        if item.kind != "qa" && item.kind != "article" {
            out.error(format!("{at}: kind must be qa or article"));
        }
        if item.question.trim().is_empty() || item.answer.trim().is_empty() {
            out.error(format!("{at}: empty question or answer"));
        }
        if item.question.chars().count() > 240 {
            out.warn(format!("{at}: question over 240 characters"));
        }
        if let Some(other) = questions.insert(item.question.trim().to_string(), item.key.clone()) {
            out.error(format!(
                "{at}: question duplicates {other}; upsert and mapping need unique questions"
            ));
        }
        let entry = per_category.entry(item.category_id.as_str()).or_default();
        if item.kind == "article" {
            entry.1 += 1;
        } else {
            entry.0 += 1;
        }
    }
    for node in &kb.nodes {
        let (qa, articles) = per_category
            .get(node.id.as_str())
            .copied()
            .unwrap_or_default();
        let empty = kb.manifest.empty_leaves.contains(&node.id);
        if kb.is_leaf(&node.id) {
            if empty && qa + articles > 0 {
                out.error(format!("node {}: declared empty but has items", node.id));
            }
            if !empty && qa == 0 {
                out.warn(format!("node {}: leaf without qa items", node.id));
            }
            if kb.manifest.dense_leaves.contains(&node.id) && qa < 40 {
                out.warn(format!("node {}: dense leaf has only {qa} items", node.id));
            }
        } else {
            if articles != 1 {
                out.warn(format!(
                    "node {}: intermediate node should have exactly one article (has {articles})",
                    node.id
                ));
            }
            if qa > 0 {
                out.warn(format!(
                    "node {}: qa items on an intermediate node",
                    node.id
                ));
            }
        }
    }
    for name in kb.manifest.variants.keys() {
        if let Err(error) = Variant::build(kb, name) {
            out.error(error);
        }
    }
}

fn context_ok(context: &[Value]) -> Result<(), String> {
    if context.len() > 32 {
        return Err("more than 32 context turns".into());
    }
    for turn in context {
        match turn {
            Value::String(text) if !text.trim().is_empty() => {}
            Value::Object(map) => {
                let role = map.get("role").and_then(Value::as_str).unwrap_or("");
                let text = map.get("text").and_then(Value::as_str).unwrap_or("");
                if role != "user" && role != "assistant" {
                    return Err(format!("context role must be user or assistant: {role}"));
                }
                if text.trim().is_empty() {
                    return Err("context turn without text".into());
                }
            }
            other => return Err(format!("unsupported context turn: {other}")),
        }
    }
    Ok(())
}

pub fn validate_cases(kb: &Kb, cases: &[Case], out: &mut Findings) {
    let mut ids = HashSet::new();
    let item_questions: HashSet<&str> = kb.items.iter().map(|i| i.question.trim()).collect();
    let examples: HashSet<&str> = kb
        .nodes
        .iter()
        .flat_map(|node| node.examples.iter().map(|e| e.trim()))
        .collect();
    let domain_codes: HashSet<&str> = kb
        .manifest
        .domains
        .values()
        .map(String::as_str)
        .chain(["XD", "OOS", "API", "GAP"])
        .collect();
    for case in cases {
        let at = format!("{} ({})", case.source, case.id);
        if !ids.insert(case.id.as_str()) {
            out.error(format!("{at}: duplicate id"));
        }
        if !CASE_TYPES.contains(&case.kind.as_str()) {
            out.error(format!("{at}: type must be search, ingest, or contract"));
            continue;
        }
        let parts: Vec<&str> = case.id.split('-').collect();
        let prefix = match case.kind.as_str() {
            "search" => "S",
            "ingest" => "I",
            _ => "C",
        };
        if parts.len() != 3
            || parts[0] != prefix
            || !domain_codes.contains(parts[1])
            || parts[2].len() != 3
            || !parts[2].chars().all(|c| c.is_ascii_digit())
        {
            out.error(format!(
                "{at}: id must be {prefix}-<DOMAIN|XD|OOS|API|GAP>-<NNN>"
            ));
        }
        if let Some(split) = &case.split
            && split != "dev"
            && split != "test"
        {
            out.error(format!("{at}: split must be dev or test"));
        }
        if let Some(variants) = &case.variants {
            for name in variants {
                if !kb.manifest.variants.contains_key(name) {
                    out.error(format!("{at}: unknown variant {name}"));
                }
            }
        }
        // Tags
        match &case.tags.style {
            Some(style) if !STYLES.contains(&style.as_str()) => {
                out.error(format!("{at}: unknown style {style}"))
            }
            None if case.kind != "contract" => out.error(format!("{at}: tags.style is required")),
            _ => {}
        }
        match &case.tags.diff {
            Some(diff) if !DIFFS.contains(&diff.as_str()) => {
                out.error(format!("{at}: unknown diff {diff}"))
            }
            None if case.kind != "contract" => out.error(format!("{at}: tags.diff is required")),
            _ => {}
        }
        // Contract cases carry deliberately invalid input; only their expected error matters.
        let multi = !case.input.context.is_empty() && case.kind != "contract";
        match (&case.tags.ctx, multi) {
            (Some(ctx), true) if !CTXS.contains(&ctx.as_str()) => {
                out.error(format!("{at}: unknown ctx {ctx}"))
            }
            (None, true) => out.error(format!("{at}: multi-turn case needs tags.ctx")),
            (Some(_), false) => out.error(format!("{at}: tags.ctx without context")),
            _ => {}
        }
        if case.kind != "contract"
            && let Err(error) = context_ok(&case.input.context)
        {
            out.error(format!("{at}: {error}"));
        }
        // Node references
        let node_ok = |id: &str, what: &str, out: &mut Findings| -> bool {
            if kb.has(id) {
                true
            } else {
                out.error(format!("{at}: {what} {id} does not exist"));
                false
            }
        };
        let expected = case.expect.node.clone();
        if let Some(id) = &expected {
            node_ok(id, "expect.node", out);
        }
        for id in &case.expect.also_ok {
            if node_ok(id, "also_ok", out) && Some(id) == expected.as_ref() {
                out.error(format!("{at}: also_ok repeats expect.node"));
            }
        }
        if let Some(root) = &case.input.root_node
            && node_ok(root, "root_node", out)
            && let Some(id) = &expected
            && kb.has(id)
            && !kb.is_ancestor_or_self(root, id)
        {
            out.error(format!(
                "{at}: expect.node {id} is outside root_node {root}; use null for out-of-root"
            ));
        }
        if let Some(start) = &case.input.start_node
            && node_ok(start, "start_node", out)
        {
            match &expected {
                Some(id) if kb.has(id) && !kb.is_ancestor_or_self(start, id) => out.error(format!(
                    "{at}: expect.node {id} is outside start_node {start}"
                )),
                None if case.kind != "contract" => out.error(format!(
                    "{at}: start_node cannot abstain; expect the start node with answerable=false"
                )),
                _ => {}
            }
        }
        match case.kind.as_str() {
            "search" => validate_search(kb, case, &at, &item_questions, &examples, out),
            "ingest" => validate_ingest(kb, case, &at, out),
            _ => {
                if case.expect.error.is_none() {
                    out.error(format!("{at}: contract case needs expect.error"));
                }
            }
        }
    }
}

fn validate_search(
    kb: &Kb,
    case: &Case,
    at: &str,
    item_questions: &HashSet<&str>,
    examples: &HashSet<&str>,
    out: &mut Findings,
) {
    let query = case.input.query.trim();
    if query.is_empty() {
        out.error(format!("{at}: search needs input.query"));
    }
    if !case.input.question.is_empty() || !case.input.answer.is_empty() {
        out.error(format!("{at}: search cases use input.query only"));
    }
    let style = case.tags.style.as_deref().unwrap_or("");
    if style != "canonical" && item_questions.contains(query) {
        out.warn(format!(
            "{at}: query copies an item question; tag it canonical or rephrase"
        ));
    }
    if examples.contains(query) {
        out.warn(format!("{at}: query copies a node example"));
    }
    let answerable = case.answerable();
    let mut grade2 = 0;
    for (key, grade) in &case.expect.items {
        if *grade != 1 && *grade != 2 {
            out.error(format!("{at}: grade for {key} must be 1 or 2"));
        }
        let Some(item) = kb.item(key) else {
            out.error(format!("{at}: unknown item ref {key}"));
            continue;
        };
        if *grade == 2 {
            grade2 += 1;
        }
        // Ranking only sees the landed subtree, so a relevant item must live under
        // an acceptable landing node.
        let reachable = case
            .expect
            .node
            .iter()
            .chain(&case.expect.also_ok)
            .any(|node| kb.is_ancestor_or_self(node, &item.category_id));
        if !reachable {
            out.error(format!(
                "{at}: item {key} is not under expect.node or also_ok"
            ));
        }
    }
    if case.expect.node.is_none() {
        if case.expect.answerable != Some(false) {
            out.error(format!("{at}: node null needs answerable=false"));
        }
        if !case.expect.items.is_empty() || !case.expect.also_ok.is_empty() {
            out.error(format!("{at}: node null cannot list items or also_ok"));
        }
    } else if answerable && grade2 == 0 {
        out.error(format!("{at}: answerable case needs a grade-2 item"));
    } else if !answerable && !case.expect.items.is_empty() {
        out.error(format!("{at}: answerable=false cannot list items"));
    }
    if let Some(node) = &case.expect.node
        && kb.manifest.empty_leaves.contains(node)
        && answerable
    {
        out.error(format!(
            "{at}: {node} is an empty leaf; set answerable=false"
        ));
    }
    let oos = matches!(style, "oos_near" | "oos_far");
    if oos && case.expect.node.is_some() && case.input.start_node.is_none() {
        out.warn(format!("{at}: oos style but expect.node is set"));
    }
    if case.input.raw.is_some() {
        out.error(format!("{at}: raw is for contract cases"));
    }
    if case.probe.is_some() || !case.expect.duplicates.is_empty() || case.expect.updates.is_some() {
        out.error(format!("{at}: probe/duplicates/updates are ingest fields"));
    }
}

/// Character 3-grams of the alphanumeric content. Mirrors `engine::near_duplicate`,
/// so duplicate labels can be checked without a model.
fn trigrams(text: &str) -> HashSet<String> {
    let compact: Vec<char> = text
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect();
    if compact.len() < 3 {
        return HashSet::new();
    }
    (0..=compact.len() - 3)
        .map(|i| compact[i..i + 3].iter().collect())
        .collect()
}

pub fn near_duplicate(question: &str, existing: &str) -> bool {
    let (a, b) = (trigrams(question), trigrams(existing));
    if a.is_empty() || b.is_empty() {
        return question.trim() == existing.trim();
    }
    a.intersection(&b).count() as f64 / a.len().min(b.len()) as f64 >= 0.6
}

/// Refs the engine would report in `duplicate_ids` if the question lands in `category`.
pub fn expected_duplicates(kb: &Kb, category: &str, question: &str) -> Vec<String> {
    kb.items
        .iter()
        .filter(|item| item.category_id == category && near_duplicate(question, &item.question))
        .take(5)
        .map(|item| item.key.clone())
        .collect()
}

fn validate_ingest(kb: &Kb, case: &Case, at: &str, out: &mut Findings) {
    if case.input.question.trim().is_empty() || case.input.answer.trim().is_empty() {
        out.error(format!(
            "{at}: ingest needs input.question and input.answer"
        ));
    }
    if let Some(node) = case.expect.node.as_deref()
        && kb.has(node)
    {
        let mut computed = expected_duplicates(kb, node, &case.input.question);
        let mut labelled = case.expect.duplicates.clone();
        computed.sort();
        labelled.sort();
        if computed != labelled {
            out.error(format!(
                "{at}: expect.duplicates must match the trigram rule in {node}: [{}]",
                computed.join(", ")
            ));
        }
    }
    if !case.input.query.is_empty() {
        out.error(format!("{at}: ingest cases use question/answer, not query"));
    }
    if !case.expect.items.is_empty() || case.expect.answerable.is_some() {
        out.error(format!("{at}: items/answerable are search fields"));
    }
    let node = case.expect.node.as_deref();
    for key in case.expect.duplicates.iter().chain(&case.expect.updates) {
        match (kb.item(key), node) {
            (None, _) => out.error(format!("{at}: unknown item ref {key}")),
            (Some(item), Some(node)) if item.category_id != node => out.error(format!(
                "{at}: {key} is in {}, but duplicates/updates only match inside {node}",
                item.category_id
            )),
            (Some(_), None) => out.error(format!("{at}: a rejected ingest cannot match {key}")),
            _ => {}
        }
    }
    if let Some(key) = &case.expect.updates
        && let Some(item) = kb.item(key)
        && item.question.trim() != case.input.question.trim()
    {
        out.error(format!(
            "{at}: updates {key} needs the exact same question text"
        ));
    }
    if let Some(probe) = &case.probe {
        if probe.query.trim().is_empty() {
            out.error(format!("{at}: probe.query is empty"));
        }
        if node.is_none() {
            out.error(format!("{at}: a rejected ingest has nothing to probe"));
        }
        if let Err(error) = context_ok(&probe.context) {
            out.error(format!("{at}: probe {error}"));
        }
    }
    if node.is_none() && !case.expect.also_ok.is_empty() {
        out.error(format!("{at}: node null cannot list also_ok"));
    }
}

// ---------------------------------------------------------------------------
// Stats
// ---------------------------------------------------------------------------

/// A named way to bucket cases in the stats tables.
type Facet<'a> = (&'static str, Box<dyn Fn(&Case) -> String + 'a>);

fn count_table(title: &str, counts: &BTreeMap<String, usize>, out: &mut String) {
    let total: usize = counts.values().sum();
    let _ = writeln!(out, "| {title} | cases | share |\n|---|---:|---:|");
    for (key, value) in counts {
        let share = if total == 0 {
            0.0
        } else {
            *value as f64 * 100.0 / total as f64
        };
        let _ = writeln!(out, "| {key} | {value} | {share:.0}% |");
    }
    let _ = writeln!(out);
}

pub fn stats_markdown(kb: &Kb, cases: &[Case]) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "# 골든 데이터셋 통계\n\n{} — `{}` v{} · kb hash `{}`\n\n`cargo run --release --example eval -- stats --out eval/golden/STATS.md`로 다시 만든다.\n",
        kb.manifest.title, kb.manifest.name, kb.manifest.version, kb.hash
    );
    let _ = writeln!(out, "## 지식베이스 변형\n");
    let _ = writeln!(
        out,
        "| variant | 설명 | nodes | leaves | max depth | depth histogram | items (qa/article) | leaves by item count (0/1/2-5/6+) | largest pool |\n|---|---|---:|---:|---:|---|---|---|---|"
    );
    for (name, spec) in &kb.manifest.variants {
        let Ok(variant) = Variant::build(kb, name) else {
            continue;
        };
        let mut depth_hist: BTreeMap<usize, usize> = BTreeMap::new();
        for node in &variant.nodes {
            *depth_hist.entry(variant.depth(&node.id)).or_default() += 1;
        }
        let leaves: Vec<&Node> = variant
            .nodes
            .iter()
            .filter(|node| variant.is_leaf(&node.id))
            .collect();
        let mut buckets = [0usize; 4];
        for leaf in &leaves {
            let count = variant.pool(&leaf.id);
            let slot = match count {
                0 => 0,
                1 => 1,
                2..=5 => 2,
                _ => 3,
            };
            buckets[slot] += 1;
        }
        let qa = variant.items.iter().filter(|i| i.kind == "qa").count();
        let largest = variant
            .nodes
            .iter()
            .map(|node| (variant.pool(&node.id), node.id.as_str()))
            .max()
            .map(|(pool, id)| format!("{id} ({pool})"))
            .unwrap_or_default();
        let hist = depth_hist
            .iter()
            .map(|(depth, count)| format!("d{depth}:{count}"))
            .collect::<Vec<_>>()
            .join(" ");
        let _ = writeln!(
            out,
            "| {name} | {} | {} | {} | {} | {hist} | {} ({qa}/{}) | {}/{}/{}/{} | {largest} |",
            spec.description,
            variant.nodes.len(),
            leaves.len(),
            variant.max_depth(),
            variant.items.len(),
            variant.items.len() - qa,
            buckets[0],
            buckets[1],
            buckets[2],
            buckets[3],
        );
    }
    let _ = writeln!(out);
    let _ = writeln!(out, "## 영역별 지식베이스 규모 (deep 기준)\n");
    let _ = writeln!(
        out,
        "| domain | nodes | leaves | items |\n|---|---:|---:|---:|"
    );
    for root in kb.kids(None) {
        let subtree: Vec<&KbNode> = kb
            .nodes
            .iter()
            .filter(|node| kb.domain(&node.id) == *root)
            .collect();
        let leaves = subtree.iter().filter(|n| kb.is_leaf(&n.id)).count();
        let items = kb
            .items
            .iter()
            .filter(|item| kb.domain(&item.category_id) == *root)
            .count();
        let _ = writeln!(out, "| {root} | {} | {leaves} | {items} |", subtree.len());
    }
    let _ = writeln!(out);
    let _ = writeln!(out, "## 케이스 구성\n");
    let facets: [Facet; 9] = [
        ("type", Box::new(|c: &Case| c.kind.clone())),
        ("split", Box::new(|c: &Case| c.split())),
        (
            "domain",
            Box::new(|c: &Case| {
                c.expect
                    .node
                    .as_deref()
                    .filter(|id| kb.has(id))
                    .map(|id| kb.domain(id))
                    .unwrap_or_else(|| {
                        if c.kind == "contract" {
                            "api".into()
                        } else {
                            "oos".into()
                        }
                    })
            }),
        ),
        ("turns", Box::new(|c: &Case| c.turns().to_string())),
        ("entry", Box::new(|c: &Case| c.entry().to_string())),
        (
            "style",
            Box::new(|c: &Case| c.tags.style.clone().unwrap_or_else(|| "-".into())),
        ),
        (
            "ctx",
            Box::new(|c: &Case| c.tags.ctx.clone().unwrap_or_else(|| "-".into())),
        ),
        (
            "diff",
            Box::new(|c: &Case| c.tags.diff.clone().unwrap_or_else(|| "-".into())),
        ),
        (
            "expect",
            Box::new(|c: &Case| match (c.kind.as_str(), &c.expect.node) {
                ("contract", _) => "error".into(),
                ("ingest", None) => "reject".into(),
                ("ingest", Some(_)) => "file".into(),
                (_, None) => "oos".into(),
                (_, Some(id)) if kb.manifest.empty_leaves.contains(id) => "empty_leaf".into(),
                (_, Some(_)) if !c.answerable() => "no_answer".into(),
                (_, Some(id)) if kb.has(id) && !kb.is_leaf(id) => "internal".into(),
                _ => "leaf".into(),
            }),
        ),
    ];
    let _ = writeln!(out, "총 {}건\n", cases.len());
    for (title, facet) in &facets {
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for case in cases {
            *counts.entry(facet(case)).or_default() += 1;
        }
        count_table(title, &counts, &mut out);
    }
    let _ = writeln!(out, "## 변형별 적용 케이스\n");
    let _ = writeln!(
        out,
        "| variant | search | ingest | contract | skipped |\n|---|---:|---:|---:|---:|"
    );
    for name in kb.manifest.variants.keys() {
        let Ok(variant) = Variant::build(kb, name) else {
            continue;
        };
        let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
        let mut skipped = 0;
        for case in cases {
            match resolve(kb, &variant, case) {
                Ok(_) => *counts.entry(case.kind.as_str()).or_default() += 1,
                Err(_) => skipped += 1,
            }
        }
        let _ = writeln!(
            out,
            "| {name} | {} | {} | {} | {skipped} |",
            counts.get("search").copied().unwrap_or(0),
            counts.get("ingest").copied().unwrap_or(0),
            counts.get("contract").copied().unwrap_or(0),
        );
    }
    let _ = writeln!(out);
    // Target depth and pool size in the default variant.
    let default = if kb.manifest.default_variant.is_empty() {
        "standard"
    } else {
        kb.manifest.default_variant.as_str()
    };
    if let Ok(variant) = Variant::build(kb, default) {
        let mut depth: BTreeMap<String, usize> = BTreeMap::new();
        let mut pool: BTreeMap<String, usize> = BTreeMap::new();
        for case in cases.iter().filter(|c| c.kind == "search") {
            if let Ok(resolved) = resolve(kb, &variant, case) {
                *depth.entry(format!("d{}", resolved.depth)).or_default() += 1;
                *pool
                    .entry(pool_bucket(resolved.pool).to_string())
                    .or_default() += 1;
            }
        }
        let _ = writeln!(out, "## 검색 케이스의 목표 깊이·후보 수 ({default})\n");
        count_table("target depth", &depth, &mut out);
        count_table("pool size", &pool, &mut out);
    }
    let covered: BTreeSet<&str> = cases
        .iter()
        .filter_map(|case| case.expect.node.as_deref())
        .collect();
    let leaves: Vec<&str> = kb
        .nodes
        .iter()
        .filter(|node| kb.is_leaf(&node.id))
        .map(|node| node.id.as_str())
        .collect();
    let missing: Vec<&str> = leaves
        .iter()
        .copied()
        .filter(|id| !covered.contains(id))
        .collect();
    let _ = writeln!(
        out,
        "## 리프 커버리지\n\n{} / {} 리프가 한 번 이상 기대 노드로 쓰였다.\n",
        leaves.len() - missing.len(),
        leaves.len()
    );
    if !missing.is_empty() {
        let _ = writeln!(out, "미사용 리프: {}\n", missing.join(", "));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("eval")
    }

    /// The committed golden set must stay valid: `cargo test --example eval`.
    #[test]
    fn committed_dataset_validates() {
        let kb = Kb::load(&root().join("fixtures/kb")).unwrap();
        let (cases, parse_errors) = load_cases(&root().join("golden")).unwrap();
        assert!(parse_errors.is_empty(), "{parse_errors:?}");
        let mut findings = Findings::default();
        validate_kb(&kb, &mut findings);
        validate_cases(&kb, &cases, &mut findings);
        assert!(findings.errors.is_empty(), "{:#?}", findings.errors);
    }

    #[test]
    fn variants_have_the_documented_shapes() {
        let kb = Kb::load(&root().join("fixtures/kb")).unwrap();
        let depth = |name: &str| Variant::build(&kb, name).unwrap().max_depth();
        assert_eq!(depth("deep"), 7);
        assert_eq!(depth("standard"), 4);
        assert_eq!(depth("shallow"), 2);
        assert_eq!(depth("flat"), 1);
        let sparse = Variant::build(&kb, "sparse").unwrap();
        assert!(sparse.thinned);
        for node in &sparse.nodes {
            if sparse.is_leaf(&node.id) {
                assert!(sparse.pool(&node.id) <= 1, "{}", node.id);
            }
        }
        // Collapsing keeps every leaf and every qa item.
        let deep = Variant::build(&kb, "deep").unwrap();
        let flat = Variant::build(&kb, "flat").unwrap();
        let qa = |v: &Variant| v.items.iter().filter(|i| i.kind == "qa").count();
        assert_eq!(qa(&deep), qa(&flat));
    }

    #[test]
    fn near_duplicate_follows_the_engine_rule() {
        assert!(near_duplicate(
            "와이파이가 몇 분마다 끊겼다가 다시 연결돼요.",
            "와이파이가 몇 분마다 끊겼다가 다시 연결돼요"
        ));
        assert!(!near_duplicate("공유기 초기화 방법", "카드 한도 올리는 법"));
        assert!(near_duplicate("ab", "ab"));
    }
}
