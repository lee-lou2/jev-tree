use crate::AppState;
use crate::error::Error;
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

/// Tell the listener what one model call cost: a `usage` event per call, so a
/// run's price can be read off its own stream.
async fn emit_usage(sender: &Option<mpsc::Sender<Value>>, source: &str, usage: Usage) {
    emit(
        sender,
        json!({"type":"usage","source":source,"input_tokens":usage.input_tokens,"output_tokens":usage.output_tokens}),
    )
    .await;
}

/// One evaluator call, reported as a `usage` event.
async fn judge(
    state: &AppState,
    sender: &Option<mpsc::Sender<Value>>,
    payload: Value,
    questions: BTreeMap<String, Question>,
) -> Result<Judgment, crate::jev::JevError> {
    let judgment = state.jev.evaluate(payload, questions).await?;
    emit_usage(sender, "jev", judgment.usage).await;
    Ok(judgment)
}

pub async fn execute(
    state: AppState,
    request: RunRequest,
    sender: Option<mpsc::Sender<Value>>,
) -> Result<Value, Error> {
    // Tests pin the evaluator or the LLM. A real run needs a Jev key: nothing is
    // routed without one, and the optional LLM only reviews Jev's unsure picks.
    if !state.jev.choice_scripted() && !state.jev.llm_pinned() && !state.jev.key_set() {
        return Err(Error::BadRequest(
            "Jev API key is required. Add it in Settings → Models.".into(),
        ));
    }
    let started = std::time::Instant::now();
    let mode = request.mode.clone();
    let text = request.text();
    // A search asked before, against the same knowledge and settings, is
    // answered from the cache: the route and the ranking would come out the same.
    let cache_key = (mode == "search").then(|| {
        crate::cache::ResultCache::key(&[
            &text,
            &serde_json::to_string(&request.context).unwrap_or_default(),
            request.root.as_deref().unwrap_or(""),
            request.start_node.as_deref().unwrap_or(""),
            &request.limit.to_string(),
            &state.store.generation().to_string(),
            &state.jev.generation().to_string(),
        ])
    });
    if let Some(cached) = cache_key.as_ref().and_then(|key| state.results.get(key)) {
        emit(&sender, json!({"type":"cache_hit"})).await;
        emit(
            &sender,
            json!({"type":"descent_done","run_id":cached["trace"]["run_id"],"leaf_id":cached["leaf_id"],"path":cached["path"],"score":cached["trace"]["score"],"trace":cached["trace"]}),
        )
        .await;
        emit(&sender, json!({"type":"search_done","leaf_id":cached["leaf_id"],"abstained":cached["abstained"],"items":cached["items"],"results":cached["items"],"query":cached["query"],"path":cached["path"],"trace":cached["trace"]})).await;
        tracing::info!(
            mode = "search",
            cached = true,
            root = request.root.as_deref().unwrap_or(""),
            leaf = cached["leaf_id"].as_str().unwrap_or(""),
            abstained = cached["abstained"].as_bool().unwrap_or(false),
            ms = started.elapsed().as_millis() as u64,
            query = %logged(&text),
            "run"
        );
        return Ok(cached);
    }
    let Routed {
        leaf,
        path,
        trace,
        runners_up,
    } = match filed_category(&state, &request)? {
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
        tracing::info!(
            mode = "ingest",
            root = request.root.as_deref().unwrap_or(""),
            leaf = result.category_id.as_deref().unwrap_or(""),
            router = %result.trace.router,
            route_p = %shown(result.trace.score),
            item = %item.id,
            status = %item.status,
            updated,
            duplicates = result.duplicate_ids.len(),
            ms = started.elapsed().as_millis() as u64,
            query = %logged(&result.question),
            "run"
        );
        Ok(serde_json::to_value(result)?)
    } else {
        let nodes = state.taxonomy()?;
        let scope = resolve_root(&nodes, request.root.as_deref())?;
        let in_scope = |id: &str| {
            scope
                .allowed
                .as_ref()
                .is_none_or(|allowed| allowed.contains(id))
        };
        let categories: Vec<String> = leaf
            .as_ref()
            .map(|id| {
                descendant_ids(&nodes, id)
                    .into_iter()
                    .filter(|id| in_scope(id))
                    .collect()
            })
            .unwrap_or_default();
        let mut candidates = state.store.active_in(&categories)?;
        let mut pooled: HashSet<String> = candidates.iter().map(|item| item.id.clone()).collect();
        // An unsure route also ranks what its runners-up hold: a sideways miss
        // otherwise loses the answer however well the ranking reads.
        for runner in runners_up.iter().filter(|id| in_scope(id)) {
            let below: Vec<String> = descendant_ids(&nodes, runner)
                .into_iter()
                .filter(|id| in_scope(id))
                .collect();
            let mut rescued = state.store.active_in(&below)?;
            if rescued.len() > RESCUE_MAX_ITEMS {
                rescued = state.store.active_in(std::slice::from_ref(runner))?;
            }
            candidates.extend(
                rescued
                    .into_iter()
                    .filter(|item| pooled.insert(item.id.clone())),
            );
        }
        // So do the items that share the most words with the request, wherever
        // in the scope they are filed. A node's description never names
        // everything filed under it, so the route alone misses answers that sit
        // one branch over; the words of the item itself do not.
        let walked: Option<HashSet<String>> = request
            .start_node
            .as_deref()
            .map(|id| descendant_ids(&nodes, id).into_iter().collect());
        let matched = word_matches(&state, &lexical_text(&text, &request.context), |node| {
            in_scope(node) && walked.as_ref().is_none_or(|under| under.contains(node))
        })?;
        candidates.extend(
            matched
                .into_iter()
                .filter(|item| pooled.insert(item.id.clone())),
        );
        emit(
            &sender,
            json!({"type":"retrieval_completed","count":candidates.len(),"category_id":leaf}),
        )
        .await;
        let pool = candidates.len();
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
        let abstained = ranked.first().is_none_or(|item| holds(item.score));
        let items: Vec<RankedItem> = if abstained {
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
        // A route that said "outside the tree" is contradicted by an item that
        // answers the request: the search lands where that item is filed. The
        // trace keeps what the route itself decided. (An item filed on the
        // `root` node itself lands there, above the walk: an empty path.)
        let (leaf, path) = match items.first() {
            Some(top) if leaf.is_none() && !abstained => {
                let walk: Vec<String> = scope
                    .descent_nodes
                    .iter()
                    .map(|node| node.id.clone())
                    .collect();
                let path = if walk.contains(&top.item.category_id) {
                    path_parts(&scope.descent_nodes, &walk, &top.item.category_id)
                } else {
                    Vec::new()
                };
                (Some(top.item.category_id.clone()), path)
            }
            _ => (leaf, path),
        };
        tracing::info!(
            mode = "search",
            root = request.root.as_deref().unwrap_or(""),
            leaf = leaf.as_deref().unwrap_or(""),
            router = %trace.router,
            route_p = %shown(trace.score),
            route_leaf = trace.leaf_id.as_deref().unwrap_or(""),
            pool,
            abstained,
            top = items.first().map_or("", |item| item.item.id.as_str()),
            top_score = %shown(items.first().map(|item| item.score)),
            ms = started.elapsed().as_millis() as u64,
            query = %logged(&text),
            "run"
        );
        let result = SearchResult {
            query: text,
            items,
            leaf_id: leaf,
            path,
            trace,
            abstained,
        };
        emit(&sender, json!({"type":"search_done","leaf_id":result.leaf_id,"abstained":result.abstained,"items":result.items,"results":result.items,"query":result.query,"path":result.path,"trace":result.trace})).await;
        let value = serde_json::to_value(result)?;
        if let Some(key) = cache_key {
            state.results.put(key, value.clone());
        }
        Ok(value)
    }
}

/// The request text as the run log shows it: the first 200 characters, on one
/// line. `JEV_TREE_LOG_QUERIES=0` leaves it out, for a deployment whose
/// requests must not be written to a log.
fn logged(text: &str) -> String {
    static KEEP: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    let keep = KEEP.get_or_init(|| {
        std::env::var("JEV_TREE_LOG_QUERIES").map_or(true, |value| value.trim() != "0")
    });
    if !keep {
        return String::new();
    }
    text.chars()
        .take(200)
        .map(|c| if c.is_control() { ' ' } else { c })
        .collect()
}

/// A probability or score as a log line shows it: three decimals, `-` for none.
fn shown(value: Option<f64>) -> String {
    value.map_or_else(|| "-".into(), |value| format!("{value:.3}"))
}

/// The path from the top of the walk down to `id`, as a response shows it.
fn path_parts(nodes: &[Node], pool: &[String], id: &str) -> Vec<PathPart> {
    node_chain(nodes, pool, id)
        .into_iter()
        .map(|id| PathPart {
            name: node_name(nodes, &id),
            id,
            probability: None,
            confidence: None,
        })
        .collect()
}

/// Where a walk landed, plus the nodes a search also ranks when the walk was
/// unsure of its pick.
struct Routed {
    leaf: Option<String>,
    path: Vec<PathPart>,
    trace: Trace,
    /// Next-best nodes of an unsure route, best first (see [`RESCUE_BELOW`]).
    runners_up: Vec<String>,
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
) -> Routed {
    let pool: Vec<String> = nodes.iter().map(|node| node.id.clone()).collect();
    let path = path_parts(nodes, &pool, category);
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
    Routed {
        leaf: Some(category.to_string()),
        path,
        trace,
        runners_up: vec![],
    }
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

/// Wall-clock budget for an LLM review alone. The call may retry and the model
/// may deliberate over the menu, so without a ceiling a slow reply eats the
/// whole run deadline and the request dies as a timeout instead of keeping
/// Jev's pick. A third of the deadline leaves room for the ranking that follows.
fn llm_route_budget() -> std::time::Duration {
    run_deadline() / 3
}

async fn descend(
    state: &AppState,
    query: &str,
    request: &RunRequest,
    sender: Option<mpsc::Sender<Value>>,
) -> Result<Routed, Error> {
    let mode = request.mode.as_str();
    let run_id = Uuid::new_v4().to_string();
    let scope = resolve_root(&state.taxonomy()?, request.root.as_deref())?;
    let nodes = scope.descent_nodes.clone();
    let start = match request.start_node.as_deref() {
        Some(id) if scope.root.as_ref().is_some_and(|node| node.id == id) => {
            scope.descent_start.clone()
        }
        Some(id) => {
            if !nodes.iter().any(|node| node.id == id) {
                return Err(Error::BadRequest(format!(
                    "start_node not found under the current root: {id}"
                )));
            }
            Some(id.to_string())
        }
        None => scope.descent_start.clone(),
    };
    // One Jev choice over every node in scope, or over a recalled menu when the
    // scope is too big for one choice. A doubtful pick goes to the LLM when one
    // is configured; Jev always ranks the items afterwards.
    if let Some(result) = flat_route(
        state,
        &nodes,
        query,
        request,
        start.as_deref(),
        &run_id,
        &sender,
    )
    .await?
    {
        return Ok(result);
    }
    // Nothing in scope to choose from: an empty taxonomy.
    let trace = Trace {
        run_id: run_id.clone(),
        mode: mode.to_string(),
        router: "flat".into(),
        steps: vec![],
        leaf_id: None,
        score: None,
        final_beams: vec![FinalBeam {
            node_id: None,
            path: vec![],
            score: None,
            alive: false,
        }],
    };
    emit(
        &sender,
        json!({"type":"descent_done","run_id":run_id,"leaf_id":null,"path":[],"score":null,"trace":trace}),
    )
    .await;
    Ok(Routed {
        leaf: None,
        path: vec![],
        trace,
        runners_up: vec![],
    })
}

/// Most nodes one Jev choice can name. The evaluator takes at most 255 options
/// (a 256th is HTTP 422, measured against the live API) and a forest walk
/// spends one of them on `__none__`.
const FLAT_MAX_NODES: usize = 254;

/// Most node text (ids plus labels) one Jev choice is given. The whole request
/// has to fit the evaluator's context window: a 234-node menu of 94 KB passes
/// against the live API and one of 124 KB is HTTP 400. 80 KB keeps the margin,
/// and [`HINT_MAX_BYTES`] of that is kept for the filed questions a menu shows.
const FLAT_MAX_MENU_BYTES: usize = 74_000;

/// JSON framing around one menu entry: quotes, colon, comma.
const MENU_ENTRY_FRAMING: usize = 6;

/// The rule set the flat choice is judged by. The stay/descend rule is what
/// keeps a general question on a topic node and a named sub-topic on its child.
const FLAT_RULES: &str = "요청을 가장 정확히 담을 분류 하나를 고르세요.\n- 요청에 언급된 조건·하위 주제가 모두 들어가는 가장 구체적인 분류를 고르세요.\n- 요청이 하위 분류의 주제를 부르거나 그 주제에 대해 물으면, 다른 조건이 없어도 그 하위 분류를 고르세요. 요청의 주제 명사구가 어떤 분류의 이름과 같으면 더 구체적인 분류를 고르세요.\n- 요청에 없는 조건을 가정해 더 아래 분류로 내려가지 마세요. 하위 조건이 하나도 언급되지 않았다면 상위 분류에 머무르세요.\n- 요청이 이 분류들이 다루는 주제에 대한 문의라면 __none__을 고르지 마세요.";

/// The way out of one chunk of a split coarse map.
const COARSE_CHUNK_NONE: &str =
    "이 목록에는 요청을 담을 분류가 없음 — 다른 목록에 있거나 어느 분류에도 속하지 않음";

const FLAT_NONE: &str = "어느 분류에도 속하지 않음 — 요청이 이 분류들이 다루는 주제에 대한 문의가 아님(사이트 자체에 대한 일반 질문, 인사, 잡담, 완전히 다른 주제). 주제에 대한 문의라면 절대 고르지 마세요";

/// Search holds rather than answers when its best item scores below this. One
/// line for every item in the pool, wherever it came from.
///
/// Measured on three sets with the pool below (subtree, runners-up, word
/// matches). Golden (444 searches): 98.0% end to end at 0.45, 0.50 and 0.55,
/// 97.7% at 0.60. The 1,902-node benchmark: 88.0% at 0.45, 88.3% at 0.55 and
/// 0.60. A live customer-support tree (489 real requests, 131 of them with no
/// answer in the knowledge base): at 0.45 the search answered 108 of those 131
/// with an item that does not answer, at 0.55 76, at 0.60 56 — and 0.60 starts
/// to hold real answers (16 of 358 against 6). 0.55 is the last line that
/// costs the answerable side nothing.
///
/// Before 0.11 the line was 0.45 with a second, lower one for a "direct" item
/// (0.30) and a third, higher one for items a rescue brought in (0.75). On the
/// live tree the 0.75 line threw away one right answer in six, and the 0.30
/// line let through answers nothing else supported.
const HOLD_BELOW: f64 = 0.55;

/// "Nothing fits" has to be more likely than not. `__none__` shares one
/// distribution with the nodes, so when a request could sit in several nodes
/// they split their probability and `__none__` is often the largest single
/// option while the nodes together are far ahead of it. Taking the largest
/// option refused 43 of 489 real requests on a live tree (every one of them a
/// request the tree covers) and 11 of 361 on the 1,902-node benchmark; asking
/// for the majority refuses 29 and 2, and still catches 32 of the golden set's
/// 38 out-of-scope requests (33 before). The refusals that remain are checked
/// against the items (see `execute`).
const NONE_MIN: f64 = 0.5;

/// One Jev choice over the walk's menu: every node in scope when one choice can
/// name them, else a recalled menu ([`recall_menu`]). `__none__` competes at a
/// forest walk; a `start_node`/scoped walk must land in the menu. Returns `None`
/// only when the scope holds no node at all.
async fn flat_route(
    state: &AppState,
    nodes: &[Node],
    query: &str,
    request: &RunRequest,
    start: Option<&str>,
    run_id: &str,
    sender: &Option<mpsc::Sender<Value>>,
) -> Result<Option<Routed>, Error> {
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
        return Ok(Some(Routed {
            leaf: Some(leaf),
            path,
            trace,
            runners_up: vec![],
        }));
    }
    // Every node in scope is the menu whenever one Jev choice can name it: a
    // perfect evaluator must always see the right node, and a request may arrive
    // in any script. Only a scope past the evaluator's limits gets a recalled
    // menu instead.
    let mut steps = Vec::new();
    let menu: Vec<String> = if full_menu_fits(nodes, &pool) {
        ScopeIndex::new(nodes, &pool)
            .order
            .iter()
            .map(|id| id.to_string())
            .collect()
    } else {
        let (menu, coarse) = recall_menu(
            state,
            nodes,
            &pool,
            query,
            request,
            &conversation,
            start.is_none(),
            run_id,
            sender,
        )
        .await?;
        steps.push(coarse);
        menu
    };
    if menu.is_empty() {
        return Ok(None);
    }
    // A search shows, under each node, the questions already filed there that
    // share its words: what the node holds, which its description may not say.
    // An ingest does not. Its Q&A is new, so nothing filed is the same question
    // (a row that is comes back in `duplicate_ids`), and look-alikes one branch
    // over only pull it there: with them the 1,902-node benchmark filed 118 to
    // 122 of 160 Q&As in the right node, without them 126 to 127.
    let hints = if mode == "search" {
        let on_menu: HashSet<&str> = menu.iter().map(String::as_str).collect();
        filed_hints(&word_matches(
            state,
            &lexical_text(query, &request.context),
            |node| on_menu.contains(node),
        )?)
    } else {
        HashMap::new()
    };
    let criteria = options(
        menu.iter().map(|id| {
            (
                id.clone(),
                format!(
                    "{}{}",
                    flat_label(nodes, &pool, id),
                    hints.get(id).map_or("", String::as_str)
                ),
            )
        }),
        start.is_none().then_some(FLAT_NONE),
    );
    let hint_rule = if hints.is_empty() { "" } else { HINT_RULE };
    let mut questions = BTreeMap::new();
    questions.insert(
        "flat".into(),
        Question {
            kind: "choice".into(),
            instructions: if start.is_none() {
                format!("{FLAT_RULES} 어느 분류에도 맞지 않으면 __none__을 고르세요.{hint_rule}")
            } else {
                format!("{FLAT_RULES} 여기 있는 분류 중에서만 고르세요.{hint_rule}")
            },
            criteria: Some(criteria),
        },
    );
    emit(
        sender,
        json!({"type":"descent_step","status":"asking","depth":steps.len(),"run_id":run_id,"hinted":hints.len(),"frontiers":[{"node_id":start,"path":[],"parent":parent_name,"children":menu.iter().map(|id|json!({"id":id,"name":node_name(nodes,id)})).collect::<Vec<_>>()}]}),
    )
    .await;
    let judgment = judge(
        state,
        sender,
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
    // "Nothing fits" without the majority is not a refusal: the best node is
    // the pick (see [`NONE_MIN`]). The first of equals wins, in menu order.
    let choice = if choice == "__none__"
        && probabilities.get("__none__").copied().unwrap_or(0.0) < NONE_MIN
    {
        let p = |id: &String| probabilities.get(id).copied().unwrap_or(0.0);
        menu.iter()
            .fold(None::<&String>, |best, id| match best {
                Some(best) if p(best) >= p(id) => Some(best),
                _ => Some(id),
            })
            .cloned()
            .unwrap_or(choice)
    } else {
        choice
    };
    let score = probabilities.get(&choice).copied();
    let jev_score = score;
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
        depth: steps.len(),
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
    steps.push(step);
    let mut choice = choice;
    let mut router = String::from("flat");
    let mut score = score;
    // A doubtful pick goes to the LLM when one is configured: it reads the same
    // menu, in tree order, and picks again. A confident pick never pays for it.
    let asked = std::time::Instant::now();
    let reviewed = if review_needed(score, state.jev.llm_routing()) {
        llm_review(
            state,
            nodes,
            &menu,
            query,
            &conversation,
            &probabilities,
            start.is_none(),
            &hints,
            sender,
        )
        .await
    } else {
        None
    };
    if let Some((pick, review_step)) = reviewed {
        // What the review did, so its worth can be read off the log: how often
        // it runs, how long it takes and how often it changes the pick.
        let llm = pick.as_deref().unwrap_or("__none__");
        tracing::info!(
            jev = %choice,
            jev_p = %shown(score),
            llm = %llm,
            changed = llm != choice,
            ms = asked.elapsed().as_millis() as u64,
            "llm review"
        );
        steps.push(review_step);
        router = "llm".into();
        score = None;
        abstained = pick.is_none();
        choice = pick.unwrap_or_else(|| "__none__".into());
    }
    let mut leaf = (!abstained).then(|| choice.clone());
    // A pick whose subtree did not fit the recalled menu has only been seen
    // from above: walk on inside it, with the pick itself a candidate so the
    // walk can still stay there. Each round is one level of the tree deeper,
    // so this ends; on the benchmarks no branch was ever cut.
    let mut rescue = None;
    if let Some(id) = leaf.clone()
        && Some(id.as_str()) != start
        && menu.len() < pool.len()
        && descendant_ids(nodes, &id)
            .iter()
            .any(|below| !menu.contains(below))
    {
        let inner = Box::pin(flat_route(
            state,
            nodes,
            query,
            request,
            Some(id.as_str()),
            run_id,
            sender,
        ))
        .await?;
        if let Some(inner) = inner {
            let offset = steps.len();
            steps.extend(inner.trace.steps.into_iter().map(|mut step| {
                step.depth += offset;
                step
            }));
            if inner.trace.router == "llm" {
                router = "llm".into();
            }
            score = inner.trace.score;
            leaf = inner.leaf;
            rescue = Some(inner.runners_up);
        }
    }
    let runners_up = rescue
        .unwrap_or_else(|| runners_up(nodes, &pool, leaf.as_deref(), jev_score, &probabilities));
    let path: Vec<PathPart> = leaf
        .as_deref()
        .map(|id| path_parts(nodes, &pool, id))
        .unwrap_or_default();
    let trace = Trace {
        run_id: run_id.to_string(),
        mode: mode.to_string(),
        router,
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
    Ok(Some(Routed {
        leaf,
        path,
        trace,
        runners_up,
    }))
}

/// Below this chosen probability a search also ranks the items of the next
/// best nodes. On the 1,902-node benchmark most misses are sideways (a sibling,
/// another branch, another domain), and those leave the answer outside the
/// pool however well the ranking reads; with the runners-up in the pool the
/// ranking finds it. Measured there: E2E 72.3% → 82.0%; on the golden set +0.2
/// to +0.9pp in every tree shape.
const RESCUE_BELOW: f64 = 0.9;

/// Runners-up a search ranks besides the chosen node.
const RESCUE_NODES: usize = 2;

/// A runner-up whose subtree holds more items than this contributes only its
/// own: rescue must not turn one ranking call into dozens.
const RESCUE_MAX_ITEMS: usize = 64;

/// Items a search adds to its pool by their words alone: the best BM25 matches
/// of the request among the items in scope ([`crate::recall`]), wherever they
/// are filed. Every search does this, sure of its route or not: on a live
/// customer-support tree one confident pick in ten still pointed at the wrong
/// branch.
///
/// Measured with 20. Live tree, 489 real requests: the answer was in the
/// results for 82% of the answerable ones without the matches and for 96% with
/// them, and requests refused as "outside the tree" fell from 19 to 1. The
/// 1,902-node benchmark: E2E 82.6% → 88.3%. Golden: 98.6% → 98.0%, two cases,
/// where a matched overview item out-scored the leaf's own. 40 was no better
/// than 20 (more look-alikes to rank).
const RECALL_ITEMS: usize = 20;

/// The next best nodes of an unsure pick, best first. The pick's ancestors
/// and descendants are left out: their items are either in the pool already
/// or would swallow it.
fn runners_up(
    nodes: &[Node],
    pool: &[String],
    leaf: Option<&str>,
    chosen_probability: Option<f64>,
    probabilities: &BTreeMap<String, f64>,
) -> Vec<String> {
    let Some(leaf) = leaf else {
        return vec![];
    };
    if chosen_probability.is_some_and(|p| p >= RESCUE_BELOW) {
        return vec![];
    }
    let lineage: HashSet<String> = node_chain(nodes, pool, leaf)
        .into_iter()
        .chain(descendant_ids(nodes, leaf))
        .collect();
    let mut ranked: Vec<(&String, f64)> = probabilities
        .iter()
        .filter(|(id, _)| id.as_str() != "__none__" && !lineage.contains(*id))
        .map(|(id, p)| (id, *p))
        .collect();
    ranked.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(b.0)));
    ranked
        .into_iter()
        .take(RESCUE_NODES)
        .map(|(id, _)| id.clone())
        .collect()
}

/// Below this chosen probability a configured LLM reviews Jev's pick. On the
/// golden set's `standard` tree most of Jev's route errors sit below it (22 of
/// 28 in 0.10) while most requests sit above and never wait for the LLM.
///
/// What the review is worth since the menu shows filed questions (0.11,
/// reviewer `muse-ember-1.0` at `low`): it settles 9% of golden requests, 27%
/// on the 1,902-node benchmark and 45% on a live customer-support tree. A
/// search gains little, because the pool already holds the answer when the
/// route is one node off: right node 84.9% -> 87.3% and E2E 88.5% -> 89.2% on
/// the benchmark, nothing measurable on the other two. An ingest gains a lot,
/// because there the node is the result: Q&As filed in the right node went
/// from 122 to 134 of 160 on the benchmark and from 66 to 68 of 68 on golden.
/// Its price is the tail: p95 of a search 1 s without it, 7-9 s with.
const LLM_REVIEW_BELOW: f64 = 0.8;

/// A configured LLM reviews only doubtful picks: a confident one never waits.
fn review_needed(chosen_probability: Option<f64>, llm_configured: bool) -> bool {
    llm_configured && chosen_probability.is_some_and(|p| p < LLM_REVIEW_BELOW)
}

/// Hand a doubtful flat pick to the LLM, with the menu Jev chose from. `None`
/// when the call fails or runs past its budget: Jev's pick then stands, scope
/// check and all.
#[allow(clippy::too_many_arguments)]
async fn llm_review(
    state: &AppState,
    nodes: &[Node],
    menu: &[String],
    query: &str,
    conversation: &str,
    probabilities: &BTreeMap<String, f64>,
    abstain: bool,
    hints: &HashMap<String, String>,
    sender: &Option<mpsc::Sender<Value>>,
) -> Option<(Option<String>, TraceStep)> {
    emit(
        sender,
        json!({"type":"descent_step","status":"asking","depth":1,"reviewing":menu.len()}),
    )
    .await;
    let reviewed = tokio::time::timeout(
        llm_route_budget(),
        state
            .jev
            .review(query, conversation, nodes, menu, abstain, hints),
    )
    .await;
    let route = match reviewed {
        Ok(Ok(route)) => route,
        Ok(Err(error)) => {
            tracing::warn!("llm review failed, keeping the Jev pick: {error}");
            return None;
        }
        Err(_) => {
            tracing::warn!("llm review failed, keeping the Jev pick: route budget spent");
            return None;
        }
    };
    emit(
        sender,
        json!({"type":"usage","source":"llm","calls":route.meter.calls,"input_tokens":route.meter.usage.input_tokens,"output_tokens":route.meter.usage.output_tokens}),
    )
    .await;
    let leaf = route.leaf.filter(|id| menu.contains(id));
    let mut candidates: Vec<Candidate> = menu
        .iter()
        .map(|id| Candidate {
            id: Some(id.clone()),
            name: node_name(nodes, id),
            probability: probabilities.get(id).copied(),
        })
        .collect();
    candidates.sort_by(|a, b| {
        b.probability
            .unwrap_or(0.0)
            .total_cmp(&a.probability.unwrap_or(0.0))
    });
    let step = TraceStep {
        depth: 1,
        node_id: None,
        node_name: "llm review".into(),
        path: vec![],
        candidates,
        choice_id: leaf.clone(),
        choice_name: leaf
            .as_deref()
            .map(|id| node_name(nodes, id))
            .unwrap_or_else(|| "no matching topic".into()),
        confidence: None,
        reason: format!("llm review: {query}"),
    };
    Some((leaf, step))
}

/// A choice's options in the order given, the way out last. The evaluator
/// leans toward what it reads first, so the order is part of the question:
/// nodes go in tree order, where siblings sit side by side, and `__none__`
/// never leads. (`serde_json` keeps insertion order only with its
/// `preserve_order` feature; a test below fails if that is ever dropped.)
fn options(entries: impl Iterator<Item = (String, String)>, none: Option<&str>) -> Value {
    let mut map = serde_json::Map::new();
    for (id, label) in entries {
        map.insert(id, Value::String(label));
    }
    if let Some(label) = none {
        map.insert("__none__".into(), Value::String(label.into()));
    }
    Value::Object(map)
}

/// The active items filed under the nodes `inside` accepts that share the most
/// words with the request, best first: at most [`RECALL_ITEMS`]. One lookup
/// with two readers. The route shows them under their nodes as evidence of
/// what each node holds ([`filed_hints`]); a search adds them to its pool.
fn word_matches(
    state: &AppState,
    text: &str,
    inside: impl Fn(&str) -> bool,
) -> Result<Vec<Item>, Error> {
    let index = recall_index(state)?;
    let ids: Vec<String> = index
        .items
        .rank(text)
        .into_iter()
        .filter(|(id, _)| index.item_node.get(*id).is_some_and(|node| inside(node)))
        .take(RECALL_ITEMS)
        .map(|(id, _)| id.to_string())
        .collect();
    state.store.active_by_ids(&ids)
}

/// Filed questions one node shows in a menu.
const HINTS_PER_NODE: usize = 3;

/// Characters of one filed question a menu shows.
const HINT_CHARS: usize = 80;

/// Bytes all the filed questions of one menu may take. With
/// [`FLAT_MAX_MENU_BYTES`] of node labels that is the 80 KB one choice reads
/// with room to spare.
const HINT_MAX_BYTES: usize = 6_000;
const _: () = assert!(FLAT_MAX_MENU_BYTES + HINT_MAX_BYTES <= 80_000);

/// What the menu says about the questions it shows.
const HINT_RULE: &str = " 분류 설명 끝의 '이미 있는 질문'은 그 분류에 이미 저장된 질문입니다. 요청과 같은 것을 묻는 질문이 있는 분류가 맞는 분류입니다.";

/// The questions already filed under each node that share the most words with
/// the request, by node. A description says what a node is meant to hold; the
/// questions filed there show what it does hold, and that is what the choice
/// was missing. Shown to Jev and to the LLM alike.
///
/// Measured against the same walk without them. The 1,902-node benchmark:
/// right node 69.5% → 84.9% (Jev alone; an LLM reviewing half the requests had
/// reached 78%). Golden: 95.9% → 97.1%. A live customer-support tree, whose
/// descriptions name little of what is filed: 53% → 60%.
///
/// Returns, by node, the text its label ends with: the same text for both.
fn filed_hints(matches: &[Item]) -> HashMap<String, String> {
    let mut shown: HashMap<&str, Vec<String>> = HashMap::new();
    let mut bytes = 0;
    for item in matches {
        let questions = shown.entry(item.category_id.as_str()).or_default();
        let question: String = item.question.chars().take(HINT_CHARS).collect();
        let cost = question.len() + HINT_FRAMING;
        if questions.len() < HINTS_PER_NODE && bytes + cost <= HINT_MAX_BYTES {
            bytes += cost;
            questions.push(question);
        }
    }
    shown
        .into_iter()
        .filter(|(_, questions)| !questions.is_empty())
        .map(|(node, questions)| {
            (
                node.to_string(),
                format!(" | 이미 있는 질문: {}", questions.join(" / ")),
            )
        })
        .collect()
}

/// Bytes around one filed question in a label: its share of the lead-in and
/// the separator.
const HINT_FRAMING: usize = 32;

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
        .map(|node| {
            node.description
                .chars()
                .take(crate::jev::MENU_DESCRIPTION_CHARS)
                .collect::<String>()
        })
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

/// Lookups over one walk's scope, built once per recall instead of scanning
/// the node list for every parent: a large scope asks for thousands of them.
struct ScopeIndex<'a> {
    by_id: HashMap<&'a str, &'a Node>,
    inside: HashSet<&'a str>,
    children: HashMap<&'a str, Vec<&'a str>>,
    /// Scope nodes in tree order (depth-first), so subtrees stay together.
    order: Vec<&'a str>,
}

