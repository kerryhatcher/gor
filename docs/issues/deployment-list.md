---
tags: [deployment, read]
priority: P2
phase: 4
endpoints:
  - GET /repos/{owner}/{repo}/deployments
status: todo
blockedBy: [repo-view]
blocks: [deployment-create, deployment-delete, deployment-status, deployment-view]
---

# Deployment List

## As a

developer tracking deployment history

## I want

to list deployments for a repository

## Acceptance criteria

1. Running `gor deployment list` lists deployments for the repository
2. `--repo` / `-R` flag specifies the repository explicitly
3. `--env` / `-e` flag filters by environment name
4. `--ref` flag filters by git ref (branch or tag)
5. `--sha` flag filters by commit SHA
6. Each row shows: deployment ID, environment, ref, SHA, and creation date
7. `--limit` / `-L` flag caps results (default: 30)
8. `--json` flag outputs as JSON with optional field selection
9. `--hostname` flag targets a specific host
10. If no deployments are found, a clear message is shown

## Out of scope

- Listing deployment statuses inline (use `gor deployment view`)
- Filtering by deployment status
