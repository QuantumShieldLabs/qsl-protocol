Goals: G1, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-11
Replaces: n/a
Superseded-By: n/a

# NA-0264 Desktop Sidecar Stress Audit

Directive: QSL-DIR-2026-05-10-064 / NA-0264

## Objective

Pressure-test the public desktop/sidecar prototype surface against bad sidecar
paths, unavailable sidecar, malformed sidecar output, invalid local store,
relay unavailable, protocol inactive, clear errors, no secret leakage, no
panic, and no production-ready desktop claim.

## Baseline Proof

- Starting `origin/main`: `ef4f5d52a7d7`.
- PR #780 through #761 and #708: verified merged.
- PR #750 and PR #722: verified closed and unmerged.
- Branch protection: expected required contexts were present, including
  `public-safety`; force pushes and deletions were disabled; admin enforcement
  was enabled.
- Latest starting-main `public-safety`: success.
- Queue proof: `READY_COUNT 1`, sole READY `NA-0264`.
- Decision proof: D-0497 existed once, D-0498 was absent before this patch, and
  duplicate decision count was zero.
- Corrected NA-0262A classifier proof:
  - docs/governance-only paths: `docs_only=true`, `runtime_critical=false`;
  - qsc-desktop path: `runtime_critical=true`;
  - mixed docs plus qsc-desktop path: `runtime_critical=true`;
  - empty/ambiguous path set: `runtime_critical=true`.
- The missing `selftest-full-suite-cost-control` subcommand was not required or
  added.

## Host Prerequisites

- Node/npm: `v24.15.0` / `11.12.1`.
- Rust/cargo: `rustc 1.95.0` / `cargo 1.95.0`.
- `pkg-config`: `/usr/bin/pkg-config`.
- WebKitGTK: `webkit2gtk-4.1` version `2.52.3`.
- Xvfb/scrot: `/usr/bin/xvfb-run`, `/usr/bin/scrot`.
- Firefox: `/usr/bin/firefox`.
- Disk watermark at start: `/srv/qbuild` total `468G`, used `40G`, free
  `404G`, used `9%`.

## Commands Run

Packet A and build/package proof:

```bash
command -v node
node --version
command -v npm
npm --version
command -v cargo
cargo --version
command -v rustc
rustc --version
command -v pkg-config
pkg-config --modversion webkit2gtk-4.1
command -v xvfb-run
command -v scrot
command -v firefox

cd qsl/qsl-client/qsc-desktop
npm ci
npm run build
npm run prepare:sidecar
npm run tauri:build
```

Desktop/sidecar stress and contract checks:

```bash
NA0264_ARTIFACT_DIR=/srv/qbuild/tmp/NA-0264_desktop_sidecar_stress_artifacts_20260511T104314Z \
  scripts/ci/desktop_sidecar_stress_na0264.sh

cargo test --manifest-path qsl/qsl-client/qsc-desktop/src-tauri/Cargo.toml --locked
cargo test -p qsc --locked --test desktop_gui_contract_na0215b -- --test-threads=1
cargo test -p qsc --locked --test qsp_protocol_gate -- --test-threads=1
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
```

Packaged screenshot proof:

```bash
env -u WAYLAND_DISPLAY GDK_BACKEND=x11 \
  QSC_CONFIG_DIR=/srv/qbuild/tmp/NA-0264_desktop_sidecar_stress_artifacts_20260511T104314Z/app-config \
  xvfb-run -a -s "-screen 0 1440x1000x24" \
  sh -lc '<launch AppImage, wait for rendered window, capture xwininfo, scrot, stop app>'
```

## Implementation Findings And Fixes

Concrete in-scope findings:

- Invalid `QSC_DESKTOP_QSC_BIN` values could fall back to the bundled sidecar.
- Sidecar execution had no bounded timeout.
- Non-marker sidecar stderr could be surfaced as UI error detail.
- Tauri startup used `expect("tauri app run")`.

Bounded fixes:

- `qsc-desktop` now validates configured/bundled sidecar paths, rejects missing
  paths, rejects non-regular files, and rejects non-executable files.
- Sidecar execution now has a bounded default timeout and kills a hung child.
- Non-marker stderr detail is suppressed before being surfaced to the UI.
- Tauri startup errors are printed and exited instead of panicking through
  `expect`.
- Added local desktop-sidecar stress helper
  `scripts/ci/desktop_sidecar_stress_na0264.sh`.

No protocol, wire, crypto, state-machine, qsl-server, qsl-attachments, website,
branch-protection, public-safety, `.github`, Cargo manifest, or Cargo lockfile
change was made.

## Stress Result

The stress transcript passed and emitted:

