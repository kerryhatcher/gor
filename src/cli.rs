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
    #[command(subcommand)]
    Api(ApiCommand),
    /// Manage configuration values.
    #[command(subcommand)]
    Config(ConfigCommand),
    /// Work with repository labels.
    #[command(subcommand)]
    Label(LabelCommand),
    /// Work with releases.
    #[command(subcommand)]
    Release(ReleaseCommand),
    /// Open a repository in the browser.
    Browse(BrowseCommand),
    /// Work with gists.
    #[command(subcommand)]
    Gist(GistCommand),
    /// Search GitHub.
    #[command(subcommand)]
    Search(SearchCommand),
    /// Work with GitHub Actions workflows.
    #[command(subcommand)]
    Workflow(WorkflowCommand),
    /// Manage command aliases.
    #[command(subcommand)]
    Alias(AliasCommand),
    /// Work with GitHub organizations.
    #[command(subcommand)]
    Org(OrgCommand),
    /// Manage SSH keys.
    #[command(subcommand)]
    SshKey(SshKeyCommand),
    /// Manage GPG keys.
    #[command(subcommand)]
    GpgKey(GpgKeyCommand),
    /// List and manage secrets.
    #[command(subcommand)]
    Secret(SecretCommand),
    /// List and manage Actions variables.
    #[command(subcommand)]
    Variable(VariableCommand),
    /// Work with GitHub Actions workflow runs.
    #[command(subcommand)]
    Run(RunCommand),
    /// List and manage repository caches.
    #[command(subcommand)]
    Cache(CacheCommand),
    /// Work with repository rulesets.
    #[command(subcommand)]
    Ruleset(RulesetCommand),
    /// Manage GitHub CLI extensions.
    #[command(subcommand)]
    Extension(ExtensionCommand),
    /// Work with GitHub Codespaces.
    #[command(subcommand)]
    Codespace(CodespaceCommand),
}

