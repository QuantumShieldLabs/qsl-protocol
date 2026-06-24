Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-06-24

# NA-0530 qsl remote qsc E2EE wrong-peer stale-trust retry with integrated forwarding testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Record validation for NA-0530, which recovered the D432 retained-qsc smoke command shape, verified the exact D432 `/tmp` residue paths were absent, rechecked retained remote qsc freshness with local-only output capture, wrote a qsc command manifest, and stopped before qsc E2EE because the integrated remote trigger command was quoted incorrectly.

## Expected classification

`REMOTE_E2EE_INTEGRATED_TRIGGER_QUOTING_FAILURE`

No qsc E2EE, baseline, wrong-peer negative, stale-trust negative, qsc send, or qsc receive should be claimed for this classification.

## Scope guard

Allowed implementation paths:

- `docs/governance/evidence/NA-0530_qsl_remote_qsc_e2ee_wrong_peer_stale_trust_retry_with_integrated_forwarding_harness.md`
- `tests/NA-0530_qsl_remote_qsc_e2ee_wrong_peer_stale_trust_retry_with_integrated_forwarding_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No qsc source/test/fuzz/Cargo, workflow/script/helper, dependency/lockfile, corpus/vector/input, formal/refimpl/service/public/backup, qsl-server, or qsl-attachments path may change.

## Required evidence markers

- `NA0530_D432_TMP_RESIDUE_CHECK_STARTED_OK`
- `NA0530_D432_TMP_RESIDUE_TMP_ABSENT_OR_REMOVED_OK`
- `NA0530_D432_TMP_RESIDUE_ERR_ABSENT_OR_REMOVED_OK`
- `NA0530_D432_TMP_RESIDUE_CLEANUP_OK`
- `REMOTE_E2EE_INTEGRATED_TRIGGER_QUOTING_FAILURE`
- `REMOTE_E2EE_WRONG_PEER_NEGATIVE_DEFERRED_PRECHECK_FAILURE`
- `REMOTE_E2EE_STALE_TRUST_NEGATIVE_DEFERRED_PRECHECK_FAILURE`
- `LOCAL_SENSITIVE_RUNTIME_ABSENT_OK`
- `LOCAL_PORT_39176_CLOSED_OK`
- `PROOF_PROCESS_ABSENT_OK`
- `REMOTE_E2EE_ROOT_ABSENT_OK`
- `D432_RESIDUE_ABSENT_OK`

## Validation commands

Required local validation for the implementation PR:

```bash
git diff --check
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

Additional static checks:

- exact five-path scope guard
- queue/decision parser: READY_COUNT 1, READY NA-0530, D-1050 once, D-1051 absent, duplicate decision IDs zero
- link-check
- leak/private-material scan
- overclaim scan
- docs/governance classifier
- PR body preflight
- goal-lint

## Acceptance assertions

- D432 residue cleanup checked only the two exact D432 paths and both were absent or removed.
- Remote retained qsc hash matched `6bf9e59fdae397c2e0f88538d700cccbee80d229c6a979cc79555e39fea2b4f7`.
- Remote retained qsc `--help` used local-only stdout/stderr capture.
- Command manifest was written before integrated forwarding precheck.
- Remote boundary check passed.
- Integrated listener bound on `127.0.0.1:39176`.
- Dedicated-key reverse-forward process stayed alive with `ExitOnForwardFailure=yes`.
- Remote trigger failed from command quoting before marker traversal.
- No qsc E2EE command ran.
- No qsc send/receive command ran.
- No baseline E2EE setup ran.
- Wrong-peer and stale-trust negatives were deferred because the precheck failed.
- Cleanup proof passed.
- qsl-server and qsl-attachments were not used.
- No public-readiness, production-readiness, public-internet-readiness, crypto-complete, identity-complete, trust-complete, replay-proof, downgrade-proof, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim is introduced.

## Successor expectation

The selected successor candidate is `NA-0531 -- QSL Remote qsc E2EE Integrated Trigger Quoting Remediation Harness`. Closeout must not invent a queue block without explicit approved successor text.
