Status: Supporting
Owner: Protocol Team
Last-Updated: 2026-03-26

# DOC-ATT-007 — Legacy Receive Compatibility Retirement Gate Finalization v0.1.0 DRAFT

Purpose:
- freeze the remaining explicit receive-retirement boundary after merged `W2` send-path removal validation;
- distinguish what current evidence now finalizes from the single remaining gate it does not close; and
- keep qsc, qsl-attachments, qsl-server, and canonical runtime behavior unchanged in this item.

Non-goals:
- no qsc runtime/client changes;
- no qsl-attachments runtime or deployment changes;
- no qsl-server changes;
- no website, `.github`, or workflow changes; and
- no immediate receive-compatibility retirement implementation.

## 1. Evidence baseline

The gate-finalization result in this document is grounded by the following already-merged evidence:

- `NA-0199`
  - proved the live coexistence rule directly from qsc behavior;
  - proved exact `4 MiB` stays on the legacy path, above-threshold sends fail closed without validated attachment configuration, and above-threshold attachment sends remain truthful when configured; and
  - proved legacy-path reject handling does not create false `peer_confirmed`.
- `NA-0200A`, `NA-0201`, and `NA-0201A`
  - proved the attachment path strongly enough for validated deployments across constrained-host, stronger reference-host, and bounded stress/soak lanes; and
  - kept the only degraded required stages on the weak-host / weak-relay legacy threshold path as bounded fail-closed saturation rather than attachment-service correctness failure.
- `NA-0202B`
  - froze the staged `W0` / `W1` / `W2` migration window in `DOC-ATT-004`;
  - froze configuration-only rollback to `W0`; and
  - explicitly retained receive compatibility for already-supported legacy payloads unless a later explicit item authorizes removal.
- `NA-0203` and `NA-0203A`
  - implemented and validated the explicit `W0` / `W1` migration controls;
  - kept rollback and fallback explicit and operator-visible; and
  - proved mixed receive compatibility remains intact during the migration window.
- `NA-0204`
  - justified `W2` send-path implementation only; and
  - explicitly did not authorize legacy receive-path removal.
- `NA-0205` and `NA-0205A`
  - implemented and revalidated the frozen `W2` send-path behavior;
  - kept explicit `w0` as the live rollback/coexistence control for new legacy-sized sends; and
  - re-proved receive compatibility, no-silent-fallback, honest delivery, and route-token/header invariants on current `main`.
- `NA-0206`
  - concluded direct receive-retirement implementation is still not truthful;
  - identified the remaining blockers as the still-live `w0` rollback/coexistence promise plus the missing later explicit receiver-side retirement contract; and
  - promoted `NA-0206A` as the bounded gate-finalization lane rather than a direct runtime-removal item.

## 2. Remaining gate inventory

| Gate | Status before `NA-0206A` | Current proof | `NA-0206A` result | Still missing |
| --- | --- | --- | --- | --- |
| Interaction with live `W0` rollback/coexistence | Partial | `DOC-ATT-004`, `DOC-ATT-005`, `NA-0205`, `NA-0205A`, the local runbook, and current qsc tests all keep `w0` explicit and live on current `main` | Section 4 freezes that legacy receive remains load-bearing while `w0` stays live | A later explicit item must retire or replace the live `w0` promise before runtime receive removal is truthful |
| Receiver-side contract for already-supported legacy payloads | Partial | `DOC-CAN-005` still defines `file_chunk` / `file_manifest` as a valid legacy path family, and `DOC-ATT-004` / `DOC-ATT-005` kept those payloads receivable through `W1` / `W2` | Section 4 freezes the current contract: already-supported legacy payloads remain supported receive inputs while `w0` remains available on current `main` | The post-`w0` replacement contract still needs a later explicit item; this document does not invent it |
| Operator-visible rollback/fallback expectations | Satisfied | `DOC-ATT-004`, `DOC-ATT-005`, `NA-0205`, `NA-0205A`, qsc help, and the local runbook already keep rollback/fallback explicit | Reused unchanged in Section 4 | Nothing additional is missing here before the final post-`w0` gate |
| No-silent-break obligations for a future runtime-removal lane | Partial | Current docs/tests already prove fail-closed missing-service behavior, no silent fallback, honest delivery, and no false `peer_confirmed` on the live coexistence boundary | Section 4 freezes which obligations remain mandatory for any later runtime-removal lane | Those obligations still need runtime proof in the later implementation lane once the final post-`w0` gate is closed |
| Evidence strong enough to justify implementation now | Unsatisfied | `NA-0206` already ruled out direct implementation; current tests/runbooks still rely on mixed legacy receive while `w0` is live | Section 3 narrows the blocker to a single explicit post-`w0` gate | The post-`w0` gate must be finalized before runtime receive retirement can start |

## 3. Option set

| Option | Summary | Result | Why |
| --- | --- | --- | --- |
| `RG0` | Gates can now be fully finalized and receive-retirement implementation can follow next | Rejected | Current `main` still keeps `w0` as a live rollback/coexistence promise, and current policy still lacks a later explicit post-`w0` receiver contract. Advancing directly would invent semantics or break the promised rollback boundary. |
| `RG1` | One explicit gate still blocks receive-retirement implementation | Chosen | Current evidence is strong enough to freeze the current receive boundary truthfully and narrow the blocker to one final post-`w0` gate. |
| `RG2` | Continued support is the more truthful near-term posture | Rejected | The evidence does not support treating legacy receive as the intended longer-lived product posture. It supports one smaller gate-finalization item, not indefinite or intentionally extended support. |

