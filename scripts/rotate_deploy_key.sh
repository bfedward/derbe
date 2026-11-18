#!/usr/bin/env bash
# Create a fresh SSH key for the Deploy workflow, install its public half on the server and
# store the private half in the GitHub `production` environment secret SSH_PRIVATE_KEY.
#
# Run from your machine, not from CI. It needs:
#   - gh logged in with admin rights on the repo (to write an environment secret)
#   - SSH access to the server with a user that can write /home/deploy/.ssh (root or sudo)
#
# Usage:
#   ADMIN_SSH=root@<server-ip> ./scripts/rotate_deploy_key.sh
#   ./scripts/rotate_deploy_key.sh --check    only test the current setup, change nothing
#
# The private key never touches disk outside a temp dir that is wiped on exit.
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

ENVIRONMENT="production"
SECRET_NAME="SSH_PRIVATE_KEY"
DEPLOY_USER="$(gh variable get SERVER_USER --env "$ENVIRONMENT" 2>/dev/null || echo deploy)"
SERVER_IP="$(gh variable get SERVER_IP --env "$ENVIRONMENT")"
CHECK_ONLY=false
[ "${1:-}" != "--check" ] || CHECK_ONLY=true

WORK_DIR="$(mktemp -d)"
trap 'rm -rf "$WORK_DIR"' EXIT
SSH_OPTS=(-o BatchMode=yes -o ConnectTimeout=10 -o StrictHostKeyChecking=accept-new)

echo "Server: $DEPLOY_USER@$SERVER_IP, secret: $ENVIRONMENT/$SECRET_NAME"

if [ "$CHECK_ONLY" = "true" ]; then
    echo "--check: GitHub secret values cannot be read back, so this only tests your own SSH access."
    ssh "${SSH_OPTS[@]}" "${ADMIN_SSH:-$DEPLOY_USER@$SERVER_IP}" "echo ok: ssh works as \$(whoami)"
    exit 0
fi

: "${ADMIN_SSH:?Set ADMIN_SSH to a login that can write authorized_keys of the deploy user, e.g. root@$SERVER_IP}"

echo "Generating a new ed25519 key..."
ssh-keygen -q -t ed25519 -N "" -C "derbe-github-actions-deploy" -f "$WORK_DIR/deploy_key"
PUBLIC_KEY="$(cat "$WORK_DIR/deploy_key.pub")"

echo "Installing the public key for $DEPLOY_USER on the server..."
ssh "${SSH_OPTS[@]}" "$ADMIN_SSH" bash -s -- "$DEPLOY_USER" "$PUBLIC_KEY" <<'REMOTE'
set -euo pipefail
user="$1"; key="$2"
sudo_cmd=""; [ "$(id -u)" -eq 0 ] || sudo_cmd="sudo"
id "$user" >/dev/null 2>&1 || { echo "User $user does not exist: run scripts/setup_server.sh first" >&2; exit 1; }
home="$(getent passwd "$user" | cut -d: -f6)"
$sudo_cmd install -d -m 700 -o "$user" -g "$user" "$home/.ssh"
echo "$key" | $sudo_cmd tee "$home/.ssh/authorized_keys" >/dev/null
$sudo_cmd chown "$user:$user" "$home/.ssh/authorized_keys"
$sudo_cmd chmod 600 "$home/.ssh/authorized_keys"
REMOTE

echo "Verifying the new key logs in as $DEPLOY_USER before touching the GitHub secret..."
ssh "${SSH_OPTS[@]}" -i "$WORK_DIR/deploy_key" -o IdentitiesOnly=yes "$DEPLOY_USER@$SERVER_IP" "echo ok: key accepted as \$(whoami)"

echo "Updating the GitHub secret..."
gh secret set "$SECRET_NAME" --env "$ENVIRONMENT" < "$WORK_DIR/deploy_key"

echo "Done. Trigger the deploy with ./deploy_prod.sh"
