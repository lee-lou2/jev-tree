//! Per-case scoring and per-cell aggregation. Metric definitions live in eval/METHOD.md.

use crate::data::{Case, Kb, Resolved, Variant};
use crate::exec::{Record, SearchOut};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// Everything the report needs about one executed case. `None` = not applicable.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Score {
    /// The run finished without an unexpected error.
    pub ok: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    // Routing (search and ingest)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route_ok: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route_err: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hf: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gold_depth: Option<usize>,
    // Search
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answered: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answer_ok: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confident_ok: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hit1: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hit3: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hitk: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rr: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ndcg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recall: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rec_precision: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_score: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_grade: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_role: Option<String>,
    // Ingest
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filed_ok: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reject_ok: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft_ok: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_ok: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dup_tp: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dup_fp: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dup_fn: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_ok: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub live_demoted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probe_hit: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probe_answer_ok: Option<bool>,
    // Contract
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_ok: Option<bool>,
    // Cost
    pub ms: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jev_calls: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub llm_calls: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub llm_fallback: Option<bool>,
}

struct Route {
    ok: bool,
    kind: String,
    hf: f64,
    prefix: usize,
    gold_depth: usize,
}

fn hier_f1(variant: &Variant, gold: &str, got: &str) -> f64 {
    let gold: BTreeSet<String> = variant.chain(gold).into_iter().collect();
    let got: BTreeSet<String> = variant.chain(got).into_iter().collect();
    if gold.is_empty() || got.is_empty() {
        return 0.0;
    }
    let both = gold.intersection(&got).count() as f64;
    let precision = both / got.len() as f64;
    let recall = both / gold.len() as f64;
    if precision + recall == 0.0 {
        0.0
    } else {
        2.0 * precision * recall / (precision + recall)
    }
}

fn route(kb: &Kb, variant: &Variant, expected: &Resolved, got: Option<&str>) -> Route {
    match (expected.node.as_deref(), got) {
        (None, None) => Route {
            ok: true,
            kind: "exact".into(),
            hf: 1.0,
            prefix: 0,
            gold_depth: 0,
        },
        (None, Some(_)) => Route {
            ok: false,
            kind: "false_tree".into(),
            hf: 0.0,
            prefix: 0,
            gold_depth: 0,
        },
        (Some(gold), None) => Route {
            ok: false,
            kind: "false_none".into(),
            hf: 0.0,
            prefix: 0,
            gold_depth: variant.depth(gold),
        },
        (Some(gold), Some(got)) => {
            let gold_chain = variant.chain(gold);
            let got_chain = variant.chain(got);
            let prefix = gold_chain
                .iter()
                .zip(&got_chain)
                .take_while(|(a, b)| a == b)
                .count();
            let ok = expected.accept.iter().any(|id| id == got);
            let kind = if ok {
                "exact"
            } else if gold_chain.iter().any(|id| id == got) {
                "shallow"
            } else if got_chain.iter().any(|id| id == gold) {
                "deep"
            } else if kb.domain(got) == kb.domain(gold) {
                "branch"
            } else {
                "domain"
            };
            let hf = expected
                .accept
                .iter()
                .map(|id| hier_f1(variant, id, got))
                .fold(0.0, f64::max);
            Route {
                ok,
                kind: kind.into(),
                hf,
                prefix,
                gold_depth: gold_chain.len(),
            }
        }
    }
}

fn ndcg(grades: &BTreeMap<String, u8>, returned: &[String]) -> f64 {
    let gain = |grade: u8| f64::from((1u32 << grade) - 1);
    let dcg: f64 = returned
        .iter()
        .enumerate()
        .map(|(rank, id)| gain(grades.get(id).copied().unwrap_or(0)) / ((rank + 2) as f64).log2())
        .sum();
    let mut ideal: Vec<u8> = grades.values().copied().collect();
    ideal.sort_unstable_by(|a, b| b.cmp(a));
    let idcg: f64 = ideal
        .iter()
        .take(returned.len().max(1))
        .enumerate()
        .map(|(rank, grade)| gain(*grade) / ((rank + 2) as f64).log2())
        .sum();
    if idcg == 0.0 { 0.0 } else { dcg / idcg }
}

