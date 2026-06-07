Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-07

# NA-0438 qsc Provider Error Path Fuzz / Adversarial Coverage Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0438 authorizes the next provider-error evidence step after NA-0436 and
NA-0437.

Selected classification:

`PROVIDER_ERROR_ADVERSARIAL_COVERAGE_IMPLEMENTATION_AUTHORIZED`

Selected successor:

`NA-0439 -- QSL qsc Provider Error Path Adversarial Coverage Implementation Harness`

Rationale:

- NA-0436 already provides deterministic, bounded executable no-mutation
  evidence for `pq_decap_failed`.
- NA-0437 documents `pq_encap_failed` as a defensive branch with no executable
  coverage claim.
- Existing fuzz targets do not drive qsc handshake provider-error state.
- A future adversarial harness lane can safely attach the existing
  `handshake_provider_error_no_mutation` test to the qsc adversarial script
  without changing runtime code, crypto code, dependencies, workflows, Cargo
  files, lockfiles, executable tests, fuzz targets, vectors, services, public
  surfaces, or backup/local-ops state.

Future authorized mutable implementation path:

- `scripts/ci/qsc_adversarial.sh`

Future governance paths for NA-0439:

- `docs/governance/evidence/NA-0439_qsl_qsc_provider_error_path_adversarial_coverage_implementation_harness.md`
- `tests/NA-0439_qsl_qsc_provider_error_path_adversarial_coverage_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No implementation mutation occurs in NA-0438.

## Live NA-0438 scope

Live `NEXT_ACTIONS.md` lists:

`NA-0438 -- QSL qsc Provider Error Path Fuzz / Adversarial Coverage Authorization Plan`

Status: READY.

Allowed NA-0438 mutation paths:

- `docs/governance/evidence/NA-0438_qsl_qsc_provider_error_path_fuzz_adversarial_coverage_authorization_plan.md`
- `tests/NA-0438_qsl_qsc_provider_error_path_fuzz_adversarial_coverage_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden current-lane mutation scope:

- no runtime code mutation;
- no crypto code mutation;
- no dependency, Cargo manifest, or lockfile mutation;
- no workflow, script, executable test, fuzz target, or vector mutation;
- no qsl-server, qsl-attachments, qshield runtime, website, public docs,
  README, or START_HERE mutation;
- no qwork, qstart, qresume, or qshell execution or mutation by Codex;
- no backup or restore execution;
- no qsl-backup, backup status, backup plan, rollback subtree, or backup tree
  mutation;
- no public technical paper content;
- no public-readiness claim;
- no production-readiness claim;
- no public-internet-readiness claim;
- no external-review-complete claim;
- no crypto-complete claim;
- no side-channel-free claim;
- no bug-free claim;
- no vulnerability-free claim;
- no perfect-crypto claim.

Acceptance criteria:

- fuzz/adversarial coverage need is classified.
- exact future mutable paths, if any, are future-gated.
- `pq_encap_failed` defensive branch caveat is preserved.
- `pq_decap_failed` test evidence is consumed without overclaim.
- no implementation mutation occurs.
- cargo audit remains green.
- nested qsc fuzz lock audit remains green.
- public-safety is green before merge and after merge.
- exactly one READY item remains.

Stop conditions:

- qwork proof files missing, malformed, stale, or inconsistent;
- qwork, qstart, or qresume run by Codex;
- PR #1144 not merged at the expected lineage;
- queue not READY NA-0438 at start;
- D-0862 absent or D-0863 already present at start;
- root or nested cargo audit not green;
- D283, D281, or D282 evidence cannot be consumed;
- Level-1 stewardship text creates independent steward authority;
- successor cannot be selected safely;
- forbidden mutation, backup/restore, qsl-backup mutation, or public overclaim
  occurs;
- more than one READY item exists.

## qwork proof-file verification

Codex read, but did not run, qwork proof files:

- lane workspace `.qwork` key/value startup proof;
- lane workspace `.qwork` JSON startup proof.

Required `.kv` markers were present:

- `startup_result=OK`
- `lane=NA-0438`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0438/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0438`
- `requested_lane_status=READY`

The JSON proof parsed successfully and mirrored the `.kv` proof for lane, repo,
path, head, origin/main, clean-state fields, READY count, queue top, and
requested lane status.

After `git fetch --all --prune`, live `HEAD` and `origin/main` still matched
the qwork proof at `14493b5e6c1a`. PR #1144 was verified MERGED with merge
commit `14493b5e6c1a`.

Proof root:

`/srv/qbuild/tmp/NA0438_provider_error_fuzz_adversarial_auth_20260607T155305Z`

## NA-0437 / NA-0436 inheritance

NA-0437 evidence consumed:

- D283 response for NA-0437 and its closeout.
- D-0861 defensive branch documentation decision.
- D-0862 closeout decision restoring NA-0438.
- `docs/governance/evidence/NA-0437_qsl_qsc_pq_encap_failed_defensive_branch_documentation_evidence_plan.md`.
- `tests/NA-0437_closeout_restore_na0438_testplan.md`.

Inherited NA-0437 classification:

`PQ_ENCAP_FAILED_DEFENSIVE_BRANCH_DOCUMENTED`

Inherited NA-0437 caveat:

- no executable coverage is claimed for `pq_encap_failed`;
- `pq_encap_failed` depends on provider behavior after exact-length A1 frame
  decode and identity checks;
- future executable coverage would require separate exact authorization for a
  test seam, provider fake, provider-behavior change, or another defensible
  strategy.

NA-0436 evidence consumed:

- D281 response for the `pq_decap_failed` implementation lane.
- D282 response for NA-0436 closeout.
- D-0859 and D-0860.
- `docs/governance/evidence/NA-0436_qsl_qsc_pq_decap_failed_no_mutation_test_implementation_harness.md`.
- `qsl/qsl-client/qsc/tests/handshake_provider_error_no_mutation.rs`.

Inherited NA-0436 proof markers:

- `NA0436_PQ_DECAP_FAILED_MARKER_OK`
- `NA0436_NO_SESSION_MUTATION_ON_DECAP_REJECT_OK`
- `NA0436_PENDING_STORE_NO_MUTATION_ON_DECAP_REJECT_OK`
- `NA0436_PQ_ENCAP_FAILED_CAVEAT_PRESERVED_OK`
- `NA0436_NO_RUNTIME_HOOK_USED_OK`

Interpretation:

- `pq_decap_failed` has bounded deterministic no-mutation evidence.
- `pq_encap_failed` remains defensive-branch documentation only.
- NA-0438 must not convert either inherited item into a broader public or
  executable coverage claim.

## Applicable Stewardship Review

### Crypto / Protocol Steward

Provider-error fuzz/adversarial coverage is evidence-quality work, not runtime
proof. The `pq_decap_failed` test is bounded decap-path evidence. The
`pq_encap_failed` path remains a defensive branch without executable coverage.

### CI / Dependency / Release Health Steward

Root `cargo audit --deny warnings` passed. Nested qsc fuzz lock audit passed.
Current-main public-safety and qsc-adversarial-smoke were green. Cargo audit
green is dependency-health evidence only.

### Public Claims / External Review Steward

NA-0438 makes:

- no crypto-complete claim;
- no side-channel-free claim;
- no vulnerability-free claim;
- no bug-free claim;
- no perfect-crypto claim;
- no public-readiness claim;
- no production-readiness claim;
- no external-review-complete claim.

### Product / Demo / Service Boundary Steward

qsc fuzz/adversarial coverage is internal engineering evidence. NA-0438 makes
no qsl-server readiness claim, no qsl-attachments readiness claim, no qshield
runtime readiness claim, no website readiness claim, and no public-service
readiness claim.

### Local Ops / Backup / Restore Steward

No backup, restore, or local-ops mutation is authorized or performed. qsl-backup
SHA/source-list proof remains boundary evidence only.

## Level-1 stewardship rollout status

Level 1 is active now: concise applicable stewardship summaries are embedded in
relevant evidence docs, including this NA-0438 evidence doc.

Level 2 is future-gated: after the current provider-error chain stabilizes, QSL
may consider a standard evidence-template section for "Applicable Stewardship
Review."

Level 3 is future-gated: later, QSL may consider lightweight lint/helper checks
for high-risk lanes.

Boundaries:

- no separate Directors;
- no independent READY promotion;
- no independent merge authority;
- Lead Director final authority is preserved;
- stewards remain advisory only.

## Current qsc fuzz / adversarial coverage inventory

Existing qsc fuzz targets:

- `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_route_http.rs`
- `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_payload_boundaries.rs`
- `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_vault_envelope.rs`

Existing corpora:

- route HTTP seeds;
- payload boundary JSON seeds;
- vault envelope seeds.

Existing qsc adversarial script phases:

- `cargo +stable test --manifest-path qsl/qsl-client/qsc/Cargo.toml --locked --test adversarial_properties`
- `cargo +stable test --manifest-path qsl/qsl-client/qsc/Cargo.toml --locked --test adversarial_miri`
- `cargo +nightly fuzz run qsc_route_http`
- `cargo +nightly fuzz run qsc_payload_boundaries`
- `cargo +nightly fuzz run qsc_vault_envelope`

Existing qsc-adversarial workflow behavior:

- installs stable and nightly toolchains;
- installs `cargo-fuzz`;
- runs `sh scripts/ci/qsc_adversarial.sh`;
- runs a separate nightly miri job for `adversarial_miri`;
- skips pull-request adversarial jobs for docs-only PRs through the classifier.

Provider-error reachability from current fuzz targets:

- current fuzz targets do not instantiate the qsc handshake state machine;
- current fuzz targets do not create A1/B1/A2 handshake flows;
- current fuzz targets do not mutate pending KEM secret state;
- current fuzz targets do not drive provider encapsulation or decapsulation
  error returns;
- current fuzz targets cannot presently reach `pq_decap_failed`;
- current fuzz targets cannot presently reach `pq_encap_failed`.

Provider-error reachability from current adversarial tests:

- `adversarial_properties` covers route, payload, vault-envelope, bundle, and
  Suite-2 establish properties;
- `adversarial_miri` covers similar parser/helper/establish invariants under
  miri;
- neither adversarial test binary directly asserts `pq_decap_failed` or
  `pq_encap_failed`;
- the separate deterministic integration test
  `handshake_provider_error_no_mutation` covers `pq_decap_failed`.

Local cargo-fuzz availability:

- local `cargo fuzz --version` returned `error: no such command: fuzz`.
- local qsc adversarial script can prove the Rust test phases before the fuzz
  phase, but CI remains the authoritative cargo-fuzz-backed smoke path.

CI status:

- current main public-safety passed on `14493b5e6c1a`;
- current main qsc-adversarial-smoke passed on `14493b5e6c1a`;
- recent workflow-dispatched qsc-adversarial run `27096472686` passed
  qsc-adversarial-smoke and qsc-adversarial-miri on PR #1143 head.

Coverage gaps:

- qsc-adversarial smoke does not yet run the existing
  `handshake_provider_error_no_mutation` integration test.
- fuzz targets do not yet cover handshake provider-error paths.
- `pq_encap_failed` remains blocked as executable coverage by current provider
  behavior and lack of an authorized test seam.

This inventory is read-only and does not implement any fuzz target, adversarial
harness, test, runtime, crypto, dependency, workflow, vector, service, public,
or backup/local-ops change.

## Provider-error coverage gap review

### `pq_decap_failed`

Deterministic test status:

- `pq_decap_failed_reject_does_not_mutate_sessions_or_pending_state` exists.
- It emits the expected marker.
- It proves session store, pending store, vault bytes, and no-A2 emission
  invariants for the reject path.

Fuzz value:

- Fuzzing may be useful later, but current fuzz targets do not have handshake
  state fixtures.
- A new or modified fuzz target would need more scope analysis because it may
  require corpus design, handshake frame construction, pending-state mutation,
  or Cargo target changes.

Adversarial harness value:

- Adding the existing deterministic test to `scripts/ci/qsc_adversarial.sh` is
  a bounded way to make qsc-adversarial smoke carry provider-error evidence.
- It avoids modifying the test source, runtime code, crypto code, dependencies,
  workflows, fuzz targets, or vectors.

Classification:

- `DECAP_FUZZ_COVERAGE_BACKLOG`
- `PROVIDER_ERROR_ADVERSARIAL_COVERAGE_IMPLEMENTATION_AUTHORIZED`

### `pq_encap_failed`

Defensive branch status:

- documented as `PQ_ENCAP_FAILED_DEFENSIVE_BRANCH_DOCUMENTED`;
- no executable coverage claim exists;
- likely not externally triggerable through current qsc external APIs with the
  active provider.

Fuzz/adversarial feasibility:

- current fuzz targets cannot reach it;
- an adversarial script cannot force it without an executable test path;
- future executable coverage would require a test seam/provider fake or another
  separately authorized strategy.

Classification:

- `ENCAP_FUZZ_COVERAGE_BLOCKED_BY_DEFENSIVE_BRANCH`
- `ENCAP_FUZZ_COVERAGE_REQUIRES_TEST_SEAM`

### General provider-error fuzzing

Current parser fuzzing covers route, payload, and vault envelope boundaries.
Those are useful fail-closed parser boundaries, but they are not provider-error
coverage.

Malformed frame, malformed pending-state, or corrupted relay-artifact coverage
could be useful in a future fuzz or deterministic-test lane. The exact target
shape is not yet justified for direct fuzz implementation in NA-0439.

Classification:

- `GENERAL_PROVIDER_ERROR_FUZZ_COVERAGE_SCOPE_NEEDED`
- `NO_FUZZ_COVERAGE_CHANGE_NEEDED_NOW`

Rejected classification:

- `FUZZ_COVERAGE_AMBIGUOUS` is rejected because the current inventory supports
  a bounded adversarial-script successor.

## Options matrix

### Option 1 -- Authorize future fuzz target enhancement for `pq_decap_failed`

Recommendation: reject for immediate NA-0439 implementation; keep as backlog.

Evidence: existing fuzz targets do not instantiate handshake flows or pending
state. A direct fuzz target may require new target paths, corpus design, and
possibly Cargo target updates.

Future paths if later authorized:

- exact fuzz target path to be selected by a future scope lane;
- exact corpus paths to be selected by a future scope lane;
- governance evidence, testplan, DECISIONS, TRACEABILITY, and journal paths.

Risk: noisy target design, larger CI cost, and unclear value beyond the
deterministic decap test without a narrowed plan.

Validation requirements if later selected:

- qsc-adversarial-smoke in CI;
- root and nested cargo audits;
- exact provider-error markers without `pq_encap_failed` overclaim.

Public-claim caveat: fuzz target coverage would remain bounded internal
engineering evidence.

### Option 2 -- Authorize future adversarial harness scenario for `pq_decap_failed`

Recommendation: select.

Evidence: the existing `handshake_provider_error_no_mutation` test already
proves the decap marker and no-mutation behavior. Adding that existing test to
`scripts/ci/qsc_adversarial.sh` makes the qsc-adversarial smoke lane consume the
provider-error test without changing runtime, crypto, dependencies, workflows,
test source, fuzz targets, or vectors.

Future mutable path:

- `scripts/ci/qsc_adversarial.sh`

Future governance paths:

- `docs/governance/evidence/NA-0439_qsl_qsc_provider_error_path_adversarial_coverage_implementation_harness.md`
- `tests/NA-0439_qsl_qsc_provider_error_path_adversarial_coverage_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Risk: increased adversarial smoke time and possible local cargo-fuzz tooling
unavailability. The implementation should be POSIX shell compatible and rely on
CI for cargo-fuzz-backed smoke when local `cargo fuzz` is unavailable.

