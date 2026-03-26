Status: Supporting
Owner: Protocol Team
Last-Updated: 2026-03-26

# DOC-ATT-008 — Post-W0 Receive Compatibility Boundary Decision v0.1.0 DRAFT

Purpose:
- decide whether current merged evidence now freezes the post-`w0` receiver-side retirement boundary clearly enough for runtime retirement implementation to become the next truthful lane;
- distinguish "implementation next," "continued support," and "still undecidable from current evidence"; and
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

## 2. Post-`w0` boundary inventory

| Boundary element | Status | Current proof | Still missing |
| --- | --- | --- | --- |
| What current `main` still promises while `w0` remains live | Satisfied | `DOC-ATT-007`, qsc help, the local runbook, `attachment_streaming_na0197c.rs`, and current `W2` docs all keep `w0` explicit and operator-visible | Nothing additional is missing for the live boundary |
| What happens to already-supported `file_chunk` / `file_manifest` payloads once `w0` is no longer live | Unsatisfied | `DOC-CAN-005` still recognizes the legacy payload family, while `DOC-ATT-007` freezes only that those payloads remain supported while `w0` stays live | No merged source defines whether post-`w0` legacy payloads become immediate hard rejects, remain receivable for a bounded drain window, or remain supported until some other explicit condition |
| Whether operator-visible rollback/fallback expectations change after `w0` disappears | Unsatisfied | Current rollback/fallback expectations are explicit only while `w0` is still available | No merged source defines a truthful replacement rollback/fallback story once `w0` itself is retired or replaced |
| Whether a truthful replacement receiver-side contract can now be frozen from current evidence | Unsatisfied | `NA-0206` and `NA-0206A` narrowed the blocker to the post-`w0` boundary only | The replacement contract itself still is not defined by current evidence |
| Whether current evidence is enough to move directly to runtime retirement afterward | Unsatisfied | qsl-attachments/qsl-server posture is strong enough and live `W2` send behavior is clear | Runtime retirement would still require choosing unresolved post-`w0` semantics first |

## 3. Receive compatibility option set

| Option | Summary | Consistency with `DOC-ATT-004`..`DOC-ATT-007` | Effect on validated deployment flows | Rollback/fallback clarity | Delivery-semantics risk | More evidence required | Result |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `PB0` | Freeze the post-`w0` boundary now and make retirement implementation next | Inconsistent. Current docs freeze only the live `w0` boundary and explicitly stop short of the replacement post-`w0` receiver contract | Could silently break already-supported legacy payloads if the wrong post-`w0` rule is guessed | Not clear. No merged source defines the replacement rollback/fallback posture after `w0` disappears | High. Any immediate hard-reject or drain-window rule would be new semantics | Yes | Rejected |
| `PB1` | Continued support is the more truthful next posture | Inconsistent with `DOC-ATT-006` and `DOC-ATT-007`, which reject intentional longer-lived continued support as the evidence-backed near-term posture | Preserves current flows, but only by inventing a new longer-lived product posture and reevaluation model | Would require a new explicit continued-support contract and triggers not present today | Medium. It avoids an abrupt break but still adds unsupported semantics | Yes | Rejected |
| `PB2` | Current evidence still cannot freeze the post-`w0` boundary without semantic invention | Consistent. It preserves the already-frozen live boundary and refuses to guess the missing replacement contract | Keeps current validated flows unchanged on `main` | Truthful: live rollback/fallback stays unchanged, post-`w0` posture remains unresolved | Low. It avoids both dishonest removal and dishonest continued-support claims | No new runtime evidence; explicit policy direction is still required | Chosen |

`PB2` is the leading candidate because current merged evidence only freezes the live coexistence boundary. It does not define what policy replaces that boundary after `w0` is retired or replaced.

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
| `NA-0206A` / `DOC-ATT-007` | Freezes the live boundary truthfully and narrows the blocker to one final post-`w0` gate | Any evidence-backed answer to the post-`w0` question beyond "still unresolved" |

The accumulated evidence therefore answers the required questions as follows:
1. `w0` still makes receive compatibility load-bearing today because current `main` still exposes `w0` as an explicit rollback/coexistence mode and current tests/runbooks still rely on mixed legacy receive while that mode is live.
2. The project cannot yet freeze what happens after `w0` no longer exists without choosing semantics not present in merged policy.
3. Retirement implementation is therefore not truthfully next.
4. Continued support is also not truthfully next, because current evidence still does not justify an intentional longer-lived support posture as the product-state decision.

## 5. Decision

Chosen result:
- `PR2` / `PB2` — current evidence still cannot freeze the post-`w0` receive-retirement boundary without semantic invention.

Exact reason:
- Current merged evidence is strong enough to freeze the live boundary only:
  - `w0` remains the explicit rollback/coexistence control on current `main`;
  - already-supported `file_chunk` / `file_manifest` payloads remain supported while `w0` stays live; and
  - qsl-attachments and qsl-server add no new service-side blocker.
- Current merged evidence is not strong enough to freeze the replacement boundary after `w0` disappears.
- `DOC-CAN-005` still recognizes the legacy payload family, and `DOC-ATT-007` explicitly refused to invent the post-`w0` receiver contract.
- Advancing to implementation now would require selecting one of several materially different behaviors that the repo has not yet frozen:
  - immediate hard reject of all remaining legacy receive payloads once `w0` is retired;
  - a bounded drain-window receive contract tied to some observable condition; or
  - some other continued-support rule.

Exact remaining blocker:
- The repo still lacks an evidence-backed replacement receiver-side contract for already-supported legacy payloads after `w0` is retired or replaced.

Why this is the smallest truthful decision:
- it preserves the already-frozen live boundary exactly as documented on current `main`;
- it does not claim implementation is next when the missing post-`w0` semantics are still undefined;
- it does not relabel unresolved policy as intentional continued support; and
- it stops before governance closeout instead of inventing a successor queue item that current evidence does not justify.

Normative freeze from this item:
1. While `w0` remains live on current `main`, already-supported `file_chunk` / `file_manifest` payloads remain within the supported receiver contract.
2. Current rollback/fallback expectations remain unchanged while `w0` remains live.
3. This item does not authorize any post-`w0` immediate hard-reject rule, bounded drain-window rule, or intentional continued-support rule.
4. Runtime receive-retirement implementation is not truthful next from current evidence.
5. Governance closeout is not truthful next from current evidence without explicit new direction for the missing post-`w0` receiver contract.

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