impl<'a> ScopeIndex<'a> {
    fn new(nodes: &'a [Node], pool: &'a [String]) -> Self {
        let by_id: HashMap<&str, &Node> =
            nodes.iter().map(|node| (node.id.as_str(), node)).collect();
        let inside: HashSet<&str> = pool.iter().map(String::as_str).collect();
        let mut children: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut roots = Vec::new();
        for id in pool {
            match by_id
                .get(id.as_str())
                .and_then(|node| node.parent_id.as_deref())
                .filter(|parent| inside.contains(parent))
            {
                Some(parent) => children.entry(parent).or_default().push(id.as_str()),
                None => roots.push(id.as_str()),
            }
        }
        let mut order = Vec::with_capacity(pool.len());
        let mut stack: Vec<&str> = roots.into_iter().rev().collect();
        while let Some(id) = stack.pop() {
            order.push(id);
            if let Some(kids) = children.get(id) {
                stack.extend(kids.iter().rev());
            }
        }
        Self {
            by_id,
            inside,
            children,
            order,
        }
    }

    /// `id` and its ancestors inside the scope, topmost first.
    fn chain(&self, id: &'a str) -> Vec<&'a str> {
        let mut chain = vec![id];
        let mut cursor = self
            .by_id
            .get(id)
            .and_then(|node| node.parent_id.as_deref());
        while let Some(parent) = cursor.filter(|parent| self.inside.contains(parent)) {
            if chain.len() > self.order.len() {
                break;
            }
            chain.push(parent);
            cursor = self
                .by_id
                .get(parent)
                .and_then(|node| node.parent_id.as_deref());
        }
        chain.reverse();
        chain
    }

    fn is_leaf(&self, id: &str) -> bool {
        self.children.get(id).is_none_or(Vec::is_empty)
    }

    /// `id` and everything under it, depth-first.
    fn subtree(&self, id: &'a str) -> Vec<&'a str> {
        let mut out = Vec::new();
        let mut stack = vec![id];
        while let Some(current) = stack.pop() {
            out.push(current);
            if let Some(kids) = self.children.get(current) {
                stack.extend(kids.iter().rev());
            }
        }
        out
    }

    /// Menu label, the same text [`flat_label`] builds.
    fn label(&self, id: &'a str) -> String {
        let names: Vec<&str> = self
            .chain(id)
            .into_iter()
            .filter_map(|id| self.by_id.get(id).map(|node| node.name.as_str()))
            .collect();
        let description: String = self
            .by_id
            .get(id)
            .map(|node| {
                node.description
                    .chars()
                    .take(crate::jev::MENU_DESCRIPTION_CHARS)
                    .collect()
            })
            .unwrap_or_default();
        format!("{} | {}", names.join("/"), description)
    }

    fn entry_bytes(&self, id: &'a str) -> usize {
        id.len() + self.label(id).len() + MENU_ENTRY_FRAMING
    }

    /// The coarse map: the deepest level whose nodes, plus any shallower
    /// leaves, one choice can still name. When even the top level is too wide,
    /// it is split into several menus asked in the same request (the
    /// evaluator's limits hold per question, not per request).
    fn coarse(&self) -> Vec<Vec<&'a str>> {
        let depth: HashMap<&str, usize> = self
            .order
            .iter()
            .map(|id| (*id, self.chain(id).len()))
            .collect();
        let deepest = depth.values().copied().max().unwrap_or(1);
        let level = |d: usize| -> Vec<&'a str> {
            self.order
                .iter()
                .copied()
                .filter(|id| depth[id] == d || (depth[id] < d && self.is_leaf(id)))
                .collect()
        };
        let fits = |menu: &[&'a str]| {
            menu.len() <= FLAT_MAX_NODES
                && menu.iter().map(|id| self.entry_bytes(id)).sum::<usize>() <= FLAT_MAX_MENU_BYTES
        };
        if let Some(menu) = (1..=deepest)
            .rev()
            .map(level)
            .find(|menu| menu.len() > 1 && fits(menu))
        {
            return vec![menu];
        }
        // Even the first level that branches is too wide for one choice (or
        // the walk's only top node is its own start): split that level.
        let first = (1..=deepest)
            .map(level)
            .find(|menu| menu.len() > 1)
            .unwrap_or_else(|| level(1));
        let mut chunks: Vec<Vec<&str>> = Vec::new();
        let (mut current, mut bytes) = (Vec::new(), 0);
        for id in first {
            let cost = self.entry_bytes(id);
            if !current.is_empty()
                && (current.len() + 1 > FLAT_MAX_NODES || bytes + cost > FLAT_MAX_MENU_BYTES)
            {
                chunks.push(std::mem::take(&mut current));
                bytes = 0;
            }
            current.push(id);
            bytes += cost;
        }
        if !current.is_empty() {
            chunks.push(current);
        }
        chunks
    }
}

