Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-06

# NA-0432 qsc Provider Error Path / No-Mutation Read-Only Audit Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0432 completed a bounded read-only audit of qsc provider-error paths and
no-mutation-on-reject evidence after the D257 / NA-0418 `ml-kem` provider
replacement, the NA-0426/NA-0427 provider-boundary audits, and the NA-0431
nested qsc fuzz lock cleanup.

No runtime code, crypto code, dependencies, Cargo manifests, lockfiles,
workflows, executable tests, fuzz targets, vectors, qsl-server, qsl-attachments,
qshield runtime, website, public docs, README, START_HERE, qwork tooling,
backup state, or qsl-backup path was mutated by this audit.

The audit found:

- qsc has runtime marker paths for `pq_encap_failed` and `pq_decap_failed`.
- qsc maps provider errors to fixed `handshake_reject` reason labels rather
  than exposing raw provider errors.
- Provider-level wrong-length input tests exist for `PqKem768`.
- qsc has broader handshake reject/no-mutation tests and formal no-mutation
  models, but no executable qsc test directly forces `pq_encap_failed` or
  `pq_decap_failed` and snapshots local qsc durable state before and after
  those exact provider-error rejects.
- qsc adversarial fuzz targets cover route parsing, payload parsing, and vault
  envelope parsing; they do not currently target qsc handshake provider-error
  encap/decap paths.
- Root and nested dependency health remain green after NA-0431; nested pqcrypto
  residual package IDs are absent.

No immediate provider-error/no-mutation BLOCKER was found. The selected
successor is:

`NA-0433 -- QSL qsc Provider Error Path / No-Mutation Findings Triage Authorization Plan`

Classification:

`NO_MUTATION_PROOF_QSC_LEVEL_PARTIAL`

This classification means qsc has meaningful broader reject/no-mutation
evidence, but exact provider-error no-mutation evidence at the qsc boundary is
incomplete.

## Live NA-0432 scope

Live `NEXT_ACTIONS.md` lists:

`NA-0432 -- QSL qsc Provider Error Path / No-Mutation Read-Only Audit Plan`

Status: READY.

Allowed NA-0432 mutation paths:

- `docs/governance/evidence/NA-0432_qsl_qsc_provider_error_path_no_mutation_read_only_audit_plan.md`
- `tests/NA-0432_qsl_qsc_provider_error_path_no_mutation_read_only_audit_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Read-only inspection covered:

- qwork proof files under `/srv/qbuild/work/NA-0432/.qwork/`
- `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`, and the rolling journal
- NA-0431, NA-0430, NA-0427, and NA-0426 evidence
- `qsl/qsl-client/qsc/`
- `qsl/qsl-client/qsc/fuzz/`
- `tools/refimpl/quantumshield_refimpl/src/crypto/`
- `tools/refimpl/quantumshield_refimpl/tests/`
- `formal/`
- `inputs/`
- root and nested Cargo lock health by read-only commands

Forbidden mutation scope:

- no runtime or crypto implementation mutation;
- no dependency, Cargo manifest, or lockfile mutation;
- no workflow, script, executable test, fuzz target, or vector mutation;
- no qsl-server, qsl-attachments, qshield runtime, website, public docs,
  README, or START_HERE mutation;
- no qwork, qstart, qresume, or qshell execution or mutation by Codex;
- no backup or restore execution;
- no qsl-backup, backup status, backup plan, rollback subtree, or `/backup/qsl`
  mutation;
- no public technical paper content;
- no public readiness, production readiness, external-review completion,
  crypto-complete, side-channel-free, bug-free, vulnerability-free, or
  perfect-crypto claim.

Acceptance criteria:

- qsc provider-error path audit remains read-only;
- no-mutation-on-reject evidence is assessed without overclaiming;
- findings matrix is created;
- NA-0433 successor is selected;
- no runtime/crypto/dependency/Cargo/lockfile/workflow/test/vector mutation
  occurs;
- root cargo audit and nested fuzz lock audit remain green;
- public-safety remains green before merge and after merge;
- exactly one READY item remains.

Stop conditions preserved:

- qwork proof files missing, malformed, stale, or inconsistent;
- qwork, qstart, or qresume run by Codex;
- PR #1133 not merged at the expected lineage;
- queue not READY NA-0432 at start;
- D-0851 absent or D-0852 already present at start;
- root or nested cargo audit not green;
- provider-error paths cannot be inspected safely;
- forbidden mutation, backup/restore, qsl-backup mutation, or public overclaim
  occurs;
- more than one READY item exists.

## qwork proof-file verification

Codex read, but did not run, qwork proof files:

- `/srv/qbuild/work/NA-0432/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0432/.qwork/startup.qsl-protocol.json`

Required `.kv` markers were present:

- `startup_result=OK`
- `lane=NA-0432`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0432/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0432`
- `requested_lane_status=READY`

