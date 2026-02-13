#!/usr/bin/env bash
set -euo pipefail

NAME="${NAME:-gateflow}"
RUN_DIR="${RUN_DIR:-./run}"
PID_FILE="$RUN_DIR/$NAME.pid"
LOG_FILE="$RUN_DIR/$NAME.log"

mkdir -p "$RUN_DIR"

# 已经在跑就不重复启动
if [[ -f "$PID_FILE" ]]; then
  PID="$(cat "$PID_FILE" || true)"
  if [[ -n "${PID:-}" ]] && kill -0 "$PID" 2>/dev/null; then
    echo "[$NAME] already running (pid=$PID)"
    exit 0
  else
    echo "[$NAME] stale pid file found, removing: $PID_FILE"
    rm -f "$PID_FILE"
  fi
fi

RUST_LOG=debug cargo run -p infra --bin infra_app

PID="$!"
echo "$PID" > "$PID_FILE"

echo "[$NAME] started (pid=$PID)"
echo "log: $LOG_FILE"
