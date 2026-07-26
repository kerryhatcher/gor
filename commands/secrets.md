---
description: Manage secrets, variables, SSH keys, and GPG keys
---

# Secrets, Variables & Keys via gor

Use the `gor` CLI to manage secrets, variables, SSH keys, and GPG keys.

## Secrets

```bash
gor secret list -R owner/repo
gor secret set MY_SECRET -R owner/repo
gor secret delete MY_SECRET -R owner/repo
```

## Variables

```bash
gor variable list -R owner/repo
gor variable set MY_VAR -R owner/repo
gor variable delete MY_VAR -R owner/repo
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
gor gpg-key add ~/.gnupg/pubkey.asc
gor gpg-key delete KEY_ID
```
