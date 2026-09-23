//! Markdown report, the cumulative history file, and paired run comparison.

use crate::exec::Record;
use crate::score::{CellSummary, Metric, Row};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

fn pct(value: Option<f64>) -> String {
    value
        .map(|v| format!("{:.1}%", v * 100.0))
        .unwrap_or_else(|| "-".into())
}

fn num(value: Option<f64>) -> String {
    value
        .map(|v| format!("{v:.3}"))
        .unwrap_or_else(|| "-".into())
}

/// `72.3% [67.1–77.0] (n=250)`
fn rate(metric: &Metric) -> String {
    match (metric.value, metric.lo, metric.hi) {
        (Some(v), Some(lo), Some(hi)) => format!(
            "{:.1}% [{:.1}–{:.1}] (n={})",
            v * 100.0,
            lo * 100.0,
            hi * 100.0,
            metric.n
        ),
        (Some(v), _, _) => format!("{:.1}% (n={})", v * 100.0, metric.n),
        _ => "-".into(),
    }
}

fn mean(metric: &Metric) -> String {
    match (metric.value, metric.lo, metric.hi) {
        (Some(v), Some(lo), Some(hi)) => {
            format!(
                "{v:.3} [{:.3}–{:.3}] (n={})",
                lo.max(0.0),
                hi.min(1.0),
                metric.n
            )
        }
        (Some(v), _, _) => format!("{v:.3} (n={})", metric.n),
        _ => "-".into(),
    }
}

fn ms(value: Option<u64>) -> String {
    value
        .map(|v| format!("{:.1}s", v as f64 / 1000.0))
        .unwrap_or_else(|| "-".into())
}

fn md(text: &str) -> String {
    text.replace('|', "\\|").replace('\n', " ")
}

const FACET_TITLES: &[(&str, &str)] = &[
    ("expect", "기대 결과 유형"),
    ("turns", "턴 수"),
    ("ctx", "문맥 패턴"),
    ("entry", "진입점"),
    ("level", "목표 노드 종류"),
    ("depth", "목표 깊이"),
    ("pool", "착지 후보 수"),
    ("style", "질의 스타일"),
    ("domain", "영역"),
    ("diff", "난이도"),
    ("split", "split"),
];

fn facet_table(title: &str, rows: &[Row], out: &mut String) {
    let _ = writeln!(
        out,
        "**{title}**\n\n| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |\n|---|---:|---:|---:|---:|---:|---:|"
    );
    for row in rows {
        let _ = writeln!(
            out,
            "| {} | {} | {} | {} | {} | {} | {} |",
            row.key,
            row.n,
            pct(row.e2e),
            pct(row.route_acc),
            pct(row.hit1),
            num(row.mrr),
            ms(row.p50_ms)
        );
    }
    let _ = writeln!(out);
}

