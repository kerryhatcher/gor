---
tags: [pr, write]
priority: P1
phase: 1
endpoints:
  - POST /repos/{owner}/{repo}/pulls/{number}/reviews
status: todo
blockedBy: [pr-view]
blocks: []
---

# PR Review

## As a

developer reviewing a pull request

## I want

to submit an approving review, request changes, or leave a comment

## Acceptance criteria

1. Running `gor pr review 42` submits a review for PR #42
2. `--approve` flag submits an approving review
3. `--request-changes` flag requests changes
4. `--comment` flag leaves a general comment (default if no flag given)
5. `--body` flag sets the review body text
6. `--repo` / `-R` flag specifies the repository explicitly
7. `--hostname` flag targets a specific host
8. A confirmation message with the review state is printed

## Out of scope

- Inline line-specific review comments
- Review dismissal
- Re-requesting reviews from specific users
