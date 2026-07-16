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
use std::io::Write;

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
        ReleaseCommand::Delete {
            release,
            repo,
            yes,
            skip_tag,
            hostname,
        } => delete(
            &release,
            repo.as_deref(),
            yes,
            skip_tag,
            hostname.as_deref(),
        ),
        ReleaseCommand::Edit {
            release,
            repo,
            title,
            notes,
            notes_file,
            draft,
            prerelease,
            tag,
            target,
            hostname,
        } => edit(
            &release,
            repo.as_deref(),
            title.as_deref(),
            notes.as_deref(),
            notes_file.as_deref(),
            draft,
            prerelease,
            tag.as_deref(),
            target.as_deref(),
            hostname.as_deref(),
        ),
        ReleaseCommand::Upload {
            release,
            files,
            repo,
            name,
            mime_type,
            hostname,
        } => upload(
            &release,
            &files,
            repo.as_deref(),
            name.as_deref(),
            mime_type.as_deref(),
            hostname.as_deref(),
        ),
        ReleaseCommand::Create {
            tag,
            repo,
            title,
            notes,
            notes_file,
            draft,
            prerelease,
            target,
            discussion_category,
            hostname,
        } => create_release(
            &tag,
            repo.as_deref(),
            title.as_deref(),
            notes.as_deref(),
            notes_file.as_deref(),
            draft,
            prerelease,
            target.as_deref(),
            discussion_category.as_deref(),
            hostname.as_deref(),
        ),
        ReleaseCommand::Download {
            release,
            repo,
            patterns,
            dir,
            skip_existing,
            hostname,
        } => download(
            &release,
            repo.as_deref(),
            &patterns,
            &dir,
            skip_existing,
            hostname.as_deref(),
        ),
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

/// Execute `gor release delete`.
///
/// Deletes a release by tag name or database ID. Prompts for confirmation
/// unless `--yes` is passed. Optionally keeps the associated git tag.
///
/// # Errors
///
/// Returns an error if the repository cannot be found, the release does not
/// exist, or the API request fails.
fn delete(
    release: &str,
    repo: Option<&str>,
    yes: bool,
    skip_tag: bool,
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

    // Resolve the release to get its ID and tag name
    let (release_id, tag_name) = resolve_release(&client, &spec.owner, &spec.repo, release)?;

    // Confirmation prompt
    if !yes {
        print!("Delete release '{tag_name}' (ID: {release_id}) from '{spec}'? [y/N] ");
        std::io::stdout().flush().ok();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        let trimmed = input.trim().to_lowercase();
        if trimmed != "y" && trimmed != "yes" {
            println!("Cancelled.");
            return Ok(());
        }
    }

    // Build the delete URL with optional skip_tag
    let mut path = format!("/repos/{}/{}/releases/{release_id}", spec.owner, spec.repo);
    if skip_tag {
        path.push_str("?skip_tag=true");
    }

    let response = client
        .request("DELETE", &path, &[], None)
        .context("failed to delete release")?;
    let status = response.status();

    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("release '{release}' not found in '{spec}'");
    }
    if status == reqwest::StatusCode::NO_CONTENT {
        let tag_msg = if skip_tag { " (git tag preserved)" } else { "" };
        println!("✓ Deleted release '{tag_name}'{tag_msg}");
        return Ok(());
    }
    if !status.is_success() {
        anyhow::bail!("failed to delete release '{release}': HTTP {status}");
    }

    Ok(())
}

/// Resolve a release identifier (tag or ID) to its numeric ID and tag name.
///
/// Tries by tag first, then falls back to numeric ID.
fn resolve_release(
    client: &Client,
    owner: &str,
    repo: &str,
    release: &str,
) -> anyhow::Result<(u64, String)> {
    // Try by tag first
    let tag_path = format!("/repos/{owner}/{repo}/releases/tags/{release}");
    if let Ok(resp) = client.get(&tag_path) {
        if resp.status().is_success() {
            let data: serde_json::Value =
                resp.json().context("failed to parse release response")?;
            let id = data["id"]
                .as_u64()
                .context("release response missing 'id' field")?;
            let tag = data["tag_name"].as_str().unwrap_or(release).to_string();
            return Ok((id, tag));
        }
    }

    // Try by numeric ID
    let id: u64 = release
        .parse()
        .context("release not found by tag; provide a valid numeric release ID")?;
    let id_path = format!("/repos/{owner}/{repo}/releases/{id}");
    let resp = client.get(&id_path).context("failed to fetch release")?;
    let status = resp.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("release '{release}' not found in '{owner}/{repo}'");
    }
    if !status.is_success() {
        anyhow::bail!("failed to resolve release '{release}': HTTP {status}");
    }
    let data: serde_json::Value = resp.json().context("failed to parse release response")?;
    let tag = data["tag_name"].as_str().unwrap_or(release).to_string();
    Ok((id, tag))
}

