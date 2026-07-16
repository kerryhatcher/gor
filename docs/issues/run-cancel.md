---
tags: [ci, write]
priority: P2
phase: 2
endpoints:
  - POST /repos/{owner}/{repo}/actions/runs/{id}/cancel
status: done
blockedBy: [run-view]
blocks: []
---

# Run Cancel

## As a

developer who has a workflow run that no longer needs to continue

## I want

to cancel a workflow run by its ID

## Acceptance criteria

1. Running `gor run cancel 12345` cancels the workflow run with database ID `12345`
2. The run is identified by its database ID passed as a positional argument
3. `--repo` / `-R` flag specifies the repository explicitly
4. `--hostname` flag targets a specific host
5. A confirmation message is printed on success showing the cancelled run's ID and URL
6. If the run has already reached a terminal state (`completed`, `cancelled`, `skipped`, or `timed_out`), the command exits gracefully with a clear message and a non-zero exit code
7. If the run is in progress, the cancel request is sent and a success message is printed

## Out of scope

- Cancelling multiple runs at once
- Cancelling all runs for a workflow (use the workflow-level dispatch)
- Watching the run until it actually stops (use `gor run watch`)
