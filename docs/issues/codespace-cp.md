---
tags: [codespace, write]
priority: P2
phase: 4
endpoints:
  - POST /user/codespaces/{codespace_name}/exports
status: todo
blockedBy: [codespace-list]
blocks: []
---

# Codespace CP

## As a

developer who needs to move files between my local machine and a codespace

## I want

to copy files to and from a GitHub Codespace

## Acceptance criteria

1. Running `gor codespace cp src/file.txt remote:/workspace/` copies a local file into the codespace
2. Running `gor codespace cp remote:/workspace/output.txt .` copies a file from the codespace locally
3. The codespace is identified by name (first positional argument)
4. Source and destination paths use `remote:` prefix for codespace paths
5. `--recursive` / `-r` flag copies directories recursively
6. `--hostname` flag targets a specific host (codespaces only on github.com)
7. A progress indicator is shown during transfer
8. The command exits non-zero if the codespace is not running

## Out of scope

- Streaming file contents to stdout
- Copying between two codespaces
