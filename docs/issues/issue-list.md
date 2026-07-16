---
tags: [issue, read]
priority: P0
phase: 0
endpoints:
  - GET /repos/{owner}/{repo}/issues
status: todo
blockedBy: [repo-view]
blocks: [issue-view, issue-create]
---

# Issue List

## As a

developer triaging work in a repository

## I want

to list issues with filtering

## Acceptance criteria

1. Running `gor issue list` in a repo directory lists open issues
2. Each row shows: number, title, author, labels, state
3. `--repo` / `-R` flag specifies the repository explicitly
4. `--state` flag filters by `open`, `closed`, or `all` (default: `open`)
5. `--label` flag filters by label (repeatable)
6. `--assignee` flag filters by assignee
7. `--author` flag filters by issue author
8. `--mention` flag filters by @mention
9. `--milestone` flag filters by milestone
10. `--limit` / `-L` flag caps results (default: 30)
11. `--web` / `-w` flag opens the issue list in the browser
12. `--json` flag outputs as JSON with optional field selection
13. `--hostname` flag targets a specific host

## Out of scope

- Pull requests appearing in the issue list (this is GitHub API behavior; use `--json` + `--jq` to filter)
- Sorting beyond what the API provides
