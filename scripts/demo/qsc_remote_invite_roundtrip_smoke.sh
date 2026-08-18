#!/usr/bin/env bash
set -euo pipefail

# NA-0744 / D-1382 (R353 §5, WF-0086). THE INVITE FIRST-CONTACT ROUND TRIP.
#
# WHY IT EXISTS. `invite` occurs ZERO times in all of scripts/demo/ (measured at
# d484c065): no script in this tree, loopback or remote, exercises the invite
# first-contact path. `qsc relay serve` answers only /v1/push and /v1/pull -- no
# invite routes, no /v1/pull/ack -- and the invite-capable qsl_server is a
# [dev-dependencies] git pin reachable only from Rust test code. So this flow
# cannot be driven on loopback: it is OUT OF BOUNDS BY POLICY (a cargo-install
# route exists and is forbidden), never "impossible". THIS SCRIPT IS NEVER
# DESCRIBED AS LOOPBACK-VERIFIED END TO END.
#
# Structurally modelled on qsc_remote_handshake_smoke.sh and reusing its proven
# helpers by name. Every assertion EXTRACTS a value and compares it with `=`;
# nothing is matched as a substring (ENG-0191's lesson: `established` is a PREFIX
# of `established_recv_only`, and the unanchored regex passed on the wrong state
# for 187 days).

usage() {
  cat <<'USAGE'
qsc_remote_invite_roundtrip_smoke.sh --out <dir> [--seed <u64>]

Environment:
  RELAY_URL   (required) remote relay endpoint
  RELAY_TOKEN (required) bearer token for auth-gated relay
USAGE
}

seed="1"
out="./_remote_invite_roundtrip_out"

while [ $# -gt 0 ]; do
  case "$1" in
    --help|-h) usage; exit 0 ;;
    --seed) seed="$2"; shift 2 ;;
    --out) out="$2"; shift 2 ;;
    *) echo "Unknown arg: $1" >&2; usage; exit 2 ;;
  esac
done

if [ -z "${RELAY_URL:-}" ]; then
  echo "RELAY_URL is required" >&2
  exit 2
fi
if [ -z "${RELAY_TOKEN:-}" ]; then
  echo "RELAY_TOKEN is required" >&2
  exit 2
fi

umask 077
mkdir -p "$out"
chmod 700 "$out"

# Normalize env payloads in case secrets are supplied as KEY=value.
relay_url="$(printf '%s' "$RELAY_URL" | sed -E 's/^[[:space:]]*RELAY_URL[[:space:]]*=[[:space:]]*//')"
relay_token="$(printf '%s' "$RELAY_TOKEN" | sed -E 's/^[[:space:]]*RELAY_TOKEN[[:space:]]*=[[:space:]]*//')"

relay_addr="$relay_url"
case "$relay_addr" in
  http://*|https://*) : ;;
  *) relay_addr="http://$relay_addr" ;;
esac

# Avoid cross-run mailbox/session collisions on shared remote relays.
run_tag="${GITHUB_RUN_ID:-local}-${GITHUB_RUN_ATTEMPT:-0}-invite-${seed}"
run_tag="$(printf '%s' "$run_tag" | tr -c 'a-zA-Z0-9_-' '-')"
proto_alice="alice-${run_tag}"
proto_bob="bob-${run_tag}"
alice_route_token="route_token_alice_${run_tag}"
bob_route_token="route_token_bob_${run_tag}"
# Local-only aliases. Required, user-typed, never pre-populated.
alias_at_bob="alicevia-${run_tag}"
alias_at_alice="bobvia-${run_tag}"

state_root="$out/state_${run_tag}"
peer_alice="$state_root/peer_alice"
peer_bob="$state_root/peer_bob"
out_alice="$out/out_alice"
out_bob="$out/out_bob"
secret_root="${RUNNER_TEMP:-${TMPDIR:-/tmp}}/qsc_remote_invite_${run_tag}"
secret_dir="$secret_root/passphrases"
mkdir -p "$peer_alice" "$peer_bob" "$out_alice" "$out_bob" "$secret_dir"
chmod 700 "$state_root" "$peer_alice" "$peer_bob" "$out_alice" "$out_bob" "$secret_root" "$secret_dir"

alice_passphrase_file="$secret_dir/alice.passphrase"
bob_passphrase_file="$secret_dir/bob.passphrase"
printf '%s\n' "na0744-${run_tag}-alice-vault-passphrase" > "$alice_passphrase_file"
printf '%s\n' "na0744-${run_tag}-bob-vault-passphrase" > "$bob_passphrase_file"
chmod 600 "$alice_passphrase_file" "$bob_passphrase_file"

