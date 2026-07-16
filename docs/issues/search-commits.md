---
tags: [search, read]
priority: P2
phase: 2
endpoints:
  - GET /search/commits
status: todo
blockedBy: [auth-login]
blocks: []
---

# Search Commits

## As a

developer tracking down when a change was made

## I want

to search commit history across GitHub repositories

## Acceptance criteria

1. Running `gor search commits "fix null pointer"` returns matching commits
2. Each row shows: commit SHA (short), commit message, author, repository, and commit date
3. `--author` flag filters by commit author (login or full name)
4. `--committer` flag filters by committer (login or full name)
5. `--repo` flag scopes search to a specific repository
6. `--org` flag scopes search to all repositories in a specific organization
7. `--sort` flag sorts by `author-date` or `committer-date` (default: `committer-date`)
8. `--order` flag sets sort direction (`asc` or `desc`, default: `desc`)
9. `--limit` / `-L` flag caps results (default: 30)
10. `--json` flag outputs as JSON with optional field selection
11. `--web` / `-w` flag opens the search results in the browser
12. `--hostname` flag targets a specific host

## Out of scope

- Searching by file path only (use query qualifier `path:`)
- Commit comment search (use query qualifier `comment:`)
- Date-range filtering (use query qualifiers `author-date:` or `committer-date:`)
