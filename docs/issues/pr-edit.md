---
tags: [pr, write]
priority: P1
phase: 1
endpoints:
  - PATCH /repos/{owner}/{repo}/pulls/{number}
status: done
blockedBy: [pr-view]
blocks: []
---

# PR Edit

## As a

developer who needs to update a pull request's metadata after it has been opened

## I want

to edit a pull request's title, body, base branch, labels, assignees, or milestone

## Acceptance criteria

1. Running `gor pr edit 42` edits PR #42
2. `--title` flag updates the pull request title
3. `--body` flag updates the pull request body
4. `--base` flag changes the base branch the pull request targets
5. `--add-label` flag adds labels (repeatable)
6. `--remove-label` flag removes labels (repeatable)
7. `--add-assignee` flag adds assignees (repeatable)
8. `--remove-assignee` flag removes assignees (repeatable)
9. `--milestone` flag sets the milestone
10. `--repo` / `-R` flag specifies the repository explicitly
11. `--hostname` flag targets a specific host
12. The updated pull request's number, title, base branch, labels, assignees, and milestone are printed on success

## Out of scope

- Editing the PR's head branch (a PR's source branch cannot be changed via the API)
- Adding or removing reviewers (see the separate PR review story)
- Closing or reopening the PR (see the separate PR close/reopen story)
