# AGENTS.md — jev-tree

Read this before changing anything. It is the only document you need to decide
*what* to fix, *where* to put it, and *what must not break*.

HTTP details live in [`docs/api.md`](docs/api.md) and [`static/openapi.json`](static/openapi.json).

## What this is

A hierarchical knowledge service. One Rust process serves the HTTP API and the web UI.
There are two operations, and they are the same descent:

- `search` — walk the tree with the full context, then rank items under the selected node.
- `ingest` — walk the same tree, then draft or publish a Q&A there.

There is no `classify` mode; `search` already returns `leaf_id` and `path`.
There is no Python. The port is 8768.

The UI path `/products` (and `/products/products_stock`, …) is a **virtual root**.
Each segment must be a direct child of the previous node. Search and ingest then walk
only that subtree. Settings, login, and API keys stay project-wide.

## Run and verify

```bash
cargo run --release                              # repo root, http://127.0.0.1:8768
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt
# harness self-check, no key, no cost: scripted evaluator must route and file 100%
cargo run --release --example eval -- run --mock oracle --router both --variant all
cargo run --release --example eval -- run --eval-dir eval/large --variant full --mock oracle --router both --sample 300
```

Two datasets: `eval/` (golden, 219 nodes — fits one Jev choice) and `eval/large` (1,902 nodes,
synthetic). A routing change has to be measured on both: the golden tree never exercises
the coarse map, the recall menu, or the zoom.

Working directory must be the repo root: `JEV_TREE_DB`, `JEV_TREE_SEED`, and
`JEV_TREE_STATIC_DIR` default to relative paths. If `/docs` 404s or the UI looks stale,
an old binary owns the port — `lsof -iTCP:8768`, kill it, run again.

Changes to descent, ranking, or abstain must come with a test in `src/engine.rs`
using `JevClient::script_choices`. Changes to routes must update `static/openapi.json`
and `docs/api.md` in the same commit; a test asserts the path set matches the router.

## Data flow

```
POST /api/run {mode: search|ingest, root?}
  → search asked before against the same store and settings generation → cached result
  → engine::flat_route   the walk's menu + __none__, one Jev choice
        menu = every node in scope when one choice can name them (≤ 254 nodes, ≤ 80 KB)
             = else engine::recall_menu: a coarse Jev choice over the deepest tree level
               that fits, its best branches opened whole, plus BM25 matches (src/recall.rs)
  → pick < 0.8 and an LLM configured → engine::llm_review: the LLM picks from the same menu
    pick < 0.85 and no review        → one scope judgment, which can hold the request
    pick whose subtree was cut from the menu → the walk repeats inside that pick
  → leaf or __none__
  → search: retrieve that subtree (+ the two runners-up when the pick was < 0.9),
        rank with Noul/Score, assign roles
        (items ON the chosen node win near-ties against the rest — the walk already
        picked that level of specificity; `LEVEL_BONUS` in `src/engine.rs`)
  → ingest: upsert as draft, or publish when auto_publish is true
```

The route is **one Jev choice** (`flat_route`):

- The menu is **every node in the scope** whenever one choice can name them, so a perfect
  evaluator always sees the right node and a request may arrive in any script. `__none__`
  competes at a forest walk ("nothing fits"). A `root` scope walks the root's children *as*
  the forest, so it has `__none__` too: `leaf_id: null` there means "outside this root".
  A walk with a `start_node` (or a childless `root`) has no terminal: the start node itself
  is a candidate, and picking a parent is how the walk stays there.
