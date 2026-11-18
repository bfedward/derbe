#!/usr/bin/env bash
# Deploy main to production: fail fast on this machine, then trigger and watch the Deploy workflow.
# Usage: scripts/deploy_prod.sh [--skip-checks]
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

skip_checks=false
for arg in "$@"; do
  case "$arg" in
    --skip-checks) skip_checks=true ;;
    *) echo "Unknown option: $arg" >&2; exit 1 ;;
  esac
done

branch="$(git rev-parse --abbrev-ref HEAD)"
if [ "$branch" != "main" ]; then
  echo "Deploy runs from main, current branch is $branch" >&2
  exit 1
fi
if [ -n "$(git status --porcelain)" ]; then
  echo "Working tree is dirty, commit or stash first" >&2
  exit 1
fi
git fetch origin main --quiet
if [ "$(git rev-parse HEAD)" != "$(git rev-parse origin/main)" ]; then
  echo "Local main differs from origin/main, push or pull first" >&2
  exit 1
fi

# Same gates as .github/workflows/quality.yml, cheapest first, so a broken commit fails here and not in CI.
step() { echo; echo "==> $*"; }

if [ "$skip_checks" = true ]; then
  echo "Skipping local checks (--skip-checks), CI still runs them"
else
  # The integration tests create a throwaway database on the local PostgreSQL.
  if ! (exec 3<>/dev/tcp/127.0.0.1/5432) 2>/dev/null; then
    echo "PostgreSQL is not reachable on 127.0.0.1:5432, start it (see README) or use --skip-checks" >&2
    exit 1
  fi

  step "Backend: cargo fmt --check"
  (cd derbe-backend && cargo fmt --check)

  step "Frontend: typecheck, lint, knip"
  pnpm install --frozen-lockfile
  pnpm check:quality

  step "Backend: cargo clippy"
  (cd derbe-backend && SQLX_OFFLINE=true cargo clippy --all-targets --locked)

  step "Backend: cargo test"
  (cd derbe-backend && SQLX_OFFLINE=true cargo test --locked)

  step "Generated TypeScript bindings are committed"
  (cd derbe-backend && SQLX_OFFLINE=true cargo run --locked --bin export_bindings)
  git diff --exit-code -- derbe-frontend

  step "Frontend: build"
  pnpm build

  echo
  echo "All local checks passed"
fi

read -r -p "Deploy $(git rev-parse --short HEAD) to production? [y/N] " answer
[ "$answer" = "y" ] || { echo "Aborted"; exit 1; }

# Deploy tag: marks what is in production, useful to diff or roll back to.
tag="deploy_$(date +%Y%m%d_%H%M%S)"
git tag "$tag"
git push origin "$tag"

sha="$(git rev-parse HEAD)"
gh workflow run deploy.yml --ref main

# Find the run we just started (matching this commit), not an older one.
run_id=""
for _ in $(seq 1 15); do
  run_id="$(gh run list --workflow deploy.yml --event workflow_dispatch --commit "$sha" --limit 1 \
    --json databaseId,status --jq '[.[] | select(.status != "completed")][0].databaseId // empty')"
  [ -n "$run_id" ] && break
  sleep 2
done
if [ -z "$run_id" ]; then
  echo "Could not find the Deploy run for $sha, check GitHub Actions" >&2
  exit 1
fi

gh run watch "$run_id" --exit-status
