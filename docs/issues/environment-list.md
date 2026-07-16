---
tags: [environment, read]
priority: P2
phase: 4
endpoints:
  - GET /repos/{owner}/{repo}/environments
status: todo
blockedBy: [repo-view]
blocks: [environment-create, environment-delete, environment-edit]
---

# Environment List

## As a

DevOps engineer managing deployment environments

## I want

to list environments for a repository

## Acceptance criteria

1. Running `gor environment list` lists all environments for the repository
2. `--repo` / `-R` flag specifies the repository explicitly
3. Each row shows: environment name, protection rules status, and deployment branch policy
4. `--limit` / `-L` flag caps results (default: 30)
5. `--json` flag outputs as JSON with optional field selection
6. `--hostname` flag targets a specific host
7. If no environments are found, a clear message is shown

## Out of scope

- Listing environment secrets (use `gor secret list --env`)
- Listing environment variables (use `gor variable list --env`)
