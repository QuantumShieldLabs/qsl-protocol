# DOC-ATT-003 — Default Attachment Path Promotion and Legacy In-Message Policy v0.1.0 DRAFT

Status: Draft

Purpose:
- Freeze the default-path and legacy-path policy implied by the accumulated attachment evidence.
- State exactly when attachment-path default promotion is justified and what still blocks any legacy `<= 4 MiB` deprecation.
- Define the next truthful implementation boundary without changing qsc, qsl-attachments, or qsl-server runtime behavior in this item.

Non-goals:
- No qsc runtime/client changes.
- No qsl-attachments runtime or deployment changes.
- No qsl-server changes.
- No default-path promotion implementation in this item.
- No legacy deprecation implementation in this item.

## 1. Evidence baseline

The policy in this document is grounded by the following completed evidence:
- `NA-0199` proved the current coexistence rule directly from live qsc behavior:
  - `<= 4 MiB` sends stay on the legacy in-message path.
  - exact-threshold `4 MiB` sends also stay on the legacy path.
  - `> 4 MiB` sends reject cleanly without `--attachment-service`.
  - `> 4 MiB` sends use the attachment path when explicitly configured.
  - neither path can produce a false `peer_confirmed`.
- `NA-0200A` proved the constrained-host lane on weak-host `qsl` plus the real relay:
  - service-backed `> 4 MiB`, `16 MiB`, `64 MiB`, and `100 MiB` runs completed truthfully.
  - restart/recovery, quota/reject paths, and limited concurrency remained contract-faithful.
  - the degraded exact-threshold legacy case was bounded weak-relay saturation, not attachment-service correctness failure.
- `NA-0201` proved the stronger reference deployment on `qatt`:
  - message-only, attachment-only, and mixed traffic succeeded over the real relay.
  - restart/recovery, bounded concurrency, and short mixed soak stayed bounded.
  - the remaining degraded threshold stages stayed on the weak relay / legacy path rather than on the stronger attachment-service host.
- `NA-0201A` proved the bounded kitchen-sink lane:
  - mixed message + attachment traffic, large files through `100 MiB`, restart/recovery, concurrency up to `8`, and a `30` minute soak stayed truthful on `qatt`.
  - the only degraded required stages remained the weak-host / weak-relay legacy threshold path at `< 4 MiB` and exact `4 MiB`, both as bounded fail-closed saturation with honest delivery state.

## 2. Option set

| Option | Summary | Result | Why |
| --- | --- | --- | --- |
| `P0` | Keep the current coexistence rule unchanged: legacy in-message for `<= 4 MiB`; attachment path only above threshold and only when `--attachment-service` is explicitly supplied | Rejected | Still truthful today, but it is now too conservative relative to the accumulated validated-deployment evidence. The next blocker is no longer more validation before above-threshold promotion. |
| `P1` | Promote the attachment path by default above threshold in validated deployments, while leaving legacy `<= 4 MiB` unchanged | Chosen | This matches the constrained-host, reference-host, and kitchen-sink evidence while preserving the validated legacy path for `<= 4 MiB`. |
| `P2` | Promote the attachment path by default for all sizes in validated deployments | Rejected | No evidence justifies replacing the validated `<= 4 MiB` legacy path, and doing so would create unnecessary silent-break and operator-surprise risk. |
| `P3` | Begin deprecating the legacy `<= 4 MiB` in-message path | Rejected | The explicit deprecation gates from `DOC-ATT-002` are still unmet: migration/rollback and no-silent-break proof are missing. |
| `P4` | Keep coexistence indefinitely as the long-term product posture | Rejected | Earlier evidence validated coexistence as a truthful current rule, not as the justified permanent product posture. |

## 3. Chosen policy

### 3.1 Policy result

The chosen policy result is:
- promote the attachment path by default above the threshold in validated deployments; and
- keep the legacy in-message path unchanged for `<= 4 MiB`; and
- keep legacy deprecation blocked until separate readiness evidence exists.

