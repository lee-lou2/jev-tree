use crate::AppState;
use crate::error::Error;
use crate::jev::{overlap_score, tokens};
use crate::models::*;
use futures_util::StreamExt;
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
    let (leaf, path, trace) = match filed_category(&state, &request)? {
        Some(category) => stored_route(&state.taxonomy()?, &category, &mode, &sender).await,
        None => descend(&state, &text, &request, sender.clone()).await?,
    };
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
        let abstained = ranked
            .first()
            .map(|item| holds(item.score, item.directness))
            .unwrap_or(true);
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

/// The category a stored row is already filed under, when an ingest addresses
/// it by `item_id` and does not pin a `start_node`. Publishing a reviewed draft
/// has to file it where the reviewer saw it: routing again with an edited
/// answer can land elsewhere, or nowhere, and then a reviewed draft could not be
/// published at all. `None` routes the request afresh, which is also what
/// happens when the stored category is gone or outside the current `root`.
fn filed_category(state: &AppState, request: &RunRequest) -> Result<Option<String>, Error> {
    if request.mode != "ingest" || request.start_node.is_some() {
        return Ok(None);
    }
    let Some(id) = request.item_id.as_deref() else {
        return Ok(None);
    };
    let item = state
        .store
        .editable_item(id)?
        .ok_or_else(|| Error::NotFound("item not found".into()))?;
    let nodes = state.taxonomy()?;
    let scope = resolve_root(&nodes, request.root.as_deref())?;
    let usable = nodes.iter().any(|node| node.id == item.category_id)
        && scope
            .allowed
            .as_ref()
            .is_none_or(|allowed| allowed.contains(&item.category_id));
    Ok(usable.then_some(item.category_id))
}

/// A route nobody was asked for: the row's own category, with the same shape a
/// forced single-node start has (`flat`, no steps, no probabilities).
async fn stored_route(
    nodes: &[Node],
    category: &str,
    mode: &str,
    sender: &Option<mpsc::Sender<Value>>,
) -> (Option<String>, Vec<PathPart>, Trace) {
    let pool: Vec<String> = nodes.iter().map(|node| node.id.clone()).collect();
    let path: Vec<PathPart> = node_chain(nodes, &pool, category)
        .into_iter()
        .map(|id| PathPart {
            name: node_name(nodes, &id),
            id,
            probability: None,
            confidence: None,
        })
        .collect();
    let run_id = Uuid::new_v4().to_string();
    let trace = Trace {
        run_id: run_id.clone(),
        mode: mode.to_string(),
        router: "flat".into(),
        steps: vec![],
        leaf_id: Some(category.to_string()),
        score: None,
        final_beams: vec![FinalBeam {
            node_id: Some(category.to_string()),
            path: path.clone(),
            score: None,
            alive: false,
        }],
    };
    emit(
        sender,
        json!({"type":"descent_done","run_id":run_id,"leaf_id":category,"path":path,"score":null,"trace":trace}),
    )
    .await;
    (Some(category.to_string()), path, trace)
}