fn score_search(kb: &Kb, variant: &Variant, record: &Record, out: &SearchOut) -> Score {
    let expected = &record.expected;
    let mut score = Score {
        ms: out.ms,
        route_ms: out.route_ms,
        pool: out.pool,
        jev_calls: Some(out.beam_rounds + out.pool.unwrap_or(0).div_ceil(32)),
        llm_calls: Some(out.llm_calls),
        llm_fallback: (record.router == "jev_llm").then_some(out.router == "beam"),
        ..Default::default()
    };
    if let Some(error) = &out.error {
        score.error = Some(error.code.clone());
        score.route_ok = Some(false);
        score.route_err = Some("error".into());
        score.hf = Some(0.0);
        score.answered = Some(false);
        score.answer_ok = Some(false);
        if expected.answerable {
            score.confident_ok = Some(false);
            score.hit1 = Some(false);
            score.hit3 = Some(false);
            score.hitk = Some(false);
            score.rr = Some(0.0);
            score.ndcg = Some(0.0);
            score.recall = Some(0.0);
        }
        return score;
    }
    score.ok = true;
    let routed = route(kb, variant, expected, out.leaf.as_deref());
    score.route_ok = Some(routed.ok);
    score.route_err = Some(routed.kind);
    score.hf = Some(routed.hf);
    score.prefix = expected.node.as_ref().map(|_| routed.prefix);
    score.gold_depth = expected.node.as_ref().map(|_| routed.gold_depth);
    let answered = out.leaf.is_some() && !out.abstained && !out.items.is_empty();
    score.answered = Some(answered);
    let grade = |id: &str| expected.grades.get(id).copied().unwrap_or(0);
    let top = out.items.first();
    score.top_score = top.map(|item| item.score);
    score.top_grade = top.map(|item| grade(&item.id));
    score.top_role = top.map(|item| item.role.clone());
    if expected.answerable {
        let top_ok = top.is_some_and(|item| grade(&item.id) == 2);
        score.answer_ok = Some(answered && top_ok);
        score.confident_ok =
            Some(answered && top_ok && top.is_some_and(|item| item.role == "recommended"));
        let ids: Vec<String> = out.items.iter().map(|item| item.id.clone()).collect();
        let first = ids.iter().position(|id| grade(id) == 2);
        score.hit1 = Some(first == Some(0));
        score.hit3 = Some(first.is_some_and(|rank| rank < 3));
        score.hitk = Some(first.is_some());
        score.rr = Some(first.map(|rank| 1.0 / (rank + 1) as f64).unwrap_or(0.0));
        score.ndcg = Some(ndcg(&expected.grades, &ids));
        let wanted = expected.grades.values().filter(|g| **g == 2).count();
        let found = ids.iter().filter(|id| grade(id) == 2).count();
        score.recall = Some(if wanted == 0 {
            0.0
        } else {
            found as f64 / wanted as f64
        });
    } else {
        score.answer_ok = Some(!answered);
    }
    if !out.abstained {
        let recommended: Vec<&str> = out
            .items
            .iter()
            .filter(|item| item.role == "recommended")
            .map(|item| item.id.as_str())
            .collect();
        if !recommended.is_empty() {
            let relevant = recommended.iter().filter(|id| grade(id) > 0).count();
            score.rec_precision = Some(relevant as f64 / recommended.len() as f64);
        }
    }
    score
}

fn score_ingest(kb: &Kb, variant: &Variant, record: &Record) -> Score {
    let expected = &record.expected;
    let Some(ingest) = &record.ingest else {
        return Score::default();
    };
    let draft = &ingest.draft;
    let rounds = draft.beam_rounds
        + ingest.publish.as_ref().map(|p| p.beam_rounds).unwrap_or(0)
        + ingest.probe.as_ref().map(|p| p.beam_rounds).unwrap_or(0);
    let ranked = ingest
        .probe
        .as_ref()
        .and_then(|p| p.pool)
        .unwrap_or(0)
        .div_ceil(32);
    let mut score = Score {
        ms: ingest.ms,
        route_ms: Some(draft.ms),
        jev_calls: Some(rounds + ranked),
        llm_calls: Some(
            draft.llm_calls
                + ingest.publish.as_ref().map(|p| p.llm_calls).unwrap_or(0)
                + ingest.probe.as_ref().map(|p| p.llm_calls).unwrap_or(0),
        ),
        llm_fallback: (record.router == "jev_llm" && draft.error.is_none())
            .then_some(draft.router == "beam"),
        ..Default::default()
    };
    let routed = route(kb, variant, expected, draft.category.as_deref());
    score.route_ok = Some(routed.ok);
    score.route_err = Some(routed.kind.clone());
    score.hf = Some(routed.hf);
    if expected.node.is_none() {
        // A refusal is the right outcome: 400, nothing written.
        let refused = draft
            .error
            .as_ref()
            .is_some_and(|e| e.code == "invalid_request");
        score.ok = draft.error.is_none() || refused;
        score.error = draft
            .error
            .as_ref()
            .filter(|_| !refused)
            .map(|e| e.code.clone());
        score.reject_ok = Some(refused && ingest.rows_after_draft == ingest.rows_before);
        score.route_ok = Some(refused);
        score.route_err = Some(if refused { "exact" } else { "false_tree" }.into());
        return score;
    }
    if let Some(error) = &draft.error {
        score.error = Some(error.code.clone());
        score.filed_ok = Some(false);
        score.route_ok = Some(false);
        score.route_err = Some(if error.code == "invalid_request" {
            "false_none".into()
        } else {
            "error".into()
        });
        score.hf = Some(0.0);
        score.ok = error.code == "invalid_request";
        return score;
    }
    score.ok = true;
    score.filed_ok = Some(routed.ok);
    score.prefix = Some(routed.prefix);
    score.gold_depth = Some(routed.gold_depth);
    score.draft_ok = Some(
        draft.published_id.is_none()
            && ingest.draft_status.as_deref() == Some("draft")
            && ingest.draft_active == Some(false)
            && ingest.draft_search_leak != Some(true),
    );
    if let Some(publish) = &ingest.publish {
        score.publish_ok = Some(
            publish.error.is_none()
                && publish.published_id.is_some()
                && publish.published_id == draft.item_id
                && publish.version == draft.version + 1,
        );
        score.stable = Some(publish.error.is_none() && publish.category == draft.category);
    }
    if draft.category == expected.node {
        let got: BTreeSet<&String> = draft.duplicate_ids.iter().collect();
        let want: BTreeSet<&String> = expected.duplicate_ids.iter().collect();
        score.dup_tp = Some(got.intersection(&want).count());
        score.dup_fp = Some(got.difference(&want).count());
        score.dup_fn = Some(want.difference(&got).count());
    }
    score.update_ok = Some(match &expected.update_id {
        Some(id) => draft.updated && draft.item_id.as_ref() == Some(id),
        None => !draft.updated,
    });
    score.live_demoted = expected
        .update_id
        .as_ref()
        .map(|id| draft.updated && draft.item_id.as_ref() == Some(id));
    if let (Some(probe), Some(new_id)) = (&ingest.probe, &draft.item_id) {
        score.probe_hit = Some(probe.items.iter().any(|item| &item.id == new_id));
        score.probe_answer_ok = Some(
            probe.error.is_none()
                && !probe.abstained
                && probe.items.first().is_some_and(|item| &item.id == new_id),
        );
    }
    score
}

