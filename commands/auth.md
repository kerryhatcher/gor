---
description: Authenticate with GitHub — login, logout, status, token, and git credential setup
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

Print the current token. Use `--secure` to mask the output (shows only first
and last few characters):

```bash
gor auth token
gor auth token --secure
```

> **Warning:** Tokens grant full API access. Never expose them in logs,
> terminal history, screenshots, or agent output. Use `--secure` whenever
> possible to reduce accidental disclosure.

## Configure git credential helper

Set up git to use `gor` as a credential helper (so git commands authenticate
via your stored token):

```bash
gor auth setup-git
gor auth setup-git --hostname github.mycompany.com
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