/// Upper bound on a single descent so a degraded evaluator cannot pin a
/// connection open. `JEV_TREE_RUN_TIMEOUT_SECS` overrides the 120s default.
pub fn run_deadline() -> std::time::Duration {
    let seconds = std::env::var("JEV_TREE_RUN_TIMEOUT_SECS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(120);
    std::time::Duration::from_secs(seconds)
}

/// Wall-clock budget for the LLM route step alone. The route call may retry and
/// the model may deliberate over the full menu, so without a ceiling a slow
/// reply eats the whole run deadline and the request dies as a timeout instead
/// of falling back to the Jev beam. A third of the deadline leaves room for the
/// beam and the ranking that follow.
fn llm_route_budget() -> std::time::Duration {
    run_deadline() / 3
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
        let routed = tokio::time::timeout(
            llm_route_budget(),
            state
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
                }),
        )
        .await;
        match routed {
            Ok(Ok(route)) => {
                return Ok(llm_route_result(&nodes, route, mode, query, &run_id, &sender).await);
            }
            Ok(Err(error)) => {
                tracing::warn!("llm route failed, using Jev routing: {error}");
            }
            Err(_) => {
                tracing::warn!("llm route failed, using Jev routing: route budget spent");
            }
        }
    }
    // The default Jev route: one choice over every node in scope. The
    // level-by-level beam loses a leaf as soon as one level guesses wrong, and
    // the loss grows with depth; one choice removes the cascade and the round
    // trips. The beam below runs only when the scope is too large to name in
    // one call and recall finds no evidence.
    if let Some(result) = flat_route(
        state,
        &nodes,
        query,
        request,
        beams[0].node_id.as_deref(),
        &run_id,
        &sender,
    )
    .await?
    {
        return Ok(result);
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

/// Most nodes one Jev choice can name. The evaluator takes at most 255 options
/// (a 256th is HTTP 422, measured against the live API) and a forest walk
/// spends one of them on `__none__`.
const FLAT_MAX_NODES: usize = 254;

/// Most menu text (ids plus labels) one Jev choice can read. The whole request
/// has to fit the evaluator's context window: a 234-node menu of 94 KB passes
/// against the live API and one of 124 KB is HTTP 400. 80 KB keeps the margin.
const FLAT_MAX_MENU_BYTES: usize = 80_000;

/// JSON framing around one menu entry: quotes, colon, comma.
const MENU_ENTRY_FRAMING: usize = 6;

/// Minimum lexical evidence before a recall menu is worth asking about. One
/// shared character bigram is worth 0.35, so 1.0 means at least a word of the
/// taxonomy matched. Only scopes past [`FLAT_MAX_NODES`] or
/// [`FLAT_MAX_MENU_BYTES`] get here: without evidence the semantic beam runs
/// instead.
const FLAT_MIN_EVIDENCE: f64 = 1.0;

/// Global matches first, then a per-root quota, so one crowded subtree cannot
/// push every other topic out of the choice. Each match keeps its full ancestor
/// chain: a request that names only a broad topic must still see that topic.
const FLAT_GLOBAL_K: usize = 15;
const FLAT_PER_ROOT: usize = 2;

/// The rule set the flat choice is judged by. The stay/descend rule is what
/// keeps a general question on a topic node and a named sub-topic on its child.
const FLAT_RULES: &str = "요청을 가장 정확히 담을 분류 하나를 고르세요.\n- 요청에 언급된 조건·하위 주제가 모두 들어가는 가장 구체적인 분류를 고르세요.\n- 요청이 하위 분류의 주제를 부르거나 그 주제에 대해 물으면, 다른 조건이 없어도 그 하위 분류를 고르세요. 요청의 주제 명사구가 어떤 분류의 이름과 같으면 더 구체적인 분류를 고르세요.\n- 요청에 없는 조건을 가정해 더 아래 분류로 내려가지 마세요. 하위 조건이 하나도 언급되지 않았다면 상위 분류에 머무르세요.\n- 요청이 이 분류들이 다루는 주제에 대한 문의라면 __none__을 고르지 마세요.";

const FLAT_NONE: &str = "어느 분류에도 속하지 않음 — 요청이 이 분류들이 다루는 주제에 대한 문의가 아님(사이트 자체에 대한 일반 질문, 인사, 잡담, 완전히 다른 주제). 주제에 대한 문의라면 절대 고르지 마세요";

/// Search holds rather than answers when the best item is weak. A score of 0.45
/// or more answers outright; below that only a *direct* item answers (directness
/// ≥ 1.0), and anything weaker is held. Swept over the golden set's 2,102 search
/// cases: end-to-end +0.9pp with **no variant regressing** (deep +0.2, standard
/// +0.2, shallow +0.5, flat 0.0, sparse +3.4), false answers 62 → 41 against
/// false holds 10 → 14. Stricter lines trade that for regressions: a flat 0.45
/// line alone gains more on `sparse` but loses +1.4pp on `flat`, where nothing
/// is ever missing to hold back.
const RANK_MIN_SCORE: f64 = 0.45;

/// Below [`RANK_MIN_SCORE`] but at or above this, a direct item still answers.
const RANK_GRAY_ZONE: f64 = 0.30;

/// Directness (0..2) that lets a gray-zone item answer instead of holding.
const RANK_DIRECT_MIN: f64 = 1.0;

/// Below this chosen probability the route is unsure of its own pick, and one
/// more judgment asks only whether this tree covers the request at all. Out-of-
/// scope landings concentrate in that zone (median chosen probability 0.64
/// against 0.99 for correct in-tree picks), so the check catches them without
/// touching the confident majority.
const ROUTE_SCOPE_CHECK_BELOW: f64 = 0.85;

/// The scope check runs on an uncertain forest walk only. A `start_node` walk
/// must land on a node and cannot abstain, so it never asks.
fn scope_check_needed(chosen_probability: Option<f64>, start_forced: bool) -> bool {
    !start_forced && chosen_probability.is_some_and(|p| p < ROUTE_SCOPE_CHECK_BELOW)
}

/// Only a confident `out` refuses the request. A hesitant "out" keeps the menu
/// choice: one cross-domain question ("laptop battery swelled, where do I apply?")
/// was refused at 0.5 and cost more than the timid refusals were worth.
const SCOPE_OUT_MIN: f64 = 0.8;

/// Only a confident `out` refuses: `choice` plus its probability.
fn scope_refuses(choice: &str, out_probability: f64) -> bool {
    choice == "out" && out_probability >= SCOPE_OUT_MIN
}

/// One judgment on scope alone. The menu asks "which node fits" and an
/// out-of-scope request can still win one; this asks "does this tree cover the
/// request at all". `None` when the call fails — the menu choice stands.
async fn scope_check(
    state: &AppState,
    nodes: &[Node],
    query: &str,
    request: &RunRequest,
    conversation: &str,
    chosen: &str,
) -> Option<(bool, TraceStep)> {
    let mut criteria = serde_json::Map::new();
    criteria.insert(
        "in".into(),
        Value::String("이 요청은 이 트리가 다루는 주제를 묻는다".into()),
    );
    criteria.insert(
        "out".into(),
        Value::String("이 요청은 이 트리가 다루지 않는 주제를 묻는다".into()),
    );
    let mut questions = BTreeMap::new();
    questions.insert(
        "scope".into(),
        Question {
            kind: "choice".into(),
            instructions: "이 요청이 이 지식 트리가 다루는 주제인지 판정하세요. 참고로 고른 분류는 적혀 있지만, 판단은 요청 자체가 이 트리의 주제에 대한 문의인지로만 하세요. 인사, 잡담, 사이트 자체에 대한 일반 질문, 완전히 다른 주제는 out입니다.".into(),
            criteria: Some(Value::Object(criteria)),
        },
    );
    let judgment = state
        .jev
        .evaluate(
            json!({
                "conversation": conversation,
                "current_request": query,
                "background": &request.context,
                "mode": request.mode,
                "chosen": flat_label_for_scope(nodes, chosen),
            }),
            questions,
        )
        .await
        .ok()?;
    let Answer::Choice {
        choice,
        probabilities,
        confidence,
    } = judgment.answers.get("scope")?
    else {
        return None;
    };
    let in_scope = !scope_refuses(choice, probabilities.get("out").copied().unwrap_or(0.0));
    let step = TraceStep {
        depth: 1,
        node_id: Some(chosen.to_string()),
        node_name: "scope check".into(),
        path: vec![],
        candidates: vec![
            Candidate {
                id: Some("in".into()),
                name: "request is in scope".into(),
                probability: probabilities.get("in").copied(),
            },
            Candidate {
                id: Some("out".into()),
                name: "request is out of scope".into(),
                probability: probabilities.get("out").copied(),
            },
        ],
        choice_id: Some(choice.clone()),
        choice_name: if in_scope {
            "request is in scope".into()
        } else {
            "request is out of scope".into()
        },
        confidence: Some(*confidence),
        reason: "flat: scope check".into(),
    };
    Some((in_scope, step))
}

/// The scope check's evidence for the chosen node: names plus description,
/// without the examples, so the node's own wording cannot talk the check into
/// seeing the request as in scope.
fn flat_label_for_scope(nodes: &[Node], id: &str) -> String {
    let Some(node) = nodes.iter().find(|node| node.id == id) else {
        return id.to_string();
    };
    format!("{}: {}", node.name, node.description)
}

/// One Jev choice over every node in scope: `__none__` at a forest walk, no
/// terminal when a `start_node`/scoped walk already fixes the landing. Returns
/// `None` when the scope is too large to name in one call and the recall
/// shortlist has no evidence to stand on — then the beam runs instead.
async fn flat_route(
    state: &AppState,
    nodes: &[Node],
    query: &str,
    request: &RunRequest,
    start: Option<&str>,
    run_id: &str,
    sender: &Option<mpsc::Sender<Value>>,
) -> Result<Option<(Option<String>, Vec<PathPart>, Trace)>, Error> {
    let mode = request.mode.as_str();
    let conversation = render_context(query, &request.context);
    let pool: Vec<String> = match start {
        Some(id) => descendant_ids(nodes, id),
        None => nodes.iter().map(|node| node.id.clone()).collect(),
    };
    if pool.is_empty() {
        return Ok(None);
    }
    let parent_name = start
        .and_then(|id| nodes.iter().find(|node| node.id == id))
        .map(|node| node.name.clone())
        .unwrap_or_else(|| "the knowledge root".into());
    // A forced start on a single node has nothing to choose between; asking Jev
    // there would be theater. A forest walk still asks: `__none__` must be able
    // to abstain even when only one candidate was recalled.
    if start.is_some() && pool.len() == 1 {
        let leaf = pool[0].clone();
        let path = vec![PathPart {
            name: node_name(nodes, &leaf),
            id: leaf.clone(),
            probability: None,
            confidence: None,
        }];
        let trace = Trace {
            run_id: run_id.to_string(),
            mode: mode.to_string(),
            router: "flat".into(),
            steps: vec![],
            leaf_id: Some(leaf.clone()),
            score: None,
            final_beams: vec![FinalBeam {
                node_id: Some(leaf.clone()),
                path: path.clone(),
                score: None,
                alive: false,
            }],
        };
        return Ok(Some((Some(leaf), path, trace)));
    }
    // Every node in scope is the menu whenever one Jev choice can name it: a
    // perfect evaluator must always see the right node, and a request may arrive
    // in any script. Only scopes past the evaluator's limits fall back to
    // lexical recall.
    let menu: Vec<String> = if full_menu_fits(nodes, &pool) {
        pool.clone()
    } else {
        let query_tokens = tokens(query);
        let best = pool
            .iter()
            .map(|id| overlap_score(&query_tokens, query, &node_text(nodes, id, false)))
            .fold(0.0_f64, f64::max);
        if best < FLAT_MIN_EVIDENCE {
            return Ok(None);
        }
        recall_candidates(nodes, query, &pool)
    };
    if menu.is_empty() {
        return Ok(None);
    }
    let mut criteria = serde_json::Map::new();
    for id in &menu {
        criteria.insert(id.clone(), Value::String(flat_label(nodes, &pool, id)));
    }
    if start.is_none() {
        criteria.insert("__none__".into(), Value::String(FLAT_NONE.into()));
    }
    let mut questions = BTreeMap::new();
    questions.insert(
        "flat".into(),
        Question {
            kind: "choice".into(),
            instructions: if start.is_none() {
                format!("{FLAT_RULES} 어느 분류에도 맞지 않으면 __none__을 고르세요.")
            } else {
                format!("{FLAT_RULES} 여기 있는 분류 중에서만 고르세요.")
            },
            criteria: Some(Value::Object(criteria)),
        },
    );
    emit(
        sender,
        json!({"type":"descent_step","status":"asking","depth":0,"run_id":run_id,"frontiers":[{"node_id":start,"path":[],"parent":parent_name,"children":menu.iter().map(|id|json!({"id":id,"name":node_name(nodes,id)})).collect::<Vec<_>>()}]}),
    )
    .await;
    let judgment = state
        .jev
        .evaluate(
            json!({
                "conversation":&conversation,
                "current_request":query,
                "background":&request.context,
                "mode":mode,
                "path":start.map(|id| node_chain(nodes, &pool, id)).unwrap_or_default()
            }),
            questions,
        )
        .await?;
    let (choice, probabilities, confidence) = match judgment.answers.get("flat") {
        Some(Answer::Choice {
            choice,
            probabilities,
            confidence,
            ..
        }) => (choice.clone(), probabilities.clone(), *confidence),
        _ => return Err(Error::Internal("missing flat choice".into())),
    };
    let score = probabilities.get(&choice).copied();
    let mut abstained = choice == "__none__";
    let mut ranked: Vec<(&String, f64)> = menu
        .iter()
        .map(|id| (id, probabilities.get(id).copied().unwrap_or(0.0)))
        .collect();
    ranked.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(b.0)));
    let mut candidates: Vec<Candidate> = ranked
        .into_iter()
        .map(|(id, probability)| Candidate {
            id: Some(id.clone()),
            name: node_name(nodes, id),
            probability: Some(probability),
        })
        .collect();
    if let Some(probability) = probabilities.get("__none__") {
        candidates.push(Candidate {
            id: None,
            name: "no matching topic".into(),
            probability: Some(*probability),
        });
    }
    let step = TraceStep {
        depth: 0,
        node_id: start.map(str::to_string),
        node_name: parent_name.clone(),
        path: vec![],
        candidates,
        choice_id: if abstained {
            None
        } else {
            Some(choice.clone())
        },
        choice_name: if abstained {
            "no matching topic".into()
        } else {
            node_name(nodes, &choice)
        },
        confidence: Some(confidence),
        reason: format!("flat: {mode}: {query}"),
    };
    let mut steps = vec![step];
    if !abstained
        && scope_check_needed(score, start.is_some())
        && let Some((in_scope, scope_step)) =
            scope_check(state, nodes, query, request, &conversation, &choice).await
    {
        steps.push(scope_step);
        if !in_scope {
            abstained = true;
        }
    }
    let leaf = (!abstained).then(|| choice.clone());
    let path: Vec<PathPart> = leaf
        .as_deref()
        .map(|id| {
            node_chain(nodes, &pool, id)
                .into_iter()
                .map(|id| PathPart {
                    name: node_name(nodes, &id),
                    id,
                    probability: None,
                    confidence: None,
                })
                .collect()
        })
        .unwrap_or_default();
    let trace = Trace {
        run_id: run_id.to_string(),
        mode: mode.to_string(),
        router: "flat".into(),
        steps: steps.clone(),
        leaf_id: leaf.clone(),
        score,
        final_beams: vec![FinalBeam {
            node_id: leaf.clone(),
            path: path.clone(),
            score,
            alive: false,
        }],
    };
    emit(
        sender,
        json!({"type":"descent_step","status":"answered","depth":0,"run_id":run_id,"nodes":steps,"beams":[{"node_id":leaf,"path":path,"score":score,"alive":false}]}),
    )
    .await;
    emit(
        sender,
        json!({"type":"descent_done","run_id":run_id,"leaf_id":leaf,"path":path,"score":score,"trace":trace}),
    )
    .await;
    Ok(Some((leaf, path, trace)))
}