- Past `FLAT_MAX_NODES` (254) nodes, or `FLAT_MAX_MENU_BYTES` (80 KB) of menu text, the menu
  is **recalled** (`recall_menu`) from two sources, both bounded by what one choice can read:
  - the **coarse map** — one Jev choice over the deepest level of the walk's tree (plus any
    shallower leaves) that one choice can name. Its best nodes are opened whole: until they
    cover `COARSE_MASS` (0.95) of the probability, at most `COARSE_BRANCHES` (8), none below
    `COARSE_FLOOR` (0.02) but the first. It reads meaning, so a request with no word in
    common with the taxonomy (another script, a paraphrase) still reaches its branch. When
    even the first branching level is too wide, it is split into chunks asked as separate
    questions, each with its own `__none__`, in as few requests as the per-request window
    allows, all at once;
  - **BM25** (`src/recall.rs`) over each node's path, description, examples and the
    questions of its active items — `RECALL_LEXICAL` (30) matches. The index is cached in
    `AppState.recall` and rebuilt when `Store::generation` moves.
  Opened branches go in shallow levels first and leave room for the matches; every node
  brings its ancestors; the menu is in tree order. A pick whose subtree did not fit the
  menu has only been seen from above, so the walk repeats inside it (the pick stays a
  candidate) — one level deeper each round, so it ends.
- With `llm_base_url`, `llm_token`, and `llm_model` all set, a pick below
  `LLM_REVIEW_BELOW` (0.8) is **reviewed by the LLM** (`llm_review` → `JevClient::review`):
  the one-shot prompt over the **same menu, in tree order**. Not Jev's top candidates: shown
  only those, best first, the model agrees with Jev's mistakes (93% route accuracy on the
  golden queries against 97% with the whole menu). Confident picks never wait for it. The
  review gets a third of `JEV_TREE_RUN_TIMEOUT_SECS`; a failed or slow one keeps Jev's pick.
  `trace.router` is `llm` when the review decided.
- A pick below 0.85 that no LLM reviewed gets **one more judgment on scope alone** ("does
  this tree cover the request at all"). Only a confident `out` (p ≥ 0.8) refuses; a hesitant
  one keeps the pick. A `start_node` walk must land on a node and never asks.
- **Search rescues an unsure route** (`RESCUE_BELOW`, 0.9): the pool also holds the items
  of the two next-best nodes of Jev's choice (`RESCUE_NODES`), never the pick's own
  ancestors or descendants, and only a runner-up's own items when its subtree holds more
  than `RESCUE_MAX_ITEMS` (64). Most misses on a big tree are sideways (a sibling, another
  branch, another domain); without the runners-up the answer is outside the pool however
  well the ranking reads.
- A Jev API key is required. Search and ingest return 400 without one.
  `GET /api/health` → `evaluator` is `jev` or `unconfigured`. That field is the ranker,
  not which router ran.
- Item **ranking** always uses Jev, over every active item in the pool, in batches of up to
  32 items and 24 KB (`RANK_BATCH`, `RANK_BATCH_BYTES`), four calls at a time, in order.
- Nothing routes by word overlap. BM25 only proposes menu entries; Jev (or the LLM) makes
  every decision. The level-by-level beam is gone (0.10): it lost a leaf as soon as one
  level guessed wrong, and the coarse map made it unreachable. `beam_width` is accepted
  and ignored.
- Every model call leaves a `usage` event on the run's stream (`source`, `input_tokens`,
  `output_tokens`); the eval runner reads its cost from them.
- A search is cached (`AppState.results`, 1,024 results, 10 minutes) under its text,
  context, `root`, `start_node`, `limit`, `Store::generation` and `JevClient::generation`.
  Any write to the taxonomy or an item, and any settings change, moves a generation, so a
  stale result never comes back. A hit emits `cache_hit` and makes no model call. The eval
  runner turns the cache off: every case is measured.

### Evaluator limits

Measured against the live API, not documented by it. Each one was a 502 on every search
before it was a constant.

- One `choice` question takes at most **255 options** (256 is HTTP 422), so a forest walk
  names at most 254 nodes plus `__none__` (`FLAT_MAX_NODES`). The LLM has no such cap.
- One question (its options plus the shared state) has to fit about **32K tokens**; past it
  the answer is HTTP 400 `max_tokens_exceeded`. A 254-option menu of 78 KB was 28.8K tokens
  and passed; 115 KB failed. That is bytes only loosely (Korean runs about 2.7 bytes a
  token): a batch of real Korean items passed at 100 KB and failed at 116 KB. The engine
  keeps a menu under 80 KB and a ranking call under 24 KB.
