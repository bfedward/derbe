use serde::Serialize;
use sqlx::{types::Json, PgConnection};

use crate::{error::AppError, features::email::model::EmailTemplate};

/// Queues an email. A repeated `event_key` is ignored, so retries can't create duplicates.
pub async fn enqueue(
    conn: &mut PgConnection,
    event_key: &str,
    recipient: &str,
    template: EmailTemplate,
    payload: &impl Serialize,
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
            INSERT INTO email_outbox (event_key, recipient, template, payload)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (event_key) DO NOTHING
        "#,
        event_key,
        recipient,
        template.to_string(),
        Json(payload) as _
    )
    .execute(&mut *conn)
    .await?;

    Ok(())
}
