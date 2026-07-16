---
tags: [ssh-key, read]
priority: P2
phase: 3
endpoints:
  - GET /user/keys
status: todo
blockedBy: [auth-login]
blocks: []

# SSH Key List

## As a

developer managing my account's SSH keys

## I want

to list the SSH public keys registered on my GitHub account

## Acceptance criteria

1. Running `gor ssh-key list` prints all SSH keys for the authenticated user
2. Each row shows: key title, key fingerprint/SHA, and last-used date
3. `--hostname` flag targets a specific host
4. `--json` flag outputs as JSON with optional field selection
5. If no keys exist, a message is printed and the command exits 0

## Out of scope

- Adding or deleting SSH keys (separate stories)
- Showing the full public key body
