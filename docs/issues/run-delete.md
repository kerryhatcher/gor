---
tags: [ci, write]
priority: P2
phase: 4
endpoints:
  - DELETE /repos/{owner}/{repo}/actions/runs/{id}
status: done
blockedBy: [run-view]
blocks: []
---

# Run Delete

## As a

developer cleaning up old CI runs

## I want

to delete a workflow run and its logs from the repository

## Acceptance criteria

1. Running `gor run delete 12345` deletes workflow run #12345
2. The run ID is a required positional argument
3. `--repo` / `-R` flag specifies the repository explicitly
4. `--hostname` flag targets a specific host
5. A confirmation prompt is shown before deletion (bypassed with `--yes`)
6. A success message is printed after deletion
7. If the run does not exist, the command exits non-zero with a clear error

## Out of scope

- Bulk deletion of multiple runs
- Deleting runs by workflow name or branch filter
