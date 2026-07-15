# NA-0645 Testplan — Retire the qsc TUI (D581)

Lane: NA-0645 (QSL-DIR-2026-07-14-581, D581, operator-approved; begins at D-1268)
Scope: DELETE the qsc TUI surface entirely; RE-HOME the relay token setters to the
CLI FIRST (proven green before any deletion); FILE the three co-deleted TUI-only
security features as ONE ledger item (ENG-0044); KEEP the GUI-relevant
routing/emitters/keys. No core-extraction/crate-split/exit->Result work, no GUI
code, no protocol/crypto/wire change, no `tui.*` key rename.

## 1. The re-home gate (Phase 1 — BEFORE any deletion)

- NEW CLI setters mirroring the existing `relay inbox-set` shape (both were
  TUI-only-settable before this lane):
  - `qsc relay token-set --token <TOKEN>` → vault secret `tui.relay.token`
    (the account-secret auth source);
  - `qsc relay token-file-set --path <PATH>` → vault secret
    `tui.relay.token_file` (canonicalize-or-keep, exactly the TUI's semantics);
    output redacts the path.
- PORTED test: `relay_auth_header.rs::relay_auth_uses_account_token_file_when_env_missing`
  now drives the NEW CLI setter (`cli_set_relay_token_file`) instead of the TUI
  script. Env-token tests unchanged.
- GATE (met): `cargo test -p qsc --test relay_auth_header` = 4/4 green with the
  TUI still fully in-tree (run BEFORE Phase 2) — the capability transfer was
  verified, not assumed.
- The auth resolution ORDER (env → account-secret → token-file,
  `transport::relay_auth_token`) is UNTOUCHED — the re-home changes only how the
  values are SET.

## 2. The deletion (Phase 2)

- `src/tui/` deleted whole; `Cmd::Tui` + `TuiTransport` removed; the main.rs
  dispatch arm, imports, consts, normalize_tui_* helpers, tui_perf_tests, and
  the vault attempt-limit machinery removed; TUI-only helpers removed from
  identity/vault/contacts/transport/output/store; the 4 TUI-only store keys
  removed (plus the orphaned `tui.relay.endpoint` const and
  `account.verification_seed_v1` — TUI-only, dead once the TUI is gone); the
  4 TUI-only Cargo deps dropped from qsc's manifest.
- 43 whole TUI test files deleted + `relay_url_policy.rs` (see §3) + partial
  edits in 6 files (receive_e2e, trust_onboarding_na0187, trust_remediation_na0178,
  rng_failure_residual_surfaces, modularization_na0158, relay_auth_header).
- Structural gate (met): `cargo check -p qsc --all-targets` = 0 errors,
  0 warnings on the final tree.

## 3. The coverage-gap contingency (D581 STOP-condition remedy, exercised once)

- FOUND: `relay_url_policy.rs` (whole file, TUI-driven) was the ONLY test of the
  CORE relay URL policy (`adversarial::route::validate_relay_endpoint_url`:
  loopback-http + https accepted; non-loopback http → QSC_ERR_RELAY_TLS_REQUIRED;
  non-http(s) scheme → relay_endpoint_invalid_scheme) — deleting it would have
  dropped core coverage not independently present.
- REMEDY (per D581: "STOP and re-home it first"): NEW
  `tests/NA_0645_relay_url_policy_cli.rs` ports the matrix to a hermetic CLI
  vehicle (`receive --transport relay --relay <url>` validates the endpoint
  before contact/from handling; an ACCEPTED URL falls through to the
  deterministic `recv_from_required` gate, a REJECTED one dies at the policy
  gate with its own code — no network, no persisted state). 2/2 green; only
  then was `relay_url_policy.rs` deleted. The old test's persisted-endpoint
  no-mutation half died legitimately with the TUI-only persisted-endpoint
  feature.

## 4. The KEEP audit (Phase 3)

- `MarkerRouting::InApp` + `set_marker_routing` + `marker_queue` + the InApp
  branch of `emit_marker` survive; NEW unit test
  `output::inapp_routing_tests::inapp_routing_queues_markers_and_stdout_routing_bypasses_queue`
  (green) keeps the dormant routing off zero coverage.
- Core QSC_TUI_* emitters survive: `emit_tui_named_marker`,
  `emit_tui_contact_request`, `emit_tui_delivery_state_with_device`,
  `emit_tui_file_delivery_with_device`, `emit_tui_receipt_ignored_wrong_device`
  — proven live from the CLI by `receipt_policy_mvp_na0177` (3/3 green).
- The 6 core-read persisted `tui.*` keys survive verbatim: tui.relay.token,
  tui.relay.token_file, tui.relay.inbox_token, tui.trust.mode, tui.receipt.*
  (mode/batch_window_ms/jitter_ms), tui.file_confirm.mode.
- Named-KEEP items whose only caller was the TUI (the `handshake_init`/`handshake_poll`
  wrappers; the `VaultSession` session API; `RelaySendOutcome.action/.delivered`)
  are KEPT with `#[allow(dead_code)]` + a D581 marker comment — dormant by
  decision, not deleted.

## 5. The merge gate (Phase 6)

- Full `cargo test -p qsc` green LOCALLY (the suite skips on PRs) on the final
  tree — see the as-built §5 for the run numbers.
- The NA-0640 full-stack e2e (`NA_0640_full_stack_e2e`) green UNCHANGED (zero
  edits to it this lane).
- Scope guard: no core-extraction/crate-split/exit->Result change; no GUI code;
  no protocol/crypto/wire change; no `tui.*` key rename; no KEEP-item deletion;
  auth resolution order unchanged; no qsl-server change; no `formal/`, vectors,
  canonical, `.github` change.
- goal-lint green locally (synthesized event; core paths changed →
  DECISIONS + TRACEABILITY in the diff, tests touched).

## 6. What this plan does NOT cover (stated)

- The three co-deleted security features (attempt-limit, autolock,
  account-destroy) are FILED (ENG-0044), not tested — their contracts live in
  git history with their deleted tests.
- The out-of-scope `apps/qsl-tui` demo client still references the 4 ratatui/
  crossterm deps at the WORKSPACE level (the lockfile keeps the packages for
  it); qsc's own manifest is clean. Hygiene candidate for a later lane.
- Core extraction and the GUI are later lanes; nothing here claims them.
