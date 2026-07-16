---
tags: [variable, write]
priority: P2
phase: 3
endpoints:
  - DELETE /repos/{owner}/{repo}/actions/variables/{name}
  - DELETE /orgs/{org}/actions/variables/{name}
status: done
blockedBy: [variable-list]
blocks: []

# Variable Delete

## As a

developer cleaning up obsolete CI configuration

## I want

to delete an Actions variable from a repository or organization

## Acceptance criteria

1. Running `gor variable delete NAME` deletes the repository variable `NAME`
2. `--org` flag deletes the variable at the organization level
3. `--hostname` flag targets a specific host
4. A confirmation message is printed on success
5. Deleting a non-existent variable fails with a clear error and non-zero exit code

## Out of scope

- Bulk deletion of variables
- Restoring a deleted variable
