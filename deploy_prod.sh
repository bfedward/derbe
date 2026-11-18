#!/usr/bin/env bash
# Deploy main to production through the Deploy workflow (Hetzner VPS).
# Fails fast on this machine: the same gates as .github/workflows/quality.yml run here first.
#
# Usage: ./deploy_prod.sh [flags]
#   --skip-preflight  allow a dirty working tree (auto-fix commits are disabled)
#   --skip-all        skip every check (typos, tests, audits, builds)
#   --skip-audit      skip cargo audit and pnpm audit
#   --skip-clippy     skip the Rust quality gates (machete, clippy)
#   --skip-sqlx       skip the SQLx offline data check
#   --force-checks    ignore "unchanged since the last deploy tag" shortcuts
#   --strict-audit    make cargo audit, pnpm audit and machete blocking (advisory by default)
set -eo pipefail

cd "$(git rev-parse --show-toplevel)"

SOUND_DIR="$PWD/scripts/deploy_sounds"
SOUND_SUCCESS="$SOUND_DIR/deploy_success.mp3"
SOUND_FAILURE="$SOUND_DIR/deploy_failed.mp3"
SOUND_VOLUME="0.5"
JOB_LOG_DIR=""
TIMINGS_FILE=$(mktemp)
export TIMINGS_FILE
TAIL_PID=""
DEPLOY_TAG_PUSHED=false

on_exit() {
    local exit_code=$?
    print_timings || true
    [ -z "$TAIL_PID" ] || kill "$TAIL_PID" 2>/dev/null || true
    [ -z "$JOB_LOG_DIR" ] || rm -rf "$JOB_LOG_DIR" || true
    rm -f "$TIMINGS_FILE" || true
    if command -v afplay >/dev/null 2>&1; then
        if [ "$exit_code" -ne 0 ]; then
            afplay -v "$SOUND_VOLUME" "$SOUND_FAILURE" >/dev/null 2>&1 || true
        elif [ "$DEPLOY_TAG_PUSHED" = true ]; then
            afplay -v "$SOUND_VOLUME" "$SOUND_SUCCESS" >/dev/null 2>&1 || true
        fi
    fi
    return "$exit_code"
}
trap on_exit EXIT

SCRIPT_STARTED_AT=$(date +%s)

timing_record() { # $1=start_epoch $2=group $3=step $4=seconds $5=result
    echo "$1|$2|$3|$4|$5" >>"$TIMINGS_FILE"
}

timed() { # $1=group $2=step $3=dir $4...=command
    local group="$1" step="$2" dir="$3" started finished rc
    shift 3
    started=$(date +%s)
    if (cd "$dir" && "$@"); then rc=0; else rc=$?; fi
    finished=$(date +%s)
    if [ "$rc" -eq 0 ]; then
        timing_record "$started" "$group" "$step" "$((finished - started))" "OK"
    else
        timing_record "$started" "$group" "$step" "$((finished - started))" "FAIL"
    fi
    return "$rc"
}

# Same as timed, but a failure only warns unless --strict-audit is set.
timed_advisory() { # $1=group $2=step $3=dir $4...=command
    local group="$1" step="$2" dir="$3" started finished rc
    shift 3
    started=$(date +%s)
    if (cd "$dir" && "$@"); then rc=0; else rc=$?; fi
    finished=$(date +%s)
    if [ "$rc" -eq 0 ]; then
        timing_record "$started" "$group" "$step" "$((finished - started))" "OK"
    elif [ "$STRICT_AUDIT" = true ]; then
        timing_record "$started" "$group" "$step" "$((finished - started))" "FAIL"
        return "$rc"
    else
        echo "⚠️  $step reported problems (advisory, use --strict-audit to block)"
        timing_record "$started" "$group" "$step" "$((finished - started))" "WARN"
    fi
    return 0
}

