---
tags: [repo, write]
priority: P2
phase: 4
endpoints:
  - PATCH /repos/{owner}/{repo}
status: todo
blockedBy: [repo-view]
blocks: []
---

# Repo Archive

## As a

developer cleaning up inactive repositories

## I want

to archive or unarchive a repository

## Acceptance criteria

1. Running `gor repo archive owner/repo` archives the repository
2. Running `gor repo archive owner/repo --unarchive` unarchives a previously archived repository
3. `--unarchive` flag reverses the operation (unarchive instead of archive)
4. A confirmation prompt is shown before archiving (bypassed with `--yes`)
5. `--repo` / `-R` flag specifies the repository explicitly
6. `--hostname` flag targets a specific host
7. A success message is printed confirming the operation
8. If the repository is already in the target state, the command exits gracefully with a clear message

## Out of scope

- Bulk archiving multiple repositories
- Deleting repositories (see repo-delete.md)
