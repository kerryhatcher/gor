//! Git VCS operations backed by `gix` (gitoxide).
//!
//! This module provides a pure-Rust interface to local git repositories,
//! wrapping the `gix` crate for operations like status, diff, log, and
//! other common git porcelain commands.
//!
//! # Examples
//!
//! ```no_run
//! use std::path::Path;
//! use gor::vcs::GitRepo;
//!
//! let repo = GitRepo::open(Path::new(".")).expect("should find a git repo");
//! let status = repo.status().expect("status should succeed");
//! println!("On branch: {:?}", status.branch);
//! ```

mod repo;
mod types;

pub use repo::GitRepo;
pub use types::{ChangeType, FileStatus, VcsError, WorkingTreeStatus};