print_timings() {
    [ -s "$TIMINGS_FILE" ] || return 0
    echo ""
    echo "⏱  Deploy timings"
    printf '+------------+--------------------------+--------+---------\n'
    printf '| %-10s | %-24s | %6s | %s\n' "Group" "Step" "Time" "Result"
    printf '+------------+--------------------------+--------+---------\n'
    sort -t'|' -k1,1n "$TIMINGS_FILE" | while IFS='|' read -r _started group step seconds result; do
        case "$result" in OK) mark="✅" ;; WARN) mark="⚠️ " ;; FAIL) mark="❌" ;; *) mark="🛑" ;; esac
        printf '| %-10s | %-24s | %5ss | %s %s\n' "$group" "$step" "$seconds" "$mark" "$result"
    done
    printf '+------------+--------------------------+--------+---------\n'
    echo "Total wall time: $(($(date +%s) - SCRIPT_STARTED_AT))s"
}

push_main() {
    git pull --rebase --autostash origin main
    git push origin main
}

FORCE=false
SKIP_ALL=false
SKIP_AUDIT=false
SKIP_CLIPPY=false
SKIP_SQLX=false
FORCE_CHECKS=false
STRICT_AUDIT=false
for arg in "$@"; do
    case "$arg" in
        --skip-preflight) FORCE=true ;;
        --skip-all) SKIP_ALL=true; SKIP_AUDIT=true; SKIP_CLIPPY=true; SKIP_SQLX=true ;;
        --skip-audit) SKIP_AUDIT=true ;;
        --skip-clippy) SKIP_CLIPPY=true ;;
        --skip-sqlx) SKIP_SQLX=true ;;
        --force-checks) FORCE_CHECKS=true ;;
        --strict-audit) STRICT_AUDIT=true ;;
        *) echo "❌ Unknown option: $arg" >&2; exit 1 ;;
    esac
done

echo "🔍 Pre-flight checks..."
BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [ "$BRANCH" != "main" ]; then
    echo "❌ Must be on main branch (currently on '$BRANCH')"
    exit 1
fi

TREE_CLEAN=true
if [ -n "$(git status --porcelain)" ]; then
    TREE_CLEAN=false
    if [ "$FORCE" = "true" ]; then
        echo "⚠️  Uncommitted changes: checks run on your working tree, deploy uses the last pushed commit (--skip-preflight)"
    else
        echo "❌ Working tree is dirty. Commit or stash first."
        exit 1
    fi
fi

git fetch origin main --quiet

# Work out what changed since the last deploy so unchanged parts skip their checks.
LAST_DEPLOY_TAG=$(git tag -l 'deploy_prod_*' --sort=-creatordate | head -1)
BACKEND_UNCHANGED=false
BACKEND_DEPENDENCIES_UNCHANGED=false
FRONTEND_UNCHANGED=false
FRONTEND_DEPENDENCIES_UNCHANGED=false

if [ -n "$LAST_DEPLOY_TAG" ] && [ "$FORCE_CHECKS" != "true" ]; then
    if git diff --quiet "$LAST_DEPLOY_TAG" HEAD -- derbe-backend Dockerfile.backend .dockerignore rust-toolchain.toml; then
        BACKEND_UNCHANGED=true
    fi
    if git diff --quiet "$LAST_DEPLOY_TAG" HEAD -- derbe-backend/Cargo.toml derbe-backend/Cargo.lock; then
        BACKEND_DEPENDENCIES_UNCHANGED=true
    fi
    if git diff --quiet "$LAST_DEPLOY_TAG" HEAD -- derbe-frontend Dockerfile.frontend nginx.frontend.conf .dockerignore package.json pnpm-lock.yaml pnpm-workspace.yaml .npmrc; then
        FRONTEND_UNCHANGED=true
    fi
    if git diff --quiet "$LAST_DEPLOY_TAG" HEAD -- derbe-frontend/package.json package.json pnpm-lock.yaml; then
        FRONTEND_DEPENDENCIES_UNCHANGED=true
    fi
fi
[ -z "$LAST_DEPLOY_TAG" ] || echo "ℹ️  Last deploy tag: $LAST_DEPLOY_TAG"

