Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-07

# NA-0440 qsc Provider Error Path Formal / Model Alignment Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0440 reviews whether current formal/model evidence directly aligns with the
completed qsc provider-error evidence chain:

- NA-0436 deterministic `pq_decap_failed` no-mutation test.
- NA-0437 `pq_encap_failed` defensive-branch documentation.
- NA-0439 adversarial script integration of the deterministic provider-error
  no-mutation test.

Selected authorization classification:

`PROVIDER_ERROR_FORMAL_MODEL_SUPPORTING_ONLY_NO_ACTION`

Current formal/model checks are relevant supporting G4 evidence because they
assert bounded fail-closed and no-mutation properties for SCKA, Suite-2
negotiation, and qsc suite-id admission. They do not directly model qsc KEM
provider failures, `pq_decap_failed`, `pq_encap_failed`, the qsc pending secret
store, the qsc session store, or the exact `StdCrypto` encap/decap call sites.

Selected successor:

`NA-0441 -- QSL Nonce / Key / RNG Lifecycle Read-Only Audit Plan`

No formal/model implementation is authorized by this directive. No runtime,
crypto, dependency, Cargo, lockfile, workflow, executable-test, fuzz-target,
vector, public-surface, service, formal-file, qwork, backup, restore, or
qsl-backup mutation is authorized or performed.

## Live NA-0440 scope

Live `NEXT_ACTIONS.md` lists:

`NA-0440 -- QSL qsc Provider Error Path Formal / Model Alignment Authorization Plan`

Status: READY.

Allowed NA-0440 mutation paths:

- `docs/governance/evidence/NA-0440_qsl_qsc_provider_error_path_formal_model_alignment_authorization_plan.md`
- `tests/NA-0440_qsl_qsc_provider_error_path_formal_model_alignment_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden current-lane mutation scope includes runtime code, crypto code,
dependencies, Cargo manifests, lockfiles, workflows, executable tests, fuzz
targets, vectors, formal model files, qsl-server, qsl-attachments, qshield
runtime, website, public docs, README, START_HERE, qwork/qstart/qresume/qshell,
qsl-backup, backup status, backup plan, rollback subtree, backup tree, and
backup/local-ops state.

Acceptance criteria:

- provider-error formal/model alignment need is classified;
- `pq_encap_failed` defensive branch caveat is preserved;
- `pq_decap_failed` test and adversarial evidence are consumed without
  overclaim;
- no implementation mutation occurs;
- root cargo audit remains green;
- nested qsc fuzz lock audit remains green;
- public-safety is green before merge and after merge;
- exactly one READY item remains.

Stop conditions included missing or inconsistent qwork proof, PR #1148 not
merged, queue drift from READY NA-0440, D-0866 absence, D-0867 preexistence,
audit failures, missing inherited evidence, unsafe formal/model classification,
unsafe successor selection, forbidden mutation, backup/restore execution,
qsl-backup/source-list regression, public overclaim, or more than one READY.

## qwork proof-file verification

Codex read, but did not run, qwork proof files:

- `/srv/qbuild/work/NA-0440/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0440/.qwork/startup.qsl-protocol.json`

Required `.kv` markers passed:

- `startup_result=OK`
- `lane=NA-0440`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0440/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0440`
- `requested_lane_status=READY`

The JSON proof parsed successfully and mirrored the `.kv` proof for lane, repo,
path, HEAD, origin/main, ready count, top READY item, requested lane status, and
clean-state fields.

Initial live `HEAD` and `origin/main` matched qwork proof SHA
`336831c84118`. After `git fetch --all --prune`, `origin/main` still matched
`336831c84118`, which is PR #1148's merge commit. PR #1148 was verified
MERGED with merge commit `336831c84118`.

Proof root:

`/srv/qbuild/tmp/NA0440_provider_error_formal_model_auth_20260608T024038Z`

## NA-0439 / NA-0437 / NA-0436 inheritance

NA-0439 inheritance:

- D-0865 integrated the existing provider-error no-mutation test into
  `scripts/ci/qsc_adversarial.sh`.
- D-0866 closed NA-0439 after PR #1147 merge, post-merge public-safety,
  qsc-adversarial-smoke, and qsc-adversarial-miri completed success.
- The script emits
  `NA0439_QSC_PROVIDER_ERROR_NO_MUTATION_ADVERSARIAL_STEP`.