/// A menu being filled up to what one Jev choice can read.
struct MenuFill<'a> {
    ids: Vec<&'a str>,
    seen: HashSet<&'a str>,
    bytes: usize,
}

impl<'a> MenuFill<'a> {
    fn new() -> Self {
        Self {
            ids: Vec::new(),
            seen: HashSet::new(),
            bytes: 0,
        }
    }

    fn full(&self) -> bool {
        self.ids.len() >= FLAT_MAX_NODES || self.bytes >= FLAT_MAX_MENU_BYTES
    }

    /// Add `id` with the ancestors it brings along, unless that would overflow
    /// the choice. A node that does not fit is skipped, not the end of the
    /// fill: a later one may be cheaper.
    fn push(&mut self, scope: &ScopeIndex<'a>, id: &'a str) {
        let fresh: Vec<&str> = scope
            .chain(id)
            .into_iter()
            .filter(|id| !self.seen.contains(id))
            .collect();
        let cost: usize = fresh.iter().map(|id| scope.entry_bytes(id)).sum();
        if !self.ids.is_empty()
            && (self.ids.len() + fresh.len() > FLAT_MAX_NODES
                || self.bytes + cost > FLAT_MAX_MENU_BYTES)
        {
            return;
        }
        self.bytes += cost;
        for id in fresh {
            self.seen.insert(id);
            self.ids.push(id);
        }
    }

