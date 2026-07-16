---
tags: [codespace, read]
priority: P3
phase: 4
endpoints:
  - GET /user/codespaces/{codespace_name}
status: todo
blockedBy: [codespace-list]
blocks: []
---

# Codespace View

## As a

developer inspecting a codespace

## I want

to view detailed information about a codespace

## Acceptance criteria

1. Running `gor codespace view <name>` shows detailed information about the codespace
2. The output includes: name, state, repository, branch, machine type, idle timeout, and creation date
3. `--json` flag outputs as JSON with optional field selection
4. `--hostname` flag targets a specific host
5. If the codespace does not exist, the command exits non-zero with a clear error

## Out of scope

- Viewing codespace resource usage (CPU, memory, disk)
- Viewing codespace environment variables
