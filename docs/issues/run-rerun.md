---
tags: [ci, write]
priority: P2
phase: 2
endpoints:
  - POST /repos/{owner}/{repo}/actions/runs/{id}/rerun
status: todo
blockedBy: [run-view]
blocks: []
---

# Run Rerun

## As a

developer who needs to retry a CI run after a fix or flaky failure

## I want

to rerun a workflow run from the command line

## Acceptance criteria

1. Running `gor run rerun 12345` reruns workflow run #12345
2. `--repo` / `-R` flag specifies the repository explicitly
3. `--hostname` flag targets a specific host
4. `--failed-jobs` flag reruns only the failed jobs in the run instead of the whole run
5. `--debug` flag enables debug logging for the rerun
6. The new run URL is printed on success

## Out of scope

- Watching the rerun to completion (use `gor run watch`)
- Canceling a run before rerunning (separate `gor run cancel` story)
