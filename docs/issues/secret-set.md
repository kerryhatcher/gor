---
tags: [secret, write]
priority: P2
phase: 3
endpoints:
  - PUT /repos/{owner}/{repo}/actions/secrets/{name}
  - PUT /orgs/{org}/actions/secrets/{name}
status: todo
blockedBy: [repo-view]
blocks: []

# Secret Set

## As a

developer who needs to add or update a CI secret

## I want

to set an encrypted secret for a repository or organization

## Acceptance criteria

1. Running `gor secret set TOKEN -b "value"` sets the repository secret `TOKEN`
2. `--body` / `-b` flag provides the secret value inline
3. `--org` flag sets the secret at the organization level
4. `--repos` flag restricts an org secret to specific repositories (comma-separated)
5. `--visibility` flag sets org secret visibility (`all`, `private`, or `selected`)
6. The secret value is encrypted client-side with the repository/organization public key before upload
7. `--hostname` flag targets a specific host
8. A success message is printed on completion

## Out of scope

- Reading back secret values (GitHub never returns them)
- Deleting secrets (separate story)