/// Lexical evidence of one node: its ancestor names plus its own name and
/// description, optionally the examples. The evidence gate skips examples so a
/// stray example word cannot make an unrelated request look recallable.
fn node_text(nodes: &[Node], id: &str, with_examples: bool) -> String {
    let Some(node) = nodes.iter().find(|node| node.id == id) else {
        return String::new();
    };
    let names = ancestor_names(nodes, id).join(" ");
    if with_examples {
        format!(
            "{names} {} {} {}",
            node.name,
            node.description,
            node.examples.join(" ")
        )
    } else {
        format!("{names} {} {}", node.name, node.description)
    }
}

/// Names of the ancestors of `id`, topmost first.
fn ancestor_names(nodes: &[Node], id: &str) -> Vec<String> {
    let mut chain = Vec::new();
    let mut cursor = nodes
        .iter()
        .find(|node| node.id == id)
        .and_then(|node| node.parent_id.clone());
    while let Some(parent) = cursor {
        let Some(node) = nodes.iter().find(|node| node.id == parent) else {
            break;
        };
        chain.push(node.name.clone());
        cursor = node.parent_id.clone();
    }
    chain.reverse();
    chain
}

/// Candidate label for the flat choice: the node's full path inside the walk
/// plus its own description. The path is what lets the choice tell a node from
/// its siblings and stay at a parent when the request names no deeper topic.
fn flat_label(nodes: &[Node], pool: &[String], id: &str) -> String {
    let names: Vec<String> = node_chain(nodes, pool, id)
        .iter()
        .map(|id| node_name(nodes, id))
        .collect();
    let description = nodes
        .iter()
        .find(|node| node.id == id)
        .map(|node| node.description.chars().take(240).collect::<String>())
        .unwrap_or_default();
    format!("{} | {}", names.join("/"), description)
}

/// Node ids root→leaf inside `pool`, inclusive of `id`.
fn node_chain(nodes: &[Node], pool: &[String], id: &str) -> Vec<String> {
    let mut chain = vec![id.to_string()];
    let mut cursor = nodes
        .iter()
        .find(|node| node.id == id)
        .and_then(|node| node.parent_id.clone());
    while let Some(parent) = cursor {
        if !pool.iter().any(|entry| entry == &parent) {
            break;
        }
        chain.push(parent.clone());
        cursor = nodes
            .iter()
            .find(|node| node.id == parent)
            .and_then(|node| node.parent_id.clone());
    }
    chain.reverse();
    chain
}

