# Dev Changelog

Internal development history for Derbe. Not user-facing.

## [Unreleased]

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
