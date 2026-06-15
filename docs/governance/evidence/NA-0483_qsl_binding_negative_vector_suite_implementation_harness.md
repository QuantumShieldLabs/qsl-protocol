Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-15

# NA-0483 QSL Binding Negative Vector Suite Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0483 consumes the NA-0482 authorization decision and adds the selected
internal negative binding vector README and JSON manifest under
`inputs/suite2/internal_negative_binding_vectors/`.

The manifest includes 34 internal metadata vector entries across KEM binding,
signature binding, transcript/replay/suite binding, stale identity/rollback,
refimpl signature provider-boundary metadata, and NA-0478 formal-token mapping.

This is bounded internal negative evidence only. It is not public/conformance
vector evidence, not interoperability evidence, not external-review-complete
evidence, and not a public security completion claim.

Primary marker set:

- `NA0483_VECTOR_SCOPE_CONSUMED_OK`
- `NA0483_NO_SECRET_MATERIAL_IN_VECTORS_OK`
- `NA0483_NO_RUNTIME_CHANGE_OK`
- `NA0483_NO_DEPENDENCY_CHANGE_OK`
- `NA0483_NO_WORKFLOW_CHANGE_OK`
- `NA0483_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0483_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0483_NO_VECTOR_COMPLETE_CLAIM_OK`
- `NA0483_ONE_READY_INVARIANT_OK`

## Live NA-0483 scope

Live READY item at startup:

`NA-0483 -- QSL Binding Negative Vector Suite Implementation Harness`

Allowed implementation paths:

- `inputs/suite2/internal_negative_binding_vectors/README.md`
- `inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json`

Allowed governance paths:

- this evidence document;
- `tests/NA-0483_qsl_binding_negative_vector_suite_implementation_testplan.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Forbidden mutation scope preserved:

- `inputs/suite2/vectors/`;
- runtime, crypto, dependency, Cargo, lockfile, workflow, executable test,
  fuzz target, formal model, qsc source/test, refimpl source/test, qsl-server,
  qsl-attachments, qshield runtime, qshield-cli, website, public docs, README,
  START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status, backup
  plan, rollback, backup tree, durable Director State Index output, and public
  technical paper content.

## qwork proof-file verification

Codex read and copied:

- `/srv/qbuild/work/NA-0483/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0483/.qwork/startup.qsl-protocol.json`

Verified fields:

- `startup_result=OK`;
- lane `NA-0483`;
- repo `qsl-protocol`;
- path `/srv/qbuild/work/NA-0483/qsl-protocol`;
- clean worktree, index, and untracked state;
- `READY_COUNT 1`;
- READY item `NA-0483`;
- requested lane status READY.

Proof HEAD and proof `origin/main` both matched live pre-fetch state at
`9174881e578d`. Fetch did not advance `origin/main`. PR #1235 was verified
MERGED at `9174881e578d`.

Codex did not run qwork, qstart, qresume, sudo, backup, restore, cargo update,
or cargo generate-lockfile.

## NA-0482 inheritance

NA-0482 selected:

- classification `BINDING_NEGATIVE_VECTOR_COMBINED_IMPLEMENTATION_READY`;
- secret-material classification `VECTOR_SECRET_MATERIAL_SAFE_SCOPE_READY`;
- manifest-plus-test internal vector strategy;
- selected input paths now implemented by NA-0483;
- separate qsc binding, refimpl signature provider-boundary, and
  formal-token mapping sections.

NA-0482 required that checked-in vectors contain no private keys, signing keys,
KEM secret keys, passphrases, runtime keys, backup keys, operator data, user
data, or live service data. NA-0483 preserves that boundary.

## Pre-mutation review

Before implementation, both selected input paths were absent:

- `inputs/suite2/internal_negative_binding_vectors/README.md`
- `inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json`

The worktree was clean. Existing public/conformance-style Suite-2 vectors were
inventoried under `inputs/suite2/vectors/` and were not modified.

## Vector suite implementation summary

Added:

- `inputs/suite2/internal_negative_binding_vectors/README.md`
- `inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json`

The README defines the manifest schema, internal-only claim boundary, no-secret
material policy, sections, and JSON validation command.

The JSON manifest is valid JSON and contains:

- top-level `schema_version: "1"`;
- suite `qsl-internal-negative-binding`;
- status `internal-negative-evidence-only`;
- `public_claim_boundary`;
- `secret_material_policy`;
- `traceability`;
- `sections`;
- 34 vector entries.

## README implementation proof

The README states:

- these are internal negative evidence only;
- they are not public, conformance, or interoperability vectors;
- no private keys, KEM secret keys, signing keys, passphrases, runtime keys,
  backup keys, operator data, user data, or live service data are stored;
- secret material needed by future tests must be generated ephemerally;
- manifest sections are `qsc_binding`,
  `refimpl_signature_provider_boundary`, and `formal_token_mapping`;
- validation command is
  `python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null`;
- no public-readiness, crypto-complete, vector-complete, KEM-complete,
  signature-complete, identity-complete, transcript-complete, replay-proof,
  downgrade-proof, side-channel-free, vulnerability-free, bug-free,
  perfect-crypto, external-review-complete, backup-complete, or restore-proof
  claim is made.

## Manifest implementation proof

The manifest schema is metadata-first. Every vector includes:

- `id`;
- `group`;
- `layer`;
- `title`;
- `description`;
- `source_evidence`;
- `material_policy`;
- `input_kind`;
- `mutation_kind`;
- `expected_result`;
- `validation_status`;
- `public_claim_caveat`;
- `related_markers`.

Allowed layers are:

- `qsc_frame`;
- `refimpl_signature_provider_boundary`;
- `formal_token_mapping`.

Every vector has `contains_secret_material`, `contains_private_key`,
`contains_passphrase`, and `contains_user_data` set to false.

## KEM vector proof

KEM entries:

- `kem_wrong_peer_public_key`;
- `kem_stale_public_record`;
- `kem_wrong_ciphertext`;
- `kem_wrong_key_ciphertext_pair`.

Markers include:

- `NA0483_VECTOR_KEM_WRONG_PUBLIC_KEY_CASE_OK`;
- `NA0483_VECTOR_KEM_WRONG_CIPHERTEXT_CASE_OK`;
- `NA0483_VECTOR_STALE_PUBLIC_RECORD_CASE_OK`.

## Signature vector proof

Signature entries:

- `signature_wrong_identity_public_record`;
- `signature_cross_message_replay_b1_as_a2`;
- `signature_wrong_message_context`;
- `signature_tampered_signature`;
- `signature_wrong_public_key`.

Markers include:

- `NA0483_VECTOR_SIGNATURE_WRONG_IDENTITY_CASE_OK`;
- `NA0483_VECTOR_SIGNATURE_CROSS_MESSAGE_REPLAY_CASE_OK`;
- `NA0483_VECTOR_REFIMPL_SIGNATURE_BOUNDARY_CASE_OK`.

## Transcript / replay / suite vector proof

Transcript, replay, and suite entries:

- `transcript_mutation`;
- `transcript_truncation`;
- `replayed_a1`;
- `replayed_b1`;
- `replayed_a2`;
- `wrong_role_replay`;
- `suite_confusion_wrong_suite_token`;
- `downgrade_wrong_suite_block`.

Markers include:

- `NA0483_VECTOR_TRANSCRIPT_MUTATION_CASE_OK`;
- `NA0483_VECTOR_REPLAY_CASE_OK`;
- `NA0483_VECTOR_SUITE_CONFUSION_CASE_OK`.

## Stale identity / rollback vector proof

Stale identity and rollback entries:

- `stale_public_record_replay`;
- `public_record_rollback`;
- `identity_rotation_stale_peer_state`;
- `stale_trusted_pin_mismatch`.

Markers include:

- `NA0483_VECTOR_STALE_PUBLIC_RECORD_CASE_OK`;
- `NA0483_NO_IDENTITY_COMPLETE_CLAIM_OK`.

## Refimpl signature provider-boundary vector proof

Refimpl signature provider-boundary entries:

- `refimpl_signature_wrong_public_key_length`;
- `refimpl_signature_wrong_signature_length`;
- `refimpl_signature_malformed_signing_key`;
- `refimpl_signature_tampered_signature_invalid`;
- `refimpl_signature_wrong_public_key_invalid`;
- `refimpl_signature_err_vs_false_classification`.

Markers include:

- `NA0483_VECTOR_REFIMPL_SIGNATURE_BOUNDARY_CASE_OK`;
- NA-0481 provider-boundary markers for wrong length, malformed signing-key,
  tampered signature, wrong public key, and Err versus false classification.

## Formal-token mapping vector proof

Formal-token entries:

- `formal_wrong_kem_token`;
- `formal_wrong_signature_token`;
- `formal_transcript_mutation`;
- `formal_replay`;
- `formal_suite_confusion`;
- `formal_stale_public_record`;
- `formal_no_session_mutation_on_reject`.

These map to NA-0478 opaque-token evidence and remain supporting-only. They do
not claim provider-internal proof or formal-proof completion.

## No secret material proof

The manifest validator checked:

- JSON parses;
- vector IDs are unique;
- all required vector IDs are present;
- all required NA-0483 markers are present;
- every vector has `material_policy`;
- every vector sets secret/private/passphrase/user-data booleans false;
- no PEM/private key header-like value is present;
- no live endpoint secret-like value is present;
- no obvious passphrase marker-like value is present.

Result: PASS.

## Public/conformance vector separation proof

No path under `inputs/suite2/vectors/` was modified. The internal manifest is
under `inputs/suite2/internal_negative_binding_vectors/` and labels itself as
internal negative evidence only.

## No runtime / source / dependency / workflow mutation proof

Changed paths are limited to:

- `inputs/suite2/internal_negative_binding_vectors/README.md`
- `inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json`
- `docs/governance/evidence/NA-0483_qsl_binding_negative_vector_suite_implementation_harness.md`
- `tests/NA-0483_qsl_binding_negative_vector_suite_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No runtime, qsc, refimpl, dependency, Cargo, lockfile, workflow, fuzz, formal,
service, website, public-doc, backup, qsl-backup, qwork, qstart, qresume, or
qshell path is changed.

