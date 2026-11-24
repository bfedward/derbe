# Dev Changelog

Internal development history for Derbe. Not user-facing.

## [Unreleased]

- Split the backend into a Rust 2024 Cargo workspace: one crate per feature, with shared runtime state, errors, configuration, session identity, and domain types in `derbe-core`; the app crate now assembles feature routes.

- Replaced separate leave-request date inputs with the shadcn range calendar (Calendar + Popover). Preserved half-day selection and duration preview; single-day ranges offer full day or half day, multi-day ranges can halve either endpoint or both.

- Added List and Organisation Chart views to Organisation. The chart groups Admin, Manager, and employee roles into levels, links employees to their manager where available, and uses a dedicated hook for hierarchy wiring. Migration `10001_____demo_org_chart_data.sql` seeds three managers and ten assigned employees without changing the applied seed migration checksum.

- Added List and Organisation Chart views to Organisation. The chart groups Admin, Manager, and employee roles into levels, links employees to their manager where available, and uses a dedicated hook for hierarchy wiring. The temporary seed now includes three managers and ten assigned employees so the chart has a realistic demo tree.

- Added List and Organisation Chart views to Organisation. The chart groups Admin, Manager, and employee roles into levels, links employees to their manager where available, and uses a dedicated hook for hierarchy wiring. Migration `10001_____demo_org_chart_data.sql` seeds three managers and ten assigned employees without changing the applied seed migration checksum.

- Added a public Derbe homepage at `/`, moved the authenticated application under `/dashboard/`, updated internal navigation, and kept legacy dashboard URLs redirecting to their `/dashboard/...` equivalents. Login now returns users to the protected URL they originally requested.

- Replaced the top dropdown navigation with the official shadcn `sidebar-07` collapsible sidebar components. Derbe routes are grouped by feature, active pages and role-based employee links are shown in the sidebar, and the user menu handles logout. Sidebar works on mobile, supports icon collapse, and inherits light/dark theme colors. Sidebar uses official neutral accent colors, and only active links receive the active state. The group for leave types, locations, and divisions is named “Lists”. Removed sample projects and team-switcher data because Derbe has no project or team switching.

- Added migration `10000_____demo_account_roles.sql` to keep the applied seed migration checksum intact, make `employee@example.com` a RegularUser, and add `admin@example.com` as Admin. Both use password `password`.

- The leave request page shows the employee's name and the leave type's name instead of their ids. GET /leaveRequests/{id} now returns LeaveRequestDetailsResponse, which flattens LeaveRequestResponse and adds employee_name and leave_type. Names are joined after the permission check, and soft-deleted employees still show their name. Accepting or rejecting now also refreshes Team Leave.

- Queries no longer retry 4xx responses, so a 403 or 404 shows its error straight away instead of a loading skeleton for several seconds. API errors show the backend's message instead of raw JSON, and 403 reads 'You don't have permission to view this.' On the Organisation page, employee names are only links for admins and managers, and the Role column uses the same labels as the rest of the app.

- Fixed clicking an employee in Organisation showing 'Employee not found.': the route parameter was :id but the page read employee_unid, so it never loaded the employee.

- An employee's manager must now be an admin or a manager. The Manager dropdown only lists admins and managers (plus the employee's current manager, when editing older data), and POST/PUT /employees return 400 for any other manager.

- Employee emails are now unique among active employees (employees_email_active_key, a partial unique index on email WHERE deleted_at IS NULL). Previously two employees could share an address, and login picked one of them arbitrarily. Creating or updating an employee with an email that's already in use returns 409 'An employee with this email address already exists'. A soft-deleted employee's address can be reused. create_employee in the test helpers now derives the email from the first name.