pub fn render(meta: &Value, cells: &[CellSummary]) -> String {
    let mut out = String::new();
    let text = |key: &str| meta[key].as_str().unwrap_or("-").to_string();
    let _ = writeln!(out, "# 평가 결과 — {}\n", text("run_id"));
    let _ = writeln!(
        out,
        "- 실행: {} → {} (UTC) · git `{}`{}",
        text("started_at"),
        text("finished_at"),
        meta["git"]["sha"].as_str().unwrap_or("-"),
        if meta["git"]["dirty"].as_bool().unwrap_or(false) {
            " (커밋 안 된 변경 있음)"
        } else {
            ""
        }
    );
    let _ = writeln!(
        out,
        "- 데이터셋: `{}` v{} · kb `{}` · cases `{}`",
        meta["dataset"]["name"].as_str().unwrap_or("-"),
        meta["dataset"]["version"].as_str().unwrap_or("-"),
        meta["dataset"]["kb_hash"].as_str().unwrap_or("-"),
        meta["dataset"]["cases_hash"].as_str().unwrap_or("-"),
    );
    let _ = writeln!(
        out,
        "- 설정: routers {} · variants {} · suites {} · split {} · repeat {} · concurrency {} · beam_width {} · limit {}",
        meta["routers"],
        meta["variants"],
        meta["suites"],
        text("split"),
        meta["repeat"],
        meta["concurrency"],
        meta["beam_width"],
        meta["limit"],
    );
    if let Some(models) = meta["models"].as_object() {
        for (router, info) in models {
            let _ = writeln!(
                out,
                "- 모델({router}): jev `{}`{}",
                info["jev_model"].as_str().unwrap_or("-"),
                info["llm_model"]
                    .as_str()
                    .map(|m| format!(" · LLM `{m}`"))
                    .unwrap_or_default()
            );
        }
    }
    if let Some(filter) = meta["filter"].as_str().filter(|f| !f.is_empty()) {
        let _ = writeln!(out, "- 필터: {filter}");
    }
    if text("mock") != "off" {
        let _ = writeln!(
            out,
            "- **mock {}**: 스크립트 평가기로 하네스만 점검한 실행이다. 품질 수치가 아니다.",
            text("mock")
        );
    }
    if let Some(note) = meta["note"].as_str().filter(|n| !n.is_empty()) {
        let _ = writeln!(out, "- 메모: {note}");
    }
    let _ = writeln!(out);

    let _ = writeln!(out, "## 요약\n");
    let _ = writeln!(
        out,
        "| router | variant | 검색 n | E2E 정답률 | 라우팅 정확도 | hF1 | Hit@1 | MRR | nDCG | OOS F1 | 오답변율 | 오보류율 | p50 / p95 | Jev 호출/건 | 오류 | 저장 정확도 | 계약 |\n|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---:|---:|---:|---:|"
    );
    for cell in cells {
        let s = &cell.search;
        let _ = writeln!(
            out,
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} / {} | {} | {} | {} | {} |",
            cell.router,
            cell.variant,
            s.n,
            pct(s.e2e.value),
            pct(s.route_acc.value),
            num(s.hf.value),
            pct(s.hit1.value),
            num(s.mrr.value),
            num(s.ndcg.value),
            num(s.oos_f1),
            pct(s.false_answer.value),
            pct(s.false_abstain.value),
            ms(s.p50_ms),
            ms(s.p95_ms),
            s.mean_jev_calls
                .map(|v| format!("{v:.1}"))
                .unwrap_or_else(|| "-".into()),
            s.errors,
            pct(cell.ingest.ingest_acc.value),
            pct(cell.contract.pass.value),
        );
    }
    let _ = writeln!(out);

    for cell in cells {
        let _ = writeln!(out, "## {} · {}\n", cell.router, cell.variant);
        let s = &cell.search;
        if s.n > 0 {
            let _ = writeln!(out, "### 검색 ({}건, 오류 {}건)\n", s.n, s.errors);
            if !s.error_codes.is_empty() {
                let codes: Vec<String> = s
                    .error_codes
                    .iter()
                    .map(|(code, n)| format!("{code} {n}"))
                    .collect();
                let _ = writeln!(out, "오류 코드: {}\n", codes.join(", "));
            }
            let _ = writeln!(out, "| 지표 | 값 |\n|---|---|");
            let lines: Vec<(&str, String)> = vec![
                ("E2E 정답률", rate(&s.e2e)),
                ("  └ 답할 수 있는 질문", rate(&s.e2e_answerable)),
                (
                    "  └ 답할 수 없는 질문(보류가 정답)",
                    rate(&s.e2e_unanswerable),
                ),
                ("확신 정답률 (top1 정답 + recommended)", rate(&s.confident)),
                ("라우팅 정확도 (정확 일치)", rate(&s.route_acc)),
                ("계층 F1 (hF1)", mean(&s.hf)),
                ("트리 밖 판정 precision", rate(&s.oos_precision)),
                ("트리 밖 판정 recall", rate(&s.oos_recall)),
                ("트리 밖 판정 F1", num(s.oos_f1)),
                ("오보류율 (답할 수 있는데 보류)", rate(&s.false_abstain)),
                ("오답변율 (답할 수 없는데 답함)", rate(&s.false_answer)),
                ("Hit@1", rate(&s.hit1)),
                ("Hit@3", rate(&s.hit3)),
                ("Hit@k (limit)", rate(&s.hitk)),
                ("MRR@k", mean(&s.mrr)),
                ("nDCG@k", mean(&s.ndcg)),
                ("Recall@k", mean(&s.recall)),
                ("recommended 정밀도", mean(&s.rec_precision)),
                ("라우팅이 맞았을 때 Hit@1", rate(&s.hit1_given_route)),
                ("라우팅이 맞았을 때 MRR", mean(&s.mrr_given_route)),
                (
                    "지연 p50 / p90 / p95 / 평균",
                    format!(
                        "{} / {} / {} / {}",
                        ms(s.p50_ms),
                        ms(s.p90_ms),
                        ms(s.p95_ms),
                        s.mean_ms
                            .map(|v| format!("{:.1}s", v / 1000.0))
                            .unwrap_or_else(|| "-".into())
                    ),
                ),
                (
                    "라우팅 구간 평균",
                    s.mean_route_ms
                        .map(|v| format!("{:.1}s", v / 1000.0))
                        .unwrap_or_else(|| "-".into()),
                ),
                (
                    "랭킹 후보 수 평균",
                    s.mean_pool
                        .map(|v| format!("{v:.1}"))
                        .unwrap_or_else(|| "-".into()),
                ),
                (
                    "Jev 호출 수 평균 (라운드 + 32개 배치)",
                    s.mean_jev_calls
                        .map(|v| format!("{v:.2}"))
                        .unwrap_or_else(|| "-".into()),
                ),
                (
                    "LLM 라우팅 호출 수 평균",
                    s.mean_llm_calls
                        .map(|v| format!("{v:.2}"))
                        .unwrap_or_else(|| "-".into()),
                ),
                ("LLM 실패 → Jev 빔 대체율", rate(&s.llm_fallback)),
            ];
            for (name, value) in lines {
                let _ = writeln!(out, "| {name} | {value} |");
            }
            let _ = writeln!(out);
            if !cell.levels.is_empty() {
                let _ = writeln!(
                    out,
                    "**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)\n"
                );
                let header: Vec<String> =
                    (1..=cell.levels.len()).map(|k| format!("d{k}")).collect();
                let _ = writeln!(
                    out,
                    "| | {} |\n|---|{}",
                    header.join(" | "),
                    "---:|".repeat(header.len())
                );
                let values: Vec<String> = cell
                    .levels
                    .iter()
                    .map(|m| format!("{} (n={})", pct(m.value), m.n))
                    .collect();
                let _ = writeln!(out, "| 일치율 | {} |\n", values.join(" | "));
            }
            if !cell.route_errors.is_empty() {
                let _ = writeln!(
                    out,
                    "**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)\n"
                );
                let keys: Vec<&String> = cell.route_errors.keys().collect();
                let _ = writeln!(
                    out,
                    "| {} |\n|{}",
                    keys.iter()
                        .map(|k| k.as_str())
                        .collect::<Vec<_>>()
                        .join(" | "),
                    "---:|".repeat(keys.len())
                );
                let values: Vec<String> =
                    cell.route_errors.values().map(|v| v.to_string()).collect();
                let _ = writeln!(out, "| {} |\n", values.join(" | "));
            }
            let _ = writeln!(out, "### 세부 분석\n");
            for (facet, title) in FACET_TITLES {
                if let Some(rows) = cell.breakdowns.get(*facet)
                    && !rows.is_empty()
                {
                    facet_table(title, rows, &mut out);
                }
            }
            if !cell.sweep.is_empty() {
                let _ = writeln!(
                    out,
                    "**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)\n\n| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |\n|---:|---:|---:|---:|---:|"
                );
                for row in &cell.sweep {
                    let _ = writeln!(
                        out,
                        "| {:.2} | {} | {} | {} | {} |",
                        row.threshold,
                        pct(Some(row.e2e)),
                        pct(Some(row.coverage)),
                        pct(Some(row.precision)),
                        pct(Some(row.false_answer))
                    );
                }
                let _ = writeln!(out);
            }
            if let Some(stability) = &cell.stability {
                let _ = writeln!(
                    out,
                    "**반복 안정성** ({}개 케이스를 여러 번 실행)\n\n| 지표 | 값 |\n|---|---|\n| 같은 착지 노드 | {} |\n| 같은 top1 | {} |\n| 같은 정오 판정 | {} |\n",
                    stability.cases,
                    rate(&stability.same_leaf),
                    rate(&stability.same_top1),
                    rate(&stability.same_answer)
                );
            }
        }
        let g = &cell.ingest;
        if g.n > 0 {
            let _ = writeln!(out, "### 저장 (ingest, {}건)\n", g.n);
            let _ = writeln!(out, "| 지표 | 값 |\n|---|---|");
            let lines: Vec<(&str, String)> = vec![
                ("저장 정확도 (저장 + 거절)", rate(&g.ingest_acc)),
                ("  └ 올바른 카테고리에 저장", rate(&g.file_acc)),
                ("  └ 트리/루트 밖 Q&A 거절", rate(&g.reject_acc)),
                ("계층 F1", mean(&g.hf)),
                ("초안이 검색에 안 보임", rate(&g.draft_ok)),
                ("게시 성공 (버전 +1)", rate(&g.publish_ok)),
                ("초안→게시 카테고리 유지", rate(&g.stable)),
                ("중복 탐지 precision", pct(g.dup_precision)),
                ("중복 탐지 recall", pct(g.dup_recall)),
                ("갱신/신규 판정 일치", rate(&g.update_ok)),
                (
                    "기존 게시 답변이 초안으로 내려간 건수 (관찰)",
                    g.live_demoted.to_string(),
                ),
                ("게시 후 probe 검색 top-k 포함", rate(&g.probe_hit)),
                ("게시 후 probe 검색 top1 답변", rate(&g.probe_answer)),
                ("지연 p50 (초안+게시+probe)", ms(g.p50_ms)),
                ("LLM 실패 → Jev 빔 대체율", rate(&g.llm_fallback)),
            ];
            for (name, value) in lines {
                let _ = writeln!(out, "| {name} | {value} |");
            }
            let _ = writeln!(out);
        }
        let c = &cell.contract;
        if c.n > 0 {
            let _ = writeln!(
                out,
                "### API 계약 ({}건)\n\n통과 {}{}\n",
                c.n,
                rate(&c.pass),
                if c.failed.is_empty() {
                    String::new()
                } else {
                    format!(" · 실패: {}", c.failed.join(", "))
                }
            );
        }
        if !cell.confusions.is_empty() {
            let _ = writeln!(
                out,
                "### 자주 틀린 착지 (기대 → 실제)\n\n| 기대 | 실제 | 건수 |\n|---|---|---:|"
            );
            for (gold, got, n) in &cell.confusions {
                let _ = writeln!(out, "| {gold} | {got} | {n} |");
            }
            let _ = writeln!(out);
        }
        if !cell.misses.is_empty() {
            let shown = cell.misses.len().min(60);
            let _ = writeln!(
                out,
                "### 틀린 케이스 ({}건 중 {}건)\n\n| id | 질의 | 기대 | 실제 | 이유 |\n|---|---|---|---|---|",
                cell.misses.len(),
                shown
            );
            for miss in cell.misses.iter().take(shown) {
                let _ = writeln!(
                    out,
                    "| {} | {} | {} | {} | {} |",
                    miss.case_id,
                    md(&miss.text),
                    md(&miss.expected),
                    md(&miss.got),
                    md(&miss.why)
                );
            }
            let _ = writeln!(out);
        }
    }
    out
}

