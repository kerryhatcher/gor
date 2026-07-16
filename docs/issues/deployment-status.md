---
tags: [deployment, write]
priority: P2
phase: 4
endpoints:
  - POST /repos/{owner}/{repo}/deployments/{deployment_id}/statuses
status: todo
blockedBy: [deployment-list]
blocks: []
---

# Deployment Status

## As a

CI/CD system reporting deployment progress

## I want

to create a deployment status update

## Acceptance criteria

1. Running `gor deployment status 12345 --state success` reports a successful deployment
2. The deployment ID is a required positional argument
3. `--state` flag specifies the status state (required): `error`, `failure`, `inactive`, `in_progress`, `queued`, `pending`, `success`
4. `--log-url` flag sets the URL to the deployment log output
5. `--env-url` flag sets the URL to the deployed environment
6. `--description` / `-d` flag sets a short description of the status
7. `--auto-inactive` flag marks all other deployments in the environment as inactive
8. `--repo` / `-R` flag specifies the repository explicitly
9. `--hostname` flag targets a specific host
10. A success message is printed with the status ID

## Out of scope

- Listing deployment statuses (use `gor deployment view`)
- Updating existing statuses
