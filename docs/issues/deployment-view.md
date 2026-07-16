---
tags: [deployment, read]
priority: P2
phase: 4
endpoints:
  - GET /repos/{owner}/{repo}/deployments/{deployment_id}
  - GET /repos/{owner}/{repo}/deployments/{deployment_id}/statuses
status: todo
blockedBy: [deployment-list]
blocks: []
---

# Deployment View

## As a

developer investigating deployment issues

## I want

to view details of a specific deployment

## Acceptance criteria

1. Running `gor deployment view 12345` shows details of deployment #12345
2. The deployment ID is a required positional argument
3. The output includes: environment, ref, SHA, creator, creation date, and latest status
4. `--statuses` flag includes the full status history
5. `--repo` / `-R` flag specifies the repository explicitly
6. `--json` flag outputs as JSON with optional field selection
7. `--hostname` flag targets a specific host
8. If the deployment does not exist, the command exits non-zero with a clear error

## Out of scope

- Viewing deployment logs (use the log URL from the status)
- Viewing deployment protection rules
