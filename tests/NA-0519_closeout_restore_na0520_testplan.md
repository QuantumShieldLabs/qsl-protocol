Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-22

# NA-0519 closeout and NA-0520 restoration testplan

## Scope

This closeout marks NA-0519 DONE after the NA-0519 proof-review PR merged and post-merge public-safety completed success. It restores NA-0520 as the sole READY successor and does not implement NA-0520.

Allowed closeout paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0519_closeout_restore_na0520_testplan.md`

## Required checks

Validate:

- exact five-path closeout scope guard;
- D-1027 exists once;
- D-1028 exists once after patch;
- duplicate decision count 0;
- NA-0519 status DONE;
- NA-0520 status READY;
- READY_COUNT 1;
- no NA-0520 implementation;
- no remote action;
- no SSH execution;
- no qsc send/receive;
- no remote E2EE;
- no qsl-server/qsl-attachments;
- no qsc source/test/fuzz/Cargo mutation;
- no workflow/script/helper/dependency mutation;
- no corpus/vector/input mutation;
- no formal/refimpl/service/public/backup mutation;
- no public/security/completion claim expansion.

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode full --paths NEXT_ACTIONS.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md tests/NA-0519_closeout_restore_na0520_testplan.md
bash scripts/ci/classify_ci_scope.sh NEXT_ACTIONS.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md tests/NA-0519_closeout_restore_na0520_testplan.md
```

Goal-lint must pass with a PR body containing:

```text
Goals: G1, G2, G3, G4, G5
Impact:
No-regression:
Tests/Vectors:
```

## Acceptance criteria

- NA-0519 proof-review PR #1310 merged as a merge commit.
- Post-merge public-safety completed success on the merge commit.
- D-1028 records closeout and NA-0520 restoration.
- NA-0519 is DONE.
- NA-0520 is READY.
- Exactly one READY item remains.
- Closeout does not implement NA-0520.
- No remote action, SSH execution, forwarding probe, qsc send/receive, remote E2EE, authorized_keys mutation, key generation/installation, SSH config mutation, known_hosts mutation, remote host mutation, sudo/admin action, package installation, qwork/qstart/qresume mutation, qsl-backup execution, qsl-server/qsl-attachments use, qsc implementation mutation, workflow/dependency mutation, corpus/vector/input mutation, formal/refimpl/service/public/backup mutation, or public/security/completion claim expansion occurs.
