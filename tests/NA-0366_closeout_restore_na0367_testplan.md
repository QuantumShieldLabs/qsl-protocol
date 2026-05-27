Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-27

# NA-0366 Closeout and NA-0367 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Close NA-0366 after the real off-host target/tool blocker-resolution evidence
merged, and restore exactly one READY successor:

`NA-0367 -- Metadata Runtime Off-Host Backup Target Access / Host Identity Prerequisite Plan`

The closeout must not implement NA-0367.

## Protected Invariants

- Exactly one READY item exists after the closeout patch.
- NA-0366 is DONE.
- D-0714 exists once.
- D-0715 exists once.
- D-0716 is absent before NA-0367 work begins.
- No runtime, protocol, crypto, service, dependency, workflow, website,
  README, START_HERE, docs/public, backup script/timer/fstab, restore target,
  off-host destination, repository, tool, key, passphrase, recovery envelope,
  deploy, rollback, backup, restore, or branch-protection setting is changed.
- No production-readiness, public-internet-readiness, external-review-complete,
  anonymity, metadata-free, untraceable, hidden-size, hidden-timing,
  hidden-traffic-shape, off-host-backup-complete, restore-drill-complete,
  real-restore-complete, real-key-custody-implemented,
  real-key-recovery-implemented, or disaster-recovery-complete claim is added.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0366_closeout_restore_na0367_testplan.md`

## Forbidden Scope

- `README.md`
- `START_HERE.md`
- `docs/public/**`
- `.github/**`
- `Cargo.toml`
- `Cargo.lock`
- `qsp/**`
- `qsc/**`
- `qsl/**`
- `qsl-client/**`
- `apps/**`
- `tools/**`
- `inputs/**`
- `formal/**`
- `scripts/**`
- `qsc-desktop/**`
- `qsl-server/**`
- `qsl-attachments/**`
- `website/**`
- runtime/protocol/crypto/demo/service implementation paths
- branch-protection/public-safety configuration
- backup scripts/timers/fstab/local system paths

## Queue Requirements

- Before closeout: `READY_COUNT 1`, READY `NA-0366`.
- After patch: `READY_COUNT 1`, READY `NA-0367`.
- `NA-0366` must be DONE.
- `NA-0367` must be restored with the exact selected title from D-0714.

## Decision Requirements

D-0715 must state:

- NA-0366 delivered real off-host target/tool blocker-resolution evidence.
- NA-0367 is selected based on NA-0366 evidence.
- no NA-0367 implementation is authorized by closeout.
- metadata reduction remains bounded and not overclaimed.

## Traceability Requirements

`TRACEABILITY.md` must link:

- D-0715;
- qsl-protocol PR #994;
- NA-0366 blocker-resolution evidence;
- selected NA-0367 successor;
- no-real-operation and public-claim boundaries.

## Backup-Impact Requirements

Closeout does not require a backup-plan update because changed paths remain
qsl-protocol governance/testplan/journal paths under `/srv/qbuild/work`.

Future real target/tool implementation, real key custody/recovery, repository
initialization, real isolated restore, monitoring, local-ops workflow support,
source-list changes, backup scripts/timers/fstab/services, backup, restore,
deploy, rollback, public-claim mutation, and durable evidence outside current
scope remain backup-plan gated.

## Required Local Checks

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
  with the closeout allowed paths.
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- goal-lint through PR CI or a local event-equivalent run.
- classifier proof for changed paths.

## CI Expectations

- qsl-protocol branch protection must continue requiring `public-safety`.
- The closeout PR must merge only after required checks are green.
- Post-merge public-safety must complete success on the closeout merge commit.

## Successor Handoff

NA-0367 starts from the target-access / host-identity prerequisite plan. It must
not treat NA-0366 blocker-resolution evidence as real off-host setup, real
repository initialization, off-host backup completion, real key custody, real
key recovery, real restore execution, or complete disaster recovery proof.
