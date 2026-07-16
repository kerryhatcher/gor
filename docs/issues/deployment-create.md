---
tags: [deployment, write]
priority: P2
phase: 4
endpoints:
  - POST /repos/{owner}/{repo}/deployments
status: todo
blockedBy: [deployment-list]
blocks: []
---

# Deployment Create

## As a

developer triggering a deployment

## I want

to create a new deployment for a repository

## Acceptance criteria

1. Running `gor deployment create --ref main --env production` creates a deployment
2. `--ref` flag specifies the git ref to deploy (branch, tag, or SHA) — required
3. `--env` / `-e` flag specifies the target environment (required)
4. `--task` flag specifies the deployment task (default: `deploy`)
5. `--description` / `-d` flag sets a description for the deployment
6. `--payload` flag sets a JSON payload for the deployment (or `@file.json` to read from file)
7. `--auto-merge` flag auto-merges the default branch into the deployment ref
8. `--required-contexts` flag specifies required status checks (comma-separated)
9. `--repo` / `-R` flag specifies the repository explicitly
10. `--hostname` flag targets a specific host
11. A success message is printed with the deployment ID

## Out of scope

- Creating deployment statuses (see deployment-status.md)
- Production environment protection bypass
