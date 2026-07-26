---
description: Manage Gists — list, view, create, edit, and delete
---

# Gists via gor

Use the `gor` CLI to manage GitHub gists.

```bash
gor gist list
gor gist list --limit 10 --json id,description,files

gor gist view GIST_ID
gor gist view GIST_ID --json id,description,files,created_at

gor gist create file.md --desc "A useful snippet"
gor gist create file1.rs file2.rs --desc "Rust examples" --public

gor gist edit GIST_ID --desc "Updated description"
gor gist edit GIST_ID --add new_file.rs --filename old_name:new_name

gor gist delete GIST_ID
```
