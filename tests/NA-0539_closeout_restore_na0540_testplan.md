Status: Supporting
Owner: Verification
Last-Updated: 2026-06-25

# NA-0539 Closeout / NA-0540 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Validate that NA-0539 is closed only after D-1068 implementation PR #1351 merged, post-merge public-safety/advisories completed success, and the Lead Director successor override is recorded. Confirm that exactly one successor, NA-0540, is restored as READY for daily public Progress cadence authorization without implementing NA-0540.

## Required startup gates

- qwork proof files exist at `/srv/qbuild/work/NA-0539/.qwork/startup.qsl-protocol.kv` and `/srv/qbuild/work/NA-0539/.qwork/startup.qsl-protocol.json`.
- qwork proof records `startup_result=OK`, lane `NA-0539`, repo `qsl-protocol`, path `/srv/qbuild/work/NA-0539/qsl-protocol`, clean worktree/index/untracked state, HEAD equals origin/main, READY_COUNT 1, queue top READY `NA-0539`, and requested lane status READY.
- qwork proof written time is after the D451 response timestamp.
- Live pre-fetch HEAD and origin/main match qwork proof.
- `/` disk usage is below the 95% stop threshold before fetch.
- origin/main equals or descends from `bf9faadad5af`.

## Required inheritance gates

- D451 implementation PR #1351 merged at `bf9faadad5af`.
- D451 classification is `PUBLIC_EVIDENCE_SYNC_IMPLEMENTATION_PASS`.
- D-1068 exists once.
- README.md and selected docs/public files were synchronized by D451.
- D-1066 path bundle and claim policy were applied.
- No `public/` or `website/` path was created.
- No raw proof logs or private material were published.
- No qsc source/workflow/dependency mutation occurred.
- No qsl-server or qsl-attachments use occurred.
- No remote action occurred.
- No public/production/security-completion claim was made.
- D451 selected an SSD-hygiene authorization successor.
- Lead Director supersedes that immediate successor with daily public Progress cadence authorization.
- Formal SSD/shared-target governance remains deferred, not rejected.

## Required green gate

- PR #1351 state is merged.
- PR #1351 merge commit is `bf9faadad5af`; full-SHA API proof is stored under the directive proof root.
- Check-runs for the merge commit show public-safety completed success.
- Check-runs for the merge commit show advisories completed success.
- Required checks show no failed required checks.
- No relevant completed red closeout blocker is present.

## Required closeout changes

- `NEXT_ACTIONS.md` marks NA-0539 DONE.
- `NEXT_ACTIONS.md` restores exactly one READY item: `NA-0540 -- QSL Daily Public Progress Update Cadence Authorization Plan`.
- `DECISIONS.md` adds D-1069 for NA-0539 closeout, Lead Director successor override, SSD/shared-target deferral, and NA-0540 restoration.
- `TRACEABILITY.md` maps D-1068, D-1069, PR #1351, D451/D450/D449/D448/D446 inheritance, override proof, and NA-0540 restoration.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` records qwork proof, inheritance, green-gate, validation, closeout, override, SSD context, and boundary evidence.
- This testplan records the closeout validation scope.

## Required markers

- `NA0539_CLOSEOUT_PR1351_MERGED_OK`
- `NA0539_CLOSEOUT_D1068_ACCEPTED_OK`
- `NA0539_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0539_CLOSEOUT_ADVISORIES_GREEN_OK`
- `NA0539_CLOSEOUT_PUBLIC_SYNC_PASS_CONSUMED_OK`
- `NA0539_CLOSEOUT_DIRECTOR_SUCCESSOR_OVERRIDE_RECORDED_OK`
- `NA0539_CLOSEOUT_SSD_GOVERNANCE_DEFERRED_NOT_REJECTED_OK`
- `NA0539_CLOSEOUT_D1069_RESTORED_NA0540_OK`
- `NA0539_CLOSEOUT_NO_NA0540_IMPLEMENTATION_OK`
- `NA0539_CLOSEOUT_NO_PUBLIC_DOC_MUTATION_OK`
- `NA0539_CLOSEOUT_NO_LOCAL_OPS_MUTATION_OK`
- `NA0539_CLOSEOUT_NO_REMOTE_ACTION_OK`
- `NA0539_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0539_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0539_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0539_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Post-patch validation

- `git diff --check`
- exact five-path scope guard including untracked files
- queue/decision proof:
  - READY_COUNT 1
  - READY NA-0540
  - NA-0539 DONE
  - D-1068 once
  - D-1069 once
  - D-1070 absent
  - duplicate decision count zero
- local markdown link check
- added-line/new-file private-material scan
- added-line/new-file overclaim scan with negation and forbidden-scope handling
- docs-only classifier
- marker proof
- PR body preflight
- goal-lint
- `cargo audit --deny warnings`
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

## Boundary assertions

- No NA-0540 implementation occurs.
- No public Progress content update occurs.
- No README or docs/public content mutation occurs.
- No local-ops script, unit, timer, service, or maintenance configuration mutation occurs.
- No remote-host action, SSH execution, qsc send/receive, remote E2EE, qsc protocol command, qsl-server, qsl-attachments, qwork/qstart/qresume, qsl-backup, dependency/lockfile mutation, qsc source/test/fuzz/Cargo mutation, workflow/script/helper mutation, corpus/vector/input mutation, or formal/refimpl/service/public/backup mutation occurs.
- No public-readiness claim is made.
- No production-readiness claim is made.
- No public-internet-readiness claim is made.
- No external-review-complete claim is made.
- No crypto-complete claim is made.
- No identity-complete claim is made.
- No trust-complete claim is made.
- No replay-proof claim is made.
- No downgrade-proof claim is made.
- No secret-material-complete claim is made.
- No side-channel-free claim is made.
- No vulnerability-free claim is made.
- No bug-free claim is made.
- No perfect-crypto claim is made.
