---
description: Manage GitHub repositories — view, list, create, fork, clone, and edit
---

# Repository Management via gor

Use the `gor` CLI to manage GitHub repositories.

## View a repository

```bash
gor repo view owner/repo
gor repo view owner/repo --json name,description,stars,language
```

## List repositories

```bash
gor repo list
gor repo list --org my-org
gor repo list --type fork --sort updated
```

## Create a repository

Default visibility is public; use `--private` for private repos:

```bash
gor repo create my-new-repo --description "A great project"
gor repo create private-repo --description "Internal tool" --private
```

## Fork a repository

```bash
gor repo fork owner/repo
gor repo fork owner/repo --org my-org
```

## Clone a repository

```bash
gor repo clone owner/repo
gor repo clone owner/repo --directory ./my-dir
```

## Other operations

```bash
# Edit repo settings (use -R for target)
gor repo edit -R owner/repo --description "New description" --visibility private

# Delete
gor repo delete owner/repo

# Sync a fork with its upstream
gor repo sync
```
