# Security

Report vulnerabilities privately to **lee@lou2.kr**. Do not open a public issue for
secrets, auth bypasses, or key handling. Include the affected commit, a reproduction,
and the impact.

## How secrets are stored

- Login passwords: salted SHA-256 hashes.
- Jev API keys and LLM tokens: AES-256-GCM at rest. The key material lives **outside**
  the database (`JEV_TREE_SECRET_KEY_FILE`, `JEV_TREE_SECRET_KEY`, or a `0600` sibling
  `.jev-tree.key`), so a database dump alone is not enough.
- Session tokens and integration API keys: SHA-256 hashes only.

Never commit `.env`, `.jev-tree.key`, `.jev-tree.salt`, or `data/*.db`.

## Before exposing the server

The default bind is `127.0.0.1`. Without a login password the process runs in open mode, where
an unauthenticated caller can claim the password, reset the seed, and read settings.
Set a password before changing `JEV_TREE_BIND`. See
[`AGENTS.md`](AGENTS.md#deployment-safety) for the current deployment caveats.
