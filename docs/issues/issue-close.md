---
tags: [issue, write]
priority: P0
phase: 0
endpoints:
  - PATCH /repos/{owner}/{repo}/issues/{number}
status: done
blockedBy: [issue-view]
blocks: []
---

# Issue Close / Reopen

## As a

developer managing issue lifecycle

## I want

to close or reopen an issue

## Acceptance criteria

1. Running `gor issue close 42` closes issue #42
2. Running `gor issue reopen 42` reopens a closed issue #42
3. `--repo` / `-R` flag specifies the repository explicitly
4. `--comment` flag adds a closing comment
5. `--reason` flag sets the close reason (`completed` or `not_planned`)
6. A confirmation message with the new state is printed
7. `--hostname` flag targets a specific host

## Out of scope

- Bulk close/reopen
- Closing as duplicate with a reference
