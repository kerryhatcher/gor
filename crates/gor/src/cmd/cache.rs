//! Implementation of the `gor cache` subcommand.

#![allow(clippy::print_stdout)]

use crate::cli::CacheCommand;
use crate::cmd::util::truncate;
use crate::render::print_json;
use gor_core::Client;
use gor_core::cache::{self, DeleteOptions};
use gor_core::repository::{detect_remote, parse_repo_spec};

/// Run the `gor cache` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cmd: CacheCommand) -> anyhow::Result<()> {
    match cmd {
        CacheCommand::List {
            repo,
            json,
            hostname,
        } => list(repo.as_deref(), json, hostname.as_deref()),
        CacheCommand::Delete {
            key,
            repo,
            all,
            key_prefix,
            ref_,
            hostname,
        } => delete(
            key.as_deref(),
            repo.as_deref(),
            all,
            key_prefix.as_deref(),
            ref_.as_deref(),
            hostname.as_deref(),
        ),
    }
}

fn resolve_spec(repo: Option<&str>) -> anyhow::Result<gor_core::RepoSplit> {
    match repo {
        Some(s) => Ok(parse_repo_spec(s)?),
        None => detect_remote().ok_or_else(|| {
            anyhow::anyhow!("could not detect repository; specify OWNER/REPO with --repo")
        }),
    }
}

fn build_client(hostname: Option<&str>) -> anyhow::Result<Client> {
    let host = hostname.unwrap_or("github.com");
    Client::new(host).map_err(|e| anyhow::anyhow!("failed to create HTTP client: {e}"))
}

fn list(
    repo: Option<&str>,
    json: Option<Vec<String>>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let spec = resolve_spec(repo)?;
    let client = build_client(hostname)?;

    let caches = cache::list(&client, &spec)?;

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        let values: Vec<serde_json::Value> = caches
            .into_iter()
            .map(|c| serde_json::to_value(c).unwrap_or_default())
            .collect();
        print_json(&values, fields_ref);
        return Ok(());
    }

    if caches.is_empty() {
        println!("No caches found.");
        return Ok(());
    }

    println!("{:<30}  {:<10}  CREATED", "KEY", "SIZE (MB)");
    for c in &caches {
        let key_truncated = truncate(&c.key, 30);
        println!(
            "{key_truncated:<30}  {:<10}  {}",
            c.size_in_bytes / 1024 / 1024,
            c.created_at.as_deref().unwrap_or("—"),
        );
    }

    Ok(())
}

fn delete(
    key: Option<&str>,
    repo: Option<&str>,
    all: bool,
    key_prefix: Option<&str>,
    ref_: Option<&str>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let spec = resolve_spec(repo)?;
    let client = build_client(hostname)?;

    let opts = DeleteOptions {
        key,
        key_prefix,
        ref_,
    };
    let count = cache::delete(&client, &spec, &opts)?;

    if all {
        println!("Deleted all caches ({count} total).");
    } else if let Some(k) = key {
        println!("Deleted cache '{k}'.");
    } else if let Some(prefix) = key_prefix {
        println!("Deleted {count} cache(s) with prefix '{prefix}'.");
    }

    Ok(())
}
