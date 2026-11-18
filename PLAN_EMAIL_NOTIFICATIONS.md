# Plan: Email notifications for leave requests

## What this change should do

Send two kinds of email:

1. Employee submits leave request: notify that employee's manager.
2. Manager accepts or rejects request: notify the employee who requested leave.

Saving the request or manager's decision must succeed even when the email service is down. Store each email in PostgreSQL first; send it in the background and retry temporary failures.

## Decisions for first version

- Send through the Resend HTTP API.
- Use one PostgreSQL outbox table. The outbox is the durable list of emails waiting to send.
- Notify only the assigned manager. If employee has no manager, do not guess another recipient; save request and write a warning log.
- Notify employee on `Accepted` and `Rejected` transitions. Do not send another email when status stays the same.
- Do not send notifications for edits, cancellations, or `MoreInfoReq` in this first version.
- Keep API key in environment configuration. Never put it in YAML committed to Git or log it.

## 1. Add email settings

Extend `derbe-backend/src/configuration.rs` with an `EmailSettings` section and add it to `Settings`. Use optional API key so local app can still start when email is not configured.

Example settings shape:

```rust
pub struct EmailSettings {
    pub resend_api_key: Option<SecretBox<String>>,
    pub from_address: String,
    pub app_base_url: String,
}
```

Set production values through environment variables, following existing `DERBE_...__...` configuration naming. For example:

```text
DERBE_EMAIL__RESEND_API_KEY=re_...
DERBE_EMAIL__FROM_ADDRESS=Derbe <leave@example.com>
DERBE_EMAIL__APP_BASE_URL=https://derbe.example.com
```

Wire these variables into `docker-compose.prod.yml` and document them in `DEPLOYMENT.md` and `README.md`. Add no real key to either file. Verify the sender domain with Resend before production use.

Add `reqwest` as a backend runtime dependency. Create a small sender function that submits `{ from, to, subject, html }` to `https://api.resend.com/emails` with bearer auth. Treat non-2xx response as send failure. Do not log the key or full email body.

## 2. Add a PostgreSQL outbox

Create the next numbered migration under `derbe-backend/migrations/`. Store enough data to send an email later without looking up mutable request data.

Suggested schema:

```sql
CREATE TABLE email_outbox (
    id                  bigserial PRIMARY KEY,
    event_key           text NOT NULL UNIQUE,
    recipient           text NOT NULL,
    template            text NOT NULL,
    payload             jsonb NOT NULL,
    created_at          timestamptz NOT NULL DEFAULT now(),
    next_attempt_at     timestamptz NOT NULL DEFAULT now(),
    sent_at             timestamptz,
    attempt_count       integer NOT NULL DEFAULT 0,
    locked_until        timestamptz,
    last_error          text
);

CREATE INDEX email_outbox_pending_idx
    ON email_outbox (next_attempt_at, id)
    WHERE sent_at IS NULL;
```

Use stable event keys to prevent duplicate messages, such as `leave-request:123:created` and `leave-request:123:accepted`. `ON CONFLICT (event_key) DO NOTHING` makes enqueue safe if a request handler retries.

Payload examples:

```json
{
  "employee_name": "Avery Chen",
  "leave_type": "Annual leave",
  "date_from": "2026-12-21",
  "date_to": "2026-12-24",
  "duration": 4.0,
  "notes": "Family trip",
  "request_id": 123
}
```

Decision email payload should also include `decision` (`Accepted` or `Rejected`) and manager's name. Include only fields needed by the templates.

## 3. Queue emails in the same transaction as leave changes

Keep leave-request code inside `derbe-backend/src/features/leave_requests/`. Add outbox repository functions there, or add a small `features/email/` module if the sender/outbox will serve more features soon.

Refactor the create and status-update repository methods to use a SQLx transaction. Business write and outbox insert must commit together. If either fails, roll both back.

Creation flow:

1. Load employee and manager contact details.
2. Insert leave request.
3. If manager and manager email exist, insert `leave-request:{id}:created` into outbox.
4. Commit transaction and return created request.

Example SQL shape:

```sql
INSERT INTO email_outbox (event_key, recipient, template, payload)
VALUES ($1, $2, 'leave_request_submitted', $3)
ON CONFLICT (event_key) DO NOTHING;
```

Status flow:

1. Read the current request status and employee/manager email details.
2. Apply existing permission and transition validation.
3. Update status, returning the updated row.
4. If old status differs from new status and new status is `Accepted` or `Rejected`, enqueue one decision email in that same transaction.
5. Commit and return updated request.

Do not enqueue based only on the incoming status. Compare persisted old status with requested new status, so repeated updates do not send duplicate decision emails.

## 4. Build background delivery worker

Start the worker from `derbe-backend/src/http.rs` after creating the database pool. If no API key is configured, log once that email delivery is disabled and do not start the sender loop. App still starts normally.

Poll about once per minute and process a small batch. Claim rows in a short database transaction, then commit before calling Resend. A lease (`locked_until`) allows recovery if process dies after claiming a row.

Example claim query shape:

```sql
WITH due AS (
    SELECT id
    FROM email_outbox
    WHERE sent_at IS NULL
      AND next_attempt_at <= now()
      AND (locked_until IS NULL OR locked_until < now())
    ORDER BY next_attempt_at, id
    FOR UPDATE SKIP LOCKED
    LIMIT 20
)
UPDATE email_outbox AS email
SET locked_until = now() + interval '2 minutes'
FROM due
WHERE email.id = due.id
RETURNING email.id, email.recipient, email.template, email.payload,
          email.attempt_count;
```

For each claimed row:

- Render the matching template.
- Send through Resend.
- On success, set `sent_at = now()`, clear `locked_until` and `last_error`.
- On failure, increment `attempt_count`, clear the lease, save a short sanitized error, and set a later `next_attempt_at` (for example 1, 2, 4, 8, then 24 hours).
- After a small fixed maximum, stop retrying automatically and leave the error visible for manual inspection. Document how to inspect failed rows with SQL.
- Unknown template: do not silently mark sent. Record an error and log the template name.

## 5. Add email templates

Add readable HTML files under `derbe-backend/src/features/leave_requests/` (or a dedicated email feature folder):

- `templates/leave-request-submitted.html`
- `templates/leave-request-decision.html`

Keep them simple and mobile-friendly. Include Derbe name, useful request details, and a link such as `{app_base_url}/leave/requests/{request_id}`. Confirm the actual frontend route before building the link.

Escape every value that comes from employee input or database text before inserting it into HTML. Do not interpolate notes as raw HTML. Add plain text alternative only if the chosen sending approach supports it cleanly; HTML is required for first version.

Example subjects:

```text
New leave request from Avery Chen
Your leave request was accepted
Your leave request was rejected
```

## 6. Deployment and documentation

- Add required production environment variable wiring in `docker-compose.prod.yml`.
- Add setup instructions to `README.md`: create Resend account, verify sender domain, set API key and sender address locally, then restart backend.
- Add deployment instructions to `DEPLOYMENT.md`; keep secrets in the server's untracked `.env` file.
- Update `CHANGELOG_DEV.md` under `[Unreleased]` when implementation lands.
- Do not change app startup behavior when email settings are omitted.

## 7. Verify behavior

Add backend tests for:

- Creating a request adds one outbox row for the assigned manager.
- Creating a request with no manager or manager email still succeeds and adds no row.
- Accepting and rejecting a request adds one employee notification.
- Repeating the same status does not add another notification.
- Outbox insert and leave change commit or roll back together.
- Failed send remains pending and receives a later retry time.
- Successful send gets `sent_at` set.
- Template rendering escapes notes and names.
- Missing API key does not prevent backend startup.

Run formatting, backend checks, migration checks, and SQLx offline metadata refresh if query macros need it. Review final `git diff` and production configuration before merging.

## Done when

- Manager gets an email after employee submits a leave request.
- Employee gets an email after manager accepts or rejects it.
- Emails survive backend restarts and provider outages.
- Email failure never rolls back a submitted request or saved manager decision.
- Same request event cannot enqueue duplicate emails.
- Production key stays outside Git; local and deployment setup are documented.
