Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-01

# NA-0399 Closeout and NA-0400 Restoration Testplan

Goals: G1, G2, G3, G4, G5

Directive: QSL-DIR-2026-06-01-219

## Objective

Validate that NA-0399 is closed after the backup / restore / key custody
external guidance mapping PR merges, and that exactly one successor is restored:
`NA-0400 -- QSL External Review / Disclosure / Public Claim Readiness Plan`.

This closeout must not implement NA-0400.

## Protected Invariants

- READY_COUNT is exactly one.
- READY is NA-0400 after closeout.
- NA-0399 is DONE.
- D-0780 exists once.
- D-0781 exists once.
- D-0782 is absent.
- No runtime, service, protocol, crypto, dependency, Cargo, workflow,
  public-doc, website, backup-script, backup-timer, fstab, source-list,
  qsl-server, qsl-attachments, qshield runtime, qstart/qresume, response
  archive, local tool, off-host target, key, credential, passphrase, private
  key, recovery envelope, known_hosts, or secret-bearing path is changed.
- No real backup, real restore, remote/off-host setup, host-key scan,
  repository init, key operation, credential handling, recovery-envelope
  creation, deploy, rollback, or restore target creation/mount/copy occurs.
- Same-host continuity is not complete disaster recovery.
- No off-host-backup-complete, restore-proven, restore-drill-complete,
  real-restore-complete, key-custody-implemented, key-recovery-implemented,
  recovery-envelope-ready, target-configured, host-identity-verified,
  production-ready, public-internet-ready, bug-free, perfect-crypto, or
  external-review-complete claim is introduced.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0399_closeout_restore_na0400_testplan.md`

## Forbidden Scope

Forbidden changes include `README.md`, `START_HERE.md`, `docs/public/**`,
`.github/**`, `Cargo.toml`, `Cargo.lock`, `qsp/**`, `qsc/**`, `qsl/**`,
`qsl-client/**`, `apps/**`, `tools/**`, `inputs/**`, `formal/**`,
`scripts/**`, `qsc-desktop/**`, `qsl-server/**`, `qsl-attachments/**`,
`website/**`, runtime/protocol/crypto/demo/service implementation paths,
branch-protection/public-safety configuration, backup scripts/timers/fstab,
local system paths, branch deletion, `/home/victor/work/qsl/codex/**`, and
`/srv/qbuild/tools/**`.

## Required Evidence

Verify the closeout records:

- Packet Y PR #1061.
- Packet Y validated head `8dac9c1528ac`.
- Packet Y merge `43c90b60b34c`.
- Packet Y post-merge public-safety success.
- D-0780 as the NA-0399 mapping decision.
- D-0781 as the closeout decision.
- Selected successor exactly:
  `NA-0400 -- QSL External Review / Disclosure / Public Claim Readiness Plan`.
- No NA-0400 implementation authorization.

## Queue Requirements

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Required result:

- READY_COUNT 1.
- READY NA-0400.
- NA-0399 DONE.
- latest decision D-0781.
- duplicate decision count zero.

## Scope Guard Requirements

Run scope guard against the exact allowed closeout paths. The changed path set
must contain no other paths.

## Link / Leak / Overclaim Requirements

Run:

- markdown link check.
- leak scan over added lines and changed files.
- high-risk phrase scan over added lines.

All high-risk phrase matches must be negated, prohibited, future-gated, or
claim-boundary wording. Unsafe match count must be zero.

## Validation Requirements

Run the same representative local validation profile used for governance
closeouts:

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- queue/decisions.
- scope guard.
- link-check.
- leak-scan.
- classifier proof.
- goal-lint / PR body preflight.

## Public-Safety Expectations

Public-safety must be required and green before the closeout PR merges and
after the merge.

## Successor Handoff

The closeout restores NA-0400 only. It does not implement NA-0400 and does not
authorize public docs, website changes, production claims, public-internet
readiness claims, external-review-complete claims, public technical paper work,
or public-claim expansion.
