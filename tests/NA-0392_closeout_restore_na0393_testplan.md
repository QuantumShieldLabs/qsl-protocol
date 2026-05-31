Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-30

# NA-0392 Closeout and NA-0393 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0392 closeout marks NA-0392 DONE, records D-0767, restores the exact selected NA-0393 successor as the sole READY item, and does not implement NA-0393.

## Preconditions

- Packet R PR #1047 is merged.
- Packet R post-merge public-safety is green.
- READY_COUNT is 1 and READY is NA-0392 before closeout.
- D-0766 exists once.
- D-0767 is absent before closeout.
- No durable external-watch report exists outside authorized governance evidence.
- Selected successor is exact:
  `NA-0393 -- QSL External Standards / Threat Watch Findings Triage and Queue Candidate Plan`.

## Allowed Scope

Allowed closeout paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0392_closeout_restore_na0393_testplan.md`

## Forbidden Scope

Closeout must not mutate runtime, service, protocol, crypto, dependencies, Cargo files, workflows, public docs, website, README, START_HERE, qsl-server, qsl-attachments, qshield runtime, qsc-desktop, qstart/qresume tools, backup scripts/timers/fstab/services, local history roots, response archives, secrets, keys, remote/off-host targets, branch protection, or public-safety configuration.

## Required Queue State

After closeout patch:

- READY_COUNT 1.
- READY NA-0393.
- NA-0392 DONE.
- D-0766 exists once.
- D-0767 exists once.
- D-0768 is absent.

## Successor Requirements

NA-0393 must be restored with:

- Status: READY.
- Goals: G1, G2, G3, G4, G5.
- Objective: triage NA-0392 source-cited external watch findings into explicit queue candidates, claim-boundary updates, evidence gaps, and future directive recommendations without code/runtime/workflow/dependency mutation and without automatic READY promotion.

NA-0393 implementation is not authorized by this closeout.

## Evidence Requirements

Closeout evidence must record:

- PR #1047 head `b3b736e5e054`.
- PR #1047 merge `f0594d4d93cb`.
- Packet R post-merge public-safety success.
- D-0766 and D-0767.
- Selected successor.
- No code/runtime/dependency/workflow mutation.
- No durable external-watch report outside governance evidence.
- No qsl-server/qsl-attachments mutation.
- No backup-plan update required for governance-only evidence.

## Validation

Run or record:

- `git status --porcelain=v1 --branch`
- `git diff --name-only origin/main...HEAD`
- `git diff --stat origin/main...HEAD`
- `git diff --check`
- queue and decisions proof;
- exact scope guard;
- link-check;
- leak-scan;
- overclaim scan;
- cargo audit;
- `cargo tree -i rustls-webpki --locked`;
- `cargo fmt --check`;
- qsc send_commit;
- formal model checks;
- qshield-cli build/test if feasible;
- PR-body goal proof.

## CI Expectations

Required qsl-protocol CI must pass normally before merge. `public-safety` must remain required and green before merge and after merge.

No admin bypass, direct push, squash, rebase, force-push, amend-after-PR, branch deletion command, or delete-branch flag is authorized.

## Handoff

After merge, NA-0393 is the only READY successor. NA-0393 triage work remains future-scoped and unimplemented by NA-0392 closeout.
