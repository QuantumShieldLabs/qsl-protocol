Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-22

# NA-0523 QSL Remote qsc E2EE Replay / Corrupt Delivery Negative Boundary Implementation Testplan

## Purpose

Validate that NA-0523 consumed D418 and NA-0522/D417 inheritance, rechecked retained qsc and forwarding boundaries, executed bounded remote qsc E2EE replay and corrupt-delivery negative tests with synthetic data, proved selected no-mutation and valid-path usability, cleaned up local/remote sensitive runtime, and preserved no qsl-server/qsl-attachments and no public/production readiness boundaries.

## Scope guard

Allowed mutation paths:
- `docs/governance/evidence/NA-0523_qsl_remote_qsc_e2ee_replay_corrupt_negative_boundary_implementation_harness.md`
- `tests/NA-0523_qsl_remote_qsc_e2ee_replay_corrupt_negative_boundary_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No qsc source/test/fuzz/Cargo, workflow/script/helper, dependency/lockfile, corpus/vector/input, formal/refimpl/service/public/backup, qsl-server, qsl-attachments, qshield, qshield-cli, qsl-backup, public docs, website, archive, move, delete, or backup path mutation is allowed.

## Required evidence checks

- qwork proof files were read and verified; qwork/qstart/qresume were not run.
- Proof HEAD and proof origin/main matched live pre-fetch refs.
- READY_COUNT was 1 and READY was NA-0523.
- NA-0522 and NA-0521 were DONE.
- D-1033 and D-1034 existed once.
- D-1035 was absent before the patch and exists once after the patch.
- Duplicate decision count is zero using the `- **ID:** D-####` parser.
- D418 response was consumed as a procedural startup stop, not an E2EE result.
- D417/NA-0522 inheritance was consumed with classification `REMOTE_E2EE_REPLAY_CORRUPT_NEGATIVE_IMPLEMENTATION_READY`.
- Command manifest exists under the proof root.
- Retained remote qsc path/owner/digest/help were rechecked.
- Dedicated reverse-forwarding path was rechecked.
- Local qsc was built or selected from clean current checkout and smoked.
- Relevant qsc source/Cargo/test paths had no drift from retained source baseline.
- Isolated local and remote runtime roots were used.
- Baseline E2EE setup reached replay/corrupt negative test points.
- Replay negative was executed and rejected with replay markers.
- Corrupt delivery negative was executed and rejected with qsp decode/verify failure markers.
- Selected session/output state did not mutate across executed negatives.
- Valid path remained usable after replay and corrupt negatives.
- qsc stdout/stderr and proof root were scanned for private key/passphrase/token/password material.
- Remote E2EE root was removed.
- Local sensitive runtime root was removed.
- Local relay and SSH forward were stopped.
- No qsl-server was used.
- No qsl-attachments was used.
- No public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim was introduced.

## Required markers

Evidence or proof must contain:

- `REMOTE_E2EE_REPLAY_CORRUPT_NEGATIVES_PASS`
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

## Static validation

Static validation must prove:
- exact five-path scope;
- READY_COUNT remains 1 and READY remains NA-0523 before the implementation PR;
- D-1035 exists once;
- D-1036 is absent before optional closeout;
- duplicate decision count is zero;
- checked-in evidence has no private key blocks;
- checked-in evidence has no private key, passphrase, password, token, credential, production endpoint, backup material, qsc vault material, authorized_keys dump, known_hosts dump, or raw private qsc material;
- local sensitive runtime root was deleted or a STOP reason records it;
- remote E2EE root was deleted or a STOP reason records it;
- checked-in evidence does not introduce unsupported public, production, internet, external-review, crypto-completion, replay-completion, downgrade-completion, side-channel, vulnerability-free, bug-free, or perfect-crypto claims;
- qsl-server and qsl-attachments paths were not mutated;
- qsc source/test/fuzz/Cargo paths were not mutated;
- workflow/script/helper/dependency paths were not mutated;
- corpus/vector/input paths were not mutated;
- formal/refimpl/service/public/backup paths were not mutated.

## Acceptance classification

Expected classification:

`REMOTE_E2EE_REPLAY_CORRUPT_NEGATIVES_PASS`

## Successor

Expected successor after successful closeout:

`NA-0524 -- QSL Remote qsc E2EE Wrong-Peer / Stale-Trust Negative Scope Authorization Plan`

## Boundaries

This testplan does not authorize qsl-server, qsl-attachments, package installation, public service deployment, remote source checkout/build, qwork/qstart/qresume, qsl-backup execution, qsc source/test/fuzz/Cargo mutation, workflow/script/helper mutation, dependency mutation, corpus/vector/input mutation, formal/refimpl/service/public/backup mutation, or any public/production readiness claim.
