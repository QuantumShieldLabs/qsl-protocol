Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-19

# NA-0499 QSL Side-Channel / Secret-Material Lifecycle Assurance Scope Authorization Plan

## Executive summary

NA-0499 is an authorization-only core assurance lane. It consumes the NA-0498 / D377 checkpoint, inventories qsc and refimpl secret-material lifecycle surfaces, classifies existing direct and supporting evidence, and selects the next highest-value successor without implementing code or tests.

Primary classification selected: `SECRET_MATERIAL_DIAGNOSTIC_NO_OUTPUT_TEST_READY`.

Selected successor:

### NA-0500 -- QSL qsc Secret-Material Diagnostic / No-Output Boundary Test Implementation Harness
Status: READY
Goals: G1, G2, G3, G4, G5

The successor is a narrow qsc integration-test lane. It should exercise selected reject/error/diagnostic paths and assert stdout, stderr, panic/crash-visible output, and test-visible diagnostics do not expose secret-shaped material, private-key markers, passphrases, shared-secret labels, signing-key labels, KEM secret labels, production-like private endpoints, or user/operator data markers.

NA-0499 makes no implementation mutation, no qsc source/test/fuzz/Cargo mutation, no corpus/vector/input mutation, no workflow/script/helper mutation, no dependency/lockfile mutation, no formal/refimpl/service/public/backup mutation, and no public claim expansion.

## Live NA-0499 scope

Allowed mutation paths used by this evidence PR:

- `docs/governance/evidence/NA-0499_qsl_side_channel_secret_material_lifecycle_assurance_scope_authorization_plan.md`
- `tests/NA-0499_qsl_side_channel_secret_material_lifecycle_assurance_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Read-only inspection covered qwork proof files, D377 response evidence, NA-0498 evidence and testplan, D-0985, D-0986, the NA-0499 queue block, qsc source/tests/fuzz/corpus surfaces, qsc diagnostic/output surfaces, qsc TUI bootstrap evidence, qsc stored identity/session/vault surfaces, refimpl KEM/signature provider-boundary tests, formal binding models, validator and qsc-adversarial scripts, recent governance evidence, and read-only backup boundary material.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

The qwork proof files existed and were copied into the proof root:

- `/srv/qbuild/work/NA-0499/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0499/.qwork/startup.qsl-protocol.json`

Proof fields verified:

- `startup_result=OK`
- `lane=NA-0499`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0499/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0499`
- `requested_lane_status=READY`

The proof HEAD and proof origin/main matched live pre-fetch HEAD and origin/main at `89a50cfa5ecb`. The qwork `.kv` and `.json` proofs mirrored required values. Disk usage was below the hard stop before fetch: `/` was 77% and `/backup/qsl` was 24%.

After fetch, local `main` was fast-forward checked against `origin/main`. `origin/main` equals or descends from `89a50cfa5ecb`.

Read-only qsl-backup boundary:

- installed helper SHA-256 matched `e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`;
- the Codex ops source inclusion count was exactly 1;
- no backup or restore was run.

## NA-0498 / D377 inheritance

NA-0498 is closed and NA-0499 is restored READY as the sole READY item.

Inherited facts accepted from D377, D-0985, D-0986, NA-0498 evidence, NA-0498 testplan, and current main health:

- NA-0498 selected primary classification `CORE_ASSURANCE_SIDE_CHANNEL_SECRET_MATERIAL_NEXT`.
- The classification was selected because side-channel and secret-material lifecycle assurance is now a higher-value core security reducer than process/tooling lanes.
- Public-safety is green on current main.
- qsc-adversarial-smoke is green on current main.
- qsc binding negative tests exist and cover selected wrong KEM, signature, transcript, replay, suite-confusion, stale-record, reject, no-success-output, and no completed-session mutation surfaces.
- The vector consumer test exists and passes as internal manifest schema/mapping/no-secret-policy evidence.
- The corpus validator exists and passes against the binding corpus and all qsc fuzz corpus paths.
- qsc-adversarial integrates the corpus validator before fuzz execution.
- Key lifecycle zeroization and cleanup evidence exists for selected pending/session/vault/passphrase boundaries.
- Provider-error no-mutation evidence exists for selected qsc decapsulation failure behavior.

Inherited supporting-only evidence:

- The formal KEM/signature/transcript binding model is bounded opaque-token evidence only.
- refimpl KEM/signature provider-boundary evidence supports qsc reasoning but does not replace qsc lifecycle evidence.
- Governance/no-claim evidence preserves release and public-claim boundaries.

Inherited residuals:

- side-channel non-claim;
- secret-material lifecycle and memory-erasure residuals;
- X25519/ephemeral provider RNG residual;
- refimpl provider RNG residual;
- qsc/refimpl/formal mapping residual;
- external-review readiness residual;
- backup/off-host/restore/key-custody residual as governance-only local-ops context.

No secret-material-complete claim and no side-channel-free claim is inherited or introduced.

## Secret-material lifecycle inventory

| Area | Secret material or sensitive material | Lifecycle phase | Current evidence | Risk | Likely next lane |
|---|---|---|---|---|---|
| qsc identity KEM secret key | identity KEM secret key generated by rotation/bootstrap and stored through vault secret names | generation, storage, use, load, output/error, zeroization/drop | direct selected tests for identity rotation/provider failure and key lifecycle cleanup; vault storage evidence; governance caveats | output leakage, retention/lifetime, partial-state persistence, diagnostic leakage, side-channel caveat | NA-0500 diagnostic/no-output first; later zeroization expansion |
| qsc identity signing key | identity signing key generated by rotation/bootstrap and stored through vault secret names | generation, storage, use, output/error, zeroization/drop | direct selected provider-failure tests and TUI bootstrap transactionality; refimpl signature boundary supporting evidence | output leakage, retention/lifetime, signing-key label leakage, side-channel caveat | NA-0500 diagnostic/no-output |
| qsc handshake pending secret | pending KEM secret, pending DH secret, confirm key, transcript hash, and pending state in vault/session flow | generation, storage, use, error/reject, cleanup | direct key lifecycle tests for pending cleanup and provider-error no-mutation; binding negative tests for reject/no session mutation | retention/lifetime, partial-state persistence, crash artifact leakage | NA-0500 for diagnostics; later zeroization expansion |
| qsc KEM shared secret | provider KEM shared secret from encap/decap and derived transcript inputs | generation, use, reject path, output/log path | direct qsc binding reject tests; formal bounded support; no direct memory lifetime proof | output leakage, diagnostic leakage, side-channel caveat | NA-0500 diagnostic/no-output |
| qsc X25519/ephemeral secret | ephemeral DH private key and shared secret in handshake | generation, use, reject path | governance residual; source inventory only in this lane | side-channel caveat, RNG/provider boundary, memory retention | future X25519/ephemeral provider RNG scope |
| qsc transcript/confirm material | transcript hash, confirm key, confirm MAC, suite context | generation, use, reject path | direct qsc binding negative tests and formal bounded evidence | output leakage, misuse of "no success output" as no-secret-output proof | NA-0500 diagnostic/no-output |
| qsc session-store key | `qsp_session_store_key_v1` in vault and encrypted session blobs | storage, use, error/reject, backup/archive | direct session at-rest tests and key lifecycle session-store tests | retention, crash/test artifact leakage, backup/key-custody | NA-0500 diagnostic/no-output; backup remains governance-only |
| qsc vault passphrase/operator data | passphrase files, stdin/env bridge, process passphrase, vault KDF key | generation/input, storage in memory, output/log path, zeroization/drop | direct vault passphrase redaction and no-plaintext tests; output redaction functions; TUI bootstrap tests | passphrase leakage to stdout/stderr/log/panic/test artifacts | NA-0500 diagnostic/no-output |
| qsc diagnostics/log/output | markers, stderr, stdout, panic redaction, optional logs | output/log/diagnostic path | direct generic diagnostic tests and output redaction helpers; not unified across selected secret-bearing reject paths | diagnostic/log leakage; demo/operator confusion | NA-0500 diagnostic/no-output |
| qsc TUI bootstrap | pre-generated identity material and vault/default writes | generation, in-memory use, durable commit/reject, output | direct transactionality tests; explicit in-memory lifetime caveat | in-memory retention and TUI output leakage | NA-0500 diagnostic/no-output first; later TUI lifetime scope |
| qsc public record / trusted pin | public KEM/signature record, trusted pin files/records | storage, reject path, output/diagnostic path | direct binding/stale public-record tests and vector metadata; public material is safe by policy | private endpoint or private identity metadata leakage if diagnostics overprint | NA-0500 diagnostic/no-output |
| qsc fuzz corpus/vector inputs | checked-in binding corpus and internal vector manifest | test/fuzz/corpus artifact | direct validator and vector consumer evidence | checked-in secret material, private endpoint labels | already validator-gated; maintain through NA-0500 markers |
| refimpl KEM | KEM secret key and shared secret in provider roundtrip tests | generation, provider return, malformed/reject path | direct provider tests for shape/fail-closed behavior; no qsc lifecycle proof | provider-return overinterpretation, side-channel caveat | future refimpl boundary lane |
| refimpl signature | signing key and signature verifier provider behavior | generation, sign/verify, malformed/reject path | direct provider-boundary tests for malformed vs invalid returns | signing-key/output leakage not exhaustively tested | future refimpl boundary lane |
| formal model | opaque KEM/signature/transcript tokens | model state, reject/no-output assertion | formal bounded evidence | overinterpretation as crypto, side-channel, or qsc/refimpl proof | future qsc/refimpl/formal mapping lane |
| backup/off-host/restore | local backup source list and external backup material | backup/archive path | read-only qsl-backup hash/source-list proof only | backup/key-custody and restore residual | governance-only residual, not NA-0500 substitute |

