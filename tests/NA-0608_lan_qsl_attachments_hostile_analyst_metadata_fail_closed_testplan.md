# NA-0608 LAN qsl-attachments Hostile Analyst / Metadata and Fail-Closed Adversarial Test Plan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-06

Goals: G1, G2, G3, G4, G5

## Scope

This test plan records class-safe proof review for the NA-0608 hostile-analyst
stress and analysis of the working LAN qsl-attachments path, executed under
directive QSL-DIR-2026-07-06-541 (D541) and its Addendum A (bounded LAN runtime
provisioning). Per operator direction, negatives and analysis ran on the
build-host loopback harness; the SSH loopback-forward transport is inherited
from NA-0607 and not re-run here.

It does not authorize qsc source mutation, qsl-server source mutation,
qsl-attachments source mutation, dependency or lockfile mutation, workflow
dispatch, Tailnet, public endpoint exposure, sudo, system install, personal
laptop file access, or private-value publication. The disposable runtime clones
and builds occurred outside the tracked repository and are not committed.

## Required Markers

- NA0608_D1207_TRANSITION_CONSUMED_OK
- NA0608_D1208_CLOSEOUT_CONSUMED_OK
- NA0608_FRESH_QWORK_PROOF_OK
- NA0608_CURRENT_MAIN_HEALTH_OK
- NA0608_D1209_ABSENT_BEFORE_IMPLEMENTATION_OK
- NA0608_LAN_REACHABILITY_IDENTITY_OK
- NA0608_ADDENDUM_A_PROVISIONING_AUTHORIZED_OK
- NA0608_LOOPBACK_ONLY_BIND_CLASSIFIED_OK
- NA0608_SATELLITE_VALIDATION_OK
- NA0608_QSC_BUILD_NO_TRACKED_SOURCE_MUTATION_OK
- NA0608_BASELINE_REAL_PATH_NO_SEED_FALLBACK_PASS_OK
- NA0608_NEG_WRONG_CAPABILITY_FAIL_CLOSED_OK
- NA0608_NEG_WRONG_RESUME_TOKEN_FAIL_CLOSED_OK
- NA0608_NEG_CORRUPTED_OBJECT_FAIL_CLOSED_OK
- NA0608_NEG_MISSING_OBJECT_FAIL_CLOSED_OK
- NA0608_NEG_REPLAY_DUPLICATE_FAIL_CLOSED_OK
- NA0608_NEG_WRONG_ROUTE_FAIL_CLOSED_OK
- NA0608_NEG_WRONG_PEER_FAIL_CLOSED_OK
- NA0608_NEG_CORRUPTED_DESCRIPTOR_CLASSIFIED_OK
- NA0608_NO_MUTATION_ON_REJECT_CLASSIFIED_OK
- NA0608_PLAINTEXT_KEY_CAPABILITY_EXPOSURE_CLASSIFIED_OK
- NA0608_SEED_FALLBACK_NOT_USED_OK
- NA0608_METADATA_MATRIX_OK
- NA0608_CLEANUP_DONE_OK
- NA0608_PRIVATE_MATERIAL_SCAN_OK
- NA0608_RESULT_CLASSIFICATION_SELECTED_OK
- NA0608_SUCCESSOR_SELECTED_OK
- NA0608_ONE_READY_INVARIANT_OK

## Validation Plan (class-only)

1. Verify operator-run qwork proof and live main health before any mutation;
   verify D-1207/D-1208 once and D-1209 absent; READY_COUNT 1 with READY NA-0608.
2. Verify LAN reachability and remote user class read-only; verify the
   qscwork-owned test workspace present.
3. Provision the disposable build-host runtime (satellite clone/build/validate;
   qsc build with no tracked-source mutation); verify loopback-only bind.
4. Exercise one real two-party baseline send/receive/decrypt/validate with real
   identities and no seed fallback; confirm digest match.
5. Exercise each hostile-analyst negative; confirm a deterministic fail-closed
   reject class and zero plaintext output; confirm no-mutation-on-reject class.
6. Review qsl-server and qsl-attachments logs and object storage for plaintext,
   key, and capability exposure using a unique canary payload; confirm zero
   exposure.
7. Confirm seed fallback not used.
8. Record the class-only metadata-minimization matrix.
9. Clean up runtime listeners and the qscwork test scratch; run the
   no-private-material publication scan.
10. Select result classification and the NA-0609 successor.

## Result

Selected result classification:
`LAN_QSL_ATTACHMENTS_HOSTILE_ANALYST_FAIL_CLOSED_PASS` (loopback-harness variant).

Evidence: `docs/governance/evidence/NA-0608_lan_qsl_attachments_hostile_analyst_metadata_fail_closed_harness.md`.
