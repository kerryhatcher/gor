---
tags: [codespace, write]
priority: P2
phase: 4
endpoints:
  - POST /user/codespaces/{codespace_name}/rebuild
status: done
blockedBy: [codespace-list]
blocks: []
---

# Codespace Rebuild

## As a

developer who needs a fresh dev environment after configuration changes

## I want

to rebuild a GitHub Codespace from its devcontainer configuration

## Acceptance criteria

1. Running `gor codespace rebuild <name>` triggers a full rebuild of the named codespace
2. The codespace name is a required positional argument
3. `--hostname` flag targets a specific host (codespaces only on github.com)
4. A confirmation prompt is shown before rebuild (bypassed with `--yes`)
5. A success message is printed with the rebuild status
6. The command exits non-zero if the codespace is not running or does not exist

## Out of scope

- Watching the rebuild until completion
- Rebuilding with a different machine type or devcontainer config
