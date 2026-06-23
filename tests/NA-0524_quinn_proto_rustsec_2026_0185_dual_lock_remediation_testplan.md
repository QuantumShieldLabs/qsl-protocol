Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-23

# NA-0524 quinn-proto RUSTSEC-2026-0185 Dual-Lockfile Remediation Testplan

## Purpose

Validate the narrow NA-0524 dependency-security remediation for RUSTSEC-2026-0185 without closing NA-0524, restoring NA-0525, changing manifests, changing qsc source/test/fuzz source, or weakening advisory gates.

## Scope guard

Allowed mutation paths:

- `Cargo.lock`
- `qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0524_quinn_proto_rustsec_2026_0185_dual_lock_remediation_testplan.md`
- `docs/governance/evidence/NA-0524_quinn_proto_rustsec_2026_0185_dual_lock_remediation.md`

Forbidden mutation paths include `NEXT_ACTIONS.md`, any `Cargo.toml`, qsc source/test/fuzz source, workflow/script/helper files, corpus/vector/input files, formal/refimpl/service/public/backup paths, qsl-server, qsl-attachments, qshield, qshield-cli, qsl-backup, backup status/plan files, qwork, qstart, and qresume.

## Required evidence checks

- qwork proof files were read and verified; qwork/qstart/qresume were not run.
- Proof HEAD and proof origin/main matched live pre-fetch refs.
- READY_COUNT was 1 and READY was NA-0524 before mutation.
- NA-0523, NA-0522, and NA-0521 were DONE.
- D-1037 existed once before mutation.
- D-1038 and D-1039 were absent before mutation.
- Duplicate decision count was zero using the `- **ID:** D-####` parser.
- D420 and D421 response files existed and were consumed.
- Current main advisory failure was RUSTSEC-2026-0185 for `quinn-proto 0.11.14`, fixed by `>=0.11.15`.
- Root `Cargo.lock` contained `quinn-proto 0.11.14` before remediation.
- Nested `qsl/qsl-client/qsc/fuzz/Cargo.lock` contained `quinn-proto 0.11.14` before remediation.
- Root pre-remediation cargo audit failed for RUSTSEC-2026-0185.
- Nested qsc fuzz lock pre-remediation cargo audit failed for RUSTSEC-2026-0185.
- Root remediation command was exactly `cargo update -p quinn-proto --precise 0.11.15`.
- Nested remediation command was exactly `cargo update --manifest-path qsl/qsl-client/qsc/fuzz/Cargo.toml -p quinn-proto --precise 0.11.15`.
- Root `Cargo.lock` changed `quinn-proto` from `0.11.14` to `0.11.15`.
- Nested qsc fuzz `Cargo.lock` changed `quinn-proto` from `0.11.14` to `0.11.15`.
- No `Cargo.toml` changed.
- No qsc source/test/fuzz source changed.
- No workflow/script/helper changed.
- No corpus/vector/input changed.
- No formal/refimpl/service/public/backup path changed.
- No qsl-server, qsl-attachments, qshield, or qshield-cli path changed.
- No remote action, SSH, scp, sftp, rsync, qsc send/receive, remote E2EE, qsl-backup, backup, or restore was run.
- NA-0524 remains READY after the remediation.
- D-1038 exists once after the governance patch.
- D-1039 remains absent.
- NA-0525 is not restored by this remediation.

## Required local validation

Run:

```bash
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i quinn-proto
cargo tree --manifest-path qsl/qsl-client/qsc/fuzz/Cargo.toml -i quinn-proto
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

`cargo tree -i quinn-proto` may print no reverse path for target-conditional dependencies; if so, also record `cargo tree --target all -i quinn-proto` and the nested `--target all` equivalent as dependency-path proof.

## PR validation

Before PR creation:

- exact seven-path scope guard passes;
- D-1038 exists once;
- D-1039 is absent;
- READY_COUNT is 1 and READY is NA-0524;
- NA-0525 is not READY;
- duplicate decision count is zero;
- PR body includes `Goals: G4` near the top;
- PR body records impact, no-regression, tests/vectors, no audit waiver, no remote action, no closeout, and no NA-0525 restoration.

Merge only after required PR checks pass. Use a merge commit only.

## Post-merge validation

After merge:

- `main` equals `origin/main`.
- Root `Cargo.lock` has `quinn-proto 0.11.15`.
- Nested qsc fuzz `Cargo.lock` has `quinn-proto 0.11.15`.
- D-1038 exists once.
- D-1039 is absent.
- READY remains NA-0524.
- public-safety succeeds on the remediation merge commit.
- advisories succeeds on the remediation merge commit.
- No required red checks remain.

## Release-claim boundary

This remediation is dependency-health evidence only. It makes no public-readiness claim, no production-readiness claim, no public-internet-readiness claim, no external-review-complete claim, no crypto-complete claim, no identity-complete claim, no trust-complete claim, no replay-proof claim, no downgrade-proof claim, no secret-material-complete claim, no side-channel-free claim, no vulnerability-free claim, no bug-free claim, and no perfect-crypto claim.
