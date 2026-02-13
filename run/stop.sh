#!/usr/bin/env bash
set -euo pipefail

NAME="${NAME:-gateflow}"
RUN_DIR="${RUN_DIR:-./run}"
PID_FILE="$RUN_DIR/$NAME.pid"

TIMEOUT_SEC="${TIMEOUT_SEC:-8}"

if [[ ! -f "$PID_FILE" ]]; then
  echo "[$NAME] not running (no pid file: $PID_FILE)"
  exit 0
fi

PID="$(cat "$PID_FILE" || true)"
if [[ -z "${PID:-}" ]]; then
  echo "[$NAME] pid file empty, removing: $PID_FILE"
  rm -f "$PID_FILE"
  exit 0
fi

if ! kill -0 "$PID" 2>/dev/null; then
  echo "[$NAME] not running (stale pid=$PID), removing: $PID_FILE"
  rm -f "$PID_FILE"
  exit 0
fi

echo "[$NAME] stopping pid=$PID (SIGTERM)..."
kill -TERM "$PID" 2>/dev/null || true

# 等待优雅退出
for ((i=0; i<TIMEOUT_SEC*10; i++)); do
  if ! kill -0 "$PID" 2>/dev/null; then
    echo "[$NAME] stopped"
    rm -f "$PID_FILE"
    exit 0
  fi
  sleep 0.1
done

echo "[$NAME] still running after ${TIMEOUT_SEC}s, killing (SIGKILL)..."
kill -KILL "$PID" 2>/dev/null || true
rm -f "$PID_FILE"
echo "[$NAME] killed"
