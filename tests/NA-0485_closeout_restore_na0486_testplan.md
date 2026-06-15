# NA-0485 Closeout / NA-0486 Restoration Test Plan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-06-15
Replaces:
Superseded-By:

## Scope

This closeout test plan verifies that NA-0485 is marked DONE and the selected
NA-0486 helper/API design authorization successor is restored as the sole READY
item without implementing NA-0486.

## Required Checks

- Queue proof: `READY_COUNT 1`, READY NA-0486, and NA-0485 DONE.
- Decision proof: D-0959 exists once, D-0960 is absent, and duplicate decision
  count is zero.
- Scope guard: changed paths are limited to `NEXT_ACTIONS.md`, `DECISIONS.md`,
  `TRACEABILITY.md`, `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`, and this
  testplan.
- Diff hygiene: `git diff --check` passes.
- Link integrity: repository markdown local-link check passes.
- Leak scan: added closeout content does not introduce sensitive endpoints,
  tokens, auth headers, private keys, operator data, user data, live service
  data, or long secret-like dumps.
- Claim boundary: added closeout content introduces no public-readiness claim,
  no production-readiness claim, no public-internet-readiness claim, no
  external-review-complete claim, no crypto-complete claim, no fuzz-complete
  claim, no vector-complete claim, no KEM-complete claim, no signature-complete
  claim, no identity-complete claim, no transcript-complete claim, no
  qsc/refimpl-equivalence-complete claim, no provider-boundary-complete claim,
  no provider-RNG-complete claim, no formal-proof-complete claim, no
  replay-proof claim, no downgrade-proof claim, no side-channel-free claim, no
  vulnerability-free claim, no bug-free claim, and no perfect-crypto claim.
- PR metadata: PR body includes `Goals: G1, G2, G3, G4, G5` and the required
  Impact, No-regression, and Tests/Vectors fields.
- Post-merge proof: public-safety on the closeout merge commit completes
  success before Director handoff.

## Expected Result

NA-0485 is closed, NA-0486 is the sole READY item, and the repository records a
governance-only transition. This closeout does not mutate fuzz targets, fuzz
corpus, qsc runtime/source, qsc executable tests, dependencies, Cargo manifests,
lockfiles, workflows, scripts, vectors/inputs, formal models, refimpl
source/tests, services, public docs, website, backup files, qsl-backup, qwork,
qstart, qresume, or qshell.
