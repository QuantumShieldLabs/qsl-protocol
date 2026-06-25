Status: Supporting
Owner: Verification
Last-Updated: 2026-06-25

# NA-0538 Closeout / NA-0539 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Validate that NA-0538 is closed only after the D-1066 authorization PR #1349 merged and post-authorization public-safety/advisories completed success, and that exactly one successor, NA-0539, is restored as READY without implementing NA-0539.

## Required startup gates

- qwork proof files exist at `/srv/qbuild/work/NA-0538/.qwork/startup.qsl-protocol.kv` and `/srv/qbuild/work/NA-0538/.qwork/startup.qsl-protocol.json`.
- qwork proof records `startup_result=OK`, lane `NA-0538`, repo `qsl-protocol`, path `/srv/qbuild/work/NA-0538/qsl-protocol`, clean worktree/index/untracked state, HEAD equals origin/main, READY_COUNT 1, queue top READY `NA-0538`, and requested lane status READY.
- qwork proof written time is after the D449 response timestamp.
- Live pre-fetch HEAD and origin/main match qwork proof.
- `/` disk usage is below the 95% stop threshold before fetch.
- origin/main equals or descends from `176ae640b333`.

## Required inheritance gates

- D449 authorization PR #1349 merged at `176ae640b333`.
- D-1066 exists once.
- D449 classification is `PUBLIC_EVIDENCE_SYNC_IMPLEMENTATION_READY`.
- D449 selected successor is `NA-0539 -- QSL Website / Repository Public Evidence Sync Implementation Harness`.
- D449 selected the future path bundle and claim policy.
- D449 did not implement website, README, or public-doc changes.
- D448 restored NA-0538 READY after NA-0537 closeout.
- D446 repeated-run cleanup/freshness success is consumed.
- No qsl-server, qsl-attachments, qsl-backup, qwork/qstart/qresume by Codex, or public/production/security-completion claim is inherited or introduced.

## Required green gate

- PR #1349 state is merged.
- PR #1349 merge commit is `176ae640b333`; full-SHA API proof is stored under the directive proof root.
- Check-runs for PR #1349 merge commit `176ae640b333` show public-safety completed success.
- Check-runs for PR #1349 merge commit `176ae640b333` show advisories completed success.
- No failed required checks or completed red check-runs relevant to closeout acceptance are present.

## Required closeout changes

- `NEXT_ACTIONS.md` marks NA-0538 DONE.
- `NEXT_ACTIONS.md` restores exactly one READY item: `NA-0539 -- QSL Website / Repository Public Evidence Sync Implementation Harness`.
- `DECISIONS.md` adds D-1067 for NA-0538 closeout and NA-0539 restoration.
- `TRACEABILITY.md` maps D-1066, PR #1349, D449 response, `PUBLIC_EVIDENCE_SYNC_IMPLEMENTATION_READY`, green-gate proof, D-1067, and NA-0539 restoration.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` records qwork proof, inheritance, green-gate, validation, closeout, and boundary evidence.
- This testplan records the closeout validation scope.

## Required markers

- `NA0538_CLOSEOUT_QWORK_PROOF_FRESH_OK`
- `NA0538_CLOSEOUT_D449_INHERITANCE_CONSUMED_OK`
- `NA0538_CLOSEOUT_D448_INHERITANCE_CONSUMED_OK`
- `NA0538_CLOSEOUT_D446_INHERITANCE_CONSUMED_OK`
- `NA0538_CLOSEOUT_PR1349_MERGED_OK`
- `NA0538_CLOSEOUT_D1066_ACCEPTED_OK`
- `NA0538_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0538_CLOSEOUT_ADVISORIES_GREEN_OK`
- `NA0538_CLOSEOUT_D1067_ADDED_OK`
- `NA0538_CLOSEOUT_NA0538_DONE_OK`
- `NA0538_CLOSEOUT_NA0539_READY_OK`
- `NA0538_CLOSEOUT_NO_NA0539_IMPLEMENTATION_OK`
- `NA0538_CLOSEOUT_NO_PUBLIC_DOC_IMPLEMENTATION_OK`
- `NA0538_CLOSEOUT_NO_REMOTE_ACTION_OK`
- `NA0538_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK`
- `NA0538_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0538_CLOSEOUT_NO_DEPENDENCY_LOCKFILE_MUTATION_OK`
- `NA0538_CLOSEOUT_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0538_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Post-patch validation

- `git diff --check`
- exact five-path scope guard including untracked files
- queue/decision proof:
  - READY_COUNT 1
  - READY NA-0539
  - NA-0538 DONE
  - D-1066 once
  - D-1067 once
  - D-1068 absent
  - duplicate decision count zero
- local markdown link check
- added-line/new-file private-material scan
- added-line/new-file overclaim scan with negation and forbidden-scope handling
- docs-only classifier
- PR body preflight
- goal-lint
- `cargo audit --deny warnings`
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

## Boundary assertions

- No NA-0539 implementation occurs.
- No website, README, or public-doc implementation mutation occurs outside the queue successor block.
- No remote-host action, SSH execution, qsc send/receive, remote E2EE, qsc protocol command, qsl-server, qsl-attachments, qwork/qstart/qresume, qsl-backup, dependency/lockfile mutation, qsc source/test/fuzz/Cargo mutation, workflow/script/helper mutation, corpus/vector/input mutation, or formal/refimpl/service/public/backup mutation occurs.
- Missing `public/` and `website/` paths remain forbidden unless a later directive explicitly authorizes that path creation.
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
