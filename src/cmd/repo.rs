//! Implementation of the `gor repo` subcommand.
//!
//! Provides repository viewing, listing, and management commands.
//! Currently supports `gor repo view` for displaying repository metadata.

#![allow(clippy::print_stdout, clippy::print_stderr)]

use crate::cli::RepoCommand;
use crate::client::Client;
use crate::output::{format_count, format_date, print_json};
use crate::repository::{detect_remote, parse_repo_spec};
use anyhow::Context;
use std::io::Write;

/// Run the `gor repo` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cmd: RepoCommand) -> anyhow::Result<()> {
    match cmd {
        RepoCommand::View {
            owner_repo,
            web,
            json,
            hostname,
        } => view(owner_repo, web, json, hostname.as_deref()),
        RepoCommand::List {
            owner,
            visibility,
            fork,
            language,
            limit,
            json,
            hostname,
        } => list(
            owner.as_deref(),
            &visibility,
            &fork,
            language.as_deref(),
            limit,
            json,
            hostname.as_deref(),
        ),
        RepoCommand::Clone {
            owner_repo,
            directory,
            upstream_remote_name,
            hostname,
        } => clone(
            &owner_repo,
            directory.as_deref(),
            &upstream_remote_name,
            hostname.as_deref(),
        ),
        RepoCommand::Create {
            name,
            description,
            private,
            internal,
            org,
            template,
            clone,
            remote,
            push,
            disable_wiki,
            disable_issues,
            hostname,
        } => repo_create(
            &name,
            description.as_deref(),
            private,
            internal,
            org.as_deref(),
            template.as_deref(),
            clone,
            &remote,
            push,
            disable_wiki,
            disable_issues,
            hostname.as_deref(),
        ),
        RepoCommand::Fork {
            owner_repo,
            org,
            clone,
            remote,
            fork_name,
            hostname,
        } => repo_fork(
            &owner_repo,
            org.as_deref(),
            clone,
            &remote,
            fork_name.as_deref(),
            hostname.as_deref(),
        ),
        RepoCommand::Delete {
            owner_repo,
            yes,
            hostname,
        } => repo_delete(&owner_repo, yes, hostname.as_deref()),
        RepoCommand::Edit {
            repo,
            description,
            visibility,
            add_topic,
            remove_topic,
            default_branch,
            enable_wiki,
            enable_issues,
            enable_projects,
            hostname,
        } => repo_edit(
            repo.as_deref(),
            description.as_deref(),
            visibility.as_deref(),
            &add_topic,
            &remove_topic,
            default_branch.as_deref(),
            enable_wiki.as_deref(),
            enable_issues.as_deref(),
            enable_projects.as_deref(),
            hostname.as_deref(),
        ),
        RepoCommand::Transfer {
            target,
            repo,
            new_name,
            team,
            yes,
            hostname,
        } => repo_transfer(
            &target,
            repo.as_deref(),
            new_name.as_deref(),
            &team,
            yes,
            hostname.as_deref(),
        ),
        RepoCommand::Sync {
            repo,
            branch,
            hostname,
        } => sync(repo.as_deref(), branch.as_deref(), hostname.as_deref()),
    }
}

/// Execute `gor repo view`.
///
/// Displays repository metadata including description, stats, language,
/// license, and other details. Supports JSON output and browser opening.
///
/// # Errors
///
/// Returns an error if the repository cannot be found or the API request fails.
fn view(
    owner_repo: Option<String>,
    web: bool,
    json: Option<Vec<String>>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    // Resolve the repo spec
    let spec = match owner_repo {
        Some(s) => parse_repo_spec(&s).context("invalid repository spec")?,
        None => detect_remote().ok_or_else(|| {
            anyhow::anyhow!(
                "could not detect repository from current directory; specify OWNER/REPO"
            )
        })?,
    };

    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let path = format!("/repos/{}/{}", spec.owner, spec.repo);
    let response = client
        .get(&path)
        .context("failed to fetch repository data")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("repository '{spec}' not found");
    }
    if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
        anyhow::bail!("authentication required to view repository '{spec}'");
    }
    if !status.is_success() {
        anyhow::bail!("failed to fetch repository '{spec}': HTTP {status}");
    }

    let repo: serde_json::Value = response
        .json()
        .context("failed to parse repository response")?;

    // Handle --web flag: open in browser
    if web {
        if let Some(url) = repo["html_url"].as_str() {
            open_in_browser(url);
        }
        return Ok(());
    }

    // Handle --json flag
    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&repo, fields_ref);
        return Ok(());
    }

    // Default: print formatted table
    print_repo_table(&repo);
    Ok(())
}

