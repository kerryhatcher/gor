---
tags: [search, read]
priority: P2
phase: 2
endpoints:
  - GET /search/repositories
status: done
blockedBy: [auth-login]
blocks: []
---

# Search Repos

## As a

developer looking for projects on GitHub

## I want

to search for repositories by name, language, topic, or other criteria

## Acceptance criteria

1. Running `gor search repos "rust cli"` returns matching repositories
2. Each row shows: full name, description, stars, language, last-updated
3. `--language` flag filters by primary language
4. `--topic` flag filters by repository topic
5. `--stars` flag filters by star count range (e.g., `">100"`, `"10..50"`)
6. `--followers` flag filters by follower count range
7. `--created` flag filters by creation date range
8. `--updated` flag filters by last-updated date range
9. `--archived` flag includes or excludes archived repos
10. `--sort` flag sorts by `stars`, `forks`, `updated`, or `best-match`
11. `--order` flag sets sort direction (`asc` or `desc`)
12. `--limit` / `-L` flag caps results (default: 30)
13. `--json` flag outputs as JSON with optional field selection
14. `--web` / `-w` flag opens the search results in the browser
15. `--hostname` flag targets a specific host

## Out of scope

- Saved searches
- Search within a specific organization only (use query qualifier `org:`)
