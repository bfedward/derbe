use std::time::Duration;

use secrecy::{ExposeSecret, SecretBox};
use sqlx::PgPool;
use tokio::time::MissedTickBehavior;

use crate::features::email::{
    render::render,
    sender::{OutgoingEmail, ResendClient, SendError},
};

const POLL_INTERVAL: Duration = Duration::from_secs(30);
const BATCH_SIZE: i64 = 10;
/// Delay after the 1st, 2nd, ... failure. One more failure after the last delay gives up.
const RETRY_DELAYS_MINUTES: [i32; 5] = [1, 5, 30, 120, 1440];
pub const MAX_ATTEMPTS: i32 = RETRY_DELAYS_MINUTES.len() as i32 + 1;

struct ClaimedEmail {
    id: i64,
    event_key: String,
    recipient: String,
    template: String,
    payload: serde_json::Value,
    attempt_count: i32,
}

pub struct EmailWorker {
    pool: PgPool,
    client: ResendClient,
    app_base_url: String,
}

/// Starts the worker, or logs once and returns if no API key is set.
pub fn spawn(
    pool: PgPool,
    api_key: Option<SecretBox<String>>,
    from: String,
    resend_base_url: String,
    app_base_url: String,
) {
    let Some(api_key) = api_key.filter(|key| !key.expose_secret().trim().is_empty()) else {
        tracing::info!("RESEND_API_KEY is not set, email delivery is disabled");
        return;
    };

    let worker = EmailWorker::new(
        pool,
        ResendClient::new(resend_base_url, api_key, from),
        app_base_url,
    );
    tracing::info!("email worker started");
    tokio::spawn(worker.run());
}

impl EmailWorker {
    pub fn new(pool: PgPool, client: ResendClient, app_base_url: String) -> Self {
        Self {
            pool,
            client,
            app_base_url,
        }
    }

    async fn run(self) {
        let mut ticker = tokio::time::interval(POLL_INTERVAL);
        ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);
        loop {
            ticker.tick().await;
            if let Err(error) = self.process_batch().await {
                tracing::error!(%error, "email worker batch failed");
            }
        }
    }

    pub async fn process_batch(&self) -> Result<(), sqlx::Error> {
        for email in self.claim_due().await? {
            self.deliver(email).await?;
        }
        Ok(())
    }

    /// Takes a lease on due rows. A single statement commits by itself, before any network call.
    async fn claim_due(&self) -> Result<Vec<ClaimedEmail>, sqlx::Error> {
        sqlx::query_as!(
            ClaimedEmail,
            r#"
                WITH due AS (
                    SELECT id
                    FROM email_outbox
                    WHERE sent_at IS NULL
                      AND attempt_count < $1
                      AND next_attempt_at <= now()
                      AND (locked_until IS NULL OR locked_until < now())
                    ORDER BY next_attempt_at, id
                    FOR UPDATE SKIP LOCKED
                    LIMIT $2
                )
                UPDATE email_outbox AS email
                SET locked_until = now() + interval '5 minutes'
                FROM due
                WHERE email.id = due.id
                RETURNING email.id, email.event_key, email.recipient,
                          email.template, email.payload, email.attempt_count
            "#,
            MAX_ATTEMPTS,
            BATCH_SIZE
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn deliver(&self, email: ClaimedEmail) -> Result<(), sqlx::Error> {
        let rendered = match render(&email.template, email.payload, &self.app_base_url) {
            Ok(rendered) => rendered,
            Err(error) => {
                tracing::error!(id = email.id, template = %email.template, %error, "cannot render email");
                return self.give_up(email.id, &error.to_string()).await;
            }
        };

        let result = self
            .client
            .send(OutgoingEmail {
                idempotency_key: &email.event_key,
                to: &email.recipient,
                subject: &rendered.subject,
                html: &rendered.html,
            })
            .await;

        match result {
            Ok(()) => {
                tracing::info!(id = email.id, event_key = %email.event_key, "email sent");
                self.mark_sent(email.id).await
            }
            Err(SendError::Temporary(detail)) => {
                let attempt = email.attempt_count + 1;
                tracing::warn!(id = email.id, attempt, %detail, "email send failed, will retry");
                self.schedule_retry(email.id, attempt, &detail).await
            }
            Err(SendError::Permanent(detail)) => {
                tracing::error!(id = email.id, %detail, "email send failed permanently");
                self.give_up(email.id, &detail).await
            }
        }
    }

    async fn mark_sent(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE email_outbox SET sent_at = now(), locked_until = NULL, last_error = NULL WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn schedule_retry(&self, id: i64, attempt: i32, error: &str) -> Result<(), sqlx::Error> {
        let Some(&delay) = RETRY_DELAYS_MINUTES.get((attempt - 1) as usize) else {
            return self.give_up(id, error).await;
        };
        sqlx::query!(
            r#"
                UPDATE email_outbox
                SET attempt_count = $2,
                    next_attempt_at = now() + make_interval(mins => $3),
                    locked_until = NULL,
                    last_error = $4
                WHERE id = $1
            "#,
            id,
            attempt,
            delay,
            truncate(error)
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Stops automatic retries. The row stays unsent with its error, for manual inspection.
    async fn give_up(&self, id: i64, error: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE email_outbox SET attempt_count = $2, locked_until = NULL, last_error = $3 WHERE id = $1",
            id,
            MAX_ATTEMPTS,
            truncate(error)
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

fn truncate(error: &str) -> String {
    error.chars().take(500).collect()
}
