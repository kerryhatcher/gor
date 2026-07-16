---
tags: [codespace, write]
priority: P3
phase: 4
endpoints:
  - POST /user/codespaces/{codespace_name}/stop
---

# Codespace Stop

## As a

developer who wants to pause a codespace to save costs

## I want

to stop a running codespace

## Acceptance criteria

1. Running `gor codespace stop my-codespace` stops the named codespace
2. `--repo` / `-R` flag resolves the codespace by repository
3. Multiple codespace names may be passed to stop several at once
4. A confirmation message is printed on success
5. Only available on github.com — a clear error is shown when used on GHES
6. Exit code 0 on success

## Out of scope

- Deleting the codespace (separate story)
- Stopping all codespaces at once (pass multiple names)