pub fn score(kb: &Kb, variant: &Variant, case: &Case, record: &Record) -> Score {
    let _ = case;
    match record.kind.as_str() {
        "search" => match &record.search {
            Some(out) => score_search(kb, variant, record, out),
            None => Score::default(),
        },
        "ingest" => score_ingest(kb, variant, record),
        _ => {
            let Some(out) = &record.contract else {
                return Score::default();
            };
            let got = out.error.as_ref().map(|e| e.code.clone());
            Score {
                ok: true,
                ms: out.ms,
                error: got.clone(),
                contract_ok: Some(got == record.expected.error),
                ..Default::default()
            }
        }
    }
}

pub fn one_line(record: &Record) -> String {
    let s = &record.score;
    let secs = s.ms as f64 / 1000.0;
    match record.kind.as_str() {
        "search" => {
            if let Some(error) = &s.error {
                return format!("ERROR {error} {secs:.1}s");
            }
            let got = record
                .search
                .as_ref()
                .and_then(|o| o.leaf.clone())
                .unwrap_or_else(|| "null".into());
            format!(
                "route={} ({got}) answer={} top={} {secs:.1}s",
                s.route_err.as_deref().unwrap_or("-"),
                if s.answer_ok == Some(true) {
                    "ok"
                } else {
                    "miss"
                },
                s.top_score
                    .map(|v| format!("{v:.2}"))
                    .unwrap_or_else(|| "-".into()),
            )
        }
        "ingest" => format!(
            "ingest={} route={} {secs:.1}s",
            if s.filed_ok == Some(true) || s.reject_ok == Some(true) {
                "ok"
            } else {
                "miss"
            },
            s.route_err.as_deref().unwrap_or("-"),
        ),
        _ => format!(
            "contract={} got={}",
            if s.contract_ok == Some(true) {
                "ok"
            } else {
                "FAIL"
            },
            s.error.as_deref().unwrap_or("success")
        ),
    }
}

// ---------------------------------------------------------------------------
// Aggregation
// ---------------------------------------------------------------------------

/// A rate or mean with a 95% interval (Wilson for rates, normal for means).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Metric {
    pub n: usize,
    pub value: Option<f64>,
    pub lo: Option<f64>,
    pub hi: Option<f64>,
}

impl Metric {
    pub fn rate(flags: impl IntoIterator<Item = bool>) -> Metric {
        let (mut hits, mut n) = (0usize, 0usize);
        for flag in flags {
            n += 1;
            if flag {
                hits += 1;
            }
        }
        if n == 0 {
            return Metric::default();
        }
        let p = hits as f64 / n as f64;
        let z = 1.96f64;
        let nf = n as f64;
        let denom = 1.0 + z * z / nf;
        let center = (p + z * z / (2.0 * nf)) / denom;
        let half = z * (p * (1.0 - p) / nf + z * z / (4.0 * nf * nf)).sqrt() / denom;
        Metric {
            n,
            value: Some(p),
            lo: Some((center - half).max(0.0)),
            hi: Some((center + half).min(1.0)),
        }
    }

