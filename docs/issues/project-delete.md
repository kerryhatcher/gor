---
tags: [project, write]
priority: P3
phase: 4
endpoints:
  - DELETE /projects/{project_id}
status: todo
blockedBy: [project-create]
blocks: []
---

# Project Delete

## As a

maintainer cleaning up old projects

## I want

to delete a project board

## Acceptance criteria

1. Running `gor project delete 5` deletes project #5
2. The project number is a required positional argument
3. `--org` flag scopes to an organization's projects
4. `--owner` flag scopes to a user's projects
5. A confirmation prompt is shown before deletion (bypassed with `--yes`)
6. `--hostname` flag targets a specific host
7. A success message is printed after deletion
8. If the project does not exist, the command exits non-zero with a clear error

## Out of scope

- Deleting Projects V2
- Bulk project deletion
