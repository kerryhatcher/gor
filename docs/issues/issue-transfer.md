---
tags: [issue, write]
priority: P2
phase: 4
endpoints:
  - POST /repos/{owner}/{repo}/issues/{number}/transfer
status: todo
blockedBy: [issue-view]
blocks: []
---

# Issue Transfer

## As a

maintainer reorganizing issues between repositories

## I want

to transfer an issue to a different repository

## Acceptance criteria

1. Running `gor issue transfer 42 owner/other-repo` transfers issue #42 to `owner/other-repo`
2. The issue number is a required positional argument
3. The destination repository (OWNER/REPO format) is a required positional argument
4. `--repo` / `-R` flag specifies the source repository explicitly
5. `--hostname` flag targets a specific host
6. A success message is printed with the new issue URL
7. If the destination repository does not exist or is inaccessible, the command exits non-zero

## Out of scope

- Transferring issues between GitHub Enterprise Server instances
- Bulk issue transfer