markers="$out/markers"
summary="$out/summary.txt"
subset="$out/normalized_subset.txt"
counts="$out/normalized_counts.txt"
alice_log="$out/alice.log"
bob_log="$out/bob.log"
alice_recv_log="$out/alice_recv.log"
bob_recv_log="$out/bob_recv.log"
alice_residue_log="$out/alice_residue.log"
bob_residue_log="$out/bob_residue.log"

: > "$markers"
: > "$summary"
: > "$subset"
: > "$counts"
: > "$alice_log"
: > "$bob_log"
: > "$alice_recv_log"
: > "$bob_recv_log"
: > "$alice_residue_log"
: > "$bob_residue_log"

if [ -x "target/debug/qsc" ]; then
  qsc_cmd=("target/debug/qsc")
else
  qsc_cmd=("cargo" "run" "-p" "qsc" "--")
fi

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

mark_count() {
  local pattern="$1"
  if [ "$have_rg" -eq 1 ]; then
    rg -c "$pattern" "$markers" || true
  else
    grep -Ec "$pattern" "$markers" || true
  fi
}

# NA-0744 / R353 §5. `marker_values` (qsc_remote_handshake_smoke.sh:126) aggregates a
# field across the WHOLE marker file, so a log carrying both halves of the boundary
# publishes ONE BLENDED VALUE for a field name the halves deliberately SHARE (D-1324).
# Filter by EVENT first -- the push marker has no `op=` and cannot be selected by one.
#
# ⚠ The key is anchored LEFT: unanchored, `class` also harvests status_class=,
# error_class=, diagnostic_class= and timeout_phase_class= (m8).
#
# ⚠⚠ `.*` AND NOT `[^\n]*`, AND THAT IS A MEASUREMENT, NOT A PREFERENCE. Under
# `rg` the two agree. Under `grep -E` -- the fallback this script keeps alive and
# the only one present on a runner without ripgrep -- `[^\n]` is a bracket
# expression over the LITERAL characters `\` and `n`, so it truncates every line
# at its first `n`. Measured: `grep -Eo 'event=x[^\n]*'` on
# `event=x op=pull noise` returns `event=x op=pull `. Every field this helper
# harvests sits AFTER `op=`, so the fallback path would have published
# `diagnostic_unavailable` for all of them and nothing would have gone red.
marker_values_for_event() {
  local event="$1"
  local key="$2"
  local opsel="${3:-}"
  local values=""
  local sel="event=${event}"
  if [ -n "$opsel" ]; then
    sel="${sel}.*\bop=${opsel}\b"
  fi
  values=$(
    (mark_grep -o "${sel}.*" "$markers" 2>/dev/null || true) \
      | (grep -Eo "(^| )${key}=[^ ]+" || true) \
      | sed -E "s/^ ?${key}=//" \
      | sort -u \
      | paste -sd, -
  )
  if [ -z "$values" ]; then
    echo "diagnostic_unavailable"
  else
    echo "$values"
  fi
}

run_qsc_step() {
  local actor="$1"
  local step="$2"
  local log_file="$3"
  shift 3
  local tmp="$out/.${actor}_${step}.tmp"
  local home=""
  local peer=""
  local passphrase_file=""
  if [ "$actor" = "alice" ]; then
    home="$peer_alice"
    peer="bob"
    passphrase_file="$alice_passphrase_file"
  else
    home="$peer_bob"
    peer="alice"
    passphrase_file="$bob_passphrase_file"
  fi

  set +e
  (
    export XDG_CONFIG_HOME="$home/.config"
    export XDG_DATA_HOME="$home/.local/share"
    export XDG_STATE_HOME="$home/.local/state"
    export XDG_CACHE_HOME="$home/.cache"
    export QSC_CONFIG_DIR="$home/.qsc"
    export QSC_SELF_LABEL="$actor"
    export QSC_SEED="$seed"
    export RELAY_URL="$relay_addr"
    export RELAY_TOKEN="$relay_token"
    export QSC_RELAY_TOKEN="$relay_token"
    export QSC_RELAY_PUSH_DIAGNOSTIC=redacted
    # NA-0744: the half this lane builds. Both halves of the boundary are on, so
    # a single run publishes both dialects and the event-first helper above is
    # what keeps them apart.
    export QSC_RELAY_PULL_DIAGNOSTIC=redacted
    unset QSC_ALLOW_SEED_FALLBACK
    unset QSC_UNSAFE_TEST_SEED_FALLBACK
    unset QSC_QSP_SEED
    mkdir -p "$XDG_CONFIG_HOME" "$XDG_DATA_HOME" "$XDG_STATE_HOME" "$XDG_CACHE_HOME" "$QSC_CONFIG_DIR"
    chmod 700 "$XDG_CONFIG_HOME" "$XDG_DATA_HOME" "$XDG_STATE_HOME" "$XDG_CACHE_HOME" "$QSC_CONFIG_DIR"
    if [ "$step" = "vault_init" ]; then
      "${qsc_cmd[@]}" "$@"
    else
      "${qsc_cmd[@]}" --unlock-passphrase-file "$passphrase_file" "$@"
    fi
  ) >"$tmp" 2>&1
  local rc=$?
  set -e

  cat "$tmp" >> "$log_file"
  if [ "$have_rg" -eq 1 ]; then
    rg '^QSC_MARK/1' "$tmp" | sed -E "s/$/ actor=${actor} peer=${peer} step=${step}/" >> "$markers" || true
  else
    grep -E '^QSC_MARK/1' "$tmp" | sed -E "s/$/ actor=${actor} peer=${peer} step=${step}/" >> "$markers" || true
  fi
  rm -f "$tmp"
  return "$rc"
}

