---
tags: [attestation, read]
priority: P2
phase: 4
endpoints:
  - GET /repos/{owner}/{repo}/attestations
---

# Attestation Verify

## As a

developer who wants to verify build provenance of artifacts

## I want

to verify a GitHub artifact attestation against a digest or OCI image

## Acceptance criteria

1. Running `gor attestation verify <file> --owner myorg` verifies the attestation for `<file>`
2. `--owner` flag sets the expected artifact owner/organization
3. `--repo` flag scopes verification to a specific repository
4. `--bundle` flag supplies a local Sigstore bundle file instead of fetching from the API
5. Verification checks the signing certificate, signature, and subject digest
6. The command exits 0 only when the attestation is valid and the subject matches
7. A clear success or failure message is printed
8. `--hostname` flag targets a specific host

## Out of scope

- Generating attestations (done at build time by GitHub Actions)
- Downloading the trusted root (separate story)
