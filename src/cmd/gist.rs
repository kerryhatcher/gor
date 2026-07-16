//! Implementation of the `gor gist` subcommand.
//!
//! Provides gist listing and creation commands.

#![allow(clippy::print_stdout)]

use crate::cli::GistCommand;
use crate::client::Client;
use crate::output::print_json;
use anyhow::Context;
use std::fs;

/// Run the `gor gist` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cmd: GistCommand) -> anyhow::Result<()> {
    match cmd {
        GistCommand::List {
            public,
            secret,
            user,
            limit,
            json,
            hostname,
        } => list(
            public,
            secret,
            user.as_deref(),
            limit,
            json,
            hostname.as_deref(),
        ),
        GistCommand::Create {
            files,
            desc,
            public,
            filename,
            web,
            hostname,
        } => create(
            &files,
            desc.as_deref(),
            public,
            filename.as_deref(),
            web,
            hostname.as_deref(),
        ),
        GistCommand::View {
            gist_id,
            raw,
            filename,
            web,
            json,
            hostname,
        } => view(
            &gist_id,
            raw,
            filename.as_deref(),
            web,
            json,
            hostname.as_deref(),
        ),
    }
}

/// Execute `gor gist list`.
///
/// Lists gists for the authenticated user or a specific user.
///
/// # Errors
///
/// Returns an error if the API request fails.
fn list(
    public: bool,
    secret: bool,
    user: Option<&str>,
    limit: u32,
    json: Option<Vec<String>>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let path = user.map_or_else(
        || format!("/gists?per_page={}", limit.min(100)),
        |u| format!("/users/{u}/gists?per_page={}", limit.min(100)),
    );

    let response = client.get(&path).context("failed to fetch gists")?;

    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("failed to list gists: HTTP {status}");
    }

    let mut gists: Vec<serde_json::Value> =
        response.json().context("failed to parse gists response")?;

    // Filter by visibility
    gists.retain(|g| {
        let is_public = g["public"].as_bool().unwrap_or(false);
        if public && !secret {
            is_public
        } else if secret && !public {
            !is_public
        } else {
            true
        }
    });

    gists.truncate(limit as usize);

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&gists, fields_ref);
        return Ok(());
    }

    print_gist_table(&gists);
    Ok(())
}

/// Execute `gor gist create`.
///
/// Creates a new gist from one or more files.
///
/// # Errors
///
/// Returns an error if the file cannot be read or the API request fails.
fn create(
    files: &[String],
    desc: Option<&str>,
    public: bool,
    filename: Option<&str>,
    web: bool,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    if files.is_empty() {
        anyhow::bail!("no files specified for gist creation");
    }

    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let mut files_map = serde_json::Map::new();
    for (i, file) in files.iter().enumerate() {
        let content =
            fs::read_to_string(file).with_context(|| format!("failed to read file: {file}"))?;
        let gist_filename = if files.len() == 1 {
            filename.unwrap_or(file).to_string()
        } else {
            file.clone()
        };
        let file_entry = serde_json::json!({ "content": content });
        files_map.insert(gist_filename, file_entry);
        if files.len() > 1 && i == 0 {
            // Only the first file gets the custom filename
        }
    }

    let mut body_map = serde_json::Map::new();
    body_map.insert("public".to_string(), serde_json::Value::Bool(public));
    body_map.insert("files".to_string(), serde_json::Value::Object(files_map));
    if let Some(d) = desc {
        body_map.insert(
            "description".to_string(),
            serde_json::Value::String(d.to_string()),
        );
    }

    let body_value = serde_json::Value::Object(body_map);
    let response = client
        .post("/gists", &body_value)
        .context("failed to create gist")?;

    let status = response.status();
    if !status.is_success() {
        let err_body: serde_json::Value = response.json().unwrap_or_default();
        let msg = err_body["message"].as_str().unwrap_or("creation failed");
        anyhow::bail!("failed to create gist: {msg}");
    }

    let gist: serde_json::Value = response.json().context("failed to parse gist response")?;
    let gist_url = gist["html_url"].as_str().unwrap_or("");

    if web && !gist_url.is_empty() {
        open_in_browser(gist_url);
    }

    println!("{gist_url}");
    Ok(())
}

