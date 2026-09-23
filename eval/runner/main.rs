//! Golden-set evaluation runner for jev-tree.
//!
//! ```text
//! cargo run --release --example eval -- validate
//! cargo run --release --example eval -- run --router both --variant all
//! ```
//!
//! See `eval/README.md` for the workflow and `eval/METHOD.md` for the metrics.

mod data;
mod exec;
mod report;
mod score;

use data::{Case, Findings, Kb, Variant};
use exec::{Credentials, Mock, Record, Router, Settings};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct Args {
    pub command: String,
    pub options: HashMap<String, String>,
    pub flags: HashSet<String>,
    pub positional: Vec<String>,
}

impl Args {
    fn parse() -> Args {
        let mut raw = std::env::args().skip(1);
        let command = raw.next().unwrap_or_default();
        let mut options = HashMap::new();
        let mut flags = HashSet::new();
        let mut positional = Vec::new();
        let rest: Vec<String> = raw.collect();
        let mut index = 0;
        while index < rest.len() {
            let arg = &rest[index];
            if let Some(name) = arg.strip_prefix("--") {
                if let Some((key, value)) = name.split_once('=') {
                    options.insert(key.to_string(), value.to_string());
                } else if index + 1 < rest.len() && !rest[index + 1].starts_with("--") {
                    options.insert(name.to_string(), rest[index + 1].clone());
                    index += 1;
                } else {
                    flags.insert(name.to_string());
                }
            } else {
                positional.push(arg.clone());
            }
            index += 1;
        }
        Args {
            command,
            options,
            flags,
            positional,
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.options.get(key).map(String::as_str)
    }

    pub fn flag(&self, key: &str) -> bool {
        self.flags.contains(key)
    }

    pub fn list(&self, key: &str) -> Vec<String> {
        self.get(key)
            .map(|value| {
                value
                    .split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(str::to_string)
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn number(&self, key: &str) -> Result<Option<usize>, String> {
        self.get(key)
            .map(|value| {
                value
                    .parse::<usize>()
                    .map_err(|_| format!("--{key} needs a number"))
            })
            .transpose()
    }

    pub fn eval_dir(&self) -> PathBuf {
        PathBuf::from(self.get("eval-dir").unwrap_or("eval"))
    }
}

const USAGE: &str = "\
usage: cargo run --release --example eval -- <command> [options]

commands
  validate [--strict] [--only a,b]  check the fixture and golden cases (no key needed)
  stats [--out FILE]                dataset composition as markdown (no key needed)
  export --variant NAME [--out F]   write one variant as a JEV_TREE_SEED file
  run [options]                     execute cases and record results
  report RUN_DIR                    rebuild summary.json and report.md from recorded cases
  compare RUN_A [RUN_B] [--a router/variant] [--b router/variant] [--out FILE]
                                    paired comparison of two cells

run options
  --router jev|jev_llm|both         routing configuration (default jev)
  --variant NAME,...|all            deep, standard, shallow, flat, sparse (default standard)
  --suite search,ingest,contract    suites to run (default all)
  --split dev|test|all              case split (default all)
  --ids ID,...  --grep TEXT         pick cases by id, or by id/text substring
  --tag key=value,...               filter by style, ctx, diff, turns, entry, domain, type
  --sample N [--seed S]             deterministic subset of N cases
  --repeat N                        run every case N times (stability)
  --concurrency N                   parallel cases (default 4)
  --timeout SECS                    per run deadline (default JEV_TREE_RUN_TIMEOUT_SECS or 120)
  --beam-width N  --limit N         override every case
  --verify-draft-search             also search before publishing to prove drafts stay hidden
  --settings-db PATH                read the Jev key and LLM settings from a local server DB
  --mock oracle|abstain             scripted evaluator, no key: tests the harness only
  --out DIR                         results directory (default eval/results/runs/<run id>)
  --note TEXT                       free text stored with the run
  --no-history                      do not append to eval/results/history.csv
  --no-preflight                    skip the one-case check that the key and LLM work
  --allow-llm-fallback              keep going when the LLM route falls back to the beam
  --dry-run                         print the plan and stop

credentials (never written to results)
  Jev: JEV_EVAL_JEV_KEY | JEV_TREE_INIT_API_KEY | TYPESAFE_API_KEY
       optional JEV_EVAL_JEV_MODEL, JEV_EVAL_JEV_BASE_URL
  LLM: JEV_EVAL_LLM_BASE_URL | JEV_TREE_INIT_BASE_URL, JEV_EVAL_LLM_TOKEN | JEV_TREE_INIT_TOKEN,
       JEV_EVAL_LLM_MODEL | JEV_TREE_INIT_MODEL

common options
  --eval-dir DIR                    dataset root (default: eval)
";

fn load(args: &Args) -> Result<(Kb, Vec<Case>, Vec<String>), String> {
    let root = args.eval_dir();
    let kb = Kb::load(&root.join("fixtures/kb"))?;
    let (cases, errors) = data::load_cases(&root.join("golden"))?;
    Ok((kb, cases, errors))
}

fn check(kb: &Kb, cases: &[Case], parse_errors: Vec<String>) -> Findings {
    let mut findings = Findings::default();
    findings.errors.extend(parse_errors);
    data::validate_kb(kb, &mut findings);
    data::validate_cases(kb, cases, &mut findings);
    findings
}

fn cmd_validate(args: &Args) -> Result<(), String> {
    let (kb, cases, parse_errors) = load(args)?;
    let mut findings = check(&kb, &cases, parse_errors);
    let only = args.list("only");
    if !only.is_empty() {
        let shown = |text: &String| only.iter().any(|word| text.contains(word.as_str()));
        findings.errors.retain(shown);
        findings.warnings.retain(shown);
    }
    for warning in &findings.warnings {
        println!("warning: {warning}");
    }
    for error in &findings.errors {
        println!("error: {error}");
    }
    let count = |kind: &str| cases.iter().filter(|c| c.kind == kind).count();
    println!(
        "kb {} v{} ({} nodes, {} items) · cases {} (search {}, ingest {}, contract {}) · {} errors, {} warnings",
        kb.manifest.name,
        kb.manifest.version,
        kb.nodes.len(),
        kb.items.len(),
        cases.len(),
        count("search"),
        count("ingest"),
        count("contract"),
        findings.errors.len(),
        findings.warnings.len()
    );
    if !findings.errors.is_empty() || (args.flag("strict") && !findings.warnings.is_empty()) {
        return Err("validation failed".into());
    }
    Ok(())
}

fn write_or_print(path: Option<&str>, text: &str) -> Result<(), String> {
    match path {
        Some(path) => {
            if let Some(parent) = Path::new(path).parent() {
                std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            std::fs::write(path, text).map_err(|e| format!("{path}: {e}"))?;
            eprintln!("wrote {path}");
            Ok(())
        }
        None => {
            println!("{text}");
            Ok(())
        }
    }
}

fn cmd_stats(args: &Args) -> Result<(), String> {
    let (kb, cases, _) = load(args)?;
    write_or_print(args.get("out"), &data::stats_markdown(&kb, &cases))
}

fn cmd_export(args: &Args) -> Result<(), String> {
    let (kb, _, _) = load(args)?;
    let name = args.get("variant").unwrap_or("standard");
    let variant = Variant::build(&kb, name)?;
    let text = serde_json::to_string_pretty(&variant.seed_json()).map_err(|e| e.to_string())?;
    write_or_print(args.get("out"), &text)
}

// ---------------------------------------------------------------------------
// run
// ---------------------------------------------------------------------------

/// UTC `(2026-09-23T22:40:05Z, 20260923-224005)` without a date crate.
fn utc_now() -> (String, String) {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let (days, rem) = (secs.div_euclid(86_400), secs.rem_euclid(86_400));
    // Days since 1970-01-01 to a civil date (H. Hinnant).
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    let (h, m, s) = (rem / 3600, (rem % 3600) / 60, rem % 60);
    (
        format!("{year:04}-{month:02}-{day:02}T{h:02}:{m:02}:{s:02}Z"),
        format!("{year:04}{month:02}{day:02}-{h:02}{m:02}{s:02}"),
    )
}

fn git(args: &[&str]) -> Option<String> {
    std::process::Command::new("git")
        .args(args)
        .output()
        .ok()
        .filter(|out| out.status.success())
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn case_matches(kb: &Kb, case: &Case, args: &Args) -> Result<bool, String> {
    let suites = args.list("suite");
    if !suites.is_empty() && !suites.iter().any(|s| s == "all" || *s == case.kind) {
        return Ok(false);
    }
    if let Some(split) = args.get("split")
        && split != "all"
        && case.split() != split
    {
        return Ok(false);
    }
    let ids = args.list("ids");
    if !ids.is_empty() && !ids.contains(&case.id) {
        return Ok(false);
    }
    if let Some(needle) = args.get("grep") {
        let text = format!("{} {} {}", case.id, case.input.query, case.input.question);
        if !text.contains(needle) {
            return Ok(false);
        }
    }
    for pair in args.list("tag") {
        let (key, want) = pair
            .split_once('=')
            .ok_or_else(|| format!("--tag {pair}: use key=value"))?;
        let have = match key {
            "style" => case.tags.style.clone().unwrap_or_default(),
            "ctx" => case.tags.ctx.clone().unwrap_or_default(),
            "diff" => case.tags.diff.clone().unwrap_or_default(),
            "turns" => case.turns().into(),
            "entry" => case.entry().into(),
            "type" => case.kind.clone(),
            "split" => case.split(),
            "domain" => case
                .expect
                .node
                .as_deref()
                .filter(|id| kb.has(id))
                .map(|id| kb.domain(id))
                .unwrap_or_else(|| "oos".into()),
            other => return Err(format!("--tag {other}: unknown key")),
        };
        if have != want {
            return Ok(false);
        }
    }
    Ok(true)
}

fn write_json(path: &Path, value: &Value) -> Result<(), String> {
    let text = serde_json::to_string_pretty(value).map_err(|e| e.to_string())?;
    std::fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))
}

fn write_records(path: &Path, records: &[Record]) -> Result<(), String> {
    let mut text = String::new();
    for record in records {
        text.push_str(&serde_json::to_string(record).map_err(|e| e.to_string())?);
        text.push('\n');
    }
    std::fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))
}

fn read_records(path: &Path) -> Result<Vec<Record>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    text.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str(line).map_err(|e| format!("{}: {e}", path.display())))
        .collect()
}

