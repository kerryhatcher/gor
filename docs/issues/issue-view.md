---
tags: [issue, read]
priority: P0
phase: 0
endpoints:
  - GET /repos/{owner}/{repo}/issues/{number}
---

# Issue View

## As a

developer looking at a specific issue

## I want

to see the full details of an issue including its comment thread

## Acceptance criteria

1. Running `gor issue view 42` in a repo directory shows issue #42
2. Title, body, author, state, labels, assignees, and milestone are displayed
3. `--repo` / `-R` flag specifies the repository explicitly
4. `--web` / `-w` flag opens the issue in the browser
5. `--comments` flag includes the issue's comment thread
6. `--json` flag outputs as JSON with optional field selection
7. `--hostname` flag targets a specific host

## Out of scope

- Issue timeline events (cross-references, mentions, etc.)
- Reaction display
