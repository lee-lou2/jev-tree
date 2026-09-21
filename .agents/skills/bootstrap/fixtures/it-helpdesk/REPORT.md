# it-helpdesk fixture

A synthetic IT helpdesk dataset used to exercise this skill end to end.

- **Input**: `raw_tickets.csv` (14 rows) + `it_guide.md` (3 sections) + `raw_newhire.csv`
  (3 rows, used for the graft pass).
- **Excluded**: one row carrying contact details (rewritten as a neutral instruction),
  one junk test row. No exact duplicates.
- **Tree**: 17 nodes, max depth 3. Root `it`, branches `it_network`, `it_vpn`,
  `it_printer`, `it_mail`, `it_account`, and `it_device` (added by the graft pass).
- **Loaded**: 21 items across 17 categories — `taxonomy.json` + `items.jsonl`.
- **Drill**: 11 paraphrase queries in `drill.json`.

`taxonomy.json` holds `nodes` only and `items.jsonl` holds the items, so they must be
merged into a single `{"nodes": [...], "items": [...]}` file before use as `JEV_TREE_SEED`.
Otherwise load with `POST /api/run` ingest.

`source_path` values in `items.jsonl` are repository-relative.
