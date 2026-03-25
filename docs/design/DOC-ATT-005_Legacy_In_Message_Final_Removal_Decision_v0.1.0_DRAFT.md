Status: Supporting
Owner: Protocol Team
Last-Updated: 2026-03-25

# DOC-ATT-005 — Legacy In-Message Final Removal Decision v0.1.0 DRAFT

Purpose:
- freeze the final evidence-backed decision on whether the project may advance from the validated `W0` / `W1` migration window to a true final-removal implementation lane for legacy in-message send-path carriage;
- distinguish implementation readiness from immediate cutover; and
- define the exact boundaries that must continue to govern the next lane.

Non-goals:
- no qsc runtime/client changes;
- no qsl-attachments runtime or deployment changes;
- no qsl-server changes;
- no website, `.github`, or workflow changes; and
- no immediate removal of legacy receive compatibility.

## 1. Evidence baseline

The decision in this document is grounded by the following already-merged evidence:

- `NA-0199`
  - proved the live coexistence rule directly from qsc behavior;
  - proved exact `4 MiB` stays legacy, `> 4 MiB` rejects without validated attachment configuration, and service-backed sends stay truthful when configured; and
  - proved neither path can produce false `peer_confirmed`.
- `NA-0200A`
  - proved constrained-host service-backed behavior against the restored canonical relay;
  - proved service-backed `> 4 MiB`, `16 MiB`, `64 MiB`, and `100 MiB` runs remained contract-faithful; and
  - showed the degraded exact-threshold legacy case was weak-host / weak-relay saturation, not qsl-attachments correctness failure.
- `NA-0201`
  - proved the stronger `qatt` reference deployment over the real relay;
  - proved message-only, attachment-only, and mixed traffic stayed bounded; and
  - separated weak-relay legacy saturation from reference-host attachment correctness.
- `NA-0201A`
  - proved bounded kitchen-sink mixed traffic on `qatt`;
  - proved large files through `100 MiB`, restart/recovery, concurrency up to `8`, and a `30` minute soak; and
  - left only the weak-host / weak-relay legacy threshold stages as bounded fail-closed saturation.
- `NA-0202B`
  - froze the staged deprecation-readiness boundary in `DOC-ATT-004`;
  - defined `W0`, `W1`, and `W2`;
  - froze configuration-only rollback to `W0`; and
  - made the no-silent-break and operator-visible fallback proof model explicit.
- `NA-0203`
  - implemented the `W1` migration stage in qsc with explicit operator-controlled `w0` / `w1` stage selection;
  - kept `> 4 MiB` validated-deployment behavior unchanged from `NA-0202A`; and
  - preserved no-silent-fallback, receive compatibility, honest delivery, and route-token invariants.
- `NA-0203A`
  - validated and cleaned up the merged `W0` / `W1` lane without changing canonical semantics;
  - refreshed operator-facing docs/help so rollback and fallback remain explicit and truthful; and
  - removed stale legacy-only wording that could otherwise undermine the final-removal decision.

## 2. Option set

| Option | Summary | Result | Why |
| --- | --- | --- | --- |
| `F0` | Do not advance to final removal yet; create a smaller gate-finalization item | Rejected | `DOC-ATT-004` already identified the required gates, and the merged `NA-0203` / `NA-0203A` proof now satisfies those `W1` obligations. No separate load-bearing governance/evidence gap remains. |
| `F1` | Final-removal implementation is now justified, within the existing migration/rollback/fallback boundary | Chosen | This is the smallest truthful next step after the validated `W0` / `W1` lane. The remaining work is implementation plus regression proof, not another evidence-only gate. |
| `F2` | Keep legacy longer and do not advance to a removal lane yet | Rejected | The evidence still does not justify indefinite permanence of the legacy send path. It justifies moving to the next staged implementation lane. |
| `F3` | Treat final removal as immediate once implementation starts | Rejected | Current evidence justifies a staged `W2` implementation lane, not an instantaneous cutover or receive-compatibility removal. |

## 3. Evaluation against the required gates

### 3.1 Consistency with `DOC-ATT-003` and `DOC-ATT-004`

- `DOC-ATT-003` kept legacy `<= 4 MiB` unchanged until a later explicit item authorized change.
- `DOC-ATT-004` made `W2` contingent on satisfying the `W1` proof obligations and operator-surface updates.
- The merged `NA-0203` / `NA-0203A` work satisfies that structure rather than bypassing it, so advancing to a final-removal implementation lane is consistent with both documents.

### 3.2 Constrained-host, reference-host, and stress/soak evidence

- The constrained-host, reference-host, and bounded kitchen-sink evidence already proved the attachment path strongly enough for validated deployments.
- The only degraded stages remained on the weak-host / weak-relay legacy threshold path and were classified truthfully as bounded saturation, not attachment correctness failure.
- That evidence supports retiring the legacy send path for validated deployments; it does not support immediate receive-path removal or any qsl-attachments/qsl-server redesign.

### 3.3 Migration-window completeness

The `DOC-ATT-004` `W1` proof model is now satisfied by merged tests and operator surfaces:

- explicit path-selection coverage for small and exact-threshold files:
  - `w1_legacy_sized_selection_is_explicit_for_small_and_threshold_files`
