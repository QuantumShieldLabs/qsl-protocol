Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-06

# NA-0435 qsc Provider Error Path Test Hook / Defensive Branch Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0435 consumed the D278/D279 stop and recovery evidence for the qsc
provider-error no-mutation test attempt. The selected strategy is:

`NARROW_DECAP_ONLY_TEST_AUTHORIZATION_READY`

Selected successor:

`NA-0436 -- QSL qsc pq_decap_failed No-Mutation Test Implementation Harness`

Rationale:

- D278 showed `pq_encap_failed` is not currently externally triggerable through
  qsc CLI/frame APIs with the active `ml-kem` provider.
- qsc frame decode rejects wrong-length A1 KEM public keys before provider
  encapsulation.
- Correct-length malformed public-key byte patterns did not make
  `StdCrypto.encap` fail.
- No safe test-only provider seam is already available in qsc: the handshake
  path constructs `StdCrypto` directly at the provider-error branches.
- D278 showed `pq_decap_failed` appears reachable through malformed pending KEM
  secret evidence.
- Existing qsc integration-test patterns can build mock-vault fixtures, post
  raw relay frames, inspect session files, and mutate mock-vault JSON in
  test-local state without runtime or crypto implementation changes.

This lane does not implement NA-0436. It authorizes a future narrowed test lane
only for `pq_decap_failed`, with the `pq_encap_failed` defensive-branch caveat
preserved. It does not authorize runtime hooks, provider fakes, trait wiring,
crypto/provider behavior changes, dependency changes, Cargo/lockfile changes,
workflow changes, fuzz target changes, vector changes, public-surface changes,
qsl-server/qsl-attachments changes, or backup/local-ops mutation.

## Live NA-0435 scope

Live `NEXT_ACTIONS.md` lists:

`NA-0435 -- QSL qsc Provider Error Path Test Hook / Defensive Branch Authorization Plan`

Status: READY.

Allowed NA-0435 mutation paths:

- `docs/governance/evidence/NA-0435_qsl_qsc_provider_error_path_test_hook_defensive_branch_authorization_plan.md`
- `tests/NA-0435_qsl_qsc_provider_error_path_test_hook_defensive_branch_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Read-only inspection covered:

- qwork proof files under `/srv/qbuild/work/NA-0435/.qwork/`
- D278 response and proof root
- D279 response and NA-0434 recovery evidence
- `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`, and the rolling journal
- NA-0432 and NA-0433 provider-error evidence
- stewardship and project-goal governance canon
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/tests/`
- `qsl/qsl-client/qsc/fuzz/`
- `tools/refimpl/quantumshield_refimpl/src/crypto/`
- `tools/refimpl/quantumshield_refimpl/tests/`
- `formal/`, `inputs/`, and dependency-health evidence

Forbidden current-lane mutation scope:

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
- no public readiness, production readiness, public-internet readiness,
  external-review completion, crypto-complete, side-channel-free, bug-free,
  vulnerability-free, or perfect-crypto claim.

Acceptance criteria:

- D278/D279 stop evidence is consumed.
- `pq_encap_failed` reachability is classified.
- `pq_decap_failed` test feasibility is classified.
- runtime hook/test seam need is accepted or rejected.
- exactly one NA-0436 successor is selected.
- public-claim caveats are explicit.
- no implementation mutation occurs.
- root cargo audit remains green.
- nested qsc fuzz lock audit remains green.
- public-safety is green before merge and after merge.
- exactly one READY item remains.

Stop conditions preserved:

- qwork proof files missing, malformed, stale, or inconsistent;
- qwork, qstart, or qresume run by Codex;
- PR #1138 not merged at the expected lineage;
- queue not READY NA-0435 at start;
- D-0856 absent or D-0857 already present at start;
- root or nested cargo audit not green;
- D278/D279 evidence cannot be consumed;
- successor cannot be selected safely;
- forbidden mutation, backup/restore, qsl-backup mutation, or public overclaim
  occurs;
- more than one READY item exists.

## qwork proof-file verification

Codex read, but did not run, qwork proof files:

- `/srv/qbuild/work/NA-0435/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0435/.qwork/startup.qsl-protocol.json`

