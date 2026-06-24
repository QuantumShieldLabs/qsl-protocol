# NA-0535 Closeout and NA-0536 Restoration Testplan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-06-24

## Purpose

Verify that NA-0535 closeout accepts the merged D-1060 implementation evidence, marks NA-0535 DONE, restores the provided NA-0536 successor as the sole READY item, and preserves the qsc/qsl-server/qsl-attachments/security claim boundaries.

## Preconditions

- PR #1343 is merged as `954e840b66ce`.
- D-1060 exists exactly once.
- Post-merge public-safety for `954e840b66ce` completed success inside the short attach/early-failure window.
- NA-0535 result classification remains `REMOTE_E2EE_WRONG_PEER_STALE_TRUST_NEGATIVES_PASS`.

## Scope

Allowed closeout paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0535_closeout_restore_na0536_testplan.md`

No implementation, qsc source/test/fuzz/Cargo, workflow/script/helper, dependency/lockfile, corpus/vector/input, formal/refimpl/service/public/backup, qsl-server, qsl-attachments, qwork, qsl-backup, remote host, SSH, or branch-protection mutation is allowed.

## Validation

1. Queue proof:
   - `READY_COUNT=1`
   - READY item is `NA-0536 -- QSL Remote qsc E2EE Repeated-Run / Cleanup / Freshness Scope Authorization Plan`
   - NA-0535 is DONE

2. Decision proof:
   - D-1060 exists once
   - D-1061 exists once
   - No duplicate decision IDs

3. Scope proof:
   - `git diff --name-only origin/main...HEAD` contains exactly the five allowed closeout paths.
   - No qsc source/test/fuzz/Cargo, workflow/script/helper, dependency/lockfile, corpus/vector/input, formal/refimpl/service/public/backup, qsl-server, or qsl-attachments path changes.

4. Content proof:
   - D-1061 consumes D-1060 and PR #1343.
   - D-1061 records post-merge public-safety success.
   - D-1061 restores only the supplied NA-0536 successor block.
   - NA-0536 is authorization-only and not implemented.

5. Safety scans:
   - Link check reports `TOTAL_MISSING 0`.
   - Added-line private-material scan finds no private keys, passphrases, tokens, or secret-looking dumps.
   - Added-line overclaim scan confirms the closeout introduces no readiness or completeness claims.

6. Local validation:
   - `git diff --check`
   - `cargo audit --deny warnings`
   - `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
   - `cargo fmt --check`

## Expected Result

Closeout passes if NA-0535 is DONE, NA-0536 is the only READY item, D-1061 exists once, all scope and safety scans pass, and local validation is green.