The JSON proof parsed successfully and mirrored the required `.kv` values.
After `git fetch --all --prune`, live `HEAD` and `origin/main` still matched
the qwork proof at `87ccb96d90ef`. PR #1133 was verified MERGED with merge
commit `87ccb96d90ef`.

Proof root:

`/srv/qbuild/tmp/NA0432_qsc_provider_error_no_mutation_audit_20260606T232840Z`

The qwork proof files were copied into the proof root under `qwork/`.

## NA-0431 inheritance

Inherited from D274/D275 and NA-0431:

- PR #1132 cleaned the nested qsc fuzz lock with the D-0847 precise-version
  strategy and merged at `77df962590e4`.
- PR #1133 closed out NA-0431 and restored NA-0432 as the sole READY item at
  `87ccb96d90ef`.
- Root `cargo audit --deny warnings` is green.
- Nested qsc fuzz lock `cargo audit --deny warnings --file
  qsl/qsl-client/qsc/fuzz/Cargo.lock` is green.
- Nested pqcrypto residual package ID scan returned zero matches.
- `qsc-adversarial-smoke` succeeded on PR #1132 head, PR #1132 merge commit,
  and PR #1133 merge commit.
- PR #1127 remains closed and unmerged, with its branch retained as
  failed-attempt comparison evidence.

## Stewardship template application

### Crypto / Protocol Steward

Review question: How do qsc provider encap/decap errors propagate, and is
no-mutation-on-reject evidence sufficient at the qsc boundary?

Evidence reviewed: `PqKem768` trait, `StdCrypto` `ml-kem` implementation,
provider tests, qsc handshake source, qsc handshake tests, qsc suite-id formal
model, suite/vector roots, NA-0426 and NA-0427 evidence.

Findings: qsc calls `StdCrypto.encap` and `StdCrypto.decap` through the
`PqKem768` trait surface. Provider failures are mapped to fixed
`handshake_reject` labels `pq_encap_failed` and `pq_decap_failed`. The code
paths return before local qsp session store or responder pending store, and
the initiator decap failure returns before pending clear. However, direct qsc
tests that force those exact provider-error labels and snapshot local durable
state are absent.

Risk classification: MEDIUM / EVIDENCE_INCOMPLETE.

Public-claim impact: no crypto-complete, side-channel-free, bug-free,
vulnerability-free, perfect-crypto, or external-review completion claim is
supported.

Scope impact: future tests or remediation require exact future scope.

Recommended action: select NA-0433 findings triage to decide whether exact
qsc provider-error no-mutation tests and marker documentation are needed.

### CI / Dependency / Release Health Steward

Review question: Is dependency/fuzz health still clear after NA-0431, and do
current adversarial/fuzz checks cover provider-error paths?

Evidence reviewed: root cargo audit, nested fuzz lock audit, root inverse
dependency trees, nested pqcrypto residual scan, `scripts/ci/qsc_adversarial.sh`,
`.github/workflows/qsc-adversarial.yml`, qsc fuzz targets, and qsc adversarial
tests.

Findings: root and nested cargo audits are green. Root `rustls-webpki` is
`v0.103.13`; root `ml-kem` is present through `quantumshield_refimpl`; root
pqcrypto package IDs are absent. The nested fuzz lock no longer contains the
pqcrypto package IDs. The qsc adversarial script runs Rust adversarial tests
and fuzz targets for route, payload, and vault parsers, but not qsc handshake
provider-error encap/decap paths.

Risk classification: LOW for dependency health; MEDIUM / EVIDENCE_INCOMPLETE
for provider-error adversarial coverage.

