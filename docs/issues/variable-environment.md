---
tags: [variable, write]
priority: P2
phase: 4
endpoints:
  - GET /repos/{owner}/{repo}/environments/{env}/variables
  - PATCH /repos/{owner}/{repo}/environments/{env}/variables/{name}
  - DELETE /repos/{owner}/{repo}/environments/{env}/variables/{name}
status: done
blockedBy: [variable-list, variable-set, variable-delete]
blocks: []
---

# Variable Environment

## As a

developer managing deployment-specific configuration

## I want

to list, set, and delete variables scoped to a GitHub Actions environment

## Acceptance criteria

1. Running `gor variable list --env production` lists variables for the `production` environment
2. Running `gor variable set NODE_ENV --env staging --body "staging"` sets an environment-scoped variable
3. Running `gor variable delete NODE_ENV --env staging` deletes an environment-scoped variable
4. `--env` / `-e` flag specifies the environment name (required for environment operations)
5. `--repo` / `-R` flag specifies the repository explicitly
6. `--hostname` flag targets a specific host
7. Environment variables are listed with the environment name shown in the output
8. Setting a variable in a non-existent environment fails with a clear error

## Out of scope

- Creating or managing environments themselves
- Environment-level secrets (separate story)
