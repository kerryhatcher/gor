//! Implementation of the `gor cache` subcommand.
//!
//! Provides repository cache listing and deletion.

#![allow(clippy::print_stdout)]

use crate::cli::CacheCommand;
use crate::client::Client;
use crate::output::print_json;
use crate::repository;
use anyhow::Context;

use std::fmt::Write;

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

fn list(
    repo: Option<&str>,
    json: Option<Vec<String>>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let spec = match repo {
        Some(s) => repository::parse_repo_spec(s).context("invalid repository spec")?,
        None => repository::detect_remote().ok_or_else(|| {
            anyhow::anyhow!("could not detect repository; specify OWNER/REPO with --repo")
        })?,
    };

    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let path = format!(
        "/repos/{}/{}/actions/caches?per_page=100",
        spec.owner, spec.repo
    );

    let response = client.get(&path).context("failed to fetch caches")?;
    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("failed to list caches: HTTP {status}");
    }

    let result: serde_json::Value = response.json().context("failed to parse response")?;
    let caches: Vec<serde_json::Value> = result["actions_caches"]
        .as_array()
        .map_or_else(Vec::new, Clone::clone);

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&caches, fields_ref);
        return Ok(());
    }

    if caches.is_empty() {
        println!("No caches found.");
        return Ok(());
    }

    println!("{:<30}  {:<10}  CREATED", "KEY", "SIZE (MB)");
    for c in &caches {
        let key = c["key"].as_str().unwrap_or("—");
        let size = c["size_in_bytes"].as_u64().unwrap_or(0);
        let created = c["created_at"].as_str().unwrap_or("—");
        let key_truncated = crate::cmd::util::truncate(key, 30);
        println!("{key_truncated:<30}  {:<10}  {created}", size / 1024 / 1024);
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
    let spec = match repo {
        Some(s) => repository::parse_repo_spec(s).context("invalid repository spec")?,
        None => repository::detect_remote().ok_or_else(|| {
            anyhow::anyhow!("could not detect repository; specify OWNER/REPO with --repo")
        })?,
    };

    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let mut path = format!("/repos/{}/{}/actions/caches", spec.owner, spec.repo);

    if let Some(k) = key {
        let _ = write!(path, "?key={k}");
    } else if let Some(prefix) = key_prefix {
        let _ = write!(path, "?key={prefix}");
    }

    if let Some(r) = ref_ {
        let sep = if path.contains('?') { "&" } else { "?" };
        let _ = write!(path, "{sep}ref={r}");
    }

    let response = client
        .request("DELETE", &path, &[], None)
        .context("failed to delete caches")?;

    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("failed to delete caches: HTTP {status}");
    }

    let result: serde_json::Value = response.json().context("failed to parse response")?;
    let count = result["total_count"].as_u64().unwrap_or(0);

    if all {
        println!("Deleted all caches ({count} total).");
    } else if let Some(k) = key {
        println!("Deleted cache '{k}'.");
    } else if let Some(prefix) = key_prefix {
        println!("Deleted {count} cache(s) with prefix '{prefix}'.");
    }

    Ok(())
}
