---
tags: [issue, write]
priority: P2
phase: 4
endpoints:
  - PUT /repos/{owner}/{repo}/issues/{number}/pin
  - DELETE /repos/{owner}/{repo}/issues/{number}/pin
status: todo
blockedBy: [issue-view]
blocks: []
---

# Issue Pin

## As a

maintainer highlighting important issues

## I want

to pin or unpin an issue in a repository

## Acceptance criteria

1. Running `gor issue pin 42` pins issue #42 to the top of the issue list
2. Running `gor issue unpin 42` unpins a previously pinned issue
3. The issue number is a required positional argument
4. `--repo` / `-R` flag specifies the repository explicitly
5. `--hostname` flag targets a specific host
6. A success message is printed confirming the pin/unpin operation
7. If the issue is already in the target state, the command exits gracefully with a clear message
8. If the issue does not exist, the command exits non-zero with a clear error
9. A repository can have at most 3 pinned issues (GitHub limit)

## Out of scope

- Listing pinned issues (use `gor issue list --label pinned`)
- Pinning issues across multiple repositories
