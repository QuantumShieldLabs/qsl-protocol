# Remote AWS Issue Ledger (NA-0184)

Purpose: secret-safe issue tracking for external-relay two-client validation.

Evidence policy:
- No token values, route token values, or auth headers.
- No secret-bearing URIs.
- No long hex dumps.

## AWS-FILE-001
- Severity: S1
- Scenario: P0 setup guardrail
- Repro: run `receive` with output parent directories not locked down.
- Expected: deterministic fail-closed refusal with actionable remediation.
- Actual: deterministic refusal observed (`error code=unsafe_parent_perms`).
- Evidence markers: `event=error code=unsafe_parent_perms`.
- Suspected anchors: `qsl/qsl-client/qsc/src/main.rs` (receive path output-dir checks), `qsl/qsl-client/qsc/tests/cli.rs` (`unsafe_parent_perms_no_mutation`).
- Fix plan: FIX in runbook/ops discipline (set dirs to 0700 before receive). No production code change required.

## AWS-FILE-002
- Severity: S1
- Scenario: handshake/send endpoint normalization
- Repro: run handshake with relay value in host:port form on AWS path.
- Expected: either deterministic normalization or clear deterministic reject.
- Actual: deterministic reject observed (`error code=relay_endpoint_invalid_host`).
- Evidence markers: `event=error code=relay_endpoint_invalid_host`.
- Suspected anchors: `qsl/qsl-client/qsc/src/main.rs` (`normalize_relay_endpoint` callsites in handshake/send/receive).
- Fix plan: FIX in runbook/ops discipline (always use `https://<relay-host>` for end-to-end AWS runs). Optional follow-on: improve help text.

## AWS-FILE-003
- Severity: S0
- Scenario: file receive integrity under contaminated mailbox state
- Repro: run file transfer after earlier failed/partial runs on the same mailbox tokens.
- Expected: fail-closed integrity handling, then successful clean re-run path.
- Actual: observed fail-closed integrity rejects (`manifest_mismatch`, `qsp_verify_failed`) in contaminated mailbox sequence.
- Evidence markers: `event=file_xfer_reject code=manifest_mismatch`, `event=qsp_unpack code=qsp_verify_failed`.
- Suspected anchors: `qsl/qsl-client/qsc/src/main.rs` (file receive + qsp unpack handling), `qsl/qsl-client/qsc/tests/file_transfer_mvp.rs`.
- Status: MITIGATED (client)
- Observed in this run: operator AWS re-run pending; deterministic harness confirms mitigation path.
- Implemented mitigation:
  - fail-clean on receive integrity reject: transfer state set `FAILED`, partial chunks purged, deterministic remediation emitted.
  - rerun recovery guard: chunk index `0` on failed/stale transfer resets state deterministically (`event=file_xfer_reset reason=rerun_detected`).
  - qsp verify integrity hint emits deterministic remediation marker (`QSC_FILE_INTEGRITY_FAIL ... action=rotate_mailbox_hint`).
- Added tests:
  - `qsl/qsl-client/qsc/tests/aws_file_robustness_na0186.rs`:
    - `integrity_failure_manifest_mismatch_fail_clean_and_rerun_reset`
 - Notes:
   - If this still reproduces on AWS after mailbox/token rotation, file relay-side issue with captured markers and no client bypass.

## AWS-FILE-004
- Severity: S1
- Scenario: medium/large file chunk burst over external relay
- Repro: sustained chunk push for 1MB+ files.
- Expected: bounded retries/backoff with clear state outcome; no silent ambiguity.
- Actual: intermittent `relay_inbox_push_failed` during chunk progression.
- Evidence markers: `event=relay_event action=push_fail`, `event=error code=relay_inbox_push_failed`.
- Suspected anchors: `qsl/qsl-client/qsc/src/main.rs` (relay_inbox_push + file send loop).
- Status: MITIGATED (client)
- Observed in this run: operator AWS re-run pending; deterministic harness confirms bounded retry behavior.
- Implemented mitigation:
  - bounded retry for file chunk/manifest pushes (`max_attempts=3`, backoff `50/100` ms).
  - deterministic retry marker (`QSC_FILE_PUSH_RETRY attempt=<n> backoff_ms=<n> reason=<code>`).
  - fail-closed exhaustion keeps send failed (no false success, no infinite loop).
