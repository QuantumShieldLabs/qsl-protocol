Status: Supporting
Owner: Protocol Team
Last-Updated: 2026-03-28

# DOC-ATT-009 — Post-W0 Activation and Legacy Mode Retirement Decision v0.1.0 DRAFT

Purpose:
- decide whether validated deployments now have enough merged evidence to stop operating with live `w0` coexistence and adopt the post-`w0` retired posture by default;
- freeze when that cutover happens, what operator-visible controls remain, narrow, or disappear, and whether implementation or more gate work comes next; and
- keep qsc, qsl-attachments, qsl-server, and canonical runtime behavior unchanged in this item.

Non-goals:
- no qsc runtime/client changes;
- no qsl-attachments runtime or deployment changes;
- no qsl-server changes;
- no website, `.github`, or workflow changes; and
- no governance closeout or queue promotion in this artifact.

## 1. Authoritative current posture

Current repo state relevant to this activation decision is:
- qsc `main` already defaults validated deployments to `w2` send behavior for new legacy-sized sends when validated attachment configuration is present, while still exposing explicit `w0` rollback/coexistence for those sends through `QSC_LEGACY_IN_MESSAGE_STAGE` / `--legacy-in-message-stage`;
- qsc `main` already implements post-`w0` receive retirement as an explicit operator-selected mode: `qsc receive` defaults `--legacy-receive-mode` to `coexistence` while `w0` remains live, and validated post-`w0` lanes can already switch explicitly to `retired`;
- qsl-attachments remains unchanged in this item: repo-local `READY=0`, single-node local-disk, opaque ciphertext-only runtime, no plaintext service surfaces, no capability-like secrets in canonical URLs, and the `rust` check as the current `main` protection baseline; and
- qsl-server remains unchanged in this item: repo-local `READY=0`, transport-only relay posture, canonical header-based route-token compatibility guard present, and fail-fast detection for legacy-only deployed relays.

Those facts mean current `main` still keeps live `w0` coexistence only because the operator-visible promise is still exposed. The remaining question is no longer service maturity, relay drift, send-path correctness, or receive-retirement semantics. It is whether that live coexistence promise should still survive as the truthful validated-deployment posture after the evidence already gathered.

## 2. Activation / legacy-mode option set

| Option | Summary | Consistency with `DOC-ATT-004` / `DOC-ATT-005` / `DOC-ATT-008` | Effect on validated deployment flows | Operator-visible controls after cutover | Rollback / fallback clarity | More evidence still required? | Result |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `PW0` | Keep live `w0` coexistence longer for now without promoting a new explicit blocker | Weak. It matches current `main`, but it no longer matches the merged evidence chain that already justified `W2`, froze the post-`w0` receive boundary, and validated retired-mode runtime behavior. | Preserves current flows, but only by keeping a coexistence promise that no longer has an evidence-backed future need. | Leaves `w0`, deprecated `w1`, and receive `coexistence` live. | Keeps current rollback unchanged, but only by refusing to act on already-cleared gates. | No new missing proof is identified. | Rejected |
| `PW1` | Post-`w0` activation / legacy-mode retirement implementation is now justified | Strong. `DOC-ATT-004` and `DOC-ATT-005` already cleared the send-side lane, and `DOC-ATT-008` plus `NA-0207` / `NA-0207A` already clear the post-`w0` receive-side lane. | Current `main` stays unchanged until the activation lane lands; the cutover then removes live coexistence without inventing new protocol or service semantics. | Live `w0` / `w1` / `coexistence` surfaces can be retired; retired-mode and fail-closed attachment-path surfaces remain. | Clear: configuration-only rollback to `w0` exists only while `w0` stays live and disappears at activation; fail-closed fallback remains explicit. | No. The remaining work is implementation detail and operator-surface cleanup. | Chosen |
| `PW2` | Activation is directionally justified, but one explicit gate still blocks implementation | Weak. `DOC-ATT-008` already closed the last post-`w0` receive-side policy gap, and `NA-0207A` already validated the merged retired-mode lane. | Would hold the queue on a non-existent gate. | Would preserve current coexistence controls without naming a truthful reason. | Would imply a missing blocker that the repo no longer shows. | No truthful new blocker remains. | Rejected |
| `PW3` | Continued live coexistence should become the explicit next posture rather than retirement | Inconsistent. `DOC-ATT-005` rejected indefinite send-side legacy posture, and `DOC-ATT-008` rejected drain/continued-support/rollback once `w0` is gone. | Would relabel a temporary coexistence rule as the near-term product posture without supporting evidence. | Would require keeping or re-justifying `w0` / `coexistence` as ongoing supported controls. | Would blur the already-frozen post-`w0` no-rollback boundary. | Would require new evidence that does not exist. | Rejected |

