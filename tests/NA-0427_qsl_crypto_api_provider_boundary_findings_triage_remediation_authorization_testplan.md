Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0427 Crypto API / Provider Boundary Findings Triage Remediation Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Validate that NA-0427 consumes every NA-0426 provider-boundary finding, records
bounded stewardship triage, selects the exact NA-0428 successor, and preserves
all no-runtime, no-crypto, no-dependency, no-Cargo, no-workflow, no-service,
no-public-surface, no-backup, and no-public-overclaim boundaries.

## Preconditions

- qwork proof files exist and are read without running qwork:
  - `/srv/qbuild/work/NA-0427/.qwork/startup.qsl-protocol.kv`
  - `/srv/qbuild/work/NA-0427/.qwork/startup.qsl-protocol.json`
- qwork proof fields report lane NA-0427, repo qsl-protocol, clean worktree,
  READY_COUNT 1, queue top READY NA-0427, requested lane status READY, and
  matching head/origin_main.
- Live `HEAD` and `origin/main` match qwork proof after fetch.
- PR #1122 is MERGED at `d3daaad926c6`.
- Queue helper reports READY_COUNT 1 and READY NA-0427.
- NA-0426 is DONE.
- Decision helper reports latest D-0841 and duplicate count zero.
- D-0840 exists once, D-0841 exists once, and D-0842 is absent at start.
- public-safety is green on current `origin/main`.
- Root `cargo audit --deny warnings` is green.
- `cargo tree -i rustls-webpki --locked` reports `rustls-webpki v0.103.13` or
  newer safe version.
- pqcrypto unmaintained RustSec blocker packages are absent from the root
  locked graph.
- qsl-backup checksum/source-count boundary matches the directive.

## Allowed scope

Allowed changed paths for the NA-0427 evidence PR:

- `docs/governance/evidence/NA-0427_qsl_crypto_api_provider_boundary_findings_triage_remediation_authorization_plan.md`
- `tests/NA-0427_qsl_crypto_api_provider_boundary_findings_triage_remediation_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No other qsl-protocol paths may change.

## Forbidden scope

Forbidden changes include:

- runtime code;
- crypto implementation code;
- dependencies, `Cargo.toml`, or `Cargo.lock`;
- qsc fuzz lock or fuzz manifest mutation;
- tests or vectors outside this governance testplan;
- workflows or branch protection;
- qsl-server, qsl-attachments, qshield runtime, website, public docs, README,
  or START_HERE;
- qwork, qstart, qresume, or qshell;
- qsl-backup;
- backup status or backup plan files;
- `/backup/qsl`;
- rollback subtree paths;
- public technical paper content;
- secret material.

Forbidden public assurance claims:

- no public-readiness claim;
- no production-readiness claim;
- no public-internet-readiness claim;
- no external-review-complete claim;
- no crypto-complete claim;
- no side-channel-free claim;
- no metadata-free claim;
- no anonymity claim;
- no untraceability claim;
- no off-host-backup-complete claim;
- no disaster-recovery-complete claim;
- no restore-proven claim;
- no backup-complete claim;
- no vulnerability-free claim;
- no bug-free claim;
- no perfect-crypto claim.

## Required evidence assertions

Verify the evidence doc includes:

- Executive summary.
- Live NA-0427 scope.
- qwork proof-file verification.
- NA-0426 inheritance.
- Stewardship template application.
- Findings matrix consumption.
- Dependency / nested fuzz lock triage.
- Provider error path / no-mutation triage.
- Feature-name / claim boundary triage.
- Formal / vector / property / fuzz triage.
- Secret material / side-channel / service boundary triage.
- Prioritization and successor selection.
- Future path/scope bundle.
- Future validation/marker plan.
- Public claim/external review/website boundary.
- Rejected alternatives.
- Backup-impact statement.
- Next recommendation.

## Stewardship assertions

Verify advisory summaries are present for:

- Crypto / Protocol Steward.
- CI / Dependency / Release Health Steward.
- Public Claims / External Review Steward.
- Product / Demo / Service Boundary Steward.
- Local Ops / Backup / Restore Steward.

Each summary must record:

- review question;
- evidence reviewed;
- findings;
- risk classification;
- public-claim impact;
- scope impact;
- recommended action.

## Findings matrix assertions

Verify every inherited finding F-0426-01 through F-0426-09 records:

- original severity;
- current triage classification;
- whether immediate remediation is needed;
- whether a future lane is needed;
- whether the finding can be grouped with another finding;
- steward domain;
- public-claim impact;
- exact recommended action.

Allowed triage outcomes:

- ACCEPTED_NO_ACTION
- BACKLOG_CANDIDATE
- NEXT_CANDIDATE
- BLOCKER_CANDIDATE
- CLAIM_BOUNDARY_ONLY
- EVIDENCE_GAP
- WATCH_ONLY

## Dependency and fuzz-lock assertions

Required read-only commands:

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock || true
rg -n "pqcrypto|pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz Cargo.lock Cargo.toml qsl/qsl-client/qsc/Cargo.toml || true
```

