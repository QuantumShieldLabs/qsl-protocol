Status: Archive
Owner: QSL governance
Last-Updated: 2026-03-31

# NA-0217B Filesystem / Config / Locking Foundation Extraction Evidence

Goals: G4, G5

## Scope

- Repo: `qsl-protocol`
- Lane: `NA-0217B`
- Posture: implementation already merged; this closeout is governance-only and introduces no runtime changes
- Implementation PR: #629
- Implementation branch head before merge: `e4504111d1b5`
- Implementation merge SHA: `7c697463a5a5`

## Authority Proof

- refreshed `qsl-protocol` checkout `HEAD`, `mirror/main`, bare mirror `main`, and `origin/main` all resolved to `7c697463a5a5`
- refreshed merged main contains `DECISIONS.md` `D-0347`, the `TRACEABILITY.md` `NA-0217B implementation/evidence` entry, `qsl/qsl-client/qsc/src/fs_store/mod.rs`, and `qsl/qsl-client/qsc/tests/fs_store_contract_na0217b.rs`
- refreshed live queue still showed `READY_COUNT=1` with `NA-0217B` as the sole live `READY` item before this closeout
- refreshed sibling-repo proof stayed read-only and truthful: `qsl-server READY=0`, `qsl-attachments READY=0`

## LOC Proof

- `qsl/qsl-client/qsc/src/main.rs` before merge parent: `21,338` LOC
- `qsl/qsl-client/qsc/src/main.rs` after merged extraction: `20,992` LOC
- `qsl/qsl-client/qsc/src/fs_store/mod.rs`: `374` LOC

## Practical Moved-Helper Inventory

- config-directory and policy-profile helpers:
  - `config_dir`
  - `normalize_profile`
  - `read_policy_profile`
  - `write_config_atomic`
- secure directory/store layout and atomic write helpers:
  - `ensure_dir_secure`
  - `ensure_store_layout`
  - `write_atomic`
  - directory fsync helpers
- fail-closed path-safety and locking helpers:
  - safe-parent / symlink checks
  - shared and exclusive store locking helpers
  - writable-directory probing
  - permission enforcement helpers
  - umask setup

## No-Drift Proof Surface

### Symlink-safe rejection

- clean-main truth probe: symlinked config-dir case matched extracted branch with fail-closed `unsafe_path_symlink`
- `cargo test --test fs_store_contract_na0217b`

### `0700` / `0600` enforcement

- `cargo test --test fs_store_contract_na0217b`
- `cargo test --test identity_secret_at_rest`
- `cargo test --test session_state_at_rest`

### Atomic write behavior

- `cargo test --test fs_store_contract_na0217b`
- `cargo test --test timeline_store`

### Lock behavior

- `cargo test --test fs_store_contract_na0217b`

### qsc-desktop-sensitive store/config proof

- `cargo test --test desktop_gui_contract_na0215b`
- existing sidecar/store-config assumptions remained intact while runtime moved from `main.rs` into `fs_store`

### `NA-0217A` output-marker canary

- `cargo test --test output_marker_contract_na0217a`

## Baseline Truth-Probe Summary

- clean main matched the extracted branch on the symlinked-config-dir case
- both returned fail-closed `unsafe_path_symlink` when the probe used absolute paths
- no `config.txt` was created
- no `store.meta` was created
- no temp-file residue remained

## Exact Commands / Tests Run For The Merged Implementation Lane

- `cargo fmt --check`
- `cargo build`
- `cargo clippy -- -D warnings`
- `cargo test --test identity_secret_at_rest`
- `cargo test --test session_state_at_rest`
- `cargo test --test timeline_store`
- `cargo test --test desktop_gui_contract_na0215b`
- `cargo test --test output_marker_contract_na0217a`
- `cargo test --test fs_store_contract_na0217b`
- local `tools/goal_lint.py` revalidation via synthesized event payload after adding the required governance companions

## Why NA-0217B Stayed Narrower Than NA-0217C

- `NA-0217B` moved only the generic filesystem/config/locking/path-safety foundation
- protocol activation/status truth and encrypted session-at-rest ownership stayed in `qsl/qsl-client/qsc/src/main.rs`
- that boundary kept the fs-store lane below the higher-semantic-risk protocol-state seam and leaves `NA-0217C` as the next truthful move

## Closeout Note

- This closeout PR is governance-only.
- No runtime, workflow, protocol, relay, attachment, or qsc-desktop paths change in this closeout.
