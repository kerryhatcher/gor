---
description: Manage GitHub Actions — workflows, runs, caches, and CI/CD operations
---

# CI/CD via gor

Use the `gor` CLI to manage GitHub Actions workflows, runs, and caches.

## Workflows

```bash
gor workflow list -R owner/repo
gor workflow view .github/workflows/ci.yml -R owner/repo
gor workflow run .github/workflows/ci.yml -R owner/repo
gor workflow run .github/workflows/ci.yml -R owner/repo --ref main
gor workflow enable .github/workflows/ci.yml -R owner/repo
gor workflow disable .github/workflows/ci.yml -R owner/repo
```

## Workflow runs

```bash
gor run list -R owner/repo
gor run list -R owner/repo --branch main --workflow ci.yml
gor run view 1234567890 -R owner/repo
gor run watch 1234567890 -R owner/repo
gor run cancel 1234567890 -R owner/repo
gor run rerun 1234567890 -R owner/repo --failed-jobs
gor run download 1234567890 -R owner/repo --dir ./artifacts
```

## Cache management

```bash
gor cache list -R owner/repo
gor cache list -R owner/repo --json key,size_in_bytes,ref

# Delete by exact key
gor cache delete --key "node-linux-pnpm-" -R owner/repo

# Delete all caches
gor cache delete --all -R owner/repo
```
