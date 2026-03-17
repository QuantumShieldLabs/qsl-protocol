# NA-0199 Legacy Attachment Path Transition + Validation

## Scope
- Validate the live coexistence rule between the legacy `<= 4 MiB` in-message file path and the `qsl-attachments` service-backed attachment path.
- Decide the truthful next blocker without changing canonical semantics.
- Keep `qsl-attachments` and `qsl-server` read-only unless a concrete service defect is proven.

## Pre-work transition options

| Option | Summary | Result | Reason |
| --- | --- | --- | --- |
| `T0` | Keep the current coexistence rule unchanged for now | Chosen short-term posture | Current behavior is correct and explicit, but the next blocker is service operational maturity rather than client default-path promotion. |
| `T1` | Make the attachment path the default above the threshold while keeping the legacy path as fallback | Rejected for now | The client path is working, but `qsl-attachments` is still the public single-node local-disk runtime with only minimal CI/protection and no deployment/operational contract. |
| `T2` | Begin deprecating the legacy in-message path entirely | Rejected | No proof yet supports deprecation without first hardening service deployment/operations and making the promotion contract explicit. |
| `T3` | Treat coexistence as long-term/indefinite posture | Rejected | The current split is validated as a temporary truthful rule, not a frozen long-term product posture. |

## Current coexistence rule
- Legacy in-message path remains authoritative for `<= 4 MiB`.
- Attachment service path is used only above `<= 4 MiB` and only when `--attachment-service` is supplied.
- `accepted_by_relay`, attachment/service acceptance, and `peer_confirmed` remain distinct milestones on both path families.

## Direct validation added in NA-0199
- `legacy_path_roundtrip_rejects_then_confirms_without_false_peer_confirmed`
  - proves a small-file send still stays on the legacy path even when `--attachment-service` is configured
  - proves a legacy receive reject on `size_exceeds_max` does not advance durable receive state and does not cause false `peer_confirmed`
  - proves a later valid legacy receive emits completion confirmation and only then yields `peer_confirmed`
- `threshold_boundary_and_service_requirement_are_explicit`
  - proves a file of exactly `4 MiB` still uses the legacy path
  - proves a file of `4 MiB + 1` rejects without `--attachment-service`
  - proves the same above-threshold file uses the attachment path when `--attachment-service` is supplied

## Reused regression proof surfaces
- `attachment_streaming_na0197c`
  - service-backed `> 4 MiB` send/receive/confirm path
  - attachment reject/no-mutation behavior
  - secret-safe output checks
- `route_header_migration_docs_na0195a`
  - route-token migration integrity remains intact
- `relay_auth_header`, `tui_relay_config`, `qsp_qse_onwire`, `handshake_mvp`, `identity_secret_at_rest`
  - no regression across relay auth, config, on-wire behavior, handshake flow, or at-rest fail-closed behavior

## Readiness decision
- `R1` chosen.
- Reason:
  - The coexistence rule is now validated directly.
  - The blocker to default-path promotion is not client correctness; it is the operational/deployment maturity of `qsl-attachments`.
  - `qsl-attachments` still truthfully describes itself as the current single-node local-disk runtime, with minimal `rust` CI and branch protection only.
- Implication:
  - The next truthful item is `NA-0200 — qsl-attachments Deployment / Operational Hardening Contract`.

## qsl-attachments correction decision
- No qsl-attachments runtime corrections needed.
- No runtime defect violating `DOC-CAN-006` was proven during coexistence validation.

## Sources of truth
- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/src/cmd/mod.rs`
- `qsl/qsl-client/qsc/tests/attachment_streaming_na0197c.rs`
- `tests/NA-0197C_attachment_client_evidence.md`
- `tests/NA-0198_runtime_hardening_evidence.md`
- `qsl-attachments/README.md`
- `NEXT_ACTIONS.md`
- `TRACEABILITY.md`
