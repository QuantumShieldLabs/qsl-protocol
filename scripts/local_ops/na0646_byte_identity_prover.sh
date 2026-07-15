#!/usr/bin/env bash
# NA-0646 (D582) byte-identity prover — corpus runner.
#
# Captures stdout + stderr + exit code for a FIXED command corpus against a given
# qsc binary, one case per converted exit-funnel shape plus happy paths, so two
# capture trees can be diffed BYTE-FOR-BYTE (diff -r) across a refactor:
#   PR-A: pre-move capture vs post-move capture (crate split byte-identity)
#   PR-B: PR-A-merged capture (BEFORE) vs exit->Result capture (AFTER)
#
# Funnel shapes covered (D582):
#   S1 plain &'static-code error   (print_error_marker family)  -> send_file_required
#   S2 code-via-ErrorCode error    (print_error family)         -> io_write_failed
#   S3 code+kv error               (require_unlocked)           -> vault_locked op/reason
#   S4 dynamic-reason error        (protocol_inactive_exit)     -> protocol_inactive missing_seed
#   S5 marker-THEN-error           (file_xfer_reject)           -> file_xfer_reject + error
#   S6 usage exit(2)               (util_sanitize)              -> exit 2 (must stay 2)
# plus happy paths (help stub, status, config set/get, identity show,
# contacts show, util sanitize ok) and one retired-env negative.
#
# The send/receive ROUND is deliberately NOT in the corpus: send output embeds
# fresh-nonce-derived material and is not byte-deterministic across runs; the
# round is covered behaviorally by the standard suite + the NA-0640 e2e.
#
# FIXTURE: created ONCE (vault init + contact peer-0) and REUSED for every later
# capture, so key material is identical across BEFORE/AFTER. Create it with the
# baseline binary; never delete it between captures.
#
# Usage:
#   scripts/local_ops/na0646_byte_identity_prover.sh \
#     --bin <path-to-qsc> --out <capture-dir> --work <scratch-base> [--fixture <dir>]
#
# Then: diff -r <before-capture> <after-capture>  (empty output = byte-identical)

set -euo pipefail

BIN="" OUT="" WORK="" FIXTURE=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --bin) BIN="$2"; shift 2 ;;
    --out) OUT="$2"; shift 2 ;;
    --work) WORK="$2"; shift 2 ;;
    --fixture) FIXTURE="$2"; shift 2 ;;
    *) echo "unknown arg: $1" >&2; exit 2 ;;
  esac
done
[[ -n "$BIN" && -n "$OUT" && -n "$WORK" ]] || {
  echo "usage: $0 --bin <qsc> --out <dir> --work <dir> [--fixture <dir>]" >&2; exit 2; }
[[ -x "$BIN" ]] || { echo "not executable: $BIN" >&2; exit 2; }
FIXTURE="${FIXTURE:-$WORK/fixture}"

mkdir -p "$OUT" "$WORK"
chmod 700 "$WORK"

# --- one-time fixture (vault + pinned contact peer-0); reused across captures ---
if [[ ! -d "$FIXTURE/store" ]]; then
  echo "prover: creating fixture at $FIXTURE (one-time)" >&2
  mkdir -m 700 -p "$FIXTURE/store"
  printf 'prover-pass-1' > "$FIXTURE/pass.txt"
  chmod 600 "$FIXTURE/pass.txt"
  QSC_CONFIG_DIR="$FIXTURE/store" QSC_DISABLE_KEYCHAIN=1 HOME=/nonexistent \
    "$BIN" vault init --non-interactive --key-source passphrase \
      --passphrase-file "$FIXTURE/pass.txt" >/dev/null
  QSC_CONFIG_DIR="$FIXTURE/store" QSC_DISABLE_KEYCHAIN=1 HOME=/nonexistent \
    "$BIN" --unlock-passphrase-file "$FIXTURE/pass.txt" \
      contacts add --label peer-0 --fp 00ff00ff00ff00ff >/dev/null
  QSC_CONFIG_DIR="$FIXTURE/store" QSC_DISABLE_KEYCHAIN=1 HOME=/nonexistent \
    "$BIN" --unlock-passphrase-file "$FIXTURE/pass.txt" \
      identity rotate --confirm >/dev/null
fi
PASS="$FIXTURE/pass.txt"

MSG="$WORK/msg.txt"
printf 'hello-prover' > "$MSG"

# run_case <name> <store-mode: fixture|fresh> [--unlock] -- <args...>
run_case() {
  local name="$1" mode="$2"; shift 2
  local unlock=""
  if [[ "$1" == "--unlock" ]]; then unlock=1; shift; fi
  [[ "$1" == "--" ]] && shift
  local store="$WORK/case-store"
  rm -rf "$store"
  if [[ "$mode" == "fixture" ]]; then
    cp -a "$FIXTURE/store" "$store"
  else
    mkdir -m 700 -p "$store"
  fi
  local dir="$OUT/$name"
  mkdir -p "$dir"
  local -a cmd=("$BIN")
  [[ -n "$unlock" ]] && cmd+=(--unlock-passphrase-file "$PASS")
  cmd+=("$@")
  set +e
  env -i PATH="$PATH" LC_ALL=C TZ=UTC HOME=/nonexistent \
    QSC_CONFIG_DIR="$store" QSC_DISABLE_KEYCHAIN=1 \
    "${cmd[@]}" </dev/null >"$dir/stdout" 2>"$dir/stderr"
  echo $? > "$dir/exit"
  set -e
  printf '%s\n' "$name" >> "$OUT/MANIFEST"
}

: > "$OUT/MANIFEST"

# --- error shapes, one per funnel ---
run_case s1_send_file_required        fixture --unlock -- send --transport relay --relay http://127.0.0.1:9 --to peer-0
run_case s2_errorcode_io_write_failed fresh            -- doctor --check-only --export /nonexistent/dir/x.json
run_case s3_vault_locked_kv           fresh            -- contacts add --label peer-x --fp 00ff
run_case s4_protocol_inactive_dynamic fixture --unlock -- send --transport relay --relay http://127.0.0.1:9 --to peer-0 --file "$MSG"
run_case s5_file_xfer_reject          fixture --unlock -- file send --to peer-0 --path /nonexistent/f.bin
run_case s6_usage_exit2               fresh            -- util sanitize

# --- happy paths + one extra negative ---
run_case h1_help_stub                 fresh            --
run_case h2_status                    fixture          -- status
run_case h3_config_set                fixture          -- config set policy-profile strict
run_case h4_config_get                fixture          -- config get policy-profile
run_case h5_identity_show             fixture --unlock -- identity show
run_case h6_contacts_show             fixture --unlock -- contacts show --label peer-0
run_case h7_sanitize_ok               fresh            -- util sanitize --print hello-prover-text
run_case n1_env_retired               fresh            -- --unlock-passphrase-env QSC_WRONG_ENV status

echo "prover: captured $(wc -l < "$OUT/MANIFEST") cases into $OUT" >&2