- explicit fail-closed missing-service behavior:
  - `w1_missing_service_fails_closed_without_legacy_fallback`
- explicit no-silent-fallback behavior after attachment failure:
  - `w1_attachment_rejects_do_not_fallback_to_legacy`
- explicit rollback to `W0`:
  - `config_rollback_to_w0_restores_legacy_selection`
- preserved mixed receive compatibility during `W1`:
  - `mixed_receive_compatibility_is_preserved_during_w1`
- truthful delivery-state behavior for legacy-sized attachment-first sends:
  - `legacy_sized_w1_roundtrip_confirms_without_false_peer_confirmed`
- operator discoverability and truthful runbook/help coverage:
  - `file_send_help_documents_legacy_migration_controls`
  - `supported_docs_and_scripts_do_not_embed_route_tokens_in_urls`
  - `canonical_operator_examples_use_route_token_header_and_not_authorization_overload`

### 3.4 Rollback clarity

- For the currently merged `W0` / `W1` lane, rollback remains explicit and configuration-only by returning to `w0`.
- That rollback model is now fully documented and regression-covered.
- No extra gate remains before entering the final-removal implementation lane; the next lane may retire the `w0` / `w1` send-stage control only if it preserves the invariants listed in Section 5.

### 3.5 Explicit fallback clarity

- Missing validated attachment configuration still fails closed with `attachment_service_required`.
- Attachment-path failure still rejects explicitly and does not retry the legacy path silently.
- Operator docs now state those boundaries truthfully for both local and remote runbooks.

### 3.6 No-silent-break confidence

- The current merged system already proves explicit path selection, explicit rollback, explicit failure on missing service, no silent fallback, honest delivery milestones, and receive compatibility during the migration window.
- That is sufficient evidence to begin the final-removal implementation lane.
- It is not evidence for immediate receive-path removal, hidden fallback, or any widened scope into qsl-attachments or qsl-server.

## 4. Decision

Chosen result:
- `R1` — final-removal implementation is ready enough to become the next implementation lane.

Exact reason:
- The explicit gates frozen in `DOC-ATT-004` are now met by already-merged `NA-0203` and `NA-0203A` implementation plus validation evidence.
- No additional qsl-attachments-only, qsl-server-only, or governance-only proof item now outranks the final-removal implementation work.
- The remaining work is implementation and regression proof inside qsc, not another missing decision artifact.

Why this is the smallest truthful decision:
- it does not claim immediate cutover;
- it does not claim receive-compatibility removal;
- it does not invent a new gate that the existing readiness contract never required; and
- it does not hold the legacy send path indefinitely without evidence.

## 5. Boundaries for the implementation lane

`NA-0205` is authorized only within the following boundaries:

- validated deployments only; do not infer arbitrary attachment-service endpoints;
- qsl-server remains transport-only and unchanged;
- qsl-attachments remains unchanged unless a new concrete correctness defect is proven;
- no silent fallback from attachment selection to the legacy send path;
- fail-closed missing-service behavior remains explicit;
- honest delivery milestones and route-token/header invariants remain unchanged;
- receive compatibility for already-supported legacy payloads remains in scope and must not be removed in the same lane; and
- no wire/protocol/auth/route-token semantic change is authorized outside the qsc send-path selection needed to complete `W2`.

Rollback/fallback expectations for the current merged release remain unchanged until `NA-0205` lands:
- `w0` remains the explicit rollback/coexistence target on current `main`;
- `w1` remains the explicit canary mode on current `main`; and
- `NA-0205` must not weaken those explicit guarantees while completing `W2`.

Further evidence required before implementation:
- none beyond the implementation-time regression proof that belongs inside `NA-0205` itself.

## 6. Queue implication

The next truthful queue item after this decision is:
- `NA-0205 — Legacy In-Message Final Removal Implementation`

That lane should implement `W2` completion for validated deployments only. It must not collapse into:
- qsl-attachments architecture redesign,
- qsl-server compatibility redesign, or
- legacy receive-path removal.

## References

- `docs/design/DOC-ATT-002_qsl-attachments_Deployment_and_Operational_Hardening_Contract_v0.1.0_DRAFT.md`
- `docs/design/DOC-ATT-003_Default_Attachment_Path_Promotion_and_Legacy_In_Message_Policy_v0.1.0_DRAFT.md`
- `docs/design/DOC-ATT-004_Legacy_In_Message_Deprecation_Readiness_v0.1.0_DRAFT.md`
- `tests/NA-0199_legacy_transition_validation.md`
- `qsl-attachments/tests/NA-0003_constrained_host_validation_evidence.md`
- `qsl-attachments/tests/NA-0004_reference_deployment_validation_evidence.md`
- `qsl-attachments/tests/NA-0005_stress_soak_chaos_evidence.md`
- `qsl/qsl-client/qsc/tests/attachment_streaming_na0197c.rs`
- `qsl/qsl-client/qsc/tests/cli.rs`
- `qsl/qsl-client/qsc/tests/route_header_migration_docs_na0195a.rs`
- `DECISIONS.md`
- `TRACEABILITY.md`
