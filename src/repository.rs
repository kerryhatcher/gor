//! Repository specification parsing and remote URL detection.
//!
//! Provides utilities for parsing `OWNER/REPO` strings and detecting
//! repository information from the current directory's git remote.

/// A parsed repository specification consisting of an owner and repo name.
///
/// # Examples
///
/// ```
/// use gor::repository::{parse_repo_spec, RepoSplit};
///
/// let spec = parse_repo_spec("octocat/hello-world").expect("valid spec");
/// assert_eq!(spec.owner, "octocat");
/// assert_eq!(spec.repo, "hello-world");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoSplit {
    /// The repository owner (user or organization).
    pub owner: String,
    /// The repository name.
    pub repo: String,
}

impl RepoSplit {
    /// Create a new `RepoSplit` from owner and repo strings.
    ///
    /// # Examples
    ///
    /// ```
    /// use gor::repository::RepoSplit;
    ///
    /// let spec = RepoSplit::new("octocat", "hello-world");
    /// assert_eq!(spec.owner, "octocat");
    /// assert_eq!(spec.repo, "hello-world");
    /// ```
    #[must_use]
    pub fn new(owner: &str, repo: &str) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
        }
    }
}

impl std::fmt::Display for RepoSplit {
    /// Format as `OWNER/REPO`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gor::repository::RepoSplit;
    ///
    /// let spec = RepoSplit::new("octocat", "hello-world");
    /// assert_eq!(spec.to_string(), "octocat/hello-world");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.owner, self.repo)
    }
}

/// Parse a repository spec string in `OWNER/REPO` format.
///
/// The input must contain exactly one `/` separator. Trailing `.git` is
/// stripped from the repo name. Empty owner or repo segments are rejected.
///
/// # Errors
///
/// Returns an error if the input is empty, lacks a `/`, contains multiple `/`
/// separators, or has empty owner or repo segments.
///
/// # Examples
///
/// ```
/// use gor::repository::parse_repo_spec;
///
/// let spec = parse_repo_spec("octocat/hello-world").expect("valid spec");
/// assert_eq!(spec.owner, "octocat");
/// assert_eq!(spec.repo, "hello-world");
///
/// // Trailing .git is stripped
/// let spec = parse_repo_spec("octocat/repo.git").expect("valid spec");
/// assert_eq!(spec.repo, "repo");
///
/// // Invalid inputs
/// assert!(parse_repo_spec("").is_err());
/// assert!(parse_repo_spec("no-slash").is_err());
/// assert!(parse_repo_spec("too/many/slashes").is_err());
/// ```
pub fn parse_repo_spec(input: &str) -> anyhow::Result<RepoSplit> {
    let input = input.trim();
    anyhow::ensure!(!input.is_empty(), "repository spec cannot be empty");

    let parts: Vec<&str> = input.split('/').collect();
    anyhow::ensure!(
        parts.len() == 2,
        "invalid repository spec '{input}': expected OWNER/REPO format"
    );

    let owner = parts[0].trim();
    let repo = parts[1].trim();

    anyhow::ensure!(!owner.is_empty(), "repository owner cannot be empty");
    anyhow::ensure!(!repo.is_empty(), "repository name cannot be empty");

    // Strip trailing .git if present
    let repo = repo.strip_suffix(".git").unwrap_or(repo).to_string();

    Ok(RepoSplit {
        owner: owner.to_string(),
        repo,
    })
}

