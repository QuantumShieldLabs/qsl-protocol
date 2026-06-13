Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-13

# NA-0469 Closeout Restore NA-0470 Testplan

## Objective

Close out NA-0469 after implementation PR #1207 merged and post-merge
public-safety completed success, then restore NA-0470 as the sole READY
successor without implementing NA-0470.

## Protected Invariants

- NA-0469 is DONE only after PR #1207 merged at `94c0695a194a` and
  post-merge public-safety completed success.
- NA-0470 is READY and authorization-only.
- Exactly one READY item remains.
- Closeout changes only `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`,
  `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`, and this testplan.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, qsl-server,
  qsl-attachments, qshield runtime, qshield-cli runtime, website, public docs,
  README, START_HERE, qsc source, executable tests, fuzz targets, vectors,
  formal models, or refimpl mutation occurs.
- No backup is run. No restore is run. No qsl-backup, backup status, backup
  plan, rollback subtree, or backup tree path is mutated.
- No public-readiness claim is introduced.
- No production-readiness claim is introduced.
- No external-review-complete claim is introduced.
- No crypto-complete claim is introduced.
- No signature-complete claim is introduced.
- No identity-complete claim is introduced.
- No RNG-failure-complete claim is introduced.
- No provider-RNG-complete claim is introduced.
- No side-channel-free claim is introduced.
- No vulnerability-free claim is introduced.
- No bug-free claim is introduced.
- No perfect-crypto claim is introduced.

## Validation

- Queue helper proves READY_COUNT 1 and READY NA-0470.
- Decision helper proves latest D-0926, D-0925 once, D-0926 once, D-0927 absent,
  and duplicate decision count zero.
- Scope guard proves only the five allowed closeout paths changed.
- Link check reports `TOTAL_MISSING 0`.
- Leak scan reports `SECRET_FINDING_COUNT 0`.
- Added-line overclaim scan reports zero affirmative findings.
- PR body preflight passes with Goals, Impact, No-regression, and Tests/Vectors.
- Root cargo audit remains green.
- Nested qsc fuzz lock cargo audit remains green.
- Public-safety is green before merge and after merge.

## Public Claim Boundary

This closeout is governance-only. It does not implement NA-0470. It does not
expand NA-0469 beyond CLI rotation provider RNG failure evidence. Cargo audit
green is dependency-health evidence only.
