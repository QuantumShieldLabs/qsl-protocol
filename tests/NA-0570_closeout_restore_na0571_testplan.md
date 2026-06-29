Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0570 Closeout and NA-0571 Restoration Testplan

## Objective

Verify that NA-0570 is closed only after D-1130 merged, post-merge checks passed,
and the exact NA-0571 qsl-server source/build recovery authorization successor is
restored as the sole READY item without implementing NA-0571.

## Required Markers

- NA0570_CLOSEOUT_IMPLEMENTATION_PR_MERGED_OK
- NA0570_CLOSEOUT_D1130_ACCEPTED_OK
- NA0570_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0570_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0570_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK
- NA0570_CLOSEOUT_D1131_RESTORED_NA0571_OK
- NA0570_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK
- NA0570_CLOSEOUT_NO_NA0571_IMPLEMENTATION_OK
- NA0570_CLOSEOUT_NO_QSL_SERVER_MUTATION_OK
- NA0570_CLOSEOUT_NO_QSL_SERVER_DEPLOYMENT_OK
- NA0570_CLOSEOUT_NO_QSL_ATTACHMENTS_OK
- NA0570_CLOSEOUT_NO_REMOTE_ACTION_OK
- NA0570_CLOSEOUT_NO_SSH_SCP_TAILSCALE_OK
- NA0570_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK
- NA0570_CLOSEOUT_NO_WORKFLOW_DISPATCH_OK
- NA0570_CLOSEOUT_NO_SOURCE_MUTATION_OK
- NA0570_CLOSEOUT_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
- NA0570_CLOSEOUT_NO_PUBLIC_SITE_MUTATION_OK
- NA0570_CLOSEOUT_NO_CLOUDFLARE_MUTATION_OK
- NA0570_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0570_CLOSEOUT_ONE_READY_INVARIANT_OK

## Verification Expectations

- Implementation PR #1413 merged at `d1289d754c7a` from head `464166a50202`.
- D-1130 exists once and is Accepted.
- D-1131 is added exactly once by closeout.
- D-1132 remains absent.
- NA-0570 is DONE.
- READY_COUNT is 1.
- READY item is NA-0571.
- NA-0571 block exactly restores the approved qsl-server source/build recovery
  authorization successor with no placeholders.
- No NA-0571 implementation occurs.
- No qsl-server mutation, qsl-server deployment, qsl-attachments work, remote
  action, SSH, scp, Tailscale, qsc send/receive, workflow dispatch/rerun,
  source mutation, dependency/lockfile mutation, public-site mutation,
  Cloudflare mutation, or private-material publication occurs.

## Validation Commands

- `git diff --check`
- exact five-path closeout scope guard
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