Why `RG1` is the leading candidate:
- it is consistent with `DOC-ATT-004`, `DOC-ATT-005`, and `DOC-ATT-006`;
- it matches the merged `W2` behavior that still keeps `w0` live and mixed legacy receive supported;
- it preserves validated deployment flows and operator-visible rollback/fallback expectations exactly as documented today;
- it avoids dishonest delivery-state or hidden-fallback risk; and
- it requires no new qsl-attachments or qsl-server evidence lane.

## 4. Frozen boundary after `NA-0206A`

The current receive-retirement boundary is now frozen as follows:

1. Live `w0` keeps legacy receive load-bearing on current `main`.
   - `w0` remains the explicit rollback/coexistence control for new legacy-sized sends on current `main`.
   - While that promise remains live, already-supported legacy `file_chunk` / `file_manifest` payloads remain within the supported receiver contract.
   - Direct runtime receive retirement is therefore out of bounds while `w0` remains available as an operator-visible coexistence mode.

2. The current receiver-side contract is explicit and unchanged.
   - `attachment_descriptor` payloads remain the supported attachment-plane receive path.
   - `file_chunk` / `file_manifest` remain the supported legacy receive path family while current policy keeps `w0` live.
   - This item does not invent a new post-`w0` support window, expiry rule, or hidden compatibility mode.

3. Rollback and fallback expectations remain exactly as they are today.
   - `w0` remains the only explicit rollback/coexistence restore for new legacy-sized sends.
   - `w2` remains the validated-deployment default for new legacy-sized sends when validated attachment configuration is present.
   - Missing validated attachment configuration still fails closed with `attachment_service_required`.
   - Attachment-path failures still must not retry silently on the legacy path.
   - No hidden receive-mode flip is authorized.

4. The later runtime-removal lane is still gated.
   - A later explicit item must first retire or replace the live `w0` rollback/coexistence promise and freeze the post-`w0` receiver-side contract for already-supported legacy payloads.
   - Only after that later item is merged may a runtime-removal lane implement legacy receive retirement.
   - That later runtime-removal lane must preserve deterministic reject/no-mutation behavior, honest delivery milestones, no false `peer_confirmed`, no capability-like secrets in canonical URLs, no route-token/header regression, unchanged qsl-server transport-only posture, and unchanged qsl-attachments opaque ciphertext-only posture.

5. The exact remaining gate is now singular and explicit.
   - `FG1` — finalize the post-`w0` receive-retirement boundary strongly enough that legacy receive can be retired without contradicting the current live rollback/coexistence promise.

## 5. Decision

Chosen result:
- `GR1` — one explicit gate still blocks implementation and needs one smaller follow-on item.

Exact reason:
- Current evidence is sufficient to freeze the live receive boundary truthfully.
- Current evidence is not sufficient to freeze a post-`w0` runtime-removal contract without semantic invention.
- The remaining blocker is therefore not attachment-service immaturity, relay drift, or another `W2` cleanup pass; it is the still-unresolved post-`w0` receive-retirement boundary.

Why this is the smallest truthful decision:
- it does not claim receive-retirement implementation is ready when `w0` is still live;
- it does not pretend legacy receive should remain the near-term product posture indefinitely;
- it keeps current operator-visible rollback/fallback guarantees unchanged; and
- it narrows the next blocker to one bounded docs/evidence item instead of widening into qsc runtime, qsl-attachments, or qsl-server changes.

## 6. Queue implication

The next truthful queue item after this gate-finalization result is:
- `NA-0206C — Receive Compatibility Retirement Final Gate`

That lane should:
- decide whether and how the live `w0` rollback/coexistence promise can be retired or replaced; and
- freeze the post-`w0` receiver-side contract needed before runtime receive retirement becomes implementable.

It should not collapse into:
- direct qsc runtime receive-retirement work;
- qsl-attachments redesign;
- qsl-server compatibility redesign; or
- intentional long-term continued support without a new evidence-backed posture decision.

## References

- `docs/design/DOC-ATT-004_Legacy_In_Message_Deprecation_Readiness_v0.1.0_DRAFT.md`
- `docs/design/DOC-ATT-005_Legacy_In_Message_Final_Removal_Decision_v0.1.0_DRAFT.md`
- `docs/design/DOC-ATT-006_Legacy_Receive_Compatibility_Retirement_Decision_v0.1.0_DRAFT.md`
- `docs/canonical/DOC-CAN-005_QSP_Attachment_Descriptor_and_Control_Plane_v0.1.0_DRAFT.md`
- `tests/NA-0199_legacy_transition_validation.md`
- `qsl/qsl-client/qsc/tests/attachment_streaming_na0197c.rs`
- `qsl/qsl-client/qsc/tests/cli.rs`
- `qsl/qsl-client/qsc/tests/route_header_migration_docs_na0195a.rs`
- `qsl/qsl-client/qsc/LOCAL_TWO_CLIENT_RUNBOOK.md`
- `qsl/qsl-client/qsc/REMOTE_TWO_CLIENT_AWS_RUNBOOK.md`
- `qsl-attachments/README.md`
- `qsl-attachments/TRACEABILITY.md`
- `qsl-server/README.md`
- `qsl-server/TRACEABILITY.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
