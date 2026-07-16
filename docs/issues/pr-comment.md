---
tags: [pr, write]
priority: P0
phase: 0
endpoints:
  - POST /repos/{owner}/{repo}/issues/{number}/comments
status: done
blockedBy: [pr-view]
blocks: []
---

# PR Comment

## As a

developer reviewing a pull request

## I want

to add a comment to a pull request's conversation thread

## Acceptance criteria

1. Running `gor pr comment 42 --body "Looks good!"` adds a comment to PR #42
2. The PR is identified by its number passed as a positional argument
3. `--body` flag sets the comment text
4. `--body-file` flag reads the comment text from a file
5. `--repo` / `-R` flag specifies the repository explicitly
6. `--hostname` flag targets a specific host
7. `--web` / `-w` flag opens the new comment in the browser
8. The comment URL is printed on success

## Out of scope

- Inline review comments on specific lines of code (use `gor pr review`)
- Editing or deleting existing comments
