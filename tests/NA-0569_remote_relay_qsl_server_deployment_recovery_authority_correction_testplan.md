Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0569 Remote Relay qsl-server Deployment Recovery Authority Correction Testplan

## Objective

Verify that NA-0569 is authorization-only, consumes D-1126/D-1127, corrects the
qsc relay command discovery successor to a qsl-server-centered recovery
successor, and preserves the one-READY invariant without remote, runtime,
source, workflow, public-site, Cloudflare, or private-material mutation.

## Required Markers

- NA0569_D1126_CONSUMED_OK
- NA0569_D1127_CONSUMED_OK
- NA0569_D490_STOP_CONSUMED_OK
- NA0569_FRESH_QWORK_PROOF_OK
- NA0569_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0569_RUSTSEC_ANYHOW_RECOVERED_OK
- NA0569_ROOT_CARGO_AUDIT_PASS_OK
- NA0569_NESTED_QSC_FUZZ_AUDIT_PASS_OK
- NA0569_DEPENDENCY_SCOPE_BOUNDED_OK
- NA0569_ARCHITECTURE_CORRECTION_RECORDED_OK
- NA0569_QSC_CLIENT_ROLE_RECORDED_OK
- NA0569_QSL_SERVER_RELAY_ROLE_RECORDED_OK
- NA0569_QSL_ATTACHMENTS_BOUNDARY_RECORDED_OK
- NA0569_QSC_RELAY_COMMAND_SUCCESSOR_CORRECTED_OK
- NA0569_QSL_SERVER_RECOVERY_AUTHORITY_SELECTED_OK
- NA0569_NA0570_COMMAND_ALLOWLIST_SELECTED_OK
- NA0569_NA0570_PRIVATE_MATERIAL_POLICY_SELECTED_OK
- NA0569_NA0570_DECISION_TREE_SELECTED_OK
- NA0569_NO_REMOTE_ACTION_OK
- NA0569_NO_SSH_TAILSCALE_REMOTE_COMMAND_OK
- NA0569_NO_QSC_COMMAND_EXECUTION_OK
- NA0569_NO_QSL_SERVER_COMMAND_EXECUTION_OK
- NA0569_NO_QSL_ATTACHMENTS_COMMAND_EXECUTION_OK
- NA0569_NO_WORKFLOW_DISPATCH_OK
- NA0569_NO_SOURCE_MUTATION_OK
- NA0569_NO_ACCOUNT_SERVICE_MUTATION_OK
- NA0569_NO_PUBLIC_SITE_MUTATION_OK
- NA0569_NO_CLOUDFLARE_MUTATION_OK
- NA0569_NO_SECRET_VALUES_PUBLISHED_OK
- NA0569_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0569_ONE_READY_INVARIANT_OK

## Verification Expectations

- D-1126 exists once and is Accepted.
- D-1127 exists once and is Accepted.
- D490 stop evidence was consumed.
- D-1128 is added exactly once by NA-0569.
- D-1129 remains absent before optional closeout.
- NA-0568 and NA-0567 are DONE.
- NA-0569 remains the sole READY item until optional closeout.
- `RUSTSEC-2026-0190` for `anyhow 1.0.100` is recovered.
- Root cargo audit passes after remediation.
- Nested qsc fuzz cargo audit passes after remediation.
- Dependency scope is bounded to root `Cargo.lock` only.
- D-1128 records the architecture correction: qsc is client/demo CLI,
  qsl-server is relay/server, and qsl-attachments is separate.
- D-1128 selects NA-0570 as the exact successor.
- No remote action, qsc command, qsl-server command, qsl-attachments command,
  workflow dispatch, source mutation, account/service mutation, public-site
  mutation, Cloudflare mutation, or private-material publication occurs.

## Validation Commands

- `git diff --check`
- exact implementation scope guard
- dependency diff guard
- queue/decision proof
- marker proof
- link-check
- added-line/new-file private-material scan
- overclaim scan
- docs/governance/dependency-recovery classifier
- PR body preflight
- goal-lint if available
- root cargo audit
- nested qsc fuzz lock cargo audit
- `cargo metadata --locked --format-version=1`
- `cargo tree -i anyhow`
- `cargo check --workspace --locked`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Focused qsc runtime tests may be skipped because NA-0569 mutates no qsc source,
runtime, workflow, executable test, fuzz target, or vector. The only dependency
mutation is the root `Cargo.lock` `anyhow` advisory remediation, covered by
cargo audit, metadata, workspace check, and required CI.
