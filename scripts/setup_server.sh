#!/usr/bin/env bash
# One-time Hetzner VPS setup (Ubuntu 24.04). Run as root:
#   DEPLOY_PUBLIC_KEY="ssh-ed25519 AAAA... github-actions" bash setup_server.sh
set -euo pipefail

DEPLOY_USER="deploy"
DEPLOY_DIR_NAME="derbe"
: "${DEPLOY_PUBLIC_KEY:?Set DEPLOY_PUBLIC_KEY to the public half of the GitHub Actions SSH key}"

export DEBIAN_FRONTEND=noninteractive
apt-get update
apt-get install -y ca-certificates curl nginx certbot python3-certbot-nginx ufw cron

# Docker from the official convenience script (includes the compose plugin).
command -v docker >/dev/null || curl -fsSL https://get.docker.com | sh

id "$DEPLOY_USER" >/dev/null 2>&1 || useradd --create-home --shell /bin/bash "$DEPLOY_USER"
usermod -aG docker "$DEPLOY_USER"
install -d -m 700 -o "$DEPLOY_USER" -g "$DEPLOY_USER" "/home/$DEPLOY_USER/.ssh"
echo "$DEPLOY_PUBLIC_KEY" > "/home/$DEPLOY_USER/.ssh/authorized_keys"
chown "$DEPLOY_USER:$DEPLOY_USER" "/home/$DEPLOY_USER/.ssh/authorized_keys"
chmod 600 "/home/$DEPLOY_USER/.ssh/authorized_keys"
install -d -o "$DEPLOY_USER" -g "$DEPLOY_USER" "/home/$DEPLOY_USER/$DEPLOY_DIR_NAME"

# The deploy user may only run the exact commands the workflow needs.
cat > /etc/sudoers.d/derbe-deploy <<SUDOERS
$DEPLOY_USER ALL=(root) NOPASSWD: /usr/sbin/nginx -t, /usr/bin/systemctl reload nginx, /usr/bin/certbot certonly *, /usr/bin/test -f /etc/letsencrypt/live/*
SUDOERS
chmod 440 /etc/sudoers.d/derbe-deploy
visudo -cf /etc/sudoers.d/derbe-deploy

# nginx serves whatever the deploy workflow symlinks as nginx.active.conf.
cat > /etc/nginx/sites-available/derbe <<NGINX
include /home/$DEPLOY_USER/$DEPLOY_DIR_NAME/nginx.active.conf;
NGINX
ln -sf /etc/nginx/sites-available/derbe /etc/nginx/sites-enabled/derbe
rm -f /etc/nginx/sites-enabled/default
# Placeholder so nginx starts before the first deploy.
echo "# replaced by the deploy workflow" > "/home/$DEPLOY_USER/$DEPLOY_DIR_NAME/nginx.active.conf"
chown "$DEPLOY_USER:$DEPLOY_USER" "/home/$DEPLOY_USER/$DEPLOY_DIR_NAME/nginx.active.conf"
nginx -t && systemctl enable --now nginx

# Firewall: SSH + web only. App ports are bound to 127.0.0.1 anyway.
ufw allow OpenSSH
ufw allow "Nginx Full"
ufw --force enable

# Nightly database backup at 03:00.
( crontab -u "$DEPLOY_USER" -l 2>/dev/null | grep -v backup-db.sh || true
  echo "0 3 * * * /home/$DEPLOY_USER/$DEPLOY_DIR_NAME/backup-db.sh >> /home/$DEPLOY_USER/$DEPLOY_DIR_NAME/backup.log 2>&1" ) | crontab -u "$DEPLOY_USER" -

echo "Server ready. Point the DNS A record of your domain to this IP, then run the Deploy workflow."
