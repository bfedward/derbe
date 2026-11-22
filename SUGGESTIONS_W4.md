# Suggestions, week 4

Reviewed: `main` at `ebd38ee`, covering email notifications (`a45ca7d`) and CI test settings (`5425f30`), 2026-11-21. Backend tests: 66 unit and 56 integration passed with the CI Postgres environment. Clippy passed with warnings. None of these suggestions block CI. Earlier review: [SUGGESTIONS_W3.md](SUGGESTIONS_W3.md).

## Time box

Spend about 45 minutes on items 1 and 2, 15 minutes on item 3, then 20 minutes on item 4. Items 5 and 6 are lower-priority database hardening. Leave template redesign and admin UI for later.

## Senior review

The email path already has the right foundation: leave changes and outbox inserts share a transaction, the worker claims rows with `FOR UPDATE SKIP LOCKED`, leases protect concurrent workers, and provider requests use idempotency keys. Keep this design; focus next on making retries safe and terminal failures explicit.

## Summary

| # | Priority | Where | What |
|---|---|---|---|
| 1 | high | `derbe-backend/src/features/email/worker.rs` | Final retry starts after Resend's idempotency key expires |
| 2 | medium | `derbe-backend/src/features/email/sender.rs` | Retryable concurrent-idempotency 409 becomes a permanent failure |
| 3 | medium | `derbe-backend/src/configuration.rs` | Production silently defaults email links to localhost |
| 4 | medium | `derbe-backend/migrations/0008_create_email_outbox.sql`, `worker.rs` | Terminal failure is encoded as a retry-count sentinel |
| 5 | low | `derbe-backend/migrations/0008_create_email_outbox.sql` | Outbox accepts empty keys/templates and non-object payloads |
| 6 | low | `derbe-backend/migrations/0008_create_email_outbox.sql` | Outbox retains recipient and rendered input payload indefinitely |

---

## 1. Final retry starts after Resend forgets the idempotency key

```text
T+00:00  Resend accepts email, response times out
T+00:01  retry with same key, still protected
T+26:36  final retry, key expired, duplicate can send
```

**Where:** `derbe-backend/src/features/email/worker.rs:14-16`

**Test first:** first compile fails because the window constant does not exist. Add it, then the assertion fails at 1,596 minutes. Shorten the last delay; test passes.

```rust
#[test]
fn all_retries_start_within_resend_idempotency_window() {
    let retry_window_minutes = RETRY_DELAYS_MINUTES.iter().sum::<i32>();
    assert!(retry_window_minutes < RESEND_IDEMPOTENCY_WINDOW_MINUTES);
}
```

**Before**

```rust
const RETRY_DELAYS_MINUTES: [i32; 5] = [1, 5, 30, 120, 1440];
pub const MAX_ATTEMPTS: i32 = RETRY_DELAYS_MINUTES.len() as i32 + 1;
```

**After**

```rust
const RESEND_IDEMPOTENCY_WINDOW_MINUTES: i32 = 24 * 60;
const RETRY_DELAYS_MINUTES: [i32; 5] = [1, 5, 30, 120, 600];
pub const MAX_ATTEMPTS: i32 = RETRY_DELAYS_MINUTES.len() as i32 + 1;
```

The final attempt now starts after 756 minutes, inside Resend's 24-hour key window. [Resend idempotency keys](https://resend.com/changelog/idempotency-keys).

**Also change:** keep retry count and user-visible retry policy unchanged. Add the test beside worker tests. No caller or plan change.

---

## 2. A retryable Resend 409 gives up immediately

```text
Request with key K is still processing
Second request with key K -> 409 concurrent_idempotent_requests
Current code -> Permanent -> email never retries
```

**Where:** `derbe-backend/src/features/email/sender.rs:67-78`

**Test first:** add this unit test first. It fails to compile until the classifier exists, then passes when only the safe Resend conflict is temporary.

```rust
#[test]
fn concurrent_idempotency_conflict_is_retryable() {
    assert!(is_retryable_conflict(
        StatusCode::CONFLICT,
        Some(CONCURRENT_IDEMPOTENT_REQUESTS),
    ));
    assert!(!is_retryable_conflict(
        StatusCode::CONFLICT,
        Some("invalid_idempotent_request"),
    ));
}
```

**Before**

```rust
pub async fn send(&self, email: OutgoingEmail<'_>) -> Result<(), SendError> {
    let response = self
        .http
        .post(format!("{}/emails", self.base_url.trim_end_matches('/')))
        .bearer_auth(self.api_key.expose_secret())
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
```

**After**