- Added tests:
  - `qsl/qsl-client/qsc/tests/aws_file_robustness_na0186.rs`:
    - `file_chunk_push_retry_is_bounded_and_deterministic`
    - `file_chunk_push_retry_exhaustion_fails_closed`
- Notes:
  - Server capacity can still dominate under sustained external load; this mitigation prevents unbounded client churn and clarifies operator outcome.

## AWS-FILE-005
- Severity: S2
- Scenario: receiver offline + sender restart semantics
- Repro: send while receiver offline, then restart sender before confirmation arrives.
- Expected: `accepted_by_relay` truthfully shown; no promotion to peer-confirmed until valid confirmation processed.
- Actual: behavior matched expectation in this run.
- Evidence markers: `QSC_DELIVERY state=accepted_by_relay`, no premature `peer_confirmed`.
- Suspected anchors: `qsl/qsl-client/qsc/src/main.rs` delivery state mapping.
- Fix plan: no fix required; keep covered by existing semantics tests.

## AWS-FILE-006
- Severity: S2
- Scenario: wrong-device confirmation under primary-only policy
- Repro status: not executed in this AWS run (single trusted device per alias).
- Expected: wrong-device confirm ignored with no mutation.
- Actual: N/A for this run.
- Fix plan: FILE_NA if external multi-device AWS validation is required in this phase; core deterministic coverage already exists in local tests.

## AWS-ONB-001
- Severity: S1
- Area: onboarding
- Scenario: strict vs balanced trust progression
- Repro: set trust mode strict, run add+verify, then send without trust.
- Expected: verified-only, send blocked until explicit trust.
- Actual: deterministic verified-only + no_trusted_device observed.
- Evidence markers: `QSC_TRUST_PROMOTION result=verified_only reason=strict_mode`, `QSC_SEND_BLOCKED reason=no_trusted_device`.
- Suspected anchors: `qsl/qsl-client/qsc/src/main.rs` (`contacts_device_verify`, `tui_msg_autotrust_first_use`).
- Fix direction: FIXED (client) in NA-0187.

## AWS-ONB-002
- Severity: S1
- Area: onboarding
- Scenario: balanced trust progression
- Repro: set trust mode balanced, run add+verify.
- Expected: verified match promotes to trusted without separate trust command.
- Actual: deterministic auto-promotion observed.
- Evidence markers: `QSC_TRUST_PROMOTION result=trusted reason=verified_match ... mode=balanced`.
- Suspected anchors: `qsl/qsl-client/qsc/src/main.rs` (`contacts_device_verify`, TUI verify paths).
- Fix direction: FIXED (client) in NA-0187.

## AWS-ONB-003
- Severity: S1
- Area: requests
- Scenario: unknown inbound sender handling
- Repro: receive pull from unknown alias with non-decodable inbound data.
- Expected: request created, no trust escalation.
- Actual: request marker created; accept keeps contact untrusted/discovered.
- Evidence markers: `QSC_CONTACT_REQUEST action=created`, `QSC_CONTACT_REQUEST action=accept`, blocked send remains `no_trusted_device`.
- Suspected anchors: `qsl/qsl-client/qsc/src/main.rs` (`receive_pull_and_write`, contact request helpers).
- Fix direction: FIXED (client) in NA-0187.

## AWS-ONB-004
- Severity: S2
- Area: requests
- Scenario: request block action
- Repro: block inbound request alias.
- Expected: alias blocked/revoked and future send/receive remains fail-closed.
- Actual: deterministic block action available; requires full AWS matrix rerun for external confirmation.
- Evidence markers: `QSC_CONTACT_REQUEST action=block`.
- Suspected anchors: `qsl/qsl-client/qsc/src/main.rs` (`contacts_request_block`, TUI requests block path).
- Fix direction: MITIGATED (client), validate in AWS operator pass.