Validation requirements:

- exact qsc provider-error integration test passes;
- qsc adversarial script passes in CI;
- root and nested cargo audits remain green;
- no `pq_encap_failed` executable coverage overclaim.

Public-claim caveat: adversarial smoke coverage remains bounded internal
engineering evidence.

### Option 3 -- Do not add fuzz/adversarial coverage now; move to formal/model alignment

Recommendation: reject for immediate successor.

Evidence: formal/model alignment remains useful, but qsc-adversarial smoke can
consume the existing decap test with a small exact script change first.

Risk: deferring the adversarial-script attachment would leave provider-error
evidence outside the adversarial smoke lane longer than necessary.

Validation requirements if later selected:

- formal model checks;
- evidence doc/testplan only unless exact model mutation is authorized.

Public-claim caveat: no public-readiness proof and no crypto-complete proof.

### Option 4 -- Authorize provider-error test seam / provider fake before fuzz

Recommendation: reject now; future-gate.

Evidence: this is only needed for executable `pq_encap_failed` coverage. NA-0437
already documents that path as defensive under current evidence.

Risk: test seams or provider fakes can touch runtime, crypto, or provider
abstractions and require a separate exact authorization lane.

Validation requirements if later selected:

- exact seam path authorization;
- fail-closed behavior tests;
- DECISIONS and TRACEABILITY updates;
- no public claim expansion.

