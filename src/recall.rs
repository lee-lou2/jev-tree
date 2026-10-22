//! Lexical recall over the taxonomy: BM25 on words and character bigrams.
//!
//! Only scopes too large for one Jev choice use it, to decide which nodes the
//! choice gets to see. Nothing is routed by word overlap: the evaluator still
//! makes every decision, this only builds its menu. Domain-free.
//!
//! A node's document is its path, description and examples plus the questions
//! of its active items. The items are where most of a node's vocabulary lives:
//! on the golden set's 219 nodes, node text alone puts the right node in the
//! top 20 for 82% of requests and adding item questions makes that 89%.

use crate::models::Node;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// The last index built, keyed by the store generation and taxonomy size it
/// was built from. Shared by every request on one state.
#[derive(Clone, Default)]
pub struct Cache(Arc<Mutex<Option<Built>>>);

struct Built {
    key: (u64, usize),
    index: Arc<NodeIndex>,
}

impl Cache {
    /// The cached index when `key` still matches, else a fresh one from `build`.
    pub fn get_or_build<E>(
        &self,
        key: (u64, usize),
        build: impl FnOnce() -> Result<NodeIndex, E>,
    ) -> Result<Arc<NodeIndex>, E> {
        if let Ok(guard) = self.0.lock()
            && let Some(built) = guard.as_ref()
            && built.key == key
        {
            return Ok(built.index.clone());
        }
        let index = Arc::new(build()?);
        if let Ok(mut guard) = self.0.lock() {
            *guard = Some(Built {
                key,
                index: index.clone(),
            });
        }
        Ok(index)
    }
}

const K1: f64 = 1.2;
const B: f64 = 0.75;

/// Words (2+ characters) and character bigrams of the alphanumeric content.
/// Bigrams carry Korean, where particles and endings change the word but not
/// the stem; words carry the exact terms.
pub fn terms(text: &str) -> Vec<String> {
    let lower = text.to_lowercase();
    let mut out: Vec<String> = lower
        .split(|c: char| !c.is_alphanumeric())
        .filter(|word| word.chars().count() > 1)
        .map(str::to_string)
        .collect();
    let compact: Vec<char> = lower.chars().filter(|c| c.is_alphanumeric()).collect();
    out.extend(
        compact
            .windows(2)
            .map(|pair| pair.iter().collect::<String>()),
    );
    out
}

/// BM25 over one document per node.
pub struct NodeIndex {
    ids: Vec<String>,
    lengths: Vec<f64>,
    average: f64,
    postings: HashMap<String, Vec<(usize, f64)>>,
    idf: HashMap<String, f64>,
}

