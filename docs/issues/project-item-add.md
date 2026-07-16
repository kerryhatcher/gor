---
tags: [project, write]
priority: P2
phase: 3
endpoints:
  - POST /projects/{project_id}/items
status: done
blockedBy: [project-view]
blocks: []
---

# Project Item Add

## As a

maintainer organizing work into a project

## I want

to add an issue or pull request as an item to a GitHub Project

## Acceptance criteria

1. Running `gor project item-add 7 --issue 42` adds issue #42 to project number 7
2. `--pull-request` / `--pr` flag adds a pull request instead of an issue
3. `--owner` / `--org` flag sets the project owner context
4. `--project` flag selects the project by number or ID
5. The created item's ID is printed on success
6. `--hostname` flag targets a specific host

## Out of scope

- Setting column/status field values on add
- Removing or reordering project items