assert_marker_present() {
  local pattern="$1"
  local file="$2"
  local msg="$3"
  if ! mark_grep "$pattern" "$file" >/dev/null 2>&1; then
    echo "$msg" >&2
    exit 1
  fi
}

run_vault_init() {
  local actor="$1"
  local log_file="$2"
  local passphrase_file=""
  if [ "$actor" = "alice" ]; then
    passphrase_file="$alice_passphrase_file"
  else
    passphrase_file="$bob_passphrase_file"
  fi
  if run_qsc_step "$actor" vault_init "$log_file" vault init --non-interactive --passphrase-file "$passphrase_file" --key-source passphrase; then
    if ! run_qsc_step "$actor" vault_status "$log_file" vault status; then
      echo "vault status failed for $actor after initialization" >&2
      exit 1
    fi
    assert_marker_present 'event=vault_status present=true' "$log_file" "vault status missing after init for $actor"
    return 0
  fi
  if mark_grep 'event=error code=vault_exists' "$log_file" >/dev/null 2>&1; then
    if ! run_qsc_step "$actor" vault_status "$log_file" vault status; then
      echo "vault status failed for $actor after existing vault detection" >&2
      exit 1
    fi
    assert_marker_present 'event=vault_status present=true' "$log_file" "vault status missing after existing vault for $actor"
    return 0
  fi
  echo "vault initialization failed for $actor before relay interaction" >&2
  exit 1
}

run_required_qsc_step() {
  local actor="$1"
  local step="$2"
  local log_file="$3"
  local expected_marker="$4"
  local msg="$5"
  shift 5
  if ! run_qsc_step "$actor" "$step" "$log_file" "$@"; then
    echo "$msg" >&2
    exit 1
  fi
  assert_marker_present "$expected_marker" "$log_file" "$msg"
}

extract_identity_field() {
  local log_file="$1"
  local actor="$2"
  local key="$3"
  local what="$4"
  local value=""
  value="$(sed -n -E "s/^${key}=([^[:space:]]+).*/\1/p" "$log_file" | tail -n1)"
  if [ -z "$value" ]; then
    echo "$what missing for $actor before relay interaction" >&2
    exit 1
  fi
  printf '%s\n' "$value"
}

extract_identity_fp() {
  extract_identity_field "$1" "$2" identity_fp "identity fingerprint"
}

# NA-0744 E10 (m7). `invite create` prints the code BARE (main.rs:339-341): no
# `key=` prefix, no marker, nothing for a field extractor to anchor on. And
# `run_qsc_step` MERGES STDERR INTO THE CAPTURE (`>"$tmp" 2>&1`), so any warning
# the binary or the runtime writes lands in the very same file.
#
# ⚠ THE EXTRACTOR IS THEREFORE A POSITIVE SOLE-LINE ANCHOR -- never `tail -n1`,
# never positional -- so a prepended warning line cannot change what it extracts.
# The anchor is TIGHTENED past a bare length rule to the code's OWN prefix:
# `invite_create` emits `QSLI-1-<base64url>` unpadded (invite/mod.rs:71, :410) and
# base64url's alphabet is exactly [A-Za-z0-9_-], so a long token line that is not
# an invite code can no longer be mistaken for one.
#
# ⚠ `invite create` prints exactly one line today and NOTHING PINS THAT. If it
# ever prints two, the count guard fails BY NAME rather than silently taking one.
extract_invite_code() {
  local log_file="$1"
  local codes=""
  local n=0
  codes="$(sed -n -E 's/^(QSLI-1-[A-Za-z0-9_-]+)[[:space:]]*$/\1/p' "$log_file")"
  if [ -z "$codes" ]; then
    echo "invite code missing: invite create printed no QSLI-1- sole line" >&2
    exit 1
  fi
  n="$(printf '%s\n' "$codes" | wc -l | tr -d ' ')"
  if [ "$n" -ne 1 ]; then
    echo "invite create printed $n sole-line invite codes, expected exactly 1" >&2
    exit 1
  fi
  printf '%s\n' "$codes"
}

