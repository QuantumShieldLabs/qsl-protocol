Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0425 Code / Crypto Audit Follow-Up Resumption Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Validate that NA-0425 resumes QSL code/crypto audit follow-up planning with
bounded read-only inventory, stewardship-template application, explicit audit
domain prioritization, and exact NA-0426 successor selection, without mutating
runtime, crypto, dependencies, workflows, public surfaces, service surfaces, or
backup/local-ops state.

## Preconditions

- qwork proof files exist and are read without running qwork:
  - `/srv/qbuild/work/NA-0425/.qwork/startup.qsl-protocol.kv`
  - `/srv/qbuild/work/NA-0425/.qwork/startup.qsl-protocol.json`
- qwork proof fields report lane NA-0425, repo qsl-protocol, clean worktree,
  READY_COUNT 1, queue top READY NA-0425, requested lane status READY, and
  matching head/origin_main.
- Live `HEAD` and `origin/main` match the qwork proof after fetch.
- PR #1118 is MERGED at `cd88811051a7`.
- Queue helper reports READY_COUNT 1 and READY NA-0425.
- NA-0424 is DONE.
- Decision helper reports latest D-0837 and duplicate count zero.
- D-0836 exists once, D-0837 exists once, and D-0838 is absent at start.
- public-safety is green on current `origin/main`.
- `cargo audit --deny warnings` is green.
- `cargo tree -i rustls-webpki --locked` reports `rustls-webpki v0.103.13` or
  newer safe version.
- pqcrypto unmaintained RustSec blocker packages are not active in the locked
  dependency graph.
- qsl-backup SHA/source-count boundary matches the directive.

## Allowed scope

Allowed changed paths for the NA-0425 evidence PR:

- `docs/governance/evidence/NA-0425_qsl_code_crypto_audit_follow_up_resumption_plan.md`
- `tests/NA-0425_qsl_code_crypto_audit_follow_up_resumption_testplan.md`
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

Forbidden claims include no production-readiness, no public-internet-readiness,
no external-review-complete, no crypto-complete, no side-channel-free, no
metadata-free, no anonymity, no untraceability, no off-host-backup-complete, no
disaster-recovery-complete, no restore-proven, no backup-complete, no
vulnerability-free, no bug-free, and no perfect-crypto claim.

## Required evidence assertions

Verify the evidence doc includes:

- Executive summary.
- Live NA-0425 scope.
- qwork proof-file verification.
- NA-0424 stewardship canon inheritance.
- Prior code/crypto evidence intake.
- Stewardship template application.
- Read-only code/crypto surface inventory.
- Audit domain matrix.
- Prioritization.
- Selected successor.
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

## Prior evidence intake assertions

Verify the evidence records:

- NA-0394 PQC standards mapping inheritance.
- NA-0395 IETF/CFRG RFC and draft boundary inheritance.
- NA-0396 dependency/advisory trigger policy inheritance.
- NA-0397 code/crypto audit candidate groups A through J.
- NA-0418 pqcrypto remediation and `ml-kem` provider replacement.
- Previously identified no-go claims.
- Known evidence gaps.
- Current validation used for qsc/formal/qshield-family evidence.
- Residual public-claim constraints.

## Read-only inventory assertions

Verify read-only inventory records:

- command family used;
- roots requested;
- roots scanned;
- missing top-level roots, if any;
- count summaries;
- top file clusters;
- caveat that counts are planning signals, not bug findings.

Required term groups:

- crypto API / provider terms;
- nonce / key / RNG / transcript terms;
- panic / fail-closed terms;
- unsafe / memory / FFI terms;
- formal / fuzz / vector terms;
- side-channel / timing terms.

## Audit domain matrix assertions

Verify the matrix covers:

- A. Crypto API misuse / provider boundary.
- B. Nonce / key / RNG lifecycle.
- C. KEM / signature / key schedule / transcript binding.
- D. Panic / unwrap / expect / abort / fail-closed behavior.
- E. Unsafe / memory safety / FFI.
- F. Side-channel / timing / secret-dependent behavior caveats.
- G. Fuzz / property / differential / vector testing.
- H. Formal model / implementation alignment.
- I. Dependency duplication / crypto dependency family.
- J. Demo / refimpl / service boundary.

Each domain must include:

- purpose;
- representative files or roots;
- inherited evidence;
- current validation coverage;
- evidence gaps;
- public-claim caveats;
- recommended future lane type;
- priority classification;
- steward review recommendation;
- what must not happen.

## Successor selection assertions

Verify normal successor selection:

`NA-0426 -- QSL Crypto API / Provider Boundary Read-Only Audit Plan`

Verify rationale:

- D257/NA-0418 recently changed provider implementation from pqcrypto to
  `ml-kem`.
- `PqKem768` provider boundary is the highest-leverage first audit domain.
- Future lane can remain read-only.
- Future lane sets the pattern for later nonce/key/RNG and fail-closed audits.

Alternative successors must be rejected unless an active blocker is proven:

- dependency blocker triage;
- fail-closed crypto error boundary blocker triage;
- evidence gap resolution plan.

## Required decision assertions

Verify D-0838 exists once after the patch and states:

- code/crypto audit stream is resumed;
- stewardship canon is used as advisory structure;
- selected first audit domain is crypto API/provider boundary;
- selected NA-0426 successor is exact;
- no runtime/crypto/dependency/workflow mutation occurred;
- no public crypto-complete claim is made;
- no backup/restore occurred;
- no public overclaim is made.

D-0839 must remain absent before optional closeout.

## Validation commands

Required local validation:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file /tmp/na0425_pr_body.md --scan-overclaims
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

Scope guard must prove only the five allowed qsl-protocol paths changed.

## Public-safety and CI

Before merge, required PR checks must pass, including public-safety. After
merge, public-safety must complete success on the merge commit. Use bounded
REST polling only; do not use watch modes.

## Post-fix hardening review requirements

Before declaring complete, report:

- Correctness under stress: the selected successor remains read-only and no
  audit planning signal is treated as a bug finding.
- Minimality: only the five allowed governance/testplan paths changed.
- Maintainability: matrix and steward summaries are structured for future lane
  reuse.
- Coverage quality: validation checks prove scope, queue, decisions, link,
  leak, overclaim, dependency, qsc, and formal boundaries.
- Cross-lane stability: no macOS/Linux runtime behavior changed because no
  runtime, crypto, dependency, or workflow path changed.

## Acceptance criteria

- qwork proof files are verified without running qwork.
- READY_COUNT remains exactly one.
- READY remains NA-0425 until optional closeout.
- D-0838 exists once.
- D-0839 remains absent before closeout.
- No duplicate decision IDs exist.
- Evidence doc and testplan are present.
- TRACEABILITY and rolling journal are updated.
- Normal NA-0426 successor is selected unless a blocker is proven.
- Cargo audit is green.
- rustls-webpki remains `v0.103.13` or newer safe version.
- pqcrypto RustSec blocker packages are absent.
- No runtime/crypto/dependency/workflow/public/service/backup mutation occurs.
- No public-readiness, no production-readiness, and no public-internet-readiness
  claim is introduced.
- No external-review-complete, no crypto-complete, and no side-channel-free claim
  is introduced.
- No vulnerability-free, no bug-free, and no perfect-crypto claim is introduced.
