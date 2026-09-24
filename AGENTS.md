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
  → engine::descend      virtual root (or forest) → children, beam (default 3), geometric-mean score
  → leaf, __stop__, or __none__
  → search: retrieve that subtree, rank with Noul/Score, assign roles
        (items ON the chosen node win near-ties against deeper ones — the walk already
        picked that level of specificity; `LEVEL_BONUS` in `src/engine.rs`)
  → ingest: upsert as draft, or publish when auto_publish is true
```

One descent step:

- Candidates are the node's **direct children** plus one terminal option:
  `__none__` at the root ("nothing fits"), `__stop__` deeper ("stay here").
- The terminal option competes on score. When it wins, that beam stops and its children
  are not expanded. A beam that lands on a childless node settles the same way.
- Path score is the geometric mean of edge probabilities, so depth is not penalised.
- A Jev API key is required. Search and ingest return 400 without one.
  `GET /api/health` → `evaluator` is `jev` or `unconfigured`. That field is the ranker,
  not which router ran.
- With `llm_base_url`, `llm_token`, and `llm_model` all set, **routing** is **one LLM call
  in `src/jev.rs` naming every node in scope**. `__none__` abstains (outside the tree);
  under a `root`/`start_node` the walk stays in that subtree and must land on a node there.
  Above `ONE_SHOT_MAX_NODES` (800) nodes in scope it falls back to the stepped walk: the
  root topic, then one node in a lexical shortlist of that subtree.
  A missing setting or a failed call uses the Jev beam. Item **ranking** always uses Jev,
  over every active item in the landed subtree, in batches of 32.
- The lexical heuristic in `src/jev.rs` is test-only. It is not a runtime fallback.
  It never scores the terminal option's own wording, and a single child with no overlap
  abstains (`terminal_bar`).

Result roles: `recommended` ≥ 0.65, `alternative` ≥ 0.40, else `reference`.
Top score < 0.30 sets `abstained: true`. `leaf_id: null` means outside the tree.

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

## Auth

1. `GET /api/setup/status` needs no token.
2. `has_server_key: false` → **open mode**: every endpoint is reachable without a token.
3. `has_server_key: true` → `Authorization: Bearer <token>` or 401.
   Log in with `POST /api/auth/login`.
4. Integrations get their own revocable API keys from `POST /api/keys`.
   Do not hand out the login password or the Jev key.
5. Runtime secrets are owned by the database. `JEV_TREE_INIT_*` and the legacy
   `TYPESAFE_API_KEY` fill **empty slots at boot only**.

The Jev API key and the LLM token are different things, and both belong with models.
The login password and integration API keys belong with the server. The Jev key is
required. The LLM base URL, token, and model route search/ingest when all three are
set, and they list models. If any of them is missing, or the call fails, the Jev beam
runs. Item ranking always posts to TypeSafe System One.

## Deployment safety

- The default bind is `127.0.0.1`. With `JEV_TREE_BIND` set to anything else, the server
  **refuses to start in open mode** — set `JEV_TREE_INIT_SERVER_KEY` (6+ characters) or
  `JEV_TREE_ALLOW_OPEN=1`. Without that guard any caller could claim the login password through
  `POST /api/setup/server-key`, reset the seed, and read settings.
- `POST /api/seed` requires `{"confirm":"RESET"}` and deletes all categories and items.
  Never call it against a live database without explicit instruction.
- Boot also fails fast when `JEV_TREE_STATIC_DIR` has no `index.html`, and a handler panic
  returns 500 through `CatchPanicLayer` instead of dropping the connection.

## Known gaps

Verified against the current code. Do not assume the rest of this document hides these.

| Severity | Issue |
|---|---|
| Low | Above 800 nodes in scope the LLM router falls back to a stepped walk over a lexical shortlist of 20 nodes plus their parents, where a node whose wording shares nothing with the request can be skipped. Smaller scopes pick from every node in one call. A failed or unconfigured LLM call uses the Jev beam, which scores every sibling. |
| Low | Ranking loads every active row in the landed subtree and scores it with Jev in batches of 32. Fine at seed scale (largest subtree 187 items). Not a plan for tens of thousands of rows under one node. |
| Low | A degraded Jev call still makes up to three attempts (the first try and two retries) before the run deadline trips. |

## Common mistakes

1. Trusting `items[0]`. Read `role` and `abstained`.
2. Reading `evaluator: "unconfigured"` as a Jev outage. It means no key is saved,
   and `/api/run` will return 400 until one is.
3. Treating `ingest` as a save. The default is a draft.
4. Editing a route without updating `static/openapi.json` and `docs/api.md`.
5. Growing the seed without describing siblings. Siblings must be mutually exclusive,
   and each `description` should say how it differs from its siblings.
6. Adding a second runtime, a `classify` mode, or a new default port.
