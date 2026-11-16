#!/usr/bin/env bash
# Drops the local database, recreates it and runs all migrations.
# Local development only. Run from the repository root: bash reset_db.sh --yes
set -euo pipefail

# .env and migrations live in derbe-backend, whatever the caller's directory.
cd "$(dirname "${BASH_SOURCE[0]}")/derbe-backend"

if [[ "${1:-}" != "--yes" ]]; then
    echo "Refusing to reset. Run: bash reset_db.sh --yes"
    exit 1
fi

if [[ -f .env ]]; then
    set -a
    # shellcheck disable=SC1091
    source .env
    set +a
fi

: "${DATABASE_URL:?DATABASE_URL must be set (see .env)}"

# postgres://user:password@host:port/derbe -> database name "derbe", admin URL ".../postgres"
DB_NAME="${DATABASE_URL##*/}"
ADMIN_URL="${DATABASE_URL%/*}/postgres"

case "$ADMIN_URL" in
    *127.0.0.1* | *localhost*) ;;
    *)
        echo "Refusing to reset a non-local database: $DB_NAME"
        exit 1
        ;;
esac

psql -v ON_ERROR_STOP=1 "$ADMIN_URL" \
    -c "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '$DB_NAME';"
psql -v ON_ERROR_STOP=1 "$ADMIN_URL" -c "DROP DATABASE IF EXISTS \"$DB_NAME\";"
psql -v ON_ERROR_STOP=1 "$ADMIN_URL" -c "CREATE DATABASE \"$DB_NAME\";"

sqlx migrate run --source migrations
echo "Database $DB_NAME recreated and migrated."
