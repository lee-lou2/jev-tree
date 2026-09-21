---
name: bootstrap
description: Build a jev-tree taxonomy and knowledge set from raw input (CSV, TSV, JSON, JSONL, MD, TXT, FAQ exports, ticket dumps, chat logs). Use when the tree does not exist yet, when grafting one dataset onto an existing tree, or when verifying a load.
license: MIT
---

# bootstrap — first or bulk data load

Turn raw material into a taxonomy plus normalised Q&A, load it, and verify the routing.
Keep domain concepts in `data/seed.json` or this skill's `fixtures/` — never in
`src/engine.rs` or `src/jev.rs`.

## 0. Pick a mode and say which one

| Mode | When | Output |
|---|---|---|
| **bootstrap** (default) | No tree, or a whole new domain | New taxonomy + items + load + report |
| **graft** | One dataset onto an existing tree | Reuse nodes, add only what is missing |
| **verify** | Inspect a loaded database | Drill report, no writes |

If it is ambiguous, ask: new tree or addition to an existing one?

## 1. Read the input

Any format is fine; the output is always Q&A units.

- **Tabular** (CSV/TSV/JSON/JSONL): confirm the field mapping with the user
  ("question ← `title`, answer ← `body`?"). A label column is a hint, not ground truth.
  Several text columns (`symptom`, `cause`, `fix`) usually means split rows, not one row.
- **Documents** (MD/TXT): split on headings, else on blank lines. Numbered lists split per item.
  Anything over ~2,000 characters is split into one topic per unit.
- **Tickets / chat**: first requester message is the question candidate, final agent reply is
  the answer. Drop filler turns. Split a thread when the topic changes.

Report counts before continuing: total units, field mapping, dropped rows.
Personal data (names, phone numbers, account numbers, order ids) is masked or dropped
here — it never reaches a seed or fixture.

## 2. Normalise to Q&A

Storage unit is `question` / `answer`, kind `qa` or `article`.

**Question** (≤ 240 characters)
- Keep the conditions. "on iPhone" is what separates two leaves — never summarise it away.
- Rewrite statements as questions.
- One question per condition set; split "how long and how much?" into two.
- Drop greetings and emotion, keep dates, amounts, and device names.

**Answer** (verbatim)
- Do not truncate. Procedures, amounts, deadlines, and exceptions all stay — ranking reads
  the answer body.
- If the answer branches ("if A then X, if B then Y"), split into one item per branch.
- Empty or tautological answers become `article` with the title only.
- "Contact support" endings must say what to prepare, or go back to the source.

**kind**
- `qa`: can be sent as a reply on its own.
- `article`: background, or the guide item that sits on a non-leaf node.

Merge exact duplicates. Items that look similar but carry different conditions are **not**
duplicates — the condition is what selects the leaf.

Show five normalised samples and get approval.

## 3. Design the tree

Bottom-up for bootstrap:

1. Group the normalised items and pick 5–12 root branches yourself. The model is a tree
   *user*, not a tree *designer*.
2. **Siblings must be mutually exclusive.** If an item could plausibly go to either of two
   siblings, merge them or sharpen both descriptions.
3. Naming:
   - `id`: snake_case path that extends the parent id (`payments_refund_timing_card`).
   - `name`: label in the reader's language, distinct from its siblings.
   - `description`: one sentence saying how this node differs from its siblings.
     A parent description lists its children.
   - `examples`: 1–3 real phrasings. They are sent to the evaluator verbatim.
4. Depth 3–5 — only as deep as queries actually separate.
5. Give every non-leaf node one `article` item so vague questions can stop there.

For graft: read `GET /api/tree` (add `?root=…` only when working inside a virtual root),
reuse whatever fits, sharpen sibling descriptions when a new topic is close to an
existing node, and add nodes only for genuinely new branches.
Re-run `cargo test` and a real search after editing an existing description.

### Exchange format

`data/seed.json` is `{"nodes": [...], "items": [...]}`.

```json
{"id": "payments_refund_timing_card", "name": "Card refunds",
 "description": "How long a card reversal takes; unlike bank refunds (7-10 business days).",
 "examples": ["When do I get my money back on the card?"],
 "parent_id": "payments_refund_timing"}

{"category_id": "payments_refund_timing_card", "kind": "qa",
 "question": "...", "answer": "..."}
```

Fixtures may keep `nodes` and `items` in two files, but anything passed to `JEV_TREE_SEED`
must be a single file with **both** keys, or boot fails.

The server rejects a seed with duplicate ids, missing parents, cycles, duplicate sibling
names, or empty names and descriptions.

## 4. Verify before loading

Never load straight into a live database.

```bash
JEV_TREE_DB=/tmp/jev_tree_boot/boot.db \
JEV_TREE_SEED=/tmp/jev_tree_boot/seed.json \
cargo run --release
# POST /api/run {"mode":"search","query":"..."} and check leaf_id against the intended node
```

Write the drill as paraphrases, not copied source text. With no Jev key the evaluator is a
lexical heuristic: it will not match `VPN` to `브이피엔` or tolerate heavy inflection, so keep
at least one shared keyword per drill query. If routing is wrong, fix the tree — not the model.

## 5. Load and report

Load with `POST /api/run` ingest and `auto_publish: true`. The same question in the same
category upserts. Promote to the real database only after verification and approval.

Leave behind `fixtures/<dataset>/` with the seed files, the drill, and a short `REPORT.md`
covering input counts, exclusions, tree shape, and drill results.

## Do not

1. Load raw text without normalising it.
2. Ship siblings whose meanings overlap.
3. Write directly to a live database.
4. Put domain concepts in `src/engine.rs` or `src/jev.rs`.
5. Include personal or real customer data in a seed or fixture.
6. Add a Python script or change the default port.
