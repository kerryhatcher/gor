---
tags: [auth, write]
priority: P0
phase: 0
endpoints:
  - POST https://github.com/login/device/code
  - POST https://github.com/login/oauth/access_token
  - GET /user
---

# Auth Login

## As a

developer who wants to use `gor` with my GitHub account

## I want

to authenticate via the OAuth device flow so that `gor` can act on my behalf

## Acceptance criteria

1. Running `gor auth login` opens the device flow
2. The user is shown a one-time code and instructed to visit `https://github.com/login/device`
3. `gor` polls the token endpoint until the user completes authorization (or times out)
4. On success, the token is stored in the OS keyring
5. On success, the authenticated user's login is displayed
6. `--hostname` flag supports GitHub Enterprise Server instances
7. `--scopes` flag allows requesting specific OAuth scopes
8. Token is verified by calling `GET /user` before storing

## Out of scope

- Web-based OAuth callback flow
- GitHub App installation auth
- Fine-grained PAT creation wizard
