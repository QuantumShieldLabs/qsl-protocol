Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0427 QSL Crypto API / Provider Boundary Findings Triage and Remediation Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0427 consumed the NA-0426 findings matrix F-0426-01 through F-0426-09
without changing runtime code, crypto code, dependency manifests, lockfiles,
tests, vectors, workflows, service repositories, public surfaces, local backup
state, qwork tooling, or qsl-backup.

The highest-priority finding is F-0426-04. The separate qsc cargo-fuzz
workspace is an active committed fuzz workspace, and
`scripts/ci/qsc_adversarial.sh` runs its targets through `cargo fuzz`. Its
separate `qsl/qsl-client/qsc/fuzz/Cargo.lock` still records pqcrypto packages,
and a read-only nested lock audit reports denied RustSec findings for that
lock, including stale `rustls-webpki` and pqcrypto package advisories. Root
workspace dependency health remains green, but the nested fuzz lock cannot be
treated as a harmless archived residue.

Selected successor:

`NA-0428 -- QSL qsc Fuzz Lock pqcrypto Residual Dependency Blocker Authorization Plan`

NA-0428 must remain an exact-scope governance authorization lane unless and
until a later directive authorizes lockfile, manifest, dependency, test, vector,
runtime, crypto, or workflow mutation.

## Live NA-0427 scope

Live `NEXT_ACTIONS.md` lists:

`NA-0427 -- QSL Crypto API / Provider Boundary Findings Triage and Remediation Authorization Plan`

Status: READY.

Goals: G1, G2, G3, G4, G5.

Allowed mutation paths for the NA-0427 evidence PR:

- `docs/governance/evidence/NA-0427_qsl_crypto_api_provider_boundary_findings_triage_remediation_authorization_plan.md`
- `tests/NA-0427_qsl_crypto_api_provider_boundary_findings_triage_remediation_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Allowed read-only inspection covered qwork proof files, the live queue,
DECISIONS, TRACEABILITY, rolling journal, NA-0426 evidence/testplans,
stewardship canon, provider code/tests, qsc provider-error call paths, the qsc
cargo-fuzz workspace, dependency trees, formal roots, and input/vector roots.

Forbidden mutation scope:

- no runtime code mutation;
- no crypto implementation mutation;
- no dependency, Cargo manifest, or lockfile mutation;
- no workflow mutation;
- no qsl-server or qsl-attachments mutation;
- no qshield runtime mutation;
- no website, public docs, README, or START_HERE mutation;
- no tests or vectors mutation outside this governance testplan;
- no qwork, qstart, qresume, or qshell execution or mutation by Codex;
- no backup execution;
- no restore execution;
- no qsl-backup mutation;
- no backup status or backup plan mutation;
- no rollback subtree or `/backup/qsl` mutation;
- no public technical paper content;
- no secret material handling;
- no unsupported public assurance claim.

Acceptance criteria:

- every NA-0426 finding is consumed;
- remediation or follow-up need is classified;
- public-claim caveats are explicit;
- no runtime, crypto, dependency, Cargo, workflow, service, public, backup, or
  local-ops mutation occurs;
- root cargo audit remains green;
- public-safety is green before merge and after merge;
- exactly one READY item remains.

Stop conditions preserved:

- qwork proof files missing, malformed, stale, or inconsistent;
- qwork, qstart, or qresume run by Codex;
- PR #1122 not merged at the expected merge commit lineage;
- `origin/main` not equal to or descended from `d3daaad926c6`;
- queue not READY NA-0427 at start;
- D-0841 absent or D-0842 already present at start;
- root cargo audit not green;
- NA-0426 findings cannot be found or safely consumed;
- qsl-backup source-list regression;
- backup or restore execution by Codex;
- forbidden runtime, crypto, dependency, Cargo, workflow, public, service,
  qwork, backup, restore, qsl-backup, status/plan, rollback, README,
  START_HERE, or website mutation;
- public-safety red or missing;
- more than one READY item;
- unsupported public assurance claim introduced.

## qwork proof-file verification

Codex read, but did not run, qwork proof files:

- `/srv/qbuild/work/NA-0427/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0427/.qwork/startup.qsl-protocol.json`

The `.kv` proof reported the required values:

- `startup_result=OK`
- `lane=NA-0427`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0427/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0427`
- `requested_lane_status=READY`