/// Execute `gor repo list`.
///
/// Lists repositories for the authenticated user or a specified owner.
/// Supports filtering by visibility, fork status, and language.
/// Supports table output, JSON output, and pagination.
///
/// # Errors
///
/// Returns an error if the API request fails.
fn list(
    owner: Option<&str>,
    visibility: &str,
    fork: &str,
    language: Option<&str>,
    limit: u32,
    json: Option<Vec<String>>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    // Build the API path based on whether an owner is specified
    let path = match owner {
        Some(owner_name) => {
            // Try users endpoint first; if 404, try orgs endpoint
            let user_path = format!("/users/{owner_name}/repos");
            let response = client
                .get(&user_path)
                .context("failed to fetch repositories")?;
            if response.status() == reqwest::StatusCode::NOT_FOUND {
                format!("/orgs/{owner_name}/repos")
            } else if !response.status().is_success() {
                let status = response.status();
                anyhow::bail!("failed to fetch repositories for '{owner_name}': HTTP {status}");
            } else {
                // We got a valid response — process it inline
                let repos: Vec<serde_json::Value> = response
                    .json()
                    .context("failed to parse repository response")?;
                let filtered = filter_repos(repos, visibility, fork, language, limit);
                output_repos(&filtered, json);
                return Ok(());
            }
        }
        None => "/user/repos".to_string(),
    };

    // Build query parameters
    let mut query_params = vec![
        ("per_page", limit.min(100).to_string()),
        ("type", "all".to_string()),
    ];

    // Add visibility filter
    if visibility != "all" {
        query_params.push(("visibility", visibility.to_string()));
    }

    // Add sort by updated
    query_params.push(("sort", "updated".to_string()));

    let query_string = query_params
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("&");

    let full_path = format!("{path}?{query_string}");

    let response = client
        .get(&full_path)
        .context("failed to fetch repositories")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        if let Some(owner_name) = owner {
            anyhow::bail!("user or organization '{owner_name}' not found");
        }
        anyhow::bail!("could not fetch repositories");
    }
    if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
        anyhow::bail!("authentication required to list repositories");
    }
    if !status.is_success() {
        anyhow::bail!("failed to list repositories: HTTP {status}");
    }

    let repos: Vec<serde_json::Value> = response
        .json()
        .context("failed to parse repository response")?;

    let filtered = filter_repos(repos, visibility, fork, language, limit);
    output_repos(&filtered, json);
    Ok(())
}

/// Execute `gor repo clone`.
///
/// Clones a GitHub repository to the local machine using `gix`.
/// Supports OWNER/REPO format and full URLs. For forks, automatically
/// adds an upstream remote pointing to the parent repository.
///
/// # Errors
///
/// Returns an error if the repository cannot be found, the clone fails,
/// or the upstream remote cannot be added.
fn clone(
    owner_repo: &str,
    directory: Option<&str>,
    upstream_remote_name: &str,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");

    // Determine if input is a URL or OWNER/REPO
    let (clone_url, _is_fork, parent_clone_url) =
        if owner_repo.starts_with("https://") || owner_repo.starts_with("git@") {
            // Full URL provided — use it directly, no API lookup for fork info
            (owner_repo.to_string(), false, None)
        } else {
            // OWNER/REPO format — fetch repo info from API
            let spec = parse_repo_spec(owner_repo).context("invalid repository spec")?;
            let client = Client::new(host).context("failed to create HTTP client")?;

            let path = format!("/repos/{}/{}", spec.owner, spec.repo);
            let response = client
                .get(&path)
                .context("failed to fetch repository data")?;

            let status = response.status();
            if status == reqwest::StatusCode::NOT_FOUND {
                anyhow::bail!("repository '{spec}' not found");
            }
            if !status.is_success() {
                anyhow::bail!("failed to fetch repository '{spec}': HTTP {status}");
            }

            let repo: serde_json::Value = response
                .json()
                .context("failed to parse repository response")?;

            let clone_url = repo["clone_url"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("no clone_url in repository response"))?
                .to_string();

            let is_fork = repo["fork"].as_bool().unwrap_or(false);
            let parent_clone_url = if is_fork {
                repo["parent"]["clone_url"].as_str().map(String::from)
            } else {
                None
            };

            (clone_url, is_fork, parent_clone_url)
        };

    // Determine target directory
    let dest_dir = directory.map_or_else(
        || {
            // Derive directory name from the repo name
            let repo_name = clone_url.rfind('/').map_or(clone_url.as_str(), |pos| {
                clone_url[pos + 1..].trim_end_matches(".git")
            });
            std::path::PathBuf::from(repo_name)
        },
        std::path::PathBuf::from,
    );

    // Clone the repository using gix
    eprintln!("Cloning into '{}'...", dest_dir.display());

    let mut prepare_fetch =
        gix::prepare_clone(clone_url.as_str(), &dest_dir).context("failed to prepare clone")?;

    let (mut prepare_checkout, _outcome) = prepare_fetch
        .fetch_then_checkout(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)
        .context("failed to fetch repository")?;

    let (_repo, _checkout_outcome) = prepare_checkout
        .main_worktree(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)
        .context("failed to checkout worktree")?;

    // If it's a fork, add the upstream remote
    if let Some(parent_url) = parent_clone_url {
        eprintln!("Adding upstream remote '{upstream_remote_name}'...");

        // Open the cloned repo to add the remote
        let repo = gix::open(&dest_dir).context("failed to open cloned repository")?;

        // Write the remote section directly to the git config file
        let config_path = repo.git_dir().join("config");
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(&config_path)
            .context("failed to open git config for appending")?;

        let url_escaped = parent_url.replace('"', "\\\"");
        writeln!(
            file,
            "[remote \"{upstream_remote_name}\"]\n\turl = {url_escaped}\n\tfetch = +refs/heads/*:refs/remotes/{upstream_remote_name}/*"
        )
        .context("failed to write upstream remote config")?;

        eprintln!("Added upstream remote '{upstream_remote_name}'");
    }

    eprintln!("Clone complete.");
    Ok(())
}

