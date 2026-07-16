---
tags: [pr, write]
priority: P2
phase: 4
endpoints:
  - PUT /repos/{owner}/{repo}/pulls/{number}/lock
  - DELETE /repos/{owner}/{repo}/pulls/{number}/lock
status: todo
blockedBy: [pr-view]
blocks: []
---

# PR Lock

## As a

maintainer managing heated discussions

## I want

to lock or unlock a pull request conversation

## Acceptance criteria

1. Running `gor pr lock 42` locks the conversation on PR #42
2. Running `gor pr unlock 42` unlocks a previously locked PR conversation
3. The PR number is a required positional argument
4. `--lock-reason` flag specifies the reason: `off-topic`, `too-heated`, `resolved`, or `spam`
5. `--repo` / `-R` flag specifies the repository explicitly
6. `--hostname` flag targets a specific host
7. A success message is printed confirming the lock/unlock operation
8. If the PR is already in the target state, the command exits gracefully with a clear message
9. If the PR does not exist, the command exits non-zero with a clear error

## Out of scope

- Locking multiple PRs at once
- Viewing lock status (use `gor pr view`)
