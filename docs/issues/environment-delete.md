---
tags: [environment, write]
priority: P2
phase: 4
endpoints:
  - DELETE /repos/{owner}/{repo}/environments/{env}
status: todo
blockedBy: [environment-list]
blocks: []
---

# Environment Delete

## As a

DevOps engineer cleaning up unused environments

## I want

to delete a deployment environment

## Acceptance criteria

1. Running `gor environment delete staging` deletes the `staging` environment
2. The environment name is a required positional argument
3. `--repo` / `-R` flag specifies the repository explicitly
4. A confirmation prompt is shown before deletion (bypassed with `--yes`)
5. `--hostname` flag targets a specific host
6. A success message is printed after deletion
7. If the environment does not exist, the command exits non-zero with a clear error

## Out of scope

- Deleting environments with active deployments
- Bulk environment deletion
