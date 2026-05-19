Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-19

# NA-0320 Closeout and NA-0321 Restoration Testplan

## Objective

Close NA-0320 after the bounded qshield embedded relay sanitized-error and
retention/purge executable harness merged, then restore exactly one READY
successor: NA-0321 -- Metadata Runtime Timing and Traffic-Shape Threat Model /
Executable Evidence Plan.

## Protected Invariants

- NA-0320 is DONE only after the implementation PR merged and post-merge
  `public-safety` was green.
- Exactly one READY item exists after closeout: NA-0321.
- D-0619 exists once, D-0620 exists once, and D-0621 is absent.
- This closeout does not implement NA-0321.
- The qshield embedded relay proof remains distinct from qsl-server and
  qsl-attachments production semantics.
- No production, public-internet, external-review, anonymity, metadata-free, or
  untraceable claim is introduced.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0320_closeout_restore_na0321_testplan.md`

## Forbidden Scope

- qshield, qsl-server, qsl-attachments, qsc, qsp, protocol, crypto, key
  schedule, service, website, external website, qsc-desktop, `.github`, Cargo,
  dependency, README, START_HERE, branch-protection, and public-safety
  implementation/configuration paths.

## Required Validation

1. Confirm queue state: READY_COUNT is `1`, READY is NA-0321, and NA-0320 is
   DONE.
2. Confirm decisions: D-0619 exists once, D-0620 exists once, D-0621 is absent,
   and duplicate decision count is zero.
3. Run scope guard against `origin/main` with the allowed closeout paths.
4. Run link-check and added leak-scan.
5. Run cargo audit, qsc `send_commit`, and formal model checks required by the
   directive.
6. Run goal-lint or PR body preflight with a standalone
   `Goals: G1, G2, G3, G4, G5` line.
7. Confirm required checks and post-merge public-safety are green after the
   closeout PR merges.

## CI Expectations

The closeout is governance/testplan-only. Public-safety must remain required and
green. Cost-control may skip full runtime suites only if the existing CI
classifier truthfully treats the closeout as docs/governance-only; that skip is
evidence of existing cost-control behavior, not a relaxation of the gate.

## Successor Handoff

NA-0321 must begin from a timing and traffic-shape threat model / executable
evidence plan posture. It must not claim metadata-free behavior, anonymity,
untraceability, production readiness, public-internet readiness, or external
review completion.