- The provider-error test command runs before cargo-fuzz phases.
- The integration consumes bounded `pq_decap_failed` evidence and creates no
  executable `pq_encap_failed` coverage claim.

NA-0437 inheritance:

- Classification: `PQ_ENCAP_FAILED_DEFENSIVE_BRANCH_DOCUMENTED`.
- Caveat: `PQ_ENCAP_FAILED_PROVIDER_BEHAVIOR_DEPENDENT`.
- Current active provider and qsc external API evidence did not reach an
  executable `pq_encap_failed` failure path.
- Future executable coverage for that branch would require separately
  authorized exact scope such as a test seam, provider fake, provider behavior
  change, or equivalent defensible strategy.

NA-0436 inheritance:

- The deterministic test path is
  `qsl/qsl-client/qsc/tests/handshake_provider_error_no_mutation.rs`.
- The test corrupts Alice's test-local pending KEM secret after a valid B1,
  observes `pq_decap_failed`, and proves Alice/Bob session and pending/vault
  state remain unchanged by the reject.
- Local NA-0440 preflight reran the test successfully and observed:
  - `NA0436_PQ_DECAP_FAILED_MARKER_OK`
  - `NA0436_NO_SESSION_MUTATION_ON_DECAP_REJECT_OK`
  - `NA0436_PENDING_STORE_NO_MUTATION_ON_DECAP_REJECT_OK`
  - `NA0436_PQ_ENCAP_FAILED_CAVEAT_PRESERVED_OK`
  - `NA0436_NO_RUNTIME_HOOK_USED_OK`

## Applicable Stewardship Review

### Crypto / Protocol Steward

Formal/model alignment must preserve bounded provider-error evidence. Existing
formal checks remain supporting evidence unless directly mapped to qsc
implementation semantics. `pq_decap_failed` is executable-covered only by the
deterministic qsc no-mutation test and adversarial-script integration.
`pq_encap_failed` remains defensive-branch documentation only.

### CI / Dependency / Release Health Steward

Root `cargo audit --deny warnings` passed. Nested qsc fuzz lock audit passed.
The provider-error no-mutation test passed. The qsc adversarial script contains
the NA-0439 marker and command before cargo-fuzz phases. Public-safety was
required and green on current main. Formal checks passing is bounded supporting
evidence, not full implementation proof.

### Public Claims / External Review Steward

NA-0440 makes no crypto-complete claim. It makes no side-channel-free claim. It
makes no vulnerability-free claim. It makes no bug-free claim. It makes no
perfect-crypto claim. It makes no public-readiness claim. It makes no
production-readiness claim. It makes no external-review-complete claim.

### Product / Demo / Service Boundary Steward

qsc formal/provider-error evidence is internal engineering evidence. NA-0440
makes no qsl-server readiness claim, no qsl-attachments readiness claim, no
qshield runtime readiness claim, no website readiness claim, and no
public-service readiness claim.

### Local Ops / Backup / Restore Steward

No backup, restore, or local-ops mutation is authorized or performed.
qsl-backup proof remains boundary evidence only. The expected qsl-backup SHA
matched the directive boundary proof, and the reviewed scheduled manifest source
entry count for `/home/victor/work/qsl/codex/ops` was exactly one.

Level 1 stewardship is active in this evidence lane. Level 2 and Level 3 remain
future-gated. Stewards remain advisory only: no separate Directors, no
independent READY promotion, no independent merge authority, and Lead Director
final authority is preserved.

## Formal / model inventory

Formal/model files reviewed:

- `formal/README.md`
- `formal/__init__.py`
- `formal/model_scka_bounded.py`
- `formal/model_suite2_negotiation_bounded.py`
- `formal/model_qsc_handshake_suite_id_bounded.py`
- `formal/run_model_checks.py`

Current modeled properties:

- `formal/model_scka_bounded.py` models SCKA control-plane invariants:
  monotonic ADV acceptance, one-time CTXT targeting, tombstones, no state change
  on reject, and transactional commit. It is crypto-agnostic.
- `formal/model_suite2_negotiation_bounded.py` models Suite-2 downgrade and
  commitment rejection. It asserts rejected negotiation attempts leave modeled
  accepted/durable negotiation state unchanged. It is crypto-agnostic.
