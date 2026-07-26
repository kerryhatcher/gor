//! Git repository wrapper backed by `gix` (gitoxide).
//!
//! Provides a [`GitRepo`] struct that wraps a `gix::Repository` and
//! exposes high-level VCS operations like status.

use std::path::Path;

use gix::bstr::BString;
use gix::index::entry::Stage;

use crate::vcs::types::{ChangeType, FileStatus, VcsError, WorkingTreeStatus};

/// A local git repository backed by `gix` (gitoxide).
///
/// Uses [`gix::discover`] to find the repository root by walking up
/// parent directories from a given path.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use gor::vcs::GitRepo;
///
/// let repo = GitRepo::open(Path::new(".")).expect("should find a git repo");
/// let status = repo.status().expect("status should succeed");
/// ```
pub struct GitRepo {
    repo: gix::Repository,
}

impl GitRepo {
    /// Open or discover a git repository from the given directory.
    ///
    /// Walks up parent directories looking for a `.git/` directory.
    ///
    /// # Errors
    ///
    /// Returns [`VcsError::NotARepository`] if no git repository is found.
    /// Returns [`VcsError::Io`] if an I/O error occurs during discovery.
    pub fn open(path: &Path) -> Result<Self, VcsError> {
        let repo = gix::discover(path).map_err(|e| VcsError::NotARepository(format!("{e}")))?;
        Ok(Self { repo })
    }

    /// Get the full working tree status in a single pass.
    ///
    /// Returns staged, unstaged, untracked, and conflicted files along
    /// with branch information.
    ///
    /// # Errors
    ///
    /// Returns [`VcsError::Other`] if a git operation fails.
    /// Returns [`VcsError::Io`] if an I/O error occurs.
    pub fn status(&self) -> Result<WorkingTreeStatus, VcsError> {
        let branch = self.get_branch_name();
        let (upstream, ahead, behind) = self.get_upstream_info(&branch);
        let (staged, unstaged, untracked) = self.collect_status_changes()?;
        let conflicted = self.get_conflicted_files()?;

        Ok(WorkingTreeStatus {
            branch: if branch.is_empty() {
                None
            } else {
                Some(branch)
            },
            upstream,
            ahead,
            behind,
            staged,
            unstaged,
            untracked,
            conflicted,
        })
    }

    /// Get the current branch name, or an empty string if detached HEAD.
    fn get_branch_name(&self) -> String {
        self.repo
            .head()
            .ok()
            .map(|head| head.name().as_bstr().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_default()
    }

    /// Get upstream tracking info for the current branch.
    fn get_upstream_info(&self, branch_name: &str) -> (Option<String>, u32, u32) {
        if branch_name.is_empty() {
            return (None, 0, 0);
        }

        let branch_ref = format!("refs/heads/{branch_name}");
        let Ok(reference) = self.repo.find_reference(&branch_ref) else {
            return (None, 0, 0);
        };

        let upstream = self
            .repo
            .branch_remote_tracking_ref_name(
                reference.inner.name.as_ref(),
                gix::remote::Direction::Fetch,
            )
            .and_then(Result::ok)
            .map(|cow| cow.to_string());

        // Ahead/behind requires commit walking; simplified to (0, 0) for now.
        (upstream, 0, 0)
    }

    /// Collect staged, unstaged, and untracked changes from the status iterator.
    #[allow(clippy::type_complexity)]
    fn collect_status_changes(
        &self,
    ) -> Result<(Vec<FileStatus>, Vec<FileStatus>, Vec<String>), VcsError> {
        let mut staged = Vec::new();
        let mut unstaged = Vec::new();
        let mut untracked = Vec::new();

        let platform = self
            .repo
            .status(gix::progress::Discard)
            .map_err(|e| VcsError::Other(format!("failed to create status platform: {e}")))?;

        let iter = platform
            .into_iter(Vec::<BString>::new())
            .map_err(|e| VcsError::Other(format!("failed to create status iterator: {e}")))?;

        for item in iter {
            let item = item.map_err(|e| VcsError::Other(format!("status iteration error: {e}")))?;
            match item {
                gix::status::Item::TreeIndex(change) => {
                    let location = change.location().to_string();
                    let change_type = match change {
                        gix::diff::index::ChangeRef::Addition { .. }
                        | gix::diff::index::ChangeRef::Rewrite { .. } => ChangeType::Added,
                        gix::diff::index::ChangeRef::Deletion { .. } => ChangeType::Deleted,
                        gix::diff::index::ChangeRef::Modification { .. } => ChangeType::Modified,
                    };
                    staged.push(FileStatus {
                        path: location,
                        status: change_type,
                    });
                }
                gix::status::Item::IndexWorktree(wt_item) => {
                    Self::collect_worktree_item(wt_item, &mut unstaged, &mut untracked);
                }
            }
        }

        Ok((staged, unstaged, untracked))
    }

    /// Process a single worktree status item, classifying it as unstaged or untracked.
    fn collect_worktree_item(
        item: gix::status::index_worktree::Item,
        unstaged: &mut Vec<FileStatus>,
        untracked: &mut Vec<String>,
    ) {
        match item {
            gix::status::index_worktree::Item::Modification {
                rela_path, status, ..
            } => {
                use gix::status::plumbing::index_as_worktree::{Change, EntryStatus};
                let path = rela_path.to_string();
                match status {
                    EntryStatus::Change(change) => {
                        let change_type = match change {
                            Change::Removed => ChangeType::Deleted,
                            Change::Modification { .. } | Change::SubmoduleModification(_) => {
                                ChangeType::Modified
                            }
                            Change::Type { .. } => ChangeType::TypeChange,
                        };
                        unstaged.push(FileStatus {
                            path,
                            status: change_type,
                        });
                    }
                    EntryStatus::IntentToAdd => {
                        unstaged.push(FileStatus {
                            path,
                            status: ChangeType::Added,
                        });
                    }
                    EntryStatus::Conflict { .. } => {
                        untracked.push(path);
                    }
                    EntryStatus::NeedsUpdate(_) => {
                        // Stat cache refresh, no user-visible change
                    }
                }
            }
            gix::status::index_worktree::Item::DirectoryContents { entry, .. } => {
                if entry.status == gix::dir::entry::Status::Untracked {
                    untracked.push(entry.rela_path.to_string());
                }
            }
            gix::status::index_worktree::Item::Rewrite {
                dirwalk_entry,
                source,
                ..
            } => {
                let path = dirwalk_entry.rela_path.to_string();
                let from = source.rela_path().to_string();
                unstaged.push(FileStatus {
                    path,
                    status: ChangeType::Renamed { from },
                });
            }
        }
    }

    /// Get the list of conflicted files from the index.
    fn get_conflicted_files(&self) -> Result<Vec<String>, VcsError> {
        let index = self
            .repo
            .index()
            .map_err(|e| VcsError::Other(format!("failed to open index: {e}")))?;
        let state: &gix::index::State = &index;
        let entries = state.entries();
        let conflicted: Vec<String> = entries
            .iter()
            .filter(|e| e.stage() != Stage::Unconflicted)
            .map(|e| e.path(state).to_string())
            .collect();
        Ok(conflicted)
    }
}
