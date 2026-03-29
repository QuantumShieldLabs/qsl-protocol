Goals: G4, G5

Status: Supporting
Owner: QSL client
Last-Updated: 2026-03-29

# QSC Desktop Prototype

This directory contains the bounded Linux/macOS desktop GUI prototype for `NA-0215B`.

What it is:
- a Tauri-first shell under `qsl/qsl-client/**`;
- a message-first, single-profile UI over the frozen `qsc` command/output subset; and
- a Rust-only bridge that keeps unlock handling memory-only and child-scoped.

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

Frontend build:
- `npm run build`

Local package build:
- `npm run tauri:build`

For backend-only tests or local dev without a bundled resource, set:
- `QSC_DESKTOP_QSC_BIN=/absolute/path/to/qsc`

## Truthful prototype limitations

The current prototype keeps two direct implementation gaps explicit:
1. keychain-backed active operations are not surfaced yet because the current shell contract still exposes the passphrase unlock path for sidecar-driven operations; and
2. the frozen GUI slice does not yet include handshake/session-establish UI, so message send/receive remains fail-closed when the sidecar reports `protocol_inactive`.