/// Execute `gor release edit`.
///
/// Edits an existing release's metadata. Supports updating title, body,
/// draft/prerelease status, tag name, and target commitish.
///
/// # Errors
///
/// Returns an error if the repository cannot be found, the release does not
/// exist, or the API request fails.
#[allow(clippy::too_many_arguments)]
fn edit(
    release: &str,
    repo: Option<&str>,
    title: Option<&str>,
    notes: Option<&str>,
    notes_file: Option<&str>,
    draft: Option<bool>,
    prerelease: Option<bool>,
    tag: Option<&str>,
    target: Option<&str>,
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

    // Resolve the release to get its ID
    let (release_id, _tag_name) = resolve_release(&client, &spec.owner, &spec.repo, release)?;

    // Read notes from file if specified
    let body = if let Some(file) = notes_file {
        let content = if file == "-" {
            use std::io::Read;
            let mut buf = String::new();
            std::io::stdin()
                .read_to_string(&mut buf)
                .context("failed to read notes from stdin")?;
            buf
        } else {
            std::fs::read_to_string(file)
                .with_context(|| format!("failed to read notes file: {file}"))?
        };
        Some(content)
    } else {
        notes.map(String::from)
    };

    // Build the PATCH body
    let mut patch = serde_json::Map::new();
    if let Some(t) = title {
        patch.insert("name".to_string(), serde_json::Value::String(t.to_string()));
    }
    if let Some(ref b) = body {
        patch.insert("body".to_string(), serde_json::Value::String(b.clone()));
    }
    if let Some(d) = draft {
        patch.insert("draft".to_string(), serde_json::Value::Bool(d));
    }
    if let Some(p) = prerelease {
        patch.insert("prerelease".to_string(), serde_json::Value::Bool(p));
    }
    if let Some(t) = tag {
        patch.insert(
            "tag_name".to_string(),
            serde_json::Value::String(t.to_string()),
        );
    }
    if let Some(t) = target {
        patch.insert(
            "target_commitish".to_string(),
            serde_json::Value::String(t.to_string()),
        );
    }

    if patch.is_empty() {
        anyhow::bail!(
            "no changes specified; use --title, --notes, --draft, --prerelease, --tag, or --target"
        );
    }

    let api_path = format!("/repos/{}/{}/releases/{release_id}", spec.owner, spec.repo);
    let response = client
        .request("PATCH", &api_path, &[], Some(serde_json::to_vec(&patch)?))
        .context("failed to edit release")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("release '{release}' not found in '{spec}'");
    }
    if !status.is_success() {
        anyhow::bail!("failed to edit release '{release}': HTTP {status}");
    }

    let updated: serde_json::Value = response.json().context("failed to parse response")?;
    let html_url = updated["html_url"].as_str().unwrap_or("(unknown URL)");
    println!("✓ Release updated: {html_url}");
    Ok(())
}

