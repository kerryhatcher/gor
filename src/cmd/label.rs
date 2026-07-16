//! Implementation of the `gor label` subcommand.
//!
//! Provides label listing functionality.
//! Currently supports `gor label list` for listing repository labels.

#![allow(clippy::print_stdout)]

use crate::cli::LabelCommand;
use crate::client::Client;
use crate::output::print_json;
use crate::repository::{detect_remote, parse_repo_spec};
use anyhow::Context;

/// Run the `gor label` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cmd: LabelCommand) -> anyhow::Result<()> {
    match cmd {
        LabelCommand::List {
            repo,
            search,
            limit,
            json,
            hostname,
        } => list(
            repo.as_deref(),
            search.as_deref(),
            limit,
            json,
            hostname.as_deref(),
        ),
        LabelCommand::Create {
            name,
            color,
            description,
            repo,
            hostname,
        } => create(
            &name,
            color.as_deref(),
            description.as_deref(),
            repo.as_deref(),
            hostname.as_deref(),
        ),
        LabelCommand::Edit {
            name,
            rename,
            color,
            description,
            repo,
            hostname,
        } => edit(
            &name,
            rename.as_deref(),
            color.as_deref(),
            description.as_deref(),
            repo.as_deref(),
            hostname.as_deref(),
        ),
        LabelCommand::Delete {
            name,
            repo,
            yes,
            hostname,
        } => delete(&name, repo.as_deref(), yes, hostname.as_deref()),
        LabelCommand::Clone {
            source,
            repo,
            force,
            hostname,
        } => clone_labels(&source, repo.as_deref(), force, hostname.as_deref()),
    }
}

/// Execute `gor label list`.
///
/// Lists all labels in a repository. Supports filtering by name substring,
/// limiting results, and JSON output with field selection.
///
/// # Errors
///
/// Returns an error if the repository cannot be found or the API request fails.
fn list(
    repo: Option<&str>,
    search: Option<&str>,
    limit: u32,
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
        "/repos/{}/{}/labels?per_page={}",
        spec.owner,
        spec.repo,
        limit.min(100)
    );
    let response = client.get(&path).context("failed to fetch labels")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("repository '{spec}' not found");
    }
    if !status.is_success() {
        anyhow::bail!("failed to list labels for '{spec}': HTTP {status}");
    }

    let mut labels: Vec<serde_json::Value> =
        response.json().context("failed to parse labels response")?;

    // Client-side search filter
    if let Some(query) = search {
        let query_lower = query.to_lowercase();
        labels.retain(|label| {
            label["name"]
                .as_str()
                .is_some_and(|name| name.to_lowercase().contains(&query_lower))
        });
    }

    // Apply limit
    labels.truncate(limit as usize);

    // Handle --json flag
    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&labels, fields_ref);
        return Ok(());
    }

    // Default: print formatted table
    print_label_table(&labels);
    Ok(())
}

/// Print a formatted label list table.
///
/// Columns: NAME, COLOR, DESCRIPTION
fn print_label_table(labels: &[serde_json::Value]) {
    if labels.is_empty() {
        println!("No labels found.");
        return;
    }

    let name_width = 24;
    let color_width = 10;
    let desc_width = 60;

    println!(
        "{:<name_width$}  {:<color_width$}  {:<desc_width$}",
        "NAME", "COLOR", "DESCRIPTION",
    );

    for label in labels {
        let name = label["name"].as_str().unwrap_or("—");
        let color = label["color"].as_str().unwrap_or("—");
        let description = label["description"].as_str().unwrap_or("—");

        let name_truncated = crate::cmd::util::truncate(name, name_width);
        let desc_truncated = crate::cmd::util::truncate(description, desc_width);

        println!(
            "{name_truncated:<name_width$}  #{color:<color_width$}  {desc_truncated:<desc_width$}",
        );
    }
}