```rust
const CONCURRENT_IDEMPOTENT_REQUESTS: &str = "concurrent_idempotent_requests";

#[derive(serde::Deserialize)]
struct ResendErrorBody {
    name: String,
}

fn is_retryable_conflict(status: StatusCode, error_name: Option<&str>) -> bool {
    status == StatusCode::CONFLICT && error_name == Some(CONCURRENT_IDEMPOTENT_REQUESTS)
}

pub async fn send(&self, email: OutgoingEmail<'_>) -> Result<(), SendError> {
    let response = self
        .http
        .post(format!("{}/emails", self.base_url.trim_end_matches('/')))
        .bearer_auth(self.api_key.expose_secret())
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

    let body = response.text().await.unwrap_or_default();
    let error_name = serde_json::from_str::<ResendErrorBody>(&body)
        .ok()
        .map(|error| error.name);
    let retryable_conflict = is_retryable_conflict(status, error_name.as_deref());
    let detail = format!(
        "HTTP {status}: {}",
        body.chars().take(300).collect::<String>()
    );

    if status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error() || retryable_conflict {
        Err(SendError::Temporary(detail))
    } else {
        Err(SendError::Permanent(detail))
    }
}
```

Parse Resend's structured error name; keep `invalid_idempotent_request` permanent. Resend says concurrent requests are safe to retry later. [Resend idempotency errors](https://resend.com/changelog/idempotency-keys).

**Also change:** add a tiny error-body type and a fake-provider case in `email_worker_it.rs`. No caller or plan change.

---

## 3. Missing production URL silently creates localhost email links

```text
APP_BASE_URL missing in runtime
             |
             v
http://localhost:4200/leave-request/7 in a real email
```

**Where:** `derbe-backend/src/configuration.rs:20-21,43-45`. The production compose already sets `APP_BASE_URL`; the email Ed received shows the running backend did not use that value.

**Test first:** add `APP_BASE_URL` to the shared test environment, then assert that settings parsing fails if it is omitted. Current default makes the new assertion fail.

```rust
#[test]
fn app_base_url_is_required() {
    let vars = vars(&[])
        .into_iter()
        .filter(|(key, _)| key != "APP_BASE_URL");
    assert!(envy::from_iter::<_, Settings>(vars).is_err());
}
```

**Before**

```rust
    #[serde(default = "default_app_base_url")]
    pub app_base_url: String,
```

```rust
fn default_app_base_url() -> String {
    "http://localhost:4200".into()
}
```

**After**

```rust
    pub app_base_url: String,
```

Delete `default_app_base_url`. Missing config now stops startup instead of sending broken links.

**Also change:** add `APP_BASE_URL` to `quality.yml` test environment and the `configuration.rs` test fixture. `.env.example` and production compose already set it. Check the live container config when deploying.

---

## 4. Model terminal email failure explicitly

```text
Current: sent_at IS NULL means pending OR permanently failed
         attempt_count = MAX_ATTEMPTS is the hidden failed flag

Suggested: pending -> sent | failed
```

**Where:** `derbe-backend/migrations/0008_create_email_outbox.sql:1-16`, `derbe-backend/src/features/email/worker.rs:84-185`

`give_up` stores `MAX_ATTEMPTS` in `attempt_count` to keep the row out of the worker query. That overloads a retry counter as state: dashboards and future admin actions must know the worker's constant to distinguish dead letters from pending rows.

**Suggested change:** add an explicit terminal state, such as nullable `failed_at` (least disruptive) or a status column if admin workflows are planned. Set it in `give_up`, filter it from claims, and expose counts for pending, sent, and failed. Keep `attempt_count` as an actual count. Add tests proving permanent errors set failed state and cannot be claimed again.

Avoid a PostgreSQL enum for this small state machine unless states are expected to grow; a constrained text column or timestamps fit current needs with simpler migrations.

---

## 5. Add cheap outbox shape constraints

The table currently permits empty `event_key`/`template` and any JSON value in `payload`; Rust callers prevent most of these today, but the DB is the final integrity boundary.

**Suggested change:** add checks for non-empty trimmed `event_key` and `template`, and `jsonb_typeof(payload) = 'object'`. Add a test that direct invalid inserts fail. Do not add a DB enum/check listing every template unless the team wants each new template to require a schema migration too.

This is small hardening, not a reason to block email delivery.

---

## 6. Define an outbox retention policy

`payload` can include request notes, and rows retain it alongside the recipient after delivery. The outbox has no cleanup or archival path.

**Suggested change:** decide a retention window with the product owner, then periodically delete or archive sent rows older than that window. Keep unsent and failed rows until resolved. Add a test for the cleanup predicate when retention work is scheduled; do not delete rows based on an arbitrary age in the worker.

This is a later privacy/storage task; the app is not in active use yet.

---

## Order of work

```text
1  Bound retry schedule to provider dedup window
   |
2  Retry Resend's safe concurrent-key conflict
   |
3  Require app base URL and cover config in CI
   |
4  Represent terminal failures explicitly
```
