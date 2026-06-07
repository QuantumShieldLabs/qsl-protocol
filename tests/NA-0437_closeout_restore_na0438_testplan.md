# NA-0437 Closeout and NA-0438 Restoration Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-07

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0437 closes only after PR #1143 is merged and post-merge
public-safety is green, and that NA-0438 is restored as the sole READY successor
without implementing NA-0438.

## Protected invariants

- PR #1143 remains merged at `64d3488513d2`.
- `pq_encap_failed` remains documented as a defensive branch with no executable
  coverage claim.
- NA-0436 `pq_decap_failed` test evidence remains bounded to that marker.
- NA-0438 is authorization-only.
- Exactly one READY item remains.
- No backup or restore is run.
- No public-claim expansion is introduced.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0437_closeout_restore_na0438_testplan.md`

## Forbidden scope

Forbidden changed paths include runtime, crypto, dependency, Cargo manifest,
lockfile, workflow, script, executable test, fuzz target, vector, qsl-server,
qsl-attachments, qshield runtime, website, public docs, README, START_HERE,
qwork/qstart/qresume/qshell, qsl-backup, backup status, backup plan, rollback,
and `/backup/qsl` paths.

Forbidden commands include qwork, qstart, qresume, sudo, backup, restore,
`cargo update`, `cargo generate-lockfile`, and dependency remediation commands.

## Merge / public-safety checks

Confirm:

- PR #1143 state is MERGED.
- PR #1143 merge commit begins with `64d3488513d2`.
- `origin/main` equals or descends from `64d3488513d2`.
- post-merge public-safety completed success on `64d3488513d2`.

If public-safety is missing, red, ambiguous, or still in progress after bounded
polling, stop before closeout patching.

## Queue and decision checks

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Required after patching:

- READY_COUNT 1.
- NA-0437 DONE.
- READY NA-0438.
- NA-0436 DONE.
- NA-0435 DONE.
- NA-0434 BLOCKED.
- NA-0429 BLOCKED.
- D-0861 exists once.
- D-0862 exists once after this closeout.
- D-0863 absent.
- duplicate decision count zero.

## Scope guard

Run after patching and before PR creation:

```bash
git diff --name-only origin/main
git ls-files --others --exclude-standard
```

The combined changed-path set must be exactly:

- `DECISIONS.md`
- `NEXT_ACTIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0437_closeout_restore_na0438_testplan.md`

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
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
rg -n "pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz/Cargo.lock || true
cargo fmt --check
```

Required:

- the NA-0436 `pq_decap_failed` test still passes and emits its markers;
- root cargo audit passes;
- nested qsc fuzz lock audit passes;
- `rustls-webpki` is `v0.103.13` or newer safe version;
- root pqcrypto inverse-tree probes are absent or explicitly explained as
  expected zero-match proofs;
- nested qsc fuzz lock pqcrypto residual scan returns zero matches;
- formatting check passes.

## Public claim boundary

Confirm:

- no production readiness claim is introduced;
- no public-internet readiness claim is introduced;
- no external-review completion claim is introduced;
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

- READY is NA-0438.
- NA-0437 is DONE.
- D-0862 exists on main.
- public-safety is green on the closeout merge commit.
- no qwork post-merge command was run by Codex.
