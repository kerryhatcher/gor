---
description: Manage secrets, variables, SSH keys, and GPG keys
---

# Secrets, Variables & Keys via gor

Use the `gor` CLI to manage secrets, variables, SSH keys, and GPG keys.

## Secrets (org- or environment-scoped)

```bash
gor secret list --org my-org
gor secret set MY_SECRET --body "value-here" --org my-org
gor secret delete MY_SECRET --org my-org
```

## Variables (org- or environment-scoped)

```bash
gor variable list --org my-org
gor variable set MY_VAR --body "value-here" --org my-org
gor variable delete MY_VAR --org my-org
```

## SSH keys

```bash
gor ssh-key list
gor ssh-key add ~/.ssh/id_ed25519.pub --title "My laptop"
gor ssh-key delete KEY_ID
```

## GPG keys

```bash
gor gpg-key list
gor gpg-key add --file ~/.gnupg/pubkey.asc
gor gpg-key delete KEY_ID
```
