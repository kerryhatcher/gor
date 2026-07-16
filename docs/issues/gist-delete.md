---
tags: [gist, write]
priority: P2
phase: 2
endpoints:
  - DELETE /gists/{id}
---

# Gist Delete

## As a

developer cleaning up old or sensitive gists

## I want

to delete a gist by its ID

## Acceptance criteria

1. Running `gor gist delete abc123def456` deletes the gist with that ID
2. A confirmation prompt is shown before deletion (bypassed with `--yes`)
3. `--hostname` flag targets a specific host
4. A success message is printed after deletion
5. If the gist does not exist or the user lacks permission, a clear error is shown and the command exits non-zero
6. Exit code 0 on success, non-zero on failure

## Out of scope

- Restoring a deleted gist
- Deleting multiple gists at once