/// Execute `gor gist view`.
///
/// Views a gist's content and metadata.
///
/// # Errors
///
/// Returns an error if the API request fails.
fn view(
    gist_id: &str,
    raw: bool,
    filename: Option<&str>,
    web: bool,
    json: Option<Vec<String>>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let path = format!("/gists/{gist_id}");
    let response = client.get(&path).context("failed to fetch gist")?;

    let status = response.status();
    if !status.is_success() {
        let err_body: serde_json::Value = response.json().unwrap_or_default();
        let msg = err_body["message"].as_str().unwrap_or("view failed");
        anyhow::bail!("failed to view gist: {msg}");
    }

    let gist: serde_json::Value = response.json().context("failed to parse gist response")?;

    // --web / -w: open in browser
    if web {
        if let Some(url) = gist["html_url"].as_str() {
            open_in_browser(url);
            return Ok(());
        }
    }

    // --raw: output raw content of a specific file
    if raw {
        let files = gist["files"]
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("gist has no files"))?;

        let selected = if let Some(name) = filename {
            files
                .get(name)
                .ok_or_else(|| anyhow::anyhow!("file '{name}' not found in gist"))?
        } else {
            files
                .values()
                .next()
                .ok_or_else(|| anyhow::anyhow!("gist has no files"))?
        };

        let content = selected["content"].as_str().unwrap_or("");
        print!("{content}");
        return Ok(());
    }

    // --json: output as JSON
    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&gist, fields_ref);
        return Ok(());
    }

    // Default: print description and files
    let description = gist["description"].as_str().unwrap_or("No description");
    println!("Description: {description}");
    println!("Files:");

    let files = gist["files"]
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("gist has no files"))?;
    for (name, file_info) in files {
        let language = file_info["language"].as_str().unwrap_or("Unknown");
        let content = file_info["content"].as_str().unwrap_or("");
        println!("\n  {name} ({language}):");
        for line in content.lines() {
            println!("    {line}");
        }
    }

    Ok(())
}

/// Print a formatted gist list table.
fn print_gist_table(gists: &[serde_json::Value]) {
    if gists.is_empty() {
        println!("No gists found.");
        return;
    }

    let id_width = 16;
    let desc_width = 40;
    let files_width = 8;
    let date_width = 16;

    println!(
        "{:<id_width$}  {:<desc_width$}  {:>files_width$}  {:>date_width$}",
        "ID", "DESCRIPTION", "FILES", "UPDATED",
    );

    for gist in gists {
        let gist_id = gist["id"].as_str().unwrap_or("—");
        let description = gist["description"].as_str().unwrap_or("—");
        let file_count = gist["files"].as_object().map_or(0, serde_json::Map::len);
        let updated = gist["updated_at"]
            .as_str()
            .map_or_else(|| "—".to_string(), crate::output::format_date);

        let desc_truncated = crate::cmd::util::truncate(description, desc_width);

        println!(
            "{gist_id:<id_width$}  {desc_truncated:<desc_width$}  {file_count:>files_width$}  {updated:>date_width$}",
        );
    }
}

/// Open a URL in the default browser.
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
    fn print_gist_table_basic() {
        let gists = vec![json!({
            "id": "abc123",
            "description": "My first gist",
            "files": { "hello.py": { "filename": "hello.py" } },
            "updated_at": "2024-01-15T10:30:00Z",
            "public": false
        })];
        print_gist_table(&gists);
    }

    #[test]
    fn print_gist_table_empty() {
        let gists: Vec<serde_json::Value> = vec![];
        print_gist_table(&gists);
    }

    #[test]
    fn print_gist_table_multiple() {
        let gists = vec![
            json!({
                "id": "abc123",
                "description": "My first gist",
                "files": { "hello.py": {} },
                "updated_at": "2024-01-15T10:30:00Z",
                "public": true
            }),
            json!({
                "id": "def456",
                "description": null,
                "files": { "a.rs": {}, "b.rs": {}, "c.rs": {} },
                "updated_at": "2024-03-01T00:00:00Z",
                "public": false
            }),
        ];
        print_gist_table(&gists);
    }

    #[test]
    fn open_in_browser_does_not_panic() {
        open_in_browser("https://gist.github.com/abc123");
    }
}
