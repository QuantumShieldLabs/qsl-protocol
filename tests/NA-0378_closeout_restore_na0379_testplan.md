Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-28

# NA-0378 Closeout and NA-0379 Restoration Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0378 closes after the qstart/qresume guard evidence merge and
restores the exact NA-0379 successor without implementing NA-0379 or changing
runtime, workflow, dependency, backup-configuration, qshell, service, secret,
target, or public-claim surfaces.

## Protected Invariants

- READY_COUNT is 1.
- READY is NA-0379 after closeout.
- NA-0378 is DONE.
- D-0738 exists once.
- D-0739 exists once.
- D-0740 is absent.
- NA-0379 is not implemented by closeout.
- qshell is not modified by closeout.
- public-safety remains required and green.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0378_closeout_restore_na0379_testplan.md`

## Forbidden Scope

No changes are allowed in:

- `/srv/qbuild/tools/**`
- qsl-server
- qsl-attachments
- qshield runtime paths
- qsc/qsp/qsl runtime paths
- `.github/**`
- `Cargo.toml`
- `Cargo.lock`
- dependency files
- scripts/runtime helpers
- website or external website repositories
- `README.md`
- `START_HERE.md`
- `docs/public/**`
- `/usr/local/sbin/qsl-backup`
- backup scripts, timers, fstab, services, source lists, keys, credentials, or
  secret material
- `/home/victor/work/qsl/codex/**` except the required final D197 response file

## Required Proof

The closeout evidence must record:

- qsl-protocol PR #1019 head and merge SHAs;
- qshell original/new checksums, rollback, patch, and harness log by reference;
- post-merge `public-safety` success on the Packet I merge commit;
- D-0739 closeout decision;
- NA-0379 selected successor;
- no NA-0379 implementation;
- no qshell mutation in closeout.

## Required Local Checks

Run and record:

- queue helper showing READY NA-0379 and NA-0378 DONE;
- decision helper showing D-0739 once and D-0740 absent;
- scope guard for the exact closeout path set;
- link-check;
- leak-scan;
- changed-line overclaim scan;
- classifier proof;
- cargo audit;
- rustls-webpki dependency proof;
- qsc send_commit test;
- formal model checks;
- PR body preflight and goal-lint.

## CI Expectations

The closeout PR must include Goals, Impact, No-regression, and Tests/Vectors
metadata. Required checks, including `public-safety`, must pass before merge.
After merge, post-merge `public-safety` must complete successfully.

## Successor Handoff

After merge, `NA-0379 -- QSL Local Ops Bounded CI Polling Helper Implementation
Authorization Plan` is the sole READY item. NA-0379 implementation remains for
a future exact directive.