The JSON proof parsed successfully and mirrored the required `.kv` fields for
lane, repo, path, head, origin/main, clean state, READY count, queue-top READY,
and requested-lane status.

After `git fetch --all --prune`, live `HEAD` and `origin/main` still matched
the qwork proof at `d3daaad926c6`. PR #1122 was verified MERGED with merge
commit `d3daaad926c6`. The `public-safety` check on the current main commit was
present and completed success.

Proof root:

`/srv/qbuild/tmp/NA0427_provider_boundary_findings_triage_20260605T160131-0500`

The qwork proof files were copied into the proof root under `qwork/`.

## NA-0426 inheritance

NA-0426 completed the bounded read-only crypto API / provider boundary audit
after NA-0418/D257 replaced the runtime-reachable pqcrypto ML-KEM provider with
RustCrypto `ml-kem`. NA-0426 found no BLOCKER or HIGH runtime issue and
recorded F-0426-01 through F-0426-09 for triage before any remediation or
follow-up lane is authorized.

Inherited findings:

- F-0426-01: Provider boundary preserved after ml-kem remediation, INFO.
- F-0426-02: Provider-level wrong-length rejects covered; qsc provider-error
  paths need proof, MEDIUM / EVIDENCE_INCOMPLETE.
- F-0426-03: Historical `pqcrypto` feature name maps to ml-kem/ml-dsa, LOW /
  CLAIM_BOUNDARY.
- F-0426-04: Nested qsc fuzz lock still contains pqcrypto packages, MEDIUM /
  EVIDENCE_INCOMPLETE.
- F-0426-05: Formal/vector evidence does not directly prove provider
  implementation alignment, MEDIUM / EVIDENCE_INCOMPLETE.
- F-0426-06: Property/fuzz/differential coverage incomplete, MEDIUM /
  EVIDENCE_INCOMPLETE.
- F-0426-07: Raw KEM secret material crosses boundary as `Vec<u8>`, MEDIUM /
  EVIDENCE_INCOMPLETE.
- F-0426-08: Side-channel/timing status not established, CLAIM_BOUNDARY /
  EVIDENCE_INCOMPLETE.
- F-0426-09: no service/demo/public-readiness claim; boundaries remain
  separate, CLAIM_BOUNDARY.

Inherited dependency state:

- root `cargo audit --deny warnings` is green;
- root `rustls-webpki` remains `v0.103.13`;
- root `ml-kem v0.2.1` remains the active KEM provider dependency through
  `quantumshield_refimpl`;
- root `pqcrypto-mlkem`, `pqcrypto-traits`, and `pqcrypto-internals` package
  IDs are absent from the locked graph;
- the separate qsc fuzz lock still records pqcrypto packages and stale
  dependency state.

## Stewardship template application

### Crypto / Protocol Steward

Review question: Do the inherited provider-boundary findings require immediate
runtime or crypto remediation, or can they be sequenced as evidence and
authorization lanes?

Evidence reviewed: NA-0426 evidence; `PqKem768` trait; `StdCrypto` ml-kem
implementation; provider tests; qsc handshake encap/decap call paths; qsc
handshake reject/no-mutation tests; formal roots; input/vector roots.

Findings: provider-level wrong-length KEM rejects are covered, and qsc emits
sanitized `pq_encap_failed` / `pq_decap_failed` markers when provider calls
fail. No qsc test directly asserts those two provider-error marker paths or
state no-mutation at that exact boundary. Formal/vector evidence remains
state-machine and category evidence, not direct provider implementation proof.
Raw KEM shared-secret and key material still cross provider boundaries as
`Vec<u8>`.

Risk classification: MEDIUM / EVIDENCE_INCOMPLETE for F-0426-02, F-0426-05,
F-0426-06, and F-0426-07. CLAIM_BOUNDARY for F-0426-08.

Public-claim impact: no crypto-complete claim, no side-channel-free claim, no
external-review-complete claim, no vulnerability-free claim, no bug-free claim,
and no perfect-crypto claim is supported.

Scope impact: no runtime, crypto, test, vector, or formal mutation is
authorized by NA-0427.

Recommended action: preserve qsc provider-error/no-mutation proof as a high
next candidate after the dependency blocker authorization lane.

### CI / Dependency / Release Health Steward

Review question: Is F-0426-04 a stale archival residue, a dev-only cleanup
candidate, or an active blocker candidate?