    pub fn mean(values: impl IntoIterator<Item = f64>) -> Metric {
        let values: Vec<f64> = values.into_iter().collect();
        let n = values.len();
        if n == 0 {
            return Metric::default();
        }
        let mean = values.iter().sum::<f64>() / n as f64;
        let var = if n > 1 {
            values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1) as f64
        } else {
            0.0
        };
        let half = 1.96 * (var / n as f64).sqrt();
        Metric {
            n,
            value: Some(mean),
            lo: Some(mean - half),
            hi: Some(mean + half),
        }
    }
}

pub fn percentile(values: &[u64], p: f64) -> Option<u64> {
    if values.is_empty() {
        return None;
    }
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    let rank = ((p / 100.0) * (sorted.len() - 1) as f64).round() as usize;
    sorted.get(rank.min(sorted.len() - 1)).copied()
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchSummary {
    pub n: usize,
    pub errors: usize,
    pub error_codes: BTreeMap<String, usize>,
    pub e2e: Metric,
    pub e2e_answerable: Metric,
    pub e2e_unanswerable: Metric,
    pub confident: Metric,
    pub route_acc: Metric,
    pub hf: Metric,
    pub oos_precision: Metric,
    pub oos_recall: Metric,
    pub oos_f1: Option<f64>,
    pub false_abstain: Metric,
    pub false_answer: Metric,
    pub hit1: Metric,
    pub hit3: Metric,
    pub hitk: Metric,
    pub mrr: Metric,
    pub ndcg: Metric,
    pub recall: Metric,
    pub rec_precision: Metric,
    pub hit1_given_route: Metric,
    pub mrr_given_route: Metric,
    pub p50_ms: Option<u64>,
    pub p90_ms: Option<u64>,
    pub p95_ms: Option<u64>,
    pub mean_ms: Option<f64>,
    pub mean_route_ms: Option<f64>,
    pub mean_pool: Option<f64>,
    pub mean_jev_calls: Option<f64>,
    pub mean_llm_calls: Option<f64>,
    pub llm_fallback: Metric,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IngestSummary {
    pub n: usize,
    pub errors: usize,
    pub ingest_acc: Metric,
    pub file_acc: Metric,
    pub reject_acc: Metric,
    pub hf: Metric,
    pub draft_ok: Metric,
    pub publish_ok: Metric,
    pub stable: Metric,
    pub dup_precision: Option<f64>,
    pub dup_recall: Option<f64>,
    pub update_ok: Metric,
    pub live_demoted: usize,
    pub probe_hit: Metric,
    pub probe_answer: Metric,
    pub p50_ms: Option<u64>,
    pub llm_fallback: Metric,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContractSummary {
    pub n: usize,
    pub pass: Metric,
    pub failed: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Row {
    pub key: String,
    pub n: usize,
    pub e2e: Option<f64>,
    pub route_acc: Option<f64>,
    pub hit1: Option<f64>,
    pub mrr: Option<f64>,
    pub p50_ms: Option<u64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Sweep {
    pub threshold: f64,
    pub e2e: f64,
    pub coverage: f64,
    pub precision: f64,
    pub false_answer: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Stability {
    pub cases: usize,
    pub same_leaf: Metric,
    pub same_top1: Metric,
    pub same_answer: Metric,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Miss {
    pub case_id: String,
    pub kind: String,
    pub text: String,
    pub expected: String,
    pub got: String,
    pub why: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CellSummary {
    pub router: String,
    pub variant: String,
    pub search: SearchSummary,
    pub ingest: IngestSummary,
    pub contract: ContractSummary,
    /// Share of forest search cases whose top k levels match the gold path.
    pub levels: Vec<Metric>,
    pub route_errors: BTreeMap<String, usize>,
    pub confusions: Vec<(String, String, usize)>,
    pub breakdowns: BTreeMap<String, Vec<Row>>,
    pub sweep: Vec<Sweep>,
    pub stability: Option<Stability>,
    pub misses: Vec<Miss>,
}

const FACETS: &[&str] = &[
    "expect", "domain", "turns", "ctx", "entry", "level", "depth", "pool", "style", "diff", "split",
];

fn rows(records: &[&Record], facet: &str) -> Vec<Row> {
    let mut groups: BTreeMap<String, Vec<&Record>> = BTreeMap::new();
    for record in records {
        let key = record
            .facets
            .get(facet)
            .cloned()
            .unwrap_or_else(|| "-".into());
        groups.entry(key).or_default().push(record);
    }
    groups
        .into_iter()
        .map(|(key, group)| {
            let ms: Vec<u64> = group.iter().map(|r| r.score.ms).collect();
            Row {
                key,
                n: group.len(),
                e2e: Metric::rate(group.iter().filter_map(|r| r.score.answer_ok)).value,
                route_acc: Metric::rate(group.iter().filter_map(|r| r.score.route_ok)).value,
                hit1: Metric::rate(group.iter().filter_map(|r| r.score.hit1)).value,
                mrr: Metric::mean(group.iter().filter_map(|r| r.score.rr)).value,
                p50_ms: percentile(&ms, 50.0),
            }
        })
        .collect()
}

fn describe_got(record: &Record) -> String {
    if let Some(error) = &record.score.error {
        return format!("error {error}");
    }
    match record.kind.as_str() {
        "search" => {
            let Some(out) = &record.search else {
                return "-".into();
            };
            let leaf = out.leaf.clone().unwrap_or_else(|| "null".into());
            let top = out
                .items
                .first()
                .map(|item| {
                    format!(
                        "{} {:.2} {}",
                        item.key.clone().unwrap_or_else(|| item.id.clone()),
                        item.score,
                        item.role
                    )
                })
                .unwrap_or_else(|| "no items".into());
            format!(
                "{leaf}{} · top {top}",
                if out.abstained { " · abstained" } else { "" }
            )
        }
        "ingest" => record
            .ingest
            .as_ref()
            .map(|i| {
                i.draft
                    .category
                    .clone()
                    .unwrap_or_else(|| "rejected".into())
            })
            .unwrap_or_default(),
        _ => record
            .score
            .error
            .clone()
            .unwrap_or_else(|| "success".into()),
    }
}

fn describe_expected(record: &Record) -> String {
    let expected = &record.expected;
    match record.kind.as_str() {
        "contract" => expected.error.clone().unwrap_or_default(),
        _ => {
            let node = expected.node.clone().unwrap_or_else(|| "null".into());
            let mut text = if expected.accept.len() > 1 {
                format!("{node} (+{})", expected.accept.len() - 1)
            } else {
                node
            };
            if record.kind == "search" && !expected.answerable {
                text.push_str(" · no answer");
            }
            text
        }
    }
}

pub fn summarize(router: &str, variant: &str, records: &[Record]) -> CellSummary {
    let mut cell = CellSummary {
        router: router.into(),
        variant: variant.into(),
        ..Default::default()
    };
    let search: Vec<&Record> = records.iter().filter(|r| r.kind == "search").collect();
    let ingest: Vec<&Record> = records.iter().filter(|r| r.kind == "ingest").collect();
    let contract: Vec<&Record> = records.iter().filter(|r| r.kind == "contract").collect();

    // Search
    let s = &mut cell.search;
    s.n = search.len();
    for record in &search {
        if let Some(code) = &record.score.error {
            s.errors += 1;
            *s.error_codes.entry(code.clone()).or_default() += 1;
        }
    }
    let answerable: Vec<&&Record> = search.iter().filter(|r| r.expected.answerable).collect();
    let unanswerable: Vec<&&Record> = search.iter().filter(|r| !r.expected.answerable).collect();
    s.e2e = Metric::rate(search.iter().filter_map(|r| r.score.answer_ok));
    s.e2e_answerable = Metric::rate(answerable.iter().filter_map(|r| r.score.answer_ok));
    s.e2e_unanswerable = Metric::rate(unanswerable.iter().filter_map(|r| r.score.answer_ok));
    s.confident = Metric::rate(answerable.iter().filter_map(|r| r.score.confident_ok));
    s.route_acc = Metric::rate(search.iter().filter_map(|r| r.score.route_ok));
    s.hf = Metric::mean(search.iter().filter_map(|r| r.score.hf));
    let predicted_none: Vec<&&Record> = search
        .iter()
        .filter(|r| {
            r.score.error.is_none() && r.search.as_ref().is_some_and(|out| out.leaf.is_none())
        })
        .collect();
    let gold_none: Vec<&&Record> = search
        .iter()
        .filter(|r| r.expected.node.is_none())
        .collect();
    s.oos_precision = Metric::rate(predicted_none.iter().map(|r| r.expected.node.is_none()));
    s.oos_recall = Metric::rate(gold_none.iter().map(|r| {
        r.score.error.is_none() && r.search.as_ref().is_some_and(|out| out.leaf.is_none())
    }));
    if let (Some(p), Some(r)) = (s.oos_precision.value, s.oos_recall.value) {
        s.oos_f1 = Some(if p + r == 0.0 {
            0.0
        } else {
            2.0 * p * r / (p + r)
        });
    }
    s.false_abstain = Metric::rate(
        answerable
            .iter()
            .filter(|r| r.score.error.is_none())
            .filter_map(|r| r.score.answered.map(|answered| !answered)),
    );
    s.false_answer = Metric::rate(
        unanswerable
            .iter()
            .filter(|r| r.score.error.is_none())
            .filter_map(|r| r.score.answered),
    );
    s.hit1 = Metric::rate(answerable.iter().filter_map(|r| r.score.hit1));
    s.hit3 = Metric::rate(answerable.iter().filter_map(|r| r.score.hit3));
    s.hitk = Metric::rate(answerable.iter().filter_map(|r| r.score.hitk));
    s.mrr = Metric::mean(answerable.iter().filter_map(|r| r.score.rr));
    s.ndcg = Metric::mean(answerable.iter().filter_map(|r| r.score.ndcg));
    s.recall = Metric::mean(answerable.iter().filter_map(|r| r.score.recall));
    s.rec_precision = Metric::mean(search.iter().filter_map(|r| r.score.rec_precision));
    let routed: Vec<&&&Record> = answerable
        .iter()
        .filter(|r| r.score.route_ok == Some(true))
        .collect();
    s.hit1_given_route = Metric::rate(routed.iter().filter_map(|r| r.score.hit1));
    s.mrr_given_route = Metric::mean(routed.iter().filter_map(|r| r.score.rr));
    let ms: Vec<u64> = search.iter().map(|r| r.score.ms).collect();
    s.p50_ms = percentile(&ms, 50.0);
    s.p90_ms = percentile(&ms, 90.0);
    s.p95_ms = percentile(&ms, 95.0);
    s.mean_ms = Metric::mean(ms.iter().map(|v| *v as f64)).value;
    s.mean_route_ms = Metric::mean(
        search
            .iter()
            .filter_map(|r| r.score.route_ms.map(|v| v as f64)),
    )
    .value;
    s.mean_pool = Metric::mean(search.iter().filter_map(|r| r.score.pool.map(|v| v as f64))).value;
    s.mean_jev_calls = Metric::mean(
        search
            .iter()
            .filter_map(|r| r.score.jev_calls.map(|v| v as f64)),
    )
    .value;
    s.mean_llm_calls = Metric::mean(
        search
            .iter()
            .filter_map(|r| r.score.llm_calls.map(|v| v as f64)),
    )
    .value;
    s.llm_fallback = Metric::rate(search.iter().filter_map(|r| r.score.llm_fallback));

    // Level accuracy on forest cases: top k levels of the landed path match the gold path.
    let forest: Vec<&&Record> = search
        .iter()
        .filter(|r| r.facets.get("entry").map(String::as_str) == Some("forest"))
        .filter(|r| r.expected.node.is_some())
        .collect();
    let deepest = forest
        .iter()
        .filter_map(|r| r.score.gold_depth)
        .max()
        .unwrap_or(0);
    for level in 1..=deepest {
        cell.levels.push(Metric::rate(
            forest
                .iter()
                .filter(|r| r.score.gold_depth.unwrap_or(0) >= level)
                .map(|r| r.score.prefix.unwrap_or(0) >= level),
        ));
    }
    for record in search.iter().chain(ingest.iter()) {
        if let Some(kind) = &record.score.route_err {
            *cell.route_errors.entry(kind.clone()).or_default() += 1;
        }
    }
    let mut confusion: HashMap<(String, String), usize> = HashMap::new();
    for record in &search {
        if record.score.route_ok == Some(false) && record.score.error.is_none() {
            let gold = record
                .expected
                .node
                .clone()
                .unwrap_or_else(|| "null".into());
            let got = record
                .search
                .as_ref()
                .and_then(|out| out.leaf.clone())
                .unwrap_or_else(|| "null".into());
            *confusion.entry((gold, got)).or_default() += 1;
        }
    }
    let mut confusions: Vec<(String, String, usize)> = confusion
        .into_iter()
        .map(|((gold, got), n)| (gold, got, n))
        .collect();
    confusions.sort_by(|a, b| b.2.cmp(&a.2).then(a.0.cmp(&b.0)).then(a.1.cmp(&b.1)));
    confusions.truncate(20);
    cell.confusions = confusions;
    for facet in FACETS {
        cell.breakdowns
            .insert((*facet).to_string(), rows(&search, facet));
    }

    // Abstain threshold sweep on the recorded top scores.
    let valid: Vec<&&Record> = search.iter().filter(|r| r.score.error.is_none()).collect();
    if !valid.is_empty() {
        for threshold in [0.20, 0.25, 0.30, 0.35, 0.40, 0.50, 0.60, 0.65] {
            let mut correct = 0usize;
            let mut answered_answerable = 0usize;
            let mut answered = 0usize;
            let mut answered_right = 0usize;
            let mut unanswerable_answered = 0usize;
            let answerable_n = valid.iter().filter(|r| r.expected.answerable).count();
            let unanswerable_n = valid.len() - answerable_n;
            for record in &valid {
                let leaf = record.search.as_ref().is_some_and(|out| out.leaf.is_some());
                let says = leaf && record.score.top_score.is_some_and(|t| t >= threshold);
                let top_right = record.score.top_grade == Some(2);
                if record.expected.answerable {
                    if says {
                        answered_answerable += 1;
                        answered += 1;
                        if top_right {
                            answered_right += 1;
                            correct += 1;
                        }
                    }
                } else if says {
                    answered += 1;
                    unanswerable_answered += 1;
                } else {
                    correct += 1;
                }
            }
            cell.sweep.push(Sweep {
                threshold,
                e2e: correct as f64 / valid.len() as f64,
                coverage: if answerable_n == 0 {
                    0.0
                } else {
                    answered_answerable as f64 / answerable_n as f64
                },
                precision: if answered == 0 {
                    0.0
                } else {
                    answered_right as f64 / answered as f64
                },
                false_answer: if unanswerable_n == 0 {
                    0.0
                } else {
                    unanswerable_answered as f64 / unanswerable_n as f64
                },
            });
        }
    }

    // Repeat stability.
    let mut by_case: BTreeMap<&str, Vec<&Record>> = BTreeMap::new();
    for record in &search {
        by_case
            .entry(record.case_id.as_str())
            .or_default()
            .push(record);
    }
    let repeated: Vec<&Vec<&Record>> = by_case.values().filter(|runs| runs.len() > 1).collect();
    if !repeated.is_empty() {
        let same = |runs: &Vec<&Record>, key: &dyn Fn(&Record) -> String| {
            let first = key(runs[0]);
            runs.iter().all(|run| key(run) == first)
        };
        let leaf = |r: &Record| format!("{:?}", r.search.as_ref().map(|o| o.leaf.clone()));
        let top = |r: &Record| {
            format!(
                "{:?}",
                r.search
                    .as_ref()
                    .and_then(|o| o.items.first().map(|i| i.id.clone()))
            )
        };
        let answer = |r: &Record| format!("{:?}", r.score.answer_ok);
        cell.stability = Some(Stability {
            cases: repeated.len(),
            same_leaf: Metric::rate(repeated.iter().map(|runs| same(runs, &leaf))),
            same_top1: Metric::rate(repeated.iter().map(|runs| same(runs, &top))),
            same_answer: Metric::rate(repeated.iter().map(|runs| same(runs, &answer))),
        });
    }

    // Ingest
    let g = &mut cell.ingest;
    g.n = ingest.len();
    g.errors = ingest.iter().filter(|r| !r.score.ok).count();
    g.ingest_acc = Metric::rate(
        ingest
            .iter()
            .filter_map(|r| r.score.filed_ok.or(r.score.reject_ok)),
    );
    g.file_acc = Metric::rate(ingest.iter().filter_map(|r| r.score.filed_ok));
    g.reject_acc = Metric::rate(ingest.iter().filter_map(|r| r.score.reject_ok));
    g.hf = Metric::mean(
        ingest
            .iter()
            .filter(|r| r.expected.node.is_some())
            .filter_map(|r| r.score.hf),
    );
    g.draft_ok = Metric::rate(ingest.iter().filter_map(|r| r.score.draft_ok));
    g.publish_ok = Metric::rate(ingest.iter().filter_map(|r| r.score.publish_ok));
    g.stable = Metric::rate(ingest.iter().filter_map(|r| r.score.stable));
    let tp: usize = ingest.iter().filter_map(|r| r.score.dup_tp).sum();
    let fp: usize = ingest.iter().filter_map(|r| r.score.dup_fp).sum();
    let fne: usize = ingest.iter().filter_map(|r| r.score.dup_fn).sum();
    g.dup_precision = (tp + fp > 0).then(|| tp as f64 / (tp + fp) as f64);
    g.dup_recall = (tp + fne > 0).then(|| tp as f64 / (tp + fne) as f64);
    g.update_ok = Metric::rate(ingest.iter().filter_map(|r| r.score.update_ok));
    g.live_demoted = ingest
        .iter()
        .filter(|r| r.score.live_demoted == Some(true))
        .count();
    g.probe_hit = Metric::rate(ingest.iter().filter_map(|r| r.score.probe_hit));
    g.probe_answer = Metric::rate(ingest.iter().filter_map(|r| r.score.probe_answer_ok));
    let ms: Vec<u64> = ingest.iter().map(|r| r.score.ms).collect();
    g.p50_ms = percentile(&ms, 50.0);
    g.llm_fallback = Metric::rate(ingest.iter().filter_map(|r| r.score.llm_fallback));

    // Contract
    cell.contract.n = contract.len();
    cell.contract.pass = Metric::rate(contract.iter().filter_map(|r| r.score.contract_ok));
    cell.contract.failed = contract
        .iter()
        .filter(|r| r.score.contract_ok != Some(true))
        .map(|r| r.case_id.clone())
        .collect();

    // Misses for the error analysis section.
    for record in records {
        let missed = match record.kind.as_str() {
            "search" => record.score.answer_ok != Some(true) || record.score.route_ok != Some(true),
            "ingest" => {
                record.score.filed_ok.or(record.score.reject_ok) != Some(true)
                    || record.score.draft_ok == Some(false)
                    || record.score.publish_ok == Some(false)
            }
            _ => record.score.contract_ok != Some(true),
        };
        if !missed {
            continue;
        }
        let why = match record.kind.as_str() {
            "search" => {
                let mut parts = Vec::new();
                if record.score.route_ok != Some(true) {
                    parts.push(format!(
                        "route {}",
                        record.score.route_err.as_deref().unwrap_or("-")
                    ));
                }
                if record.score.answer_ok != Some(true) {
                    parts.push(if record.expected.answerable {
                        if record.score.answered == Some(false) {
                            "abstained".to_string()
                        } else {
                            "wrong top1".to_string()
                        }
                    } else {
                        "answered unanswerable".to_string()
                    });
                }
                parts.join(", ")
            }
            "ingest" => record.score.route_err.clone().unwrap_or_default(),
            _ => "contract".into(),
        };
        cell.misses.push(Miss {
            case_id: record.case_id.clone(),
            kind: record.kind.clone(),
            text: record.text.chars().take(80).collect(),
            expected: describe_expected(record),
            got: describe_got(record),
            why,
        });
    }
    cell.misses.sort_by(|a, b| a.case_id.cmp(&b.case_id));
    cell.misses.dedup_by(|a, b| a.case_id == b.case_id);
    cell
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{Kb, Variant};
    use std::path::Path;

    fn kb() -> Kb {
        Kb::load(&Path::new(env!("CARGO_MANIFEST_DIR")).join("eval/fixtures/kb")).unwrap()
    }

    fn expected(node: Option<&str>, accept: &[&str]) -> Resolved {
        Resolved {
            root_path: None,
            start_node: None,
            node: node.map(str::to_string),
            accept: accept.iter().map(|s| s.to_string()).collect(),
            grades: BTreeMap::new(),
            answerable: true,
            domain: String::new(),
            level: String::new(),
            depth: 0,
            pool: 0,
            duplicate_ids: vec![],
            update_id: None,
            dropped: false,
            error: None,
        }
    }

    #[test]
    fn route_classifies_every_kind_of_miss() {
        let kb = kb();
        let variant = Variant::build(&kb, "standard").unwrap();
        let gold = "it_net_wifi_drop";
        let want = expected(Some(gold), &[gold]);
        let kind = |got: Option<&str>| route(&kb, &variant, &want, got).kind;
        assert_eq!(kind(Some(gold)), "exact");
        assert_eq!(kind(Some("it_net_wifi")), "shallow");
        assert_eq!(kind(Some("it_net_wifi_slow")), "branch");
        assert_eq!(kind(Some("money_card_limit")), "domain");
        assert_eq!(kind(None), "false_none");
        let vague = expected(Some("it_net"), &["it_net"]);
        assert_eq!(route(&kb, &variant, &vague, Some(gold)).kind, "deep");
        let outside = expected(None, &[]);
        assert!(route(&kb, &variant, &outside, None).ok);
        assert_eq!(
            route(&kb, &variant, &outside, Some(gold)).kind,
            "false_tree"
        );
    }

    #[test]
    fn hierarchical_f1_gives_partial_credit_to_siblings() {
        let kb = kb();
        let variant = Variant::build(&kb, "standard").unwrap();
        let exact = hier_f1(&variant, "it_net_wifi_drop", "it_net_wifi_drop");
        let sibling = hier_f1(&variant, "it_net_wifi_drop", "it_net_wifi_slow");
        let other = hier_f1(&variant, "it_net_wifi_drop", "money_card_limit");
        assert!((exact - 1.0).abs() < 1e-9);
        assert!(sibling > 0.5 && sibling < 1.0, "{sibling}");
        assert_eq!(other, 0.0);
    }

    #[test]
    fn ndcg_is_one_for_the_ideal_order() {
        let grades: BTreeMap<String, u8> = [("a".to_string(), 2), ("b".to_string(), 1)]
            .into_iter()
            .collect();
        let ideal = ndcg(&grades, &["a".into(), "b".into(), "c".into()]);
        let flipped = ndcg(&grades, &["b".into(), "a".into(), "c".into()]);
        let missing = ndcg(&grades, &["c".into(), "d".into()]);
        assert!((ideal - 1.0).abs() < 1e-9);
        assert!(flipped < ideal && flipped > 0.0);
        assert_eq!(missing, 0.0);
    }

    #[test]
    fn wilson_interval_stays_inside_zero_and_one() {
        let none = Metric::rate([false; 10]);
        assert_eq!(none.value, Some(0.0));
        assert_eq!(none.lo, Some(0.0));
        assert!((none.hi.unwrap() - 0.2775).abs() < 1e-3);
        let all = Metric::rate([true; 10]);
        assert_eq!(all.hi, Some(1.0));
        assert!(Metric::rate(std::iter::empty()).value.is_none());
    }

    #[test]
    fn percentile_picks_nearest_rank() {
        assert_eq!(percentile(&[5, 1, 3], 50.0), Some(3));
        assert_eq!(percentile(&[], 50.0), None);
    }
}
