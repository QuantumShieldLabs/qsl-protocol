Status: Supporting
Owner: Protocol Team
Last-Updated: 2026-03-22

# DOC-ATT-004 — Legacy In-Message Deprecation Readiness v0.1.0 DRAFT

Purpose:
- Freeze the smallest truthful migration, rollback, no-silent-break, and fallback policy needed before any legacy `<= 4 MiB` deprecation implementation begins.
- Distinguish what current evidence already proves from what the future implementation lane must still add.
- Keep qsc, qsl-attachments, and qsl-server runtime behavior unchanged in this item.

Non-goals:
- No qsc runtime/client changes.
- No qsl-attachments runtime or deployment changes.
- No qsl-server changes.
- No website, `.github`, or workflow changes.
- No immediate legacy deprecation implementation.

## 1. Evidence baseline

The readiness policy in this document is grounded by the following completed evidence:
- `NA-0199` proved the current coexistence rule directly from live qsc behavior:
  - `<= 4 MiB` sends stay on the legacy in-message path.
  - exact-threshold `4 MiB` sends also stay on the legacy path.
  - above-threshold sends reject cleanly without attachment-service configuration.
  - above-threshold sends use the attachment path when explicitly configured.
  - neither path produces a false `peer_confirmed`.
- `DOC-ATT-002` froze the deployment/readiness contract and deprecation gates:
  - default-path promotion must be justified first.
  - legacy deprecation additionally requires an explicit migration plan, an explicit rollback plan, proof of no silent break during the migration window, and explicit operator-visible fallback behavior.
- `NA-0200A` proved constrained-host service-backed behavior over the real relay:
  - `> 4 MiB`, `16 MiB`, `64 MiB`, and `100 MiB` service-backed runs completed truthfully.
  - restart, resume, quota/reject paths, and limited concurrency remained contract-faithful.
  - degraded exact-threshold legacy cases were bounded weak-relay saturation, not qsl-attachments correctness failure.
- `NA-0201` proved the stronger `qatt` reference deployment:
  - message-only, attachment-only, and mixed traffic succeeded over the real relay.
  - restart/recovery, bounded concurrency, and short mixed soak stayed bounded.
  - the remaining degraded threshold stages stayed on the weak relay / legacy path rather than on the stronger attachment-service host.
- `NA-0201A` proved the bounded kitchen-sink lane:
  - mixed traffic, large files through `100 MiB`, restart/recovery, concurrency up to `8`, and a `30` minute soak stayed truthful on `qatt`.
  - the only degraded required stages remained the weak-host / weak-relay legacy threshold path at `< 4 MiB` and exact `4 MiB`, both as bounded fail-closed saturation with honest delivery state.
- `NA-0202A` proved the current operator-controlled validated-deployment behavior:
  - validated deployments now default to the attachment path for sends strictly above `4 MiB`.
  - missing validated attachment configuration fails closed explicitly with `attachment_service_required`.
  - exact `4 MiB` and smaller files remain on the legacy path unchanged.
  - attachment-path failures do not silently retry the legacy path.

## 2. Gate inventory

| Gate | Pre-NA-0202B status | Existing proof | NA-0202B output | Follow-on implementation requirement |
| --- | --- | --- | --- | --- |
| Migration plan | Partial | `NA-0199`, `DOC-ATT-002`, `DOC-ATT-003`, `NA-0202A` define the current coexistence and validated-deployment boundary | Section 4 freezes the staged migration window | Implement the frozen migration window without changing unrelated semantics |
| Rollback plan | Partial | Existing coexistence rule and current receive support for both path families make configuration rollback feasible | Section 5 freezes rollback triggers and config-only rollback expectations | Implement explicit rollback control and test it |
| No-silent-break proof model | Partial | `NA-0199`, `NA-0200A`, `NA-0201`, `NA-0201A`, and `NA-0202A` already prove truthful path selection and fail-closed behavior on the current boundary | Section 7 freezes the proof model for the implementation lane | Add the missing implementation-time tests for legacy-sized attachment-first sends and rollback |
| Operator-visible fallback behavior | Partial | `NA-0202A` already proves fail-closed above-threshold behavior and forbids silent fallback | Section 6 freezes the only permissible fallback behavior for the migration window | Surface the explicit control/error behavior in qsc and operator docs |
| Operator communication / discoverability | Partial | Governance/design surfaces exist today | Section 8 freezes the operator-facing surfaces that the implementation lane must update | Update the named operator surfaces in the implementation PR |
| Validated-deployment scope boundary | Satisfied | `DOC-ATT-002`, `DOC-ATT-003`, `NA-0202A`, and qsl-server `NA-0011` already freeze this boundary | Reused unchanged in Sections 3 and 4 | Preserve the same boundary; do not infer arbitrary service endpoints |
| Deprecation preconditions vs implementation preconditions | Unsatisfied | The prior docs state the missing gates but do not split them cleanly | Section 8 freezes the exact split | Use that split as the acceptance boundary for the implementation lane |

