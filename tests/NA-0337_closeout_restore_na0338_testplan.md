Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-22

# NA-0337 Closeout and NA-0338 Restoration Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0337 is marked DONE after the qshield embedded relay/demo
padding bucket expansion implementation PR merged and post-merge
`public-safety` completed success, and restore exactly one READY successor:

`NA-0338 -- Metadata Runtime Attachment Size-Class Authorization Plan`

The closeout must not implement NA-0338.

## Protected Invariants

- Exactly one READY item after closeout: NA-0338.
- NA-0337 is DONE.
- D-0656 and D-0657 each exist once.
- D-0658 is absent.
- No NA-0338 implementation is included.
- No attachment-size padding, qsl-server production padding,
  qsl-attachments production object-size padding, transport padding expansion,
  runtime timing mitigation, qshield implementation, qsl-server,
  qsl-attachments, qsc/qsp/protocol/crypto key schedule, dependency,
  workflow, branch-protection, public-safety, qsc-desktop, website, README,
  START_HERE, docs/public, formal, input, tools/refimpl, app runtime, or
  service implementation change is included.
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

- PR #936 merged with normal merge.
- PR #936 validated head: `ca20c304ab12`.
- PR #936 merge: `a061addb8b5e`.
- Post-merge `public-safety` completed success on `a061addb8b5e`.
- NA-0337 implementation selected NA-0338 exactly.
- Queue after patch reports READY_COUNT `1` and READY `NA-0338`.
- Decision helper reports latest decision D-0657 and duplicate count zero.

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

After merge and post-merge `public-safety` success, NA-0338 is the sole READY
successor. It remains an attachment size-class authorization plan and is not
implemented by this closeout.