Public-claim impact: cargo audit green is dependency-health evidence only.

Scope impact: no dependency, Cargo, workflow, script, fuzz target, or test
mutation is authorized by NA-0432.

Recommended action: carry provider-error fuzz/adversarial coverage into
NA-0433 triage.

### Public Claims / External Review Steward

Review question: Does the read-only audit support public readiness or broad
security claims?

Evidence reviewed: GOALS, Project Charter, stewardship canon, NA-0426/NA-0427
claim caveats, current findings matrix, dependency health evidence, and
formal/test coverage limits.

Findings: NA-0432 is internal governance evidence only. It creates no public
docs, website content, README/START_HERE text, public technical paper content,
or external-review package. No claim category is expanded.

Risk classification: CLAIM_BOUNDARY.

Public-claim impact: no public readiness, production readiness,
public-internet readiness, external-review completion, crypto-complete,
side-channel-free, bug-free, vulnerability-free, or perfect-crypto claim is
supported.

Scope impact: no public-surface mutation is authorized.

Recommended action: preserve explicit caveats in NA-0433.

### Product / Demo / Service Boundary Steward

Review question: Does qsc provider-error evidence blur qsc, refimpl, demo, or
service boundaries?

Evidence reviewed: qsc runtime path, `quantumshield_refimpl` provider path,
qsc fuzz/adversarial script, qsl-server/qsl-attachments boundary inheritance,
and service references in prior traceability.

Findings: Provider-error evidence is qsc/refimpl internal evidence. It does
not mutate qsl-server or qsl-attachments and does not prove service-local,
demo, deployment, or public-internet properties. Relay queue semantics are
separate from local qsc durable-state no-mutation unless a future directive
brings that boundary into exact scope.

Risk classification: CLAIM_BOUNDARY / INFO.

Public-claim impact: no demo-as-production, service readiness, or public
deployment claim is supported.

Scope impact: no service or public surface mutation is authorized.

Recommended action: keep future provider-error tests scoped to qsc local state
unless future scope explicitly includes relay queue behavior.

### Local Ops / Backup / Restore Steward

Review question: Were qwork proof files, qsl-backup, backup state, and local
ops boundaries preserved?

Evidence reviewed: qwork proof files, live git state, qsl-backup SHA,
qsl-backup source-list count, prior responses, and rolling journal.

Findings: qwork proof files were read and copied to the proof root; Codex did
not run qwork, qstart, or qresume. qsl-backup SHA matched the required value,
and the source inclusion count for the Codex ops path was exactly one. Codex
did not run backup or restore and did not mutate backup state.

Risk classification: INFO.

Public-claim impact: no backup completion, off-host backup, disaster recovery,
or restore proof claim is supported.

Scope impact: only the directive-authorized proof root and repository
governance paths were changed.

Recommended action: continue no-backup/no-restore discipline.

## qsc provider error path inventory

Commands used:

```bash
rg -n -i -e 'pq_encap_failed' -e 'pq_decap_failed' -e 'encap_failed' -e 'decap_failed' -e 'provider' -e 'PqKem768' -e 'StdCrypto' -e 'pqkem' -e 'kem' -e 'ml-kem' -e 'ML-KEM' -e 'error' -e 'reject' -e 'fail' -e 'abort' -e 'commit' -e 'transcript' -e 'send_commit' -e 'recv' -e 'handshake' -e 'state' -e 'mutate' -e 'pending' -e 'session' -e 'rollback' <allowed roots> --count-matches
rg -n -i 'pq_encap_failed|pq_decap_failed|encap_failed|decap_failed' <allowed roots>
rg -n 'PqKem768|StdCrypto|runtime_pq|encap\(|decap\(|InvalidKey|AuthFail|MlKem|ml_kem|pqkem' <provider/qsc roots>
rg -n -i 'pq_encap_failed|pq_decap_failed|provider-error|provider error|no mutation|no-mutation|state unchanged|snapshot|rollback|mutat' <test/formal roots>
```

Roots scanned:

- `qsl/qsl-client/qsc/`
- `tools/refimpl/quantumshield_refimpl/src/crypto/`
- `tools/refimpl/quantumshield_refimpl/tests/`
- `qsl/qsl-client/qsc/fuzz/`
- `formal/`
- `inputs/`
- `tests/`
- `docs/governance/evidence/`

