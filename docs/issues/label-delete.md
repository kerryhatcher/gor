---
tags: [label, write]
priority: P1
phase: 1
endpoints:
  - DELETE /repos/{owner}/{repo}/labels/{name}
---

# Label Delete

## As a

developer cleaning up a repository's label set

## I want

to delete a label by name

## Acceptance criteria

1. Running `gor label delete bug` deletes the label named `bug`
2. A confirmation prompt is shown before deletion (bypassed with `--yes`)
3. `--repo` / `-R` flag specifies the repository explicitly
4. `--hostname` flag targets a specific host
5. A success message is printed on success
6. If the label does not exist, a clear error is shown and the command exits non-zero

## Out of scope

- Bulk label deletion
- Restoring a deleted label
