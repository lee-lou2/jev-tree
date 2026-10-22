# Changelog

## 0.10.0

Built and measured against a new benchmark ten times the golden set's size
(`eval/large`: 1,902 nodes, 14,870 items, 3,860 cases). The golden tree fits in one Jev
choice, so it never showed what happens when a tree does not: at 1,902 nodes the old flat
route landed on the right node for 48.5% of requests. Every change below was first
measured on recorded judgments, then confirmed with paired runs (McNemar).

### Changed

- **A scope too big for one choice is routed through a coarse map, not word overlap.**
  Past 254 nodes the menu used to be the nodes sharing the most characters with the
  request, which held the right node only 69% of the time on the benchmark (paraphrases
  and English queries had nothing to match). Now one Jev choice over the deepest tree level
  that still fits opens its best branches whole, and BM25 over every node's text **and the
  questions filed under it** adds 30 matches (`src/recall.rs`). The right node is in the
  menu 98% of the time and the menus are smaller (about 90 nodes against 254). A pick whose
  branch did not fit is walked into; a coarse map wider than one choice is split into
  questions with their own way out, asked in parallel.
- **An unsure search also ranks the runners-up.** Most misses on a big tree are sideways —
  a sibling, another branch, another domain — and they left the answer outside the pool
  however well Jev ranked. Below 0.9 the two next-best nodes' items join the pool (never
  the pick's own ancestors or descendants; only a runner-up's own items past 64), and a
  rescued item answers only at 0.75 or more, so false answers do not rise.
- **The LLM reviews Jev's unsure picks instead of routing every request.** A pick below 0.8
  goes to the LLM with the same menu Jev chose from, in tree order; confident picks never
  wait for it. Shown only Jev's top candidates the model agreed with Jev's mistakes (93%
  route accuracy on the golden queries against 97% with the whole menu), so it reads the
  whole menu. The LLM-everywhere router, its two-step walk above 800 nodes, and the
  lexical shortlist behind it are gone.
- **The level-by-level beam is gone.** The coarse map made it unreachable. `beam_width` is
  still accepted and ignored; `trace.router` is `flat` or `llm`.
- **Repeated searches are served from a cache** (`src/cache.rs`): 1,024 results for 10
  minutes, keyed by the request plus the store's and the settings' generation, so any
  write or settings change invalidates it. A hit makes no model call and emits `cache_hit`.
- Every model call leaves a `usage` event (`source`, `input_tokens`, `output_tokens`); the
  eval runner reports billed Jev tokens and LLM tokens per case from them, and counts Jev
  calls instead of estimating them.

### Measured

Large benchmark, 1,500 paired cases (`--sample 1500 --seed final`), `jev-latest`:

| | E2E | route | Hit@1 | ingest | p50 | Jev tokens |
|---|---:|---:|---:|---:|---:|---:|
| 0.9.0 Jev | 68.4% | 48.5% | 67.8% | 73.4% | 0.7s | 45.4K |
| 0.9.0 LLM routes every request (600 of the cases) | 73.7% | 64.2% | 72.4% | 88.5% | 12.1s | 12.4K + 6.2K LLM |
| 0.10.0 Jev | **82.0%** | 68.7% | 82.6% | 73.4% | 0.9s | 38.7K |
| 0.10.0 Jev + LLM review | **83.8%** | **75.5%** | **84.5%** | **76.6%** | 1.8s | 36.4K + 3.3K LLM |

0.9.0 → 0.10.0 Jev: E2E +13.6pp (232 fixed, 36 broken, p<0.0001), false holds 8.4% →
3.0%, false answers 25.0% → 24.2%. The LLM review adds +1.7pp E2E (p=0.014) and +6.8pp
route accuracy (p<0.0001); it reviewed 50% of requests on this tree. Against 0.9.0's LLM
router on the same 600 cases: E2E 73.7% → 85.0% (p<0.0001) at p50 3.7s instead of 11.9s;
Jev alone already reaches 83.3% at 0.9s.

Golden set (`eval/`, paired against the 0.9.0 baselines): Jev E2E up in every tree shape
(deep 97.3 → 97.8, standard 97.3 → 98.0, shallow 97.2 → 97.8, flat 96.7 → 97.6, sparse
90.5 → 90.8), none of it a regression. With the LLM: 98.4% on `standard` (LLM routing
everything: 97.7%), ingest still 100%, **p50 0.4s against 3.8s**, LLM tokens per search
1.7K against 11.7K; the LLM reviewed 13% of requests.

### Evaluator limits found

- A request has to fit about 64K tokens across all its questions (three 21K-token menus
  pass, four fail), on top of the ~32K per question. Big menus now go in separate requests.

## 0.9.0

Found by a five-agent review of 0.8.1 (correctness, accuracy, performance, security,
end-to-end) and re-checked against the live Jev API. Two of the findings turned every
search into a 502; the limits behind them are written down in `AGENTS.md`
("Evaluator limits").

### Fixed

