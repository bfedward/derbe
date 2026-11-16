# Suggestions, week 3

Reviewed: branch `feature/auth` at `ae2460d` ("More AccessController functions..."), 5 commits from 2026-11-12 to 2026-11-14, on top of `main` (`5298189`).

**What I checked, and what I did not**

- I read `auth.rs`, `permissions.rs`, the leave request and employee handlers, `http.rs`, the migrations and the tests.
- I did **not** run `cargo test` or `cargo clippy`. The `query!` macros check the live database, and the database on my machine is on the old schema (`column "unid" does not exist`, see item 3).
- Items 1 and 2 are deduced from reading. Each has a small test that will confirm it, or prove me wrong.

None of this blocks you from continuing. You already know about the broken backend tests, the missing frontend work and `temporary_password` in the create response, so they are not repeated here (item 4 only shows how to fix `test_helpers.rs`).

## Are you following the plan? Yes.

| Plan item (`TODO.md`) | State |
|---|---|
| Login endpoint, JWT | Done (cookie, 30 days instead of 30 minutes, no refresh yet) |
| Whole app protected by default | Done (`route_layer(require_auth)`, only `/health`, `/login`, `/logout` are public) |
| Roles, one enum, CHECK constraint | Done (`role` column + CHECK, 4 roles: you added `Demo`) |
| Permission trait + access controller | Done, used in every handler |
| Only an employee and their manager touch the employee's leave | Done for create/update/approve/reject, with the gaps in items 1, 2 and 5 |
| Seed one user per role, bcrypt | Not yet |
| Integration tests for authn and authz | Not yet (item 4) |

## Time box

About 30 minutes: items 1, 2, 4 (the two bugs and the test that catches them). Item 3 needs a decision before your next push. Items 5 to 7 are the next session. Items 8 to 12 can wait until the auth tests exist.

## Summary

| # | Priority | Where | What |
|---|---|---|---|
| 1 | high | `leave_requests/handlers.rs:89` | A RegularUser can put their own decided leave request back to `Pending` |
| 2 | high | `permissions.rs:135` | A Manager cannot create or update their own leave request |
| 3 | medium | `migrations/20240423195045_create_employees.sql` | An already-applied migration was edited |
| 4 | medium | `permissions.rs`, `tests/test_helpers.rs` | No test for the permission matrix, test helpers do not compile |
| 5 | medium | `leave_requests/handlers.rs:49` | Two `authorize_get_*` functions are never called |
| 6 | medium | `permissions.rs:135-253` | Two functions with the same body |
| 7 | low | `permissions.rs:39,93` | `AccessController` is a namespace, `Permissions` has one impl |
| 8 | low | `auth.rs:75-117` | The role is read as a `String`, then parsed by hand |
| 9 | low | `auth.rs:46` | `require_auth` builds six error responses by hand |
| 10 | low | `auth.rs:168` | A deleted employee can still log in |
| 11 | low | `permissions.rs:7`, `role_accesses` | Permissions and a table nothing uses |
| 12 | next | `src/auth.rs`, `src/permissions.rs` | Auth is not a feature folder, `login` runs SQL in the handler, `permissions.rs` knows every feature (split one `permission.rs` per feature) |

---

## 1. A RegularUser can put their own decided leave request back to `Pending`

```
Ed (RegularUser) creates leave            -> Pending
Manager rejects it                        -> Rejected
Ed calls PUT /leaveRequests/7/status
         { "leave_status": "Pending" }    -> 200, the rejection is gone
```

**Where:** `derbe-backend/src/features/leave_requests/handlers.rs:89`

The `Pending` arm only asks "may this user update this leave request?". A RegularUser may, on their own. Nothing looks at the current status.

Before (on `feature/auth`):

```rust
pub async fn update_leave_request_status(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(input): Json<LeaveRequestStatusUpdate>,
) -> Result<Json<LeaveRequestResponse>, AppError> {
    let leave_request = service::get_by_id(id, &state).await?;
    let target = employees::service::get_access_by_id(leave_request.employee_unid, &state).await?;

    match input.leave_status {
        Pending => AccessController::authorize_update_leave_request(&user, &target)?,
        Accepted => AccessController::authorize_approve_leave_request(&user, &target)?,
        _ => AccessController::authorize_reject_leave_request(&user, &target)?,
    }

    let leave_request = service::update_leave_request_status(id, input, &state).await?;
    Ok(Json(leave_request.into()))
}
```