### 3.2 Validated deployment meaning

For the purpose of default-path promotion, a deployment is "validated" only when all of the following are true:
- the `DOC-ATT-002` default-promotion gate is satisfied:
  - the full constrained-host ladder has been executed against deployed `qsl-attachments` plus the real relay;
  - at least one stronger reference deployment has also completed the ladder cleanly enough to separate saturation from correctness;
  - ingress, TLS, and log-hygiene requirements are met;
  - restart, recovery, retention, and quota evidence is recorded;
  - no unresolved correctness blocker remains; and
  - the project can state an honest operating envelope for constrained hosts and for the reference profile;
- the relay deployment passes the qsl-server canonical-compatibility guard rather than relying on a stale legacy-only relay deployment;
- the attachment-service endpoint is provided through an operator-controlled validated configuration surface, not inferred from an arbitrary network guess.

### 3.3 Default-path behavior

Once `NA-0202A` is implemented, the normative above-threshold behavior for validated deployments is:
- for file sends with size `> 4 MiB`, qsc should choose the attachment path by default when validated deployment configuration provides the attachment-service endpoint;
- the current per-send `--attachment-service` flag should no longer be mandatory in validated deployments; it may remain as an explicit override or diagnostic surface;
- if validated attachment-service configuration is absent for an above-threshold send, the send must fail closed explicitly rather than silently choosing another path.

This item does not change current qsc behavior. Until `NA-0202A` lands, the existing explicit `--attachment-service` requirement remains the live runtime behavior.

### 3.4 Legacy-path behavior

The legacy in-message path remains explicitly unchanged in this policy item:
- `<= 4 MiB` remains on the legacy in-message path;
- exact-threshold `4 MiB` remains on the legacy in-message path;
- no silent fallback from an above-threshold attachment attempt to the legacy path is allowed;
- `accepted_by_relay`, attachment acceptance, and `peer_confirmed` remain distinct and truthful on both path families.

### 3.5 Legacy deprecation blockers

Legacy `<= 4 MiB` deprecation remains blocked until all of the following exist:
- default-path promotion above threshold has already been implemented and justified in practice;
- an explicit migration plan exists;
- an explicit rollback plan exists;
- the project can prove no silent break for legacy-sized flows during the migration window; and
- any remaining fallback behavior is explicit and operator-visible rather than silent.

## 4. Why this is the smallest truthful decision

This is the smallest truthful decision because it:
- uses only already-gathered evidence from `NA-0199`, `NA-0200A`, `NA-0201`, and `NA-0201A`;
- authorizes the next implementation step only for above-threshold default promotion on validated deployments;
- does not overclaim readiness to deprecate the validated `<= 4 MiB` legacy path;
- preserves the no-silent-break and no-silent-fallback rules; and
- leaves runtime implementation to the follow-on queue item instead of mutating policy and code in the same directive.

## 5. Queue implication

The next truthful queue step after this policy freeze is:
- `NA-0202A — qsc Default Attachment Path Promotion Above Threshold`

The legacy-deprecation track remains a separate follow-on and must not be collapsed into the promotion step:
- `NA-0202B — Legacy In-Message Deprecation Readiness`

## References

- `tests/NA-0199_legacy_transition_validation.md`
- `docs/design/DOC-ATT-002_qsl-attachments_Deployment_and_Operational_Hardening_Contract_v0.1.0_DRAFT.md`
- `qsl-attachments/tests/NA-0003_constrained_host_validation_evidence.md`
- `qsl-attachments/tests/NA-0004_reference_deployment_validation_evidence.md`
- `qsl-attachments/tests/NA-0005_stress_soak_chaos_evidence.md`
- `qsl-server/scripts/check_relay_compatibility.sh`
- `DECISIONS.md`
- `TRACEABILITY.md`
