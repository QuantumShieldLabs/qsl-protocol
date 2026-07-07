Goals: G4 (primary), supports G1, G2, G3, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# NA-0610 — Malformed Envelope/Descriptor/Object Adversarial Negative-Test Expansion

## Summary

NA-0610 implements DOC-G5-005 §9 rank 1 under directive QSL-DIR-2026-07-07-547
(D547): it converts the NA-0608 "corrupted descriptor / not separately exercised"
hedge into explicit, deterministic negative tests at the attachment descriptor and
confirm parse boundary, each asserting fail-closed reject. The change is test-only
(a co-located `#[cfg(test)]` module in `qsl/qsl-client/qsc/src/adversarial/payload.rs`);
it changes no production parse behavior — it observes and pins it.

Result classification: `MALFORMED_ATTACHMENT_DESCRIPTOR_NEGATIVES_PINNED`.

No malformed input was accepted; all reject fail-closed. This is not a
public-readiness, production-readiness, security-completion, crypto-complete,
attachment-complete, vulnerability-free, or bug-free claim.

## Required Markers

- NA0610_D1217_CONSUMED_OK
- NA0610_D1218_CONSUMED_OK
- NA0610_FRESH_QWORK_PROOF_OK
- NA0610_CURRENT_MAIN_HEALTH_OK
- NA0610_D1219_ABSENT_BEFORE_IMPLEMENTATION_OK
- NA0610_DESCRIPTOR_NEGATIVES_ADDED_OK
- NA0610_SANITY_TEMPLATE_PARSES_OK
- NA0610_ALL_MALFORMED_REJECT_FAIL_CLOSED_OK
- NA0610_NO_PRODUCTION_SEMANTIC_CHANGE_OK
- NA0610_NO_NEW_DEPENDENCY_OK
- NA0610_TEST_ONLY_SINGLE_SOURCE_FILE_OK
- NA0610_REGRESSION_SUITES_PASS_OK
- NA0610_NA0608_DESCRIPTOR_HEDGE_CLOSED_OK
- NA0610_PRIVATE_MATERIAL_SCAN_OK
- NA0610_RESULT_CLASSIFICATION_SELECTED_OK
- NA0610_SUCCESSOR_NA0611_SELECTED_OK

## Qwork, Queue, And Main Gates

Fresh operator-run qwork proof for lane NA-0610 from `2026-07-07T02:21:29Z`
(regenerated via the WF-0004 drop-first workflow against the current READY lane
NA-0610, not the completed NA-0609) verified before mutation; HEAD == origin/main ==
main == `981425f61ac1`; worktree clean; READY_COUNT 1 with READY NA-0610; D-1217
once, D-1218 once, D-1219 absent.

## Inheritance

D-1217 (NA-0609 implementation) and D-1218 (NA-0609 closeout) consumed once each and
Accepted. This lane implements the DOC-G5-005 §9 rank-1 backlog item.

## The Change

In `qsl/qsl-client/qsc/src/adversarial/payload.rs`, the existing `#[cfg(test)] mod
tests` was extended with attachment-descriptor and confirm negative tests. The
attachment descriptor struct uses `#[serde(deny_unknown_fields)]` with required
fields, and `parse_attachment_descriptor_payload` additionally filters on version
and type. The new tests feed malformed inputs and assert each yields None (reject):

- a well-formed template parses (sanity, so the negatives are meaningful);
- empty input; non-JSON garbage; truncated JSON; wrong version; wrong type; a
  missing required field — all reject;
- attachment confirm: empty, non-JSON, missing required fields, and wrong
  discriminant — all reject.

No production parse/accept/reject/wire/crypto code was changed; no new dependency was
added; the only file changed is `payload.rs` (a test module).

## Validation

- `cargo fmt --check`: OK. `cargo build`: OK.
- New + existing payload negative tests: 10 passed (`payload::tests`), including the
  sanity template-parses case.
- Regression suites pass unchanged: `adversarial_properties` (8),
  `attachments_contract_na0217h` (1), `receive_no_mutation` (1, the malformed-envelope
  no-mutation-on-reject test — so the envelope-plane hedge was already covered); all
  qsc binary unit tests 32 passed.
- `cargo metadata --locked`: OK; Cargo.toml/Cargo.lock unchanged (no new dependency).

## Scope Note And Follow-Ups

This lane covers the attachment descriptor and confirm parse boundary (the specific
NA-0608 hedge) within the declared `src/adversarial/**` scope. Additional adjacent
negatives remain future work outside this scope: `attachment_decode_enc_ctx`
(REJECT_ATT_DESC_ENC_CTX, in `src/attachments/mod.rs`) and a repeatable corrupted
attachment-object test (REJECT_ATT_CIPHERTEXT_PRECHECK, via the mock service
harness). These are recorded for a future negative-test lane.

## Boundary And Claim

Implementation mutates only `qsl/qsl-client/qsc/src/adversarial/payload.rs` (test
module), `docs/governance/evidence/NA-0610_..._harness.md`,
`tests/NA-0610_..._testplan.md`, `DECISIONS.md`, `TRACEABILITY.md`, and
`docs/ops/ROLLING_OPERATIONS_JOURNAL.md`. No Cargo change; no other source; no
production semantic change; no `.github`/workflow; no canonical spec/input; no
`.claude`/hook; no qwork/qstart/qresume execution; no runtime/LAN action. No
endpoint, port, token, capability, key, seed, plaintext, ciphertext body, or raw
private material is published. No public-readiness, production-readiness,
security-completion, crypto-complete, attachment-complete, vulnerability-free, or
bug-free claim is made.