Evidence reviewed: root cargo audit; root inverse dependency trees; qsc fuzz
manifest; qsc fuzz lock; `scripts/ci/qsc_adversarial.sh`;
`.github/workflows/qsc-adversarial.yml`; nested lock audit; pqcrypto search
results.

Findings: root dependency health is green and the root locked graph does not
contain the pqcrypto package IDs. The qsc fuzz directory is a separate
`cargo-fuzz` workspace with committed targets, a committed lockfile, and an
active CI smoke path for non-docs scopes. A nested lock audit reports denied
findings against the separate fuzz lock. The nested lock therefore is not
treated as resolved by root cargo audit.

Risk classification: BLOCKER_CANDIDATE for queue selection; dependency/fuzz
lock classification `FUZZ_LOCK_ACTIVE_SECURITY_BLOCKER`.

Public-claim impact: root cargo audit green is dependency-health evidence
only. It is not vulnerability-free proof, bug-free proof, or perfect-crypto
proof. The nested fuzz lock red state prevents using fuzz evidence as clean
dependency-health support.

Scope impact: no Cargo, dependency, workflow, or fuzz-lock mutation is
authorized by NA-0427.

Recommended action: select NA-0428 as a qsc fuzz-lock pqcrypto residual
dependency blocker authorization lane.

### Public Claims / External Review Steward

Review question: Does triaging provider-boundary findings create any public
security, readiness, or external-review claim?

Evidence reviewed: NA-0426 public-claim caveats; stewardship canon; GOALS;
PROJECT_CHARTER; TRACEABILITY; findings F-0426-01 through F-0426-09.

Findings: NA-0427 is internal governance evidence only. It does not create
public docs, website content, README text, START_HERE text, a public technical
paper, no external-review package, no production-readiness statement, and no
cryptographic completeness statement.

Risk classification: CLAIM_BOUNDARY.

Public-claim impact: no public assurance category is supported by this lane.

Scope impact: no public-surface mutation is allowed.

Recommended action: keep the future NA-0428 block internal, exact-scope, and
claim-conservative.

### Product / Demo / Service Boundary Steward

Review question: Do provider-boundary findings change qsl-server,
qsl-attachments, qshield runtime, demo, or service-readiness state?

Evidence reviewed: NA-0426 service/demo boundary notes; qsc/refimpl provider
evidence; qsl-server/qsl-attachments scope prohibitions; TRACEABILITY.

Findings: qsl-server and qsl-attachments are not mutated or audited by this
lane. qsc fuzz-lock dependency health is a qsl-protocol CI/dependency concern,
not service-readiness proof. Demo and refimpl evidence remains separated from
public deployment claims.

Risk classification: CLAIM_BOUNDARY.

Public-claim impact: no production-readiness claim, no public-internet
readiness claim, and no demo-as-production claim is supported.

Scope impact: no sibling repo or service-runtime mutation is authorized.

Recommended action: preserve service/demo caveats in NA-0428.

### Local Ops / Backup / Restore Steward

Review question: Were qwork proof files, qsl-backup, backup state, and local
ops boundaries preserved?

Evidence reviewed: qwork proof files; live git state; qsl-backup checksum;
source inclusion count; prior response file; disk watermark; rolling journal.

Findings: qwork proof files were read and copied to the proof root. Codex did
not run qwork, qstart, or qresume. qsl-backup checksum matched the
directive-required value, and the Codex ops source inclusion count was exactly
one. Codex did not run backup or restore and did not mutate backup status,
backup plan, qsl-backup, rollback subtree paths, or `/backup/qsl`.

Risk classification: INFO.

Public-claim impact: no backup-complete claim, no off-host-backup-complete
claim, no disaster-recovery-complete claim, and no restore-proven claim is
made.

Scope impact: no local mutable path changed except the directive-required
temporary proof root.

Recommended action: continue no-backup/no-restore discipline.

## Findings matrix consumption