- `formal/model_qsc_handshake_suite_id_bounded.py` models qsc suite-id
  admission: QHSM v2 canonical context, explicit compatibility gate for v1,
  A1/B1/A2 context equality, transcript and key-context binding, deterministic
  rejects, no modeled accepted-state mutation, no output/recv_commit, and no
  secret sentinel leak on reject. It is crypto-agnostic.
- `formal/run_model_checks.py` executes the SCKA, negotiation, and qsc suite-id
  models as a fail-closed CI entry point.

Provider-error alignment inventory:

| Question | Current finding |
|---|---|
| Is no-mutation / reject / fail-closed behavior modeled? | Yes, generically for SCKA, negotiation, and qsc suite-id admission. |
| Are provider-error markers modeled? | No. |
| Are qsc session/pending store semantics modeled? | No. |
| Is KEM/provider failure modeled? | No. |
| Is `pq_decap_failed` directly represented? | No. |
| Is `pq_encap_failed` directly represented? | No. |
| Is the formal model crypto-agnostic? | Yes. |
| Do existing formal checks support qsc provider-error evidence? | Yes, as supporting fail-closed/no-mutation discipline only. |
| Do existing formal checks prove qsc provider-error implementation behavior? | No. |

Prior NA-0427 evidence already recorded the same boundary: formal models are
bounded and state-machine focused; they do not prove KEM provider byte
conversions, provider failure classes, side-channel behavior, or provider
implementation equivalence.

## Alignment gap review

### `pq_decap_failed`

Implementation evidence:

- deterministic no-mutation test exists;
- qsc adversarial script runs that test before cargo-fuzz phases;
- local NA-0440 preflight reran the test and observed the required markers.

Formal/model alignment:

- Existing models cover abstract reject-no-mutation patterns.
- Existing models do not directly represent `StdCrypto.decap`, malformed
  pending KEM secret state, `pq_decap_failed`, qsc pending secret keys, qsc
  vault bytes, qsc session files, or relay A2 absence.

Classification:

- `FORMAL_MODEL_SUPPORTING_ONLY`
- `FORMAL_MODEL_ALIGNMENT_EVIDENCE_GAP`

### `pq_encap_failed`

Implementation evidence:

- NA-0437 documents `pq_encap_failed` as a defensive branch.
- No executable coverage is claimed.
- Current provider/API evidence does not force the branch without separately
  authorized scope.

Formal/model alignment:

- Existing models do not directly represent `StdCrypto.encap`, KEM public-key
  provider failure, `pq_encap_failed`, or responder-side pending/session store
  effects at that exact branch.
- The provider-behavior caveat remains load-bearing.

Classification:

- `FORMAL_MODEL_SUPPORTING_ONLY`
- `FORMAL_MODEL_ALIGNMENT_EVIDENCE_GAP`

### General provider-error reject/no-mutation

Existing formal models support the broader engineering principle that rejects
should be deterministic, fail closed, and avoid accepted-state mutation. They do
not map that principle to qsc provider-error implementation state. A future
model extension could add value if scoped exactly, but the current evidence
chain is strong enough to avoid blocking on a formal/model implementation lane
now.

Required classification results:

- `FORMAL_MODEL_SUPPORTING_ONLY`: selected.
- `FORMAL_MODEL_DIRECT_ALIGNMENT_PRESENT`: rejected.
- `FORMAL_MODEL_ALIGNMENT_EVIDENCE_GAP`: present and caveated.
- `FORMAL_MODEL_EXTENSION_AUTHORIZATION_READY`: not selected.
- `FORMAL_MODEL_SCOPE_AUTHORIZATION_NEEDED`: not selected for immediate
  successor.
- `FORMAL_MODEL_NOT_RELEVANT_TO_PROVIDER_ERROR`: rejected.
- `FORMAL_MODEL_ALIGNMENT_AMBIGUOUS`: rejected.

## Options matrix

