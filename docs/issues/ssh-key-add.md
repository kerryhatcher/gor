---
tags: [ssh-key, write]
priority: P3
phase: 3
endpoints:
  - POST /user/keys
---

# SSH Key Add

## As a

developer who wants to authenticate git operations via SSH

## I want

to add a new SSH public key to my account

## Acceptance criteria

1. Running `gor ssh-key add "ssh-ed25519 AAAA... user@host"` adds the given public key
2. `--title` flag sets a friendly name for the key (required)
3. `--type` flag sets the key type (`authentication` or `signing`, default: `authentication`)
4. The key body may also be read from a file path argument instead of inline
5. `--hostname` flag targets a specific host
6. A confirmation message with the key ID is printed on success
7. Exit code 0 on success

## Out of scope

- Generating SSH key pairs (use `ssh-keygen`)
- Deleting keys (separate story)
