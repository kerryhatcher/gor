---
tags: [ci, read]
priority: P2
phase: 2
endpoints:
  - GET /repos/{owner}/{repo}/actions/workflows/{id}
---

# Workflow View

## As a

developer who wants to inspect a CI workflow's configuration and recent activity

## I want

to view the details of a single GitHub Actions workflow

## Acceptance criteria

1. Running `gor workflow view deploy.yml` views the workflow identified by filename `deploy.yml`
2. A workflow may also be identified by its workflow database ID (e.g. `12345`) or by its workflow name
3. `--repo` / `-R` flag specifies the repository explicitly
4. `--hostname` flag targets a specific host
5. `--json` flag outputs as JSON with optional field selection
6. The workflow's display name is shown
7. The workflow's state (`active` or `disabled`) is shown
8. The workflow's path (e.g. `.github/workflows/deploy.yml`) is shown
9. The workflow's trigger events (e.g. `push`, `pull_request`, `schedule`) are listed
10. The workflow's last runs (most recent first, up to a small default like 5) are listed with run ID, status, conclusion, branch, and timestamp

## Out of scope

- Listing all workflows in a repository (see workflow-list.md)
- Triggering a workflow dispatch (see workflow-run.md)
- Enabling or disabling a workflow (see workflow-enable.md)
- Viewing individual job or step details of a run (see run-view.md)
