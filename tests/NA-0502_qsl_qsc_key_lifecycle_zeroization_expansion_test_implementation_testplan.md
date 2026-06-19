Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-19
Goals: G1, G2, G3, G4, G5

# NA-0502 qsc Key Lifecycle Zeroization Expansion Test Implementation Testplan

## Objective

Verify that NA-0502 implements the bounded qsc integration test at `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization_expansion.rs`, exercises selected real qsc lifecycle/cleanup/artifact surfaces, and preserves all scope and public-claim boundaries.

## Protected invariants

- NA-0502 advances G4 verification evidence without regressing G1, G2, G3, or G5.
- qsc source, Cargo metadata, dependencies, workflows, scripts, helpers, corpus files, vectors, inputs, formal models, refimpl, service paths, public docs, and backup paths remain unchanged.
- The test uses existing qsc integration-test patterns and no new dependencies.
- The implementation does not claim public readiness, production readiness, crypto completeness, secret-material completeness, zeroization completeness, memory erasure, side-channel freedom, vulnerability freedom, bug freedom, or perfect crypto.
- Exactly one READY item remains mandatory until closeout.

## Allowed scope

Implementation path:

- `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization_expansion.rs`

Governance paths:

- `docs/governance/evidence/NA-0502_qsl_qsc_key_lifecycle_zeroization_expansion_test_implementation_harness.md`
- `tests/NA-0502_qsl_qsc_key_lifecycle_zeroization_expansion_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsc source mutation.
- existing qsc test mutation outside the one new test file.
- qsc fuzz target/Cargo/lockfile mutation.
- root Cargo metadata or lockfile mutation.
- dependency mutation.
- corpus/vector/input mutation.
- validator script mutation.
- qsc-adversarial script mutation.
- workflow/script/helper mutation.
- formal/refimpl/service/public/backup mutation.
- qsl-server, qsl-attachments, qshield, or qshield-cli mutation.
- public docs, website, README, START_HERE mutation.
- backup, restore, qsl-backup, qwork, qstart, or qresume execution.

## lifecycle surface selection

Required selected surfaces:

- identity KEM/signing secret rotation and public-record artifact boundary.
- responder pending-confirm reject and session-artifact boundary.

Rejected or deferred surfaces:

- direct X25519/ephemeral memory lifetime or erasure proof without source instrumentation.
- passphrase/output redaction as a primary NA-0502 surface because NA-0446 and NA-0500 already cover selected output boundaries.
- broad same-host client-to-client E2E implementation, which is selected as the default next authorization lane after NA-0502.

## expanded zeroization/lifecycle checks

Run:

```bash
cargo test -p qsc --locked --test key_lifecycle_zeroization_expansion -- --test-threads=1 --nocapture
```

Expected:

- all tests pass;
- `NA0502_KEY_LIFECYCLE_SCOPE_CONSUMED_OK` appears;
- `NA0502_ZEROIZATION_EXPANSION_TEST_IMPLEMENTED_OK` appears;
- `NA0502_SELECTED_LIFECYCLE_SURFACES_CHECKED_OK` appears;
- identity public records do not carry private/secret fields;
- rotated identity material is not retained in the current selected vault/public artifacts.

## reject/artifact boundary behavior

The same test command must also prove:

- responder pending-confirm malformed input rejects through qsc;
- no handshake completion marker appears on reject;
- no completed session artifact is created or changed by reject;
- no session-store key is populated before confirm completion;
- no legacy pending plaintext artifact appears;
- `NA0502_REJECT_OR_ARTIFACT_BOUNDARY_CHECKED_OK` appears.

## inherited key lifecycle/diagnostic/provider-error tests

Run:

```bash
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test binding_negative_vector_consumer -- --test-threads=1 --nocapture
```

Expected: all pass.

## validator scans

Run:

```bash
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
```

Expected: scans pass with no findings.

## audit/fmt checks

Run:

```bash
git diff --check
python3 formal/run_model_checks.py
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Expected: all pass. Cargo audit green is dependency-health evidence only.

## public claim boundary

Scan added lines and PR body for prohibited overclaims. Expected:

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
- no vulnerability-free claim;
- no bug-free claim;
- no perfect-crypto claim.

## closeout prerequisites

Closeout to NA-0503 may proceed only after:

- implementation PR merges;
- post-merge public-safety is attached and green inside the short attach/early-failure window;
- queue state still has NA-0502 READY before closeout;
- D-0993 exists once on main;
- no red required checks are observed;
- closeout edits are limited to the optional closeout allowed paths.

If post-merge public-safety is healthy but still running after the short attach window, stop and hand off closeout.

## Post-fix hardening review checklist

- Correctness under stress: generated test material stays in isolated qsc temp roots; reject path uses qsc's own command behavior, not a fake oracle.
- Minimality: only the one new qsc integration test and allowed governance files are changed.
- Maintainability: helper logic follows existing qsc integration-test patterns for mock vault reads, qsc command execution, and inbox server use.
- Coverage quality: tests fail on missing rotation replacement, public-record leakage, reject success markers, session artifact mutation, or marker retention; they do not pass by scanning hardcoded strings alone.
- Cross-lane stability: Linux/macOS expectations and qsc-adversarial scripts/workflows remain unchanged.

## Stop conditions

Stop if qwork proof is missing or stale, qwork is run by Codex, queue is not READY NA-0502 at startup, D-0992 is absent, D-0993 already exists at startup, `/` usage is at or above 95 percent, current main public-safety is red or missing, qsc-adversarial-smoke is non-terminal after the startup cap, selected test file exists unexpectedly before mutation, fewer than two bounded lifecycle/cleanup surfaces are feasible without forbidden mutation, any forbidden mutation is attempted, key lifecycle expansion or inherited qsc tests fail, validator scans fail, root or nested audit is red, qsl-backup source-list proof regresses, more than one READY appears, or any forbidden public/completion/free/perfect claim is introduced.
