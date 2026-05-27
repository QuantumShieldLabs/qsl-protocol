Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-27

# NA-0368 Closeout and NA-0369 Restoration Testplan

## Objective

Validate the governance-only closeout for NA-0368 after the prerequisite PR
merged and restore exactly one READY successor:

`NA-0369 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Action Packet`

This closeout must not implement NA-0369.

## Protected Invariants

- Exactly one READY item exists after closeout: NA-0369.
- NA-0368 is DONE.
- D-0718 and D-0719 each exist once.
- D-0720 is absent.
- qsl-protocol is the only mutable repository.
- qsl-server and qsl-attachments are not mutated.
- qshield runtime is not mutated.
- qsc/qsp/protocol/crypto/key-schedule implementation is not mutated.
- Cargo dependencies and workflows are unchanged.
- README, START_HERE, website, and docs/public are unchanged.
- Backup scripts, timers, fstab, service units, source lists, restore paths,
  rollback paths, remote destinations, key material, passphrase material,
  credential material, recovery-envelope content, and local backup
  configuration are unchanged.
- No remote/off-host connection, host-key scan, `known_hosts` mutation,
  repository init, tool installation, backup, restore, deploy, rollback, real
  restore target creation/mount/copy, real key generation, key upload,
  passphrase collection, credential handling, private-key inspection,
  recovery-envelope content creation, or secret handling occurs.
- No claim states or implies production readiness, public-internet readiness,
  external-review completion, anonymity, metadata-free behavior,
  untraceability, hidden attachment size, hidden timing metadata, hidden
  traffic shape, hidden all metadata, complete off-host backup, complete
  disaster recovery, real restore completion, host identity verification,
  target configuration, real key custody implementation, or real key recovery
  implementation.

## Allowed Scope

Allowed files:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0368_closeout_restore_na0369_testplan.md`

## Forbidden Scope

Forbidden changes include README, START_HERE, docs/public, `.github`, Cargo
manifests or lockfiles, qsp, qsc, qsl, qsl-client, apps, tools, inputs, formal,
scripts, qsc-desktop, qsl-server, qsl-attachments, website, runtime/protocol
implementation paths, branch-protection/public-safety configuration, backup
scripts/timers/fstab/local system paths, and branch deletion.

## Required Closeout Checks

Validation must confirm:

- qsl-protocol PR #998 merged from validated head `d313e9f8119d`.
- qsl-protocol PR #998 merge commit `f245f1aaa912` exists on `origin/main`.
- post-merge qsl-protocol `public-safety` completed success on
  `f245f1aaa912`.
- D-0718 records the accepted NA-0368 prerequisite plan.
- D-0719 records closeout and NA-0369 restoration.
- NA-0368 is DONE.
- NA-0369 is READY.
- D-0720 is absent before NA-0369 work begins.
- No NA-0369 implementation is included.

## Required Local Checks

Run:

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main` with exact allowed paths
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- local goal-lint using the closeout PR body
- classifier proof for the changed path set

## CI Expectations

The closeout PR must merge only after required qsl-protocol checks complete
normally and `public-safety` is green. No admin bypass, direct push, squash,
rebase, or delete-branch flag is allowed.

## Successor Handoff

After merge and post-merge `public-safety` success, NA-0369 is the sole READY
item. NA-0369 may produce an operator action packet in a future directive, but
this closeout does not authorize target setup, remote connection, host-key
scan, credential handling, secret handling, repository init, tool installation,
backup, restore, deploy, rollback, or public-claim expansion.
