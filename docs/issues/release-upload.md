---
tags: [release, write]
priority: P1
phase: 1
endpoints:
  - POST /repos/{owner}/{repo}/releases/{id}/assets
status: done
blockedBy: [release-view]
blocks: []
---

# Release Upload

## As a

developer shipping build artifacts for a release

## I want

to upload one or more asset files to an existing GitHub release

## Acceptance criteria

1. Running `gor release upload v1.0.0 ./dist/app-linux.zip ./dist/app-macos.zip` uploads the given files to the release identified by tag `v1.0.0`
2. A release may be identified by tag name or by release database ID (e.g. `12345`)
3. Multiple asset files may be passed as positional arguments and are uploaded in order
4. `--repo` / `-R` flag specifies the repository explicitly
5. `--hostname` flag targets a specific host
6. `--name` flag overrides the asset name on the release (only meaningful for a single asset upload)
7. The content type is auto-detected from the file extension (e.g. `.zip` → `application/zip`)
8. `--mime-type` flag overrides the auto-detected content type
9. A progress indicator is displayed during each upload
10. The asset URL is printed on success for each uploaded file
11. A duplicate asset name fails with a clear error and non-zero exit code

## Out of scope

- Creating a new release (use `gor release create`)
- Deleting or replacing existing assets
- Uploading from stdin
