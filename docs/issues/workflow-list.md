---
tags: [ci, read]
priority: P2
phase: 2
endpoints:
  - GET /repos/{owner}/{repo}/actions/workflows
status: done
blockedBy: [repo-view]
blocks: [workflow-view]
---

# Workflow List

## As a

developer checking CI configuration

## I want

to list the GitHub Actions workflows in a repository

## Acceptance criteria

1. Running `gor workflow list` in a repo directory lists all workflows
2. Each row shows: workflow name, state (active/disabled), path, workflow ID
3. `--repo` / `-R` flag specifies the repository explicitly
4. `--limit` / `-L` flag caps results (default: 30)
5. `--json` flag outputs as JSON with optional field selection
6. `--hostname` flag targets a specific host

## Out of scope

- Workflow YAML content display
- Workflow usage statistics
