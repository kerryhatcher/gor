---
tags: [codespace, write]
priority: P2
phase: 4
endpoints:
  - DELETE /user/codespaces/{codespace_name}
status: todo
blockedBy: [codespace-list]
blocks: []
---

# Codespace Delete

## As a

developer who no longer needs a cloud dev environment

## I want

to delete a GitHub Codespace

## Acceptance criteria

1. Running `gor codespace delete <name>` deletes the named codespace
2. The codespace name is resolved from `gor codespace list` output
3. A confirmation prompt is shown before deletion (bypassed with `--yes`)
4. `--repo` flag scopes selection when only a partial name is given
5. A success message is printed after deletion
6. Codespaces are only available on github.com

## Out of scope

- Bulk deletion of all codespaces
- Recovering a deleted codespace
