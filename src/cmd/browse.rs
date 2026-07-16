//! Implementation of the `gor browse` subcommand.
//!
//! Opens a repository, branch, commit, issue, PR, or other resource
//! in the default web browser.

#![allow(clippy::print_stdout, clippy::print_stderr)]

use crate::cli::BrowseCommand;
use crate::repository::{detect_remote, parse_repo_spec};
use anyhow::Context;

/// Run the `gor browse` subcommand.
///
/// # Errors
///
/// Returns an error if the repository cannot be determined.
pub fn run(cmd: BrowseCommand, hostname: Option<&str>) -> anyhow::Result<()> {
    let spec = match cmd.repo.as_deref() {
        Some(s) => parse_repo_spec(s).context("invalid repository spec")?,
        None => detect_remote().ok_or_else(|| {
            anyhow::anyhow!(
                "could not detect repository from current directory; specify OWNER/REPO with --repo"
            )
        })?,
    };

    let host = hostname.unwrap_or("github.com");
    let base_url = format!("https://{host}/{}/{}", spec.owner, spec.repo);

    let url = if let Some(issue) = cmd.issue {
        format!("{base_url}/issues/{issue}")
    } else if let Some(pr) = cmd.pr {
        format!("{base_url}/pull/{pr}")
    } else if let Some(branch) = cmd.branch {
        format!("{base_url}/tree/{branch}")
    } else if let Some(commit) = cmd.commit {
        format!("{base_url}/commit/{commit}")
    } else if cmd.projects {
        format!("{base_url}/projects")
    } else if cmd.wiki {
        format!("{base_url}/wiki")
    } else if cmd.settings {
        format!("{base_url}/settings")
    } else {
        base_url
    };

    open_in_browser(&url);
    println!("Opening {url}");
    Ok(())
}

/// Open a URL in the default browser.
pub fn open_in_browser(url: &str) {
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

    #[test]
    fn open_in_browser_does_not_panic() {
        open_in_browser("https://github.com/octocat/hello-world");
    }
}
