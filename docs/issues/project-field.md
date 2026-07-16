---
tags: [project, read, write]
priority: P3
phase: 4
endpoints: []
status: todo
blockedBy: [project-v2]
blocks: []
---

# Project Field

## As a

maintainer customizing project tracking

## I want

to list, create, and delete custom fields in a Projects V2 board

## Acceptance criteria

1. Running `gor project field-list 5` lists custom fields for project #5
2. Each field shows: name, type (text, number, date, single-select, iteration), and ID
3. Running `gor project field-create 5 --name "Priority" --type single-select` creates a new field
4. `--type` flag specifies the field type (required): `text`, `number`, `date`, `single_select`, `iteration`
5. `--options` flag specifies options for single-select fields (comma-separated)
6. Running `gor project field-delete 5 --field-id PVT_abc123` deletes a field
7. `--org` flag scopes to an organization's projects
8. `--owner` flag scopes to a user's projects
9. `--hostname` flag targets a specific host
10. `--json` flag outputs as JSON with optional field selection

## Out of scope

- Editing existing fields
- Reordering fields
- Field-level permissions
