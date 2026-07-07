Goals: G5 (primary), supports G1, G2, G3, G4

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# NA-0612 — Error/Retry Normalization Review (read-only audit)

## Summary

NA-0612 is a read-only audit (DOC-G5-005 §9 rank 3 / ledger ENG-0006) executed under
directive QSL-DIR-2026-07-07-549 (D549) as a LITE-CEREMONY lane (single PR, single
decision D-1222). It reviews whether distinct internal failure causes in qsc are
externally distinguishable by reject-code granularity, timing, or retry/backoff shape
beyond the deterministic reject taxonomy. No source was changed; every fix (if any)
returns as its own lane.

Result classification: `ERROR_RETRY_DISTINGUISHABILITY_LOCAL_ONLY_NO_REMOTE_ORACLE`.

No P0/P1 was substantiated. Within qsc the reject taxonomy is local-only and the retry
path is cause-agnostic; there is no remotely-observable failure-cause oracle beyond
the send/fetch timing/size metadata already documented in NA-0608 / DOC-G5-005. This
is a bounded internal review, not an external/formal review and not an unlinkability
or metadata-free claim.

## Required Markers

- NA0612_D1220_CONSUMED_OK
- NA0612_D1221_CONSUMED_OK
- NA0612_FRESH_QWORK_PROOF_OK
- NA0612_CURRENT_MAIN_HEALTH_OK
- NA0612_D1222_ABSENT_BEFORE_IMPLEMENTATION_OK
- NA0612_LITE_CEREMONY_CERTIFIED_OK
- NA0612_READ_ONLY_NO_SOURCE_MUTATION_OK
- NA0612_OBSERVATION_CHANNEL_MODEL_APPLIED_OK
- NA0612_REJECT_TAXONOMY_LOCAL_ONLY_OK
- NA0612_RETRY_CAUSE_AGNOSTIC_OK
- NA0612_NO_WIRE_NACK_OK
- NA0612_NO_REMOTE_FAILURE_CAUSE_ORACLE_OK
- NA0612_ENG0006_RESOLVED_OK
- NA0612_ENG0009_DETERMINISTIC_JITTER_FILED_OK
- NA0612_SERVICE_SIDE_SCOPE_NOTE_RECORDED_OK
- NA0612_SUCCESSOR_NA0613_ENG0007_SELECTED_OK
- NA0612_PRIVATE_MATERIAL_SCAN_OK
- NA0612_RESULT_CLASSIFICATION_SELECTED_OK

## Qwork, Queue, And Main Gates

Fresh operator-run qwork proof for lane NA-0612 from `2026-07-07T03:14:37Z`
(regenerated via the drop-NA-0611/qwork-NA-0612 workflow) verified before mutation;
HEAD == origin/main == main == `daef5d6532fb`; worktree clean; READY_COUNT 1 with
READY NA-0612; D-1220 once, D-1221 once, D-1222 absent.

## Inheritance

D-1220 (NA-0610 closeout) and D-1221 (NA-0611) consumed once each and Accepted. The
review uses the DOC-G5-002/004 leakage-inventory / logging-contract frame.

## Observation-Channel Model

Each failure/reject/retry surface is classified by who can observe it:
- Local-only: emitted to the operator's stdout/logs (deterministic markers). Not a
  metadata leak; this is the intended fail-closed taxonomy relied on by tests.
- Remotely observable: transmitted on the wire, or inferable from response
  presence/timing or retry/backoff shape by a peer, relay, or attachment service.
Only the remotely-observable set is a hostile-analyst distinguishability concern.

## Findings

### Verified sound / local-only (not a remote leak)

- Receive rejects `recv_reject_parse` / `recv_reject_size`
  (`qsc/src/main.rs:2428-2437`): emitted via `emit_marker` + `print_error_marker`
  during client-side `receive` parsing; no response is transmitted on the wire.
  Local-only.
- Attachment rejects `REJECT_ATT_*` (e.g. `REJECT_ATT_CIPHERTEXT_PRECHECK`,
  `attachments/mod.rs:1253-1261`): client-side precheck/decrypt errors surfaced as
  local markers; not sent to the attachment service. Local-only.
- Handshake rejects `REJECT_QSC_HS_*`: local fail-closed markers.
- All `reason` strings (`protocol_active_or_reason_for_send_peer`,
  `emit_cli_send_blocked`, `trust_allows_peer_send_strict`): local CLI/trust-gate
  feedback, not a wire NACK.

### Retry is cause-agnostic and uniform

- `bounded_retry` (`main.rs:2649`) operates on a unit error (`Result<(), ()>`) that
  carries no failure cause; the backoff is `RETRY_BASE_MS` plus an attempt-count-derived
  jitter, capped at `RETRY_MAX_MS`. The retry shape depends only on attempt count, not
  on why the operation failed, so it exposes no cause-dependent retry oracle.

### No wire-sent reason NACK

- `send_wire_canon` emits the uniform canonical envelope; on failure the client fails
  closed locally and transmits no reason-carrying error response. There is no
  wire-observable failure-cause channel.

### ENG-0009 (P3, optional defense-in-depth) — deterministic retry backoff jitter

- Severity: P3 (defense-in-depth; NOT a failure-cause oracle)
- Surface: `bounded_retry` jitter is deterministic (attempt-count-derived), not
  randomized.
- Why it matters: in the current model retry attempts are not remotely observable and
  the retry is cause-agnostic, so this leaks no failure cause. It is recorded only
  because, in a hypothetical live send-retry-to-relay scenario, a deterministic
  backoff is more predictable than a randomized one; any concern there ties to the
  send-timing metadata already tracked (NA-0608 / DOC-G5-005), not to failure-cause
  distinguishability.
- Fix direction: randomize the retry jitter if and only if send-retry-to-relay timing
  is ever made a live mitigation target; otherwise no action.

## Scope Note (out of qsc-repo scope)

Service-side error/reject normalization in qsl-server and qsl-attachments
(`REJECT_QATTSVC_*` / service HTTP responses) is remotely observable by design and is
outside this repository's scope. A follow-up error-normalization review is recommended
in those repositories, referenced against the in-repo attachment contract; it is not
part of qsc and not implemented here.

## No P0/P1 Escalation

No exploitable remotely-observable failure-cause oracle was substantiated within qsc;
no stop-and-escalate was triggered.

## ENG-0006 Resolution And Successor

Ledger ENG-0006 is resolved-into-findings: within qsc, error/reject/retry behavior is
local-only and cause-agnostic; no remotely-observable failure-cause oracle. The
residual is the optional P3 ENG-0009 plus the service-side scope note. Because no
remotely-observable distinguishability finding warrants a normalization lane, the
successor proceeds as planned to NA-0613 = DOC-G5-005 §9 rank 4 / ENG-0007
(attachment-plane metadata mitigation feasibility).

## Boundary And Claim

This lane mutated only docs/evidence/ledger/governance paths; it changed no `.rs`,
test, Cargo, workflow, spec, `.claude`, or hook file, and applied no fix. No
runtime/LAN action occurred. No endpoint, port, token, capability, key, seed,
plaintext, ciphertext body, or raw private material is published. No public-readiness,
production-readiness, security-completion, crypto-complete, metadata-free,
unlinkability, traffic-analysis-resistant, or bug-free claim is made; "local-only" and
"cause-agnostic" are scoped to the qsc surfaces read.
