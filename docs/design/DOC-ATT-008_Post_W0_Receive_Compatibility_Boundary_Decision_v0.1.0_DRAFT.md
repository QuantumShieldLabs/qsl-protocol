Status: Supporting
Owner: Protocol Team
Last-Updated: 2026-03-27

# DOC-ATT-008 — Post-W0 Receive Compatibility Boundary Decision v0.1.0 DRAFT

Purpose:
- freeze the approved post-`w0` receiver-side retirement boundary clearly enough for runtime retirement implementation to become the next truthful lane;
- map the approved post-`w0` policy onto the existing `W0` / `W2` coexistence evidence without changing current live behavior; and
- keep qsc, qsl-attachments, qsl-server, and canonical runtime behavior unchanged in this item.

Non-goals:
- no qsc runtime/client changes;
- no qsl-attachments runtime or deployment changes;
- no qsl-server changes;
- no website, `.github`, or workflow changes; and
- no governance closeout or successor promotion in this artifact.

## 1. Authoritative current posture

Current repo state relevant to this final gate is:
- qsl-protocol `main` still keeps explicit `w0` as the rollback/coexistence control for new legacy-sized sends and `w2` as the validated-deployment default when attachment configuration is present.
- qsl-protocol tests and runbooks still treat mixed legacy receive plus attachment-descriptor receive as a supported invariant on current `main`.
- qsl-attachments remains unchanged in this item: repo-local `READY=0`, single-node local-disk, opaque ciphertext-only runtime, no plaintext service surfaces, no capability-like secrets in canonical URLs, and only the `rust` check required on `main`.
- qsl-server remains unchanged in this item: repo-local `READY=0`, transport-only relay posture, canonical header-based API guard present, and fail-fast detection for legacy-only deployed relays.

Those facts rule out "service immaturity" or "relay drift" as the remaining blocker. The only remaining blocker is the receiver-side policy boundary after `w0` is no longer live.

## 2. Approved post-`w0` receiver-side boundary

| Approved policy element | Frozen requirement | Current-source mapping |
| --- | --- | --- |
| While `w0` remains live | Mixed legacy receive compatibility remains exactly as already frozen and implemented on current `main` | `DOC-ATT-007`, qsc help, the local runbook, and `attachment_streaming_na0197c.rs` already keep `w0` explicit and operator-visible |
| After `w0` is no longer live | Legacy receive-side compatibility for `file_chunk` / `file_manifest` is retired on validated deployments | This item supplies the previously missing post-`w0` policy and narrows future runtime work to implementation only |
| Drain / continued-support / fallback posture | No implicit drain window, no implicit continued-support posture, no implicit fallback to legacy receive support, and no implicit rollback once `w0` itself is gone | This removes the unresolved policy gap left open by `DOC-ATT-007` without changing live `w0` behavior |
| Required post-`w0` runtime behavior | Receipt of legacy `file_chunk` / `file_manifest` payloads must fail closed, the failure must be explicit and operator-visible, and it must not reconstruct a legacy file, persist or promote durable completion state, advance `peer_confirmed`, or create a dishonest delivery outcome | Runtime marker/reject-code details stay deferred to `NA-0207`; the policy boundary itself is now frozen here |
| Unchanged cross-repo invariants | Route-token/header-carriage behavior remains unchanged, qsl-server remains transport-only, and qsl-attachments remains opaque ciphertext-only | Existing canonical/design docs plus sibling repo posture remain consistent with this boundary |

## 3. Consistency with current evidence

The approved policy is consistent with the already-merged evidence set:
- `DOC-ATT-004` through `DOC-ATT-007` already freeze that current `main` keeps explicit `w0` rollback/coexistence live and that receive compatibility remains required while `w0` exists.
- `DOC-CAN-005` still recognizes the legacy payload family, but that canonical payload definition does not require indefinite support after the explicit `w0` coexistence mechanism is removed.
- qsl-attachments and qsl-server remain strong enough that no service-side immaturity blocks this policy. The remaining work is local qsc retirement implementation once the approved boundary is recorded.
- Because this item changes only docs/decision/governance artifacts, current validated deployments remain unchanged until `NA-0207` implements the already-frozen post-`w0` fail-closed behavior.

## 4. Accumulated evidence review

| Evidence source | What it proves | What it does not prove |
| --- | --- | --- |
| `NA-0199` | Live coexistence rule is truthful: `<= 4 MiB` legacy path, exact `4 MiB` legacy path, above-threshold fail-closed without service, no false `peer_confirmed` on legacy reject | Any post-`w0` receiver contract |
| `NA-0200A` | Attachment service is operationally strong enough on the constrained-host lane for validated deployments | Any receiver-side retirement semantics |
| `NA-0201` | Stronger reference deployment keeps mixed message + attachment traffic bounded | Any post-`w0` legacy receive cutoff or drain rule |
| `NA-0201A` | Stress/soak/chaos evidence leaves only weak-host / weak-relay threshold saturation as bounded degradation | Any replacement contract for already-supported legacy payloads |
| `NA-0202B` / `DOC-ATT-004` | Freezes `W0` / `W1` / `W2`, configuration-only rollback to `W0`, explicit fallback, and explicit retention of receive compatibility until a later item authorizes removal | The later-item removal semantics themselves |
| `NA-0203` | Implements explicit `W0` / `W1` migration controls and mixed receive compatibility during migration | Any rule for what happens after rollback/coexistence itself disappears |
| `NA-0203A` | Keeps help/runbook surfaces truthful about rollback/fallback and removes stale wording | Any post-`w0` receiver contract |
| `NA-0204` / `DOC-ATT-005` | Freezes send-side final-removal decision only and explicitly keeps receive compatibility out of scope | Receiver-side retirement behavior |
| `NA-0205` | Implements `W2` send behavior, keeps `w0` live, keeps receive compatibility intact for both supported path families | Any post-`w0` receive boundary |
| `NA-0205A` | Revalidates `W2`, deprecated `w1` alias to `w2`, mixed receive compatibility, honest delivery, and no-silent-fallback | Any rule for retiring already-supported legacy receive payloads |
| `NA-0206` / `DOC-ATT-006` | Direct retirement was not yet truthful; remaining blockers are live `w0` plus missing later receiver contract | The missing contract itself |
| `NA-0206A` / `DOC-ATT-007` | Freezes the live boundary truthfully and narrows the blocker to one final post-`w0` gate | The previously missing post-`w0` answer before this approved policy was supplied |