## AWS-R2-001
- Severity: S2
- Area: files
- Repro steps:
  - fresh isolated Alice/Bob state using `/tmp/qsc-aws-round2.env`
  - add/verify/trust both sides, complete handshake, exchange one message, then send a small file Bob -> Alice
  - sender uses `--to alice`; receiver pulls with `--from bob`
- Expected:
  - receiver verifies the manifest and completes the file receive
  - sender progresses `accepted_by_relay` -> `awaiting_confirmation` -> `peer_confirmed`
- Actual before fix:
  - sender reached `accepted_by_relay` and `awaiting_confirmation`
  - receiver failed with `manifest_mismatch` and executed fail-clean purge
- Secret-safe evidence markers:
  - before fix: `QSC_FILE_DELIVERY state=accepted_by_relay`, `QSC_FILE_DELIVERY state=awaiting_confirmation`, `QSC_FILE_INTEGRITY_FAIL reason=manifest_mismatch action=purge_partials`
  - after fix: `event=file_xfer_manifest id=<redacted> ok=true`, `event=file_xfer_complete id=<redacted> ok=true`, `QSC_FILE_DELIVERY state=peer_confirmed`
- Suspected code anchors:
  - `qsl/qsl-client/qsc/src/main.rs:12063`
  - `qsl/qsl-client/qsc/src/main.rs:12390`
  - `qsl/qsl-client/qsc/src/main.rs:12628`
- Status after this directive: FIXED
- Fix summary:
  - manifest hashing no longer includes the local peer label, so sender and receiver derive the same manifest hash even when local alias strings differ (`alice` vs `bob`)
- Deterministic test lock:
  - added `qsl/qsl-client/qsc/tests/aws_r2_file_integrity_na0189.rs`
  - retained fail-clean coverage in `qsl/qsl-client/qsc/tests/aws_file_robustness_na0186.rs`

## AWS-R2-002
- Severity: S3
- Area: tui
- Repro steps:
  - fresh isolated client state
  - configure relay endpoint and token-file in headless TUI
  - run `/relay test` followed by `/exit`
- Expected:
  - relay test should use the same probe/auth path as live traffic and return a deterministic actionable result
- Actual before fix:
  - headless `/relay test` returned a generic error path even when later AWS traffic succeeded
- Secret-safe evidence markers:
  - before fix: `event=tui_cmd_result kind=err command=relay_test`
  - after fix: `QSC_TUI_RELAY_TEST result=started code=pending`, `QSC_TUI_RELAY_TEST result=ok code=relay_authenticated`, `event=tui_relay_test_done ok=true reason=relay_authenticated`
- Suspected code anchors:
  - `qsl/qsl-client/qsc/src/main.rs:657`
  - `qsl/qsl-client/qsc/src/main.rs:2882`
  - `qsl/qsl-client/qsc/src/main.rs:6611`
  - `qsl/qsl-client/qsc/src/main.rs:10217`
- Status after this directive: FIXED
- Fix summary:
  - headless `/relay test` now runs the real probe helper, waits for completion before exit, and emits explicit `QSC_TUI_RELAY_TEST result=<...> code=<...>` markers
- Deterministic test lock:
  - extended `qsl/qsl-client/qsc/tests/tui_relay_config.rs` with authenticated and unauthorized local probe tests

## NA-0190 — AWS TUI Command-Surface Audit (Two-Client)

## AWS-TUI-001
- Severity: S2
- Area: tui
- Exact repro steps:
  - create fresh isolated Alice/Bob configs
  - initialize identity with the default CLI label (`self`)
  - configure relay endpoint + token-file in headless TUI
  - add/verify contacts in headless TUI
  - run TUI handshake without setting `QSC_SELF_LABEL`
- Expected vs actual before fix:
  - expected: TUI handshake uses the same default self identity as CLI identity setup
  - actual: TUI handshake defaulted to `peer-0`, producing `identity_mismatch` / `peer_mismatch` on clean AWS onboarding
