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

# Project Edit

## As a

maintainer updating project details

## I want

to edit a project board's name and description

## Acceptance criteria

1. Running `gor project edit 5 --name "Sprint 43"` renames project #5
2. `--name` / `-n` flag sets a new project name
3. `--body` / `-b` flag sets a new project description
4. `--org` flag scopes to an organization's projects
5. `--owner` flag scopes to a user's projects
6. `--hostname` flag targets a specific host
7. A success message is printed confirming the changes
8. If the project does not exist, the command exits non-zero with a clear error

## Out of scope

- Editing Projects V2
- Changing project visibility or template
