Goals: G1, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-11
Replaces: n/a
Superseded-By: n/a

# NA-0265 Closeout and NA-0266 Restoration Testplan

## Objective

Close NA-0265 after its clean-host reviewer reproduction bundle merged and
restore exactly one READY successor, NA-0266, for bounded demo soak and
repeated-run stability.

## Protected Invariants

- Exactly one READY item exists after closeout.
- NA-0265 is DONE.
- NA-0266 is READY.
- D-0500 remains the NA-0265 implementation/evidence decision.
- D-0501 records the closeout and successor restoration.
- `public-safety` remains required and green.
- Demo remains non-production.
- No production-ready demo, relay, desktop, qsl-server, or qsl-attachments
  claim is inferred.
- No protocol/crypto state-machine change is made.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0265_closeout_restore_na0266_testplan.md`

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
- runtime, protocol, crypto, demo, service code
- branch-protection or public-safety configuration

## Required Proof

Before patch:

- PR #783 is merged.
- PR #783 merge commit is on `origin/main`.
- D-0500 exists once.
- D-0501 is absent.
- Queue helper reports `READY_COUNT 1`, READY `NA-0265`.
- Post-merge main `public-safety` is success.

After patch:

- Queue helper reports `READY_COUNT 1`, READY `NA-0266`.
- NA-0265 block is `DONE`.
- D-0501 exists once.
- D-0502 is absent.
- Scope guard reports only allowed closeout paths.
- Link-check reports `TOTAL_MISSING 0`.
- Leak-scan reports no high-confidence secret findings.
- Goal-lint passes.
- Cargo audit, rustls-webpki reverse tree, and qsc `send_commit` remain green.

## Successor Boundary

NA-0266 may implement or invoke bounded demo repeated-run/soak proof. It must
not make protocol/crypto state-machine changes, qsl-server/qsl-attachments
production changes, website changes, branch-protection changes, public-safety
configuration changes, Cargo dependency changes, production relay/service
claims, or production-hardening claims.
