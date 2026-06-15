# NA-0484 Closeout Restore NA-0485 Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Verify NA-0484 can be closed after the evidence PR merged and post-merge
public-safety completed success, then restore exactly one READY successor:
`NA-0485 -- QSL Fuzz Binding Coverage Split-Scope Authorization Plan`.

## Protected invariants

- Exactly one READY item remains.
- NA-0484 is marked DONE.
- NA-0485 is READY and authorization scoped only.
- No NA-0485 implementation is performed.
- No public claim boundary is expanded.
- No fuzz target, corpus, vector/input, runtime, crypto, dependency, Cargo,
  lockfile, workflow, script, executable test, formal model, qsc source/test,
  refimpl source/test, service, public, website, backup, qsl-backup, status,
  plan, rollback, or qwork/qstart/qresume/qshell mutation occurs outside the
  allowed closeout governance paths.
- no public-readiness claim is introduced.
- no production-readiness claim is introduced.
- no public-internet-readiness claim is introduced.
- no external-review-complete claim is introduced.
- no crypto-complete claim is introduced.
- no fuzz-complete claim is introduced.
- no vector-complete claim is introduced.
- no KEM-complete claim is introduced.
- no signature-complete claim is introduced.
- no replay-proof claim is introduced.
- no downgrade-proof claim is introduced.
- no qsc/refimpl-equivalence-complete claim is introduced.
- no formal-proof-complete claim is introduced.
- no side-channel-free claim is introduced.
- no vulnerability-free claim is introduced.
- no bug-free claim is introduced.
- no perfect-crypto claim is introduced.

## Allowed scope

- `NEXT_ACTIONS.md`.
- `DECISIONS.md`.
- `TRACEABILITY.md`.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.
- this testplan.

## Forbidden scope

- Implementation mutation.
- Fuzz target or corpus mutation.
- Vector/input mutation.
- Runtime, crypto, dependency, Cargo, lockfile, workflow, or script mutation.
- Executable test or formal model mutation.
- qsc source/test or refimpl source/test mutation.
- qsl-server, qsl-attachments, qshield runtime, qshield-cli, service, website,
  public-doc, README, START_HERE mutation.
- qwork, qstart, qresume, qshell mutation.
- backup, restore, qsl-backup, backup status, backup plan, rollback, systemd,
  timer, fstab, or backup tree mutation.

## Preconditions

- NA-0484 evidence PR #1238 is merged.
- Post-merge public-safety on the evidence merge commit is success.
- D-0956 exists once.
- D-0957 is absent before closeout.
- Queue has exactly one READY item: NA-0484.

## Queue validation

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
```

Required after closeout patch:

- `READY_COUNT 1`;
- READY NA-0485;
- NA-0484 DONE.

## Decision validation

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Required:

- D-0957 exists once;
- D-0958 absent;
- no duplicate decision IDs.

## Scope guard

Compare changed paths against the allowed closeout path set. Require zero
changes outside:

- `NEXT_ACTIONS.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`;
- `tests/NA-0484_closeout_restore_na0485_testplan.md`.

Require no mutation under qsc, fuzz targets, fuzz corpora, inputs/vectors,
refimpl, formal, Cargo/lockfile, workflow/script, service, public, website,
backup, qwork, README, or START_HERE paths.

## Claim-boundary validation

Run link-check, leak-scan, PR body preflight, and added-line overclaim scan.
Require no public readiness overclaim. Require no completion overclaim.
Require no proof overclaim. Require no side-channel-free overclaim.
Require no vulnerability-free overclaim. Require no bug-free overclaim.
Require no perfect-crypto overclaim.

## Dependency and inherited health

- Root cargo audit and nested qsc fuzz lock audit remain dependency-health
  evidence only.
- No dependency files are changed by closeout.
- Formal/qsc/refimpl implementation tests are inherited from the evidence PR
  and post-merge public-safety; this closeout does not introduce executable
  test changes.

## PR validation

- PR body must include Goals, Impact, No-regression, and Tests/Vectors.
- Goal-lint must pass.
- Required PR checks must pass before merge.
- Post-merge public-safety must complete success after closeout merge.

## Acceptance criteria

- NA-0484 DONE.
- NA-0485 READY.
- D-0957 accepted.
- Exactly one READY item remains.
- No implementation mutation.
- No public overclaim.