# `invite list` prints `invite=<id> state=<state> expiry=<n>` (main.rs). Counted,
# not tailed: exactly one slot exists at this point in the flow, and if that ever
# stops being true this fails by name instead of picking the last one.
extract_sole_invite_id() {
  local log_file="$1"
  local ids=""
  local n=0
  ids="$(sed -n -E 's/^invite=([^[:space:]]+)[[:space:]]+state=.*/\1/p' "$log_file")"
  if [ -z "$ids" ]; then
    echo "invite list printed no invite id for alice" >&2
    exit 1
  fi
  n="$(printf '%s\n' "$ids" | wc -l | tr -d ' ')"
  if [ "$n" -ne 1 ]; then
    echo "invite list printed $n invite ids, expected exactly 1" >&2
    exit 1
  fi
  printf '%s\n' "$ids"
}

# ⚠ `invite accept` PRINTS THE SAME SHAPE AS `invite redeem` ON SUCCESS
# (`contact=<alias> status=pinned fp=<fp>`) and `invite=<id> accepted=none` on the
# silent no-op. Both are bare `println!`, not markers, so no marker-count guard can
# observe them. THE DISCRIMINATOR IS THE fp EQUALITY BELOW -- the same shape with
# different values.
extract_pinned_contact_fp() {
  local log_file="$1"
  local alias="$2"
  local what="$3"
  local value=""
  value="$(sed -n -E "s/^contact=${alias} status=pinned fp=([^[:space:]]+)[[:space:]]*\$/\1/p" "$log_file" | tail -n1)"
  if [ -z "$value" ]; then
    echo "$what did not print a pinned contact line for alias ${alias}" >&2
    exit 1
  fi
  printf '%s\n' "$value"
}

assert_equal() {
  local what="$1"
  local actual="$2"
  local expected="$3"
  if [ "$actual" != "$expected" ]; then
    echo "$what is '$actual', expected exactly '$expected'" >&2
    exit 1
  fi
}

extract_invite_finish() {
  local log_file="$1"
  local value=""
  value="$(sed -n -E 's/^invite_finish=([^[:space:]]+)[[:space:]]*$/\1/p' "$log_file" | tail -n1)"
  if [ -z "$value" ]; then
    echo "invite finish printed no invite_finish= line" >&2
    exit 1
  fi
  printf '%s\n' "$value"
}

count_in_file() {
  local pattern="$1"
  local file="$2"
  local n=""
  n="$( (mark_grep -c "$pattern" "$file" 2>/dev/null || true) | tail -n1 )"
  if [ -z "$n" ]; then
    n=0
  fi
  printf '%s\n' "$n"
}

# ---------------------------------------------------------------------------
# STEP 1 — stores, inboxes, identities
# ---------------------------------------------------------------------------
run_vault_init alice "$alice_log"
run_vault_init bob "$bob_log"
run_required_qsc_step alice relay_inbox_set "$alice_log" 'event=relay_inbox_set ok=true' \
  "relay inbox setup failed for alice before the invite" relay inbox-set --token "$alice_route_token"
run_required_qsc_step bob relay_inbox_set "$bob_log" 'event=relay_inbox_set ok=true' \
  "relay inbox setup failed for bob before the invite" relay inbox-set --token "$bob_route_token"
run_required_qsc_step alice identity_rotate "$alice_log" 'event=identity_rotate ok=true' \
  "identity initialization failed for alice before the invite" identity rotate --as "$proto_alice" --confirm
run_required_qsc_step bob identity_rotate "$bob_log" 'event=identity_rotate ok=true' \
  "identity initialization failed for bob before the invite" identity rotate --as "$proto_bob" --confirm
alice_fp="$(extract_identity_fp "$alice_log" alice)"
bob_fp="$(extract_identity_fp "$bob_log" bob)"

# ---------------------------------------------------------------------------
# STEP 2 — A mints the invite. THE ONLY OUT-OF-BAND STEP.
# ---------------------------------------------------------------------------
invite_create_log="$out/alice_invite_create.log"
: > "$invite_create_log"
if ! run_qsc_step alice invite_create "$invite_create_log" invite create --relay "$relay_addr"; then
  echo "invite create failed for alice" >&2
  exit 1
