---
tags: [codespace, write]
priority: P2
phase: 4
endpoints:
  - GET /user/codespaces/{codespace_name}/ssh
---

# Codespace SSH

## As a

developer who wants a shell in my cloud dev environment

## I want

to open an SSH session into a GitHub Codespace

## Acceptance criteria

1. Running `gor codespace ssh <name>` opens an interactive SSH session to the named codespace
2. The SSH connection details (host, port, user, private key) are fetched from `GET /user/codespaces/{name}/ssh`
3. A temporary SSH key is registered with the codespace for the session
4. `--repo` flag scopes selection when only a partial name is given
5. `--profile` flag selects an SSH config profile
6. `--config` flag prints the SSH connection config instead of connecting
7. Codespaces are only available on github.com

## Out of scope

- File transfer over SSH (use `gor codespace cp`)
- Port forwarding configuration
