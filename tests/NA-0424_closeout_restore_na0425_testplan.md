Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0424 Closeout / Restore NA-0425 Testplan

## Purpose

Verify that NA-0424 is closed only after the stewardship canon implementation
PR merged and post-merge public-safety passed, and that NA-0425 is restored as
the sole READY successor without implementing NA-0425.

## Preconditions

- PR #1117 is MERGED.
- PR #1117 merge commit is `d72243588499`.
- Post-merge public-safety on `d72243588499` completed success.
- Queue before closeout has READY_COUNT 1 and READY NA-0424.
- D-0836 exists once.
- D-0837 is absent before closeout.
- Duplicate decision count is zero.

## Allowed closeout paths

- `NEXT_ACTIONS.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`;
- `tests/NA-0424_closeout_restore_na0425_testplan.md`.

No other qsl-protocol paths may change.

## Forbidden scope

This closeout must not:

- implement NA-0425;
- mutate runtime, crypto, dependency, workflow, qsl-server, qsl-attachments,
  qshield runtime, website, public docs, README, or START_HERE paths;
- run backup or restore;
- mutate qsl-backup;
- mutate backup status or backup plan files;
- mutate qwork/qstart/qresume/qshell;
- create public technical paper content;
- create independent autonomous Directors;
- allow more than one READY item;
- handle secret material;
- create or imply unsupported public/security/readiness/privacy/backup/restore
  claims.

## Required NEXT_ACTIONS changes

- Mark NA-0424 DONE.
- Restore exactly one READY item:
  `NA-0425 -- QSL Code / Crypto Audit Follow-Up Resumption Plan`.
- NA-0425 block must include:
  - read-only planning/inventory scope;
  - no runtime mutation;
  - no crypto mutation;
  - no dependency mutation;
  - no workflow mutation;
  - no public claim expansion;
  - no backup/restore/qsl-backup/status/plan mutation;
  - exactly-one-READY discipline.

## Required decision

Add D-0837:

`NA-0424 closeout and NA-0425 restoration`

D-0837 must state:

- NA-0424 is closed after PR #1117 merged and post-merge public-safety passed.
- D-0836 implemented the stewardship canon.
- NA-0425 is restored as the sole READY successor.
- The closeout does not implement NA-0425.
- The closeout preserves Lead Director final authority, advisory-only stewards,
  one-READY discipline, public-claim boundaries, no runtime/crypto/dependency/
  workflow mutation, no backup mutation, and no public overclaim.

## Validation commands

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --head HEAD \
  --allowed NEXT_ACTIONS.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0424_closeout_restore_na0425_testplan.md \
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
- READY NA-0425.
- NA-0424 DONE.
- D-0837 exists once.
- D-0838 absent.
- Duplicate decision count 0.
- Only the five allowed closeout paths changed.
- No NA-0425 implementation by closeout.
- No runtime/crypto/dependency/workflow/public/service/backup mutation.
- No public overclaim.

## Post-fix hardening review checklist

- Correctness under stress: queue restoration is deterministic and preserves
  one READY item.
- Minimality: only closeout governance/testplan paths changed.
- Maintainability: NA-0425 carries explicit allowed/forbidden scope and
  acceptance criteria.
- Coverage quality: queue, decisions, scope, link, leak, overclaim, classifier,
  dependency, qsc, and formal/model checks verify the closeout.
- Cross-lane stability: public-safety remains green and no platform-specific
  runtime path changes are introduced.