| Option | Recommendation | Evidence | Future paths | Risk | Validation requirements | Public-claim caveat |
|---|---|---|---|---|---|---|
| Option 1 - Accept formal/model evidence as supporting-only, no immediate formal/model implementation | Recommended | Formal checks are relevant but not directly mapped to qsc provider-error semantics; NA-0436/NA-0439 provide bounded executable decap evidence; NA-0437 preserves encap caveat | No implementation paths; future closeout may restore nonce/key/RNG audit governance paths | Direct formal alignment gap remains caveated | Formal checks stay green; provider-error test stays green; public-safety stays green | No crypto-complete, production-readiness, public-readiness, external-review-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim |
| Option 2 - Authorize formal/model scope planning | Rejected for immediate successor | A gap exists, but current provider-error chain is sufficiently documented and the next audit domain has higher value | Would use NA-0441 formal/model scope governance paths if selected | Could slow audit progress without changing implementation evidence | Read-only scope plan; formal checks; public-safety | Same no-overclaim boundary |
| Option 3 - Authorize formal/model implementation | Rejected | Exact invariant and state mapping would need additional design before mutation; no formal-file mutation is allowed here | None authorized by NA-0440 | Premature model edits could imply stronger alignment than proven | Would require exact formal paths and tests in future lane | Same no-overclaim boundary |
| Option 4 - Defer to fuzz/adversarial or deterministic test work | Rejected as immediate successor | The deterministic decap test and adversarial integration are already in place | Possible future fuzz target/provider-seam scope only if new evidence warrants | Could overfocus provider-error after the current chain has stabilized | qsc adversarial smoke and targeted tests | Same no-overclaim boundary |
| Option 5 - Move to next audit domain: Nonce / Key / RNG lifecycle | Recommended successor | Provider-error chain is documented; formal/model evidence is supporting-only; nonce/key/RNG lifecycle is a natural next code/crypto read-only audit domain | NA-0441 governance evidence/testplan, DECISIONS, TRACEABILITY, rolling journal | New audit must stay read-only and avoid secret handling | Root/nested audits, formal checks as applicable, public-safety, scope guard | Same no-overclaim boundary |
| Option 6 - Stop / ambiguity | Rejected | Formal/model evidence is safely classifiable | NA-0441 ambiguity plan not needed | None | N/A | Same no-overclaim boundary |

## Authorization decision

Selected primary classification:

`PROVIDER_ERROR_FORMAL_MODEL_SUPPORTING_ONLY_NO_ACTION`

Decision details:

- Existing formal/model checks are accepted as supporting-only evidence for
  fail-closed/no-mutation discipline.
- Direct formal alignment to provider-error implementation semantics is not
  present.
- The alignment gap is documented but does not require an immediate
  formal/model scope or implementation lane.
- No formal model mutation is authorized by NA-0440.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, executable-test,
  fuzz-target, vector, public-surface, service, backup, restore, or qsl-backup
  mutation is authorized by NA-0440.
- No public-claim expansion is authorized.

Exact future mutable implementation paths authorized by NA-0440: none.

Future validation that remains relevant:

- root `cargo audit --deny warnings`;
- nested qsc fuzz lock audit;
- `cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture`;
- adversarial script syntax and PR qsc-adversarial smoke where attached;
- `cargo fmt --check`;
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`;
- `python3 formal/run_model_checks.py`;
- public-safety required and green before and after merge.

## Successor selection

Selected exact NA-0441 successor:

`NA-0441 -- QSL Nonce / Key / RNG Lifecycle Read-Only Audit Plan`

Rationale:

- The provider-error chain now has deterministic decap evidence, adversarial
  script integration, and a preserved encap defensive-branch caveat.
- Formal/model evidence is supporting-only, not ambiguous.
- Immediate formal/model implementation would require additional exact scoping
  and is not necessary before moving to the next audit domain.
- Nonce/key/RNG lifecycle review continues the code/crypto audit sequence
  without implementation mutation.

NA-0441 is not implemented by NA-0440.

## Future path/scope bundle

Future NA-0441 allowed mutable paths:

- `docs/governance/evidence/NA-0441_qsl_nonce_key_rng_lifecycle_read_only_audit_plan.md`
- `tests/NA-0441_qsl_nonce_key_rng_lifecycle_read_only_audit_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0441 may inspect read-only:

- `qsl/`
- `tools/refimpl/`
- `formal/`
- `inputs/`
- `tests/`
- `docs/governance/evidence/`
- `Cargo.toml`
- `Cargo.lock`
- `qsl/qsl-client/qsc/fuzz/`
- relevant scripts/workflows read-only

Future NA-0441 forbidden unless later exact scope authorizes:

- runtime/crypto implementation mutation;
- dependency, Cargo, or lockfile mutation;
- workflow mutation;
- executable test mutation;
- fuzz target source mutation;
- vector mutation;
- public docs or website mutation;
- qsl-server or qsl-attachments mutation;
- backup, restore, qsl-backup, backup status, backup plan, or qwork mutation;
- public technical paper content;
- no public-readiness claim;
- no production-readiness claim;
- no external-review-complete claim;
- no crypto-complete claim;
- no side-channel-free claim;
- no vulnerability-free claim;
- no bug-free claim;
- no perfect-crypto claim.

