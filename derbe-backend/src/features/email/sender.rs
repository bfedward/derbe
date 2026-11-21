use std::time::Duration;

use reqwest::StatusCode;
use secrecy::{ExposeSecret, SecretBox};
use serde_json::json;

pub struct ResendClient {
    http: reqwest::Client,
    base_url: String,
    api_key: SecretBox<String>,
    from: String,
}

pub struct OutgoingEmail<'a> {
    pub idempotency_key: &'a str,
    pub to: &'a str,
    pub subject: &'a str,
    pub html: &'a str,
}

#[derive(Debug, thiserror::Error)]
pub enum SendError {
    /// Worth retrying: network error, timeout, 429 or 5xx.
    #[error("temporary send failure: {0}")]
    Temporary(String),
    /// Retrying won't help: bad request, bad key, unverified domain.
    #[error("permanent send failure: {0}")]
    Permanent(String),
}

impl ResendClient {
    pub fn new(base_url: String, api_key: SecretBox<String>, from: String) -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(15))
            .build()
            .expect("HTTP client should build");
        Self {
            http,
            base_url,
            api_key,
            from,
        }
    }

    pub async fn send(&self, email: OutgoingEmail<'_>) -> Result<(), SendError> {
        let response = self
            .http
            .post(format!("{}/emails", self.base_url.trim_end_matches('/')))
            .bearer_auth(self.api_key.expose_secret())
            // Resend skips a repeat of the same key within 24 hours, so a retry can't double-send.
            .header("Idempotency-Key", email.idempotency_key)
            .json(&json!({
                "from": self.from,
                "to": [email.to],
                "subject": email.subject,
                "html": email.html,
            }))
            .send()
            .await
            .map_err(|error| SendError::Temporary(format!("request failed: {error}")))?;

        let status = response.status();
        if status.is_success() {
            return Ok(());
        }

        // Resend's error bodies are short JSON. Keep only the start, never the request.
        let body = response.text().await.unwrap_or_default();
        let detail = format!(
            "HTTP {status}: {}",
            body.chars().take(300).collect::<String>()
        );

        if status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error() {
            Err(SendError::Temporary(detail))
        } else {
            Err(SendError::Permanent(detail))
        }
    }
}
