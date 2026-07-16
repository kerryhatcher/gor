---
tags: [gist, write]
priority: P2
phase: 2
endpoints:
  - PATCH /gists/{id}
status: todo
blockedBy: [gist-view]
blocks: []
---

# Gist Edit

## As a

developer who needs to update a shared code snippet

## I want

to edit an existing gist by updating its description, adding files, or renaming files

## Acceptance criteria

1. Running `gor gist edit abc123def456` edits the gist identified by the given ID
2. `--desc` flag updates the gist description
3. `--add` flag adds a file to the gist (repeatable; `path=content` or reads from a local file path)
4. `--filename` flag renames a file in the gist (`old:new`)
5. `--hostname` flag targets a specific host
6. The updated gist URL is printed on success

## Out of scope

- Creating a new gist (use `gor gist create`)
- Deleting a gist (separate story)