Leading candidate:
- `PW1` is the only option that matches the full evidence chain without inventing a new blocker or pretending that coexistence has become the intended steady-state posture.

## 3. Accumulated evidence review

| Evidence source | What it proves | What it does not prove | Decision impact |
| --- | --- | --- | --- |
| `NA-0199` | The live coexistence rule was truthful: `<= 4 MiB` and exact `4 MiB` stayed on the legacy path, above-threshold sends failed closed without service, configured attachment sends stayed truthful, and legacy-path rejects could not create false `peer_confirmed`. | Any deployment-hardening result or any post-`w0` retirement boundary. | Establishes the baseline that coexistence was temporary and evidence-driven, not permanent posture. |
| `NA-0200A` | The constrained-host service-backed lane over the restored real relay was strong enough for validated deployments, including restart/resume/quota/reject behavior. | Any send- or receive-side post-`w0` semantic decision. | Clears the weak-host operational gate that originally blocked promotion/deprecation work. |
| `NA-0201` | The stronger `qatt` reference deployment kept message-only, attachment-only, and mixed traffic bounded over the real relay. | Any claim about post-`w0` operator controls. | Separates weak-host saturation from attachment correctness and removes reference-host doubt. |
| `NA-0201A` | Stress/soak/chaos evidence left only weak-host / weak-relay threshold saturation as bounded degradation, not correctness failure. | Any post-`w0` cutover rule. | Removes the remaining service-readiness reason to keep coexistence live. |
| `NA-0202B` | `DOC-ATT-004` froze `W0` / `W1` / `W2`, configuration-only rollback to `W0`, explicit fallback, and no-silent-break proof obligations. | Whether `w0` must remain live forever. | Shows rollback/coexistence was intentionally temporary and must be retired explicitly rather than silently ignored. |
| `NA-0203` | qsc implemented explicit operator-controlled `w0` / `w1` migration controls plus configuration-only rollback and truthful file-send policy markers. | Whether those controls still need to survive after activation. | Proves the live coexistence promise is an operator-visible control that must now be retired deliberately if the project moves on. |
| `NA-0203A` | CLI/help/runbook surfaces were cleaned up so `w0` / `w1`, rollback, and fallback were truthful and explicit. | Any justification for keeping those surfaces after the remaining gates are closed. | Confirms the current control surface is already clear enough to retire without semantic guesswork. |
| `NA-0204` | `DOC-ATT-005` concluded that the explicit `DOC-ATT-004` gates were satisfied strongly enough to justify the final `W2` implementation lane. | Any receive-side post-`w0` rule. | Removes the last send-side policy reason to keep coexistence live as a future posture. |
| `NA-0205` | qsc implemented `W2`: validated deployments now default new legacy-sized sends to the attachment path, while `w0` remains only as explicit rollback/coexistence and `w1` is only a deprecated alias. | Whether `w0` should still remain exposed after the receive-side policy is closed. | Narrows the live coexistence question to the remaining operator-visible override itself. |
| `NA-0205A` | `W2` validation/cleanup stayed green, and deprecated `w1` is already just a compatibility alias to `w2`. | Any post-`w0` receive boundary. | Shows the canary-stage surface is already functionally obsolete. |
| `NA-0206` | Direct receive retirement was not yet truthful because `w0` was still live and the post-`w0` receiver contract was still missing. | The missing contract itself. | Historical blocker only. |
| `NA-0206A` | The live receive boundary was frozen truthfully: legacy receive stayed load-bearing only while `w0` remained live, and one final post-`w0` gate still remained. | The final post-`w0` answer. | Narrows the old blocker to a single post-`w0` policy question. |
| `NA-0206C` | `DOC-ATT-008` froze the post-`w0` receive boundary: once `w0` is no longer live, legacy receive compatibility is retired with no drain window, continued-support posture, fallback, or rollback. | The implementation details that make that cutover operator-visible. | Closes the last receive-side policy gate. |
| `NA-0207` | qsc already implements retired receive mode with explicit fail-closed reject markers and no dishonest delivery-state mutation. | Whether retired mode should become the default posture. | Proves the necessary runtime behavior already exists and does not require semantic invention. |
| `NA-0207A` | Post-merge validation/cleanup for the retired-mode lane is complete and green. | Any reason to keep retired mode as an opt-in only once `w0` is intentionally retired. | Removes the last local runtime/docs cleanup concern. |