- Added a Team Leave screen (`/team-leave`, under Days Off) listing the pending leave requests the logged-in user can decide on: an admin sees everyone's, a manager sees their direct reports'. `MoreInfoReq` requests are not listed, because they are waiting on the employee and already show on their My Leave screen. Each row has the employee's name, leave type, dates, duration, date requested and a Review link to the request page. The menu item is only shown to admins and managers.
  - New `GET /teamLeave` endpoint instead of extending `/myLeave`. The backend decides whose team it is from the session (`None` for an admin, `Some(user.unid)` for a manager), so the client can never ask for another manager's team. It also needs a different response shape (`TeamLeaveResponse`, with `employee_name`). The SQL excludes soft-deleted employees and orders by `date_from`. A manager's own pending request doesn't appear, because they can't approve it.
  - New `LeaveRequestPermission::ListTeam` (`Admin`, `Manager`). `RegularUser` and `Demo` get 403.
  - Accepting or rejecting a request now also invalidates the Team Leave query, so the request disappears when the manager goes back to the list.
  - Tests in `team_leave_it.rs`: a manager sees only their direct reports, an admin sees everyone (including employees with no manager), a manager doesn't see their own request, accepted, rejected and cancelled requests and deleted employees are not listed, `RegularUser` and `Demo` get 403, no session gets 401.

- Fixed `GET /leaveRequests/{id}` checking only the role (`View`) and not whose request it is: any manager or regular user could open any request by changing the id in the URL. It now uses `authorize_on` with the request's employee, so a regular user can only view their own requests and a manager their own and their direct reports'. Tests added to `permissions_it.rs`.

Before:

```rust
authorize(&user, LeaveRequestPermission::View)?;
let leave_request = service::get_by_id(id, &state).await?;
```

After:

```rust
let leave_request = service::get_by_id(id, &state).await?;
let target = employees::service::get_access_by_id(leave_request.employee_unid, &state).await?;
authorize_on(&user, &target, LeaveRequestPermission::View)?;
```

- Added a user menu to the header, showing the logged-in user's name and role ("Employee" for `RegularUser`). It opens to show their email and a Log out item. Logging out calls `POST /logout`, navigates to `/login`, then runs `queryClient.clear()`, so the next user never briefly sees the previous user's cached data. Switching users no longer means clearing browser cookies.
  - `/logout` stays a public route: it only clears the caller's own cookie, and it must still work when the session is expired or invalid, which is exactly when `require_auth` would reject it. Sessions are stateless JWTs, so logout deletes the cookie but doesn't revoke the token.

- `GET /me` now returns `first_name` and `last_name`. `CurrentUser` and `EmployeeRole` are exported with ts-rs, so the frontend uses the generated `CurrentUser` binding instead of a hand-written interface in `api.ts`.

- Added `currentUserQueryOptions()` and `derbeKeys.currentUser()`, used by `ProtectedRoute`, `Layout` and the login page instead of three copies of an inline `["currentUser"]` query. Updating the login page's invalidation at the same time matters: if it invalidated a stale key, a cached 401 from an expired session would survive a successful login and send the user straight back to `/login`.

Before:

```tsx
useQuery({ queryKey: ["currentUser"], queryFn: getCurrentUser, retry: false });
```

After:

```tsx
useQuery(currentUserQueryOptions());
```

- Fixed the employee form's Manager dropdown: it converted the selected UUID with `Number(value)`, which stored `NaN`, left the dropdown blank and failed validation with "Invalid input".

- Documented that migration `9999_____seed_data.sql` is temporary fake demo data, acceptable pre-launch and to remove before onboarding real users.

- Fixed the Quality workflow's tests failing with MissingValue("database_url"): CI has no .env, so the Tests step now sets DATABASE_URL, JWT_SECRET, APP_ENV and CORS_ORIGINS for the Postgres service.

