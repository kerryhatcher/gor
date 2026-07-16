---
tags: [codespace, write]
priority: P3
phase: 4
endpoints:
  - PATCH /user/codespaces/{codespace_name}
status: todo
blockedBy: [codespace-list]
blocks: []
---

# Codespace Edit

## As a

developer adjusting codespace settings

## I want

to modify a codespace's configuration

## Acceptance criteria

1. Running `gor codespace edit <name> --machine standardLinux32gb` changes the machine type
2. `--machine` / `-m` flag sets the machine type (e.g., `basicLinux32gb`, `standardLinux32gb`)
3. `--idle-timeout` flag sets the idle timeout in minutes before auto-shutdown
4. `--display-name` flag sets a friendly display name for the codespace
5. `--hostname` flag targets a specific host
6. A success message is printed confirming the changes
7. If the codespace is not running, the command exits non-zero with a clear error

## Out of scope

- Changing the devcontainer configuration (requires rebuilding)
- Changing the repository or branch
