Goals: G1, G4, G5

Status: Supporting
Owner: QSL client
Last-Updated: 2026-05-09
Replaces: n/a
Superseded-By: n/a

# Native Desktop Package and Screenshot Readiness

This note records the NA-0258 provisioned-host proof for the bounded
`qsc-desktop` prototype. It validates a local native Linux package and a
packaged-app Xvfb screenshot on one provisioned Ubuntu qbuild host. It does not
promote the desktop prototype to production readiness.

## Provisioned Host

Detected host prerequisites:

- OS: Ubuntu 24.04.4 LTS on x86_64 Linux.
- Node: `v24.15.0`.
- npm: `11.12.1`.
- cargo: `1.95.0`.
- rustc: `1.95.0`.
- `pkg-config`: `/usr/bin/pkg-config`, version `1.8.1`.
- GLib: `glib-2.0` version `2.80.0`.
- GIO: `gio-2.0` version `2.80.0`.
- GTK: `gtk+-3.0` version `3.24.41`.
- WebKitGTK: `webkit2gtk-4.1` version `2.52.3`.
- Optional older WebKitGTK name: `webkit2gtk-4.0` was not installed.
- Xvfb: `/usr/bin/xvfb-run`.
- scrot: `/usr/bin/scrot`.
- Firefox: `/usr/bin/firefox`, `Mozilla Firefox 150.0.1`.

Xvfb screenshot preflight passed:

```bash
xvfb-run -a -s "-screen 0 1440x1000x24" sh -lc 'firefox --version; scrot /tmp/qsl-na0258-xvfb-preflight.png'
ls -lh /tmp/qsl-na0258-xvfb-preflight.png
```

Output summary:

- `/tmp/qsl-na0258-xvfb-preflight.png`
- PNG image data, `1440 x 1000`, RGB.

## Commands Run

From `qsl/qsl-client/qsc-desktop`:

```bash
npm ci
npm run build
npm run prepare:sidecar
npm run tauri:build
```

From the repository root:

```bash
cargo +stable test -p qsc --locked --test desktop_gui_contract_na0215b -- --test-threads=1
cargo +stable test -p qsc --locked --test qsp_protocol_gate -- --test-threads=1
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
```

Packaged-app screenshot command shape:

```bash
env -u WAYLAND_DISPLAY GDK_BACKEND=x11 \
  xvfb-run -a -s "-screen 0 1440x1000x24" \
  sh -lc '<launch AppImage, wait for window, capture scrot, stop app>'
```

The screenshot path is a native AppImage launch under Xvfb. It is not a
browser-only frontend screenshot.

## Package Artifact

Generated package:

```text
/srv/qbuild/cache/targets/qsl-protocol/release/bundle/appimage/QSC Desktop Prototype_0.1.0_amd64.AppImage
```

Captured artifact copy:

```text
/srv/qbuild/tmp/NA-0258_native_desktop_artifacts_20260509T194934Z/QSC Desktop Prototype_0.1.0_amd64.AppImage
```

Artifact summary:

- Type: ELF 64-bit x86_64 AppImage executable.
- Size: `78M`.
- Artifact manifest:
  `/srv/qbuild/tmp/NA-0258_native_desktop_artifacts_20260509T194934Z/ARTIFACT_MANIFEST.txt`.

## Screenshot Artifact

Captured packaged-app screenshot:

```text
/srv/qbuild/tmp/NA-0258_native_desktop_artifacts_20260509T194934Z/qsc-desktop-appimage-xvfb-scrot.png
```

Screenshot summary:

- Type: PNG image data.
- Dimensions: `1440 x 1000`.
- Size: `256K`.
- Visual check: the screenshot shows the native QSC Desktop Prototype window
  with bridge, identity, doctor, readiness, peer-state, and message-session UI
  surfaces.

The launch log recorded only non-fatal Xvfb/EGL acceleration warnings. The
window rendered and the screenshot captured successfully.

## Contract and Build Status

Local validation passed:

- `npm ci`: passed, with existing npm audit notices reported by npm.
- `npm run build`: passed.
- `npm run prepare:sidecar`: passed and copied the release `qsc` sidecar.
- `npm run tauri:build`: passed and emitted one Linux AppImage bundle.
- `desktop_gui_contract_na0215b`: passed, 3 tests.
- `qsp_protocol_gate`: passed, 6 tests.
- `send_commit`: passed, 3 tests.
- `cargo audit --deny warnings`: passed.
- `cargo tree -i rustls-webpki --locked`: resolved `rustls-webpki v0.103.13`.

No Rust source changed in this proof, so `cargo fmt`, `cargo build`, and
`cargo clippy` were not required by the NA-0258 validation rule.

## Repeating the Proof on a Provisioned Host

Prerequisites:

- Linux host with GTK/WebKitGTK development packages visible to `pkg-config`.
- `pkg-config` resolves `glib-2.0`, `gio-2.0`, and `webkit2gtk-4.1`.
- `xvfb-run`, `scrot`, and a browser are installed for headless display proof.
- Rust, cargo, Node, and npm are available.

Repeat:

```bash
cd qsl/qsl-client/qsc-desktop
npm ci
npm run build
npm run prepare:sidecar
npm run tauri:build
```

Then launch the emitted AppImage under Xvfb with `WAYLAND_DISPLAY` unset and
`GDK_BACKEND=x11`, capture with `scrot`, and store the package plus screenshot
outside the repository under `/srv/qbuild/tmp/`.

## Non-Production Boundary

This proof supports only native package and screenshot readiness for the bounded
prototype on a provisioned host. It does not show:

- production desktop readiness;
- signed installers;
- release distribution approval;
- keychain-backed active operations;
- production relay or service readiness;
- qsl-server production readiness;
- qsl-attachments production readiness; or
- any protocol, crypto, state-machine, or wire-semantics change.

The desktop remains a non-production prototype unless later release evidence
changes that status.

## Known Gaps

- Keychain-backed active operations remain deferred.
- Native package coverage is limited to this Linux AppImage proof on one
  provisioned host; broader Linux distro and macOS package coverage remain
  separate work.
- Signed installers are not produced by this proof.
- Production relay/service readiness remains out of scope.
- The npm install path reported existing npm audit notices; this proof did not
  change package dependencies and did not treat those notices as release
  approval.

## Related Evidence

- [NA-0258 audit](../governance/evidence/NA-0258_native_desktop_package_screenshot_audit.md)
- [NA-0258 testplan](../../tests/NA-0258_native_desktop_package_screenshot_testplan.md)
- QSC desktop prototype README — retired at NA-0651 (D-1274, 2026-07-16); see git history and DOC-QSC-009/010 (superseded, retained as history)