## Future validation/marker plan

Common NA-0441 markers:

- `NA0441_NEXT_AUDIT_SCOPE_OK`
- `NA0441_PROVIDER_ERROR_EVIDENCE_CONSUMED_OK`
- `NA0441_PQ_ENCAP_FAILED_CAVEAT_PRESERVED_OK`
- `NA0441_PQ_DECAP_FAILED_TEST_EVIDENCE_CONSUMED_OK`
- `NA0441_NO_RUNTIME_CHANGE_OK`
- `NA0441_NO_DEPENDENCY_CHANGE_OK`
- `NA0441_NO_WORKFLOW_CHANGE_OK`
- `NA0441_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0441_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0441_NO_SECRET_MATERIAL_OK`
- `NA0441_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0441_ONE_READY_INVARIANT_OK`
- `NA0441_NONCE_KEY_RNG_AUDIT_START_OK`

Recommended NA-0441 validation:

- queue and decision helper checks;
- scope guard limited to NA-0441 governance paths;
- link check;
- leak scan;
- overclaim scan;
- PR body preflight and goal-lint;
- root cargo audit;
- nested qsc fuzz lock audit;
- provider-error no-mutation test;
- formal model checks;
- public-safety before and after merge.

## Public claim/external review/website boundary

NA-0440 is internal governance evidence only.

- Formal/model checks are not production-readiness proof.
- Formal/model checks are not public-internet-readiness proof.
- Formal/model checks are not crypto-complete proof.
- Formal/model checks are not side-channel-free proof.
- Formal/model checks are not bug-free proof.
- Formal/model checks are not vulnerability-free proof.
- Formal/model checks are not perfect-crypto proof.
- Formal/model checks are not public technical paper content.
- Cargo audit green is dependency-health evidence only.
- Model checks passing remains bounded evidence, not full correctness proof.
- `pq_encap_failed` defensive branch documentation is not executable coverage.
- `pq_decap_failed` test/adversarial evidence remains bounded to that marker.
- No README, START_HERE, public docs, website, or public technical paper path
  is changed.

## Rejected alternatives

- Claim direct formal/model alignment to `pq_decap_failed`: rejected because no
  model represents `StdCrypto.decap`, malformed pending KEM secret state, qsc
  vault bytes, qsc pending store semantics, or qsc session files.
- Claim executable or formal coverage for `pq_encap_failed`: rejected because
  the branch remains provider-behavior dependent and no model represents
  `StdCrypto.encap` failure.
- Authorize formal/model implementation now: rejected because exact model
  paths and invariants need a separate future scope decision before formal-file
  mutation.
- Select a fuzz/adversarial follow-up now: rejected because NA-0439 already
  integrated the deterministic provider-error test into the adversarial script.
- Stop as ambiguous: rejected because existing formal/model evidence can be
  safely classified as supporting-only with an explicit direct-alignment gap.

## Backup-impact statement

NA-0440 does not run backup or restore. It does not mutate qsl-backup, backup
status files, backup plan files, source lists, rollback subtree paths, timers,
fstab, systemd units, or `/backup/qsl`.

qsl-backup proof remains a boundary check only:

- qsl-backup binary SHA matched the directive's expected value;
- the reviewed scheduled manifest source entry for
  `/home/victor/work/qsl/codex/ops` counted exactly once after using the
  manifest's whitespace-aware line format;
- no off-host backup completion, disaster recovery, restore proof, or backup
  completion claim is made.

Recovered backup-boundary command-shape note:

- Failing command: exact-line grep for `/home/victor/work/qsl/codex/ops` in the
  scheduled manifest.
- Classification: recoverable command-shape mismatch; the manifest indents
  source paths.
- Corrective action: reran a whitespace-aware source-line count.
- Final result: source entry count exactly one.

## Next recommendation

After NA-0440 merges and post-merge public-safety is green, close out NA-0440
and restore exactly:

`NA-0441 -- QSL Nonce / Key / RNG Lifecycle Read-Only Audit Plan`

That successor should consume the provider-error evidence boundaries, preserve
the `pq_encap_failed` caveat, avoid implementation mutation, and produce a
read-only nonce/key/RNG lifecycle findings matrix.
