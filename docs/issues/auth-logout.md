---
tags: [auth, write]
priority: P0
phase: 0
endpoints: []
status: done
blockedBy: [auth-login]
blocks: []
---

# Auth Logout

## As a

developer who wants to remove stored credentials

## I want

to log out and have my token removed from the OS keyring

## Acceptance criteria

1. Running `gor auth logout` removes the stored token from `~/.config/gor/hosts.yml`
2. A confirmation message is displayed on success
3. `--hostname` flag targets a specific host
4. If no token exists, a message indicates already logged out
5. Exit code 0 on success

## Out of scope

- Revoking the token on the server side (this is a local-only operation)
- Bulk logout of all hosts
