//! Types for VCS (git) operations.
//!
//! Defines the status types and error types used by the VCS module.

/// Errors from VCS operations.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum VcsError {
    /// Error when the directory is not a git repository.
    #[error("not a git repository: {0}")]
    NotARepository(String),

    /// A git operation failed with an unexpected error.
    #[error("git operation failed: {0}")]
    Other(String),

    /// IO error during file or network access.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Complete working tree status.
#[derive(Debug, Clone, serde::Serialize)]
pub struct WorkingTreeStatus {
    /// The name of the current branch (if any).
    pub branch: Option<String>,
    /// The upstream tracking branch (if any).
    pub upstream: Option<String>,
    /// Number of commits ahead of the upstream.
    pub ahead: u32,
    /// Number of commits behind the upstream.
    pub behind: u32,
    /// List of staged changes.
    pub staged: Vec<FileStatus>,
    /// List of unstaged changes.
    pub unstaged: Vec<FileStatus>,
    /// List of untracked files.
    pub untracked: Vec<String>,
    /// List of conflicted files.
    pub conflicted: Vec<String>,
}

/// A single changed file with its status.
#[derive(Debug, Clone, serde::Serialize)]
pub struct FileStatus {
    /// Path to the file inside the repository.
    pub path: String,
    /// The type of change for this file.
    #[serde(flatten)]
    pub status: ChangeType,
}

/// The type of change for a file.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "status")]
pub enum ChangeType {
    /// File has been modified.
    #[serde(rename = "modified")]
    Modified,
    /// New file added to the index/working tree.
    #[serde(rename = "added")]
    Added,
    /// File removed from the repository.
    #[serde(rename = "deleted")]
    Deleted,
    /// File type changed (e.g., symlink vs regular file).
    #[serde(rename = "typechange")]
    TypeChange,
    /// File was renamed or moved.
    #[serde(rename = "renamed")]
    Renamed {
        /// The old path of the renamed file.
        from: String,
    },
}