fi
cat "$invite_create_log" >> "$alice_log"
invite_code="$(extract_invite_code "$invite_create_log")"

invite_list_log="$out/alice_invite_list.log"
: > "$invite_list_log"
if ! run_qsc_step alice invite_list "$invite_list_log" invite list; then
  echo "invite list failed for alice" >&2
  exit 1
fi
cat "$invite_list_log" >> "$alice_log"
invite_id="$(extract_sole_invite_id "$invite_list_log")"

# ---------------------------------------------------------------------------
# STEP 3 — B redeems. ASSERT the pinned fp EQUALS A's identity, by `=`.
# ---------------------------------------------------------------------------
invite_redeem_log="$out/bob_invite_redeem.log"
: > "$invite_redeem_log"
if ! run_qsc_step bob invite_redeem "$invite_redeem_log" invite redeem --code "$invite_code" --alias "$alias_at_bob"; then
  echo "invite redeem failed for bob" >&2
  exit 1
fi
cat "$invite_redeem_log" >> "$bob_log"
redeem_fp="$(extract_pinned_contact_fp "$invite_redeem_log" "$alias_at_bob" "invite redeem")"
assert_equal "the fp bob pinned for alice at redeem" "$redeem_fp" "$alice_fp"

# ---------------------------------------------------------------------------
# STEP 4 — A accepts. Same printed SHAPE as redeem; the fp equality is what tells
# the success apart from the `invite=<id> accepted=none` silent no-op.
# ---------------------------------------------------------------------------
invite_accept_log="$out/alice_invite_accept.log"
: > "$invite_accept_log"
if ! run_qsc_step alice invite_accept "$invite_accept_log" invite accept --invite-id "$invite_id" --alias "$alias_at_alice"; then
  echo "invite accept failed for alice" >&2
  exit 1
fi
cat "$invite_accept_log" >> "$alice_log"
accept_fp="$(extract_pinned_contact_fp "$invite_accept_log" "$alias_at_alice" "invite accept")"
assert_equal "the fp alice pinned for bob at accept" "$accept_fp" "$bob_fp"

# ---------------------------------------------------------------------------
# STEP 5 — B finishes. EXTRACTED and compared by `=`; `none` is a FAILURE here,
# not a quieter success. Positive corroboration from the producer-ack markers.
# ---------------------------------------------------------------------------
invite_finish_log="$out/bob_invite_finish.log"
: > "$invite_finish_log"
if ! run_qsc_step bob invite_finish "$invite_finish_log" invite finish --alias "$alias_at_bob" --relay "$relay_addr"; then
  echo "invite finish failed for bob" >&2
  exit 1
fi
cat "$invite_finish_log" >> "$bob_log"
invite_finish_value="$(extract_invite_finish "$invite_finish_log")"
assert_equal "invite_finish" "$invite_finish_value" "ok"
assert_marker_present 'event=producer_ack caller=finish' "$invite_finish_log" \
  "invite finish did not emit a producer_ack for the frame it consumed"
assert_marker_present 'event=invite_scan_summary' "$invite_finish_log" \
  "invite finish did not emit invite_scan_summary"

# ---------------------------------------------------------------------------
# STEP 6 — handshake to completion.
# ⚠ POSITIVE EVIDENCE, NEVER ABSENCE-OF-ERROR (ENG-0198's lesson). The count of
# `event=handshake_complete` -- emitted at exactly two sites, handshake/mod.rs
# :2005 and :2178 -- must INCREASE across this block. A flow that quietly did
# nothing leaves it unchanged and fails here by name.
# ---------------------------------------------------------------------------
hs_complete_before="$(mark_count 'event=handshake_complete')"
run_qsc_step alice hs_poll_a1 "$alice_log" handshake poll --peer "$alias_at_alice" --relay "$relay_addr" --max 4 || true
run_qsc_step bob   hs_poll_b1 "$bob_log"   handshake poll --peer "$alias_at_bob"   --relay "$relay_addr" --max 4 || true
run_qsc_step alice hs_poll_a2 "$alice_log" handshake poll --peer "$alias_at_alice" --relay "$relay_addr" --max 4 || true
run_qsc_step bob   hs_poll_b2 "$bob_log"   handshake poll --peer "$alias_at_bob"   --relay "$relay_addr" --max 4 || true
hs_complete_after="$(mark_count 'event=handshake_complete')"
if [ "$hs_complete_after" -le "$hs_complete_before" ]; then
  echo "handshake_complete did not increase (${hs_complete_before} -> ${hs_complete_after}): the invite round trip did not reach a completed handshake" >&2
  exit 1
fi

