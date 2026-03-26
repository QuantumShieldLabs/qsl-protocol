Status: Supporting
Owner: Protocol Team
Last-Updated: 2026-03-26

# DOC-ATT-006 — Legacy Receive Compatibility Retirement Decision v0.1.0 DRAFT

Purpose:
- decide whether the remaining legacy receive-side compatibility may advance directly to a retirement implementation lane after merged `W2` send-path removal;
- distinguish a true implementation-ready result from a smaller remaining gate-finalization result; and
- keep qsc, qsl-attachments, qsl-server, and canonical runtime behavior unchanged in this item.

Non-goals:
- no qsc runtime/client changes;
- no qsl-attachments runtime or deployment changes;
- no qsl-server changes;
- no website, `.github`, or workflow changes; and
- no immediate receive-compatibility retirement implementation.

## 1. Evidence baseline

The decision in this document is grounded by the following already-merged evidence:

- `NA-0199`
  - proved the live coexistence rule directly from qsc behavior;
  - proved legacy `<= 4 MiB` sends, exact `4 MiB`, and above-threshold attachment selection/fail-closed behavior truthfully; and
  - proved legacy receive reject paths do not produce false `peer_confirmed`.
- `NA-0200A`
  - proved constrained-host attachment behavior over the restored canonical relay;
  - proved service-backed sends through `100 MiB` remained contract-faithful; and
  - showed degraded exact-threshold behavior was weak-relay legacy saturation, not qsl-attachments correctness failure.
- `NA-0201`
  - proved the stronger `qatt` reference deployment;
  - proved mixed message + attachment traffic remained bounded on the stronger host; and
  - separated weak-relay legacy saturation from attachment-service correctness.
- `NA-0201A`
  - proved bounded kitchen-sink mixed traffic on `qatt`;
  - proved large files, restart/recovery, concurrency up to `8`, and a `30` minute soak; and
  - left only the weak-host / weak-relay legacy threshold stages as bounded fail-closed saturation.
- `NA-0202B`
  - froze the staged `W0` / `W1` / `W2` migration window in `DOC-ATT-004`;
  - froze configuration-only rollback to `W0`; and
  - explicitly retained receive compatibility for already-supported legacy payloads unless a later explicit item authorizes removal.
- `NA-0203`
  - implemented the `W1` migration stage in qsc with explicit operator-controlled `w0` / `w1` stage selection;
  - kept rollback configuration-only by returning to `w0`; and
  - added mixed receive compatibility coverage for the migration window.
- `NA-0203A`
  - validated and cleaned up the merged `W0` / `W1` lane without changing canonical semantics; and
  - kept rollback and fallback surfaces truthful and explicit for operators.
- `NA-0204`
  - froze the final-removal decision for the legacy send path only;
  - justified `W2` implementation; and
  - explicitly did not authorize legacy receive-path removal.
- `NA-0205`
  - implemented the frozen `W2` send-path behavior in qsc;
  - kept `w0` as the explicit rollback/coexistence control for new legacy-sized sends; and
  - kept receive compatibility intact for both already-supported path families.
- `NA-0205A`
  - revalidated the merged `W2` lane;
  - refreshed deterministic regression evidence without runtime change; and
  - re-proved receive compatibility, no-silent-fallback, honest delivery, and route-token/header invariants.

## 2. Option set

| Option | Summary | Result | Why |
| --- | --- | --- | --- |
| `RC0` | Keep remaining receive compatibility unchanged for now with no new gating artifact | Rejected | The evidence now supports a more explicit result than "leave it alone"; the remaining blocker is specific enough to queue a smaller gate-finalization lane truthfully. |
| `RC1` | Receive-side retirement implementation is now justified | Rejected | Current merged policy still keeps legacy receive load-bearing through explicit `w0` rollback/coexistence and through the retained receive-support boundary frozen in `DOC-ATT-004` / `DOC-ATT-005`. |
| `RC2` | Retirement is still blocked by explicit gates and needs a smaller gate-finalization item | Chosen | This is the smallest truthful next step. The merged evidence is enough to reject direct implementation now, but not enough to skip the remaining gate-finalization work. |
| `RC3` | Keep receive compatibility longer as an intentional product posture | Rejected | Current evidence does not justify permanent or intentionally long-lived legacy receive support as the product end state; it just shows that one remaining policy boundary still needs to be finalized before retirement can be implemented truthfully. |

## 3. Evaluation

### 3.1 Consistency with `DOC-ATT-004` and `DOC-ATT-005`

- `DOC-ATT-004` explicitly states that `W2` retains receive compatibility for already-supported legacy payloads unless a later explicit item authorizes removing it.
- `DOC-ATT-004` also freezes rollback as a configuration-only return to `W0` that preserves receive compatibility for both path families.
- `DOC-ATT-005` explicitly states that the `NA-0205` implementation lane must not collapse into legacy receive-path removal.
- `NA-0205` and `NA-0205A` then implemented and revalidated exactly that boundary rather than superseding it.

