use crate::error::Error;
use crate::models::*;
use crate::AppState;
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap};
use tokio::sync::mpsc;
use uuid::Uuid;

pub async fn emit(sender: &Option<mpsc::Sender<Value>>, value: Value) {
    if let Some(sender) = sender {
        let _ = sender.send(value).await;
    }
}

pub async fn execute(
    state: AppState,
    request: RunRequest,
    sender: Option<mpsc::Sender<Value>>,
) -> Result<Value, Error> {
    let mode = request.mode.clone();
    let text = request.text();
    let (leaf, path, trace) = descend(
        &state,
        &text,
        &request.context,
        &mode,
        request.beam_width,
        request.start_node.as_deref(),
        sender.clone(),
    )
    .await?;
    if mode == "ingest" {
        let (question, answer) = if !request.question.trim().is_empty() {
            (
                request.question.trim().to_string(),
                request.answer.trim().to_string(),
            )
        } else {
            normalize(&text)
        };
        if question.is_empty() {
            return Err(Error::BadRequest("question or title is required".into()));
        }
        if answer.is_empty() {
            return Err(Error::BadRequest("answer or body is required".into()));
        }
        let Some(category_id) = leaf.clone() else {
            return Err(Error::BadRequest(
                "no matching category in the tree; set start_node or make the question more specific"
                    .into(),
            ));
        };
        let duplicate_ids: Vec<String> = state
            .store
            .retrieve(&question, std::slice::from_ref(&category_id), 20)?
            .into_iter()
            .filter(|existing| near_duplicate(&question, &existing.question))
            .take(5)
            .map(|existing| existing.id)
            .collect();
        let (item, updated) = state.store.upsert(
            &category_id,
            &question,
            &answer,
            "qa",
            request.item_id.as_deref(),
            request.expected_version,
            request.auto_publish,
        )?;
        let published_id = if item.status == "active" {
            Some(item.id.clone())
        } else {
            None
        };
        let result = IngestResult {
            question,
            answer,
            kind: "qa".into(),
            category_id: Some(category_id),
            path,
            duplicate_ids,
            confidence: None,
            trace,
            item_id: Some(item.id.clone()),
            version: item.version,
            published_id,
            updated,
        };
        if request.auto_publish {
            emit(&sender, json!({"type":"ingest_saved","item":item,"updated":updated,"path":result.path,"trace":result.trace})).await;
        } else {
            emit(&sender, json!({"type":"ingest_draft","item":item,"duplicate_ids":result.duplicate_ids,"path":result.path,"trace":result.trace})).await;
        }
        Ok(serde_json::to_value(result)?)
    } else {
        let nodes = state.taxonomy()?;
        let categories = leaf
            .as_ref()
            .map(|id| descendant_ids(&nodes, id))
            .unwrap_or_default();
        let mut candidates = state.store.retrieve(&text, &categories, request.limit)?;
        emit(
            &sender,
            json!({"type":"retrieval_completed","count":candidates.len(),"category_id":leaf}),
        )
        .await;
        let ranked = rank(
            &state,
            &text,
            &request.context,
            &mut candidates,
            request.limit,
            sender.clone(),
        )
        .await?;
        let abstained = ranked.first().map(|item| item.score < 0.30).unwrap_or(true);
        let items = if abstained {
            ranked
                .into_iter()
                .map(|mut item| {
                    item.role = "reference".into();
                    item
                })
                .collect()
        } else {
            ranked
        };
        let result = SearchResult {
            query: text,
            items,
            leaf_id: leaf,
            path,
            trace,
            abstained,
        };
        emit(&sender, json!({"type":"search_done","leaf_id":result.leaf_id,"abstained":result.abstained,"items":result.items,"results":result.items,"query":result.query,"path":result.path,"trace":result.trace})).await;
        Ok(serde_json::to_value(result)?)
    }
}

