---
tags: [cache, read]
priority: P4
phase: 4
endpoints:
  - GET /repos/{owner}/{repo}/actions/caches
---

# Cache List

## As a

developer debugging CI performance

## I want

to list the GitHub Actions caches for a repository

## Acceptance criteria

1. Running `gor cache list` in a repo directory lists all caches
2. Each row shows: cache key, size, ref, created date, last accessed date
3. `--repo` / `-R` flag specifies the repository explicitly
4. `--limit` / `-L` flag caps results (default: 30)
5. `--json` flag outputs as JSON with optional field selection
6. `--hostname` flag targets a specific host
7. Exit code 0 on success

## Out of scope

- Deleting caches (separate story)
- Cache creation (handled by workflows automatically)
