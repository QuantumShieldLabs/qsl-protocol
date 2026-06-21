Status: Supporting
Owner: QSL Governance / Core Assurance
Last-Updated: 2026-06-21

# NA-0512 Closeout and NA-0513 Restoration Testplan

## Objective

Verify that NA-0512 is closed only after the implementation PR merged and
post-merge public-safety completed success, and that the selected NA-0513 remote
qsc staging strategy authorization lane is restored as the sole READY successor
without implementing NA-0513.

## Protected invariants

- Exactly one READY item remains mandatory.
- NA-0512 advances G4 without regressing G1, G2, G3, or G5.
- Closeout performs no NA-0513 implementation.
- Closeout runs no SSH.
- Closeout runs no remote command.
- Closeout performs no remote E2E.
- Closeout performs no package installation.
- Closeout performs no sudo/admin action.
- Closeout performs no key generation or installation.
- Closeout performs no SSH config mutation.
- Closeout performs no known_hosts mutation.
- Closeout performs no remote host mutation.
- Closeout performs no qwork/qstart/qresume mutation.
- Closeout performs no qsl-backup execution.
- Closeout performs no qsc source/test/fuzz/Cargo mutation.
- Closeout performs no workflow/dependency mutation.
- Closeout performs no corpus/vector/input mutation.
- Closeout performs no formal/refimpl/service/public/backup mutation.
- Closeout makes no public-readiness claim.
- Closeout makes no production-readiness claim.
- Closeout makes no public-internet-readiness claim.
- Closeout makes no crypto-complete claim.
- Closeout makes no replay-proof claim.
- Closeout makes no downgrade-proof claim.
- Closeout makes no secret-material-complete claim.
- Closeout makes no side-channel-free claim.
- Closeout makes no vulnerability-free claim.
- Closeout makes no bug-free claim.
- Closeout makes no perfect-crypto claim.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0512_closeout_restore_na0513_testplan.md`

## Required proof checks

- PR #1296 merged at `d20e700fcb2c`.
- D-1013 exists once.
- NA-0512 evidence and testplan are in-tree on main.
- post-merge public-safety on `d20e700fcb2c` completed success inside the short
  attach/early-failure window.
- NA-0512 is marked DONE.
- selected NA-0513 block is restored READY.
- D-1014 exists once after patch.
- duplicate decision count is 0.
- exact five-path closeout scope guard passes.
- link-check passes.
- leak-scan passes.
- overclaim scan passes.
- PR body preflight passes.
- goal-lint passes.

## Required markers

- `NA0512_CLOSEOUT_PR1296_MERGED_OK`
- `NA0512_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0512_CLOSEOUT_D1013_ACCEPTED_OK`
- `NA0512_CLOSEOUT_D1014_RESTORED_NA0513_OK`
- `NA0512_CLOSEOUT_NO_NA0513_IMPLEMENTATION_OK`
- `NA0512_CLOSEOUT_NO_REMOTE_ACTION_OK`
- `NA0512_CLOSEOUT_NO_SSH_EXECUTION_OK`
- `NA0512_CLOSEOUT_NO_REMOTE_E2E_OK`
- `NA0512_CLOSEOUT_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0512_CLOSEOUT_NO_WORKFLOW_DEPENDENCY_MUTATION_OK`
- `NA0512_CLOSEOUT_NO_CORPUS_VECTOR_INPUT_MUTATION_OK`
- `NA0512_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0512_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0512_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Post-fix hardening review

1. Correctness under stress: closeout consumes only merged implementation
   evidence and post-merge public-safety before changing queue state.
2. Minimality: closeout changes only queue, decision, traceability, journal, and
   closeout testplan paths.
3. Maintainability: NA-0513 remains authorization-only with exact future scope
   and forbidden boundaries recorded in NEXT_ACTIONS.
4. Coverage quality: static guards prove scope, links, leaks, overclaim wording,
   goal metadata, decision uniqueness, and one-READY queue state.
5. Cross-lane stability: closeout performs no remote command and no qsc,
   workflow, dependency, corpus, formal, service, public, or backup mutation.
