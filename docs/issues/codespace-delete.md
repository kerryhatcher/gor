---
tags: [codespace, write]
priority: P3
phase: 4
endpoints:
  - DELETE /user/codespaces/{codespace_name}
---

# Codespace Delete

## As a

developer who no longer needs a cloud environment

## I want

to delete a codespace

## Acceptance criteria

1. Running `gor codespace delete my-codespace` deletes the named codespace
2. A confirmation prompt is shown before deletion (bypassed with `--yes`)
3. Only available on github.com — a clear error is shown when used on GHES
4. A success message is printed after deletion
5. Exit code 0 on success

## Out of scope

- Bulk deletion of all codespaces
- Stopping without deleting (separate story)
