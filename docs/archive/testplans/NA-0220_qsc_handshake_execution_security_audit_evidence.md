Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-05

# NA-0220 qsc Handshake Execution Security Audit Evidence

Goals: G4, G5

## Scope

- Repo: `qsl-protocol`
- Lane: `NA-0220`
- Posture: implementation already merged; this closeout is governance-only and introduces no runtime changes
- Implementation PR: #652
- Implementation branch head before merge: `84a8ba62db96`
- Implementation merge SHA: `11b74ddb6c82`
- Implementation mergedAt: `2026-04-05T02:42:14Z`

## Authority Proof

- refreshed `qsl-protocol` checkout `HEAD`, `mirror/main`, bare mirror `main`, and `origin/main` all resolved to `11b74ddb6c82`
- refreshed merged main contains `DECISIONS.md` `D-0372`, the `TRACEABILITY.md` `NA-0220 implementation/evidence` entry, `docs/audit/DOC-AUD-002_qsc_Handshake_Execution_Security_Audit_v0.1.0_DRAFT.md`, and `tests/NA-0220_handshake_security_audit_testplan.md`
- refreshed live queue still showed `READY_COUNT=1` with `NA-0220` as the sole live `READY` item before this closeout
- refreshed sibling-repo proof stayed read-only and truthful: `qsl-server READY=0`, `qsl-attachments READY=0`
- PR `#652` is already `MERGED`; refreshed merged `main` is authoritative for queue truth, and this closeout lane does not mutate that PR

## Findings Summary

- `P0=0`
- `P1=1`
- `P2=1`
- `P3=0`

## Implementation Evidence Source On Main

- `DECISIONS.md` `D-0372`
- `TRACEABILITY.md` `NA-0220 implementation/evidence`
- `docs/audit/DOC-AUD-002_qsc_Handshake_Execution_Security_Audit_v0.1.0_DRAFT.md`
- `tests/NA-0220_handshake_security_audit_testplan.md`

## Acceptance-Proof Surface

- the audit report is checked in on merged `main`
- the findings are mapped to exact surfaces, specs, and minimal remediation shapes in the audit report
- the lane remained read-only and evidence-first throughout implementation and closeout
- no runtime surfaces changed in the merged implementation lane or this governance-only closeout lane

## Implementation / CI Nuance Summary

- the audit content is already on `main`; this closeout only records durable repo-local archive evidence and truthful queue progression
- the highest-priority next lane is the `P1` fail-closed remediation, not the `P2` status-honesty follow-up
- the implementation lane completed with all 34 protected checks green before merge
- this closeout PR is governance-only and introduces no runtime changes

## Exact Findings and Successor Rationale

- the `P1` finding is that current qsc accept paths can still commit durable pending/session state for unknown or unpinned peers before the canonical authenticated-identity/base-handshake contract is satisfied
- the `P2` finding is that local handshake-status surfaces can overstate mutual completion during the initiator-confirm window
- `NA-0221` is the next truthful successor because the authenticated-establishment fail-closed gap is higher severity than the status-honesty issue and must be corrected first

## Closeout Note

- This closeout PR is governance-only.
- No runtime, workflow, protocol, relay, qsc-desktop, qsl-server, or qsl-attachments paths change in this closeout.
- PR `#652` is already merged and is not mutated in this lane.
