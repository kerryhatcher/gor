---
tags: [ssh-key, write]
priority: P2
phase: 3
endpoints:
  - POST /user/keys
status: done
blockedBy: [auth-login]
blocks: []

# SSH Key Add

## As a

developer who wants to authenticate git operations with a new key

## I want

to add an SSH public key to my GitHub account

## Acceptance criteria

1. Running `gor ssh-key add --title "laptop" -f ~/.ssh/id_ed25519.pub` adds the key
2. `--title` / `-t` flag sets the human-readable key title (required)
3. `--file` / `-f` flag reads the public key from a file
4. `--body` / `-b` flag provides the public key inline (alternative to `--file`)
5. `--hostname` flag targets a specific host
6. The created key's ID and title are printed on success
7. A missing or malformed key fails with a clear error and non-zero exit code

## Out of scope

- Uploading private keys (never sent to GitHub)
- Associating keys with specific repositories
