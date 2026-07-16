---
tags: [repo, write]
priority: P1
phase: 1
endpoints:
  - POST /repos/{owner}/{repo}/transfer
status: done
blockedBy: [repo-view]
blocks: []
---

# Repo Transfer

## As a

developer reorganizing repositories across accounts or organizations

## I want

to transfer a repository to another user or organization

## Acceptance criteria

1. Running `gor repo transfer target-org` transfers the current repository to the organization `target-org`
2. The destination is a positional argument (user login or org name)
3. `--repo` / `-R` flag specifies the repository explicitly
4. `--new-name` flag renames the repository as part of the transfer
5. `--team` flag grants a team access in the new organization (repeatable)
6. `--hostname` flag targets a specific host
7. A confirmation prompt is shown before transfer (bypassed with `--yes`)
8. A success message with the new repository URL is printed
9. If the destination lacks permission, a clear authorization error is shown and the command exits non-zero

## Out of scope

- Transferring multiple repositories at once
- Transferring to a different GitHub Enterprise Server instance
