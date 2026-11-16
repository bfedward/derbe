# Suggestions, week 3

Reviewed: branch `feature/auth` at `ae2460d` ("More AccessController functions..."), 5 commits from 2026-11-12 to 2026-11-14, on top of `main` (`5298189`).

Most items of the first version of this review are now done in the commits on top of your branch (`feature/auth`, from `3da2352` to the latest). Read them in order, each one has a `CHANGELOG_DEV.md` entry with before and after code. This file keeps only what is still open.

## Done in the commits (removed from this file)

Both bugs with their tests (leave status transitions, Manager acting on their own leave and record), the permission matrix tests, the migration decision (reset on purpose, a local reset script, migrations numbered, PostgreSQL 17 fix), the single `Scope` rule, `AppError` in the auth middleware and handlers, deleted employees blocked at login, unused permissions and the `role_accesses` table removed, `auth` moved to `features/auth/`, and `permissions.rs` replaced by `access_control.rs` plus one `permission.rs` per feature (this also settles the `AccessController` namespace question). The role stays `text` with `sqlx::Type` over TEXT, not a Postgres enum, so roles can change over time.

## Still open

| # | Priority | Where | What |
|---|---|---|---|
| 1 | high | `tests/test_helpers.rs` | Integration tests do not compile, and nothing tests authn and authz yet |
| 2 | medium | `leave_requests/handlers.rs` | Who may read which leave request |
| 3 | medium | `leave_requests/handlers.rs` | `update_leave_request` still edits any request, whatever its status |
| 4 | low | `auth/service.rs` | Unknown email answers faster than a wrong password |
| 5 | next | plan (`TODO.md`) | Seed one user per role, then one test per step of the leave flow |

---

## 1. Integration tests do not compile, and nothing tests authn and authz

**Where:** `tests/test_helpers.rs`

Before:

```rust
let ctx = AppState { pool: db };
let app = app_router(ctx.clone());
```

After (same shape as `serve` in `http.rs`; the secret is a fixed test value):

```rust
let ctx = AppState {
    pool: db,
    jwt_secret: Arc::new(SecretBox::new(Box::new("test-secret".to_string()))),
    app_env: configuration.application.app_env,
};
let app = app_router(ctx.clone(), &configuration.application);
```

Then add a `login_as(role)` helper, one seeded employee per role, and one test per rule, for example "a RegularUser gets 403 creating leave for someone else" and "a rejected request cannot go back to Pending". Every route needs a login now, so each existing test has to log in first.

---

## 2. Who may read which leave request

Today the three GET handlers only check the role, so any employee can read `GET /leaveRequests/{id}` for any id. Your plan says "Employee: see All Leave", so this may be what you want.

**Question:** who may read what? Write the answer in `TODO.md` before the code. If only the owner and their manager may read a request, `authorize_on` with a `LeaveRequestPermission::View` check on the target does it, the same way update works.

---

## 3. `update_leave_request` still edits any request, whatever its status

Changing the status is checked (`LeaveStatus::can_become`), but editing the dates is not, so a RegularUser can still change the dates of an `Accepted` request.

**Question:** is editing only allowed while `Pending` or `MoreInfoReq`?

---

## 4. Unknown email answers faster than a wrong password

`login` skips the bcrypt check when the email does not exist, so an attacker can tell which emails exist from the response time. Worth knowing, not worth the code yet.

---

## 5. Seed users and the end of the plan

From `TODO.md`: seed one user per role with hashed passwords (needed by item 1), then integration tests for authn and authz.

## Order of work

```
item 5 + 1   seed users, fix test_helpers, login_as helper
item 2, 3    decide, write the answer in TODO.md, then code with a test first
item 4       later
```