After: a status can only move along allowed steps, checked before the permission match.

```rust
// enums.rs
impl LeaveStatus {
    pub const fn can_become(self, next: Self) -> bool {
        use LeaveStatus::*;
        matches!(
            (self, next),
            (Pending, Accepted | Rejected | Cancelled | MoreInfoReq)
                | (MoreInfoReq, Pending | Cancelled)
                | (Accepted, Cancelled)
        )
    }
}
```

```rust
// leave_requests/handlers.rs
pub async fn update_leave_request_status(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(input): Json<LeaveRequestStatusUpdate>,
) -> Result<Json<LeaveRequestResponse>, AppError> {
    let leave_request = service::get_by_id(id, &state).await?;
    if !leave_request.leave_status.can_become(input.leave_status) {
        return Err(AppError::ValidationFailed(format!(
            "cannot change a {} leave request to {}",
            leave_request.leave_status, input.leave_status
        )));
    }
    let target = employees::service::get_access_by_id(leave_request.employee_unid, &state).await?;

    match input.leave_status {
        Pending => AccessController::authorize_update_leave_request(&user, &target)?,
        Accepted => AccessController::authorize_approve_leave_request(&user, &target)?,
        _ => AccessController::authorize_reject_leave_request(&user, &target)?,
    }

    let leave_request = service::update_leave_request_status(id, input, &state).await?;
    Ok(Json(leave_request.into()))
}
```

**The test, written first** (in `enums.rs`, no database). It fails to compile (no `can_become`), then fails, then passes:

```rust
#[cfg(test)]
mod tests {
    use super::LeaveStatus::*;

    #[test]
    fn a_decided_request_cannot_go_back_to_pending() {
        assert!(!Rejected.can_become(Pending));
        assert!(!Accepted.can_become(Pending));
        assert!(Pending.can_become(Accepted));
    }
}
```

**What else changes:** `update_leave_request` (line 73) has the same hole: a RegularUser can edit the dates of an `Accepted` request. Decide if editing is only allowed while `Pending` or `MoreInfoReq`.

**Question:** the `_ =>` arm treats `Cancelled` and `MoreInfoReq` as "reject". Is "a RegularUser may reject their own request" really "may cancel it"?

---

## 2. A Manager cannot create or update their own leave request

```
Manager Sam, manager_unid = Dana (Admin)
Sam creates leave for Sam:   target.manager_unid == Some(Dana)  != Some(Sam)  -> 403
```

**Where:** `derbe-backend/src/permissions.rs:135` (and `authorize_update_employee`, line 223, same arm)

You wrote "only an employee and their manager can create leave requests for the employee". A Manager is also an employee. The `Manager` arm only checks the target's manager, so Sam can create leave for Sam's team and never for Sam.

Before:

```rust
fn authorize_leave_request_core(
    actor: &AuthenticatedUser,
    target: &EmployeeAccess,
    permission: Permission,
) -> Result<(), AppError> {
    if !actor.role.has_permission(permission) {
        return Err(AppError::Forbidden);
    }

    match actor.role {
        EmployeeRole::Admin => Ok(()),

        EmployeeRole::Manager => {
            if target.manager_unid == Some(actor.unid) {
                Ok(())
            } else {
                Err(AppError::Forbidden)
            }
        }

        EmployeeRole::RegularUser => {
            if target.unid == actor.unid {
                Ok(())
            } else {
                Err(AppError::Forbidden)
            }
        }

        EmployeeRole::Demo => Err(AppError::Forbidden),
    }
}
```

After: the Manager arm also accepts themselves.

```rust
fn authorize_leave_request_core(
    actor: &AuthenticatedUser,
    target: &EmployeeAccess,
    permission: Permission,
) -> Result<(), AppError> {
    if !actor.role.has_permission(permission) {
        return Err(AppError::Forbidden);
    }

    match actor.role {
        EmployeeRole::Admin => Ok(()),

        EmployeeRole::Manager => {
            if target.unid == actor.unid || target.manager_unid == Some(actor.unid) {
                Ok(())
            } else {
                Err(AppError::Forbidden)
            }
        }

        EmployeeRole::RegularUser => {
            if target.unid == actor.unid {
                Ok(())
            } else {
                Err(AppError::Forbidden)
            }
        }

        EmployeeRole::Demo => Err(AppError::Forbidden),
    }
}
```

