//! CLI argument definitions using clap's derive API.
//!
//! Defines the top-level [`Args`] struct and the [`Command`] enum
//! for all subcommands.

use clap::{Parser, Subcommand};

/// A fast, self-contained GitHub CLI written in Rust.
#[derive(Parser, Debug)]
#[command(
    name = "gor",
    version,
    about = "GitHub on Rust — a fast, self-contained GitHub CLI",
    long_about = "A fast, self-contained GitHub CLI written in Rust. Equivalent to `gh` but with no external dependencies on `git` or OpenSSL."
)]
pub struct Args {
    /// GitHub hostname for GitHub Enterprise Server (default: github.com).
    #[arg(long, env = "GH_HOST", global = true)]
    pub hostname: Option<String>,

    /// Subcommand to execute.
    #[command(subcommand)]
    pub command: Command,
}

/// Top-level subcommands for `gor`.
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Authenticate with a GitHub account.
    #[command(subcommand)]
    Auth(AuthCommand),
    /// Work with GitHub repositories.
    #[command(subcommand)]
    Repo(RepoCommand),
    /// Work with GitHub pull requests.
    #[command(subcommand)]
    Pr(PrCommand),
    /// Work with GitHub issues.
    #[command(subcommand)]
    Issue(IssueCommand),
    /// Make an authenticated GitHub API request.
    Api(ApiCommand),
    /// Manage configuration values.
    #[command(subcommand)]
    Config(ConfigCommand),
}

/// Arguments for `gor api`.
#[derive(clap::Args, Debug)]
pub struct ApiCommand {
    /// The API endpoint path (e.g. /repos/owner/repo).
    pub endpoint: String,

    /// HTTP method (default: GET).
    #[arg(short = 'X', long, default_value = "GET")]
    pub method: String,

    /// Add a typed field parameter (key=value) for the request body.
    #[arg(short = 'F', long = "field")]
    pub fields: Vec<String>,

    /// Add a raw field parameter (key=value) for the request body.
    #[arg(short = 'f', long = "raw-field")]
    pub raw_fields: Vec<String>,

    /// Add a custom HTTP header (key: value).
    #[arg(short = 'H', long = "header")]
    pub headers: Vec<String>,

    /// Read the request body from a file (use @- for stdin).
    #[arg(long)]
    pub input: Option<String>,

    /// Follow Link headers to fetch all pages.
    #[arg(long)]
    pub paginate: bool,

    /// GitHub hostname for GitHub Enterprise Server (default: github.com).
    #[arg(long, env = "GH_HOST")]
    pub hostname: Option<String>,

    /// Filter JSON output with a jq expression.
    #[arg(long)]
    pub jq: Option<String>,

    /// Format output via a Go/Handlebars template.
    #[arg(long)]
    pub template: Option<String>,

    /// Suppress status output.
    #[arg(long)]
    pub silent: bool,

    /// Include response headers in the output.
    #[arg(short = 'i', long)]
    pub include: bool,
}