Required questions answered from the accumulated evidence:

1. Is live `w0` coexistence still load-bearing for truthful validated deployment operation?
   - Only because current `main` still exposes it as an operator-visible promise. It is no longer load-bearing because of missing attachment-service, relay, send-path, or receive-retirement evidence.

2. If not, is activation to the post-`w0` retired posture now justified?
   - Yes. The remaining work is implementation of a cutover that current policy already defines.

3. If not yet, what exact gate still blocks it?
   - No explicit gate remains.

4. What operator-visible controls remain or disappear at that cutover?
   - Remain: validated attachment-service configuration, fail-closed `attachment_service_required` behavior when required configuration is absent, and explicit retired-mode reject markers for residual legacy receive attempts.
   - Narrow: any surviving legacy-mode surface may only describe or preserve the retired/default posture; it may not restore live coexistence.
   - Disappear: `w0` rollback/coexistence for new legacy-sized sends, the deprecated `w1` alias, and `coexistence` as a supported receive-mode choice on validated post-`w0` deployments.

## 4. Decision

Chosen result:
- `PA1` / `AO1` — post-`w0` activation implementation is now the next truthful lane.

Exact reason:
- the qsl-attachments operational and deployment evidence is already complete enough for validated deployments;
- qsl-server already provides the canonical header-based relay API plus fail-fast compatibility guard and remains transport-only;
- the send-side final-removal lane (`W2`) is already implemented and validated;
- the post-`w0` receive boundary is already frozen by `DOC-ATT-008` and already implemented/validated by `NA-0207` / `NA-0207A`; and
- the only remaining difference between current `main` and the intended post-`w0` posture is the still-exposed live coexistence promise itself.

Exact activation boundary:
1. Current `main` remains unchanged until the activation implementation lane lands.
2. Validated deployments leave live `w0` coexistence when the `NA-0209` implementation lane merges and its updated operator-facing cutover surfaces become authoritative on `main`.
3. At that cutover, validated deployments adopt the post-`w0` retired posture by default for both send and receive. There is no later implicit drain window, continued-support window, or delayed second cutover after that implementation lands.

Frozen operator-visible control surface at cutover:

Remain:
- validated attachment-service configuration remains operator-controlled through the existing validated deployment surfaces;
- explicit fail-closed attachment-path errors such as `attachment_service_required` remain truthful; and
- explicit retired-mode reject markers for residual legacy `file_chunk` / `file_manifest` receive attempts remain operator-visible.

Narrow:
- any surviving legacy-mode CLI/env/config surface may describe only the retired/default posture after activation;
- any surviving receive-mode surface may keep `retired` only or become a compatibility no-op at the retired default; and
- no surviving surface may restore `w0`, `w1`, or `coexistence` for validated post-`w0` deployments.

