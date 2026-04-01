Status: Archive
Owner: QSL governance
Last-Updated: 2026-03-31

# NA-0217D Identity Foundation Extraction Evidence

Goals: G4, G5

## Scope

- Repo: `qsl-protocol`
- Lane: `NA-0217D`
- Posture: implementation already merged; this closeout is governance-only and introduces no runtime changes
- Implementation PR: #633
- Implementation branch head before merge: `f3ae1981e2ee`
- Implementation merge SHA: `c663b14f3cf8`

## Authority Proof

- refreshed `qsl-protocol` checkout `HEAD`, `mirror/main`, bare mirror `main`, and `origin/main` all resolved to `c663b14f3cf8`
- refreshed merged main contains `DECISIONS.md` `D-0351`, the `TRACEABILITY.md` `NA-0217D implementation/evidence` entry, `qsl/qsl-client/qsc/src/identity/mod.rs`, and `qsl/qsl-client/qsc/tests/identity_foundation_contract_na0217d.rs`
- refreshed live queue still showed `READY_COUNT=1` with `NA-0217D` as the sole live `READY` item before this closeout
- refreshed sibling-repo proof stayed read-only and truthful: `qsl-server READY=0`, `qsl-attachments READY=0`

## LOC Proof

- `qsl/qsl-client/qsc/src/main.rs` before merge parent: `20,546` LOC
- `qsl/qsl-client/qsc/src/main.rs` after merged extraction: `20,149` LOC
- `qsl/qsl-client/qsc/src/identity/mod.rs`: `409` LOC

## Practical Moved-Helper Inventory

- identity record helpers:
  - `IdentityKeypair`
  - `IdentityPublicRecord`
  - public-record read/write path helpers
  - vault-backed secret-name and secret-store helpers
- fingerprint and display helpers:
  - `identity_fingerprint_from_pk`
  - `identity_self_fingerprint`
  - `format_verification_code_from_fingerprint`
  - `identity_marker_display`
  - `identity_pin_matches_seen`
- pin-read helpers:
  - `identity_read_pin`
  - `identity_read_sig_pin`
- legacy import / migration helpers:
  - `identity_migrate_legacy`
  - self-public / self-keypair load helpers
- intentionally left for `NA-0217E`:
  - contact/device stores
  - trust remediation
  - primary routing-target resolution
  - route-token normalization
  - contact-owned pin writes / trust mutations

## No-Drift Proof Surface

### vault-backed secret storage

- `cargo test --test identity_secret_at_rest`

### legacy import / migration no-mutation-on-failure

- `cargo test --test identity_secret_at_rest`

### pin / fingerprint stability

- `cargo test --test handshake_security_closure`
- `cargo test --test identity_foundation_contract_na0217d`

### qsc-desktop-sensitive identity / store proof

- `cargo test --test desktop_gui_contract_na0215b`
- no `qsc-desktop` path changed while identity/store ownership moved from `main.rs` into `identity`

### protocol_state canary

- `cargo test --test protocol_state_contract_na0217c`

### fs_store canary

- `cargo test --test fs_store_contract_na0217b`

### marker / output canary

- not claimed as part of the merged `NA-0217D` implementation evidence
- the implementation lane did not run a separate marker/output canary because the identity extraction and repaired regression did not touch marker/status emission paths

## Clean-Main Baseline Probe Summary

- clean main and the extracted branch both failed `identity show --as alice` with the same fail-closed missing-identity contract until the regression created the identity explicitly
- the failed `identity show` path created no identity/public/secret mutation in either clean main or the extracted branch
- the probe harness reported no unexpected residue after the failing `show`

## Exact Commands / Tests Run For The Merged Implementation Lane

- clean-main baseline harness against clean `qsc` and the extracted branch `qsc`
- `cargo fmt --check`
- `cargo build`
- `cargo clippy -- -D warnings`
- `cargo test --test identity_secret_at_rest`
- `cargo test --test handshake_security_closure`
- `cargo test --test desktop_gui_contract_na0215b`
- `cargo test --test protocol_state_contract_na0217c`
- `cargo test --test fs_store_contract_na0217b`
- `cargo test --test identity_foundation_contract_na0217d`
- local `tools/goal_lint.py` revalidation via a synthesized event payload using the actual local base/head SHAs

## Why NA-0217D Stayed Narrower Than NA-0217E

- `NA-0217D` moved only the identity public/secret record, fingerprint, pin-read, and legacy identity-migration foundation
- contacts/device stores, trust remediation, primary routing-target resolution, route-token normalization, and contact-owned pin/trust mutations stayed in `qsl/qsl-client/qsc/src/main.rs`
- `DOC-QSC-011` orders contacts / trust / routing as the direct successor once the identity foundation is merged

## Closeout Note

- This closeout PR is governance-only.
- No runtime, workflow, protocol, relay, attachment, or qsc-desktop paths change in this closeout.
