---
tags: [deployment, write]
priority: P2
phase: 4
endpoints:
  - DELETE /repos/{owner}/{repo}/deployments/{deployment_id}
status: todo
blockedBy: [deployment-list]
blocks: []
---

# Deployment Delete

## As a

developer cleaning up deployment history

## I want

to delete a deployment

## Acceptance criteria

1. Running `gor deployment delete 12345` deletes deployment #12345
2. The deployment ID is a required positional argument
3. `--repo` / `-R` flag specifies the repository explicitly
4. A confirmation prompt is shown before deletion (bypassed with `--yes`)
5. `--hostname` flag targets a specific host
6. A success message is printed after deletion
7. If the deployment does not exist, the command exits non-zero with a clear error

## Out of scope

- Deleting deployment statuses
- Bulk deployment deletion
