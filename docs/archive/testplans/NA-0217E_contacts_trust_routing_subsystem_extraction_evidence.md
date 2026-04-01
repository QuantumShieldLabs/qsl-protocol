Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-01

# NA-0217E Contacts / Trust / Routing Subsystem Extraction Evidence

Goals: G4, G5

## Scope

- Repo: `qsl-protocol`
- Lane: `NA-0217E`
- Posture: implementation already merged; this closeout is governance-only and introduces no runtime changes
- Implementation PR: #635
- Implementation branch head before merge: `43a475e706dd`
- Implementation merge SHA: `b227872f9f59`

## Authority Proof

- refreshed `qsl-protocol` checkout `HEAD`, `mirror/main`, bare mirror `main`, and `origin/main` all resolved to `b227872f9f59`
- refreshed merged main contains `DECISIONS.md` `D-0353`, the `TRACEABILITY.md` `NA-0217E implementation/evidence` entry, `qsl/qsl-client/qsc/src/contacts/mod.rs`, `qsl/qsl-client/qsc/tests/relay_auth_header.rs`, and `qsl/qsl-client/qsc/tests/trust_remediation_ux_na0178.rs`
- refreshed live queue still showed `READY_COUNT=1` with `NA-0217E` as the sole live `READY` item before this closeout
- refreshed sibling-repo proof stayed read-only and truthful: `qsl-server READY=0`, `qsl-attachments READY=0`

## LOC Proof

- `qsl/qsl-client/qsc/src/main.rs` before merge parent: `20,149` LOC
- `qsl/qsl-client/qsc/src/main.rs` after merged extraction: `18,445` LOC
- `qsl/qsl-client/qsc/src/contacts/mod.rs`: `1,713` LOC

## Practical Moved-Helper Inventory

- route-token foundation:
  - `route_token_hash8`
  - route-token normalize / generate helpers
  - self-inbox and peer route-token resolution helpers
- trust onboarding and remediation helpers:
  - `TrustOnboardingMode`
  - account-backed trust-mode load helper
  - trust-remediation step / hint helpers
  - blocked-send / blocked-peer emission helpers
- contact/device normalization and routing-target resolution:
  - device-state normalization helpers
  - primary-device lookup/update helpers
  - channel/device label helpers
  - `SendRoutingTarget`
  - peer-device / send-target resolution helpers
- contact/device store mutation helpers and contact command wrappers:
  - add/update device helpers
  - trust / block / unblock / primary-set helpers
  - contact/device list/show/status wrappers
- intentionally left for `NA-0217F`:
  - timeline persistence
  - delivery-state transitions
  - confirmation apply helpers
  - attachment-linked delivery bookkeeping

## No-Drift Proof Surface

### primary-device routing

- `cargo test --test relay_auth_header`

### trust remediation / blocked-peer behavior

- `cargo test --test trust_remediation_ux_na0178`

### route-token normalization

- `cargo test --test relay_auth_header`

### qsc-desktop-sensitive contacts / store proof

- `cargo test --test desktop_gui_contract_na0215b`
- no `qsc-desktop` path changed while contact/device-store ownership moved from `main.rs` into `contacts`

### honest-delivery canary

- `cargo test --test message_state_model`

### identity canary

- `cargo test --test identity_foundation_contract_na0217d`

### protocol_state canary

- `cargo test --test protocol_state_contract_na0217c`

### fs_store canary

- `cargo test --test fs_store_contract_na0217b`

### marker / output canary

- `cargo test --test output_marker_contract_na0217a`
- the implementation lane actually ran the marker/output canary because the extracted routing/status-adjacent seam still participates in user-visible blocked-send and routing surfaces

## Clean-Main / CI Nuance Summary

- CodeQL alert `#95` was resolved in-scope without changing the human-facing trust-remediation hint text
- the relay-auth regression proves an explicitly selected primary device still drives the normalized relay header contract
- the trust-remediation regression proves the verify-vs-trusted CLI hint remained unchanged

## Exact Commands / Tests Run For The Merged Implementation Lane

- `cargo fmt --check`
- `cargo build`
- `cargo clippy -- -D warnings`
- `cargo test --test relay_auth_header`
- `cargo test --test message_state_model`
- `cargo test --test desktop_gui_contract_na0215b`
- `cargo test --test identity_foundation_contract_na0217d`
- `cargo test --test protocol_state_contract_na0217c`
- `cargo test --test fs_store_contract_na0217b`
- `cargo test --test output_marker_contract_na0217a`
- `cargo test --test trust_remediation_ux_na0178`
- local `tools/goal_lint.py` revalidation via a synthesized event payload using the actual local base/head SHAs

## Why NA-0217E Stayed Narrower Than NA-0217F

- `NA-0217E` moved only the contacts / trust / routing subsystem
- timeline persistence, delivery-state transitions, confirmation apply helpers, and attachment-linked delivery bookkeeping stayed in `qsl/qsl-client/qsc/src/main.rs`
- `DOC-QSC-011` orders timeline / delivery state as the direct successor because that delivery ownership depends on the now-extracted contacts/routing device-target decisions

## Closeout Note

- This closeout PR is governance-only.
- No runtime, workflow, protocol, relay, attachment, or qsc-desktop paths change in this closeout.
