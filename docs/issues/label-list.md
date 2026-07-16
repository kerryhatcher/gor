---
tags: [label, read]
priority: P1
phase: 1
endpoints:
  - GET /repos/{owner}/{repo}/labels
status: done
blockedBy: [repo-view]
blocks: [label-edit, label-delete, label-clone]
---

# Label List

## As a

developer triaging issues and pull requests

## I want

to list all labels in a repository

## Acceptance criteria

1. Running `gor label list` in a repo directory lists all labels
2. Each row shows: name, color (as a swatch), description
3. `--repo` / `-R` flag specifies the repository explicitly
4. `--search` flag filters labels by name substring
5. `--limit` / `-L` flag caps results (default: 30)
6. `--json` flag outputs as JSON with optional field selection
7. `--hostname` flag targets a specific host

## Out of scope

- Sorting by issue/PR count
- Label usage statistics