- Secret-safe evidence markers:
  - before fix: `event=identity_mismatch`, `event=error code=peer_mismatch`, `event=handshake_reject reason=peer_mismatch`
  - after fix: `event=handshake_send msg=B1`, no `peer_mismatch` in the same TUI handshake sequence
- Suspected code anchors:
  - `qsl/qsl-client/qsc/src/main.rs:2460`
  - `qsl/qsl-client/qsc/src/main.rs:4927`
  - `qsl/qsl-client/qsc/tests/tui_relay_config.rs:776`
- Fix direction: client fix
- Status after this directive: FIXED
- Deterministic test lock:
  - `qsl/qsl-client/qsc/tests/tui_relay_config.rs:776` `tui_handshake_uses_default_self_identity_label`

## AWS-TUI-002
- Severity: S2
- Area: tui
- Exact repro steps:
  - create fresh isolated Alice/Bob configs and fresh inbox route tokens
  - configure relay endpoint + token-file in headless TUI
  - add/verify contacts in headless TUI
  - run `/messages select <peer>`, `/handshake init`, `/handshake poll`, `/handshake poll`, `/handshake poll`
- Expected vs actual:
  - expected: clean TUI handshake completes `A1 -> B1 -> A2` and establishes a session
  - actual: after `A1` and `B1` succeed, the Bob-side third step still fails with `handshake_reject reason=decode_failed`; CLI handshake succeeds on the same AWS relay pattern
- Secret-safe evidence markers:
  - clean AWS rerun after `AWS-TUI-001` fix: `event=handshake_send msg=A1`, `event=handshake_send msg=B1`, `event=handshake_reject reason=decode_failed`
- Suspected code anchors:
  - `qsl/qsl-client/qsc/src/main.rs:16285`
  - `qsl/qsl-client/qsc/src/main.rs:16308`
  - `qsl/qsl-client/qsc/src/main.rs:16489`
- Fix direction: follow-on NA
- Status after this directive: OPEN
- Follow-on NA candidate:
  - Title: `NA-0191 — TUI Handshake AWS Decode-Failure Root Cause + Deterministic Lock`
  - Acceptance: reproduce on fresh AWS mailboxes, identify whether the initiator is pulling a non-HS2 payload or decoding the wrong envelope class, fix or file with deterministic local harness coverage

## AWS-FILE-007
- Severity: S2
- Area: files
- Exact repro steps:
  - create fresh isolated Alice/Bob configs and fresh inbox route tokens
  - complete clean onboarding, trust, handshake, and a successful small-file roundtrip
  - send one medium file Bob -> Alice and receive on Alice with `complete-only` file confirm behavior
- Expected vs actual:
  - expected: medium file receives cleanly on a fresh mailbox and advances to completion markers
  - actual: receiver fails with `QSC_FILE_INTEGRITY_FAIL reason=qsp_verify_failed action=rotate_mailbox_hint` on a clean AWS rerun
- Secret-safe evidence markers:
  - `QSC_DELIVERY state=accepted_by_relay`
  - `QSC_FILE_INTEGRITY_FAIL reason=qsp_verify_failed action=rotate_mailbox_hint`
  - `event=qsp_unpack code=qsp_verify_failed ok=false`
  - `event=error code=qsp_verify_failed`
- Suspected code anchors:
  - `qsl/qsl-client/qsc/src/main.rs:12349`
  - `qsl/qsl-client/qsc/src/main.rs:13101`
  - `qsl/qsl-client/qsc/src/main.rs:17553`
- Fix direction: follow-on NA
- Status after this directive: OPEN
- Follow-on NA candidate:
  - Title: `NA-0192 — AWS Medium-File Integrity Failure (qsp_verify_failed) Root Cause`
  - Acceptance: reproduce on fresh AWS route tokens, determine whether failure is client chunk assembly / envelope verification / relay payload corruption, then fix client-side with deterministic tests or file a relay-boundary issue with proof
