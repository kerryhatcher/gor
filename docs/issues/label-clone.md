---
tags: [label, write]
priority: P1
phase: 1
endpoints:
  - GET /repos/{owner}/{repo}/labels
  - POST /repos/{owner}/{repo}/labels
---

# Label Clone

## As a

developer setting up a new repository

## I want

to copy all labels from an existing repository to a new one

## Acceptance criteria

1. Running `gor label clone owner/source` clones all labels from the source repo into the current repository
2. `--source` flag specifies the source repository (OWNER/REPO format)
3. `--repo` / `-R` flag specifies the target repository (defaults to the current repo)
4. `--force` flag overwrites existing labels in the target repo that share a name
5. `--hostname` flag targets a specific host
6. Labels created, updated, and skipped are printed as a summary
7. Labels are cloned with their name, color, and description preserved
8. A confirmation message shows the count of labels cloned

## Out of scope

- Cloning a subset of labels by name (use `--source` + manual label delete)
- Cloning issue templates or other repo metadata
- Bidirectional label sync
