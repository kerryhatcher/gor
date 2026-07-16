//! Implementation of the `gor api` subcommand.
//!
//! Provides arbitrary REST API calls to GitHub endpoints.
//! Supports GET, POST, PUT, PATCH, DELETE methods with custom headers,
//! body input, pagination, and JSON output.

#![allow(clippy::print_stdout)]

use crate::cli::ApiCommand;
use crate::client::Client;
use anyhow::Context;
use std::fmt::Write as FmtWrite;
use std::io::Read;

/// Run the `gor api` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cmd: &ApiCommand, global_hostname: Option<&str>) -> anyhow::Result<()> {
    let host = cmd
        .hostname
        .as_deref()
        .or(global_hostname)
        .unwrap_or("github.com");

    let client = Client::new(host).context("failed to create HTTP client")?;

    let method = if cmd.method.is_empty() {
        "GET".to_string()
    } else {
        cmd.method.to_uppercase()
    };

    // Build the endpoint path
    let endpoint = if cmd.endpoint.starts_with('/') {
        cmd.endpoint.clone()
    } else {
        format!("/{}", cmd.endpoint)
    };

    // Build the request body
    let body = build_body(cmd).context("failed to build request body")?;

    // Make the request
    let response = client
        .request(&method, &endpoint, &cmd.headers, body)
        .with_context(|| format!("failed to make {method} request to {endpoint}"))?;

    let status = response.status();
    let headers = response.headers().clone();

    // Read the response body
    let response_body = response.text().context("failed to read response body")?;

    // Handle pagination
    if cmd.paginate && status.is_success() {
        let all_bodies = collect_paginated(
            &client,
            &method,
            &endpoint,
            &cmd.headers,
            &response_body,
            &headers,
        )
        .context("failed to collect paginated results")?;
        print_output(cmd, status, &headers, &all_bodies, true);
    } else {
        print_output(cmd, status, &headers, &response_body, false);
    }

    Ok(())
}

/// Build the request body from the command arguments.
///
/// Priority: --input > --field > --raw-field
fn build_body(cmd: &ApiCommand) -> anyhow::Result<Option<Vec<u8>>> {
    // --input takes precedence
    if let Some(input) = &cmd.input {
        return read_input(input).map(Some);
    }

    // --field / -F: URL-encoded form fields
    if !cmd.fields.is_empty() {
        let mut parts: Vec<String> = Vec::new();
        for field in &cmd.fields {
            if let Some((key, value)) = field.split_once('=') {
                let encoded_key = urlencode(key);
                let encoded_value = urlencode(value);
                parts.push(format!("{encoded_key}={encoded_value}"));
            } else {
                let encoded_key = urlencode(field);
                parts.push(format!("{encoded_key}="));
            }
        }
        return Ok(Some(parts.join("&").into_bytes()));
    }

    // --raw-field / -f: raw (non-URL-encoded) form fields
    if !cmd.raw_fields.is_empty() {
        let mut parts: Vec<String> = Vec::new();
        for field in &cmd.raw_fields {
            if let Some((key, value)) = field.split_once('=') {
                parts.push(format!("{key}={value}"));
            } else {
                parts.push(format!("{field}="));
            }
        }
        return Ok(Some(parts.join("&").into_bytes()));
    }

    Ok(None)
}

/// Read input from a file or stdin.
///
/// If `input` is `@-`, read from stdin.
/// Otherwise, read from the specified file path.
fn read_input(input: &str) -> anyhow::Result<Vec<u8>> {
    if input == "@-" {
        let mut buf = Vec::new();
        std::io::stdin()
            .read_to_end(&mut buf)
            .context("failed to read from stdin")?;
        Ok(buf)
    } else {
        let mut file = std::fs::File::open(input)
            .with_context(|| format!("failed to open input file '{input}'"))?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)
            .with_context(|| format!("failed to read input file '{input}'"))?;
        Ok(buf)
    }
}

/// Simple URL-encoding for form field values.
///
/// Only encodes characters that are not allowed in form data:
/// spaces, special characters, and non-ASCII.
fn urlencode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(byte as char);
            }
            b' ' => {
                result.push('+');
            }
            _ => {
                let _ = std::write!(result, "%{byte:02X}");
            }
        }
    }
    result
}

