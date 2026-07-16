---
tags: [gpg-key, write]
priority: P2
phase: 4
endpoints:
  - DELETE /user/gpg_keys/{key_id}
status: todo
blockedBy: [gpg-key-list]
blocks: []
---

# GPG Key Delete

## As a

developer rotating or retiring old GPG signing keys

## I want

to delete a GPG public key from my GitHub account

## Acceptance criteria

1. Running `gor gpg-key delete 12345` deletes the GPG key with database ID `12345`
2. The key ID is a required positional argument
3. `--hostname` flag targets a specific host
4. A confirmation prompt is shown before deletion (bypassed with `--yes`)
5. A success message is printed with the deleted key's name/email
6. If the key does not exist, the command exits non-zero with a clear error

## Out of scope

- Deleting keys by email or fingerprint (use the key ID from `gor gpg-key list`)
- Bulk deletion of multiple keys
