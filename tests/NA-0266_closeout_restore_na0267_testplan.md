Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-11
Replaces: n/a
Superseded-By: n/a

# NA-0266 Closeout and NA-0267 Restoration Testplan

## Objective

Close NA-0266 after the bounded demo soak and repeated-run stability matrix
merged and post-merge public-safety completed green, then restore NA-0267 as
the sole READY successor for advisories fetch resilience and external
dependency failure handling.

## Protected Invariants

- Exactly one READY item exists after closeout.
- NA-0266 is DONE.
- NA-0267 is READY.
- D-0502 remains the NA-0266 implementation/evidence decision.
- D-0503 records the closeout and successor restoration.
- `public-safety` remains required and green.
- Real vulnerabilities and advisories fail closed.
- Transient external advisory database fetch failures are classified clearly.
- No branch-protection weakening is made or implied.
- No protocol/runtime/crypto/demo implementation, qsl-server, qsl-attachments,
  qsc-desktop, website, external website, Cargo dependency, or production
  hardening change is made.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0266_closeout_restore_na0267_testplan.md`

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

- PR #785 is merged.
- PR #785 merge commit is on `origin/main`.
- D-0502 exists once.
- D-0503 is absent.
- Queue helper reports `READY_COUNT 1`, READY `NA-0266`.
- Post-merge main `public-safety` is success.

After patch:

- Queue helper reports `READY_COUNT 1`, READY `NA-0267`.
- NA-0266 block is `DONE`.
- D-0503 exists once.
- D-0504 is absent.
- Scope guard reports only allowed closeout paths.
- Link-check reports `TOTAL_MISSING 0`.
- Leak-scan reports no high-confidence secret findings.
- Goal-lint passes.
- Cargo audit, rustls-webpki reverse tree, and qsc `send_commit` remain green.

## Successor Boundary

NA-0267 may implement CI/helper resilience and fixture tests that distinguish
real advisory failures from transient advisory database fetch failures. It must
not mask real vulnerabilities, weaken cargo audit enforcement, weaken
public-safety, mutate branch protection, change Cargo dependencies, or make
protocol/runtime/crypto/demo/service, qsl-server, qsl-attachments,
qsc-desktop, website, external website, or production-hardening changes.

## CI Expectations

- Required protected checks pass normally.
- CodeQL neutral is acceptable only under the repository's existing acceptance
  basis.
- `public-safety` remains required on `main`.
- This closeout is docs/governance-only, so NA-0262A cost-control is expected
  to skip heavy full-suite waits/jobs for the closeout main push.
