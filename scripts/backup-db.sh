#!/usr/bin/env bash
# Dump the production database to ~/derbe/backups, keep the last 14 dumps.
# Run on the server. Schedule with cron: 0 3 * * * ~/derbe/backup-db.sh
set -euo pipefail

DEPLOY_DIR="$HOME/derbe"
BACKUP_DIR="$DEPLOY_DIR/backups"
KEEP_COUNT=14

cd "$DEPLOY_DIR"
mkdir -p "$BACKUP_DIR"
set -a; . ./.env; set +a

dump_file="$BACKUP_DIR/derbe-$(date +%Y%m%d-%H%M%S).sql.gz"
docker compose -f docker-compose.prod.yml --env-file .env exec -T db \
  pg_dump -U "${POSTGRES_USER:-derbe}" "${POSTGRES_DATABASE:-derbe}" | gzip > "$dump_file"

ls -1t "$BACKUP_DIR"/derbe-*.sql.gz | tail -n +$((KEEP_COUNT + 1)) | xargs -r rm --
echo "backup written: $dump_file"