- One **request** has to fit about **64K tokens** across all its questions: three 21K-token
  questions pass, four fail. Questions share the request's state (billed once), so many
  small questions are cheap — 1,000 yes/no questions in one request took 0.37 s — but big
  menus go in separate requests (`REQUEST_MAX_BYTES`, 160 KB).
- A 4xx from the evaluator is never retried (only 408, 429 and 5xx are), so one oversized
  call fails the whole run. Size things before the call, not after.
- Only input tokens are billed. A call takes 0.2–0.5 s whatever its size.

Result roles: `recommended` ≥ 0.65, `alternative` ≥ 0.40, else `reference`.
A weak best item sets `abstained: true` (score < 0.45, or < 0.30 with directness < 1.0).
`leaf_id: null` means outside the tree.

## Where code goes

Ingest is two steps. The server files a draft (`status=draft`, invisible to search) and
returns its `item_id`; publishing is a separate call with `auto_publish: true`. Discard a
draft with `DELETE /api/items/{id}`.

| Change | File | Do not put it in |
|---|---|---|
| Descent, coarse map, LLM review, rescue, ranking, abstain | `src/engine.rs` | HTTP handlers, UI |
| BM25 recall index over the taxonomy | `src/recall.rs` | `src/jev.rs` (no network here) |
| Search result cache | `src/cache.rs` | |
| Jev calls, optional LLM review | `src/jev.rs` | Anywhere else — this is the only network call |
| Routes, auth, OpenAPI serving | `src/http.rs` | `src/engine.rs` |
| SQLite schema and queries | `src/store.rs` | |
| Secret encryption, key hashing, sessions | `src/secret.rs` | |
| `AppState`, boot env seeding | `src/lib.rs` | |
| Port, paths, process | `src/main.rs` | |
| Screens, in-flight state | `static/lab.js`, `static/lab.css` | Server-side product copy |
| Machine contract | `static/openapi.json` (hand-maintained) | The server does not generate it |
| Seed content | `data/seed.json` | Core source files |

Domain and product concepts belong in `data/seed.json` or bootstrap fixtures only.
`src/engine.rs`, `src/recall.rs` and `src/jev.rs` stay domain-free.

## Invariants

1. The evaluator receives the **full context** — current request, prior `context` turns,
   ancestor path, and sibling descriptions. Never a bare sentence.
2. `start_node` is optional. `search` and `ingest` must work with it unset.
   `root` is optional too. Unset `root` is the whole forest. Set `root` scopes the
   same descent to that node's children. Settings and keys never take a `root`.
3. `mode` is `search` or `ingest`. Do not add `classify`.
4. `__none__` must be able to win wherever a walk may abstain (a forest or `root` walk): in
   the menu, in each chunk of a split coarse map, and in an LLM review.
5. A recalled menu brings every node's ancestors along, so a walk can always stop at a
   parent; a pick whose subtree was cut from the menu is walked into, never dropped.
6. Ingest without `auto_publish` is a draft and must stay out of search results.
7. The taxonomy lives in SQLite. `AppState.nodes` is a cache that `reload_taxonomy` refreshes.
8. Never commit secrets: `data/.jev-tree.key`, `data/.jev-tree.salt`, `.env`, `data/*.db`.
9. Keep the UI out of the core and the core out of the UI.
10. `JevClient::script_choices` and `script_llm` exist for unit tests and the `eval` example's
    mock router (a `~id` script entry picks `id` at 0.5, to reach the paths an unsure route
    takes). They are compiled into the binary but unreachable over HTTP. Keep them that way.

## Auth

1. `GET /api/setup/status` needs no token.
2. `has_server_key: false` → **open mode**: every endpoint is reachable without a token.
3. `has_server_key: true` → `Authorization: Bearer <token>` or 401.
   Log in with `POST /api/auth/login`. Five wrong passwords in a row start a backoff
   (1 s doubling to 60 s, `429` + `Retry-After`, in memory, `LoginGuard` in `src/store.rs`).
   The password is stored as an Argon2id hash (`v2:`); an older SHA-256 hash (`v1:`) is
   replaced at the next good login.
4. Integrations get their own revocable API keys from `POST /api/keys`.
   Do not hand out the login password or the Jev key. A key reaches the data endpoints
   only; settings, `/api/keys`, `/api/llm/models` and `/api/seed` need a login session and
   answer a key with 403 (`require_admin` in `src/http.rs`). A key that could mint keys
   would outlive its own revocation.
