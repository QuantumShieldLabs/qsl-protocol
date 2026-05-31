Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-31

# NA-0393 Closeout and NA-0394 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0393 closeout marks NA-0393 DONE, records D-0769, restores
the exact selected NA-0394 successor as the sole READY item, and does not
implement NA-0394.

## Preconditions

- Packet R PR #1049 is merged.
- Packet R post-merge public-safety is green.
- READY_COUNT is 1 and READY is NA-0393 before closeout.
- D-0768 exists once.
- D-0769 is absent before closeout.
- No durable external-watch report exists outside authorized governance evidence.
- Selected successor is exact:
  `NA-0394 -- QSL PQC Standards Alignment / Migration Evidence Mapping Plan`.

## Allowed Scope

Allowed closeout paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0393_closeout_restore_na0394_testplan.md`

## Forbidden Scope

Closeout must not mutate runtime, service, protocol, crypto, dependencies,
Cargo files, workflows, public docs, website, README, START_HERE, qsl-server,
qsl-attachments, qshield runtime, qsc-desktop, qstart/qresume tools, backup
scripts/timers/fstab/services, local history roots, response archives, secrets,
keys, remote/off-host targets, branch protection, or public-safety
configuration.

## Required Queue State

After closeout patch:

- READY_COUNT 1.
- READY NA-0394.
- NA-0393 DONE.
- D-0768 exists once.
- D-0769 exists once.
- D-0770 is absent.

## Successor Requirements

NA-0394 must be restored with:

- Status: READY.
- Goals: G1, G2, G3, G4, G5.
- Objective: create a qsl-protocol governance evidence map that compares QSL's
  current protocol/evidence posture with NIST PQC standards and migration
  guidance, while making no standards conformance, certification,
  production-readiness, or crypto-implementation-change claim.

NA-0394 implementation is not authorized by this closeout.

## Evidence Requirements

Closeout evidence must record:

- PR #1049 head `b72115428cba`.
- PR #1049 merge `eb628571eb81`.
- Packet R post-merge public-safety success.
- D-0768 and D-0769.
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
- queue and decisions proof
- exact scope guard
- link-check
- leak-scan
- overclaim scan
- cargo audit
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- qsc send_commit
- formal model checks
- qshield-cli build/test if feasible
- PR-body goal proof

## CI Expectations

Required qsl-protocol CI must pass normally before merge. `public-safety` must
remain required and green before merge and after merge.

No admin bypass, direct push, squash, rebase, force-push, amend-after-PR, branch
deletion command, or delete-branch flag is authorized.

## Handoff

After merge, NA-0394 is the only READY successor. NA-0394 standards alignment
work remains future-scoped and unimplemented by NA-0393 closeout.
