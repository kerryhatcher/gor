//! Implementation of the `gor release` subcommand.
//!
//! Provides release listing functionality.
//! Currently supports `gor release list` for listing repository releases.

#![allow(clippy::print_stdout)]

use crate::cli::ReleaseCommand;
use crate::client::Client;
use crate::output::{format_date, print_json};
use crate::repository::{detect_remote, parse_repo_spec};
use anyhow::Context;

/// Run the `gor release` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cmd: ReleaseCommand) -> anyhow::Result<()> {
    match cmd {
        ReleaseCommand::List {
            repo,
            limit,
            exclude_drafts,
            exclude_prereleases,
            json,
            hostname,
        } => list(
            repo.as_deref(),
            limit,
            exclude_drafts,
            exclude_prereleases,
            json,
            hostname.as_deref(),
        ),
        ReleaseCommand::View {
            release,
            repo,
            web,
            json,
            hostname,
        } => view(&release, repo.as_deref(), web, json, hostname.as_deref()),
    }
}

/// Execute `gor release list`.
///
/// Lists releases for a repository. Supports filtering by draft and prerelease
/// status, limiting results, and JSON output with field selection.
///
/// # Errors
///
/// Returns an error if the repository cannot be found or the API request fails.
fn list(
    repo: Option<&str>,
    limit: u32,
    exclude_drafts: bool,
    exclude_prereleases: bool,
    json: Option<Vec<String>>,
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

    let path = format!(
        "/repos/{}/{}/releases?per_page={}",
        spec.owner,
        spec.repo,
        limit.min(100)
    );
    let response = client.get(&path).context("failed to fetch releases")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("repository '{spec}' not found");
    }
    if !status.is_success() {
        anyhow::bail!("failed to list releases for '{spec}': HTTP {status}");
    }

    let mut releases: Vec<serde_json::Value> = response
        .json()
        .context("failed to parse releases response")?;

    // Client-side filtering
    if exclude_drafts {
        releases.retain(|r| !r["draft"].as_bool().unwrap_or(false));
    }
    if exclude_prereleases {
        releases.retain(|r| !r["prerelease"].as_bool().unwrap_or(false));
    }

    // Apply limit
    releases.truncate(limit as usize);

    // Handle --json flag
    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&releases, fields_ref);
        return Ok(());
    }

    // Default: print formatted table
    print_release_table(&releases);
    Ok(())
}

/// Print a formatted release list table.
///
/// Columns: TAG NAME, RELEASE NAME, PUBLISHED, STATUS
fn print_release_table(releases: &[serde_json::Value]) {
    if releases.is_empty() {
        println!("No releases found.");
        return;
    }

    let tag_width = 20;
    let name_width = 40;
    let date_width = 16;
    let status_width = 12;

    println!(
        "{:<tag_width$}  {:<name_width$}  {:<date_width$}  {:<status_width$}",
        "TAG", "NAME", "PUBLISHED", "STATUS",
    );

    for release in releases {
        let tag = release["tag_name"].as_str().unwrap_or("—");
        let name = release["name"].as_str().unwrap_or("—");
        let published = release["published_at"]
            .as_str()
            .map_or_else(|| "—".to_string(), format_date);
        let is_draft = release["draft"].as_bool().unwrap_or(false);
        let is_prerelease = release["prerelease"].as_bool().unwrap_or(false);
        let status = if is_draft {
            "Draft"
        } else if is_prerelease {
            "Pre-release"
        } else {
            "Latest"
        };

        let tag_truncated = crate::cmd::util::truncate(tag, tag_width);
        let name_truncated = crate::cmd::util::truncate(name, name_width);

        println!(
            "{tag_truncated:<tag_width$}  {name_truncated:<name_width$}  {published:<date_width$}  {status:<status_width$}",
        );
    }
}

/// Execute `gor release view`.
///
/// Views details of a single release by tag name or database ID.
/// Supports opening in browser, JSON output, and asset listing.
///
/// # Errors
///
/// Returns an error if the repository cannot be found, the release does not
/// exist, or the API request fails.
fn view(
    release: &str,
    repo: Option<&str>,
    web: bool,
    json: Option<Vec<String>>,
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

    // Handle --web flag: open in browser
    if web {
        let web_url = format!(
            "https://{host}/{}/{}/releases/tag/{release}",
            spec.owner, spec.repo
        );
        open_in_browser(&web_url);
        return Ok(());
    }

    let client = Client::new(host).context("failed to create HTTP client")?;

    // Try by tag first, then by ID
    let tag_path = format!(
        "/repos/{}/{}/releases/tags/{release}",
        spec.owner, spec.repo
    );
    let response = client.get(&tag_path);

    let release_data: serde_json::Value = match response {
        Ok(resp) if resp.status().is_success() => {
            resp.json().context("failed to parse release response")?
        }
        _ => {
            // Try by numeric ID
            let id: u64 = release
                .parse()
                .context("release not found by tag; provide a valid numeric release ID")?;
            let id_path = format!("/repos/{}/{}/releases/{id}", spec.owner, spec.repo);
            let resp = client.get(&id_path).context("failed to fetch release")?;
            let status = resp.status();
            if status == reqwest::StatusCode::NOT_FOUND {
                anyhow::bail!("release '{release}' not found in '{spec}'");
            }
            if !status.is_success() {
                anyhow::bail!("failed to view release '{release}': HTTP {status}");
            }
            resp.json().context("failed to parse release response")?
        }
    };

    // Handle --json flag
    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&release_data, fields_ref);
        return Ok(());
    }

    // Print formatted view
    print_release_view(&release_data);
    Ok(())
}

