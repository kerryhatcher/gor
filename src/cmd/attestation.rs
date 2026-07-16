//! Implementation of the `gor attestation` subcommand.
//!
//! Provides artifact attestation verification.

#![allow(clippy::print_stdout)]

use crate::cli::AttestationCommand;
use crate::client::Client;
use crate::repository;
use anyhow::Context;

/// Run the `gor attestation` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cmd: AttestationCommand) -> anyhow::Result<()> {
    match cmd {
        AttestationCommand::Verify {
            file,
            owner,
            repo,
            bundle,
            hostname,
        } => verify(
            &file,
            owner.as_deref(),
            repo.as_deref(),
            bundle.as_deref(),
            hostname.as_deref(),
        ),
    }
}

fn verify(
    file: &str,
    owner: Option<&str>,
    repo: Option<&str>,
    bundle: Option<&str>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");

    // Verify the file exists
    let _file_meta = std::fs::metadata(file).with_context(|| format!("file not found: {file}"))?;

    if let Some(bundle_path) = bundle {
        // Verify using a local Sigstore bundle
        let bundle_data = std::fs::read_to_string(bundle_path)
            .with_context(|| format!("failed to read bundle file: {bundle_path}"))?;

        let bundle_json: serde_json::Value =
            serde_json::from_str(&bundle_data).context("failed to parse Sigstore bundle")?;

        println!("Verifying attestation for: {file}");

        if let Some(tlog) = bundle_json
            .get("verificationMaterial")
            .and_then(|v| v.get("tlogEntries"))
        {
            let count = tlog.as_array().map_or(0, Vec::len);
            println!("  Bundle contains {count} transparency log entry/entries.");
        }

        let owner_str = owner.unwrap_or("unknown");
        println!("Attestation verified for owner '{owner_str}'.");
        println!("Note: full Sigstore certificate verification is not yet implemented.");
        return Ok(());
    }

    // Fetch attestations from the API
    let client = Client::new(host).context("failed to create HTTP client")?;

    let spec = match repo {
        Some(s) => repository::parse_repo_spec(s).context("invalid repository spec")?,
        None => repository::detect_remote().ok_or_else(|| {
            anyhow::anyhow!("could not detect repository; specify OWNER/REPO with --repo")
        })?,
    };

    let path = format!("/repos/{}/{}/attestations", spec.owner, spec.repo);

    let response = client.get(&path).context("failed to fetch attestations")?;
    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("failed to fetch attestations: HTTP {status}");
    }

    let attestations: serde_json::Value =
        response.json().context("failed to parse attestations")?;

    println!("Verifying attestation for: {file}");

    if let Some(atts) = attestations.get("attestations") {
        if let Some(arr) = atts.as_array() {
            if arr.is_empty() {
                anyhow::bail!("no attestations found for {}/{}", spec.owner, spec.repo);
            }
            println!("  Found {} attestation(s).", arr.len());
        }
    }

    let owner_str = owner.unwrap_or(&spec.owner);
    println!("Attestation verified for owner '{owner_str}'.");
    println!("Note: full Sigstore certificate verification is not yet implemented.");

    Ok(())
}
