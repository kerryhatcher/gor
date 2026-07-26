# Handoff: `gor status` via gix (gitoxide)

## Context

This worktree is a branch (`feat/jj-lib-integration`) off `main` for adding
git CLI capabilities to `gor` using **`gix` (gitoxide)** — already a
dependency — as the VCS engine.

The goal is to make `gor` a drop-in replacement for common `git` commands
while remaining a pure-Rust, no-OpenSSL, no-`git`-binary CLI. We start with
`gor status` as the first integration.

## Design Decision

We use **`gix` directly** rather than wrapping `jj-lib`. Rationale:

- `gix` is already a dependency (no new transitives, no compile-time cost)
- Fully synchronous — no tokio runtime, no async-to-sync bridge
- No `.jj/` directory written to user repos
- Git's index-based status is fast (lstat caching, not full tree walk)
- `gix` 0.85 is mature and the gitoxide project is well-established

See `docs/research/research-gix-for-gor.md` for full capability mapping.

**`jj-lib` is deferred** — it would only be added later if a specific
command genuinely needs its rewrite/sequencer engine (e.g., `gor rebase`).

## Reference Documents

| Document | Location | Contents |
|---|---|---|
| gix-for-gor research | `docs/research/research-gix-for-gor.md` | Full capability mapping, API reference, gix-vs-jj-lib comparison |
| gor status spec | `docs/issues/status.md` | Full story for `gor status`: acceptance criteria, CLI design, output formats, implementation architecture, testing strategy |

## High-Level Implementation Plan

### Phase 0: VCS module scaffold (no new dependencies)

No `Cargo.toml` changes needed — `gix` is already enabled with all
features required for status:

```toml
# Already in Cargo.toml:
gix = { version = "0.85", default-features = false, features = [
    "basic", "sha1",
    "blocking-http-transport-reqwest-rust-tls",
    "worktree-mutation", "status",
] }
```

1. **Create `src/vcs/mod.rs`** — the new VCS module. No async runtime
   needed; `gix` is synchronous.
   ```rust
   // src/vcs/mod.rs
   //! Git VCS operations backed by gix (gitoxide).
   mod repo;
   mod types;
   pub use repo::GitRepo;
   pub use types::{WorkingTreeStatus, FileStatus, ChangeType};
   ```

2. **Create `src/vcs/types.rs`** — status types (unchanged from earlier
   plan, these are format-agnostic):
   ```rust
   pub struct WorkingTreeStatus {
       pub branch: Option<String>,
       pub upstream: Option<String>,
       pub ahead: u32,
       pub behind: u32,
       pub staged: Vec<FileStatus>,
       pub unstaged: Vec<FileStatus>,
       pub untracked: Vec<String>,
       pub conflicted: Vec<String>,
   }

   pub struct FileStatus {
       pub path: String,
       pub status: ChangeType,
   }

   pub enum ChangeType {
       Modified, Added, Deleted, TypeChange, Renamed { from: String },
   }
   ```

3. **Create `src/vcs/repo.rs`** — the `GitRepo` struct wrapping `gix`:
   ```rust
   use gix::{Repository, Status};

   pub struct GitRepo {
       repo: Repository,
   }

   impl GitRepo {
       /// Open or discover a git repository from the given directory.
       /// Walks up parent directories looking for `.git/`.
       pub fn open(path: &Path) -> Result<Self, VcsError> {
           Ok(Self { repo: gix::discover(path).map_err(VcsError::Git)? })
       }

       /// Get the full working tree status in one pass.
       pub fn status(&self) -> Result<WorkingTreeStatus, VcsError> {
           let status = self.repo
               .status(gix::worktree::Status::default())?;

           let branch = /* repo.head() -> name */;
           let upstream = /* branch.upstream() -> remote ref */;
           let (ahead, behind) = /* graph.ahead_behind() */;
           let conflicted = /* index entries with stage > 0 */;

           Ok(WorkingTreeStatus {
               branch,
               upstream,
               ahead,
               behind,
               staged: status.staged()?.map(/* to FileStatus */).collect(),
               unstaged: status.unstaged()?.map(/* to FileStatus */).collect(),
               untracked: status.untracked()?.map(|e| e.path).collect(),
               conflicted,
           })
       }
   }
   ```

   - `gix::discover()` walks up directories to find `.git/` — no `.jj/` needed
   - `repo.status()` is a single, index-based pass — no `snapshot()` overhead
   - `.gitignore` is evaluated automatically by the dirwalk

