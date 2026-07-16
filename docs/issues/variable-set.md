---
tags: [variable, write]
priority: P3
phase: 3
endpoints:
  - POST /repos/{owner}/{repo}/actions/variables
  - PUT /repos/{owner}/{repo}/actions/variables/{name}
  - POST /orgs/{org}/actions/variables
  - PUT /orgs/{org}/actions/variables/{name}
---

# Variable Set

## As a

developer automating CI/CD pipelines

## I want

to create or update a repository or organization variable

## Acceptance criteria

1. Running `gor variable set NAME value` creates or updates variable `NAME`
2. `--repo` / `-R` flag specifies the repository explicitly
3. `--org` flag sets an organization-level variable
4. `--visibility` flag sets visibility (`all`, `private`, or `selected`)
5. `--repos` flag scopes visibility to specific repositories (comma-separated)
6. `--hostname` flag targets a specific host
7. A confirmation message is printed on success
8. Exit code 0 on success

## Out of scope

- Deleting variables (separate story)
- Environment-scoped variables
