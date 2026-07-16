---
tags: [issue, write]
priority: P0
phase: 0
endpoints:
  - POST /repos/{owner}/{repo}/issues
status: todo
blockedBy: [issue-list]
blocks: []
---

# Issue Create

## As a

developer reporting a bug or requesting a feature

## I want

to create a new issue in a repository

## Acceptance criteria

1. Running `gor issue create` in a repo directory creates a new issue
2. `--title` flag sets the issue title (required)
3. `--body` flag sets the issue body (markdown)
4. `--repo` / `-R` flag specifies the repository explicitly
5. `--label` flag adds labels (repeatable)
6. `--assignee` flag assigns users (repeatable)
7. `--milestone` flag sets the milestone
8. `--project` flag adds to a project board
9. `--web` / `-w` flag opens the new issue in the browser
10. The issue URL and number are printed on success
11. `--hostname` flag targets a specific host

## Out of scope

- Issue templates
- Auto-filling from a template file
