---
tags: [gist, read]
priority: P2
phase: 2
endpoints:
  - GET /gists/{id}
status: done
blockedBy: [gist-list]
blocks: [gist-edit, gist-delete]
---

# Gist View

## As a

developer reading a shared code snippet

## I want

to view a gist's content and metadata

## Acceptance criteria

1. Running `gor gist view abc123def456` shows the gist's description and files
2. Each file's name, language, and content are displayed
3. `--raw` flag outputs the raw content of a specific file
4. `--filename` flag selects a specific file to view
5. `--web` / `-w` flag opens the gist in the browser
6. `--json` flag outputs as JSON with optional field selection
7. `--hostname` flag targets a specific host

## Out of scope

- Gist revision history
- Gist comments
