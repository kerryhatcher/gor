---
description: Manage GitHub Codespaces — list, create, stop, delete, and SSH
---

# Codespaces via gor

Use the `gor` CLI to manage GitHub Codespaces.

```bash
gor codespace list
gor codespace list --json name,repository,state,branch

gor codespace create -R owner/repo
gor codespace create -R owner/repo --branch feature-branch

gor codespace ssh
gor codespace ssh --codespace my-codespace-name

gor codespace stop --codespace my-codespace-name

gor codespace delete --codespace my-codespace-name
gor codespace delete --all
```
