# Research: Using `gix` (gitoxide) as the Git Engine for `gor`

## Summary

`gix` (gitoxide, v0.85) is a pure-Rust implementation of the Git version
control system, split across ~40 subcrates. `gor` already depends on `gix`
for HTTP transport, remote detection, and clone operations.

This document evaluates how much of a git porcelain `gor` can build using
only `gix` — without adding `jj-lib` or any other VCS dependency.

---

## gix Architecture

```
┌────────────────────────────────────────────┐
│              gix (high-level)              │
│  Repository, Reference, Worktree, Status   │
├──────────┬──────────┬──────────┬───────────┤
│ gix-odb  │ gix-ref  │ gix-index│ gix-diff  │
│ (object  │ (refs,   │ (staging │ (diff     │
│  store)  │  HEAD)   │  area)   │  engine)  │
├──────────┼──────────┼──────────┼───────────┤
│ gix-pack │ gix-prot │ gix-work │ gix-traver│
│ (pack    │ ocol     │ tree-stat│ se        │
│  files)  │ (fetch/  │ e (workt │ (history  │
│          │  push)   │  ree I/O)│  walk)    │
├──────────┴──────────┴──────────┴───────────┤
│           gix-object (blobs, trees,         │
│            commits, tags)                   │
└────────────────────────────────────────────┘
```

### Key subcrates and their roles

