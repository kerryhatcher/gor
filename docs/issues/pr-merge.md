---
tags: [pr, write]
priority: P0
phase: 0
endpoints:
  - PUT /repos/{owner}/{repo}/pulls/{number}/merge
status: todo
blockedBy: [pr-view]
blocks: []
---

# PR Merge

## As a

developer who has an approved pull request

## I want

to merge it into the base branch

## Acceptance criteria

1. Running `gor pr merge 42` merges PR #42
2. `--repo` / `-R` flag specifies the repository explicitly
3. `--merge` flag uses a merge commit (default strategy)
4. `--squash` flag squashes all commits into one
5. `--rebase` flag rebases commits onto the base branch
6. `--body` flag sets the merge commit message body
7. `--subject` flag sets the merge commit message subject
8. `--delete-branch` flag deletes the head branch after merging
9. `--admin` flag uses admin privileges to bypass branch protection
10. `--auto` flag enables auto-merge (merge when all checks pass)
11. `--hostname` flag targets a specific host
12. A success message with the merge SHA is printed

## Out of scope

- Merge queue management
- Merge conflict resolution