**Do not change** `authorize_approve_leave_request` (line 167). A Manager approving is checked with `manager_unid == actor.unid`, so Sam cannot approve their own leave. That is right, keep it.

**The test, written first** (bottom of `permissions.rs`, no database):

```rust
#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    fn user(role: EmployeeRole) -> AuthenticatedUser {
        AuthenticatedUser { unid: Uuid::new_v4(), role }
    }

    #[test]
    fn manager_can_create_leave_for_themselves() {
        let sam = user(EmployeeRole::Manager);
        let sam_as_target = EmployeeAccess { unid: sam.unid, manager_unid: Some(Uuid::new_v4()) };
        assert!(AccessController::authorize_create_leave_request(&sam, &sam_as_target).is_ok());
    }

    #[test]
    fn manager_cannot_approve_their_own_leave() {
        let sam = user(EmployeeRole::Manager);
        let sam_as_target = EmployeeAccess { unid: sam.unid, manager_unid: Some(Uuid::new_v4()) };
        assert!(AccessController::authorize_approve_leave_request(&sam, &sam_as_target).is_err());
    }
}
```

**What else changes:** same one-line fix in `authorize_update_employee` (line 234). It fails today for the same reason.

---

## 3. An already-applied migration was edited

```
main:          employees(id SERIAL, manager_id, phone ...)
feature/auth:  same file, now employees(unid UUID, manager_unid, password ...)
any database that ran the old file:  the new file never runs on it
```

**Where:** `migrations/20240423195045_create_employees.sql` and `20240425180156_create_leave_requests.sql`

sqlx remembers each migration by checksum. A database that already ran the old file refuses the edited one (`migration ... was previously applied but has been modified`, from memory, I did not trigger it). On my machine `cargo check` fails with 7 times `column "unid" does not exist`, because my database is on the old schema.

Two honest options, pick one and write it in `CHANGELOG_DEV.md`:

- **Reset on purpose.** Derbe has no production data. Keep the edit, run `sqlx database reset`, say in the changelog that everyone must reset. Cheapest.
- **Real migration.** Restore the two old files, add `20261112082313_employees_uuid_and_auth.sql` that does the change with `ALTER TABLE`. Right if the database ever holds data you care about, and good practice to learn.

**Question:** which one, and why?

---

## 4. No test for the permission matrix, and `test_helpers.rs` does not compile

**Where:** `permissions.rs` (no `#[cfg(test)]`), `tests/test_helpers.rs:29`

The matrix is 24 permissions times 4 roles. One wrong line gives a user too much access and nothing complains.

Matrix test (same `tests` module as item 2, no database):

```rust
#[test]
fn permission_matrix() {
    use EmployeeRole::*;
    use Permission::*;

    let cases = [
        (Demo, ViewLeaveRequests, true),
        (Demo, CreateLeaveRequest, false),
        (RegularUser, CreateEmployee, false),
        (RegularUser, ApproveLeaveRequest, false),
        (Manager, ApproveLeaveRequest, true),
        (Manager, CreateDivision, false),
        (Admin, CreateDivision, true),
    ];
    for (role, permission, expected) in cases {
        assert_eq!(role.has_permission(permission), expected, "{role:?} {permission:?}");
    }
}
```

`test_helpers.rs`, before:

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

Then the plan's last line: a `login_as(role)` helper, one seeded employee per role, and one test per step of the leave flow ("a RegularUser gets 403 creating leave for someone else").

---

## 5. Two `authorize_get_*` functions are never called

**Where:** `permissions.rs:107` and `:114`, `leave_requests/handlers.rs:29-57`

`authorize_get_leave_request` and `authorize_get_my_leave_requests` exist, but the three GET handlers call `authorize(&user.role, ...)`, a role check only. So any employee can read `GET /leaveRequests/{id}` for any id. Check if `/myLeave` also lets the caller name the employee in its body.

The plan says "Employee: see All Leave", so reading everything may be what you want. If yes, delete the two unused functions. If a request should only be readable by its owner and their manager, use them.

Before:

```rust
pub async fn get_leave_request(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<LeaveRequestResponse>, AppError> {
    AccessController::authorize(&user.role, Permission::ViewLeaveRequest)?;
    let leave_request = service::get_by_id(id, &state).await?;
    Ok(Json(leave_request.into()))
}
```

After (owner and manager only):