| ID | Original severity | Current triage classification | Immediate remediation needed | Future lane needed | Grouping | Steward domain | Public-claim impact | Exact recommended action |
|---|---|---|---|---|---|---|---|---|
| F-0426-01 | INFO | ACCEPTED_NO_ACTION | No | No | Baseline for all follow-up | Crypto / Protocol | No crypto-complete or external-review-complete claim | Accept as current provider-boundary baseline after ml-kem remediation. |
| F-0426-02 | MEDIUM / EVIDENCE_INCOMPLETE | NEXT_CANDIDATE | No runtime/test change in NA-0427 | Yes, after the fuzz-lock blocker authorization lane | Related to F-0426-05 and F-0426-06 | Crypto / Protocol | No fail-closed-complete claim | Plan a qsc provider-error path / no-mutation read-only audit after dependency blocker authorization. |
| F-0426-03 | LOW / CLAIM_BOUNDARY | CLAIM_BOUNDARY_ONLY | No | Backlog candidate | Related to F-0426-04 by dependency-family naming | CI / Dependency / Public Claims | Avoid implying pqcrypto provider is active in root graph | Keep the historical feature-name caveat visible; consider narrow naming/docs clarification only after dependency blocker handling. |
| F-0426-04 | MEDIUM / EVIDENCE_INCOMPLETE | BLOCKER_CANDIDATE | Yes, but not within NA-0427 scope | Yes, selected NA-0428 | Related to F-0426-06 and partially F-0426-03 | CI / Dependency / Release Health | Root audit green is not fuzz-lock clean proof | Select NA-0428 as qsc fuzz-lock pqcrypto residual dependency blocker authorization. |
| F-0426-05 | MEDIUM / EVIDENCE_INCOMPLETE | EVIDENCE_GAP | No | Yes | Related to F-0426-02 and F-0426-06 | Crypto / Protocol | No formally-proven implementation claim | Defer formal/vector provider alignment until the dependency blocker is sequenced. |
| F-0426-06 | MEDIUM / EVIDENCE_INCOMPLETE | EVIDENCE_GAP | No direct test/fuzz change in NA-0427 | Yes | Group partially with F-0426-04 because fuzz evidence depends on clean fuzz lock | Crypto / Protocol and CI / Dependency | No exhaustive-testing, bug-free, or vulnerability-free claim | Treat property/fuzz/differential expansion as blocked behind fuzz-lock dependency authorization. |
| F-0426-07 | MEDIUM / EVIDENCE_INCOMPLETE | BACKLOG_CANDIDATE | No | Yes | Separate key-material lifecycle lane | Crypto / Protocol | No secret-handling-complete or memory-hygiene-complete claim | Plan a future key-material lifecycle / zeroization review after provider-error and dependency blockers. |
| F-0426-08 | CLAIM_BOUNDARY / EVIDENCE_INCOMPLETE | CLAIM_BOUNDARY_ONLY | No | Yes, only if a future side-channel audit is authorized | Related to F-0426-07 but not the same lane | Crypto / Protocol and Public Claims | No side-channel-free or constant-time-guaranteed claim | Preserve caveat; do not expand public claims. |
| F-0426-09 | CLAIM_BOUNDARY | CLAIM_BOUNDARY_ONLY | No | No immediate lane | Service/demo caveat only | Product / Demo / Service and Public Claims | No production-readiness or public-internet-readiness claim | Preserve separation of qsl-protocol evidence from service/demo/public readiness. |

## Dependency / nested fuzz lock triage

Focus finding: F-0426-04.

Read-only evidence:

- `qsl/qsl-client/qsc/fuzz/Cargo.toml` is a separate package named
  `qsc-fuzz` with `[workspace]`, `cargo-fuzz = true`, and three fuzz targets.
- `qsl/qsl-client/qsc/fuzz/Cargo.lock` still records
  `pqcrypto-mlkem`, `pqcrypto-traits`, and `pqcrypto-internals`.
- The same nested lock records `rustls-webpki v0.103.10`.
- Root `Cargo.lock` records `rustls-webpki v0.103.13`.
- Root `cargo audit --deny warnings` is green.
- Root inverse trees for pqcrypto package IDs report package-ID absence.
- `scripts/ci/qsc_adversarial.sh` runs `cargo +nightly fuzz run` for all three
  fuzz targets from the qsc fuzz workspace.
- `.github/workflows/qsc-adversarial.yml` runs the qsc adversarial smoke job for
  non-docs scopes.
- A nested lock audit with `cargo audit --deny warnings --file
  qsl/qsl-client/qsc/fuzz/Cargo.lock` reported denied findings for that
  separate lock.

Classification:

`FUZZ_LOCK_ACTIVE_SECURITY_BLOCKER`

Rationale:

- The fuzz lock is committed and paired with active fuzz targets.
- The qsc adversarial smoke script actively invokes the fuzz workspace.
- The nested lock audit is red even though the root workspace audit is green.
- The nested lock still holds the pqcrypto packages that NA-0418 removed from
  the root locked graph.
