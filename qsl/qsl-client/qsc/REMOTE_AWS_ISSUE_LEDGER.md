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
