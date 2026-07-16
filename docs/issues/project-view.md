---
tags: [project, read]
priority: P2
phase: 3
endpoints:
  - GET /projects/{project_id}
---

# Project View

## As a

developer who wants to inspect a project's details

## I want

to view a single GitHub Project by number or ID

## Acceptance criteria

1. Running `gor project view 7` shows project number 7 for the current repository context
2. `--owner` / `--org` flag sets the project owner context when not in a repo
3. The displayed fields include: title, body, state, creator, and created/updated dates
4. `--web` / `-w` flag opens the project in the browser
5. `--json` flag outputs as JSON with optional field selection
6. `--hostname` flag targets a specific host

## Out of scope

- Rendering the project board / column layout
- Editing project fields (separate story)
