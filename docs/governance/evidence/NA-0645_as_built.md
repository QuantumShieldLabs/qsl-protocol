# NA-0645 As-Built — Retire the qsc TUI (D581, D-1268)

Lane: NA-0645 per QSL-DIR-2026-07-14-581 (D581, operator-approved), seated by
promotion PR #1569. Base main `9018ae4f` (= `d7bc5f3d` + the NA-0645 promotion's
NEXT_ACTIONS-only change — the normal seating pattern, confirmed at Phase 0).
Product decision (operator, 2026-07-14): the GUI is the only end-user UI; the CLI
stays a thin test-harness/operator surface; the TUI — redundant, and a correctness
hazard (its receive loop was a simplified FORK lacking the NA-0644 ack-lease/dedup
durability) — is retired. First lane of the core-extraction/GUI phase.

## 1. Phase 0 (health)

- HEAD == origin/main == `9018ae4f` (fresh `git ls-remote`, not the mirror);
  ready_count=1; queue top NA-0645 sole READY.
- DECISIONS highest D-1267 → this lane begins at D-1268. ENG-0040 confirmed
  CLOSED on the ledger; ledger highest was ENG-0043 → this lane files ENG-0044.

## 2. The re-home (Phase 1 — green BEFORE any deletion)

Both relay auth setters were TUI-only-settable (writers only under `src/tui`);
per D581 both were re-homed to the CLI, mirroring the existing `relay inbox-set`
shape (`require_unlocked` gate → `vault::secret_set` → redacted marker):

- `relay token-set --token <TOKEN>` → `tui.relay.token` (account-secret source);
  trims, rejects empty (`relay_token_missing`), output redacts the value.