Verify:

- root cargo audit is green;
- root `rustls-webpki` is `v0.103.13` or newer safe version;
- root `ml-kem` inverse tree reaches `quantumshield_refimpl`;
- root pqcrypto inverse trees report package absence;
- qsc fuzz lock exists and records pqcrypto package entries;
- nested qsc fuzz lock audit red state is recorded as triage evidence;
- F-0426-04 is classified `FUZZ_LOCK_ACTIVE_SECURITY_BLOCKER`;
- selected NA-0428 is
  `QSL qsc Fuzz Lock pqcrypto Residual Dependency Blocker Authorization Plan`;
- no Cargo, dependency, workflow, runtime, crypto, test, or vector file is
  mutated by NA-0427.

## Provider-error assertions

Verify the evidence records:

- provider-level wrong-length KEM reject coverage exists;
- qsc provider-error marker paths exist for `pq_encap_failed` and
  `pq_decap_failed`;
- no qsc tests were found that directly assert those exact marker paths;
- F-0426-02 remains a qsc provider-error/no-mutation evidence gap;
- F-0426-02 does not outrank the active nested fuzz-lock blocker.

## Formal/vector/property/fuzz assertions

Verify the evidence records:

- formal roots are bounded and state-machine focused;
- vector roots include ML-KEM-768 categories but do not directly prove provider
  implementation alignment;
- qsc fuzz targets do not directly target `PqKem768`;
- property/fuzz/differential expansion should not proceed before fuzz-lock
  dependency blocker authorization.

## Required decision assertions

Verify D-0842 exists once after the patch and states:

- NA-0426 findings were consumed;
- selected NA-0428 successor is exact;
- dependency/fuzz-lock triage result is recorded;
- provider-error/no-mutation triage result is recorded;
- no runtime/crypto/dependency/Cargo/workflow mutation occurred;
- no backup/restore occurred;
- no public crypto-complete claim is made;
- no vulnerability-free or perfect-crypto claim is made;
- stewardship template was used;
- exactly one READY remains mandatory.

D-0843 must remain absent before optional closeout.

## Validation commands

Required local validation:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --head HEAD \
  --allowed docs/governance/evidence/NA-0427_qsl_crypto_api_provider_boundary_findings_triage_remediation_authorization_plan.md \
  --allowed tests/NA-0427_qsl_crypto_api_provider_boundary_findings_triage_remediation_authorization_testplan.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --forbidden .github/ \
  --forbidden Cargo.toml \
  --forbidden Cargo.lock \
  --forbidden qsl/ \
  --forbidden qsl-server/ \
  --forbidden qsl-attachments/ \
  --forbidden apps/ \
  --forbidden website/ \
  --forbidden README.md \
  --forbidden START_HERE.md
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file /tmp/na0427_pr_body.md --scan-overclaims
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Additional checks:

- exact changed-path guard for the five allowed NA-0427 paths;
- added-line overclaim scan;
- classifier proof;
- goal-lint with `Goals: G1, G2, G3, G4, G5`;
- required checks, including public-safety, green before merge;
- post-merge public-safety green after merge.

## Acceptance criteria

- READY_COUNT 1.
- READY NA-0427 before optional closeout.
- NA-0426 DONE.
- D-0840 exists once.
- D-0841 exists once.
- D-0842 exists once.
- D-0843 absent before optional closeout.
- Duplicate decision count 0.
- Only the five allowed evidence paths changed.
- F-0426-01 through F-0426-09 are consumed.
- Selected NA-0428 is the qsc fuzz-lock pqcrypto residual dependency blocker
  authorization plan.
- Root cargo audit is green.
- No runtime/crypto/dependency/Cargo/workflow/public/service/backup mutation.
- No public overclaim.

## Post-fix hardening review checklist

- Correctness under stress: qwork proof, live queue, decision uniqueness,
  dependency-root health, nested fuzz-lock red state, and public-safety are all
  checked independently.
- Minimality: only authorized governance evidence/testplan/decision/
  traceability/journal paths change.
- Maintainability: findings remain stable IDs with explicit classifications and
  a single exact successor.
- Coverage quality: provider-error, fuzz-lock, formal/vector, property/fuzz,
  secret-material, side-channel, and service-boundary gaps are not treated as
  remediated by triage.
- Cross-lane stability: no macOS/Linux runtime, Cargo, workflow, test, or
  vector path changes are introduced by NA-0427.
