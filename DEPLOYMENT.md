# Deployment

Derbe runs on one Hetzner VPS with Docker Compose behind the host nginx, with TLS from certbot.
Deploys are triggered by hand (GitHub Actions `Deploy` workflow) and always run the quality checks first.

```
internet
   |
   v
host nginx :443 (certbot TLS)
   |-- /      -> 127.0.0.1:8080  frontend container (static SPA, nginx-unprivileged)
   '-- /api/  -> 127.0.0.1:8001  backend container (axum, /api prefix stripped)
                                    |
                                    v
                                 db container (postgres 18, named volume)
```

The SPA and the API share one origin, so the session cookie (`Secure`, `SameSite=Lax`) needs no CORS tricks.
The backend applies SQL migrations itself at startup.

## One-time setup

### 1. Buy the server and the domain

- Hetzner Cloud VPS, Ubuntu 24.04, at least 2 vCPU / 4 GB RAM.
- DNS: an `A` record from your domain to the server IP.

### 2. Create a deploy SSH key (on your laptop)

```bash
ssh-keygen -t ed25519 -f derbe_deploy -C github-actions -N ""
```

`derbe_deploy.pub` goes to the server, `derbe_deploy` (private) goes to GitHub as a secret.

### 3. Prepare the server (as root)

```bash
scp scripts/setup_server.sh root@SERVER_IP:/root/
ssh root@SERVER_IP 'DEPLOY_PUBLIC_KEY="$(cat)" bash /root/setup_server.sh' < derbe_deploy.pub
```

This installs Docker, nginx and certbot, creates the `deploy` user (limited sudo), opens only SSH and web ports in `ufw`, and schedules the nightly database backup.

### 4. GitHub settings

Settings > Environments > create `production` (optionally add yourself as required reviewer). 

(Add POSTGRES_DATABASE and POSTGRES_USER)

| Kind     | Name                    | Value                                             |
|----------|-------------------------|---------------------------------------------------|
| secret   | `SSH_PRIVATE_KEY`       | content of `derbe_deploy`                         |
| secret   | `PROD_DOCKER_USERNAME`  | Docker Hub username                               |
| secret   | `PROD_DOCKER_PASSWORD`  | Docker Hub access token (read/write)              |
| secret   | `POSTGRES_PASSWORD`     | long random string (`openssl rand -hex 32`)       |
| secret   | `JWT_SECRET`            | long random string (`openssl rand -hex 48`)       |
| secret   | `RESEND_API_KEY`        | content of `derbe-production`     |
| variable | `SERVER_IP`             | server IPv4                                       |
| variable | `PROD_DOCKER_REPOSITORY`| Docker Hub repository name, e.g. `derbe`          |
| variable | `DERBE_DOMAIN`          | e.g. `derbe.example.com`                          |
| variable | `LETSENCRYPT_EMAIL`     | email for certificate expiry notices              |
| variable | `SERVER_USER`           | optional, defaults to `deploy`                    |

The Docker Hub repository can be private. Both images live in it, tagged `backend-<sha>` and `frontend-<sha>`.

## Deploy

```bash
./deploy_prod.sh
```

It checks you are on `main`, then runs the same gates as `quality.yml` on your machine so a broken commit fails in seconds instead of after a CI round trip: rustfmt (auto-committed if the tree is clean), SQLx offline data, TypeScript bindings, then frontend (pnpm install, biome, knip, build) and Rust (clippy, tests) in parallel with fail-fast, plus typos. Parts unchanged since the last `deploy_prod_*` tag skip their checks. It needs PostgreSQL on `127.0.0.1:5432` for the SQLx check and tests. Then it checks local `main` equals `origin/main`, asks for confirmation, pushes a `deploy_prod_YYYY/MM/DD_HHhMMmSSs` tag, runs the `Deploy` workflow with `confirm_deploy=true` and watches that run. CI runs the gates again as the authoritative check. A timing table and a sound (`scripts/deploy_sounds/`) close the run.

Flags: `--skip-preflight` (allow a dirty tree), `--skip-all`, `--skip-audit`, `--skip-clippy`, `--skip-sqlx`, `--force-checks` (ignore the last-tag shortcuts), `--strict-audit` (machete, cargo audit and pnpm audit block; advisory by default).

Git hooks live in `.githooks/` and are enabled by `pnpm install` (`prepare` sets `core.hooksPath`). `pre-commit` runs rustfmt and typos on staged files, `pre-push` runs typecheck, biome, knip, rustfmt and clippy for what changed.
Or use GitHub: Actions > Deploy > Run workflow.

Pipeline:

```
quality (fmt, clippy, tests, bindings, biome, knip, build)
   -> build + push backend and frontend images (parallel, layer cache)
   -> copy compose + nginx files, write ~/derbe/.env from secrets
   -> pull images, swap backend and frontend only (db untouched)
   -> first run only: certbot issues the certificate
   -> smoke test https://DOMAIN/health and https://DOMAIN/ (retries, logs on failure)
```

The first deploy takes a few minutes longer (cold build, certificate).

## Rollback

Every deploy keeps the previous `.env` (it holds the previous image tags) and never prunes tagged images.

```bash
ssh deploy@SERVER_IP
cd ~/derbe
cp .env.previous .env
docker compose -f docker-compose.prod.yml --env-file .env up -d --no-deps backend frontend
```

Migrations are not reverted. Write migrations so the previous version still works against the new schema (additive first, destructive later).

## Backups

`~/derbe/backup-db.sh` runs nightly at 03:00 and keeps the last 14 gzipped dumps in `~/derbe/backups`. Copy them off the server regularly (Hetzner snapshots or `rsync`). Restore:

```bash
gunzip -c backups/derbe-YYYYMMDD-HHMMSS.sql.gz | docker compose -f docker-compose.prod.yml --env-file .env exec -T db psql -U derbe derbe
```

## Useful commands (on the server)

```bash
cd ~/derbe
docker compose -f docker-compose.prod.yml --env-file .env ps
docker compose -f docker-compose.prod.yml --env-file .env logs -f backend
sudo nginx -t && sudo systemctl reload nginx
```

If email notifications fail to send, this query shows the stuck rows:
```sql
SELECT id, event_key, attempt_count, next_attempt_at, last_error
FROM email_outbox
WHERE sent_at IS NULL
ORDER BY id;
```

After fixing the cause, this makes a row retry straight away:
```sql
UPDATE email_outbox SET attempt_count = 0, next_attempt_at = now(), last_error = NULL WHERE id = 1;
```

## Changing SQL queries

The Docker and CI builds compile with `SQLX_OFFLINE=true`. After changing a `query_as!` query or a migration, run `pnpm sqlx:prepare` (or `cd derbe-backend` && `cargo sqlx prepare -- --all-targets`) and commit .sqlx. The pre-push hook checks this when Postgres is running.

## CI overview

| Workflow             | Trigger                       | What it does                                              |
|----------------------|-------------------------------|-----------------------------------------------------------|
| `quality.yml`        | PR, push to main, reused      | rustfmt, clippy, tests (Postgres), bindings, biome, knip, build, Docker build check |
| `deploy.yml`         | manual                        | quality, build and push images, deploy, smoke test        |
| `audit.yml`          | weekly, lockfile PRs          | `cargo audit`, `pnpm audit`                               |
| `cleanup-caches.yml` | PR closed                     | frees that PR's Actions caches                            |
| `dependabot.yml`     | weekly                        | cargo, npm, Actions and Docker updates                    |
