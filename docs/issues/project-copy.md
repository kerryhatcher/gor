---
tags: [project, write]
priority: P3
phase: 4
endpoints:
  - POST /projects/{project_id}/copy
status: todo
blockedBy: [project-create]
blocks: []
---

# Project Copy

## As a

maintainer reusing project templates

## I want

to copy an existing project board

## Acceptance criteria

1. Running `gor project copy 5 --target-org myorg` copies project #5 to the target organization
2. `--target-org` flag specifies the destination organization
3. `--target-repo` flag specifies the destination repository
4. `--name` / `-n` flag sets a name for the copied project
5. `--include-drafts` flag includes draft issues in the copy
6. `--hostname` flag targets a specific host
7. A success message is printed with the new project number
8. If the source project does not exist, the command exits non-zero with a clear error

## Out of scope

- Copying Projects V2
- Copying projects across hosts