/// Print a formatted single-release view.
///
/// Displays tag, name, status, published date, author, URL, body, and assets.
fn print_release_view(release: &serde_json::Value) {
    let tag = release["tag_name"].as_str().unwrap_or("—");
    let name = release["name"].as_str().unwrap_or("—");
    let body = release["body"].as_str().unwrap_or("");
    let html_url = release["html_url"].as_str().unwrap_or("—");
    let published = release["published_at"]
        .as_str()
        .map_or_else(|| "—".to_string(), format_date);
    let author = release["author"]["login"].as_str().unwrap_or("—");
    let is_draft = release["draft"].as_bool().unwrap_or(false);
    let is_prerelease = release["prerelease"].as_bool().unwrap_or(false);
    let status = if is_draft {
        "Draft"
    } else if is_prerelease {
        "Pre-release"
    } else {
        "Latest"
    };

    println!("Tag:       {tag}");
    println!("Name:      {name}");
    println!("Status:    {status}");
    println!("Published: {published}");
    println!("Author:    {author}");
    println!("URL:       {html_url}");

    if !body.is_empty() {
        println!();
        println!("{body}");
    }

    // Assets
    let assets = release["assets"].as_array();
    if let Some(assets) = assets {
        if !assets.is_empty() {
            println!();
            println!("Assets:");
            let name_width = 40;
            let size_width = 12;
            let dl_width = 12;
            println!(
                "  {:<name_width$}  {:>size_width$}  {:>dl_width$}",
                "NAME", "SIZE", "DOWNLOADS"
            );
            for asset in assets {
                let asset_name = asset["name"].as_str().unwrap_or("—");
                let size = asset["size"]
                    .as_u64()
                    .map_or_else(|| "—".to_string(), format_size);
                let downloads = asset["download_count"]
                    .as_u64()
                    .map_or_else(|| "—".to_string(), |d| d.to_string());
                let name_truncated = crate::cmd::util::truncate(asset_name, name_width);
                println!(
                    "  {name_truncated:<name_width$}  {size:>size_width$}  {downloads:>dl_width$}"
                );
            }
        }
    }
}

/// Format a byte size as a human-readable string.
#[allow(clippy::cast_precision_loss)]
fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    if unit_idx == 0 {
        format!("{bytes} B")
    } else {
        format!("{size:.1} {}", UNITS[unit_idx])
    }
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
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn print_release_table_basic() {
        let releases = vec![json!({
            "tag_name": "v1.0.0",
            "name": "First Release",
            "published_at": "2024-01-15T10:30:00Z",
            "draft": false,
            "prerelease": false
        })];
        print_release_table(&releases);
    }

    #[test]
    fn print_release_table_empty() {
        let releases: Vec<serde_json::Value> = vec![];
        print_release_table(&releases);
    }

    #[test]
    fn print_release_table_multiple() {
        let releases = vec![
            json!({
                "tag_name": "v1.0.0",
                "name": "First Release",
                "published_at": "2024-01-15T10:30:00Z",
                "draft": false,
                "prerelease": false
            }),
            json!({
                "tag_name": "v2.0.0-beta",
                "name": "Beta Release",
                "published_at": "2024-03-01T00:00:00Z",
                "draft": false,
                "prerelease": true
            }),
            json!({
                "tag_name": "v2.0.0-wip",
                "name": "Work in Progress",
                "published_at": null,
                "draft": true,
                "prerelease": false
            }),
        ];
        print_release_table(&releases);
    }

    #[test]
    fn print_release_table_null_fields() {
        let releases = vec![json!({
            "tag_name": null,
            "name": null,
            "published_at": null,
            "draft": null,
            "prerelease": null
        })];
        print_release_table(&releases);
    }

    #[test]
    fn print_release_view_basic() {
        let release = json!({
            "tag_name": "v1.0.0",
            "name": "First Release",
            "body": "Release notes here",
            "html_url": "https://github.com/owner/repo/releases/tag/v1.0.0",
            "published_at": "2024-01-15T10:30:00Z",
            "author": { "login": "octocat" },
            "draft": false,
            "prerelease": false,
            "assets": [
                {
                    "name": "app-linux-amd64.tar.gz",
                    "size": 5_242_880,
                    "download_count": 42
                },
                {
                    "name": "app-darwin-amd64.tar.gz",
                    "size": 4_194_304,
                    "download_count": 18
                }
            ]
        });
        print_release_view(&release);
    }

    #[test]
    fn print_release_view_draft() {
        let release = json!({
            "tag_name": "v2.0.0-beta",
            "name": null,
            "body": "",
            "html_url": "https://github.com/owner/repo/releases/tag/v2.0.0-beta",
            "published_at": null,
            "author": null,
            "draft": true,
            "prerelease": false,
            "assets": []
        });
        print_release_view(&release);
    }

    #[test]
    fn print_release_view_prerelease() {
        let release = json!({
            "tag_name": "v2.0.0-rc1",
            "name": "Release Candidate 1",
            "body": "Pre-release testing",
            "html_url": "https://github.com/owner/repo/releases/tag/v2.0.0-rc1",
            "published_at": "2024-06-01T00:00:00Z",
            "author": { "login": "devbot" },
            "draft": false,
            "prerelease": true,
            "assets": null
        });
        print_release_view(&release);
    }

    #[test]
    fn format_size_bytes() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(500), "500 B");
        assert_eq!(format_size(1024), "1.0 KB");
        assert_eq!(format_size(1_048_576), "1.0 MB");
        assert_eq!(format_size(1_073_741_824), "1.0 GB");
        assert_eq!(format_size(1_536), "1.5 KB");
    }

    #[test]
    fn open_in_browser_does_not_panic() {
        // This should not panic even if no browser is available
        open_in_browser("https://example.com");
    }
}
