Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-21

# NA-0518 closeout and NA-0519 restoration testplan

## Purpose

Validate the closeout-only governance lane after NA-0518 authorization evidence merged. This testplan verifies PR #1308 merge proof, post-merge public-safety proof, D-1025 acceptance, D-1026 closeout decision, NA-0518 DONE state, NA-0519 READY restoration, exact closeout scope, and no NA-0519 implementation.

## Scope guard

Allowed changed paths for closeout:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0518_closeout_restore_na0519_testplan.md`

No NA-0519 evidence implementation, qsc source/test/fuzz/Cargo, workflow/script/helper, dependency/lockfile, corpus/vector/input, formal/refimpl/service/public/backup, qsl-server, qsl-attachments, qshield, qshield-cli, public docs, website, README, START_HERE, backup, rollback, archive, move, or deletion path may change.

## Required closeout proof

- PR #1308 merged with a merge commit.
- Evidence PR head was recorded.
- Merge commit was recorded.
- Post-merge `public-safety` completed success inside the short attach/early-failure window.
- D-1025 exists once.
- D-1026 exists once after patch.
- D-1027 absent.
- No duplicate decision IDs exist.
- READY_COUNT is 1 after patch.
- NA-0518 is DONE.
- NA-0519 is READY.

## Required NA-0519 successor proof

The restored NA-0519 block must be proof-review only and include:

- read-only review of operator-provided redacted proof.
- no SSH by Codex.
- no authorized_keys edit by Codex.
- no key generation or installation by Codex.
- no private key reading.
- no local SSH config mutation.
- no known_hosts mutation.
- no remote host mutation.
- no qsc send/receive.
- no remote E2EE.
- no qsl-server/qsl-attachments.
- no sudo/admin.
- no qwork/qstart/qresume.
- no qsl-backup.
- no public-readiness claim and no production-readiness claim.

## Static validation

Run and record:

- `git diff --check`
- exact five-path scope guard.
- link-check.
- leak-scan.
- overclaim scan.
- classifier.
- PR body preflight.
- goal-lint.
- queue and decision proof.

## Required markers

- `NA0518_CLOSEOUT_PR1308_MERGED_OK`
- `NA0518_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0518_CLOSEOUT_D1025_ACCEPTED_OK`
- `NA0518_CLOSEOUT_D1026_RESTORED_NA0519_OK`
- `NA0518_CLOSEOUT_NO_NA0519_IMPLEMENTATION_OK`
- `NA0518_CLOSEOUT_NO_REMOTE_ACTION_OK`
- `NA0518_CLOSEOUT_NO_SSH_EXECUTION_OK`
- `NA0518_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK`
- `NA0518_CLOSEOUT_NO_REMOTE_E2EE_OK`
- `NA0518_CLOSEOUT_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0518_CLOSEOUT_NO_WORKFLOW_DEPENDENCY_MUTATION_OK`
- `NA0518_CLOSEOUT_NO_CORPUS_VECTOR_INPUT_MUTATION_OK`
- `NA0518_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0518_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0518_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Forbidden commands

Closeout validation must not run SSH, scp, sftp, rsync to remote, qsc send/receive, remote E2EE, ssh-keygen, ssh-keyscan, sudo, qwork, qstart, qresume, qsl-backup, backup, or restore.

## Post-fix hardening review

1. Correctness under stress: closeout only advances queue state after evidence PR merge and post-merge public-safety success.
2. Minimality: closeout mutates only queue/governance/traceability/journal/testplan paths.
3. Maintainability: NA-0519 successor scope is explicit and proof-review only.
4. Coverage quality: static validation proves exact scope, markers, queue state, decision IDs, and claim boundaries.
5. Cross-lane stability: qsc source/tests/workflows/dependencies remain untouched, preserving macOS/Linux behavior for affected gates.
