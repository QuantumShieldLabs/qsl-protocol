# NA-0450 Closeout / Restore NA-0451 Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-09

## Objective

Verify NA-0450 closeout after PR #1169 merged, post-merge public-safety
completed success, and restoration of NA-0451 as the sole READY successor.

## Protected invariants

- NA-0450 is DONE only after PR #1169 is merged and post-merge public-safety
  is success.
- NA-0451 is the sole READY item.
- NA-0434 and NA-0429 remain BLOCKED.
- D-0888 exists once and D-0889 remains absent until future NA-0451 work.
- Duplicate decision count remains zero.
- No public claim expands.

## Allowed scope

- `NEXT_ACTIONS.md`.
- `DECISIONS.md`.
- `TRACEABILITY.md`.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.
- `tests/NA-0450_closeout_restore_na0451_testplan.md`.

## Forbidden scope

- No source, runtime, crypto, Cargo, lockfile, workflow, executable test,
  fuzz target, vector, formal model, qsl-server, qsl-attachments, qshield
  runtime, qshield-cli, website, public docs, README, START_HERE,
  qwork/qstart/qresume/qshell, qsl-backup, backup status file, backup plan
  file, rollback subtree, or backup tree mutation.
- No backup or restore execution.
- No public-readiness claim.
- No production-readiness claim.
- No public-internet-readiness claim.
- No external-review-complete claim.
- No crypto-complete claim.
- No RNG-failure-complete claim.
- No secret-material-complete claim.
- No side-channel-free claim.
- No vulnerability-free claim.
- No bug-free claim.
- No perfect-crypto claim.
- No metadata-free claim.
- No anonymity claim.
- No untraceable claim.
- No off-host-backup claim.
- No disaster-recovery claim.
- No restore-proof claim.
- No backup-complete claim.

## PR #1169 merge/public-safety checks

- Verify `gh pr view 1169 --repo QuantumShieldLabs/qsl-protocol --json number,state,mergedAt,mergeCommit,title,url,statusCheckRollup`.
- Required result: PR #1169 is MERGED with merge commit beginning
  `4ef871952a28`.
- Verify merge-commit check-runs through REST polling only.
- Required result: public-safety is completed success and qsc adversarial smoke
  is success or accepted by repo policy.

## NA-0450 triage consumption check

- Verify D-0887 exists once.
- Verify the NA-0450 evidence doc selected
  `RNG_RESIDUAL_TRIAGE_ROUTE_CONTACT_ATTACHMENT_NEXT`.
- Verify the selected successor is
  `NA-0451 -- QSL qsc Route / Contact / Attachment RNG Failure Scope
  Authorization Plan`.
- Required result: NA-0450 is represented as governance triage, not
  implementation proof.

## NA-0451 restoration check

- Run `python3 scripts/ci/qsl_evidence_helper.py queue`.
- Required result: `READY_COUNT 1`.
- Required result: READY item is NA-0451.
- Required result: NA-0450 is DONE, NA-0449 through NA-0435 are DONE,
  NA-0434 is BLOCKED, and NA-0429 is BLOCKED.

## Decision proof

- Run `python3 scripts/ci/qsl_evidence_helper.py decisions`.
- Required result: latest decision is D-0888.
- Required result: D-0887 exists once.
- Required result: D-0888 exists once.
- Required result: D-0889 is absent before future NA-0451 work.
- Required result: duplicate decision count is zero.

## qsl-protocol closeout scope guard

- Run `git diff --name-only` before commit and compare against the allowed
  closeout path list.
- Required result: only the five allowed closeout paths are changed.

## No source/workflow/test/lockfile/vector/formal mutation

- Run a forbidden path guard over `git diff --name-only`.
- Required result: no source, workflow, executable test outside this
  governance testplan, lockfile, vector, fuzz target, or formal model path is
  changed by closeout.

## No public overclaim

- Run added-line overclaim scan.
- Required result: no affirmative public-readiness claim.
- Required result: no affirmative production-readiness claim.
- Required result: no affirmative public-internet-readiness claim.
- Required result: no affirmative external-review-complete claim.
- Required result: no affirmative crypto-complete claim.
- Required result: no affirmative RNG-failure-complete claim.
- Required result: no affirmative secret-material-complete claim.
- Required result: no affirmative side-channel-free claim.
- Required result: no affirmative vulnerability-free claim.
- Required result: no affirmative bug-free claim.
- Required result: no affirmative perfect-crypto claim.
- Required result: no affirmative metadata-free claim.
- Required result: no affirmative anonymity claim.
- Required result: no affirmative untraceable claim.
- Required result: no affirmative off-host-backup claim.
- Required result: no affirmative disaster-recovery claim.
- Required result: no affirmative restore-proof claim.
- Required result: no affirmative backup-complete claim.

## Dependency-health checks

- Run `cargo audit --deny warnings`.
- Run `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`.
- Required result: both audits pass.
- Caveat: audit green is dependency-health evidence only, not public readiness.
- Caveat: audit green is not vulnerability-free proof.

## Public-safety before merge

- Verify the closeout PR public-safety check completes success before merge.
- Required result: no failing required checks.

## Public-safety after merge

- Verify the closeout merge commit public-safety check completes success.
- Required result: public-safety is success and no required check is failing.

## Backup / restore boundary

- Verify Codex did not run backup or restore.
- Verify no qsl-backup, backup status file, backup plan file, rollback subtree,
  or backup tree path changed.
- Required result: backup impact is none.