/// Execute `gor repo create`.
///
/// Creates a new GitHub repository under the authenticated user or an
/// organization. Supports setting description, visibility, feature toggles,
/// and optionally cloning the new repository locally.
///
/// # Errors
///
/// Returns an error if the API request fails.
#[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools)]
fn repo_create(
    name: &str,
    description: Option<&str>,
    private: bool,
    internal: bool,
    org: Option<&str>,
    _template: Option<&str>,
    clone: bool,
    _remote: &str,
    _push: bool,
    disable_wiki: bool,
    disable_issues: bool,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    // Build the request body
    let mut body_map = serde_json::Map::new();
    body_map.insert(
        "name".to_string(),
        serde_json::Value::String(name.to_string()),
    );
    body_map.insert(
        "private".to_string(),
        serde_json::Value::Bool(private || internal),
    );
    if internal {
        body_map.insert(
            "visibility".to_string(),
            serde_json::Value::String("internal".to_string()),
        );
    }
    if let Some(d) = description {
        body_map.insert(
            "description".to_string(),
            serde_json::Value::String(d.to_string()),
        );
    }
    if disable_wiki {
        body_map.insert("has_wiki".to_string(), serde_json::Value::Bool(false));
    }
    if disable_issues {
        body_map.insert("has_issues".to_string(), serde_json::Value::Bool(false));
    }

    let body_value = serde_json::Value::Object(body_map);

    // Determine the API path
    let path = org.map_or_else(|| "/user/repos".to_string(), |o| format!("/orgs/{o}/repos"));

    let response = client
        .post(&path, &body_value)
        .context("failed to create repository")?;

    let status = response.status();
    if !status.is_success() {
        let err_body: serde_json::Value = response.json().unwrap_or_default();
        let msg = err_body["message"].as_str().unwrap_or("creation failed");
        anyhow::bail!("failed to create repository: {msg}");
    }

    let repo: serde_json::Value = response
        .json()
        .context("failed to parse repository response")?;

    let clone_url = repo["clone_url"].as_str().unwrap_or("");
    let html_url = repo["html_url"].as_str().unwrap_or("");
    let full_name = repo["full_name"].as_str().unwrap_or("");

    println!("{html_url}");

    // Clone the new repository locally if requested
    if clone && !clone_url.is_empty() {
        let dest_dir = std::path::PathBuf::from(name);
        eprintln!("Cloning into '{}'...", dest_dir.display());

        let url =
            gix::url::parse(clone_url.as_bytes().into()).context("failed to parse clone URL")?;
        let mut prepare_fetch =
            gix::prepare_clone(url, &dest_dir).context("failed to prepare clone")?;
        let (mut prepare_checkout, _outcome) = prepare_fetch
            .fetch_then_checkout(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)
            .context("failed to fetch repository")?;
        let (_repo, _checkout_outcome) = prepare_checkout
            .main_worktree(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)
            .context("failed to checkout worktree")?;
        eprintln!("Cloned {full_name}");
    }

    Ok(())
}