```rust
pub async fn get_leave_request(
    Extension(user): Extension<AuthenticatedUser>,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<LeaveRequestResponse>, AppError> {
    let leave_request = service::get_by_id(id, &state).await?;
    let target = employees::service::get_access_by_id(leave_request.employee_unid, &state).await?;
    AccessController::authorize_get_leave_request(&user, &target)?;
    Ok(Json(leave_request.into()))
}
```

**Question:** who may read what? Write the answer in `TODO.md` before the code.

---

## 6. Two functions with the same body

**Where:** `permissions.rs:135` (`authorize_leave_request_core`) and `:223` (`authorize_update_employee`)

After item 2, both are "Admin: yes, Manager: self or a report, RegularUser: self, Demo: no". Only the permission differs. The same shape in `authorize_reject_leave_request` (line 191) gets a third copy, with one difference: a Manager may not reject themselves.

Before: `authorize_update_employee` as it is on the branch (lines 223 to 253, the `match` from item 2).

After: say who a role may act on once, as data.

```rust
#[derive(Clone, Copy)]
enum Scope {
    Everyone,
    SelfAndReports,
    SelfOnly,
    Nobody,
}

impl Scope {
    fn of(role: EmployeeRole) -> Self {
        match role {
            EmployeeRole::Admin => Self::Everyone,
            EmployeeRole::Manager => Self::SelfAndReports,
            EmployeeRole::RegularUser => Self::SelfOnly,
            EmployeeRole::Demo => Self::Nobody,
        }
    }

    fn covers(self, actor: &AuthenticatedUser, target: &EmployeeAccess) -> bool {
        match self {
            Self::Everyone => true,
            Self::SelfAndReports => {
                target.unid == actor.unid || target.manager_unid == Some(actor.unid)
            }
            Self::SelfOnly => target.unid == actor.unid,
            Self::Nobody => false,
        }
    }
}

fn authorize_on(
    actor: &AuthenticatedUser,
    target: &EmployeeAccess,
    permission: Permission,
) -> Result<(), AppError> {
    if actor.role.has_permission(permission) && Scope::of(actor.role).covers(actor, target) {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

pub fn authorize_update_employee(
    actor: &AuthenticatedUser,
    employee: &EmployeeAccess,
) -> Result<(), AppError> {
    Self::authorize_on(actor, employee, Permission::UpdateEmployee)
}
```

Adding a role now fails to compile in `Scope::of` until you decide its scope. That is the "force the implementation" you asked about. Leave approve and reject as they are: their rules differ (no self-approval).

**The test:** the tests from item 2 must stay green with no change. That is the proof the refactor changed nothing.

---

## 7. `AccessController` is a namespace, `Permissions` has one impl

**Where:** `permissions.rs:39` and `:93`

`AccessController` is a unit struct with only associated functions, which is a module. `Permissions` is a trait with one implementer (`EmployeeRole`), and `authorize<P: Permissions>` is generic for no caller that needs it.

Before, in every handler:

```rust
AccessController::authorize(&user.role, Permission::ViewEmployees)?;
```

After: one method on the user, no trait, no generic.

```rust
impl AuthenticatedUser {
    pub fn require(&self, permission: Permission) -> Result<(), AppError> {
        if self.role.has_permission(permission) {
            Ok(())
        } else {
            Err(AppError::Forbidden)
        }
    }
}
```

```rust
user.require(Permission::ViewEmployees)?;
```

**Not for now:** you asked for a reusable version so the check is not in every handler. A typed extractor `Authorized<P>` puts the permission in the handler signature, as in the RUSTIFY image CDN (`access_macros`), but it needs one marker type per permission (24 here), and the checks that depend on the target (item 6) need a database row, which an extractor does not have. The method above is enough until a second check makes you want it.

---

## 8. The role is read as a `String`, then parsed by hand

**Where:** `derbe-backend/src/auth.rs:75-117`

`require_auth` selects `role` as text, then runs `EmployeeRole::from_str` with its own 500 branch. Your other enums (`LeaveStatus`, `EmploymentType`) use `sqlx::Type` with a Postgres type (`valid_leave_statuses`), so the database already guarantees the values.

Before:

```rust
let role = match sqlx::query_scalar!(
    r#"SELECT role FROM employees WHERE unid = $1 AND deleted_at IS NULL"#,
    claims.sub
)
.fetch_optional(&state.pool)
.await
{
    Ok(Some(role)) => role,
    // ... 2 more arms
};

let role = match EmployeeRole::from_str(&role) {
    Ok(role) => role,
    // ... 500 "invalid employee role"
};
```

