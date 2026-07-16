---
tags: [ci, write]
priority: P2
phase: 2
endpoints:
  - POST /repos/{owner}/{repo}/actions/workflows/{id}/dispatches
status: todo
blockedBy: [workflow-view]
blocks: []
---

# Workflow Run (Dispatch)

## As a

developer triggering a CI pipeline

## I want

to manually trigger a workflow dispatch event

## Acceptance criteria

1. Running `gor workflow run deploy.yml` triggers the named workflow
2. The workflow can be identified by filename, workflow ID, or workflow name
3. `--repo` / `-R` flag specifies the repository explicitly
4. `--ref` flag specifies the branch or tag to run on (default: default branch)
5. `--field` / `-F` flag passes input parameters (repeatable, `key=value`)
6. `--json` flag passes JSON-encoded inputs
7. A confirmation message with the workflow run URL is printed
8. `--hostname` flag targets a specific host

## Out of scope

- Waiting for the workflow run to complete (use `gor run watch`)
- Triggering non-dispatch workflows (push, PR, etc.)
