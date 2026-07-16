---
tags: [ci, read]
priority: P2
phase: 2
endpoints:
  - GET /repos/{owner}/{repo}/actions/runs/{id}
  - GET /repos/{owner}/{repo}/actions/runs/{id}/jobs
status: todo
blockedBy: [run-view]
blocks: []
---

# Run Watch

## As a

developer waiting on a CI run to finish

## I want

to watch a workflow run live until it completes

## Acceptance criteria

1. Running `gor run watch 12345` polls run #12345 until it reaches a terminal state
2. The run is identified by its database ID (passed as a positional argument)
3. Live job status updates are displayed as they change (job name, status, conclusion)
4. The command exits when the run reaches a terminal state (`completed`, `cancelled`, or `skipped`)
5. `--repo` / `-R` flag specifies the repository explicitly
6. `--hostname` flag targets a specific host
7. `--interval` flag sets the poll frequency in seconds (default: 3)
8. `--exit-status` flag causes `gor` to exit with the run's conclusion code (`success` → 0, `failure` → 1, `cancelled`/`skipped`/`timed_out` → non-zero)
9. Interrupted runs (no terminal state after polling) are handled gracefully with a clear message and a non-zero exit code
10. A final summary line shows the run's conclusion and URL

## Out of scope

- Downloading logs from the watched run (use `gor run view --log` or a separate download story)
- Watching multiple runs concurrently
