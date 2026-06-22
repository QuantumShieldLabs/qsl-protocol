Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-22

# NA-0522 QSL Remote qsc E2EE Negative / Residual Hardening Scope Authorization Testplan

## Purpose

Validate that NA-0522 is authorization-only, consumes NA-0521 / D416, inventories direct remote qsc E2EE residuals, selects the exact NA-0523 implementation lane, and preserves all no-remote-action, no-service-integration, no-code-mutation, cleanup, redaction, and public-claim boundaries.

## Scope guard

Allowed mutation paths:
- `docs/governance/evidence/NA-0522_qsl_remote_qsc_e2ee_negative_residual_hardening_scope_authorization_plan.md`
- `tests/NA-0522_qsl_remote_qsc_e2ee_negative_residual_hardening_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No qsc source/test/fuzz/Cargo, workflow/script/helper, dependency/lockfile, corpus/vector/input, formal/refimpl/service/public/backup, qsl-server, qsl-attachments, qshield, qshield-cli, qsl-backup, public docs, website, archive, move, delete, or backup path mutation is allowed.

## Required evidence checks

- qwork proof files were read and verified; qwork/qstart/qresume were not run.
- Proof HEAD and proof origin/main matched live pre-fetch refs.
- READY_COUNT was 1 and READY was NA-0522.
- NA-0521, NA-0520, and NA-0519 were DONE.
- D-1031 and D-1032 existed once.
- D-1033 was absent before the patch and exists once after the patch.
- Duplicate decision count is zero.
- D416 response was consumed.
- NA-0521 evidence and testplan were consumed.
- Classification `REMOTE_BUILD_TO_INSPIRON_E2EE_PASS_WITH_NEGATIVE_BOUNDARY` was inherited.
- Residual inventory includes replay, corrupt delivery, wrong peer, stale trust, repeated run/cleanup, retained qsc freshness, forwarding cleanup, route-token/capability redaction, no-secret-output review, remote root cleanup/retention, scheduled remote CI residuals, qsl-server/qsl-attachments deferral, and public/production claim boundary.
- Options 1 through 9 were reviewed.
- Selected classification is `REMOTE_E2EE_REPLAY_CORRUPT_NEGATIVE_IMPLEMENTATION_READY`.
- Selected successor is `NA-0523 -- QSL Remote qsc E2EE Replay / Corrupt Delivery Negative Boundary Implementation Harness`.
- Future command family, proof/redaction rules, stop conditions, markers, and scope bundle are recorded.
- Best-Known-Method, Hostile Cryptographer, Red-Team, Production SRE, Side-Channel Caveat, Formal-Model Mapping Residual, External-Review Readiness, Release-Claim Boundary, and Assurance Gap Review Trigger are recorded.
- No remote action occurred in NA-0522.
- No SSH execution occurred in NA-0522.
- No qsc send/receive occurred in NA-0522.
- No remote E2EE occurred in NA-0522.
- No qsl-server or qsl-attachments work was used or selected.
- No qsc source/test/fuzz/Cargo mutation occurred.
- No workflow/script/helper/dependency mutation occurred.
- No corpus/vector/input mutation occurred.
- No formal/refimpl/service/public/backup mutation occurred.
- No public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim was introduced.

## Required markers

Evidence or validation proof must contain:

- `REMOTE_E2EE_REPLAY_CORRUPT_NEGATIVE_IMPLEMENTATION_READY`
- `NA-0523 -- QSL Remote qsc E2EE Replay / Corrupt Delivery Negative Boundary Implementation Harness`
- `NA0523_REMOTE_E2EE_NEGATIVE_SCOPE_CONSUMED_OK`
- `NA0523_RETAINED_QSC_HASH_RECHECKED_OK`
- `NA0523_FORWARDING_PATH_RECHECKED_OK`
- `NA0523_BASELINE_REMOTE_E2EE_READY_FOR_NEGATIVE_OK`
- `NA0523_REPLAY_NEGATIVE_REJECTED_OK`
- `NA0523_CORRUPT_DELIVERY_REJECTED_OK`
- `NA0523_NEGATIVE_NO_MUTATION_OK`
- `NA0523_VALID_PATH_REMAINS_USABLE_OK`
- `NA0523_NO_SECRET_OUTPUT_OK`
- `NA0523_CLEANUP_COMPLETED_OK`
- `NA0523_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0523_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0523_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0523_ONE_READY_INVARIANT_OK`

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

Do not run SSH, scp, sftp, rsync, qsc send/receive, ssh-keygen, ssh-keyscan, sudo, qwork, qstart, qresume, qsl-backup, backup, or restore during NA-0522 validation.

## Static validation

Static validation must prove:
- exact five-path scope.
- READY_COUNT remains 1 and READY remains NA-0522 before the authorization PR.
- D-1033 exists once.
- D-1034 is absent before optional closeout.
- duplicate decision count is zero.
- checked-in evidence has no private key blocks.
- checked-in evidence has no private key, passphrase, password, token, credential, production endpoint, backup material, qsc vault material, or raw private qsc material.
- checked-in evidence does not introduce unsupported public, production, internet, external-review, crypto-completion, replay-completion, downgrade-completion, side-channel, vulnerability-free, bug-free, or perfect-crypto claims.
- qsl-server and qsl-attachments paths were not mutated.
- qsc source/test/fuzz/Cargo paths were not mutated.
- workflow/script/helper/dependency paths were not mutated.
- corpus/vector/input paths were not mutated.
- formal/refimpl/service/public/backup paths were not mutated.

## Acceptance classification

Expected classification:

`REMOTE_E2EE_REPLAY_CORRUPT_NEGATIVE_IMPLEMENTATION_READY`

## Successor

Expected successor:

`NA-0523 -- QSL Remote qsc E2EE Replay / Corrupt Delivery Negative Boundary Implementation Harness`

## Boundaries

This testplan does not authorize NA-0523 implementation. It does not authorize qsl-server, qsl-attachments, package installation, public service deployment, remote source checkout/build, qwork/qstart/qresume, qsl-backup execution, qsc source/test/fuzz/Cargo mutation, workflow/script/helper mutation, dependency mutation, corpus/vector/input mutation, formal/refimpl/service/public/backup mutation, or any public/production readiness claim.