/// Execute `gor repo fork`.
///
/// Forks a repository to the authenticated user's account or an organization.
/// Optionally clones the fork locally after creation.
///
/// # Errors
///
/// Returns an error if the API request fails.
fn repo_fork(
    owner_repo: &str,
    org: Option<&str>,
    clone: bool,
    _remote: &str,
    fork_name: Option<&str>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let spec = parse_repo_spec(owner_repo).context("invalid repository spec")?;
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    // Build the request body
    let mut body_map = serde_json::Map::new();
    if let Some(o) = org {
        body_map.insert(
            "organization".to_string(),
            serde_json::Value::String(o.to_string()),
        );
    }
    if let Some(n) = fork_name {
        body_map.insert("name".to_string(), serde_json::Value::String(n.to_string()));
    }

    let body_value = serde_json::Value::Object(body_map);

    let path = format!("/repos/{}/{}/forks", spec.owner, spec.repo);
    let response = client
        .post(&path, &body_value)
        .context("failed to fork repository")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("repository '{spec}' not found");
    }
    if status == reqwest::StatusCode::ACCEPTED {
        // Fork is being created asynchronously
        let fork: serde_json::Value = response.json().context("failed to parse fork response")?;
        let html_url = fork["html_url"].as_str().unwrap_or("");
        println!("{html_url}");
        eprintln!("Fork is being created. It may take a few minutes.");
        return Ok(());
    }
    if !status.is_success() {
        let err_body: serde_json::Value = response.json().unwrap_or_default();
        let msg = err_body["message"].as_str().unwrap_or("fork failed");
        anyhow::bail!("failed to fork repository: {msg}");
    }

    let fork: serde_json::Value = response.json().context("failed to parse fork response")?;

    let clone_url = fork["clone_url"].as_str().unwrap_or("");
    let html_url = fork["html_url"].as_str().unwrap_or("");
    let full_name = fork["full_name"].as_str().unwrap_or("");

    println!("{html_url}");

    // Clone the fork locally if requested
    if clone && !clone_url.is_empty() {
        let repo_name = fork_name.unwrap_or(&spec.repo);
        let dest_dir = std::path::PathBuf::from(repo_name);
        eprintln!("Cloning into '{}'...", dest_dir.display());

        let url =
            gix::url::parse(clone_url.as_bytes().into()).context("failed to parse clone URL")?;
        let mut prepare_fetch =
            gix::prepare_clone(url, &dest_dir).context("failed to prepare clone")?;
        let (mut prepare_checkout, _outcome) = prepare_fetch
            .fetch_then_checkout(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)
            .context("failed to fetch repository")?;
        let (_repo, _checkout_outcome) = prepare_checkout
            .main_worktree(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)
            .context("failed to checkout worktree")?;
        eprintln!("Cloned {full_name}");
    }

    Ok(())
}

/// Execute `gor repo delete`.
///
/// Deletes a repository. Requires confirmation unless --yes is provided.
///
/// # Errors
///
/// Returns an error if the API request fails or the user cancels.
fn repo_delete(owner_repo: &str, yes: bool, hostname: Option<&str>) -> anyhow::Result<()> {
    let spec = parse_repo_spec(owner_repo).context("invalid repository spec")?;

    // Confirmation prompt
    if !yes {
        eprintln!("WARNING: This will delete the repository '{spec}' and all its data.");
        eprint!("Type the repository name '{spec}' to confirm: ");
        std::io::stdout().flush().ok();
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .context("failed to read input")?;
        let input = input.trim();
        if input != spec.to_string() {
            anyhow::bail!("confirmation failed: expected '{spec}', got '{input}'");
        }
    }

    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let path = format!("/repos/{}/{}", spec.owner, spec.repo);
    let response = client
        .request("DELETE", &path, &[], None)
        .context("failed to delete repository")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("repository '{spec}' not found");
    }
    if status == reqwest::StatusCode::FORBIDDEN {
        anyhow::bail!("you do not have permission to delete '{spec}'");
    }
    if !status.is_success() {
        anyhow::bail!("failed to delete repository '{spec}': HTTP {status}");
    }

    println!("Deleted repository '{spec}'");
    Ok(())
}

