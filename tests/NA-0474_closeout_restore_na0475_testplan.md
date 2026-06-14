Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-13

# NA-0474 Closeout and NA-0475 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate closeout of NA-0474 after the NA-0474 evidence PR merged and
post-merge public-safety completed success. The closeout must mark NA-0474
DONE, restore exactly one selected NA-0475 successor as READY, add D-0937, and
preserve governance-only closeout scope without implementing NA-0475.

## Protected invariants

- NA-0474 is DONE after closeout.
- NA-0475 is the sole READY item after closeout.
- D-0936 remains accepted evidence for the NA-0474 audit.
- D-0937 records closeout and NA-0475 restoration.
- No NA-0475 implementation occurs.
- No runtime mutation occurs.
- No crypto mutation occurs.
- No dependency mutation occurs.
- No Cargo manifest mutation occurs.
- No lockfile mutation occurs.
- No workflow mutation occurs.
- No executable test mutation occurs.
- No fuzz target mutation occurs.
- No vector mutation occurs.
- No formal model mutation occurs.
- No refimpl mutation occurs.
- No qsl-server mutation occurs.
- No qsl-attachments mutation occurs.
- No qshield runtime mutation occurs.
- No qshield-cli mutation occurs.
- No website mutation occurs.
- No public docs mutation occurs.
- No README mutation occurs.
- No START_HERE mutation occurs.
- No qwork/qstart/qresume/qshell mutation occurs.
- No backup is run.
- No restore is run.
- No qsl-backup, backup status, backup plan, rollback subtree, or backup tree
  path is mutated.
- No public-readiness claim is introduced.
- No production-readiness claim is introduced.
- No public-internet-readiness claim is introduced.
- No external-review-complete claim is introduced.
- No crypto-complete claim is introduced.
- No KEM-complete claim is introduced.
- No signature-complete claim is introduced.
- No identity-complete claim is introduced.
- No transcript-complete claim is introduced.
- No downgrade-proof claim is introduced.
- No replay-proof claim is introduced.
- No RNG-failure-complete claim is introduced.
- No provider-RNG-complete claim is introduced.
- No secret-material-complete claim is introduced.
- No side-channel-free claim is introduced.
- No vulnerability-free claim is introduced.
- No bug-free claim is introduced.
- No perfect-crypto claim is introduced.
- Cargo audit green remains dependency-health evidence only.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- this testplan

## Forbidden scope

- NA-0475 implementation.
- Runtime source mutation.
- Crypto source mutation.
- Dependency, Cargo manifest, or lockfile mutation.
- Workflow mutation.
- Executable test mutation.
- Fuzz target mutation.
- Vector mutation.
- Formal model mutation.
- refimpl mutation.
- qsl-server mutation.
- qsl-attachments mutation.
- qshield runtime mutation.
- qshield-cli mutation.
- Website, public docs, README, or START_HERE mutation.
- qwork, qstart, qresume, or qshell mutation.
- Backup, restore, qsl-backup, backup status, backup plan, rollback subtree,
  `/backup/qsl`, timers, fstab, or systemd mutation.
- Public technical paper work.
- Durable Director State Index output.

## Validation commands

Run from the qsl-protocol repo root:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
```

Also run the exact closeout scope guard, link check, leak scan, overclaim scan,
classifier, PR body preflight, and goal-lint.

## Expected results

- READY_COUNT is 1.
- The sole READY item is NA-0475.
- NA-0474 is DONE.
- NA-0473 is DONE.
- D-0936 exists exactly once.
- D-0937 exists exactly once.
- D-0938 is absent.
- Duplicate decision count is zero.
- Changed paths are limited to the five allowed closeout governance paths.
- No public overclaim is introduced.
- No runtime/crypto/dependency/Cargo/lockfile/workflow/test/fuzz/vector/formal
  mutation occurs.
- No backup or restore is run.
- No qsl-backup mutation occurs.
- No status or plan mutation occurs.

## Markers

- `NA0474_CLOSEOUT_PR1218_MERGED_OK`
- `NA0474_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0474_DONE_OK`
- `NA0475_READY_OK`
- `NA0474_CLOSEOUT_D0937_OK`
- `NA0474_CLOSEOUT_NO_IMPLEMENTATION_MUTATION_OK`
- `NA0474_CLOSEOUT_NO_PUBLIC_OVERCLAIM_OK`
- `NA0474_CLOSEOUT_BACKUP_BOUNDARY_OK`
