# Dev Changelog

Internal development history for Derbe. Not user-facing.

## [Unreleased]

- Redundant `#[strum(serialize = "...")]` attributes are removed. `EmployeeRole` variants already serialize to their own names, so the four attributes are gone. `HalfDayOptions` uses `serialize_all = "camelCase"` and keeps overrides only for `FullDays` (`allDaysFull`) and `BothHalfDays` (`firstAndLastHalfs`). Parsed values are unchanged.

- `src/enums.rs` is removed and each enum now lives with its feature. `EmployeeRole` and `EmploymentType` moved to `features/employees/model.rs`, `LeaveStatus` (with `can_become`) and `HalfDayOptions` moved to `features/leave_requests/model.rs`, along with their unit tests. Variants, serde names, sqlx type names and generated TypeScript bindings are unchanged.

Before:

```rust
use crate::enums::{EmployeeRole, LeaveStatus};
```

After:

```rust
use crate::features::employees::model::EmployeeRole;
use crate::features::leave_requests::model::LeaveStatus;
```

- SQL migrations `0001` to `0007` are reformatted in the pg_dump style used by Rustify. Columns line up in three blocks (name, type, modifiers), types are lowercase (`character varying`, `timestamp with time zone`, `double precision`), tables are schema-qualified (`public.`), enum values sit one per line, and every constraint is named and listed at the end. Foreign keys are now explicit named constraints (`fk_<table>_<column>`) and `!=` became `<>`. Schema semantics are unchanged, and the missing comma before `date_check` in `0007` is fixed. sqlx stores a checksum per migration, so existing local databases must be recreated (`bash reset_db.sh --yes`).

Before:

```sql
CREATE TABLE locations (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    CONSTRAINT unique_location_name UNIQUE (name)
);
```

After:

```sql
CREATE TABLE public.locations (
    id      serial               NOT NULL,
    name    character varying    NOT NULL,
    CONSTRAINT locations_pkey PRIMARY KEY (id),
    CONSTRAINT unique_location_name UNIQUE (name)
);
```

- The local database reset script moved from `derbe-backend/` to the repository root, next to `README.md`. It now changes into `derbe-backend` itself to find `.env` and `migrations`, so it runs from the root. Behaviour is unchanged (needs `--yes`, refuses non-local hosts).

Before:

```text
cd derbe-backend && bash reset_db.sh --yes
```

After:

```text
bash reset_db.sh --yes
```

- Integration tests compile and pass again, and the permission layer is now tested through the real router. `test_helpers.rs` builds the current `AppState` and `app_router`, and gets `seed_employee` (inserts an employee with a role, bcrypt password), `login_client` (logs in via `POST /login`, keeps the session cookie) and `login_as(role)`. The five old `*_controller_it.rs` files log in as Admin and use the current JSON shapes (UUID `employee_unid`, `manager_unid`, no `phone`). New `tests/permissions_it.rs` has 18 tests: 401 without a session, wrong password, deleted employee cannot log in, `/me` per role, Admin-only creates (divisions, locations, leave types), every role can read, Manager creates employees but RegularUser and Demo cannot, no update of someone else, own leave request only, Manager approves a report but not their own leave or a stranger's, rejected cannot return to Pending. `EmployeeRole` derives `Display` (used by the helpers). Seeded employees own one "Seed division" and one "Seed location", so list tests find their row by name.

Before:

```rust
let ctx = AppState { pool: db };
let app = app_router(ctx.clone());
let client = reqwest::Client::new();
```

After:

```rust
let ctx = AppState { pool: db, jwt_secret, app_env: configuration.application.app_env };
let app = app_router(ctx.clone(), &configuration.application);
let (_admin, client) = test_helpers::login_as(&app, EmployeeRole::Admin).await;
```

- `access_control.rs` gets `ALL_ROLES`, replacing the four-role list repeated in each `permission.rs`, and `authorize` / `authorize_on` now log a `warn` with the user, role, permission (and target) when they deny, so a 403 can be explained from the logs. `Permission` now requires `Debug`. The 403 response body is unchanged.