/// Execute `gor repo edit`.
///
/// Edits a repository's settings including description, visibility, topics,
/// default branch, and feature toggles.
///
/// # Errors
///
/// Returns an error if the repository cannot be found or the API request fails.
#[allow(clippy::too_many_arguments)]
fn repo_edit(
    repo: Option<&str>,
    description: Option<&str>,
    visibility: Option<&str>,
    add_topic: &[String],
    remove_topic: &[String],
    default_branch: Option<&str>,
    enable_wiki: Option<&str>,
    enable_issues: Option<&str>,
    enable_projects: Option<&str>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let spec = match repo {
        Some(s) => parse_repo_spec(s).context("invalid repository spec")?,
        None => detect_remote().ok_or_else(|| {
            anyhow::anyhow!(
                "could not detect repository from current directory; specify OWNER/REPO with --repo"
            )
        })?,
    };

    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    // Build the request body with only the fields that were provided
    let mut body_map = serde_json::Map::new();

    if let Some(d) = description {
        body_map.insert(
            "description".to_string(),
            serde_json::Value::String(d.to_string()),
        );
    }
    if let Some(v) = visibility {
        body_map.insert(
            "visibility".to_string(),
            serde_json::Value::String(v.to_string()),
        );
    }
    if let Some(b) = default_branch {
        body_map.insert(
            "default_branch".to_string(),
            serde_json::Value::String(b.to_string()),
        );
    }
    if let Some(w) = enable_wiki {
        body_map.insert("has_wiki".to_string(), serde_json::Value::Bool(w == "true"));
    }
    if let Some(i) = enable_issues {
        body_map.insert(
            "has_issues".to_string(),
            serde_json::Value::Bool(i == "true"),
        );
    }
    if let Some(p) = enable_projects {
        body_map.insert(
            "has_projects".to_string(),
            serde_json::Value::Bool(p == "true"),
        );
    }

    let body_value = serde_json::Value::Object(body_map);

    let path = format!("/repos/{}/{}", spec.owner, spec.repo);
    let response = client
        .request("PATCH", &path, &[], Some(serde_json::to_vec(&body_value)?))
        .context("failed to update repository")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("repository '{spec}' not found");
    }
    if !status.is_success() {
        let err_body: serde_json::Value = response.json().unwrap_or_default();
        let msg = err_body["message"].as_str().unwrap_or("update failed");
        anyhow::bail!("failed to update repository '{spec}': {msg}");
    }

    let updated: serde_json::Value = response
        .json()
        .context("failed to parse repository response")?;

    // Handle topics separately (GitHub API has a separate endpoint)
    if !add_topic.is_empty() || !remove_topic.is_empty() {
        // Get current topics first
        let topics_path = format!("/repos/{}/{}/topics", spec.owner, spec.repo);
        let topics_response = client.get(&topics_path).context("failed to fetch topics")?;
        let mut current_topics: Vec<String> = topics_response
            .json::<serde_json::Value>()
            .ok()
            .and_then(|v| v["names"].as_array().cloned())
            .unwrap_or_default()
            .into_iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect();

        // Add new topics
        for topic in add_topic {
            if !current_topics.contains(topic) {
                current_topics.push(topic.clone());
            }
        }

        // Remove topics
        current_topics.retain(|t| !remove_topic.contains(t));

        // Update topics
        let topics_body = serde_json::json!({"names": current_topics});
        let accept_header = "application/vnd.github.mercy-preview+json".to_string();
        if let Err(e) = client.request(
            "PUT",
            &topics_path,
            &[format!("Accept: {accept_header}")],
            Some(serde_json::to_vec(&topics_body)?),
        ) {
            eprintln!("Warning: failed to update topics: {e}");
        }
    }

    // Print updated details
    let full_name = updated["full_name"].as_str().unwrap_or("");
    let desc = updated["description"].as_str().unwrap_or("—");
    let vis = updated["visibility"].as_str().unwrap_or("public");
    let branch = updated["default_branch"].as_str().unwrap_or("—");
    let wiki = updated["has_wiki"].as_bool().unwrap_or(true);
    let issues = updated["has_issues"].as_bool().unwrap_or(true);
    let projects = updated["has_projects"].as_bool().unwrap_or(true);

    println!("Updated repository {full_name}");
    println!("  description:  {desc}");
    println!("  visibility:   {vis}");
    println!("  default:      {branch}");
    println!("  wiki:         {wiki}");
    println!("  issues:       {issues}");
    println!("  projects:     {projects}");

    Ok(())
}

