# Changelog

## 0.6.0

### Changed

- **Stopping at a node now prefers that node's own items.** A descent that stops at `X` has
  already decided `X` is the right level of specificity, so `X`'s items win near-ties against
  items deeper in the subtree (score bonus `LEVEL_BONUS`, only when deeper candidates are
  actually in the pool). Jev's scores routinely put a deep item 0.015-0.021 above the node's
  overview item, which was enough to lose the answer to a vague question.

### Fixed

- A vague question that correctly stops at a topic node used to be answered by a more specific
  item from below it instead of that node's overview. This is the gap the golden set measured
  as `internal` targets: 56.5% (Jev beam) / 73.9% (LLM routing).

## 0.5.0

### Changed

- **LLM routing is one call over every node in scope**, not "the root topic, then a lexical
  shortlist of 20 nodes below it". Two calls compounded errors — a wrong root was
  unrecoverable — and the shortlist could drop a node whose wording shares nothing with the
  request. One call removes both and halves the round trips. `__none__` now competes with
  every node, so an out-of-scope request can abstain at any depth instead of only at the
  root. Above `ONE_SHOT_MAX_NODES` (800) nodes in scope the previous stepped walk runs.

### Fixed

- An out-of-scope request that merely looked like a covered topic ("cancel my YouTube
  Premium subscription") used to be filed into the tree, because abstaining was only on
  offer at the root choice.
- A request in a different script from the node descriptions ("Someone is clutching their
  throat and cannot breathe" against `목에 걸려 숨을 못 쉴 때`) used to land one or more
  levels too high: no shared tokens meant the lexical shortlist ranked the real target out.

On the golden set's routing-blind-spot cases (12) this moves routing accuracy from 50.0%
to 83.3% and halves the time to an answer (p50 39.2s -> 12.5s). Details in `eval/`.

## 0.4.1

### Changed

- Category choice stays with the configured LLM, and item ranking stays with Jev.
  A missing LLM setting or a failed call still walks the tree with Jev.
- An empty LLM reply is read from a reasoning field when that is where the text
  went, then asked once more without reasoning mode. If it is still empty, the
  Jev beam runs.

## 0.4.0

### Changed

- A Jev API key is required. Search and ingest return 400 `invalid_request` without
  one. `GET /api/health` reports `evaluator: "unconfigured"` instead of `"heuristic"`.
  The lexical heuristic remains in unit tests only.
- The optional LLM router is unchanged in role: all three settings route the category,
  and a missing setting or a failed call uses the Jev beam. Ranking never uses the LLM.
- Search scores every active item in the landed subtree with Jev, in batches of 32,
  then applies `limit`. It no longer keeps a lexical shortlist of size `limit`.
- An LLM trace no longer writes probability `1.0` and confidence `0.9`. `probability`
  and `trace.score` are null, and `trace.router` is `llm` or `beam`.
- Removed the unused `upsert` field from the OpenAPI request bodies.

### Fixed

- A single-child heuristic step abstains when that child has no lexical overlap.
  This path is not used at runtime.

## 0.3.0

### Added

- **LLM routing.** When Settings → Models has an LLM base URL, token, and model,
  search and ingest choose the category with that model (root, then a shortlist
  inside the subtree) instead of the Jev/heuristic beam. Out-of-tree requests abstain.
  Item ranking is unchanged. Clearing the token or model restores the beam.

- **Virtual roots.** `/products` (then `/products/products_stock`, …) makes that
  child the page root. The tree and table show its children; search and ingest walk
  only that subtree. The same value is `root` on `POST /api/run` and on
  `GET /api/tree`, `/api/items`, `/api/health`, `/api/seed/stats`. Each path segment
  must be a **direct child** of the previous node. Unknown paths return 404.
  Settings, login, and API keys stay project-wide.

## 0.2.5

### Changed

- README shows a demo video (`static/demo.mp4`). The GIF is removed.

## 0.2.4

### Changed

- Public UI copy, API error `detail` strings, and default chrome are English.
  Taxonomy content (`data/seed.json` and fixtures) is unchanged.
- `GET /api/seed/stats` no longer reports a hardcoded `ko-counseling-v2` version
  or a `synthetic` flag. It returns `{nodes, items, max_depth, runtime}`.
- Login secret is labelled **password** in the UI and docs; integration credentials
  are **API keys**. JSON still uses `server_key` / `has_server_key`.
- Custom login passwords may be 6 characters (was 20). Generated values are unchanged.
- Settings tabs are Site / Models / Server. The Jev API key sits with the LLM settings
  under Models; login password and integration API keys sit under Server.
- Login modal is compact; Enter in the password field submits. Favicon is a three-branch
  tree with one selected child.

## 0.2.3

### Fixed

- **The heuristic evaluator could not say "nothing fits".** It scored the terminal choice
  (`__stop__` / `__none__`) against that option's fixed English wording, so the outcome
  depended on the query's alphabet: English queries drifted toward abstaining and other
  languages almost never did. It now descends only when one child stands out from its
  siblings, which is scale- and language-neutral. Out-of-tree queries return
  `leaf_id: null`, and out-of-tree ingest returns 400 as documented. Exact leaf match over
  150 sampled seed questions rose from 93/150 to 104/150, with 4 of the remaining misses
  being honest abstains.
- `duplicate_ids` is now a real near-duplicate check (character trigram overlap ≥ 0.6)
  instead of the top five items in the same category.

### Added

- Two-step ingest in the UI: the draft is shown with its route and duplicates, then
  published or discarded.
- `IngestResult.item_id` and `.version`, so a draft is addressable over plain HTTP.
- `GET /api/items?status=active|draft|all` and `DELETE /api/items/{id}` (archive).
- A run deadline (`JEV_TREE_RUN_TIMEOUT_SECS`, default 120) and cancellation when a
  streaming client disconnects.

### Removed

- The `knowledge_fts` mirror, which was written on every upsert and never read. Existing
  databases drop it on open.
- Every CDN dependency. `/docs` is a small self-contained OpenAPI renderer and the UI uses
  system fonts, so neither makes an external request.
- `RunRequest.upsert`, which was accepted and ignored.

### Changed

- Descent runs for as many rounds as the taxonomy is deep instead of a fixed 12.
- `GET /api/items` filters and paginates in SQL rather than loading every active row.

## 0.2.2

### Fixed

- **Descent dropped beams that reached a leaf.** A beam sitting on a childless node was
  excluded from the frontier and not carried into the next round, so whenever another beam
  could still descend, the chosen shallow leaf disappeared and the run drifted to the
  deepest available branch. Beams that cannot expand now settle and keep competing on
  score. On the bundled seed, exact leaf match over 150 sampled questions went from
  71/150 to 93/150 with no change in abstain behaviour.
- **`GET /api/settings` panicked on a non-ASCII secret.** `mask_secret` sliced bytes
  instead of characters, so storing a multi-byte key broke the settings endpoint until the
  value was cleared.

### Added

- The server refuses to start on a non-loopback bind while in open mode. Set
  `JEV_TREE_INIT_SERVER_KEY` or `JEV_TREE_ALLOW_OPEN=1`.
- Boot fails fast when `JEV_TREE_STATIC_DIR` has no `index.html` instead of serving empty
  `200` responses.
- `CatchPanicLayer` turns a handler panic into a 500 instead of a dropped connection.

### Changed

- Documentation reduced to `README.md`, `AGENTS.md`, `docs/api.md`, and the bootstrap
  skill, all in English. `direction.md`, `library-intent.md`, `architecture.md`,
  `demo-ui.md`, `accuracy-eval.md`, `roadmap.md`, and the bootstrap `references/` files
  were removed; whatever matched the code was folded into what remains.
- `AGENTS.md` carries a verified "Known gaps" table.
- OpenAPI: the terminal SSE event for a draft ingest is `ingest_draft`, not `descent_done`.
- Dropped the unused `tower-http` `cors` feature.

## 0.2.1

- Stay (`__stop__`) and no-match (`__none__`) now win the beam. A terminal Choice no longer expands children.
- Taxonomy lives in SQLite. `POST /api/seed` with `{"confirm":"RESET"}` reloads the in-memory tree without restart.
- Draft ingest is stored (`status=draft`) and omitted from search listings. Publish still requires `auto_publish: true`.
- Ingest outside the tree returns 400 instead of writing `unclassified`.
- Choice criteria include node examples. Context accepts `{role, text}` turns as well as strings.
- Default bind is `127.0.0.1`. Use `JEV_TREE_BIND=0.0.0.0` to listen on all interfaces.
- `/api/health` no longer returns the local database path.
- Engine tests script Jev Choices so stay, leaf routing, and draft ingest are fixed without a live key.
