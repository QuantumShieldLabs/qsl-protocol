Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0423 Closeout / Restore NA-0424 Testplan

## Purpose

Verify that NA-0423 is closed only after the governance authorization PR merged
and post-merge public-safety passed, and that NA-0424 is restored as the sole
READY successor without implementing NA-0424.

## Preconditions

- PR #1115 is MERGED.
- PR #1115 merge commit is `de1741e05657`.
- Post-merge public-safety on `de1741e05657` completed success.
- Queue before closeout has READY_COUNT 1 and READY NA-0423.
- D-0834 exists once.
- D-0835 is absent before closeout.
- Duplicate decision count is zero.

## Allowed closeout paths

- `NEXT_ACTIONS.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`;
- `tests/NA-0423_closeout_restore_na0424_testplan.md`.

No other qsl-protocol paths may change.

## Forbidden scope

This closeout must not:

- implement NA-0424;
- create `docs/governance/DOMAIN_STEWARDSHIP_AND_DIRECTOR_WORKFLOW.md`;
- create the NA-0424 evidence doc or NA-0424 testplan;
- create independent autonomous Directors;
- allow more than one READY item;
- change branch protection;
- mutate runtime, crypto, dependency, workflow, qsl-server, qsl-attachments,
  qshield runtime, website, public docs, README, or START_HERE paths;
- run backup or restore;
- mutate qsl-backup;
- mutate backup status or backup plan files;
- mutate qwork/qstart/qresume/qshell;
- create public technical paper content;
- handle secret material;
- create or imply unsupported public/security/readiness/privacy/backup/restore
  claims.

## Required NEXT_ACTIONS changes

- Mark NA-0423 DONE.
- Restore exactly one READY item:
  `NA-0424 -- QSL Domain Stewardship Operating Model Canon Implementation Harness`.
- NA-0424 block must include:
  - Lead Director final authority;
  - advisory-only stewards;
  - exactly-one-READY discipline;
  - public-claim boundaries;
  - no runtime/dependency/workflow mutation;
  - no backup mutation;
  - no public-readiness overclaim;
  - no independent autonomous Directors.

## Required decision

Add D-0835:

`NA-0423 closeout and NA-0424 restoration`

D-0835 must state:

- NA-0423 is closed after PR #1115 merged and post-merge public-safety passed.
- D-0834 authorized advisory domain stewardship.
- NA-0424 is restored as the sole READY successor.
- The closeout does not implement NA-0424.
- The closeout preserves Lead Director final authority, advisory-only stewards,
  one-READY discipline, public-claim boundaries, no runtime/dependency/workflow
  mutation, no backup mutation, and no public-readiness overclaim.

## Validation commands

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --head HEAD \
  --allowed NEXT_ACTIONS.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0423_closeout_restore_na0424_testplan.md \
  --forbidden .github/ \
  --forbidden Cargo.toml \
  --forbidden Cargo.lock \
  --forbidden qsl/ \
  --forbidden qsl-server/ \
  --forbidden qsl-attachments/ \
  --forbidden apps/ \
  --forbidden website/ \
  --forbidden README.md \
  --forbidden START_HERE.md \
  --forbidden docs/governance/DOMAIN_STEWARDSHIP_AND_DIRECTOR_WORKFLOW.md \
  --forbidden docs/governance/evidence/NA-0424_qsl_domain_stewardship_operating_model_canon_implementation_harness.md \
  --forbidden tests/NA-0424_qsl_domain_stewardship_operating_model_canon_implementation_testplan.md
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Additional checks:

- exact changed-path guard for the five allowed closeout paths;
- added-line overclaim scan;
- classifier proof;
- PR body preflight;
- goal-lint with `Goals: G1, G2, G3, G4, G5`;
- required checks, including public-safety, green before merge;
- post-merge public-safety green after merge.

## Acceptance criteria

- READY_COUNT 1.
- READY NA-0424.
- NA-0423 DONE.
- D-0835 exists once.
- D-0836 absent.
- Duplicate decision count 0.
- Only the five allowed closeout paths changed.
- No NA-0424 implementation by closeout.
- No runtime/dependency/workflow/public/backup mutation.
- No public overclaim.

## Post-fix hardening review checklist

- Correctness under stress: queue restoration is deterministic and preserves
  one READY item.
- Minimality: only closeout governance/testplan paths changed.
- Maintainability: NA-0424 carries explicit allowed/forbidden scope and
  acceptance criteria.
- Coverage quality: queue, decisions, scope, link, leak, overclaim, classifier,
  dependency, qsc, and formal/model checks verify the closeout.
- Cross-lane stability: public-safety remains green and no platform-specific
  runtime path changes are introduced.
