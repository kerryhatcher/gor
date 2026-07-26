---
description: Manage repository labels — list, create, edit, delete, and clone
---

# Labels via gor

Use the `gor` CLI to manage repository labels.

```bash
gor label list -R owner/repo
gor label list -R owner/repo --json name,color,description

gor label create bug -R owner/repo --color d73a4a --description "Bug report"
gor label edit bug -R owner/repo --name bug --color d73a4a
gor label delete bug -R owner/repo

gor label clone -R source/repo --target target/repo
```