/// Execute `gor label create`.
///
/// Creates a new label with the given name, optional color, and description.
///
/// # Errors
///
/// Returns an error if the repository cannot be found or the API request fails.
fn create(
    name: &str,
    color: Option<&str>,
    description: Option<&str>,
    repo: Option<&str>,
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

    let color_value = color.unwrap_or("ededed");

    let mut body_map = serde_json::Map::new();
    body_map.insert(
        "name".to_string(),
        serde_json::Value::String(name.to_string()),
    );
    body_map.insert(
        "color".to_string(),
        serde_json::Value::String(color_value.to_string()),
    );
    if let Some(desc) = description {
        body_map.insert(
            "description".to_string(),
            serde_json::Value::String(desc.to_string()),
        );
    }

    let path = format!("/repos/{}/{}/labels", spec.owner, spec.repo);
    let body_value = serde_json::Value::Object(body_map);
    let response = client
        .post(&path, &body_value)
        .context("failed to create label")?;

    let status = response.status();
    if status == reqwest::StatusCode::UNPROCESSABLE_ENTITY {
        anyhow::bail!("label '{name}' already exists in '{spec}'");
    }
    if !status.is_success() {
        let err_body: serde_json::Value = response.json().unwrap_or_default();
        let msg = err_body["message"].as_str().unwrap_or("creation failed");
        anyhow::bail!("failed to create label '{name}': {msg}");
    }

    let label: serde_json::Value = response.json().context("failed to parse response")?;
    let label_name = label["name"].as_str().unwrap_or(name);
    let label_color = label["color"].as_str().unwrap_or(color_value);
    println!("✓ Created label '{label_name}' (#{label_color})");
    Ok(())
}

/// Execute `gor label edit`.
///
/// Edits an existing label's name, color, or description.
///
/// # Errors
///
/// Returns an error if the label does not exist or the API request fails.
fn edit(
    name: &str,
    rename: Option<&str>,
    color: Option<&str>,
    description: Option<&str>,
    repo: Option<&str>,
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

    let mut body_map = serde_json::Map::new();
    if let Some(new_name) = rename {
        body_map.insert(
            "new_name".to_string(),
            serde_json::Value::String(new_name.to_string()),
        );
    }
    if let Some(c) = color {
        body_map.insert(
            "color".to_string(),
            serde_json::Value::String(c.to_string()),
        );
    }
    if let Some(desc) = description {
        body_map.insert(
            "description".to_string(),
            serde_json::Value::String(desc.to_string()),
        );
    }

    if body_map.is_empty() {
        anyhow::bail!("no changes specified; use --rename, --color, or --description");
    }

    let path = format!(
        "/repos/{}/{}/labels/{}",
        spec.owner,
        spec.repo,
        urlencoding(name)
    );
    let body_value = serde_json::Value::Object(body_map);
    let response = client
        .request("PATCH", &path, &[], Some(serde_json::to_vec(&body_value)?))
        .context("failed to edit label")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("label '{name}' not found in '{spec}'");
    }
    if !status.is_success() {
        let err_body: serde_json::Value = response.json().unwrap_or_default();
        let msg = err_body["message"].as_str().unwrap_or("edit failed");
        anyhow::bail!("failed to edit label '{name}': {msg}");
    }

    let label: serde_json::Value = response.json().context("failed to parse response")?;
    let label_name = label["name"].as_str().unwrap_or(name);
    let label_color = label["color"].as_str().unwrap_or("—");
    let label_desc = label["description"].as_str().unwrap_or("—");
    println!("✓ Updated label '{label_name}' (#{label_color}): {label_desc}");
    Ok(())
}

/// Execute `gor label delete`.
///
/// Deletes a label by name. Prompts for confirmation unless `--yes` is passed.
///
/// # Errors
///
/// Returns an error if the label does not exist or the API request fails.
fn delete(name: &str, repo: Option<&str>, yes: bool, hostname: Option<&str>) -> anyhow::Result<()> {
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
        use std::io::Write;
        print!("Delete label '{name}' from '{spec}'? [y/N] ");
        std::io::stdout().flush().ok();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        let trimmed = input.trim().to_lowercase();
        if trimmed != "y" && trimmed != "yes" {
            println!("Cancelled.");
            return Ok(());
        }
    }

    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let path = format!(
        "/repos/{}/{}/labels/{}",
        spec.owner,
        spec.repo,
        urlencoding(name)
    );
    let response = client
        .request("DELETE", &path, &[], None)
        .context("failed to delete label")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("label '{name}' not found in '{spec}'");
    }
    if status == reqwest::StatusCode::NO_CONTENT {
        println!("✓ Deleted label '{name}'");
        return Ok(());
    }
    if !status.is_success() {
        anyhow::bail!("failed to delete label '{name}': HTTP {status}");
    }

    Ok(())
}

