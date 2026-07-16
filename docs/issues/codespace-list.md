---
tags: [codespace, read]
priority: P2
phase: 4
endpoints:
  - GET /user/codespaces
status: todo
blockedBy: [auth-login]
blocks: [codespace-delete, codespace-stop, codespace-ssh, codespace-logs]
---

# Codespace List

## As a

developer with cloud development environments on github.com

## I want

to list my GitHub Codespaces

## Acceptance criteria

1. Running `gor codespace list` prints all of the authenticated user's codespaces
2. Each row shows: name, repository, branch, state, and created date
3. `--repo` flag filters codespaces for a specific repository
4. `--limit` / `-L` flag caps results (default: 30)
5. `--json` flag outputs as JSON with optional field selection
6. Codespaces are only available on github.com — GHES returns a clear "unsupported" error

## Out of scope

- Creating or deleting codespaces (separate stories)
- Filtering by machine type
