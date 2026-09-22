# Contributing

Read [`AGENTS.md`](AGENTS.md) first. It holds the invariants, the file map, and the
current known gaps.

## Local loop

```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt
cargo run --release      # repo root, http://127.0.0.1:8768
```

The working directory must be the repo root; default paths are relative.
The default bind is loopback — use `JEV_TREE_BIND=0.0.0.0` only on purpose.

## Rules for a change

- Descent, stay, and ranking belong in `src/engine.rs`. Cover them with a test that
  drives `JevClient::script_choices`, not a live key.
- Jev and the optional LLM are called only from `src/jev.rs`. A run without a Jev key
  returns 400. Do not route that case through the lexical heuristic.
- A route change must update `static/openapi.json` and `docs/api.md` in the same commit.
  A test compares the spec's path set with the router.
- `POST /api/seed` keeps its `{"confirm":"RESET"}` guard.
- No second runtime, no `classify` mode, no new default port.
- Never commit `.env`, `.jev-tree.key`, `.jev-tree.salt`, or `data/*.db`.

## Pull requests

CI runs the test suite and clippy with warnings denied. If a change touches stay,
`__none__`, draft ingest, or seed reload, say in the PR which test pins the new behaviour.
