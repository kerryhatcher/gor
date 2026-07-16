---
tags: [repo, read]
priority: P0
phase: 0
endpoints:
  - GET /repos/{owner}/{repo}
---

# Repo View

## As a

developer exploring a repository

## I want

to see a repository's description, stats, and metadata

## Acceptance criteria

1. Running `gor repo view owner/repo` displays the repository name, description, and URL
2. Star count, fork count, and open issue count are shown
3. Primary language, license, and last-pushed date are shown
4. Default branch is displayed
5. Repository visibility (public/private) is shown
6. `--web` / `-w` flag opens the repository in the default browser
7. `--json` flag outputs the full API response as JSON
8. `--json field1,field2` outputs only the specified fields
9. `--hostname` flag targets a specific host
10. If no `owner/repo` is given, auto-detects from the current directory's git remote

## Out of scope

- README rendering
- File tree browsing
- Language breakdown percentages
