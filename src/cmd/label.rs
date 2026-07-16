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