/// Bytes one node costs in the flat menu: its id key plus its label.
fn menu_entry_bytes(nodes: &[Node], pool: &[String], id: &str) -> usize {
    id.len() + flat_label(nodes, pool, id).len() + MENU_ENTRY_FRAMING
}

/// True when one Jev choice can name every node in `pool`.
fn full_menu_fits(nodes: &[Node], pool: &[String]) -> bool {
    pool.len() <= FLAT_MAX_NODES
        && pool
            .iter()
            .map(|id| menu_entry_bytes(nodes, pool, id))
            .sum::<usize>()
            <= FLAT_MAX_MENU_BYTES
}

/// Put `id` and the ancestors it brings along into the menu, unless that would
/// take it past what one Jev choice can read. The first node always goes in.
fn push_with_ancestors(
    nodes: &[Node],
    pool: &[String],
    id: &str,
    out: &mut Vec<String>,
    seen: &mut HashSet<String>,
    bytes: &mut usize,
) {
    let fresh: Vec<String> = node_chain(nodes, pool, id)
        .into_iter()
        .filter(|id| !seen.contains(id))
        .collect();
    let cost: usize = fresh
        .iter()
        .map(|id| menu_entry_bytes(nodes, pool, id))
        .sum();
    if !out.is_empty()
        && (out.len() + fresh.len() > FLAT_MAX_NODES || *bytes + cost > FLAT_MAX_MENU_BYTES)
    {
        return;
    }
    *bytes += cost;
    for id in fresh {
        seen.insert(id.clone());
        out.push(id);
    }
}

/// Recall-first candidate set for the flat choice, never larger than one Jev
/// choice can read ([`FLAT_MAX_NODES`], [`FLAT_MAX_MENU_BYTES`]). The strongest
/// matches come first, then the rest of the budget goes to the next best nodes:
/// a menu that stops at a few dozen leaves most of what one choice can read
/// unused, and the node the request meant is more often missing from it (route
/// accuracy on the golden queries: 94% with the full menu, 79% with a recall
/// menu of ~47 nodes, 93% with the same recall menu filled to 254).
fn recall_candidates(nodes: &[Node], query: &str, pool: &[String]) -> Vec<String> {
    let query_tokens = tokens(query);
    let mut scored: Vec<(String, f64)> = pool
        .iter()
        .map(|id| {
            (
                id.clone(),
                overlap_score(&query_tokens, query, &node_text(nodes, id, true)),
            )
        })
        .collect();
    scored.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let mut picked: Vec<String> = Vec::new();
    let mut per_root: HashMap<String, usize> = HashMap::new();
    for (id, score) in &scored {
        if *score <= 0.0 {
            break;
        }
        let root = node_chain(nodes, pool, id)[0].clone();
        let used = per_root.entry(root).or_insert(0);
        if picked.len() >= FLAT_GLOBAL_K && *used >= FLAT_PER_ROOT {
            continue;
        }
        *used += 1;
        picked.push(id.clone());
    }
    // A pick that no longer fits is skipped, not the end of the list: a later
    // match may be cheaper.
    let mut out: Vec<String> = Vec::new();
    let mut seen = HashSet::new();
    let mut bytes = 0;
    for id in &picked {
        push_with_ancestors(nodes, pool, id, &mut out, &mut seen, &mut bytes);
    }
    for (id, _) in &scored {
        if out.len() >= FLAT_MAX_NODES || bytes >= FLAT_MAX_MENU_BYTES {
            break;
        }
        if !seen.contains(id) {
            push_with_ancestors(nodes, pool, id, &mut out, &mut seen, &mut bytes);
        }
    }
    out
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

/// Request bytes (state plus questions) one ranking call may carry. The
/// evaluator reads about 32K tokens: a batch of real Korean items passed at
/// 100 KB and failed at 116 KB (HTTP 400). 24 KB stays under that even when
/// every byte costs a token. A batch of 32 ordinary items is well below it, so
/// the item count still decides there and those batches do not change.
const RANK_BATCH_BYTES: usize = 24_000;

/// Longest question and answer, in characters, the evaluator is shown. The API
/// takes at most 8,000 characters of question plus answer, so nothing it stores
/// is ever cut (the longest answer in the live databases is 6,963 characters);
/// this only bounds a row that got into the database some other way. Longer text
/// is cut with an ellipsis before it is sent, and the stored item and the result
/// keep it whole.
const RANK_QUESTION_CHARS: usize = 8_000;
const RANK_ANSWER_CHARS: usize = 8_000;

/// Ranking calls in flight at once. Batches are consumed in order, so this
/// changes the wall clock and nothing else.
const RANK_CONCURRENCY: usize = 4;

/// Score arithmetic runs in f64: `0.5 * 0.7 + 1.0 * 0.3` is 0.6499999999999999,
/// one step under the 0.65 line it sits on. A line is met when the score is
/// within this much below it.
const SCORE_EPSILON: f64 = 1e-9;

fn at_least(value: f64, line: f64) -> bool {
    value + SCORE_EPSILON >= line
}

/// `recommended` >= 0.65, `alternative` >= 0.40, else `reference`.
fn role_for(score: f64) -> &'static str {
    if at_least(score, 0.65) {
        "recommended"
    } else if at_least(score, 0.4) {
        "alternative"
    } else {
        "reference"
    }
}

/// Search holds instead of answering when the best item is weak: below
/// [`RANK_MIN_SCORE`], unless it is a direct item in the gray zone.
fn holds(score: f64, directness: f64) -> bool {
    !at_least(score, RANK_MIN_SCORE)
        && !(at_least(score, RANK_GRAY_ZONE) && at_least(directness, RANK_DIRECT_MIN))
}

/// First `max` characters of `text`, with an ellipsis when something was cut.
fn clip_chars(text: &str, max: usize) -> String {
    match text.char_indices().nth(max) {
        Some((end, _)) => format!("{}…", &text[..end]),
        None => text.to_string(),
    }
}

/// The item as the evaluator sees it.
fn rank_view(item: &Item) -> Item {
    Item {
        question: clip_chars(&item.question, RANK_QUESTION_CHARS),
        answer: clip_chars(&item.answer, RANK_ANSWER_CHARS),
        ..item.clone()
    }
}

/// The two questions asked about one item.
fn rank_questions(item: &Item) -> [(String, Question); 2] {
    [
        (
            format!("use_{}", item.id),
            Question {
                kind: "noul".into(),
                instructions: format!("Could item {} be sent directly as a reply?", item.id),
                criteria: None,
            },
        ),
        (
            format!("direct_{}", item.id),
            Question {
                kind: "score".into(),
                instructions: format!("How directly does item {} answer the request?", item.id),
                criteria: Some(json!(["unrelated", "useful", "direct"])),
            },
        ),
    ]
}

/// Bytes one item adds to a ranking call: itself plus its two questions.
fn rank_item_bytes(view: &Item) -> usize {
    let questions: BTreeMap<String, Question> = rank_questions(view).into_iter().collect();
    serde_json::to_string(view).map_or(0, |text| text.len())
        + serde_json::to_string(&questions).map_or(0, |text| text.len())
}