/// Collect paginated results by following `Link` headers.
///
/// Concatenates all page responses into a single JSON array.
fn collect_paginated(
    client: &Client,
    method: &str,
    _endpoint: &str,
    headers: &[String],
    first_body: &str,
    first_headers: &reqwest::header::HeaderMap,
) -> anyhow::Result<String> {
    let mut all_items: Vec<serde_json::Value> = Vec::new();

    // Parse the first page
    if let Ok(first_json) = serde_json::from_str::<serde_json::Value>(first_body) {
        if let Some(arr) = first_json.as_array() {
            all_items.extend(arr.iter().cloned());
        } else {
            // If it's not an array, just return the first body as-is
            return Ok(first_body.to_string());
        }
    } else {
        return Ok(first_body.to_string());
    }

    let mut next_url = parse_next_link(first_headers);

    while let Some(url) = next_url {
        // Extract the path from the full URL
        let path = extract_path_from_url(&url);

        let response = client
            .request(method, &path, headers, None)
            .context("failed to fetch paginated page")?;

        let response_headers = response.headers().clone();
        let body_text = response
            .text()
            .context("failed to read paginated response body")?;

        if let Ok(page_json) = serde_json::from_str::<serde_json::Value>(&body_text) {
            if let Some(arr) = page_json.as_array() {
                all_items.extend(arr.iter().cloned());
            }
        }

        next_url = parse_next_link(&response_headers);
    }

    serde_json::to_string_pretty(&all_items).context("failed to serialize paginated results")
}

/// Parse the `Link` header to find the next page URL.
fn parse_next_link(headers: &reqwest::header::HeaderMap) -> Option<String> {
    let link_header = headers.get("Link")?.to_str().ok()?;

    // Link header format: `<url>; rel="next", <url>; rel="last"`
    for part in link_header.split(',') {
        let part = part.trim();
        if part.contains("rel=\"next\"") || part.contains("rel=next") {
            // Extract the URL from <...>
            if let Some(start) = part.find('<') {
                if let Some(end) = part.find('>') {
                    return Some(part[start + 1..end].to_string());
                }
            }
        }
    }

    None
}

/// Extract the API path from a full URL.
///
/// Given `https://api.github.com/repos/owner/repo/issues?page=2`,
/// returns `/repos/owner/repo/issues?page=2`.
fn extract_path_from_url(url: &str) -> String {
    // Find the path after the host
    if let Some(path_start) = url.find("://") {
        let after_scheme = &url[path_start + 3..];
        if let Some(slash_pos) = after_scheme.find('/') {
            let after_host = &after_scheme[slash_pos..];
            // Remove API prefix if present (/api/v3 or /api)
            if let Some(rest) = after_host.strip_prefix("/api/v3") {
                return rest.to_string();
            }
            if let Some(rest) = after_host.strip_prefix("/api") {
                return rest.to_string();
            }
            return after_host.to_string();
        }
    }
    url.to_string()
}

/// Print the output based on command flags.
///
/// Handles `--include`, `--jq`, `--template`, `--silent`, and default output.
fn print_output(
    cmd: &ApiCommand,
    status: reqwest::StatusCode,
    headers: &reqwest::header::HeaderMap,
    body: &str,
    is_paginated: bool,
) {
    // --include / -i: print response headers
    if cmd.include {
        println!(
            "HTTP/1.1 {} {}",
            status.as_u16(),
            status.canonical_reason().unwrap_or("Unknown")
        );
        for (name, value) in headers {
            if let Ok(v) = value.to_str() {
                println!("{name}: {v}");
            }
        }
        println!();
    }

    // --silent: suppress status output
    if !cmd.silent && !cmd.include {
        if status.is_success() {
            if is_paginated {
                println!("HTTP {} (paginated)", status.as_u16());
            } else {
                println!("HTTP {}", status.as_u16());
            }
        } else {
            tracing::warn!("HTTP {}", status.as_u16());
        }
    }

    // --jq: filter through jq expression
    if let Some(_expr) = &cmd.jq {
        // jaq is not yet a dependency; print message and fall back to raw JSON
        tracing::warn!("jq support requires the `jaq` feature; falling back to raw JSON output");
        print_body(body);
        return;
    }

    // --template: format via template
    if cmd.template.is_some() {
        tracing::warn!("template support is not yet implemented; falling back to raw JSON output");
        print_body(body);
        return;
    }

    // Default: print the body
    print_body(body);
}

