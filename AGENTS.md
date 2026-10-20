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
```

Working directory must be the repo root: `JEV_TREE_DB`, `JEV_TREE_SEED`, and
`JEV_TREE_STATIC_DIR` default to relative paths. If `/docs` 404s or the UI looks stale,
an old binary owns the port — `lsof -iTCP:8768`, kill it, run again.

Changes to descent, ranking, or abstain must come with a test in `src/engine.rs`
using `JevClient::script_choices`. Changes to routes must update `static/openapi.json`
and `docs/api.md` in the same commit; a test asserts the path set matches the router.

## Data flow

```
POST /api/run {mode: search|ingest, root?}
  → engine::flat_route   virtual root (or forest) → every node in scope + __none__, one Jev choice
  → engine::descend      beam fallback for scopes one choice cannot name (see "Evaluator limits")
  → leaf, __stop__, or __none__
  → search: retrieve that subtree, rank with Noul/Score, assign roles
        (items ON the chosen node win near-ties against deeper ones — the walk already
        picked that level of specificity; `LEVEL_BONUS` in `src/engine.rs`)
  → ingest: upsert as draft, or publish when auto_publish is true
```

The default route is **one Jev choice** (`flat_route`):

- The menu is **every node in the scope**, so a perfect evaluator always sees the right
  node and a request may arrive in any script. `__none__` competes at a forest walk
  ("nothing fits"). A `root` scope walks the root's children *as* the forest, so it has
  `__none__` too: `leaf_id: null` there means "outside this root". A walk with a `start_node`
  (or a childless `root`) has no terminal: the start node itself is a candidate, and picking
  a parent is how the walk stays there.
- Past `FLAT_MAX_NODES` (254) nodes, or `FLAT_MAX_MENU_BYTES` (80 KB) of menu text, the menu
  falls back to lexical recall (strongest matches, a per-root quota, full ancestor chains,
  then the next best nodes until the menu is as full as one choice can read), and below
  `FLAT_MIN_EVIDENCE` the semantic beam below runs instead.
- An uncertain pick (chosen probability < 0.85) gets **one more judgment on scope alone**
  ("does this tree cover the request at all"). Only a confident `out` (p ≥ 0.8) refuses;
  a hesitant one keeps the menu choice. A `start_node` walk must land on a node and never
  asks. The step shows up as `trace.steps[1]`.

The beam fallback descends level by level:

- Candidates are the node's **direct children** plus one terminal option:
  `__none__` at the root ("nothing fits"), `__stop__` deeper ("stay here").
- The terminal option competes on score. When it wins, that beam stops and its children
  are not expanded. A beam that lands on a childless node settles the same way.
- Path score is the geometric mean of edge probabilities, so depth is not penalised.
- A Jev API key is required. Search and ingest return 400 without one.
  `GET /api/health` → `evaluator` is `jev` or `unconfigured`. That field is the ranker,
  not which router ran.
- With `llm_base_url`, `llm_token`, and `llm_model` all set, **routing** is **one LLM call
  in `src/jev.rs` naming every node in scope**. `__none__` abstains (outside the tree, or
  outside the `root`); under a `start_node` the walk stays in that subtree and must land on
  a node there. Above `ONE_SHOT_MAX_NODES` (800) nodes in scope it falls back to the
  stepped walk: the root topic, then one node in a lexical shortlist of that subtree.
  A missing setting or a failed call keeps Jev routing (flat choice first). The route gets
  a third of `JEV_TREE_RUN_TIMEOUT_SECS`, so a deadline under ~20 s cuts it off every time.
  Item **ranking** always uses Jev, over every active item in the landed subtree, in
  batches of up to 32 items and 24 KB (`RANK_BATCH`, `RANK_BATCH_BYTES`), four calls at a
  time, in order.
- The lexical heuristic that pre-0.4 runs fell back to is **gone** (0.6.1). Nothing routes
  by word overlap at runtime. Word overlap only builds candidate menus: the stepped LLM
  walk above `ONE_SHOT_MAX_NODES` (`llm_shortlist` in `src/jev.rs`) and the flat route's
  recall menu on scopes one choice cannot name (`recall_candidates` in `src/engine.rs`).

### Evaluator limits

Measured against the live API, not documented by it. Each one was a 502 on every search
before it was a constant.

- One `choice` question takes at most **255 options** (256 is HTTP 422), so a forest walk
  names at most 254 nodes plus `__none__` (`FLAT_MAX_NODES`). The LLM router has no such cap.
- A request has to fit about **32K tokens**; past it the answer is HTTP 400. That is bytes
  only loosely: a menu of 94 KB passed and 124 KB failed, a batch of real Korean items
  passed at 100 KB and failed at 116 KB, and a plain string of 48 KB passed. The engine keeps
  the flat menu under 80 KB and a ranking call under 24 KB, which holds even when every
  byte costs a token.
- A 4xx from the evaluator is never retried (only 408, 429 and 5xx are), so one oversized
  call fails the whole run. Size things before the call, not after.

Result roles: `recommended` ≥ 0.65, `alternative` ≥ 0.40, else `reference`.
A weak best item sets `abstained: true` (score < 0.45, or < 0.30 with directness < 1.0).
`leaf_id: null` means outside the tree.

## Where code goes

Ingest is two steps. The server files a draft (`status=draft`, invisible to search) and
returns its `item_id`; publishing is a separate call with `auto_publish: true`. Discard a
draft with `DELETE /api/items/{id}`.

| Change | File | Do not put it in |
|---|---|---|
| Descent, beam, ranking, abstain | `src/engine.rs` | HTTP handlers, UI |
| Jev calls, optional LLM routing | `src/jev.rs` | Anywhere else — this is the only network call |
| Routes, auth, OpenAPI serving | `src/http.rs` | `src/engine.rs` |
| SQLite schema and queries | `src/store.rs` | |
| Secret encryption, key hashing, sessions | `src/secret.rs` | |
| `AppState`, boot env seeding | `src/lib.rs` | |
| Port, paths, process | `src/main.rs` | |
| Screens, in-flight state | `static/lab.js`, `static/lab.css` | Server-side product copy |
| Machine contract | `static/openapi.json` (hand-maintained) | The server does not generate it |
| Seed content | `data/seed.json` | Core source files |

Domain and product concepts belong in `data/seed.json` or bootstrap fixtures only.
`src/engine.rs` and `src/jev.rs` stay domain-free.

## Invariants

1. The evaluator receives the **full context** — current request, prior `context` turns,
   ancestor path, and sibling descriptions. Never a bare sentence.
2. `start_node` is optional. `search` and `ingest` must work with it unset.
   `root` is optional too. Unset `root` is the whole forest. Set `root` scopes the
   same descent to that node's children. Settings and keys never take a `root`.
3. `mode` is `search` or `ingest`. Do not add `classify`.
4. Terminal choices (`__stop__`, `__none__`) must be able to win and must stop expansion.
5. A beam that cannot expand — a real leaf, or one that chose a terminal — keeps competing
   on score. Never drop it, or a deeper branch wins by default.
6. Ingest without `auto_publish` is a draft and must stay out of search results.
7. The taxonomy lives in SQLite. `AppState.nodes` is a cache that `reload_taxonomy` refreshes.
8. Never commit secrets: `data/.jev-tree.key`, `data/.jev-tree.salt`, `.env`, `data/*.db`.
9. Keep the UI out of the core and the core out of the UI.
10. `JevClient::script_choices` and `script_llm` exist for unit tests and the `eval` example's
    mock router. They are compiled into the binary but unreachable over HTTP. Keep them that way.

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
required. The LLM base URL, token, and model route search/ingest when all three are
set, and they list models. If any of them is missing, or the call fails, Jev routes
(flat choice first; the beam only when one choice cannot name the scope). Item ranking
always posts to TypeSafe System One.

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
| Low | Past 254 nodes (or 80 KB of menu text) in scope the flat Jev menu stops naming every node, and above 800 the LLM router does too: the LLM router falls back to a stepped walk over a lexical shortlist of 20 nodes plus their parents, and the flat Jev menu falls back to a recall menu (strongest matches, a per-root quota, full ancestor chains, then the next best nodes up to 254). A node whose wording shares nothing with the request can be skipped; below `FLAT_MIN_EVIDENCE` the beam runs instead, which scores every sibling. Smaller scopes pick from every node in one call. Measured on 187 golden queries with the Jev router: 94.1-94.7% exact leaf (two runs) on the full 219-node menu; on a 279-node tree the same queries score 93.6% with the filled recall menu (79.1% when the menu stopped at the strongest matches, ~47 of the 254 slots), 90.8% at 500 nodes and 89.7% at 800. A node with 255 or more direct children is still unguarded in the beam. |
| Low | Ranking loads every active row in the landed subtree and scores it with Jev in batches of up to 32 items / 24 KB, four at a time. A 2,201-item subtree is about a hundred calls. Fine at seed scale; not a plan for tens of thousands of rows under one node. |
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