## 3. Validated-deployment scope boundary

The readiness defined here is limited to validated deployments only. For this document, validated deployment means all of the following remain true:
- the `DOC-ATT-002` default-promotion gate is already satisfied;
- the relay passes qsl-server's canonical compatibility guard;
- attachment-service configuration comes from an operator-controlled validated surface rather than a network guess;
- qsl-attachments remains the current opaque ciphertext, single-node local-disk runtime; and
- qsl-server remains transport-only.

This readiness item does not authorize:
- removal of qsl-server compatibility behavior;
- qsl-attachments architecture redesign;
- removal of qsc receive support for already-supported legacy payloads; or
- any wire/protocol change beyond future qsc send-path selection inside validated deployments.

## 4. Frozen migration window

The migration window for legacy `<= 4 MiB` deprecation is frozen as three stages:

### W0 — Current coexistence baseline

This is the live post-`NA-0202A` state:
- sends strictly above `4 MiB` use the attachment path by default in validated deployments;
- exact `4 MiB` and smaller sends remain on the legacy in-message path;
- missing validated attachment configuration fails closed explicitly;
- no silent fallback from an attachment attempt to the legacy path is allowed.

### W1 — Attachment-first canary for legacy-sized sends

This is the first implementation stage authorized by this readiness item:
- W1 may be enabled only through an explicit operator-controlled migration mode in validated deployments.
- Under W1, new sends in the legacy-sized class may be routed to the attachment path instead of the legacy in-message path.
- Receive compatibility for both path families must remain enabled throughout W1:
  - legacy manifests/chunks already in circulation must still be receivable;
  - attachment descriptors already in circulation must still be receivable.
- If the explicit migration mode is absent, qsc must remain in W0.

W1 is the only truthful first implementation stage because it keeps rollback cheap and operator-visible while adding no hidden network inference.

### W2 — Legacy send-path deprecation completion

This stage may be declared only after the W1 proof obligations in Section 7 are satisfied and the operator-facing updates in Section 8 land.

W2 authorizes:
- attachment-first send selection for the legacy-sized class in validated deployments without requiring a special canary toggle; and
- retention of receive compatibility for already-supported legacy payloads unless a later explicit item authorizes removing it.

W2 does not authorize:
- silent fallback to the legacy send path;
- silent fallback from attachment failure to legacy send behavior; or
- removal of legacy receive support in the same lane.

## 5. Frozen rollback plan

The rollback plan for the future implementation lane is:

Rollback triggers:
- any false `peer_confirmed`, dishonest delivery milestone, or no-mutation violation;
- any silent path flip or any requirement for implicit legacy retry to make the send succeed;
- any secret-bearing URL, capability leak, plaintext-on-service-surface leak, or other secret-hygiene regression;
- any load-bearing correctness regression on legacy-sized attachment-first sends that is not already classifiable as bounded saturation; or
- inability to keep the deployment in an explicit, operator-classifiable state.

Rollback mechanism expectations:
- rollback must be configuration-only and must restore W0 for new legacy-sized sends;
- rollback must not require wire/data migration, qsl-server rollback, or qsl-attachments rollback;
- rollback must preserve receive compatibility for both path families; and
- rollback may keep the already-shipped `> 4 MiB` default-attachment behavior from `NA-0202A` intact unless the triggering defect proves that broader rollback is required.

The smallest truthful rollback is therefore "restore the current coexistence rule for new legacy-sized sends while leaving receive compatibility and the validated above-threshold policy intact."

## 6. Frozen operator-visible fallback behavior

During any W1 or W2 migration window, the only permissible fallback behavior is explicit and operator-selected.

Forbidden behavior:
- automatic fallback from an attachment attempt to the legacy send path;
- hidden retry on the legacy path after any attachment start, upload, commit, or receive failure;
- silent reclassification of a legacy-sized send back onto the legacy path when migration mode was intended.

Required explicit behavior:
- if validated attachment configuration is missing, qsc must fail closed explicitly before send start;
- if the attachment path rejects or fails after selection, qsc must surface an attachment-path failure explicitly and must not retry silently on the legacy path;
- any temporary use of the legacy send path during the migration window must require an explicit operator-chosen coexistence or rollback control, not an implicit fallback; and
- operator-visible output must make it clear whether the send ran in W0, W1, or W2 semantics.

