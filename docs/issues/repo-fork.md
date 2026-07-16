---
tags: [repo, write]
priority: P1
phase: 1
endpoints:
  - POST /repos/{owner}/{repo}/forks
status: done
blockedBy: [repo-view]
blocks: []
---

# Repo Fork

## As a

developer who wants to contribute to an upstream project

## I want

to fork a repository to my account

## Acceptance criteria

1. Running `gor repo fork owner/repo` creates a fork under the authenticated user
2. `--org` flag forks into an organization instead of the user account
3. `--clone` flag clones the fork locally after creation
4. `--remote` flag sets the remote name (default: `origin`)
5. `--fork-name` flag sets a custom name for the fork
6. If the fork already exists, a message is shown and the command succeeds
7. `--hostname` flag targets a specific host
8. The fork's URL is printed on success

## Out of scope

- Syncing a fork from upstream (separate story)
- Forking into a different GHES instance