# ---------------------------------------------------------------------------
# STEP 7 — a message each way, asserted BY EQUALITY on the delivered bytes.
#
# ⚠ `receive` IS INVOKED WITH `--mailbox <ROUTE TOKEN>`, NEVER A LABEL, and that
# is the operator's confirmed choice [O] 2026-08-18: the omitted-`--mailbox` path
# is covered nightly by the handshake smoke, while this OVERRIDE is the path that
# broke NA-0736 -- a LABEL was passed where a route token is required -- and the
# one nothing gates. Each peer receives on its OWN inbox route token.
# ---------------------------------------------------------------------------
alice_payload="$out/alice_to_bob.txt"
bob_payload="$out/bob_to_alice.txt"
printf 'hello-from-alice-na0744\n' > "$alice_payload"
printf 'hello-from-bob-na0744\n' > "$bob_payload"

assert_payload_delivered() {
  local dir="$1"
  local expected_file="$2"
  local what="$3"
  local n=0
  local f=""
  # ⚠ COMPARED BY EQUALITY (`cmp`), not by substring: the bytes the peer wrote
  # must equal the bytes the sender sent.
  while IFS= read -r f; do
    if cmp -s "$f" "$expected_file"; then
      n=$((n + 1))
    fi
  done < <(find "$dir" -type f 2>/dev/null)
  if [ "$n" -ne 1 ]; then
    echo "$what: expected exactly 1 delivered file byte-equal to the sent payload, found $n" >&2
    exit 1
  fi
}

run_required_qsc_step alice send_ab "$alice_log" 'event=qsp_pack ok=true' \
  "alice failed to send to bob after the invite round trip" \
  send --transport relay --relay "$relay_addr" --to "$alias_at_alice" --file "$alice_payload"
if ! run_qsc_step bob recv_from_alice "$bob_recv_log" receive --transport relay --relay "$relay_addr" --mailbox "$bob_route_token" --from "$alias_at_bob" --max 4 --out "$out_bob"; then
  echo "bob failed to receive alice's message" >&2
  exit 1
fi
assert_marker_present 'event=qsp_unpack ok=true' "$bob_recv_log" "bob did not unpack alice's message"
assert_payload_delivered "$out_bob" "$alice_payload" "alice -> bob"

run_required_qsc_step bob send_ba "$bob_log" 'event=qsp_pack ok=true' \
  "bob failed to send to alice after the invite round trip" \
  send --transport relay --relay "$relay_addr" --to "$alias_at_bob" --file "$bob_payload"
if ! run_qsc_step alice recv_from_bob "$alice_recv_log" receive --transport relay --relay "$relay_addr" --mailbox "$alice_route_token" --from "$alias_at_alice" --max 4 --out "$out_alice"; then
  echo "alice failed to receive bob's message" >&2
  exit 1
fi
assert_marker_present 'event=qsp_unpack ok=true' "$alice_recv_log" "alice did not unpack bob's message"
assert_payload_delivered "$out_alice" "$bob_payload" "bob -> alice"

# ---------------------------------------------------------------------------
# STEP 8 — THE ZERO-RESIDUE CLOSE, WITH ITS CLOCK ANTECEDENT (R353 §6, M5).
#
# ⚠⚠ THE ASSERTION DEPENDS ON THE PULL LEASE NOT HAVING EXPIRED. A leased frame
# that was delivered and acked stays gone; one whose lease expired is REDELIVERED,
# and a redelivery is not residue -- it is the lease contract working. The lease
# is [O-provenance and UNADVERTISED by `server-info`], so the script cannot read
# it and must instead publish the interval it actually measured and say what that
# interval licenses. The arm is a RESULT only while `elapsed < 30`; past that it
# is a NON-RESULT, one immediate retry is taken, and a second miss is a NAMED
# FAILURE. This is the remote mirror of NA-0742's own committed T4, not a new claim.
# ---------------------------------------------------------------------------
residue_lease_bound=30
residue_ref_epoch="$(date -u +%s)"

residue_run() {
  local attempt="$1"
  : > "$alice_residue_log"
  : > "$bob_residue_log"
  run_qsc_step alice "residue_alice_${attempt}" "$alice_residue_log" receive --transport relay --relay "$relay_addr" --mailbox "$alice_route_token" --from "$alias_at_alice" --max 4 --out "$out_alice" || true
  run_qsc_step bob   "residue_bob_${attempt}"   "$bob_residue_log"   receive --transport relay --relay "$relay_addr" --mailbox "$bob_route_token"   --from "$alias_at_bob"   --max 4 --out "$out_bob"   || true
}

