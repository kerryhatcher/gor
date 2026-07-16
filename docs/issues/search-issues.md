---
tags: [search, read]
priority: P2
phase: 2
endpoints:
  - GET /search/issues
status: done
blockedBy: [auth-login]
blocks: []
---

# Search Issues & PRs

## As a

developer looking for specific issues or pull requests

## I want

to search across repositories for issues and PRs matching my criteria

## Acceptance criteria

1. Running `gor search issues "bug crash"` returns matching issues and PRs
2. Each row shows: type (issue/PR), number, title, repository, author, state, labels
3. `--type` flag filters by `issue`, `pr`, or `both` (default: `both`)
4. `--state` flag filters by `open` or `closed`
5. `--label` flag filters by label
6. `--author` flag filters by author
7. `--assignee` flag filters by assignee
8. `--mention` flag filters by @mention
9. `--repo` flag scopes search to a specific repository
10. `--org` flag scopes search to a specific organization
11. `--sort` flag sorts by `comments`, `reactions`, `created`, `updated`, or `best-match`
12. `--limit` / `-L` flag caps results (default: 30)
13. `--json` flag outputs as JSON with optional field selection
14. `--web` / `-w` flag opens the search results in the browser
15. `--hostname` flag targets a specific host

## Out of scope

- Search within issue/PR bodies only (use query qualifier `in:body`)
- Date-range filtering (use query qualifiers `created:`, `updated:`)
