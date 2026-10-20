# Security

Report vulnerabilities privately to **lee@lou2.kr**. Do not open a public issue for
secrets, auth bypasses, or key handling. Include the affected commit, a reproduction,
and the impact.

## How secrets are stored

- Login passwords: Argon2id (`v2:`, PHC string with its own random salt). Hashes written
  before 0.9 were salted SHA-256 (`v1:`); they still verify and are replaced at the next login.
  Five wrong passwords in a row start a backoff (1 s doubling to 60 s).
- Jev API keys and LLM tokens: AES-256-GCM at rest. The key material lives **outside**
  the database (`JEV_TREE_SECRET_KEY_FILE`, `JEV_TREE_SECRET_KEY`, or a `0600` sibling
  `.jev-tree.key`), so a database dump alone is not enough.
- Session tokens and integration API keys: SHA-256 hashes only. An API key reaches the data
  endpoints; settings, key management and the seed reset need a login session.

Never commit `.env`, `.jev-tree.key`, `.jev-tree.salt`, or `data/*.db`.

## Before exposing the server

The default bind is `127.0.0.1`. Without a login password the process runs in open mode, where
an unauthenticated caller can claim the password, reset the seed, and read settings.
Set a password before changing `JEV_TREE_BIND`. See
[`AGENTS.md`](AGENTS.md#deployment-safety) for the current deployment caveats.
