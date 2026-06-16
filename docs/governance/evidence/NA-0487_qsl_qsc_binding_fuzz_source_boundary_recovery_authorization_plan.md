Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-16

# NA-0487 QSL qsc Binding Fuzz Source-Boundary Recovery Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0487 remains READY, but D347 proved the current implementation scope is too
narrow to produce truthful semantic qsc binding fuzz evidence. The current qsc
library exposes only `adversarial` parser helpers and `envelope`; the real
A1/B1/A2, suite, replay, transcript, signature, KEM, and stale-public-record
reject paths live behind qsc binary-module boundaries.

This recovery consumes D347, inventories the current qsc source boundary, and
selects the minimal future recovery classification:

`SOURCE_BOUNDARY_RECOVERY_MINIMAL_READY`

The next NA-0487 implementation should use a narrow cfg-gated qsc
library/source-boundary expansion, not a fake helper-local oracle. The future
helper must be compiled only under exact cfg `qsc_binding_fuzz_helper`, must not
alter no-cfg production behavior, and must route through real qsc handshake and
identity reject-path routines.

No implementation mutation occurs in this directive. This directive does not
mutate qsc source, fuzz targets, qsc fuzz Cargo metadata, scripts, workflows,
dependencies, lockfiles, vectors, corpora, formal models, refimpl code, service
repos, public docs, backup state, restore state, or qsl-backup.

## Live NA-0487 recovery scope

Startup READY item:

`NA-0487 -- QSL qsc Binding Fuzz Helper and Target Implementation Harness`

This recovery PR is governance-only. Allowed mutation paths are limited to:

- `docs/governance/evidence/NA-0487_qsl_qsc_binding_fuzz_source_boundary_recovery_authorization_plan.md`
- `tests/NA-0487_qsl_qsc_binding_fuzz_source_boundary_recovery_authorization_testplan.md`
- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden mutation scope preserved:

- qsc source mutation;
- qsc fuzz target, qsc fuzz Cargo, qsc fuzz lockfile, and corpus mutation;
- qsc-adversarial script and workflow mutation;
- dependency and lockfile mutation;
- vector/input, formal, and refimpl mutation;
- qsl-server, qsl-attachments, qshield runtime, qshield-cli, service, website,
  public-doc, README, and START_HERE mutation;
- backup, restore, qsl-backup, rollback, backup status, backup plan, and backup
  tree mutation;
- no public-readiness or completion-claim expansion.

## qwork proof-file verification

Codex read the operator-provided qwork proof files and did not run qwork,
qstart, or qresume.

Verified proof files:

- `/srv/qbuild/work/NA-0487/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0487/.qwork/startup.qsl-protocol.json`

Required fields matched:

- `startup_result=OK`;
- `lane=NA-0487`;
- `repo=qsl-protocol`;
- path `/srv/qbuild/work/NA-0487/qsl-protocol`;
- `head_equals_origin_main=yes`;
- clean worktree, index, and untracked state;
- `ready_count=1`;
- `queue_top_ready=NA-0487`;
- `requested_lane_status=READY`.

Freshness proof:

- proof HEAD and live pre-fetch HEAD both began at `9a321cee5924`;
- proof `origin/main` and live pre-fetch `origin/main` both began at
  `9a321cee5924`;
- fetch did not change `origin/main`;
- PR #1243 was verified merged with merge commit `9a321cee5924`;
- `origin/main` equals or descends from `9a321cee5924`.

Startup queue/decision proof:

- READY_COUNT 1;
- READY NA-0487;
- NA-0486 DONE;
- D-0960 exists once;
- D-0961 exists once;
- D-0962 absent at startup;
- duplicate decision count 0.

Startup health proof:

- current main public-safety completed success;
- root `cargo audit --deny warnings` completed success;
- nested qsc fuzz lock audit completed success;
- `rustls-webpki` and `ml-kem` inverse dependency probes completed success;
- qsl-backup SHA matched the expected installed helper, recorded with short
  SHA `e9ecff3d22ed`;
- latest scheduled same-host manifest included the Codex ops source path
  exactly once.

