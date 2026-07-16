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
    /// Manage configuration values.
    #[command(subcommand)]
    Config(ConfigCommand),
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