/// Execute `gor release upload`.
///
/// Uploads one or more asset files to an existing release. Auto-detects
/// MIME types from file extensions, with optional override.
///
/// # Errors
///
/// Returns an error if the repository cannot be found, the release does not
/// exist, a file cannot be read, or the upload fails.
fn upload(
    release: &str,
    files: &[String],
    repo: Option<&str>,
    name: Option<&str>,
    mime_type: Option<&str>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    if files.is_empty() {
        anyhow::bail!("no files specified for upload");
    }

    if name.is_some() && files.len() > 1 {
        anyhow::bail!("--name can only be used with a single file upload");
    }

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

    // Resolve the release to get its upload URL
    let (release_id, tag_name) = resolve_release(&client, &spec.owner, &spec.repo, release)?;

    // Fetch the full release to get the upload_url
    let get_path = format!("/repos/{}/{}/releases/{release_id}", spec.owner, spec.repo);
    let resp = client.get(&get_path).context("failed to fetch release")?;
    let release_data: serde_json::Value =
        resp.json().context("failed to parse release response")?;
    let upload_url_template = release_data["upload_url"]
        .as_str()
        .context("release response missing 'upload_url'")?;

    for file_path in files {
        let file_name = name.unwrap_or_else(|| {
            std::path::Path::new(file_path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(file_path)
        });

        // Read the file
        let file_data = std::fs::read(file_path)
            .with_context(|| format!("failed to read file: {file_path}"))?;
        let file_size = file_data.len();

        // Determine MIME type
        let content_type = mime_type.map_or_else(|| detect_mime_type(file_name), String::from);

        // Build the upload URL (replace {?name,label} with ?name=...)
        let upload_url =
            upload_url_template.replace("{?name,label}", &format!("?name={file_name}"));

        // Upload the asset
        print!(
            "Uploading {file_name} ({}) ... ",
            format_size(file_size as u64)
        );
        std::io::stdout().flush().ok();

        let response = client
            .upload_asset(&upload_url, &file_data, &content_type)
            .with_context(|| format!("failed to upload {file_name}"))?;

        let status = response.status();
        if status == reqwest::StatusCode::UNPROCESSABLE_ENTITY {
            anyhow::bail!(
                "asset '{file_name}' already exists on release '{tag_name}'; use a different name or delete the existing asset first"
            );
        }
        if !status.is_success() {
            let err_body: serde_json::Value = response.json().unwrap_or_default();
            let msg = err_body["message"].as_str().unwrap_or("upload failed");
            anyhow::bail!("failed to upload '{file_name}': {msg}");
        }

        let asset: serde_json::Value = response.json().context("failed to parse asset response")?;
        let asset_url = asset["browser_download_url"]
            .as_str()
            .unwrap_or("(unknown URL)");

        println!("done");
        println!("{asset_url}");
    }

    Ok(())
}

/// Detect MIME type from a file extension.
fn detect_mime_type(filename: &str) -> String {
    let ext = std::path::Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "zip" => "application/zip".to_string(),
        "tar" => "application/x-tar".to_string(),
        "gz" | "tgz" => "application/gzip".to_string(),
        "bz2" => "application/x-bzip2".to_string(),
        "xz" => "application/x-xz".to_string(),
        "dmg" => "application/x-apple-diskimage".to_string(),
        "deb" => "application/vnd.debian.binary-package".to_string(),
        "rpm" => "application/x-rpm".to_string(),
        "msi" => "application/x-msi".to_string(),
        "exe" => "application/x-msdownload".to_string(),
        "apk" => "application/vnd.android.package-archive".to_string(),
        "jar" => "application/java-archive".to_string(),
        "txt" => "text/plain".to_string(),
        "md" => "text/markdown".to_string(),
        "json" => "application/json".to_string(),
        "xml" => "application/xml".to_string(),
        "html" => "text/html".to_string(),
        "css" => "text/css".to_string(),
        "js" => "application/javascript".to_string(),
        "wasm" => "application/wasm".to_string(),
        "png" => "image/png".to_string(),
        "jpg" | "jpeg" => "image/jpeg".to_string(),
        "gif" => "image/gif".to_string(),
        "svg" => "image/svg+xml".to_string(),
        "ico" => "image/x-icon".to_string(),
        "pdf" => "application/pdf".to_string(),
        _ => "application/octet-stream".to_string(),
    }
}