## Existing evidence classification

| Evidence item | Classification | Notes |
|---|---|---|
| qsc key lifecycle zeroization | DIRECT_ZEROIZATION_EVIDENCE | Direct selected evidence for pending secret cleanup, session-store insertion after success, vault passphrase redaction, and no plaintext boundary. It does not prove comprehensive memory erasure. |
| provider-error no mutation | DIRECT_NO_MUTATION_EVIDENCE | Direct qsc evidence for selected `pq_decap_failed` no session/pending/vault mutation behavior. |
| qsc binding negative rejection/no session mutation | DIRECT_QSC_REJECT_EVIDENCE | Direct selected qsc reject/no completed-session mutation/no success-output evidence for KEM, signature, transcript, replay, suite, and stale-record cases. |
| qsc-adversarial validator integration | DIRECT_VALIDATOR_EVIDENCE | Validator runs as part of qsc-adversarial before fuzz phases. |
| corpus validator | DIRECT_VALIDATOR_EVIDENCE | Direct validator evidence for checked-in secret-like material in qsc fuzz corpus paths. |
| vector consumer | DIRECT_VALIDATOR_EVIDENCE | Direct qsc integration evidence for manifest schema/mapping/no-secret-policy boundaries, not dynamic crypto execution. |
| formal binding model | FORMAL_BOUNDED_EVIDENCE | Bounded opaque-token state-machine evidence only. |
| refimpl signature provider-boundary | DIRECT_REFIMPL_PROVIDER_BOUNDARY_EVIDENCE | Direct refimpl provider-boundary evidence; supporting for qsc. |
| refimpl KEM boundary | DIRECT_REFIMPL_PROVIDER_BOUNDARY_EVIDENCE | Direct refimpl KEM roundtrip/fail-closed shape evidence; supporting for qsc. |
| TUI bootstrap pre-generation transactionality | DIRECT_NO_MUTATION_EVIDENCE | Direct selected TUI durable-state evidence; in-memory lifetime remains residual. |
| X25519 / ephemeral secrets | RESIDUAL_OPEN | Source inventory only in this lane; no direct lifecycle or provider RNG evidence selected here. |
| qsc diagnostic/logging output | SUPPORTING_ONLY_EVIDENCE | Generic diagnostics and output redaction exist; cross-path secret-material reject/error no-output remains not covered as a unified test. |
| qsc CLI stdout/stderr | SUPPORTING_ONLY_EVIDENCE | Several CLI tests assert deterministic/redacted output; not focused on selected secret-bearing reject paths. |
| TUI output | SUPPORTING_ONLY_EVIDENCE | TUI tests cover selected output routing and bootstrap no-secret-output, but not broad TUI secret lifetime. |
| crash/test artifact output | RESIDUAL_OPEN | Panic redaction hook exists, but reject/error/test artifact no-secret scanning is not unified across lifecycle surfaces. |
| backup/off-host/restore/key custody | GOVERNANCE_CAVEAT_ONLY | qsl-backup boundary is read-only local ops evidence only. |
| side-channel behavior | GOVERNANCE_CAVEAT_ONLY | no side-channel-free claim. Provider/dependency constant-time behavior is not established. |
| secret-material-complete status | NOT_COVERED | no secret-material-complete claim. |

