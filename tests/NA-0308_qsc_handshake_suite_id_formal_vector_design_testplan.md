Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-17
Replaces: n/a
Superseded-By: n/a

Goals: G1, G2, G3, G4, G5

# NA-0308 qsc Handshake Suite-ID Formal Vector Design Testplan

## Objective

Produce a design-only evidence package for future explicit qsc handshake
suite-id formal/model properties, vector schema, vector categories, refimpl
oracle requirements, qsc harness requirements, and successor selection without
implementing qsc wire-format or production handshake behavior.

## Protected invariants

- No qsc suite-id wire-format implementation in NA-0308.
- No qsc suite-id production field implementation in NA-0308.
- No QHSM or QSP frame schema implementation change.
- No qsc runtime source or qsc runtime test change.
- No formal model implementation change.
- No vector fixture implementation change.
- No refimpl oracle implementation change.
- No crypto state-machine or key schedule behavior change.
- No production handshake implementation change.
- No dependency, workflow, service, website, docs/public, README, START_HERE,
  branch-protection, or public-safety configuration drift.
- Missing explicit qsc handshake suite-id admission evidence remains visible.
- Persisted Suite-2 state is not represented as explicit qsc suite-id
  admission evidence.
- No unsupported broad readiness, external-review, anonymity, metadata-free,
  or untraceable claim is introduced.

## Allowed scope

