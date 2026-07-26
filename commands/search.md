---
description: Search GitHub — repositories, issues, pull requests, code, and commits
---

# Search via gor

Use the `gor` CLI to search GitHub across multiple domains.

## Search repositories

```bash
gor search repos "rust cli"
gor search repos "rust cli" --limit 10 --json name,stars,description
```

## Search issues and pull requests

```bash
gor search issues "bug in login" -R owner/repo
gor search prs "state:open label:enhancement" -R owner/repo
```

## Search code

```bash
gor search code "fn main" --lang rust
gor search code "TODO" -R owner/repo --path src/
```

## Search commits

```bash
gor search commits "fix clippy" -R owner/repo
```

## Useful qualifiers

```bash
gor search issues "label:bug is:open"
gor search prs "author:username"
gor search issues "created:>2024-01-01"
gor search code "class Repository" --lang python
```
