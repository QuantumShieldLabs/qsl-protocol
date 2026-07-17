Goals: G1, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-09
Replaces: n/a
Superseded-By: n/a

# NA-0258 Native Desktop Package and Screenshot Audit

Directive: QSL-DIR-2026-05-09-055 / NA-0258

## Objective

Validate native desktop package and screenshot proof for the bounded
`qsc-desktop` prototype on the now-provisioned qbuild host, while preserving
the non-production desktop boundary and avoiding protocol, crypto, qsl-server,
qsl-attachments, website, workflow, branch-protection, public-safety, Cargo, or
production-service changes.

## Baseline Proof

- Starting `origin/main`: `8541a7d67bea`.
- PR #765, #764, #763, #762, #761, and #708: merged.
- PR #750 and #722: closed and unmerged.
- Branch protection: `public-safety` required with expected protected contexts;
  force pushes and deletions disabled; admin enforcement enabled.
- Latest main `public-safety`: success on the starting main SHA.
- Queue proof: `READY_COUNT 1`, sole READY `NA-0258`.
- Decision proof: D-0482 existed once; D-0483 absent; duplicate decision count
  zero.

## Host Prerequisites Detected

- OS: Ubuntu 24.04.4 LTS on x86_64 Linux.
- Node/npm: `v24.15.0` / `11.12.1`.
- Rust/cargo: `rustc 1.95.0` / `cargo 1.95.0`.
- `pkg-config`: `/usr/bin/pkg-config`, version `1.8.1`.
- `glib-2.0`: `2.80.0`.
- `gio-2.0`: `2.80.0`.
- `gtk+-3.0`: `3.24.41`.
- `webkit2gtk-4.1`: `2.52.3`.
- `webkit2gtk-4.0`: not installed, not required for this Tauri 2 proof.
- `xvfb-run`: `/usr/bin/xvfb-run`.
- `scrot`: `/usr/bin/scrot`.
- Firefox: `/usr/bin/firefox`, `Mozilla Firefox 150.0.1`.

Xvfb screenshot preflight succeeded and produced:

```text
/tmp/qsl-na0258-xvfb-preflight.png
```

## Commands Run

Host preflight:

```bash
uname -a
lsb_release -a || cat /etc/os-release
command -v node
node --version
command -v npm
npm --version
command -v cargo
cargo --version
command -v rustc
rustc --version
command -v pkg-config
pkg-config --version
pkg-config --modversion glib-2.0
pkg-config --modversion gio-2.0
pkg-config --modversion gtk+-3.0 || true
pkg-config --modversion webkit2gtk-4.0 || true
pkg-config --modversion webkit2gtk-4.1
command -v xvfb-run
command -v scrot
command -v firefox
command -v google-chrome || true
command -v chromium || true
xvfb-run -a -s "-screen 0 1440x1000x24" sh -lc 'firefox --version; scrot /tmp/qsl-na0258-xvfb-preflight.png'
```

Package and screenshot proof:

```bash
cd qsl/qsl-client/qsc-desktop
npm ci
npm run build
npm run prepare:sidecar
npm run tauri:build
env -u WAYLAND_DISPLAY GDK_BACKEND=x11 xvfb-run -a -s "-screen 0 1440x1000x24" <packaged AppImage launch plus scrot>
```

Contract and dependency-health checks:

```bash
cargo +stable test -p qsc --locked --test desktop_gui_contract_na0215b -- --test-threads=1
cargo +stable test -p qsc --locked --test qsp_protocol_gate -- --test-threads=1
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
```

## Package Result

Package build succeeded and emitted one Linux AppImage:

```text
/srv/qbuild/cache/targets/qsl-protocol/release/bundle/appimage/QSC Desktop Prototype_0.1.0_amd64.AppImage
```

Captured artifact copy:

```text
/srv/qbuild/tmp/NA-0258_native_desktop_artifacts_20260509T194934Z/QSC Desktop Prototype_0.1.0_amd64.AppImage
```

