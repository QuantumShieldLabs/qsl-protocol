Goals: G4, G5

Status: Supporting
Owner: QSL client
Last-Updated: 2026-03-30

# QSC Desktop Prototype

This directory contains the bounded Linux/macOS desktop GUI prototype for `NA-0215B`.

What it is:
- a Tauri-first shell under `qsl/qsl-client/**`;
- a message-first, single-profile UI over the frozen `qsc` command/output subset; and
- a Rust-only bridge that keeps unlock handling memory-only and child-scoped.

Validation posture:
- qbuild/local first and AWS-free for this bounded prototype lane;
- use the existing local `qsc` runbooks and relay harness flows for operator proof; and
- treat remote relay/AWS artifacts as outside this prototype validation surface.

What it is not:
- not an attachments UI;
- not a transcript-history UI;
- not a multiprofile client;
- not a TUI scraper; and
- not a second client-core implementation in frontend JavaScript.

## Build notes

The desktop app does not join the repo root Cargo workspace. Its Tauri backend lives in:
- `src-tauri/`

Its frontend build lives in:
- `src/`

Sidecar prep:
- `npm run prepare:sidecar`
- this builds `qsc` in release mode and copies the target-matched binary to `src-tauri/resources/bin/qsc`
- if `CARGO_TARGET_DIR` is set for isolated builds, the prep step now reads the release binary from that target directory instead of assuming the repo-root `target/`

Frontend build:
- `npm run build`

Local package build:
- `npm run tauri:build`
- the default Linux proof emits the AppImage lane under `src-tauri/target/release/bundle/appimage/`
- if `CARGO_TARGET_DIR` is set or `tauri build --debug` is used, read the bundle from that target directory/profile instead
- macOS proof is limited to the `.app` bundle lane on a macOS host

For backend-only tests or local dev without a bundled resource, set:
- `QSC_DESKTOP_QSC_BIN=/absolute/path/to/qsc`

## Truthful prototype limitations

The current prototype keeps the remaining active-ops boundary explicit:
1. active ops in this build mean passphrase-backed init/unlock, relay/contact/device mutations, and send/receive through the bundled `qsc` sidecar;
2. keychain-backed active operations are still deferred, so a keychain-backed vault may be detected but is not treated as active-ready by the GUI; and
3. the GUI surfaces peer-specific protocol readiness truthfully, but handshake/session-establish remains outside the prototype, so send requires `send_ready=yes`, receive requires `established` or `established_recv_only`, and inactive peers stay fail-closed with `protocol_inactive` until `qsc` is activated out of band.
