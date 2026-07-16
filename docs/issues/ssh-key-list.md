---
tags: [ssh-key, read]
priority: P3
phase: 3
endpoints:
  - GET /user/keys
---

# SSH Key List

## As a

developer managing my GitHub account's SSH keys

## I want

to list my registered SSH public keys

## Acceptance criteria

1. Running `gor ssh-key list` lists all SSH keys for the authenticated user
2. Each row shows: key title, key fingerprint, last-used date
3. `--hostname` flag targets a specific host
4. `--json` flag outputs as JSON with optional field selection
5. Exit code 0 on success

## Out of scope

- Adding or deleting keys (separate stories)
- Viewing the full public key body
