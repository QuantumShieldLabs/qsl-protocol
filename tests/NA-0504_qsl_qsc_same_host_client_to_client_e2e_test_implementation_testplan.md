Status: Supporting
Owner: QSL Governance / Core Assurance
Last-Updated: 2026-06-20

# NA-0504 qsc Same-Host Client-to-Client E2E Test Implementation Testplan

## Objective

Verify that NA-0504 implements the selected same-host qsc Alice/Bob
client-to-client integration test using existing qsc CLI/test-visible patterns
and without forbidden source, dependency, workflow, corpus, formal, refimpl,
service, public, or backup mutation.

## Protected invariants

- Exactly one READY item remains mandatory until closeout.
- NA-0504 advances G4 without regressing G1, G2, G3, or G5.
- Fail-closed behavior is preserved on the selected negative path.
- The test exercises real qsc CLI behavior, not static fixture scanning and
  not a fake oracle.
- no public-readiness claim is made.
- no production-readiness claim is made.
- no crypto-complete claim is made.
- no replay-proof claim is made.
- no downgrade-proof claim is made.
- no secret-material-complete claim is made.
- no side-channel-free claim is made.

## Allowed scope

- `qsl/qsl-client/qsc/tests/same_host_client_to_client_e2e.rs`
- `docs/governance/evidence/NA-0504_qsl_qsc_same_host_client_to_client_e2e_test_implementation_harness.md`
- `tests/NA-0504_qsl_qsc_same_host_client_to_client_e2e_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsc source mutation.
- existing qsc test mutation outside the new NA-0504 test file.
- qsc fuzz target/Cargo/lockfile mutation.
- root Cargo/lockfile or dependency mutation.
- workflow, script, helper, validator, or qsc-adversarial mutation.
- corpus, vector, input, or internal-manifest mutation.
- formal, refimpl, qsl-server, qsl-attachments, qshield, qshield-cli,
  service, public-doc, website, README, START_HERE, backup, qsl-backup,
  backup status, backup plan, qwork, qstart, qresume, qshell, archive, move,
  or delete mutation.
- remote SSH, remote account setup, host mutation, LAN setup, or two-machine
  setup.

## two-root client setup

Expected:

- Alice uses one independent `common::TestIsolation` root.
- Bob uses a second independent `common::TestIsolation` root.
- Alice and Bob have distinct `QSC_CONFIG_DIR` paths.
- qsc commands apply the corresponding isolation.

Validation marker:

- `NA0504_TWO_INDEPENDENT_CLIENT_ROOTS_OK`

## identity/public-record/trust exchange

Expected:

- Alice and Bob run qsc `identity rotate --confirm`.
- The test reads qsc identity fingerprints through `identity show`.
- Alice pins/trusts Bob's public record/contact device.
- Bob pins/trusts Alice's public record/contact device.

Validation markers:

- `NA0504_ALICE_BOB_IDENTITY_SETUP_OK`
- `NA0504_PUBLIC_RECORD_TRUST_EXCHANGE_OK`

## send/receive

Expected:

- Alice and Bob complete local relay-backed handshake setup.
- Alice sends a synthetic payload to Bob with qsc `send`.
- Bob receives with qsc `receive`.
- The received file bytes match the synthetic payload.

Validation marker:

- `NA0504_SEND_RECEIVE_FLOW_OK`

## reply

Expected:

- Bob sends a synthetic reply to Alice with qsc `send`.
- Alice receives with qsc `receive`.
- The received file bytes match the synthetic reply.

Validation marker:

- `NA0504_REPLY_FLOW_OK`

## negative reject/no-mutation

Expected:

- A wrong-mailbox receive attempt fails closed.
- No receive-output artifact is created by the failed receive.
- The selected Bob session artifact does not change across the failed receive.
- A later valid receive still commits the queued message.

Validation marker:

- `NA0504_NEGATIVE_REJECT_NO_MUTATION_OK`

## diagnostic/no-secret output

Expected:

- Captured stdout/stderr/diagnostics from selected real qsc commands pass the
  no-secret-shaped-output scanner.
- Synthetic forbidden marker strings are rejected by the scanner.

Validation marker:

- `NA0504_STDOUT_STDERR_NO_SECRET_OUTPUT_OK`

## inherited tests

Run and require pass:

```bash
cargo test -p qsc --locked --test receive_e2e -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test key_lifecycle_zeroization_expansion -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test binding_negative_vector_consumer -- --test-threads=1 --nocapture
```

## validator scans

Run and require pass:

```bash
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
```

## audit/fmt checks

Run and require pass unless a directive-defined stop condition is reached:

```bash
git diff --check
python3 formal/run_model_checks.py
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Also run local proof checks:

- exact allowed path scope guard.
- link-check.
- leak-scan.
- added-line overclaim scan.
- classifier.
- PR body preflight.
- goal-lint.

## public claim boundary

The PR and evidence must preserve:

- no public-readiness claim.
- no production-readiness claim.
- no public-internet-readiness claim.
- no external-review-complete claim.
- no crypto-complete claim.
- no KEM-complete claim.
- no signature-complete claim.
- no identity-complete claim.
- no provider-RNG-complete claim.
- no secret-material-complete claim.
- no zeroization-complete claim.
- no memory-erasure-complete claim.
- no replay-proof claim.
- no downgrade-proof claim.
- no side-channel-free claim.
- no vulnerability-free claim.
- no bug-free claim.
- no perfect-crypto claim.

## closeout prerequisites

NA-0504 closeout to NA-0505 is allowed only after:

- implementation PR merges.
- post-merge public-safety is green.
- required full suites are green or policy-accepted inside the short
  attach/early-failure window.
- D-0997 exists once on main.
- NA-0504 remains READY before closeout.
- NA-0505 is restored by a separate closeout patch without implementing
  remote SSH/test account setup.
