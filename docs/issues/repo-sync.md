---
tags: [repo, git, write]
priority: P1
phase: 1
endpoints:
  - POST /repos/{owner}/{repo}/merge-upstream
status: todo
blockedBy: [repo-view]
blocks: []
---

# Repo Sync

## As a

developer maintaining a fork of an upstream project

## I want

to sync my fork with its upstream repository

## Acceptance criteria

1. Running `gor repo sync owner/repo` syncs the fork's default branch from its upstream
2. `--repo` / `-R` flag specifies the repository explicitly
3. `--hostname` flag targets a specific host
4. `--branch` flag specifies the branch to sync into (default: the fork's default branch)
5. A success message is printed showing the new merge commit SHA after a successful sync
6. If the fork is already up to date with upstream, a message indicating no sync was needed is printed and the command exits 0
7. If the sync would result in merge conflicts, an error is shown with a clear message and a non-zero exit code
8. If the repository is not a fork (no upstream configured), an error is shown with a clear message and a non-zero exit code

## Out of scope

- Two-way syncing (pushing local commits back to upstream)
- Syncing arbitrary branches between unrelated repositories
- Interactive conflict resolution
