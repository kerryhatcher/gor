---
tags: [release, write]
priority: P1
phase: 1
endpoints:
  - DELETE /repos/{owner}/{repo}/releases/{id}
status: todo
blockedBy: [release-view]
blocks: []
---

# Release Delete

## As a

developer managing a project's releases

## I want

to delete a GitHub release that is no longer needed

## Acceptance criteria

1. Running `gor release delete v1.0.0` deletes the release identified by tag `v1.0.0`
2. Running `gor release delete 12345` deletes the release identified by its database ID `12345`
3. A confirmation prompt is shown before deletion (bypassed with `--yes`)
4. `--skip-tag` flag keeps the associated git tag instead of deleting it
5. `--repo` / `-R` flag specifies the repository explicitly
6. `--hostname` flag targets a specific host
7. A success message is printed after the release is deleted
8. A release that does not exist is handled gracefully with a clear message and a non-zero exit code

## Out of scope

- Bulk deletion of multiple releases
- Deleting the underlying git tag without deleting the release
