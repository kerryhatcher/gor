---
tags: [issue, write]
priority: P0
phase: 0
endpoints:
  - POST /repos/{owner}/{repo}/issues/{number}/comments
status: todo
blockedBy: [issue-view]
blocks: []
---

# Issue Comment

## As a

developer participating in an issue discussion

## I want

to add a comment to an existing issue

## Acceptance criteria

1. Running `gor issue comment 42 --body "Looks good to me"` posts a comment to issue #42
2. `--body` flag sets the comment text (markdown supported)
3. `--body-file` flag reads the comment body from a file (`@-` for stdin)
4. `--repo` / `-R` flag specifies the repository explicitly
5. `--hostname` flag targets a specific host
6. `--web` / `-w` flag opens the issue in the browser after commenting
7. The comment URL is printed on success

## Out of scope

- Editing or deleting existing comments
- Threaded replies to a specific comment
