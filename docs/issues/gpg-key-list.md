---
tags: [gpg-key, read]
priority: P3
phase: 3
endpoints:
  - GET /user/gpg_keys
---

# GPG Key List

## As a

developer managing my GitHub account's GPG keys

## I want

to list my registered GPG public keys

## Acceptance criteria

1. Running `gor gpg-key list` lists all GPG keys for the authenticated user
2. Each row shows: key ID, key fingerprint, last-used date
3. `--hostname` flag targets a specific host
4. `--json` flag outputs as JSON with optional field selection
5. Exit code 0 on success

## Out of scope

- Adding or deleting keys (separate stories)
- Viewing the full public key body