/// Execute `gor repo transfer`.
///
/// Transfers a repository to another user or organization.
/// Requires confirmation unless --yes is provided.
///
/// # Errors
///
/// Returns an error if the API request fails or the user cancels.
fn repo_transfer(
    target: &str,
    repo: Option<&str>,
    new_name: Option<&str>,
    _teams: &[String],
    yes: bool,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let spec = match repo {
        Some(s) => parse_repo_spec(s).context("invalid repository spec")?,
        None => detect_remote().ok_or_else(|| {
            anyhow::anyhow!(
                "could not detect repository from current directory; specify OWNER/REPO with --repo"
            )
        })?,
    };

    // Confirmation prompt
    if !yes {
        eprintln!("WARNING: This will transfer repository '{spec}' to '{target}'.");
        eprint!("Type the repository name '{spec}' to confirm: ");
        std::io::stdout().flush().ok();
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .context("failed to read input")?;
        let input = input.trim();
        if input != spec.to_string() {
            anyhow::bail!("confirmation failed: expected '{spec}', got '{input}'");
        }
    }

    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    // Build the request body
    let mut body_map = serde_json::Map::new();
    body_map.insert(
        "new_owner".to_string(),
        serde_json::Value::String(target.to_string()),
    );
    if let Some(n) = new_name {
        body_map.insert(
            "new_name".to_string(),
            serde_json::Value::String(n.to_string()),
        );
    }

    let body_value = serde_json::Value::Object(body_map);

    let path = format!("/repos/{}/{}/transfer", spec.owner, spec.repo);
    let response = client
        .request("POST", &path, &[], Some(serde_json::to_vec(&body_value)?))
        .context("failed to transfer repository")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("repository '{spec}' not found");
    }
    if status == reqwest::StatusCode::FORBIDDEN {
        anyhow::bail!("you do not have permission to transfer '{spec}' to '{target}'");
    }
    if !status.is_success() {
        let err_body: serde_json::Value = response.json().unwrap_or_default();
        let msg = err_body["message"].as_str().unwrap_or("transfer failed");
        anyhow::bail!("failed to transfer repository: {msg}");
    }

    let result: serde_json::Value = response
        .json()
        .context("failed to parse transfer response")?;

    let html_url = result["html_url"].as_str().unwrap_or("");
    println!("Transferred repository to {target}");
    println!("{html_url}");

    Ok(())
}

/// Filter repositories by visibility, fork status, and language.
fn filter_repos(
    repos: Vec<serde_json::Value>,
    visibility: &str,
    fork: &str,
    language: Option<&str>,
    limit: u32,
) -> Vec<serde_json::Value> {
    let mut filtered = repos;

    // Client-side visibility filter (API may not support all cases)
    if visibility != "all" {
        filtered.retain(|repo| {
            let is_private = repo["private"].as_bool().unwrap_or(false);
            match visibility {
                "public" => !is_private,
                "private" => is_private,
                _ => true,
            }
        });
    }

    // Fork filter
    match fork {
        "exclude" => filtered.retain(|repo| !repo["fork"].as_bool().unwrap_or(false)),
        "only" => filtered.retain(|repo| repo["fork"].as_bool().unwrap_or(false)),
        _ => {} // "include" — no filter
    }

    // Language filter
    if let Some(lang) = language {
        filtered.retain(|repo| {
            repo["language"]
                .as_str()
                .is_some_and(|l| l.eq_ignore_ascii_case(lang))
        });
    }

    // Apply limit
    filtered.truncate(limit as usize);
    filtered
}

/// Output repositories as either JSON or a formatted table.
fn output_repos(repos: &Vec<serde_json::Value>, json: Option<Vec<String>>) {
    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(repos, fields_ref);
    } else {
        print_repo_list_table(repos);
    }
}

/// Print a formatted repository list table.
///
/// Columns: NAME, DESCRIPTION, VISIBILITY, LANGUAGE, UPDATED
fn print_repo_list_table(repos: &[serde_json::Value]) {
    if repos.is_empty() {
        println!("No repositories found.");
        return;
    }

    // Column widths
    let name_width = 30;
    let desc_width = 50;
    let vis_width = 10;
    let lang_width = 14;
    let date_width = 16;

    // Header
    println!(
        "{:<name_width$}  {:<desc_width$}  {:<vis_width$}  {:<lang_width$}  {:<date_width$}",
        "NAME", "DESCRIPTION", "VISIBILITY", "LANGUAGE", "UPDATED",
    );

    for repo in repos {
        let name = repo["full_name"].as_str().unwrap_or("—");
        let description = repo["description"].as_str().unwrap_or("—");
        let is_private = repo["private"].as_bool().unwrap_or(false);
        let visibility = if is_private { "private" } else { "public" };
        let language = repo["language"].as_str().unwrap_or("—");
        let updated = repo["updated_at"]
            .as_str()
            .map_or_else(|| "—".to_string(), format_date);

        let name_truncated = crate::cmd::util::truncate(name, name_width);
        let desc_truncated = crate::cmd::util::truncate(description, desc_width);

        println!(
            "{name_truncated:<name_width$}  {desc_truncated:<desc_width$}  {visibility:<vis_width$}  {language:<lang_width$}  {updated:<date_width$}",
        );
    }
}

