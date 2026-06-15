Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-15

# NA-0482 QSL Binding Negative Vector Suite Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0482 consumes the qsc binding negative tests, the bounded qsc binding
formal model, the qsc/refimpl mapping evidence, and the refimpl signature
provider-boundary tests. It authorizes a combined internal negative-vector
implementation successor.

Primary classification:

`BINDING_NEGATIVE_VECTOR_COMBINED_IMPLEMENTATION_READY`

Secret-material classification:

`VECTOR_SECRET_MATERIAL_SAFE_SCOPE_READY`

Selected successor:

`NA-0483 -- QSL Binding Negative Vector Suite Implementation Harness`

Selected future vector paths:

- `inputs/suite2/internal_negative_binding_vectors/README.md`
- `inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json`

The selected strategy is a manifest-plus-test strategy. The checked-in manifest
may store public or mutated message-frame bytes, abstract token traces,
metadata, expected reject outcomes, no-mutation expectations, and evidence
links. It must not store private keys, signing keys, KEM secret keys,
passphrases, runtime keys, backup keys, operator data, or user data. Any case
that requires private material must keep that material ephemeral during local
generation or test validation and store only public/mutated bytes or metadata.

NA-0482 does not implement vectors. It does not mutate inputs, runtime code,
crypto code, dependencies, Cargo files, lockfiles, workflows, qsc executable
tests, fuzz targets, formal models, refimpl source/tests, qsl-server,
qsl-attachments, qshield runtime, qshield-cli, website, public docs, README,
START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status, backup
plan, rollback paths, backup tree paths, durable Director State Index output,
or public technical paper content.

This is internal governance evidence only. No public-readiness claim is made.
No production-readiness claim is made. No public-internet-readiness claim is
made. No external-review-complete claim is made. No crypto-complete claim is
made. No vector-complete claim is made. No KEM-complete claim is made. No
signature-complete claim is made. No identity-complete claim is made. No
transcript-complete claim is made. No qsc/refimpl-equivalence-complete claim is
made. No provider-boundary-complete claim is made. No provider-RNG-complete
claim is made. No downgrade-proof claim is made. No replay-proof claim is made.
No side-channel-free claim is made. No vulnerability-free claim is made. No
bug-free claim is made. No perfect-crypto claim is made. Cargo audit green
remains dependency-health evidence only.

## Live NA-0482 scope

Live READY item at startup:

`NA-0482 -- QSL Binding Negative Vector Suite Authorization Plan`

Allowed NA-0482 mutation paths:

- this evidence document;
- `tests/NA-0482_qsl_binding_negative_vector_suite_authorization_testplan.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Read-only inspection covered qwork proof files, `NEXT_ACTIONS.md`,
`DECISIONS.md`, `TRACEABILITY.md`, rolling journal state, qsc source/tests/fuzz
layout, refimpl source/tests, formal models, `inputs/`, evidence docs,
testplans, CI scripts, workflows, backup status/plan files, qsl-backup hash
evidence, and prior response files.

Forbidden mutation scope preserved:

- implementation mutation;
- input/vector mutation;
- runtime, crypto, dependency, Cargo, lockfile, workflow mutation;
- qsc source or qsc executable-test mutation;
- refimpl source or test mutation;
- fuzz target or formal model mutation;
- qsl-server, qsl-attachments, qshield runtime, qshield-cli mutation;
- website, public docs, README, START_HERE mutation;
- qwork, qstart, qresume, qshell mutation;
- backup, restore, qsl-backup, backup status, backup plan, rollback, backup
  tree, systemd, timer, fstab mutation;
- durable Director State Index output and public technical paper content.

Acceptance criteria:

- qwork proof files verified without rerunning qwork;
- NA-0481 inheritance consumed;
- current input/vector surface inventoried;
- negative vector candidate inventory completed;
- vector format/storage strategy selected;
- secret-material and public-claim risk reviewed;
- combined versus split vector scope decided;
- exact NA-0483 successor selected;
- no implementation or vector mutation in NA-0482;
- no public overclaim;
- exactly one READY item remains before optional closeout.

Stop conditions preserved: stale qwork proof, PR #1233 not merged, queue not
READY NA-0482 at start, D-0952 present at start, omitted vector inventory or
risk review, unsafe successor ambiguity, root or nested audit failure,
qsl-backup source-list regression, public-safety red or missing, more than one
READY item, any forbidden mutation, or any prohibited readiness/completion
security claim.

## qwork proof-file verification

Codex read and copied the qwork proof files:

- `/srv/qbuild/work/NA-0482/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0482/.qwork/startup.qsl-protocol.json`

Verified required startup facts:

- `startup_result=OK`;
- lane `NA-0482`;
- repo `qsl-protocol`;
- path `/srv/qbuild/work/NA-0482/qsl-protocol`;
- clean worktree, index, and untracked state before NA-0482 edits;
- `READY_COUNT 1`;
- sole READY item: NA-0482;
- requested lane status: READY;
- proof HEAD and proof `origin/main`: `9c2c490a055c`.

Freshness proof:

- proof HEAD matched live HEAD before fetch;
- proof `origin_main` matched live `origin/main` before fetch;
- fetch did not advance `origin/main`;
- PR #1233 was verified MERGED at `9c2c490a055c`;
- current main public-safety completed success.

Codex did not run qwork, qstart, or qresume.

Recovered local validation artifact:

- Command context: required formal model checks were run during startup
  validation.
- Classification: recoverable local validation byproduct; Python created
  untracked bytecode cache under `formal/__pycache__`.
- Corrective action: removed only the generated bytecode cache and set the
  expectation that future formal reruns should use `PYTHONDONTWRITEBYTECODE=1`.
- Final result: no formal source/model mutation remains.

## NA-0481 inheritance

NA-0481 provided:

- D-0950 refimpl signature provider-boundary test implementation;
- D-0951 closeout restoring NA-0482;
- direct refimpl signature provider-boundary tests for wrong public-key length,
  wrong signature length, malformed signing-key length, tampered signature,
  wrong public key, and Err versus `Ok(false)` classification;
- feature-gated test coverage in
  `tools/refimpl/quantumshield_refimpl/tests/signature_provider_boundary.rs`;
- preserved caveats for provider RNG, qsc sanitized error mapping, KEM
  provider-boundary, qsc/refimpl equivalence, vectors, fuzz binding,
  side-channel, public-claim, and external-review readiness.

NA-0481 did not mutate refimpl source, qsc source/tests, vectors, fuzz targets,
formal models, dependencies, workflows, services, website, public docs, backup,
restore, or qsl-backup. NA-0482 consumes this as bounded internal evidence
only.

## Applicable Stewardship and Assurance Review

Level 1 stewardship is active. Level 2 and Level 3 remain future-gated. No
separate Directors are created. No steward has independent READY promotion,
merge authority, public-claim authority, or directive authority. Lead Director
final authority is preserved.

| Review item | Classification | Evidence | Disposition |
|---|---|---|---|
| Crypto / Protocol Steward | `BINDING_NEGATIVE_VECTOR_COMBINED_IMPLEMENTATION_READY` | NA-0476 qsc negative tests, NA-0478 opaque-token model, NA-0479 mapping, NA-0481 signature boundary tests | Authorize one internal vector suite with separate qsc/refimpl/formal sections |
| CI / Dependency / Release Health Steward | `DEPENDENCY_HEALTH_ACCEPTED_SUPPORTING_ONLY` | current-main public-safety, root audit, nested qsc fuzz lock audit, dependency probes | Supporting only; no vulnerability-free claim |
| Public Claims / External Review Steward | `EXTERNAL_REVIEW_READINESS_INCREMENTAL` | internal evidence chain and selected no-public-claim vector path | Internal vectors improve review readiness but are not external review |
| Product / Demo / Service Boundary Steward | `SERVICE_BOUNDARY_UNCHANGED` | no service, qshield, qshield-cli, website, README, or public-doc mutation | Keep vectors internal and away from product/demo claims |
| Local Ops / Backup / Restore Steward | `LOCAL_OPS_READ_ONLY_OK` | qwork proof files, qsl-backup SHA, installed source-list count, latest manifest count | No backup/restore; no off-host or restore claim |
| Best-Known-Method Review | `BEST_KNOWN_METHOD_FOR_SCOPE` | deterministic negative tests and formal/refimpl evidence already exist | Internal manifest-plus-test vectors are the best next deterministic artifact |
| Hostile Cryptographer Review | `HOSTILE_REVIEW_VECTOR_SCOPE_ACCEPTED_WITH_CAVEATS` | wrong KEM, wrong ciphertext, wrong signature, transcript mutation, replay, suite confusion, stale record, and provider-boundary cases | Include reject reason and no-mutation expectations; avoid completion claims |
| Red-Team Review | `REPLAY_ROLLBACK_DOWNGRADE_INCLUDED_NEXT` | NA-0476 and NA-0478 cover replay, stale record, and suite confusion | Keep these cases in the vector manifest before fuzz expansion |
| Production SRE Review | `SRE_RELEASE_CLAIM_BOUNDARY_PRESERVED` | vectors are internal evidence and not a deployment surface | No production or public-internet readiness claim |
| Side-Channel Caveat | `SIDE_CHANNEL_RESIDUAL_ACTIVE` | no timing, cache, branch, power, fault, or memory-access analysis in scope | Carry caveat into NA-0483 |
| Formal-Model Mapping Residual | `FORMAL_MODEL_MAPPING_SUPPORTING_ONLY` | NA-0478 uses opaque tokens, not provider internals | Include abstract token mapping cases as supporting-only manifest section |
| External-Review Readiness | `EXTERNAL_REVIEW_READINESS_INCREMENTAL` | selected manifest can be reviewed later, but public review is not complete | Keep internal labels and no-public-claim caveats |
| Release-Claim Boundary | `RELEASE_CLAIM_BOUNDARY_PRESERVED` | selected path is internal and not public interop vector directory | No release or security completion wording |
| Assurance Gap Review Trigger | `HIGHER_PRIORITY_RESIDUAL_SUPERSEDES_ASSURANCE_REVIEW` | exact NA-0483 vector successor is selected | No assurance-gap review is required now |

## Current input / vector surface inventory

Existing input conventions:

- `inputs/suite2/vectors/*.json` contains authoritative Suite-2 conformance
  vector JSON fixtures. The directory README labels them authoritative and CI
  executed. Files use `schema_version`, `format`, `protocol`, `source`, and
  `vectors` fields.
- `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json` is consumed by
  qsc/refimpl suite-id tests.
- `inputs/metadata_runtime/*.json` and `inputs/metadata_phase2/*.json` hold
  metadata/runtime fixture packs.
- `inputs/local_ops/**` holds internal local-ops fixtures for directive,
  response, polling, scope, and Director-state tooling.
- `inputs/phase2` and `inputs/phase3` contain archived/supporting zip inputs.

Existing consumer conventions:

- `scripts/ci/run_suite2_*_vectors.py` and `.github/workflows/public-ci.yml`
  consume `inputs/suite2/vectors/*.json`.
- refimpl tests consume several Suite-2 vector files directly.
- qsc tests consume the suite-id vector file.
- local-ops scripts consume `inputs/local_ops/**` fixtures.

Current binding evidence surface:

- qsc binding negative tests are executable Rust tests in
  `qsl/qsl-client/qsc/tests/kem_signature_transcript_binding_negative.rs`.
- formal binding cases are opaque-token scenarios in
  `formal/model_qsc_kem_signature_transcript_binding_bounded.py`.
- refimpl signature provider-boundary cases are executable tests in
  `tools/refimpl/quantumshield_refimpl/tests/signature_provider_boundary.rs`.
- There is no checked-in binding negative vector manifest for qsc frames,
  refimpl provider-boundary metadata, or formal-token mapping cases.

Path decision:

- Future binding negative vectors should not be placed directly under
  `inputs/suite2/vectors/` because that directory is labeled authoritative
  conformance vectors and is consumed by public CI globs.
- Future binding negative vectors should use
  `inputs/suite2/internal_negative_binding_vectors/` so the path carries an
  explicit internal-negative-evidence label and avoids confusion with public
  interoperability or conformance vectors.
- The path can support qsc/refimpl/formal traceability through separate
  manifest sections and evidence references.

## Negative vector candidate inventory

Future path for all selected candidates:

`inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json`

| Group | Candidate | Source evidence | Feasibility | Secret storage | Deterministic | Layer | Public-claim risk | Complexity | Acceptance criteria | Disposition |
|---|---|---|---|---|---|---|---|---|---|---|
| KEM | wrong peer KEM public key | NA-0476 wrong-public-key test; NA-0478 wrong KEM token | feasible as qsc frame/metadata | no private key stored | yes with captured public frame or metadata | qsc/formal | medium if placed in public vectors | medium | expected reject `peer_mismatch` or formal wrong-KEM reject; no completed session | select |
| KEM | stale KEM public record | NA-0476 stale public-record test; NA-0478 stale KEM/public-record token | feasible as metadata plus public-record bytes where safe | no private key stored | yes | qsc/formal | medium | medium | stale record reject and protected prior session unchanged | select |
| KEM | wrong/corrupted KEM ciphertext | NA-0476 corrupted B1 KEM ciphertext; NA-0479 KEM mapping | feasible with public mutated ciphertext bytes | no secret stored | yes | qsc/refimpl-supporting/formal | medium | medium | fail-closed reject, no A2 or completed session | select |
| KEM | wrong KEM key/ciphertext pairing | NA-0479 KEM mapping; pqkem768 tamper/mismatch evidence | feasible as generated ephemeral case with metadata-only if secret needed | no checked-in secret | yes with generation recipe | refimpl/qsc/formal | medium | medium | mismatch cannot produce success; expected reject/mismatch metadata | select as metadata/generation recipe |
| Signature | wrong signature identity/public record | NA-0476 wrong signature public-record test; NA-0478 wrong signature token | feasible as qsc metadata and public record bytes | no secret stored | yes | qsc/formal | medium | medium | reject `peer_mismatch` or wrong-signature token reject | select |
| Signature | cross-message B1/A2 signature replay | NA-0476 A2 signature spliced into B1; NA-0478 cross-message context | feasible with public signature bytes or metadata | no secret stored | yes | qsc/formal | medium | medium | `sig_invalid` or cross-message reject; no A2/session success | select |
| Signature | wrong signature message context | NA-0478 context token; NA-0476 cross-message replay | feasible as formal token and qsc metadata | no secret stored | yes | qsc/formal | low-medium | low | reject on context mismatch | select |
| Signature | tampered signature | NA-0481 tampered signature `Ok(false)`; NA-0476 signature reject | feasible as refimpl metadata and public signature bytes | no secret stored | yes | refimpl/qsc | medium | low | malformed-vs-invalid classification recorded | select |
| Signature | wrong public key | NA-0481 wrong public key `Ok(false)`; NA-0476 wrong identity | feasible as refimpl metadata and public key bytes | no secret stored | yes | refimpl/qsc | medium | low | invalid result, not success | select |
| Transcript/replay/suite | transcript mutation | NA-0476 transcript mutation; NA-0478 transcript token | feasible with mutated public frame bytes | no secret stored | yes | qsc/formal | medium | low | transcript-context reject; no completed session | select |
| Transcript/replay/suite | transcript truncation | NA-0476 frame parser/mutation helpers; existing parse vector conventions | feasible as metadata or future frame bytes | no secret stored | yes | qsc | medium | low | malformed/truncated frame rejects fail-closed | select |
| Transcript/replay/suite | replayed A1/B1/A2 | NA-0476 replayed A1; NA-0478 replay state | feasible as qsc/formal cases | no secret stored | yes | qsc/formal | medium | medium | replay reject; no duplicate output/session | select |
| Transcript/replay/suite | wrong-role replay | NA-0478 role tokens; qsc role transitions | feasible as formal metadata first | no secret stored | yes | formal/qsc | low-medium | medium | wrong role is rejected; no success output | select |
| Transcript/replay/suite | suite confusion / wrong suite token | NA-0476 suite confusion; NA-0478 wrong suite token | feasible with public parameter-block mutation | no secret stored | yes | qsc/formal | medium | low | suite-required reject; no fallback | select |
| Transcript/replay/suite | downgrade-style wrong suite block | NA-0476 suite block replacement; Suite-2 downgrade vectors | feasible with public bytes and expected reject metadata | no secret stored | yes | qsc/formal | medium | low | no silent fallback; reject only | select |
| Stale identity/rollback | stale public-record replay | NA-0476 stale public-record; NA-0478 stale record token | feasible as metadata and public record bytes | no secret stored | yes | qsc/formal | medium | medium | reject and preserve existing session bytes | select |
| Stale identity/rollback | public-record rollback | NA-0476 identity rotation stale state; qsc identity tests | feasible as metadata/generation recipe | no secret stored | yes | qsc | medium | medium | reject rolled-back record | select |
| Stale identity/rollback | identity rotation stale peer state | NA-0469/NA-0476 identity rotation evidence | feasible as qsc metadata | no secret stored | yes | qsc | medium | medium | stale peer state reject/no partial mutation | select |
| Stale identity/rollback | stale trusted pin mismatch | NA-0476 contact signature pin mutation; formal stale pin token | feasible as metadata | no secret stored | yes | qsc/formal | medium | low | trusted-pin mismatch reject | select |
| Refimpl provider-boundary | signature wrong public-key length | NA-0481 | feasible as metadata and length facts | no secret stored | yes | refimpl | low-medium | low | Err `InvalidKey` metadata | select |
| Refimpl provider-boundary | wrong signature length | NA-0481 | feasible as metadata and length facts | no secret stored | yes | refimpl | low-medium | low | Err `InvalidKey` metadata | select |
| Refimpl provider-boundary | malformed signing key | NA-0481 | feasible as metadata only | no signing key stored | yes | refimpl | low-medium | low | Err `InvalidKey` metadata | select metadata-only |
| Refimpl provider-boundary | tampered signature | NA-0481 | feasible with public signature mutation metadata | no secret stored | yes | refimpl | low-medium | low | `Ok(false)` metadata | select |
| Refimpl provider-boundary | wrong public key invalid | NA-0481 | feasible with public key metadata | no secret stored | yes | refimpl | low-medium | low | `Ok(false)` metadata | select |
| Refimpl provider-boundary | Err versus `Ok(false)` classification | NA-0481 | feasible as manifest classification table | no secret stored | yes | refimpl | low | low | malformed inputs Err, well-shaped invalid inputs false | select |
| Formal mapping | opaque-token trace cases | NA-0478 | feasible as abstract token cases | no secret stored | yes | formal | low if caveated | low | marker-to-scenario map included | select |

## Vector format / storage strategy review

| Option | Decision | Evidence | Future exact paths | Secret-material risk | Public-claim risk | Validation strategy | Why not broader |
|---|---|---|---|---|---|---|---|
| Static JSON negative vector manifests | select as part of manifest-plus-test | existing Suite-2 vectors use JSON with `schema_version`, `format`, `protocol`, `source`, and `vectors` | selected manifest JSON | safe only for public/mutated bytes and metadata | medium if confused with public conformance | JSON parse, schema checks, leak scan, existing tests | do not store private material |
| Generated vectors with checked-in manifest only | select for secret-needing cases | KEM pairings may require ephemeral secret material to generate | selected manifest JSON with generation recipe fields | low | low-medium | generation recipe reviewed, no private bytes in manifest | less external-review friendly if used alone |
| Test-fixture-only vectors under qsc tests | reject | qsc tests already exist; fixture-only near tests is less reusable | not selected | low | low | cargo test only | hides vector evidence from cross-layer traceability |
| Inputs-level internal negative vector suite | select | `inputs/` already holds vector/fixture packs and local-op fixtures | selected internal inputs subdirectory | low with no-secret rule | low-medium with explicit internal label | JSON parse, leak scan, existing qsc/refimpl/formal tests | not under public conformance directory |
| Formal-token vectors only | reject as sole strategy | NA-0478 already covers opaque tokens | not selected alone | very low | low | formal runner | too weak for qsc/refimpl byte traceability |
| Split qsc frame vectors and refimpl provider-boundary vectors | reject as separate lanes, select separate manifest sections | layers differ but can share one internal manifest | same selected manifest | low | low-medium | per-section validation markers | more lanes without reducing risk |
| No vector implementation yet; fuzz or side-channel first | reject | deterministic vector evidence should precede fuzz; side-channel remains residual | not selected | n/a | n/a | n/a | vectors are exact and lower risk than fuzz expansion |

Selected format:

- one JSON manifest with `schema_version`, `format`, `classification`,
  `claim_boundary`, `no_secret_material`, `source_evidence`, `sections`, and
  `vectors`;
- separate sections for `qsc_frame_cases`, `refimpl_provider_boundary_cases`,
  and `formal_token_cases`;
- per-case fields for `id`, `group`, `source_evidence`, `stored_material`,
  `generation_strategy`, `expected_reject`, `expected_no_mutation`,
  `public_claim_caveat`, and `validation_marker`;
- optional base64/hex bytes only for public or mutated handshake messages,
  public keys, public signatures, KEM ciphertexts, and non-secret metadata;
- no private keys, signing keys, KEM secret keys, passphrases, runtime keys,
  backup keys, operator data, or user data.

## Secret-material / public-claim risk review

Future vectors do not require checked-in private keys, signing keys, KEM secret
keys, passphrases, runtime keys, backup keys, or user data. Cases that require
key generation can be generated from ephemeral local test roots and then store
only public/mutated frame bytes or metadata. If a candidate cannot be expressed
without storing secret material, NA-0483 must omit the bytes and record a
metadata/generation-recipe case only, or stop for a split authorization.

Required storage rule:

- store only public messages, mutated public messages, public keys, KEM
  ciphertexts, signatures, abstract tokens, case metadata, expected reject
  outcomes, and no-mutation expectations;
- do not store private keys, secret keys, passphrases, runtime keys, backup
  keys, operator data, user data, or live service data.

Future label rule:

- `README.md` and manifest `claim_boundary` must label the suite as internal
  negative evidence, not public interoperability vectors, not external review,
  and not completion evidence.

Classification:

`VECTOR_SECRET_MATERIAL_SAFE_SCOPE_READY`

Rejected classifications:

- `VECTOR_SECRET_MATERIAL_REQUIRES_GENERATED_FIXTURES`: rejected as primary
  because the selected path can store public/mutated bytes or metadata, though
  some cases may use ephemeral generation recipes.
- `VECTOR_SECRET_MATERIAL_RISK_SPLIT_NEEDED`: rejected because secret-bearing
  storage is forbidden and no selected case requires it.
- `VECTOR_SECRET_MATERIAL_UNSAFE_STOP`: rejected because the suite can be
  formulated without checked-in secret material.

## Combined vs split vector scope decision

| Option | Decision | Evidence | Future exact paths if selected | Markers | Public-claim caveat |
|---|---|---|---|---|---|
| Combined binding negative vector suite | selected | qsc/refimpl/formal evidence is complete enough for an internal manifest and the path can keep sections separate | selected internal input directory plus NA-0483 governance paths | NA0483 vector markers | no vector-complete claim |
| qsc binding negative vectors first | rejected as sole lane | qsc frame cases are exact, but refimpl/formal metadata can be included safely | not selected alone | qsc-only markers would be incomplete | no qsc/refimpl-equivalence-complete claim |
| refimpl provider-boundary vectors first | rejected as sole lane | NA-0481 metadata is ready but lower value alone than cross-layer manifest | not selected alone | refimpl-only markers | no provider-boundary-complete claim |
| formal-token vectors first | rejected | formal tokens are already modeled and need qsc/refimpl mapping context | not selected alone | formal-only markers | no formal-proof-complete claim |
| split-scope authorization again | rejected | exact path, format, and no-secret rules are clear | not selected | n/a | no public overclaim |
| fuzz binding scope next | rejected | deterministic vectors should precede fuzz binding coverage | not selected | future fuzz markers only | no fuzz-complete claim |
| side-channel / secret-material scope next | rejected | side-channel remains residual, but vector material is safe with the selected rules | not selected | future side-channel markers only | no side-channel-free claim |

Decision:

KEM, signature, transcript/replay/suite, stale-record, refimpl
provider-boundary, and formal-token vector work should be combined in one
internal manifest with separate sections. Negative vectors should precede fuzz
binding coverage because they create deterministic, reviewable cases and clear
expected reject/no-mutation outcomes before randomized/adversarial expansion.

## Authorization decision

Primary classification:

`BINDING_NEGATIVE_VECTOR_COMBINED_IMPLEMENTATION_READY`

NA-0481 is consumed. Current input/vector surfaces are inventoried. The
negative vector candidate matrix is accepted. The vector format/storage
strategy is selected as manifest-plus-test. Secret-material/public-claim risk
is classified as safe for the selected no-secret storage scope. Combined
implementation is selected with separate manifest sections.

Selected successor:

`NA-0483 -- QSL Binding Negative Vector Suite Implementation Harness`

No implementation mutation is performed in NA-0482. No input/vector mutation is
performed in NA-0482. No runtime, crypto, dependency, Cargo, lockfile,
workflow, executable test, fuzz target, vector, formal model, refimpl, qsc,
service, public, website, backup, restore, or qsl-backup path is mutated by
this authorization decision.

## Future scope bundle

Future NA-0483 title:

`NA-0483 -- QSL Binding Negative Vector Suite Implementation Harness`

Future allowed paths:

- `inputs/suite2/internal_negative_binding_vectors/README.md`
- `inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json`
- `docs/governance/evidence/NA-0483_qsl_binding_negative_vector_suite_implementation_harness.md`
- `tests/NA-0483_qsl_binding_negative_vector_suite_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future forbidden unless a later exact directive changes scope:

- runtime/source mutation;
- crypto implementation mutation;
- dependency, Cargo, lockfile, workflow mutation;
- qsc source or executable-test mutation;
- refimpl source or test mutation;
- formal/fuzz mutation;
- vector mutation outside the selected internal path;
- qsl-server, qsl-attachments, qshield runtime, qshield-cli mutation;
- website, public docs, README, START_HERE mutation;
- backup, restore, qsl-backup, status, plan, rollback, backup tree mutation;
- public readiness or crypto completion claims.

Future NA-0483 objective:

Implement the selected internal binding negative vector manifest and README,
linking qsc frame cases, refimpl provider-boundary metadata, and formal-token
cases to existing evidence while preserving no-secret-material and no-public
overclaim boundaries.

## Future validation / marker plan

Common future NA-0483 no-overclaim markers:

- `NA0483_VECTOR_SCOPE_CONSUMED_OK`
- `NA0483_NO_SECRET_MATERIAL_IN_VECTORS_OK`
- `NA0483_NO_RUNTIME_CHANGE_OK`
- `NA0483_NO_DEPENDENCY_CHANGE_OK`
- `NA0483_NO_WORKFLOW_CHANGE_OK`
- `NA0483_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0483_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0483_NO_KEM_COMPLETE_CLAIM_OK`
- `NA0483_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0483_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0483_NO_TRANSCRIPT_COMPLETE_CLAIM_OK`
- `NA0483_NO_VECTOR_COMPLETE_CLAIM_OK`
- `NA0483_NO_REPLAY_PROOF_CLAIM_OK`
- `NA0483_NO_DOWNGRADE_PROOF_CLAIM_OK`
- `NA0483_ONE_READY_INVARIANT_OK`

Future vector markers:

- `NA0483_VECTOR_KEM_WRONG_PUBLIC_KEY_CASE_OK`
- `NA0483_VECTOR_KEM_WRONG_CIPHERTEXT_CASE_OK`
- `NA0483_VECTOR_SIGNATURE_WRONG_IDENTITY_CASE_OK`
- `NA0483_VECTOR_SIGNATURE_CROSS_MESSAGE_REPLAY_CASE_OK`
- `NA0483_VECTOR_TRANSCRIPT_MUTATION_CASE_OK`
- `NA0483_VECTOR_REPLAY_CASE_OK`
- `NA0483_VECTOR_SUITE_CONFUSION_CASE_OK`
- `NA0483_VECTOR_STALE_PUBLIC_RECORD_CASE_OK`
- `NA0483_VECTOR_REFIMPL_SIGNATURE_BOUNDARY_CASE_OK`
- `NA0483_VECTOR_FORMAL_TOKEN_MAPPING_CASE_OK`

Future validation commands should include:

- JSON parse for the selected manifest;
- deterministic no-secret scan over the selected internal vector directory;
- scope guard against the exact NA-0483 paths;
- link-check, leak-scan, overclaim scan, classifier, PR body preflight, and
  goal-lint;
- existing qsc binding negative test;
- existing refimpl signature provider-boundary test;
- formal binding model and full formal runner;
- root cargo audit and nested qsc fuzz lock audit;
- qsc adversarial syntax and PR qsc-adversarial-smoke.

## Public claim / external review / website boundary

The selected future vector path is internal evidence. It is not public website
content, not public docs, not a public technical paper, not public
interoperability vectors, not external review completion, and not release
readiness. The future manifest must carry these caveats in its README and
metadata. The existing `inputs/suite2/vectors/` authoritative conformance
directory remains unchanged by NA-0482.

## Rejected alternatives

- Put binding vectors directly under `inputs/suite2/vectors/`: rejected because
  that directory is authoritative conformance vector surface and CI-globbed.
- Store full generated private-key fixtures: rejected because secret/private
  material must not be checked in.
- Implement qsc executable tests in NA-0482: rejected because NA-0482 is
  authorization-only.
- Implement fuzz binding before vectors: rejected because deterministic
  negative vectors should set expected reject/no-mutation behavior first.
- Run a side-channel lane before vectors: rejected because side-channel remains
  residual but the selected vector material can be kept no-secret and internal.
- Split KEM/signature/transcript/stale-record into more authorization lanes:
  rejected because exact format/path and no-secret rules are clear.

## Backup-impact statement

NA-0482 changes only qsl-protocol governance/testplan/traceability/journal
paths. Codex did not run backup or restore. Codex did not mutate qsl-backup,
backup status files, backup plan files, rollback paths, `/backup/qsl`, systemd,
timers, or fstab. The qsl-backup installed script SHA matched
`e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`. The
installed source list contains the Codex ops path exactly once, and the latest
read-only daily manifest reviewed during NA-0482 contains that source exactly
once. This remains same-host continuity evidence only. No off-host backup,
restore proof, disaster recovery, backup completion, key custody, or key
recovery claim is made.

## Next recommendation

Proceed to NA-0483 after NA-0482 evidence merges and post-merge public-safety
is green. NA-0483 should implement only the selected internal vector README and
manifest, preserve the no-secret-material rule, preserve no runtime/source
mutation by default, and keep all public/release/security completion caveats.
