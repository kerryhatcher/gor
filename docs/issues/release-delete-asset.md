---
tags: [release, write]
priority: P2
phase: 4
endpoints:
  - DELETE /repos/{owner}/{repo}/releases/assets/{asset_id}
status: todo
blockedBy: [release-view]
blocks: []
---

# Release Delete-Asset

## As a

developer who uploaded the wrong binary to a release

## I want

to delete a specific asset from a GitHub release

## Acceptance criteria

1. Running `gor release delete-asset 12345` deletes the asset with database ID `12345`
2. The asset ID is a required positional argument
3. `--repo` / `-R` flag specifies the repository explicitly
4. `--hostname` flag targets a specific host
5. A confirmation prompt is shown before deletion (bypassed with `--yes`)
6. A success message is printed with the deleted asset name
7. If the asset does not exist, the command exits non-zero with a clear error

## Out of scope

- Deleting assets by filename (use the asset ID)
- Bulk deletion of multiple assets
