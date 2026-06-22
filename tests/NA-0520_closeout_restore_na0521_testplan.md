Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-22

# NA-0520 closeout and NA-0521 restoration testplan

## Scope

This testplan validates the NA-0520 closeout-only governance patch after PR #1312 merged the forwarding capability probe. Closeout marks NA-0520 DONE, restores NA-0521 as the sole READY successor, records D-1030, updates TRACEABILITY and the rolling journal, and does not implement NA-0521.

Allowed closeout paths:

```text
NEXT_ACTIONS.md
DECISIONS.md
TRACEABILITY.md
docs/ops/ROLLING_OPERATIONS_JOURNAL.md
tests/NA-0520_closeout_restore_na0521_testplan.md
```

## Required proof

Closeout proof must show:

- PR #1312 merged at `3b4e30b7b04b`.
- PR #1312 final head was `2abfb5c01abe`.
- Post-merge public-safety on `3b4e30b7b04b` completed success inside the short attach/early-failure window.
- D-1029 exists once.
- D-1030 exists once after patch.
- NA-0520 is DONE.
- NA-0521 is READY.
- READY_COUNT is exactly 1.
- Duplicate decision count is zero.

## Required validation

Run from the qsl-protocol repo root:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh NEXT_ACTIONS.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md tests/NA-0520_closeout_restore_na0521_testplan.md
```

Run exact closeout scope guard allowing only the five paths above.

Run PR-body preflight and goal-lint with a PR body containing:

```text
Goals: G1, G2, G3, G4, G5
```

Run added-line overclaim scan and require zero affirmative findings.

## Forbidden closeout behavior

Closeout must not:

- implement NA-0521;
- run qsc E2EE;
- run qsc send/receive;
- run qsl-server or qsl-attachments;
- run SSH or remote commands;
- perform remote file writes;
- install packages;
- use sudo/admin action;
- generate or install SSH keys;
- mutate authorized_keys;
- mutate SSH config;
- mutate known_hosts;
- run qwork/qstart/qresume;
- run qsl-backup;
- mutate qsc source/test/fuzz/Cargo paths;
- mutate workflow/script/helper/dependency paths;
- mutate corpus/vector/input paths;
- mutate formal/refimpl/service/public/backup paths.

## Acceptance criteria

- Scope guard is exactly the five closeout paths.
- D-1030 records NA-0520 closeout and NA-0521 restoration.
- NEXT_ACTIONS marks NA-0520 DONE and NA-0521 READY.
- TRACEABILITY records closeout as governance-only.
- Rolling journal records PR #1312 merge, post-merge public-safety, closeout branch/PR state, and no NA-0521 implementation.
- No public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim is introduced.
- Exactly one READY item remains.
