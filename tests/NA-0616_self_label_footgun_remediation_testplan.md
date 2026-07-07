Goals: G3 (primary), supports G1, G2, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# NA-0616 — Self-Label Footgun Remediation Test Plan

## Scope

Validation for the NA-0616 fail-closed self-identity selection fix (ENG-0001) under
directive QSL-DIR-2026-07-07-553 (D553). Client-side identity gating + a CLI default; no
handshake/crypto/wire/state-machine semantic change; no dependency/workflow change.

## Required Markers

- NA0616_D1225_CONSUMED_OK
- NA0616_D1226_CONSUMED_OK
- NA0616_FRESH_STARTUP_PROOF_OK
- NA0616_D1227_ABSENT_BEFORE_IMPLEMENTATION_OK
- NA0616_DESIGN_LOCK_ONE_SELF_PER_DIR_OK
- NA0616_GATE_IN_AUTOCREATE_BRANCH_OK
- NA0616_ERRORCODE_IDENTITY_SELF_AMBIGUOUS_OK
- NA0616_HANDSHAKE_AS_DEFAULT_SELF_OK
- NA0616_DIVERGENT_LABEL_FAILS_CLOSED_END_TO_END_OK
- NA0616_NO_MUTATION_ON_REJECT_OK
- NA0616_FIRSTRUN_AND_ROTATE_PRESERVED_OK
- NA0616_NO_CRYPTO_SEMANTIC_CHANGE_OK
- NA0616_REGRESSION_SUITES_PASS_OK
- NA0616_ENG0001_RESOLVED_OK
- NA0616_PRIVATE_MATERIAL_SCAN_OK

## Validation Plan (class-only)

1. Verify fresh operator startup proof (lane NA-0616) and main health; D-1225/D-1226
   once, D-1227 absent; sole READY NA-0616.
2. Design-lock: one-self-per-dir; all auto-create sites are the handshake paths; explicit
   rotate bypasses the gate; no legitimate same-dir multi-identity-via-handshake flow.
3. Unit/integration (`na_0616_self_label_footgun`): rotate creates canonical self; a
   second divergent `--as` fails closed end-to-end with `identity_self_ambiguous` and
   mints nothing; the consistent label is not gated; explicit rotate of a second label is
   allowed.
4. Regression: identity/handshake/adversarial suites pass; all qsc unit tests pass.
5. `cargo fmt/build/clippy/metadata --locked` green; Cargo unchanged; scope guard
   confirms only identity/model/cmd/tests changed and no crypto-semantic change.

## Result

`SELF_LABEL_FOOTGUN_FAIL_CLOSED`. Evidence:
`docs/governance/evidence/NA-0616_self_label_footgun_remediation_harness.md`.
