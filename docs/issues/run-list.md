---
tags: [ci, read]
priority: P2
phase: 2
endpoints:
  - GET /repos/{owner}/{repo}/actions/runs
---

# Run List

## As a

developer monitoring CI activity

## I want

to list recent workflow runs in a repository

## Acceptance criteria

1. Running `gor run list` in a repo directory lists recent workflow runs
2. Each row shows: run ID, workflow name, event, branch, status, conclusion, timestamp
3. `--repo` / `-R` flag specifies the repository explicitly
4. `--workflow` flag filters by workflow filename or ID
5. `--branch` flag filters by branch
6. `--event` flag filters by triggering event (`push`, `pull_request`, `schedule`, etc.)
7. `--status` flag filters by status (`queued`, `in_progress`, `completed`)
8. `--limit` / `-L` flag caps results (default: 20)
9. `--json` flag outputs as JSON with optional field selection
10. `--hostname` flag targets a specific host

## Out of scope

- Run log streaming
- Run artifact listing