    /// The menu in tree order, so siblings sit next to each other.
    fn finish(self, scope: &ScopeIndex<'a>) -> Vec<String> {
        scope
            .order
            .iter()
            .filter(|id| self.seen.contains(*id))
            .map(|id| id.to_string())
            .collect()
    }
}

/// Bytes of choice menus one evaluator request may carry. The limits hold per
/// question (about 32K tokens) and per request (about 64K: three questions of
/// 21K tokens pass, four fail), so a coarse map wider than two full menus is
/// asked in several requests at once.
const REQUEST_MAX_BYTES: usize = 160_000;

/// Probability mass the coarse pass's opened branches must cover.
const COARSE_MASS: f64 = 0.95;

/// Most coarse branches opened into the final menu.
const COARSE_BRANCHES: usize = 8;

/// A coarse branch below this probability is not opened, except the best one:
/// a request the coarse map calls out of scope does not drag in eight
/// arbitrary subtrees.
const COARSE_FLOOR: f64 = 0.02;

/// Lexical matches the final menu takes after the opened branches.
const RECALL_LEXICAL: usize = 30;

/// The menu for a scope one Jev choice cannot name, from two sources:
///
/// - a coarse Jev choice over the deepest level of the tree that still fits
///   one choice, whose best branches are opened whole. It reads meaning, so a
///   request in another script or without a word in common with the taxonomy
///   still reaches its branch;
/// - BM25 over node text and the questions filed under each node
///   ([`crate::recall`]), for the words the request does use.
///
/// Returns the menu (tree order) and the coarse step for the trace.
#[allow(clippy::too_many_arguments)]
async fn recall_menu(
    state: &AppState,
    nodes: &[Node],
    pool: &[String],
    query: &str,
    request: &RunRequest,
    conversation: &str,
    abstain: bool,
    run_id: &str,
    sender: &Option<mpsc::Sender<Value>>,
) -> Result<(Vec<String>, TraceStep), Error> {
    let scope = ScopeIndex::new(nodes, pool);
    let chunks = scope.coarse();
    // One question per chunk; the questions go out in as few requests as the
    // evaluator's per-request window allows, all at once.
    let chunked = chunks.len() > 1;
    let mut requests: Vec<BTreeMap<String, Question>> = vec![BTreeMap::new()];
    let mut request_bytes = 0;
    for (index, chunk) in chunks.iter().enumerate() {
        let bytes: usize = chunk.iter().map(|id| scope.entry_bytes(id)).sum();
        // Each chunk of a split map needs its own way out: forced to choose,
        // a chunk without the right node would still answer with confidence,
        // and its pick would crowd out the chunk that has it.
        let criteria = options(
            chunk.iter().map(|id| (id.to_string(), scope.label(id))),
            if chunked {
                Some(COARSE_CHUNK_NONE)
            } else if abstain {
                Some(FLAT_NONE)
            } else {
                None
            },
        );
        if request_bytes > 0 && request_bytes + bytes > REQUEST_MAX_BYTES {
            requests.push(BTreeMap::new());
            request_bytes = 0;
        }
        request_bytes += bytes;
        requests.last_mut().expect("one request").insert(
            format!("coarse_{index}"),
            Question {
                kind: "choice".into(),
                instructions: if chunked {
                    format!("{FLAT_RULES} 이 목록에 맞는 분류가 없으면 __none__을 고르세요.")
                } else if abstain {
                    format!("{FLAT_RULES} 어느 분류에도 맞지 않으면 __none__을 고르세요.")
                } else {
                    format!("{FLAT_RULES} 여기 있는 분류 중에서만 고르세요.")
                },
                criteria: Some(criteria),
            },
        );
    }
    emit(
        sender,
        json!({"type":"descent_step","status":"asking","depth":0,"run_id":run_id,"frontiers":[{"node_id":null,"path":[],"parent":"coarse map","children":chunks.iter().flatten().map(|id|json!({"id":id,"name":node_name(nodes,id)})).collect::<Vec<_>>()}]}),
    )
    .await;
    let payload = json!({
        "conversation": conversation,
        "current_request": query,
        "background": &request.context,
        "mode": request.mode,
        "path": [],
    });
    let judgments = futures_util::future::try_join_all(
        requests
            .into_iter()
            .map(|questions| judge(state, sender, payload.clone(), questions)),
    )
    .await?;
    let mut coarse: Vec<(&str, f64)> = Vec::new();
    let mut none = 0.0_f64;
    for answer in judgments
        .iter()
        .flat_map(|judgment| judgment.answers.values())
    {
        if let Answer::Choice { probabilities, .. } = answer {
            for (id, probability) in probabilities {
                if id == "__none__" {
                    none = none.max(*probability);
                } else if let Some(id) = scope.inside.get(id.as_str()) {
                    coarse.push((id, *probability));
                }
            }
        }
    }
    coarse.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(b.0)));
    let mut covered = 0.0;
    let mut branches = Vec::new();
    for (id, probability) in &coarse {
        if !branches.is_empty()
            && (branches.len() >= COARSE_BRANCHES
                || covered >= COARSE_MASS
                || *probability < COARSE_FLOOR)
        {
            break;
        }
        branches.push(*id);
        covered += probability;
    }
    // The opened branches, shallow levels of every branch before the deep
    // levels of any: a branch too big for the menu still shows its upper
    // levels, and the walk can stop there and rank the whole subtree.
    let mut opened: Vec<(usize, usize, &str)> = Vec::new();
    for (rank, branch) in branches.iter().enumerate() {
        let base = scope.chain(branch).len();
        for node in scope.subtree(branch) {
            opened.push((scope.chain(node).len() - base, rank, node));
        }
    }
    opened.sort_by_key(|(level, rank, _)| (*level, *rank));
    let index = recall_index(state)?;
    let text = lexical_text(query, &request.context);
    let lexical: Vec<&str> = index
        .nodes
        .rank(&text)
        .into_iter()
        .filter_map(|(id, _)| scope.inside.get(id).copied())
        .take(RECALL_LEXICAL)
        .collect();
    // Branches first, but they leave room for the lexical matches; whatever
    // room the matches do not use goes back to the branches.
    let mut fill = MenuFill::new();
    let mut rest = Vec::new();
    for (_, _, node) in opened {
        if fill.ids.len() + lexical.len() < FLAT_MAX_NODES {
            fill.push(&scope, node);
        } else {
            rest.push(node);
        }
    }
    for id in lexical.into_iter().chain(rest) {
        if fill.full() {
            break;
        }
        fill.push(&scope, id);
    }
    let step = TraceStep {
        depth: 0,
        node_id: None,
        node_name: "coarse map".into(),
        path: vec![],
        candidates: coarse
            .iter()
            .take(12)
            .map(|(id, probability)| Candidate {
                id: Some(id.to_string()),
                name: node_name(nodes, id),
                probability: Some(*probability),
            })
            .chain(abstain.then(|| Candidate {
                id: None,
                name: "no matching topic".into(),
                probability: Some(none),
            }))
            .collect(),
        choice_id: branches.first().map(|id| id.to_string()),
        choice_name: branches
            .iter()
            .map(|id| node_name(nodes, id))
            .collect::<Vec<_>>()
            .join(", "),
        confidence: None,
        reason: format!("flat: coarse map, {} branches opened", branches.len()),
    };
    emit(
        sender,
        json!({"type":"descent_step","status":"answered","depth":0,"run_id":run_id,"nodes":[&step]}),
    )
    .await;
    Ok((fill.finish(&scope), step))
}