- **A scope of 255 to 800 nodes no longer fails every search.** One Jev choice takes at
  most 255 options (256 is HTTP 422), but the flat route named every node up to 800, so a
  tree one node past 254 answered 502 to everything, and the 234-node live tree had 21
  nodes of room. The flat menu now stays within `FLAT_MAX_NODES` (254) and
  `FLAT_MAX_MENU_BYTES` (80 KB: a 234-node menu of 94 KB passes, 124 KB is HTTP 400);
  past either line it names the strongest lexical matches with their ancestors, then fills
  the rest of the 254 slots with the next best nodes, and without lexical evidence the beam
  runs. Measured on 187 golden queries (Jev router): 94.1-94.7% exact leaf (two runs) with the full
  219-node menu; with 60 unrelated nodes added (279, so past the line) 93.6% with the
  filled menu against 79.1% when the menu stopped at the ~47 strongest matches; 90.8% at
  500 nodes and 89.7% at 800. The golden set's variants (at most 219 nodes, 50 KB of menu)
  are unchanged.
- **A subtree of large items no longer fails its search.** Ranking sent 32 items per call
  whatever their size; a live subtree of 2,201 items reached batches of 157 KB and the
  evaluator answered HTTP 400 on the 66th call, 15 seconds into the run. A call now carries
  at most 32 items **and** 24 KB (ordinary batches are unchanged), and text over 8,000
  characters, which the API cannot store (question plus answer is capped there), is cut for
  the evaluator only; the result still carries the stored text. The longest answer in the
  live databases is 6,963 characters, so nothing live is cut.
- **Publishing a reviewed draft files it where it was reviewed.** The publish call routed
  the Q&A again; with an edited answer the second route could land elsewhere or nowhere,
  and the call failed with 400 so the draft could never be published. A call with
  `item_id` now keeps the row's category and asks no one (`trace.router` is `flat` with no
  steps). Pass `start_node` to move a row. If the stored category is gone or outside the
  `root`, the request routes afresh as before. An unknown `item_id` is a 404 before any
  routing.
- **`recommended`/`alternative` and the hold line include their exact value.**
  `0.5 * 0.7 + 1.0 * 0.3` is 0.6499999999999999 in f64, one step under the 0.65 it
  mathematically is, and a third of the grid of round scores fell on the low side of a line.
- **Every `/api` error is `{code, detail}`.** Malformed JSON, a field of the wrong type
  (was 422), a wrong content type (415), a body over 2 MB (413), a bad query or path value,
  and a method the path does not take (405) came back as plain text, and a handler panic
  as a bare 500.
- `GET /api/items?q=%` listed everything: `%` and `_` in a search are now ordinary
  characters.
- A broken or missing seed file names the file in the error.
- The log said "using beam descent" when an LLM route failed; the fallback is the flat
  Jev choice. An evaluator 400/413/422 now says the input may be too large instead of
  blaming the API key.

### Security

- **An API key can no longer mint keys, rotate the login password, or reset the seed.**
  Settings, `/api/keys`, `POST /api/llm/models` and `POST /api/seed` need a login session
  and answer a key with 403 `forbidden`. A leaked key could create more keys that survived
  its own revocation, change the password to lock the owner out, or point the LLM base URL
  at a host of its choosing and receive the saved token. Keys keep the data endpoints.
- **Login passwords are hashed with Argon2id** (`v2:`). SHA-256 hashes written by earlier
  versions still verify and are replaced at the next good login, and the comparison is
  constant-time.
- **Wrong passwords back off.** Five in a row start a wait of 1 s doubling to 60 s
  (`429 rate_limited`, `Retry-After`), refused before the password is hashed. Global, in
  memory, cleared by a good login or a quiet quarter hour. At most two passwords are
  hashed at a time and a queued attempt asks the guard again on its turn, so a burst of
  200 simultaneous guesses costs 6 hashes and 194 refusals instead of 200 hashes of
  19 MiB each.
- First-run setup, logins, settings changes, key issue and revoke, and a seed reset leave a
  log line (names and ids, never values).

### Changed

- Ranking runs four calls at a time and consumes them in order, so a large subtree takes a
  fraction of the serial time and the result does not depend on which call returns first.
  A listener that went away stops the run before the next call.
- `static/openapi.json` is at this version (it said 0.4.1), declares 401 on every guarded
  operation, 403 on the operator ones and 429 on login; a test keeps the version and those
  declarations in step.
- Docs: a `root` scope abstains (`leaf_id: null` means outside the root) for both routers,
  as the code and its test always did; re-sending a published question without
  `auto_publish` unpublishes it until it is published again; `GET /api/keys` is listed.

## 0.8.1

### Fixed

- **LLM routing no longer drops replies it can read.** Two warnings filled the log
  ("LLM returned no content", "LLM route did not return an allowed id") and every one
  of them fell back to Jev beam descent. Both had the same two roots: the route call
  asked for 900 output tokens while the model also bills its thinking against that
  budget, so a long deliberation over the menu left `content` empty; and the pick was
  read from the last line only, so a model that answered mid-sentence — or with the
  node's name instead of its id — was treated as no answer at all.
  The budget is now 16,384 — room for the thinking as well as the one id that comes
  out the far end — and 4,096 if the endpoint rejects a larger ask. The reply is
  parsed as a whole: the last unambiguous allowed id anywhere in it wins, with the
  node name as a last resort. A line naming two candidates is still refused rather
  than guessed. Covered in `src/jev.rs`.
