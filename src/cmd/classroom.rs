//! Implementation of the `gor classroom` subcommand.
//!
//! Provides classroom listing and assignment viewing for GitHub Classroom.

#![allow(clippy::print_stdout)]

use crate::cli::ClassroomCommand;
use crate::client::Client;
use crate::output::print_json;
use anyhow::Context;

/// Run the `gor classroom` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cmd: ClassroomCommand) -> anyhow::Result<()> {
    match cmd {
        ClassroomCommand::List { json, hostname } => list(json, hostname.as_deref()),
        ClassroomCommand::Assignments { id, json, hostname } => {
            assignments(id, json, hostname.as_deref())
        }
    }
}

/// List classrooms for the authenticated user.
///
/// # Errors
///
/// Returns an error if the API request fails.
fn list(json: Option<Vec<String>>, hostname: Option<&str>) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let response = client
        .get("/classrooms")
        .context("failed to fetch classrooms")?;

    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("failed to list classrooms: HTTP {status}");
    }

    let classrooms: Vec<serde_json::Value> = response.json().context("failed to parse response")?;

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&classrooms, fields_ref);
        return Ok(());
    }

    if classrooms.is_empty() {
        println!("No classrooms found.");
        return Ok(());
    }

    println!("{:<8}  {:<30}  ORGANIZATION", "ID", "NAME");
    for c in &classrooms {
        let id = c["id"].as_u64().unwrap_or(0);
        let name = c["name"].as_str().unwrap_or("—");
        let org = c["organization"]["login"].as_str().unwrap_or("—");
        let name_truncated = crate::cmd::util::truncate(name, 30);
        println!("{id:<8}  {name_truncated:<30}  {org}");
    }

    Ok(())
}

/// List assignments for a classroom.
///
/// # Errors
///
/// Returns an error if the API request fails.
fn assignments(id: u64, json: Option<Vec<String>>, hostname: Option<&str>) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let path = format!("/classrooms/{id}/assignments");
    let response = client.get(&path).context("failed to fetch assignments")?;

    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("failed to list assignments: HTTP {status}");
    }

    let assignments: Vec<serde_json::Value> =
        response.json().context("failed to parse response")?;

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&assignments, fields_ref);
        return Ok(());
    }

    if assignments.is_empty() {
        println!("No assignments found for classroom {id}.");
        return Ok(());
    }

    println!("{:<8}  {:<30}  {:<20}  INVITED", "ID", "TITLE", "DEADLINE");
    for a in &assignments {
        let aid = a["id"].as_u64().unwrap_or(0);
        let title = a["title"].as_str().unwrap_or("—");
        let deadline = a["deadline"].as_str().unwrap_or("—");
        let invited = a["invitations_count"].as_u64().unwrap_or(0);
        let title_truncated = crate::cmd::util::truncate(title, 30);
        println!("{aid:<8}  {title_truncated:<30}  {deadline:<20}  {invited}");
    }

    Ok(())
}
