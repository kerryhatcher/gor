---
description: Manage GitHub Issues — list, view, create, comment, close, and edit
---

# Issues via gor

Use the `gor` CLI to manage GitHub issues.

## List issues

```bash
gor issue list -R owner/repo
gor issue list -R owner/repo --state closed --label bug
gor issue list -R owner/repo --json number,title,labels,assignees
```

## View an issue

```bash
gor issue view 123 -R owner/repo
gor issue view 123 -R owner/repo --json number,title,body,comments
```

## Create an issue

```bash
gor issue create -R owner/repo --title "Bug found" --body "Steps to reproduce..."
gor issue create -R owner/repo --title "Feature request" --label enhancement
```

## Modify an issue

```bash
gor issue comment 123 -R owner/repo --body "I can reproduce this"
gor issue close 123 -R owner/repo
gor issue reopen 123 -R owner/repo
gor issue edit 123 -R owner/repo --title "Updated title"
```

## Locking and pins

```bash
gor issue lock 123 -R owner/repo --reason spam
gor issue unlock 123 -R owner/repo
gor issue pin 123 -R owner/repo
gor issue unpin 123 -R owner/repo
```

## Transfer and delete

```bash
gor issue transfer 123 -R owner/repo --target-repo target-owner/target-repo
gor issue delete 123 -R owner/repo
```