async fn descend(
    state: &AppState,
    query: &str,
    context: &[Value],
    mode: &str,
    beam_width: usize,
    start_node: Option<&str>,
    sender: Option<mpsc::Sender<Value>>,
) -> Result<(Option<String>, Vec<PathPart>, Trace), Error> {
    let run_id = Uuid::new_v4().to_string();
    let conversation = render_context(query, context);
    let nodes = state.taxonomy()?;
    let (initial_node, initial_names) = if let Some(id) = start_node {
        let node = nodes
            .iter()
            .find(|node| node.id == id)
            .ok_or_else(|| Error::BadRequest(format!("start_node not found: {id}")))?;
        (Some(id.to_string()), vec![node.name.clone()])
    } else {
        (None, vec![])
    };
    let mut beams = vec![Beam {
        node_id: initial_node,
        path: vec![],
        probs: vec![],
        alive: true,
        names: initial_names,
    }];
    let mut steps = vec![];
    for depth_value in 0..tree_depth(&nodes) {
        // Nobody is listening any more: stop before paying for another evaluator round.
        if sender.as_ref().is_some_and(mpsc::Sender::is_closed) {
            return Err(Error::Cancelled);
        }
        let frontiers: Vec<(usize, Vec<&Node>, String, String)> = beams
            .iter()
            .enumerate()
            .filter(|(_, beam)| beam.alive)
            .filter_map(|(index, beam)| {
                let children: Vec<&Node> = nodes
                    .iter()
                    .filter(|node| node.parent_id.as_deref() == beam.node_id.as_deref())
                    .collect();
                if children.is_empty() {
                    None
                } else {
                    let parent = beam
                        .node_id
                        .as_deref()
                        .and_then(|id| nodes.iter().find(|node| node.id == id))
                        .map(|node| node.name.clone())
                        .unwrap_or_else(|| "the knowledge root".into());
                    Some((
                        index,
                        children,
                        parent,
                        if beam.node_id.is_none() {
                            "__none__".into()
                        } else {
                            "__stop__".into()
                        },
                    ))
                }
            })
            .collect();
        if frontiers.is_empty() {
            break;
        }
        let mut questions = BTreeMap::new();
        for (index, children, parent, terminal) in &frontiers {
            let mut criteria = serde_json::Map::new();
            for child in children {
                criteria.insert(child.id.clone(), Value::String(choice_label(child)));
            }
            criteria.insert(
                terminal.clone(),
                Value::String(if terminal == "__none__" {
                    "No root topic fits; choose no matching topic.".into()
                } else {
                    format!("Stay at {parent}: it already holds the best match.")
                }),
            );
            questions.insert(
                format!("depth_{depth_value}_{index}"),
                Question {
                    kind: "choice".into(),
                    instructions: format!(
                        "Choose the most specific child for the current request. Read the current request first; background is supporting context. Path: {}.",
                        beams[*index].names.join(" / ")
                    ),
                    criteria: Some(Value::Object(criteria)),
                },
            );
        }
        emit(&sender, json!({"type":"descent_step","status":"asking","depth":depth_value,"run_id":run_id,"frontiers":frontiers.iter().map(|(index,children,parent,_)|json!({"node_id":beams[*index].node_id,"path":beams[*index].names,"parent":parent,"children":children.iter().map(|node|json!({"id":node.id,"name":node.name})).collect::<Vec<_>>() })).collect::<Vec<_>>() })).await;
        let judgment = state
            .jev
            .evaluate(
                json!({
                    "conversation":conversation,
                    "current_request":query,
                    "background":context,
                    "mode":mode,
                    "path":beams.first().map(|beam| beam.names.clone()).unwrap_or_default()
                }),
                questions,
            )
            .await?;
        // Every beam that is not expanding this round survives on its own score.
        // An alive beam with no children has reached a real leaf: settle it instead of
        // dropping it, or a deeper sibling branch would win by default.
        let expanding: std::collections::HashSet<usize> =
            frontiers.iter().map(|(index, ..)| *index).collect();
        let mut candidates: Vec<Beam> = beams
            .iter()
            .enumerate()
            .filter(|(index, _)| !expanding.contains(index))
            .map(|(_, beam)| Beam {
                alive: false,
                ..beam.clone()
            })
            .collect();
        let mut trace_nodes = vec![];
        for (index, children, parent, terminal) in frontiers {
            let qid = format!("depth_{depth_value}_{index}");
            let (probabilities, confidence) = match judgment.answers.get(&qid) {
                Some(Answer::Choice {
                    probabilities,
                    confidence,
                    ..
                }) => (probabilities.clone(), *confidence),
                _ => return Err(Error::Internal(format!("missing choice {qid}"))),
            };
            let valid: HashMap<String, &Node> = children
                .iter()
                .map(|node| (node.id.clone(), *node))
                .collect();
            let mut ranked: Vec<(String, f64)> = probabilities
                .into_iter()
                .filter(|(id, _)| valid.contains_key(id) || id == &terminal)
                .collect();
            ranked.sort_by(|a, b| b.1.total_cmp(&a.1));
            let terminal_prob = ranked
                .iter()
                .find(|(id, _)| id == &terminal)
                .map(|(_, probability)| *probability)
                .unwrap_or(0.0);
            let selected = ranked.first().cloned().unwrap_or((terminal.clone(), 0.0));
            let stayed = selected.0 == terminal;
            let choice_name = if stayed {
                if terminal == "__none__" {
                    "no matching topic".into()
                } else {
                    "stay here".into()
                }
            } else {
                valid[&selected.0].name.clone()
            };
            trace_nodes.push(TraceStep {
                depth: depth_value,
                node_id: beams[index].node_id.clone(),
                node_name: parent.clone(),
                path: beams[index].path.clone(),
                candidates: ranked
                    .iter()
                    .map(|(id, probability)| Candidate {
                        id: if id == &terminal {
                            None
                        } else {
                            Some(id.clone())
                        },
                        name: if id == &terminal {
                            if terminal == "__none__" {
                                "no matching topic".into()
                            } else {
                                "stay here".into()
                            }
                        } else {
                            valid[id].name.clone()
                        },
                        probability: *probability,
                    })
                    .collect(),
                choice_id: if stayed {
                    None
                } else {
                    Some(selected.0.clone())
                },
                choice_name,
                confidence: Some(confidence),
                reason: if stayed {
                    format!("{mode}: stay at {parent}")
                } else {
                    format!("{mode}: {query}")
                },
            });
            let mut terminal_probs = beams[index].probs.clone();
            terminal_probs.push(terminal_prob);
            candidates.push(Beam {
                node_id: beams[index].node_id.clone(),
                path: beams[index].path.clone(),
                probs: terminal_probs,
                alive: false,
                names: beams[index].names.clone(),
            });
            if !stayed {
                for (id, probability) in ranked
                    .into_iter()
                    .filter(|(id, _)| id != &terminal)
                    .take(beam_width)
                {
                    let child = valid[&id];
                    let mut path = beams[index].path.clone();
                    path.push(PathPart {
                        id: child.id.clone(),
                        name: child.name.clone(),
                        probability,
                        confidence: Some(confidence),
                    });
                    let mut probs = beams[index].probs.clone();
                    probs.push(probability);
                    let mut names = beams[index].names.clone();
                    names.push(child.name.clone());
                    candidates.push(Beam {
                        node_id: Some(child.id.clone()),
                        path,
                        probs,
                        alive: true,
                        names,
                    });
                }
            }
        }
        steps.extend(trace_nodes.clone());
        beams = candidates;
        beams.sort_by(|a, b| path_score(&b.probs).total_cmp(&path_score(&a.probs)));
        beams.truncate(beam_width);
        emit(&sender, json!({"type":"descent_step","status":"answered","depth":depth_value,"run_id":run_id,"nodes":trace_nodes,"beams":beams.iter().map(|beam|json!({"node_id":beam.node_id,"path":beam.path,"score":path_score(&beam.probs),"alive":beam.alive})).collect::<Vec<_>>() })).await;
        if !beams.iter().any(|beam| beam.alive) {
            break;
        }
    }
    let winner = beams
        .first()
        .cloned()
        .ok_or_else(|| Error::Internal("empty beam".into()))?;
    let score = path_score(&winner.probs);
    let trace = Trace {
        run_id: run_id.clone(),
        mode: mode.to_string(),
        steps,
        leaf_id: winner.node_id.clone(),
        score,
        final_beams: beams
            .iter()
            .map(|beam| FinalBeam {
                node_id: beam.node_id.clone(),
                path: beam.path.clone(),
                score: path_score(&beam.probs),
                alive: beam.alive,
            })
            .collect(),
    };
    emit(&sender, json!({"type":"descent_done","run_id":run_id,"leaf_id":winner.node_id,"path":winner.path,"score":score,"trace":trace})).await;
    Ok((winner.node_id, winner.path, trace))
}

