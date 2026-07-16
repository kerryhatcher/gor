---
tags: [pr, read]
priority: P0
phase: 0
endpoints:
  - GET /repos/{owner}/{repo}/pulls
status: todo
blockedBy: [repo-view]
blocks: [pr-view, pr-create, pr-checkout]
---

# PR List

## As a

developer reviewing open work in a repository

## I want

to list pull requests with filtering and status information

## Acceptance criteria

1. Running `gor pr list` in a repo directory lists open pull requests
2. Each row shows: number, title, author, branch names, labels, state
3. `--repo` / `-R` flag specifies the repository explicitly
4. `--state` flag filters by `open`, `closed`, `merged`, or `all` (default: `open`)
5. `--base` flag filters by base branch
6. `--head` flag filters by head branch (useful for finding your own PRs)
7. `--author` flag filters by PR author
8. `--label` flag filters by label (repeatable)
9. `--assignee` flag filters by assignee
10. `--limit` / `-L` flag caps results (default: 30)
11. `--web` / `-w` flag opens the PR list in the browser
12. `--json` flag outputs as JSON with optional field selection
13. `--hostname` flag targets a specific host

## Out of scope

- Draft PR filtering (use `--state` with `--json` + `--jq`)
- Milestone filtering
