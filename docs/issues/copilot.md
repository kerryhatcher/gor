---
tags: [copilot, read]
priority: P4
phase: 4
endpoints: []
status: todo
blockedBy: [auth-login]
blocks: []
---

# Copilot

## As a

developer using GitHub Copilot

## I want

to check Copilot status, usage, and manage seat assignments from the CLI

## Acceptance criteria

1. Running `gor copilot status` shows the authenticated user's Copilot subscription status
2. The status output includes: plan type, seat status, and renewal date
3. Running `gor copilot usage` shows Copilot usage statistics for an organization
4. `--org` flag scopes usage queries to an organization (requires admin)
5. `--json` flag outputs as JSON with optional field selection
6. `--hostname` flag targets a specific host
7. If Copilot is not enabled for the user or org, a clear message is shown

## Out of scope

- Assigning or revoking Copilot seats
- Configuring Copilot policies or content exclusion
- Copilot Chat or IDE integration
