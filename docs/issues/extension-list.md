---
tags: [extension, read]
priority: P2
phase: 3
endpoints: []
---

# Extension List

## As a

developer who extends `gor` with third-party plugins

## I want

to see which extensions are installed

## Acceptance criteria

1. Running `gor extension list` shows all installed extensions
2. Each row shows: extension name, repository URL, version
3. `--hostname` flag lists extensions for a specific host
4. `--json` flag outputs as JSON
5. If no extensions are installed, a message indicates none are installed
6. Exit code 0 on success

## Out of scope

- Installing new extensions (separate story)
- Checking for updates
