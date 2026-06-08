# NA-0439 Closeout and NA-0440 Restoration Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-07

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0439 closes only after PR #1147 is merged, the authorized
macOS rerun and public-safety recovery are green, and the selected NA-0440
formal/model alignment authorization successor is restored as the sole READY
item without implementing NA-0440.

## Protected invariants

- PR #1147 remains merged at `07f4c0ab79f5`.
- NA-0439 is DONE.
- NA-0440 is READY.
- Exactly one READY item remains.
- `pq_encap_failed` remains documented as a defensive branch with no executable
  coverage claim.
- NA-0436 `pq_decap_failed` test evidence remains bounded to that marker.
- The NA-0439 provider-error test remains integrated into qsc adversarial
  script before cargo-fuzz phases.
- No backup or restore is run.
- No public-claim expansion is introduced.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0439_closeout_restore_na0440_testplan.md`

## Forbidden scope

Forbidden changed paths include runtime, crypto, dependency, Cargo manifest,
lockfile, workflow, script implementation, executable test, fuzz target, vector,
qsl-server, qsl-attachments, qshield runtime, website, public docs, README,
START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status, backup plan,
rollback, and backup tree paths.

Forbidden commands include qwork, qstart, qresume, sudo, backup, restore,
generated operator scripts, `cargo update`, `cargo generate-lockfile`, and
dependency remediation commands.

## PR #1147 merge/public-safety checks

Confirm:

- PR #1147 state is MERGED.
- PR #1147 merge commit begins with `07f4c0ab79f5`.
- `origin/main` equals or descends from `07f4c0ab79f5`.
- post-merge public-safety completed success on `07f4c0ab79f5`.
- qsc-adversarial-smoke completed success on `07f4c0ab79f5`.
- qsc-adversarial-miri completed success on `07f4c0ab79f5`.

If public-safety is missing, red, ambiguous, or still in progress after bounded
polling, stop before closeout patching.

## macOS rerun / public-safety recovery checks

Confirm:

- macOS rerun job `79997092974` completed success.
- the original public-safety failure was stale aggregate state from the earlier
  macOS check failure.
- exactly one minimum aggregate rerun was issued.
- public-safety aggregate rerun job `80002570830` completed success.
- no repeated aggregate rerun was issued.

## qsc-adversarial success check

Confirm qsc-adversarial-smoke completed success on the PR #1147 merge commit.

## qsc-adversarial-miri success check

Confirm qsc-adversarial-miri completed success on the PR #1147 merge commit, or
record the exact accepted check shape if not attached.

## adversarial script provider-error test integration check

Run:

```bash
rg -n "NA0439_QSC_PROVIDER_ERROR_NO_MUTATION_ADVERSARIAL_STEP|handshake_provider_error_no_mutation" scripts/ci/qsc_adversarial.sh
rg -n "handshake_provider_error_no_mutation|cargo fuzz|cargo-fuzz|fuzz" scripts/ci/qsc_adversarial.sh
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Required:

- marker is present.
- provider-error command is present.
- provider-error command appears before cargo-fuzz target execution.
- shell syntax checks pass.

## `pq_encap_failed` caveat preservation check

Run the provider-error test and confirm:

- `NA0436_PQ_ENCAP_FAILED_CAVEAT_PRESERVED_OK` appears.
- no executable coverage claim is made for `pq_encap_failed`.

## root cargo audit green check

Run:

```bash
cargo audit --deny warnings
```

Required: command exits 0. Treat output as dependency-health evidence only.

## nested fuzz lock audit green check

Run:

```bash
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
```

Required: command exits 0. Treat output as dependency-health evidence only.

## NA-0439 DONE / NA-0440 READY check

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Required after patching:

- READY_COUNT 1.
- NA-0439 DONE.
- READY NA-0440.
- NA-0438 DONE.
- NA-0437 DONE.
- NA-0436 DONE.
- NA-0435 DONE.
- NA-0434 BLOCKED.
- NA-0429 BLOCKED.
- D-0865 exists once.
- D-0866 exists once after this closeout.
- D-0867 absent.
- duplicate decision count zero.

## qsl-protocol closeout scope guard

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
- `tests/NA-0439_closeout_restore_na0440_testplan.md`

## no runtime/dependency/workflow/test/vector mutation

Confirm the closeout diff contains no runtime, crypto, dependency, Cargo,
lockfile, workflow, script implementation, executable test, fuzz target, vector,
service, website, README, START_HERE, qwork/qstart/qresume/qshell, backup,
qsl-backup, status, plan, rollback, or backup tree path.

## no public overclaim

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

- READY is NA-0440.
- NA-0439 is DONE.
- NA-0434 is BLOCKED.
- D-0866 exists on main.
- public-safety is green on the closeout merge commit.
- no qwork post-merge command was run by Codex.