- **A slow LLM can no longer cost the request its answer.** The route step shares the
  run deadline with the beam and the ranking that follow it, so a model that
  deliberates past the budget used to time the whole run out. The route step now gets
  a third of `JEV_TREE_RUN_TIMEOUT_SECS` and falls back to the Jev beam when that is
  spent.

## 0.8.0

### Changed

- **Holds are calibrated; weak answers no longer ship.** A best item scoring ≥ 0.45 answers
  outright; below that only a *direct* item (directness ≥ 1.0) answers, and anything weaker
  is held. The lines were swept on the golden set's 2,102 search cases (thresholds chosen on
  the recorded judgments) and confirmed with live runs: false answers 62 → 38 (−39%) with
  false holds 10 → 13, end-to-end +0.8~1.0pp, and `sparse` — where the answer is often
  missing — **+3.8%p** (McNemar p<0.0001). No tree shape regressed beyond the ±1%p run
  noise (repeat run agrees on every cell).
- **An uncertain route asks one more judgment on scope alone** ("does this tree cover the
  request at all"). The menu asks *which node fits* and an out-of-scope request can still
  win one; the scope judgment catches those (caught 73% → 77%). Only a confident `out`
  (p ≥ 0.8) refuses — a timid one kept refusing real questions ("laptop battery swelled,
  where do I apply?"). The step shows as `trace.steps[1]`; a `start_node` walk must land on
  a node and never asks.
- `jev_llm` re-measured with the DB-registered LLM (`muse-spark`) for the comparison:
  end-to-end is now a tie (97.3% vs 97.7% on `standard`, McNemar p=0.69). The LLM still
  routes better (route accuracy +3.5%p, p=0.001) and files better (ingest 100% vs 92.6%),
  at eleven times the latency (p50 5.6s vs 0.5s).

## 0.7.0

### Changed

- **Jev routing is one choice over every node in scope**, not a level-by-level beam.
  The beam guessed a path from the root down and one wrong level lost the leaf; the
  loss grew with depth (routing accuracy 88.7–89.2% on 2–7 level trees). One choice
  removes the cascade and the round trips. `__none__` competes with every node, so an
  out-of-scope request abstains at any depth. Under `root`/`start_node` the walk stays
  in that subtree and must land on a node there. Above `ONE_SHOT_MAX_NODES` (800) nodes
  in scope the menu falls back to lexical recall and, without lexical evidence, to the
  beam (`trace.router` is `flat`, `beam`, or `llm`).
- Measured on the golden set (`eval/`, `--router jev`, 2,404 cases, 0 errors) against the
  beam baseline: E2E **+4.0%p on `deep`** (McNemar p=0.0005), **+4.7%p on `standard`**
  (p<0.0001), +2.5%p on `sparse`; Hit@1 +5.0–9.4%p. Latency p50 1.0s → **0.5s** and
  Jev calls 4.4 → **2.2** per case. The gap to LLM routing (`jev_llm`, gpt-6-luna) on
  `standard` E2E narrows from +6.1%p to **+1.4%p**, at 0.5s versus 12.6s.
- A flat trace lists the candidates with probabilities and puts the chosen probability
  in `trace.score`; per-edge `probability` stays null.
- The eval mock oracle scripts most-specific-first, so the one-shot menu and the stepped
  walk both see the expected node first (`eval/runner/exec.rs`).

## 0.6.1

### Fixed

- **The Docker image could not reach TypeSafe or the LLM host.** `debian:bookworm-slim` ships
  without `ca-certificates`, and the client verifies TLS against the system trust store
  (`rustls-native-certs`), so every outbound call failed certificate verification. The runtime
  stage now installs `ca-certificates`.
- The HTTP client identified itself as `jev-tree/0.4`. It now sends the package version.

### Changed

- Graceful shutdown handles `SIGTERM` as well as Ctrl-C, so `docker stop` and Kubernetes drain
  in-flight runs instead of cutting them.
- Toolchain pinned at 1.98.1 (was 1.89.0) across `rust-toolchain.toml`, CI, and the Docker
  build; `rust-version` follows. Dependency lock refreshed (`cargo update`); direct
  dependencies were already current.
- **Removed the test-only lexical heuristic** (`JevClient::heuristic`, `terminal_bar`,
  `TERMINAL_MARGIN`, about 130 lines) and the two tests that exercised it. No production path
  called it — routing is the Jev beam or the LLM router, ranking is Jev — so it guarded
  nothing at runtime. `JevClient::with_config` takes the config only; the taxonomy clone it
  needed is gone. Lexical overlap stays where it is used: `llm_shortlist`'s candidate menu.
- Tests for `engine::path_score`, `engine::normalize`, and `JevConfig::with_db` moved from
  `src/http.rs` into the modules that own those functions.

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
