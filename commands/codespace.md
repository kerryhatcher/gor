---
description: Manage GitHub Codespaces — list, create, stop, delete, and SSH
---

# Codespaces via gor

Use the `gor` CLI to manage GitHub Codespaces.

```bash
# List
gor codespace list
gor codespace list --json name,repository,state,branch

# Create
gor codespace create owner/repo
gor codespace create owner/repo --branch feature-branch

# SSH (positional: codespace name)
gor codespace ssh my-codespace-name

# Stop (positional: codespace name)
gor codespace stop my-codespace-name

# Delete (positional: codespace name, use --yes to skip prompt)
gor codespace delete my-codespace-name --yes
```
