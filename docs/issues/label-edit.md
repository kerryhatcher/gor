---
tags: [label, write]
priority: P1
phase: 1
endpoints:
  - PATCH /repos/{owner}/{repo}/labels/{name}
status: done
blockedBy: [label-list]
blocks: []
---

# Label Edit

## As a

developer maintaining a repository's label taxonomy

## I want

to edit an existing label's name, color, or description

## Acceptance criteria

1. Running `gor label edit bug` edits the label named "bug"
2. `--name` flag renames the label to a new name
3. `--color` flag changes the label color (hex, without `#`)
4. `--description` flag changes the label description
5. `--repo` / `-R` flag specifies the repository explicitly
6. `--hostname` flag targets a specific host
7. The updated label's name, color, and description are printed on success

## Out of scope

- Creating a new label (use `gor label create`)
- Deleting a label (separate story)
- Bulk label editing