/// The request as BM25 reads it: the current request plus the text of the
/// prior turns, which carry the topic an elliptical follow-up leaves out.
fn lexical_text(query: &str, context: &[Value]) -> String {
    let mut text = query.to_string();
    for turn in context {
        let words = match turn {
            Value::String(words) => Some(words.as_str()),
            Value::Object(map) => map
                .get("text")
                .or_else(|| map.get("content"))
                .and_then(Value::as_str),
            _ => None,
        };
        if let Some(words) = words {
            text.push(' ');
            text.push_str(words);
        }
    }
    text
}

/// The lexical index for the current taxonomy and items, rebuilt when the
/// store has changed since it was built.
fn recall_index(state: &AppState) -> Result<std::sync::Arc<crate::recall::Index>, Error> {
    let nodes = state.nodes.read().map_err(|_| Error::Poisoned)?;
    state
        .recall
        .get_or_build((state.store.generation(), nodes.len()), || {
            Ok::<_, Error>(crate::recall::Index::build(
                &nodes,
                &state.store.active_items()?,
            ))
        })
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

/// Search holds instead of answering when its best item is below
/// [`HOLD_BELOW`].
fn holds(score: f64) -> bool {
    !at_least(score, HOLD_BELOW)
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

/// Break near-ties toward the node the walk stopped at.
///
/// A descent that stops at `X` has already decided `X` is the right level of
/// specificity, and `X`'s own items sit there; what is filed below it is more
/// specific than the request asked for, and what the pool brought in from
/// elsewhere (runners-up, word matches) has only the ranking on its side.
/// Jev's scores routinely put a deeper item a hair above `X`'s overview item
/// (measured gaps 0.015-0.021), which is enough to lose the answer to a vague
/// question. Applied only where other items are actually competing: a node's
/// lone item beats nothing and must not be inflated.
const LEVEL_BONUS: f64 = 0.10;

async fn rank(
    state: &AppState,
    query: &str,
    context: &[Value],
    items: Vec<Item>,
    // The node the walk stopped at, if any. Its own items sit at the specificity
    // the request asked for, so they win near-ties against the rest of the pool.
    prefer: Option<&str>,
    limit: usize,
    sender: Option<mpsc::Sender<Value>>,
) -> Result<Vec<RankedItem>, Error> {
    if items.is_empty() {
        return Ok(vec![]);
    }
    // Only worth a nudge when other candidates are in the pool to nudge against.
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
        emit_usage(&sender, "jev", judgment.usage).await;
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
            recall: crate::recall::Cache::default(),
            results: crate::cache::ResultCache::default(),
        };
        (dir, state)
    }

    fn wide_state() -> (tempfile::TempDir, AppState) {
        // The flat route names every node in scope in one call; past
        // `FLAT_MAX_NODES` it routes through the coarse map and a recalled
        // menu. Pad the fixture past that line to reach that path.
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
            recall: crate::recall::Cache::default(),
            results: crate::cache::ResultCache::default(),
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
    async fn flat_choice_none_abstains() {
        // `__none__` competes in the same choice and leaves the tree. The items
        // that share the request's words are still ranked, and when none of
        // them answers (a script that picks `__none__` scores them low) the
        // search stays outside and holds.
        let (_dir, state) = fixture_state();
        state.jev.script_choices(vec!["__none__".into()]);
        let result = execute(state, run_req("해외 배송이 통관에서 멈췄어요"), None)
            .await
            .unwrap();
        assert!(result["leaf_id"].is_null(), "{result}");
        assert!(result["abstained"].as_bool().unwrap(), "{result}");
        assert!(
            result["items"]
                .as_array()
                .unwrap()
                .iter()
                .all(|row| row["role"] == "reference"),
            "{result}"
        );
    }

    #[tokio::test]
    async fn none_without_the_majority_is_not_a_refusal() {
        // `__none__` is the largest single option at 0.4 while the nodes hold
        // the other 0.6 between them: the best node is the pick.
        let (_dir, state) = fixture_state();
        state
            .jev
            .script_choices(vec!["__none__@0.4".into(), "orders_tracking@0.35".into()]);
        let result = execute(state, run_req("해외 배송이 통관에서 멈췄어요"), None)
            .await
            .unwrap();
        assert_eq!(result["leaf_id"], "orders_tracking", "{result}");
        assert_eq!(result["trace"]["leaf_id"], "orders_tracking", "{result}");
        assert_eq!(result["trace"]["score"], 0.35, "{result}");
    }

    #[tokio::test]
    async fn none_with_the_majority_refuses() {
        let (_dir, state) = fixture_state();
        state.jev.script_choices(vec!["__none__@0.5".into()]);
        let result = execute(state, run_req("오늘 날씨가 어때요?"), None)
            .await
            .unwrap();
        assert!(result["leaf_id"].is_null(), "{result}");
        assert!(result["trace"]["leaf_id"].is_null(), "{result}");
    }

    #[tokio::test]
    async fn a_refusal_the_items_contradict_lands_on_the_answer() {
        // The route says "outside the tree"; an item in scope shares the
        // request's words and the ranking says it answers. The search lands
        // where that item is filed, and the trace keeps the route's own verdict.
        let (_dir, state) = fixture_state();
        // Jev is unsure, the LLM pin reviews and refuses, and the ranking keeps
        // its usual scripted scores, which clear the hold line.
        state.jev.script_choices(vec!["~account".into()]);
        state.jev.script_llm(Some("__none__".into()));
        let result = execute(state, run_req("해외 배송이 통관에서 멈췄어요"), None)
            .await
            .unwrap();
        assert!(result["trace"]["leaf_id"].is_null(), "{result}");
        assert!(!result["abstained"].as_bool().unwrap(), "{result}");
        let top = &result["items"][0]["item"]["category_id"];
        assert_eq!(&result["leaf_id"], top, "{result}");
        assert_eq!(
            result["path"].as_array().unwrap().last().unwrap()["id"],
            *top,
            "{result}"
        );
    }

    #[tokio::test]
    async fn a_menu_shows_the_filed_questions_that_share_the_requests_words() {
        let (_dir, state) = fixture_state();
        let everywhere = |_: &str| true;
        let matches = word_matches(&state, "해외 배송이 통관에서 멈췄어요", everywhere).unwrap();
        assert!(!matches.is_empty());
        let hints = filed_hints(&matches);
        let tail = hints
            .get("orders_tracking")
            .expect("the node that holds them");
        assert!(tail.starts_with(" | 이미 있는 질문: "), "{tail}");
        // Only nodes that hold a match get a tail, and only inside the scope asked.
        assert!(
            hints
                .keys()
                .all(|node| { matches.iter().any(|item| &item.category_id == node) })
        );
        let only_account = word_matches(
            &state,
            "해외 배송이 통관에서 멈췄어요",
            |node| node == "account",
        )
        .unwrap();
        assert!(
            only_account
                .iter()
                .all(|item| item.category_id == "account")
        );
    }

    /// How many nodes of the menu a run showed filed questions under.
    async fn hinted(state: &AppState, request: RunRequest) -> u64 {
        let (tx, mut rx) = mpsc::channel(1024);
        execute(state.clone(), request, Some(tx)).await.unwrap();
        let mut hinted = 0;
        while let Ok(event) = rx.try_recv() {
            if event["type"] == "descent_step" && event["status"] == "asking" {
                hinted = event["hinted"].as_u64().unwrap();
            }
        }
        hinted
    }

    #[tokio::test]
    async fn a_search_reads_the_filed_questions_and_an_ingest_does_not() {
        let (_dir, state) = fixture_state();
        state.jev.script_choices(vec!["orders_tracking".into()]);
        assert!(hinted(&state, run_req("해외 배송이 통관에서 멈췄어요")).await > 0);
        // The same words as a new Q&A: nothing filed is the same question, and
        // look-alikes must not pull the filing.
        let ingest = ingest_request(
            "해외 배송이 통관에서 멈췄어요",
            "세관 통관 안내를 확인하세요.",
        );
        assert_eq!(hinted(&state, ingest).await, 0);
    }

    #[test]
    fn filed_questions_are_capped_per_node_per_question_and_per_menu() {
        let long = "가".repeat(300);
        let many: Vec<Item> = (0..200)
            .map(|index| Item {
                id: index.to_string(),
                category_id: format!("node_{}", index / 5),
                question: long.clone(),
                answer: String::new(),
                kind: "qa".into(),
                status: "active".into(),
                version: 1,
            })
            .collect();
        let hints = filed_hints(&many);
        let lead = " | 이미 있는 질문: ";
        for tail in hints.values() {
            let questions: Vec<&str> = tail[lead.len()..].split(" / ").collect();
            assert!(questions.len() <= HINTS_PER_NODE);
            assert!(questions.iter().all(|q| q.chars().count() == HINT_CHARS));
        }
        let bytes: usize = hints.values().map(String::len).sum();
        assert!(bytes <= HINT_MAX_BYTES, "{bytes}");
    }

    #[test]
    fn options_keep_the_order_given_and_none_comes_last() {
        // The evaluator leans toward what it reads first. This fails if
        // `serde_json` loses `preserve_order` and goes back to sorting keys,
        // which put `__none__` ahead of every lowercase id.
        let criteria = options(
            ["orders", "orders_tracking", "account", "Zeta"]
                .into_iter()
                .map(|id| (id.to_string(), format!("label {id}"))),
            Some("none of these"),
        );
        let sent = serde_json::to_string(&criteria).unwrap();
        let at = |key: &str| sent.find(&format!("\"{key}\":")).expect(key);
        assert!(at("orders") < at("orders_tracking"));
        assert!(at("orders_tracking") < at("account"));
        assert!(at("account") < at("Zeta"));
        assert!(at("Zeta") < at("__none__"));
    }

    #[tokio::test]
    async fn the_menu_is_asked_in_tree_order() {
        // Siblings sit next to each other and a child follows its parent,
        // whatever order the rows come out of the store in.
        let (_dir, state) = fixture_state();
        state.jev.script_choices(vec!["orders_tracking".into()]);
        let (tx, mut rx) = mpsc::channel(1024);
        execute(state, run_req("해외 배송이 통관에서 멈췄어요"), Some(tx))
            .await
            .unwrap();
        let mut asked = Vec::new();
        while let Ok(event) = rx.try_recv() {
            if event["type"] == "descent_step" && event["status"] == "asking" {
                asked = event["frontiers"][0]["children"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|child| child["id"].as_str().unwrap().to_string())
                    .collect();
            }
        }
        let at = |id: &str| asked.iter().position(|known| known == id).expect(id);
        assert_eq!(at("orders_tracking"), at("orders") + 1, "{asked:?}");
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
        assert!(result["abstained"].as_bool().unwrap(), "{result}");
        // Whatever the request's words bring in comes from inside the root.
        assert!(
            result["items"].as_array().unwrap().iter().all(|row| {
                row["item"]["category_id"]
                    .as_str()
                    .unwrap()
                    .starts_with("orders")
            }),
            "{result}"
        );
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
    fn llm_route_budget_leaves_room_for_the_ranking() {
        // A third of the run deadline: enough for a slow model to deliberate
        // and still leave the ranking time to finish.
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
            recall: crate::recall::Cache::default(),
            results: crate::cache::ResultCache::default(),
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
        // 리프에 아이템이 하나뿐이고 요청의 낱말로 딸려 오는 것도 없으면 가산할
        // 대상이 없다. 점수를 부풀리면 안 된다.
        let (_dir, state) = nested_state();
        state.jev.script_choices(vec!["topic_case".into()]);
        let result = execute(state, run_req("how is one specific case handled"), None)
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

    #[tokio::test]
    async fn a_scope_too_large_for_one_choice_routes_through_the_coarse_map() {
        // 258 nodes: the coarse map is the forest's top level, its best branch
        // opens whole, and the final choice sees that branch's leaf. The
        // request shares no word with the taxonomy; the coarse map reads meaning.
        let (_dir, state) = wide_state();
        state
            .jev
            .script_choices(vec!["orders_tracking".into(), "orders".into()]);
        let result = execute(state, run_req("where is my parcel stuck at customs"), None)
            .await
            .unwrap();
        assert_eq!(result["leaf_id"], "orders_tracking", "{result}");
        assert_eq!(result["trace"]["router"], "flat");
        let steps = result["trace"]["steps"].as_array().unwrap();
        assert_eq!(steps[0]["node_name"], "coarse map", "{result}");
        assert_eq!(steps[0]["choice_id"], "orders");
        let offered = steps[1]["candidates"].as_array().unwrap();
        assert!(offered.iter().any(|c| c["id"] == "orders_tracking"));
        assert!(offered.len() <= FLAT_MAX_NODES + 1);
    }

    #[tokio::test]
    async fn lexical_matches_join_the_coarse_branches() {
        // The coarse map opens the padding, but the request's words sit in a
        // question filed under `orders_tracking`: BM25 brings that node (and
        // its parent) into the menu next to the padding's 255 nodes.
        let (_dir, state) = wide_state();
        state
            .jev
            .script_choices(vec!["orders_tracking".into(), "pad".into()]);
        let result = execute(state, run_req("해외 배송이 통관에서 멈췄어요"), None)
            .await
            .unwrap();
        let steps = result["trace"]["steps"].as_array().unwrap();
        assert_eq!(steps[0]["choice_id"], "pad", "{result}");
        assert_eq!(result["leaf_id"], "orders_tracking", "{result}");
        let offered = steps[1]["candidates"].as_array().unwrap();
        assert!(offered.iter().any(|c| c["id"] == "orders"), "parent kept");
        assert!(
            offered.len() <= FLAT_MAX_NODES + 1,
            "{} options",
            offered.len()
        );
    }

    #[tokio::test]
    async fn a_branch_cut_from_the_menu_is_walked_into() {
        // `pad` has 300 children, more than one menu holds next to the rest:
        // the first choice can only stop at `pad`, so the walk goes on inside
        // it and reaches the child the request meant.
        let (_dir, state) = padded_state(300);
        state
            .jev
            .script_choices(vec!["pad_280".into(), "pad".into()]);
        let result = execute(state, run_req("where is that one"), None)
            .await
            .unwrap();
        assert_eq!(result["leaf_id"], "pad_280", "{result}");
        let steps = result["trace"]["steps"].as_array().unwrap();
        assert!(steps.len() >= 4, "two coarse-and-final rounds: {result}");
        assert_eq!(steps[1]["choice_id"], "pad", "the first round stops above");
        let path: Vec<&str> = result["path"]
            .as_array()
            .unwrap()
            .iter()
            .map(|part| part["id"].as_str().unwrap())
            .collect();
        assert_eq!(path, vec!["pad", "pad_280"]);
    }

    #[test]
    fn a_lone_start_is_not_a_coarse_map() {
        // Under a start node the top level is the start alone; the coarse map
        // is its children instead, split when they are too many for one choice.
        let mut nodes = vec![plain_node("top", "상위", "상위", None)];
        for i in 0..300 {
            nodes.push(plain_node(
                &format!("c{i}"),
                &format!("하위 {i}"),
                "하위",
                Some("top"),
            ));
        }
        let pool = ids_of(&nodes);
        let scope = ScopeIndex::new(&nodes, &pool);
        let coarse = scope.coarse();
        assert_eq!(coarse.iter().map(Vec::len).sum::<usize>(), 300);
        assert!(coarse.iter().all(|chunk| !chunk.contains(&"top")));
    }

    #[test]
    fn the_coarse_map_is_the_deepest_level_one_choice_can_name() {
        // 12 topics, 20 children each, 2 grandchildren each: 492 nodes. The
        // 480 grandchildren are too many for one choice; the 240 children fit.
        let mut nodes = Vec::new();
        for t in 0..12 {
            nodes.push(plain_node(
                &format!("t{t}"),
                &format!("주제 {t}"),
                "주제",
                None,
            ));
            for c in 0..20 {
                let id = format!("t{t}_{c}");
                nodes.push(plain_node(
                    &id,
                    &format!("하위 {t}-{c}"),
                    "하위",
                    Some(&format!("t{t}")),
                ));
                for g in 0..2 {
                    nodes.push(plain_node(
                        &format!("{id}_{g}"),
                        &format!("세부 {t}-{c}-{g}"),
                        "세부",
                        Some(&id),
                    ));
                }
            }
        }
        let pool = ids_of(&nodes);
        let scope = ScopeIndex::new(&nodes, &pool);
        let coarse = scope.coarse();
        assert_eq!(coarse.len(), 1);
        assert_eq!(coarse[0].len(), 240);
        assert!(coarse[0].iter().all(|id| id.matches('_').count() == 1));
    }

    #[test]
    fn a_forest_wider_than_one_choice_is_split_across_questions() {
        let nodes: Vec<Node> = (0..600)
            .map(|i| plain_node(&format!("n{i}"), &format!("분류 {i}"), "분류", None))
            .collect();
        let pool = ids_of(&nodes);
        let scope = ScopeIndex::new(&nodes, &pool);
        let coarse = scope.coarse();
        assert!(coarse.len() >= 3, "{} chunks", coarse.len());
        assert!(coarse.iter().all(|chunk| chunk.len() <= FLAT_MAX_NODES));
        assert_eq!(coarse.iter().map(Vec::len).sum::<usize>(), 600);
    }

    #[test]
    fn the_menu_fill_keeps_ancestors_and_stops_at_one_choice() {
        let mut nodes = vec![plain_node("topic", "주제", "일반 주제", None)];
        for i in 0..400 {
            nodes.push(plain_node(
                &format!("n{i:03}"),
                &format!("분류 {i}"),
                "안내",
                Some("topic"),
            ));
        }
        let pool = ids_of(&nodes);
        let scope = ScopeIndex::new(&nodes, &pool);
        let mut fill = MenuFill::new();
        fill.push(&scope, pool[322].as_str());
        assert_eq!(fill.ids, vec!["topic", "n321"], "the parent comes along");
        for id in &pool {
            fill.push(&scope, id.as_str());
        }
        assert_eq!(fill.ids.len(), FLAT_MAX_NODES);
        assert!(fill.bytes <= FLAT_MAX_MENU_BYTES);
        let menu = fill.finish(&scope);
        assert_eq!(menu[0], "topic", "tree order");
        assert_eq!(menu.len(), FLAT_MAX_NODES);
    }

    #[tokio::test]
    async fn a_doubtful_pick_is_settled_by_the_llm() {
        // Jev picks the parent at 0.5; the LLM reads the same menu and lands on
        // the child. The route is the LLM's, so it carries no probability.
        let (_dir, state) = fixture_state();
        state.jev.script_choices(vec!["~orders".into()]);
        state.jev.script_llm(Some("orders_tracking".into()));
        let result = execute(state, run_req("해외 배송이 통관에서 멈췄어요"), None)
            .await
            .unwrap();
        assert_eq!(result["leaf_id"], "orders_tracking", "{result}");
        assert_eq!(result["trace"]["router"], "llm");
        assert!(result["trace"]["score"].is_null(), "{result}");
        let steps = result["trace"]["steps"].as_array().unwrap();
        assert_eq!(steps.last().unwrap()["node_name"], "llm review");
        assert_eq!(result["path"][0]["id"], "orders");
    }

    #[tokio::test]
    async fn the_llm_review_can_say_nothing_fits() {
        let (_dir, state) = fixture_state();
        state.jev.script_choices(vec!["~orders".into()]);
        state.jev.script_llm(Some("__none__".into()));
        let result = execute(state, run_req("안드로메다 은하까지의 거리"), None)
            .await
            .unwrap();
        assert!(result["leaf_id"].is_null(), "{result}");
        assert!(result["items"].as_array().unwrap().is_empty());
    }

    #[tokio::test]
    async fn a_confident_pick_never_waits_for_the_llm() {
        let (_dir, state) = fixture_state();
        state.jev.script_choices(vec!["orders_tracking".into()]);
        state.jev.script_llm(Some("account".into()));
        let result = execute(state, run_req("해외 배송이 통관에서 멈췄어요"), None)
            .await
            .unwrap();
        assert_eq!(result["leaf_id"], "orders_tracking", "{result}");
        assert_eq!(result["trace"]["router"], "flat");
    }

    #[tokio::test]
    async fn a_failed_review_keeps_the_jev_pick() {
        let (_dir, state) = fixture_state();
        state.jev.reconfigure(crate::jev::JevConfig {
            llm_base_url: "http://127.0.0.1:9".into(),
            llm_token: Some("not-a-real-token".into()),
            llm_model: "unused".into(),
            ..crate::jev::JevConfig::default()
        });
        state.jev.script_choices(vec!["~orders_tracking".into()]);
        let result = execute(state, run_req("해외 배송이 통관에서 멈췄어요"), None)
            .await
            .unwrap();
        assert_eq!(result["leaf_id"], "orders_tracking", "{result}");
        assert_eq!(result["trace"]["router"], "flat");
        let steps = result["trace"]["steps"].as_array().unwrap();
        assert_eq!(steps.len(), 1, "no review step, and nothing asked after it");
    }

    #[tokio::test]
    async fn an_unsure_pick_also_ranks_what_its_runners_up_hold() {
        // `account` wins at 0.5: the search ranks its item and, next to it, the
        // runners-up `orders` and its child, so a sideways miss keeps the answer
        // in reach. The routed node's own item still leads a tie.
        let (_dir, state) = fixture_state();
        state.jev.script_choices(vec!["~account".into()]);
        let result = execute(state, run_req("해외 배송이 통관에서 멈췄어요"), None)
            .await
            .unwrap();
        assert_eq!(result["leaf_id"], "account", "{result}");
        let categories: Vec<&str> = result["items"]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| row["item"]["category_id"].as_str().unwrap())
            .collect();
        assert_eq!(categories.first(), Some(&"account"), "{result}");
        assert!(categories.contains(&"orders_tracking"), "{result}");
    }

    #[tokio::test]
    async fn a_confident_pick_takes_no_runners_up() {
        // Nothing outside `account` shares a word with this request, so the
        // pool is the routed subtree alone: runners-up are for unsure picks.
        let (_dir, state) = fixture_state();
        state.jev.script_choices(vec!["account".into()]);
        let result = execute(state, run_req("password reset"), None)
            .await
            .unwrap();
        let items = result["items"].as_array().unwrap();
        assert!(!items.is_empty(), "{result}");
        assert!(
            items
                .iter()
                .all(|row| row["item"]["category_id"] == "account"),
            "{result}"
        );
    }

    #[tokio::test]
    async fn items_that_share_the_requests_words_join_the_pool() {
        // A confident route to the wrong branch: the item that answers is
        // filed under `orders_tracking`, and its own words bring it in.
        let (_dir, state) = fixture_state();
        state.jev.script_choices(vec!["account".into()]);
        let result = execute(state, run_req("해외 배송이 통관에서 멈췄어요"), None)
            .await
            .unwrap();
        assert_eq!(result["leaf_id"], "account", "{result}");
        let categories: Vec<&str> = result["items"]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| row["item"]["category_id"].as_str().unwrap())
            .collect();
        assert!(categories.contains(&"orders_tracking"), "{result}");
    }

    #[tokio::test]
    async fn word_matches_stay_inside_the_root_and_the_start_node() {
        let (_dir, state) = fixture_state();
        state.jev.script_choices(vec!["account".into()]);
        let mut request = run_req("해외 배송이 통관에서 멈췄어요");
        request.start_node = Some("account".into());
        let result = execute(state.clone(), request, None).await.unwrap();
        assert!(
            result["items"]
                .as_array()
                .unwrap()
                .iter()
                .all(|row| row["item"]["category_id"] == "account"),
            "{result}"
        );
        let mut request = run_req("해외 배송이 통관에서 멈췄어요");
        request.root = Some("account".into());
        let result = execute(state, request, None).await.unwrap();
        assert!(
            result["items"]
                .as_array()
                .unwrap()
                .iter()
                .all(|row| row["item"]["category_id"] == "account"),
            "{result}"
        );
    }

    async fn events_of(state: &AppState, query: &str) -> Vec<Value> {
        let (tx, mut rx) = mpsc::channel(1024);
        execute(state.clone(), run_req(query), Some(tx))
            .await
            .unwrap();
        let mut events = Vec::new();
        while let Ok(event) = rx.try_recv() {
            events.push(event);
        }
        events
    }

    #[tokio::test]
    async fn a_repeated_search_is_served_from_the_cache_until_the_knowledge_changes() {
        let (_dir, state) = fixture_state();
        state.jev.script_choices(vec!["orders_tracking".into()]);
        let first = events_of(&state, "해외 배송이 통관에서 멈췄어요").await;
        assert!(
            first.iter().any(|e| e["type"] == "usage"),
            "the first one pays"
        );
        let again = events_of(&state, "해외 배송이 통관에서 멈췄어요").await;
        assert!(again.iter().any(|e| e["type"] == "cache_hit"));
        assert!(!again.iter().any(|e| e["type"] == "usage"), "no model call");
        let done = again.iter().find(|e| e["type"] == "search_done").unwrap();
        assert_eq!(done["leaf_id"], "orders_tracking");
        // A write anywhere moves the store's generation: the next search pays again.
        state
            .store
            .upsert("account", "새 질문", "새 답", "qa", None, None, true)
            .unwrap();
        let after = events_of(&state, "해외 배송이 통관에서 멈췄어요").await;
        assert!(!after.iter().any(|e| e["type"] == "cache_hit"));
    }

    #[test]
    fn review_runs_only_on_a_doubtful_pick_with_an_llm() {
        assert!(review_needed(Some(0.79), true));
        assert!(!review_needed(Some(LLM_REVIEW_BELOW), true));
        assert!(!review_needed(Some(0.5), false));
        assert!(!review_needed(None, true));
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
        let on_hold = 0.7 * 0.7 + (0.4_f64 / 2.0) * 0.3;
        assert!(on_recommended < 0.65 && on_alternative < 0.4 && on_hold < HOLD_BELOW);
        assert_eq!(role_for(on_recommended), "recommended");
        assert_eq!(role_for(on_alternative), "alternative");
        assert!(!holds(on_hold), "the line itself answers");
        // A real step under a line still falls on the lower side.
        assert_eq!(role_for(0.649), "alternative");
        assert_eq!(role_for(0.399), "reference");
        assert!(holds(0.549));
        assert!(!holds(0.55));
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