fn cmd_run(args: &Args) -> Result<(), String> {
    // Credentials first: reading a server DB sets JEV_TREE_DB, which must happen
    // before the runtime starts any thread.
    let creds = match args.get("settings-db") {
        Some(path) => Credentials::from_db(Path::new(path))?,
        None => Credentials::from_env(),
    };
    let (kb, cases, parse_errors) = load(args)?;
    let findings = check(&kb, &cases, parse_errors);
    if !findings.errors.is_empty() {
        for error in findings.errors.iter().take(20) {
            eprintln!("error: {error}");
        }
        return Err(format!(
            "{} dataset errors; run `validate` and fix them first",
            findings.errors.len()
        ));
    }
    let mock = match args.get("mock").unwrap_or("off") {
        "off" => Mock::Off,
        "oracle" => Mock::Oracle,
        "abstain" => Mock::Abstain,
        other => return Err(format!("--mock {other}: use oracle or abstain")),
    };
    let routers = match args.get("router").unwrap_or("jev") {
        "jev" => vec![Router::Jev],
        "jev_llm" | "llm" => vec![Router::JevLlm],
        "both" => vec![Router::Jev, Router::JevLlm],
        other => return Err(format!("--router {other}: use jev, jev_llm, or both")),
    };
    let variants: Vec<String> = match args.list("variant").as_slice() {
        [] => vec![kb.manifest.default_variant.clone()],
        [all] if all == "all" => ["deep", "standard", "shallow", "flat", "sparse"]
            .iter()
            .filter(|name| kb.manifest.variants.contains_key(**name))
            .map(|name| name.to_string())
            .collect(),
        names => names.to_vec(),
    };
    for name in &variants {
        if !kb.manifest.variants.contains_key(name) {
            return Err(format!("unknown variant {name}"));
        }
    }
    if mock == Mock::Off {
        if creds.jev_key.is_empty() {
            return Err(
                "no Jev key: set JEV_EVAL_JEV_KEY (or TYPESAFE_API_KEY), pass --settings-db, or use --mock oracle"
                    .into(),
            );
        }
        if routers.contains(&Router::JevLlm) && !creds.llm_ready() {
            return Err(
                "jev_llm needs JEV_EVAL_LLM_BASE_URL, JEV_EVAL_LLM_TOKEN and JEV_EVAL_LLM_MODEL (or --settings-db)"
                    .into(),
            );
        }
    }
    let mut selected = Vec::new();
    for case in &cases {
        if case_matches(&kb, case, args)? {
            selected.push(case.clone());
        }
    }
    if let Some(n) = args.number("sample")? {
        let seed = args.get("seed").unwrap_or("jev-eval");
        selected.sort_by_key(|case| {
            let digest = Sha256::digest(format!("{seed}:{}", case.id).as_bytes());
            digest[..8].to_vec()
        });
        selected.truncate(n);
        selected.sort_by(|a, b| a.id.cmp(&b.id));
    }
    if selected.is_empty() {
        return Err("no case matches the filters".into());
    }
    let timeout = args
        .number("timeout")?
        .map(|s| s as u64)
        .or_else(|| {
            std::env::var("JEV_TREE_RUN_TIMEOUT_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
        })
        .unwrap_or(120);
    let concurrency = args.number("concurrency")?.unwrap_or(4);
    let repeat = args.number("repeat")?.unwrap_or(1).max(1);
    let beam_width = args.number("beam-width")?;
    let limit = args.number("limit")?;
    let kb = Arc::new(kb);

    // Plan
    let mut plan: Plan = Vec::new();
    for router in &routers {
        for name in &variants {
            let variant = Variant::build(&kb, name)?;
            let mut runnable = Vec::new();
            let mut skipped = 0;
            for case in &selected {
                match data::resolve(&kb, &variant, case) {
                    Ok(resolved) => runnable.push((case.clone(), resolved)),
                    Err(_) => skipped += 1,
                }
            }
            eprintln!(
                "plan {}/{}: {} cases × {repeat} (skipped {skipped} not present in this variant)",
                router.name(),
                name,
                runnable.len()
            );
            plan.push((*router, Arc::new(variant), runnable));
        }
    }
    if args.flag("dry-run") {
        return Ok(());
    }

    let (started_at, stamp) = utc_now();
    let run_id = format!(
        "{stamp}-{}{}",
        routers
            .iter()
            .map(|r| r.name())
            .collect::<Vec<_>>()
            .join("+"),
        if mock == Mock::Off {
            String::new()
        } else {
            format!("-mock-{}", mock.name())
        }
    );
    // Mock runs only check the harness; keep them out of the results history.
    let out_dir = args.get("out").map(PathBuf::from).unwrap_or_else(|| {
        if mock == Mock::Off {
            args.eval_dir().join("results/runs").join(&run_id)
        } else {
            std::env::temp_dir().join("jev-eval-mock").join(&run_id)
        }
    });
    let filter: Vec<String> = ["suite", "split", "ids", "grep", "tag", "sample", "seed"]
        .iter()
        .filter_map(|key| args.get(key).map(|value| format!("--{key} {value}")))
        .collect();
    let mut meta = json!({
        "run_id": run_id,
        "started_at": started_at,
        "git": {
            "sha": git(&["rev-parse", "--short", "HEAD"]).unwrap_or_else(|| "unknown".into()),
            "dirty": git(&["status", "--porcelain"]).is_some_and(|s| !s.is_empty()),
        },
        "dataset": {
            "name": kb.manifest.name,
            "version": kb.manifest.version,
            "kb_hash": kb.hash,
            "cases_hash": data::cases_hash(&args.eval_dir().join("golden")),
        },
        "routers": routers.iter().map(|r| r.name()).collect::<Vec<_>>(),
        "variants": variants,
        "suites": if args.list("suite").is_empty() { vec!["all".to_string()] } else { args.list("suite") },
        "split": args.get("split").unwrap_or("all"),
        "filter": filter.join(" "),
        "cases": selected.len(),
        "repeat": repeat,
        "concurrency": concurrency,
        "timeout_secs": timeout,
        "beam_width": beam_width.map(|v| json!(v)).unwrap_or(json!("case/default")),
        "limit": limit.map(|v| json!(v)).unwrap_or(json!("case/default")),
        "mock": mock.name(),
        "verify_draft_search": args.flag("verify-draft-search"),
        "models": routers.iter().map(|r| (r.name().to_string(), creds.describe(*r))).collect::<BTreeMap<_, _>>(),
        "note": args.get("note").unwrap_or(""),
        "command": std::env::args().skip(1).collect::<Vec<_>>().join(" "),
    });

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| e.to_string())?;
    if mock == Mock::Off && !args.flag("no-preflight") {
        preflight(&runtime, &kb, &plan, &creds, timeout, args)?;
    }
    std::fs::create_dir_all(out_dir.join("cells")).map_err(|e| e.to_string())?;
    let mut cells = Vec::new();
    for (router, variant, runnable) in plan {
        if runnable.is_empty() {
            continue;
        }
        let settings = Arc::new(Settings {
            router,
            mock,
            creds: creds.clone(),
            concurrency,
            timeout: Duration::from_secs(timeout),
            beam_width,
            limit,
            verify_draft_search: args.flag("verify-draft-search"),
            repeat,
        });
        let label = format!("{}/{}", router.name(), variant.name);
        let records = runtime.block_on(exec::run_cell(exec::CellInput {
            kb: kb.clone(),
            variant: variant.clone(),
            settings,
            cases: runnable,
            label,
        }))?;
        write_records(
            &out_dir
                .join("cells")
                .join(format!("{}__{}.jsonl", router.name(), variant.name)),
            &records,
        )?;
        cells.push(score::summarize(router.name(), &variant.name, &records));
    }
    let (finished_at, _) = utc_now();
    meta["finished_at"] = json!(finished_at);
    write_json(&out_dir.join("meta.json"), &meta)?;
    write_json(
        &out_dir.join("summary.json"),
        &json!({"meta": meta, "cells": cells}),
    )?;
    let report_text = report::render(&meta, &cells);
    std::fs::write(out_dir.join("report.md"), &report_text).map_err(|e| e.to_string())?;
    if mock == Mock::Off && !args.flag("no-history") {
        report::append_history(&args.eval_dir().join("results/history.csv"), &meta, &cells)?;
    }
    for cell in &cells {
        eprintln!(
            "{}/{}: E2E {:.1}% · route {:.1}% · Hit@1 {:.1}% · ingest {:.1}% · errors {}",
            cell.router,
            cell.variant,
            cell.search.e2e.value.unwrap_or(0.0) * 100.0,
            cell.search.route_acc.value.unwrap_or(0.0) * 100.0,
            cell.search.hit1.value.unwrap_or(0.0) * 100.0,
            cell.ingest.ingest_acc.value.unwrap_or(0.0) * 100.0,
            cell.search.errors
        );
    }
    eprintln!("report: {}", out_dir.join("report.md").display());
    Ok(())
}

