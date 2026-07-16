---
tags: [pr, write]
priority: P0
phase: 0
endpoints:
  - POST /repos/{owner}/{repo}/pulls
---

# PR Create

## As a

developer who has finished work on a feature branch

## I want

to create a pull request from my branch to the base branch

## Acceptance criteria

1. Running `gor pr create` in a repo directory creates a PR from the current branch
2. The base branch is auto-detected (default branch of the repository)
3. `--title` flag sets the PR title
4. `--body` flag sets the PR body (markdown)
5. `--base` flag overrides the base branch
6. `--head` flag overrides the head branch
7. `--repo` / `-R` flag specifies the repository explicitly
8. `--draft` flag creates the PR as a draft
9. `--label` flag adds labels (repeatable)
10. `--assignee` flag assigns reviewers (repeatable)
11. `--milestone` flag sets the milestone
12. `--project` flag adds to a project board
13. `--web` / `-w` flag opens the new PR in the browser
14. The PR URL and number are printed on success
15. `--hostname` flag targets a specific host

## Out of scope

- Auto-filling the PR body from a template
- Interactive editor for the PR body
