Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0570 qsl-server Source-Audit Stop Handoff Testplan

## Objective

Verify that NA-0570 records the D493 qsl-server source-audit stop, preserves
fail-closed source/build/remote boundaries, selects NA-0571 as the exact
authorization-only source/build recovery successor, and keeps exactly one READY
item.

## Required Markers

- NA0570_D1128_AUTHORITY_CONSUMED_OK
- NA0570_D1129_CLOSEOUT_CONSUMED_OK
- NA0570_D493_SOURCE_AUDIT_STOP_CONSUMED_OK
- NA0570_FRESH_QWORK_PROOF_OK
- NA0570_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0570_QSL_SERVER_AUDIT_FINDING_RECORDED_OK
- NA0570_QSL_SERVER_BUILD_SKIPPED_RECORDED_OK
- NA0570_REMOTE_RECOVERY_NOT_ATTEMPTED_OK
- NA0570_SOURCE_AUDIT_STOP_ACCEPTED_OK
- NA0570_NA0571_SUCCESSOR_SELECTED_OK
- NA0570_NO_REMOTE_ACTION_OK
- NA0570_NO_SSH_SCP_TAILSCALE_OK
- NA0570_NO_QSC_SEND_RECEIVE_OK
- NA0570_NO_QSL_SERVER_MUTATION_OK
- NA0570_NO_QSL_ATTACHMENTS_OK
- NA0570_NO_WORKFLOW_DISPATCH_OK
- NA0570_NO_SOURCE_MUTATION_OK
- NA0570_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
- NA0570_NO_PUBLIC_SITE_MUTATION_OK
- NA0570_NO_CLOUDFLARE_MUTATION_OK
- NA0570_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0570_ONE_READY_INVARIANT_OK

## Verification Expectations

- D-1128 exists once and is Accepted.
- D-1129 exists once and is Accepted.
- D493 source-audit stop evidence is consumed.
- D-1130 is added exactly once by this implementation patch.
- D-1131 remains absent before optional closeout.
- qwork proof is fresh and matches live pre-fetch HEAD and origin/main.
- Current main checks are classified before mutation.
- qsl-server source commit `d40e6003fdf0` is recorded.
- qsl-server audit finding `RUSTSEC-2026-0185` for `quinn-proto 0.11.14` via
  the qsl-server dev dependency path through reqwest/quinn is recorded.
- qsl-server build skipped status is recorded.
- Remote recovery was not attempted.
- Result classification is `QSL_SERVER_RECOVERY_SOURCE_AUDIT_STOP_ACCEPTED`.
- NA-0571 is selected as the exact successor.
- No remote action, SSH, scp, Tailscale, qsc send/receive, qsl-server mutation,
  qsl-attachments work, workflow dispatch, source mutation, dependency/lockfile
  mutation, public-site mutation, Cloudflare mutation, or private-material
  publication occurs.
- Exactly one READY remains mandatory.

## Validation Commands

- `git diff --check`
- exact five-path implementation scope guard
- queue/decision proof
- marker proof
- deterministic markdown link check
- added-line/new-file private-material scan
- added-line overclaim scan
- docs/governance-only classifier
- PR body preflight
- goal-lint if available
- root `cargo audit --deny warnings`
- nested qsc fuzz `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo metadata --locked --format-version=1`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Focused qsc runtime tests may be skipped because this lane is a governance-only
source-audit stop handoff with no qsl-protocol source, runtime, dependency,
workflow, executable test, fuzz target, or vector mutation.
