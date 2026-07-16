---
tags: [pr, ci, read]
priority: P1
phase: 1
endpoints:
  - GET /repos/{owner}/{repo}/commits/{ref}/check-runs
---

# PR Checks

## As a

developer reviewing a pull request

## I want

to see the CI check status for a pull request before merging it

## Acceptance criteria

1. Running `gor pr checks 42` shows the CI checks for PR #42
2. The PR is identified by its number passed as a positional argument
3. For each check, the check name, status, conclusion, and details URL are displayed
4. `--repo` / `-R` flag specifies the repository explicitly
5. `--hostname` flag targets a specific host
6. `--watch` flag polls until all checks reach a terminal state (completed), refreshing the displayed status as checks progress
7. `--json` flag outputs as JSON with optional field selection
8. The exit code reflects the overall check status: `0` when all checks are `success` (or have no conclusion), non-zero when any check has a `failure`, `timed_out`, `cancelled`, or `skipped` (action_required) conclusion
9. A summary line shows the overall check status and count (e.g., `3 passed, 1 failed`)

## Out of scope

- Triggering or re-running checks (use `gor run rerun`)
- Viewing the full log output of a check (use `gor run view --log`)
- Diff viewing (use `gor pr diff`)
