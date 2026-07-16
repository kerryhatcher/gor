---
tags: [release, read]
priority: P1
phase: 1
endpoints:
  - GET /repos/{owner}/{repo}/releases/assets/{id}
  - GET /repos/{owner}/{repo}/releases/tags/{tag}
status: done
blockedBy: [release-view]
blocks: []
---

# Release Download

## As a

developer who wants the binaries or files attached to a release

## I want

to download release assets from a specific release

## Acceptance criteria

1. Running `gor release download v1.0.0` downloads all assets from the release for that tag
2. Running `gor release download 12345` downloads all assets from the release by database ID
3. `--repo` / `-R` flag specifies the repository explicitly
4. `--hostname` flag targets a specific host
5. `--pattern` flag filters assets to download by glob pattern (e.g., `*.tar.gz`), repeatable
6. `--dir` / `-D` flag sets the output directory (default: current directory)
7. `--skip-existing` flag skips assets whose file already exists in the output directory
8. A progress bar is displayed for each asset during download
9. The downloaded file paths are printed on success
10. If no assets match the `--pattern`, a message is shown and the command exits non-zero

## Out of scope

- Uploading assets (see release-upload.md)
- Downloading the release source archive (use `gor repo download` or `gor api`)
- Resuming interrupted downloads