5. Runtime secrets are owned by the database. `JEV_TREE_INIT_*` and the legacy
   `TYPESAFE_API_KEY` fill **empty slots at boot only**.

The Jev API key and the LLM token are different things, and both belong with models.
The login password and integration API keys belong with the server. The Jev key is
required. The LLM base URL, token, and model review Jev's unsure picks when all three are
set, and they list models. If any of them is missing, or the call fails, Jev's pick
stands. Item ranking always posts to TypeSafe System One.

## Deployment safety

- The default bind is `127.0.0.1`. With `JEV_TREE_BIND` set to anything else, the server
  **refuses to start in open mode** — set `JEV_TREE_INIT_SERVER_KEY` (6+ characters) or
  `JEV_TREE_ALLOW_OPEN=1`. Without that guard any caller could claim the login password through
  `POST /api/setup/server-key`, reset the seed, and read settings.
- `POST /api/seed` requires `{"confirm":"RESET"}` and deletes all categories and items.
  Never call it against a live database without explicit instruction.
- Boot also fails fast when `JEV_TREE_STATIC_DIR` has no `index.html`, and a handler panic
  returns a JSON 500 through `CatchPanicLayer` instead of dropping the connection. Every
  `/api` error, including extractor failures, comes back as `{code, detail}`.
- Security-relevant changes (first-run password, login, settings, key issue/revoke, seed
  reset) leave a log line. Names and ids only, never values.

## Known gaps

Verified against the current code. Do not assume the rest of this document hides these.

| Severity | Issue |
|---|---|
| Medium | On the 1,902-node benchmark (`eval/large`) the right node is in the recalled menu for 98% of requests, but the final choice picks it for only 69% (Jev alone); most misses are sideways between nodes whose scopes overlap. Rescue recovers much of the answer (E2E 82%), the LLM review more. The benchmark's own labels are strict and its domains overlap by design, so part of that gap is label noise — but not all of it. |
| Low | The coarse map costs one more Jev call (about 19K input tokens on the benchmark) for every request on a scope past 254 nodes, and a pick whose branch was cut repeats it inside that branch. Not measured beyond 1,902 nodes. |
| Low | On a big tree Jev is unsure more often: the LLM reviewed 43% of requests at 1,902 nodes against 12% on the golden tree, each review about 6.5K tokens and a few seconds. |
| Low | The search result cache (`src/cache.rs`, 1,024 results for 10 minutes) is per process. Several processes behind a balancer each keep their own. |
| Low | Ranking loads every active row in the landed subtree (plus up to 64 rows from each of two runners-up) and scores it with Jev in batches of up to 32 items / 24 KB, four at a time. A 2,201-item subtree is about a hundred calls. Fine at seed scale; not a plan for tens of thousands of rows under one node. |
| Low | A degraded Jev call still makes up to three attempts (the first try and two retries) before the run deadline trips. |
| Low | `context` (32 turns, no per-turn limit) is sent twice per call. A very long one can push a request past the evaluator's window and the run ends 502 with a "shorten the input" message. |
| Low | Re-sending the question of a *published* row without `auto_publish` turns it back into a draft, so it leaves search until it is published again. The golden set counts this as `live_demoted` and expects it; a separate pending revision would need a schema change. |
| Low | The login backoff is global, not per client: someone who keeps guessing can keep the door mostly shut. A session that already exists is unaffected. |

## Common mistakes

1. Trusting `items[0]`. Read `role` and `abstained`.
2. Reading `evaluator: "unconfigured"` as a Jev outage. It means no key is saved,
   and `/api/run` will return 400 until one is.
3. Treating `ingest` as a save. The default is a draft. Passing `item_id` files the row where
   it already is: to move it, pass `start_node`.
4. Editing a route without updating `static/openapi.json` and `docs/api.md`.
5. Growing the seed without describing siblings. Siblings must be mutually exclusive,
   and each `description` should say how it differs from its siblings.
6. Adding a second runtime, a `classify` mode, or a new default port.