residue_is_clean() {
  local none_a none_b skip_a skip_b
  none_a="$(count_in_file 'event=recv_none' "$alice_residue_log")"
  none_b="$(count_in_file 'event=recv_none' "$bob_residue_log")"
  skip_a="$(count_in_file 'event=recv_frame_skipped' "$alice_residue_log")"
  skip_b="$(count_in_file 'event=recv_frame_skipped' "$bob_residue_log")"
  residue_detail="recv_none_alice=${none_a} recv_none_bob=${none_b} recv_frame_skipped_alice=${skip_a} recv_frame_skipped_bob=${skip_b}"
  [ "$none_a" -ge 1 ] && [ "$none_b" -ge 1 ] && [ "$skip_a" -eq 0 ] && [ "$skip_b" -eq 0 ]
}

residue_detail=""
residue_run 1
residue_elapsed=$(( $(date -u +%s) - residue_ref_epoch ))
residue_verdict=""
residue_attempts=1

if residue_is_clean; then
  residue_verdict="clean"
elif [ "$residue_elapsed" -ge "$residue_lease_bound" ]; then
  # NON-RESULT: past the lease bound a redelivery is the contract, not residue.
  # ONE immediate retry, and the clock restarts with it.
  residue_attempts=2
  residue_ref_epoch="$(date -u +%s)"
  residue_run 2
  residue_elapsed=$(( $(date -u +%s) - residue_ref_epoch ))
  if residue_is_clean; then
    residue_verdict="clean_after_lease_retry"
  else
    echo "zero-residue close FAILED on the retry (${residue_detail}); elapsed=${residue_elapsed}s" >&2
    exit 1
  fi
else
  echo "zero-residue close FAILED inside the lease window (${residue_detail}); elapsed=${residue_elapsed}s < ${residue_lease_bound}s, so this is residue and not a redelivery" >&2
  exit 1
fi

# ---------------------------------------------------------------------------
# STEP 9 — COUNTS. Every needle anchored at token end. The script writes no line
# of its own into the marker stream.
# ---------------------------------------------------------------------------
relay_esc="$(printf '%s' "$relay_addr" | sed -E 's/[][\/.^$*+?(){}|]/\\&/g')"
token_esc="$(printf '%s' "$relay_token" | sed -E 's/[][\/.^$*+?(){}|]/\\&/g')"

redacted="$out/.markers.redacted"
sed -E \
  -e "s/${relay_esc}/RELAY_URL_REDACTED/g" \
  -e "s/${token_esc}/RELAY_TOKEN_REDACTED/g" \
  -e "s/${proto_alice}/alice/g" \
  -e "s/${proto_bob}/bob/g" \
  -e "s/${alias_at_alice}/bobvia/g" \
  -e "s/${alias_at_bob}/alicevia/g" \
  "$markers" > "$redacted"

# The parent's OWN id-stripping list (qsc_remote_handshake_smoke.sh:511-517),
# inherited rather than re-invented so the two suites normalize identically.
awk '
  /QSC_MARK\/1/ {
    line=$0
    gsub(/ id=[^ ]+/, "", line)
    gsub(/ sid=[^ ]+/, "", line)
    gsub(/ channel=[^ ]+/, "", line)
    gsub(/ seq=[^ ]+/, "", line)
    gsub(/ idx=[^ ]+/, "", line)
    gsub(/ msg_idx=[^ ]+/, "", line)
    gsub(/ ck_idx=[^ ]+/, "", line)
    print line
  }
' "$redacted" > "$subset"

qsp_pack_ok_count="$(mark_count 'event=qsp_pack ok=true')"
qsp_unpack_ok_count="$(mark_count 'event=qsp_unpack ok=true')"
recv_commit_count="$(mark_count 'event=recv_commit')"
recv_commit_alice="$(count_in_file 'event=recv_commit' "$alice_recv_log")"
recv_commit_bob="$(count_in_file 'event=recv_commit' "$bob_recv_log")"
producer_ack_count="$(mark_count 'event=producer_ack')"
invite_scan_summary_count="$(mark_count 'event=invite_scan_summary')"
relay_unauthorized_count="$(mark_count 'code=relay_unauthorized')"

relay_push_diagnostic_count="$(mark_count 'event=relay_push_diagnostic')"
relay_pull_diagnostic_count="$(mark_count 'event=relay_pull_diagnostic')"