Top source/evidence clusters by broad search count:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`: 559 matches.
- `qsl/qsl-client/qsc/src/main.rs`: 409 matches.
- `docs/governance/evidence/NA-0426_qsl_crypto_api_provider_boundary_read_only_audit_plan.md`: 325 matches.
- qsc TUI/transport/contacts/attachment modules: high incidental counts from
  state, error, and reject terminology.
- `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`: 140 matches.

Marker names found:

- `pq_encap_failed`: found in `qsl/qsl-client/qsc/src/handshake/mod.rs`.
- `pq_decap_failed`: found in `qsl/qsl-client/qsc/src/handshake/mod.rs`.
- `handshake_reject`: many qsc reject paths.
- `handshake_send`, `handshake_recv`, `handshake_complete`,
  `handshake_pending`.

Marker names absent from executable qsc tests:

- direct test references to `pq_encap_failed`: absent.
- direct test references to `pq_decap_failed`: absent.

Caveat: search counts are planning signals, not bug findings. The broad terms
intentionally include high-noise words such as `state`, `error`, and `commit`.

## qsc call path / error mapping review

Provider selection / instantiation:

- qsc imports `StdCrypto` and `PqKem768` in `qsl/qsl-client/qsc/src/handshake/mod.rs`.
- qsc instantiates the provider with `let c = StdCrypto;`.
- qsc calls `c.encap(&init.kem_pk)` on the responder path.
- qsc calls `c.decap(&pending.kem_sk, &resp.kem_ct)` on the initiator path.
- qsc uses runtime KEM size helpers for frame encoding/decoding.

Provider implementation:

- `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs` defines
  `PqKem768`.
- `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs` implements
  `PqKem768` for `StdCrypto` using `ml-kem`.
- Wrong-size public key, secret key, and ciphertext conversions map to
  `CryptoError::InvalidKey`.
- Provider encapsulation failure maps to `CryptoError::InvalidKey`.
- Provider decapsulation failure maps to `CryptoError::AuthFail`, while
  wrong-size ciphertext conversion maps to `CryptoError::InvalidKey`.

qsc mapping:

- Responder encap failure emits `event=handshake_reject reason=pq_encap_failed`
  and continues without responder pending store, B1 send, or session store.
- Initiator decap failure emits `event=handshake_reject reason=pq_decap_failed`
  and returns before session store or pending clear.
- qsc does not expose raw provider error variants or third-party provider text
  at the qsc marker boundary.

Malformed provider input behavior:

- Provider-level wrong-length KEM inputs fail closed in `pqkem768` tests.
- qsc frame decoding enforces fixed KEM public-key and ciphertext lengths before
  provider calls.
- Direct qsc tests for malformed-but-frame-shaped provider values are not
  present.

Evidence gaps:

- No executable qsc test directly forces `pq_encap_failed`.
- No executable qsc test directly forces `pq_decap_failed`.
- No executable qsc test snapshots local durable state before and after either
  exact provider-error marker.
- No formal model currently includes provider byte conversion failure classes.
- No qsc fuzz target covers provider-error handshake encap/decap paths.

Public-claim caveat: current evidence supports internal provider-error path
mapping, not a complete fail-closed proof for all provider-originated faults.

## no-mutation-on-reject review

qsc state that can be mutated by relevant flows:

- handshake pending state through `hs_pending_store` and `hs_pending_clear`;
- qsp session state through `qsp_session_store`;
- relay inbox state through transport pull/push operations;
- send state and outbox state in send/commit flows;
- receive/session state in receive and ratchet flows.

Current evidence:

- qsc handshake tests assert no session creation for malformed/tampered/out of
  order handshake inputs.
- qsc suite-id parameter-block tests assert no pending state, no session state,
  no `recv_commit`, and no qsp output for several reject categories.
- qsc A2 replay tests snapshot session blobs before and after replay rejects.
- qsc `send_commit` tests assert failed relay send does not advance send state.
- qsc receive, relay, file transfer, vault, and locked-state tests contain
  additional no-mutation patterns.
- Formal models assert no accepted-state mutation on reject for SCKA,
  Suite-2 negotiation, and qsc suite-id model slices.

Exact provider-error evidence status:

- Provider-level KEM operations are stateless at the test boundary.
- qsc source review indicates the provider-error branches return before local
  qsp session store and before responder pending store; the initiator decap
  branch returns before pending clear.
- Direct qsc pre/post local durable-state snapshots for `pq_encap_failed` and
  `pq_decap_failed` are absent.
- Relay queue effects are not classified as local qsc durable-state mutation in
  the current evidence; a future lane should state that boundary explicitly if
  it tests provider-error rejection through relay-poll flows.

Classification:

`NO_MUTATION_PROOF_QSC_LEVEL_PARTIAL`

Future exact evidence needed:

- a qsc-level test that forces a responder-side provider encap error, asserts
  `reason=pq_encap_failed`, and snapshots local qsc durable state before and
  after;
- a qsc-level test that forces an initiator-side provider decap error, asserts
  `reason=pq_decap_failed`, and snapshots local qsc durable state before and
  after;
- explicit expected behavior for pending-state retention or clearance on each
  provider-error reject;
- explicit statement whether relay queue consumption is outside or inside the
  no-mutation invariant being tested;
- optional formal/model update only if provider-error failure classes are meant
  to become modeled qsc state-machine categories.

## test / fuzz / formal coverage review

Existing provider tests:

- `tools/refimpl/quantumshield_refimpl/tests/pqkem768.rs`
  - `pqkem768_roundtrip_matches`
  - `pqkem768_tamper_changes_secret`
  - `pqkem768_wrong_length_inputs_fail_closed`
- unit tests in `stdcrypto.rs` cover roundtrip, tamper, and runtime helper
  lengths.

Existing qsc reject/no-mutation tests:

- `qsl/qsl-client/qsc/tests/handshake_mvp.rs` covers malformed, tampered,
  out-of-order, signature-tamper, and replay rejects.
- `qsl/qsl-client/qsc/tests/na_0313_handshake_suite_id_parameter_block.rs`
  covers suite-id reject categories, no pending state, no session state,
  no output, and no accepted session mutation on replay.
- `qsl/qsl-client/qsc/tests/send_commit.rs` covers no send-state advance on
  relay send failure.
- `qsl/qsl-client/qsc/tests/receive_no_mutation.rs`,
  `ratchet_step.rs`, `unlock_gate.rs`, and service/attachment tests cover
  additional reject/no-mutation categories.

Fuzz/adversarial coverage:

- `scripts/ci/qsc_adversarial.sh` runs qsc adversarial Rust tests and three
  cargo-fuzz targets.
- Existing fuzz targets cover qsc route HTTP parsing, payload boundaries, and
  vault envelope parsing.
- Existing adversarial property tests cover route token handling, bundle
  invariants, suite2 establish rejection for unauthenticated commitment, and
  snapshot roundtrip properties.
- No fuzz target currently drives qsc handshake provider-error encap/decap
  branches.
- Local validation note: `scripts/ci/qsc_adversarial.sh` completed
  `adversarial_properties` with 8 passed tests and `adversarial_miri` with 6
  passed tests, then stopped at the local tooling boundary with
  `error: no such command: fuzz`. This was classified as recoverable local
  cargo-fuzz availability, not a qsc Rust test failure; PR CI
  `qsc-adversarial-smoke` remains the required fuzz-stage gate.

Formal/model coverage:

- `formal/run_model_checks.py` runs SCKA, Suite-2 negotiation, and qsc
  suite-id bounded checks.
- The qsc suite-id model asserts deterministic reject, no accepted-state
  mutation, no output, no `recv_commit`, and no leak flags for the modeled
  suite-id slice.
- Formal models are crypto-agnostic and do not model provider byte conversion
  or provider error classes.

Future test/remediation lane need:

- A future triage lane should decide whether to authorize qsc provider-error
  no-mutation tests.
- A future triage lane should decide whether provider-error marker behavior
  should be documented as an internal qsc invariant.
- A future triage lane should decide whether provider-error failure classes
  should be added to fuzz/adversarial or formal/model coverage.

## findings matrix

| ID | Title | Domain | Severity | Evidence references | Affected files/roots | Steward domain | Public-claim impact | Recommended queue action | Immediate blocker successor needed |
|---|---|---|---|---|---|---|---|---|---|
| F-0432-01 | qsc provider-error markers exist and are sanitized | Provider-error mapping | INFO | `qsl/qsl-client/qsc/src/handshake/mod.rs`; `StdCrypto`; `PqKem768` | `qsl/qsl-client/qsc/src/handshake/mod.rs`; `tools/refimpl/quantumshield_refimpl/src/crypto/` | Crypto / Protocol | No crypto-complete or external-review completion claim | Carry as accepted baseline into NA-0433 | No |
| F-0432-02 | exact qsc provider-error no-mutation test proof is incomplete | No-mutation evidence | MEDIUM / EVIDENCE_INCOMPLETE | `pq_encap_failed`; `pq_decap_failed`; qsc handshake tests; suite-id tests | `qsl/qsl-client/qsc/src/handshake/mod.rs`; `qsl/qsl-client/qsc/tests/` | Crypto / Protocol | No fail-closed-complete or correctness-complete claim | NA-0433 should decide whether to authorize qsc-level tests | No |
| F-0432-03 | provider-level wrong-length reject coverage exists, but qsc-boundary coverage is narrower | Coverage boundary | MEDIUM / EVIDENCE_INCOMPLETE | provider `pqkem768` tests; qsc marker absence in tests | `tools/refimpl/quantumshield_refimpl/tests/pqkem768.rs`; `qsl/qsl-client/qsc/tests/` | Crypto / Protocol | No broad provider-boundary completeness claim | NA-0433 should classify provider-level versus qsc-level evidence | No |
| F-0432-04 | formal models cover reject no-mutation generally, not provider-error classes | Formal alignment | EVIDENCE_INCOMPLETE | `formal/README.md`; `formal/model_qsc_handshake_suite_id_bounded.py` | `formal/`; `inputs/` | Crypto / Protocol | No formally-proven provider implementation claim | NA-0433 should decide whether formal alignment is needed | No |
| F-0432-05 | qsc fuzz/adversarial smoke does not target provider-error encap/decap paths | Fuzz/adversarial coverage | EVIDENCE_INCOMPLETE | `scripts/ci/qsc_adversarial.sh`; qsc fuzz targets | `qsl/qsl-client/qsc/fuzz/`; qsc adversarial tests | CI / Dependency / Release Health | No exhaustive fuzzing or bug-free claim | NA-0433 should decide whether coverage expansion is needed | No |
| F-0432-06 | public claims must stay conservative | Claim boundary | CLAIM_BOUNDARY | GOALS; stewardship canon; NA-0426/NA-0427 caveats | Governance docs | Public Claims / External Review | No public readiness, production readiness, crypto-complete, side-channel-free, bug-free, vulnerability-free, or perfect-crypto claim | Preserve caveats in NA-0433 | No |
| F-0432-07 | service/demo boundary remains separate from qsc provider-error evidence | Service/demo boundary | CLAIM_BOUNDARY / INFO | qsc/refimpl evidence; service boundary inheritance | qsc/refimpl roots; qsl-server/qsl-attachments references only | Product / Demo / Service | No service or public deployment claim | Keep service/demo out of NA-0433 unless exact scope authorizes | No |
| F-0432-08 | dependency health is green after NA-0431 | Dependency health | INFO | root and nested cargo audits; cargo tree; nested pqcrypto scan | root `Cargo.lock`; qsc fuzz `Cargo.lock` | CI / Dependency / Release Health | Cargo audit green is dependency-health evidence only | Carry as resolved inheritance | No |

## successor selection

Selected successor:

`NA-0433 -- QSL qsc Provider Error Path / No-Mutation Findings Triage Authorization Plan`

Selection rationale:

- no immediate provider-error/no-mutation runtime blocker was found;
- qsc marker paths exist and are sanitized;
- source review suggests local qsc durable-state mutation is avoided on the
  two provider-error branches;
- exact qsc provider-error no-mutation tests are incomplete;
- normal findings triage is sufficient and preserves queue discipline.

Rejected successor options:

- `QSL qsc Provider Error Path / No-Mutation Blocker Remediation Authorization
  Plan`: rejected because no immediate BLOCKER or HIGH runtime issue was found.
- `QSL Nonce / Key / RNG Lifecycle Read-Only Audit Plan`: rejected because the
  provider-error/no-mutation findings should be consumed first.
- `QSL qsc Provider Error Path Evidence Gap Resolution Plan`: rejected because
  evidence was clear enough to create a findings matrix and select normal
  triage.

## future path/scope bundle

Future allowed paths for normal NA-0433:

- `docs/governance/evidence/NA-0433_qsl_qsc_provider_error_path_no_mutation_findings_triage_authorization_plan.md`
- `tests/NA-0433_qsl_qsc_provider_error_path_no_mutation_findings_triage_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0433 may:

- triage F-0432 findings;
- decide whether qsc no-mutation test implementation should be authorized;
- decide whether provider-error marker documentation should be authorized;
- decide whether formal/model alignment should be authorized;
- decide whether provider-error fuzz/adversarial coverage should be authorized;
- decide whether the next audit domain should move to nonce/key/RNG lifecycle.

Future forbidden unless exact scope authorizes:

- runtime or crypto implementation changes;
- dependency or Cargo changes;
- lockfile changes;
- workflow changes;
- executable test, fuzz target, or vector mutations;
- public docs, README, START_HERE, or website changes;
- qsl-server or qsl-attachments changes;
- backup, restore, qsl-backup, backup status, or backup plan changes;
- public assurance claims.

## future validation/marker plan

Future NA-0433 markers:

- `NA0433_PROVIDER_ERROR_FINDINGS_TRIAGE_OK`
- `NA0433_NO_MUTATION_FINDINGS_CONSUMED_OK`
- `NA0433_QSC_LEVEL_EVIDENCE_DECISION_OK`
- `NA0433_NO_RUNTIME_CHANGE_OK`
- `NA0433_NO_DEPENDENCY_CHANGE_OK`
- `NA0433_NO_TEST_MUTATION_OK`
- `NA0433_NO_WORKFLOW_CHANGE_OK`
- `NA0433_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0433_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0433_NO_SECRET_MATERIAL_OK`
- `NA0433_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0433_ONE_READY_INVARIANT_OK`

