# NA-0371 Closeout / NA-0372 Restoration Testplan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-27

## Objective

Validate that NA-0371 is closed only after the no-secret operator response
collection request merged and post-merge public-safety completed success, and
that the exact selected NA-0372 successor is restored without implementing
NA-0372.

## Protected Invariants

- Exactly one READY item exists after closeout.
- NA-0371 is DONE.
- NA-0372 is READY.
- D-0725 exists once.
- D-0726 is absent.
- NA-0372 is not implemented by the closeout.
- The closeout introduces no target setup, remote connection, host-key scan,
  known_hosts mutation, credential handling, secret handling, repository init,
  tool installation, backup, restore, deploy, rollback, real restore target
  creation/mount/copy, real key generation, key upload, passphrase collection,
  private-key inspection, recovery-envelope content creation, or backup
  script/timer/fstab mutation.
- The closeout introduces no qsl-server, qsl-attachments, qshield runtime,
  qsc/qsp/protocol/crypto/key-schedule, dependency, workflow, website,
  README, START_HERE, docs/public, branch-protection, or public-safety
  configuration change.
- The closeout introduces no production-readiness, public-internet-readiness,
  external-review-complete, anonymity, metadata-free, untraceable,
  hidden-size, hidden-timing, hidden-traffic-shape, target-configured,
  host-identity-verified, off-host-backup-complete, real-restore-complete,
  real-key-custody-implemented, real-key-recovery-implemented, or
  disaster-recovery-complete claim.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0371_closeout_restore_na0372_testplan.md`

## Forbidden Scope

All runtime, protocol, crypto, service, qsl-server, qsl-attachments,
qshield, qsc/qsp implementation, dependency, workflow, website/public-doc,
README, START_HERE, input-artifact, script, backup, restore, deploy,
rollback, key, credential, secret, host-key, known_hosts, repository-init,
tool-install, branch-protection, and public-safety configuration paths are
forbidden.

## Queue Requirements

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Required:

- `READY_COUNT 1`
- `READY NA-0372`
- `NA-0371 DONE`
- latest decision is D-0725
- duplicate decision count is zero
- D-0726 is absent

## Evidence Requirements

- `NEXT_ACTIONS.md` records qsl-protocol PR #1004, head `044ed1492a3`,
  merge `9fa290eaa46d`, collection request result
  `COLLECTION_REQUEST_CREATED`, and the selected NA-0372 successor.
- `DECISIONS.md` includes D-0725 with Goals G1, G2, G3, G4, G5.
- `TRACEABILITY.md` links D-0725, the closeout testplan, PR #1004, and the
  selected NA-0372 successor.
- The rolling journal records the resumed post-merge public-safety wait and
  closeout restoration.

## Scope Guard Requirements

Run scope guard with the exact allowed closeout paths:

```bash
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed NEXT_ACTIONS.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0371_closeout_restore_na0372_testplan.md
```

Required:

- forbidden changed path count is zero.

## Link / Leak / Overclaim Requirements

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

Required:

- no missing local markdown links.
- no secret findings in added lines.
- high-risk phrase matches, if any, are negated, prohibited, future-gated,
  or explicit boundary statements.

## Required Local Checks

Run:

```bash
cargo audit --deny warnings
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 scripts/ci/qsl_evidence_helper.py goal-lint
```

Goal-lint may be run with a synthesized pull-request event payload before PR
creation.

## CI Expectations

- The closeout PR must include a standalone Goals line:
  `Goals: G1, G2, G3, G4, G5`.
- Required checks, including public-safety, must complete green before merge.
- Merge must use normal merge with `--match-head-commit`.
- Do not use admin bypass, direct push, squash, rebase, or delete-branch flags.
- After merge, post-merge public-safety must complete success.

## Successor Handoff

The selected successor is:

`NA-0372 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Response Intake After Collection Request`

The closeout must not implement NA-0372. NA-0372 must begin by intaking any
deliberate no-secret operator response created from the NA-0371 collection
request, or recording that the response remains absent.