- `docs/governance/evidence/NA-0308_qsc_handshake_suite_id_formal_vector_design.md`
- `tests/NA-0308_qsc_handshake_suite_id_formal_vector_design_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsc runtime source under `qsl/qsl-client/qsc/src/**`.
- qsc runtime tests under `qsl/qsl-client/qsc/tests/**`.
- QSP/refimpl/protocol-core implementation source.
- formal model implementation files under `formal/**`.
- input/vector fixtures under `inputs/**`.
- refimpl implementation or test files under `tools/refimpl/**`.
- crypto state-machine, key schedule, production handshake, or QSP wire-format
  implementation.
- `Cargo.toml` or `Cargo.lock`.
- `.github/**` and `scripts/**`.
- qsl-server, qsl-attachments, qsc-desktop, apps, website/external website,
  README, START_HERE, docs/public, branch-protection, and public-safety
  configuration.

## Prior design review requirements

The evidence must preserve the NA-0304 through NA-0307 findings:

- current qsc `QHSM` v1 A1/B1/A2 frames have no explicit suite-id field;
- valid qsc handshake activation persists Suite-2 state with protocol version
  `0x0500` and suite id `0x0002`;
- persisted state is not explicit suite-id admission evidence;
- a future version-gated `QHSM` v2 negotiated-parameter block is selected for
  explicit qsc suite-id semantics;
- legacy v1 compatibility is explicit-only and never suite-id admission
  evidence;
- byte-exact canonical parameter blocks must be transcript-bound across
  A1/B1/A2; and
- future qsc handshake key-schedule context must include the negotiated suite
  context or an explicit equivalent selected by the model/vector lane.

## Formal/model property requirements

The evidence must define at least these properties:

- suite context present and canonical in negotiated parameter context;
- transcript includes suite context;
- key schedule context includes suite context or explicit equivalent;
- unsupported suite id rejects before state mutation;
- downgraded or stripped suite id rejects before state mutation;
- mismatched suite id across A1/B1/A2 rejects before state mutation;
- duplicate suite parameter rejects;
- unknown critical parameter rejects;
- noncanonical parameter order rejects;
- malformed length rejects;
- legacy frame rejects in suite-id-required mode;
- compatibility mode is explicit and cannot silently downgrade required mode;
- reject path produces no `recv_commit` or output;
- reject path leaks no secret, plaintext, or sentinel; and
- accepted valid path preserves current Suite-2 semantics.

Each property must include model name, preconditions, expected result, mutation
boundary, relation to NA-0307 design, and future executable command or
placeholder.

## Vector schema requirements

The evidence must define required fields for future vector fixtures:

- `vector_id`
- `purpose`
- `qhsm_version`
- `frame_sequence`
- `negotiated_parameters`
- `suite_id`
- `protocol_version`
- `transcript_binding_expected`
- `key_schedule_context_expected`
- `expected_result`
- `expected_reject_code` or `reason_label`
- `mutation_expected`
- `recv_commit_expected`
- `secret_leak_expected`
- `canonical_encoding_expected`
- `compatibility_mode`
- `legacy_mode`
- `notes`

## Vector category requirements

The evidence must define categories for:

- valid v2 Suite-2 parameter block;
- legacy v1 compatibility allowed;
- legacy v1 rejected in suite-id-required mode;
- unsupported suite id;
- downgraded suite id;
- stripped suite-id parameter;
- A1/B1 mismatch;
- B1/A2 mismatch;
- duplicate suite-id parameter;
- unknown critical parameter;
- unknown noncritical parameter under selected policy;
- noncanonical parameter order;
- malformed parameter length;
- inconsistent protocol version and suite id;
- replayed A1 or A2 with suite context;
- valid Suite-2 with transcript binding;
- transcript-binding mismatch; and
- key-schedule context mismatch.

Each category must map to expected result, refimpl oracle requirement, qsc
harness requirement, and formal/model property.

## Refimpl oracle requirements

The evidence must require future oracle work to parse the vector schema,
validate canonical parameter encoding, compute expected transcript/context
labels when safe, validate accepted/rejected outcomes, expose deterministic
reason labels without leaking secrets, and run as a bounded test/harness.

## qsc harness requirements

The evidence must require future qsc harness work to construct future `QHSM`
v2 frames or negotiated-parameter fixtures only after explicit authorization,
prove valid Suite-2 state on accepted paths, prove no mutation/output on
invalid paths, assert deterministic reason labels, scan for secret/plaintext
leaks, test compatibility and suite-required modes separately, test old/new
parser behavior where available, and assert no panic/backtrace.

## Coverage matrix requirements

The evidence must include a coverage matrix mapping properties, vector
categories, refimpl oracle requirements, qsc harness requirements, future
artifacts, current status, risk, and needed next action.

## Successor-selection requirements

The evidence must select one exact NA-0309 successor:

- `NA-0309 -- qsc Handshake Suite-ID Formal Model Properties`
- `NA-0309 -- qsc Handshake Suite-ID Vector Schema and Refimpl Oracle`
- `NA-0309 -- qsc Handshake Suite-ID Parameter-Block Implementation Authorization`
- `NA-0309 -- qsc Handshake Suite-ID Model/Vector Blocker Resolution`

The selected successor must be justified against rejected alternatives.

## Claim-boundary requirements

The evidence must explicitly reject:

- treating design-only vectors as runtime implementation;
- treating model design as executable proof;
- treating persisted Suite-2 state as explicit qsc suite-id admission evidence;
- hiding missing qsc suite-id admission implementation;
- smuggling production wire/schema change into NA-0308; and
- implying external review completion, unrestricted deployment readiness,
  anonymity, metadata-free behavior, or untraceability.

## Backup-impact requirements

The evidence must state whether NA-0308 changes important evidence locations,
response paths, source roots, excluded backup paths, or creates
non-rebuildable artifacts outside current backup scope.

Expected result: no backup-plan update is required if the patch only adds
qsl-protocol governance evidence and testplan files under already-backed-up
repository paths.

## Required local checks

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/run_model_checks.py`
- `scripts/ci/metadata_conformance_smoke.sh`
- `scripts/ci/metadata_phase2_identifier_padding_harness.sh`
- `scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh`
- `scripts/ci/demo_cli_smoke.sh`
- `DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh`
- `DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh`
- `cargo +stable test -p qsc --locked --test na_0304_handshake_suite_id_negotiation -- --test-threads=1 --nocapture`
- `cargo +stable test -p qsc --locked --test na_0303_handshake_activation_negotiation -- --test-threads=1 --nocapture`
- `cargo +stable test -p qsc --locked --test na_0302_suite2_negotiation_cross_surface -- --test-threads=1 --nocapture`
- `cargo +stable test -p quantumshield_refimpl --locked --test na_0301_suite2_negotiation_downgrade -- --test-threads=1 --nocapture`
- `cargo +stable test -p quantumshield_refimpl --locked --test na_0300_core_replay_reject_no_mutation -- --test-threads=1 --nocapture`
- `cargo +stable build -p qshield-cli --locked`
- `cargo +stable test -p qshield-cli --locked -- --test-threads=1`
- `git diff --check`
- direct overclaim phrase scan over changed lines
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed docs/governance/evidence/NA-0308_qsc_handshake_suite_id_formal_vector_design.md --allowed tests/NA-0308_qsc_handshake_suite_id_formal_vector_design_testplan.md --allowed DECISIONS.md --allowed TRACEABILITY.md --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- goal-lint using a PR body containing `Goals: G1, G2, G3, G4, G5`
- `bash scripts/ci/classify_ci_scope.sh <changed_paths>`

## CI expectations

Required CI and public-safety must complete normally. Because NA-0308 is
governance/testplan-only, full-suite cost-control skips are acceptable only
when classifier and public-safety jobs report intentional skip behavior.

## Successor handoff

After the NA-0308 PR merges and post-merge public-safety is green, a separate
closeout may mark NA-0308 DONE and restore exactly one READY successor:

NA-0309 -- qsc Handshake Suite-ID Formal Model Properties

The closeout must not implement NA-0309.
