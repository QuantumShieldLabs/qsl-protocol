Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-22

# NA-0521 Build-to-Inspiron Remote qsc E2EE Retry Implementation Testplan

## Purpose

Validate the NA-0521 evidence that recovered D415, cleaned up or proved absent the exact D415 residue paths, rechecked retained remote qsc and forwarding boundaries, executed a bounded Build-to-Inspiron qsc E2EE send/receive and Inspiron-to-Build reply, executed one wrong-mailbox no-mutation negative boundary, and cleaned up sensitive runtime state.

## Scope guard

Allowed mutation paths:
- `docs/governance/evidence/NA-0521_qsl_build_to_inspiron_remote_qsc_e2ee_retry_implementation_harness.md`
- `tests/NA-0521_qsl_build_to_inspiron_remote_qsc_e2ee_retry_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No qsc source/test/fuzz/Cargo, workflow/script/helper, dependency/lockfile, corpus/vector/input, formal/refimpl/service/public/backup, qsl-server, or qsl-attachments mutation is allowed.

## Required evidence checks

- qwork proof files were read and verified; qwork/qstart/qresume were not run.
- D415 response was consumed as `REMOTE_E2EE_BOUNDARY_RECHECK_FAILURE`.
- The exact D415 residue paths `/tmp/na0521_sudo_probe_out` and `/tmp/na0521_sudo_probe_err` were absent or safely removed by one bounded SSH command.
- Remote scripts after cleanup used no shell positional parameters and passed local `bash -n`.
- Remote sudo negative probe used `sudo -n true >/dev/null 2>&1`.
- Remote writes after D415 cleanup stayed under the remote E2EE root.
- Retained remote qsc path, owner, digest, and help smoke were rechecked.
- Local qsc was built from current clean checkout with proof-root target dir and help smoke.
- qsc command manifest exists in proof root and references current CLI/test surfaces.
- Dedicated-key reverse forwarding was rechecked before E2EE.
- Local qsc relay bound only to loopback and was stopped.
- Build-to-Inspiron synthetic send/receive succeeded.
- Inspiron-to-Build synthetic reply succeeded.
- Wrong-mailbox negative boundary failed closed, did not mutate selected state, and valid receive remained usable.
- No-secret-output scan passed.
- Remote E2EE root was removed.
- Local sensitive runtime root was removed.
- No qsl-server or qsl-attachments was used.
- No public/production/security completion claim was introduced.

## Required markers

Evidence or proof logs must contain:

- `NA0521_D415_RESIDUE_CHECK_OK`
- `NA0521_D415_RESIDUE_CLEANUP_OK`
- `NA0521_REMOTE_BOUNDARY_RECHECK_OK`
- `NA0521_RETAINED_QSC_HELP_OK`
- `NA0521_FORWARDING_RECHECK_OK`
- `NA0521_LOCAL_QSC_PROVENANCE_OK`
- `NA0521_QSC_RELAY_STARTED_OK`
- `NA0521_E2EE_REVERSE_FORWARD_STARTED_OK`
- `NA0521_HANDSHAKE_ESTABLISHED_BOTH_SIDES_OK`
- `NA0521_BUILD_TO_INSPIRON_SEND_OK`
- `NA0521_BUILD_TO_INSPIRON_RECEIVE_OK`
- `NA0521_BUILD_TO_INSPIRON_SEND_RECEIVE_OK`
- `NA0521_INSPIRON_TO_BUILD_SEND_OK`
- `NA0521_INSPIRON_TO_BUILD_REPLY_RECEIVE_OK`
- `NA0521_REMOTE_NEGATIVE_REJECT_NO_MUTATION_OK`
- `NA0521_NO_SECRET_OUTPUT_SCAN_OK`
- `NA0521_REMOTE_E2EE_ROOT_REMOVED_OK`
- `NA0521_LOCAL_SENSITIVE_RUNTIME_REMOVED_OK`
- `NA0521_E2EE_CLEANUP_OK`
- `REMOTE_BUILD_TO_INSPIRON_E2EE_PASS_WITH_NEGATIVE_BOUNDARY`

## Required local validation

Run:

```bash
git diff --check
cargo test -p qsc --locked --test same_host_client_to_client_e2e -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test receive_e2e -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test key_lifecycle_zeroization_expansion -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
python3 formal/run_model_checks.py
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

## Static validation

Static validation must prove:
- exact five-path scope;
- D-1031 exists once;
- D-1032 absent before optional closeout;
- duplicate decision count zero;
- READY_COUNT remains 1 and READY remains NA-0521 before implementation PR;
- checked-in evidence has no private key blocks;
- checked-in evidence has no passphrase, password, token, credential, production endpoint, backup material, qsc vault material, or raw private qsc material;
- added lines introduce no public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim.

## Acceptance classification

Expected classification for this run:

`REMOTE_BUILD_TO_INSPIRON_E2EE_PASS_WITH_NEGATIVE_BOUNDARY`

## Successor

Expected successor:

`NA-0522 -- QSL Remote qsc E2EE Negative / Residual Hardening Scope Authorization Plan`

## Boundaries

This testplan does not authorize qsl-server, qsl-attachments, package installation, public service deployment, remote source checkout/build, qwork/qstart/qresume, qsl-backup execution, qsc source/test/fuzz/Cargo mutation, workflow/script/helper mutation, dependency mutation, corpus/vector/input mutation, formal/refimpl/service/public/backup mutation, or any public/production readiness claim.
