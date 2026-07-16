---
tags: [search, read]
priority: P2
phase: 2
endpoints:
  - GET /search/code
status: todo
blockedBy: [auth-login]
blocks: []
---

# Search Code

## As a

developer looking for code examples or usages

## I want

to search for code across GitHub repositories

## Acceptance criteria

1. Running `gor search code "fn main"` returns matching code results
2. Each row shows: file path, repository, and a snippet with the match
3. `--language` flag filters by programming language
4. `--repo` flag scopes search to a specific repository
5. `--org` flag scopes search to a specific organization
6. `--owner` flag scopes search to a specific user's repositories
7. `--path` flag scopes search to a specific path prefix
8. `--extension` flag filters by file extension
9. `--filename` flag searches only in files matching the given name
10. `--sort` flag sorts by `indexed` or `best-match` (default)
11. `--limit` / `-L` flag caps results (default: 30)
12. `--json` flag outputs as JSON with optional field selection
13. `--web` / `-w` flag opens the search results in the browser
14. `--hostname` flag targets a specific host

## Out of scope

- Search within forks (use query qualifier `fork:true`)
- Search by file size (use query qualifier `size:`)
