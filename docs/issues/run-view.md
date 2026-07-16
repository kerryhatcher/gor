---
tags: [ci, read]
priority: P2
phase: 2
endpoints:
  - GET /repos/{owner}/{repo}/actions/runs/{id}
  - GET /repos/{owner}/{repo}/actions/runs/{id}/jobs
---

# Run View

## As a

developer debugging a CI failure

## I want

to view the details and job status of a workflow run

## Acceptance criteria

1. Running `gor run view 12345` shows the run's status, conclusion, and timing
2. Each job is listed with name, status, conclusion, and duration
3. `--repo` / `-R` flag specifies the repository explicitly
4. `--web` / `-w` flag opens the run in the browser
5. `--log` flag shows the log output for a specific job
6. `--log-failed` flag shows logs only for failed jobs
7. `--json` flag outputs as JSON with optional field selection
8. `--hostname` flag targets a specific host

## Out of scope

- Live log tailing (use `gor run watch`)
- Log downloading to a file (separate story)