- Added email notifications for leave requests, sent through the Resend HTTP API from `notify.derbe.app`. The manager gets an email when an employee submits a request, and the employee gets one when it is accepted or rejected. Other changes (edits, cancellations, `MoreInfoReq`) send nothing yet. No email is sent when the employee has no active manager (a warning is logged) or when the manager files leave for their own report.
  - Transactional outbox: new migration `0008_create_email_outbox.sql`. The leave change and its outbox row are written in one SQLx transaction, so an email failure can never lose a saved request or decision. A unique `event_key` (`leave-request:{id}:created`, `:accepted`, `:rejected`) with `ON CONFLICT DO NOTHING` prevents duplicates. The payload is a snapshot of the fields the email needs.
  - New `features/email/` module: `model.rs` (template enum and payload structs), `repository.rs` (`enqueue`), `sender.rs` (small `reqwest` client for `POST /emails`), `worker.rs` (background loop), `render.rs` and `templates/` (`layout.html`, `leave-request-submitted.html`, `leave-request-decision.html`, compiled in with `include_str!`).
  - Worker: started from `http::serve` after migrations, polls every 30 s. Claims due rows with a 5-minute `locked_until` lease (`FOR UPDATE SKIP LOCKED`), commits, then calls Resend. Sends `event_key` as the `Idempotency-Key` header, so a crash between sending and saving `sent_at` can't double-send. Network errors, 429 and 5xx retry after 1 min, 5 min, 30 min, 2 h and 24 h; 4xx errors, unknown templates and bad payloads give up at once. A given-up row stays unsent with `last_error` for manual inspection (queries in `DEPLOYMENT.md`). 15 s request timeout. The API key is only used in the `Authorization` header and never logged.
  - Rendering: one-pass `{{placeholder}}` fill. User and database text is always HTML-escaped and never re-scanned, so `{{link}}` in notes stays literal. A missing placeholder is an error, not a panic, so it can't kill the worker task.
  - New optional settings: `RESEND_API_KEY` (missing or blank disables the worker, and the app starts normally), `EMAIL_FROM`, `APP_BASE_URL`, `RESEND_BASE_URL` (lets tests point at a fake server). Production gets `RESEND_API_KEY` from a new `production` secret written into `~/derbe/.env` by `deploy.yml`. Compose sets `EMAIL_FROM` and derives `APP_BASE_URL` from `DERBE_DOMAIN`. Documented in `.env.example`, `README.md` and `DEPLOYMENT.md`.
  - `reqwest` moved from dev-dependencies to dependencies (rustls, no OpenSSL in the runtime image).

- Fixed a race in leave status updates. The update now only matches if the status is still the one that was validated, and returns the new `AppError::Conflict` (409) if someone else changed it first. This also stops two managers deciding at once from queuing contradictory emails. Removed the duplicate `validate_status_update_rules` call: the handler path ran it twice.

Before:

```sql
UPDATE leave_requests SET leave_status = $1, ... WHERE id = $4
```

After:

```sql
UPDATE leave_requests SET leave_status = $1, ... WHERE id = $4 AND leave_status = $5
```

- Leave-request repository `create` and `update_status` now take `&mut PgConnection` instead of `&AppState`, so the service can run them in a transaction. `service::create` and `service::update_leave_request_status` take the logged-in user's `unid` (used for the manager self-check and the decision email's `decided_by`).

- Fixed the leave request form's Employee dropdown: it converted the selected UUID with `Number(value)`, which stored `NaN` and failed validation with "expected string, received NaN".

Before:

```tsx
onValueChange={(value) => field.onChange(Number(value))}
```

After:

```tsx
onValueChange={(value) => field.onChange(value)}
```

