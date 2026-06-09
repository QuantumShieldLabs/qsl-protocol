Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-09

# NA-0452 Closeout Restore NA-0453 Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Verify NA-0452 closeout after PR #1173 merged, D306/D307 macOS recovery
completed, public-safety returned to green, and NA-0453 was restored as the
sole READY successor.

## Protected invariants

- NA-0452 is DONE only after PR #1173 is merged and public-safety is success.
- NA-0453 is the sole READY item.
- NA-0434 and NA-0429 remain BLOCKED.
- D-0892 exists once and D-0893 remains absent until future NA-0453 work.
- Duplicate decision count remains zero.
- Production semantics remain unchanged without `qsc_rng_failure_test_seam`.
- TUI account verification seed and provider-dependent RNG remain deferred.
- No public claim expands.

## Allowed scope

- `NEXT_ACTIONS.md`.
- `DECISIONS.md`.
- `TRACEABILITY.md`.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.
- `tests/NA-0452_closeout_restore_na0453_testplan.md`.

## Forbidden scope

- No source, runtime, crypto, Cargo, lockfile, workflow, executable test
  source, fuzz target, vector, formal model, qsl-server, qsl-attachments,
  qshield runtime, qshield-cli, website, public docs, README, START_HERE,
  qwork/qstart/qresume/qshell, qsl-backup, backup status file, backup plan
  file, rollback subtree, or backup tree mutation.
- No backup or restore execution.
- No public-readiness claim.
- No production-readiness claim.
- No public-internet-readiness claim.
- No external-review-complete claim.
- No crypto-complete claim.
- No RNG-failure-complete claim.
- No secret-material-complete claim.
- No side-channel-free claim.
- No vulnerability-free claim.
- No bug-free claim.
- No perfect-crypto claim.
- No metadata-free claim.
- No anonymity claim.
- No untraceable claim.
- No off-host-backup claim.
- No disaster-recovery claim.
- No restore-proof claim.
- No backup-complete claim.

## PR #1173 merge/public-safety checks

- Verify `gh pr view 1173 --repo QuantumShieldLabs/qsl-protocol --json number,state,mergedAt,mergeCommit,title,url,statusCheckRollup`.
- Required result: PR #1173 is MERGED with merge commit beginning
  `50b89b1e8e32`.
- Verify merge-commit check-runs through REST polling only.
- Required result: public-safety is completed success after D308 recovery.
- Required result: qsc adversarial smoke is success or accepted by repo
  policy.

## D306 macOS failure recovery check

- Verify the D306 response exists and records post-merge public-safety failure
  due to `macos-qsc-full-serial`.
- Verify the failure is not treated as a deterministic source defect unless
  terminal logs prove one.
- Required result: D306 remains failure evidence, not remediation authority.

## D307 macOS rerun evidence check

- Verify the D307 response exists and records the single allowed failed macOS
  rerun.
- Verify original failed macOS job `80397644192` and rerun job `80416944384`
  are tied to run `27227247523`.
- Required result: no additional macOS rerun is issued by D308.

## D308 evidence-based long-step wait policy check

- Verify D308 treats an attached in-progress `macos-qsc-full-serial` job inside
  `Test qsc full serial suite (locked)` as normal progress unless GitHub
  reports a concrete fault signal.
- Verify D308 does not stop solely because REST step metadata is unchanged.
- Required result: rerun job `80416944384` completed success.

## D308 final public-safety proof check

- Verify prerequisites were green before public-safety aggregate recovery:
  qsc-linux-full-suite, macos-qsc-full-serial, qsc-adversarial-smoke, and
  qsc-adversarial-miri when attached.
- Verify stale public-safety aggregate was rerun exactly once as job
  `80432383338`.
- Required result: public-safety is completed success on merge commit
  `50b89b1e8e32`.

## cfg residual-surface seam proof

- Run
  `RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_residual_surfaces -- --test-threads=1 --nocapture`.
- Required result: route/contact/attachment forced-failure markers are emitted.
- Required result: selected failures do not write partial route/contact/
  attachment state or outputs.

## production semantics unchanged proof

- Run
  `cargo test -p qsc --locked --test rng_failure_residual_surfaces -- --test-threads=1 --nocapture`.
- Required result: `NA0452_PRODUCTION_SEMANTICS_UNCHANGED_OK` is emitted.
- Required result: no-cfg builds keep production `OsRng` behavior and ignore
  the test-only selector.

## deferred TUI account verification seed check

- Verify `NA0452_TUI_ACCOUNT_VERIFICATION_SEED_DEFERRED_OK`.
- Required result: the TUI account verification seed is not labeled as
  implemented by NA-0452.

## provider RNG deferred check

- Verify `NA0452_PROVIDER_RNG_DEFERRED_OK`.
- Required result: provider-dependent qsc RNG and refimpl/provider RNG remain
  future authorization-plan scope.

## No RNG-failure-complete claim caveat check

- Verify `NA0452_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`.
- Required result: route/contact/attachment evidence is not represented as RNG-failure-complete proof.

## root cargo audit green check

- Run `cargo audit --deny warnings`.
- Required result: audit passes.
- Caveat: audit green is dependency-health evidence only.

## nested fuzz lock audit green check

- Run `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`.
- Required result: nested fuzz lock audit passes.
- Caveat: audit green is not vulnerability-free proof.

## NA-0452 DONE / NA-0453 READY check

- Run `python3 scripts/ci/qsl_evidence_helper.py queue`.
- Required result: `READY_COUNT 1`.
- Required result: NA-0452 is DONE.
- Required result: NA-0453 is READY.
- Required result: NA-0434 and NA-0429 remain BLOCKED.

## qsl-protocol closeout scope guard

- Run `git diff --name-only` before commit and compare against the allowed
  closeout path list.
- Required result: only the five allowed closeout paths are changed.

## no source/workflow/test/lockfile/vector/formal mutation

- Run a forbidden path guard over `git diff --name-only`.
- Required result: no source, workflow, executable test source outside this
  governance testplan, lockfile, vector, fuzz target, or formal model path is
  changed by closeout.

## no public overclaim

- Run added-line overclaim scan.
- Required result: no affirmative public-readiness claim.
- Required result: no affirmative production-readiness claim.
- Required result: no affirmative public-internet-readiness claim.
- Required result: no affirmative external-review-complete claim.
- Required result: no affirmative crypto-complete claim.
- Required result: no affirmative RNG-failure-complete claim.
- Required result: no affirmative secret-material-complete claim.
- Required result: no affirmative side-channel-free claim.
- Required result: no affirmative vulnerability-free claim.
- Required result: no affirmative bug-free claim.
- Required result: no affirmative perfect-crypto claim.
- Required result: no affirmative metadata-free claim.
- Required result: no affirmative anonymity claim.
- Required result: no affirmative untraceable claim.
- Required result: no affirmative off-host-backup claim.
- Required result: no affirmative disaster-recovery claim.
- Required result: no affirmative restore-proof claim.
- Required result: no affirmative backup-complete claim.
