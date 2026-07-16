---
tags: [repo, write]
priority: P1
phase: 1
endpoints:
  - DELETE /repos/{owner}/{repo}
---

# Repo Delete

## As a

developer cleaning up old repositories

## I want

to delete a repository I own

## Acceptance criteria

1. Running `gor repo delete owner/repo` deletes the repository
2. A confirmation prompt is shown before deletion (bypassed with `--yes`)
3. The repository name must be confirmed by typing it (bypassed with `--yes`)
4. `--hostname` flag targets a specific host
5. A success message is printed after deletion
6. Exit code 0 on success, non-zero on failure or cancellation

## Out of scope

- Bulk repository deletion
- Transferring a repository (see repo-transfer.md)
