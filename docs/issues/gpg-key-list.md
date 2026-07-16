---
tags: [gpg-key, read]
priority: P2
phase: 3
endpoints:
  - GET /user/gpg_keys
status: done
blockedBy: [auth-login]
blocks: []

# GPG Key List

## As a

developer managing my account's GPG keys

## I want

to list the GPG keys registered on my GitHub account

## Acceptance criteria

1. Running `gor gpg-key list` prints all GPG keys for the authenticated user
2. Each row shows: key ID, key title, and the email addresses the key is associated with
3. `--hostname` flag targets a specific host
4. `--json` flag outputs as JSON with optional field selection
5. If no keys exist, a message is printed and the command exits 0

## Out of scope

- Adding or deleting GPG keys (separate stories)
- Showing the full public key body
