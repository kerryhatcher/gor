---
tags: [browse]
priority: P2
phase: 2
endpoints: []
---

# Browse

## As a

developer who wants to quickly open a GitHub page

## I want

to open a repository, issue, PR, or other resource in my default browser

## Acceptance criteria

1. Running `gor browse` in a repo directory opens the repository in the browser
2. `--repo` / `-R` flag specifies the repository explicitly
3. `--branch` / `-b` flag opens a specific branch or tree
4. `--commit` / `-c` flag opens a specific commit
5. `--projects` flag opens the projects tab
6. `--wiki` flag opens the wiki
7. `--settings` flag opens the settings page
8. `--issue` / `-i` flag opens a specific issue by number
9. `--pr` / `-p` flag opens a specific pull request by number
10. `--hostname` flag targets a specific host
11. The `$BROWSER` environment variable is respected; falls back to OS default

## Out of scope

- Opening multiple pages at once
- Opening specific files at a specific line number
