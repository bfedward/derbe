#!/usr/bin/env bash
# Trigger the production deploy workflow for the current commit on main.
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

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

read -r -p "Deploy $(git rev-parse --short HEAD) to production? [y/N] " answer
[ "$answer" = "y" ] || { echo "Aborted"; exit 1; }

gh workflow run deploy.yml --ref main
sleep 5
run_id="$(gh run list --workflow deploy.yml --limit 1 --json databaseId --jq '.[0].databaseId')"
gh run watch "$run_id" --exit-status
