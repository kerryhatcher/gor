---
tags: [extension, read]
priority: P2
phase: 3
endpoints: []
status: todo
blockedBy: [auth-login]
blocks: [extension-remove, extension-upgrade]
---

# Extension List

## As a

developer who has installed `gor` extensions

## I want

to see which extensions are installed and where they came from

## Acceptance criteria

1. Running `gor extension list` prints all installed extensions
2. Each row shows: extension name, repository URL, and installed version/tag
3. `--hostname` flag scopes to extensions installed for a specific host
4. `--json` flag outputs as JSON with optional field selection
5. If no extensions are installed, a message is printed and the command exits 0

## Out of scope

- Installing or updating extensions (separate stories)
- Browsing the extension marketplace
