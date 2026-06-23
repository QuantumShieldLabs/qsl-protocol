Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-23

# NA-0524 quinn-proto RUSTSEC-2026-0185 Dual-Lockfile Remediation

## Executive summary

This NA-0524 remediation updates only the two affected lockfiles for RUSTSEC-2026-0185:

- root `Cargo.lock`: `quinn-proto 0.11.14` to `0.11.15`
- nested `qsl/qsl-client/qsc/fuzz/Cargo.lock`: `quinn-proto 0.11.14` to `0.11.15`

The remediation uses no audit waiver, changes no manifest, changes no qsc source/test/fuzz source, changes no workflow/script/helper, and runs no remote action. It does not close out NA-0524 and does not restore NA-0525.

Proof root: `/srv/qbuild/tmp/NA0524_quinn_proto_rustsec_2026_0185_dual_lock_remediation_20260623T013035Z`

## D420 / D421 advisory inheritance

- NA-0524 authorization PR #1320 merged at `d309fd9d10c`.
- D-1037 exists once.
- NA-0524 remains READY.
- D-1038 was absent before remediation.
- D-1039 was absent before remediation.
- Optional closeout was blocked by public-safety/advisories on RUSTSEC-2026-0185.
- The advisory was for `quinn-proto 0.11.14` with fixed target `>=0.11.15`.
- D420/D421 observed the root path `quinn-proto -> quinn -> reqwest -> qsl-tui/qsc`.
- D421 observed the nested qsc fuzz path `quinn-proto -> quinn -> reqwest -> qsc -> qsc-fuzz`.
- Root `Cargo.lock` contained `quinn-proto 0.11.14`.
- Nested `qsl/qsl-client/qsc/fuzz/Cargo.lock` contained `quinn-proto 0.11.14`.
- D421 stopped because nested lockfile remediation was not authorized in D421.
- This directive is a dedicated dual-lockfile dependency-security remediation lane.
- This directive does not close out NA-0524 and does not restore NA-0525.

## Pre-remediation dependency proof

Structured lockfile parser evidence:

- root `Cargo.lock`: one `quinn-proto` package entry at `0.11.14`.
- nested qsc fuzz `Cargo.lock`: one `quinn-proto` package entry at `0.11.14`.

Pre-remediation audit classification:

- `cargo audit --deny warnings` exited 1 for RUSTSEC-2026-0185 on `quinn-proto 0.11.14`.
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock` exited 1 for RUSTSEC-2026-0185 on `quinn-proto 0.11.14`.

Dependency tree proof:

- Plain `cargo tree -i quinn-proto` and nested equivalent exited 0 but printed no reverse path because the dependency is target-conditional in this workspace view.
- `--target all` tree proof was captured for root and nested manifests to preserve the dependency-path evidence.

Proof files:

- `remediation/pre_lock_versions.txt`
- `remediation/pre_root_cargo_tree_i_quinn_proto.txt`
- `remediation/pre_nested_cargo_tree_i_quinn_proto.txt`
- `remediation/pre_root_cargo_tree_target_all_i_quinn_proto.txt`
- `remediation/pre_nested_cargo_tree_target_all_i_quinn_proto.txt`
- `remediation/pre_root_cargo_audit.txt`
- `remediation/pre_nested_cargo_audit.txt`
- `remediation/pre_audit_classification.txt`

## Exact root remediation command

```bash
cargo update -p quinn-proto --precise 0.11.15
```

Result:

- command exit code: 0
- root `Cargo.lock` now contains `quinn-proto 0.11.15`
- no `Cargo.toml` changed
- only `Cargo.lock` changed at this step

Proof files:

- `remediation/root_cargo_update_precise_0.11.15.txt`
- `remediation/root_post_update_verify.txt`
- `remediation/root_Cargo_lock_diff.patch`

## Exact nested fuzz remediation command

```bash
cargo update --manifest-path qsl/qsl-client/qsc/fuzz/Cargo.toml -p quinn-proto --precise 0.11.15
```

Result:

- command exit code: 0
- nested `qsl/qsl-client/qsc/fuzz/Cargo.lock` now contains `quinn-proto 0.11.15`
- no `qsl/qsl-client/qsc/fuzz/Cargo.toml` changed
- no command-shape correction was needed

Proof files:

- `remediation/nested_cargo_update_precise_0.11.15.txt`
- `remediation/nested_post_update_verify.txt`
- `remediation/nested_qsc_fuzz_Cargo_lock_diff.patch`

## Root Cargo.lock diff summary

The root lockfile package change is limited to the `quinn-proto` package entry:

- `version = "0.11.14"` removed
- `version = "0.11.15"` added
- checksum updated for the `0.11.15` crate

## Nested qsc fuzz Cargo.lock diff summary

The nested fuzz lockfile package change is limited to the `quinn-proto` package entry:

- `version = "0.11.14"` removed
- `version = "0.11.15"` added
- checksum updated for the `0.11.15` crate

The lockfile package-change guard found no unrelated package name in the changed hunks.

## Post-remediation audit proof

The required audits passed after remediation:

- `cargo audit --deny warnings`
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`

Proof files:

- `validation/post_root_cargo_audit.txt`
- `validation/post_nested_cargo_audit.txt`

## qsc validation proof

The required qsc/local validation bundle passed:

- `cargo tree -i quinn-proto`
- `cargo tree --manifest-path qsl/qsl-client/qsc/fuzz/Cargo.toml -i quinn-proto`
- `cargo tree --target all -i quinn-proto`
- `cargo tree --manifest-path qsl/qsl-client/qsc/fuzz/Cargo.toml --target all -i quinn-proto`
- `cargo test -p qsc --locked --test same_host_client_to_client_e2e -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test receive_e2e -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test key_lifecycle_zeroization_expansion -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture`
- `python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics`
- `python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus`
- `python3 formal/run_model_checks.py`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Proof files are under `validation/` in the proof root. Every `.exit` file for the validation bundle is `0`.

## Scope guard

Pre-governance changed paths were limited to:

- `Cargo.lock`
- `qsl/qsl-client/qsc/fuzz/Cargo.lock`

Final intended changed paths are limited to:

- `Cargo.lock`
- `qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0524_quinn_proto_rustsec_2026_0185_dual_lock_remediation_testplan.md`
- `docs/governance/evidence/NA-0524_quinn_proto_rustsec_2026_0185_dual_lock_remediation.md`

No `NEXT_ACTIONS.md` mutation is authorized or expected.

## No remote action proof

This remediation ran no SSH, scp, sftp, rsync, qsc send/receive, remote E2EE, remote qsc command, qsl-server, qsl-attachments, qwork, qstart, qresume, qsl-backup, backup, or restore operation.

## No closeout / no NA-0525 restoration proof

NA-0524 remains READY. D-1039 remains absent. NA-0525 is not restored by this remediation. A later closeout-only directive must handle NA-0524 closeout and any NA-0525 restoration, provided post-merge public-safety/advisories are green.

## Public-safety expectation

The remediation is expected to turn the advisory gate green because both affected lockfiles now use the fixed `quinn-proto 0.11.15` crate and both local cargo audits pass without waivers.

## Release-claim boundary

This remediation is dependency-health evidence only. It makes no public-readiness claim, no production-readiness claim, no public-internet-readiness claim, no external-review-complete claim, no crypto-complete claim, no identity-complete claim, no trust-complete claim, no replay-proof claim, no downgrade-proof claim, no secret-material-complete claim, no side-channel-free claim, no vulnerability-free claim, no bug-free claim, and no perfect-crypto claim.
