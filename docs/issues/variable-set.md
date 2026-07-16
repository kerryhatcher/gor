---
tags: [variable, write]
priority: P2
phase: 3
endpoints:
  - POST /repos/{owner}/{repo}/actions/variables
  - POST /orgs/{org}/actions/variables
  - PATCH /repos/{owner}/{repo}/actions/variables/{name}
  - PATCH /orgs/{org}/actions/variables/{name}
status: done
blockedBy: [repo-view]
blocks: []

# Variable Set

## As a

developer who needs to add or update a CI variable

## I want

to set an Actions variable for a repository or organization

## Acceptance criteria

1. Running `gor variable set NAME "value"` sets the repository variable `NAME`
2. `--body` / `-b` flag provides the variable value inline
3. `--org` flag sets the variable at the organization level
4. `--repos` flag restricts an org variable to specific repositories (comma-separated)
5. `--visibility` flag sets org variable visibility (`all`, `private`, or `selected`)
6. Setting an existing variable updates it (idempotent)
7. `--hostname` flag targets a specific host
8. A success message is printed on completion

## Out of scope

- Encrypting values (variables are stored plaintext by GitHub)
- Deleting variables (separate story)