## 7. Frozen no-silent-break proof model

The implementation lane must prove no silent break by satisfying all of the following proof obligations:

| Proof obligation | Current evidence already available | Implementation proof still required |
| --- | --- | --- |
| Path-selection boundary is explicit and deterministic | `NA-0199` proves current `<= 4 MiB` / `= 4 MiB` legacy behavior and above-threshold service selection; `NA-0202A` proves validated-deployment default selection above threshold and fail-closed missing-service behavior | Add deterministic tests for attachment-first legacy-sized sends, exact `4 MiB`, smaller-than-threshold files, and restored W0 selection after rollback |
| Delivery milestones remain truthful | `NA-0199`, `NA-0200A`, `NA-0201`, and `NA-0201A` prove no false `peer_confirmed` and honest state progression on the current path families | Add equivalent truthfulness tests for legacy-sized attachment-first sends and post-rollback legacy sends |
| No silent fallback occurs | `DOC-ATT-003` and `NA-0202A` already forbid and test silent fallback for above-threshold sends | Add explicit tests that legacy-sized attachment-first sends fail closed rather than silently retrying legacy |
| Receive compatibility remains intact during migration | Current qsc evidence already covers both path families independently | Add mixed migration tests proving that legacy receive support remains intact while new legacy-sized sends can use the attachment path |
| Rollback is deterministic and operator-visible | The current coexistence rule already provides the intended post-rollback state | Add tests proving that the explicit rollback control restores W0 for new sends without mutating pending receive compatibility |

This proof model is evidence-backed because the current system already has:
- a validated legacy path for `<= 4 MiB`;
- a validated attachment path for service-backed sends;
- current receive support for the attachment path; and
- a fail-closed, operator-controlled validated-deployment selection surface.

## 8. Preconditions and implementation boundary

Preconditions satisfied now for starting the implementation lane:
- default attachment-path promotion above threshold is already justified and implemented;
- the migration window shape is now explicit;
- the rollback triggers and rollback mechanism expectations are now explicit;
- the no-silent-break proof model is now explicit; and
- the fallback behavior is now explicit and operator-visible by policy.

Implementation preconditions for `NA-0203`:
- add the explicit operator-controlled migration/rollback surface required by Sections 4 through 6;
- add deterministic tests covering legacy-sized attachment-first sends, exact-threshold behavior, fail-closed missing-service behavior, no silent fallback, receive compatibility, and rollback restoration;
- update operator-facing discoverability surfaces for the migration mode and rollback mode; and
- keep qsl-server untouched and transport-only, and keep qsl-attachments unchanged unless a concrete new correctness defect is proven.

Required operator-facing discoverability surfaces in the implementation lane:
- qsc user/help or equivalent operator-facing command documentation for the migration mode;
- qsc operator runbook or repo-local operator documentation for validated-deployment configuration and rollback;
- `NEXT_ACTIONS.md`, `TRACEABILITY.md`, and `DECISIONS.md`; and
- a public docs index or README link only if the migration mode becomes a discoverability problem for non-governance readers.

## 9. Readiness decision

Chosen readiness result:
- `R1` — readiness is complete enough to start a staged deprecation implementation lane.

Why `R1` is truthful:
- the remaining blocker is no longer missing governance or evidence structure;
- the remaining work is runtime implementation plus the deterministic proof obligations frozen above;
- no additional qsl-attachments repo-local evidence item outranks that implementation work; and
- immediate deprecation remains unjustified, so the next lane must be staged rather than immediate.

Rejected readiness results:
- `R0` rejected because this document closes the governance/evidence gap that previously blocked implementation.
- `R2` rejected because no evidence supports immediate cutover without a staged migration window and rollback surface.
- `R3` rejected because current evidence does not justify keeping the legacy send path indefinitely as the permanent product posture.

## References

- `docs/design/DOC-ATT-002_qsl-attachments_Deployment_and_Operational_Hardening_Contract_v0.1.0_DRAFT.md`
- `docs/design/DOC-ATT-003_Default_Attachment_Path_Promotion_and_Legacy_In_Message_Policy_v0.1.0_DRAFT.md`
- `tests/NA-0199_legacy_transition_validation.md`
- `qsl-attachments/tests/NA-0003_constrained_host_validation_evidence.md`
- `qsl-attachments/tests/NA-0004_reference_deployment_validation_evidence.md`
- `qsl-attachments/tests/NA-0005_stress_soak_chaos_evidence.md`
- `qsl/qsl-client/qsc/tests/attachment_streaming_na0197c.rs`
- `qsl-server/scripts/check_relay_compatibility.sh`
- `DECISIONS.md`
- `TRACEABILITY.md`