After (new migration `CREATE TYPE valid_employee_roles AS ENUM (...)`, derive `sqlx::Type` on `EmployeeRole` with `#[sqlx(type_name = "valid_employee_roles")]`, same as `LeaveStatus`; I did not compile this):

```rust
let role = sqlx::query_scalar!(
    r#"SELECT role AS "role: EmployeeRole" FROM employees WHERE unid = $1 AND deleted_at IS NULL"#,
    claims.sub
)
.fetch_optional(&state.pool)
.await?
.ok_or(AppError::Unauthorized("employee not found"))?;
```

This also makes the CHECK constraint on `employees.role` and `role_accesses.role` unnecessary. Do it together with item 3 if you reset the database.

---

## 9. `require_auth` builds six error responses by hand

**Where:** `derbe-backend/src/auth.rs:46`

You already have `AppError` with `IntoResponse` and one JSON shape. `require_auth` rebuilds `(StatusCode, Json(json!(...)))` six times, and `extract_session_cookie` (line 139) splits the cookie header by hand while `CookieJar` is imported in the same file.

Before (first part of the function):

```rust
pub async fn require_auth(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    let path = request.uri().path().to_owned();

    let Some(token) = extract_session_cookie(request.headers()) else {
        tracing::warn!(path, "require_auth: no session cookie");
        return (
            StatusCode::UNAUTHORIZED,
            axum::Json(json!({"error": "no session"})),
        )
            .into_response();
    };

    let claims = match verify_token(&token, state.jwt_secret.expose_secret()) {
        Ok(claims) => claims,
        Err(e) => {
            tracing::warn!(path, error = %e, "Invalid session token");
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "invalid token"})),
            )
                .into_response();
        }
    };
    // ...
```

After: one `AppError::Unauthorized(&'static str)` variant (status 401 in `into_response`), and the function returns `Result`.

```rust
pub async fn require_auth(
    State(state): State<AppState>,
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = jar
        .get("session")
        .ok_or(AppError::Unauthorized("no session"))?;

    let claims = verify_token(token.value(), state.jwt_secret.expose_secret()).map_err(|e| {
        tracing::warn!(error = %e, "invalid session token");
        AppError::Unauthorized("invalid token")
    })?;
    // role lookup from item 8, then:
    request.extensions_mut().insert(AuthenticatedUser { unid: claims.sub, role });
    Ok(next.run(request).await)
}
```

Delete `extract_session_cookie`. The cookie name `"session"` is also written in `login` and `logout`: make it one `const SESSION_COOKIE`.

---

## 10. A deleted employee can still log in

**Where:** `derbe-backend/src/auth.rs:168`

`require_auth` filters `deleted_at IS NULL`. `login` does not, so a deleted employee gets a cookie and then a 401 on every request.

Before:

```rust
"SELECT unid, password FROM employees WHERE email = $1",
```

After:

```rust
"SELECT unid, password FROM employees WHERE email = $1 AND deleted_at IS NULL",
```

**Not for now:** an unknown email returns faster than a wrong password (bcrypt is skipped), which tells an attacker which emails exist. Worth knowing, not worth the code yet.

---

## 11. Permissions and a table that nothing uses

**Where:** `permissions.rs:7-37`, `migrations/20261112082313_create_role_accesses.sql`

- 8 `Permission` variants have no route: `UpdateDivision`, `DeleteDivision`, `UpdateLocation`, `DeleteLocation`, `UpdateLeaveType`, `DeleteLeaveType`, `DeleteLeaveRequest`, `DeleteEmployee`. `DeleteLeaveRequest` is even granted to two roles.
- `role_accesses` is created, and `grep role_accesses src tests` finds nothing.

Does it need to exist at all? Delete them, add them back with the route that needs them. The matrix in item 4 gets shorter and every line has a route behind it.

---

## 12. Auth is not a feature folder, and `login` runs SQL in the handler

**Where:** `derbe-backend/src/auth.rs`, `derbe-backend/src/permissions.rs`, `auth.rs:166`

Every other domain lives under `features/<feature>/` with its handlers, DTOs, repository and service (your `AGENTS.md`). Auth is two files at the root of `src/`, and `login` calls `sqlx::query_as!` inside the handler, which is the split the 0.2.0 refactor removed.

