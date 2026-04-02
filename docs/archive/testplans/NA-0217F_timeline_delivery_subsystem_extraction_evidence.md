Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-01

# NA-0217F Timeline / Delivery Subsystem Extraction Evidence

Goals: G4, G5

## Scope

- Repo: `qsl-protocol`
- Lane: `NA-0217F`
- Posture: implementation already merged; this closeout is governance-only and introduces no runtime changes
- Implementation PR: #637
- Implementation branch head before merge: `2beeab701740`
- Implementation merge SHA: `37b4c0f2af1c`

## Authority Proof

- refreshed `qsl-protocol` checkout `HEAD`, `mirror/main`, bare mirror `main`, and `origin/main` all resolved to `37b4c0f2af1c`
- refreshed merged main contains `DECISIONS.md` `D-0355`, the `TRACEABILITY.md` `NA-0217F implementation/evidence` entry, `qsl/qsl-client/qsc/src/timeline/mod.rs`, and `qsl/qsl-client/qsc/tests/timeline_delivery_contract_na0217f.rs`
- refreshed live queue still showed `READY_COUNT=1` with `NA-0217F` as the sole live `READY` item before this closeout
- refreshed sibling-repo proof stayed read-only and truthful: `qsl-server READY=0`, `qsl-attachments READY=0`

## LOC Proof

- `qsl/qsl-client/qsc/src/main.rs` before merge parent: `18,445` LOC
- `qsl/qsl-client/qsc/src/main.rs` after merged extraction: `17,775` LOC
- `qsl/qsl-client/qsc/src/timeline/mod.rs`: `712` LOC

## Practical Moved-Helper Inventory

- timeline persistence and list/show/clear ownership:
  - `TimelineEntry`
  - `timeline_ts_default`
  - timeline entry load/filter helpers
  - timeline list/show/clear wrappers
- delivery-state semantics and emitters:
  - `MessageState`
  - confirm-policy formatting / semantic helpers
  - entry-state derivation and emission helpers
  - transition validation helpers
- confirmation-apply and attachment-linked delivery bookkeeping:
  - file-transfer confirmation helpers
  - attachment-transfer confirmation helpers
  - outbound target-device helpers
  - attachment/file timeline-id linkage helpers
- intentionally left for `NA-0217G`:
  - relay inbox push/pull execution
  - outbox replay
  - auth-token resolution
  - local relay HTTP parsing
  - transport retry/send/receive orchestration

## No-Drift Proof Surface

### timeline persistence

- `cargo test --test timeline_store`

### delivery-state transitions

- `cargo test --test message_state_model`
- `cargo test --test attachment_streaming_na0197c`

### confirmation-apply semantics

- `cargo test --test timeline_delivery_contract_na0217f`

### honest-delivery semantics

- `cargo test --test message_state_model`
- `cargo test --test timeline_delivery_contract_na0217f`
- `accepted_by_relay` remained distinct from `peer_confirmed`

### no mutation on reject

- `cargo test --test timeline_delivery_contract_na0217f`

### device-target confirmation gating

- `cargo test --test timeline_delivery_contract_na0217f`
- the regression proves wrong-device receipt application leaves the outbound timeline entry unchanged until the targeted device confirms it

### qsc-desktop-sensitive delivery / store proof

- `cargo test --test desktop_gui_contract_na0215b`
- no `qsc-desktop` path changed while delivery/store ownership moved from `main.rs` into `timeline`

### contacts canary

- `cargo test --test relay_auth_header`

### identity canary

- `cargo test --test identity_foundation_contract_na0217d`

### protocol_state canary

- `cargo test --test protocol_state_contract_na0217c`

### fs_store canary

- `cargo test --test fs_store_contract_na0217b`

### marker / output canary

- `cargo test --test output_marker_contract_na0217a`
- the implementation lane actually ran the marker/output canary because delivery-state emission and status-adjacent marker surfaces remain user-visible after the timeline move

## Clean-Main / CI Nuance Summary

- the remaining `CONFIRM_POLICY` reference was repaired in-scope without widening beyond the timeline seam
- the timeline-delivery regression proves wrong-device receipt application leaves the outbound timeline entry unchanged until the targeted device confirms it
- the PR-resume lane recovered a non-material check-run evidence command mistake without changing repo state, and no additional code changes were required before merge

## Exact Commands / Tests Run For The Merged Implementation Lane

- `cargo fmt --check`
- `cargo build`
- `cargo clippy -- -D warnings`
- `cargo test --test timeline_store`
- `cargo test --test message_state_model`
- `cargo test --test attachment_streaming_na0197c`
- `cargo test --test desktop_gui_contract_na0215b`
- `cargo test --test relay_auth_header`
- `cargo test --test identity_foundation_contract_na0217d`
- `cargo test --test protocol_state_contract_na0217c`
- `cargo test --test fs_store_contract_na0217b`
- `cargo test --test timeline_delivery_contract_na0217f`
- `cargo test --test output_marker_contract_na0217a`
- local `tools/goal_lint.py` revalidation via a synthesized event payload using the actual local base/head SHAs

## Why NA-0217F Stayed Narrower Than NA-0217G

- `NA-0217F` moved only the timeline persistence / delivery-state seam
- relay inbox push/pull execution, auth-token resolution, local relay HTTP parsing, send/receive wrappers, retry policy, and outbox replay stayed in `qsl/qsl-client/qsc/src/main.rs`
- `DOC-QSC-011` orders the relay transport send/receive seam immediately after the timeline/delivery seam because transport still depends on, but should not co-own, the extracted delivery-state subsystem

## Closeout Note

- This closeout PR is governance-only.
- No runtime, workflow, protocol, relay, attachment, or qsc-desktop paths change in this closeout.