Before:

```rust
Self::View => &[Admin, Manager, RegularUser, Demo],
```

After:

```rust
Self::View => ALL_ROLES,
```

- Employees and leave requests get their own `permission.rs` too (`EmployeePermission`, `LeaveRequestPermission`), and the central `permissions.rs` with `AccessController` and the `Permissions` trait is deleted. The `Scope` rule moved to `access_control.rs` as `authorize_on(actor, target, permission)`, a free function instead of a method on an empty struct, with its own tests. Approve and reject keep their rules in `leave_requests/permission.rs` as `authorize_approve` and `authorize_reject`. The 16-permission matrix test became one `roles_per_permission` test per feature, so a permission change fails a test next to the code. Role access is unchanged.

Before:

```rust
AccessController::authorize_update_employee(&user, &employee)?;
AccessController::authorize_approve_leave_request(&user, &target)?;
```

After:

```rust
authorize_on(&user, &employee, EmployeePermission::Update)?;
authorize_approve(&user, &target)?;
```

- Permissions move next to the feature that uses them. New `access_control.rs` holds the `Permission` trait (`roles_required()` says which roles may do an action) and `authorize(&user, permission)`. Divisions, locations and leave types each get a `permission.rs` with their own enum, and each role is listed explicitly (Admin included, no `Admin => true` shortcut, so adding a role never grants anything by accident). Role access is unchanged. Employees and leave requests follow in the next commits, then the central `permissions.rs` goes away.

Before:

```rust
AccessController::authorize(&user.role, Permission::CreateDivision)?;
```

After:

```rust
authorize(&user, DivisionPermission::Create)?;
```

- Moved `src/auth.rs` into `src/features/auth/`, like the other features: `handlers.rs` (login, me, logout), `dto.rs` (`LoginRequest`, `MeResponse`), `model.rs`, `repository.rs` (the three SQL queries), `service.rs` (password check, cookies), `jwt.rs` (`make_token` now takes the secret, so it no longer needs `AppState`; 2 unit tests) and `middleware.rs` (`require_auth`, `AuthenticatedUser`). Behaviour, routes, cookies and JSON shapes are unchanged (`me` still returns `unid`, `email`, `role`). Imports change from `crate::auth::AuthenticatedUser` to `crate::features::auth::middleware::AuthenticatedUser`.

- `EmployeeRole` derives `sqlx::Type` over the existing `text` column (`#[sqlx(type_name = "TEXT")]`), so `require_auth` and `me` read the role as `EmployeeRole` directly and the manual `from_str` parse with its 500 branch is gone. No Postgres enum on purpose: roles may change over time, and the `CHECK` constraint on `employees.role` is easier to change than a Postgres enum type. An unknown value in the column fails to decode instead of being granted a role. No migration change.

Before:

```rust
let role = match EmployeeRole::from_str(&role) {
    Ok(role) => role,
    Err(e) => { /* 500 "invalid employee role" */ }
};
```

After:

```rust
SELECT role AS "role!: EmployeeRole" FROM employees WHERE unid = $1 AND deleted_at IS NULL
```

- `authorize_leave_request_core` and `authorize_update_employee` had the same `match` on the role (Admin: yes, Manager: self or a report, RegularUser: self, Demo: no). The rule is now data: `Scope::of(role)` says whose records a role may act on, and one `authorize_on(actor, target, permission)` applies it. A new role fails to compile in `Scope::of` until its scope is decided. Approve and reject keep their own rules (no self-approval). The existing tests pass unchanged.

Before:

```rust
EmployeeRole::Manager => {
    if employee.unid == actor.unid || employee.manager_unid == Some(actor.unid) {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}
// ... same match again in authorize_leave_request_core
```

After:

```rust
pub fn authorize_update_employee(actor: &AuthenticatedUser, employee: &EmployeeAccess) -> Result<(), AppError> {
    Self::authorize_on(actor, employee, Permission::UpdateEmployee)
}
```

