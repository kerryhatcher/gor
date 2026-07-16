---
tags: [environment, write]
priority: P2
phase: 4
endpoints:
  - PUT /repos/{owner}/{repo}/environments/{env}
status: todo
blockedBy: [environment-list]
blocks: []
---

# Environment Create

## As a

DevOps engineer setting up deployment pipelines

## I want

to create or update a deployment environment

## Acceptance criteria

1. Running `gor environment create staging` creates a `staging` environment
2. The environment name is a required positional argument
3. `--repo` / `-R` flag specifies the repository explicitly
4. `--wait-timer` flag sets the wait timer in minutes before deployments can proceed
5. `--branch` / `-b` flag restricts deployments to a specific branch
6. `--reviewers` flag adds required reviewers (repeatable, team slug or user login)
7. `--hostname` flag targets a specific host
8. A success message is printed confirming the environment was created/updated
9. If the environment already exists, it is updated with the new settings

## Out of scope

- Setting deployment branch policies with custom rules
- Environment-level secrets and variables (use `gor secret` / `gor variable`)
