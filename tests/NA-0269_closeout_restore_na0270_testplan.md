Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-12
Replaces: n/a
Superseded-By: n/a

# NA-0269 Closeout and NA-0270 Restoration Test Plan

## Objective

Close NA-0269 after the production-boundary hardening plan merged and
post-merge public-safety completed green, then restore NA-0270 as the sole
READY successor for qsl-server read-only code audit and test-harness design.

## Protected Invariants

- Exactly one READY item exists after closeout.
- NA-0269 is DONE.
- NA-0270 is READY.
- D-0508 remains the NA-0269 production-boundary planning decision.
- D-0509 records the closeout and successor restoration.
- `public-safety` remains required and green.
- No production readiness claim is made or implied.
- Current qsl-server/qsl-attachments hardening remains future gated work.
- No qsl-server or qsl-attachments implementation change is made.
- No protocol, wire, crypto, auth, or state-machine change is made.
- No website, external website, workflow, script, Cargo, dependency,
  branch-protection, or public-safety configuration change is made.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0269_closeout_restore_na0270_testplan.md`

## Forbidden Scope

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
- external website source
- runtime, protocol, crypto, demo, service code
- branch-protection settings
- public-safety/check configuration

## Required Proof

Before patch:

- PR #791 is merged.
- PR #791 merge commit is on `origin/main`.
- D-0508 exists once.
- D-0509 is absent.
- Queue helper reports `READY_COUNT 1`, READY `NA-0269`.
- Post-merge main `public-safety` is success.

After patch:

- Queue helper reports `READY_COUNT 1`, READY `NA-0270`.
- NA-0269 block is `DONE`.
- D-0508 exists once.
- D-0509 exists once.
- Scope guard reports only allowed closeout paths.
- Link-check reports `TOTAL_MISSING 0`.
- Added-line leak-scan reports zero high-confidence secret findings.
- Goal-lint passes.
- Cargo audit, rustls-webpki reverse tree, qsc `send_commit`, and formal/model
  checks remain green.

## Successor Boundary

NA-0270 is read-only qsl-server audit and test-harness design. It must not
change qsl-server implementation, qsl-attachments implementation, protocol or
crypto state machines, website content, branch protection, public-safety,
workflows, scripts, Cargo dependencies, or any production service behavior.

## CI Expectations

- Required protected checks pass normally.
- CodeQL neutral is acceptable only under the repository's existing acceptance
  basis.
- `public-safety` remains required on `main`.
- This closeout is docs/governance-only, so NA-0262A cost-control is expected
  to skip heavy full-suite waits/jobs for the closeout main push.