# The integration tests and the SQLx check need the local PostgreSQL (see README).
needs_postgres=false
if [ "$SKIP_ALL" != "true" ] && [ "$BACKEND_UNCHANGED" != "true" ]; then needs_postgres=true; fi
if [ "$SKIP_SQLX" != "true" ] && [ "$BACKEND_UNCHANGED" != "true" ]; then needs_postgres=true; fi
if [ "$needs_postgres" = "true" ] && ! (exec 3<>/dev/tcp/127.0.0.1/5432) 2>/dev/null; then
    echo "❌ PostgreSQL is not reachable on 127.0.0.1:5432. Start it (docker start derbe-postgres) or use --skip-all."
    exit 1
fi

export SQLX_OFFLINE=true

if [ "$BACKEND_UNCHANGED" = "true" ]; then
    echo "⏭️  Skipping cargo fmt, backend unchanged since $LAST_DEPLOY_TAG"
else
    echo "🎨 Formatting derbe-backend..."
    if [ "$TREE_CLEAN" = "true" ]; then
        timed preflight "rustfmt (format)" derbe-backend cargo fmt --all
        if ! git diff --quiet -- derbe-backend; then
            echo "📝 rustfmt changed derbe-backend, committing and pushing..."
            git add derbe-backend/
            git commit -m "chore: apply rustfmt"
            push_main
        fi
    else
        timed preflight "rustfmt (check)" derbe-backend cargo fmt --all --check
    fi
    echo "✅ derbe-backend formatted"
fi

if [ "$SKIP_SQLX" = "true" ]; then
    echo "⏭️  Skipping cargo sqlx prepare"
elif [ "$BACKEND_UNCHANGED" = "true" ]; then
    echo "⏭️  Skipping cargo sqlx prepare, backend unchanged since $LAST_DEPLOY_TAG"
else
    echo "🗄️  Checking SQLx offline data (derbe-backend)..."
    # Needs the live database: unset SQLX_OFFLINE for this step only.
    if SQLX_OFFLINE=false timed preflight "sqlx prepare --check" derbe-backend cargo sqlx prepare --check -- --all-targets; then
        echo "✅ SQLx offline data is up to date"
    elif [ "$TREE_CLEAN" = "true" ]; then
        echo "⚠️  SQLx offline data is stale, refreshing..."
        SQLX_OFFLINE=false timed preflight "sqlx prepare (refresh)" derbe-backend cargo sqlx prepare -- --all-targets
        if [ -n "$(git status --porcelain -- derbe-backend/.sqlx)" ]; then
            echo "📝 .sqlx files changed, committing and pushing..."
            git add derbe-backend/.sqlx/
            git commit -m "chore: update derbe-backend .sqlx query cache"
            push_main
        fi
        echo "✅ SQLx offline data refreshed"
    else
        echo "❌ SQLx offline data is stale. Run: cd derbe-backend && cargo sqlx prepare -- --all-targets"
        exit 1
    fi
fi

# Bindings are generated by Rust and read by the frontend, so check them before the parallel jobs start.
if [ "$SKIP_ALL" = "true" ]; then
    echo "⏭️  Skipping TypeScript bindings check (--skip-all)"
elif [ "$BACKEND_UNCHANGED" = "true" ]; then
    echo "⏭️  Skipping TypeScript bindings check, backend unchanged since $LAST_DEPLOY_TAG"
else
    echo "🔗 Checking generated TypeScript bindings..."
    timed preflight "export_bindings" derbe-backend cargo run --locked --bin export_bindings
    if ! git diff --quiet -- derbe-frontend; then
        echo "❌ Generated bindings differ from the committed ones. Review and commit derbe-frontend/src/bindings."
        git --no-pager diff --stat -- derbe-frontend
        exit 1
    fi
    echo "✅ Bindings are committed"
fi

JOB_LOG_DIR=$(mktemp -d)
PIDS=()
NAMES=()
JOB_STARTS=()

