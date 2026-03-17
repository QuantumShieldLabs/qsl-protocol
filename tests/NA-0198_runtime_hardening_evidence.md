# NA-0198 Runtime Hardening Evidence

## Scope

- qsl-protocol runtime/test hardening only.
- No canonical semantic changes.
- No qsl-server changes.
- No qsl-attachments changes in this item.

## Load-Bearing Confidence Gaps Before Work

- Ordinary local confidence still depended on fragmented proof surfaces:
  - attachment integration was green in targeted tests,
  - route-token migration regressions were green,
  - but Suite-2 runtime equivalence was still mostly indirect.
- `cargo test -p qsc --locked` had not yet been restored as an ordinary-confidence signal:
  - `NA-0197C` evidence showed a `300s` timeout cap that did not complete the full suite,
  - the known bottleneck was `aws_file_medium_boundary_na0192a`.
- The full-suite surface also still contained at least one hidden parallelism hazard:
  - a first `NA-0198` full-suite attempt completed far enough to reveal a missing-path failure in `ratchet_durability_na0155`,
  - targeted rerun passed, proving a brittle shared-temp-root test harness issue rather than a runtime semantic failure.

## Hardening Changes Implemented

### 1) Direct Suite-2 Runtime Equivalence

- Added `qsl/qsl-client/qsc/tests/suite2_runtime_equivalence_na0198.rs`.
- The test uses the live `qsc` binary, real relay traffic, seeded fallback session setup, and persisted encrypted `.qsv` session blobs.
- It proves:
  - emitted `Envelope.payload` equals canonical `send_wire_canon(...)` output,
  - decrypted persisted qsc session state matches canonical post-send and post-receive refimpl state,
  - the roundtrip holds in both directions for the same peer.

### 2) Full-Suite Parallelism Hardening

- Updated these tests to stop deleting the shared `qsc-test-tmp` root in `safe_test_root()`:
  - `qsl/qsl-client/qsc/tests/ratchet_durability_na0155.rs`
  - `qsl/qsl-client/qsc/tests/outbox_abort.rs`
  - `qsl/qsl-client/qsc/tests/relay_drop_no_mutation.rs`
  - `qsl/qsl-client/qsc/tests/relay_dup_no_mutation.rs`
- Each now ensures the shared root exists, but only per-test case directories are removed/recreated.
- This removed the cross-test interference that could make the full parallel suite fail nondeterministically on missing paths.

## Deterministic Coverage Added / Refreshed

- Added:
  - `qsl/qsl-client/qsc/tests/suite2_runtime_equivalence_na0198.rs`
- Refreshed / tightened:
  - `qsl/qsl-client/qsc/tests/ratchet_durability_na0155.rs`
  - `qsl/qsl-client/qsc/tests/outbox_abort.rs`
  - `qsl/qsl-client/qsc/tests/relay_drop_no_mutation.rs`
  - `qsl/qsl-client/qsc/tests/relay_dup_no_mutation.rs`

## Required Local Gates

- `cargo fmt --all -- --check`
  - PASS
- `cargo clippy --workspace --all-targets -- -D warnings`
  - PASS
- `cargo build --workspace --locked`
  - PASS

## Required Regression Proof

- `cargo test -p qsc --locked --test attachment_streaming_na0197c -- --nocapture`
  - PASS (`4 passed`, `1 ignored`) in `35.46s real`
- `cargo test -p qsc --locked --test relay_auth_header -- --nocapture`
  - PASS (`3 passed`) in `2.82s real`
- `cargo test -p qsc --locked --test tui_relay_config -- --nocapture`
  - PASS (`16 passed`) in `106.02s real`
- `cargo test -p qsc --locked --test route_header_migration_docs_na0195a -- --nocapture`
  - PASS (`2 passed`) in `0.52s real`
- `cargo test -p qsc --locked --test qsp_qse_onwire -- --nocapture`
  - PASS (`5 passed`) in `3.12s real`
- `cargo test -p qsc --locked --test handshake_mvp -- --nocapture`
  - PASS (`7 passed`) in `7.17s real`
- `cargo test -p qsc --locked --test identity_secret_at_rest -- --nocapture`
  - PASS (`4 passed`) in `2.31s real`

## Additional Hardening Proof

- `cargo test -p qsc --locked --test suite2_runtime_equivalence_na0198 -- --nocapture`
  - PASS (`1 passed`) in `2.21s test time`
- Initial full-suite attempt:
  - `cargo test -p qsc --locked`
  - Completed far enough to expose `ratchet_durability_na0155` missing-path interference under parallel load
  - NOT counted as a pass
- Affected targeted reruns after the temp-root fix:
  - `ratchet_durability_na0155`
  - `outbox_abort`
  - `relay_drop_no_mutation`
  - `relay_dup_no_mutation`
  - all PASS
- Final full-suite rerun:
  - `cargo test -p qsc --locked`
  - PASS in `1164.16s real`

## Known Long-Running Surface Treatment

- `aws_file_medium_boundary_na0192a` remains slow, but it is no longer the load-bearing confidence blocker.
- Investigation showed that widening the receive pull window produced cross-platform inbox-parse failures on macOS, so that change was rejected rather than merged.
- The truthful hardening move was:
  - keep the medium-boundary semantics unchanged,
  - add direct Suite-2 runtime equivalence proof,
  - eliminate the shared-temp-root full-suite flake,
  - and restore successful ordinary `cargo test -p qsc --locked` completion locally.

## Attachment / Legacy Coexistence Proof

- Attachment path remains explicit and truthful:
  - `attachment_streaming_na0197c` remained green
  - `accepted_by_relay` / `peer_confirmed` semantics stayed separated
- Legacy coexistence remained explicit:
  - `attachment_path_coexists_with_legacy_below_threshold` stayed green
  - legacy file delivery semantics remained green in `file_transfer_mvp`

## Large-File Local Proof

- `QSC_ATTACHMENT_LARGE_BYTES=67108864 cargo test -p qsc --locked --test attachment_streaming_na0197c attachment_large_local_roundtrip_proof -- --ignored --nocapture`
  - PASS in `80.22s real`
- `QSC_ATTACHMENT_LARGE_BYTES=104857600 cargo test -p qsc --locked --test attachment_streaming_na0197c attachment_large_local_roundtrip_proof -- --ignored --nocapture`
  - PASS in `116.63s real`

## qsl-attachments Correction Decision

- No qsl-attachments runtime corrections were needed.
- The live `qsl-attachments` runtime remained contract-faithful for the hardened qsc/runtime confidence surface.

## Source of Truth

- `qsl/qsl-client/qsc/tests/suite2_runtime_equivalence_na0198.rs`
- `qsl/qsl-client/qsc/tests/ratchet_durability_na0155.rs`
- `qsl/qsl-client/qsc/tests/outbox_abort.rs`
- `qsl/qsl-client/qsc/tests/relay_drop_no_mutation.rs`
- `qsl/qsl-client/qsc/tests/relay_dup_no_mutation.rs`
- `qsl/qsl-client/qsc/tests/attachment_streaming_na0197c.rs`
- `tests/NA-0197C_attachment_client_evidence.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
