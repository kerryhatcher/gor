---
tags: [pr, read]
priority: P0
phase: 0
endpoints:
  - GET /repos/{owner}/{repo}/pulls/{number}
status: todo
blockedBy: [pr-list]
blocks: [pr-close, pr-comment, pr-merge, pr-diff, pr-review, pr-checks, pr-edit]
---

# PR View

## As a

developer reviewing a specific pull request

## I want

to see the full details of a pull request

## Acceptance criteria

1. Running `gor pr view 42` in a repo directory shows PR #42
2. Title, body, author, state, branch names, and labels are displayed
3. Review status (approved, changes requested, commented) is shown
4. Merge status (mergeable, conflicts, merged) is shown
5. CI check status summary is shown
6. `--repo` / `-R` flag specifies the repository explicitly
7. `--web` / `-w` flag opens the PR in the browser
8. `--comments` flag includes the PR's comment thread
9. `--json` flag outputs as JSON with optional field selection
10. `--hostname` flag targets a specific host

## Out of scope

- Inline diff viewing (use `gor pr diff`)
- Review comment threading
