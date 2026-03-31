Status: Archive
Owner: QSL governance
Last-Updated: 2026-03-31

# NA-0217A Marker / Output Foundation Extraction Evidence

Goals: G4, G5

## Scope

- Repo: `qsl-protocol`
- Lane: `NA-0217A`
- Posture: implementation already merged; this closeout is governance-only and introduces no runtime changes
- Implementation PR: #627
- Implementation branch head before merge: `c5482c5edcbd`
- Implementation merge SHA: `3ecf3a4c44c8`

## Authority Proof

- refreshed `qsl-protocol` checkout `HEAD`, `mirror/main`, bare mirror `main`, and `origin/main` all resolved to `3ecf3a4c44c8`
- refreshed merged main contains `DECISIONS.md` `D-0345`, the `TRACEABILITY.md` `NA-0217A implementation/evidence` entry, `qsl/qsl-client/qsc/src/output/mod.rs`, and `qsl/qsl-client/qsc/tests/output_marker_contract_na0217a.rs`
- refreshed live queue still showed `READY_COUNT=1` with `NA-0217A` as the sole live `READY` item before this closeout
- refreshed sibling-repo proof stayed read-only and truthful: `qsl-server READY=0`, `qsl-attachments READY=0`

## LOC Proof

- `qsl/qsl-client/qsc/src/main.rs` before merge parent: `21,627` LOC
- `qsl/qsl-client/qsc/src/main.rs` after merged extraction: `21,338` LOC
- `qsl/qsl-client/qsc/src/output/mod.rs`: `318` LOC

## Practical Moved-Helper Inventory

- named marker emitters and marker entry points:
  - `emit_marker`
  - `emit_tui_named_marker`
  - doctor/status/send/delivery marker wrappers
  - `qsc_mark`
- marker routing and output policy:
  - `MarkerRouting`
  - `MarkerFormat`
  - `OutputPolicy`
  - `set_marker_routing`
- marker formatting and output redaction:
  - plain `QSC_MARK/1` formatter
  - JSONL formatter
  - `redact_value_for_output`
  - `redact_value_for_log`
  - `should_redact_value`
  - `redact_text_for_output`
- marker/log writing and terminal hygiene:
  - `log_marker`
  - `qsc_sanitize_terminal_text`
  - panic redaction hook / associated marker constants and static policy state

## No-Drift Proof Surface

### Plain marker contract

- `cargo test --test output_marker_contract_na0217a`
- explicit expected plain-marker bytes remained unchanged

### JSONL marker contract

- `cargo test --test output_marker_contract_na0217a`
- explicit JSON field/shape assertions remained unchanged

### Redaction behavior

- `cargo test --test output_marker_contract_na0217a`
- secret-like values remained redacted in both marker and log-sensitive assertions

### Routing behavior

- `cargo test --test tui_marker_routing`
- stdout versus in-app queued marker delivery remained unchanged

### Sidecar-contract-sensitive proof

- `cargo test --test desktop_gui_contract_na0215b`
- qsc-desktop profile, contact/device, and delivery/timeline parsing assumptions remained intact

## Exact Commands / Tests Run For The Merged Implementation Lane

- `cargo fmt --check`
- `cargo build`
- `cargo test --test output_marker_contract_na0217a`
- `cargo test --test tui_marker_routing`
- `cargo test --test tui_charter`
- `cargo test --test tui_product_polish_na0214a`
- `cargo test --test desktop_gui_contract_na0215b`
- `cargo test --test route_header_migration_docs_na0195a`
- `cargo test --test remote_soak_diag_mapping_na0168`
- `cargo clippy -- -D warnings`
- local `goal-lint` revalidation via synthesized event payload after adding the required governance companions

## Why NA-0217A Stayed Narrower Than NA-0217B

- `NA-0217A` moved only the marker/output foundation:
  - marker formatting
  - routing
  - redaction
  - log writing
  - terminal sanitization
- filesystem/config/locking/path-safety helpers stayed in `qsl/qsl-client/qsc/src/main.rs`
- that boundary preserved no-drift proof for marker truth while leaving the next fail-closed storage-safety seam intact for `NA-0217B`

## Closeout Note

- This closeout PR is governance-only.
- No runtime, workflow, protocol, relay, attachment, or qsc-desktop paths change in this closeout.
