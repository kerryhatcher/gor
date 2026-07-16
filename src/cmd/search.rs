//! Implementation of the `gor search` subcommand.
//!
//! Provides search functionality for repositories, code, issues, and commits.

#![allow(clippy::print_stdout)]

use crate::cli::SearchCommand;
use crate::client::Client;
use crate::output::{format_count, format_date, print_json};
use anyhow::Context;
use std::fmt::Write as FmtWrite;

/// Run the `gor search` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cmd: SearchCommand) -> anyhow::Result<()> {
    match cmd {
        SearchCommand::Repos {
            query,
            language,
            topic,
            stars,
            sort,
            order,
            limit,
            json,
            web,
            hostname,
        } => search_repos(
            &query.join(" "),
            language.as_deref(),
            topic.as_deref(),
            stars.as_deref(),
            &sort,
            &order,
            limit,
            json,
            web,
            hostname.as_deref(),
        ),
        SearchCommand::Code {
            query,
            language,
            repo,
            limit,
            json,
            web,
            hostname,
        } => search_code(
            &query.join(" "),
            language.as_deref(),
            repo.as_deref(),
            limit,
            json,
            web,
            hostname.as_deref(),
        ),
        SearchCommand::Issues {
            query,
            r#type,
            state,
            labels,
            limit,
            json,
            web,
            hostname,
        } => search_issues(
            &query.join(" "),
            r#type.as_deref(),
            state.as_deref(),
            labels.as_deref(),
            limit,
            json,
            web,
            hostname.as_deref(),
        ),
        SearchCommand::Commits {
            query,
            author,
            repo,
            limit,
            json,
            web,
            hostname,
        } => search_commits(
            &query.join(" "),
            author.as_deref(),
            repo.as_deref(),
            limit,
            json,
            web,
            hostname.as_deref(),
        ),
    }
}

fn build_query(base: &str, qualifiers: &[(&str, &str)]) -> String {
    let mut q = base.to_string();
    for (key, value) in qualifiers {
        let _ = write!(q, " {key}:{value}");
    }
    q
}

#[allow(clippy::too_many_arguments)]
fn search_repos(
    query: &str,
    language: Option<&str>,
    _topic: Option<&str>,
    stars: Option<&str>,
    sort: &str,
    order: &str,
    limit: u32,
    json: Option<Vec<String>>,
    web: bool,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");

    if web {
        let encoded = urlencoding(query);
        let web_url = format!("https://{host}/search?q={encoded}&type=repositories");
        open_in_browser(&web_url);
        return Ok(());
    }

    let client = Client::new(host).context("failed to create HTTP client")?;

    let mut qualifiers: Vec<(&str, &str)> = Vec::new();
    if let Some(l) = language {
        qualifiers.push(("language", l));
    }
    if let Some(s) = stars {
        qualifiers.push(("stars", s));
    }
    let q = build_query(query, &qualifiers);

    let path = format!(
        "/search/repositories?q={}&sort={sort}&order={order}&per_page={}",
        urlencoding(&q),
        limit.min(100)
    );
    let response = client.get(&path).context("failed to search repositories")?;
    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("search failed: HTTP {status}");
    }

    let data: serde_json::Value = response.json().context("failed to parse search response")?;
    let items: Vec<serde_json::Value> = data["items"].as_array().cloned().unwrap_or_default();

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&items, fields_ref);
        return Ok(());
    }

    if items.is_empty() {
        println!("No repositories found.");
        return Ok(());
    }

    println!(
        "{:<40}  {:<50}  {:<8}  {:<12}  {:<16}",
        "NAME", "DESCRIPTION", "STARS", "LANGUAGE", "UPDATED"
    );
    for item in &items {
        let full_name = item["full_name"].as_str().unwrap_or("—");
        let desc = item["description"].as_str().unwrap_or("—");
        let stars_count = item["stargazers_count"].as_u64().unwrap_or(0);
        let lang = item["language"].as_str().unwrap_or("—");
        let updated = item["updated_at"]
            .as_str()
            .map_or_else(|| "—".to_string(), format_date);

        let name_truncated = crate::cmd::util::truncate(full_name, 40);
        let desc_truncated = crate::cmd::util::truncate(desc, 50);

        println!(
            "{name_truncated:<40}  {desc_truncated:<50}  {:<8}  {lang:<12}  {updated:<16}",
            format_count(stars_count)
        );
    }
    Ok(())
}

