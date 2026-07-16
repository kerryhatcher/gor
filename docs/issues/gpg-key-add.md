---
tags: [gpg-key, write]
priority: P3
phase: 3
endpoints:
  - POST /user/gpg_keys
---

# GPG Key Add

## As a

developer who wants to sign commits and tags with GPG

## I want

to add a new GPG public key to my account

## Acceptance criteria

1. Running `gor gpg-key add "-----BEGIN PGP PUBLIC KEY BLOCK-----..."` adds the given armored public key
2. The key body may also be read from a file path argument instead of inline
3. `--hostname` flag targets a specific host
4. A confirmation message with the key ID is printed on success
5. Exit code 0 on success

## Out of scope

- Generating GPG key pairs (use `gpg --gen-key`)
- Deleting keys (separate story)
