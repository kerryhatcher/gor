---
tags: [codespace, read]
priority: P3
phase: 4
endpoints:
  - GET /user/codespaces/{codespace_name}
---

# Codespace SSH

## As a

developer who wants to work in a codespace from their local terminal

## I want

to open an SSH session into a codespace

## Acceptance criteria

1. Running `gor codespace ssh my-codespace` opens an SSH connection to the named codespace
2. The SSH connection uses the codespace's `remote_user` and `ssh_url`
3. `--profile` flag selects a named SSH config profile
4. `--config` flag prints the SSH connection config instead of connecting
5. `--repo` / `-R` flag resolves the codespace by repository
6. `--debug` / `-d` flag enables debug logging for the SSH connection
7. Only available on github.com — a clear error is shown when used on GHES
8. Exit code reflects the SSH session exit code

## Out of scope

- File transfer to/from the codespace (use `scp` with `--config` output)
- Port forwarding setup