/// Arguments for `gor api`.
#[derive(clap::Subcommand, Debug)]
pub enum ApiCommand {
    /// Make a REST API request to a GitHub endpoint.
    Rest {
        /// The API endpoint path (e.g. /repos/owner/repo).
        endpoint: String,

        /// HTTP method (default: GET).
        #[arg(short = 'X', long, default_value = "GET")]
        method: String,

        /// Add a typed field parameter (key=value) for the request body.
        #[arg(short = 'F', long = "field")]
        fields: Vec<String>,

        /// Add a raw field parameter (key=value) for the request body.
        #[arg(short = 'f', long = "raw-field")]
        raw_fields: Vec<String>,

        /// Add a custom HTTP header (key: value).
        #[arg(short = 'H', long = "header")]
        headers: Vec<String>,

        /// Read the request body from a file (use @- for stdin).
        #[arg(long)]
        input: Option<String>,

        /// Follow Link headers to fetch all pages.
        #[arg(long)]
        paginate: bool,

        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,

        /// Filter JSON output with a jq expression.
        #[arg(long)]
        jq: Option<String>,

        /// Format output via a Go/Handlebars template.
        #[arg(long)]
        template: Option<String>,

        /// Suppress status output.
        #[arg(long)]
        silent: bool,

        /// Include response headers in the output.
        #[arg(short = 'i', long)]
        include: bool,
    },
    /// Make a GraphQL API request.
    Graphql {
        /// The GraphQL query string. Reads from stdin if omitted.
        #[arg(short = 'q', long = "query")]
        query: Option<String>,

        /// GraphQL variables as key=value pairs (repeatable).
        #[arg(short = 'F', long = "field")]
        fields: Vec<String>,

        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,

        /// Filter JSON output with a jq expression.
        #[arg(long)]
        jq: Option<String>,

        /// Format output via a Go/Handlebars template.
        #[arg(long)]
        template: Option<String>,
    },
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
    /// List repositories for a user or organization.
    List {
        /// Owner to list repos for (username or org). Defaults to authenticated user.
        #[arg(long)]
        owner: Option<String>,
        /// Filter by visibility: public, private, or all (default: all).
        #[arg(long, default_value = "all")]
        visibility: String,
        /// Filter by fork status: include, exclude, or only (default: include).
        #[arg(long, default_value = "include")]
        fork: String,
        /// Filter by primary language.
        #[arg(short = 'l', long)]
        language: Option<String>,
        /// Maximum number of repos to show (default: 30).
        #[arg(short = 'L', long, default_value = "30")]
        limit: u32,
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Clone a repository locally.
    Clone {
        /// Repository to clone (OWNER/REPO format or full URL).
        owner_repo: String,
        /// Target directory name.
        #[arg(short = 'd', long)]
        directory: Option<String>,
        /// Name of the upstream remote (default: upstream).
        #[arg(long, default_value = "upstream")]
        upstream_remote_name: String,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Create a repository.
    Create {
        /// Repository name.
        name: String,
        /// Repository description.
        #[arg(long)]
        description: Option<String>,
        /// Create a private repository.
        #[arg(long)]
        private: bool,
        /// Create an internal repository (GHES only).
        #[arg(long)]
        internal: bool,
        /// Organization to create the repository under.
        #[arg(long)]
        org: Option<String>,
        /// Template repository (OWNER/REPO).
        #[arg(long)]
        template: Option<String>,
        /// Clone the new repository locally after creation.
        #[arg(long)]
        clone: bool,
        /// Remote name when cloning (default: origin).
        #[arg(long, default_value = "origin")]
        remote: String,
        /// Push local content to the new repository.
        #[arg(long)]
        push: bool,
        /// Disable the wiki.
        #[arg(long)]
        disable_wiki: bool,
        /// Disable the issue tracker.
        #[arg(long)]
        disable_issues: bool,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Fork a repository.
    Fork {
        /// Repository to fork (OWNER/REPO format).
        owner_repo: String,
        /// Organization to fork into.
        #[arg(long)]
        org: Option<String>,
        /// Clone the fork locally after creation.
        #[arg(long)]
        clone: bool,
        /// Remote name when cloning (default: origin).
        #[arg(long, default_value = "origin")]
        remote: String,
        /// Custom name for the fork.
        #[arg(long)]
        fork_name: Option<String>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Delete a repository.
    Delete {
        /// Repository to delete (OWNER/REPO format).
        owner_repo: String,
        /// Skip confirmation prompt.
        #[arg(long)]
        yes: bool,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Edit a repository's settings.
    Edit {
        /// Repository to edit (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Repository description.
        #[arg(long)]
        description: Option<String>,
        /// Repository visibility: public, private, or internal.
        #[arg(long, value_parser = ["public", "private", "internal"])]
        visibility: Option<String>,
        /// Add a topic (repeatable).
        #[arg(long = "add-topic")]
        add_topic: Vec<String>,
        /// Remove a topic (repeatable).
        #[arg(long = "remove-topic")]
        remove_topic: Vec<String>,
        /// Default branch name.
        #[arg(long)]
        default_branch: Option<String>,
        /// Enable wiki (true/false).
        #[arg(long, value_parser = ["true", "false"])]
        enable_wiki: Option<String>,
        /// Enable issues (true/false).
        #[arg(long, value_parser = ["true", "false"])]
        enable_issues: Option<String>,
        /// Enable projects (true/false).
        #[arg(long, value_parser = ["true", "false"])]
        enable_projects: Option<String>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Transfer a repository to another user or organization.
    Transfer {
        /// Target user or organization.
        target: String,
        /// Repository to transfer (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// New name for the repository after transfer.
        #[arg(long)]
        new_name: Option<String>,
        /// Team slug to grant access in the new organization (repeatable).
        #[arg(long)]
        team: Vec<String>,
        /// Skip confirmation prompt.
        #[arg(long)]
        yes: bool,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Sync a fork with its upstream repository.
    Sync {
        /// Repository to sync (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Branch to sync into (default: the fork's default branch).
        #[arg(short = 'b', long)]
        branch: Option<String>,
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
    /// Create a pull request.
    Create {
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// PR title.
        #[arg(long)]
        title: Option<String>,
        /// PR body (markdown).
        #[arg(long)]
        body: Option<String>,
        /// Base branch to merge into. Auto-detected from the repo's default branch.
        #[arg(long)]
        base: Option<String>,
        /// Head branch. Auto-detected from the current branch.
        #[arg(long)]
        head: Option<String>,
        /// Create as a draft PR.
        #[arg(long)]
        draft: bool,
        /// Add labels (repeatable).
        #[arg(long = "label")]
        labels: Vec<String>,
        /// Assign people by login (repeatable).
        #[arg(long)]
        assignee: Vec<String>,
        /// Milestone ID or title.
        #[arg(long)]
        milestone: Option<String>,
        /// Add to project board by number.
        #[arg(long)]
        project: Option<u32>,
        /// Open the PR in the default browser after creation.
        #[arg(short = 'w', long)]
        web: bool,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Close a pull request.
    Close {
        /// Pull request number.
        number: u64,
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Add a closing comment.
        #[arg(long)]
        comment: Option<String>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Reopen a closed pull request.
    Reopen {
        /// Pull request number.
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
    /// Add a comment to a pull request.
    Comment {
        /// Pull request number.
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
        /// Open the PR in the default browser after commenting.
        #[arg(short = 'w', long)]
        web: bool,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Merge a pull request.
    Merge {
        /// Pull request number.
        number: u64,
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Merge with a merge commit (default).
        #[arg(long, conflicts_with_all = &["squash", "rebase"])]
        merge: bool,
        /// Squash commits into one.
        #[arg(long, conflicts_with_all = &["merge", "rebase"])]
        squash: bool,
        /// Rebase commits onto the base branch.
        #[arg(long, conflicts_with_all = &["merge", "squash"])]
        rebase: bool,
        /// Merge commit message body.
        #[arg(long)]
        body: Option<String>,
        /// Merge commit subject.
        #[arg(long)]
        subject: Option<String>,
        /// Delete the head branch after merging.
        #[arg(long)]
        delete_branch: bool,
        /// Use admin privileges to bypass branch protection.
        #[arg(long)]
        admin: bool,
        /// Enable auto-merge (merge when all checks pass).
        #[arg(long)]
        auto: bool,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Check out a pull request's head branch locally.
    Checkout {
        /// Pull request number.
        number: u64,
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Custom local branch name.
        #[arg(short = 'b', long)]
        branch: Option<String>,
        /// Initialize and update submodules.
        #[arg(long)]
        recurse_submodules: bool,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// View the diff of a pull request.
    Diff {
        /// Pull request number.
        number: u64,
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Control colorized output: always, never, auto.
        #[arg(long, default_value = "auto")]
        color: String,
        /// Show only the names of changed files.
        #[arg(long)]
        name_only: bool,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Edit a pull request.
    Edit {
        /// Pull request number.
        number: u64,
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// New PR title.
        #[arg(long)]
        title: Option<String>,
        /// New PR body (markdown).
        #[arg(long)]
        body: Option<String>,
        /// Change the base branch.
        #[arg(long)]
        base: Option<String>,
        /// Add labels (repeatable).
        #[arg(long = "add-label")]
        add_label: Vec<String>,
        /// Remove labels (repeatable).
        #[arg(long = "remove-label")]
        remove_label: Vec<String>,
        /// Add assignees by login (repeatable).
        #[arg(long = "add-assignee")]
        add_assignee: Vec<String>,
        /// Remove assignees by login (repeatable).
        #[arg(long = "remove-assignee")]
        remove_assignee: Vec<String>,
        /// Milestone ID or title.
        #[arg(long)]
        milestone: Option<String>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Review a pull request.
    Review {
        /// Pull request number.
        number: u64,
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Submit an approving review.
        #[arg(long)]
        approve: bool,
        /// Request changes.
        #[arg(long)]
        request_changes: bool,
        /// Leave a general comment (default).
        #[arg(long)]
        comment: bool,
        /// Review body text.
        #[arg(long)]
        body: Option<String>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// View CI checks for a pull request.
    Checks {
        /// Pull request number.
        number: u64,
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Poll until all checks complete.
        #[arg(long)]
        watch: bool,
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
    /// Create an issue.
    Create {
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Issue title (required).
        #[arg(long)]
        title: Option<String>,
        /// Issue body (markdown).
        #[arg(long)]
        body: Option<String>,
        /// Add labels (repeatable).
        #[arg(long = "label")]
        labels: Vec<String>,
        /// Assign users by login (repeatable).
        #[arg(long)]
        assignee: Vec<String>,
        /// Milestone ID or title.
        #[arg(long)]
        milestone: Option<String>,
        /// Add to project board by number.
        #[arg(long)]
        project: Option<u32>,
        /// Open the issue in the default browser after creation.
        #[arg(short = 'w', long)]
        web: bool,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Edit an issue.
    Edit {
        /// Issue number.
        number: u64,
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// New issue title.
        #[arg(long)]
        title: Option<String>,
        /// New issue body (markdown).
        #[arg(long)]
        body: Option<String>,
        /// Add labels (repeatable).
        #[arg(long = "add-label")]
        add_label: Vec<String>,
        /// Remove labels (repeatable).
        #[arg(long = "remove-label")]
        remove_label: Vec<String>,
        /// Add assignees by login (repeatable).
        #[arg(long = "add-assignee")]
        add_assignee: Vec<String>,
        /// Remove assignees by login (repeatable).
        #[arg(long = "remove-assignee")]
        remove_assignee: Vec<String>,
        /// Milestone ID or title.
        #[arg(long)]
        milestone: Option<String>,
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

/// Subcommands for `gor label`.
#[derive(Subcommand, Debug)]
pub enum LabelCommand {
    /// List labels in a repository.
    List {
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Filter labels by name substring.
        #[arg(long)]
        search: Option<String>,
        /// Maximum number of labels to show (default: 30).
        #[arg(short = 'L', long, default_value = "30")]
        limit: u32,
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Create a label.
    Create {
        /// Label name.
        name: String,
        /// Label color (hex, without #). Auto-generated if omitted.
        #[arg(long)]
        color: Option<String>,
        /// Label description.
        #[arg(long)]
        description: Option<String>,
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Edit a label.
    Edit {
        /// Current label name.
        name: String,
        /// New label name.
        #[arg(long)]
        rename: Option<String>,
        /// New label color (hex, without #).
        #[arg(long)]
        color: Option<String>,
        /// New label description.
        #[arg(long)]
        description: Option<String>,
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Delete a label.
    Delete {
        /// Label name.
        name: String,
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Skip the confirmation prompt.
        #[arg(short = 'y', long)]
        yes: bool,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Clone labels from another repository.
    Clone {
        /// Source repository to clone labels from (OWNER/REPO format).
        source: String,
        /// Target repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Overwrite existing labels in the target repo.
        #[arg(long)]
        force: bool,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
}

/// Subcommands for `gor release`.
#[derive(Subcommand, Debug)]
pub enum ReleaseCommand {
    /// List releases in a repository.
    List {
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Maximum number of releases to show (default: 30).
        #[arg(short = 'L', long, default_value = "30")]
        limit: u32,
        /// Exclude draft releases.
        #[arg(long)]
        exclude_drafts: bool,
        /// Exclude prereleases.
        #[arg(long)]
        exclude_prereleases: bool,
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// View details of a release.
    View {
        /// Tag name or release ID.
        release: String,
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Open the release in the default browser.
        #[arg(short = 'w', long)]
        web: bool,
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Delete a release.
    Delete {
        /// Tag name or release ID.
        release: String,
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Skip the confirmation prompt.
        #[arg(short = 'y', long)]
        yes: bool,
        /// Keep the associated git tag (do not delete it).
        #[arg(long)]
        skip_tag: bool,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Edit a release.
    Edit {
        /// Tag name or release ID.
        release: String,
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// New release title.
        #[arg(long)]
        title: Option<String>,
        /// New release body (markdown).
        #[arg(long)]
        notes: Option<String>,
        /// Read release body from a file (use "-" for stdin).
        #[arg(long)]
        notes_file: Option<String>,
        /// Set draft status (true or false).
        #[arg(long)]
        draft: Option<bool>,
        /// Set prerelease status (true or false).
        #[arg(long)]
        prerelease: Option<bool>,
        /// Change the tag the release points to.
        #[arg(long)]
        tag: Option<String>,
        /// Change the target commitish (branch or commit SHA).
        #[arg(long)]
        target: Option<String>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Upload assets to a release.
    Upload {
        /// Tag name or release ID.
        release: String,
        /// Asset files to upload.
        files: Vec<String>,
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Override the asset name (only meaningful for a single asset upload).
        #[arg(long)]
        name: Option<String>,
        /// Override the auto-detected MIME type.
        #[arg(long = "mime-type")]
        mime_type: Option<String>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Create a new release.
    Create {
        /// Tag name for the release (must already exist).
        tag: String,
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Release title (defaults to tag name).
        #[arg(long)]
        title: Option<String>,
        /// Release body (markdown).
        #[arg(long)]
        notes: Option<String>,
        /// Read release notes from a file (use "-" for stdin).
        #[arg(long)]
        notes_file: Option<String>,
        /// Create as a draft release.
        #[arg(long)]
        draft: bool,
        /// Mark as a prerelease.
        #[arg(long)]
        prerelease: bool,
        /// Target commitish (branch or commit SHA).
        #[arg(long)]
        target: Option<String>,
        /// Discussion category for the release.
        #[arg(long)]
        discussion_category: Option<String>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Download release assets.
    Download {
        /// Tag name or release ID.
        release: String,
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Filter assets by glob pattern (repeatable).
        #[arg(long = "pattern")]
        patterns: Vec<String>,
        /// Output directory (default: current directory).
        #[arg(short = 'D', long, default_value = ".")]
        dir: String,
        /// Skip assets whose file already exists.
        #[arg(long)]
        skip_existing: bool,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
}

/// Arguments for `gor browse`.
#[derive(clap::Args, Debug)]
pub struct BrowseCommand {
    /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
    #[arg(short = 'R', long)]
    pub repo: Option<String>,
    /// Open a specific branch or tree.
    #[arg(short = 'b', long)]
    pub branch: Option<String>,
    /// Open a specific commit.
    #[arg(short = 'c', long)]
    pub commit: Option<String>,
    /// Open the projects tab.
    #[arg(long)]
    pub projects: bool,
    /// Open the wiki.
    #[arg(long)]
    pub wiki: bool,
    /// Open the settings page.
    #[arg(long)]
    pub settings: bool,
    /// Open a specific issue by number.
    #[arg(short = 'i', long)]
    pub issue: Option<u64>,
    /// Open a specific pull request by number.
    #[arg(short = 'p', long)]
    pub pr: Option<u64>,
    /// GitHub hostname for GitHub Enterprise Server (default: github.com).
    #[arg(long, env = "GH_HOST")]
    pub hostname: Option<String>,
}

/// Subcommands for `gor gist`.
#[derive(Subcommand, Debug)]
pub enum GistCommand {
    /// List gists.
    List {
        /// List public gists.
        #[arg(long)]
        public: bool,
        /// List secret gists.
        #[arg(long)]
        secret: bool,
        /// List gists for a specific user (public only).
        #[arg(long)]
        user: Option<String>,
        /// Maximum number of gists to show (default: 30).
        #[arg(short = 'L', long, default_value = "30")]
        limit: u32,
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Create a gist.
    Create {
        /// Files to include in the gist.
        files: Vec<String>,
        /// Gist description.
        #[arg(long)]
        desc: Option<String>,
        /// Create a public gist (default: secret).
        #[arg(long)]
        public: bool,
        /// Override the filename in the gist.
        #[arg(long)]
        filename: Option<String>,
        /// Open the new gist in the browser.
        #[arg(short = 'w', long)]
        web: bool,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// View a gist's content and metadata.
    View {
        /// Gist ID to view.
        gist_id: String,
        /// Output the raw content of the gist.
        #[arg(long)]
        raw: bool,
        /// Select a specific file to view.
        #[arg(long)]
        filename: Option<String>,
        /// Open the gist in the browser.
        #[arg(short = 'w', long)]
        web: bool,
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Edit a gist.
    Edit {
        /// Gist ID to edit.
        gist_id: String,
        /// New description for the gist.
        #[arg(long)]
        desc: Option<String>,
        /// Add a file to the gist (path=content or file path).
        #[arg(long)]
        add: Vec<String>,
        /// Rename a file (old:new).
        #[arg(long)]
        filename: Option<String>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Delete a gist.
    Delete {
        /// Gist ID to delete.
        gist_id: String,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
}

/// Subcommands for `gor search`.
#[derive(Subcommand, Debug)]
pub enum SearchCommand {
    /// Search repositories.
    Repos {
        /// Search query.
        query: Vec<String>,
        /// Filter by primary language.
        #[arg(long)]
        language: Option<String>,
        /// Filter by repository topic.
        #[arg(long)]
        topic: Option<String>,
        /// Filter by star count range (e.g., ">100", "10..50").
        #[arg(long)]
        stars: Option<String>,
        /// Sort by: stars, forks, updated, best-match.
        #[arg(long, default_value = "best-match")]
        sort: String,
        /// Sort order: asc or desc.
        #[arg(long, default_value = "desc")]
        order: String,
        /// Maximum number of results (default: 30).
        #[arg(short = 'L', long, default_value = "30")]
        limit: u32,
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// Open the search results in the browser.
        #[arg(short = 'w', long)]
        web: bool,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Search code.
    Code {
        /// Search query.
        query: Vec<String>,
        /// Filter by language.
        #[arg(long)]
        language: Option<String>,
        /// Search within a specific repository (OWNER/REPO).
        #[arg(long)]
        repo: Option<String>,
        /// Maximum number of results (default: 30).
        #[arg(short = 'L', long, default_value = "30")]
        limit: u32,
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// Open the search results in the browser.
        #[arg(short = 'w', long)]
        web: bool,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Search issues and pull requests.
    Issues {
        /// Search query.
        query: Vec<String>,
        /// Filter by type: issue, pr.
        #[arg(long)]
        r#type: Option<String>,
        /// Filter by state: open, closed.
        #[arg(long)]
        state: Option<String>,
        /// Filter by labels (comma-separated).
        #[arg(long)]
        labels: Option<String>,
        /// Maximum number of results (default: 30).
        #[arg(short = 'L', long, default_value = "30")]
        limit: u32,
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// Open the search results in the browser.
        #[arg(short = 'w', long)]
        web: bool,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Search commits.
    Commits {
        /// Search query.
        query: Vec<String>,
        /// Filter by author.
        #[arg(long)]
        author: Option<String>,
        /// Search within a specific repository (OWNER/REPO).
        #[arg(long)]
        repo: Option<String>,
        /// Maximum number of results (default: 30).
        #[arg(short = 'L', long, default_value = "30")]
        limit: u32,
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// Open the search results in the browser.
        #[arg(short = 'w', long)]
        web: bool,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
}

/// Subcommands for `gor workflow`.
#[derive(Subcommand, Debug)]
pub enum WorkflowCommand {
    /// List workflows in a repository.
    List {
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Maximum number of workflows to show (default: 30).
        #[arg(short = 'L', long, default_value = "30")]
        limit: u32,
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// View a workflow's details.
    View {
        /// Workflow ID, filename (e.g. deploy.yml), or name.
        workflow: String,
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Enable a workflow.
    Enable {
        /// Workflow ID, filename (e.g. deploy.yml), or name.
        workflow: String,
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Disable a workflow.
    Disable {
        /// Workflow ID, filename (e.g. deploy.yml), or name.
        workflow: String,
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Run a workflow.
    Run {
        /// Workflow ID, filename (e.g. deploy.yml), or name.
        workflow: String,
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Branch to run the workflow on.
        #[arg(short = 'b', long)]
        branch: Option<String>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
}

/// Subcommands for `gor org`.
#[derive(Subcommand, Debug)]
pub enum OrgCommand {
    /// List organizations for the authenticated user.
    List {
        /// Maximum number of organizations to show (default: 30).
        #[arg(short = 'L', long, default_value = "30")]
        limit: u32,
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// View an organization's profile.
    View {
        /// Organization name.
        org: String,
        /// Open the organization in the browser.
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

/// Subcommands for `gor ssh-key`.
#[derive(Subcommand, Debug)]
pub enum SshKeyCommand {
    /// List SSH keys.
    List {
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Add an SSH key.
    Add {
        /// Title for the key.
        #[arg(short = 't', long)]
        title: String,
        /// Read the public key from a file.
        #[arg(short = 'f', long)]
        file: Option<String>,
        /// Provide the public key inline.
        #[arg(short = 'b', long)]
        body: Option<String>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
}

/// Subcommands for `gor gpg-key`.
#[derive(Subcommand, Debug)]
pub enum GpgKeyCommand {
    /// List GPG keys.
    List {
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Add a GPG key.
    Add {
        /// Read the armored public key from a file.
        #[arg(short = 'f', long)]
        file: Option<String>,
        /// Provide the armored key inline.
        #[arg(short = 'b', long)]
        body: Option<String>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
}

/// Subcommands for `gor secret`.
#[derive(Subcommand, Debug)]
pub enum SecretCommand {
    /// List secrets.
    List {
        /// Organization to list secrets for.
        #[arg(long)]
        org: Option<String>,
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Set a secret.
    Set {
        /// Secret name.
        name: String,
        /// Secret value body.
        #[arg(short = 'b', long)]
        body: Option<String>,
        /// Read secret value from a file.
        #[arg(short = 'f', long)]
        file: Option<String>,
        /// Organization to set the secret for.
        #[arg(long)]
        org: Option<String>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
}

/// Subcommands for `gor variable`.
#[derive(Subcommand, Debug)]
pub enum VariableCommand {
    /// List variables.
    List {
        /// Organization to list variables for.
        #[arg(long)]
        org: Option<String>,
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Set a variable.
    Set {
        /// Variable name.
        name: String,
        /// Variable value.
        #[arg(short = 'b', long)]
        body: Option<String>,
        /// Read variable value from a file.
        #[arg(short = 'f', long)]
        file: Option<String>,
        /// Organization to set the variable for.
        #[arg(long)]
        org: Option<String>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
}

/// Subcommands for `gor run`.
#[derive(Subcommand, Debug)]
pub enum RunCommand {
    /// List workflow runs.
    List {
        /// Workflow filename or ID to filter by.
        #[arg(short = 'w', long)]
        workflow: Option<String>,
        /// Branch to filter by.
        #[arg(short = 'b', long)]
        branch: Option<String>,
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Maximum number of runs to show (default: 30).
        #[arg(short = 'L', long, default_value = "30")]
        limit: u32,
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// View a workflow run's details and jobs.
    View {
        /// Run ID.
        id: u64,
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Open the run in the browser.
        #[arg(short = 'w', long)]
        web: bool,
        /// Show logs for a specific job.
        #[arg(long)]
        log: Option<u64>,
        /// Show logs only for failed jobs.
        #[arg(long)]
        log_failed: bool,
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
}

/// Subcommands for `gor cache`.
#[derive(Subcommand, Debug)]
pub enum CacheCommand {
    /// List caches.
    List {
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
}

/// Subcommands for `gor ruleset`.
#[derive(Subcommand, Debug)]
pub enum RulesetCommand {
    /// List rulesets.
    List {
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// View a ruleset's details.
    View {
        /// Ruleset ID.
        id: u32,
        /// Repository (OWNER/REPO format). Auto-detected from git remote if omitted.
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Open the ruleset in the browser.
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

/// Subcommands for `gor extension`.
#[derive(Subcommand, Debug)]
pub enum ExtensionCommand {
    /// List installed extensions.
    List {
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Install an extension.
    Install {
        /// Extension name or URL.
        name: String,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
}

/// Subcommands for `gor codespace`.
#[derive(Subcommand, Debug)]
pub enum CodespaceCommand {
    /// List codespaces.
    List {
        /// Repository (OWNER/REPO format).
        #[arg(short = 'R', long)]
        repo: Option<String>,
        /// Output as JSON. Optionally specify comma-separated field names.
        #[arg(long, num_args = 0.., value_delimiter = ',')]
        json: Option<Vec<String>>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Create a codespace.
    Create {
        /// Repository (OWNER/REPO format).
        repo: String,
        /// Branch to create the codespace from.
        #[arg(short = 'b', long)]
        branch: Option<String>,
        /// Machine type.
        #[arg(long)]
        machine: Option<String>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
}

/// Subcommands for `gor alias`.
#[derive(Subcommand, Debug)]
pub enum AliasCommand {
    /// List aliases.
    List {
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Set an alias.
    Set {
        /// Alias name.
        name: String,
        /// Command to alias (with arguments).
        command: Vec<String>,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
    /// Delete an alias.
    Delete {
        /// Alias name to delete.
        name: String,
        /// GitHub hostname for GitHub Enterprise Server (default: github.com).
        #[arg(long, env = "GH_HOST")]
        hostname: Option<String>,
    },
}