## public claim/external review/website boundary

This qsc provider-error/no-mutation audit is internal governance evidence only.

- It is not production readiness.
- It is not public-internet readiness.
- It is not external-review completion.
- It is not crypto-complete proof.
- It is not side-channel-free proof.
- It is not bug-free proof.
- It is not vulnerability-free proof.
- It is not perfect-crypto proof.
- It is not public technical paper content.
- It does not update README, START_HERE, public docs, docs-public, or website.
- It does not mutate qsl-server or qsl-attachments.
- Cargo audit green is dependency-health evidence only.
- No-mutation evidence in this audit is bounded evidence, not full correctness
  proof.

## rejected alternatives

- Mutate qsc provider-error tests now: rejected because executable test
  mutation is out of NA-0432 scope.
- Mutate qsc handshake code now: rejected because runtime/crypto mutation is
  out of NA-0432 scope and no immediate blocker was found.
- Mutate qsc fuzz targets now: rejected because fuzz target mutation is out of
  NA-0432 scope.
- Mutate formal models now: rejected because formal mutation is out of NA-0432
  scope.
- Move directly to nonce/key/RNG audit: rejected because F-0432 findings should
  be triaged first.
- Create public-facing claim text: rejected because public surfaces are out of
  scope and the evidence is internal and bounded.

## backup-impact statement

Codex did not run backup or restore. Codex did not run sudo. Codex did not
mutate qsl-backup, `/backup/qsl`, backup logs, backup manifests, backup status
files, backup plan files, rollback subtree paths, systemd, timers, fstab,
source lists, retention, or backup scripts.

Read-only proof:

- qsl-backup SHA matched the required value.
- qsl-backup source inclusion count for the Codex ops path was exactly `1`.

This audit makes no backup completion, off-host backup, disaster recovery, or
restore proof claim.

## next recommendation

Merge the NA-0432 read-only audit evidence PR if validation and required checks
remain green. After post-merge public-safety is green, close out NA-0432 and
restore the selected NA-0433 findings triage successor without implementing
NA-0433.
