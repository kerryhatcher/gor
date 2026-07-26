//! Implementation of the `gor search` subcommand.

#![allow(
    clippy::print_stdout,
    clippy::too_many_arguments,
    clippy::format_push_string
)]

use crate::cli::SearchCommand;
use crate::cmd::util::truncate;
use crate::render::{format_date, print_json};
use gor_core::Client;
use gor_core::search;

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

fn client(hostname: Option<&str>) -> anyhow::Result<Client> {
    let host = hostname.unwrap_or("github.com");
    Client::new(host).map_err(|e| anyhow::anyhow!("{e}"))
}

fn search_repos(
    query: &str,
    language: Option<&str>,
    topic: Option<&str>,
    stars: Option<&str>,
    sort: &str,
    order: &str,
    limit: u32,
    json: Option<Vec<String>>,
    web: bool,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let c = client(hostname)?;
    let mut q = query.to_string();
    if let Some(l) = language {
        q.push_str(&format!("+language:{l}"));
    }
    if let Some(t) = topic {
        q.push_str(&format!("+topic:{t}"));
    }
    if let Some(s) = stars {
        q.push_str(&format!("+stars:{s}"));
    }
    if web {
        let url = format!("https://github.com/search?q={}", q.replace('+', "%20"));
        println!("Open {url} in your browser");
        return Ok(());
    }
    let items = search::search_repos(&c, &q, sort, order, limit)?;
    if let Some(fields) = json {
        let fields_ref = if fields.is_empty() {
            None
        } else {
            Some(fields.as_slice())
        };
        print_json(&items, fields_ref);
        return Ok(());
    }
    if items.is_empty() {
        println!("No repositories found.");
        return Ok(());
    }
    println!(
        "{:<40}  {:<10}  {:<10}  DESCRIPTION",
        "NAME", "STARS", "UPDATED"
    );
    for r in &items {
        let name = r["full_name"].as_str().unwrap_or("—");
        let stars = r["stargazers_count"].as_u64().unwrap_or(0);
        let updated = format_date(r["updated_at"].as_str().unwrap_or(""));
        let desc = truncate(r["description"].as_str().unwrap_or("—"), 50);
        println!("{name:<40}  {stars:<10}  {updated:<10}  {desc}");
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
    let c = client(hostname)?;
    let mut q = query.to_string();
    if let Some(l) = language {
        q.push_str(&format!("+language:{l}"));
    }
    if let Some(r) = repo {
        q.push_str(&format!("+repo:{r}"));
    }
    if web {
        let url = format!(
            "https://github.com/search?type=code&q={}",
            q.replace('+', "%20")
        );
        println!("Open {url} in your browser");
        return Ok(());
    }
    let items = search::search_code(&c, &q, limit)?;
    if let Some(fields) = json {
        let fields_ref = if fields.is_empty() {
            None
        } else {
            Some(fields.as_slice())
        };
        print_json(&items, fields_ref);
        return Ok(());
    }
    for item in &items {
        let path = item["path"].as_str().unwrap_or("—");
        let repo_name = item["repository"]["full_name"].as_str().unwrap_or("—");
        println!("{repo_name} {path}");
    }
    Ok(())
}

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
    let c = client(hostname)?;
    let mut q = query.to_string();
    if let Some(t) = r#type {
        q.push_str(&format!("+type:{t}"));
    }
    if let Some(s) = state {
        q.push_str(&format!("+state:{s}"));
    }
    if let Some(l) = labels {
        q.push_str(&format!("+label:{l}"));
    }
    if web {
        let url = format!("https://github.com/issues?q={}", q.replace('+', "%20"));
        println!("Open {url} in your browser");
        return Ok(());
    }
    let items = search::search_issues(&c, &q, limit)?;
    if let Some(fields) = json {
        let fields_ref = if fields.is_empty() {
            None
        } else {
            Some(fields.as_slice())
        };
        print_json(&items, fields_ref);
        return Ok(());
    }
    if items.is_empty() {
        println!("No issues found.");
        return Ok(());
    }
    println!("{:<40}  {:<15}  TITLE", "ISSUE", "STATE");
    for i in &items {
        let repo = i["repository_url"]
            .as_str()
            .and_then(|u| u.rsplit('/').nth(1))
            .unwrap_or("—");
        let num = i["number"].as_u64().unwrap_or(0);
        let state = i["state"].as_str().unwrap_or("—");
        let title = truncate(i["title"].as_str().unwrap_or("—"), 50);
        println!("{repo}/#{num:<8}  {state:<15}  {title}");
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
    let c = client(hostname)?;
    let mut q = query.to_string();
    if let Some(a) = author {
        q.push_str(&format!("+author:{a}"));
    }
    if let Some(r) = repo {
        q.push_str(&format!("+repo:{r}"));
    }
    if web {
        let url = format!(
            "https://github.com/search?type=commits&q={}",
            q.replace('+', "%20")
        );
        println!("Open {url} in your browser");
        return Ok(());
    }
    let items = search::search_commits(&c, &q, limit)?;
    if let Some(fields) = json {
        let fields_ref = if fields.is_empty() {
            None
        } else {
            Some(fields.as_slice())
        };
        print_json(&items, fields_ref);
        return Ok(());
    }
    for item in &items {
        let sha = item["sha"]
            .as_str()
            .unwrap_or("—")
            .chars()
            .take(7)
            .collect::<String>();
        let msg = truncate(item["commit"]["message"].as_str().unwrap_or("—"), 80);
        let author = item["commit"]["author"]["name"].as_str().unwrap_or("—");
        let date = format_date(item["commit"]["author"]["date"].as_str().unwrap_or(""));
        println!("{sha} {author:<20} {date:<10} {msg}");
    }
    Ok(())
}
