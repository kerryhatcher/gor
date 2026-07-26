---
description: Manage GitHub repositories — view, list, create, fork, clone, edit, and delete
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

```bash
gor repo create my-new-repo --description "A great project" --public
```

## Fork a repository

```bash
gor repo fork owner/repo
gor repo fork owner/repo --org my-org
```

## Clone a repository

```bash
gor repo clone owner/repo
gor repo clone owner/repo --target ./my-dir
```

## Other operations

```bash
gor repo edit owner/repo --description "New description"
gor repo archive owner/repo
gor repo rename owner/repo new-name
gor repo delete owner/repo
gor repo sync
```