Required `.kv` markers were present:

- `startup_result=OK`
- `lane=NA-0435`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0435/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0435`
- `requested_lane_status=READY`

The JSON proof parsed successfully and mirrored the required `.kv` values for
lane, repo, path, head, origin/main, clean-state fields, READY count, queue top,
and requested lane status.

After `git fetch --all --prune`, live `HEAD` and `origin/main` still matched
the qwork proof at `a1c15d8ac377`. PR #1138 was verified MERGED with merge
commit `a1c15d8ac377`.

Proof root:

`/srv/qbuild/tmp/NA0435_provider_error_strategy_authorization_20260607T030026Z`

The qwork proof files and D278/D279 inheritance files were copied into the proof
root.

## D278 / D279 inheritance

D278 response:

`/home/victor/work/qsl/codex/responses/NA0434_20260607T013227Z_D278.md`

D278 proof root:

`/srv/qbuild/tmp/NA0434_provider_error_no_mutation_test_impl_20260607T012707Z`

D279 response:

`/home/victor/work/qsl/codex/responses/NA0434_20260607T023903Z_D279.md`

D279 recovery evidence:

`docs/governance/evidence/NA-0434_qsl_qsc_provider_error_no_mutation_test_implementation_stop_recovery.md`

Inherited facts:

- D278 stopped before repository mutation.
- D278 classification was `PROVIDER_ERROR_NO_MUTATION_RUNTIME_HOOK_NEEDED`.
- The authorized NA-0434 test file remained absent.
- D278 provider probe output recorded:
  - `encap zero: None`
  - `encap ff: None`
  - `encap a5: None`
  - `encap inc: None`
  - `decap short sk: Some(InvalidKey)`
- D279 merged PR #1138 and recorded NA-0434 as BLOCKED, not DONE.
- D279 restored NA-0435 as the sole READY item.
- D279 preserved the need for a new authorization lane before any hook, fake,
  test seam, documentation-only finding, or narrowed executable test.

Provider-error strategy authorization objective:

- decide the next exact strategy for qsc provider-error no-mutation evidence;
- preserve fail-closed/no-mutation evidence honesty;
- avoid claiming `pq_encap_failed` executable coverage when no current qsc
  external path can trigger it;
- authorize only future scope that is exact and defensible.

## Stewardship template application

### Crypto / Protocol Steward

Review question: Should QSL authorize a test-only hook/provider fake, defensive
branch documentation, or a narrowed provider-error test after D278?

Evidence reviewed: D278/D279, NA-0432/NA-0433 evidence,
`qsl/qsl-client/qsc/src/handshake/mod.rs`, qsc tests, `PqKem768`, `StdCrypto`,
provider `pqkem768` tests, and formal roots.

Findings: qsc emits fixed provider-error markers and returns before session
store mutation on the relevant paths. `pq_encap_failed` depends on provider
encapsulation returning an error after A1 decode succeeds. The active provider
accepted the tested correct-length malformed public keys, and wrong-length A1
keys fail frame decode first. qsc currently constructs `StdCrypto` directly and
has no already-available provider injection seam. `pq_decap_failed` can be
exercised by malformed pending KEM secret state and a structurally valid B1.

Risk classification: MEDIUM / EVIDENCE_INCOMPLETE for `pq_encap_failed`;
LOW-to-MEDIUM for a narrowed `pq_decap_failed` test lane because it is
test-only and uses existing fixture patterns.

Public-claim impact: no crypto-complete, side-channel-free, bug-free,
vulnerability-free, perfect-crypto, or external-review completion claim is
supported.

Scope impact: current lane stays governance-only. Future narrowed test scope
can be exact: one qsc integration test file plus governance evidence/testplan,
DECISIONS, TRACEABILITY, and rolling journal paths.

Recommended action: authorize the narrowed `pq_decap_failed` implementation
harness and preserve `pq_encap_failed` as a caveated defensive branch unless a
future exact hook/provider-fake authorization is approved.

### CI / Dependency / Release Health Steward

Review question: Does dependency, nested fuzz-lock, or qsc adversarial health
change the strategy selection?

Evidence reviewed: public-safety on current main, root `cargo audit`, nested
qsc fuzz lock audit, root inverse dependency trees, nested pqcrypto residual
scan, qsc adversarial script/workflow, and NA-0431 through NA-0434 evidence.

Findings: public-safety is green on `a1c15d8ac377`; root and nested cargo
audits are green; `rustls-webpki` is `v0.103.13`; root `ml-kem` is present
through the intended provider path; root pqcrypto package-ID probes report
absence; nested qsc fuzz lock pqcrypto residual scan returned zero matches.
Provider-error adversarial/fuzz coverage remains a future candidate after
deterministic tests.

Risk classification: LOW for dependency health; MEDIUM / EVIDENCE_INCOMPLETE
for provider-error adversarial coverage.

Public-claim impact: cargo audit green is dependency-health evidence only.

Scope impact: no dependency, Cargo, workflow, script, fuzz target, or vector
mutation is authorized by NA-0435.

Recommended action: keep dependency/fuzz state unchanged; rely on required PR
CI and local validations for this governance lane.

### Public Claims / External Review Steward

Review question: Does the strategy decision support public readiness or broad
security claims?

Evidence reviewed: GOALS, Project Charter, stewardship canon, project-goal
canon, D278/D279, and public-claim caveats from prior provider-boundary lanes.

Findings: NA-0435 is internal governance evidence only. It creates no public
docs, website content, README/START_HERE text, public technical paper content,
or external-review package. A future no-mutation test remains bounded qsc
evidence, not broad correctness proof.

Risk classification: CLAIM_BOUNDARY.

Public-claim impact: no public readiness, production readiness,
public-internet readiness, external-review completion, crypto-complete,
side-channel-free, bug-free, vulnerability-free, or perfect-crypto claim is
supported.

Scope impact: no public-surface mutation is authorized.

Recommended action: preserve explicit caveats in D-0857, TRACEABILITY, and any
future NA-0436 PR body.

### Product / Demo / Service Boundary Steward

Review question: Does this qsc/refimpl provider-error strategy affect qshield
demo, qsl-server, qsl-attachments, or service readiness boundaries?

Evidence reviewed: qsc handshake path, refimpl provider path, qsl-server and
qsl-attachments forbidden scope, qshield runtime forbidden scope, and prior
traceability caveats.

Findings: The strategy is confined to qsc/refimpl internal evidence. It does
not mutate qsl-server, qsl-attachments, qshield runtime, website, public docs,
or service deployment surfaces. It does not convert demo/refimpl/test evidence
into service readiness evidence.

Risk classification: LOW / CLAIM_BOUNDARY.

Public-claim impact: no service readiness or public-internet readiness claim is
supported.

Scope impact: no service, demo runtime, website, or sibling-repo mutation is
authorized.

Recommended action: keep NA-0436 qsc-local and test-only if restored.

### Local Ops / Backup / Restore Steward

Review question: Are qwork proof, proof-root, response archive, and backup
boundaries intact?

Evidence reviewed: qwork `.kv` and JSON proof files, D278/D279 response files,
proof roots, rolling journal, qsl-backup SHA, and source-list count.

Findings: qwork proof files were present, valid, and consistent with live repo
state. Codex did not run qwork, qstart, or qresume. qsl-backup SHA matched
`e9ecff3d22ed`; the source-list count for the codex ops path was exactly 1.
Codex did not run backup or restore and did not mutate backup status, backup
plan, qsl-backup, rollback, or `/backup/qsl` paths.

Risk classification: LOW.

Public-claim impact: same-host proof and qsl-backup SHA proof are local
operations evidence only, not off-host backup, restore, key custody, or
disaster recovery proof.

Scope impact: no backup/local-ops mutation is authorized.

Recommended action: keep backup/restore residuals in forward audit only.

## `pq_encap_failed` reachability strategy review

Questions and findings:

- Is `pq_encap_failed` currently externally triggerable through qsc frame/CLI
  APIs? No current path was found.
- Is `pq_encap_failed` only reachable if provider encapsulation returns an
  error? Yes. In the responder A1 path, qsc emits that marker only when
  `StdCrypto.encap(&init.kem_pk)` returns `Err`.
- Does current active provider accept correct-length malformed public-key byte
  patterns? D278 evidence says yes for the tested zero, ff, a5, and incrementing
  patterns because all returned successful encapsulation.
- Is there a safe test-only seam already available? No. The qsc handshake code
  constructs `StdCrypto` directly at the encap/decap branches.
- Would a test-only fake provider require runtime trait wiring? Yes, unless a
  future exact design finds a narrower compile-time-only path. No such path is
  currently established.
- Would a provider mock/fake require changing qsc runtime code? Under the
  current architecture, yes for qsc handshake behavior.
- Would documenting `pq_encap_failed` as a defensive branch be appropriate?
  Yes, but only as current-provider/current-API evidence; it must not overstate
  impossibility for all future providers.
- Could a narrowed `pq_decap_failed`-only implementation improve evidence
  without misrepresenting `pq_encap_failed`? Yes, if the future test names only
  decap coverage and preserves the encap caveat.

Classification:

`ENCAP_FAILED_DEFENSIVE_BRANCH_DOCUMENTATION_CANDIDATE`

Secondary classification:

`ENCAP_FAILED_TEST_ONLY_SEAM_REQUIRES_RUNTIME_CHANGE`

Rejected classifications:

- `ENCAP_FAILED_EXTERNALLY_REACHABLE`
- `ENCAP_FAILED_TEST_ONLY_SEAM_AVAILABLE`
- `ENCAP_FAILED_REACHABILITY_AMBIGUOUS`

## `pq_decap_failed` narrow test feasibility review

Questions and findings:

- Can `pq_decap_failed` be tested through existing APIs without runtime hooks?
  Yes, by creating an initiator pending state through qsc flow, corrupting the
  test-local mock-vault pending KEM secret, then delivering a structurally valid
  B1 response through existing relay-frame patterns.
- Can pre/post session store state be captured with current test helpers? Yes.
  Existing tests inspect `qsp_sessions/<peer>.qsv` paths and session snapshots.
- Can responder pending store state be captured or inferred with current
  helpers? For decap-only, the initiator pending state is the primary state to
  prove unchanged; responder pending is not the target of that branch.
- Would a narrowed `pq_decap_failed`-only no-mutation test be truthful and
  useful? Yes, provided it does not claim `pq_encap_failed` coverage.
- Would it leave `pq_encap_failed` documented as defensive/unreachable? Yes.
- Is this enough for a next implementation lane, or should strategy/docs be
  decided first? NA-0435 is the strategy decision. After this decision, a
  decap-only implementation lane is the next exact successor.

Classification:

`DECAP_ONLY_TEST_IMPLEMENTATION_READY`

Secondary classification:

`DECAP_ONLY_TEST_SCOPE_NEEDS_AUTHORIZATION`

Rejected classifications:

- `DECAP_ONLY_TEST_NOT_USEFUL_WITHOUT_ENCAP`
- `DECAP_ONLY_AMBIGUOUS`

## Test-only hook / provider fake option review

### Option 1 -- Test-only provider hook / fake provider seam

Recommendation: reject for immediate successor.

Evidence: qsc handshake currently constructs `StdCrypto` directly at the
provider-error branches. No existing test-only hook was found.

Exact future paths if later authorized: not selected here. A future
authorization plan would need exact source paths and proof that production
behavior is unchanged.

Risks: runtime wiring drift, accidental production behavior change, and a test
that proves hook behavior rather than active-provider behavior.

Validation requirements: source-level scope proof, production binary behavior
proof, qsc tests, provider tests, formal sanity checks, public-safety, and
dependency audits.

Public-claim caveat: hook evidence would remain bounded test evidence only.

### Option 2 -- qsc trait-level provider injection for tests

Recommendation: reject for immediate successor.

Evidence: the trait exists in the refimpl provider layer, but qsc handshake does
not accept a provider object at the call site. Injection would require runtime
or architecture changes not authorized here.

Exact future paths if later authorized: not selected here; must be exact in a
future authorization lane.

Risks: widened runtime surface and possible provider-boundary confusion.

Validation requirements: same as Option 1, plus explicit trait-boundary review.

Public-claim caveat: no public assurance expansion.

### Option 3 -- Defensive branch documentation / evidence-only finding

Recommendation: partially accept as caveat, but do not select as the only next
successor.

Evidence: D278 supports current-provider/current-API defensive-branch
classification for `pq_encap_failed`.

Exact future paths if selected alone: NA-0436 governance evidence/testplan,
DECISIONS, TRACEABILITY, and rolling journal only.

Risks: documentation-only work would leave the feasible `pq_decap_failed` qsc
no-mutation test unimplemented.

Validation requirements: link/leak/claim scans, dependency audits, public-safety
proof, and exact no-overclaim wording.

Public-claim caveat: no executable coverage overclaim.

### Option 4 -- Narrowed `pq_decap_failed`-only no-mutation test implementation

Recommendation: select.

Evidence: D278 provider probe showed malformed secret-key decap failure, and
existing qsc tests have enough fixture patterns for mock vaults, raw relay
frames, pending/session state inspection, and test-local encrypted-vault JSON
mutation.

Exact future paths:

- `qsl/qsl-client/qsc/tests/handshake_provider_error_no_mutation.rs`
- `docs/governance/evidence/NA-0436_qsl_qsc_pq_decap_failed_no_mutation_test_implementation_harness.md`
- `tests/NA-0436_qsl_qsc_pq_decap_failed_no_mutation_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Risks: future test must avoid claiming `pq_encap_failed` coverage and must not
weaken or bypass runtime fail-closed behavior.

