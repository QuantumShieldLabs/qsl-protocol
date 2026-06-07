# NA-0438 qsc Provider Error Path Fuzz / Adversarial Coverage Authorization Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-07

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0438 classifies qsc provider-error fuzz/adversarial coverage
need, consumes NA-0436/NA-0437 evidence without overclaim, selects exactly one
NA-0439 successor, and makes only governance/testplan mutations.

## Protected invariants

- READY_COUNT remains 1.
- READY remains NA-0438 until optional closeout.
- D-0863 exists once after this lane.
- D-0864 remains absent until optional closeout.
- `pq_decap_failed` evidence remains bounded to the NA-0436 no-mutation test.
- `pq_encap_failed` remains defensive-branch documentation only.
- No executable coverage claim is made for `pq_encap_failed`.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, script, executable
  test, fuzz target, vector, service, public-surface, or backup/local-ops path
  is changed.
- No public claim expansion is introduced.

## Allowed scope

- `docs/governance/evidence/NA-0438_qsl_qsc_provider_error_path_fuzz_adversarial_coverage_authorization_plan.md`
- `tests/NA-0438_qsl_qsc_provider_error_path_fuzz_adversarial_coverage_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

Forbidden changed paths include runtime, crypto, dependency, Cargo manifest,
lockfile, workflow, script, executable test, fuzz target, vector, qsl-server,
qsl-attachments, qshield runtime, website, public docs, README, START_HERE,
qwork/qstart/qresume/qshell, qsl-backup, backup status, backup plan, rollback,
and backup tree paths.

Forbidden commands include qwork, qstart, qresume, sudo, backup, restore,
`cargo update`, `cargo generate-lockfile`, and dependency remediation commands.

## Queue and decision checks

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Required:

- READY_COUNT 1.
- READY NA-0438.
- NA-0437 DONE.
- NA-0436 DONE.
- NA-0435 DONE.
- NA-0434 BLOCKED.
- NA-0429 BLOCKED.
- D-0861 exists once.
- D-0862 exists once.
- D-0863 exists once after patching.
- D-0864 absent.
- duplicate decision count zero.

## Scope guard

Run after patching and before PR creation:

```bash
git diff --name-only origin/main
git ls-files --others --exclude-standard
```

The combined changed-path set must be exactly:

- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/governance/evidence/NA-0438_qsl_qsc_provider_error_path_fuzz_adversarial_coverage_authorization_plan.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0438_qsl_qsc_provider_error_path_fuzz_adversarial_coverage_authorization_testplan.md`

## Link, leak, classifier, and PR-body checks

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file "$PR_BODY_FILE"
```

Required:

- no whitespace errors;
- link check passes;
- added-line leak scan has zero findings;
- PR body includes `Goals:`, `Impact:`, `No-regression:`, and
  `Tests/Vectors:`;
- PR body does not contain prohibited public-claim phrases.

## Dependency and regression checks

Run:

```bash
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
rg -n "pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz/Cargo.lock || true
cargo fmt --check
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Required:

- the NA-0436 `pq_decap_failed` test still passes and emits its markers;
- qsc `send_commit` test still passes;
- provider `pqkem768` test still passes;
- root cargo audit passes;
- nested qsc fuzz lock audit passes;
- `rustls-webpki` is `v0.103.13` or newer safe version;
- root pqcrypto inverse-tree probes are absent or explicitly explained as
  expected zero-match proofs;
- nested qsc fuzz lock pqcrypto residual scan returns zero matches;
- formatting check passes;
- formal model checks pass.

## qsc adversarial smoke check

If locally feasible, run:

```bash
sh scripts/ci/qsc_adversarial.sh
```

If local `cargo fuzz` is unavailable, record exact output and rely on PR CI
qsc-adversarial-smoke for cargo-fuzz-backed smoke proof.

## Public claim boundary

Confirm:

- no production-readiness claim is introduced;
- no public-internet-readiness claim is introduced;
- no external-review-complete claim is introduced;
- no crypto-complete claim is introduced;
- no side-channel-free claim is introduced;
- no bug-free claim is introduced;
- no vulnerability-free claim is introduced;
- no perfect-crypto claim is introduced;
- no public technical paper content is introduced;
- no README, START_HERE, public docs, or website path is changed;
- cargo audit green is dependency-health evidence only;
- `pq_encap_failed` defensive branch documentation does not claim executable
  coverage;
- `pq_decap_failed` test evidence remains bounded to that marker.

## Post-merge checks

After merge, verify:

- READY remains NA-0438 until optional closeout.
- D-0863 exists on main.
- D-0864 remains absent until optional closeout.
- public-safety is green on the evidence merge commit.
- no qwork post-merge command was run by Codex.