Disappear:
- explicit live-coexistence rollback/restore for new legacy-sized sends (`QSC_LEGACY_IN_MESSAGE_STAGE=w0`, `--legacy-in-message-stage w0`);
- the deprecated `w1` alias as an operator-visible supported stage; and
- `coexistence` as a supported receive-mode choice for validated post-`w0` deployments.

Rollback / fallback expectations after activation:
- the configuration-only rollback/coexistence promise to live `w0` ends at activation;
- no implicit or explicit fallback to the legacy send path or to continued legacy receive compatibility is permitted after activation; and
- failures remain fail-closed and operator-visible only.

Why this is the smallest truthful decision:
- it does not change current runtime behavior in this item;
- it does not invent new wire, relay, attachment-service, or cryptographic semantics;
- it freezes exactly the remaining operator-surface and cutover boundary needed for implementation; and
- it avoids inventing another gate-finalization or continued-support item when the repo no longer shows one.

## 5. Queue implication

The next truthful queue item after this decision is:
- `NA-0209 — Post-W0 Activation / Legacy Mode Retirement Implementation`

That lane should:
- implement the already-frozen activation/cutover behavior for validated deployments only;
- remove or narrow the live `w0` / `w1` / `coexistence` controls consistently with this decision;
- update operator/help/runbook surfaces truthfully; and
- preserve fail-closed send/receive behavior, honest delivery semantics, route-token/header invariants, qsl-server transport-only posture, and qsl-attachments opaque ciphertext-only posture.

It should not collapse into:
- new policy or gate-finalization work;
- qsl-attachments redesign;
- qsl-server redesign; or
- a continued-support lane for live `w0`.

## References

- `docs/design/DOC-ATT-002_qsl-attachments_Deployment_and_Operational_Hardening_Contract_v0.1.0_DRAFT.md`
- `docs/design/DOC-ATT-003_Default_Attachment_Path_Promotion_and_Legacy_In_Message_Policy_v0.1.0_DRAFT.md`
- `docs/design/DOC-ATT-004_Legacy_In_Message_Deprecation_Readiness_v0.1.0_DRAFT.md`
- `docs/design/DOC-ATT-005_Legacy_In_Message_Final_Removal_Decision_v0.1.0_DRAFT.md`
- `docs/design/DOC-ATT-006_Legacy_Receive_Compatibility_Retirement_Decision_v0.1.0_DRAFT.md`
- `docs/design/DOC-ATT-007_Legacy_Receive_Compatibility_Retirement_Gate_Finalization_v0.1.0_DRAFT.md`
- `docs/design/DOC-ATT-008_Post_W0_Receive_Compatibility_Boundary_Decision_v0.1.0_DRAFT.md`
- `docs/canonical/DOC-CAN-005_QSP_Attachment_Descriptor_and_Control_Plane_v0.1.0_DRAFT.md`
- `docs/canonical/DOC-CAN-006_QATT_Attachment_Service_Contract_v0.1.0_DRAFT.md`
- `docs/canonical/DOC-CAN-007_QATT_Attachment_Encryption_Context_and_Part_Cipher_v0.1.0_DRAFT.md`
- `tests/NA-0199_legacy_transition_validation.md`
- `qsl/qsl-client/qsc/src/cmd/mod.rs`
- `qsl/qsl-client/qsc/tests/attachment_streaming_na0197c.rs`
- `qsl/qsl-client/qsc/tests/cli.rs`
- `qsl/qsl-client/qsc/tests/route_header_migration_docs_na0195a.rs`
- `qsl/qsl-client/qsc/LOCAL_TWO_CLIENT_RUNBOOK.md`
- `qsl/qsl-client/qsc/REMOTE_TWO_CLIENT_AWS_RUNBOOK.md`
- `qsl-attachments/NEXT_ACTIONS.md`
- `qsl-attachments/TRACEABILITY.md`
- `qsl-server/NEXT_ACTIONS.md`
- `qsl-server/TRACEABILITY.md`
- `qsl-server/scripts/check_relay_compatibility.sh`
- `DECISIONS.md`
- `TRACEABILITY.md`