Validation requirements: exact qsc test, root and nested cargo audits, qsc
`send_commit`, provider `pqkem768`, formal model checks, qsc adversarial smoke
when feasible, scope guard, link/leak scans, and public-safety.

Public-claim caveat: bounded no-mutation evidence only.

### Option 5 -- Runtime/crypto behavior change

Recommendation: reject.

Evidence: no runtime or crypto bug requiring behavior change was identified in
this authorization lane.

Exact future paths: none selected.

Risks: out-of-scope protocol/crypto drift.

Validation requirements: would require a new exact authorization lane.

Public-claim caveat: no public assurance expansion.

### Option 6 -- Stop / ambiguity

Recommendation: reject.

Evidence: a safe next strategy can be selected: narrowed decap-only test
implementation with encap caveat preserved.

Exact future paths: not applicable.

Risks: none for current lane if scope remains governance-only.

Validation requirements: current NA-0435 validation bundle.

Public-claim caveat: no overclaim.

## Authorization decision

Primary classification:

`NARROW_DECAP_ONLY_TEST_AUTHORIZATION_READY`

Future mutable paths authorized for NA-0436:

- `qsl/qsl-client/qsc/tests/handshake_provider_error_no_mutation.rs`
- `docs/governance/evidence/NA-0436_qsl_qsc_pq_decap_failed_no_mutation_test_implementation_harness.md`
- `tests/NA-0436_qsl_qsc_pq_decap_failed_no_mutation_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Explicit non-authorization:

- no runtime code changes;
- no crypto/provider behavior changes;
- no dependency changes;
- no Cargo manifest or lockfile changes;
- no workflow changes;
- no fuzz target or vector changes;
- no public-surface changes;
- no qsl-server, qsl-attachments, qshield runtime, backup, restore, qsl-backup,
  status/plan, rollback, or qwork-tool changes.

Validation plan:

- prove queue remains exactly one READY before closeout;
- prove D-0857 exists once and D-0858 remains absent before closeout;
- prove changed paths are limited to the five NA-0435 paths;
- run link, leak, claim-boundary, PR-body, goal-lint, dependency, qsc, refimpl,
  formal, and qsc-adversarial validations as directed;
- require PR checks, including public-safety, before merge;
- after merge, optionally close out NA-0435 only after post-merge public-safety
  is green.

## Successor selection

Selected successor:

`NA-0436 -- QSL qsc pq_decap_failed No-Mutation Test Implementation Harness`

The successor is selected because it advances executable evidence through a
feasible branch without misrepresenting the `pq_encap_failed` branch. The
successor must preserve the encap caveat explicitly.

## Future path/scope bundle

Future NA-0436 allowed paths:

- `qsl/qsl-client/qsc/tests/handshake_provider_error_no_mutation.rs`
- `docs/governance/evidence/NA-0436_qsl_qsc_pq_decap_failed_no_mutation_test_implementation_harness.md`
- `tests/NA-0436_qsl_qsc_pq_decap_failed_no_mutation_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0436 forbidden unless exact later scope authorizes:

