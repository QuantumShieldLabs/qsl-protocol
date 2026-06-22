Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-22

# NA-0524 QSL Remote qsc E2EE Wrong-Peer / Stale-Trust Negative Scope Authorization Testplan

## Purpose

Validate that NA-0524 is authorization-only, consumes NA-0523 / D419, inventories remote qsc E2EE identity/trust residuals, selects the exact NA-0525 implementation lane, and preserves no-remote-action, no-service-integration, no-code-mutation, cleanup, redaction, and public-claim boundaries.

## Scope guard

Allowed mutation paths:

- `docs/governance/evidence/NA-0524_qsl_remote_qsc_e2ee_wrong_peer_stale_trust_negative_scope_authorization_plan.md`
- `tests/NA-0524_qsl_remote_qsc_e2ee_wrong_peer_stale_trust_negative_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No qsc source/test/fuzz/Cargo, workflow/script/helper, dependency/lockfile, corpus/vector/input, formal/refimpl/service/public/backup, qsl-server, qsl-attachments, qshield, qshield-cli, qsl-backup, public docs, website, archive, move, delete, or backup path mutation is allowed.

## Required evidence checks

- qwork proof files were read and verified; qwork/qstart/qresume were not run.
- Proof HEAD and proof origin/main matched live pre-fetch refs.
- READY_COUNT was 1 and READY was NA-0524.
- NA-0523, NA-0522, and NA-0521 were DONE.
- D-1035 and D-1036 existed once.
- D-1037 was absent before the patch and exists once after the patch.
- Duplicate decision count is zero using the `- **ID:** D-####` parser.
- D419 response was consumed.
- NA-0523 evidence and testplan were consumed.
- Classification `REMOTE_E2EE_REPLAY_CORRUPT_NEGATIVES_PASS` was inherited.
- Residual inventory includes wrong-peer receive/send, stale public/trust material, wrong-device/replaced-peer identity, missing trust/downgrade, repeated-run cleanup, retained qsc freshness, forwarding cleanup, route-token/capability redaction, no-secret-output review, remote root cleanup/retention, scheduled remote CI residuals, qsl-server/qsl-attachments deferral, and public/production claim boundary.
- Options 1 through 9 were reviewed.
- Selected classification is `REMOTE_E2EE_WRONG_PEER_STALE_TRUST_IMPLEMENTATION_READY`.
- Selected successor is `NA-0525 -- QSL Remote qsc E2EE Wrong-Peer / Stale-Trust Negative Implementation Harness`.
- Future command family, proof/redaction rules, stop conditions, markers, and scope bundle are recorded.
- Best-Known-Method, Hostile Cryptographer, Red-Team, Production SRE, Side-Channel Caveat, Formal-Model Mapping Residual, External-Review Readiness, Release-Claim Boundary, and Assurance Gap Review Trigger are recorded.
- No remote action occurred in NA-0524.
- No SSH execution occurred in NA-0524.
- No qsc send/receive occurred in NA-0524.
- No remote E2EE occurred in NA-0524.
- No qsl-server or qsl-attachments work was used or selected.
- No qsc source/test/fuzz/Cargo mutation occurred.
- No workflow/script/helper/dependency mutation occurred.
- No corpus/vector/input mutation occurred.
- No formal/refimpl/service/public/backup mutation occurred.
- No public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, identity-complete, trust-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim was introduced.

## Required markers

Evidence or validation proof must contain:

- `REMOTE_E2EE_WRONG_PEER_STALE_TRUST_IMPLEMENTATION_READY`
- `NA-0525 -- QSL Remote qsc E2EE Wrong-Peer / Stale-Trust Negative Implementation Harness`
- `NA0525_REMOTE_E2EE_IDENTITY_TRUST_NEGATIVE_SCOPE_CONSUMED_OK`
- `NA0525_RETAINED_QSC_HASH_RECHECKED_OK`
- `NA0525_FORWARDING_PATH_RECHECKED_OK`
- `NA0525_BASELINE_REMOTE_E2EE_READY_FOR_NEGATIVE_OK`
- `NA0525_WRONG_PEER_NEGATIVE_REJECTED_OK`
- `NA0525_STALE_TRUST_NEGATIVE_REJECTED_OK`
- `NA0525_NEGATIVE_NO_MUTATION_OK`
- `NA0525_VALID_PATH_REMAINS_USABLE_OK`
- `NA0525_NO_SECRET_OUTPUT_OK`
- `NA0525_CLEANUP_COMPLETED_OK`
- `NA0525_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0525_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0525_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0525_ONE_READY_INVARIANT_OK`

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

The evidence PR must prove:

- changed paths are limited to the five allowed mutation paths;
- D-1037 exists once and D-1038 is absent;
- duplicate decision count is zero;
- READY_COUNT remains 1 and READY remains NA-0524 before the authorization PR;
- link-check passes;
- leak scan passes;
- added-line overclaim scan passes;
- PR body preflight passes;
- goal-lint passes;
- marker proof passes;
- checked-in evidence contains no private-key block headers;
- checked-in evidence contains no private key, passphrase, token, password, production endpoint, backup material, or personal data.

## Acceptance classification

Expected classification:

`REMOTE_E2EE_WRONG_PEER_STALE_TRUST_IMPLEMENTATION_READY`

Expected selected successor:

`NA-0525 -- QSL Remote qsc E2EE Wrong-Peer / Stale-Trust Negative Implementation Harness`

## Non-authorization boundary

This testplan does not authorize NA-0525 implementation. It does not authorize SSH, scp, sftp, rsync, qsc send/receive, remote E2EE, qsl-server, qsl-attachments, package installation, public service deployment, remote source checkout/build, qwork/qstart/qresume, qsl-backup execution, qsc source/test/fuzz/Cargo mutation, workflow/script/helper mutation, dependency mutation, corpus/vector/input mutation, formal/refimpl/service/public/backup mutation, or any public/production readiness claim.