- `relay token-file-set --path <PATH>` → `tui.relay.token_file`; canonicalize-
  or-keep (exactly the TUI's `set_relay_token_file` semantics); output redacts
  the path (`("path","redacted")`), and the ported test asserts the setter
  output does not leak it.

Ported test: `relay_auth_header.rs:681 relay_auth_uses_account_token_file_when_env_missing`
now drives `relay token-file-set` (helper `cli_set_relay_token_file` replaced
`tui_set_relay_token_file`). Env-token tests unchanged. **The Phase-1 gate ran
with the TUI still fully in-tree: `cargo test -p qsc --test relay_auth_header`
= 4 passed / 0 failed (16.79s)** — capability transfer verified before deletion.
The resolution ORDER (`relay_auth_token`: env → account-secret → token-file) is
character-untouched. The re-home stayed a small setter + test port (2 clap
variants, 2 handler arms, 1 test helper): no STOP.

## 3. The deletion (Phase 2)

- `src/tui/` deleted whole: **18 files, 10,007 lines** (matches the 2026-07-14
  scoping exactly).
- `cmd/mod.rs`: `Cmd::Tui` variant + `TuiTransport` enum removed.
- `main.rs`: the Tui dispatch arm; `mod tui;`/`use tui::*;`; the crossterm +
  ratatui-core/-crossterm/-widgets import blocks; TUI_AUTOLOCK_*/TUI_POLL_*
  consts; `tui_color_enabled`; `normalize_tui_{autolock_minutes,poll_interval_seconds,poll_mode}`;
  `mod tui_perf_tests`; and the whole vault ATTEMPT-LIMIT machinery
  (`vault_attempt_limit_note`, `parse_vault_attempt_limit_config`,
  `parse_vault_failed_unlocks`, `vault_security_state_{load,store,clear_files}`,
  `wipe_vault_file_best_effort`) — co-deleted per D581; see ENG-0044.
- `identity/mod.rs`: `identity_tui_bootstrap_keypair` + the kem/sig sub-helpers
  (both cfg variants) + the two QSC.TUI.BOOTSTRAP.* rng-failure label consts;
  `identity_self_fingerprint` and the orphaned `zeroize_secrets` method
  (TUI-only callers).
- `vault/mod.rs`: `destroy_with_passphrase` (account-destroy) — co-deleted; see
  ENG-0044.
- `contacts/mod.rs`: `tui_resolve_peer_device_target`, `tui_contact_blocked`,
  `tui_enforce_peer_not_blocked`, `emit_tui_contact_flow`,
  `emit_tui_trust_promotion`, `emit_tui_trust_remediation`, plus the discovered
  TUI-only `emit_tui_routing_marker` and `trust_remediation_verify_vs_trusted_hint`.
- `transport/mod.rs`: `fault_injector_from_tui`; the `tui_thread: Option<&str>`
  plumbing (struct field, destructuring, the two `if let Some(thread)` emitter
  branches, the four `tui_thread: None` initializers across main/transport/
  attachments).
- `timeline/mod.rs`: the dead TUI-only wrappers `emit_tui_confirm_policy`,
  `emit_tui_delivery_state`, `emit_tui_file_delivery`,
  `file_delivery_semantic_from_state`, `message_delivery_semantic_from_state_str`
  (the `_with_device` KEEP emitters they wrapped survive with live callers).
- `protocol_state/mod.rs`: the TUI status-pane formatters `qsp_status_parts`,
  `qsp_status_string`, `qsp_status_user_note` (the qsp ENGINE is untouched).
- `adversarial/route.rs` + main.rs: the dead `relay_probe_url` pair and ~19 other
  TUI-status-screen helper fns in main.rs (labels/notes/hashes/fingerprint
  display helpers — full list in the lane diff), all with zero non-TUI callers.
- Store keys: the 4 D581 keys REMOVED (`tui.autolock.minutes`, `tui.poll.mode`,
  `tui.poll.interval_seconds`, `tui.last_command_result`) plus the orphaned
  `tui.relay.endpoint` and `account.verification_seed_v1` consts (TUI-only,
  dead once the TUI is gone — covered by D581's general TUI-only-code clause);
  the vault-security consts/types (`VAULT_SECURITY_CONFIG_NAME`,
  `VAULT_UNLOCK_COUNTER_NAME`, `VAULT_ATTEMPT_LIMIT_MIN/MAX`,
  `QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS`, `VaultSecurityState`,
  `UnlockAttemptOutcome`) removed with the attempt-limit machinery.
- Cargo deps: `crossterm`, `ratatui-core`, `ratatui-crossterm`,
  `ratatui-widgets` dropped from **qsc's** manifest; `cargo check` green
  without them (nothing else in qsc uses them). ⚠ VERIFY-finding, recorded:
  the D581 premise "used ONLY by the TUI" holds at the qsc-crate level but not
  at the WORKSPACE level — the out-of-scope `apps/qsl-tui` (a separate ~820-line
  Linux TUI demo client against the refimpl, workspace member, NOT the qsc TUI)
  also declares all four, so the workspace `Cargo.lock` retains the package
  entries for it (qsc's 4 dependency edges removed). `apps/qsl-tui` is outside
  D581's allowed paths and was NOT touched; its disposition is a hygiene
  candidate for a later lane (operator decision).
- Tests: **44 whole files deleted** (the 43 tui-named/`vault_attempt_limit`/
  `relay_ui_integration`/`aws_tui_handshake_na0191`/`NA_0640_tui_focus_semantics`
  files + `relay_url_policy.rs` — see §4) and **6 partial edits**
  (`receive_e2e.rs` dropped `tui_receive_headless_marks`, its 2 CLI e2e tests
  kept; `trust_onboarding_mainstream_flow_na0187.rs` dropped its 1 TUI-driven
  test of 8 + helper; `trust_remediation_ux_na0178.rs` dropped its 1 TUI-driven
  test of 5 + helper; `rng_failure_residual_surfaces.rs` dropped the 2 QSC.TUI.*
  label tests + helpers, KEEPING the QSC.CONTACT.ROUTE_TOKEN CLI test;
  `modularization_na0158.rs` one-line `--help` "tui" assertion removed;
  `relay_auth_header.rs` the §2 port).
- Structural gate: `cargo check -p qsc --all-targets` = **0 errors, 0 warnings**
  on the final tree.

## 4. The coverage-gap contingency (the D581 STOP-condition remedy, exercised once)

The scoping's "every core behavior has non-TUI coverage" premise FAILED for one
file: `relay_url_policy.rs` (whole-file TUI-driven) was the ONLY test of the
core relay URL policy (`adversarial::route::validate_relay_endpoint_url`,
enforced on every relay transport path: loopback-http + https accepted;
non-loopback http → `QSC_ERR_RELAY_TLS_REQUIRED`; non-http(s) scheme →
`relay_endpoint_invalid_scheme`). Searched: diagnostics.rs (redaction only),
the remote_soak files (drive a python script), zero other assertions of the
policy codes. Per D581's own remedy ("STOP and re-home it first"), the matrix
was PORTED before deletion: NEW `tests/NA_0645_relay_url_policy_cli.rs` drives
`receive --transport relay --relay <url>` (the endpoint is validated before
contact/from handling: an accepted URL falls through to the deterministic
`recv_from_required` gate; a rejected URL dies at the policy gate with its own
code — hermetic, no network). **2/2 green**, then the TUI file was deleted.
The old file's persisted-endpoint no-mutation half died legitimately: the
persisted relay endpoint (`tui.relay.endpoint`) was itself a TUI-only feature;
the CLI takes endpoints per-invocation via `--relay`.

## 5. The KEEP audit (Phase 3)

- **InApp routing** — `MarkerRouting::InApp`, `set_marker_routing`,
  `marker_queue`, the InApp branch of `emit_marker`: present, functional; NEW
  unit test `output::inapp_routing_tests::inapp_routing_queues_markers_and_stdout_routing_bypasses_queue`
  green (routing switch both directions; queued line carries the full formatted
  marker; Stdout routing bypasses the queue).
- **Core QSC_TUI_* emitters** — `emit_tui_named_marker` (output),
  `emit_tui_contact_request` (contacts, reached from transport on inbound),
  `emit_tui_delivery_state_with_device` / `emit_tui_file_delivery_with_device` /
  `emit_tui_receipt_ignored_wrong_device` (timeline, unconditional receive-path
  callers): present; **proven live from the CLI by `receipt_policy_mvp_na0177`
  = 3 passed / 0 failed (59.02s)** (asserts QSC_TUI_RECEIPT mode=off/immediate/
  batched). Vocabulary rename stays a later lane per D581.
- **Persisted `tui.*` keys** — the 6 core-read names survive verbatim in
  `store/mod.rs`: `tui.relay.token`, `tui.relay.token_file`,
  `tui.relay.inbox_token`, `tui.trust.mode`, `tui.receipt.mode`/
  `tui.receipt.batch_window_ms`/`tui.receipt.jitter_ms`, `tui.file_confirm.mode`.
  No rename (data migration = out of scope).
- **Shared core** — `transport::relay_send_with_payload`, the qsp_* engine,
  `handshake_init_with_suite_mode`/`handshake_poll_with_suite_mode`,
  `VaultSession`, contacts load/store, `config_dir`: untouched.
- **KEEP-item nuance, recorded honestly:** three named-KEEP items turned out to
  have had the TUI as their ONLY caller — the 2-line `handshake_init`/`handshake_poll`
  wrappers (the CLI uses the `*_with_suite_mode` core directly), the
  `VaultSession` long-lived-session API cluster (`open_session`,
  `open_session_with_passphrase`, `session_get`, `session_set`,
  `persist_session`, `perf_snapshot`, `secret_set_with_passphrase`,
  `has_process_passphrase` — the CLI unlocks per-command), and the
  `RelaySendOutcome.action`/`.delivered` fields. Per D581's "deleting any KEEP
  item" prohibition they are KEPT, annotated `#[allow(dead_code)]` with a
  "D581 KEEP (NA-0645)" marker comment — dormant by decision, visible to the
  core-extraction lane, which should adjudicate them.

## 6. The FILED ledger item (Phase 4)

**ENG-0044** (P2, one coherent vault/account-protection item): the three
TUI-only security features co-deleted with the TUI — (a) vault failed-unlock
ATTEMPT-LIMIT (wipe-on-repeated-failure), (b) vault idle AUTOLOCK, (c)
ACCOUNT-DESTROY (`destroy_with_passphrase`) — **the GUI phase must not close
without restoring all three or a recorded operator decision to drop each.**
Git history preserves the implementations and their deleted tests document the
contracts. This lane deleted + filed; it did NOT re-home their logic (D581:
that would mix a deletion with a preserve-and-unit-test effort).

## 7. Docs (Phase 5)

- SUPERSEDED banners (history retained, nothing destroyed) on the five TUI
  docs: `QSC_TUI_SPEC.md`, `QSC_TUI_INVARIANTS.md`,
  `QSC_TUI_IMPLEMENTATION_CHECKLIST.md`, `DOC-QSC-001` (TUI Charter),
  `DOC-QSC-005` (TUI Relay Integration).
- DOC-PROG-003 truthed-up (Last-Updated bumped): the "what qsc IS" UI bullet
  now records the retirement; the storage bullet no longer advertises the
  co-deleted failed-unlock wipe limit (owed via ENG-0044); the PQ-status
  sentence no longer references the TUI; the §6 DOC-QSC pointer marks the TUI
  docs superseded with their security-lens principles carried to the GUI.
- Directive-vs-reality deltas, recorded not silently fixed: D581 names
  "DOC-QSC-001/005/007" — **no DOC-QSC-007 exists** (docs/qsc has 001–006);
  D581 also lists DOC-PROG-002 for truth-up — **DOC-PROG-002 contains zero TUI
  mentions** (nothing to change).

## 8. Validation (Phase 6)

- Full `cargo test -p qsc` LOCALLY on the final tree (the merge gate — the
  suite skips on PRs): **see §8a below** (run output preserved).
- NA-0640 e2e: green UNCHANGED, zero edits to the e2e this lane.
- Scope guard: no core-extraction/crate-split/exit->Result work; no GUI code;
  no protocol/crypto/wire change; no `tui.*` key rename; no KEEP-item deletion
  (dormant KEEP items annotated, not removed); auth resolution order unchanged;
  no qsl-server change; no `formal/`, vectors, canonical, or `.github` change
  (the retired relay-ui workflow was already deleted at NA-0640).
- goal-lint: green locally (synthesized `GITHUB_EVENT_PATH`; core `src/**`
  paths changed → DECISIONS.md + TRACEABILITY.md in the diff, tests touched).

### 8a. Final run numbers

- Full `cargo test -p qsc` (local, final tree): **422 passed / 0 failed / 1 ignored (pre-existing, attachment_streaming_na0197c) across all 107 test-result sets, exit 0** — reconciles exactly against the NA-0644 baseline (609/150): minus the 44 deleted TUI test files' tests and the partials' TUI sections (2 of the baseline's 3 ignored lived in the deleted dormant relay_ui_integration.rs), plus this lane's 3 new tests (2 URL-policy CLI + 1 InApp unit).
- NA-0640 e2e: 2 passed / 0 failed (119.17s, dedicated re-run on the final tree; zero e2e edits this lane).
- Structural: `cargo check -p qsc --all-targets` = 0 errors / 0 warnings.
- KEEP proofs: bin unit tests 47/47 (incl. the new InApp test); receipt_policy_mvp_na0177 3/3 (59.02s); relay_auth_header 4/4 (16.79s, run pre-deletion); NA_0645_relay_url_policy_cli 2/2.

## 9. Result classification

**RETIRE_TUI_PASS** — the TUI is deleted (18 src files/10,007 lines + all
TUI-only helpers/consts/keys + 44 test files + 6 partial edits + qsc's 4 TUI
deps); the CLI relay token setters are added with the token-file test ported
and green BEFORE deletion; the three security features are FILED as ENG-0044;
the KEEP items survive (InApp routing + new unit test; core emitters proven by
CLI tests; persisted keys verbatim); TUI docs archived; full suite + NA-0640
e2e green locally. NOT claimed: core extraction (the NEXT lane), restoration of
the ENG-0044 features (the GUI phase), any change to the protocol claim
boundary (UNCHANGED).