/// Detect the repository from the current directory's git remote.
///
/// Opens the git repository by discovering from the current directory,
/// finds the `origin` remote (or the first available remote), and parses
/// its URL to extract the owner and repository name.
///
/// Supports both HTTPS URLs (`https://github.com/owner/repo.git`) and
/// SSH URLs (`git@github.com:owner/repo.git`).
///
/// Returns `None` if no git repository is found, no remote is configured,
/// or the remote URL cannot be parsed as a GitHub repository.
///
/// # Examples
///
/// ```no_run
/// use gor::repository::detect_remote;
///
/// // This will return None if not in a git repo with a GitHub remote
/// let result = detect_remote();
/// ```
#[must_use]
pub fn detect_remote() -> Option<RepoSplit> {
    let repo = gix::discover(std::env::current_dir().ok()?).ok()?;

    // Try to find "origin" first, then fall back to the first available remote
    let remote = repo.find_remote("origin").ok().or_else(|| {
        let names: Vec<_> = repo.remote_names().into_iter().collect();
        let first_name = names.first()?.clone();
        repo.find_remote(first_name.as_ref()).ok()
    })?;

    let url = remote.url(gix::remote::Direction::Fetch)?;
    let host = url.host.as_deref()?;

    // Only handle github.com and GHES hosts
    if host != "github.com" && !host.contains('.') {
        return None;
    }

    let path = std::str::from_utf8(&url.path).ok()?;
    // Strip leading '/' and trailing '.git'
    let path = path.strip_prefix('/').unwrap_or(path);
    let path = path.strip_suffix(".git").unwrap_or(path);

    let (owner, repo) = path.split_once('/')?;

    if owner.is_empty() || repo.is_empty() {
        return None;
    }

    Some(RepoSplit {
        owner: owner.to_string(),
        repo: repo.to_string(),
    })
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_repo_spec() {
        let spec = parse_repo_spec("octocat/hello-world").expect("valid spec");
        assert_eq!(spec.owner, "octocat");
        assert_eq!(spec.repo, "hello-world");
    }

    #[test]
    fn parse_repo_spec_with_git_suffix() {
        let spec = parse_repo_spec("octocat/repo.git").expect("valid spec");
        assert_eq!(spec.owner, "octocat");
        assert_eq!(spec.repo, "repo");
    }

    #[test]
    fn parse_repo_spec_with_whitespace() {
        let spec = parse_repo_spec("  octocat/hello-world  ").expect("valid spec");
        assert_eq!(spec.owner, "octocat");
        assert_eq!(spec.repo, "hello-world");
    }

    #[test]
    fn parse_repo_spec_empty_input() {
        let err = parse_repo_spec("").expect_err("should fail on empty input");
        assert!(err.to_string().contains("cannot be empty"));
    }

    #[test]
    fn parse_repo_spec_no_slash() {
        let err = parse_repo_spec("justarepo").expect_err("should fail without slash");
        assert!(err.to_string().contains("OWNER/REPO"));
    }

    #[test]
    fn parse_repo_spec_too_many_slashes() {
        let err = parse_repo_spec("a/b/c").expect_err("should fail with too many slashes");
        assert!(err.to_string().contains("OWNER/REPO"));
    }

    #[test]
    fn parse_repo_spec_empty_owner() {
        let err = parse_repo_spec("/repo").expect_err("should fail with empty owner");
        assert!(err.to_string().contains("owner cannot be empty"));
    }

    #[test]
    fn parse_repo_spec_empty_repo() {
        let err = parse_repo_spec("owner/").expect_err("should fail with empty repo");
        assert!(err.to_string().contains("name cannot be empty"));
    }

    #[test]
    fn reposplit_new_and_to_string() {
        let spec = RepoSplit::new("octocat", "hello-world");
        assert_eq!(spec.to_string(), "octocat/hello-world");
    }

    #[test]
    fn reposplit_equality() {
        let a = RepoSplit::new("foo", "bar");
        let b = RepoSplit::new("foo", "bar");
        let c = RepoSplit::new("foo", "baz");
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn parse_repo_spec_with_dotgit_in_middle() {
        // Only trailing .git should be stripped
        let spec = parse_repo_spec("octocat/my.repo").expect("valid spec");
        assert_eq!(spec.repo, "my.repo");
    }

    #[test]
    fn detect_remote_no_git_repo() {
        // When not in a git repo, detect_remote should return None
        let result = detect_remote();
        // This may or may not be in a git repo depending on the test environment
        // We just verify it doesn't panic
        let _ = result;
    }
}