- `require_auth`, `login`, `me` and `logout` now return `Result<_, AppError>` instead of building `(StatusCode, Json(json!(...)))` six times. New `AppError::Unauthorized(&'static str)` (401) and `AppError::TokenFailure` (500). Cookie reading uses `CookieJar` instead of splitting the `Cookie` header by hand (`extract_session_cookie` is gone), and the cookie name is one `SESSION_COOKIE` constant. Error messages for 401 are unchanged (`no session`, `invalid token`, `employee not found`, `invalid credentials`); a database error is now the standard 500 `AppError::DatabaseQueryError` (it was a custom `db error` body).

Before:

```rust
let Some(token) = extract_session_cookie(request.headers()) else {
    tracing::warn!(path, "require_auth: no session cookie");
    return (StatusCode::UNAUTHORIZED, axum::Json(json!({"error": "no session"}))).into_response();
};
```

After:

```rust
let token = jar
    .get(SESSION_COOKIE)
    .ok_or(AppError::Unauthorized("no session"))?;
```

- Fixed: `0005_create_employees.sql` failed on PostgreSQL 17 with a syntax error. `CONSTRAINT x NOT NULL col` as a table constraint only exists from PostgreSQL 18. The six lines were redundant (each column already says `NOT NULL`), so they are removed and the migration now runs on PostgreSQL 17 and 18. Local databases must be recreated (`reset_db.sh`).

Before:

```sql
created_at timestamptz DEFAULT now() NOT NULL,
CONSTRAINT employees_created_at_not_null NOT NULL created_at,
```

After:

```sql
created_at timestamptz DEFAULT now() NOT NULL,
```

- Removed the 8 `Permission` variants that no route uses (`UpdateDivision`, `DeleteDivision`, `UpdateLocation`, `DeleteLocation`, `UpdateLeaveType`, `DeleteLeaveType`, `DeleteLeaveRequest`, `DeleteEmployee`) and the unused `role_accesses` migration (nothing in `src` or `tests` reads that table). Add them back with the route that needs them. The matrix tests now cover 16 permissions. Local databases must be recreated (`reset_db.sh`), as the migration list changed.

- Fixed: a deleted employee could log in and received a session cookie, then got a 401 on every request. `login` now skips soft-deleted employees, like `require_auth` and `me` already do.

Before:

```rust
"SELECT unid, password FROM employees WHERE email = $1",
```

After:

```rust
"SELECT unid, password FROM employees WHERE email = $1 AND deleted_at IS NULL",
```

- Removed `authorize_get_leave_request` and `authorize_get_my_leave_requests` from `permissions.rs`. No handler called them: the three GET leave request handlers only check the role, so any employee can read any leave request. Behaviour is unchanged. Who may read what is still to decide (see `SUGGESTIONS_W3.md` item 5).

- Added permission matrix tests in `permissions.rs`: one test per role lists every permission it gets, checked against all 24 `Permission` variants. Adding or removing a permission in `has_permission` now fails a test until the expected list is updated on purpose.

- Fixed: a RegularUser could put their own rejected or accepted leave request back to `Pending`. Added `LeaveStatus::can_become` and checked it in `update_leave_request_status` before the permission match. A rejected request is final, an accepted one can only be cancelled, and a request waiting for more info can be resubmitted or decided. `update_leave_request` (editing dates) is unchanged and still to decide.

Before:

```rust
let leave_request = service::get_by_id(id, &state).await?;
let target = employees::service::get_access_by_id(leave_request.employee_unid, &state).await?;
```

After:

```rust
let leave_request = service::get_by_id(id, &state).await?;
if !leave_request.leave_status.can_become(input.leave_status) {
    return Err(AppError::ValidationFailed(format!(
        "cannot change a {} leave request to {}",
        leave_request.leave_status, input.leave_status
    )));
}
let target = employees::service::get_access_by_id(leave_request.employee_unid, &state).await?;
```

- Fixed: a Manager could not create or update their own leave request, or update their own employee record. The `Manager` arm only checked the target's manager. Approving stays restricted: a Manager cannot approve their own leave. Added unit tests for these cases in `permissions.rs`.

