//! Implementation of the `gor label` subcommand.
//!
//! Thin CLI handler that resolves CLI arguments, delegates to `gor_core::label`
//! typed operations, and renders the results.

#![allow(clippy::print_stdout)]

use crate::cli::LabelCommand;
use crate::cmd::util::truncate;
use crate::render::print_json;
use gor_core::Client;
use gor_core::label::{self, CloneResult, CreateOptions, Label, ListOptions, UpdateOptions};
use gor_core::repository::{detect_remote, parse_repo_spec};
use std::io::Write;

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

fn resolve_spec(repo: Option<&str>) -> anyhow::Result<gor_core::RepoSplit> {
    match repo {
        Some(s) => Ok(parse_repo_spec(s)?),
        None => detect_remote().ok_or_else(|| {
            anyhow::anyhow!(
                "could not detect repository from current directory; specify OWNER/REPO with --repo"
            )
        }),
    }
}

fn build_client(hostname: Option<&str>) -> anyhow::Result<Client> {
    let host = hostname.unwrap_or("github.com");
    Client::new(host).map_err(|e| anyhow::anyhow!("failed to create HTTP client: {e}"))
}

/// Execute `gor label list`.
fn list(
    repo: Option<&str>,
    search: Option<&str>,
    limit: u32,
    json: Option<Vec<String>>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let spec = resolve_spec(repo)?;
    let client = build_client(hostname)?;

    let opts = ListOptions {
        search: search.map(String::from),
        limit,
    };
    let labels = label::list(&client, &spec, &opts)?;

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        let values: Vec<serde_json::Value> = labels
            .into_iter()
            .map(|l| serde_json::to_value(l).unwrap_or_default())
            .collect();
        print_json(&values, fields_ref);
        return Ok(());
    }

    print_label_table(&labels);
    Ok(())
}

/// Execute `gor label create`.
fn create(
    name: &str,
    color: Option<&str>,
    description: Option<&str>,
    repo: Option<&str>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let spec = resolve_spec(repo)?;
    let client = build_client(hostname)?;

    let opts = CreateOptions {
        color: color.map(String::from),
        description: description.map(String::from),
    };
    let label = label::create(&client, &spec, name, &opts)?;

    println!("✓ Created label '{}' (#{})", label.name, label.color);
    Ok(())
}

/// Execute `gor label edit`.
fn edit(
    name: &str,
    rename: Option<&str>,
    color: Option<&str>,
    description: Option<&str>,
    repo: Option<&str>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let spec = resolve_spec(repo)?;
    let client = build_client(hostname)?;

    let opts = UpdateOptions {
        new_name: rename.map(String::from),
        color: color.map(String::from),
        description: description.map(String::from),
    };
    let label = label::update(&client, &spec, name, &opts)?;

    let label_desc = label.description.as_deref().unwrap_or("—");
    println!(
        "✓ Updated label '{}' (#{}): {}",
        label.name, label.color, label_desc
    );
    Ok(())
}

/// Execute `gor label delete`.
fn delete(name: &str, repo: Option<&str>, yes: bool, hostname: Option<&str>) -> anyhow::Result<()> {
    let spec = resolve_spec(repo)?;

    if !yes {
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

    let client = build_client(hostname)?;
    label::delete(&client, &spec, name)?;
    println!("✓ Deleted label '{name}'");
    Ok(())
}

/// Execute `gor label clone`.
fn clone_labels(
    source: &str,
    repo: Option<&str>,
    force: bool,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let source_spec = parse_repo_spec(source)?;
    let target_spec = resolve_spec(repo)?;
    let client = build_client(hostname)?;

    let CloneResult {
        created,
        updated,
        skipped,
    } = label::clone_from(&client, &source_spec, &target_spec, force)?;

    println!(
        "✓ Cloned labels from '{source_spec}' to '{target_spec}': {created} created, {updated} updated, {skipped} skipped"
    );
    Ok(())
}

/// Print a formatted label list table.
fn print_label_table(labels: &[Label]) {
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
        let name_truncated = truncate(&label.name, name_width);
        let desc_truncated = truncate(label.description.as_deref().unwrap_or("—"), desc_width);

        println!(
            "{name_truncated:<name_width$}  #{color:<color_width$}  {desc_truncated:<desc_width$}",
            color = label.color,
        );
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use serde_json::json;

    fn make_label(name: &str, color: &str, description: Option<&str>) -> Label {
        serde_json::from_value(json!({
            "name": name,
            "color": color,
            "description": description,
        }))
        .expect("valid label")
    }

    #[test]
    fn print_label_table_basic() {
        let labels = vec![make_label("bug", "d73a4a", Some("Something isn't working"))];
        print_label_table(&labels);
    }

    #[test]
    fn print_label_table_empty() {
        let labels: Vec<Label> = vec![];
        print_label_table(&labels);
    }

    #[test]
    fn print_label_table_multiple() {
        let labels = vec![
            make_label("bug", "d73a4a", Some("Something isn't working")),
            make_label("enhancement", "a2eeef", Some("New feature or request")),
            make_label(
                "documentation",
                "0075ca",
                Some("Improvements or additions to documentation"),
            ),
        ];
        print_label_table(&labels);
    }

    #[test]
    fn print_label_table_null_fields() {
        let labels: Vec<Label> = vec![make_label("nonesuch", "ffffff", None)];
        print_label_table(&labels);
    }
}
