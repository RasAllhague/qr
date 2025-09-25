#!/usr/bin/env bash
set -euo pipefail

echo "[entrypoint] starting container..."

if [[ -n "${DATABASE_URL:-}" ]]; then
  if [[ -d "/app/migration" ]]; then
    echo "[entrypoint] running SeaORM migrations..."
    sea-orm-cli migrate up || {
      echo "[entrypoint] migration failed"; exit 1;
    }
  else
    echo "[entrypoint] no /app/migration directory found, skipping migrations."
  fi
else
  echo "[entrypoint] DATABASE_URL not set, skipping migrations."
fi

echo "[entrypoint] starting API..."
exec /app/bin/api