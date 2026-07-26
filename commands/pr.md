---
description: Manage GitHub Pull Requests — list, view, create, merge, review, checkout, and more
---

# Pull Requests via gor

Use the `gor` CLI to manage pull requests. Always prefer `--json` for
programmatic consumption.

## List pull requests

```bash
gor pr list -R owner/repo
gor pr list -R owner/repo --state closed --limit 20
gor pr list -R owner/repo --json number,title,author,state
```

## View a pull request

```bash
gor pr view 42 -R owner/repo
gor pr view 42 -R owner/repo --json number,title,body,mergeable
```

## Create a pull request

```bash
gor pr create -R owner/repo --title "Fix bug" --body "Description of fix"
gor pr create -R owner/repo --title "WIP: feature" --draft
```

## Merge a pull request

```bash
gor pr merge 42 -R owner/repo
gor pr merge 42 -R owner/repo --squash
gor pr merge 42 -R owner/repo --merge --delete-branch
```

## Review a pull request

```bash
gor pr review 42 -R owner/repo --approve --body "Looks good!"
gor pr review 42 -R owner/repo --request-changes --body "Needs tests"
gor pr review 42 -R owner/repo --comment --body "What about edge cases?"
```

## Checkout locally

```bash
gor pr checkout 42 -R owner/repo
```

## Other operations

```bash
gor pr comment 42 -R owner/repo --body "Fixed in latest commit"
gor pr close 42 -R owner/repo
gor pr reopen 42 -R owner/repo
gor pr ready 42 -R owner/repo
gor pr diff 42 -R owner/repo
gor pr checks 42 -R owner/repo
gor pr edit 42 -R owner/repo --title "New title"
```