impl NodeIndex {
    /// `questions` are `(category_id, question)` pairs of active items.
    pub fn build<'a>(
        nodes: &[Node],
        questions: impl IntoIterator<Item = (&'a str, &'a str)>,
    ) -> Self {
        let by_id: HashMap<&str, &Node> =
            nodes.iter().map(|node| (node.id.as_str(), node)).collect();
        let mut docs: Vec<String> = nodes
            .iter()
            .map(|node| {
                let mut names = Vec::new();
                let mut cursor = node.parent_id.as_deref();
                let mut guard = 0;
                while let Some(id) = cursor {
                    guard += 1;
                    let Some(parent) = by_id.get(id).filter(|_| guard <= nodes.len()) else {
                        break;
                    };
                    names.push(parent.name.as_str());
                    cursor = parent.parent_id.as_deref();
                }
                names.reverse();
                format!(
                    "{} {} {} {}",
                    names.join(" "),
                    node.name,
                    node.description,
                    node.examples.join(" ")
                )
            })
            .collect();
        let position: HashMap<&str, usize> = nodes
            .iter()
            .enumerate()
            .map(|(index, node)| (node.id.as_str(), index))
            .collect();
        for (category, question) in questions {
            if let Some(index) = position.get(category) {
                docs[*index].push(' ');
                docs[*index].push_str(question);
            }
        }
        let mut postings: HashMap<String, Vec<(usize, f64)>> = HashMap::new();
        let mut lengths = Vec::with_capacity(docs.len());
        for (index, doc) in docs.iter().enumerate() {
            let mut counts: HashMap<String, f64> = HashMap::new();
            let words = terms(doc);
            lengths.push(words.len() as f64);
            for term in words {
                *counts.entry(term).or_default() += 1.0;
            }
            for (term, count) in counts {
                postings.entry(term).or_default().push((index, count));
            }
        }
        let n = docs.len() as f64;
        let idf = postings
            .iter()
            .map(|(term, docs)| {
                let df = docs.len() as f64;
                (term.clone(), (1.0 + (n - df + 0.5) / (df + 0.5)).ln())
            })
            .collect();
        let average = if lengths.is_empty() {
            1.0
        } else {
            (lengths.iter().sum::<f64>() / n).max(1.0)
        };
        Self {
            ids: nodes.iter().map(|node| node.id.clone()).collect(),
            lengths,
            average,
            postings,
            idf,
        }
    }

    /// Node ids that share at least one term with `text`, best first, with
    /// their scores. Ties keep index order, so the result is deterministic.
    pub fn rank(&self, text: &str) -> Vec<(&str, f64)> {
        let mut query: HashMap<String, f64> = HashMap::new();
        for term in terms(text) {
            *query.entry(term).or_default() += 1.0;
        }
        let mut scores: HashMap<usize, f64> = HashMap::new();
        for term in query.keys() {
            let (Some(idf), Some(docs)) = (self.idf.get(term), self.postings.get(term)) else {
                continue;
            };
            for (index, count) in docs {
                let norm = K1 * (1.0 - B + B * self.lengths[*index] / self.average);
                *scores.entry(*index).or_default() += idf * count * (K1 + 1.0) / (count + norm);
            }
        }
        let mut ranked: Vec<(usize, f64)> = scores.into_iter().collect();
        ranked.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
        ranked
            .into_iter()
            .map(|(index, score)| (self.ids[index].as_str(), score))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(id: &str, parent: Option<&str>, name: &str, description: &str) -> Node {
        Node {
            id: id.into(),
            name: name.into(),
            description: description.into(),
            examples: vec![],
            parent_id: parent.map(str::to_string),
        }
    }

    #[test]
    fn terms_mix_words_and_bigrams() {
        let words = terms("배송 조회!");
        assert!(words.contains(&"배송".to_string()));
        assert!(
            words.contains(&"송조".to_string()),
            "bigrams cross the space"
        );
        assert!(!words.iter().any(|term| term.contains('!')));
    }

    #[test]
    fn item_questions_make_a_node_recallable() {
        let nodes = vec![
            node("orders", None, "주문", "주문과 배송"),
            node(
                "orders_tracking",
                Some("orders"),
                "배송 추적",
                "운송장 조회",
            ),
            node("account", None, "계정", "로그인과 비밀번호"),
        ];
        let bare = NodeIndex::build(&nodes, []);
        assert!(bare.rank("통관에서 멈췄어요").is_empty());
        let index = NodeIndex::build(
            &nodes,
            [("orders_tracking", "해외 직구가 통관에서 멈췄어요")],
        );
        assert_eq!(index.rank("통관에서 멈췄어요")[0].0, "orders_tracking");
    }

    #[test]
    fn rare_terms_outweigh_common_ones() {
        let nodes = vec![
            node("a", None, "비밀번호 변경", "비밀번호를 바꾸는 방법 안내"),
            node(
                "b",
                None,
                "와이파이 비밀번호",
                "공유기 와이파이 비밀번호 안내",
            ),
            node("c", None, "결제 안내", "결제 방법 안내"),
        ];
        let index = NodeIndex::build(&nodes, []);
        assert_eq!(index.rank("공유기 비밀번호")[0].0, "b");
    }
}