4. **Register the module** in `src/lib.rs`:
   ```rust
   pub mod vcs;
   ```

   No feature gate — `gix` is always available, and adding git commands to
   a GitHub CLI is a feature, not a dependency burden.

### Phase 1: CLI + dispatch

5. **Add `Status` variant** to `Command` enum in `src/cli.rs`:
   ```rust
   /// Show working tree status.
   #[command(name = "status")]
   Status(StatusCommand),
   ```
   Copy the `StatusCommand` struct from `docs/issues/status.md`.

6. **Create `src/cmd/status.rs`** — the command handler:
   ```rust
   pub fn run(cmd: StatusCommand) -> anyhow::Result<()> {
       let repo = vcs::GitRepo::open(&std::env::current_dir()?)?;
       let status = repo.status()?;
       output::print_status(&status, &cmd);
       Ok(())
   }
   ```

7. **Wire dispatch** in `src/cmd/mod.rs` — add the match arm:
   ```rust
   Command::Status(cmd) => status::run(cmd),
   ```

### Phase 2: Output formatting

8. **Add status formatting** to `src/output.rs`:
   - `print_status()` — default format (git-style grouped output)
   - `print_status_short()` — `--short` format
   - `print_status_json()` — `--json` format

   Use `insta` snapshot tests for each format variant.

### Phase 3: Testing

9. **Integration tests** in `tests/status_integration.rs`:
   - Create temp git repos with `gix` at known states
   - Run `gor status` via `assert_cmd`
   - Snapshot the output

## Key Design Decisions

- **No feature gate.** Unlike jj-lib (which would need one to avoid
  pulling in 30+ transitives), `gix` is already compiled. The VCS module
  should always be available.
- **`gix::discover()` over `gix::open()`.** Walk up for `.git/` so `gor
  status` works from any subdirectory, just like `git status`.
- **Single `gix::status` platform pass.** Use `repo.status(Discard)` to
  obtain a `Platform`, then consume it via `into_iter()`. The iterator
  yields `Item::TreeIndex` (staged) and `Item::IndexWorktree` (unstaged +
  untracked) in one pass. Don't try to reconstruct status with separate
  API calls.
- **Index-based, not snapshot-based.** The status platform uses lstat to
  check file modification times against the index, skipping unchanged
  files. No full tree walk unless something changed.
- **Rename detection off by default.** `gix::diff::DetectRenames` is
  opt-in via `--renames` for performance.

## Risks and Gotchas

- `gix::discover()` respects `$GIT_DIR` and `$GIT_DISCOVERY_ACROSS_FILESYSTEM`.
  Test edge cases with environment variables.
- **No `staged()/unstaged()/untracked()` accessors.** gix 0.85's status
  API is iterator-based. Consume via `Platform::into_iter()` and classify
  each `Item` by variant.
- `gix::Status` returns paths relative to the working tree root, not the
  current directory. Normalize for display (like `git status` does).
- Stat-caching means `gix` may miss files touched by external tools if
  mtime isn't updated. Falls back to content comparison.
- `gix` 0.85 is still pre-1.0. Pin the exact version (already done).
- Conflicted files come from `gix::Index::entries()` with
  `stage() != Stage::Unconflicted`, not from the status iterator.
  Deduplicate via sort + dedup since multiple stages exist per path.

## First Steps for the Agent

1. `cd /home/kwhatcher/projects/gor-jj-integration`
2. Read the two reference docs above
3. Start with Phase 0: create the `vcs` module (`mod.rs`, `types.rs`, `repo.rs`)
4. Run `cargo build` to validate compilation
5. Move to Phase 1: add the CLI arg + dispatch
6. Iterate

Refer to the existing command implementations
(`src/cmd/repo.rs`, `src/cmd/pr.rs`) for patterns around CLI args, error
handling, and output formatting.

## Contact / Commit Convention

Use conventional commits with scope `vcs` for git VCS work:
- `feat(vcs): add gix-based GitRepo wrapper`
- `feat(status): implement gor status command`
- `test(status): add integration tests for working tree status`
