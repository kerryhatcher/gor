---
tags: [secret, write]
priority: P2
phase: 3
endpoints:
  - DELETE /repos/{owner}/{repo}/actions/secrets/{name}
  - DELETE /orgs/{org}/actions/secrets/{name}
status: todo
blockedBy: [secret-list]
blocks: []

# Secret Delete

## As a

developer cleaning up stale CI secrets

## I want

to delete an encrypted secret from a repository or organization

## Acceptance criteria

1. Running `gor secret delete TOKEN` deletes the repository secret `TOKEN`
2. `--org` flag deletes the secret at the organization level
3. A confirmation message with the secret name is printed on success
4. `--hostname` flag targets a specific host
5. Attempting to delete a non-existent secret fails with a clear error and non-zero exit code

## Out of scope

- Bulk secret deletion
- Deleting secrets from multiple repositories at once
