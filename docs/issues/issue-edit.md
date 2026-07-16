---
tags: [issue, write]
priority: P1
phase: 1
endpoints:
  - PATCH /repos/{owner}/{repo}/issues/{number}
---

# Issue Edit

## As a

developer who needs to update issue details

## I want

to edit an issue's title, body, labels, assignees, or milestone

## Acceptance criteria

1. Running `gor issue edit 42` edits issue #42
2. `--title` flag updates the issue title
3. `--body` flag updates the issue body
4. `--add-label` flag adds labels (repeatable)
5. `--remove-label` flag removes labels (repeatable)
6. `--add-assignee` flag adds assignees (repeatable)
7. `--remove-assignee` flag removes assignees (repeatable)
8. `--milestone` flag sets the milestone
9. `--repo` / `-R` flag specifies the repository explicitly
10. `--hostname` flag targets a specific host
11. A confirmation message is printed on success

## Out of scope

- Editing comments on an issue
- Bulk editing multiple issues
