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
- Fix plan: FILE_NA for robustness hardening.
- Follow-on acceptance template:
  - Scope: `qsl/qsl-client/qsc/src/**`, `qsl/qsl-client/qsc/tests/**`
  - Invariants: fail-closed integrity, no false peer-confirmed, deterministic markers preserved
  - Tests: deterministic contaminated-mailbox replay scenario + clean-mailbox recovery scenario
  - Evidence: local gates + CI + leak counts

## AWS-FILE-004
- Severity: S1
- Scenario: medium/large file chunk burst over external relay
- Repro: sustained chunk push for 1MB+ files.
- Expected: bounded retries/backoff with clear state outcome; no silent ambiguity.
- Actual: intermittent `relay_inbox_push_failed` during chunk progression.
- Evidence markers: `event=relay_event action=push_fail`, `event=error code=relay_inbox_push_failed`.
- Suspected anchors: `qsl/qsl-client/qsc/src/main.rs` (relay_inbox_push + file send loop).
- Fix plan: FILE_NA for bounded retry/backoff policy and deterministic marker mapping; server-side capacity may contribute.
- Follow-on acceptance template:
  - Scope: qsc send path only (no server edits in client NA)
  - Invariants: bounded retries only, no infinite loops, no semantic mislabeling
  - Tests: deterministic injected push-fail sequence validating retry budget and final state

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
