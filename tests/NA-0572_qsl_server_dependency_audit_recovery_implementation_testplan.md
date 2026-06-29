Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0572 qsl-server Dependency Audit Recovery Implementation Testplan

## Objective

Verify that NA-0572 consumes D-1132/D-1133, applies the exact qsl-server
lockfile-only dependency audit recovery, merges the qsl-server PR, records
qsl-protocol governance evidence, and preserves all no-deployment/no-remote/
no-qsc/no-qsl-attachments boundaries.

## Required Markers

- NA0572_D1132_AUTHORIZATION_CONSUMED_OK
- NA0572_D1133_CLOSEOUT_CONSUMED_OK
- NA0572_FRESH_QWORK_PROOF_OK
- NA0572_QSL_PROTOCOL_MAIN_CHECKS_CLASSIFIED_OK
- NA0572_QSL_SERVER_CHECKOUT_OK
- NA0572_QSL_SERVER_BASELINE_AUDIT_CLASSIFIED_OK
- NA0572_QSL_SERVER_LOCKFILE_ONLY_UPDATE_OK
- NA0572_QSL_SERVER_CARGO_TOML_UNCHANGED_OK
- NA0572_QSL_SERVER_SOURCE_UNCHANGED_OK
- NA0572_QSL_SERVER_WORKFLOW_UNCHANGED_OK
- NA0572_QSL_SERVER_AUDIT_PASS_OK
- NA0572_QSL_SERVER_METADATA_PASS_OK
- NA0572_QSL_SERVER_BUILD_PASS_OK
- NA0572_QSL_SERVER_TEST_PASS_OK
- NA0572_QSL_SERVER_PR_MERGED_OR_ALREADY_FIXED_OK
- NA0572_QSL_SERVER_POSTMERGE_VERIFIED_OK
- NA0572_NO_QSL_SERVER_DEPLOYMENT_OK
- NA0572_NO_QSL_ATTACHMENTS_OK
- NA0572_NO_REMOTE_ACTION_OK
- NA0572_NO_QSC_SEND_RECEIVE_OK
- NA0572_NO_WORKFLOW_DISPATCH_OK
- NA0572_NO_QSL_PROTOCOL_SOURCE_MUTATION_OK
- NA0572_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0572_SUCCESSOR_SELECTED_OK
- NA0572_ONE_READY_INVARIANT_OK

## Verification Expectations

- D-1132 exists once and is Accepted.
- D-1133 exists once and is Accepted.
- D-1134 is added exactly once by this implementation patch.
- D-1135 remains absent before optional closeout.
- qwork proof is fresh and matches live pre-fetch qsl-protocol HEAD and
  origin/main.
- qsl-protocol current-main public-safety, advisories, suite2-vectors, and
  required checks are classified before qsl-server mutation.
- qsl-server checkout is clean and reconciled with D-1132/D-1133.
- qsl-server baseline audit is classified as the expected
  `RUSTSEC-2026-0185` / `quinn-proto 0.11.14` failure.
- qsl-server source-of-truth mutation is `Cargo.lock` only.
- qsl-server `Cargo.toml`, source, and workflow paths remain unchanged.
- qsl-server cargo audit, locked metadata, build, test, and fmt check pass
  after remediation and after merge.
- qsl-server PR #57 is merged, or an already-fixed proof exists.
- No qsl-server deployment or service run occurs.
- No qsl-attachments work occurs.
- No remote action, qsc send/receive, workflow dispatch/rerun, qsl-protocol
  source mutation, public-site mutation, Cloudflare mutation, or private-material
  publication occurs.
- NA-0573 is selected as the exact successor for a later closeout.
- Exactly one READY remains mandatory until closeout mutates NEXT_ACTIONS.

## Validation Commands

- `git diff --check`
- exact five-path qsl-protocol implementation scope guard
- no qsl-protocol source/script/workflow/dependency path change
- qsl-server source-of-truth mutation path guard: `Cargo.lock` only
- qsl-server PR/merge evidence proof
- queue/decision proof
- marker proof
- deterministic markdown link check
- added-line/new-file private-material scan
- qsl-server PR/evidence private-material scan
- overclaim scan
- docs/governance-only classifier with qsl-server lockfile recovery allowance
- PR body preflight
- goal-lint if available
- root `cargo audit --deny warnings`
- nested qsc fuzz `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo metadata --locked --format-version=1`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Focused qsc runtime tests may be skipped because NA-0572 does not mutate
qsl-protocol qsc source/runtime/dependency/workflow paths and qsc send/receive
is not authorized.
