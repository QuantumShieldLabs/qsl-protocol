# NA-0438 Closeout and NA-0439 Restoration Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-07

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0438 closes only after PR #1145 is merged and post-merge
public-safety is green, and that the selected NA-0439 adversarial implementation
successor is restored as the sole READY item without implementing NA-0439.

## Protected invariants

- PR #1145 remains merged at `ea522b4024dd`.
- NA-0438 is DONE.
- NA-0439 is READY.
- Exactly one READY item remains.
- `pq_encap_failed` remains documented as a defensive branch with no executable
  coverage claim.
- NA-0436 `pq_decap_failed` test evidence remains bounded to that marker.
- NA-0439 is not implemented by this closeout.
- No backup or restore is run.
- No public-claim expansion is introduced.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0438_closeout_restore_na0439_testplan.md`

## Forbidden scope

Forbidden changed paths include runtime, crypto, dependency, Cargo manifest,
lockfile, workflow, script implementation except future NA-0439, executable
test, fuzz target, vector, qsl-server, qsl-attachments, qshield runtime,
website, public docs, README, START_HERE, qwork/qstart/qresume/qshell,
qsl-backup, backup status, backup plan, rollback, and backup tree paths.

Forbidden commands include qwork, qstart, qresume, sudo, backup, restore,
`cargo update`, `cargo generate-lockfile`, and dependency remediation commands.

## Merge / public-safety checks

Confirm:

- PR #1145 state is MERGED.
- PR #1145 merge commit begins with `ea522b4024dd`.
- `origin/main` equals or descends from `ea522b4024dd`.
- post-merge public-safety completed success on `ea522b4024dd`.
- qsc-adversarial-smoke completed success on `ea522b4024dd`.

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
- NA-0438 DONE.
- READY NA-0439.
- NA-0437 DONE.
- NA-0436 DONE.
- NA-0435 DONE.
- NA-0434 BLOCKED.
- NA-0429 BLOCKED.
- D-0863 exists once.
- D-0864 exists once after this closeout.
- D-0865 absent.
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
- `tests/NA-0438_closeout_restore_na0439_testplan.md`

## Link, leak, claim, and PR-body checks

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
- claim scan has no affirmative public overclaim;
- PR body includes `Goals:`, `Impact:`, `No-regression:`, and
  `Tests/Vectors:`.

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

- READY is NA-0439.
- NA-0438 is DONE.
- D-0864 exists on main.
- public-safety is green on the closeout merge commit.
- no qwork post-merge command was run by Codex.
