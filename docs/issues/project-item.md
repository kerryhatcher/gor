---
tags: [project, write]
priority: P3
phase: 4
endpoints: []
status: todo
blockedBy: [project-v2]
blocks: []
---

# Project Item

## As a

maintainer managing project content

## I want

to create, edit, delete, and archive items in a Projects V2 board

## Acceptance criteria

1. Running `gor project item-create 5 --title "New task"` creates a draft item in project #5
2. `--body` / `-b` flag sets a description for the item
3. `--field` flag sets field values (repeatable, format: `field_id=value`)
4. Running `gor project item-edit 5 --item-id PVTI_abc --field Priority=High` edits an item
5. Running `gor project item-delete 5 --item-id PVTI_abc` deletes an item
6. Running `gor project item-archive 5 --item-id PVTI_abc` archives an item
7. `--org` flag scopes to an organization's projects
8. `--owner` flag scopes to a user's projects
9. `--hostname` flag targets a specific host
10. A confirmation prompt is shown before deletion (bypassed with `--yes`)

## Out of scope

- Converting draft items to issues
- Bulk item operations
- Item reordering
