Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-22

# NA-0336 Closeout and NA-0337 Restoration Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0336 is marked DONE after the authorization/design PR merged
and post-merge `public-safety` completed success, and restore exactly one READY
successor:

`NA-0337 -- Metadata Runtime qshield Demo Padding Bucket Expansion Implementation Harness`

The closeout must not implement NA-0337.

## Protected Invariants

- Exactly one READY item after closeout: NA-0337.
- NA-0336 is DONE.
- D-0654 and D-0655 each exist once.
- D-0656 is absent.
- No NA-0337 implementation is included.
- No padding bucket expansion, transport padding expansion, runtime mitigation,
  qshield implementation, qsl-server, qsl-attachments, qsc/qsp/protocol/crypto
  key schedule, dependency, workflow, branch-protection, public-safety,
  qsc-desktop, website, README, START_HERE, docs/public, formal, input,
  tools/refimpl, app runtime, or service implementation change is included.
- No unsupported production, public-internet, external-review, anonymity,
  metadata-free, untraceable, timing-hidden, or traffic-shape-hidden claim is
  introduced.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- this testplan

## Forbidden Scope

- README.md
- START_HERE.md
- docs/public/**
- .github/**
- Cargo.toml
- Cargo.lock
- qsp/**
- qsc/**
- qsl/**
- qsl-client/**
- apps/**
- tools/**
- inputs/**
- formal/**
- scripts/**
- qsc-desktop/**
- qsl-server/**
- qsl-attachments/**
- website/**
- external website repo
- runtime/protocol/crypto/demo/service implementation paths
- branch-protection/public-safety configuration
- branch deletion

## Required Evidence

- PR #934 merged with normal merge.
- PR #934 validated head: `86a683379f22`.
- PR #934 merge: `074232f134f9`.
- Post-merge `public-safety` completed success on `074232f134f9`.
- NA-0336 authorization selected NA-0337 exactly.
- Queue after patch reports READY_COUNT `1` and READY `NA-0337`.
- Decision helper reports latest decision D-0655 and duplicate count zero.

## Required Checks

Run or record an exact blocker:

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
  with the exact closeout allowed paths
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- local overclaim scan over added lines
- classifier proof for the changed path set
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- goal-lint through PR body preflight or CI

## CI Expectations

The closeout PR may merge only if required checks, including `public-safety`,
pass normally. No admin bypass, direct push, squash, rebase, branch deletion,
delete-branch flag, or protection mutation is allowed.

## Successor Handoff

After merge and post-merge `public-safety` success, NA-0337 is the sole READY
successor. It remains a future implementation harness and is not implemented by
this closeout.
