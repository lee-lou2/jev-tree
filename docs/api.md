# HTTP API

Base URL `http://127.0.0.1:8768`. Everything lives under `/api`.
Machine-readable contract: `GET /api/openapi.json`. Browsable: `GET /docs`.

Behavioural caveats that this contract does not express are listed under
[Known gaps](../AGENTS.md#known-gaps).

## Virtual roots

`/` is the whole forest. `/products` makes the `products` child of the forest the
page root: the tree and table show that node's children, and search/ingest walk only
that subtree. `/products/products_stock` goes one level deeper. Each path segment must
name a **direct child** of the previous node (id first, then name): a nested node needs its
whole path, `products/products_stock`, and `products_stock` alone is a 404. The same value is
`root` on `POST /api/run` and on the read endpoints. Login, settings, and API keys are
not scoped; they belong to the process.

## Handshake

```
1. GET  /api/setup/status                          → has_server_key, jev_api_key_set, evaluator
2. if has_server_key == false: call anything without a token (open mode)
   if has_server_key == true:  POST /api/auth/login {"server_key": "..."} → {"token": "..."}
3. send Authorization: Bearer <token> on every other call
4. POST /api/run {"mode":"search","query":"..."}  (add "root":"products" to stay in a subtree)
```

`POST /api/setup/server-key {}` creates the first login password and returns it in plaintext once,
together with a session token. It fails with 400 if a password already exists. The JSON field is still `server_key`.

## Conventions

- JSON bodies require `Content-Type: application/json` and stay under 2 MB.
- Errors are always `{"code": "...", "detail": "..."}`. That includes a body that is not
  valid JSON, a field of the wrong type, a wrong content type, a body over the limit, a
  query or path value of the wrong type, and a method the path does not take.

| code | status | meaning |
|---|---|---|
| `invalid_request` | 400 | Value or format rejected; `detail` explains. Also 413 (body over 2 MB), 415 (not `application/json`), 405 (method not allowed) |
| `unauthorized` | 401 | Missing, expired, or wrong credential |
| `forbidden` | 403 | An API key reached an operator endpoint; log in for a session |
| `not_found` | 404 | Unknown id |
| `conflict` | 409 | `expected_version` did not match |
| `rate_limited` | 429 | Too many wrong passwords in a row; wait `Retry-After` seconds |
| `cancelled` | 499 | The streaming client went away mid-run |
| `internal_error` | 500 | Server fault |
| `upstream_error` | 502 | Evaluator call failed. `detail` says when the input looks too large for it |
| `timeout` | 504 | The run passed `JEV_TREE_RUN_TIMEOUT_SECS` (default 120). An LLM review gets a third of it; past that Jev's own pick stands |

- Sessions last 30 days and slide on each call. `POST /api/auth/logout` revokes one.
- API keys from `POST /api/keys` authenticate the same way but never slide.
  Use these for apps and scripts, not the login password. Revoke with `DELETE /api/keys/{id}`
  and list them with `GET /api/keys`.
- A key is for the data endpoints (`/api/run`, `/api/run/stream`, `/api/items`, `/api/tree`,
  `/api/health`, `/api/seed/stats`). Settings, key management, `POST /api/llm/models` and
  `POST /api/seed` belong to the operator and need a login session: with a key they return
  403 `forbidden`. A key that could mint keys or rotate the password would outlive its
  own revocation.
- After 5 wrong passwords in a row, `POST /api/auth/login` makes the next attempts wait
  1 s, 2 s, 4 s … up to 60 s and answers `429` with `Retry-After`, even for the right
  password. A good login clears it, and so does a quiet quarter hour. Sessions already issued
  are not affected.
- Secrets are never returned in full; `GET /api/settings` shows `****` plus a 4-character hint.

## `POST /api/run`

The only endpoint most integrations need.

| field | type | default | notes |
|---|---|---|---|
| `mode` | `"search"` \| `"ingest"` | `"search"` | Anything else is 400 |
| `query` | string | `""` | Search text. Falls back to `source` |
| `question` / `answer` | string | `""` | Ingest payload. Falls back to parsing `source` |
| `context` | array | `[]` | Prior turns, max 32. Strings or `{"role","text"}` objects |
| `start_node` | string \| null | `null` | Optional forced start; unknown id is 400. Must sit under `root` when both are set |
| `root` | string \| null | `null` | Virtual tree: slash-separated child keys (`products`, `products/products_stock`). Each segment is a **direct child** of the previous node (forest roots at the first step), matched by id then name. Unknown path is 404. Settings and keys ignore this |
| `beam_width` | int | `3` | 1–5. Accepted and ignored: the level-by-level beam is gone (0.10) |
| `limit` | int | `8` | 1–20 ranked items |
| `auto_publish` | bool | `false` | Ingest only. `false` stores a draft. Re-sending the question of a *published* row without it turns that row back into a draft: it leaves search until it is published again |
| `item_id` + `expected_version` | string + int | — | Target an existing row; supply both or neither. The row keeps its category (no routing call: `trace.router` is `flat` with no `steps`) unless `start_node` is also given. If its category is gone or outside `root`, the request routes afresh |

Combined text must be 1–8000 characters.

### `mode: "search"`

```jsonc
{
  "query": "my parcel is stuck at customs",
  "leaf_id": "orders_tracking",          // null = outside the tree, items is empty
  "abstained": false,                    // true = the best item is weak (score < 0.45, or < 0.30 with directness < 1.0), answer "I don't know"
  "path": [{"id": "orders", "name": "Orders", "probability": 0.47, "confidence": 0.6}],
  "items": [{
    "item": {"id": "1", "category_id": "orders_tracking", "question": "...", "answer": "...",
             "kind": "qa", "status": "active", "version": 1},
    "relevance": 0.68,                   // Noul: could this be sent as a reply
    "directness": 1.36,                  // Score 0..2: how directly it answers
    "role": "recommended",               // >= 0.65 · alternative >= 0.40 · else reference
    "score": 0.681,                      // relevance * 0.7 + (directness / 2) * 0.3
    "confidence": null
  }],
  "trace": {
    "run_id": "...", "mode": "search", "leaf_id": "orders_tracking", "score": 0.54,
    "steps": [{"depth": 0, "node_id": null, "node_name": "the knowledge root",
               "candidates": [{"id": "orders", "name": "Orders", "probability": 0.47},
                              {"id": null, "name": "no matching topic", "probability": 0.06}],
               "choice_id": "orders", "choice_name": "Orders",
               "confidence": 0.6, "path": [], "reason": "search: ..."}],
    "final_beams": [{"node_id": "orders_tracking", "path": [], "score": 0.54, "alive": true}]
  }
}
```

Pick the answer by `role`, not by position. When `abstained` is true every item is demoted
to `reference` — do not answer from them. A `candidate` with `id: null` is the terminal
choice (`__none__`).

Jev scores every active item in the landed subtree, then `limit` is applied. When the route
was unsure of its pick (chosen probability below 0.9) the pool also holds the items of the
two next-best nodes — never the pick's own ancestors or descendants, and only a runner-up's
own items when its subtree holds more than 64 — so an item's `category_id` can differ from
`leaf_id`. The routed node's own items win near-ties. A call carries up to 32 items and 24 KB
of request, four calls run at a time, and text over 8,000 characters (more than the API
accepts, so only a row put in the database another way) is cut for the evaluator only (the
result carries the stored text). `trace.router` says who picked the node:

| `router` | who picked the node | what the trace carries |
|---|---|---|
| `flat` (default) | Jev: one choice over the walk's menu, `__none__` included | the choice step's `candidates` carry probabilities and `trace.score` is the chosen probability; per-edge `probability` is null |
| `llm` | the optional model, reviewing a Jev pick below 0.8 | an `llm review` step listing the menu it read (with Jev's probabilities); `trace.score` is null |

`trace.steps` reads in order:

| `node_name` | when | what it says |
|---|---|---|
| `coarse map` | the scope is too big for one choice | Jev's pick over the deepest tree level one choice can name; `reason` says how many branches it opened |
| the walk's start (`the knowledge root` in a forest) | always | the choice over the menu, with probabilities |
| `llm review` | a pick below 0.8 and an LLM configured | the model's pick from the same menu |
| `scope check` | a pick below 0.85 that no LLM reviewed | whether the tree covers the request at all; only a confident `out` holds it |

A pick whose subtree did not fit the menu repeats the coarse map and choice inside that
subtree, so the steps can show two rounds.

A search asked again with the same text, `context`, `root`, `start_node`, and `limit` is
answered from a per-process cache for up to 10 minutes, unless an item, the taxonomy, or the
settings changed in between (any of those invalidates every cached result). The cached
answer is the earlier one, `trace.run_id` included.

The menu is **every node in the walk** whenever one choice can name them (up to 254 nodes,
Jev takes at most 255 options with `__none__`, and 80 KB of menu text), so the evaluator
judges meaning even when the request shares no characters with the taxonomy. A walk with
`start_node` picks inside that subtree only and has no `__none__`. The forest walk can
abstain, and so can a `root` scope: it walks the root's children as the forest, so
`leaf_id: null` there means "outside this root". A bigger scope builds the menu from two
sources: a **coarse map** (one Jev choice over the deepest tree level that still fits, whose
best branches are opened whole) and **BM25** over each node's path, description, examples,
and the questions filed under it. Both stay within what one choice can read.

### `mode: "ingest"`

```jsonc
// step 1 — draft (default): stored with status=draft, invisible to search
{"mode": "ingest", "question": "...", "answer": "..."}
// → {"category_id": "...", "path": [...], "duplicate_ids": ["528"],
//    "item_id": "1313", "version": 1, "published_id": null, "updated": false}

// step 2 — publish the reviewed draft
{"mode": "ingest", "question": "...", "answer": "...",
 "item_id": "1313", "expected_version": 1, "auto_publish": true}
// → {"published_id": "1313", "updated": true}

// discard instead
// DELETE /api/items/1313
```

- `item_id` addresses the stored row either way. Passing it back with `expected_version`
  gives optimistic locking; a stale version returns 409.
- Omitting `item_id` matches on question + category, so re-sending the same question
  updates the existing row rather than adding one.
- `duplicate_ids` lists same-category items whose question is a near match (character
  trigram overlap ≥ 0.6), capped at five.
- When the descent ends outside the tree the call returns 400 and nothing is written.

### `POST /api/run/stream`

Same request body, `text/event-stream` response, one JSON object per `data:` line.

| `type` | payload |
|---|---|
| `descent_step` (`status: asking`) | depth, frontiers with candidate children |
| `descent_step` (`status: answered`) | the step that chose |
| `retrieval_completed` | `count`, `category_id` |
| `candidate_evaluated` | `item_id`, `score`, `role` |
| `descent_done` | `leaf_id`, `path`, `score`, `trace` |
| `usage` | one model call: `source` (`jev` or `llm`), `input_tokens`, `output_tokens` (`llm` adds `calls`) |
| `cache_hit` | the search was answered from the result cache; `descent_done` and `search_done` follow, no model call |
| `search_done` | full search result |
| `ingest_draft` | `item` (includes the draft id and version), `duplicate_ids`, `path`, `trace` |
| `ingest_saved` | `item`, `updated`, `path`, `trace` |
| `error` | `code`, `message` |

Terminal events are `search_done`, `ingest_saved`, and `ingest_draft`. Do not close earlier.

## Reading endpoints

| Endpoint | Returns |
|---|---|
| `GET /api/health` | `{evaluator, categories, items, runtime}` — `evaluator` is `jev` or `unconfigured` |
| `GET /api/seed/stats` | `{nodes, items, max_depth, runtime}` |
| `GET /api/tree` | `{tree, root, path}` — `tree` is the forest, or the chosen node's children when `root` is set |
| `GET /api/items` | `{items, total, offset, limit, scope}` |
| `GET /api/site` | `{site_name, site_description, site_logo}` (no token) |

`GET /api/health`, `GET /api/seed/stats`, `GET /api/tree`, and `GET /api/items` accept
`root` (`products`, `products/products_stock`, …). The UI uses the same path as the
first URL segments (`/products`, `/products/products_stock`). Settings, login, and
API keys stay project-wide and do not change with `root`.

`GET /api/items` also accepts `q` (matched literally against question and answer; `%` and
`_` are ordinary characters), `category_id`, `offset`, `limit` (1–100, default 50, values
outside the range are clamped),
`scope` (`exact` for the category itself, `subtree` for all descendants), and `status`
(`active`, `draft`, or `all`; default `active`). Search only ever reads `active` rows.
A `category_id` outside the current `root` is 400. Without `category_id`, `root` lists
that subtree.

`DELETE /api/items/{id}` archives an item. Use it to discard a draft or retire a published
answer; archived rows leave search, listings, and upsert matching.

## Settings and keys

Project-wide. `root` / URL path never scopes them.

Settings and keys need a login session; an API key gets 403.

`PATCH /api/settings` takes any subset. Secrets are encrypted at rest and applied without a restart.

| field | rule |
|---|---|
| `site_name` / `site_description` | ≤ 60 / ≤ 200 characters |
| `site_logo` | PNG/JPEG/SVG/WebP data URL ≤ 500 KB, or `null` to clear |
| `jev_api_key` | Required. Search, ingest, and ranking use it. `""` clears it; runs then return 400 and `evaluator` becomes `unconfigured` |
| `llm_base_url` / `llm_token` / `llm_model` | Optional. When all three are set, the model **reviews** every route Jev is unsure of (chosen probability below 0.8): it reads the same menu Jev chose from and picks the node, or `__none__` where the walk may abstain. Confident picks never wait for it. Clearing any of the three, or a failed or slow call, keeps Jev's pick. Item ranking always uses Jev. The same credentials list models. |
| `server_key` | Login password, ≥ 6 characters; `""` turns login off (open mode). Stored as an Argon2id hash; a hash from before 0.9 is replaced at the next login |

In the UI, Jev and LLM settings live under **Settings → Models**. The login password and
integration API keys live under **Settings → Server**. Changing the page root does not
create a second settings store.

`POST /api/llm/models` probes `{base}/v1/models` then `{base}/models` with the supplied or
stored LLM credentials and saves nothing. The Jev key is never used for this.
A review calls `{base}/chat/completions` (or `{base}/v1/chat/completions`) with the saved model.

## `POST /api/seed` — destructive

Requires `{"confirm":"RESET"}`. Deletes every category and item, re-reads `JEV_TREE_SEED`,
and reloads the in-memory tree without a restart. Never call it against a live database.
