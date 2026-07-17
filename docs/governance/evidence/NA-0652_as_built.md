# NA-0652 As-Built — qsl-server capability document GET /v1/server-info (D588, D-1275)

Goals: G4, G5

## 0. Identity

- Lane NA-0652; directive QSL-DIR-2026-07-17-588 (D588), APPROVED 2026-07-17 with
  F1 resolved to the RELAY_ house form (`RELAY_NAME`,
  `RELAY_ATTACHMENTS_SERVICE_URL`, `RELAY_MIN_CLIENT_VERSION`); amended directive
  sha256 `76496a232a73e0ae04a54192f1021cad22ce5377fcbe3ee8cab695377448d758`.
- Cross-repo per the NA-0642/D578 model: qsl-server CODE (PR #62), spine
  governance (this closeout). Spine decision D-1275; qsl-server decision D-0012.
- Executes DOC-PROG-004 step 2 (locked decision L4). DOC-PROG-004 itself
  byte-untouched (step-1 precedent).

## 1. Phase 0 — CONFIRM-LIVE (2026-07-17)

Spine (qwork checkout `/srv/qbuild/work/NA-0652/qsl-protocol`):
- qwork proof clean: `startup_result=OK`, lane NA-0652, head == origin_main ==
  main == `07c38254` (the #1585 seating merge), worktree/index/untracked clean,
  ready_count=1, queue_top_ready=NA-0652, requested_lane_status=READY,
  shared_target_ready=yes (kv proof at `.qwork/startup.qsl-protocol.kv`).
- D-1274 canonical ×1; D-1275 ABSENT (lane begins there). Anchored
  `Status: READY` ×1 = NA-0652. Disk 50% (< 95% gate); /backup/qsl mounted.
- Main health at `07c38254`: 8/9 push workflows green; `public-ci` FAILED —
  investigated and PROVEN PRE-EXISTING (§7), not gating, not this lane's diff.

qsl-server (FRESH `git clone` to `/srv/qbuild/work/NA-0652/qsl-server` — never
the qbuild mirror):
- HEAD == origin/main == `8e4ea278` — NOT moved past the D588 pin; no STOP.
- D-0011 canonical ×1; D-0012 ABSENT. Routes exactly `/v1/push`, `/v1/pull`,
  `/v1/pull/ack` (src/lib.rs:374–376 pre-lane). ZERO hits for
  `server-info`/`server_info`/`RELAY_NAME`/`RELAY_ATTACHMENTS_SERVICE_URL`/
  `RELAY_MIN_CLIENT_VERSION` across src/, tests/, docs/, packaging/.
- `serde_json = "1"` already a direct dependency — zero dependency changes.
- CI: the single `rust` job (fmt + cargo test + clippy -D warnings) confirmed in
  `.github/workflows/ci.yml`; merge-commit-only per the satellite CLAUDE.md.

## 2. The contract as built (behavior table)

| Relay auth | Request token | Status | Body |
|---|---|---|---|
| open (`RELAY_TOKEN` unset) | none | 200 | full document, `auth.mode:"open"` |
| open | any header | 200 | full document (auth_ok is vacuously true) |
| bearer | missing | 401 | EXACTLY `{"server":"qsl-server","auth":{"mode":"bearer"}}` |
| bearer | wrong | 401 | BYTE-IDENTICAL to the missing-token body (no oracle) |
| bearer | valid | 200 | full document, `auth.mode:"bearer"` |

Full-document fields and value sources (all LIVE, never constants):

| Field | Source |
|---|---|
| `server` | `"qsl-server"` |
| `version` | `CARGO_PKG_VERSION` (0.1.0 at this build) |
| `name` | `RELAY_NAME` env; `""` when unset |
| `api` | `["push_v1","pull_v1","pull_ack_lease_v1"]` |
| `auth.mode` | `"bearer"` iff RELAY_TOKEN set non-empty, else `"open"` |
| `limits.max_body_bytes` | live `AppState.limits` |
| `limits.max_queue_depth` | live `AppState.limits` |
| `retention.ttl_secs` | validated copy of `StoreConfig::retention_ttl_secs` (same `retention_ttl_or_error` path the store applies) |
| `directory.mode` | `"none"` |
| `attachments.service_url` | `RELAY_ATTACHMENTS_SERVICE_URL` env; `null` when unset |
| `kt.mode` | `"none"` |
| `min_client_version` | `RELAY_MIN_CLIENT_VERSION` env; `null` when unset; ADVISORY (unenforced) |

Design note: the retention TTL is copied onto AppState at construction (the same
validated value `Store::open` holds) rather than adding a getter to
`src/store.rs`, keeping every edit inside the D588 allowed-path list. The three
env vars are read in lib.rs at AppState construction — the RELAY_TOKEN env-only
precedent — so `src/main.rs` needed ZERO edits (which also leaves its
`cli_tests` struct literals untouched).

## 3. Diff census (qsl-server PR #62, commit `8c5627e3` off `8e4ea278`)

- Modified (+116/−0, purely additive): `src/lib.rs` (+96/−0, exactly 4 insertion
  hunks: ServerInfoCfg; two AppState fields; the widened constructor
  `new_with_auth_controls_store_and_info` with `new_with_auth_controls_and_store`
  delegating via `ServerInfoCfg::from_env()`; the `server_info` handler + ONE
  router line); `DECISIONS.md` (+8, D-0012); `packaging/systemd/relay.env.example`
  (+9, the three vars commented-out); `packaging/runbook_ubuntu.md` (+3).
- New: `docs/server/DOC-SRV-006_Server_Info_Capability_Contract_v1.0.0_DRAFT.md`
  (the four contract rules in the spec); `tests/na0652_server_info.rs`.
- UNTOUCHED (proven by `git status`/`git diff --name-only`): `src/main.rs`,
  `src/store.rs`, `Cargo.toml`, `Cargo.lock`, every existing test file, every
  existing handler (hunk inspection: no hunk intersects push_message /
  pull_message / ack_messages / auth_ok). `git diff --check` clean.
- The ENG-0039 TRAP held: `auth_ok()` reused AS-IS; the non-constant-time compare
  untouched.

## 4. Test results (all green FIRST run)

`tests/na0652_server_info.rs` — 8 tests:
1. `open_relay_unauthenticated_gets_full_document` — 200, `auth.mode == "open"`.
2. `bearer_missing_token_gets_exact_probe_401` — exact-value equality vs the
   probe literal PLUS explicit key-count guards at BOTH nesting levels (top = 2
   keys; `auth` = 1 key) — the non-vacuous backing for "no config leaked".
3. `bearer_wrong_token_gets_byte_identical_probe` — response BYTES equal for
   missing vs wrong token (no oracle), and the bytes parse to the exact probe.
4. `bearer_valid_token_gets_full_document` — 200, `auth.mode == "bearer"`.
5. `document_values_track_injected_config` — limits 4096/9, TTL 3600, name
   "Ops Relay", service_url, min_client_version all appear verbatim; `api` and
   the "none" modes pinned.
6. `document_optional_fields_are_empty_and_null_safe_when_unset` — `""`, null,
   null.
7. `full_document_top_level_field_set_is_exact` — sorted top-level key set is
   exactly the 11 documented keys (pins the additive baseline).
8. `relay_env_vars_flow_to_document_end_to_end` — the REAL binary
   (`CARGO_BIN_EXE_qsl-server`, `env_clear`): with RELAY_TOKEN + the three
   RELAY_ vars + RETENTION_TTL_SECS=7200 → unauthorized probe exact, authorized
   doc reflects every env value; a bare binary → "open", `""`, null, null.

Full suite: **108 passed / 0 failed across 26 result sets** = the NA-0642
baseline (100/25) + this file (8/1). Raw log:
`/srv/qbuild/tmp/NA0652_server_info_capability_20260717T170500Z/full_suite.log`.

## 5. Gates

- `cargo fmt --all -- --check` clean (after one fmt pass touching ONLY the new
  test file). `cargo clippy --all-targets -- -D warnings` exit 0.
  `cargo check --all-targets` 0 errors / 0 warnings.
- Scope guard both repos: qsl-server file set exactly as §3; spine diff =
  governance/queue/decision records only (NEXT_ACTIONS, DECISIONS, TRACEABILITY,
  journal, this evidence, the testplan, IMPROVEMENT_LEDGER ENG-0046).
  DOC-PROG-004 byte-untouched. No qsc change, no GUI code.

## 6. Owed follow-ups (recorded)

- **ENG-0046 (filed this closeout):** bump the qsc dev-dep pin
  (`qsl/qsl-client/qsc/Cargo.toml:34`, currently `8e4ea278`) past this lane's
  qsl-server merge + re-run the NA-0640 e2e locally — the ENG-0041/D579 shape.
  NOT done in-lane per D588. The pin stays green meanwhile: the route is
  additive and the existing routes are byte-untouched.
- qsc client consumption of server-info = DOC-PROG-004 step 5 (GUI skeleton
  onboarding "test connection") — already on the roadmap, nothing new filed.

## 7. Flagged finding (pre-existing, NOT this lane, outside D588 scope)

Main-push `public-ci` (public-safety job, link-existence section) has failed
since the NA-0651 deletion merge: run 29592889949 at `07c38254` shows
`DENY_HITS_FILES=0`, `HC_COUNT=0`, `TOTAL_MISSING=7` — seven historical docs
still carry live relative links to the deleted
`qsl/qsl-client/qsc-desktop/README.md`:
`docs/demo/DESKTOP_SIDECAR_ADVERSARIAL_STRESS.md`,
`docs/demo/NATIVE_DESKTOP_PACKAGE_SCREENSHOT_READINESS.md`,
`docs/demo/PUBLIC_DEMO_TOUCH_AND_FEEL_READINESS.md`, and the four evidence
audits NA-0250 / NA-0256 / NA-0258 / NA-0264. The same failure exists on the
`b3cfd5df` push (pre-dates this lane and the #1585 seating). Content gates are
clean (DENY/HC = 0) — this is link residue the D587 residual manifest retained
as historical files. Disposition: surfaced to the operator at this closeout;
the natural fix is a 7-link micro-lane repeating the D587-F2 history-prose
repair (this lane's debt rule forbids fixing it here; no ledger filing made
without operator direction — D588's ledger scope names only ENG-0046).

## 8. Stop state

Per the operator instruction: STOPPED at the two PRs (qsl-server #62 + the
spine governance closeout PR), both open for operator review. No merge, no
promotion, no qwork, no successor work.