/// Execute `gor release create`.
///
/// Creates a new GitHub release for an existing tag. Supports draft,
/// prerelease, notes from file/stdin, and discussion category.
///
/// # Errors
///
/// Returns an error if the repository cannot be found, the tag does not
/// exist, or the API request fails.
#[allow(clippy::too_many_arguments)]
fn create_release(
    tag: &str,
    repo: Option<&str>,
    title: Option<&str>,
    notes: Option<&str>,
    notes_file: Option<&str>,
    draft: bool,
    prerelease: bool,
    target: Option<&str>,
    discussion_category: Option<&str>,
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

    // Read notes from file if specified
    let body = if let Some(file) = notes_file {
        let content = if file == "-" {
            use std::io::Read;
            let mut buf = String::new();
            std::io::stdin()
                .read_to_string(&mut buf)
                .context("failed to read notes from stdin")?;
            buf
        } else {
            std::fs::read_to_string(file)
                .with_context(|| format!("failed to read notes file: {file}"))?
        };
        Some(content)
    } else {
        notes.map(String::from)
    };

    // Build the request body
    let mut body_map = serde_json::Map::new();
    body_map.insert(
        "tag_name".to_string(),
        serde_json::Value::String(tag.to_string()),
    );
    body_map.insert(
        "name".to_string(),
        serde_json::Value::String(title.unwrap_or(tag).to_string()),
    );
    if let Some(ref b) = body {
        body_map.insert("body".to_string(), serde_json::Value::String(b.clone()));
    }
    if draft {
        body_map.insert("draft".to_string(), serde_json::Value::Bool(true));
    }
    if prerelease {
        body_map.insert("prerelease".to_string(), serde_json::Value::Bool(true));
    }
    if let Some(t) = target {
        body_map.insert(
            "target_commitish".to_string(),
            serde_json::Value::String(t.to_string()),
        );
    }
    if let Some(c) = discussion_category {
        body_map.insert(
            "discussion_category_name".to_string(),
            serde_json::Value::String(c.to_string()),
        );
    }

    let path = format!("/repos/{}/{}/releases", spec.owner, spec.repo);
    let body_value = serde_json::Value::Object(body_map);
    let response = client
        .post(&path, &body_value)
        .context("failed to create release")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("repository '{spec}' not found");
    }
    if !status.is_success() {
        let err_body: serde_json::Value = response.json().unwrap_or_default();
        let msg = err_body["message"].as_str().unwrap_or("creation failed");
        anyhow::bail!("failed to create release: {msg}");
    }

    let release_data: serde_json::Value = response
        .json()
        .context("failed to parse release response")?;
    let html_url = release_data["html_url"].as_str().unwrap_or("(unknown URL)");
    println!("{html_url}");
    Ok(())
}

/// Execute `gor release download`.
///
/// Downloads assets from a release. Supports glob pattern filtering,
/// output directory selection, and skipping existing files.
///
/// # Errors
///
/// Returns an error if the repository cannot be found, the release does not
/// exist, no assets match the pattern, or a download fails.
fn download(
    release: &str,
    repo: Option<&str>,
    patterns: &[String],
    dir: &str,
    skip_existing: bool,
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

    // Resolve the release
    let (release_id, _tag_name) = resolve_release(&client, &spec.owner, &spec.repo, release)?;

    // Fetch the release to get assets
    let get_path = format!("/repos/{}/{}/releases/{release_id}", spec.owner, spec.repo);
    let resp = client.get(&get_path).context("failed to fetch release")?;
    let release_data: serde_json::Value =
        resp.json().context("failed to parse release response")?;

    let assets = release_data["assets"]
        .as_array()
        .map_or(&[] as &[serde_json::Value], |a| a);

    // Filter assets by patterns
    let filtered: Vec<&serde_json::Value> = if patterns.is_empty() {
        assets.iter().collect()
    } else {
        assets
            .iter()
            .filter(|asset| {
                let name = asset["name"].as_str().unwrap_or("");
                patterns.iter().any(|p| glob_match(p, name))
            })
            .collect()
    };

    if filtered.is_empty() {
        if patterns.is_empty() {
            anyhow::bail!("release has no assets to download");
        }
        anyhow::bail!("no assets match the given pattern(s)");
    }

    // Ensure output directory exists
    std::fs::create_dir_all(dir)
        .with_context(|| format!("failed to create output directory: {dir}"))?;

    for asset in &filtered {
        let asset_name = asset["name"].as_str().unwrap_or("unknown");
        let asset_url = asset["browser_download_url"]
            .as_str()
            .context("asset missing download URL")?;
        let asset_size = asset["size"].as_u64().unwrap_or(0);

        let output_path = std::path::Path::new(dir).join(asset_name);

        // Skip if file exists
        if skip_existing && output_path.exists() {
            println!("Skipping {asset_name} (already exists)");
            continue;
        }

        // Download the asset
        print!(
            "Downloading {asset_name} ({}) ... ",
            format_size(asset_size)
        );
        std::io::stdout().flush().ok();

        let response = client
            .get_absolute(asset_url)
            .with_context(|| format!("failed to download {asset_name}"))?;

        let status = response.status();
        if !status.is_success() {
            anyhow::bail!("failed to download '{asset_name}': HTTP {status}");
        }

        let data = response
            .bytes()
            .with_context(|| format!("failed to read response body for {asset_name}"))?;

        std::fs::write(&output_path, &data)
            .with_context(|| format!("failed to write {}", output_path.display()))?;

        println!("done");
        println!("{}", output_path.display());
    }

    Ok(())
}