Before:

```
src/auth.rs          Claims, LoginRequest, login, logout, require_auth,
                     make_token, verify_token, SQL queries
src/permissions.rs   Permission, AccessController
```

After:

```
src/features/auth/
    mod.rs
    handlers.rs      login, logout
    dto.rs           LoginRequest
    repository.rs    find_login_by_email, find_role_by_unid
    service.rs       check the password, build the cookie
    jwt.rs           Claims, make_token, verify_token
    middleware.rs    require_auth, AuthenticatedUser
src/access_control.rs   the shared trait and `authorize` (see below)
src/features/<feature>/permission.rs   one small enum per feature (see below)
```

No `pub use`, as in the other features.

### `permissions.rs` knows every feature

Today one file holds the 24 permissions of five features, the role matrix of four roles, and the `authorize_*` functions that read `EmployeeAccess` and `AuthenticatedUser`. Adding "export leave requests" means editing a file that also holds divisions and locations.

```
Before: one file, role -> permissions

permissions.rs
  enum Permission { CreateDivision, ... ApproveLeaveRequest }   24 variants
  impl Permissions for EmployeeRole { match role { ... } }      4 long matches
  impl AccessController { authorize_*, authorize_*_core }
```

```
After: the shared part is tiny, each feature owns its own permissions

src/access_control.rs                       trait Permission + authorize()
src/features/divisions/permission.rs        enum DivisionPermission
src/features/leave_requests/permission.rs   enum LeaveRequestPermission + authorize_* on a target
src/features/employees/permission.rs        enum EmployeePermission
```

The shared part (`src/access_control.rs`):

```rust
pub trait Permission {
    fn roles_required(&self) -> &'static [EmployeeRole];

    fn is_granted_to(&self, role: EmployeeRole) -> bool {
        self.roles_required().contains(&role)
    }
}

pub fn authorize(user: &AuthenticatedUser, permission: impl Permission) -> Result<(), AppError> {
    if permission.is_granted_to(user.role) {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}
```

One feature (`src/features/divisions/permission.rs`):

```rust
#[derive(Debug, Clone, Copy)]
pub enum DivisionPermission {
    Create,
    View,
}

impl Permission for DivisionPermission {
    fn roles_required(&self) -> &'static [EmployeeRole] {
        match self {
            Self::Create => &[EmployeeRole::Admin],
            Self::View => &[
                EmployeeRole::Admin,
                EmployeeRole::Manager,
                EmployeeRole::RegularUser,
                EmployeeRole::Demo,
            ],
        }
    }
}
```

What changes:

- The matrix flips from "role -> permissions" to "permission -> roles". The question you ask most is "who can do X?", and now the answer is one `match` arm, not four `matches!` blocks. The matrix test of item 4 becomes one table per feature.
- A call site reads `authorize(&user, DivisionPermission::Create)?`.
- The 8 unused variants of item 11 are not carried over, so they disappear for free.
- Target-dependent rules (own leave, manager of the target) stay next to the leave request code in `features/leave_requests/permission.rs`, since they need `EmployeeAccess`. Item 6 gives them one `Scope` function there.
- The Admin shortcut (`Admin => true`) goes away: list `Admin` in each arm, so a new permission is not silently granted to Admin.

This is the layout the ERP project in my own repos uses, and the same pattern as the image CDN of week 8: a generic trait in one shared module, the enum defined with the feature it protects. Not needed now: a macro that generates the checks (`#[secured_server]`). Plain `authorize(...)` calls are enough for 5 features.

**Order:** after items 6 and 7, since they change the same functions. Do one feature per commit, start with `divisions` (smallest), run the matrix test after each.

**Do it after items 8 to 10.** They edit the same code, and moving files first makes a noisy diff. Move with `git mv` so the history follows.

**Question:** your idea of one folder per endpoint (`create-division/`) would apply here too. Do you want it everywhere, or per feature when it grows?

---

## Order of work

```
item 1 + 2 + 4   the two bugs, each with its test, then the matrix test
        |
item 3           decide: reset or real migration (before the next push)
        |
item 6 -> 7      refactor with the item 2 tests as safety net
        |
item 5           decide who reads what, then code
        |
item 8, 9, 10    together with the seed users and the auth tests
        |
item 11          cleanup
        |
item 12          move auth into features/auth, then split permissions.rs
                 into access_control.rs + one permission.rs per feature
```
