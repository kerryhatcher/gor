---
tags: [attestation, read]
priority: P4
phase: 4
endpoints:
  - GET /repos/{owner}/{repo}/attestations/{subject_digest}
---

# Attestation Verify

## As a

developer who needs to verify supply-chain provenance

## I want

to verify signed attestations for a build artifact

## Acceptance criteria

1. Running `gor attestation verify <artifact-path> --owner myorg` verifies attestations for the given artifact
2. The artifact can be a local file or OCI image reference
3. `--owner` / `-o` flag specifies the expected attestation signer (GitHub org/user)
4. `--repo` / `-R` flag scopes verification to a specific repository
5. `--bundle` flag provides an in-toto bundle file instead of fetching from the API
6. Verification checks the signature against the trusted root
7. A clear PASS/FAIL result is printed with details on failure
8. `--hostname` flag targets a specific host
9. Exit code 0 on success, non-zero on verification failure

## Out of scope

- Generating attestations (done by GitHub Actions during builds)
- Storing attestions locally
