Goals: G1 (primary), supports G2, G3, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# NA-0618 — Suite-2 Counter Overflow Hard-Stop Test Plan

## Scope

Validation for the NA-0618 fail-closed `u32::MAX` hard-stop on the Suite-2 symmetric message
counters (ENG-0013) under directive QSL-DIR-2026-07-07-555 (D555). Refimpl `suite2` source +
tests + one canonical-spec reject-code line. No key-schedule/KDF/AEAD/nonce/wire-format/
descriptor change; no qsc-client/qsl-attachments/qsl-server change; no dependency/workflow
change.

## Required Markers

- NA0618_D1230_CONSUMED_OK
- NA0618_D1231_CONSUMED_OK
- NA0618_FRESH_STARTUP_PROOF_OK
- NA0618_D1232_ABSENT_BEFORE_IMPLEMENTATION_OK
- NA0618_DESIGN_LOCK_THREE_SITES_NO_MUTATION_UNREACHABLE_OK
- NA0618_CHECKED_COUNTER_INC_HELPER_AT_ALL_THREE_SITES_OK
- NA0618_SEND_WIRE_REJECTS_AT_NS_MAX_NO_MUTATION_OK
- NA0618_RECV_GUARDS_USE_HELPER_NO_MUTATION_OK
- NA0618_REJECT_S2_COUNTER_OVERFLOW_REGISTERED_IN_DOC_CAN_003_OK
- NA0618_LOCAL_REASON_CODE_NOT_WIRE_TRANSMITTED_OK
- NA0618_NO_KEYSCHEDULE_KDF_AEAD_NONCE_WIRE_CHANGE_OK
- NA0618_FULL_REFIMPL_SUITE_GREEN_NO_REGRESSION_OK
- NA0618_FMT_CLIPPY_METADATA_LOCKED_GREEN_CARGO_UNCHANGED_OK
- NA0618_ENG0013_RESOLVED_OK
- NA0618_PRIVATE_MATERIAL_SCAN_OK

## Validation Plan (class-only)

1. Verify fresh operator startup proof (lane NA-0618) and main health; D-1230/D-1231 consumed
   once each and Accepted; D-1232 absent before implementation.
2. Design-lock: the three `saturating_add` sites (`send_wire` ns, `recv_nonboundary_ooo` nr,
   `recv_boundary_in_order` nr); the recv reject paths return unmutated `state: st`; the
   receive-side overflow is unreachable via a compliant sender.
3. Change: `checked_counter_inc` used at all three sites in place of `saturating_add`;
   `send_wire` fails closed before deriving key material; recv paths reject with
   `REJECT_S2_COUNTER_OVERFLOW` and no mutation; the reject code is registered in
   DOC-CAN-003 §10 and is a local reason code (not wire-transmitted).
4. Tests: `checked_counter_inc_boundary_and_normal` (0→Ok(1), MAX-1→Ok(MAX), MAX→Err) and
   `send_wire_rejects_counter_overflow_at_ns_max_and_no_mutation` (reject + deterministic +
   no mutation + no AEAD use). Both pass.
5. Regression: full `quantumshield_refimpl` suite green (70 lib + integration); suite2 vectors
   unaffected (small counters).
6. Build gates: `cargo fmt --check`, `cargo clippy -p quantumshield_refimpl --all-targets
   -D warnings`, `cargo build --locked`, `cargo metadata --locked`; Cargo.toml/lock unchanged.
7. Private-material scan on added lines.

## Result

`SUITE2_COUNTER_OVERFLOW_FAIL_CLOSED`. ENG-0013 resolved. No key-schedule/KDF/AEAD/nonce/wire
change; no dependency/workflow change. Does NOT implement ENG-0012 (DH ratchet / PQ reseed).
