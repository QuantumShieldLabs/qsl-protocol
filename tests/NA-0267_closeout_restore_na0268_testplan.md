Goals: G1, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-11
Replaces: n/a
Superseded-By: n/a

# NA-0267 Closeout and NA-0268 Restoration Testplan

## Objective

Close NA-0267 after the advisories fetch resilience implementation merged and
post-merge public-safety completed green, then restore NA-0268 as the sole
READY successor for cross-host/private-network soak expansion.

## Protected Invariants

- Exactly one READY item exists after closeout.
- NA-0267 is DONE.
- NA-0268 is READY.
- D-0504 remains the NA-0267 implementation/evidence decision.
- D-0505 records the closeout and successor restoration.
- `public-safety` remains required and green.
- Real vulnerabilities and cargo-audit warnings fail closed.
- Transient external advisory database fetch failures are bounded and clearly
  logged, and cannot create a green audit result without a later successful
  cargo-audit run.
- No branch-protection weakening is made or implied.
- No Cargo dependency change is made.
- No protocol/runtime/crypto/demo implementation, qsl-server, qsl-attachments,
  qsc-desktop, website, external website, or production-hardening change is
  made in this closeout.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0267_closeout_restore_na0268_testplan.md`

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

- PR #787 is merged.
- PR #787 merge commit is on `origin/main`.
- D-0504 exists once.
- D-0505 is absent.
- Queue helper reports `READY_COUNT 1`, READY `NA-0267`.
- Post-merge main `public-safety` is success.

After patch:

- Queue helper reports `READY_COUNT 1`, READY `NA-0268`.
- NA-0267 block is `DONE`.
- D-0505 exists once.
- D-0506 is absent.
- Scope guard reports only allowed closeout paths.
- Link-check reports `TOTAL_MISSING 0`.
- Leak-scan reports no high-confidence secret findings.
- Goal-lint passes.
- Cargo audit, rustls-webpki reverse tree, and qsc `send_commit` remain green.

## Successor Boundary

NA-0268 may implement or invoke bounded cross-host/private-network demo soak
proof. It must not target the public internet, mutate firewall/router/Tailscale
admin state, leak tokens/secrets/plaintext, allow state bleed, accept panics as
pass, claim production readiness, change protocol/crypto state machines, make
qsl-server/qsl-attachments production changes, change website content, weaken
public-safety, change branch protection, or change Cargo dependencies.

## CI Expectations

- Required protected checks pass normally.
- CodeQL neutral is acceptable only under the repository's existing acceptance
  basis.
- `public-safety` remains required on `main`.
- This closeout is docs/governance-only, so NA-0262A cost-control is expected
  to skip heavy full-suite waits/jobs for the closeout main push.