| Subcrate | Role | Already in lockfile? |
|---|---|---|
| `gix` | High-level facade — `Repository`, `Status`, `Worktree` | ✅ Yes |
| `gix-odb` | Object store — read/write loose + pack files | ✅ Yes |
| `gix-object` | Object types — `Blob`, `Tree`, `Commit`, `Tag` | ✅ Yes |
| `gix-ref` | Reference operations — HEAD, branches, tags | ✅ Yes |
| `gix-index` | Staging area (index) read/write | ✅ Yes |
| `gix-diff` | Tree diff engine | ✅ Yes |
| `gix-status` | Working tree status — staged, unstaged, untracked | ✅ Yes |
| `gix-dir` | Directory walk — `.gitignore` evaluation | ✅ Yes |
| `gix-worktree` | Working tree entry management | ✅ Yes |
| `gix-worktree-state` | Checkout/restore files to disk | ✅ Yes |
| `gix-traverse` | Tree/commit traversal | ✅ Yes |
| `gix-revision` | Revision parsing, merge-base, describe | ✅ Yes |
| `gix-revwalk` | Commit graph walking | ✅ Yes |
| `gix-commitgraph` | Commit-graph file (speed up history) | ✅ Yes |
| `gix-config` | `.git/config` read/write | ✅ Yes |
| `gix-protocol` | Fetch/push protocol | ✅ Yes |
| `gix-transport` | Transport layer (HTTP, SSH, file://) | ✅ Yes |
| `gix-credentials` | Credential helpers | ✅ Yes |
| `gix-refspec` | Refspec parsing and matching | ✅ Yes |
| `gix-negotiate` | Fetch negotiation (haves/wants) | ✅ Yes |
| `gix-blame` | Line-by-line blame — **available via `blame` feature** | ❌ Not enabled |
| `gix-merge` | 3-way tree merge — **available via `merge` feature** | ❌ Not enabled |
| `gix-submodule` | Submodule operations | ✅ In lockfile |
| `gix-ignore` | `.gitignore` / `.gitattributes` ignore rules | ✅ Yes |
| `gix-filter` | Git filter drivers (smudge/clean) | ✅ Yes |

---

## Command-by-Command Capability Mapping

### ✅ Ready with currently enabled features

Current `gor` features: `basic`, `sha1`, `blocking-http-transport-reqwest-rust-tls`, `worktree-mutation`, `status`

| Command | gix API | Notes |
|---|---|---|
| **status** | `repo.status(…)` → `gix::Status` | Single API call returns staged, unstaged, untracked, ignored. `.gitignore` evaluated automatically. |
| **diff** | `repo.diff_tree_to_tree(…)`, `gix::diff::DetectRenames` | Diff any two trees (HEAD↔index, index↔worktree, HEAD↔HEAD~1). Can detect renames. |
| **log** | `gix::revision::walk()` via `gix-revwalk` | Walk commits in topological/date order. Filter by path, author, etc. Format with `gix-actor`. |
| **add** | `repo.index()` → modify entries → `index.write()` | Stage files by updating index entries. Handle `.gitignore` for warnings. |
| **commit** | `repo.index().write_tree()` → `gix::Object::write()` → `repo.refs().set_referent()` | Write tree from index, create commit object, update HEAD. |
| **branch** | `repo.refs().iter()`, `repo.find_reference(…)`, `repo.refs().set_referent()` | List/create/delete/rename branches. Read upstream via `branch.upstream()`. |
| **tag** | `repo.refs().set_referent()` (lightweight), `gix::Tag::write()` (annotated) | Create annotated tags with message + signer. |
| **checkout** | `repo.head().set_target()` + `gix-worktree-state::checkout()` | Update HEAD, write index, materialize files to working tree. |
| **restore** | `repo.checkout()` with paths or `gix-worktree-state` partial checkout | Restore working tree files from index or a tree-ish. |
| **reset** | `repo.head().set_target()` + `index.write_tree_from_diff()` | Soft/mixed/hard reset via ref update + index + worktree. |
| **clean** | `gix::dir::walk()` for listing, `std::fs::remove_file()` for removal | Use dirwalk to list untracked files, then delete them. |
| **mv** | `std::fs::rename()` + `index.remove()` + `index.add()` | Move file on disk, update index entries. |
| **init** | `gix::create::into()` or manual `.git/` creation | Create HEAD, `refs/heads/`, config with `gix-config`. |
| **clone** | `gix-protocol::fetch()` + `gix-worktree-state::checkout()` | Fetch from remote, checkout HEAD. Already works via `blocking-network-client`. |
| **fetch** | `gix-protocol::fetch()` + `gix-refspec::match_and_update()` | Fetch refs from remote, update remote tracking branches. |
| **push** | `gix-protocol::push()` | Push local refs to remote. Supports atomic push. |
| **remote** | `gix-config` read/write on `remote.*` keys | List/add/remove/rename remotes. |

### ✅ Available by enabling gix feature flags

Add features to the `gix` dependency in `Cargo.toml`:

```toml
gix = { features = [
    "basic", "sha1",
    "blocking-http-transport-reqwest-rust-tls",
    "worktree-mutation", "status",
    "blame",        # adds gix-blame
    "merge",        # adds gix-merge (3-way tree merge)
    "serde",        # json serialization for gix types
] }
```

| Command | Feature flag | gix API | Notes |
|---|---|---|---|
| **blame** | `blame` | `gix::blame::Blob::blame()` | Line-by-line annotation. Supports textconv. |
| **merge** | `merge` | `gix::merge::merge_tree()` | 3-way tree-level merge (recursive strategy). Returns merge result with conflicts. |

### ❌ Not available in gix

These need either `jj-lib` or a hand-written implementation:

| Command | Gap | Workaround |
|---|---|---|
| **rebase** | No sequencer/transaction engine | Hand-write a cherry-pick loop using `gix-merge` + `gix-object` commit creation (~100 lines) |
| **cherry-pick** | Same as rebase | `gix-merge` tree merge + create commit (~30 lines) |
| **stash** | Temporary commit + checkout + pop | Stash as a ref (`refs/stash`), push/pop with commit + reset (~60 lines) |
| **bisect** | No bisect state machine | Walk commits, binary search with ref updates (~80 lines) |
| **submodule** | `gix-submodule` is informational only | No easy solution without shelling out to `git` |

---

## Why gix is the right choice for `gor status`

### What we gain vs. jj-lib

| Concern | jj-lib | gix |
|---|---|---|
| **Dependencies** | +30+ transitive crates | Already present |
| **Compile time** | + many seconds | Zero additional |
| **Async required** | Yes — need tokio runtime + block_on bridge | No — fully synchronous |
| **`.jj/` directory** | Created in user repo | Never touches `.jj/` |
| **Speed** | `snapshot()` walks full tree every time | Index-based with lstat caching — fast |
| **Stability** | 0.x, no stability guarantees | 0.85, mature, gitoxide project is well-established |

### What we lose vs. jj-lib

- **No rebase/cherry-pick** — but gor is a GitHub CLI, not a full git replacement
- **No conflict display** — `gix-merge` can detect them; display is straightforward
- **No revset language** — `gix-revision` gives merge-base and describe, which covers log use cases

For `gor`'s mission as a **GitHub CLI**, the local git operations needed are:
`status` → `diff` → `add` → `commit` → `push` (and maybe `log` + `blame`).

None of these require rebase, cherry-pick, or a change-based model.

---

## gix API Quick Reference for status

```rust
use gix::{Repository, Status};

// Open the repo
let repo = gix::discover(".")?;

// Get working tree status
let status = repo.status(gix::worktree::Status::default())?;

// Branch info
let head = repo.head()?;
let branch_name = head.name().map(|n| n.as_bstr().to_string());
let upstream = repo
    .find_reference(branch_name)?
    .into_fully_peeled_id()?
    .object()?
    .into_commit();

// Ahead/behind
let graph = repo.graph()?;
let (ahead, behind) = graph.ahead_behind(&local_oid, &upstream_oid)?;

// Staged changes (index vs HEAD)
let staged = status.staged()?;

// Unstaged changes (working tree vs index)
let unstaged = status.unstaged()?;

// Untracked files
let untracked = status.untracked()?;

// Conflicted entries (from index)
let index = repo.index()?;
let conflicted: Vec<_> = index
    .entries()
    .iter()
    .filter(|e| e.stage() > 0)
    .map(|e| e.path().to_string())
    .collect();
```

---

## Recommendation

**Use `gix` as the sole VCS engine for `gor status` and all near-term git commands.** 

Add `jj-lib` later only if a specific command requires it (e.g., `gor rebase`). The gix-first approach:

1. Requires zero new dependencies today
2. Has no `.jj/` directory side effects
3. Is fully synchronous (no tokio runtime)
4. Uses git's index-based caching (fast on large repos)
5. Is already proven in gor's existing clone/fetch code paths