```text
NA0264_SIDECAR_MISSING_REJECT_OK
NA0264_SIDECAR_BAD_PATH_REJECT_OK
NA0264_SIDECAR_NONEXEC_REJECT_OK
NA0264_SIDECAR_MALFORMED_OUTPUT_REJECT_OK
NA0264_SIDECAR_NONZERO_REJECT_OK
NA0264_SIDECAR_TIMEOUT_REJECT_OK
NA0264_MISSING_STORE_REJECT_OK
NA0264_INVALID_STORE_REJECT_OK
NA0264_RELAY_UNAVAILABLE_REJECT_OK
NA0264_PROTOCOL_INACTIVE_FAIL_CLOSED_OK
NA0264_NO_SECRET_LEAK_OK
NA0264_NO_PANIC_OK
NA0264_DESKTOP_SIDECAR_STRESS_OK
```

Coverage mapping:

- sidecar missing/bad path: `sidecar_path_validation_rejects_missing_override`;
- sidecar non-executable: `sidecar_path_validation_rejects_non_executable_file`;
- malformed output: `malformed_doctor_output_rejects_without_panic`;
- nonzero/no stable marker/no secret leak:
  `non_marker_sidecar_failure_suppresses_stderr_secrets`;
- timeout/hang: `sidecar_timeout_kills_hung_child`;
- missing store: qsc `doctor_check_only_no_dir`;
- invalid local store path: qsc `symlink_path_rejected_no_mutation`;
- protocol inactive: qsc `send_refuses_when_protocol_inactive` and
  `receive_refuses_when_protocol_inactive`;
- relay unavailable: qsc `send_failure_no_commit`.

## Package And Screenshot Result

- `npm ci`: passed, with existing npm audit notices.
- `npm run build`: passed.
- `npm run prepare:sidecar`: passed.
- `npm run tauri:build`: passed and emitted one Linux AppImage.
- Package copy:
  `/srv/qbuild/tmp/NA-0264_desktop_sidecar_stress_artifacts_20260511T104314Z/QSC Desktop Prototype_0.1.0_amd64.AppImage`.
- Counted screenshot:
  `/srv/qbuild/tmp/NA-0264_desktop_sidecar_stress_artifacts_20260511T104314Z/qsc-desktop-appimage-xvfb-scrot-final.png`.
- Screenshot dimensions: `1440 x 1000`.
- Window tree proof: `QSC Desktop Prototype` window at `1500x980`.
- Launch log warnings: non-fatal EGL acceleration warnings under Xvfb.

Short-wait screenshot attempts captured only the black Xvfb root before the
Tauri/WebKit window finished rendering. They are not counted as proof; the final
longer-wait capture is the counted screenshot.

## No-Leak / No-Panic Proof

- The stress helper passed `NA0264_NO_SECRET_LEAK_OK`.
- The non-marker sidecar stderr regression test used a fake sidecar that wrote
  passphrase/token-like sentinel material to stderr; the desktop bridge
  suppressed that raw stderr from UI detail.
- The stress helper passed `NA0264_NO_PANIC_OK`.
- Production code no longer contains the Tauri startup `expect`; remaining
  `expect` findings in `qsc-desktop/src-tauri/src/qsc.rs` are test-only.

## Npm Audit Findings

`npm audit --json` exited `1` and reported existing dependency notices:

- `postcss`: moderate, transitive.
- `vite`: high, direct dev dependency.

No package dependency change was made in this lane. These notices should be
handled in a future desktop dependency hygiene lane before any release-channel
claim.

## Unsupported / Partial Categories

- `UNSUPPORTED_KEYCHAIN_ACTIVE_OPS`: keychain-backed active operations remain
  deferred.
- `UNSUPPORTED_HANDSHAKE_UI`: handshake/session-establish UI remains out of
  scope.
- `UNSUPPORTED_PRODUCTION_RELAY`: relay-unavailable proof is qsc sidecar
  contract proof, not production relay proof.
- `UNSUPPORTED_PRODUCTION_PACKAGE_READINESS`: AppImage package/screenshot proof
  does not prove signed installer or release-channel readiness.

Unsupported categories were not faked.

## Non-Production Claim Boundary

This audit supports only bounded local desktop/sidecar stress evidence for the
non-production qsc-desktop prototype.

It does not claim:

- production desktop readiness;
- production sidecar readiness;
- production relay, qsl-server, or qsl-attachments readiness;
- signed installer or distribution readiness;
- keychain-backed active operations;
- GUI-owned protocol activation; or
- protocol, wire, crypto, or state-machine change.

## Recommendations

1. Merge NA-0264 only after required checks pass normally with `public-safety`
   still required.
2. Keep NA-0264 READY after the evidence PR until a separate closeout promotes
   exactly one successor.
3. Add a future desktop dependency hygiene lane for the npm audit findings
   before any release-channel claim.
4. Keep screenshot automation using a rendered-window wait rather than a fixed
   short sleep.

## Related Evidence

- [Desktop sidecar adversarial stress](../../demo/DESKTOP_SIDECAR_ADVERSARIAL_STRESS.md)
- [NA-0264 testplan](../../../tests/NA-0264_desktop_sidecar_stress_testplan.md)
- QSC desktop prototype README — retired at NA-0651 (D-1274, 2026-07-16); see git history and DOC-QSC-009/010 (superseded, retained as history)
