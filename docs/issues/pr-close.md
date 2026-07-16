---
tags: [pr, write]
priority: P0
phase: 0
endpoints:
  - PATCH /repos/{owner}/{repo}/pulls/{number}
status: done
blockedBy: [pr-view]
blocks: []
---

# PR Close / Reopen

## As a

developer managing pull request lifecycle

## I want

to close or reopen a pull request

## Acceptance criteria

1. Running `gor pr close 42` closes PR #42
2. Running `gor pr reopen 42` reopens a closed PR #42
3. `--repo` / `-R` flag specifies the repository explicitly
4. `--comment` flag adds a closing comment
5. A confirmation message with the new state is printed
6. `--hostname` flag targets a specific host

## Out of scope

- Closing with a "not planned" reason (GitHub-specific feature)
- Bulk close/reopen
