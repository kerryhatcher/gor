---
tags: [pr, read]
priority: P2
phase: 4
endpoints:
  - GET /search/issues
status: todo
blockedBy: [pr-list]
blocks: []
---

# PR Status

## As a

developer tracking my work across repositories

## I want

to see a summary of pull requests I'm involved in

## Acceptance criteria

1. Running `gor pr status` shows PRs relevant to the current user
2. The output is grouped by: "Current branch", "Created by you", "Requesting your review"
3. Each PR shows: number, title, repository, state, and review status
4. `--repo` / `-R` flag scopes results to a specific repository
5. `--hostname` flag targets a specific host
6. `--json` flag outputs as JSON with optional field selection
7. If no relevant PRs are found, a clear message is shown

## Out of scope

- Cross-organization PR status
- Filtering by label or milestone