/// Print a formatted repository information table.
fn print_repo_table(repo: &serde_json::Value) {
    let full_name = repo["full_name"].as_str().unwrap_or("—");
    let description = repo["description"].as_str().unwrap_or("—");
    let html_url = repo["html_url"].as_str().unwrap_or("—");
    let is_private = repo["private"].as_bool().unwrap_or(false);
    let visibility = if is_private { "private" } else { "public" };
    let stars = repo["stargazers_count"].as_u64().unwrap_or(0);
    let forks = repo["forks_count"].as_u64().unwrap_or(0);
    let issues = repo["open_issues_count"].as_u64().unwrap_or(0);
    let language = repo["language"].as_str().unwrap_or("—");
    let license_name = repo["license"]["spdx_id"]
        .as_str()
        .or_else(|| repo["license"]["name"].as_str())
        .unwrap_or("—");
    let default_branch = repo["default_branch"].as_str().unwrap_or("—");
    let pushed_at = repo["pushed_at"].as_str().unwrap_or("—");

    println!("name:        {full_name}");
    println!("description: {description}");
    println!("url:         {html_url}");
    println!("visibility:  {visibility}");
    println!("stars:       {}", format_count(stars));
    println!("forks:       {}", format_count(forks));
    println!("issues:      {}", format_count(issues));
    println!("language:    {language}");
    println!("license:     {license_name}");
    println!("default:     {default_branch}");
    println!("updated:     {}", format_date(pushed_at));
}

/// Execute `gor repo sync`.
///
/// Syncs a fork's default branch from its upstream repository.
///
/// # Errors
///
/// Returns an error if the repository is not a fork, the API request fails,
/// or the sync would result in merge conflicts.
fn sync(repo: Option<&str>, branch: Option<&str>, hostname: Option<&str>) -> anyhow::Result<()> {
    let spec = match repo {
        Some(s) => parse_repo_spec(s).context("invalid repository spec")?,
        None => detect_remote().ok_or_else(|| {
            anyhow::anyhow!(
                "could not detect repository from current directory; specify OWNER/REPO with --repo"
            )
        })?,
    };

    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let path = format!("/repos/{}/{}/merge-upstream", spec.owner, spec.repo);

    let mut body_map = serde_json::Map::new();
    body_map.insert(
        "branch".to_string(),
        serde_json::Value::String(branch.unwrap_or("main").to_string()),
    );
    let body_value = serde_json::Value::Object(body_map);

    let response = client
        .post(&path, &body_value)
        .context("failed to sync repository")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("repository '{spec}' not found or is not a fork");
    }
    if status == reqwest::StatusCode::CONFLICT {
        anyhow::bail!(
            "sync would result in merge conflicts; resolve conflicts manually and try again"
        );
    }
    if status == reqwest::StatusCode::UNPROCESSABLE_ENTITY {
        let err_body: serde_json::Value = response.json().unwrap_or_default();
        let msg = err_body["message"].as_str().unwrap_or(
            "sync failed - check that the repository is a fork and has an upstream configured",
        );
        anyhow::bail!("{msg}");
    }
    if !status.is_success() {
        let err_body: serde_json::Value = response.json().unwrap_or_default();
        let msg = err_body["message"].as_str().unwrap_or("sync failed");
        anyhow::bail!("failed to sync '{spec}': {msg}");
    }

    let result: serde_json::Value = response.json().context("failed to parse sync response")?;

    if let Some(merge_sha) = result["merge_after"].as_str() {
        println!("✓ Synced '{spec}' with upstream (merge commit: {merge_sha})");
    } else {
        println!("✓ '{spec}' is already up to date with upstream");
    }

    Ok(())
}

