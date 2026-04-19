Goals: G4

Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-19
Replaces: n/a
Superseded-By: n/a

# NA-0236 KT Serialization/Profile + BundleTBS / Bundle-Signature Canon Closure Evidence

## Summary

`NA-0236` is complete from already-merged implementation state.

Merged PR: `#705`
Merge commit SHA: `af9300ac04a8`
Prior main SHA: `1438fb2015bd`
Final PR head SHA: `22705479d3d9`
Merged at: `2026-04-19T12:53:38Z`

## Merged-State Proof

- PR `#705` is `MERGED` on refreshed GitHub truth.
- The merge commit is `af9300ac04a8`.
- Parent 1 is prior `main` `1438fb2015bd`.
- Parent 2 is final PR head `22705479d3d9`.
- The merged result is a normal merge commit, not a squash or rebase.

## Exact Implementation Outcome On Main

Refreshed `main` contains exactly the expected `NA-0236` KT canon-closure paths:

- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/canonical/DOC-CAN-008_QSP_Key_Transparency_Profile_and_Bundle_Signature_Closure_v0.1.0_DRAFT.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `docs/schemas/DOC-SCL-002_Shared_Schemas_v1.0.json`
- `docs/spec-closure/DOC-SCL-001_Suite_Parameter_Registry_Deployment_Profiles_v1.0_DRAFT.md`
- `docs/spec-closure/DOC-SCL-002_Shared_Schemas_Error_Reason_Code_Registry_v1.0_DRAFT.md`
- `tests/NA-0236_kt_serialization_profile_bundle_signature_closure_testplan.md`

This merged implementation/evidence outcome remains in-scope:

- `DOC-CAN-008` freezes `QSP-4.3.2-KT1`, canonical `BundleLeafData`, canonical `BundleTBS`, bundle-signature coverage, pinned `kt_log_id`, exact `kt_sth` / inclusion / consistency-proof meanings, and responder obligations.
- The supporting schema/spec-closure docs now point at that same canonical KT profile and fail-closed error model.
- No additional runtime, manifest, lockfile, `.github`, qsl-server, qsl-attachments, qsc-desktop, or website/public-runtime paths were part of the merged `NA-0236` change set.

## DOC-AUD-003 Blocker Closure Proof

Before PR `#705`, `DOC-AUD-003` still classified `F05` as design-blocked because the repo had not canonically frozen KT serialization/profile closure or `BundleTBS` / bundle-signature semantics.

Refreshed `main` now closes that prerequisite blocker:

- `DOC-CAN-008` explicitly states that it closes the KT prerequisite ambiguities identified by `DOC-AUD-003` and the focused KT audit.
- The canonical doc now defines the exact verifier-relevant meanings for `kt_log_id`, `kt_sth`, `kt_inclusion_proof`, `kt_consistency_proof`, `BundleLeafData`, `BundleTBS`, and responder obligations.
- The supporting schema/spec-closure docs on `main` carry the same `QSP-4.3.2-KT1` anchor, so a later implementation lane no longer has to invent KT serialization, coverage, or policy in code.

## Closeout Scope

This closeout PR is governance-only and introduces no runtime changes. It archives durable merged evidence, marks `NA-0236` `DONE`, records the blocker-closure basis in governance, and promotes `NA-0237` as the next truthful sole READY item.