## Option review

| Option | Disposition | Risk reduced | Evidence gap addressed | Feasibility | Scope risk | Public-claim risk | External-review value | Future paths |
|---|---|---|---|---|---|---|---|---|
| Option 1 - qsc diagnostic / no-output boundary tests | SELECT | output/log/diagnostic leakage | missing unified qsc reject/error/no-output proof | high; qsc integration-test only | low if test-only | low with strict no-claim markers | high | `qsl/qsl-client/qsc/tests/secret_material_diagnostic_boundary.rs`; NA-0500 evidence/testplan/governance |
| Option 2 - qsc key lifecycle / zeroization expansion | REJECT FOR NOW | retention/lifetime | deeper wipe/zeroization evidence | medium | medium because semantics could change | medium | high | future authorization lane after NA-0500 |
| Option 3 - qsc TUI / bootstrap in-memory lifetime scope | REJECT FOR NOW | TUI secret lifetime | in-memory bootstrap caveat | medium | medium | medium | medium | future TUI lifetime scope |
| Option 4 - refimpl secret-material boundary scope | REJECT FOR NOW | provider boundary | refimpl KEM/signature no-output | medium | medium | medium | medium | future refimpl lane |
| Option 5 - side-channel caveat evidence package | REJECT FOR NOW | claim hygiene | governance-only caveat consolidation | high | low | low | medium | future claim package; not direct enough now |
| Option 6 - X25519 / ephemeral provider RNG boundary | REJECT FOR NOW | ephemeral RNG/lifetime | X25519 residual | medium | medium | medium | medium | future X25519/ephemeral scope |
| Option 7 - qsc/refimpl/formal mapping core assurance | REJECT FOR NOW | equivalence mapping | bounded-model-to-runtime residual | medium | medium/high | medium | high | future mapping lane |
| Option 8 - process/tooling lane | REJECT | none unless blocker appears | no current blocker | high | low | low | low | not selected |

P0/P1/P2 risk disposition:

- P0: do not select a lane that changes protocol/wire/crypto/auth/state semantics or weakens fail-closed behavior.
- P1: select the direct qsc diagnostic/no-output test because it is narrow, direct, and claim-safe.
- P2: keep zeroization expansion, TUI lifetime, refimpl boundary, X25519/ephemeral RNG, qsc/refimpl/formal mapping, and side-channel caveat package as future lanes.

## Hostile Cryptographer Review

A hostile cryptographer would distrust first any evidence that appears to jump from bounded tests to broad lifecycle assurance. The weakest-looking areas are secret lifetime in memory, provider/dependency side-channel behavior, qsc/refimpl non-equivalence, and diagnostics that might print material during reject/error paths.

Current tests that must not be overinterpreted:

- The formal model is bounded opaque-token state-machine evidence, not cryptographic, side-channel, provider, or qsc/refimpl equivalence proof.
- The vector consumer is schema/mapping/no-secret-policy evidence, not dynamic crypto execution of every vector.
- Key lifecycle zeroization tests are selected lifecycle evidence, not comprehensive memory-erasure evidence.
- Provider-error no-mutation tests are selected qsc reject evidence, not provider-error-complete evidence.

The least directly covered secret material is the cross-cutting path from secret generation/use to stdout, stderr, logs, panic/crash-visible output, and retained test artifacts. NA-0500 best reduces that risk without overclaim because it can assert absence of secret-shaped material in selected qsc output paths without mutating qsc source or dependencies.

## Red-Team Review

An attacker or red-team reviewer would first inspect:

- stdout/stderr from reject/error paths;
- optional logs and marker output;
- panic/crash-visible output;
- test artifacts and temporary roots;
- stored session blobs and vault secrets;
- identity files and public-record/trusted-pin state;
- KEM shared secret handling and pending secret cleanup;
- signing-key handling and malformed-signing-key errors;
- passphrases/operator data from file/stdin/env bridge paths;
- backup/archive material and off-host custody.