#[derive(Clone)]
struct Beam {
    node_id: Option<String>,
    path: Vec<PathPart>,
    probs: Vec<f64>,
    alive: bool,
    names: Vec<String>,
}
pub fn path_score(probs: &[f64]) -> f64 {
    if probs.is_empty() {
        0.0
    } else {
        (probs.iter().map(|p| p.max(1e-9).ln()).sum::<f64>() / probs.len() as f64).exp()
    }
}
/// Character 3-grams of the alphanumeric content, used for duplicate detection.
/// Token overlap is useless here: particles and inflected forms become different tokens.
fn trigrams(text: &str) -> std::collections::HashSet<String> {
    let compact: Vec<char> = text
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect();
    if compact.len() < 3 {
        return std::collections::HashSet::new();
    }
    (0..=compact.len() - 3)
        .map(|i| compact[i..i + 3].iter().collect())
        .collect()
}

/// `duplicate_ids` must mean "this is the same question", not "this is nearby".
fn near_duplicate(question: &str, existing: &str) -> bool {
    let (a, b) = (trigrams(question), trigrams(existing));
    if a.is_empty() || b.is_empty() {
        return question.trim() == existing.trim();
    }
    a.intersection(&b).count() as f64 / a.len().min(b.len()) as f64 >= 0.6
}

