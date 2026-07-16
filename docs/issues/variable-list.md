---
tags: [variable, read]
priority: P3
phase: 3
endpoints:
  - GET /repos/{owner}/{repo}/actions/variables
  - GET /orgs/{org}/actions/variables
---

# Variable List

## As a

developer managing CI/CD configuration

## I want

to list repository or organization variables

## Acceptance criteria

1. Running `gor variable list` in a repo directory lists repository variables
2. Each row shows: variable name, value
3. `--repo` / `-R` flag specifies the repository explicitly
4. `--org` flag lists organization-level variables
5. `--hostname` flag targets a specific host
6. `--json` flag outputs as JSON with optional field selection
7. Secret values are never shown for sensitive variables
8. Exit code 0 on success

## Out of scope

- Setting or deleting variables (separate stories)
- Viewing environment-scoped variables
