//! HTTP client for the GitHub REST API.
//!
//! Provides a [`Client`] that handles authentication, base URL resolution,
//! and common request patterns. Tokens are resolved from the OS keyring
//! automatically.

use crate::error::GorError;
use crate::host::Host;
use crate::keyring_store;

/// An HTTP client for making authenticated requests to the GitHub API.
///
/// Wraps a [`reqwest::blocking::Client`] and automatically attaches
/// the appropriate `Authorization` header and `Accept` header.
pub struct Client {
    /// The underlying reqwest HTTP client.
    http: reqwest::blocking::Client,
    /// The GitHub host this client is configured for.
    host: Host,
    /// The OAuth token, if available.
    token: Option<String>,
}

impl Client {
    /// Create a new `Client` for the given host.
    ///
    /// Attempts to load a stored token from the OS keyring. If no token
    /// is found, the client is created in an unauthenticated state.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gor::client::Client;
    ///
    /// let client = Client::new("github.com").unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the reqwest client cannot be created.
    pub fn new(hostname: &str) -> Result<Self, GorError> {
        let host = Host::new(hostname);
        let token = keyring_store::get_token(hostname).unwrap_or(None);
        let http = reqwest::blocking::Client::builder()
            .user_agent(concat!("gor/", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(GorError::Http)?;
        Ok(Self { http, host, token })
    }

    /// Create a new `Client` with an explicit token (bypasses keyring).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gor::client::Client;
    ///
    /// let client = Client::with_token("github.com", "gho_abc123").unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the reqwest client cannot be created.
    pub fn with_token(hostname: &str, token: &str) -> Result<Self, GorError> {
        let host = Host::new(hostname);
        let http = reqwest::blocking::Client::builder()
            .user_agent(concat!("gor/", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(GorError::Http)?;
        Ok(Self {
            http,
            host,
            token: Some(token.to_string()),
        })
    }

    /// Returns the host this client is configured for.
    #[must_use]
    pub const fn host(&self) -> &Host {
        &self.host
    }

    /// Returns `true` if the client has an auth token.
    #[must_use]
    pub const fn is_authenticated(&self) -> bool {
        self.token.is_some()
    }

    /// Returns the auth token, if available.
    #[must_use]
    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    /// Make a `GET` request to the given API path.
    ///
    /// The path should start with `/`, e.g. `/user`.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails.
    pub fn get(&self, path: &str) -> Result<reqwest::blocking::Response, GorError> {
        let url = self.host.api_url(path);
        let mut req = self
            .http
            .get(&url)
            .header("Accept", "application/vnd.github+json");
        if let Some(token) = &self.token {
            req = req.header("Authorization", format!("Bearer {token}"));
        }
        tracing::debug!("GET {url}");
        req.send().map_err(GorError::Http)
    }

    /// Make a `GET` request to an absolute URL (not API-path-based).
    ///
    /// Used for downloading release assets from the GitHub CDN.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails.
    pub fn get_absolute(&self, url: &str) -> Result<reqwest::blocking::Response, GorError> {
        let mut req = self
            .http
            .get(url)
            .header("Accept", "application/octet-stream");
        if let Some(token) = &self.token {
            req = req.header("Authorization", format!("Bearer {token}"));
        }
        tracing::debug!("GET {url}");
        req.send().map_err(GorError::Http)
    }

    /// Make a request with an arbitrary HTTP method, headers, and optional body.
    ///
    /// The path should start with `/`, e.g. `/repos/owner/repo`.
    /// `headers` is a slice of "Key: Value" strings.
    /// `body` is an optional raw byte vector for the request body.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails.
    pub fn request(
        &self,
        method: &str,
        path: &str,
        headers: &[String],
        body: Option<Vec<u8>>,
    ) -> Result<reqwest::blocking::Response, GorError> {
        let url = self.host.api_url(path);
        let mut req = self
            .http
            .request(method.parse().unwrap_or(reqwest::Method::GET), &url)
            .header("Accept", "application/vnd.github+json");

        if let Some(token) = &self.token {
            req = req.header("Authorization", format!("Bearer {token}"));
        }

        for header in headers {
            if let Some((key, value)) = header.split_once(':') {
                req = req.header(key.trim(), value.trim());
            }
        }

        if let Some(body_bytes) = body {
            req = req.body(body_bytes);
        }

        tracing::debug!("{} {url}", method.to_uppercase());
        req.send().map_err(GorError::Http)
    }

    /// Make a `POST` request to the given API path with a JSON body.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails.
    pub fn post<T: serde::Serialize>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<reqwest::blocking::Response, GorError> {
        let url = self.host.api_url(path);
        let mut req = self
            .http
            .post(&url)
            .header("Accept", "application/vnd.github+json")
            .json(body);
        if let Some(token) = &self.token {
            req = req.header("Authorization", format!("Bearer {token}"));
        }
        tracing::debug!("POST {url}");
        req.send().map_err(GorError::Http)
    }

    /// Make a `POST` request to an absolute URL (not API-path-based) with form data.
    ///
    /// Used for OAuth device flow endpoints which live on the main host,
    /// not the API subdomain.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails.
    pub fn post_form_url(
        &self,
        url: &str,
        form: &std::collections::HashMap<&str, &str>,
    ) -> Result<reqwest::blocking::Response, GorError> {
        tracing::debug!("POST {url}");
        self.http
            .post(url)
            .header("Accept", "application/json")
            .form(form)
            .send()
            .map_err(GorError::Http)
    }

    /// Make a GraphQL API request.
    ///
    /// Sends a POST request to the GraphQL endpoint with the given query
    /// and optional variables.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails.
    pub fn graphql(
        &self,
        query: &str,
        variables: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, GorError> {
        let url = self.host.api_url("/graphql");
        let mut body = serde_json::json!({"query": query});
        if let Some(vars) = variables {
            body["variables"] = vars;
        }
        let mut req = self
            .http
            .post(&url)
            .header("Accept", "application/vnd.github+json")
            .json(&body);
        if let Some(token) = &self.token {
            req = req.header("Authorization", format!("Bearer {token}"));
        }
        tracing::debug!("POST {url} (GraphQL)");
        let response = req.send().map_err(GorError::Http)?;
        let result: serde_json::Value = response.json().map_err(GorError::Http)?;
        if let Some(errors) = result.get("errors") {
            let msg = errors[0]["message"].as_str().unwrap_or("GraphQL error");
            return Err(GorError::Auth(msg.to_string()));
        }
        Ok(result)
    }

    /// Upload a release asset to the given upload URL.
    ///
    /// GitHub release asset uploads use a separate domain (uploads.github.com)
    /// and require the content type to be set explicitly.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails.
    pub fn upload_asset(
        &self,
        upload_url: &str,
        data: &[u8],
        content_type: &str,
    ) -> Result<reqwest::blocking::Response, GorError> {
        let mut req = self
            .http
            .post(upload_url)
            .header("Accept", "application/vnd.github+json")
            .header("Content-Type", content_type)
            .body(data.to_vec());
        if let Some(token) = &self.token {
            req = req.header("Authorization", format!("Bearer {token}"));
        }
        tracing::debug!("POST {upload_url}");
        req.send().map_err(GorError::Http)
    }

    /// Store the current token in the OS keyring.
    ///
    /// # Errors
    ///
    /// Returns an error if the keyring operation fails.
    pub fn save_token(&self) -> Result<(), GorError> {
        self.token.as_ref().map_or_else(
            || Ok(()),
            |token| keyring_store::set_token(self.host.hostname(), token),
        )
    }

    /// Update the client's token and persist it.
    ///
    /// # Errors
    ///
    /// Returns an error if the storage operation fails.
    pub fn set_token(&mut self, token: String) -> Result<(), GorError> {
        keyring_store::set_token(self.host.hostname(), &token)?;
        self.token = Some(token);
        Ok(())
    }

    /// Store the authenticated user's login name.
    ///
    /// # Errors
    ///
    /// Returns an error if the storage operation fails.
    pub fn save_user(&self, user: &str) -> Result<(), GorError> {
        keyring_store::set_user(self.host.hostname(), user)
    }

    /// Retrieve the stored user login, if available.
    ///
    /// # Errors
    ///
    /// Returns an error if the storage read fails.
    pub fn stored_user(&self) -> Result<Option<String>, GorError> {
        keyring_store::get_user(self.host.hostname())
    }
}
