---
tags: [project, write]
priority: P3
phase: 3
endpoints:
  - POST /projects/{project_id}/columns/{column_id}/cards
---

# Project Item Add

## As a

developer organizing work into a project board

## I want

to add an issue or pull request to a project column

## Acceptance criteria

1. Running `gor project item-add 5 --issue 42` adds issue #42 to project #5
2. `--issue` flag adds an issue by number
3. `--pull-request` flag adds a pull request by number
4. `--column` / `-c` flag selects the target column (defaults to first column)
5. `--owner` / `-o` flag specifies the project owner
6. `--repo` / `-R` flag specifies the repository for resolving issue/PR numbers
7. `--hostname` flag targets a specific host
8. A confirmation message with the card ID is printed on success
9. Exit code 0 on success

## Out of scope

- Reordering items within a column
- Adding notes (non-issue/PR cards)