- Because NA-0427 does not authorize Cargo, dependency, workflow, test, or
  fuzz-lock mutation, the correct action is a blocker authorization successor,
  not an in-lane remediation.

Recovered non-zero evidence:

- Failing command: `cargo tree -i pqcrypto-mlkem --locked` and equivalent
  probes for `pqcrypto-traits` and `pqcrypto-internals`.
  Classification: recoverable zero-match dependency proof because package-ID
  absence is the required root locked-graph outcome.
  Corrective action: recorded the package-ID absence and used root cargo audit
  as the authoritative root dependency-health gate.
  Final result: root pqcrypto package IDs are absent.
- Failing command: `cargo audit --deny warnings --file
  qsl/qsl-client/qsc/fuzz/Cargo.lock`.
  Classification: recoverable triage signal because the directive requested
  this nested audit under a non-stopping `|| true` shape when supported.
  Corrective action: treated the nonzero result as F-0426-04 evidence, not as
  root cargo-audit failure.
  Final result: F-0426-04 is classified `FUZZ_LOCK_ACTIVE_SECURITY_BLOCKER`.
- Failing command: `rg -n "pq_encap_failed|pq_decap_failed" qsl/qsl-client/qsc/tests tools/refimpl/quantumshield_refimpl/tests`.
  Classification: recoverable zero-match discovery proof because absence of
  tests for those exact markers is the expected evidence-gap signal.
  Corrective action: recorded the zero-match as F-0426-02 evidence.
  Final result: F-0426-02 remains a qsc provider-error/no-mutation evidence gap.

## Provider error path / no-mutation triage

Focus finding: F-0426-02.

Evidence reviewed:

- `tools/refimpl/quantumshield_refimpl/tests/pqkem768.rs`
- unit tests in `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`
- qsc handshake provider calls in `qsl/qsl-client/qsc/src/handshake/mod.rs`
- qsc handshake tests for suite-id, replay, activation, and no-mutation reject
  properties

Findings:

- Provider-level tests cover KEM roundtrip, tamper behavior, and wrong-length
  public key, secret key, and ciphertext rejection.
- qsc emits sanitized provider-error markers:
  - `pq_encap_failed` on responder encap failure;
  - `pq_decap_failed` on initiator decap failure.
- Existing qsc tests assert many fail-closed/no-mutation reject paths, including
  suite activation, suite-id parsing, replay, and malformed handshake frames.
- No qsc test was found that directly forces the provider encap or decap error
  markers and asserts no mutation at that exact boundary.

Triage result:

F-0426-02 remains `NEXT_CANDIDATE` and `EVIDENCE_GAP`, but it does not outrank
the active nested fuzz-lock blocker.

Recommended future action:

After the NA-0428 dependency blocker authorization lane, plan
`QSL qsc Provider Error Path / No-Mutation Read-Only Audit Plan` or equivalent
exact scope.

## Feature-name / claim boundary triage

Focus finding: F-0426-03.

Evidence reviewed:

- `qsl/qsl-client/qsc/Cargo.toml`
- `tools/refimpl/quantumshield_refimpl/Cargo.toml`
- root and nested dependency search results

Findings:

- qsc depends on `quantumshield_refimpl` with `features = ["pqcrypto"]`.
- In the current root provider, `quantumshield_refimpl` feature `pqcrypto`
  maps to `pqkem` plus `ml-dsa`, and `pqkem` maps to RustCrypto `ml-kem`.
- The historical feature name can confuse reviewers because root pqcrypto KEM
  crates are absent while the feature label remains.
- The nested fuzz lock still records actual pqcrypto packages, so naming
  clarification should not be conflated with dependency cleanup.

Triage result:

F-0426-03 is `CLAIM_BOUNDARY_ONLY` and a backlog candidate. It should be folded
into future dependency-boundary documentation or a narrow feature-name
clarification lane after the fuzz-lock blocker is authorized.

## Formal / vector / property / fuzz triage

Focus findings: F-0426-05 and F-0426-06.

Evidence reviewed:

- `formal/README.md`
- `formal/model_qsc_handshake_suite_id_bounded.py`
- `formal/run_model_checks.py`
- `inputs/suite2/vectors/qshield_suite2_scka_kem_vectors_v1.json`
- `inputs/suite2/vectors/qshield_suite2_boundary_vectors_v1.json`
- qsc fuzz targets under `qsl/qsl-client/qsc/fuzz/fuzz_targets/`
- qsc property tests and adversarial tests
- provider tests under `tools/refimpl/quantumshield_refimpl/tests/`