# ⚠ EVENT-FIRST, THEN `op=`. `marker_values` would blend the two halves of the
# boundary into one value for the field names they deliberately SHARE (D-1324).
relay_push_diagnostic_classes="$(marker_values_for_event relay_push_diagnostic diagnostic_class)"
relay_pull_diagnostic_classes_pull="$(marker_values_for_event relay_pull_diagnostic diagnostic_class pull)"
relay_pull_diagnostic_classes_ack="$(marker_values_for_event relay_pull_diagnostic diagnostic_class ack)"
relay_pull_status_classes_pull="$(marker_values_for_event relay_pull_diagnostic status_class pull)"
relay_pull_status_classes_ack="$(marker_values_for_event relay_pull_diagnostic status_class ack)"
relay_pull_error_classes_pull="$(marker_values_for_event relay_pull_diagnostic error_class pull)"
relay_pull_error_classes_ack="$(marker_values_for_event relay_pull_diagnostic error_class ack)"

# ⚠⚠ E7's ANTECEDENT, PUBLISHED SO IT CANNOT BE MISREAD. A count of 0 with the
# gate UNSET is an INSTRUMENT FAULT and not a result; a count of 0 with the gate
# SET is a NAMED FAILURE. The gate's own setting is printed beside the count so
# the two can never be confused by a reader of the artifact alone.
relay_pull_diagnostic_gate="redacted"
if [ "$relay_pull_diagnostic_count" -eq 0 ]; then
  echo "relay_pull_diagnostic_count=0 with the gate SET to '${relay_pull_diagnostic_gate}': the pull-path instrumentation did not fire on a run that pulled" >&2
  exit 1
fi

{
  echo "seed=$seed"
  echo "invite_finish=$invite_finish_value"
  echo "invite_code_lines=1"
  echo "redeem_fp_equals_alice_identity=true"
  echo "accept_fp_equals_bob_identity=true"
  echo "handshake_complete_before=$hs_complete_before"
  echo "handshake_complete_after=$hs_complete_after"
  echo "qsp_pack_ok_count=$qsp_pack_ok_count"
  echo "qsp_unpack_ok_count=$qsp_unpack_ok_count"
  echo "recv_commit_count=$recv_commit_count"
  echo "recv_commit_alice=$recv_commit_alice"
  echo "recv_commit_bob=$recv_commit_bob"
  echo "producer_ack_count=$producer_ack_count"
  echo "invite_scan_summary_count=$invite_scan_summary_count"
  echo "relay_unauthorized_count=$relay_unauthorized_count"
  echo "residue_verdict=$residue_verdict"
  echo "residue_attempts=$residue_attempts"
  echo "residue_elapsed_secs=$residue_elapsed"
  echo "residue_lease_bound_secs=$residue_lease_bound"
  echo "residue_detail=$residue_detail"
  echo "relay_push_diagnostic_count=$relay_push_diagnostic_count"
  echo "relay_push_diagnostic_classes=$relay_push_diagnostic_classes"
  echo "relay_pull_diagnostic_gate=$relay_pull_diagnostic_gate"
  echo "relay_pull_diagnostic_count=$relay_pull_diagnostic_count"
  echo "relay_pull_diagnostic_classes_pull=$relay_pull_diagnostic_classes_pull"
  echo "relay_pull_diagnostic_classes_ack=$relay_pull_diagnostic_classes_ack"
  echo "relay_pull_status_classes_pull=$relay_pull_status_classes_pull"
  echo "relay_pull_status_classes_ack=$relay_pull_status_classes_ack"
  echo "relay_pull_error_classes_pull=$relay_pull_error_classes_pull"
  echo "relay_pull_error_classes_ack=$relay_pull_error_classes_ack"
} > "$counts"

{
  echo "status=pass"
  echo "seed=$seed"
  echo "invite_round_trip=ok both_directions"
  echo "invite_finish=$invite_finish_value"
  echo "handshake_complete=${hs_complete_before}->${hs_complete_after} increased"
  echo "qsp_unpack_ok=true both_directions"
  echo "recv_commit_alice=$recv_commit_alice"
  echo "recv_commit_bob=$recv_commit_bob"
  echo "residue_verdict=$residue_verdict"
  echo "residue_elapsed_secs=$residue_elapsed"
  echo "marker_lines=$(wc -l < "$markers" | tr -d ' ')"
  echo "relay_push_diagnostic_count=$relay_push_diagnostic_count"
  echo "relay_pull_diagnostic_gate=$relay_pull_diagnostic_gate"
  echo "relay_pull_diagnostic_count=$relay_pull_diagnostic_count"
  echo "relay_pull_diagnostic_classes_pull=$relay_pull_diagnostic_classes_pull"
  echo "relay_pull_diagnostic_classes_ack=$relay_pull_diagnostic_classes_ack"
  echo "relay_pull_status_classes_pull=$relay_pull_status_classes_pull"
  echo "relay_pull_status_classes_ack=$relay_pull_status_classes_ack"
  echo "normalized_subset_sha256=$(sha256sum "$subset" | awk '{print $1}')"
} > "$summary"

rm -f "$redacted"
exit 0