/// Execute `gor label clone`.
///
/// Clones all labels from a source repository to the target repository.
/// Supports `--force` to overwrite existing labels.
///
/// # Errors
///
/// Returns an error if the source or target repository cannot be found,
/// or if any API request fails.
fn clone_labels(
    source: &str,
    repo: Option<&str>,
    force: bool,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let source_spec = parse_repo_spec(source).context("invalid source repository spec")?;

    let target_spec = match repo {
        Some(s) => parse_repo_spec(s).context("invalid repository spec")?,
        None => detect_remote().ok_or_else(|| {
            anyhow::anyhow!(
                "could not detect repository from current directory; specify OWNER/REPO with --repo"
            )
        })?,
    };

    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    // Fetch source labels
    let source_path = format!(
        "/repos/{}/{}/labels?per_page=100",
        source_spec.owner, source_spec.repo
    );
    let resp = client
        .get(&source_path)
        .context("failed to fetch source labels")?;
    let status = resp.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("source repository '{source_spec}' not found");
    }
    if !status.is_success() {
        anyhow::bail!("failed to fetch labels from '{source_spec}': HTTP {status}");
    }
    let source_labels: Vec<serde_json::Value> =
        resp.json().context("failed to parse labels response")?;

    // Fetch existing target labels for conflict detection
    let target_path = format!(
        "/repos/{}/{}/labels?per_page=100",
        target_spec.owner, target_spec.repo
    );
    let existing: Vec<serde_json::Value> = client
        .get(&target_path)
        .ok()
        .and_then(|r| r.json().ok())
        .unwrap_or_default();
    let existing_names: std::collections::HashSet<&str> =
        existing.iter().filter_map(|l| l["name"].as_str()).collect();

    let mut created = 0u32;
    let mut updated = 0u32;
    let mut skipped = 0u32;

    for label in &source_labels {
        let label_name = label["name"].as_str().unwrap_or("");
        let label_color = label["color"].as_str().unwrap_or("ededed");
        let label_desc = label["description"].as_str().unwrap_or("");

        if existing_names.contains(label_name) {
            if force {
                // Update existing label
                let mut body_map = serde_json::Map::new();
                body_map.insert(
                    "color".to_string(),
                    serde_json::Value::String(label_color.to_string()),
                );
                body_map.insert(
                    "description".to_string(),
                    serde_json::Value::String(label_desc.to_string()),
                );
                let edit_path = format!(
                    "/repos/{}/{}/labels/{}",
                    target_spec.owner,
                    target_spec.repo,
                    urlencoding(label_name)
                );
                let body_value = serde_json::Value::Object(body_map);
                if client
                    .request(
                        "PATCH",
                        &edit_path,
                        &[],
                        Some(serde_json::to_vec(&body_value).unwrap_or_default()),
                    )
                    .is_ok()
                {
                    updated += 1;
                } else {
                    skipped += 1;
                }
            } else {
                skipped += 1;
            }
        } else {
            // Create new label
            let mut body_map = serde_json::Map::new();
            body_map.insert(
                "name".to_string(),
                serde_json::Value::String(label_name.to_string()),
            );
            body_map.insert(
                "color".to_string(),
                serde_json::Value::String(label_color.to_string()),
            );
            body_map.insert(
                "description".to_string(),
                serde_json::Value::String(label_desc.to_string()),
            );
            let create_path = format!("/repos/{}/{}/labels", target_spec.owner, target_spec.repo);
            let body_value = serde_json::Value::Object(body_map);
            if client.post(&create_path, &body_value).is_ok() {
                created += 1;
            } else {
                skipped += 1;
            }
        }
    }

    println!(
        "✓ Cloned labels from '{source_spec}' to '{target_spec}': {created} created, {updated} updated, {skipped} skipped"
    );
    Ok(())
}

/// URL-encode a label name for use in API paths.
fn urlencoding(s: &str) -> String {
    s.replace('#', "%23")
        .replace(' ', "%20")
        .replace('?', "%3F")
        .replace('&', "%26")
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn print_label_table_basic() {
        let labels = vec![json!({
            "name": "bug",
            "color": "d73a4a",
            "description": "Something isn't working"
        })];
        print_label_table(&labels);
    }

    #[test]
    fn print_label_table_empty() {
        let labels: Vec<serde_json::Value> = vec![];
        print_label_table(&labels);
    }

    #[test]
    fn print_label_table_multiple() {
        let labels = vec![
            json!({
                "name": "bug",
                "color": "d73a4a",
                "description": "Something isn't working"
            }),
            json!({
                "name": "enhancement",
                "color": "a2eeef",
                "description": "New feature or request"
            }),
            json!({
                "name": "documentation",
                "color": "0075ca",
                "description": "Improvements or additions to documentation"
            }),
        ];
        print_label_table(&labels);
    }

    #[test]
    fn print_label_table_null_fields() {
        let labels = vec![json!({
            "name": null,
            "color": null,
            "description": null
        })];
        print_label_table(&labels);
    }
}
