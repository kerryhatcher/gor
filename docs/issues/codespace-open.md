---
tags: [codespace, read]
priority: P3
phase: 4
endpoints: []
status: todo
blockedBy: [codespace-list]
blocks: []
---

# Codespace Open

## As a

developer working with codespaces

## I want

to open a codespace in the browser or VS Code

## Acceptance criteria

1. Running `gor codespace open <name>` opens the codespace in the default browser
2. Running `gor codespace open <name> --vscode` opens the codespace in VS Code (via `vscode://` protocol)
3. Running `gor codespace open <name> --jupyter` opens the codespace in JupyterLab
4. The codespace name is a required positional argument
5. `--hostname` flag targets a specific host
6. If the codespace is not running, the command exits non-zero with a clear error
7. If the target application (VS Code, Jupyter) is not installed, a clear error message is shown

## Out of scope

- Opening a codespace in other editors
- SSH connection (see `gor codespace ssh`)
