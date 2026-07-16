---
tags: [issue, write]
priority: P2
phase: 4
endpoints:
  - PUT /repos/{owner}/{repo}/issues/{number}/lock
  - DELETE /repos/{owner}/{repo}/issues/{number}/lock
status: todo
blockedBy: [issue-view]
blocks: []
---

# Issue Lock

## As a

maintainer managing heated discussions

## I want

to lock or unlock an issue conversation

## Acceptance criteria

1. Running `gor issue lock 42` locks the conversation on issue #42
2. Running `gor issue unlock 42` unlocks a previously locked issue conversation
3. The issue number is a required positional argument
4. `--lock-reason` flag specifies the reason: `off-topic`, `too-heated`, `resolved`, or `spam`
5. `--repo` / `-R` flag specifies the repository explicitly
6. `--hostname` flag targets a specific host
7. A success message is printed confirming the lock/unlock operation
8. If the issue is already in the target state, the command exits gracefully with a clear message
9. If the issue does not exist, the command exits non-zero with a clear error

## Out of scope

- Locking multiple issues at once
- Viewing lock status (use `gor issue view`)
