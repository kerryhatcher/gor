---
tags: [alias, read]
priority: P2
phase: 3
endpoints: []
status: done
blockedBy: [auth-login]
blocks: [alias-delete]
---

# Alias List

## As a

developer who wants to recall defined aliases

## I want

to list all configured `gor` aliases

## Acceptance criteria

1. Running `gor alias list` prints all defined aliases
2. Each row shows: alias name and the command it expands to
3. `--hostname` flag scopes the listing to a specific host
4. `--json` flag outputs as JSON with optional field selection
5. If no aliases exist, a message is printed and the command exits 0

## Out of scope

- Editing aliases in place (use `gor alias set` / `gor alias delete`)
- Showing built-in command help