Public-claim caveat: a seam would be test evidence only.

### Option 5 -- Move to next audit domain: Nonce / Key / RNG lifecycle

Recommendation: reject for immediate successor; keep as later candidate.

Evidence: provider-error chain still has a small bounded adversarial-script
integration step available.

Risk: starting the next audit domain now would leave the adversarial smoke
integration opportunity unresolved.

Validation requirements if later selected:

- read-only audit evidence;
- root/nested dependency health proof;
- public-safety proof.

Public-claim caveat: audit findings would be internal evidence only.

### Option 6 -- Stop / ambiguity

Recommendation: reject.

Evidence: the current inventory supports a precise adversarial-script successor.

Risk: stopping would delay a safe, exact, governance-bounded next lane.

## Authorization decision

Primary classification:

`PROVIDER_ERROR_ADVERSARIAL_COVERAGE_IMPLEMENTATION_AUTHORIZED`

Future NA-0439 may mutate:

- `scripts/ci/qsc_adversarial.sh`
- `docs/governance/evidence/NA-0439_qsl_qsc_provider_error_path_adversarial_coverage_implementation_harness.md`
- `tests/NA-0439_qsl_qsc_provider_error_path_adversarial_coverage_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0439 may inspect read-only:

- `qsl/qsl-client/qsc/tests/handshake_provider_error_no_mutation.rs`
- `qsl/qsl-client/qsc/fuzz/`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `scripts/ci/qsc_adversarial.sh`
- `.github/workflows/qsc-adversarial.yml`
- `formal/`
- `inputs/`
- relevant evidence docs.

Future NA-0439 must not mutate:

- runtime code;
- crypto code;
- dependencies;
- Cargo manifests;
- lockfiles;
- workflows;
- executable tests;
- fuzz target source;
- vectors;
- qsl-server;
- qsl-attachments;
- qshield runtime;
- website;
- public docs;
- README;
- START_HERE;
- qwork, qstart, qresume, or qshell;
- backup scripts, backup status files, backup plan files, rollback subtree, or
  backup tree paths.

Future validation:

- `cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture`
- `sh scripts/ci/qsc_adversarial.sh` locally if cargo-fuzz is available, or
  record local cargo-fuzz unavailability and require PR CI qsc-adversarial-smoke
  success;
- root `cargo audit --deny warnings`;
- nested qsc fuzz lock `cargo audit --deny warnings --file`;
- dependency tree/residual probes;
- `cargo fmt --check`;
- public-safety before merge and after merge;
- no public overclaim scan;
- exactly one READY item.

## Successor selection

Selected successor:

`NA-0439 -- QSL qsc Provider Error Path Adversarial Coverage Implementation Harness`

Reason:

- exact future mutable implementation path is safe and narrow:
  `scripts/ci/qsc_adversarial.sh`;
- deterministic `pq_decap_failed` evidence can be added to the adversarial
  smoke lane without changing the test source;
- `pq_encap_failed` remains caveated and blocked from executable coverage until
  a separate seam/provider-fake authorization exists;
- no fuzz target implementation is necessary now.

Do not implement NA-0439 in this lane.

## Future path/scope bundle

Future NA-0439 allowed mutation paths:

- `scripts/ci/qsc_adversarial.sh`
- `docs/governance/evidence/NA-0439_qsl_qsc_provider_error_path_adversarial_coverage_implementation_harness.md`
- `tests/NA-0439_qsl_qsc_provider_error_path_adversarial_coverage_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0439 read-only paths:

- `qsl/qsl-client/qsc/fuzz/`
- `scripts/ci/qsc_adversarial.sh`
- `.github/workflows/qsc-adversarial.yml`
- `qsl/qsl-client/qsc/tests/handshake_provider_error_no_mutation.rs`
- qsc provider-error paths;
- `formal/`
- `inputs/`
- relevant evidence docs.

Future NA-0439 forbidden scope unless separate exact authorization changes it:

- no runtime implementation mutation;
- no crypto implementation mutation;
- no dependency mutation;
- no Cargo manifest mutation;
- no lockfile mutation;
- no workflow mutation;
- no executable test mutation;
- no fuzz target source mutation;
- no vector mutation;
- no public docs or website mutation;
- no qsl-server or qsl-attachments mutation;
- no backup, restore, qsl-backup, backup status, backup plan, rollback subtree,
  or backup tree mutation;
- no public technical paper content;
- no public-readiness claim;
- no production-readiness claim;
- no external-review-complete claim;
- no crypto-complete claim;
- no side-channel-free claim;
- no bug-free claim;
- no vulnerability-free claim;
- no perfect-crypto claim.

## Future validation/marker plan

Common NA-0439 markers:

- `NA0439_PROVIDER_ERROR_FUZZ_SCOPE_DECISION_OK`
- `NA0439_PQ_ENCAP_FAILED_CAVEAT_CONSUMED_OK`
- `NA0439_PQ_DECAP_FAILED_TEST_EVIDENCE_CONSUMED_OK`
- `NA0439_NO_RUNTIME_CHANGE_OK`
- `NA0439_NO_DEPENDENCY_CHANGE_OK`
- `NA0439_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0439_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0439_NO_SECRET_MATERIAL_OK`
- `NA0439_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0439_ONE_READY_INVARIANT_OK`