## D347 stop analysis

D347 attempted the NA-0487 implementation harness selected by D-0960. It
verified startup proof, queue state, D-0960/D-0961 state, public-safety,
dependency health, qsc adversarial script markers, formal checks, vector
manifest JSON, qsc binding negative tests, and inherited qsc/refimpl tests
before stopping.

Exact stop condition:

- the selected helper path could not reach real qsc semantic handshake reject
  paths within the then-allowed NA-0487 mutation scope.

Why helper-only implementation would become a fake oracle:

- a helper implemented only under `qsl/qsl-client/qsc/src/adversarial/` could
  parse or classify bytes, but the current qsc library does not expose the
  private `handshake/mod.rs` routines that perform suite admission, transcript
  MAC/hash checks, ML-DSA signature checks, KEM encap/decap handling, replay
  checks, identity-pin checks, and session no-mutation behavior;
- duplicating those decisions in a helper-local implementation would test a new
  simplified model, not the real qsc reject paths;
- such an oracle would dilute fail-closed evidence and could drift from qsc
  behavior.

Why the current qsc library boundary is insufficient:

- `qsl/qsl-client/qsc/src/lib.rs` exports only `pub mod adversarial;` and
  `pub mod envelope;`;
- existing fuzz targets depend on `qsc = { path = ".." }` and call only public
  library APIs;
- the qsc binary root in `src/main.rs` includes `handshake`, `identity`,
  `transport`, `vault`, `protocol_state`, and related modules privately;
- the real binding reject routines are private or crate-internal within that
  binary module tree.

Why the previous allowed paths were insufficient:

- D347 allowed `adversarial/binding_fuzz.rs`, `adversarial/mod.rs`, the fuzz
  target, qsc fuzz `Cargo.toml`, qsc-adversarial script, and governance paths;
- it did not allow `src/lib.rs`, `src/handshake/mod.rs`,
  `src/identity/mod.rs`, or any equivalent source-boundary path;
- therefore the helper could not be wired to real qsc internals without
  unauthorized mutation.

Forbidden paths that would have been needed:

- `qsl/qsl-client/qsc/src/lib.rs`;
- `qsl/qsl-client/qsc/src/handshake/mod.rs`;
- `qsl/qsl-client/qsc/src/identity/mod.rs` if stale-public-record or trusted-pin
  checks need cfg-gated identity helpers;
- possibly qsc binary-root support if implementation proves the module graph
  cannot be exposed narrowly through `lib.rs`.

Why NA-0488 must not start yet:

- NA-0487 remains the sole READY item and has not merged or closed;
- starting NA-0488 would reorder the queue and leave the binding fuzz recovery
  unresolved;
- D347 did not implement or abandon NA-0487, it stopped for source-boundary
  recovery.

Why NA-0487 remains READY:

- the lane still advances G4 by improving verification evidence, and G1/G2/G3
  by preserving fail-closed binding semantics as the target of verification;
- the blocker is exact future source-boundary authorization, not a reason to
  mark NA-0487 DONE;
- exactly one READY item remains mandatory.

## NA-0486 inheritance

D-0960 selected an in-process qsc adversarial helper plus semantic fuzz target
as the first implementation shape. It rejected process harnessing as the first
implementation because it is slower and filesystem/process-heavy, and it
rejected target-local simplified oracles as insufficient.

Inherited future artifacts from NA-0486:

- `qsl/qsl-client/qsc/src/adversarial/binding_fuzz.rs`;
- `qsl/qsl-client/qsc/src/adversarial/mod.rs`;
- `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs`;
- `qsl/qsl-client/qsc/fuzz/Cargo.toml`;
- `scripts/ci/qsc_adversarial.sh`;
- NA-0487 evidence/testplan/decision/traceability/journal paths.

Inherited constraints:

- helper is test/fuzz/adversarial only;
- helper must call real qsc reject paths;
- normal no-cfg builds must not expose helper APIs or change runtime behavior;
- no secret material may be emitted;
- no checked-in corpus is selected for the first implementation;
- no workflow, dependency, or lockfile mutation is selected.

