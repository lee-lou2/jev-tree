# HTTP API

Base URL `http://127.0.0.1:8768`. Everything lives under `/api`.
Machine-readable contract: `GET /api/openapi.json`. Browsable: `GET /docs`.

Behavioural caveats that this contract does not express are listed under
[Known gaps](../AGENTS.md#known-gaps).

## Handshake

```
1. GET  /api/setup/status                          → has_server_key, jev_api_key_set, evaluator
2. if has_server_key == false: call anything without a token (open mode)
   if has_server_key == true:  POST /api/auth/login {"server_key": "..."} → {"token": "..."}
3. send Authorization: Bearer <token> on every other call
4. POST /api/run {"mode":"search","query":"..."}
```

`POST /api/setup/server-key {}` creates the first login password and returns it in plaintext once,
together with a session token. It fails with 400 if a password already exists. The JSON field is still `server_key`.

## Conventions

- JSON bodies require `Content-Type: application/json`.
- Errors are always `{"code": "...", "detail": "..."}`.

| code | status | meaning |
|---|---|---|
| `invalid_request` | 400 | Value or format rejected; `detail` explains |
| `unauthorized` | 401 | Missing, expired, or wrong credential |
| `not_found` | 404 | Unknown id |
| `conflict` | 409 | `expected_version` did not match |
| `cancelled` | 499 | The streaming client went away mid-run |
| `internal_error` | 500 | Server fault |
| `upstream_error` | 502 | Evaluator call failed |
| `timeout` | 504 | The run passed `JEV_TREE_RUN_TIMEOUT_SECS` (default 120) |

- Sessions last 30 days and slide on each call. `POST /api/auth/logout` revokes one.
- API keys from `POST /api/keys` authenticate the same way but never slide.
  Use these for apps and scripts, not the login password. Revoke with `DELETE /api/keys/{id}`.
- Secrets are never returned in full; `GET /api/settings` shows `****` plus a 4-character hint.

## `POST /api/run`

The only endpoint most integrations need.

| field | type | default | notes |
|---|---|---|---|
| `mode` | `"search"` \| `"ingest"` | `"search"` | Anything else is 400 |
| `query` | string | `""` | Search text. Falls back to `source` |
| `question` / `answer` | string | `""` | Ingest payload. Falls back to parsing `source` |
| `context` | array | `[]` | Prior turns, max 32. Strings or `{"role","text"}` objects |
| `start_node` | string \| null | `null` | Optional forced start; unknown id is 400 |
| `beam_width` | int | `3` | 1–5 |
| `limit` | int | `8` | 1–20 ranked items |
| `auto_publish` | bool | `false` | Ingest only. `false` stores a draft |
| `item_id` + `expected_version` | string + int | — | Target an existing row; supply both or neither |

Combined text must be 1–8000 characters.

### `mode: "search"`

```jsonc
{
  "query": "my parcel is stuck at customs",
  "leaf_id": "orders_tracking",          // null = outside the tree, items is empty
  "abstained": false,                    // true = top score < 0.30, answer "I don't know"
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
choice (`__stop__` / `__none__`).

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
| `descent_step` (`status: answered`) | chosen nodes, surviving beams |
| `retrieval_completed` | `count`, `category_id` |
| `candidate_evaluated` | `item_id`, `score`, `role` |
| `descent_done` | `leaf_id`, `path`, `score`, `trace` |
| `search_done` | full search result |
| `ingest_draft` | `item` (includes the draft id and version), `duplicate_ids`, `path`, `trace` |
| `ingest_saved` | `item`, `updated`, `path`, `trace` |
| `error` | `code`, `message` |

Terminal events are `search_done`, `ingest_saved`, and `ingest_draft`. Do not close earlier.

## Reading endpoints

| Endpoint | Returns |
|---|---|
| `GET /api/health` | `{evaluator, categories, items, runtime}` |
| `GET /api/seed/stats` | `{nodes, items, max_depth, runtime}` |
| `GET /api/tree` | `{tree: [{id, name, description, examples, parent_id, item_count, children}]}` — cache it |
| `GET /api/items` | `{items, total, offset, limit, scope}` |
| `GET /api/site` | `{site_name, site_description, site_logo}` (no token) |

`GET /api/items` accepts `q`, `category_id`, `offset`, `limit` (1–100, default 50),
`scope` (`exact` for the category itself, `subtree` for all descendants), and `status`
(`active`, `draft`, or `all`; default `active`). Search only ever reads `active` rows.

`DELETE /api/items/{id}` archives an item. Use it to discard a draft or retire a published
answer; archived rows leave search, listings, and upsert matching.

## Settings and keys

`PATCH /api/settings` takes any subset. Secrets are encrypted at rest and applied without a restart.

| field | rule |
|---|---|
| `site_name` / `site_description` | ≤ 60 / ≤ 200 characters |
| `site_logo` | PNG/JPEG/SVG/WebP data URL ≤ 500 KB, or `null` to clear |
| `jev_api_key` | Search/routing model key. `""` clears it and falls back to the heuristic |
| `llm_base_url` / `llm_token` / `llm_model` | LLM listing only; never used for search |
| `server_key` | Login password, ≥ 6 characters; `""` turns login off (open mode) |

In the UI, Jev and LLM settings live under **Settings → Models**. The login password and
integration API keys live under **Settings → Server**.

`POST /api/llm/models` probes `{base}/v1/models` then `{base}/models` with the supplied or
stored LLM credentials and saves nothing. The Jev key is never used for this.

## `POST /api/seed` — destructive

Requires `{"confirm":"RESET"}`. Deletes every category and item, re-reads `JEV_TREE_SEED`,
and reloads the in-memory tree without a restart. Never call it against a live database.
