Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-23

# NA-0528 Remote qsc E2EE Reverse-Forwarding Diagnostic / Retry Scope Authorization Testplan

## Objective

Validate that NA-0528 is authorization-only, consumes D427 / D426 / D425 / D414 / D413 inheritance, classifies the forwarding diagnostic successor truthfully, and selects exactly one NA-0529 successor without running SSH, qsc send/receive, remote E2EE, qsl-server, or qsl-attachments.

Accepted classification for this run:

```text
REMOTE_FORWARDING_DIAGNOSTIC_IMPLEMENTATION_READY
```

## Scope

Allowed checked-in paths:

- `docs/governance/evidence/NA-0528_qsl_remote_qsc_e2ee_reverse_forwarding_diagnostic_retry_scope_authorization_plan.md`
- `tests/NA-0528_qsl_remote_qsc_e2ee_reverse_forwarding_diagnostic_retry_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No qsc source/test/fuzz/Cargo path, workflow/script/helper path, dependency/lockfile path, corpus/vector/input path, formal/refimpl/service/public/backup path, qsl-server path, or qsl-attachments path may be mutated.

## Expected evidence

- qwork proof files were read and copied without running qwork/qstart/qresume.
- Startup queue proof shows READY_COUNT 1 and READY NA-0528.
- NA-0527, NA-0526, and NA-0525 are DONE.
- D-1044 and D-1045 each exist once before patch.
- D-1046 exists once after patch.
- D-1047 is absent before optional closeout.
- Duplicate decision count is zero.
- D427 / D426 / D425 / D414 / D413 inheritance is consumed.
- D427 forwarding failure details are recorded, including `ClearAllForwardings yes`, correction, `remote port forwarding failed for listen port 39176`, bind availability proof, and bounded recovery exhaustion.
- NA-0520 prior successful forwarding proof is compared against D427.
- Options 1 through 9 are evaluated.
- Options 1, 2, 3, and 4 are selected for NA-0529, with Option 5 deferred until evidence requires operator proof-review.
- Immediate E2EE retry, qsl-server/qsl-attachments integration, broad SSH/key/sshd remediation, and remote sprint abandonment are rejected or deferred.
- Hostile cryptographer, red-team, SRE, release-claim, side-channel, formal residual, external-review, and assurance-gap reviews are present.
- Prioritization matrix is present.
- Selected successor is `NA-0529 -- QSL Remote qsc E2EE Reverse-Forwarding Diagnostic Implementation Harness`.
- No remote action, SSH execution, qsc send/receive, remote E2EE, qsl-server, or qsl-attachments use occurs in NA-0528.

## Static validation

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed docs/governance/evidence/NA-0528_qsl_remote_qsc_e2ee_reverse_forwarding_diagnostic_retry_scope_authorization_plan.md \
  --allowed tests/NA-0528_qsl_remote_qsc_e2ee_reverse_forwarding_diagnostic_retry_scope_authorization_testplan.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh \
  docs/governance/evidence/NA-0528_qsl_remote_qsc_e2ee_reverse_forwarding_diagnostic_retry_scope_authorization_plan.md \
  tests/NA-0528_qsl_remote_qsc_e2ee_reverse_forwarding_diagnostic_retry_scope_authorization_testplan.md \
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
python3 scripts/ci/qsl_evidence_helper.py queue --select NA-0528 --select NA-0527 --select NA-0526 --select NA-0525
python3 scripts/ci/qsl_evidence_helper.py decisions --select D-1044 --select D-1045 --select D-1046 --select D-1047
```

Expected:

- READY_COUNT 1.
- READY NA-0528 before optional closeout.
- NA-0527, NA-0526, and NA-0525 are DONE.
- D-1044 count 1.
- D-1045 count 1.
- D-1046 count 1 after patch.
- D-1047 count 0 before optional closeout.
- DUPLICATE_COUNT 0.

## Marker validation

Evidence must contain:

```text
REMOTE_FORWARDING_DIAGNOSTIC_IMPLEMENTATION_READY
NA-0529 -- QSL Remote qsc E2EE Reverse-Forwarding Diagnostic Implementation Harness
NA0529_D427_FORWARDING_FAILURE_CONSUMED_OK
NA0529_NA0520_SUCCESSFUL_FORWARDING_CONSUMED_OK
NA0529_COMMAND_CONFIG_DIFF_REVIEWED_OK
NA0529_DEDICATED_KEY_USED_OK
NA0529_LOOPBACK_ONLY_BIND_CHECKED_OK
NA0529_EXIT_ON_FORWARD_FAILURE_USED_OK
NA0529_SYNTHETIC_MARKER_TRAVERSAL_RESULT_RECORDED_OK
NA0529_SSH_DEBUG_LOG_REDACTED_OK
NA0529_NO_REMOTE_FILE_WRITE_OK
NA0529_NO_QSC_E2EE_OK
NA0529_NO_QSC_SEND_RECEIVE_OK
NA0529_NO_QSL_SERVER_ATTACHMENTS_OK
NA0529_CLEANUP_COMPLETED_OK
NA0529_NO_PUBLIC_READINESS_CLAIM_OK
NA0529_NO_PRODUCTION_READINESS_CLAIM_OK
NA0529_ONE_READY_INVARIANT_OK
```

Evidence must state that NA-0528 performed no remote action, no SSH execution, no qsc send/receive, no remote E2EE, no qsl-server use, and no qsl-attachments use.

## Private material validation

Run scans over added evidence and governance diff to prove no private material was checked in:

```bash
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

Additional expected private-material marker proof:

- no OpenSSH private-key block header;
- no RSA private-key block header;
- no generic private-key block header;
- no API token style fixtures;
- no private key/passphrase/token/password material in checked-in evidence.

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

- No remote action by Codex.
- No SSH execution by Codex.
- No scp/sftp/rsync.
- No qsc send/receive.
- No remote E2EE.
- No qsl-server use.
- No qsl-attachments use.
- No package installation.
- No sudo/admin action.
- No key generation or installation.
- No authorized_keys read or mutation.
- No SSH config mutation.
- No known_hosts mutation.
- No sshd_config read or mutation.
- No remote host mutation.
- No qwork/qstart/qresume execution by Codex.
- No qsl-backup execution.
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
- No vulnerability-free claim.
- No bug-free claim.
- No perfect-crypto claim.
