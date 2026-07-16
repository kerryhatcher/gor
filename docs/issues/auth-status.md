---
tags: [auth, read]
priority: P0
phase: 0
endpoints:
  - GET /user
---

# Auth Status

## As a

developer who uses `gor` across multiple hosts

## I want

to check which account I'm authenticated as and whether my token is still valid

## Acceptance criteria

1. Running `gor auth status` displays the current user's login name
2. The hostname (github.com or GHES) is shown
3. Token scopes are listed if available
4. Token expiration is shown if the token has an expiry
5. `--hostname` flag shows status for a specific host
6. `--show-token` flag reveals the token value (masked by default)
7. Exit code 0 if authenticated, non-zero if not

## Out of scope

- Token refresh or renewal
- Multi-account switching
