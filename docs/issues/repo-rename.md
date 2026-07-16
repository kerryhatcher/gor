---
tags: [repo, write]
priority: P2
phase: 4
endpoints:
  - PATCH /repos/{owner}/{repo}
status: todo
blockedBy: [repo-view]
blocks: []
---

# Repo Rename

## As a

developer who needs to rename a repository

## I want

to rename a repository I own

## Acceptance criteria

1. Running `gor repo rename owner/repo --name new-name` renames the repository
2. `--name` / `-n` flag specifies the new repository name (required)
3. `--repo` / `-R` flag specifies the repository explicitly (auto-detected from git remote if omitted)
4. `--hostname` flag targets a specific host
5. A success message is printed with the new full name
6. If the new name is already taken, the command exits non-zero with a clear error
7. If the repository does not exist, the command exits non-zero with a clear error

## Out of scope

- Updating local git remotes after rename
- Transferring a repository (see repo-transfer.md)
