---
tags: [variable, write]
priority: P3
phase: 3
endpoints:
  - DELETE /repos/{owner}/{repo}/actions/variables/{name}
  - DELETE /orgs/{org}/actions/variables/{name}
---

# Variable Delete

## As a

developer cleaning up obsolete configuration

## I want

to delete a repository or organization variable

## Acceptance criteria

1. Running `gor variable delete NAME` deletes variable `NAME`
2. `--repo` / `-R` flag specifies the repository explicitly
3. `--org` flag deletes an organization-level variable
4. A confirmation message is printed on success
5. If the variable does not exist, a message indicates it was not found
6. `--hostname` flag targets a specific host
7. Exit code 0 on success

## Out of scope

- Bulk deletion
- Deleting environment-scoped variables