type Plan = Vec<(Router, Arc<Variant>, Vec<(Case, data::Resolved)>)>;

/// One real search per router before the full run, so a rejected key or a broken
/// LLM setting stops the run instead of filling it with errors.
fn preflight(
    runtime: &tokio::runtime::Runtime,
    kb: &Arc<Kb>,
    plan: &Plan,
    creds: &Credentials,
    timeout: u64,
    args: &Args,
) -> Result<(), String> {
    let mut checked = HashSet::new();
    for (router, variant, runnable) in plan {
        if !checked.insert(router.name()) {
            continue;
        }
        let Some(pick) = runnable.iter().find(|(case, resolved)| {
            case.kind == "search"
                && resolved.node.is_some()
                && resolved.root_path.is_none()
                && resolved.start_node.is_none()
        }) else {
            continue;
        };
        let settings = Arc::new(Settings {
            router: *router,
            mock: Mock::Off,
            creds: creds.clone(),
            concurrency: 1,
            timeout: Duration::from_secs(timeout),
            beam_width: None,
            limit: None,
            verify_draft_search: false,
            repeat: 1,
        });
        let records = runtime.block_on(exec::run_cell(exec::CellInput {
            kb: kb.clone(),
            variant: variant.clone(),
            settings,
            cases: vec![pick.clone()],
            label: format!("preflight {}", router.name()),
        }))?;
        let Some(out) = records.first().and_then(|record| record.search.as_ref()) else {
            continue;
        };
        if let Some(error) = &out.error {
            return Err(format!(
                "preflight ({}) failed with {}: {}",
                router.name(),
                error.code,
                error.message
            ));
        }
        if *router == Router::JevLlm && out.router == "beam" && !args.flag("allow-llm-fallback") {
            return Err(
                "preflight (jev_llm): the LLM route failed and the Jev beam answered instead. \
                 Check JEV_EVAL_LLM_BASE_URL/TOKEN/MODEL, or pass --allow-llm-fallback."
                    .into(),
            );
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// report / compare
// ---------------------------------------------------------------------------

fn cells_in(dir: &Path) -> Result<Vec<(String, String, PathBuf)>, String> {
    let mut cells = Vec::new();
    let entries =
        std::fs::read_dir(dir.join("cells")).map_err(|e| format!("{}: {e}", dir.display()))?;
    for entry in entries {
        let path = entry.map_err(|e| e.to_string())?.path();
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        if let Some((router, variant)) = stem.split_once("__") {
            cells.push((router.to_string(), variant.to_string(), path.clone()));
        }
    }
    cells.sort();
    Ok(cells)
}

fn cmd_report(args: &Args) -> Result<(), String> {
    let dir = PathBuf::from(args.positional.first().ok_or("usage: report RUN_DIR")?);
    let meta: Value = serde_json::from_str(
        &std::fs::read_to_string(dir.join("meta.json")).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let mut cells = Vec::new();
    for (router, variant, path) in cells_in(&dir)? {
        cells.push(score::summarize(&router, &variant, &read_records(&path)?));
    }
    write_json(
        &dir.join("summary.json"),
        &json!({"meta": meta, "cells": cells}),
    )?;
    std::fs::write(dir.join("report.md"), report::render(&meta, &cells))
        .map_err(|e| e.to_string())?;
    eprintln!("wrote {}", dir.join("report.md").display());
    Ok(())
}

fn pick_cell(
    dir: &Path,
    selector: Option<&str>,
    fallback: Option<&(String, String)>,
) -> Result<(String, Vec<Record>), String> {
    let cells = cells_in(dir)?;
    let chosen = match selector {
        Some(selector) => {
            let (router, variant) = selector
                .split_once('/')
                .ok_or("cell selector is router/variant")?;
            cells
                .iter()
                .find(|(r, v, _)| r == router && v == variant)
                .ok_or_else(|| format!("{}: no cell {selector}", dir.display()))?
        }
        None => fallback
            .and_then(|(router, variant)| {
                cells.iter().find(|(r, v, _)| r == router && v == variant)
            })
            .or_else(|| cells.first())
            .ok_or_else(|| format!("{}: no recorded cells", dir.display()))?,
    };
    let label = format!(
        "{}:{}/{}",
        dir.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default(),
        chosen.0,
        chosen.1
    );
    Ok((label, read_records(&chosen.2)?))
}

fn cmd_compare(args: &Args) -> Result<(), String> {
    let a_dir = PathBuf::from(
        args.positional
            .first()
            .ok_or("usage: compare RUN_A [RUN_B] [--a router/variant] [--b router/variant]")?,
    );
    let b_dir = args
        .positional
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| a_dir.clone());
    let (a_label, a) = pick_cell(&a_dir, args.get("a"), None)?;
    let a_key = a.first().map(|r| (r.router.clone(), r.variant.clone()));
    let b_selector = args.get("b");
    if b_selector.is_none() && a_dir == b_dir {
        return Err("comparing cells of one run needs --a and --b".into());
    }
    let (b_label, b) = pick_cell(&b_dir, b_selector, a_key.as_ref())?;
    write_or_print(
        args.get("out"),
        &report::compare(&a_label, &a, &b_label, &b),
    )
}

fn main() -> ExitCode {
    let args = Args::parse();
    let result = match args.command.as_str() {
        "validate" => cmd_validate(&args),
        "stats" => cmd_stats(&args),
        "export" => cmd_export(&args),
        "run" => cmd_run(&args),
        "report" => cmd_report(&args),
        "compare" => cmd_compare(&args),
        "" | "help" | "--help" | "-h" => {
            print!("{USAGE}");
            Ok(())
        }
        other => Err(format!("unknown command: {other}\n\n{USAGE}")),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
