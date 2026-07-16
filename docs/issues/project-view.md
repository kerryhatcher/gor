---
tags: [project, read]
priority: P3
phase: 3
endpoints:
  - GET /projects/{project_id}
  - GET /projects/{project_id}/columns
---

# Project View

## As a

developer inspecting a project's structure

## I want

to see a project's title, description, and columns

## Acceptance criteria

1. Running `gor project view 5` shows project #5 for the current owner
2. The project title, description, state, and visibility are displayed
3. Columns are listed with their names and item counts
4. `--owner` / `-o` flag specifies the project owner
5. `--web` / `-w` flag opens the project in the browser
6. `--json` flag outputs as JSON with optional field selection
7. `--hostname` flag targets a specific host
8. Exit code 0 on success

## Out of scope

- Editing project fields or items inline
- Viewing individual project items' content
