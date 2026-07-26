---
description: Authenticate with GitHub — login, logout, status, and token management
---

# GitHub Authentication via gor

Use the `gor` CLI to authenticate with GitHub. Supports fine-grained PATs,
classic PATs, and OAuth device flow.

## Login

Authenticate via OAuth device flow (opens a browser):

```bash
gor auth login
```

Or provide a token directly from a file or pipe:

```bash
gor auth login --with-token < ~/.gh-token
```

## Check status

Verify you're authenticated and see which host and user:

```bash
gor auth status
```

## Logout

Remove stored credentials:

```bash
gor auth logout
```

## View token

Print the current token for debugging:

```bash
gor auth token
```

## Environment variables

The following env vars are read automatically (no `auth login` needed):

| Variable | Purpose |
|---|---|
| `GH_TOKEN` or `GITHUB_TOKEN` | Token for github.com |
| `GH_ENTERPRISE_TOKEN` or `GITHUB_ENTERPRISE_TOKEN` | Token for GHES |
| `GH_HOST` | Target hostname for GHES |

## GitHub Enterprise Server

Target a GHES instance on any command:

```bash
gor --hostname github.mycompany.com auth status
gor --hostname github.mycompany.com repo view org/repo
```
