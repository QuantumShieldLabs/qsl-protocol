Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-03

# NA-0247 Desktop GUI Public Demo Readiness Audit

Directive: QSL-DIR-2026-05-03-027 / NA-0247
Goals: G1, G4, G5

## Scope Conclusion

NA-0247 validates the existing bounded qsc desktop GUI prototype for guided public demo readiness. It does not promote the prototype to production, does not add GUI features, and does not change qsc client-core, protocol wire behavior, KT, SCKA, qsl-server, qsl-attachments, website, Cargo metadata, branch protection, or public-safety configuration.

`NEXT_ACTIONS.md` is intentionally untouched in Packet A. NA-0247 remains READY pending a separate closeout.

## Baseline Proof

- Starting `origin/main`: `9aa93e92ba66`
- Starting READY state: `READY_COUNT 1`, sole READY `NA-0247 — Desktop GUI Prototype Validation and Public Demo Readiness`
- Decision state before work: D-0110 and D-0439 through D-0459 existed once each; D-0460 and D-0461 were absent; duplicate decision count was zero.
- Branch protection: `public-safety` was present in the required contexts with the expected CI contexts.
- Latest main `public-safety`: success on the expected starting main SHA.
- PR preservation: PR #722 was closed and unmerged; PR #708 was merged.

## Local Validation Summary

| Command | Result | Evidence |
|---|---:|---|
| `cargo audit --deny warnings` | PASS | RustSec advisory scan completed against 381 locked crate dependencies. |
| `cargo tree -i rustls-webpki --locked` | PASS | `rustls-webpki v0.103.13` is reached through `rustls v0.23.36`. |
| `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1` | PASS | 3 tests passed. |
| `cargo test -p qsc --locked --test desktop_gui_contract_na0215b -- --test-threads=1` | PASS | 3 tests passed. |
| `cargo test -p qsc --locked --test qsp_protocol_gate -- --test-threads=1` | PASS | 6 tests passed. |
| `npm ci` in `qsl/qsl-client/qsc-desktop` | PASS with warnings | Lockfile install added local node modules and reported existing npm advisory warnings: 1 moderate and 1 high. No lockfile change was made and no dependency update is authorized in this lane. |
| `npm run build` in `qsl/qsl-client/qsc-desktop` | PASS | Vite production frontend build completed. |
| `npm run prepare:sidecar` in `qsl/qsl-client/qsc-desktop` | PASS | Built release `qsc` from the isolated cargo target directory and copied the sidecar into the Tauri resources path. |
| `npm run tauri:build` in `qsl/qsl-client/qsc-desktop` | HOST-LIMITED | The command completed sidecar prep and frontend build, then failed in native backend compilation because `pkg-config` is not installed for the GLib dependency chain. |

## Recovered Failure Evidence

- Failing command: `npm run tauri:build`
- Classification: host-limited package validation, not a repo behavior failure. The Tauri native Linux build needs host packages outside the repository, and this directive forbids global tool installation and dependency updates.
- Evidence: native backend compilation failed in `glib-sys` because the `pkg-config` command was not found.
- Corrective action: checked host tool availability with a zero-failure-safe probe; it reported `pkg-config not found`. Kept the successful `npm ci`, `npm run build`, and `npm run prepare:sidecar` outputs as the bounded package/readiness proof.
- Final result: frontend and sidecar readiness are locally validated; full Linux Tauri package/AppImage build and screenshot capture are documented as host-limited on this qbuild worker.

## Desktop GUI Contract Evidence

`qsl/qsl-client/qsc/tests/desktop_gui_contract_na0215b.rs` passed 3 tests and proves the sidecar-facing desktop contract remains deterministic for:

- profile/doctor/vault/identity markers;
- contact and device trust surfaces;
- message delivery and timeline truth.

The test harness exercises the existing qsc sidecar marker contract rather than adding a second client-core implementation.

## Protocol-Inactive / qsp Gate Evidence

`qsl/qsl-client/qsc/tests/qsp_protocol_gate.rs` passed 6 tests and proves:

- send refuses when protocol state is inactive;
- receive refuses when protocol state is inactive;
- inactive sends do not create outbox state;
- inactive receives leave the output directory empty;
- midpoint handshake state stays honest;
- active send/receive paths remain allowed only when protocol state is actually ready;
- status output remains secret-safe.

This preserves the required `protocol_inactive` fail-closed behavior.

## Guided Public Demo Readiness Walkthrough

No GUI screenshot was captured because this host does not currently have the validated native Linux Tauri build prerequisites. The text walkthrough below is the evidence artifact for the guided demo flow.

1. Init: the desktop runtime initializes a passphrase-backed profile by invoking `qsc vault init --non-interactive --passphrase-stdin`, rotates identity through the sidecar, then keeps the passphrase only in backend memory.
2. Unlock: the runtime invokes `qsc vault unlock --non-interactive --passphrase-env QSC_DESKTOP_SESSION_PASSPHRASE` with the passphrase scoped to the child process environment.
3. Contact: the runtime uses sidecar commands for self inbox token setup, contact add/refresh, device list, and device trust.
4. Readiness: the runtime refreshes `status`, `doctor`, `vault status`, `identity show`, `contacts list`, and peer-specific `handshake status`; the GUI only surfaces sidecar truth.
5. Send: the runtime writes the composed message to a temporary file and invokes `qsc send` through the relay path only when sidecar protocol state is ready.
6. Receive: the runtime creates a temporary receive directory and invokes `qsc receive` with bounded `max` in the 1..16 range only when sidecar protocol state supports the flow.

## Explicit Limitations

- Keychain-backed active operations remain deferred. A keychain-backed vault is not treated as active-ready by the GUI.
- `protocol_inactive` remains fail-closed and truthful; the GUI does not seed fallback state or fake readiness.
- Handshake/session-establish UI remains out of scope; `handshake status` is inspection/readiness evidence only.
- Attachments UI remains out of scope.
- Transcript history UI remains out of scope; the current peer timeline summary is not a full history client.
- Multiprofile UI remains out of scope.
- The desktop GUI remains a qsc sidecar shell, not a second client-core.
- The prototype is not production-ready and must not be marketed as a production GUI release.

## Residual Gaps

- Install the native Linux Tauri build prerequisites on a validation host or run the package lane on an already provisioned host before claiming AppImage proof.
- Capture a small screenshot or scripted GUI transcript only after the native GUI package path is available without unsafe host setup.
- Keep keychain active operations, handshake/session-establish UI, attachments UI, transcript history, and multiprofile work behind separate explicit queue items.

## Boundary Confirmation

- `.github` untouched.
- `scripts/ci/public_safety_gate.py` untouched.
- `Cargo.toml` and `Cargo.lock` untouched.
- `qsl-server` untouched.
- `qsl-attachments` untouched.
- `website` untouched.
- protocol-core, KT, SCKA, and cryptographic state-machine paths untouched.
- qsc runtime/core paths untouched outside read-only execution of existing tests.
