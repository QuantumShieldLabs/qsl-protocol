Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-13

# NA-0473 QSL Identity / Provider RNG Assurance Gap Review Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0473 consumes the completed identity/provider RNG evidence chain through
NA-0472 and performs a governance-only assurance gap review.

Primary classification:

`CRYPTOGRAPHER_REVIEW_TRANSCRIPT_BINDING_NEXT`

Selected successor:

`NA-0474 -- QSL KEM / Signature / Transcript Binding Read-Only Audit Plan`

The completed chain now has bounded internal qsc evidence for KEM provider RNG
failure, B1 signature provider RNG failure, A2 signature provider RNG
no-output behavior, lazy identity provider RNG failure, legacy/public-record
identity provider RNG failure, CLI identity rotation provider RNG failure, TUI
bootstrap pre-generation transactionality, route/contact/attachment RNG
failure, key lifecycle and zeroization evidence, provider-error no-mutation,
qsc adversarial smoke integration, formal model supporting evidence, refimpl
`pqkem768` provider tests, root and nested dependency-health evidence, current
main public-safety, and backup/log-code chain boundary evidence.

The review finds no immediate implementation blocker inside NA-0473 scope. The
highest-value next assurance lane is a read-only audit of KEM, signature,
transcript, identity, suite, replay, downgrade, stale-record, and state
transition binding across qsc and refimpl evidence. Side-channel and
secret-material lifecycle, formal-model mapping, qsc/refimpl provider-boundary,
X25519/ephemeral, external-review readiness, release-claim, supply-chain, and
CI-helper gaps remain active residuals.

NA-0473 mutates governance evidence only. It does not mutate runtime code,
crypto code, dependencies, Cargo manifests, lockfiles, workflows, executable
tests, fuzz targets, vectors, formal models, refimpl, qsl-server,
qsl-attachments, qshield runtime, qshield-cli, website, public docs, README,
START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status files,
backup plan files, rollback subtree paths, `/backup/qsl`, or public technical
paper content.

## Live NA-0473 scope

Allowed NA-0473 mutation paths:

- `docs/governance/evidence/NA-0473_qsl_identity_provider_rng_assurance_gap_review_plan.md`
- `tests/NA-0473_qsl_identity_provider_rng_assurance_gap_review_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Read-only inspection included qwork proof files, governance evidence and
testplans, `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`, rolling
journal, qsc source/tests/fuzz, refimpl source/tests, formal models, inputs,
Cargo manifests and lockfiles, scripts, workflows, qsl-backup hash evidence,
backup logs/manifests, backup status, and backup plan.

Forbidden mutation scope was preserved for implementation, runtime, crypto,
dependency, Cargo, lockfile, workflow, executable test, fuzz target, vector,
formal model, refimpl, qsl-server, qsl-attachments, qshield runtime,
qshield-cli, website, public docs, README, START_HERE, qwork, qstart, qresume,
qshell, qsl-backup, backup status, backup plan, rollback subtree, `/backup/qsl`,
public technical paper, and durable Director State Index output paths.

Acceptance criteria:

- qwork proof files verified without rerunning qwork;
- NA-0472 consumed;
- hostile cryptographer, red-team, production SRE, side-channel,
  formal-model, external-review, release-claim, dependency/supply-chain/CI, and
  qsc/refimpl/provider-boundary reviews completed;
- completed evidence chain inventoried;
- assurance findings matrix ranked;
- exactly one NA-0474 successor selected;
- no implementation mutation;
- no public overclaim;
- exactly one READY item remains mandatory.

Stop conditions preserved: stale qwork proof, PR #1215 not merged, unexpected
queue/decision state, un-inventoriable evidence chain, omitted review domain,
unsafe successor selection, root or nested audit failure, qsl-backup source-list
regression, public-safety red or missing, more than one READY item, any
forbidden mutation, or any prohibited public/readiness/complete/security claim.

## qwork proof-file verification

Codex read the qwork proof files:

- `/srv/qbuild/work/NA-0473/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0473/.qwork/startup.qsl-protocol.json`

Verified required startup facts:

- `startup_result=OK`
- lane `NA-0473`
- repo `qsl-protocol`
- path `/srv/qbuild/work/NA-0473/qsl-protocol`
- clean worktree, index, and untracked state
- READY_COUNT 1
- sole READY item: NA-0473
- requested lane status: READY
- proof HEAD and proof `origin/main`: `3765cdfc7d05`

Freshness proof:

- proof HEAD matched live HEAD before fetch;
- proof `origin_main` matched live `origin/main` before fetch;
- fetch did not advance `origin/main`;
- `origin/main` equals and descends from PR #1215 merge commit
  `3765cdfc7d05`;
- PR #1215 was verified MERGED;
- current main public-safety completed success.

Codex did not run `qwork`, `qstart`, or `qresume`.

## NA-0472 inheritance

NA-0472 is DONE and D-0932/D-0933 are accepted.

Inherited assurance:

- TUI `/init` and init wizard pre-generate bootstrap identity KEM/signature
  material before `tui_try_vault_init` and before durable account/default
  writes.
- Forced TUI bootstrap KEM provider RNG failure under
  `QSC.TUI.BOOTSTRAP.IDENTITY.KEM_KEYPAIR` emits sanitized
  `identity_secret_unavailable` / `rng_failure_forced` output before vault,
  account/default, identity secret, and self public-record writes.
- Forced TUI bootstrap signature provider RNG failure under
  `QSC.TUI.BOOTSTRAP.IDENTITY.SIG_KEYPAIR` has the same bounded failure order.
- No-cfg TUI bootstrap ignores the seam selector and preserves normal behavior.
- The in-memory secret lifetime caveat remains active because pre-generation
  lengthens the lifetime of generated identity material during the durable
  commit sequence.

NA-0472 residuals carried into this review:

- X25519 / ephemeral generation remains residual.
- refimpl provider RNG failure remains residual.
- formal/model RNG failure state transitions remain residual.
- fuzz/vector RNG failure remains residual.
- side-channel and secret-material lifecycle caveats remain residual.
- external-review readiness and public claim boundaries remain residual.

## Completed evidence chain inventory

| Surface | NA / PR / decision | Evidence | Proof type | Direct vs supporting | Claim boundary | Residual | Severity | Next action |
|---|---|---|---|---|---|---|---|---|
| qsc KEM provider RNG seam | NA-0458; D-0901; PR evidence in NA-0458 | `NA-0458_qsl_qsc_kem_provider_rng_failure_fake_test_seam_implementation_harness.md` | cfg/no-cfg qsc tests for `QSC.KEM.KEYPAIR` and `QSC.KEM.ENCAP` | Direct for selected qsc KEM seam | No KEM-complete claim. No provider-RNG-complete claim. | X25519/refimpl/signature/identity deferred | MEDIUM | Carry into binding audit |
| B1 signature provider RNG seam | NA-0461; D-0907 | `NA-0461_qsl_qsc_b1_signature_provider_rng_failure_test_seam_implementation_harness.md` | cfg/no-cfg B1 signing tests, no responder mutation, no B1 output | Direct for B1 signing failure | No signature-complete claim. | A2 timing, identity, refimpl deferred | MEDIUM | Audit signature/transcript binding |
| A2 signature no-output seam | NA-0463; D-0913 | `NA-0463_qsl_qsc_a2_signature_provider_rng_failure_no_output_test_seam_implementation_harness.md` | cfg/no-cfg A2 signing tests, no A2 output after post-mutation failure point | Direct for no-output only | No A2 no-mutation claim. No signature-complete claim. | A2 post-mutation timing remains caveated | MEDIUM | Audit state transition binding |
| Lazy identity provider RNG seam | NA-0465; D-0917 | `NA-0465_qsl_qsc_lazy_identity_provider_rng_failure_test_seam_implementation_harness.md` | cfg/no-cfg lazy identity KEM/signature tests | Direct for lazy identity | No identity-complete claim. | Legacy/CLI/TUI then completed separately; refimpl still residual | MEDIUM | Carry as bounded identity evidence |
| Legacy/public-record provider RNG seam | NA-0467; D-0921 | `NA-0467_qsl_qsc_legacy_identity_public_record_provider_rng_failure_test_seam_implementation_harness.md` | cfg/no-cfg legacy migration/public-record upgrade tests | Direct for selected legacy/public-record paths | No identity-complete claim. | Stale public-record and replay semantics need audit | MEDIUM | Audit stale-record binding |
| CLI identity rotation provider RNG seam | NA-0469; D-0925/D-0927 | `NA-0469_qsl_qsc_cli_identity_rotation_provider_rng_failure_test_seam_implementation_harness.md` | cfg/no-cfg CLI rotation tests with stable selected identity and public record | Direct for CLI rotation | No identity-complete claim. No external-review-complete claim. | active attacker/stale record/recovery playbook residual | MEDIUM | Audit identity binding and SRE recovery |
| TUI bootstrap pre-generation transactionality | NA-0472; D-0932/D-0933; PR #1214/#1215 | `NA-0472_qsl_qsc_tui_account_bootstrap_pre_generation_transactionality_implementation_harness.md` | cfg/no-cfg TUI bootstrap tests; no vault/default/identity state on forced failure | Direct for selected TUI bootstrap path | No identity-complete claim. No side-channel-free claim. | in-memory lifetime caveat | MEDIUM | Carry into secret-material review |
| Route/contact/attachment RNG seam | NA-0452; D-0889 | `NA-0452_qsl_qsc_route_contact_attachment_rng_failure_test_seam_implementation_harness.md` | cfg/no-cfg route/contact/attachment RNG tests | Direct for selected qsc non-provider RNG surfaces | No RNG-failure-complete claim. | provider/refimpl RNG deferred | LOW | Accepted supporting background |
| Key lifecycle / zeroization | NA-0441/NA-0446 inheritance | `NA-0441_qsl_nonce_key_rng_lifecycle_read_only_audit_plan.md`; `key_lifecycle_zeroization` test | read-only audit plus executable redaction/cleanup tests | Supporting for identity/provider chain | No secret-material-complete claim. No side-channel-free claim. | comprehensive memory erasure not proven | HIGH | Future side-channel/secret-material lane |
| Provider-error no-mutation | NA-0436; D-0859 | `NA-0436_qsl_qsc_pq_decap_failed_no_mutation_test_implementation_harness.md` | deterministic qsc provider-error no-mutation test | Direct for `pq_decap_failed` no-mutation | No provider-error-complete claim. | `pq_encap_failed` defensive branch caveat | MEDIUM | Carry into binding audit |
| qsc adversarial evidence | NA-0439; D-0865/D-0866 | `NA-0439_qsl_qsc_provider_error_path_adversarial_coverage_implementation_harness.md`; `scripts/ci/qsc_adversarial.sh` marker | adversarial smoke integrates provider-error test | Supporting CI/adversarial evidence | No vulnerability-free claim. | cargo-fuzz local availability caveat | LOW | Keep in public-safety watch |
| Formal/model evidence | NA-0440 and current formal scripts | `formal/README.md`; `formal/*.py` | bounded SCKA, negotiation, qsc suite-id model checks | Supporting only | No formal-complete claim. No external-review-complete claim. | identity/provider RNG transitions not modeled | MEDIUM | Future formal mapping lane |
| refimpl pqkem768 evidence | NA-0426/NA-0441 inheritance and local validation | `tools/refimpl/quantumshield_refimpl/tests/pqkem768.rs` | provider KEM tests | Supporting provider-boundary evidence | No refimpl provider-RNG-complete claim. | provider RNG failure injection not proven | MEDIUM | Keep as provider-boundary residual |
| Root dependency health | root audit/current local proof | root `cargo audit --deny warnings`; cargo tree probes | RustSec dependency health | Supporting only | Cargo audit green is dependency-health evidence only. | provenance/SBOM/release signing residual | LOW | Supply-chain backlog |
| Nested qsc fuzz lock health | qsc fuzz lock audit/current local proof | nested cargo audit; pqcrypto scan | dependency health for fuzz lock | Supporting only | Cargo audit green is dependency-health evidence only. | cargo-fuzz tool availability caveat | LOW | Keep as CI/dependency watch |
| Public-safety evidence | current main public-safety on `3765cdfc7d05` | `qsl_evidence_helper.py public-safety-status` | GitHub check aggregate | Supporting CI gate | No public-readiness claim. | helper/check-shape caveats remain | LOW | Keep helper hardening backlog |
| Backup/log-code chain boundary | qsl-backup hash/source-list proof | SHA `e9ecff3d22ed`; source-list count 1 | read-only local ops boundary | Supporting local-ops evidence | No backup-complete claim. No restore-proof claim. | off-host/restore/key-custody residuals | MEDIUM | Keep local-ops residuals |

## Hostile cryptographer review

Classification: `CRYPTOGRAPHER_REVIEW_TRANSCRIPT_BINDING_NEXT`.

Review answers:

- A reader might over-infer that identity/provider RNG failure handling is now
  complete across all protocol and provider surfaces. That is not supported.
- qsc/refimpl/provider boundary remains indirect where qsc uses refimpl provider
  helpers and where provider RNG failure cannot be forced without contract
  changes.
- Forced cfg seams may diverge from real provider failure behavior because they
  return deterministic test errors before the actual provider API boundary.
- Transcript binding has supporting qsc/refimpl/formal/vector evidence, but the
  identity/provider RNG chain did not directly audit KEM/signature/transcript
  binding as a whole.
- Signature/KEM/domain-separation labels exist in qsc source, but this lane did
  not review the whole label set against identity/public-record and suite
  binding.
- Downgrade, replay, stale public record, and identity binding are considered
  as residuals, not closed issues.
- Side-channel and constant-time claims are avoided.
- Formal models do not directly map identity/provider RNG failure state
  transitions.

Findings:

| ID | Finding | Severity | Disposition | Evidence |
|---|---|---:|---|---|
| HC-01 | The evidence chain can be over-read as provider-RNG complete even though refimpl, X25519, formal RNG, fuzz/vector RNG, and side-channel residuals remain. | HIGH | immediate successor context | NA-0472 residuals; NA-0450 provider-dependent residual matrix |
| HC-02 | KEM/signature/transcript/identity binding is distributed across qsc source, refimpl, vectors, and formal models but has not been reviewed as one coherent post-RNG-chain surface. | HIGH | immediate successor | qsc `handshake/mod.rs`; `formal/README.md`; NA-0441 transcript-binding row |
| HC-03 | cfg failure seams force errors before selected call sites; they prove local fail-closed behavior but not real provider entropy-failure semantics. | MEDIUM | future lane | NA-0458, NA-0461, NA-0465, NA-0469, NA-0472 seam evidence |
| HC-04 | A2 signature evidence is intentionally no-output-only because forced failure occurs after initiator session storage and pending clear. | MEDIUM | accepted caveat and audit input | NA-0463 post-mutation timing acknowledgment |
| HC-05 | Formal models treat transcript/key context as abstract values and do not prove cryptographic authentication, AEAD security, provider correctness, or identity/provider RNG transitions. | MEDIUM | future lane | `formal/README.md`; `model_suite2_negotiation_bounded.py`; `model_qsc_handshake_suite_id_bounded.py` |
| HC-06 | Side-channel, timing, and memory-erasure completeness remain unproven for provider and identity secret material. | HIGH | future lane | NA-0441 key lifecycle findings; NA-0472 in-memory lifetime caveat |

## Red-team review

Classification: `RED_TEAM_IDENTITY_REPLAY_ROLLBACK_NEXT`.

Review answers:

- An active attacker would probe identity rotation/bootstrap crash windows,
  replay old identity/public-record material, and try conflicting peer pins or
  stale records.
- A relay/server can still observe route/contact/attachment and message timing
  metadata; the identity/provider RNG chain does not remove metadata exposure.
- Stale public records, replay, rollback, and conflicting identity material are
  not closed by the RNG failure seams.
- TUI pre-generation reduces partial durable writes for forced bootstrap
  identity RNG failure, but crash/recovery runbooks remain broader.
- Partial local state confusion is reduced on selected forced failure paths but
  not reviewed across every rollback/recovery path.
- Diagnostic output redaction is tested in selected paths; broader diagnostic
  surfaces remain a release-support residual.
- State rollback/recovery paths need an operator playbook before release claims.
- Test seams are cfg-gated and no-cfg tests prove the selector is ignored in
  normal builds.

Findings:

| ID | Finding | Severity | Disposition | Evidence |
|---|---|---:|---|---|
| RT-01 | Stale public-record and identity replay/rollback behavior should be audited across KEM/signature/transcript binding, not only RNG failure local writes. | HIGH | immediate successor | NA-0467, NA-0469, qsc handshake identity pin source |
| RT-02 | Relay/server-visible state and metadata remain outside the completed identity/provider RNG evidence chain. | MEDIUM | future lane | route/contact/attachment evidence; metadata/governance boundaries |
| RT-03 | Crash after pre-generation but before or during durable commit has bounded TUI proof for forced failure, but no cross-path recovery playbook. | MEDIUM | future lane | NA-0472; NA-0471 rollback/staging rejection rationale |
| RT-04 | Diagnostic redaction is path-specific; broad diagnostic/log review remains incomplete. | MEDIUM | future lane | NA-0441 redaction evidence; NA-0469/NA-0472 sanitized outputs |
| RT-05 | cfg-only seams are safe from normal builds by current no-cfg tests, but future seams need the same no-cfg proof discipline. | LOW | accepted caveat | NA-0458 through NA-0472 no-cfg tests |
| RT-06 | qshield-cli demo-local deterministic material remains a claim-boundary residual and must not be treated as production protocol evidence. | MEDIUM | future lane | NA-0441 qshield demo boundary row |

## Production SRE review

Classification: `SRE_RECOVERY_PLAYBOOK_NEXT`.

Review answers:

- Identity rotation/bootstrap failure is an operator-visible account or identity
  setup/rotation incident. Selected tests prove sanitized failure and no partial
  selected state, but they do not define operator recovery steps.
- After failure, the operator may retry selected commands; broader stale state,
  rollback, backup restore, and peer reconciliation still need a playbook.
- Logs/markers exist for selected failures and should avoid secrets, route
  tokens, passphrases, private keys, stack traces, and provider internals.
- A recovery/rollback runbook is missing for identity conflicts, stale public
  records, and backup/restore interactions.
- Public-safety and CI catch selected regressions but are not exhaustive.
- Release-support boundaries are clear: this is internal governance evidence.
- Production support requires recovery guidance, monitoring/diagnostic review,
  external review package, dependency/provenance, and claim-boundary approval.
- Monitoring/diagnostic surfaces remain unreviewed beyond selected tests.

Findings:

| ID | Finding | Severity | Disposition | Evidence |
|---|---|---:|---|---|
| SRE-01 | Operator recovery after failed identity rotation/bootstrap is not specified as a runbook. | HIGH | future lane | NA-0469 SRE notes; NA-0472 failure output |
| SRE-02 | Public-safety is necessary but not sufficient to detect every identity/provider RNG regression. | MEDIUM | accepted caveat | current public-safety and qsc-adversarial evidence |
| SRE-03 | Diagnostic/logging expectations are path-specific and need a release-support review before public support. | MEDIUM | future lane | redaction tests; sanitized output tests |
| SRE-04 | Backup/restore interactions with identity state remain local-ops residuals. | MEDIUM | future lane | qsl-backup boundary; off-host/restore/key-custody residual list |
| SRE-05 | Dependency health is green but does not prove release provenance, SBOM, or signing. | LOW | backlog | cargo audit/tree evidence |
| SRE-06 | qsl-server/qsl-attachments production observability remains out of scope and must not be inferred from qsc-local evidence. | MEDIUM | accepted caveat | service boundary evidence |

## Side-channel / secret-material review

Classification: `SIDE_CHANNEL_SCOPE_NEXT`.

Required statements:

- No side-channel-free claim.
- No constant-time proof.
- No memory-erasure completeness proof.
- No secret-material-complete claim.
- Current evidence is bounded internal evidence.

Review:

- TUI pre-generation extends in-memory lifetime for generated bootstrap
  identity KEM/signature secrets during durable account/default/identity commit.
- Lazy identity, legacy/public-record, CLI rotation, B1, A2, KEM, and TUI paths
  include selected no-secret-output and no-partial-state checks, but those are
  not comprehensive memory-erasure or side-channel proofs.
- Key lifecycle tests cover selected pending-secret cleanup, encrypted-at-rest
  boundaries, output redaction sentinels, and reject no-mutation boundaries.
- qsc/refimpl provider internals, third-party constant-time behavior, CPU/cache
  leakage, timing leakage, allocator behavior, swap/core-dump behavior, and
  compiler optimization effects are not proven.
- No plaintext persistence evidence is bounded to selected files/vaults; it is
  not a comprehensive storage proof for every secret.
- Redaction evidence is strong for selected outputs and markers but not a full
  diagnostic/logging assurance package.

Findings:

| ID | Finding | Severity | Disposition | Evidence |
|---|---|---:|---|---|
| SC-01 | Pre-generation improves failure ordering but increases in-memory secret lifetime. | HIGH | future lane | NA-0472 in-memory lifetime caveat |
| SC-02 | Zeroization evidence is targeted and does not prove memory-erasure completeness. | HIGH | future lane | `key_lifecycle_zeroization`; NA-0441 |
| SC-03 | Provider and dependency constant-time behavior is not established. | HIGH | accepted caveat/future lane | NA-0426/NA-0441 side-channel caveats |
| SC-04 | Logging/redaction tests are selected-path evidence only. | MEDIUM | future lane | NA-0465/NA-0469/NA-0472 output assertions |
| SC-05 | qshield-cli demo-local persisted establishment material remains a claim-boundary caveat. | MEDIUM | backlog | NA-0441 qshield row |

## Formal-model mapping review

Classification: `FORMAL_MODEL_MAPPING_RESIDUAL_ACTIVE`.

Current models cover:

- SCKA control-plane ADV monotonicity, one-time ciphertext targeting,
  tombstones, reject no-mutation, and transactional commit abstractions.
- Suite-2 downgrade/capability/suite commitment reject and no-mutation
  behavior.
- Future qsc handshake suite-id canonicality, compatibility gate,
  transcript/key-context binding as abstract context values, no-output,
  no-leak, no-downgrade, and reject no-mutation behavior.

Current models do not cover:

- identity/provider RNG failure state transitions;
- actual provider entropy health;
- KEM/signature provider implementation behavior;
- secret material lifetime, zeroization, or side-channel behavior;
- qsc TUI bootstrap pre-generation durable write ordering;
- CLI identity rotation/public-record replay/rollback as a formal model;
- fuzz/vector RNG failure behavior.

Future formal model work is useful, but the immediate next best lane is the
read-only KEM/signature/transcript binding audit because it can define the
exact state and binding surface that any future model should represent.

## External-review readiness review

Classification: `EXTERNAL_REVIEW_READINESS_INCREMENTAL`.

Checklist:

| Item | Status | Evidence / caveat |
|---|---|---|
| protocol spec present | partial/incremental | canonical docs exist, but this lane did not package them |
| threat model present | partial/incremental | project charter and metadata/privacy docs exist |
| state-machine mapping present | partial/incremental | TRACEABILITY and formal models cover selected slices |
| test vectors present | partial/incremental | Suite-2 and qsc suite-id vectors exist |
| negative vectors present | partial/incremental | downgrade/replay/no-mutation vectors exist |
| formal model mapping present | incomplete | identity/provider RNG transitions are not directly modeled |
| claim boundaries present | strong internal boundary | repeated no-public-overclaim decisions |
| dependency/SBOM/provenance evidence present | incomplete | cargo audit/tree only; no SBOM/release-signing package |
| side-channel caveat present | present caveat | no side-channel-free claim |
| external review package draft present | absent | no package was created in NA-0473 |

External review is not ready as a completed package. The evidence base is
incrementally improving, and a future external-review package lane becomes more
valuable after the KEM/signature/transcript binding audit and side-channel /
secret-material scope review reduce core ambiguity.

## Release-claim boundary review

Required same-line denial wording:

- No public-readiness claim.
- No production-readiness claim.
- No public-internet-readiness claim.
- No crypto-complete claim.
- No KEM-complete claim.
- No signature-complete claim.
- No identity-complete claim.
- No RNG-failure-complete claim.
- No provider-RNG-complete claim.
- No secret-material-complete claim.
- No side-channel-free claim.
- No vulnerability-free claim.
- No bug-free claim.
- No perfect-crypto claim.
- No metadata-free claim.
- No anonymity claim.
- No untraceability claim.
- No external-review-complete claim.
- No backup-complete claim.
- No restore-proof claim.
- Cargo audit green is dependency-health evidence only.

Release-claim matrix:

| Claim class | Allowed internal claim | Forbidden public claim | Conditions before expansion |
|---|---|---|---|
| Identity/provider RNG | bounded qsc path-specific evidence | No identity-complete claim. No provider-RNG-complete claim. | direct refimpl/X25519/formal/fuzz/vector/recovery review |
| KEM/signature/transcript | supporting and bounded implementation evidence | No KEM-complete claim. No signature-complete claim. | read-only binding audit, then exact follow-up tests/models if needed |
| Side-channel/secret material | caveats and selected cleanup/redaction tests | No side-channel-free claim. No secret-material-complete claim. | side-channel and secret-lifetime scope plan and evidence |
| External review | incremental readiness evidence | No external-review-complete claim. | package, scope, reviewer-ready artifacts, claim matrix |
| Dependency health | current RustSec and tree evidence | No vulnerability-free claim. No perfect-crypto claim. | SBOM/provenance/release signing and broader security review |
| Metadata/privacy | selected metadata minimization lanes only | No metadata-free claim. No anonymity claim. No untraceability claim. | service/transport/privacy evidence and public wording approval |
| Backup/restore | same-host local ops boundary evidence | No backup-complete claim. No restore-proof claim. | off-host target, restore drill, key custody, operator authorization |

## Dependency / supply-chain / CI assurance review

Classification: `DEPENDENCY_HEALTH_ACCEPTED_SUPPORTING_ONLY`.

Review:

- Root `cargo audit --deny warnings` passed for the current root lockfile.
- Nested qsc fuzz lock `cargo audit --deny warnings --file
  qsl/qsl-client/qsc/fuzz/Cargo.lock` passed.
- `rustls-webpki v0.103.13` remains present through rustls/reqwest/ureq paths.
- `ml-kem v0.2.1` remains present through `quantumshield_refimpl` and related
  workspace users.
- Root pqcrypto inverse probes report expected package-ID absence for
  `pqcrypto-mlkem`, `pqcrypto-traits`, and `pqcrypto-internals`.
- Current main public-safety and qsc-adversarial-smoke are green.
- qsc adversarial script includes the provider-error no-mutation test marker.
- Local cargo-fuzz availability remains an environment caveat; PR CI remains
  the authoritative qsc-adversarial-smoke proof.
- No SBOM, provenance, release-signing, reproducible-build, or artifact
  attestation proof is created by this lane.

qsc/refimpl/provider boundary separation:

- qsc-local cfg seams prove selected qsc error ordering and no-mutation/no-output
  behavior.
- refimpl `pqkem768` tests prove provider-level behavior for selected inputs.
- qsc evidence does not prove refimpl provider RNG failure behavior.
- refimpl evidence does not prove qsc durable-state no-mutation unless qsc
  tests exercise the boundary.
- Future provider-boundary work must avoid weakening fail-closed behavior or
  changing provider contracts without exact authorization.

## Assurance gap matrix

| ID | Domain | Finding | Evidence | Severity | Likelihood | Impact | Public-claim risk | Implementation risk | Recommended action | Candidate successor | Goals | Disposition |
|---|---|---|---|---:|---:|---:|---|---|---|---|---|---|
| F-0473-01 | Crypto binding | KEM/signature/transcript/identity binding has not been reviewed as one whole surface after the RNG chain. | qsc handshake source; formal README; NA-0441 transcript row | HIGH | Medium | High | High | Low for read-only audit | P1 next | KEM / Signature / Transcript Binding Read-Only Audit | G1,G2,G3,G4,G5 | immediate successor |
| F-0473-02 | Side-channel / secrets | Secret-material lifetime and side-channel caveats remain active. | NA-0441; NA-0472 | HIGH | Medium | High | High | Medium if implementation follows | P1 next/backlog | Side-Channel / Secret-Material Scope | G1,G2,G4 | future lane |
| F-0473-03 | Formal mapping | Identity/provider RNG transitions are not directly modeled. | formal README/scripts | MEDIUM | High | Medium | Medium | Medium | P2 backlog | Formal Model Mapping Authorization | G2,G4 | future lane |
| F-0473-04 | External review | Evidence is incremental but no external review package exists. | checklist in this doc | MEDIUM | High | Medium | High | Low | P2 backlog | External Review Readiness Package | G4,G5 | future lane |
| F-0473-05 | Release claims | Claim boundaries are explicit but need a consolidated public assurance matrix before public expansion. | D-0932/D-0933; this review | MEDIUM | Medium | High | High | Low | P2 backlog | Release Claim Boundary Matrix | G4,G5 | future lane |
| F-0473-06 | qsc/refimpl provider boundary | qsc seams are local and refimpl provider RNG failure remains indirect. | NA-0450/NA-0453/NA-0458 | MEDIUM | Medium | Medium | Medium | High | P2 backlog | refimpl Provider RNG Boundary Scope | G1,G2,G4 | future lane |
| F-0473-07 | X25519 / ephemeral | X25519 / ephemeral provider RNG failure remains backlog. | NA-0450; NA-0458; NA-0472 | MEDIUM | Medium | Medium | Medium | Medium | P2 backlog | X25519 / Ephemeral Scope | G1,G2,G4 | future lane |
| F-0473-08 | Recovery / rollback / SRE | Identity failure recovery playbooks and stale-record rollback guidance are incomplete. | NA-0469/NA-0472 SRE review | HIGH | Medium | High | High | Low | P2 backlog | SRE Recovery Playbook | G2,G4,G5 | future lane |
| F-0473-09 | Supply chain / provenance | cargo audit is green but no SBOM/provenance/release-signing evidence exists. | cargo audit/tree proof | LOW | Medium | Medium | Medium | Low | P2 backlog | Supply-Chain Provenance | G4 | future lane |
| F-0473-10 | CI helper | public-safety and qsc-adversarial evidence are green but helper/check-shape caveats remain. | public-safety status; qsc adversarial script | LOW | Medium | Medium | Medium | Low | P2 backlog | CI Evidence Helper Hardening | G4 | future lane |
| F-0473-11 | A2 timing | A2 provider RNG evidence is no-output-only because failure occurs after selected initiator state mutation. | NA-0463 | MEDIUM | Medium | Medium | Medium | Medium | P1 audit input | KEM / Signature / Transcript Binding Read-Only Audit | G1,G2,G3,G4 | accepted caveat for successor |
| F-0473-12 | Backup / restore / custody | Backup/log-code chain is unchanged, but off-host backup, restore, and key custody remain residuals. | qsl-backup SHA/source-list; local ops canon | MEDIUM | Medium | High | High | High | P2 backlog | Local Ops Backup/Restore/Key Custody | G4,G5 | future lane |

Ranking:

- P0 immediate: none selected inside NA-0473 scope.
- P1 next: F-0473-01 and F-0473-11 via the selected read-only binding audit.
- P2 backlog: F-0473-02 through F-0473-10 and F-0473-12.
- Accepted caveat: cargo audit is dependency-health evidence only; formal
  checks are bounded supporting evidence only; current qsc seams are path
  specific.

## Prioritization

The KEM/signature/transcript binding audit is selected first because it has the
highest combined cryptographic value, public-claim risk reduction, and low
implementation risk. It is read-only, directly consumes the identity/provider
RNG chain, and can determine whether any future implementation, vector, fuzz,
formal, side-channel, or recovery lane needs exact scope.

Side-channel/secret-material work is high severity, but it is more likely to
require broader scope or implementation follow-up after the binding audit
clarifies secret/state transitions. External review packaging is premature
until core binding and caveat mapping are cleaner.

## Successor selection

Selected successor:

`NA-0474 -- QSL KEM / Signature / Transcript Binding Read-Only Audit Plan`

Rationale:

- highest-value next cryptographic assurance gap;
- read-only and compatible with current governance-only momentum;
- consumes KEM, B1, A2, identity, provider-error, formal, vector, and qsc/refimpl
  evidence without mutating runtime or crypto code;
- directly addresses hostile cryptographer and red-team concerns around
  downgrade, replay, stale public record, identity binding, and transcript
  binding;
- creates clearer prerequisites for future formal, side-channel,
  external-review, and public-claim lanes.

## Rejected alternatives

- Side-Channel / Secret-Material Assurance Scope Authorization Plan: rejected
  as immediate successor because it remains high value but needs binding/state
  context first.
- Identity / Provider RNG Formal Model Mapping Authorization Plan: rejected as
  immediate successor because current models need exact binding/state mapping
  before a useful formal scope can be frozen.
- External Review Readiness Package Authorization Plan: rejected as immediate
  successor because package readiness is incremental, not ready.
- Release Claim Boundary / Public Assurance Matrix Plan: rejected as immediate
  successor because current denial boundaries are clear enough for this lane.
- refimpl Provider RNG Failure Boundary Scope Authorization Plan: rejected as
  immediate successor because it is narrower than the binding audit.
- X25519 / Ephemeral Provider RNG Failure Scope Authorization Plan: rejected as
  immediate successor because it is an implementation-residual lane and does
  not outrank the current assurance gap.
- Supply-Chain Provenance / Release Integrity Scope Authorization Plan:
  rejected as immediate successor because dependency health is green and no
  urgent supply-chain blocker is proven.
- CI Evidence Helper Hardening Plan: rejected as immediate successor because
  recurring helper caveats do not outrank cryptographic binding review.
- Identity / Provider RNG Assurance Residual Triage Plan: rejected because the
  matrix is clear enough to select a specific successor.

## Public claim boundary

This review is internal governance evidence only.

No public-readiness claim. No production-readiness claim. No
public-internet-readiness claim. No crypto-complete claim. No KEM-complete
claim. No signature-complete claim. No identity-complete claim. No
RNG-failure-complete claim. No provider-RNG-complete claim. No
secret-material-complete claim. No side-channel-free claim. No
vulnerability-free claim. No bug-free claim. No perfect-crypto claim. No
metadata-free claim. No anonymity claim. No untraceability claim. No
external-review-complete claim. No backup-complete claim. No restore-proof
claim. Cargo audit green is dependency-health evidence only.

## Backup-impact statement

NA-0473 changes only qsl-protocol governance evidence, a governance testplan,
DECISIONS, TRACEABILITY, and the rolling journal under the qwork repository
workspace. It does not run backup or restore. It does not mutate qsl-backup,
backup status, backup plan, backup source lists, backup logs, backup manifests,
rollback subtree paths, `/backup/qsl`, timers, fstab, systemd, off-host target,
restore target, key custody material, or recovery artifacts.

qsl-backup boundary proof remains read-only: SHA
`e9ecff3d22ed` prefix matches the expected script hash, and the codex ops
source-list entry remains present exactly once in the script source list.

## Next recommendation

Merge the NA-0473 evidence PR only after local validation, required PR checks,
and public-safety pass. If post-merge public-safety is green, close out NA-0473
and restore:

`NA-0474 -- QSL KEM / Signature / Transcript Binding Read-Only Audit Plan`

The NA-0474 lane must remain read-only unless a later exact directive
authorizes implementation. It must preserve no-runtime/no-crypto/no-dependency/
no-workflow/no-public-overclaim boundaries.