/// Simple glob pattern matching for asset filtering.
///
/// Supports `*` (any characters) and `?` (single character) wildcards.
fn glob_match(pattern: &str, name: &str) -> bool {
    let pattern = pattern.to_lowercase();
    let name = name.to_lowercase();

    // If no wildcards, do exact match
    if !pattern.contains('*') && !pattern.contains('?') {
        return pattern == name;
    }

    // Simple recursive glob matching
    glob_match_impl(&pattern, &name)
}

fn glob_match_impl(pattern: &str, name: &str) -> bool {
    let mut pi = pattern.chars();
    let mut ni = name.chars();

    loop {
        match (pi.next(), ni.next()) {
            (None | Some('*'), None) => return true,
            (Some('*'), Some(_)) => {
                let rest_pattern: String = pi.collect();
                if rest_pattern.is_empty() {
                    return true;
                }
                let remaining: String = ni.collect();
                for i in 0..=remaining.len() {
                    if glob_match_impl(&rest_pattern, &remaining[i..]) {
                        return true;
                    }
                }
                return false;
            }
            (Some('?'), Some(_)) => {}
            (Some(pc), Some(nc)) if pc == nc => {}
            _ => return false,
        }
    }
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

    #[test]
    fn detect_mime_type_common() {
        assert_eq!(detect_mime_type("app.zip"), "application/zip");
        assert_eq!(detect_mime_type("app.tar.gz"), "application/gzip");
        assert_eq!(detect_mime_type("app.dmg"), "application/x-apple-diskimage");
        assert_eq!(
            detect_mime_type("app.deb"),
            "application/vnd.debian.binary-package"
        );
        assert_eq!(detect_mime_type("app.msi"), "application/x-msi");
        assert_eq!(detect_mime_type("app.exe"), "application/x-msdownload");
        assert_eq!(detect_mime_type("app.txt"), "text/plain");
        assert_eq!(detect_mime_type("app.json"), "application/json");
        assert_eq!(detect_mime_type("app.png"), "image/png");
        assert_eq!(detect_mime_type("app.jpg"), "image/jpeg");
        assert_eq!(detect_mime_type("app.pdf"), "application/pdf");
    }

    #[test]
    fn detect_mime_type_unknown() {
        assert_eq!(detect_mime_type("app.xyz"), "application/octet-stream");
        assert_eq!(detect_mime_type("app"), "application/octet-stream");
    }

    #[test]
    fn glob_match_exact() {
        assert!(glob_match("app.zip", "app.zip"));
        assert!(!glob_match("app.zip", "app.tar.gz"));
    }

    #[test]
    fn glob_match_star() {
        assert!(glob_match("*.zip", "app.zip"));
        assert!(glob_match("*.tar.gz", "app.tar.gz"));
        assert!(!glob_match("*.zip", "app.tar.gz"));
        assert!(glob_match("app-*", "app-linux-amd64"));
    }

    #[test]
    fn glob_match_question() {
        assert!(glob_match("app.???", "app.zip"));
        assert!(!glob_match("app.???", "app.tar.gz"));
    }

    #[test]
    fn glob_match_case_insensitive() {
        assert!(glob_match("*.ZIP", "app.zip"));
        assert!(glob_match("App-*", "app-linux"));
    }
}