Findings:

- Formal models remain bounded and state-machine focused. They do not prove KEM
  provider byte conversions, provider failure classes, side-channel behavior,
  or provider implementation equivalence.
- Suite-2 vector roots include ML-KEM-768 categories and invalid-size
  expectations, but NA-0427 did not find direct proof that active provider
  outputs are mapped through KAT or differential fixtures.
- qsc fuzz targets cover route parsing, payload boundaries, and vault envelope
  parsing, not direct `PqKem768` provider behavior.
- Fuzz coverage cannot be used as clean dependency-health evidence while the
  separate fuzz lock is red.

Triage result:

F-0426-05 and F-0426-06 remain `EVIDENCE_GAP`. They should not precede the
fuzz-lock blocker. F-0426-05 can later become a provider formal/vector
alignment lane. F-0426-06 should be sequenced after the fuzz lock has an exact
dependency authorization path.

## Secret material / side-channel / service boundary triage

Focus findings: F-0426-07, F-0426-08, and F-0426-09.

Evidence reviewed:

- `PqKem768` trait return types;
- `StdCrypto` KEM keypair/encap/decap helpers;
- qsc handshake pending state fields;
- qsc identity vault KEM key fields;
- canonical SCKA and Suite-2 docs mentioning KEM shared secrets and key
  protection requirements;
- stewardship canon public-claim and service-boundary sections.

Findings:

- `PqKem768` returns shared secrets as `Vec<u8>`, and helper functions expose
  KEM secret-key bytes as `Vec<u8>`.
- Some classical private-key types derive zeroization traits, but NA-0427 did
  not find equivalent provider-boundary proof for KEM `Vec<u8>` outputs.
- Side-channel and timing status remains unestablished. NA-0427 did not perform
  a side-channel audit.
- qsl-server, qsl-attachments, qshield runtime, public docs, README,
  START_HERE, and website remain outside scope.

Triage result:

- F-0426-07 is a `BACKLOG_CANDIDATE` for a future key-material lifecycle lane.
- F-0426-08 is `CLAIM_BOUNDARY_ONLY`; no side-channel-free claim is made.
- F-0426-09 is `CLAIM_BOUNDARY_ONLY`; no service or public-readiness claim is
  made.

## Prioritization and successor selection

Selection rule outcome:

F-0426-04 is classified `FUZZ_LOCK_ACTIVE_SECURITY_BLOCKER`, so NA-0428 is:

`QSL qsc Fuzz Lock pqcrypto Residual Dependency Blocker Authorization Plan`

Why this outranks the normal provider-error successor:

- qsc provider-error/no-mutation proof is important but evidence-incomplete.
- The fuzz lock is an active committed fuzz workspace dependency artifact with
  a red nested audit.
- Fuzz/property/differential evidence should not be expanded or cited as clean
  while its separate lock is red.
- NA-0427 cannot mutate Cargo, dependency, lockfile, workflow, runtime, crypto,
  tests, or vectors, so the correct next step is exact blocker authorization.

## Future path/scope bundle

Future allowed paths for selected NA-0428:

- `docs/governance/evidence/NA-0428_qsl_qsc_fuzz_lock_pqcrypto_residual_dependency_blocker_authorization_plan.md`
- `tests/NA-0428_qsl_qsc_fuzz_lock_pqcrypto_residual_dependency_blocker_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0428 may inspect read-only:

- `qsl/qsl-client/qsc/fuzz/Cargo.toml`
- `qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `qsl/qsl-client/qsc/fuzz/fuzz_targets/`
- `scripts/ci/qsc_adversarial.sh`
- `.github/workflows/qsc-adversarial.yml`
- root `Cargo.toml`
- root `Cargo.lock`
- `qsl/qsl-client/qsc/Cargo.toml`
- `tools/refimpl/quantumshield_refimpl/Cargo.toml`
- relevant prior evidence docs and testplans

Future NA-0428 must not mutate unless later exact scope authorizes:

- runtime code;
- crypto implementation code;
- dependencies, Cargo manifests, or lockfiles;
- workflows;
- tests or vectors;
- qsl-server or qsl-attachments;
- qshield runtime;
- website, public docs, README, or START_HERE;
- backup, restore, qsl-backup, backup status, backup plan, rollback subtree, or
  `/backup/qsl`;
