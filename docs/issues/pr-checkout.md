---
tags: [pr, git]
priority: P0
phase: 0
endpoints:
  - GET /repos/{owner}/{repo}/pulls/{number}
status: todo
blockedBy: [pr-list]
blocks: []
---

# PR Checkout

## As a

developer who wants to test a pull request locally

## I want

to check out the PR's head branch into my local repository

## Acceptance criteria

1. Running `gor pr checkout 42` fetches and checks out the PR's head branch
2. The remote is added if not already present
3. `--repo` / `-R` flag specifies the repository explicitly
4. `--branch` / `-b` flag sets a custom local branch name
5. `--recurse-submodules` flag initializes and updates submodules
6. `--hostname` flag targets a specific host
7. A message confirms the branch that was checked out

## Out of scope

- Checking out a PR from a fork that has been deleted
- Detached HEAD checkout
