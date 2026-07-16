---
tags: [ci, write]
priority: P2
phase: 2
endpoints:
  - PUT /repos/{owner}/{repo}/actions/workflows/{id}/enable
status: todo
blockedBy: [workflow-view]
blocks: []
---

# Workflow Enable / Disable

## As a

developer managing CI automation

## I want

to enable or disable a GitHub Actions workflow from the command line

## Acceptance criteria

1. Running `gor workflow enable deploy.yml` enables the workflow identified by filename `deploy.yml`
2. Running `gor workflow disable deploy.yml` disables the same workflow
3. A workflow may be identified by filename (e.g. `deploy.yml`), workflow ID (e.g. `12345`), or workflow name
4. `--repo` / `-R` flag specifies the repository explicitly
5. `--hostname` flag targets a specific host
6. A confirmation message is printed showing the workflow's new state (e.g. `Workflow "deploy.yml" is now enabled.`)

## Out of scope

- Bulk enable/disable of multiple workflows at once
- Enabling/disabling workflows at the organization level