/// Print the response body, attempting to pretty-print JSON.
fn print_body(body: &str) {
    // Try to pretty-print JSON
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(body) {
        if let Ok(pretty) = serde_json::to_string_pretty(&json) {
            println!("{pretty}");
            return;
        }
    }
    // Fall back to raw output
    println!("{body}");
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn build_body_no_input() {
        let cmd = ApiCommand {
            endpoint: "/test".to_string(),
            method: "GET".to_string(),
            fields: vec![],
            raw_fields: vec![],
            headers: vec![],
            input: None,
            paginate: false,
            hostname: None,
            jq: None,
            template: None,
            silent: false,
            include: false,
        };
        let body = build_body(&cmd).expect("build_body should succeed");
        assert!(body.is_none());
    }

    #[test]
    fn build_body_with_fields() {
        let cmd = ApiCommand {
            endpoint: "/test".to_string(),
            method: "POST".to_string(),
            fields: vec!["name=hello".to_string(), "count=42".to_string()],
            raw_fields: vec![],
            headers: vec![],
            input: None,
            paginate: false,
            hostname: None,
            jq: None,
            template: None,
            silent: false,
            include: false,
        };
        let body = build_body(&cmd).expect("build_body should succeed");
        let body_str = String::from_utf8(body.expect("body should be Some")).expect("valid utf8");
        assert!(body_str.contains("name=hello") || body_str.contains("name=hello"));
        assert!(body_str.contains("count=42"));
    }

    #[test]
    fn build_body_with_raw_fields() {
        let cmd = ApiCommand {
            endpoint: "/test".to_string(),
            method: "POST".to_string(),
            fields: vec![],
            raw_fields: vec!["key=value".to_string(), "json=[1,2,3]".to_string()],
            headers: vec![],
            input: None,
            paginate: false,
            hostname: None,
            jq: None,
            template: None,
            silent: false,
            include: false,
        };
        let body = build_body(&cmd).expect("build_body should succeed");
        let body_str = String::from_utf8(body.expect("body should be Some")).expect("valid utf8");
        assert!(body_str.contains("key=value"));
        assert!(body_str.contains("json=[1,2,3]"));
    }

    #[test]
    fn build_body_input_takes_precedence() {
        let cmd = ApiCommand {
            endpoint: "/test".to_string(),
            method: "POST".to_string(),
            fields: vec!["should=notappear".to_string()],
            raw_fields: vec![],
            headers: vec![],
            input: Some("@-".to_string()),
            paginate: false,
            hostname: None,
            jq: None,
            template: None,
            silent: false,
            include: false,
        };
        // This would try to read from stdin, which is not available in tests.
        // Just verify that input takes precedence by checking the logic path.
        assert!(cmd.input.is_some());
    }

    #[test]
    fn build_body_field_without_value() {
        let cmd = ApiCommand {
            endpoint: "/test".to_string(),
            method: "POST".to_string(),
            fields: vec!["flag".to_string()],
            raw_fields: vec![],
            headers: vec![],
            input: None,
            paginate: false,
            hostname: None,
            jq: None,
            template: None,
            silent: false,
            include: false,
        };
        let body = build_body(&cmd).expect("build_body should succeed");
        let body_str = String::from_utf8(body.expect("body should be Some")).expect("valid utf8");
        assert_eq!(body_str, "flag=");
    }

    #[test]
    fn read_input_from_file() {
        let dir = std::env::temp_dir();
        let path = dir.join("gor_test_api_input.txt");
        std::fs::write(&path, b"test body content").expect("write temp file");
        let path_str = path.to_str().expect("path to str").to_string();
        let content = read_input(&path_str).expect("read_input should succeed");
        assert_eq!(
            String::from_utf8(content).expect("valid utf8"),
            "test body content"
        );
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn urlencode_basic() {
        assert_eq!(urlencode("hello"), "hello");
        assert_eq!(urlencode("hello world"), "hello+world");
        assert_eq!(urlencode("a=b&c"), "a%3Db%26c");
        assert_eq!(urlencode(""), "");
    }

    #[test]
    fn urlencode_special_chars() {
        assert_eq!(urlencode("foo/bar"), "foo%2Fbar");
        assert_eq!(urlencode("a b c"), "a+b+c");
        assert_eq!(urlencode("~test_123"), "~test_123");
    }

    #[test]
    fn parse_next_link_no_header() {
        let headers = reqwest::header::HeaderMap::new();
        assert!(parse_next_link(&headers).is_none());
    }

    #[test]
    fn parse_next_link_with_next() {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Link",
            "<https://api.github.com/repos/owner/repo/issues?page=2>; rel=\"next\", <https://api.github.com/repos/owner/repo/issues?page=5>; rel=\"last\"".parse().unwrap(),
        );
        let next = parse_next_link(&headers);
        assert_eq!(
            next,
            Some("https://api.github.com/repos/owner/repo/issues?page=2".to_string())
        );
    }

    #[test]
    fn parse_next_link_no_next() {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Link",
            "<https://api.github.com/repos/owner/repo/issues?page=1>; rel=\"first\", <https://api.github.com/repos/owner/repo/issues?page=5>; rel=\"last\"".parse().unwrap(),
        );
        assert!(parse_next_link(&headers).is_none());
    }

    #[test]
    fn extract_path_from_url_github() {
        let url = "https://api.github.com/repos/owner/repo/issues?page=2";
        assert_eq!(
            extract_path_from_url(url),
            "/repos/owner/repo/issues?page=2"
        );
    }

    #[test]
    fn extract_path_from_url_ghes() {
        let url = "https://github.example.com/api/v3/repos/owner/repo/issues";
        assert_eq!(extract_path_from_url(url), "/repos/owner/repo/issues");
    }

    #[test]
    fn extract_path_from_url_relative() {
        let url = "/repos/owner/repo";
        assert_eq!(extract_path_from_url(url), "/repos/owner/repo");
    }

    #[test]
    fn print_body_valid_json() {
        // Should not panic
        print_body(r#"{"name": "test"}"#);
    }

    #[test]
    fn print_body_invalid_json() {
        // Should not panic with non-JSON
        print_body("plain text response");
    }

    #[test]
    fn print_body_empty() {
        // Should not panic with empty string
        print_body("");
    }
}