/// Subcommands for `gor repo`.
#[derive(Subcommand, Debug)]
pub enum RepoCommand {
    /// View a repository's description, stats, and metadata.
    View {
        /// Repository to view (OWNER/REPO format). Auto-detected from git remote if omitted.
        owner_repo: Option<String>,
        /// Open the repository in the default browser.
        #[arg(short = 'w', long)]
        web: bool,
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
}

/// Subcommands for `gor auth`.
#[derive(Subcommand, Debug)]
pub enum AuthCommand {
    /// Log in to a GitHub account using the OAuth device flow.
    Login {
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
        /// Request specific OAuth scopes (comma-separated).
        /// Default: repo,read:org,workflow,gist
        #[arg(long)]
        scopes: Option<String>,
        /// Read token from stdin instead of starting the device flow.
        #[arg(long, conflicts_with = "scopes")]
        with_token: bool,
    },
    /// Log out of a GitHub account.
    Logout {
        /// GitHub hostname (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Show the current authentication status.
    Status {
        /// GitHub hostname (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Configure git to use gor as a credential helper.
    SetupGit {
        /// GitHub hostname (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
}

/// Subcommands for `gor pr`.
#[derive(Subcommand, Debug)]
pub enum PrCommand {
    /// List pull requests in a repository.
    List {
        /// Repository to list PRs for (OWNER/REPO format). Auto-detected from git remote if omitted.
        owner_repo: Option<String>,
        /// Filter by state: open, closed, merged, or all (default: open).
        #[arg(long, default_value = "open")]
        state: String,
        /// Filter by base branch.
        #[arg(long)]
        base: Option<String>,
        /// Filter by head branch.
        #[arg(long)]
        head: Option<String>,
        /// Filter by PR author login.
        #[arg(long)]
        author: Option<String>,
        /// Filter by label (repeatable).
        #[arg(long = "label")]
        labels: Vec<String>,
        /// Filter by assignee login.
        #[arg(long)]
        assignee: Option<String>,
        /// Maximum number of PRs to show (default: 30).
        #[arg(short = 'L', long, default_value = "30")]
        limit: u32,
        /// Open the PR list in the default browser.
        #[arg(short = 'w', long)]
        web: bool,
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// View details of a pull request.
    View {
        /// Pull request number.
        number: u64,
        /// Repository to view PR from (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Open the PR in the default browser.
        #[arg(short = 'w', long)]
        web: bool,
        /// Include the PR's comment thread.
        #[arg(long)]
        comments: bool,
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
}

/// Subcommands for `gor issue`.
#[derive(Subcommand, Debug)]
pub enum IssueCommand {
    /// List issues in a repository.
    List {
        /// Repository to list issues for (OWNER/REPO format). Auto-detected from git remote if omitted.
        owner_repo: Option<String>,
        /// Filter by state: open, closed, or all (default: open).
        #[arg(long, default_value = "open")]
        state: String,
        /// Filter by label (repeatable).
        #[arg(long = "label")]
        labels: Vec<String>,
        /// Filter by assignee login.
        #[arg(long)]
        assignee: Option<String>,
        /// Filter by issue author login.
        #[arg(long)]
        author: Option<String>,
        /// Filter by @mention.
        #[arg(long)]
        mention: Option<String>,
        /// Filter by milestone title or number.
        #[arg(long)]
        milestone: Option<String>,
        /// Maximum number of issues to show (default: 30).
        #[arg(short = 'L', long, default_value = "30")]
        limit: u32,
        /// Open the issue list in the default browser.
        #[arg(short = 'w', long)]
        web: bool,
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// View details of an issue.
    View {
        /// Issue number.
        number: u64,
        /// Repository to view issue from (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Open the issue in the default browser.
        #[arg(short = 'w', long)]
        web: bool,
        /// Include the issue's comment thread.
        #[arg(long)]
        comments: bool,
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Close an issue.
    Close {
        /// Issue number.
        number: u64,
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Add a closing comment.
        #[arg(long)]
        comment: Option<String>,
        /// Reason for closing: completed or not_planned.
        #[arg(long, value_parser = ["completed", "not_planned"])]
        reason: Option<String>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Reopen a closed issue.
    Reopen {
        /// Issue number.
        number: u64,
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Add a comment when reopening.
        #[arg(long)]
        comment: Option<String>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Add a comment to an issue.
    Comment {
        /// Issue number.
        number: u64,
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Comment body (markdown supported).
        #[arg(long, conflicts_with = "body_file")]
        body: Option<String>,
        /// Read comment body from file (use @- for stdin).
        #[arg(long, conflicts_with = "body")]
        body_file: Option<String>,
        /// Open the issue in the default browser after commenting.
        #[arg(short = 'w', long)]
        web: bool,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
}

/// Subcommands for `gor config`.
#[derive(Subcommand, Debug)]
pub enum ConfigCommand {
    /// Get a config value.
    Get {
        /// Config key to read.
        key: String,
    },
    /// Set a config value.
    Set {
        /// Config key to set.
        key: String,
        /// Value to set.
        value: String,
    },
    /// List all config values.
    List,
}
