# Changelog

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