start_job() { # $1=name $2...=function run in a subshell, output captured to a log
    local name="$1"; shift
    JOB_STARTS+=("$(date +%s)")
    (
        job_started=$(date +%s)
        if "$@"; then job_rc=0; else job_rc=$?; fi
        timing_record "$job_started" "$name" "(job total)" "$(($(date +%s) - job_started))" "$([ "$job_rc" -eq 0 ] && echo OK || echo FAIL)"
        exit "$job_rc"
    ) >"$JOB_LOG_DIR/$name.log" 2>&1 &
    PIDS+=("$!")
    NAMES+=("$name")
}

frontend_job() {
    set -e
    echo "📦 Checking pnpm lockfile..."
    timed frontend "pnpm install" . pnpm install --frozen-lockfile
    echo "✅ Lockfile is up to date"

    echo "🧭 Typecheck, biome, knip..."
    timed frontend "check:quality" . pnpm check:quality
    echo "✅ Frontend static checks OK"

    echo "🏗️  Building derbe-frontend..."
    timed frontend "vite build" . pnpm build
    echo "✅ Frontend build passed"

    if [ "$SKIP_AUDIT" = "true" ]; then
        echo "⏭️  Skipping pnpm audit"
    elif [ "$FRONTEND_DEPENDENCIES_UNCHANGED" = "true" ]; then
        echo "⏭️  Skipping pnpm audit, dependencies unchanged since $LAST_DEPLOY_TAG"
    else
        echo "🔒 pnpm audit (production dependencies, high and above)..."
        timed_advisory frontend "pnpm audit" . pnpm audit --prod --audit-level high
    fi
}

rust_job() {
    set -e
    if [ "$SKIP_CLIPPY" = "true" ]; then
        echo "⏭️  Skipping Rust quality gates (--skip-clippy)"
    elif [ "$BACKEND_UNCHANGED" = "true" ]; then
        echo "⏭️  Skipping Rust quality gates, backend unchanged since $LAST_DEPLOY_TAG"
    else
        echo "🎨 Checking rustfmt (derbe-backend)..."
        timed rust rustfmt derbe-backend cargo fmt --all --check
        echo "✅ rustfmt OK"

        echo "🧹 cargo machete (unused deps)..."
        timed_advisory rust machete derbe-backend cargo machete
    fi

    if [ "$SKIP_AUDIT" = "true" ]; then
        echo "⏭️  Skipping cargo audit"
    elif [ "$BACKEND_DEPENDENCIES_UNCHANGED" = "true" ]; then
        echo "⏭️  Skipping cargo audit, dependencies unchanged since $LAST_DEPLOY_TAG"
    else
        echo "🔒 cargo audit (derbe-backend)..."
        timed_advisory rust "cargo audit" derbe-backend cargo audit
    fi

    if [ "$SKIP_CLIPPY" != "true" ] && [ "$BACKEND_UNCHANGED" != "true" ]; then
        # Same as CI: no -D warnings yet, existing code still has clippy warnings.
        echo "📎 Running clippy on derbe-backend (--all-targets --locked)..."
        timed rust clippy derbe-backend cargo clippy --all-targets --locked
        echo "✅ Clippy OK"
    fi

    if [ "$SKIP_ALL" = "true" ]; then
        echo "⏭️  Skipping derbe-backend tests (--skip-all)"
    elif [ "$BACKEND_UNCHANGED" = "true" ]; then
        echo "⏭️  No changes in derbe-backend since $LAST_DEPLOY_TAG, auto-skipping tests"
    else
        echo "🧪 Running derbe-backend tests..."
        # ts-rs also exports bindings during tests: send them to a temp dir so they cannot race the frontend job.
        timed rust "cargo test" derbe-backend env TS_RS_EXPORT_DIR="$JOB_LOG_DIR/ts-bindings" cargo test --locked
        echo "✅ All tests passed"
    fi
}

