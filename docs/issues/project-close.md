---
tags: [project, write]
priority: P3
phase: 4
endpoints:
  - PATCH /projects/{project_id}
status: todo
blockedBy: [project-create]
blocks: []
---

# Project Close

## As a

maintainer completing a project

## I want

to close or reopen a project board

## Acceptance criteria

1. Running `gor project close 5` closes project #5
2. Running `gor project close 5 --reopen` reopens a previously closed project
3. The project number is a required positional argument
4. `--org` flag scopes to an organization's projects
5. `--owner` flag scopes to a user's projects
6. `--hostname` flag targets a specific host
7. A success message is printed confirming the operation
8. If the project is already in the target state, the command exits gracefully with a clear message

## Out of scope

- Closing Projects V2
- Bulk closing multiple projects