Adversarial successor markers:

- `NA0439_ADVERSARIAL_HARNESS_SCOPE_OK`
- `NA0439_QSC_ADVERSARIAL_SMOKE_REQUIRED_OK`

Expected future script behavior:

- run the existing provider-error no-mutation test in the qsc adversarial smoke
  script before or near existing stable adversarial test phases;
- preserve the existing fuzz target phases;
- use POSIX shell-compatible script syntax;
- preserve cargo-fuzz CI installation behavior in the workflow without changing
  workflow files.

## Public claim/external review/website boundary

NA-0438 is internal engineering authorization evidence only.

- It is not production readiness.
- It is not public-internet readiness.
- It is not crypto-complete proof.
- It is not side-channel-free proof.
- It is not bug-free proof.
- It is not vulnerability-free proof.
- It is not perfect-crypto proof.
- It is not a public technical paper.
- It is not external-review-complete evidence.
- It does not update README.
- It does not update START_HERE.
- It does not update website content.
- It does not update public docs.
- It does not create a public security claim.
- Cargo audit green is dependency-health evidence only.
- Fuzz/adversarial coverage, even if later improved, remains bounded evidence
  and not full correctness proof.
- `pq_encap_failed` defensive branch documentation is not executable coverage.

## Rejected alternatives

- Direct `pq_decap_failed` fuzz target implementation is rejected now because
  existing fuzz targets do not carry handshake state and exact target/corpus
  design needs a separate scope decision if revisited.
- `pq_encap_failed` executable coverage is rejected now because current
  evidence shows it is provider-behavior dependent and requires a test seam or
  provider fake before it can be forced.
- Workflow mutation is rejected now and not needed for the selected successor.
- Runtime/crypto/provider mutation is rejected because no runtime or crypto bug
  is established by this evidence.
- Moving immediately to formal/model alignment is rejected because the
  adversarial script can first consume the existing decap test with a bounded
  exact path.
- Moving immediately to the Nonce / Key / RNG lifecycle audit is rejected for
  the same reason.

## Backup-impact statement

NA-0438 has no backup impact beyond governance evidence tracking in this repo.
Codex did not run backup. Codex did not run restore. Codex did not mutate
qsl-backup, backup status files, backup plan files, rollback subtree paths, or
backup tree paths.

The qsl-backup SHA matched the expected value and the source-list inclusion
count remained exactly one. That proof is boundary evidence only:

- no off-host backup claim;
- no restore claim;
- no disaster-recovery claim;
- no backup-complete claim.

## Next recommendation

Open and complete:

`NA-0439 -- QSL qsc Provider Error Path Adversarial Coverage Implementation Harness`

The future implementation should be limited to adding the existing
`handshake_provider_error_no_mutation` test command to
`scripts/ci/qsc_adversarial.sh`, plus governance evidence/testplan, D/trace,
and rolling journal updates. It must preserve the `pq_encap_failed` caveat and
must not mutate runtime, crypto, dependencies, Cargo files, lockfiles,
workflows, executable tests, fuzz targets, vectors, services, public surfaces,
or backup/local-ops state.
