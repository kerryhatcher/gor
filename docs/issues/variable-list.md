---
tags: [variable, read]
priority: P2
phase: 3
endpoints:
  - GET /repos/{owner}/{repo}/actions/variables
  - GET /orgs/{org}/actions/variables
status: done
blockedBy: [repo-view]
blocks: [variable-delete]

# Variable List

## As a

developer inspecting CI configuration

## I want

to list the Actions variables defined for a repository or organization

## Acceptance criteria

1. Running `gor variable list` in a repo directory lists repository variables
2. `--org` flag lists organization variables
3. Each row shows: variable name and (for org vars) visibility/selected-repo scope
4. `--limit` / `-L` flag caps results (default: 30)
5. `--json` flag outputs as JSON with optional field selection
6. `--hostname` flag targets a specific host

## Out of scope

- Showing variable values in plaintext (names only, like the API)
- Setting or deleting variables (separate stories)
