Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-14
Replaces:
Superseded-By:

# NA-0290A Closeout Restore NA-0290 Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0290A is closed after the public attention and visibility
strategy merged, and that NA-0290 is restored as the sole READY item for
metadata phase-2 identifier rotation and padding defaults design.

## Protected invariants

- Exactly one READY item exists after closeout.
- NA-0290A is DONE.
- NA-0290 is READY.
- D-0552 exists once.
- D-0553 is absent.
- D-0550 and D-0551 remain present once.
- Public-safety remains required and green.
- No implementation or public-copy change is authorized by closeout.

## Allowed scope

Allowed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0290A_closeout_restore_na0290_testplan.md`

## Forbidden scope

Forbidden changes:

- website or external website files
- `README.md`
- `START_HERE.md`
- `.github/**`
- `scripts/**`
- `Cargo.toml`
- `Cargo.lock`
- qsl-protocol runtime, protocol, crypto, demo, or service code
- qsl-server implementation files
- qsl-attachments implementation files
- qsc-desktop files
- `tools/**`
- `inputs/**`
- `formal/**`
- branch-protection settings
- public-safety configuration
- dependency changes
- branch deletion

## Queue requirements

Required parser result:

- `READY_COUNT 1`
- READY item is NA-0290
- NA-0290A is DONE

Manual queue check:

- NA-0290A closeout evidence records PR #833, PR #834, head and merge SHAs,
  strategy deliverables, and post-strategy public-safety success.
- NA-0290 objective and scope remain metadata phase-2 identifier/padding
  design.

## Decision requirements

Required parser result:

- D-0550 exists once.
- D-0551 exists once.
- D-0552 exists once.
- D-0553 is absent.
- Duplicate decision count is zero.

## Scope and claim-boundary requirements

The closeout may state prohibited claims only as negated/protected claims. It
must not introduce affirmative claims for production readiness, public internet
readiness, completed external review, anonymity, metadata-free messaging, or
untraceability.

Required checks:

- `git diff --check`
- exact allowed-path scope guard
- direct overclaim scan over added lines
- link-check
- added-content leak-scan
- docs-only classifier
- goal-lint

## CI expectations

Required validation:

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/run_model_checks.py` when present
- PR required checks complete successfully, with existing CodeQL neutral policy
  accepted only through the helper if applicable
- post-merge main public-safety completes successfully

## Future implementation gate

NA-0290A closeout restores NA-0290 only. It does not authorize public
visibility implementation. A later public-copy lane must be separately
approved and must re-run source, claim, link, leak, scope, and public-safety
checks for its exact target surface.