NA-0500 produces the most useful direct reject/no-output evidence because it targets the observable outputs attackers and operators can inspect first while leaving deeper memory-erasure and side-channel claims untouched.

## Production SRE Review

The lifecycle gap most likely to mislead operators is a sanitized success/failure marker being mistaken for broad no-secret-output coverage. Demonstrations and future service operation would be risky if diagnostic messages, panic output, or logs expose private-key labels, passphrases, shared-secret labels, production-like endpoints, or user/operator data.

Unexpected retention could occur in temp roots, vault files, session blobs, logs, panic output, or backup/archive paths. NA-0500 is narrow enough to prove selected output boundaries while preserving the no production-readiness claim boundary.

## Release-Claim Boundary Review

This directive preserves:

- no public-readiness claim;
- no production-readiness claim;
- no public-internet-readiness claim;
- no external-review-complete claim;
- no crypto-complete claim;
- no KEM-complete claim;
- no signature-complete claim;
- no identity-complete claim;
- no provider-RNG-complete claim;
- no secret-material-complete claim;
- no zeroization-complete claim;
- no memory-erasure-complete claim;
- no side-channel-free claim;
- no replay-proof claim;
- no downgrade-proof claim;
- no vulnerability-free claim;
- no bug-free claim;
- no perfect-crypto claim.

Cargo audit green remains dependency-health evidence only.

## Prioritization matrix

| Candidate | Risk reduced | Directness of evidence | Attack relevance | External-review value | Implementation feasibility | Scope risk | Overclaim risk | Dependency/workflow risk | Recommended disposition | Next-lane yes/no |
|---|---|---|---|---|---|---|---|---|---|---|
| qsc diagnostic / no-output boundary tests | output/log/diagnostic leakage | high | high | high | high | low | low | low | select | yes |
| qsc key lifecycle / zeroization expansion | retention/lifetime | high if implemented | high | high | medium | medium | medium | low | defer | no |
| TUI bootstrap / in-memory lifetime | TUI lifetime caveat | medium | medium | medium | medium | medium | medium | low | defer | no |
| refimpl secret-material boundary | provider-boundary leakage | medium | medium | medium | medium | medium | medium | low | defer | no |
| side-channel caveat evidence package | claim hygiene | governance-only | medium | medium | high | low | low | low | defer | no |
| X25519 / ephemeral provider RNG boundary | ephemeral RNG/lifetime | medium | medium | medium | medium | medium | medium | low | defer | no |
| qsc/refimpl/formal mapping | mapping residual | medium | medium | high | medium | medium/high | medium | low | defer | no |
| process/tooling lane | process friction | low for core risk | low | low | high | low | low | varies | reject | no |

## Authorization decision

Primary classification selected: `SECRET_MATERIAL_DIAGNOSTIC_NO_OUTPUT_TEST_READY`.

Required decision gates completed:

- NA-0498/D377 consumed.
- Lifecycle inventory completed.
- Evidence classification completed.
- Option review completed.
- Hostile cryptographer/red-team/SRE/release-claim reviews completed.
- Prioritization matrix completed.
- Exact NA-0500 successor selected.
- No implementation mutation performed.
- No public claim expansion introduced.
- Exactly one READY successor remains mandatory.

## Selected NA-0500 successor

### NA-0500 -- QSL qsc Secret-Material Diagnostic / No-Output Boundary Test Implementation Harness
Status: READY
Goals: G1, G2, G3, G4, G5

Objective:
Implement a bounded qsc integration test that exercises selected reject/error/diagnostic paths and asserts stdout/stderr/error text/test-visible diagnostics do not expose secret-shaped material, private-key markers, passphrases, shared-secret labels, signing-key labels, KEM secret labels, production-like private endpoints, or user/operator data markers.

## Future scope bundle

Allowed scope for NA-0500:

