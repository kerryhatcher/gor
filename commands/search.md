---
description: Search GitHub — repositories, issues, code, and commits
---

# Search via gor

Use the `gor` CLI to search GitHub.

## Search repositories

```bash
gor search repos "rust cli"
gor search repos "rust cli" --limit 10 --json name,stars,description
```

## Search issues

```bash
gor search issues "bug in login" -R owner/repo
gor search issues "state:open label:bug" --limit 20
```

## Search code

```bash
gor search code "fn main" --language rust
gor search code "TODO" --repo owner/repo
```

## Search commits

```bash
gor search commits "fix clippy" --repo owner/repo
```

## Useful qualifiers

```bash
gor search issues "label:bug is:open"
gor search issues "created:>2024-01-01"
gor search code "class Repository" --language python
```
