---
tags: [pr, write]
priority: P2
phase: 4
endpoints:
  - PATCH /repos/{owner}/{repo}/pulls/{number}
status: todo
blockedBy: [pr-view]
blocks: []
---

# PR Ready

## As a

developer who has finished work on a draft pull request

## I want

to mark a draft PR as ready for review

## Acceptance criteria

1. Running `gor pr ready 42` marks PR #42 as ready for review
2. The PR number is a required positional argument
3. `--repo` / `-R` flag specifies the repository explicitly
4. `--hostname` flag targets a specific host
5. A success message is printed confirming the PR is now ready for review
6. If the PR is already ready (not a draft), the command exits gracefully with a clear message
7. If the PR does not exist, the command exits non-zero with a clear error

## Out of scope

- Converting a ready PR back to draft
- Bulk marking multiple PRs as ready