Artifact summary:

- Size: `78M`.
- File type: ELF 64-bit x86_64 AppImage executable.

Tauri bundling downloaded its standard AppRun/linuxdeploy helper binaries during
package creation. No repo dependency file changed.

## Screenshot Result

Packaged-app screenshot succeeded under Xvfb:

```text
/srv/qbuild/tmp/NA-0258_native_desktop_artifacts_20260509T194934Z/qsc-desktop-appimage-xvfb-scrot.png
```

Screenshot summary:

- Size: `256K`.
- File type: PNG image data.
- Dimensions: `1440 x 1000`.
- Visual proof: rendered QSC Desktop Prototype window from the packaged
  AppImage.

The launch log contains only non-fatal Xvfb/EGL acceleration warnings. No GNOME
Wayland screenshot DBus path was used.

## Artifact Manifest

Artifact directory:

```text
/srv/qbuild/tmp/NA-0258_native_desktop_artifacts_20260509T194934Z/
```

Artifacts:

- `ARTIFACT_MANIFEST.txt`.
- `QSC Desktop Prototype_0.1.0_amd64.AppImage`.
- `qsc-desktop-appimage-xvfb-launch.log`.
- `qsc-desktop-appimage-xvfb-scrot.png`.

The artifact manifest stores file sizes and digest material outside the
repository. This committed audit records paths and summaries only.

## Contract / Build Status

- `npm ci`: passed, with existing npm audit notices reported by npm.
- `npm run build`: passed.
- `npm run prepare:sidecar`: passed.
- `npm run tauri:build`: passed.
- `desktop_gui_contract_na0215b`: passed, 3 tests.
- `qsp_protocol_gate`: passed, 6 tests.
- `send_commit`: passed, 3 tests.
- `cargo audit --deny warnings`: passed.
- `cargo tree -i rustls-webpki --locked`: `rustls-webpki v0.103.13`.

## Non-Production Claim Boundary

This audit proves only local native package and screenshot readiness for a
bounded non-production desktop prototype on one provisioned Linux qbuild host.

It does not claim:

- production desktop readiness;
- signed installer readiness;
- release-channel readiness;
- keychain-backed active operations;
- production relay/service readiness;
- production qsl-server or qsl-attachments readiness;
- website readiness; or
- protocol, wire, crypto, or state-machine change.

## Residual Gaps

- Keychain-backed active operations remain deferred.
- Native package proof is Linux AppImage only on one provisioned host.
- macOS package proof remains separate.
- Signed installers remain separate.
- Production relay/service readiness remains separate.
- qsl-server and qsl-attachments production hardening remain separate.
- KT-negative and attachment demo readiness remain separate.
- Existing npm audit notices should be triaged in a future desktop dependency
  hygiene lane before any release-channel claim.

## Recommendations

1. Merge NA-0258 evidence only after required checks pass normally with
   `public-safety` still required.
2. Keep NA-0258 READY after the implementation/evidence PR until a separate
   closeout promotes one successor.
3. Do not infer production desktop readiness from this local AppImage proof.
4. Prefer a future desktop dependency-hygiene lane before broader package
   distribution work.

## Scope Conclusion

NA-0258 evidence changes are limited to documentation, governance evidence,
decision traceability, the testplan, and the rolling operations journal. The
proof did not change protocol/crypto state machines, qsl-server,
qsl-attachments, qsc-desktop implementation code, website/external website,
workflows, public-safety configuration, branch protection, Cargo manifests,
Cargo lockfiles, production relay/service code, or runtime protocol behavior.

## Related Evidence

- [Native desktop readiness](../../demo/NATIVE_DESKTOP_PACKAGE_SCREENSHOT_READINESS.md)
- [NA-0258 testplan](../../../tests/NA-0258_native_desktop_package_screenshot_testplan.md)
- QSC desktop prototype README — retired at NA-0651 (D-1274, 2026-07-16); see git history and DOC-QSC-009/010 (superseded, retained as history)
