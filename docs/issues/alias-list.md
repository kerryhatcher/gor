---
tags: [alias, read]
priority: P2
phase: 3
endpoints: []
---

# Alias List

## As a

developer who wants to review their configured shortcuts

## I want

to list all defined command aliases

## Acceptance criteria

1. Running `gor alias list` shows all defined aliases with their expansions
2. Each row shows: alias name, expansion
3. `--hostname` flag lists aliases for a specific host
4. `--json` flag outputs as JSON
5. If no aliases are defined, a message indicates no aliases exist (empty output)
6. Exit code 0 on success

## Out of scope

- Editing aliases from this view
- Showing which aliases are currently active in the shell
