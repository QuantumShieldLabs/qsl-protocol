Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-23

# NA-0527 Remote qsc E2EE Wrong-Peer / Stale-Trust Negative Retry After Restaging Implementation Testplan

## Objective

Validate NA-0527 evidence for the retained-qsc freshness recheck, command-surface inspection, remote/local boundary checks, dedicated loopback reverse-forwarding attempt, cleanup proof, and fail-closed forwarding classification.

Accepted classification for this run:

```text
REMOTE_E2EE_FORWARDING_RECHECK_FAILURE
```

## Scope

Allowed checked-in paths:

- `docs/governance/evidence/NA-0527_qsl_remote_qsc_e2ee_wrong_peer_stale_trust_negative_retry_after_restaging_implementation_harness.md`
- `tests/NA-0527_qsl_remote_qsc_e2ee_wrong_peer_stale_trust_negative_retry_after_restaging_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No qsc source/test/fuzz/Cargo path, workflow/script/helper path, dependency/lockfile path, corpus/vector/input path, formal/refimpl/service/public/backup path, qsl-server path, or qsl-attachments path may be mutated.

## Expected evidence

- qwork proof files were read and copied without running qwork/qstart/qresume.
- Startup queue proof shows READY_COUNT 1 and READY NA-0527.
- NA-0526 and NA-0525 are DONE.
- D-1042 and D-1043 each exist once before patch.
- D-1044 exists once after patch.
- D-1045 is absent before optional closeout.
- Duplicate decision count is zero.
- D426/D425/D424/D419 inheritance is consumed.
- Root and nested qsc fuzz lockfiles contain `quinn-proto 0.11.15`.
- Current local qsc was built from clean source in a proof-root target dir.
- qsc runtime/dependency paths have no diff from the NA-0526 implementation commit.
- Retained remote qsc path, owner, mode, size, hash, and help smoke match NA-0526.
- Command manifest exists under the proof root in Markdown and JSON.
- Remote boundary recheck passes before any E2EE root is created.
- Local relay binds only loopback.
- Dedicated reverse-forwarding recheck fails after bounded recovery.
- No baseline E2EE, wrong-peer negative, stale-trust negative, or qsc send/receive is executed after forwarding failure.
- Cleanup proof shows remote E2EE root absent, local sensitive runtime absent, local relay stopped, SSH forward absent, and local port closed.
- Selected successor candidate is `NA-0528 -- QSL Remote qsc E2EE Reverse-Forwarding Diagnostic / Retry Scope Authorization Plan`.

## Static validation

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed docs/governance/evidence/NA-0527_qsl_remote_qsc_e2ee_wrong_peer_stale_trust_negative_retry_after_restaging_implementation_harness.md \
  --allowed tests/NA-0527_qsl_remote_qsc_e2ee_wrong_peer_stale_trust_negative_retry_after_restaging_implementation_testplan.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh \
  docs/governance/evidence/NA-0527_qsl_remote_qsc_e2ee_wrong_peer_stale_trust_negative_retry_after_restaging_implementation_harness.md \
  tests/NA-0527_qsl_remote_qsc_e2ee_wrong_peer_stale_trust_negative_retry_after_restaging_implementation_testplan.md \
  DECISIONS.md \
  TRACEABILITY.md \
  docs/ops/ROLLING_OPERATIONS_JOURNAL.md
```

Expected:

- Scope guard reports only the five allowed implementation paths.
- Link-check passes.
- Leak scan reports zero added-line findings.
- Classifier accepts docs/governance-only scope.

## Queue and decision validation

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue --select NA-0527 --select NA-0526 --select NA-0525
python3 scripts/ci/qsl_evidence_helper.py decisions --select D-1042 --select D-1043 --select D-1044 --select D-1045
```

Expected:

- READY_COUNT 1.
- READY NA-0527 before optional closeout.
- D-1042 count 1.
- D-1043 count 1.
- D-1044 count 1 after patch.
- D-1045 count 0 before optional closeout.
- DUPLICATE_COUNT 0.

## Marker validation

Evidence must contain:

```text
REMOTE_E2EE_FORWARDING_RECHECK_FAILURE
NA-0528 -- QSL Remote qsc E2EE Reverse-Forwarding Diagnostic / Retry Scope Authorization Plan
```

Evidence must state that no baseline E2EE, no wrong-peer negative, no stale-trust negative, and no qsc send/receive were executed after forwarding failed.

## Private material validation

Run scans over added evidence and governance diff to prove no private material was checked in:

```bash
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

Also run proof-root scans for private-key block markers, bearer/API-token-style material, passphrase/password marker leakage, and raw private qsc material. Expected: zero findings.

## Required local validation

Run:

```bash
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo test -p qsc --locked --test same_host_client_to_client_e2e -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test receive_e2e -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test key_lifecycle_zeroization_expansion -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
python3 formal/run_model_checks.py
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Expected: all commands pass. If validation fails with an understood in-scope cause, fix/rerun within the directive retry budget. If remediation would require out-of-scope mutation or weakening fail-closed behavior, stop.

## Boundary assertions

- No qsl-server use.
- No qsl-attachments use.
- No public service deployment.
- No package installation.
- No remote source checkout/build.
- No qwork/qstart/qresume execution by Codex.
- No qsl-backup execution.
- No retained remote qsc mutation.
- No qsc source/test/fuzz/Cargo mutation.
- No workflow/script/helper/dependency mutation.
- No corpus/vector/input mutation.
- No formal/refimpl/service/public/backup mutation.
- No public-readiness claim.
- No production-readiness claim.
- No public-internet-readiness claim.
- No external-review-complete claim.
- No crypto-complete claim.
- No identity-complete claim.
- No trust-complete claim.
- No replay-proof claim.
- No downgrade-proof claim.
- No secret-material-complete claim.
- No side-channel-free claim.
- No vulnerability-free, bug-free, or perfect-crypto claim.
