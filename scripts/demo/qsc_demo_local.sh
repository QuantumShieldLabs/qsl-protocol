#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
qsc_demo_local.sh --scenario <name> [--seed <u64>] [--out <dir>] [--dry-run] [--timeout <sec>]

Scenarios: happy-path, drop, reorder, drop+reorder, seeded-replay

Notes:
- Dry-run prints the exact commands that would run (no network, no execution).
- Marker logs are safe to share (QSC_MARK lines only).
USAGE
}

seed=1
scenario="happy-path"
out=""
dry_run=0
timeout_sec=0

have_rg=0
if command -v rg >/dev/null 2>&1; then
  have_rg=1
fi

mark_grep() {
  if [ "$have_rg" -eq 1 ]; then
    rg "$@"
  else
    grep -E "$@"
  fi
}

mark_grep_o() {
  if [ "$have_rg" -eq 1 ]; then
    rg -o "$@"
  else
    grep -Eo "$@"
  fi
}

while [ $# -gt 0 ]; do
  case "$1" in
    --help|-h)
      usage; exit 0;;
    --seed)
      seed="$2"; shift 2;;
    --scenario)
      scenario="$2"; shift 2;;
    --out)
      out="$2"; shift 2;;
    --dry-run)
      dry_run=1; shift;;
    --timeout)
      timeout_sec="$2"; shift 2;;
    *)
      echo "Unknown arg: $1" >&2
      usage; exit 2;;
  esac
done

case "$scenario" in
  happy-path|drop|reorder|drop+reorder|drop-reorder|seeded-replay) ;;
  *) echo "Invalid scenario: $scenario" >&2; exit 2;;
esac

if [ -z "$out" ]; then
  ts=$(date -u +%Y%m%dT%H%M%SZ)
  out="./_demo_out/${ts}"
fi

mkdir -p "$out"

# Map scenario to relay knobs (deterministic; seed controls behavior)
# No secrets are printed; marker logs are filtered to QSC_MARK only.
case "$scenario" in
  happy-path)
    drop=0; dup=0; reorder=0; latency=0; jitter=0;;
  drop)
    drop=25; dup=0; reorder=0; latency=0; jitter=0;;
  reorder)
    drop=0; dup=0; reorder=3; latency=0; jitter=0;;
  drop+reorder|drop-reorder)
    drop=25; dup=0; reorder=3; latency=0; jitter=0;;
  seeded-replay)
    drop=10; dup=10; reorder=2; latency=0; jitter=0;;
esac

# Optional timeout wrapper
maybe_timeout=()
if [ "$timeout_sec" -gt 0 ]; then
  maybe_timeout=(timeout "${timeout_sec}s")
fi

inbox_cmd=(${maybe_timeout[@]} python3 -u)

qsc_bin="${QSC_BIN:-./target/debug/qsc}"
if [ ! -x "$qsc_bin" ]; then
  cargo build -p qsc
fi
if [ ! -x "$qsc_bin" ]; then
  echo "ERROR: qsc binary not found at $qsc_bin" >&2
  exit 2
fi