- runtime/crypto implementation changes;
- dependency changes;
- Cargo/lockfile changes;
- workflow changes;
- fuzz target source changes;
- vector changes;
- public docs, website, README, or START_HERE changes;
- qsl-server, qsl-attachments, or qshield runtime changes;
- backup, restore, qsl-backup, backup status, backup plan, rollback, or
  `/backup/qsl` changes;
- public claims.

## Future validation/marker plan

Common NA-0436 markers:

- `NA0436_PROVIDER_ERROR_STRATEGY_DECISION_OK`
- `NA0436_NO_RUNTIME_CHANGE_OK`
- `NA0436_NO_DEPENDENCY_CHANGE_OK`
- `NA0436_NO_WORKFLOW_CHANGE_OK`
- `NA0436_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0436_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0436_NO_SECRET_MATERIAL_OK`
- `NA0436_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0436_ONE_READY_INVARIANT_OK`

Narrowed decap-only markers:

- `NA0436_PQ_DECAP_FAILED_NO_MUTATION_TEST_OK`
- `NA0436_PQ_ENCAP_FAILED_CAVEAT_PRESERVED_OK`

Required validation:

- exact narrowed qsc test passes;
- `pq_encap_failed` caveat is preserved;
- root `cargo audit --deny warnings`;
- nested qsc fuzz lock `cargo audit --deny warnings --file`;
- root dependency tree proof for `rustls-webpki`, `ml-kem`, and pqcrypto
  absence;
