---
tags: [ssh-key, write]
priority: P2
phase: 4
endpoints:
  - DELETE /user/keys/{key_id}
status: todo
blockedBy: [ssh-key-list]
blocks: []
---

# SSH Key Delete

## As a

developer rotating or retiring old SSH keys

## I want

to delete an SSH public key from my GitHub account

## Acceptance criteria

1. Running `gor ssh-key delete 12345` deletes the SSH key with database ID `12345`
2. The key ID is a required positional argument
3. `--hostname` flag targets a specific host
4. A confirmation prompt is shown before deletion (bypassed with `--yes`)
5. A success message is printed with the deleted key's title
6. If the key does not exist, the command exits non-zero with a clear error

## Out of scope

- Deleting keys by title (use the key ID from `gor ssh-key list`)
- Bulk deletion of multiple keys
