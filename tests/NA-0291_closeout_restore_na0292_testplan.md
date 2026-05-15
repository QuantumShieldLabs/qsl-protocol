Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-15
Replaces: n/a
Superseded-By: n/a

# NA-0291 Closeout and NA-0292 Restoration Test Plan

## Objective

Close NA-0291 after the metadata phase-2 identifier/padding executable
harness PR merged and restore exactly one successor, NA-0292, for sanitized
errors and retention/purge design.

## Protected Invariants

- Exactly one READY item exists after closeout.
- NA-0291 is DONE.
- NA-0292 is READY.
- D-0555 exists once.
- D-0556 exists once.
- D-0557 is absent.
- No duplicate decision IDs exist.
- NA-0292 is not implemented by this closeout.
- Metadata phase-2 remains evidence-bound and incomplete beyond proven lanes.
- No runtime identifier rotation or runtime default padding claim is added.
- No anonymity claim.
- No metadata-free claim.
- No untraceable claim.
- No external-review-complete claim.
- No production-readiness or public-internet-readiness claim.
- No qsl-protocol runtime, protocol, crypto, qsp protocol-core, qsc/qsl
  runtime, service implementation, qsc-desktop, website/external repo, README,
  START_HERE, workflow, script, Cargo, dependency, branch-protection, or
  public-safety configuration change.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0291_closeout_restore_na0292_testplan.md`

## Required Queue Proof

After the closeout patch:

```text
READY_COUNT 1
READY NA-0292 Metadata Phase-2 Sanitized Errors and Retention/Purge Design
NA-0291 DONE Metadata Phase-2 Identifier Rotation and Padding Defaults Executable Harness
```

## Required Decision Proof

After the closeout patch:

```text
D-0555 1
D-0556 1
D-0557 0
DUPLICATE_COUNT 0
```

## Scope Guard

The PR diff must include only allowed docs/governance paths.

Forbidden paths include:

- `README.md`
- `START_HERE.md`
- `.github/**`
- `scripts/**`
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
- `qsc-desktop/**`
- `qsl-server/**`
- `qsl-attachments/**`
- `website/**`
- runtime/protocol/crypto/demo/service implementation paths.

## Validation Commands

```bash
git diff --check origin/main...HEAD
python3 scripts/ci/qsl_evidence_helper.py queue --select NA-0292 --select NA-0291
python3 scripts/ci/qsl_evidence_helper.py decisions --select D-0555 --select D-0556 --select D-0557
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allow NEXT_ACTIONS.md \
  --allow DECISIONS.md \
  --allow TRACEABILITY.md \
  --allow docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allow tests/NA-0291_closeout_restore_na0292_testplan.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
```

Goal-lint may be run with a synthetic pull-request event payload if local
execution requires `GITHUB_EVENT_PATH`.

## CI Expectations

- Required checks attach and pass normally.
- `public-safety` remains required and green.
- Docs/governance-only cost control may skip heavy qsc suites according to
  NA-0262A.
- No admin bypass, direct push to main, squash, rebase, or branch deletion is
  used.

## Successor Handoff

The next directive should target:

`NA-0292 - Metadata Phase-2 Sanitized Errors and Retention/Purge Design`

That directive must design sanitized-error expansion and retention/purge
metadata policy, include an executable-harness plan, and preserve the no
anonymity, no metadata-free, no untraceable, no external-review-complete, and
no production-readiness boundaries. This closeout does not implement NA-0292.
