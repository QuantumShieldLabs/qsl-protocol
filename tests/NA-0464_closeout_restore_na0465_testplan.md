Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-12

# NA-0464 Closeout / Restore NA-0465 Testplan

Goals: G1, G2, G3, G4, G5

## scope

This closeout testplan verifies that NA-0464 is closed after merged evidence and
that the selected NA-0465 lazy identity successor is restored as the sole READY
item.

Allowed closeout mutation paths:

- `NEXT_ACTIONS.md`.
- `DECISIONS.md`.
- `TRACEABILITY.md`.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.
- `tests/NA-0464_closeout_restore_na0465_testplan.md`.

Forbidden closeout mutation paths:

- No qsc source mutation.
- No runtime behavior mutation.
- No crypto behavior mutation.
- No dependency mutation.
- No Cargo manifest mutation.
- No lockfile mutation.
- No workflow mutation.
- No executable test mutation.
- No fuzz target mutation.
- No vector mutation.
- No formal model mutation.
- No refimpl mutation.
- No qsl-server mutation.
- No qsl-attachments mutation.
- No qshield runtime mutation.
- No qshield-cli mutation.
- No website mutation.
- No public docs mutation.
- No README mutation.
- No START_HERE mutation.
- No qwork/qstart/qresume/qshell mutation.
- No qsl-backup mutation.
- No backup status mutation.
- No backup plan mutation.
- No rollback subtree mutation.
- No backup tree mutation.

## queue proof

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
```

Required:

- `READY_COUNT 1`.
- `READY NA-0465`.
- `NA-0464 DONE`.

## decision proof

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Required:

- latest decision is D-0916.
- D-0915 exists once.
- D-0916 exists once.
- D-0917 is absent.
- duplicate decision count is zero.

## scope guard

Run a closeout path guard against the exact allowed closeout paths.

Required:

- changed path count is five.
- forbidden path count is zero.

## link check

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
```

Required:

- `TOTAL_MISSING 0`.

## leak scan

Run full-path leak scan over the closeout mutation paths.

Required:

- `SECRET_FINDING_COUNT 0`.

## overclaim scan

Run an added-line overclaim scan.

Required:

- zero affirmative overclaims.
- No added line may claim public readiness.
- No added line may claim production readiness.
- No added line may claim public-internet readiness.
- No added line may claim external-review completion.
- No added line may claim crypto-complete status.
- No added line may claim KEM-complete status.
- No added line may claim signature-complete status.
- No added line may claim identity-complete status.
- No added line may claim RNG-failure-complete status.
- No added line may claim provider-RNG-complete status.
- No added line may claim side-channel-free status.
- No added line may claim vulnerability-free status.
- No added line may claim bug-free status.
- No added line may claim perfect-crypto status.

## dependency health

Run:

```bash
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
```

Required:

- root cargo audit passes.
- nested qsc fuzz lock cargo audit passes.
- Cargo audit output is dependency-health evidence only.
- Cargo audit output must not be used as public-readiness proof.
- Cargo audit output must not be used as production-readiness proof.
- Cargo audit output must not be used as public-internet-readiness proof.
- Cargo audit output must not be used as external-review-complete proof.
- Cargo audit output must not be used as crypto-complete proof.
- Cargo audit output must not be used as signature-complete proof.
- Cargo audit output must not be used as identity-complete proof.
- Cargo audit output must not be used as RNG-failure-complete proof.
- Cargo audit output must not be used as provider-RNG-complete proof.
- Cargo audit output must not be used as vulnerability-free proof.
- Cargo audit output must not be used as bug-free proof.
- Cargo audit output must not be used as perfect-crypto proof.
- Cargo audit output must not be used as side-channel-free proof.

## public-safety

Before merge:

- PR required checks must complete without failure.
- public-safety must be completed success or accepted by repository policy.

After merge:

- public-safety must complete success on the closeout merge commit before
  declaring closeout complete.

## acceptance

This closeout passes only if:

- NA-0464 is DONE.
- NA-0465 is the sole READY item.
- D-0916 records closeout and restoration.
- NA-0465 exact allowed scope includes only the lazy identity source/test and
  governance paths selected by NA-0464.
- No NA-0465 implementation is performed.
- No backup or restore is run.
- No public claim expansion is introduced.