- `qsl/qsl-client/qsc/tests/secret_material_diagnostic_boundary.rs`
- `docs/governance/evidence/NA-0500_qsl_qsc_secret_material_diagnostic_no_output_boundary_test_implementation_harness.md`
- `tests/NA-0500_qsl_qsc_secret_material_diagnostic_no_output_boundary_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden scope for NA-0500:

- qsc source mutation;
- qsc fuzz target/Cargo mutation;
- corpus/vector/input mutation;
- workflow/script/helper mutation;
- dependency/lockfile mutation;
- refimpl/formal/service/public/qshield/qsl-server/qsl-attachments mutation;
- backup/restore/qsl-backup mutation;
- no public-readiness claim, no secret-material-complete claim, no zeroization-complete claim, no memory-erasure-complete claim, and no side-channel-free claim.

Acceptance criteria for NA-0500:

- selected reject/error/diagnostic paths are tested;
- output scrub checks fail on secret-like markers;
- no qsc source/dependency/workflow mutation occurs;
- no secret-material-complete claim and no side-channel-free claim is made;
- exactly one READY item remains after closeout.

## Future validation / marker plan

Future NA-0500 markers:

- `NA0500_SECRET_MATERIAL_SCOPE_CONSUMED_OK`
- `NA0500_NO_SECRET_OUTPUT_BOUNDARY_OK`
- `NA0500_DIAGNOSTIC_REJECT_PATHS_CHECKED_OK`
- `NA0500_PRIVATE_KEY_MARKER_ABSENT_OK`
- `NA0500_PASSPHRASE_MARKER_ABSENT_OK`
- `NA0500_KEM_SECRET_MARKER_ABSENT_OK`
- `NA0500_SIGNATURE_SECRET_MARKER_ABSENT_OK`
- `NA0500_SHARED_SECRET_MARKER_ABSENT_OK`
- `NA0500_NO_QSC_SOURCE_CHANGE_OK`
- `NA0500_NO_DEPENDENCY_CHANGE_OK`
- `NA0500_NO_WORKFLOW_CHANGE_OK`
- `NA0500_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0500_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0500_NO_SECRET_MATERIAL_COMPLETE_CLAIM_OK`
- `NA0500_NO_SIDE_CHANNEL_FREE_CLAIM_OK`
- `NA0500_ONE_READY_INVARIANT_OK`

Recommended future validation:

- `cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture`
- inherited qsc binding/vector/key-lifecycle/provider-error targeted tests as appropriate;
- validator binding corpus scan;
- validator all qsc fuzz corpus scan;
- formal model checks;
- root and nested cargo audit;
- cargo fmt;
- qsc-adversarial shell syntax checks;
- scope guard, link-check, leak-scan, overclaim scan, classifier, PR body preflight, and goal-lint.

## Public claim / website / external review boundary

NA-0499 does not modify public docs, website paths, README, `START_HERE.md`, qsl-server, qsl-attachments, qshield, qshield-cli, services, formal models, refimpl, qsc source/tests/fuzz/Cargo, workflows, scripts, helpers, corpora, vectors, inputs, backup paths, or qsl-backup.

The successor improves internal qsc assurance only. It preserves no public-readiness claim, no external-review-complete claim, no crypto-complete claim, no secret-material-complete claim, no zeroization-complete claim, no memory-erasure-complete claim, no side-channel-free claim, no vulnerability-free claim, no bug-free claim, and no perfect-crypto claim.

## Rejected alternatives

- Key lifecycle / zeroization expansion is important but broader and more likely to invite overclaim before the observable no-output boundary is unified.
- TUI bootstrap lifetime is important but narrower and already has direct transactionality evidence.
- refimpl boundary work supports qsc but does not replace qsc output evidence.
- Side-channel caveat packaging is useful claim hygiene but not direct enough for the next lane.
- X25519 / ephemeral provider RNG remains a real residual but ranks below cross-cutting qsc output leakage risk.
- qsc/refimpl/formal mapping remains useful but recent lanes already advanced it; diagnostic/no-output is the clearer next direct test.
- Process/tooling work is rejected because no current process blocker prevents core security assurance.

## Backup-impact statement

NA-0499 has no backup impact. The directive read qsl-backup boundary evidence only. It did not run backup or restore, did not mutate `/backup/qsl`, did not mutate qsl-backup, did not mutate backup status or plan files, and did not treat backup/off-host/restore/key-custody evidence as a substitute for qsc protocol assurance.

## Next recommendation

Close out NA-0499 after this evidence PR merges and post-merge public-safety is green. Restore exactly one READY successor: `NA-0500 -- QSL qsc Secret-Material Diagnostic / No-Output Boundary Test Implementation Harness`.
