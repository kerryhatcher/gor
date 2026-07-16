---
tags: [gpg-key, write]
priority: P2
phase: 3
endpoints:
  - POST /user/gpg_keys
---

# GPG Key Add

## As a

developer who wants to sign my commits with a new GPG key

## I want

to add a GPG public key to my GitHub account

## Acceptance criteria

1. Running `gor gpg-key add -f mykey.asc` adds the armored public key from the file
2. `--file` / `-f` flag reads the ASCII-armored public key from a file
3. `--body` / `-b` flag provides the armored key inline (alternative to `--file`)
4. `--hostname` flag targets a specific host
5. The created key's ID is printed on success
6. A malformed or non-armored key fails with a clear error and non-zero exit code

## Out of scope

- Uploading private keys (never sent to GitHub)
- Associating keys with specific email verification