- Tests: `email_outbox_it.rs` (one manager email per submission, none without a manager or for a manager's own filing, one employee email per accept and reject, no email for a repeated decision, outbox failure rolls back both create and decision), `email_worker_it.rs` (against a fake axum Resend: sent, retried, given up, unknown template, no start without a key), render unit tests (escaping, placeholders in user text, every placeholder filled), and a configuration test showing the email settings are optional. Moved `post_leave_request` and `put_status` from `permissions_it.rs` into `test_helpers.rs`.

- Added `pnpm sqlx:prepare`, and a pre-push check that runs `cargo sqlx prepare --check` when backend `.rs`, `.toml` or `.sql` files changed and local Postgres is reachable. It prints the fix command instead of clippy's "no cached data" error.

- Added an idempotent seed migration for the `employee@example.com` local demo admin account (`password`) and documented the credentials in the local setup steps.

- Updated both backend dev watchers to watch `derbe-backend/migrations`, so changed embedded SQLx migrations trigger a rebuild and run at startup.

- Stopped tracking `derbe-backend/.env` and added a documented `derbe-backend/.env.example`. New setup step: `cp .env.example .env` (README updated). `.env` was already in `.gitignore` and `.dockerignore`, so it never reached the image.

- Simplified backend configuration: environment variables only, read with `dotenvy` (loads `.env`) and `envy` (deserializes a flat `Settings` struct). Removed the `config` and `dotenv` crates, `derbe_config.yaml`, the `__` nesting and list separator consts, and `DatabaseSettings` (one secret `DATABASE_URL` replaces five fields and two connection string builders). Integration tests now derive their per test database from `DATABASE_URL` with `PgConnectOptions`, and `spawn_app()` returns just the `TestApp`. Env var names changed: `DATABASE_URL`, `JWT_SECRET`, `APP_ENV`, `APP_HOST`, `APP_PORT`, `CORS_ORIGINS`. Production compose now sets these names, which also fixes its old `DERBE_APPLICATION__*` vars that the unprefixed config source never read.

Before:

```rust
Config::builder()
    .add_source(File::new("derbe_config.yaml", FileFormat::Yaml).required(false))
    .add_source(Environment::default().separator("__").list_separator(",")
        .with_list_parse_key("application.cors_origins").try_parsing(true))
    .build()?.try_deserialize::<Settings>()
```

After:

```rust
dotenvy::dotenv().ok();
envy::from_env()
```

- Added `scripts/rotate_deploy_key.sh`: one command (`ADMIN_SSH=root@<ip> ./scripts/rotate_deploy_key.sh`) that generates a fresh ed25519 key, installs its public half in the deploy user's `authorized_keys`, checks the login works, then stores the private half in the `production` secret `SSH_PRIVATE_KEY`. Fixes the Deploy job failing with `Permission denied (publickey,password)`. Needs server access and repo admin rights, so the repo owner runs it.

- Removed the `[y/N]` confirmation prompt from `deploy_prod.sh`, like RUSTIFY: running the script is the confirmation, it passes `confirm_deploy=true` to the workflow after the local gates and the origin/main sync check.

- Fixed the typos step in `deploy_prod.sh`: it was missing the step label, so `timed` tried `cd typos` and failed instantly, killing the frontend and rust jobs. It now runs `typos derbe-backend/src derbe-frontend/src` from the repo root.

- Disabled Dependabot by commenting out all of `.github/dependabot.yml` (cargo, npm, github-actions, docker) because its PRs triggered the Quality workflow and added noise to Actions. Uncomment the file to re-enable.

- Reshaped the `Deploy` workflow like RUSTIFY: no `quality.yml` call (the gates already run locally in `deploy_prod.sh`), separate `Build backend` and `Build frontend` jobs that run in parallel and can be skipped, then `Deploy`. `deploy_prod.sh` passes `skip_backend_build` and `skip_frontend_build` from its unchanged-since-last-deploy detection, and a skipped service keeps its previous `:<service>-latest` image. The deploy tag is now pushed only after the run succeeds, so a failed deploy no longer marks the commit as deployed.

Before:

```yaml
confirm -> quality (quality.yml) -> build (matrix: backend, frontend) -> deploy
```

After:

```yaml
build_backend  (skip_backend_build)   -\
                                         > deploy
build_frontend (skip_frontend_build)  -/
```

```bash
# deploy_prod.sh
git tag "$TAG_NAME" && git push origin "$TAG_NAME"   # before triggering the workflow
gh workflow run deploy.yml ...
# now
gh workflow run deploy.yml -f skip_backend_build=... -f skip_frontend_build=...
gh run watch "$RUN_ID" --exit-status && git tag "$TAG_NAME" "$SHA" && git push origin "$TAG_NAME"
```

- Grouped frontend screens and leave-specific helpers by feature under `derbe-frontend/src/features/` (`auth`, `dashboard`, `employees`, `leave`, `organisation`, `settings`). Co-located theme mode with its UI toggle and removed the one-line `lib/utils.ts` re-export in favor of direct `cn` imports. Generated bindings remain in `src/bindings`; routes and behavior are unchanged.

- Fixed the `Deploy` workflow failing at `docker/login-action` with `Username and password required`: `PROD_DOCKER_USERNAME` and `PROD_DOCKER_PASSWORD` are `production` environment secrets, but the `build` job did not declare that environment, so it read empty values.

Before:

```yaml
build:
  needs: quality
  runs-on: ubuntu-latest
```

After:

```yaml
build:
  needs: quality
  runs-on: ubuntu-latest
  environment: production
```

- `deploy_prod.sh` moved to the repo root and rewritten after RUSTIFY-APP. It fails fast on the developer machine: local gates (rustfmt auto-commit, SQLx offline check, bindings, parallel frontend and Rust jobs with fail-fast, typos), last `deploy_prod_*` tag diff to skip unchanged parts, timing table, success and failure sounds in `scripts/deploy_sounds/`, `deploy_prod_YYYY/MM/DD_HHhMMmSSs` tag, and a watch on the run matching the deployed commit. Machete and audits are advisory unless `--strict-audit`. `deploy.yml` gains a `confirm_deploy` input and `confirm` job.
- `.githooks/pre-commit` and `pre-push` added, enabled by the root `prepare` script (`core.hooksPath`). `_typos.toml` added.

Before:

```bash
gh workflow run deploy.yml --ref main
sleep 5
run_id="$(gh run list --workflow deploy.yml --limit 1 ...)"  # could pick an older run
```

After:

```bash
git tag "$tag" && git push origin "$tag"
gh workflow run deploy.yml --ref main
gh run list --workflow deploy.yml --event workflow_dispatch --commit "$sha" ...  # polled until found
```

- Removed the `MeResponse` DTO and its `From<CurrentUser>` impl in `auth`: both structs had identical fields, so `GET /me` now serializes `CurrentUser` directly. `LeaveRequestResponse.employee_unid` is now a `Uuid` instead of a `String` (the JSON and the generated TS type are unchanged), enabled by the `uuid-impl` feature of `ts-rs`.

Before:

```rust
impl From<CurrentUser> for MeResponse { /* field by field copy */ }
pub async fn me(..) -> Result<Json<MeResponse>, AppError> { Ok(Json(user.into())) }
employee_unid: value.employee_unid.to_string(),
```

After:

```rust
#[derive(Debug, Serialize)]
pub struct CurrentUser { /* unid, email, role */ }
pub async fn me(..) -> Result<Json<CurrentUser>, AppError> { Ok(Json(user)) }
employee_unid: value.employee_unid,
```

- Production deployment added, see `DEPLOYMENT.md`. Two images (`Dockerfile.backend` with cargo-chef, `Dockerfile.frontend` with nginx-unprivileged), `docker-compose.prod.yml` with `${VAR:?}` guards, host nginx templates (`nginx.prod.conf`, `nginx.prod.bootstrap.conf`), the in-container `nginx.frontend.conf` (all at the repo root) and a manual `deploy.yml` workflow for a Hetzner VPS with certbot. `quality.yml` runs on every PR and is reused by the deploy. `audit.yml`, `cleanup-caches.yml` and `dependabot.yml` added. `scripts/` has `setup_server.sh` and `backup-db.sh`, `deploy_prod.sh` is at the repo root. `rust-toolchain.toml` pins 1.99.1.
- Backend is now deployable from env only. `get_configuration` splits `DERBE_APPLICATION__CORS_ORIGINS` on commas into a list, and `serve` runs `sqlx::migrate!` at startup (new `ServeError::Migration`), so a fresh container on an empty database comes up without a manual migration step. `derbe-backend/.sqlx` is committed for `SQLX_OFFLINE=true` builds.

Before:

```rust
// Environment::with_prefix("DERBE").separator("__")  (a list could not be set from an env var)
// serve(): connect the pool, then start the server (migrations were a manual step)
```

After:

```rust
Environment::with_prefix(ENV_PREFIX)
    .prefix_separator(ENV_PREFIX_SEPARATOR)
    .separator(ENV_NESTING_SEPARATOR)
    .list_separator(ENV_LIST_SEPARATOR)
    .with_list_parse_key(CORS_ORIGINS_KEY)
    .try_parsing(true)
// serve(): sqlx::migrate!("./migrations").run(&pool).await?;
```

- Name uniqueness for divisions, leave types and locations is enforced by the database only. Migrations `0001`, `0002` and `0003` replace `UNIQUE (name)` with a unique index on `lower(name)`, so "Sales" and "sales" conflict. The check-then-insert `ensure_name_available` (racy, and case sensitive) is removed. `AppError::duplicate_name_or_db` maps a unique violation to `DuplicateNamedEntity` (409) and other errors to `DatabaseQueryError`. Local DBs must be recreated with `bash reset_db.sh --yes` from the repo root.

Before:

```rust
ensure_name_available(&dto.name, ctx).await?;
let row = query_as!(...).fetch_one(&ctx.pool).await?;
```

After:

```rust
let row = query_as!(...)
    .fetch_one(&ctx.pool)
    .await
    .map_err(|error| AppError::duplicate_name_or_db(error, ENTITY))?;
```

- Employee emails are validated (`#[validate(email)]`, 400 on bad input instead of a 500 from the DB) and stored trimmed and lowercase. Login lowercases the email too.
- Each repository has a `const ENTITY: &str` instead of repeating the entity name string. This also fixes "LeaveRequest" vs "Leave Request" in leave request error messages (now "Leave Request").
- Integration tests share `leave_type_body` and `employee_body` helpers instead of repeating JSON bodies. New tests cover invalid email, lowercase email storage, and case-insensitive duplicate names.
- `EmployeeCreate.manager_unid` and `EmployeeUpdate.manager_unid` bindings are `string | null` (were `string | undefined`-like, mismatching the API). Frontend: `employeeQueryOptions` takes `string | undefined` and uses `skipToken`, the leave request form no longer sends an undefined employee, and `protected-route.tsx` button has `type="button"`. `pnpm typecheck` and `biome check` are clean.
- Dead code removed: `MyLeaveRequestRaw`, `EmployeeResponse::get_full_name`. `test_helpers.rs` allows `dead_code` because each test crate uses a different subset.

- `leave_types` replaces the two dependent booleans `show_in_others_days` and `default_in_others_days` with one enum `others_days_visibility` (`Hidden`, `Shown`, `ShownByDefault`). The old pair allowed the invalid state "default but not shown". The enum is a Postgres type (`valid_others_days_visibilities`, created in migration `0001`), a Rust enum in `features/leave_types/model.rs`, and a generated TypeScript binding. The create form uses a select instead of two checkboxes. The three independent flags (`can_book_in_future`, `default_in_leave_request`, `show_in_approve_days`) stay booleans. This changes the JSON API and `0001`, so local DBs must be recreated with `bash reset_db.sh --yes` from the repo root.

Before:

```rust
pub show_in_others_days: bool,
pub default_in_others_days: bool,
```

After:

```rust
pub others_days_visibility: OthersDaysVisibility,
```

- `divisions`, `leave_types` and `locations` query their `*Response` DTO directly. The `Division`, `LeaveType` and `Location` models and their `From` impls are removed, because they were field-for-field copies of the DTOs. Their `model.rs` files are deleted. JSON and TypeScript bindings are unchanged.

Before:

```rust
pub async fn list(ctx: &AppState) -> Result<Vec<Division>, AppError> { /* query_as!(Division, ...) */ }

Ok(Json(divisions.into_iter().map(DivisionResponse::from).collect()))
```

After:

```rust
pub async fn list(ctx: &AppState) -> Result<Vec<DivisionResponse>, AppError> { /* query_as!(DivisionResponse, ...) */ }

Ok(Json(divisions))
```

- Employee queries now fill `EmployeeResponse` directly. The `Employee` and `CreatedEmployee` models and their two `From` impls are removed, because they only turned `Uuid` into `String`. `EmployeeResponse` holds `Uuid` fields with `#[ts(type = "string")]`, so the JSON and the TypeScript bindings are unchanged. The queries alias `unid AS employee_unid`.

Before:

```rust
pub async fn list(ctx: &AppState) -> Result<Vec<Employee>, AppError> { /* query_as!(Employee, ...) */ }

employees.into_iter().map(EmployeeResponse::from).collect()
```

After:

```rust
pub async fn list(ctx: &AppState) -> Result<Vec<EmployeeResponse>, AppError> { /* query_as!(EmployeeResponse, ...) */ }

Json(service::list(&state).await?)
```

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
