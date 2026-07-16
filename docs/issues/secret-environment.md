---
tags: [secret, write]
priority: P2
phase: 4
endpoints:
  - GET /repos/{owner}/{repo}/environments/{env}/secrets
  - PUT /repos/{owner}/{repo}/environments/{env}/secrets/{name}
  - DELETE /repos/{owner}/{repo}/environments/{env}/secrets/{name}
status: todo
blockedBy: [secret-list, secret-set, secret-delete]
blocks: []
---

# Secret Environment

## As a

developer managing deployment-specific secrets

## I want

to list, set, and delete secrets scoped to a GitHub Actions environment

## Acceptance criteria

1. Running `gor secret list --env production` lists secrets for the `production` environment
2. Running `gor secret set DB_URL --env staging --body "postgres://..."` sets an environment-scoped secret
3. Running `gor secret delete DB_URL --env staging` deletes an environment-scoped secret
4. `--env` / `-e` flag specifies the environment name (required for environment operations)
5. `--repo` / `-R` flag specifies the repository explicitly
6. `--hostname` flag targets a specific host
7. Environment secrets are listed with the environment name shown in the output
8. Setting a secret in a non-existent environment fails with a clear error

## Out of scope

- Creating or managing environments themselves (use `gh api` or the web UI)
- Environment-level variables (separate story)