/// Deepest path in the taxonomy; the descent never needs more rounds than that.
fn tree_depth(nodes: &[Node]) -> usize {
    let parents: HashMap<&str, Option<&str>> = nodes
        .iter()
        .map(|node| (node.id.as_str(), node.parent_id.as_deref()))
        .collect();
    let mut deepest = 1;
    for node in nodes {
        let mut depth = 1;
        let mut cursor = node.parent_id.as_deref();
        while let Some(id) = cursor {
            depth += 1;
            if depth > nodes.len() {
                break;
            }
            cursor = parents.get(id).copied().flatten();
        }
        deepest = deepest.max(depth);
    }
    deepest
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
pub fn depth(id: &str, nodes: &[Node]) -> usize {
    let mut depth = 1;
    let mut current = id;
    while let Some(parent) = nodes
        .iter()
        .find(|node| node.id == current)
        .and_then(|node| node.parent_id.as_deref())
    {
        depth += 1;
        current = parent;
    }
    depth
}
fn choice_label(node: &Node) -> String {
    let mut label = format!("{}: {}", node.name, node.description);
    if !node.examples.is_empty() {
        label.push_str(" Examples: ");
        label.push_str(&node.examples.join(" / "));
    }
    label
}

fn render_context(query: &str, context: &[Value]) -> String {
    let mut lines = Vec::new();
    for turn in context {
        match turn {
            Value::String(text) if !text.trim().is_empty() => lines.push(text.clone()),
            Value::Object(map) => {
                let role = map.get("role").and_then(Value::as_str).unwrap_or("turn");
                let text = map
                    .get("text")
                    .or_else(|| map.get("content"))
                    .and_then(Value::as_str)
                    .map(str::to_string)
                    .unwrap_or_else(|| turn.to_string());
                if !text.trim().is_empty() {
                    lines.push(format!("{role}: {text}"));
                }
            }
            other if !other.is_null() => lines.push(other.to_string()),
            _ => {}
        }
    }
    lines.push(format!("[current request — decide for THIS] {query}"));
    lines.join("\n")
}
pub fn normalize(source: &str) -> (String, String) {
    let regex = regex::Regex::new(
        r"(?is)^\s*(?:q(?:uestion)?\s*:\s*)(.*?)\s+(?:a(?:nswer)?\s*:\s*)(.*)\s*$",
    )
    .unwrap();
    if let Some(captures) = regex.captures(source) {
        (
            captures[1].trim().to_string(),
            captures[2].trim().to_string(),
        )
    } else {
        let mut parts = source.splitn(2, '\n');
        (
            parts.next().unwrap_or("").trim().to_string(),
            parts.next().unwrap_or("").trim().to_string(),
        )
    }
}
async fn rank(
    state: &AppState,
    query: &str,
    context: &[Value],
    items: &mut Vec<Item>,
    limit: usize,
    sender: Option<mpsc::Sender<Value>>,
) -> Result<Vec<RankedItem>, Error> {
    if items.is_empty() {
        return Ok(vec![]);
    }
    let questions = items
        .iter()
        .flat_map(|item| {
            [
                (
                    format!("use_{}", item.id),
                    Question {
                        kind: "noul".into(),
                        instructions: format!(
                            "Could item {} be sent directly as a reply?",
                            item.id
                        ),
                        criteria: None,
                    },
                ),
                (
                    format!("direct_{}", item.id),
                    Question {
                        kind: "score".into(),
                        instructions: format!(
                            "How directly does item {} answer the request?",
                            item.id
                        ),
                        criteria: Some(json!(["unrelated", "useful", "direct"])),
                    },
                ),
            ]
        })
        .collect();
    let judgment = state.jev.evaluate(json!({"conversation":render_context(query, context),"current_request":query,"items":items}), questions).await?;
    let mut ranked = vec![];
    for item in items.drain(..) {
        let relevance = match judgment.answers.get(&format!("use_{}", item.id)) {
            Some(Answer::Noul { noul }) => *noul,
            _ => 0.0,
        };
        let directness = match judgment.answers.get(&format!("direct_{}", item.id)) {
            Some(Answer::Score { score, confidence }) => {
                let _ = confidence;
                *score
            }
            _ => 0.0,
        };
        let score = relevance * 0.7 + (directness / 2.0) * 0.3;
        let role = if score >= 0.65 {
            "recommended"
        } else if score >= 0.4 {
            "alternative"
        } else {
            "reference"
        };
        ranked.push(RankedItem {
            item,
            relevance,
            directness,
            confidence: None,
            role: role.into(),
            score,
        });
    }
    ranked.sort_by(|a, b| b.score.total_cmp(&a.score));
    ranked.truncate(limit);
    for item in &ranked {
        emit(&sender, json!({"type":"candidate_evaluated","item_id":item.item.id,"score":item.score,"role":item.role})).await;
    }
    Ok(ranked)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jev::{JevClient, JevConfig};
    use crate::store::Store;
    use crate::AppState;
    use std::sync::{Arc, RwLock};

    fn fixture_state() -> (tempfile::TempDir, AppState) {
        let dir = tempfile::tempdir().unwrap();
        let db = dir.path().join("t.db");
        let seed =
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/mini-seed.json");
        std::env::set_var("JEV_TREE_DB", db.to_str().unwrap());
        std::env::set_var(
            "JEV_TREE_SECRET_KEY",
            "test-secret-material-for-jev-tree-engine",
        );
        let store = Store::open(db.to_str().unwrap(), seed.to_str().unwrap()).unwrap();
        let nodes = Arc::new(RwLock::new(store.load_taxonomy().unwrap()));
        let state = AppState {
            jev: JevClient::with_config(JevConfig::default(), nodes.clone()).unwrap(),
            store,
            nodes,
        };
        (dir, state)
    }

    fn run_req(query: &str) -> RunRequest {
        RunRequest {
            mode: "search".into(),
            query: query.into(),
            beam_width: 3,
            limit: 8,
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn stay_at_root_yields_null_leaf() {
        let (_dir, state) = fixture_state();
        state.jev.script_choices(vec!["__none__".into()]);
        let result = execute(state, run_req(" unrelated astronomy question "), None)
            .await
            .unwrap();
        assert!(result["leaf_id"].is_null(), "{result}");
        assert_eq!(result["items"].as_array().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn stay_at_parent_does_not_descend() {
        let (_dir, state) = fixture_state();
        state
            .jev
            .script_choices(vec!["orders".into(), "__stop__".into()]);
        let result = execute(state, run_req("주문 상태가 궁금해요"), None)
            .await
            .unwrap();
        assert_eq!(result["leaf_id"], "orders", "{result}");
        let steps = result["trace"]["steps"].as_array().unwrap();
        assert!(steps
            .iter()
            .any(|step| { step["choice_name"] == "stay here" && step["node_id"] == "orders" }));
    }

    #[tokio::test]
    async fn shallow_leaf_beats_a_deeper_sibling_branch() {
        // `account` is a leaf directly under the root while `orders` still has a child.
        // The winning shallow beam must survive the round that expands its sibling.
        let (_dir, state) = fixture_state();
        state.jev.script_choices(vec!["account".into()]);
        let result = execute(state, run_req("비밀번호를 재설정하려면?"), None)
            .await
            .unwrap();
        assert_eq!(result["leaf_id"], "account", "{result}");
        let beams = result["trace"]["final_beams"].as_array().unwrap();
        assert_eq!(beams[0]["node_id"], "account");
    }

    #[tokio::test]
    async fn scripted_choice_reaches_tracking_leaf() {
        let (_dir, state) = fixture_state();
        state
            .jev
            .script_choices(vec!["orders".into(), "orders_tracking".into()]);
        let result = execute(state, run_req("해외 배송이 통관에서 멈췄어요"), None)
            .await
            .unwrap();
        assert_eq!(result["leaf_id"], "orders_tracking", "{result}");
        assert!(!result["abstained"].as_bool().unwrap());
    }

    #[tokio::test]
    async fn draft_ingest_persists_without_publish() {
        let (_dir, state) = fixture_state();
        state
            .jev
            .script_choices(vec!["orders".into(), "orders_tracking".into()]);
        let request = RunRequest {
            mode: "ingest".into(),
            question: "통관이 왜 멈추나요?".into(),
            answer: "개인통관번호를 확인하세요.".into(),
            auto_publish: false,
            beam_width: 3,
            limit: 8,
            ..Default::default()
        };
        let result = execute(state.clone(), request, None).await.unwrap();
        assert!(result["published_id"].is_null(), "{result}");
        assert_eq!(result["category_id"], "orders_tracking");
        let (items, _) = state
            .store
            .list_items(0, 20, Some("통관이 왜"), None, false, "active")
            .unwrap();
        assert!(
            items.is_empty(),
            "drafts must not appear in search listings"
        );
        // ... but the draft is stored, addressable, and listable on request.
        let item_id = result["item_id"].as_str().unwrap().to_string();
        let (drafts, total) = state
            .store
            .list_items(0, 20, None, None, false, "draft")
            .unwrap();
        assert_eq!(total, 1);
        assert_eq!(drafts[0].id, item_id);
        assert!(state.store.archive_item(&item_id).unwrap());
        let (_, after) = state
            .store
            .list_items(0, 20, None, None, false, "draft")
            .unwrap();
        assert_eq!(after, 0, "archived drafts disappear");
    }

    #[test]
    fn context_keeps_object_turns() {
        let rendered = render_context(
            "지금 뭐 해야 하나요?",
            &[json!({"role":"user","text":"통관이 멈췄어요"})],
        );
        assert!(rendered.contains("통관이 멈췄어요"));
        assert!(rendered.contains("지금 뭐 해야 하나요?"));
    }
}