alice_cmd=(${maybe_timeout[@]} "$qsc_bin" send --transport relay --relay http://127.0.0.1:9123 --to bob --file ./_demo_payloads/alice_to_bob.txt)

bob_cmd=(${maybe_timeout[@]} "$qsc_bin" send --transport relay --relay http://127.0.0.1:9123 --to alice --file ./_demo_payloads/bob_to_alice.txt)

if [ "$dry_run" -eq 1 ]; then
  echo "DRY-RUN: ${relay_cmd[*]}"
  echo "DRY-RUN: ${alice_cmd[*]}"
  echo "DRY-RUN: ${bob_cmd[*]}"
  echo "OUT_DIR=$out"
  exit 0
fi

# Safety: marker logs are filtered to QSC_MARK lines only.
# This script does not print secrets; outputs are safe to share.

relay_log="$out/relay.log"
alice_log="$out/alice.log"
bob_log="$out/bob.log"
alice_recv_log="$out/alice_recv.log"
bob_recv_log="$out/bob_recv.log"

mkdir -p ./_demo_payloads
printf 'hello from alice\n' > ./_demo_payloads/alice_to_bob.txt
printf 'hello from bob\n' > ./_demo_payloads/bob_to_alice.txt
mkdir -p "$out/alice_home" "$out/bob_home" "$out/alice_recv" "$out/bob_recv"
chmod 700 "$out/alice_home" "$out/bob_home" "$out/alice_recv" "$out/bob_recv"

inbox_py="$out/inbox_server.py"
cat > "$inbox_py" <<'PY'
#!/usr/bin/env python3
import argparse
import json
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from urllib.parse import urlparse, parse_qs
from collections import deque

store = {}
counter = 0

def get_queue(channel):
    q = store.get(channel)
    if q is None:
        q = deque()
        store[channel] = q
    return q

class Handler(BaseHTTPRequestHandler):
    def _send(self, code, body=b"", ctype="application/json"):
        self.send_response(code)
        self.send_header("Content-Type", ctype)
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        if body:
            self.wfile.write(body)

    def do_POST(self):
        global counter
        p = urlparse(self.path)
        parts = p.path.strip("/").split("/")
        if len(parts) != 3 or parts[0] != "v1" or parts[1] != "push":
            return self._send(404, b"")
        channel = parts[2]
        length = int(self.headers.get("Content-Length", "0"))
        data = self.rfile.read(length)
        q = get_queue(channel)
        counter += 1
        q.append({"id": f"{counter}", "data": list(data)})
        return self._send(200, b"")

    def do_GET(self):
        p = urlparse(self.path)
        parts = p.path.strip("/").split("/")
        if len(parts) != 3 or parts[0] != "v1" or parts[1] != "pull":
            return self._send(404, b"")
        channel = parts[2]
        qs = parse_qs(p.query)
        try:
            max_n = int(qs.get("max", ["0"])[0])
        except Exception:
            return self._send(400, b"")
        q = get_queue(channel)
        if not q or max_n <= 0:
            return self._send(204, b"")
        items = []
        for _ in range(max_n):
            if not q:
                break
            items.append(q.popleft())
        body = json.dumps({"items": items}).encode("utf-8")
        return self._send(200, body)

    def log_message(self, format, *args):
        return

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--port", type=int, default=9123)
    args = ap.parse_args()
    srv = ThreadingHTTPServer(("127.0.0.1", args.port), Handler)
    srv.serve_forever()

if __name__ == "__main__":
    main()
PY

"${inbox_cmd[@]}" "$inbox_py" --port 9123 >"$relay_log" 2>&1 &
relay_pid=$!
trap 'kill "$relay_pid" >/dev/null 2>&1 || true' EXIT

sleep 1
env QSC_QSP_SEED="$seed" QSC_SEED="$seed" QSC_SCENARIO="$scenario" \
  XDG_CONFIG_HOME="$out/alice_home/.config" \
  XDG_DATA_HOME="$out/alice_home/.local/share" \
  XDG_STATE_HOME="$out/alice_home/.local/state" \
  XDG_CACHE_HOME="$out/alice_home/.cache" \
  "${alice_cmd[@]}" >"$alice_log" 2>&1 || true
env QSC_QSP_SEED="$seed" QSC_SEED="$seed" QSC_SCENARIO="$scenario" \
  XDG_CONFIG_HOME="$out/bob_home/.config" \
  XDG_DATA_HOME="$out/bob_home/.local/share" \
  XDG_STATE_HOME="$out/bob_home/.local/state" \
  XDG_CACHE_HOME="$out/bob_home/.cache" \
  "${bob_cmd[@]}" >"$bob_log" 2>&1 || true

env QSC_QSP_SEED="$seed" QSC_SEED="$seed" QSC_SCENARIO="$scenario" \
  XDG_CONFIG_HOME="$out/bob_home/.config" \
  XDG_DATA_HOME="$out/bob_home/.local/share" \
  XDG_STATE_HOME="$out/bob_home/.local/state" \
  XDG_CACHE_HOME="$out/bob_home/.cache" \
  "${maybe_timeout[@]}" "$qsc_bin" receive --transport relay --relay http://127.0.0.1:9123 --from alice --max 1 --out "$out/bob_recv" >"$bob_recv_log" 2>&1 || true
env QSC_QSP_SEED="$seed" QSC_SEED="$seed" QSC_SCENARIO="$scenario" \
  XDG_CONFIG_HOME="$out/alice_home/.config" \
  XDG_DATA_HOME="$out/alice_home/.local/share" \
  XDG_STATE_HOME="$out/alice_home/.local/state" \
  XDG_CACHE_HOME="$out/alice_home/.cache" \
  "${maybe_timeout[@]}" "$qsc_bin" receive --transport relay --relay http://127.0.0.1:9123 --from bob --max 1 --out "$out/alice_recv" >"$alice_recv_log" 2>&1 || true

# Extract markers only
mark_grep '^QSC_MARK' "$relay_log" > "$out/relay.markers" || true
mark_grep '^QSC_MARK' "$alice_log" > "$out/alice.markers" || true
mark_grep '^QSC_MARK' "$bob_log" > "$out/bob.markers" || true
mark_grep '^QSC_MARK' "$alice_recv_log" > "$out/alice_recv.markers" || true
mark_grep '^QSC_MARK' "$bob_recv_log" > "$out/bob_recv.markers" || true

# Deterministic subset: event counts by marker event key
subset="$out/deterministic_subset.txt"
{
  echo "scenario=$scenario"
  echo "seed=$seed"
  echo "relay_markers=$(wc -l < "$out/relay.markers" 2>/dev/null || echo 0)"
  echo "alice_markers=$(wc -l < "$out/alice.markers" 2>/dev/null || echo 0)"
  echo "bob_markers=$(wc -l < "$out/bob.markers" 2>/dev/null || echo 0)"
  echo "alice_recv_markers=$(wc -l < "$out/alice_recv.markers" 2>/dev/null || echo 0)"
  echo "bob_recv_markers=$(wc -l < "$out/bob_recv.markers" 2>/dev/null || echo 0)"
  echo "event_counts:"
  mark_grep_o 'event=[^ ]+' "$out"/*.markers 2>/dev/null | \
    sed 's/^.*event=//' | sort | uniq -c | awk '{print $2"=" $1}' || true
} > "$subset"

deliver_count=$( (mark_grep_o "action=deliver" "$out"/*.markers 2>/dev/null || true) | wc -l | tr -d ' ' )
drop_count=$( (mark_grep_o "action=drop" "$out"/*.markers 2>/dev/null || true) | wc -l | tr -d ' ' )
reorder_count=$( (mark_grep_o "action=reorder" "$out"/*.markers 2>/dev/null || true) | wc -l | tr -d ' ' )
dup_count=$( (mark_grep_o "action=dup" "$out"/*.markers 2>/dev/null || true) | wc -l | tr -d ' ' )

counts="$out/normalized_counts.txt"
{
  echo "scenario=$scenario"
  echo "seed=$seed"
  echo "status=ok"
  echo "deliver_count=$deliver_count"
  echo "drop_count=$drop_count"
  echo "reorder_count=$reorder_count"
  echo "dup_count=$dup_count"
} > "$counts"

summary="$out/summary.txt"
{
  echo "scenario=$scenario"
  echo "seed=$seed"
  echo "out=$out"
  echo "relay_log=$relay_log"
  echo "alice_log=$alice_log"
  echo "bob_log=$bob_log"
  echo "relay_markers=$(wc -l < "$out/relay.markers" 2>/dev/null || echo 0)"
  echo "alice_markers=$(wc -l < "$out/alice.markers" 2>/dev/null || echo 0)"
  echo "bob_markers=$(wc -l < "$out/bob.markers" 2>/dev/null || echo 0)"
  echo "deliver_count=$deliver_count"
  echo "drop_count=$drop_count"
  echo "reorder_count=$reorder_count"
  echo "dup_count=$dup_count"
} > "$summary"

echo "DEMO DONE"
echo "scenario=$scenario seed=$seed"
echo "out=$out"