## Current qsc source-boundary inventory

| Path | Library-exported today | Binary-only today | Needed for real reject path | Can be exposed under cfg without production behavior drift | Dependency/Cargo/lockfile change | Fake-oracle risk if not exposed | Candidate future mutation path |
|---|---|---|---|---|---|---|---|
| `qsl/qsl-client/qsc/src/lib.rs` | yes, but only `adversarial` and `envelope` | no | yes | likely, if additions are behind `qsc_binding_fuzz_helper` | no | yes | yes |
| `qsl/qsl-client/qsc/src/main.rs` | no | yes | indirect binary-root evidence only | no, not selected for mutation | no | no | no |
| `qsl/qsl-client/qsc/src/adversarial/mod.rs` | yes | no | yes, as helper export gate | likely | no | yes | yes |
| `qsl/qsl-client/qsc/src/adversarial/binding_fuzz.rs` | no, absent today | no | yes, as helper facade | likely | no | yes | yes |
| `qsl/qsl-client/qsc/src/adversarial/payload.rs` | yes | no | no | n/a | no | no | no |
| `qsl/qsl-client/qsc/src/adversarial/route.rs` | yes | no | no | n/a | no | no | no |
| `qsl/qsl-client/qsc/src/adversarial/vault_format.rs` | yes | no | no | n/a | no | no | no |
| `qsl/qsl-client/qsc/src/envelope/` | yes | no | no | n/a | no | no | no |
| `qsl/qsl-client/qsc/src/handshake/mod.rs` | no | yes | yes | likely, if only a cfg-gated semantic driver/classifier is added | no | yes | yes |
| `qsl/qsl-client/qsc/src/identity/mod.rs` | no | yes | yes for stale-public-record/trusted-pin classes | unclear but acceptable only if exact cfg-gated helper is needed | no | yes for stale-public-record classes | yes, conditional |
| `qsl/qsl-client/qsc/src/cmd/mod.rs` | no | yes | only supplies `HandshakeSuiteMode` today | prefer no mutation; lib boundary may avoid or re-use under cfg | no | no | no |
| `qsl/qsl-client/qsc/src/transport/mod.rs` | no | yes | no for in-memory helper; yes for process harness | not selected | no | no | no |
| `qsl/qsl-client/qsc/fuzz/Cargo.toml` | n/a | n/a | yes for target registration | n/a | no dependency or lockfile change selected | no | yes |
| `qsl/qsl-client/qsc/fuzz/fuzz_targets/` | n/a | n/a | yes for semantic fuzz target | n/a | no | no | yes |
| `qsl/qsl-client/qsc/tests/kem_signature_transcript_binding_negative.rs` | no | test-only | supporting real-reject proof | no mutation selected | no | no | no |

Source-boundary findings:

- Current fuzz targets are parser-boundary targets: route HTTP, payload
  boundaries, and vault envelope.
- Current semantic binding tests are executable process/temp-root tests; they
  mutate captured A1/B1/A2 frames and assert reject/no-session/no-secret-output
  behavior.
- Real qsc reject markers include suite admission rejects, context mismatch,
  key-context mismatch, replay, bad transcript, bad confirm, signature invalid,
  KEM decap/encap failures, identity mismatch, and session restore/store
  failures.
- The next implementation must avoid reimplementing those decisions in the fuzz
  target or helper facade.

## Recovery option review

### Option 1 -- Minimal qsc library/source-boundary expansion

Selected.

Evidence:

- it is the only option that preserves the NA-0486 in-process libFuzzer shape
  while reaching real qsc reject routines;
- adding `src/lib.rs` and narrow cfg-gated `handshake/mod.rs` access fixes the
  exact D347 blocker;
- `identity/mod.rs` may be required only for stale-public-record/trusted-pin
  classes;
- existing qsc dependencies are already present in `qsl/qsl-client/qsc/Cargo.toml`,
  so no dependency or lockfile mutation is selected.

Future allowed paths:

- `qsl/qsl-client/qsc/src/lib.rs`;
- `qsl/qsl-client/qsc/src/adversarial/mod.rs`;
- `qsl/qsl-client/qsc/src/adversarial/binding_fuzz.rs`;
- `qsl/qsl-client/qsc/src/handshake/mod.rs`;
- `qsl/qsl-client/qsc/src/identity/mod.rs` only if required for
  stale-public-record/trusted-pin reachability;
- `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs`;
- `qsl/qsl-client/qsc/fuzz/Cargo.toml`;
- `scripts/ci/qsc_adversarial.sh`;
- NA-0487 implementation evidence/testplan/decision/traceability/journal paths.

Future forbidden paths:

- dependencies, lockfiles, workflows, corpora, vectors/inputs, formal models,
  refimpl, services, public docs, qshield, qsl-server, qsl-attachments,
  qsl-backup, backup/restore, and public-claim surfaces.

Validation requirements:

- no-cfg production behavior proof;
- cfg-on helper compile proof;
- helper real reject-path proof for A1, B1, A2, suite-confusion, replay, and
  stale-public-record classes;
- no secret/private material output proof;
- qsc fuzz target registration proof;
- qsc-adversarial script syntax and smoke proof;
- inherited qsc/refimpl/formal/vector checks.

Public-claim caveats:

- no public-readiness claim;
- no crypto-complete claim;
- no fuzz-complete claim;
- no vector-complete claim;
- no replay-proof claim;
- no downgrade-proof claim;
- no side-channel-free claim.

Risks:

- P0: helper accidentally duplicates protocol decisions instead of calling qsc
  routines; mitigation is mandatory real-reject-path proof.
- P1: cfg-gated source boundary accidentally changes normal library or binary
  API; mitigation is mandatory no-cfg diff/compile proof.
- P2: implementation discovers additional module-root dependencies; mitigation
  is STOP/split rather than widening scope silently.

### Option 2 -- Process-harness fuzz/adversarial approach

Rejected for the next implementation, retained as fallback if Option 1 proves
too broad.

Evidence:

- process tests already prove semantic cases, but a process harness is slow,
  filesystem-heavy, and less suitable for sustained libFuzzer execution;
- it avoids internal exposure but would likely be smoke/adversarial validation,
  not high-throughput in-process semantic fuzz.

Future allowed paths if later selected:

- qsc executable harness docs/tests only under an exact later directive;
- no source-boundary mutation would be needed.

Future forbidden paths:

- same forbidden paths as Option 1 unless an exact later directive changes
  scope.

Validation requirements:

- temp-root isolation proof;
- no secret output proof;
- bounded runtime proof;
- process failure classification proof.

Public-claim caveats:

- represent as process/adversarial smoke evidence, not semantic libFuzzer
  coverage.

Risks:

- P0: flakiness or filesystem state hides reject-path truth.
- P1: slow execution prevents meaningful fuzz depth.
- P2: more CI time and more operational artifacts.

### Option 3 -- Parser/metadata-only fuzz target

Rejected.

Evidence:

- it avoids source-boundary work but repeats the parser-only limitation already
  recorded by NA-0484/NA-0485;
- it would require explicit downgrade of NA-0487 evidence to parser/metadata
  fuzz, not semantic binding reject-path evidence.

Future allowed paths:

- parser-only fuzz target and metadata docs under a later exact directive.

Future forbidden paths:

- any claim that parser/metadata fuzz proves A1/B1/A2 semantic binding reject
  behavior.

Validation requirements:

- parser-only caveat proof and no semantic claim proof.

Risks:

- P0: overclaiming semantic assurance.
- P1: consumes queue time without resolving D347.
- P2: duplicate coverage with existing qsc fuzz targets.

### Option 4 -- Vector-consumer tests next

Deferred.

Evidence:

- deterministic vector-consumer tests are useful, but they do not replace the
  selected semantic fuzz helper goal;
- NA-0483 and inherited tests already provide vector/test support.

Future allowed paths:

- vector consumer test paths only under a later exact directive.

Future forbidden paths:

- qsc fuzz/helper implementation claims.

Validation requirements:

- manifest JSON validation and test no-mutation proof.

