Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0426 Closeout / Restore NA-0427 Testplan

## Purpose

Verify that NA-0426 is closed only after the crypto API / provider boundary
read-only audit PR merged and post-merge public-safety passed, and that NA-0427
is restored as the sole READY successor without implementing NA-0427.

## Preconditions

- PR #1121 is MERGED.
- PR #1121 merge commit is `c29131a754b4`.
- Post-merge public-safety on `c29131a754b4` completed success.
- Queue before closeout has READY_COUNT 1 and READY NA-0426.
- D-0840 exists once.
- D-0841 is absent before closeout.
- Duplicate decision count is zero.
- D-0840 selected `NA-0427 -- QSL Crypto API / Provider Boundary Findings Triage and Remediation Authorization Plan`.

## Allowed closeout paths

- `NEXT_ACTIONS.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`;
- `tests/NA-0426_closeout_restore_na0427_testplan.md`.

No other qsl-protocol paths may change.

## Forbidden scope

This closeout must not:

- implement NA-0427;
- mutate runtime, crypto, dependency, Cargo, workflow, qsl-server,
  qsl-attachments, qshield runtime, website, public docs, README, or
  START_HERE paths;
- run qwork, qstart, or qresume;
- run backup or restore;
- mutate qsl-backup;
- mutate backup status or backup plan files;
- mutate qwork/qstart/qresume/qshell;
- create public technical paper content;
- allow more than one READY item;
- handle secret material;
- create or imply unsupported public, security, readiness, privacy, backup, or restore claims;
- create or imply unsupported external-review or cryptographic-completeness claims;
- create or imply unsupported side-channel assurance, defect-absence, vulnerability-absence, or absolute-crypto-assurance claims.

## Required NEXT_ACTIONS changes

- Mark NA-0426 DONE.
- Restore exactly one READY item:
  `NA-0427 -- QSL Crypto API / Provider Boundary Findings Triage and Remediation Authorization Plan`.
- NA-0427 block must include:
  - findings-matrix triage objective;
  - no runtime mutation;
  - no crypto mutation;
  - no dependency or Cargo mutation;
  - no workflow mutation;
  - no public overclaim;
  - no backup/restore/qsl-backup/status/plan mutation;
  - no secret material handling;
  - exactly-one-READY discipline.

## Required decision

Add D-0841:

`NA-0426 closeout and NA-0427 restoration`

D-0841 must state:

- NA-0426 is closed after PR #1121 merged and post-merge public-safety passed.
- D-0840 completed the read-only crypto API / provider boundary audit.
- D-0840 selected the provider-boundary findings triage successor.
- NA-0427 is restored as the sole READY successor.
- The closeout does not implement NA-0427.
- The closeout preserves one-READY discipline, public-claim boundaries,
  no runtime/crypto/dependency/Cargo/workflow mutation, no backup mutation,
  and no public overclaim.

## Validation commands

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --head HEAD \
  --allowed NEXT_ACTIONS.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0426_closeout_restore_na0427_testplan.md \
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
  --forbidden docs/governance/evidence/NA-0426_qsl_crypto_api_provider_boundary_read_only_audit_plan.md \
  --forbidden tests/NA-0426_qsl_crypto_api_provider_boundary_read_only_audit_testplan.md
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
- READY NA-0427.
- NA-0426 DONE.
- D-0840 exists once.
- D-0841 exists once.
- D-0842 absent.
- Duplicate decision count 0.
- Only the five allowed closeout paths changed.
- No NA-0427 implementation by closeout.
- No runtime/crypto/dependency/Cargo/workflow/public/service/backup mutation.
- No public overclaim.

## Post-fix hardening review checklist

- Correctness under stress: queue restoration is deterministic and preserves
  one READY item.
- Minimality: only closeout governance/testplan paths changed.
- Maintainability: NA-0427 carries explicit allowed/forbidden scope and
  acceptance criteria.
- Coverage quality: queue, decisions, scope, link, leak, overclaim, classifier,
  dependency, qsc, and formal/model checks verify the closeout.
- Cross-lane stability: public-safety remains green and no platform-specific
  runtime path changes are introduced.