Direct receive-retirement implementation would therefore contradict the currently merged policy boundary rather than merely extend it.

### 3.2 What the accumulated evidence proves

- The send-side legacy path is now retired by default for validated deployments under `w2`.
- Explicit rollback/coexistence remains present on current `main` through `w0`.
- Mixed receive compatibility is still an actively tested and documented invariant in the merged `W2` lane.
- qsl-attachments remains an opaque ciphertext-only service, and qsl-server remains a transport-only relay; neither repo contributes a new correctness defect that would independently force receive retirement or justify redesign.

### 3.3 Remaining explicit gates

The remaining blockers are explicit and load-bearing:

1. `G1` — current `w0` rollback/coexistence still makes legacy receive load-bearing on current `main`.
   - `NA-0205` kept `w0` as the explicit rollback/coexistence control for new legacy-sized sends.
   - The merged `W2` regression set still proves that `w0` restores legacy in-message sends and that mixed `w0` legacy receive plus `w2` attachment receive both remain supported.
   - As long as current policy keeps `w0` live, a direct receive-retirement lane would break the promised rollback/coexistence model rather than merely narrow dead code.

2. `G2` — the project has not yet frozen the later-item receiver-side retirement contract for already-supported legacy payloads.
   - `DOC-ATT-004` authorizes receive removal only through a later explicit item.
   - `DOC-ATT-005` explicitly states the `NA-0205` lane did not authorize that removal.
   - `DOC-CAN-005` still treats `file_chunk` / `file_manifest` as the valid legacy path family.
   - Inference from those sources: the project still needs a separate gate-finalization item to define the exact policy boundary that would replace the current "receivable until later explicit authorization" rule before runtime retirement can proceed truthfully.

### 3.4 Required questions answered

1. Is the remaining receive-side legacy compatibility still load-bearing for truthful operation?
   - Yes. Current `main` still promises explicit `w0` rollback/coexistence, and the merged regression set still treats mixed legacy receive as a supported invariant.

2. Is retirement now justified?
   - No. The evidence justifies rejecting direct implementation now, not skipping the remaining gate-finalization work.

3. If not, what exact gate still blocks it?
   - `G1`: explicit `w0` rollback/coexistence still keeps new legacy sends possible on current `main`.
   - `G2`: no later explicit item yet freezes the replacement receiver-side contract for already-supported legacy payloads.

4. If it remains longer, why?
   - Because current policy still requires it for truthful rollback/coexistence and has not yet replaced the current receive-support boundary with a retirement-specific one.

## 4. Decision

Chosen result:
- `D0` — receive-side retirement is not ready; a smaller gate-finalization item must come next.

Exact reason:
- The accumulated evidence is strong enough to decide that direct receive-retirement implementation would be premature and policy-inconsistent today.
- The remaining blocker is not attachment-service immaturity, relay drift, or another missing `W2` validation pass.
- The remaining blocker is the still-live rollback/coexistence boundary plus the absence of the later explicit item that must define the replacement receiver-side contract.

Why this is the smallest truthful decision:
- it does not invent receive-retirement semantics that current policy has not frozen;
- it does not pretend the remaining receive path is dead or non-load-bearing when merged tests and runbooks still rely on it;
- it does not claim receive compatibility should remain indefinitely as product policy; and
- it identifies a bounded docs/evidence successor rather than widening back into qsl-attachments, qsl-server, or another qsc send-path lane.

## 5. Queue implication

The next truthful queue item after this decision is:
- `NA-0206A — Receive Compatibility Retirement Gate Finalization`

That lane should:
- freeze how receive retirement interacts with the still-live `w0` rollback/coexistence boundary; and
- freeze the later-item receiver-side retirement contract required before any runtime removal is truthful.

It should not collapse into:
- direct qsc runtime retirement work;
- qsl-attachments redesign; or
- qsl-server compatibility redesign.

## References

- `docs/design/DOC-ATT-004_Legacy_In_Message_Deprecation_Readiness_v0.1.0_DRAFT.md`
- `docs/design/DOC-ATT-005_Legacy_In_Message_Final_Removal_Decision_v0.1.0_DRAFT.md`
- `docs/canonical/DOC-CAN-005_QSP_Attachment_Descriptor_and_Control_Plane_v0.1.0_DRAFT.md`
- `tests/NA-0199_legacy_transition_validation.md`
- `qsl-attachments/tests/NA-0003_constrained_host_validation_evidence.md`
- `qsl-attachments/tests/NA-0004_reference_deployment_validation_evidence.md`
- `qsl-attachments/tests/NA-0005_stress_soak_chaos_evidence.md`
- `qsl/qsl-client/qsc/tests/attachment_streaming_na0197c.rs`
- `qsl/qsl-client/qsc/LOCAL_TWO_CLIENT_RUNBOOK.md`
- `qsl/qsl-client/qsc/REMOTE_TWO_CLIENT_AWS_RUNBOOK.md`
- `qsl/qsl-client/qsc/src/cmd/mod.rs`
- `DECISIONS.md`
- `TRACEABILITY.md`