// ---------------------------------------------------------------------------
// History
// ---------------------------------------------------------------------------

const HISTORY_HEADER: &str = "run_id,started_at,git,dirty,dataset_version,kb_hash,cases_hash,router,variant,mock,split,n_search,e2e,route_acc,hf1,hit1,mrr,ndcg,oos_f1,false_answer,false_abstain,p50_ms,p95_ms,jev_calls,errors,n_ingest,ingest_acc,probe_hit,contract_pass,note";

fn csv(value: &str) -> String {
    if value.contains([',', '"', '\n']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

fn f(value: Option<f64>) -> String {
    value.map(|v| format!("{v:.4}")).unwrap_or_default()
}

pub fn append_history(path: &Path, meta: &Value, cells: &[CellSummary]) -> Result<(), String> {
    let mut text = if path.exists() {
        std::fs::read_to_string(path).map_err(|e| e.to_string())?
    } else {
        format!("{HISTORY_HEADER}\n")
    };
    for cell in cells {
        let s = &cell.search;
        let row = [
            meta["run_id"].as_str().unwrap_or("").to_string(),
            meta["started_at"].as_str().unwrap_or("").to_string(),
            meta["git"]["sha"].as_str().unwrap_or("").to_string(),
            meta["git"]["dirty"].as_bool().unwrap_or(false).to_string(),
            meta["dataset"]["version"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            meta["dataset"]["kb_hash"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            meta["dataset"]["cases_hash"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            cell.router.clone(),
            cell.variant.clone(),
            meta["mock"].as_str().unwrap_or("").to_string(),
            meta["split"].as_str().unwrap_or("").to_string(),
            s.n.to_string(),
            f(s.e2e.value),
            f(s.route_acc.value),
            f(s.hf.value),
            f(s.hit1.value),
            f(s.mrr.value),
            f(s.ndcg.value),
            f(s.oos_f1),
            f(s.false_answer.value),
            f(s.false_abstain.value),
            s.p50_ms.map(|v| v.to_string()).unwrap_or_default(),
            s.p95_ms.map(|v| v.to_string()).unwrap_or_default(),
            f(s.mean_jev_calls),
            s.errors.to_string(),
            cell.ingest.n.to_string(),
            f(cell.ingest.ingest_acc.value),
            f(cell.ingest.probe_hit.value),
            f(cell.contract.pass.value),
            meta["note"].as_str().unwrap_or("").to_string(),
        ];
        let line: Vec<String> = row.iter().map(|v| csv(v)).collect();
        text.push_str(&line.join(","));
        text.push('\n');
    }
    std::fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))
}

// ---------------------------------------------------------------------------
// Paired comparison
// ---------------------------------------------------------------------------

/// Two-sided exact McNemar p-value for `b` vs `c` discordant pairs.
fn mcnemar(b: usize, c: usize) -> f64 {
    let n = b + c;
    if n == 0 {
        return 1.0;
    }
    let k = b.min(c);
    // P(X <= k) for X ~ Binomial(n, 0.5), computed in log space.
    let mut log_choose = 0.0f64; // ln C(n, 0)
    let mut total = 0.0f64;
    for i in 0..=k {
        if i > 0 {
            log_choose += ((n - i + 1) as f64).ln() - (i as f64).ln();
        }
        total += (log_choose - n as f64 * std::f64::consts::LN_2).exp();
    }
    (2.0 * total).min(1.0)
}

/// Percentile bootstrap interval of the mean paired difference.
fn bootstrap(diffs: &[f64]) -> (f64, f64) {
    if diffs.is_empty() {
        return (0.0, 0.0);
    }
    let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    let mut means = Vec::with_capacity(2000);
    for _ in 0..2000 {
        let mut sum = 0.0;
        for _ in 0..diffs.len() {
            sum += diffs[(next() % diffs.len() as u64) as usize];
        }
        means.push(sum / diffs.len() as f64);
    }
    means.sort_by(f64::total_cmp);
    (means[50], means[1949])
}

pub fn compare(a_label: &str, a: &[Record], b_label: &str, b: &[Record]) -> String {
    let mut out = String::new();
    let index = |records: &[Record]| -> BTreeMap<String, Record> {
        records
            .iter()
            .filter(|r| r.repeat == 0)
            .map(|r| (r.case_id.clone(), r.clone()))
            .collect()
    };
    let (a_map, b_map) = (index(a), index(b));
    let common: Vec<&String> = a_map.keys().filter(|id| b_map.contains_key(*id)).collect();
    let _ = writeln!(
        out,
        "# 비교 — A `{a_label}` vs B `{b_label}`\n\n공통 케이스 {}건 (A만 {}건, B만 {}건). 같은 케이스끼리 짝지은 paired 비교다.\n",
        common.len(),
        a_map.len() - common.len(),
        b_map.len() - common.len()
    );
    type Flag = fn(&Record) -> Option<bool>;
    let flags: [(&str, Flag); 5] = [
        ("E2E 정답", |r| r.score.answer_ok),
        ("라우팅 정확", |r| r.score.route_ok),
        ("Hit@1", |r| r.score.hit1),
        ("저장 정확", |r| r.score.filed_ok.or(r.score.reject_ok)),
        ("계약 통과", |r| r.score.contract_ok),
    ];
    let _ = writeln!(
        out,
        "| 지표 | n | A | B | B−A | A만 맞음 | B만 맞음 | McNemar p |\n|---|---:|---:|---:|---:|---:|---:|---:|"
    );
    for (name, flag) in flags {
        let pairs: Vec<(bool, bool)> = common
            .iter()
            .filter_map(|id| Some((flag(&a_map[*id])?, flag(&b_map[*id])?)))
            .collect();
        if pairs.is_empty() {
            continue;
        }
        let n = pairs.len() as f64;
        let a_rate = pairs.iter().filter(|p| p.0).count() as f64 / n;
        let b_rate = pairs.iter().filter(|p| p.1).count() as f64 / n;
        let only_a = pairs.iter().filter(|p| p.0 && !p.1).count();
        let only_b = pairs.iter().filter(|p| !p.0 && p.1).count();
        let _ = writeln!(
            out,
            "| {name} | {} | {:.1}% | {:.1}% | {:+.1}%p | {only_a} | {only_b} | {:.4} |",
            pairs.len(),
            a_rate * 100.0,
            b_rate * 100.0,
            (b_rate - a_rate) * 100.0,
            mcnemar(only_a, only_b)
        );
    }
    let diffs: Vec<f64> = common
        .iter()
        .filter_map(|id| Some(b_map[*id].score.rr? - a_map[*id].score.rr?))
        .collect();
    if !diffs.is_empty() {
        let delta = diffs.iter().sum::<f64>() / diffs.len() as f64;
        let (lo, hi) = bootstrap(&diffs);
        let _ = writeln!(
            out,
            "\nMRR 차이 (B−A): {delta:+.3}, 95% bootstrap [{lo:+.3}, {hi:+.3}] (n={})\n",
            diffs.len()
        );
    }
    let latency = |map: &BTreeMap<String, Record>| -> Vec<u64> {
        common.iter().map(|id| map[*id].score.ms).collect()
    };
    let (la, lb) = (latency(&a_map), latency(&b_map));
    let _ = writeln!(
        out,
        "지연 p50: A {} · B {} / p95: A {} · B {}\n",
        ms(crate::score::percentile(&la, 50.0)),
        ms(crate::score::percentile(&lb, 50.0)),
        ms(crate::score::percentile(&la, 95.0)),
        ms(crate::score::percentile(&lb, 95.0)),
    );
    let flipped = |from_a: bool| -> Vec<&String> {
        common
            .iter()
            .copied()
            .filter(|id| {
                let a_ok = a_map[*id].score.answer_ok.or(a_map[*id].score.filed_ok);
                let b_ok = b_map[*id].score.answer_ok.or(b_map[*id].score.filed_ok);
                if from_a {
                    a_ok == Some(true) && b_ok == Some(false)
                } else {
                    a_ok == Some(false) && b_ok == Some(true)
                }
            })
            .collect()
    };
    for (title, ids) in [
        ("B에서 나빠진 케이스", flipped(true)),
        ("B에서 좋아진 케이스", flipped(false)),
    ] {
        if ids.is_empty() {
            continue;
        }
        let _ = writeln!(
            out,
            "## {title} ({}건)\n\n| id | 질의 | A | B |\n|---|---|---|---|",
            ids.len()
        );
        for id in ids.iter().take(40) {
            let describe = |r: &Record| -> String {
                r.search
                    .as_ref()
                    .map(|s| {
                        format!(
                            "{} · {}",
                            s.leaf.clone().unwrap_or_else(|| "null".into()),
                            s.items
                                .first()
                                .and_then(|i| i.key.clone())
                                .unwrap_or_else(|| "-".into())
                        )
                    })
                    .or_else(|| {
                        r.ingest.as_ref().map(|i| {
                            i.draft
                                .category
                                .clone()
                                .unwrap_or_else(|| "rejected".into())
                        })
                    })
                    .unwrap_or_default()
            };
            let _ = writeln!(
                out,
                "| {id} | {} | {} | {} |",
                md(&a_map[*id].text.chars().take(60).collect::<String>()),
                md(&describe(&a_map[*id])),
                md(&describe(&b_map[*id]))
            );
        }
        let _ = writeln!(out);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mcnemar_matches_the_exact_binomial() {
        assert_eq!(mcnemar(0, 0), 1.0);
        // 10 discordant pairs all one way: 2 * 0.5^10.
        assert!((mcnemar(10, 0) - 2.0 / 1024.0).abs() < 1e-12);
        assert!(mcnemar(5, 5) > 0.99);
    }

    #[test]
    fn bootstrap_brackets_a_constant_difference() {
        let (lo, hi) = bootstrap(&[0.25; 40]);
        assert!((lo - 0.25).abs() < 1e-12 && (hi - 0.25).abs() < 1e-12);
    }

    #[test]
    fn csv_quotes_only_when_needed() {
        assert_eq!(csv("plain"), "plain");
        assert_eq!(csv("a,b"), "\"a,b\"");
        assert_eq!(csv("say \"hi\""), "\"say \"\"hi\"\"\"");
    }
}