if [ "$SKIP_ALL" = "true" ]; then
    echo "⏭️  Skipping typos (--skip-all)"
else
    start_job typos timed typos . typos derbe-backend/src derbe-frontend/src
fi

if [ "$SKIP_ALL" = "true" ]; then
    echo "⏭️  Skipping frontend checks and build (--skip-all)"
elif [ "$FRONTEND_UNCHANGED" = "true" ]; then
    echo "⏭️  Skipping frontend checks and build, frontend unchanged since $LAST_DEPLOY_TAG"
else
    start_job frontend frontend_job
fi

start_job rust rust_job

tail -f "$JOB_LOG_DIR"/*.log &
TAIL_PID=$!

kill_tree() { # $1=pid, kills children first
    local child
    for child in $(pgrep -P "$1" 2>/dev/null); do kill_tree "$child"; done
    kill "$1" 2>/dev/null || true
}

POLL_INTERVAL_SECONDS=1
FAILED=false
DONE=()
while [ "${#DONE[@]}" -lt "${#PIDS[@]}" ] && [ "$FAILED" != true ]; do
    for i in "${!PIDS[@]}"; do
        case " ${DONE[*]} " in *" $i "*) continue ;; esac
        kill -0 "${PIDS[$i]}" 2>/dev/null && continue
        DONE+=("$i")
        if wait "${PIDS[$i]}"; then
            echo "✅ ${NAMES[$i]} job done"
        else
            echo "❌ ${NAMES[$i]} job failed"
            FAILED=true
        fi
    done
    [ "$FAILED" = true ] || sleep "$POLL_INTERVAL_SECONDS"
done
if [ "$FAILED" = true ]; then
    for i in "${!PIDS[@]}"; do
        case " ${DONE[*]} " in *" $i "*) continue ;; esac
        echo "🛑 Killing ${NAMES[$i]} job (fail fast)"
        kill_tree "${PIDS[$i]}"
        timing_record "${JOB_STARTS[$i]}" "${NAMES[$i]}" "(job total)" "$(($(date +%s) - JOB_STARTS[i]))" "KILLED"
        wait "${PIDS[$i]}" 2>/dev/null || true
    done
fi
kill "$TAIL_PID" 2>/dev/null || true
[ "$FAILED" != true ] || { echo "❌ one or more checks failed"; exit 1; }

# The workflow deploys origin/main, so what was checked here must be what is pushed.
git fetch origin main --quiet
if [ "$(git rev-parse HEAD)" != "$(git rev-parse origin/main)" ]; then
    echo "❌ Local main differs from origin/main. Push or pull first, the workflow deploys origin/main."
    exit 1
fi

read -r -p "Deploy $(git rev-parse --short HEAD) to production? [y/N] " answer
[ "$answer" = "y" ] || { echo "Aborted"; exit 1; }

TAG_NAME="deploy_prod_$(date +'%Y/%m/%d_%Hh%Mm%Ss')"
git tag "$TAG_NAME"
git push origin "$TAG_NAME"
DEPLOY_TAG_PUSHED=true

SHA="$(git rev-parse HEAD)"
gh workflow run deploy.yml --ref main -f confirm_deploy="true"

# Find the run we just started (matching this commit), not an older one.
RUN_ID=""
for _ in $(seq 1 15); do
    RUN_ID="$(gh run list --workflow deploy.yml --event workflow_dispatch --commit "$SHA" --limit 1 \
        --json databaseId,status --jq '[.[] | select(.status != "completed")][0].databaseId // empty')"
    [ -n "$RUN_ID" ] && break
    sleep 2
done

echo "✅ derbe production deploy triggered!"
echo "   Tag:      $TAG_NAME"
echo "   Track at: https://github.com/$(gh repo view --json nameWithOwner -q .nameWithOwner)/actions"
if [ -z "$RUN_ID" ]; then
    echo "❌ Could not find the Deploy run for $SHA, check GitHub Actions" >&2
    exit 1
fi

gh run watch "$RUN_ID" --exit-status
