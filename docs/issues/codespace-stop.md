---
tags: [codespace, write]
priority: P2
phase: 4
endpoints:
  - POST /user/codespaces/{codespace_name}/stop
---

# Codespace Stop

## As a

developer who is done working in a cloud dev environment

## I want

to stop a running GitHub Codespace to avoid billing

## Acceptance criteria

1. Running `gor codespace stop <name>` stops the named codespace
2. The stop request is sent to `POST /user/codespaces/{name}/stop`
3. `--repo` flag scopes selection when only a partial name is given
4. `--all` flag stops every codespace owned by the user
5. A confirmation message is printed on success
6. Codespaces are only available on github.com

## Out of scope

- Deleting the codespace (use `gor codespace delete`)
- Restarting a stopped codespace
