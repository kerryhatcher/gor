---
tags: [repo, read]
priority: P0
phase: 0
endpoints:
  - GET /user/repos
  - GET /users/{username}/repos
  - GET /orgs/{org}/repos
---

# Repo List

## As a

developer browsing repositories

## I want

to list repositories owned by a user or organization

## Acceptance criteria

1. Running `gor repo list` lists the authenticated user's repositories
2. Each row shows: name, description (truncated), visibility, last-updated
3. `--limit` / `-L` flag caps the number of results (default: 30)
4. `--visibility` flag filters by `public`, `private`, or `all`
5. `--owner` flag lists repos for a specific user or org
6. `--fork` flag includes or excludes forks
7. `--language` flag filters by primary language
8. `--json` flag outputs as JSON with optional field selection
9. `--hostname` flag targets a specific host
10. Pagination is transparent — `--limit 200` fetches across multiple pages

## Out of scope

- Sorting by stars, forks, or other metrics beyond what the API provides
- Topic-based filtering
