---
tags: [label, write]
priority: P1
phase: 1
endpoints:
  - POST /repos/{owner}/{repo}/labels
status: done
blockedBy: [repo-view]
blocks: []
---

# Label Create

## As a

developer setting up a repository's workflow

## I want

to create a new label with a name, color, and description

## Acceptance criteria

1. Running `gor label create bug` creates a new label named "bug"
2. `--color` flag sets the label color (hex, without `#`, default: auto-generated)
3. `--description` flag sets the label description
4. `--repo` / `-R` flag specifies the repository explicitly
5. `--hostname` flag targets a specific host
6. The label name and color are printed on success

## Out of scope

- Bulk label creation
- Cloning labels from another repository (separate story)
