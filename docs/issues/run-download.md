---
tags: [ci, read]
priority: P2
phase: 2
endpoints:
  - GET /repos/{owner}/{repo}/actions/runs/{id}/artifacts
  - GET /repos/{owner}/{repo}/actions/artifacts/{id}/zip
status: done
blockedBy: [run-view]
blocks: []
---

# Run Download

## As a

developer who needs the build outputs or logs from a CI run

## I want

to download the artifacts (or job logs) from a workflow run to my local machine

## Acceptance criteria

1. Running `gor run download 12345` downloads all artifacts from run #12345
2. The run is identified by its database ID (passed as a positional argument)
3. Downloaded artifacts are each saved as a `.zip` file in the output directory
4. `--repo` / `-R` flag specifies the repository explicitly
5. `--hostname` flag targets a specific host
6. `--dir` / `-D` flag sets the output directory (default: current directory)
7. `--name` flag filters the artifacts to download by name (repeatable; only matching artifacts are downloaded)
8. `--log` flag downloads the job logs instead of the build artifacts (one log file per job)
9. A progress bar is displayed for each artifact/log during download
10. The downloaded file paths are printed on success
11. If the run has no artifacts (and `--log` is not given), a message is shown and the command exits non-zero

## Out of scope

- Filtering artifacts by other attributes (e.g. expired, size)
- Unzipping artifacts after download
- Downloading logs for a single specific job only