/// Open a URL in the default browser using the system's default handler.
fn open_in_browser(url: &str) {
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open").arg(url).spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(url).spawn();
    }
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/c", "start", url])
            .spawn();
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        println!("Open {url} in your browser");
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn print_repo_table_basic() {
        let repo = json!({
            "full_name": "octocat/hello-world",
            "description": "My first repository",
            "html_url": "https://github.com/octocat/hello-world",
            "private": false,
            "stargazers_count": 1234,
            "forks_count": 56,
            "open_issues_count": 12,
            "language": "Rust",
            "license": { "spdx_id": "MIT" },
            "default_branch": "main",
            "pushed_at": "2024-01-15T10:30:00Z"
        });
        // Should not panic
        print_repo_table(&repo);
    }

    #[test]
    fn print_repo_table_private() {
        let repo = json!({
            "full_name": "org/private-repo",
            "description": null,
            "html_url": "https://github.com/org/private-repo",
            "private": true,
            "stargazers_count": 0,
            "forks_count": 0,
            "open_issues_count": 0,
            "language": null,
            "license": null,
            "default_branch": "main",
            "pushed_at": "2024-01-15T10:30:00Z"
        });
        // Should not panic with null fields
        print_repo_table(&repo);
    }

    #[test]
    fn print_repo_table_no_license() {
        let repo = json!({
            "full_name": "user/test",
            "description": "A test repo",
            "html_url": "https://github.com/user/test",
            "private": false,
            "stargazers_count": 42,
            "forks_count": 7,
            "open_issues_count": 3,
            "language": "Python",
            "license": null,
            "default_branch": "master",
            "pushed_at": "2023-12-25T00:00:00Z"
        });
        // Should not panic with null license
        print_repo_table(&repo);
    }

    #[test]
    fn open_in_browser_does_not_panic() {
        // Just verify it doesn't panic — actual browser opening is a no-op in tests
        open_in_browser("https://github.com/octocat/hello-world");
    }

    #[test]
    fn print_repo_list_table_basic() {
        let repos = vec![json!({
            "full_name": "octocat/hello-world",
            "description": "My first repository",
            "private": false,
            "language": "Rust",
            "updated_at": "2024-01-15T10:30:00Z"
        })];
        print_repo_list_table(&repos);
    }

    #[test]
    fn print_repo_list_table_empty() {
        let repos: Vec<serde_json::Value> = vec![];
        print_repo_list_table(&repos);
    }

    #[test]
    fn print_repo_list_table_multiple() {
        let repos = vec![
            json!({
                "full_name": "alice/project-a",
                "description": "First project",
                "private": false,
                "language": "Rust",
                "updated_at": "2024-01-15T10:30:00Z"
            }),
            json!({
                "full_name": "bob/private-repo",
                "description": "A secret project with a very long description that should be truncated",
                "private": true,
                "language": "Python",
                "updated_at": "2024-02-20T12:00:00Z"
            }),
        ];
        print_repo_list_table(&repos);
    }

    #[test]
    fn print_repo_list_table_null_fields() {
        let repos = vec![json!({
            "full_name": "test/repo",
            "description": null,
            "private": false,
            "language": null,
            "updated_at": null
        })];
        print_repo_list_table(&repos);
    }

    #[test]
    fn filter_repos_by_visibility() {
        let repos = vec![
            json!({"full_name": "a/public", "private": false, "fork": false, "language": "Rust"}),
            json!({"full_name": "b/private", "private": true, "fork": false, "language": "Rust"}),
        ];
        let filtered = filter_repos(repos, "public", "include", None, 30);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0]["full_name"], "a/public");
    }

    #[test]
    fn filter_repos_by_fork_exclude() {
        let repos = vec![
            json!({"full_name": "a/original", "private": false, "fork": false, "language": "Rust"}),
            json!({"full_name": "b/forked", "private": false, "fork": true, "language": "Rust"}),
        ];
        let filtered = filter_repos(repos, "all", "exclude", None, 30);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0]["full_name"], "a/original");
    }

    #[test]
    fn filter_repos_by_fork_only() {
        let repos = vec![
            json!({"full_name": "a/original", "private": false, "fork": false, "language": "Rust"}),
            json!({"full_name": "b/forked", "private": false, "fork": true, "language": "Rust"}),
        ];
        let filtered = filter_repos(repos, "all", "only", None, 30);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0]["full_name"], "b/forked");
    }

    #[test]
    fn filter_repos_by_language() {
        let repos = vec![
            json!({"full_name": "a/rust-repo", "private": false, "fork": false, "language": "Rust"}),
            json!({"full_name": "b/python-repo", "private": false, "fork": false, "language": "Python"}),
        ];
        let filtered = filter_repos(repos, "all", "include", Some("Rust"), 30);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0]["full_name"], "a/rust-repo");
    }

    #[test]
    fn filter_repos_by_limit() {
        let repos: Vec<serde_json::Value> = (0..10)
            .map(|i| json!({"full_name": format!("owner/repo{i}"), "private": false, "fork": false, "language": "Rust"}))
            .collect();
        let filtered = filter_repos(repos, "all", "include", None, 5);
        assert_eq!(filtered.len(), 5);
    }

    #[test]
    fn clone_directory_name_from_url() {
        // Test the directory name derivation logic used in clone()
        let url = "https://github.com/octocat/hello-world.git";
        let name = url
            .rfind('/')
            .map_or(url, |pos| url[pos + 1..].trim_end_matches(".git"));
        assert_eq!(name, "hello-world");

        // Without .git suffix
        let url = "https://github.com/octocat/hello-world";
        let name = url
            .rfind('/')
            .map_or(url, |pos| url[pos + 1..].trim_end_matches(".git"));
        assert_eq!(name, "hello-world");

        // SSH URL with path
        let url = "git@github.com:octocat/repo.git";
        let name = url
            .rfind('/')
            .map_or(url, |pos| url[pos + 1..].trim_end_matches(".git"));
        assert_eq!(name, "repo");
    }
}