## Applicable stewardship and D328 assurance review

| Review | Classification | Disposition |
|---|---|---|
| Best-Known-Method Review | `BEST_KNOWN_METHOD_FOR_SCOPE` | Internal manifest-plus-test metadata is the smallest deterministic artifact after NA-0482. |
| Hostile Cryptographer Review | `HOSTILE_REVIEW_SCOPE_ACCEPTED_WITH_CAVEATS` | Wrong KEM, wrong ciphertext, wrong signature, transcript mutation, replay, suite confusion, stale-record, and provider-boundary cases are represented with reject/no-mutation expectations. |
| Red-Team Review | `REPLAY_ROLLBACK_DOWNGRADE_INCLUDED_WITH_CAVEATS` | Replay, rollback, stale-record, and downgrade-style suite confusion are included without replay-proof or downgrade-proof claims. |
| Production SRE Review | `SRE_RELEASE_CLAIM_BOUNDARY_PRESERVED` | Internal metadata only; no production or public-internet readiness claim. |
| Side-Channel Caveat | `SIDE_CHANNEL_RESIDUAL_ACTIVE` | No timing, cache, branch, power, fault, or memory-access analysis is performed. |
| Formal-Model Mapping Residual | `FORMAL_MAPPING_SUPPORTING_ONLY` | NA-0478 tokens are mapped, not extended into provider-internal proof. |
| External-Review Readiness | `EXTERNAL_REVIEW_READINESS_INCREMENTAL` | Evidence becomes easier to review later; external review is not complete. |
| Release-Claim Boundary | `RELEASE_CLAIM_BOUNDARY_PRESERVED` | No release/security completion wording is introduced. |
| Assurance Gap Review Trigger | `FUZZ_BINDING_RESIDUAL_NEXT` | Deterministic vectors leave fuzz binding coverage as the strongest next residual. |

Lead Director final authority and exactly one READY item remain preserved.

## Validation

Startup validation completed:

- qwork proof verification;
- PR #1235 merge verification;
- queue and decision proof;
- current-main public-safety proof;
- root cargo audit;
- nested qsc fuzz lock audit;
- formal model and formal runner;
- inherited qsc binding/provider-RNG/key-lifecycle/provider-error tests;
- inherited refimpl signature provider-boundary and `pqkem768` tests;
- qsc adversarial marker proof;
- qsl-backup read-only SHA/source-list proof.

Post-implementation validation includes:

- manifest JSON validation with `python3 -m json.tool`;
- custom manifest validator;
- exact scope guard;
- link-check;
- leak scan;
- added-line overclaim scan;
- classifier;
- PR body preflight and goal-lint;
- inherited formal/qsc/refimpl tests;
- root and nested audits;
- qsc adversarial syntax and smoke as available locally and in CI.

## Scope guard

The NA-0483 implementation is limited to the seven allowed paths listed in
this evidence document. The public/conformance vector path
`inputs/suite2/vectors/` remains unchanged.

## Backup-impact statement

No backup or restore was run. qsl-backup was not mutated. Backup status and
backup plan files were read-only. Same-host continuity evidence remains a
caveat and is not off-host backup, restore, or disaster-recovery proof.

## Successor selection

Selected successor after successful NA-0483 merge and post-merge public-safety:

`NA-0484 -- QSL Fuzz Binding Coverage Scope Authorization Plan`

Rationale: deterministic internal negative vectors are now present. The next
strongest residual is whether fuzzing exercises parser/binding/replay/
downgrade/stale-record semantics or remains parser-only.

## Next recommendation

Merge NA-0483 only after local validation and required PR checks pass. If
post-merge public-safety is green, close out NA-0483 and restore the selected
NA-0484 fuzz binding coverage authorization plan as the sole READY item. Do
not implement NA-0484 in NA-0483.
