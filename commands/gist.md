---
description: Manage Gists — list, view, create, edit, delete, and clone
---

# Gists via gor

Use the `gor` CLI to manage GitHub gists.

```bash
gor gist list
gor gist list --limit 10 --json id,description,files

gor gist view GIST_ID
gor gist view GIST_ID --json id,description,files,created_at

gor gist create file.md
gor gist create file1.rs file2.rs --description "Rust examples" --public

gor gist edit GIST_ID --add new_file.rs
gor gist edit GIST_ID --remove old_file.rs

gor gist delete GIST_ID

gor gist clone GIST_ID
gor gist clone GIST_ID --target ./my-gist
```
