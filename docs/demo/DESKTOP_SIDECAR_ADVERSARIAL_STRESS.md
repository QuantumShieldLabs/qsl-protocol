Goals: G1, G4, G5

Status: Supporting
Owner: QSL client
Last-Updated: 2026-05-11
Replaces: n/a
Superseded-By: n/a

# Desktop Sidecar Adversarial Stress

This note records the NA-0264 desktop/sidecar adversarial stress proof for the
bounded `qsc-desktop` prototype. The proof validates local error-surface
behavior only. It does not promote the desktop prototype, qsc sidecar, relay,
qsl-server, or qsl-attachments to production readiness.

## Artifact Directory

Local artifacts are outside the repository:

```text
/srv/qbuild/tmp/NA-0264_desktop_sidecar_stress_artifacts_20260511T104314Z/
```

Important artifacts:

- `desktop_sidecar_stress_na0264.log`
- `qsc-desktop-appimage-xvfb-scrot-final.png`
- `qsc-desktop-xwininfo-tree-final.log`
- `qsc-desktop-appimage-xvfb-launch-final.log`
- `QSC Desktop Prototype_0.1.0_amd64.AppImage`
- `ARTIFACT_MANIFEST.txt`

Earlier short-wait screenshot attempts in the artifact directory are retained as
diagnostic evidence only; the counted screenshot proof is
`qsc-desktop-appimage-xvfb-scrot-final.png`.

## Commands

From `qsl/qsl-client/qsc-desktop`:

```bash
npm ci
npm run build
npm run prepare:sidecar
npm run tauri:build
```

From the repository root:

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

Packaged-app screenshot proof used:

```bash
env -u WAYLAND_DISPLAY GDK_BACKEND=x11 \
  QSC_CONFIG_DIR=/srv/qbuild/tmp/NA-0264_desktop_sidecar_stress_artifacts_20260511T104314Z/app-config \
  xvfb-run -a -s "-screen 0 1440x1000x24" \
  sh -lc '<launch packaged AppImage, wait for window, xwininfo, scrot, stop app>'
```

## Categories Tested

The bounded local stress helper and package proof covered:

- sidecar missing or invalid configured path;
- sidecar non-executable path;
- sidecar malformed output;
- sidecar nonzero exit without stable error marker;
- sidecar timeout/hang;
- missing local store;
- invalid local store path;
- relay unavailable sidecar send path;
- protocol inactive fail-closed send and receive gates;
- no passphrase or route-token leakage in non-marker sidecar stderr detail;
- no panic across the tested desktop bridge and qsc contract paths;
- packaged desktop launch under Xvfb/scrot; and
- visible non-production desktop boundary in the UI.

## Markers

The stress transcript emitted:

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

## Unsupported Or Partial Categories

- `UNSUPPORTED_KEYCHAIN_ACTIVE_OPS`: keychain-backed active operations remain
  deferred by the desktop prototype boundary.
- `UNSUPPORTED_HANDSHAKE_UI`: handshake/session-establish UI remains out of
  scope; protocol activation is still performed through `qsc` outside the GUI.
- `UNSUPPORTED_PRODUCTION_RELAY`: relay-unavailable proof uses qsc sidecar
  contract coverage, not a production relay or qsl-server hardening claim.
- `UNSUPPORTED_PRODUCTION_PACKAGE_READINESS`: AppImage launch and screenshot
  proof does not prove signing, distribution, updater, or release-channel
  readiness.

Unsupported categories are not faked by the harness.

## What Is Proven

NA-0264 proves that the bounded desktop bridge now rejects invalid sidecar
paths, non-executable sidecars, malformed output, non-marker nonzero exits, and
hung sidecars with explicit errors. It also proves that selected qsc sidecar
contract paths reject missing/invalid local store conditions, relay-unavailable
send attempts, and protocol-inactive send/receive attempts without panic.

The packaged AppImage builds and renders the non-production prototype UI under
Xvfb/scrot on the provisioned Ubuntu qbuild host. The counted screenshot shows
the desktop prototype, bridge readiness, locked/missing profile posture, and
the explicit Linux/macOS prototype wording.

## What Is Not Proven

This proof does not prove:

- production desktop readiness;
- production qsc sidecar readiness;
- production relay, qsl-server, or qsl-attachments readiness;
- signed installer or release-channel readiness;
- keychain-backed active operations;
- GUI-owned handshake/session establishment;
- attachment UI;
- transcript-history UI;
- multiprofile UI; or
- any protocol, wire, crypto, or state-machine change.

## Non-Production Posture

Safe wording:

- "The desktop GUI is a bounded qsc sidecar prototype for guided demo review."
- "NA-0264 pressure-tests selected desktop/sidecar error surfaces."
- "The evidence is local, bounded, non-production, and release-gated."

Do not claim:

- production-ready desktop app;
- production sidecar or relay readiness;
- production qsl-server or qsl-attachments hardening;
- external release approval; or
- protocol/crypto security improvement from this desktop lane.

## Related Evidence

- [NA-0264 audit](../governance/evidence/NA-0264_desktop_sidecar_stress_audit.md)
- [NA-0264 testplan](../../tests/NA-0264_desktop_sidecar_stress_testplan.md)
- [Native desktop package and screenshot readiness](NATIVE_DESKTOP_PACKAGE_SCREENSHOT_READINESS.md)
- [Public demo touch-and-feel readiness](PUBLIC_DEMO_TOUCH_AND_FEEL_READINESS.md)
- [QSC desktop prototype README](../../qsl/qsl-client/qsc-desktop/README.md)
