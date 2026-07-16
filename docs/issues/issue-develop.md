---
tags: [issue, write]
priority: P3
phase: 4
endpoints:
  - GET /repos/{owner}/{repo}/issues/{number}/branches
  - POST /repos/{owner}/{repo}/issues/{number}/branches
status: todo
blockedBy: [issue-view]
blocks: []
---

# Issue Develop

## As a

developer starting work on an issue

## I want

to create and manage development branches linked to an issue

## Acceptance criteria

1. Running `gor issue develop 42` creates a feature branch linked to issue #42
2. The branch name is auto-generated from the issue title (e.g., `42-fix-login-bug`)
3. `--name` / `-n` flag overrides the auto-generated branch name
4. `--base` / `-b` flag specifies the base branch (default: repository default branch)
5. `--checkout` / `-c` flag checks out the new branch after creation
6. Running `gor issue develop 42 --list` lists branches linked to the issue
7. `--repo` / `-R` flag specifies the repository explicitly
8. `--hostname` flag targets a specific host
9. If the issue does not exist, the command exits non-zero with a clear error

## Out of scope

- Deleting linked branches
- Creating PRs from linked branches (use `gor pr create`)
