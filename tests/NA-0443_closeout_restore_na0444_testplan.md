Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-06-08

# NA-0443 Closeout / Restore NA-0444 Testplan

## Scope

This testplan records governance-only closeout validation for NA-0443 and
restoration of the selected NA-0444 successor.

Allowed closeout mutation paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0443_closeout_restore_na0444_testplan.md`

Forbidden closeout mutation paths:

- runtime, crypto, dependency, Cargo, lockfile, workflow, qsl-server,
  qsl-attachments, qshield runtime, website, public docs, README, and
  START_HERE paths;
- executable tests, fuzz targets, vectors, and formal models;
- qwork/qstart/qresume/qshell paths;
- qsl-backup, backup status, backup plan, rollback, and backup tree paths.

## Preconditions

- qsl-protocol PR #1155 is merged.
- PR #1155 merge commit is `087346b44b40`.
- post-merge public-safety is completed success on `087346b44b40`.
- post-merge qsc-adversarial-smoke is completed success on `087346b44b40`.
- D-0873 exists once.
- D-0874 is absent before closeout patch.
- READY_COUNT is 1 and READY is NA-0443 before closeout patch.

## Expected Queue State

After the closeout patch:

- NA-0443 is DONE.
- NA-0444 is READY.
- READY_COUNT is 1.
- NA-0444 title is:
  `QSL Key Lifecycle Secret Cleanup / Zeroization Evidence Policy Authorization Plan`.
- NA-0444 preserves no-runtime/no-crypto/no-dependency/no-test-or-fuzz-mutation
  and no-public-overclaim boundaries.
- NA-0444 does not implement cleanup, zeroization, runtime, crypto, dependency,
  Cargo, lockfile, workflow, executable test, fuzz target, vector, formal model,
  qsl-server, qsl-attachments, qshield runtime, website, public docs, README,
  START_HERE, qwork/qstart/qresume/qshell, backup, restore, or qsl-backup work.

## Expected Decision State

- D-0873 exists once.
- D-0874 exists once after the patch.
- D-0875 is absent.
- duplicate decision count is zero.

## Validation Commands

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file "$PR_BODY_FILE"
bash scripts/ci/classify_ci_scope.sh $(git diff --name-only origin/main)
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
```

## Public Claim Boundary

This closeout is internal governance evidence only.

No public-readiness claim is introduced.
No production-readiness claim is introduced.
No public-internet-readiness claim is introduced.
No external-review-complete claim is introduced.
No crypto-complete claim is introduced.
No secret-material-complete claim is introduced.
No side-channel-free claim is introduced.
No bug-free claim is introduced.
No vulnerability-free claim is introduced.
No perfect-crypto claim is introduced.

Cargo audit green remains dependency-health evidence only.

## Backup / Restore Boundary

Codex did not run backup or restore.
Codex did not run sudo.
Codex did not mutate qsl-backup, backup status files, backup plan files,
rollback subtree paths, timers, fstab, source lists, retention, backup scripts,
or backup tree paths.
