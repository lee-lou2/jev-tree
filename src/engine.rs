use crate::AppState;
use crate::error::Error;
use crate::models::*;
use serde_json::{Value, json};
use std::collections::{BTreeMap, HashMap, HashSet};
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
    // Tests pin the router or the beam. A real run needs a Jev key: nothing is
    // routed without one, and the optional LLM is only a router on top.
    if !state.jev.choice_scripted() && !state.jev.llm_pinned() && !state.jev.key_set() {
        return Err(Error::BadRequest(
            "Jev API key is required. Add it in Settings → Models.".into(),
        ));
    }
    let mode = request.mode.clone();
    let text = request.text();
    let (leaf, path, trace) = descend(&state, &text, &request, sender.clone()).await?;
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
        let scope = resolve_root(&state.taxonomy()?, request.root.as_deref())?;
        if scope
            .allowed
            .as_ref()
            .is_some_and(|allowed| !allowed.contains(&category_id))
        {
            return Err(Error::BadRequest(
                "ingest target is outside the current root".into(),
            ));
        }
        let duplicate_ids: Vec<String> = state
            .store
            .active_in(std::slice::from_ref(&category_id))?
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
        let scope = resolve_root(&nodes, request.root.as_deref())?;
        let categories: Vec<String> = leaf
            .as_ref()
            .map(|id| {
                descendant_ids(&nodes, id)
                    .into_iter()
                    .filter(|id| {
                        scope
                            .allowed
                            .as_ref()
                            .map(|allowed| allowed.contains(id))
                            .unwrap_or(true)
                    })
                    .collect()
            })
            .unwrap_or_default();
        let candidates = state.store.active_in(&categories)?;
        emit(
            &sender,
            json!({"type":"retrieval_completed","count":candidates.len(),"category_id":leaf}),
        )
        .await;
        let ranked = rank(
            &state,
            &text,
            &request.context,
            candidates,
            leaf.as_deref(),
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
    request: &RunRequest,
    sender: Option<mpsc::Sender<Value>>,
) -> Result<(Option<String>, Vec<PathPart>, Trace), Error> {
    let mode = request.mode.as_str();
    let beam_width = request.beam_width;
    let run_id = Uuid::new_v4().to_string();
    let conversation = render_context(query, &request.context);
    let scope = resolve_root(&state.taxonomy()?, request.root.as_deref())?;
    let nodes = scope.descent_nodes.clone();
    let (initial_node, initial_names) = match request.start_node.as_deref() {
        Some(id) if scope.root.as_ref().is_some_and(|node| node.id == id) => (
            scope.descent_start.clone(),
            scope
                .descent_start
                .as_deref()
                .map(|start| vec![node_name(&nodes, start)])
                .unwrap_or_default(),
        ),
        Some(id) => {
            let node = nodes.iter().find(|node| node.id == id).ok_or_else(|| {
                Error::BadRequest(format!("start_node not found under the current root: {id}"))
            })?;
            (Some(id.to_string()), vec![node.name.clone()])
        }
        None => (
            scope.descent_start.clone(),
            scope
                .descent_start
                .as_deref()
                .map(|start| vec![node_name(&nodes, start)])
                .unwrap_or_default(),
        ),
    };
    let mut beams = vec![Beam {
        node_id: initial_node,
        path: vec![],
        probs: vec![],
        alive: true,
        names: initial_names,
    }];
    // When an LLM is configured it chooses the category. Jev always ranks the
    // items afterwards. A missing setting or a failed call uses the beam below.
    if state.jev.llm_routing() {
        let restrict = beams[0].node_id.clone();
        let progress = sender.clone();
        match state
            .jev
            .route(query, &conversation, &nodes, restrict.as_deref(), |note| {
                let Some(tx) = &progress else {
                    return;
                };
                let event = if note.asking {
                    json!({
                        "type": "descent_step",
                        "status": "asking",
                        "depth": note.depth
                    })
                } else {
                    json!({
                        "type": "descent_step",
                        "status": "answered",
                        "depth": note.depth,
                        "nodes": [{
                            "node_name": note.parent_name,
                            "choice_id": note.choice_id,
                            "choice_name": note.choice_name
                        }]
                    })
                };
                let _ = tx.try_send(event);
            })
            .await
        {
            Ok(route) => {
                return Ok(llm_route_result(&nodes, route, mode, query, &run_id, &sender).await);
            }
            Err(error) => {
                tracing::warn!("llm route failed, using beam descent: {error}");
            }
        }
    }
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
                    "background":&request.context,
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
                        probability: Some(*probability),
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
                        probability: Some(probability),
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
        router: "beam".into(),
        steps,
        leaf_id: winner.node_id.clone(),
        score: Some(score),
        final_beams: beams
            .iter()
            .map(|beam| FinalBeam {
                node_id: beam.node_id.clone(),
                path: beam.path.clone(),
                score: Some(path_score(&beam.probs)),
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
async fn llm_route_result(
    nodes: &[Node],
    route: crate::jev::LlmRoute,
    mode: &str,
    query: &str,
    run_id: &str,
    sender: &Option<mpsc::Sender<Value>>,
) -> (Option<String>, Vec<PathPart>, Trace) {
    let by_id: HashMap<&str, &Node> = nodes.iter().map(|node| (node.id.as_str(), node)).collect();
    let leaf = route.leaf.filter(|id| by_id.contains_key(id.as_str()));
    let mut chain: Vec<&Node> = Vec::new();
    if let Some(id) = leaf.as_deref() {
        let mut cursor = Some(id);
        let mut guard = 0;
        while let Some(current) = cursor {
            guard += 1;
            if guard > nodes.len() + 1 {
                break;
            }
            let Some(node) = by_id.get(current) else {
                break;
            };
            chain.push(*node);
            cursor = node.parent_id.as_deref();
        }
        chain.reverse();
    }
    let path: Vec<PathPart> = chain
        .iter()
        .map(|node| PathPart {
            id: node.id.clone(),
            name: node.name.clone(),
            probability: None,
            confidence: None,
        })
        .collect();
    // Steps are the model calls, not one invented edge per ancestor. A test pin
    // has no call, so the step list is empty and the path above is only ids.
    let steps = route
        .steps
        .into_iter()
        .map(|step| TraceStep {
            depth: step.depth,
            node_id: None,
            node_name: step.parent_name,
            path: vec![],
            candidates: step
                .candidates
                .into_iter()
                .map(|(id, name)| {
                    let terminal = id == "__none__" || id == "__stop__";
                    Candidate {
                        id: if terminal { None } else { Some(id) },
                        name,
                        probability: None,
                    }
                })
                .collect(),
            choice_id: step.choice_id,
            choice_name: step.choice_name,
            confidence: None,
            reason: format!("llm: {mode}: {query}"),
        })
        .collect();
    let trace = Trace {
        run_id: run_id.to_string(),
        mode: mode.to_string(),
        router: "llm".into(),
        steps,
        leaf_id: leaf.clone(),
        score: None,
        final_beams: vec![FinalBeam {
            node_id: leaf.clone(),
            path: path.clone(),
            score: None,
            alive: false,
        }],
    };
    emit(
        sender,
        json!({"type":"descent_done","run_id":run_id,"leaf_id":leaf,"path":path,"score":trace.score,"trace":trace}),
    )
    .await;
    (leaf, path, trace)
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

/// A URL/API `root` path turned into a virtual taxonomy.
///
/// The chosen node is the displayed root. Descent walks its **children** as if
/// they were the forest (`__none__` = outside this subtree). A childless node
/// stays the only landing. Settings and keys ignore this; they stay project-wide.
#[derive(Clone, Debug)]
pub struct RootScope {
    pub descent_nodes: Vec<Node>,
    pub descent_start: Option<String>,
    pub root: Option<Node>,
    pub path: Vec<Node>,
    /// Category ids search/list may use. `None` means the whole forest.
    pub allowed: Option<HashSet<String>>,
}

/// Walk `root` as slash-separated child keys (`products`, `products/products_stock`).
/// Each segment must match a **direct child** of the previous node (forest roots
/// at the first step) by id, then case-insensitive id, then case-insensitive name.
pub fn resolve_root(nodes: &[Node], root: Option<&str>) -> Result<RootScope, Error> {
    let Some(raw) = root.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok(RootScope {
            descent_nodes: nodes.to_vec(),
            descent_start: None,
            root: None,
            path: Vec::new(),
            allowed: None,
        });
    };
    let mut parent: Option<String> = None;
    let mut path = Vec::new();
    for segment in raw.split('/').filter(|value| !value.is_empty()) {
        if !is_root_segment(segment) {
            return Err(Error::BadRequest(format!(
                "invalid root segment: {segment}"
            )));
        }
        let children: Vec<&Node> = nodes
            .iter()
            .filter(|node| node.parent_id.as_deref() == parent.as_deref())
            .collect();
        let Some(found) = match_child(&children, segment) else {
            return Err(Error::NotFound(format!("root not found: {raw}")));
        };
        parent = Some(found.id.clone());
        path.push((*found).clone());
    }
    if path.is_empty() {
        return Err(Error::BadRequest("invalid root path".into()));
    }
    let root_node = path.last().cloned();
    let root_id = root_node.as_ref().map(|node| node.id.as_str());
    let allowed_ids = root_id
        .map(|id| descendant_ids(nodes, id))
        .unwrap_or_default();
    let has_children = nodes
        .iter()
        .any(|node| node.parent_id.as_deref() == root_id);
    let (descent_nodes, descent_start) = match (has_children, root_id) {
        (true, Some(root_id)) => {
            let mut scoped: Vec<Node> = nodes
                .iter()
                .filter(|node| allowed_ids.iter().any(|id| id == &node.id) && node.id != root_id)
                .cloned()
                .collect();
            for node in &mut scoped {
                if node.parent_id.as_deref() == Some(root_id) {
                    node.parent_id = None;
                }
            }
            (scoped, None)
        }
        _ => (
            root_node.iter().cloned().collect(),
            root_node.as_ref().map(|node| node.id.clone()),
        ),
    };
    Ok(RootScope {
        descent_nodes,
        descent_start,
        root: root_node,
        path,
        allowed: Some(allowed_ids.into_iter().collect()),
    })
}

fn is_root_segment(segment: &str) -> bool {
    !segment.is_empty()
        && segment
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

fn match_child<'a>(children: &[&'a Node], key: &str) -> Option<&'a Node> {
    children
        .iter()
        .copied()
        .find(|node| node.id == key)
        .or_else(|| {
            children
                .iter()
                .copied()
                .find(|node| node.id.eq_ignore_ascii_case(key))
        })
        .or_else(|| {
            children
                .iter()
                .copied()
                .find(|node| node.name.eq_ignore_ascii_case(key))
        })
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
fn node_name(nodes: &[Node], id: &str) -> String {
    nodes
        .iter()
        .find(|node| node.id == id)
        .map(|node| node.name.clone())
        .unwrap_or_default()
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
/// Items scored in one Jev call. Each item asks two questions, Noul and Score.
const RANK_BATCH: usize = 32;

/// Break near-ties upward when the walk stops at a node.
///
/// A descent that stops at `X` has already decided `X` is the right level of
/// specificity, and `X`'s own items sit there; everything below it is more
/// specific than the request asked for. Jev's scores routinely put a deeper item
/// a hair above `X`'s overview item (measured gaps 0.015-0.021), which is enough
/// to lose the answer to a vague question. Applied only where deeper items are
/// actually competing: a leaf's lone item beats nothing and must not be inflated.
const LEVEL_BONUS: f64 = 0.10;

async fn rank(
    state: &AppState,
    query: &str,
    context: &[Value],
    items: Vec<Item>,
    // The node the walk stopped at, if any. Its own items sit at the specificity
    // the request asked for, so they win near-ties against deeper ones.
    prefer: Option<&str>,
    limit: usize,
    sender: Option<mpsc::Sender<Value>>,
) -> Result<Vec<RankedItem>, Error> {
    if items.is_empty() {
        return Ok(vec![]);
    }
    // Only worth a nudge when deeper candidates are in the pool to nudge against.
    let nested = prefer.is_some_and(|node| {
        let own = items.iter().filter(|item| item.category_id == node).count();
        items.len() > own
    });
    let mut ranked = Vec::with_capacity(items.len());
    for chunk in items.chunks(RANK_BATCH) {
        let questions = chunk
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
        let judgment = state
            .jev
            .evaluate(
                json!({"conversation":render_context(query, context),"current_request":query,"items":chunk}),
                questions,
            )
            .await?;
        for item in chunk {
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
            let score = relevance * 0.7
                + (directness / 2.0) * 0.3
                + if nested && prefer == Some(item.category_id.as_str()) {
                    LEVEL_BONUS
                } else {
                    0.0
                };
            let role = if score >= 0.65 {
                "recommended"
            } else if score >= 0.4 {
                "alternative"
            } else {
                "reference"
            };
            let ranked_item = RankedItem {
                item: item.clone(),
                relevance,
                directness,
                confidence: None,
                role: role.into(),
                score,
            };
            emit(&sender, json!({"type":"candidate_evaluated","item_id":ranked_item.item.id,"score":ranked_item.score,"role":ranked_item.role})).await;
            ranked.push(ranked_item);
        }
    }
    ranked.sort_by(|a, b| {
        b.score
            .total_cmp(&a.score)
            .then_with(|| a.item.id.cmp(&b.item.id))
    });
    ranked.truncate(limit);
    Ok(ranked)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AppState;
    use crate::jev::{JevClient, JevConfig};
    use crate::store::Store;
    use std::sync::{Arc, RwLock};

    fn fixture_state() -> (tempfile::TempDir, AppState) {
        let dir = tempfile::tempdir().unwrap();
        let db = dir.path().join("t.db");
        let seed =
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/mini-seed.json");
        // SAFETY: test-only bootstrap, called before this fixture's store opens.
        unsafe {
            std::env::set_var("JEV_TREE_DB", db.to_str().unwrap());
            std::env::set_var(
                "JEV_TREE_SECRET_KEY",
                "test-secret-material-for-jev-tree-engine",
            );
        }
        let store = Store::open(db.to_str().unwrap(), seed.to_str().unwrap()).unwrap();
        let nodes = Arc::new(RwLock::new(store.load_taxonomy().unwrap()));
        let state = AppState {
            jev: JevClient::with_config(JevConfig::default()).unwrap(),
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
    async fn llm_script_routes_search_to_that_leaf() {
        let (_dir, state) = fixture_state();
        state.jev.script_llm(Some("orders_tracking".into()));
        let result = execute(state, run_req("해외 배송이 통관에서 멈췄어요"), None)
            .await
            .unwrap();
        assert_eq!(result["leaf_id"], "orders_tracking", "{result}");
        let path = result["path"].as_array().unwrap();
        assert_eq!(path[0]["id"], "orders");
        assert_eq!(path.last().unwrap()["id"], "orders_tracking");
        assert!(path[0]["probability"].is_null(), "{result}");
        assert!(result["trace"]["score"].is_null(), "{result}");
        assert_eq!(result["trace"]["router"], "llm");
    }

    #[tokio::test]
    async fn llm_script_none_abstains() {
        let (_dir, state) = fixture_state();
        state.jev.script_llm(Some("__none__".into()));
        let result = execute(state, run_req("안드로메다 은하까지의 거리"), None)
            .await
            .unwrap();
        assert!(result["leaf_id"].is_null(), "{result}");
        assert!(result["items"].as_array().unwrap().is_empty());
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
        assert!(
            steps
                .iter()
                .any(|step| { step["choice_name"] == "stay here" && step["node_id"] == "orders" })
        );
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
    fn resolve_root_scopes_to_direct_children() {
        let (_dir, state) = fixture_state();
        let nodes = state.taxonomy().unwrap();
        let scope = resolve_root(&nodes, Some("orders")).unwrap();
        assert_eq!(scope.root.as_ref().unwrap().id, "orders");
        assert!(scope.descent_start.is_none());
        assert!(
            scope
                .descent_nodes
                .iter()
                .any(|node| node.id == "orders_tracking" && node.parent_id.is_none())
        );
        assert!(scope.descent_nodes.iter().all(|node| node.id != "orders"));
        assert!(scope.descent_nodes.iter().all(|node| node.id != "account"));
        assert!(scope.allowed.as_ref().unwrap().contains("orders_tracking"));
        assert!(!scope.allowed.as_ref().unwrap().contains("account"));
    }

    #[test]
    fn resolve_root_nested_and_unknown() {
        let (_dir, state) = fixture_state();
        let nodes = state.taxonomy().unwrap();
        let nested = resolve_root(&nodes, Some("orders/orders_tracking")).unwrap();
        assert_eq!(nested.root.as_ref().unwrap().id, "orders_tracking");
        assert_eq!(nested.descent_start.as_deref(), Some("orders_tracking"));
        assert!(matches!(
            resolve_root(&nodes, Some("missing")),
            Err(Error::NotFound(_))
        ));
        assert!(matches!(
            resolve_root(&nodes, Some("orders/../account")),
            Err(Error::BadRequest(_))
        ));
    }

    #[tokio::test]
    async fn scoped_root_descends_from_children() {
        let (_dir, state) = fixture_state();
        state.jev.script_choices(vec!["orders_tracking".into()]);
        let mut request = run_req("해외 배송이 통관에서 멈췄어요");
        request.root = Some("orders".into());
        let result = execute(state, request, None).await.unwrap();
        assert_eq!(result["leaf_id"], "orders_tracking", "{result}");
        assert_eq!(result["path"].as_array().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn scoped_root_cannot_reach_sibling_forest() {
        let (_dir, state) = fixture_state();
        state.jev.script_choices(vec!["__none__".into()]);
        let mut request = run_req("비밀번호를 재설정하려면?");
        request.root = Some("orders".into());
        let result = execute(state, request, None).await.unwrap();
        assert!(result["leaf_id"].is_null(), "{result}");
        assert_eq!(result["items"].as_array().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn scoped_leaf_root_files_there() {
        let (_dir, state) = fixture_state();
        // The root is already a leaf, so descent does not ask Jev. The key gate
        // still applies; a scripted client stands in for a configured key.
        state.jev.script_choices(vec!["__stop__".into()]);
        let request = RunRequest {
            mode: "ingest".into(),
            question: "비밀번호를 재설정하려면?".into(),
            answer: "로그인 화면에서 재설정 메일을 요청하세요.".into(),
            auto_publish: false,
            beam_width: 3,
            limit: 8,
            root: Some("account".into()),
            ..Default::default()
        };
        let result = execute(state, request, None).await.unwrap();
        assert_eq!(result["category_id"], "account", "{result}");
    }

    #[tokio::test]
    async fn missing_jev_key_is_a_client_error() {
        let (_dir, state) = fixture_state();
        let error = execute(state, run_req("통관"), None).await.unwrap_err();
        assert!(matches!(error, Error::BadRequest(_)), "{error}");
        assert!(error.to_string().contains("Jev API key"));
    }

    #[tokio::test]
    async fn llm_failure_falls_back_to_the_jev_beam() {
        let (_dir, state) = fixture_state();
        state.jev.reconfigure(crate::jev::JevConfig {
            llm_base_url: "http://127.0.0.1:9".into(),
            llm_token: Some("not-a-real-token".into()),
            llm_model: "unused".into(),
            ..crate::jev::JevConfig::default()
        });
        state
            .jev
            .script_choices(vec!["orders".into(), "orders_tracking".into()]);
        let result = execute(state, run_req("해외 배송이 통관에서 멈췄어요"), None)
            .await
            .unwrap();
        assert_eq!(result["leaf_id"], "orders_tracking", "{result}");
        assert_eq!(result["trace"]["router"], "beam");
        assert!(result["trace"]["score"].as_f64().is_some(), "{result}");
    }

    #[tokio::test]
    async fn search_ranks_every_item_in_the_subtree() {
        let (_dir, state) = fixture_state();
        state
            .jev
            .script_choices(vec!["orders".into(), "orders_tracking".into()]);
        for n in 0..9 {
            state
                .store
                .upsert(
                    "orders_tracking",
                    &format!("해외 배송 조회 {n}"),
                    "앱의 주문내역에서 운송장을 누르세요.",
                    "qa",
                    None,
                    None,
                    true,
                )
                .unwrap();
        }
        let (tx, mut rx) = mpsc::channel(64);
        let mut request = run_req("관세 미납으로 보세창고에 묶였습니다");
        request.limit = 2;
        let result = execute(state, request, Some(tx)).await.unwrap();
        assert_eq!(result["items"].as_array().unwrap().len(), 2, "{result}");
        let mut considered = 0;
        let mut scored = 0;
        while let Some(event) = rx.recv().await {
            if event["type"] == "retrieval_completed" {
                considered = event["count"].as_u64().unwrap();
            }
            if event["type"] == "candidate_evaluated" {
                scored += 1;
            }
        }
        // The seed item plus nine more. The display limit must not be the candidate pool.
        assert_eq!(considered, 10, "retrieval dropped rows before ranking");
        assert_eq!(scored, 10, "Jev did not score the whole subtree");
    }

    #[test]
    fn path_score_is_geometric_mean() {
        // Two edges at 0.8 stay at 0.8; one bad edge must drag the path below a
        // steady one. Depth itself must not change the scale.
        assert!((path_score(&[0.8, 0.8]) - 0.8).abs() < 1e-9);
        assert!(path_score(&[0.9, 0.1]) < path_score(&[0.8, 0.8]));
    }

    #[test]
    fn normalize_marked_qa() {
        let (q, a) = normalize("Q: 카드 환불은 언제 되나요? A: 3~5영업일입니다.");
        assert_eq!(q, "카드 환불은 언제 되나요?");
        assert_eq!(a, "3~5영업일입니다.");
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

    /// 내부 노드에 개요 항목 1개, 그 아래 리프에 구체 항목 1개를 둔 작은 지식베이스.
    fn nested_state() -> (tempfile::TempDir, AppState) {
        let dir = tempfile::tempdir().unwrap();
        let seed = dir.path().join("seed.json");
        std::fs::write(
            &seed,
            serde_json::json!({
                "nodes": [
                    {"id": "topic", "name": "주제", "description": "상위 주제. 하위: 세부 사례.",
                     "examples": [], "parent_id": null},
                    {"id": "topic_case", "name": "세부 사례", "description": "구체적 조건의 사례.",
                     "examples": [], "parent_id": "topic"},
                ],
                "items": [
                    {"category_id": "topic", "kind": "article",
                     "question": "주제 문제는 어떻게 구분하나요?", "answer": "세부 사례로 나뉩니다."},
                    {"category_id": "topic_case", "kind": "qa",
                     "question": "세부 사례 하나를 어떻게 처리하나요?", "answer": "구체적 절차."},
                ],
            })
            .to_string(),
        )
        .unwrap();
        let db = dir.path().join("t.db");
        // SAFETY: test-only bootstrap, called before this fixture's store opens.
        unsafe {
            std::env::set_var("JEV_TREE_DB", db.to_str().unwrap());
            std::env::set_var(
                "JEV_TREE_SECRET_KEY",
                "test-secret-material-for-jev-tree-engine",
            );
        }
        let store = Store::open(db.to_str().unwrap(), seed.to_str().unwrap()).unwrap();
        let nodes = Arc::new(RwLock::new(store.load_taxonomy().unwrap()));
        let state = AppState {
            jev: JevClient::with_config(JevConfig::default()).unwrap(),
            store,
            nodes,
        };
        (dir, state)
    }

    #[tokio::test]
    async fn stopping_at_a_node_prefers_its_own_item_over_deeper_ones() {
        // 하강이 `topic`에서 멈추면(막연한 질문) 개요 항목이 리프의 구체 항목보다 위여야 한다.
        let (_dir, state) = nested_state();
        state
            .jev
            .script_choices(vec!["topic".into(), "__stop__".into()]);
        let result = execute(state, run_req("이 주제가 어떻게 되는지 궁금해요"), None)
            .await
            .unwrap();
        assert_eq!(result["leaf_id"], "topic", "{result}");
        let items = result["items"].as_array().unwrap();
        assert_eq!(
            items[0]["item"]["category_id"].as_str().unwrap(),
            "topic",
            "착지 노드의 자기 아이템이 2위 이하입니다: {result}"
        );
    }

    #[tokio::test]
    async fn a_leaf_item_gets_no_bonus_because_nothing_competes() {
        // 리프에 아이템이 하나뿐이면 가산할 대상이 없다. 점수를 부풀리면 안 된다.
        let (_dir, state) = nested_state();
        state
            .jev
            .script_choices(vec!["topic".into(), "topic_case".into()]);
        let result = execute(state, run_req("세부 사례 하나를 어떻게 처리하나요"), None)
            .await
            .unwrap();
        let items = result["items"].as_array().unwrap();
        assert_eq!(items.len(), 1, "{result}");
        // 스크립트 점수(noul 0.8, score 1.6) = 0.8*0.7 + 0.8*0.3 = 0.8. 가산 없음.
        assert!(
            (items[0]["score"].as_f64().unwrap() - 0.8).abs() < 1e-9,
            "{result}"
        );
    }
}