/// Split the items into consecutive batches of at most [`RANK_BATCH`] items and
/// [`RANK_BATCH_BYTES`] of request, `overhead` bytes of which every call spends
/// on the request itself. An item too big for any batch still gets its own.
fn rank_batches(views: &[Item], overhead: usize) -> Vec<std::ops::Range<usize>> {
    let mut ranges = Vec::new();
    let (mut start, mut bytes) = (0, overhead);
    for (index, view) in views.iter().enumerate() {
        let cost = rank_item_bytes(view);
        if index > start && (index - start >= RANK_BATCH || bytes + cost > RANK_BATCH_BYTES) {
            ranges.push(start..index);
            start = index;
            bytes = overhead;
        }
        bytes += cost;
    }
    if start < views.len() {
        ranges.push(start..views.len());
    }
    ranges
}

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
    let conversation = render_context(query, context);
    let views: Vec<Item> = items.iter().map(rank_view).collect();
    let overhead = serde_json::to_string(
        &json!({"conversation": conversation, "current_request": query, "items": []}),
    )
    .map_or(0, |text| text.len());
    let batches = rank_batches(&views, overhead);
    // Batches run a few at a time and are consumed in order: the evaluator is the
    // slow part, and the result must not depend on which call returns first. The
    // stream is dropped on the first error or a closed listener, which cancels
    // the calls still in flight.
    let jev = &state.jev;
    let mut judged = futures_util::stream::iter(batches.into_iter().map(|range| {
        let chunk = &views[range.clone()];
        let questions: BTreeMap<String, Question> = chunk.iter().flat_map(rank_questions).collect();
        let payload =
            json!({"conversation": &conversation, "current_request": query, "items": chunk});
        async move {
            jev.evaluate(payload, questions)
                .await
                .map(|judgment| (range, judgment))
        }
    }))
    .buffered(RANK_CONCURRENCY);
    let mut ranked = Vec::with_capacity(items.len());
    while let Some(batch) = judged.next().await {
        let (range, judgment) = batch?;
        // Nobody is listening any more: stop before paying for more calls.
        if sender.as_ref().is_some_and(mpsc::Sender::is_closed) {
            return Err(Error::Cancelled);
        }
        for item in &items[range] {
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
            let ranked_item = RankedItem {
                item: item.clone(),
                relevance,
                directness,
                confidence: None,
                role: role_for(score).into(),
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

    fn wide_state() -> (tempfile::TempDir, AppState) {
        // The flat route names every node in scope in one call; past
        // `FLAT_MAX_NODES` it hands over to lexical recall and, without
        // evidence, to the beam. Pad the fixture past that line to reach the
        // beam path with a scripted evaluator.
        padded_state(FLAT_MAX_NODES)
    }

    /// The mini fixture plus one `pad` node with `pads` children: the scope
    /// holds `4 + pads` nodes.
    fn padded_state(pads: usize) -> (tempfile::TempDir, AppState) {
        let dir = tempfile::tempdir().unwrap();
        let db = dir.path().join("t.db");
        let mini =
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/mini-seed.json");
        let mut seed: Value =
            serde_json::from_str(&std::fs::read_to_string(&mini).unwrap()).unwrap();
        let nodes = seed["nodes"].as_array_mut().unwrap();
        nodes.push(json!({"id":"pad","name":"패딩","description":"트리 크기 측정용 패딩 분류입니다.","examples":[],"parent_id":null}));
        for index in 0..pads {
            nodes.push(json!({
                "id": format!("pad_{index}"),
                "name": format!("패딩 {index}"),
                "description": "트리 크기 측정용 패딩 노드입니다.",
                "examples": [],
                "parent_id": "pad"
            }));
        }
        let seed_path = dir.path().join("seed.json");
        std::fs::write(&seed_path, serde_json::to_string(&seed).unwrap()).unwrap();
        // SAFETY: test-only bootstrap, called before this fixture's store opens.
        unsafe {
            std::env::set_var("JEV_TREE_DB", db.to_str().unwrap());
            std::env::set_var(
                "JEV_TREE_SECRET_KEY",
                "test-secret-material-for-jev-tree-engine",
            );
        }
        let store = Store::open(db.to_str().unwrap(), seed_path.to_str().unwrap()).unwrap();
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
        // The beam keeps its terminal option: `__stop__` wins and stops expansion.
        // The scope is too large for the flat menu, so the beam runs.
        let (_dir, state) = wide_state();
        state
            .jev
            .script_choices(vec!["orders".into(), "__stop__".into()]);
        let result = execute(state, run_req("where is my order"), None)
            .await
            .unwrap();
        assert_eq!(result["leaf_id"], "orders", "{result}");
        assert_eq!(result["trace"]["router"], "beam");
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
        let (_dir, state) = wide_state();
        state.jev.script_choices(vec!["account".into()]);
        let result = execute(state, run_req("reset my password please"), None)
            .await
            .unwrap();
        assert_eq!(result["leaf_id"], "account", "{result}");
        assert_eq!(result["trace"]["router"], "beam");
        let beams = result["trace"]["final_beams"].as_array().unwrap();
        assert_eq!(beams[0]["node_id"], "account");
    }

    #[tokio::test]
    async fn scripted_choice_reaches_tracking_leaf() {
        // The flat route asks one choice question over every node in scope.
        let (_dir, state) = fixture_state();
        state.jev.script_choices(vec!["orders_tracking".into()]);
        let result = execute(state, run_req("해외 배송이 통관에서 멈췄어요"), None)
            .await
            .unwrap();
        assert_eq!(result["leaf_id"], "orders_tracking", "{result}");
        assert_eq!(result["trace"]["router"], "flat");
        assert!(result["trace"]["score"].as_f64().is_some(), "{result}");
        assert!(!result["abstained"].as_bool().unwrap());
    }

    #[tokio::test]
    async fn flat_choice_routes_a_request_in_another_script() {
        // The menu is every node in scope, so the evaluator judges meaning even
        // when the request shares no characters with the taxonomy.
        let (_dir, state) = fixture_state();
        state.jev.script_choices(vec!["orders_tracking".into()]);
        let result = execute(state, run_req("where is my parcel stuck at customs"), None)
            .await
            .unwrap();
        assert_eq!(result["leaf_id"], "orders_tracking", "{result}");
        assert_eq!(result["trace"]["router"], "flat");
    }

    #[tokio::test]
    async fn scope_check_says_out_of_scope() {
        // One judgment on scope alone turns an uncertain route into a hold.
        let (_dir, state) = fixture_state();
        state.jev.script_choices(vec!["out".into()]);
        let request = run_req("오늘 날씨가 어때요?");
        let (in_scope, step) = scope_check(
            &state,
            &state.taxonomy().unwrap(),
            "오늘 날씨가 어때요?",
            &request,
            "오늘 날씨가 어때요?",
            "orders",
        )
        .await
        .expect("scripted evaluator answers");
        assert!(!in_scope);
        assert_eq!(step.choice_name, "request is out of scope");
    }

    #[tokio::test]
    async fn confident_route_skips_the_scope_check() {
        // Scripted choices land at 0.95, above the check line: the second pick
        // in the script would say "out" and must never be consulted.
        let (_dir, state) = fixture_state();
        state
            .jev
            .script_choices(vec!["orders_tracking".into(), "out".into()]);
        let result = execute(state, run_req("해외 배송이 통관에서 멈췄어요"), None)
            .await
            .unwrap();
        assert_eq!(result["leaf_id"], "orders_tracking", "{result}");
        assert_eq!(
            result["trace"]["steps"].as_array().unwrap().len(),
            1,
            "{result}"
        );
    }

    #[test]
    fn a_timid_out_does_not_refuse_the_request() {
        assert!(scope_refuses("out", 0.8));
        assert!(scope_refuses("out", 0.95));
        assert!(!scope_refuses("out", 0.79));
        assert!(!scope_refuses("in", 0.95));
    }

    #[test]
    fn scope_check_guards_only_uncertain_forest_walks() {
        assert!(scope_check_needed(Some(0.64), false));
        assert!(scope_check_needed(Some(0.84), false));
        assert!(!scope_check_needed(Some(0.85), false));
        assert!(!scope_check_needed(Some(0.99), false));
        // A forced start must land on a node; it never refuses the request.
        assert!(!scope_check_needed(Some(0.10), true));
    }

    #[tokio::test]
    async fn flat_choice_none_abstains() {
        // `__none__` competes in the same choice and leaves the tree.
        let (_dir, state) = fixture_state();
        state.jev.script_choices(vec!["__none__".into()]);
        let result = execute(state, run_req("해외 배송이 통관에서 멈췄어요"), None)
            .await
            .unwrap();
        assert!(result["leaf_id"].is_null(), "{result}");
        assert_eq!(result["items"].as_array().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn flat_choice_can_stay_at_an_internal_node() {
        // Picking a parent is the flat route's "stay here": no descent below it.
        let (_dir, state) = fixture_state();
        state.jev.script_choices(vec!["orders".into()]);
        let result = execute(state, run_req("주문이 취소됐어요"), None)
            .await
            .unwrap();
        assert_eq!(result["leaf_id"], "orders", "{result}");
        let path = result["path"].as_array().unwrap();
        assert_eq!(path.len(), 1, "{result}");
        assert_eq!(path[0]["id"], "orders");
    }

    #[tokio::test]
    async fn beam_fallback_on_a_scope_too_large_to_name() {
        // Above `ONE_SHOT_MAX_NODES` the flat menu falls back to lexical recall,
        // and without lexical evidence the semantic beam runs instead.
        let (_dir, state) = wide_state();
        state
            .jev
            .script_choices(vec!["orders".into(), "orders_tracking".into()]);
        let result = execute(state, run_req("where is my parcel stuck at customs"), None)
            .await
            .unwrap();
        assert_eq!(result["leaf_id"], "orders_tracking", "{result}");
        assert_eq!(result["trace"]["router"], "beam");
    }

    #[tokio::test]
    async fn draft_ingest_persists_without_publish() {
        let (_dir, state) = fixture_state();
        state.jev.script_choices(vec!["orders_tracking".into()]);
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
    async fn llm_failure_falls_back_to_jev_routing() {
        let (_dir, state) = fixture_state();
        state.jev.reconfigure(crate::jev::JevConfig {
            llm_base_url: "http://127.0.0.1:9".into(),
            llm_token: Some("not-a-real-token".into()),
            llm_model: "unused".into(),
            ..crate::jev::JevConfig::default()
        });
        state.jev.script_choices(vec!["orders_tracking".into()]);
        let result = execute(state, run_req("해외 배송이 통관에서 멈췄어요"), None)
            .await
            .unwrap();
        assert_eq!(result["leaf_id"], "orders_tracking", "{result}");
        assert_eq!(result["trace"]["router"], "flat");
        assert!(result["trace"]["score"].as_f64().is_some(), "{result}");
    }

    #[test]
    fn llm_route_budget_leaves_room_for_the_beam_and_ranking() {
        // A third of the run deadline: enough for a slow model to deliberate
        // and still leave the beam and the ranking time to finish.
        assert_eq!(llm_route_budget(), run_deadline() / 3);
        assert!(llm_route_budget() < run_deadline());
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
        state.jev.script_choices(vec!["topic_case".into()]);
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

    fn plain_node(id: &str, name: &str, description: &str, parent: Option<&str>) -> Node {
        Node {
            id: id.into(),
            name: name.into(),
            description: description.into(),
            examples: vec![],
            parent_id: parent.map(str::to_string),
        }
    }

    fn ids_of(nodes: &[Node]) -> Vec<String> {
        nodes.iter().map(|node| node.id.clone()).collect()
    }

    #[test]
    fn full_menu_is_limited_by_option_count_and_by_bytes() {
        let short: Vec<Node> = (0..FLAT_MAX_NODES)
            .map(|i| plain_node(&format!("n{i}"), &format!("노드 {i}"), "짧은 설명", None))
            .collect();
        assert!(full_menu_fits(&short, &ids_of(&short)));
        // One node more and `__none__` would be the 256th option: Jev answers 422.
        let mut one_more = short.clone();
        one_more.push(plain_node("extra", "추가", "짧은 설명", None));
        assert!(!full_menu_fits(&one_more, &ids_of(&one_more)));
        // Under the count line but over the context window: the labels are clipped
        // at 240 characters, which is still ~700 bytes of Korean per node.
        let long = "가".repeat(240);
        let heavy: Vec<Node> = (0..150)
            .map(|i| plain_node(&format!("h{i}"), &format!("무거운 노드 {i}"), &long, None))
            .collect();
        assert!(heavy.len() <= FLAT_MAX_NODES);
        assert!(!full_menu_fits(&heavy, &ids_of(&heavy)));
    }

    #[test]
    fn recall_menu_never_outgrows_one_choice() {
        // 400 roots that all share the request's words: every node "matches".
        let many: Vec<Node> = (0..400)
            .map(|i| {
                plain_node(
                    &format!("n{i}"),
                    &format!("분류 {i}"),
                    "환불 문의를 다루는 분류",
                    None,
                )
            })
            .collect();
        let pool = ids_of(&many);
        let menu = recall_candidates(&many, "환불 문의", &pool);
        assert!(!menu.is_empty());
        assert!(menu.len() <= FLAT_MAX_NODES, "{} nodes", menu.len());
        // The same through the byte line: heavy labels end the menu early.
        let long = "환불".repeat(120);
        let heavy: Vec<Node> = (0..300)
            .map(|i| plain_node(&format!("h{i}"), &format!("무거운 {i}"), &long, None))
            .collect();
        let pool = ids_of(&heavy);
        let menu = recall_candidates(&heavy, "환불 문의", &pool);
        let bytes: usize = menu
            .iter()
            .map(|id| menu_entry_bytes(&heavy, &pool, id))
            .sum();
        assert!(!menu.is_empty());
        assert!(bytes <= FLAT_MAX_MENU_BYTES, "{bytes} bytes");
    }

    #[test]
    fn recall_menu_keeps_the_ancestors_of_a_match() {
        // 30 topics with 10 children each; only one child mentions the request.
        let mut nodes = Vec::new();
        for topic in 0..30 {
            nodes.push(plain_node(
                &format!("t{topic}"),
                &format!("주제 {topic}"),
                "일반 주제",
                None,
            ));
            for child in 0..10 {
                let description = if topic == 17 && child == 4 {
                    "해외 결제 수수료 문의"
                } else {
                    "일반 하위 분류"
                };
                nodes.push(plain_node(
                    &format!("t{topic}_{child}"),
                    &format!("하위 {topic}-{child}"),
                    description,
                    Some(&format!("t{topic}")),
                ));
            }
        }
        let pool = ids_of(&nodes);
        assert!(pool.len() > FLAT_MAX_NODES);
        let menu = recall_candidates(&nodes, "해외 결제 수수료", &pool);
        assert!(menu.contains(&"t17_4".to_string()), "{menu:?}");
        assert!(
            menu.contains(&"t17".to_string()),
            "parent must stay visible"
        );
    }

    #[tokio::test]
    async fn the_full_menu_is_used_up_to_the_option_limit() {
        let (_dir, state) = padded_state(FLAT_MAX_NODES - 4);
        assert_eq!(state.taxonomy().unwrap().len(), FLAT_MAX_NODES);
        state.jev.script_choices(vec!["pad_7".into()]);
        let result = execute(state, run_req("패딩 노드 문의"), None)
            .await
            .unwrap();
        assert_eq!(result["trace"]["router"], "flat", "{result}");
        assert_eq!(result["leaf_id"], "pad_7", "{result}");
        // Every node plus `__none__`: the most one Jev choice accepts.
        let offered = result["trace"]["steps"][0]["candidates"]
            .as_array()
            .unwrap();
        assert_eq!(offered.len(), FLAT_MAX_NODES + 1);
    }

    #[tokio::test]
    async fn a_recall_menu_is_filled_up_to_what_one_choice_can_read() {
        // 300 nodes, and the request only names a word of the padding. The
        // matches alone would be a menu of a few dozen; the rest of the room
        // goes to the next best nodes, so the node meant is rarely left out.
        let (_dir, state) = padded_state(FLAT_MAX_NODES + 50 - 4);
        assert_eq!(state.taxonomy().unwrap().len(), FLAT_MAX_NODES + 50);
        state.jev.script_choices(vec!["pad_0".into()]);
        let result = execute(state, run_req("패딩 노드 문의"), None)
            .await
            .unwrap();
        assert_eq!(result["trace"]["router"], "flat", "{result}");
        let offered = result["trace"]["steps"][0]["candidates"]
            .as_array()
            .unwrap();
        // 254 nodes plus `__none__`: the most one Jev choice accepts.
        assert_eq!(offered.len(), FLAT_MAX_NODES + 1);
    }

    #[test]
    fn the_filled_menu_keeps_matches_first_and_stays_within_the_byte_line() {
        // The one node that mentions the request must be in, with its parent,
        // however much filler follows it.
        let mut nodes = vec![plain_node("topic", "주제", "일반 주제", None)];
        for i in 0..400 {
            let description = if i == 321 {
                "해외 결제 수수료 문의"
            } else {
                "그 밖의 일반 안내"
            };
            nodes.push(plain_node(
                &format!("n{i:03}"),
                &format!("분류 {i}"),
                description,
                Some("topic"),
            ));
        }
        let pool = ids_of(&nodes);
        let menu = recall_candidates(&nodes, "해외 결제 수수료", &pool);
        assert_eq!(menu.len(), FLAT_MAX_NODES, "the budget is spent");
        assert!(menu.contains(&"n321".to_string()), "the match is in");
        assert!(menu.contains(&"topic".to_string()), "with its parent");
        let bytes: usize = menu
            .iter()
            .map(|id| menu_entry_bytes(&nodes, &pool, id))
            .sum();
        assert!(bytes <= FLAT_MAX_MENU_BYTES, "{bytes} bytes");
        // Same answer twice: the menu does not depend on iteration order.
        assert_eq!(menu, recall_candidates(&nodes, "해외 결제 수수료", &pool));
    }

    #[tokio::test]
    async fn a_scope_past_the_option_limit_is_routed_from_a_recall_menu() {
        // One node more than a choice can name used to reach Jev anyway and fail
        // with HTTP 422 on every search. It now names the lexical matches instead.
        let (_dir, state) = padded_state(FLAT_MAX_NODES - 3);
        assert_eq!(state.taxonomy().unwrap().len(), FLAT_MAX_NODES + 1);
        state.jev.script_choices(vec!["pad_0".into()]);
        let result = execute(state, run_req("패딩 노드 문의"), None)
            .await
            .unwrap();
        assert_eq!(result["trace"]["router"], "flat", "{result}");
        assert_eq!(result["leaf_id"], "pad_0", "{result}");
        let offered = result["trace"]["steps"][0]["candidates"]
            .as_array()
            .unwrap();
        assert!(
            offered.len() > 1 && offered.len() <= FLAT_MAX_NODES + 1,
            "{} options",
            offered.len()
        );
    }

    fn sized_item(id: usize, chars: usize) -> Item {
        Item {
            id: id.to_string(),
            category_id: "c".into(),
            question: format!("질문 {id}"),
            answer: "가".repeat(chars),
            kind: "qa".into(),
            status: "active".into(),
            version: 1,
        }
    }

    #[test]
    fn clip_chars_cuts_on_character_boundaries() {
        assert_eq!(clip_chars("가나다라", 2), "가나\u{2026}");
        assert_eq!(clip_chars("가나", 2), "가나");
        assert_eq!(clip_chars("abcdef", 3), "abc\u{2026}");
        assert_eq!(clip_chars("", 5), "");
    }

    #[test]
    fn rank_batches_follow_the_item_count_for_ordinary_items() {
        // Ordinary items never reach the byte line, so those batches stay as they were.
        let views: Vec<Item> = (0..70).map(|i| rank_view(&sized_item(i, 120))).collect();
        assert_eq!(rank_batches(&views, 200), vec![0..32, 32..64, 64..70]);
    }

    #[test]
    fn rank_batches_split_big_items_by_bytes() {
        // ~9 KB of Korean text per item: two fit one call, a third does not.
        let views: Vec<Item> = (0..7).map(|i| rank_view(&sized_item(i, 3000))).collect();
        let ranges = rank_batches(&views, 200);
        assert!(ranges.len() >= 3, "{ranges:?}");
        let mut next = 0;
        for range in &ranges {
            assert_eq!(range.start, next, "batches must be consecutive");
            next = range.end;
            let bytes = 200
                + views[range.clone()]
                    .iter()
                    .map(rank_item_bytes)
                    .sum::<usize>();
            assert!(
                bytes <= RANK_BATCH_BYTES || range.len() == 1,
                "{range:?} carries {bytes} bytes"
            );
        }
        assert_eq!(next, views.len(), "every item is in exactly one batch");
    }

    #[test]
    fn one_huge_item_is_clipped_and_gets_a_batch_of_its_own() {
        let huge = sized_item(1, 100_000);
        let view = rank_view(&huge);
        assert_eq!(view.answer.chars().count(), RANK_ANSWER_CHARS + 1);
        assert!(view.answer.ends_with('\u{2026}'));
        assert_eq!(
            huge.answer.chars().count(),
            100_000,
            "the stored row is untouched"
        );
        assert_eq!(rank_batches(std::slice::from_ref(&view), 500), vec![0..1]);
        // ~24 KB of Korean: past the batch line, far under what the evaluator reads.
        assert!(
            rank_item_bytes(&view) < 32_000,
            "{}",
            rank_item_bytes(&view)
        );
        let short = sized_item(2, 10);
        assert_eq!(rank_view(&short).answer, short.answer);
    }

    #[test]
    fn nothing_the_api_can_store_is_clipped() {
        // Question plus answer is capped at 8,000 characters on the way in.
        let longest = Item {
            question: "질".repeat(100),
            answer: "답".repeat(7_900),
            ..sized_item(3, 0)
        };
        let view = rank_view(&longest);
        assert_eq!(view.question, longest.question);
        assert_eq!(view.answer, longest.answer);
    }

    #[tokio::test]
    async fn a_subtree_of_big_items_is_ranked_whole_and_in_a_stable_order() {
        // 31 items of 5,000 characters is ~460 KB: far past one call, so the
        // ranking runs as many calls, a few at a time. The result must not depend
        // on which one finishes first, and must carry the stored text, not the
        // clipped copy the evaluator saw.
        let (_dir, state) = fixture_state();
        state.jev.script_choices(vec!["orders_tracking".into()]);
        for n in 0..30 {
            state
                .store
                .upsert(
                    "orders_tracking",
                    &format!("긴 답변 {n}"),
                    &"가".repeat(8000),
                    "qa",
                    None,
                    None,
                    true,
                )
                .unwrap();
        }
        let (tx, mut rx) = mpsc::channel(256);
        let mut request = run_req("해외 배송 조회");
        request.limit = 20;
        let result = execute(state, request, Some(tx)).await.unwrap();
        let mut scored = 0;
        while let Some(event) = rx.recv().await {
            if event["type"] == "candidate_evaluated" {
                scored += 1;
            }
        }
        assert_eq!(scored, 31, "every item must be scored");
        let items = result["items"].as_array().unwrap();
        assert_eq!(items.len(), 20);
        let ids: Vec<&str> = items
            .iter()
            .map(|ranked| ranked["item"]["id"].as_str().unwrap())
            .collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        assert_eq!(ids, sorted, "equal scores keep id order: {ids:?}");
        assert!(
            items.iter().any(|ranked| {
                ranked["item"]["answer"].as_str().unwrap().chars().count() == 8000
            })
        );
    }

    #[test]
    fn role_and_hold_lines_include_their_exact_value() {
        // These sums are mathematically on the line but one step under it in f64.
        let on_recommended = 0.5 * 0.7 + (2.0_f64 / 2.0) * 0.3;
        let on_alternative = 0.25 * 0.7 + (1.5_f64 / 2.0) * 0.3;
        let on_hold = 0.6 * 0.7 + (0.2_f64 / 2.0) * 0.3;
        assert!(on_recommended < 0.65 && on_alternative < 0.4 && on_hold < 0.45);
        assert_eq!(role_for(on_recommended), "recommended");
        assert_eq!(role_for(on_alternative), "alternative");
        assert!(!holds(on_hold, 0.2), "0.45 answers outright");
        // A real step under a line still falls on the lower side.
        assert_eq!(role_for(0.649), "alternative");
        assert_eq!(role_for(0.399), "reference");
        assert!(holds(0.449, 0.99));
        // The gray zone answers only a direct item.
        assert!(!holds(0.30, 1.0));
        assert!(holds(0.30, 0.9));
        assert!(holds(0.299, 2.0));
    }

    fn ingest_request(question: &str, answer: &str) -> RunRequest {
        RunRequest {
            mode: "ingest".into(),
            question: question.into(),
            answer: answer.into(),
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn publishing_a_draft_files_it_where_it_was_reviewed() {
        let (_dir, state) = fixture_state();
        state.jev.script_choices(vec!["orders_tracking".into()]);
        let draft = execute(
            state.clone(),
            ingest_request("통관이 왜 멈추나요?", "개인통관번호를 확인하세요."),
            None,
        )
        .await
        .unwrap();
        let id = draft["item_id"].as_str().unwrap().to_string();
        // The reviewer rewrote the answer. Routing the new text again would now
        // say "nowhere" and the reviewed draft could never be published.
        state.jev.script_choices(vec!["__none__".into()]);
        let mut publish = ingest_request("통관이 왜 멈추나요?", "검토 후 고친 답변입니다.");
        publish.item_id = Some(id.clone());
        publish.expected_version = Some(1);
        publish.auto_publish = true;
        let (tx, mut rx) = mpsc::channel(64);
        let published = execute(state.clone(), publish, Some(tx)).await.unwrap();
        assert_eq!(published["published_id"], id.as_str(), "{published}");
        assert_eq!(published["category_id"], "orders_tracking");
        assert_eq!(published["trace"]["router"], "flat");
        assert!(published["trace"]["steps"].as_array().unwrap().is_empty());
        let path = published["path"].as_array().unwrap();
        assert_eq!(path.last().unwrap()["id"], "orders_tracking");
        let mut types = vec![];
        while let Some(event) = rx.recv().await {
            types.push(event["type"].as_str().unwrap_or_default().to_string());
        }
        assert_eq!(types, ["descent_done", "ingest_saved"]);
        let row = state.store.editable_item(&id).unwrap().unwrap();
        assert_eq!(row.status, "active");
        assert_eq!(row.answer, "검토 후 고친 답변입니다.");
        assert_eq!(row.version, 2);
    }

    #[tokio::test]
    async fn publishing_an_unknown_item_is_not_found_before_any_routing() {
        let (_dir, state) = fixture_state();
        state.jev.script_choices(vec!["orders_tracking".into()]);
        let mut request = ingest_request("통관이 왜 멈추나요?", "개인통관번호를 확인하세요.");
        request.item_id = Some("9999".into());
        request.expected_version = Some(1);
        request.auto_publish = true;
        let error = execute(state, request, None).await.unwrap_err();
        assert!(matches!(error, Error::NotFound(_)), "{error}");
    }

    #[tokio::test]
    async fn a_start_node_still_re_files_an_edit() {
        let (_dir, state) = fixture_state();
        state.jev.script_choices(vec!["orders_tracking".into()]);
        let draft = execute(
            state.clone(),
            ingest_request("비밀번호를 재설정하려면?", "메일을 요청하세요."),
            None,
        )
        .await
        .unwrap();
        assert_eq!(draft["category_id"], "orders_tracking");
        // Asking for a place is the way to move a row: `start_node` routes afresh.
        let mut edit = ingest_request("비밀번호를 재설정하려면?", "메일을 요청하세요.");
        edit.item_id = draft["item_id"].as_str().map(str::to_string);
        edit.expected_version = Some(1);
        edit.auto_publish = true;
        edit.start_node = Some("account".into());
        let moved = execute(state.clone(), edit, None).await.unwrap();
        assert_eq!(moved["category_id"], "account", "{moved}");
    }

    #[tokio::test]
    async fn a_stored_category_outside_the_root_is_routed_afresh() {
        let (_dir, state) = fixture_state();
        state.jev.script_choices(vec!["orders_tracking".into()]);
        let draft = execute(
            state.clone(),
            ingest_request("비밀번호를 재설정하려면?", "메일을 요청하세요."),
            None,
        )
        .await
        .unwrap();
        // The draft sits under `orders`; a request scoped to `account` cannot keep
        // it there, so it is routed inside that root as before.
        state.jev.script_choices(vec!["__stop__".into()]);
        let mut edit = ingest_request("비밀번호를 재설정하려면?", "메일을 요청하세요.");
        edit.item_id = draft["item_id"].as_str().map(str::to_string);
        edit.expected_version = Some(1);
        edit.auto_publish = true;
        edit.root = Some("account".into());
        let moved = execute(state, edit, None).await.unwrap();
        assert_eq!(moved["category_id"], "account", "{moved}");
    }
}
