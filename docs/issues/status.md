---
tags: [vcs, git, status]
priority: P1
phase: 0
endpoints: []
status: todo
blockedBy: []
blocks: [diff, commit, add]
---

# Status — Working Tree Status

## As a

developer who wants to see what changed in my working directory

## I want

to run `gor status` and see a summary of changed, staged, untracked, and
conflicted files — just like `git status`

## Acceptance criteria

### Phase 1 — Basic status

1. Running `gor status` in a git repo displays:
   - **Changed (unstaged)** files with their change type (modified, deleted,
     added, type change)
   - **Staged** files (changes staged for commit)
   - **Untracked** files (new files not yet tracked)
   - **Conflicted** files (merge conflicts)
   - Files are grouped and labelled clearly (like `git status`)
2. Running outside a git repo prints a clear error
3. `--short` / `-s` flag prints the short/porcelain format (`M`, ` A`,
   `??`, etc.) one file per line, matching `git status --short`
4. `--branch` / `-b` flag (can combine with `--short`) prints the branch
   name and tracking relationship, matching `git status --short --branch`
5. `--ignored` flag shows ignored files (hidden by default)
6. `--renames` flag detects renames (default: off for performance)
7. `--json` flag outputs structured JSON with per-file status, matching
   gor's cross-cutting `--json` convention
8. Exit code 0 on success, non-zero on error

### Phase 2 — Rich output

9. `gor status` shows the number of insertions/deletions per file
10. `gor status` shows the current branch, its upstream tracking branch,
    and ahead/behind counts
11. `gor status` highlights conflicted files with visible markers and
    suggests commands to resolve

### Phase 3 — Integration

12. `--repo` / `-R OWNER/REPO` flag works inside git repos that have a
    GitHub remote (to associate the status with a repo context)
13. `gor status` respects `.gitignore` rules
14. `gor status` respects `core.excludesFile` and local exclude rules

## Out of scope

- Showing status for submodules (deferred)
- Interactive `gor add -i` or patch mode
- `gor status` as a GitHub issue/PR dashboard (that's a separate command,
  similar to `gh status`)

## Implementation notes

### Architecture

Add a new module `src/cmd/status.rs` integrating with a new `gor::vcs`
module that wraps `gix` (gitoxide) — already a dependency — as the VCS engine.

The status pipeline:

```
gor status (CLI args)
  └─ cmd::status::run()
       └─ vcs::GitRepo::open(".")       // gix::discover() — walks up for .git/
            └─ vcs::GitRepo::status()    // gix::Repository::status()
                 ├─ Changed files        // status.staged() + status.unstaged()
                 ├─ Staged files         // status.staged() (index vs HEAD)
                 ├─ Untracked files      // status.untracked() (with .gitignore)
                 └─ Conflicted files     // repo.index().entries() where stage > 0
                      └─ output::print_status()  // gor's output module
```

### Key gix integration points

- `gix::discover()` — walk up from `path` to find `.git/` (handles subdirectories)
- `gix::Repository::status()` — single-pass status collecting staged,
  unstaged, untracked, and ignored files. Uses index-based stat caching
  for performance (no full tree walk).
- `gix::Repository::head()` — current HEAD reference, branch name
- `gix::Repository::find_reference()` — upstream tracking branch
- `gix::Repository::graph().ahead_behind()` — ahead/behind counts
- `gix::Repository::index().entries().filter(stage > 0)` — conflicted files
- `gix::diff::DetectRenames` — optional rename detection (off by default)

### No new dependencies needed

`gix` is already in `Cargo.toml` with all features required:

```toml
gix = { version = "0.85", default-features = false, features = [
    "basic",
    "sha1",
    "blocking-http-transport-reqwest-rust-tls",
    "worktree-mutation",
    "status",
] }
```

No feature gate is needed — unlike `jj-lib` (which would pull in 30+
transitive crates), `gix` is already compiled.

### CLI definition (in `cli.rs`)

```rust
/// Show working tree status.
#[derive(clap::Args, Debug)]
pub struct StatusCommand {
    /// Show short/porcelain format.
    #[arg(short = 's', long)]
    pub short: bool,

    /// Show branch and tracking info (with --short).
    #[arg(short = 'b', long)]
    pub branch: bool,

    /// Show ignored files.
    #[arg(long)]
    pub ignored: bool,

    /// Detect renames (may be slow on large repos).
    #[arg(long)]
    pub renames: bool,

    /// Output as JSON. Optionally specify comma-separated field names.
    #[arg(long, num_args = 0.., value_delimiter = ',')]
    pub json: Option<Vec<String>>,

    /// Repository (OWNER/REPO format). Auto-detected from git remote.
    #[arg(short = 'R', long)]
    pub repo: Option<String>,
}
```

### Output format examples

Default output:
```
On branch feat/awesome-feature
Your branch is up to date with 'origin/feat/awesome-feature'.

Changes not staged for commit:
  (use "gor add <file>..." to update what will be committed)
  modified:   src/cmd/status.rs
  deleted:    src/old-module.rs

Changes staged for commit:
  (use "gor restore --staged <file>..." to unstage)
  modified:   Cargo.toml

Untracked files:
  (use "gor add <file>..." to include in what will be committed)
  docs/research/research-jj-lib.md
```

Short format:
```
## feat/awesome-feature...origin/feat/awesome-feature
 M src/cmd/status.rs
 D src/old-module.rs
M  Cargo.toml
?? docs/research/research-jj-lib.md
```

JSON format:
```json
{
  "branch": "feat/awesome-feature",
  "upstream": "origin/feat/awesome-feature",
  "ahead": 3,
  "behind": 0,
  "conflicted": [],
  "staged": [
    { "path": "Cargo.toml", "status": "modified" }
  ],
  "unstaged": [
    { "path": "src/cmd/status.rs", "status": "modified" },
    { "path": "src/old-module.rs", "status": "deleted" }
  ],
  "untracked": [
    "docs/research/research-jj-lib.md"
  ]
}
```

### Testing strategy

- **Unit tests:** Pure status formatting in `output.rs` with `insta` snapshots
- **Integration tests:** Create temp git repos with known states (dirty,
  staged, clean, conflicted) and verify `gor status` output using `assert_cmd`
- **Edge cases:** Empty repos, repos with submodules, repos with
  `.gitignore`, repos with merge conflicts, repos with no upstream
