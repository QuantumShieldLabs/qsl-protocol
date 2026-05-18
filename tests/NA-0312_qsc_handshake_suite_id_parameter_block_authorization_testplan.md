Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-18
Replaces: n/a
Superseded-By: n/a

Goals: G1, G2, G3, G4, G5

# NA-0312 qsc Handshake Suite-ID Parameter-Block Authorization Testplan

Directive: QSL-DIR-2026-05-18-120 / NA-0312

## Objective

Validate that NA-0312 records an exact implementation authorization plan for a
future qsc handshake suite-id parameter-block lane without implementing qsc
runtime behavior, QHSM/QSP wire-format behavior, production handshake behavior,
crypto state-machine behavior, key schedule behavior, metadata runtime
behavior, dependencies, services, website sources, or public-claim upgrades.

## Protected invariants

- Exactly one READY item remains NA-0312 until closeout.
- D-0603 exists exactly once after the authorization patch.
- D-0604 is absent during the authorization patch.
- NA-0312 is authorization only.
- Explicit qsc suite-id admission remains unproven until a future
  implementation/harness lane proves it.
- Persisted Suite-2 state is not represented as explicit suite-id admission.
- No production, public-internet, external-review, anonymity, metadata-free,
  untraceable, quantum-proof, unbreakable, guaranteed-secure, or
  broad-readiness claim is introduced.
- Metadata runtime identifier/default-padding transition remains visible.

## Allowed scope

The NA-0312 authorization patch may touch only:

- `docs/governance/evidence/NA-0312_qsc_handshake_suite_id_parameter_block_implementation_authorization.md`
- `tests/NA-0312_qsc_handshake_suite_id_parameter_block_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

The NA-0312 authorization patch must not touch:

- `qsl/qsl-client/qsc/src/**`
- `qsl/qsl-client/qsc/tests/**`
- `qsl/qsl-client/qsc/Cargo.toml`
- `Cargo.toml`
- `Cargo.lock`
- `.github/**`
- `qsl-server/**`
- `qsl-attachments/**`
- `qsl/qsl-client/qsc-desktop/**`
- `apps/**`
- `tools/refimpl/**`
- `inputs/**`
- `formal/**`
- `docs/public/**`
- `README.md`
- `START_HERE.md`
- website or external website sources
- branch-protection or public-safety configuration

## Prior qsc harness review requirements

The evidence must record:

- live NA-0312 scope;
- inherited NA-0311 conclusion;
- current qsc fixed-layout `QHSM` v1 frame surface;
- current qsc CLI/relay harness value and limits;
- why test-only synthetic fixtures are insufficient as explicit suite-id
  admission proof;
- why a shared qsc parser/runtime admission surface is required.

## Implementation boundary inventory requirements

The evidence must inventory:

- QHSM frame structs;
- parser/decoder;
- encoder/builder;
- handshake/admission state;
- compatibility and suite-required mode;
- session-state write path;
- `recv_commit` and output path;
- logging/error path;
- tests/harness.

For each surface, the evidence must record current file/function if found,
future touch decision, test-only sufficiency, QSP dependency, expected tests,
and risk.

## Authorization decision requirements

The evidence must select exactly one successor path:

- Outcome A: qsc Handshake Suite-ID Parameter-Block Implementation Harness;
- Outcome B: qsc Handshake Suite-ID Parser Harness and Compatibility Vectors;
- Outcome C: Metadata Runtime Identifier and Default Padding Transition Plan;
- Outcome D: qsc Handshake Suite-ID Implementation Blocker Resolution.

The decision must include rationale and rejected alternatives.

## Future markers requirements

If Outcome A or B is selected, the evidence must require:

- `NA0313_QHSM_V2_PARAMETER_BLOCK_PARSE_OK`
- `NA0313_VALID_SUITE2_ACCEPT_OK`
- `NA0313_LEGACY_COMPAT_ACCEPT_OK`
- `NA0313_REQUIRED_MODE_LEGACY_REJECT_OK`
- `NA0313_UNSUPPORTED_SUITE_REJECT_OK`
- `NA0313_DOWNGRADE_SUITE_REJECT_OK`
- `NA0313_STRIPPED_SUITE_REJECT_OK`
- `NA0313_MISMATCH_SUITE_REJECT_OK`
- `NA0313_DUPLICATE_SUITE_REJECT_OK`
- `NA0313_UNKNOWN_CRITICAL_REJECT_OK`
- `NA0313_NONCANONICAL_REJECT_OK`
- `NA0313_MALFORMED_REJECT_OK`
- `NA0313_TRANSCRIPT_BINDING_OK`
- `NA0313_KEY_CONTEXT_BINDING_OK`
- `NA0313_NO_MUTATION_ON_REJECT_OK`
- `NA0313_NO_OUTPUT_ON_REJECT_OK`
- `NA0313_NO_SECRET_LEAK_OK`
- `NA0313_QSC_SUITE_ID_PARAMETER_BLOCK_OK`

## Future test requirements

The evidence must require:

- vector-driven qsc harness from NA-0310 vectors;
- refimpl oracle cross-check from NA-0310;
- model cross-check from NA-0309;
- current qsc harness regressions from NA-0302, NA-0303, and NA-0304;
- refimpl regression harnesses from NA-0300 through NA-0302;
- `send_commit`;
- formal model checks;
- metadata conformance smoke and metadata phase-2 harnesses as G5 baseline;
- demo smoke/stress where feasible;
- cargo audit and `rustls-webpki` dependency proof;
- public-safety required and green.

## Metadata agenda requirements

The evidence must review:

- NA-0288 metadata phase-2 gap plan;
- NA-0290 identifier/padding design;
- NA-0291 identifier/padding harness;
- NA-0292 sanitized-error/retention design;
- NA-0293 sanitized-error/retention harness;
- public release/readiness map metadata rows;
- external review package metadata rows.

If metadata is not selected as NA-0313, the evidence must recommend the exact
future lane title:

Metadata Runtime Identifier and Default Padding Transition Plan

## Coverage matrix requirements

The evidence must map each major future requirement to:

- NA-0312 authorization;
- expected NA-0313 proof;
- stop condition.

## Successor-selection requirements

The evidence must record the exact selected successor title and must not invent
an implementation beyond the selected successor.

Selected successor expected by this testplan:

NA-0313 -- qsc Handshake Suite-ID Parameter-Block Implementation Harness

## Claim-boundary requirements

The evidence must state that:

- authorization is not implementation;
- model/vector/refimpl evidence is not qsc runtime proof;
- legacy compatibility accept is not explicit suite-id admission;
- persisted Suite-2 state is supporting state evidence only;
- external review remains future-gated;
- metadata runtime reduction remains future-gated;
- all known readiness gaps remain visible.

## Backup-impact requirements

The evidence must record whether backup-plan changes are required. Expected
result: no backup-plan update is required if changes stay under tracked
qsl-protocol governance/evidence/testplan paths in `/srv/qbuild/work`.

## Required local checks

Run before PR:

- `git diff --check`
- direct overclaim scan over changed lines or changed files
- `python3 -m json.tool inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`
- `cargo test -p quantumshield_refimpl --locked --test na_0310_qsc_suite_id_vector_oracle -- --test-threads=1`
- `cargo test -p quantumshield_refimpl --locked -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `cargo +stable test -p qsc --locked --test na_0302_suite2_negotiation_cross_surface -- --test-threads=1`
- `cargo +stable test -p qsc --locked --test na_0303_handshake_activation_negotiation -- --test-threads=1`
- `cargo +stable test -p qsc --locked --test na_0304_handshake_suite_id_negotiation -- --test-threads=1`
- `scripts/ci/metadata_conformance_smoke.sh`
- `scripts/ci/metadata_phase2_identifier_padding_harness.sh`
- `scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh`
- `scripts/ci/demo_cli_smoke.sh`
- `DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh`
- `DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh` if feasible
- `cargo test -p qshield-cli --locked -- --test-threads=1`
- `cargo build -p qshield-cli --locked`
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allow ...`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- goal-lint / PR body validation
- `bash scripts/ci/classify_ci_scope.sh <changed paths>`

## CI expectations

The PR must include a body with a standalone `Goals: G1, G2, G3, G4, G5` line
near the top. Required checks, including public-safety, must attach and pass
normally before merge. CodeQL may be accepted only according to existing
repository policy if it reports neutral/skipped for docs-only scope.

## Successor handoff

After PR merge and post-merge public-safety success, the closeout directive may
mark NA-0312 DONE and restore exactly one READY successor:

NA-0313 -- qsc Handshake Suite-ID Parameter-Block Implementation Harness

The closeout must not implement NA-0313.