Risks:

- P0: queue drift away from the current READY item.
- P1: deterministic tests may miss fuzz-discovered parser/semantic boundary
  cases.
- P2: more maintenance if not tied to a clear consumer contract.

### Option 5 -- Abort fuzz chain and move to side-channel / secret-material assurance

Rejected.

Evidence:

- D347 exposed a solvable source-boundary authorization gap, not evidence that
  qsc binding fuzz is too risky or not actionable;
- side-channel and secret-material assurance remain important residuals but do
  not supersede the current READY item.

Future allowed paths:

- none for this directive.

Future forbidden paths:

- abandoning NA-0487 without a later Director decision.

Validation requirements:

- n/a.

Risks:

- P0: leaving a known verification gap unresolved.
- P1: premature queue movement.
- P2: loss of traceability from NA-0484 through NA-0487.

## Source-boundary safety criteria

Option 1 is authorized only if all criteria below are met by the future
implementation:

- helper is behind exact cfg `qsc_binding_fuzz_helper`;
- normal builds do not export or compile the semantic fuzz helper;
- no production behavior drift;
- no public API expansion in normal builds;
- no secret/private material output;
- no unsafe code;
- no dependency changes;
- no lockfile changes;
- no workflow changes;
- helper routes through real qsc parsing, reject, binding, transcript,
  signature, KEM, replay, and identity/stale-record paths;
- helper does not duplicate protocol semantics as a fake oracle;
- helper returns only bounded classifications, markers, or enum-style outcomes;
- helper supports A1, B1, A2, suite-confusion, replay, and stale-public-record
  categories;
- fuzz target uses helper and does not read manifest JSON at runtime;
- no checked-in corpus.

Selected classification:

`SOURCE_BOUNDARY_RECOVERY_MINIMAL_READY`

Rejected classifications:

- `SOURCE_BOUNDARY_RECOVERY_PROCESS_HARNESS_READY`;
- `SOURCE_BOUNDARY_RECOVERY_PARSER_ONLY_READY`;
- `SOURCE_BOUNDARY_RECOVERY_VECTOR_CONSUMER_NEXT`;
- `SOURCE_BOUNDARY_RECOVERY_TOO_RISKY`;
- `SOURCE_BOUNDARY_RECOVERY_AMBIGUOUS`.

## Authorization decision

D-0962 authorizes revising NA-0487 from the previous helper/target-only future
scope to a minimal source-boundary recovery implementation scope.

Decision summary:

- D347 stop evidence is consumed;
- source-boundary problem is accepted;
- current qsc library boundary is inventoried;
- recovery options are reviewed;
- Option 1 is selected;
- selected classification is `SOURCE_BOUNDARY_RECOVERY_MINIMAL_READY`;
- NA-0487 remains READY;
- NA-0488 is not restored;
- no implementation mutation occurs in this directive;
- no qsc source mutation occurs in this directive;
- no qsc fuzz target/Cargo/script/workflow mutation occurs in this directive;
- no dependency or lockfile mutation occurs;
- no backup or restore occurs;
- exactly one READY item remains mandatory.

## Revised future NA-0487 scope bundle

Future implementation paths:

- `qsl/qsl-client/qsc/src/lib.rs`;
- `qsl/qsl-client/qsc/src/adversarial/mod.rs`;
- `qsl/qsl-client/qsc/src/adversarial/binding_fuzz.rs`;
- `qsl/qsl-client/qsc/src/handshake/mod.rs`;
- `qsl/qsl-client/qsc/src/identity/mod.rs` only if required for
  stale-public-record/trusted-pin reachability;