- `cargo fmt --check`;
- qsc `send_commit`;
- refimpl `pqkem768`;
- formal model checks;
- qsc adversarial smoke when locally feasible, otherwise PR CI proof;
- link check, leak scan, scope guard, PR body preflight, goal-lint, and
  public-safety.

## Public claim/external review/website boundary

This strategy authorization is internal governance evidence only.

It is not:

- production readiness;
- public-internet readiness;
- external-review completion;
- crypto-complete proof;
- side-channel-free proof;
- bug-free proof;
- vulnerability-free proof;
- perfect-crypto proof;
- public technical paper content;
- README, START_HERE, public docs, or website content.

Cargo audit green is dependency-health evidence only. Any future no-mutation
evidence remains bounded evidence for specific qsc paths, not full correctness
proof.

## Rejected alternatives

- Immediate provider hook implementation: rejected because no current seam
  exists and runtime changes are not authorized.
- Immediate provider fake/test seam implementation: rejected because qsc
  constructs `StdCrypto` directly and exact production-behavior proof would
  need a separate authorization lane.
- Defensive branch documentation as the only successor: rejected because
  `pq_decap_failed` is feasible enough to improve executable evidence first.
- Runtime/crypto behavior change: rejected because no runtime or crypto bug was
  identified and the directive forbids it.
- Stop for ambiguity: rejected because a safe narrowed successor can be chosen.

## Backup-impact statement

Codex did not run backup, restore, sudo, qwork, qstart, or qresume. Codex did
not mutate qsl-backup, backup status files, backup plan files, rollback subtree
paths, `/backup/qsl`, systemd, timers, fstab, source lists, or retention state.

qsl-backup read-only proof:

- SHA matched `e9ecff3d22ed`.
- codex ops source-list count was exactly 1.

This is local-ops boundary evidence only. It is not off-host backup, restore,
key custody, backup completion, or disaster recovery proof.

## Next recommendation

After this PR merges and post-merge public-safety is green, close out NA-0435
and restore:

`NA-0436 -- QSL qsc pq_decap_failed No-Mutation Test Implementation Harness`

NA-0436 should implement only the narrowed decap no-mutation test and preserve
the `pq_encap_failed` caveat. A later separate governance lane can decide
whether the encap defensive branch needs standalone documentation or an exact
test-only provider seam authorization.