- qwork, qstart, qresume, or qshell;
- public technical paper content;
- no public assurance claims.

Future NA-0428 deliverables should determine whether the next concrete lane is
a narrow fuzz-lock regeneration/remediation lane, a manifest feature-boundary
repair lane, a workflow/CI dependency-health lane, or a combined exact-scope
dependency remediation lane.

## Future validation/marker plan

Future NA-0428 markers:

- `NA0428_QSC_FUZZ_LOCK_BLOCKER_AUTHORIZATION_OK`
- `NA0428_FUZZ_LOCK_AUDIT_TRIAGE_OK`
- `NA0428_PQCRYPTO_RESIDUAL_CLASSIFIED_OK`
- `NA0428_EXACT_CARGO_FUZZ_LOCK_AUTHORIZATION_PLAN_OK`
- `NA0428_ROOT_CARGO_AUDIT_GREEN_OK`
- `NA0428_NESTED_FUZZ_LOCK_STATUS_RECORDED_OK`
- `NA0428_NO_RUNTIME_CHANGE_OK`
- `NA0428_NO_CRYPTO_CHANGE_OK`
- `NA0428_NO_DEPENDENCY_CHANGE_BY_AUTHORIZATION_OK`
- `NA0428_NO_WORKFLOW_CHANGE_BY_AUTHORIZATION_OK`
- `NA0428_NO_TEST_VECTOR_MUTATION_OK`
- `NA0428_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0428_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0428_NO_VULNERABILITY_FREE_CLAIM_OK`
- `NA0428_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0428_ONE_READY_INVARIANT_OK`

## Public claim/external review/website boundary

NA-0427 is internal governance evidence only.

- No external-review-complete claim is made.
- No production-readiness claim is made.
- No public-internet-readiness claim is made.
- No crypto-complete claim is made.
- No side-channel-free claim is made.
- No bug-free claim is made.
- No vulnerability-free claim is made.
- No perfect-crypto claim is made.
- No metadata-free claim is made.
- No anonymity claim is made.
- No untraceability claim is made.
- No public technical paper content is created.
- No README, START_HERE, public docs, docs-public, website, qsl-server, or
  qsl-attachments update occurred.
- Root cargo audit green is dependency-health evidence only.
- The nested fuzz lock audit red state is a blocker-classification signal, not
  a public vulnerability statement.

## Rejected alternatives

- Select the default qsc provider-error/no-mutation successor now: rejected
  because the active nested fuzz-lock blocker outranks it.
- Treat the nested fuzz lock as stale resolved residue: rejected because the
  qsc adversarial smoke script runs the qsc fuzz workspace.
- Refresh the fuzz lock now: rejected because Cargo/lockfile changes are out
  of NA-0427 scope.
- Rename the historical `pqcrypto` feature now: rejected because Cargo
  manifest changes are out of NA-0427 scope.
- Add qsc provider-error tests now: rejected because test mutations are out of
  NA-0427 scope.
- Add KAT/differential vectors now: rejected because test/vector mutations are
  out of NA-0427 scope.
- Move directly to nonce/key/RNG lifecycle audit: rejected because F-0426-04
  and F-0426-02 remain higher-priority provider-boundary residuals.
- Create website, public-doc, README, START_HERE, or public-paper claim text:
  rejected because public surfaces and public claims remain out of scope.

## Backup-impact statement

Codex did not run backup or restore. Codex did not mutate qsl-backup,
`/backup/qsl`, backup logs, backup manifests, backup status files, backup plan
files, rollback subtree paths, systemd, timers, fstab, source lists, retention,
or backup scripts.

Read-only proof:

- qsl-backup checksum matched the directive-required SHA256.
- qsl-backup source inclusion count for the Codex ops source path was exactly
  `1`.
- Disk watermark during the lane: root filesystem `47%` used; backup mount
  `4%` used.

No backup-complete claim is made. No off-host-backup-complete claim is made. No
disaster-recovery-complete claim is made. No restore-proven claim is made.

## Next recommendation

Merge the NA-0427 evidence PR if validation and required checks remain green.
After post-merge public-safety is green, close out NA-0427 and restore the
selected NA-0428 qsc fuzz-lock pqcrypto residual dependency blocker
authorization plan as the sole READY item. Do not implement NA-0428 during
NA-0427 closeout.