The accumulated evidence plus the now-approved post-`w0` policy therefore answer the required questions as follows:
1. `w0` still makes receive compatibility load-bearing today because current `main` still exposes `w0` as an explicit rollback/coexistence mode and current tests/runbooks still rely on mixed legacy receive while that mode is live.
2. The project can now freeze what happens after `w0` no longer exists because the missing receiver-side contract has been approved explicitly in this item.
3. Retirement implementation is now truthfully next, because the remaining work is implementation detail rather than policy selection.
4. Continued support is not the truthful next posture, because the approved policy explicitly rejects an implicit continued-support lane, drain window, or rollback once `w0` is gone.

## 5. Decision

Chosen result:
- `PR0` / `PB0` — the approved post-`w0` receiver-side boundary is now frozen clearly enough that runtime retirement implementation is the next truthful lane.

Exact reason:
- Current merged evidence was already strong enough to freeze the live boundary only:
  - `w0` remains the explicit rollback/coexistence control on current `main`;
  - already-supported `file_chunk` / `file_manifest` payloads remain supported while `w0` stays live; and
  - qsl-attachments and qsl-server add no new service-side blocker.
- This item now supplies the previously missing post-`w0` receiver-side policy directly:
  - once `w0` is no longer live, legacy `file_chunk` / `file_manifest` receive compatibility is retired;
  - there is no implicit drain window, continued-support posture, fallback, or rollback once `w0` is gone; and
  - remaining legacy receive attempts must fail closed explicitly and operator-visibly without reconstructing legacy files, persisting durable completion state, advancing `peer_confirmed`, or creating dishonest delivery.
- With that boundary frozen, `NA-0207` is now an implementation lane rather than another decision lane.

Exact remaining blocker:
- The repo no longer has a policy blocker for the post-`w0` receive boundary. The remaining blocker is implementation of the already-frozen fail-closed runtime behavior in `NA-0207`.

Why this is the smallest truthful decision:
- it preserves the already-frozen live boundary exactly as documented on current `main`;
- it freezes the missing post-`w0` semantics without changing runtime behavior in this item;
- it does not relabel retirement implementation as another decision lane; and
- it leaves exact runtime marker/reject-code details to `NA-0207`, which is the smallest remaining implementation step.

Normative freeze from this item:
1. While `w0` remains live on current `main`, already-supported `file_chunk` / `file_manifest` payloads remain within the supported receiver contract.
2. Current rollback/fallback expectations remain unchanged while `w0` remains live.
3. Once `w0` is no longer live, legacy receive-side compatibility for `file_chunk` / `file_manifest` payloads is retired on validated deployments.
4. Once `w0` is no longer live, receipt of those legacy payloads must fail closed explicitly and operator-visibly; that failure must not reconstruct a legacy file, persist or promote durable completion state, advance `peer_confirmed`, or create a dishonest delivery outcome.
5. There is no implicit drain window, continued-support posture, fallback to continued legacy receive support, or rollback once `w0` itself is gone.
6. Route-token/header-carriage behavior remains unchanged, qsl-server remains transport-only, and qsl-attachments remains opaque ciphertext-only.
7. `NA-0207` is now the implementation lane for this frozen policy; exact reject markers/codes and operator surfaces are implementation details for that item, not open policy questions.

## References

- `docs/design/DOC-ATT-004_Legacy_In_Message_Deprecation_Readiness_v0.1.0_DRAFT.md`
- `docs/design/DOC-ATT-005_Legacy_In_Message_Final_Removal_Decision_v0.1.0_DRAFT.md`
- `docs/design/DOC-ATT-006_Legacy_Receive_Compatibility_Retirement_Decision_v0.1.0_DRAFT.md`
- `docs/design/DOC-ATT-007_Legacy_Receive_Compatibility_Retirement_Gate_Finalization_v0.1.0_DRAFT.md`
- `docs/canonical/DOC-CAN-005_QSP_Attachment_Descriptor_and_Control_Plane_v0.1.0_DRAFT.md`
- `tests/NA-0199_legacy_transition_validation.md`
- `qsl/qsl-client/qsc/tests/attachment_streaming_na0197c.rs`
- `qsl/qsl-client/qsc/tests/cli.rs`
- `qsl/qsl-client/qsc/LOCAL_TWO_CLIENT_RUNBOOK.md`
- `qsl-attachments/README.md`
- `qsl-attachments/TRACEABILITY.md`
- `qsl-server/README.md`
- `qsl-server/scripts/check_relay_compatibility.sh`
- `qsl-server/TRACEABILITY.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