Before:

```rust
EmployeeRole::Manager => {
    if target.manager_unid == Some(actor.unid) {
```

After:

```rust
EmployeeRole::Manager => {
    if target.unid == actor.unid || target.manager_unid == Some(actor.unid) {
```

- Added `derbe-backend/reset_db.sh`: drops, recreates and migrates the local database from `DATABASE_URL`. Needs `--yes` and refuses non-local hosts. Documented in `README.md`.

- Renamed migrations from timestamps to a simple sequence (`0001_...` to `0008_...`), same order as before. **Every existing local database must be recreated**: sqlx stores the migration version, so a database that ran the old names refuses the new ones. This also covers the in-place edits of the employees and leave requests migrations from `feature/auth`.

Before:

```text
migrations/20240423195045_create_employees.sql
migrations/20261112082313_create_role_accesses.sql
```

After:

```text
migrations/0005_create_employees.sql
migrations/0008_create_role_accesses.sql
```

- Sorted `derbe-backend/Cargo.toml` dependencies alphabetically (features too) and relaxed versions to the major (`"1"`, or `"0.8"` for 0.x crates). `Cargo.lock` keeps the exact versions, so builds are unchanged.

Before:

```toml
serde_json = "1.0.149"
tokio = { version = "1.49.0", features = ["full"] }
```

After:

```toml
serde_json = "1"
tokio = { version = "1", features = ["full"] }
```

## [0.2.0] - 2026-11-07

### Frontend naming and structure

- Renamed frontend source files to kebab-case, including pages, forms, and shared components. Updated imports and page route declarations. Generated `src/bindings` filenames remain generator-owned and unchanged.
- Replaced ambiguous generic `status-badge.tsx` with `leave-status-badge.tsx` and made leave-status labels/colors explicit in a typed map.
- Added `AGENTS.md`, `CLAUDE.md`, and `__LEARNING/LEARNINGS.md`; recorded kebab-case and generated-binding naming as a confirmed learning.

### Backend architecture and behavior

- Grouped each backend feature under `derbe-backend/src/features/`, keeping its handlers, request/response DTOs, model, repository, and service together. Root `routes.rs` composes feature handlers. No `pub use` compatibility facades.
- Preserved existing HTTP route paths, methods, JSON shapes, error messages, and status codes, including existing `201 Created` create responses.
- Replaced controller terminology with handlers, renamed generic CRUD methods to `get_by_id`, `list`, `create`, `update`, `delete`, and validation methods to intent-revealing names.
- Added `AppState`; made database/listener/server startup errors fallible with `thiserror`. Replaced `println!` with `tracing` while keeping existing error response text.
- Used `strum` for enum display and parsing. Added fallible `HalfDayOptions::try_from_string`; kept `from_string` and tuple-based conversion signatures compatible. External serialized labels stay unchanged.
- Simplified SQLx error propagation with `?`, kept typed `query_as!`/`query_scalar!` queries, and made deletes ignore missing rows as before.
- Synchronized backend and frontend package versions at `0.2.0`. Release tags use annotated `vMAJOR.MINOR.PATCH` tags.

Before, handler code repeated result matching:

```rust
match service_call().await {
    Ok(value) => Ok(Json(value)),
    Err(error) => Err(error),
}
```

After, `?` propagates errors directly:

```rust
let value = service_call().await?;
Ok(Json(value))
```

Before, feature files were split by technical layer:

```text
src/controller/
src/service/
src/repository/
src/model/
src/dto/
```

After, each feature owns its implementation:

```text
src/features/employees/
  dto.rs
  dto/request.rs
  dto/response.rs
  handlers.rs
  mod.rs
  model.rs
  repository.rs
  service.rs
```

## Versioning

Use Semantic Versioning. Keep `derbe-backend/Cargo.toml` and `derbe-frontend/package.json` versions synchronized. Release tags use `vMAJOR.MINOR.PATCH`.
