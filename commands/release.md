---
description: Manage GitHub Releases — list, view, create, edit, upload, and download assets
---

# Releases via gor

Use the `gor` CLI to manage GitHub releases.

## List releases

```bash
gor release list -R owner/repo
gor release list -R owner/repo --limit 5 --json tag_name,created_at
```

## View a release

```bash
gor release view v1.0.0 -R owner/repo
gor release view v1.0.0 -R owner/repo --json tag_name,body,assets
```

## Create a release

```bash
gor release create v1.0.0 -R owner/repo --title "v1.0.0" --body "Release notes..."
gor release create v1.0.0 -R owner/repo --notes-file CHANGELOG.md --prerelease
```

## Upload and download assets

```bash
gor release upload v1.0.0 -R owner/repo ./binary.tar.gz
gor release upload v1.0.0 -R owner/repo ./binary.tar.gz --label "Linux binary"

gor release download v1.0.0 -R owner/repo --dir ./downloads
```

## Edit and delete

```bash
gor release edit v1.0.0 -R owner/repo --title "New title"
gor release delete v1.0.0 -R owner/repo
```