- `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs`;
- `qsl/qsl-client/qsc/fuzz/Cargo.toml`;
- `scripts/ci/qsc_adversarial.sh`;
- `docs/governance/evidence/NA-0487_qsl_qsc_binding_fuzz_helper_and_target_implementation_harness.md`;
- `tests/NA-0487_qsl_qsc_binding_fuzz_helper_and_target_implementation_testplan.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Future still forbidden unless a later exact directive changes scope:

- dependency mutation;
- `Cargo.lock` mutation;
- `qsl/qsl-client/qsc/fuzz/Cargo.lock` mutation;
- workflow mutation;
- vector/input/corpus mutation;
- refimpl mutation;
- formal mutation;
- qsl-server, qsl-attachments, qshield runtime, qshield-cli mutation;
- backup, restore, qsl-backup mutation;
- public docs/website mutation;
- production/public readiness claims.

Future implementation must STOP if it needs additional qsc source mutation
outside the exact listed source-boundary paths.

## Revised validation / marker plan

Required future local validation:

- `git diff --check`;
- exact scope guard;
- link-check;
- leak-scan;
- added-line overclaim scan;
- classifier;
- PR body preflight;
- goal-lint;
- `cargo fmt --check`;
- `sh -n scripts/ci/qsc_adversarial.sh`;
- `bash -n scripts/ci/qsc_adversarial.sh`;
- qsc no-cfg compile/test proof showing no helper export or production drift;
- cfg-on helper compile proof under `qsc_binding_fuzz_helper`;
- semantic helper marker proof for A1, B1, A2, suite-confusion, replay, and
  stale-public-record categories;
- no checked-in corpus proof;
- no secret/private material output proof;
- internal negative binding vector manifest JSON validation;
- formal model checks;
- qsc binding negative test;
- inherited qsc provider-RNG/key-lifecycle/provider-error tests as needed;
- stable qsc `send_commit`;
- refimpl signature provider-boundary and `pqkem768`;
- root cargo audit and nested qsc fuzz lock audit.

Future markers should include bounded internal markers such as:

- `NA0487_SOURCE_BOUNDARY_CFG_HELPER_ONLY_OK`;
- `NA0487_NO_CFG_PRODUCTION_BEHAVIOR_OK`;
- `NA0487_REAL_REJECT_PATH_A1_OK`;
- `NA0487_REAL_REJECT_PATH_B1_OK`;
- `NA0487_REAL_REJECT_PATH_A2_OK`;
- `NA0487_REAL_REJECT_PATH_SUITE_CONFUSION_OK`;
- `NA0487_REAL_REJECT_PATH_REPLAY_OK`;
- `NA0487_REAL_REJECT_PATH_STALE_PUBLIC_RECORD_OK`;
- `NA0487_NO_FAKE_ORACLE_OK`;
- `NA0487_NO_SECRET_OUTPUT_OK`;
- `NA0487_NO_CORPUS_OK`.

## Public claim boundary

This recovery is internal governance evidence only.

- No public-readiness claim is made.
- No production-readiness claim is made.
- No external-review-complete claim is made.
- No crypto-complete claim is made.
- No fuzz-complete claim is made.
- No vector-complete claim is made.
- No KEM-complete claim is made.
- No signature-complete claim is made.
- No qsc-refimpl-equivalence-complete claim is made.
- No replay-proof claim is made.
- No downgrade-proof claim is made.
- No side-channel-free claim is made.
- No vulnerability-free claim is made.
- No bug-free claim is made.
- No perfect-crypto claim is made.

Cargo audit and public-safety evidence remain dependency/protection health
evidence only.

## Backup impact

Backup impact: none.

This directive used read-only qsl-backup boundary checks only. Codex did not run
backup, restore, qsl-backup, rollback, qstart, qresume, qwork, or qshell.

The installed qsl-backup helper SHA matched the expected helper, recorded in
this document with short SHA `e9ecff3d22ed`. The latest scheduled same-host
manifest included the Codex ops source path exactly once. This remains
same-host continuity evidence only. No off-host backup-complete claim is made.
No restore-drill-complete claim is made. No disaster-recovery-complete claim is
made.

## Next recommendation

Proceed with NA-0487 as the sole READY item under the revised Option 1 scope.
The next implementation directive should authorize the exact source-boundary
paths listed above, require no-cfg production behavior proof and real
reject-path proof, and STOP if additional source, dependency, lockfile,
workflow, vector, corpus, formal, refimpl, service, public-doc, backup, or
public-claim scope is needed.
