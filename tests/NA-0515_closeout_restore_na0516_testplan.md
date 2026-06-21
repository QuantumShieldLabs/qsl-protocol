Status: Supporting
Owner: QSL Governance / QA
Last-Updated: 2026-06-21

Goals: G1, G2, G3, G4, G5

# NA-0515 closeout and NA-0516 restoration testplan

## Objective

Validate that NA-0515 is closed only after the authorization PR merged and post-merge public-safety completed success, and that NA-0516 is restored as the sole READY successor without implementing NA-0516.

## Scope checks

Changed paths must be exactly:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0515_closeout_restore_na0516_testplan.md`

No qsc source/test/fuzz/Cargo, workflow/script/helper, dependency/lockfile, corpus/vector/input, formal/refimpl/service/public/backup, qsl-server, qsl-attachments, qshield, qshield-cli, website, README, START_HERE, backup, or remote-host path may change.

## Required proof

- PR #1302 merged.
- Authorization head `36aadd2fd0f2` recorded.
- Merge commit `e0ded610fce8` recorded.
- Post-merge public-safety completed success on `e0ded610fce8`.
- D-1019 exists once.
- D-1020 exists once after patch.
- Duplicate decision count is zero.
- NA-0515 is DONE.
- NA-0516 is READY.
- READY_COUNT is 1.

## No implementation checks

Closeout must not:

- implement NA-0516.
- run remote commands.
- execute SSH.
- run scp, sftp, or rsync.
- run qsc send/receive.
- execute remote E2EE.
- transfer a binary.
- install packages.
- run remote source checkout/build.
- generate or install SSH keys.
- mutate SSH config or known_hosts.
- mutate remote host state.
- run qwork, qstart, qresume, qsl-backup, backup, or restore.

## Static validation

Required checks:

- `git diff --check`.
- exact five-path closeout scope guard.
- queue/decision invariant proof.
- deterministic markdown link-check.
- leak-scan against added closeout lines and testplan.
- added-line overclaim scan.
- docs/governance classifier.
- PR body preflight.
- goal-lint preflight.
- marker proof for closeout markers.

Required markers:

- `NA0515_CLOSEOUT_PR1302_MERGED_OK`
- `NA0515_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0515_CLOSEOUT_D1019_ACCEPTED_OK`
- `NA0515_CLOSEOUT_D1020_RESTORED_NA0516_OK`
- `NA0515_CLOSEOUT_NO_NA0516_IMPLEMENTATION_OK`
- `NA0515_CLOSEOUT_NO_REMOTE_ACTION_OK`
- `NA0515_CLOSEOUT_NO_SSH_EXECUTION_OK`
- `NA0515_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK`
- `NA0515_CLOSEOUT_NO_REMOTE_E2EE_OK`
- `NA0515_CLOSEOUT_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0515_CLOSEOUT_NO_WORKFLOW_DEPENDENCY_MUTATION_OK`
- `NA0515_CLOSEOUT_NO_CORPUS_VECTOR_INPUT_MUTATION_OK`
- `NA0515_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0515_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0515_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Claim boundary

- no public-readiness claim is made.
- no production-readiness claim is made.
- no public-internet-readiness claim is made.
- no external-review-complete claim is made.
- no crypto-complete claim is made.
- no replay-proof claim is made.
- no downgrade-proof claim is made.
- no secret-material-complete claim is made.
- no side-channel-free claim is made.
- no vulnerability-free claim is made.
- no bug-free claim is made.
- no perfect-crypto claim is made.

## Post-fix hardening review

- Correctness under stress: NA-0516 starts from a single READY queue state and still must perform fresh qwork, retained-binary, remote-boundary, local-qsc, redaction, synthetic-data, and cleanup/retention checks before implementation.
- Minimality: closeout is limited to queue, decision, traceability, journal, and closeout testplan.
- Maintainability: NA-0516 block carries objective, allowed scope, forbidden scope, deliverables, and acceptance criteria in one queue entry.
- Coverage quality: static proof targets queue state, decision uniqueness, scope, link integrity, leak safety, overclaim boundaries, PR body, goal-lint, and markers.
- Cross-lane stability: no runtime, qsc, workflow, dependency, formal, service, public, backup, or remote-host path changes are introduced.
