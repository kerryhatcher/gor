---
tags: [codespace, read]
priority: P3
phase: 4
endpoints:
  - GET /user/codespaces
---

# Codespace List

## As a

developer who uses GitHub Codespaces

## I want

to list my active codespaces

## Acceptance criteria

1. Running `gor codespace list` lists all my codespaces
2. Each row shows: display name, repository, branch, state, created date
3. `--repo` / `-R` flag filters by repository
4. `--limit` / `-L` flag caps results (default: 30)
5. `--json` flag outputs as JSON with optional field selection
6. Only available on github.com (not GHES) — a clear error is shown when used on GHES
7. Exit code 0 on success

## Out of scope

- Starting or stopping codespaces (separate stories)
- Codespace creation
