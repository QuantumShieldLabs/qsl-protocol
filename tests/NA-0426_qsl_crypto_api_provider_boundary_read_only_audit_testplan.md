Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0426 Crypto API / Provider Boundary Read-Only Audit Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Validate that NA-0426 performs a bounded read-only audit of the QSL crypto API
and provider boundary, records a findings matrix, selects the exact NA-0427
successor, and preserves all runtime, crypto, dependency, workflow, service,
backup, local-ops, and public-claim boundaries.

## Preconditions

- qwork proof files exist and are read without running qwork:
  - `/srv/qbuild/work/NA-0426/.qwork/startup.qsl-protocol.kv`
  - `/srv/qbuild/work/NA-0426/.qwork/startup.qsl-protocol.json`
- qwork proof fields report lane NA-0426, repo qsl-protocol, clean worktree,
  READY_COUNT 1, queue top READY NA-0426, requested lane status READY, and
  matching head/origin_main.
- Live `HEAD` and `origin/main` match qwork proof after fetch.
- PR #1120 is MERGED at `36b342c4e71e`.
- Queue helper reports READY_COUNT 1 and READY NA-0426.
- NA-0425 is DONE.
- Decision helper reports latest D-0839 and duplicate count zero.
- D-0838 exists once, D-0839 exists once, and D-0840 is absent at start.
- public-safety is green on current `origin/main`.
- `cargo audit --deny warnings` is green.
- `cargo tree -i rustls-webpki --locked` reports `rustls-webpki v0.103.13` or
  newer safe version.
- pqcrypto unmaintained RustSec blocker packages are absent from the root
  locked graph.
- qsl-backup SHA/source-count boundary matches the directive.

## Allowed scope

Allowed changed paths for the NA-0426 evidence PR:

- `docs/governance/evidence/NA-0426_qsl_crypto_api_provider_boundary_read_only_audit_plan.md`
- `tests/NA-0426_qsl_crypto_api_provider_boundary_read_only_audit_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No other qsl-protocol paths may change.

## Forbidden scope

Forbidden changes include:

- runtime code;
- protocol or crypto semantics;
- dependencies, `Cargo.toml`, or `Cargo.lock`;
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

Forbidden claims include no public-readiness, no production-readiness, no
public-internet-readiness, no external-review-complete, no crypto-complete, no
side-channel-free, no metadata-free, no anonymity, no untraceability, no
off-host-backup-complete, no disaster-recovery-complete, no restore-proven, no
backup-complete, no vulnerability-free, no bug-free, and no perfect-crypto
claim.

## Required evidence assertions

Verify the evidence doc includes:

- Executive summary.
- Live NA-0426 scope.
- qwork proof-file verification.
- NA-0425 inheritance.
- Stewardship template application.
- Provider boundary file inventory.
- `PqKem768` / `ml-kem` provider boundary review.
- Fail-closed / negative test coverage review.
- Dependency / feature boundary review.
- Formal / vector / implementation alignment review.
- Public claim / service / demo boundary review.
- Findings matrix.
- Successor selection.
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

## Provider-boundary assertions

Verify the audit records:

- `PqKem768` definition location.
- `StdCrypto` provider implementation location.
- `ml-kem` types and functions used.
- public key, secret key, ciphertext, and shared-secret representations.
- explicit conversion/length failure mapping to deterministic errors.
- fail-closed malformed public-key, secret-key, and ciphertext behavior.
- shared-secret output handling.
- qsc provider-boundary usage.
- historical `pqcrypto` feature-name caveat.
- root pqcrypto package absence.
- nested qsc fuzz lock dependency-health evidence gap.

## Fail-closed and coverage assertions

Verify the audit records:

- roundtrip provider test coverage;
- tampered ciphertext provider test coverage;
- wrong-length public-key rejection;
- wrong-length secret-key rejection;
- wrong-length ciphertext rejection;
- deterministic reject behavior where feasible;
- stateless provider no-mutation assessment;
- qsc higher-boundary coverage gaps;
- future property/fuzz/differential coverage recommendation.

## Dependency and feature assertions

Required commands:

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
cargo metadata --locked --format-version=1
```

Verify:

- cargo audit is green;
- `rustls-webpki` is `v0.103.13` or newer safe version;
- root `ml-kem` inverse tree reaches `quantumshield_refimpl`;
- root pqcrypto inverse trees report package absence;
- root metadata shows `qsc` depends on `quantumshield_refimpl` with feature
  `pqcrypto`;
- root metadata shows `quantumshield_refimpl` feature `pqcrypto` maps to
  `pqkem` plus `ml-dsa`;
- nested `qsl/qsl-client/qsc/fuzz/Cargo.lock` pqcrypto entries are recorded as
  a finding, not silently ignored.

## Formal/vector assertions

Verify the audit records:

- formal roots reviewed;
- formal models are crypto-agnostic and bounded;
- SCKA KEM vectors exist and include roundtrip/tamper/invalid-size categories;
- qsc suite-id vectors are boundary/transcript evidence, not provider-output
  proof;
- provider behavior is not claimed to be formally proven;
- future KAT/differential/vector mapping is recommended.

## Required decision assertions

Verify D-0840 exists once after the patch and states:

- provider boundary audit completed read-only;
- findings matrix was created;
- selected NA-0427 successor is exact;
- no runtime/crypto/dependency/workflow mutation occurred;
- no backup/restore occurred;
- no public crypto-complete claim is made;
- no vulnerability-free or perfect-crypto claim is made;
- steward template was used;
- exactly one READY remains mandatory.

D-0841 must remain absent before optional closeout.

## Validation commands

Required local validation:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file /tmp/na0426_pr_body.md --scan-overclaims
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

- exact changed-path guard for the five allowed NA-0426 paths;
- added-line overclaim scan;
- classifier proof;
- PR body preflight;
- goal-lint with `Goals: G1, G2, G3, G4, G5`;
- required checks, including public-safety, green before merge;
- post-merge public-safety green after merge.

## Public-safety and CI

Before merge, required PR checks must pass, including public-safety. After
merge, public-safety must complete success on the merge commit. Use bounded
REST polling only; do not use watch modes.

## Acceptance criteria

- READY_COUNT 1.
- READY NA-0426 before optional closeout.
- NA-0425 DONE.
- D-0838 exists once.
- D-0839 exists once.
- D-0840 exists once.
- D-0841 absent before optional closeout.
- Duplicate decision count 0.
- Only the five allowed NA-0426 paths changed.
- Findings matrix exists.
- Selected NA-0427 successor is normal findings triage / remediation
  authorization.
- No runtime/crypto/dependency/workflow/public/service/backup mutation.
- No public overclaim.

## Post-fix hardening review checklist

- Correctness under stress: audit conclusions are evidence-backed and no
  blocker is hidden by normal successor selection.
- Minimality: only governance evidence, testplan, decision, traceability, and
  rolling journal paths changed.
- Maintainability: findings are stable IDs that NA-0427 can consume.
- Coverage quality: validation includes provider test command, qsc targeted
  test, formal checks, link/leak/overclaim scans, and dependency health.
- Cross-lane stability: Linux/macOS-facing affected areas are not changed;
  cargo fmt, qsc targeted test, provider test, and formal checks remain green.
