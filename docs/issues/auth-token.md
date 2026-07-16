---
tags: [auth, read]
priority: P2
phase: 4
endpoints:
  - PATCH /applications/{client_id}/token
  - POST /applications/{client_id}/token
status: todo
blockedBy: [auth-login]
blocks: []
---

# Auth Token

## As a

developer who needs to inspect or refresh my authentication token

## I want

to print my current token and refresh it when needed

## Acceptance criteria

1. Running `gor auth token` prints the current authentication token to stdout
2. `--hostname` flag targets a specific host (default: github.com)
3. The token is read from the OS keyring or environment variable (`GITHUB_TOKEN` / `GH_TOKEN`)
4. Running `gor auth token --refresh` refreshes the OAuth token and prints the new one
5. `--scopes` / `-s` flag requests additional OAuth scopes during refresh (repeatable)
6. `--secure` flag limits token output (only prints the first and last few characters)
7. If no token is found, the command exits non-zero with a clear message

## Out of scope

- Token revocation (use GitHub web UI)
- Fine-grained PAT management
