---
tags: [codespace, read]
priority: P3
phase: 4
endpoints:
  - GET /user/codespaces/{codespace_name}/logs
---

# Codespace Logs

## As a

developer debugging a codespace setup failure

## I want

to view the lifecycle logs of a codespace

## Acceptance criteria

1. Running `gor codespace logs my-codespace` streams the codespace's creation/setup logs
2. `--repo` / `-R` flag resolves the codespace by repository
3. `--follow` / `-f` flag tails the logs until the codespace is ready
4. Only available on github.com — a clear error is shown when used on GHES
5. Logs are streamed to stdout
6. Exit code 0 on success

## Out of scope

- Application runtime logs inside the codespace
- SSH session logs