fn search_code(
    query: &str,
    language: Option<&str>,
    repo: Option<&str>,
    limit: u32,
    json: Option<Vec<String>>,
    web: bool,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");

    if web {
        let encoded = urlencoding(query);
        let web_url = format!("https://{host}/search?q={encoded}&type=code");
        open_in_browser(&web_url);
        return Ok(());
    }

    let client = Client::new(host).context("failed to create HTTP client")?;

    let mut qualifiers: Vec<(&str, &str)> = Vec::new();
    if let Some(l) = language {
        qualifiers.push(("language", l));
    }
    if let Some(r) = repo {
        qualifiers.push(("repo", r));
    }
    let q = build_query(query, &qualifiers);

    let path = format!(
        "/search/code?q={}&per_page={}",
        urlencoding(&q),
        limit.min(100)
    );
    let response = client.get(&path).context("failed to search code")?;
    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("search failed: HTTP {status}");
    }

    let data: serde_json::Value = response.json().context("failed to parse search response")?;
    let items: Vec<serde_json::Value> = data["items"].as_array().cloned().unwrap_or_default();

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&items, fields_ref);
        return Ok(());
    }

    if items.is_empty() {
        println!("No code results found.");
        return Ok(());
    }

    for item in &items {
        let repo_name = item["repository"]["full_name"].as_str().unwrap_or("—");
        let path = item["path"].as_str().unwrap_or("—");
        let html_url = item["html_url"].as_str().unwrap_or("—");
        println!("{repo_name}/{path}");
        println!("  {html_url}");
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn search_issues(
    query: &str,
    r#type: Option<&str>,
    state: Option<&str>,
    labels: Option<&str>,
    limit: u32,
    json: Option<Vec<String>>,
    web: bool,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");

    if web {
        let encoded = urlencoding(query);
        let web_url = format!("https://{host}/search?q={encoded}&type=issues");
        open_in_browser(&web_url);
        return Ok(());
    }

    let client = Client::new(host).context("failed to create HTTP client")?;

    let mut qualifiers: Vec<(&str, &str)> = Vec::new();
    if let Some(t) = r#type {
        qualifiers.push(("type", t));
    }
    if let Some(s) = state {
        qualifiers.push(("state", s));
    }
    if let Some(l) = labels {
        qualifiers.push(("label", l));
    }
    let q = build_query(query, &qualifiers);

    let path = format!(
        "/search/issues?q={}&per_page={}",
        urlencoding(&q),
        limit.min(100)
    );
    let response = client.get(&path).context("failed to search issues")?;
    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("search failed: HTTP {status}");
    }

    let data: serde_json::Value = response.json().context("failed to parse search response")?;
    let items: Vec<serde_json::Value> = data["items"].as_array().cloned().unwrap_or_default();

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&items, fields_ref);
        return Ok(());
    }

    if items.is_empty() {
        println!("No issues found.");
        return Ok(());
    }

    for item in &items {
        let number = item["number"].as_u64().unwrap_or(0);
        let title = item["title"].as_str().unwrap_or("—");
        let state_str = item["state"].as_str().unwrap_or("—");
        let html_url = item["html_url"].as_str().unwrap_or("—");
        println!("#{number} [{state_str}] {title}");
        println!("  {html_url}");
    }
    Ok(())
}

fn search_commits(
    query: &str,
    author: Option<&str>,
    repo: Option<&str>,
    limit: u32,
    json: Option<Vec<String>>,
    web: bool,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");

    if web {
        let encoded = urlencoding(query);
        let web_url = format!("https://{host}/search?q={encoded}&type=commits");
        open_in_browser(&web_url);
        return Ok(());
    }

    let client = Client::new(host).context("failed to create HTTP client")?;

    let mut qualifiers: Vec<(&str, &str)> = Vec::new();
    if let Some(a) = author {
        qualifiers.push(("author", a));
    }
    if let Some(r) = repo {
        qualifiers.push(("repo", r));
    }
    let q = build_query(query, &qualifiers);

    let path = format!(
        "/search/commits?q={}&per_page={}",
        urlencoding(&q),
        limit.min(100)
    );
    let response = client.get(&path).context("failed to search commits")?;
    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("search failed: HTTP {status}");
    }

    let data: serde_json::Value = response.json().context("failed to parse search response")?;
    let items: Vec<serde_json::Value> = data["items"].as_array().cloned().unwrap_or_default();

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&items, fields_ref);
        return Ok(());
    }

    if items.is_empty() {
        println!("No commits found.");
        return Ok(());
    }

    for item in &items {
        let sha = item["sha"].as_str().unwrap_or("—");
        let short_sha = &sha[..sha.len().min(7)];
        let message = item["commit"]["message"].as_str().unwrap_or("—");
        let first_line = message.lines().next().unwrap_or("—");
        let author_login = item["author"]["login"].as_str().unwrap_or("—");
        let html_url = item["html_url"].as_str().unwrap_or("—");
        println!("{short_sha} {first_line}");
        println!("  {author_login} — {html_url}");
    }
    Ok(())
}

fn urlencoding(s: &str) -> String {
    s.replace(' ', "+")
        .replace('#', "%23")
        .replace('&', "%26")
        .replace('?', "%3F")
}

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
