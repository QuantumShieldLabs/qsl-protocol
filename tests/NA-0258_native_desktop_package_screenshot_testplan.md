Goals: G1, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-09
Replaces: n/a
Superseded-By: n/a

# NA-0258 Native Desktop Package and Screenshot Test Plan

## Objective

Validate native desktop package and screenshot readiness for the bounded
`qsc-desktop` prototype on a provisioned host, while preserving non-production
desktop boundaries and avoiding protocol, crypto, qsl-server, qsl-attachments,
website, workflow, branch-protection, public-safety, Cargo, or production
service changes.

## Protected Invariants

- Desktop remains non-production unless later release evidence changes that
  status.
- No production-ready desktop claim.
- Host prerequisites are explicit.
- Package and screenshot artifacts are captured or the exact host blocker is
  recorded.
- Screenshot proof comes from the packaged native app, not a browser-only
  frontend capture.
- No protocol, wire, crypto, or state-machine change.
- qsl-server and qsl-attachments are untouched.
- Website/external website, `.github`, branch protection, public-safety
  configuration, Cargo manifests, and Cargo lockfiles are untouched.
- No global package installation, sudo, display-manager mutation, or GNOME
  Wayland screenshot DBus dependency.

## Host Preflight

Run:

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
ls -lh /tmp/qsl-na0258-xvfb-preflight.png
```

Expected:

- `pkg-config`, `xvfb-run`, `scrot`, and Firefox exist.
- `glib-2.0`, `gio-2.0`, and `webkit2gtk-4.1` resolve.
- Xvfb preflight screenshot succeeds.

## Package Proof

Run from `qsl/qsl-client/qsc-desktop`:

```bash
npm ci
npm run build
npm run prepare:sidecar
npm run tauri:build
```

Expected:

- frontend build succeeds;
- release `qsc` sidecar is copied into the Tauri resource path;
- Tauri build emits a native package under the target bundle directory;
- artifact path, size, and file type are recorded under `/srv/qbuild/tmp/`.

## Screenshot Proof

Run the packaged app under Xvfb with Wayland unset and GTK forced to X11:

```bash
env -u WAYLAND_DISPLAY GDK_BACKEND=x11 \
  xvfb-run -a -s "-screen 0 1440x1000x24" \
  sh -lc '<launch packaged app, wait, scrot screenshot, stop app>'
```

Expected:

- screenshot is captured with `scrot`;
- screenshot is stored under the artifact directory;
- screenshot shows the native QSC Desktop Prototype window;
- proof is labeled native packaged-app screenshot, not browser-only screenshot;
- launch warnings, if any, are recorded without hiding a runtime blocker.

## Contract / Dependency Checks

Run from repo root:

```bash
cargo +stable test -p qsc --locked --test desktop_gui_contract_na0215b -- --test-threads=1
cargo +stable test -p qsc --locked --test qsp_protocol_gate -- --test-threads=1
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
```

Expected:

- desktop GUI contract tests pass;
- QSP protocol gate tests pass;
- `send_commit` passes;
- cargo audit passes;
- `rustls-webpki` resolves to the patched dependency level.

## Governance / Scope Validation

Run:

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allow 'docs/demo/**' \
  --allow 'docs/governance/evidence/NA-0258_native_desktop_package_screenshot_audit.md' \
  --allow 'tests/NA-0258_native_desktop_package_screenshot_testplan.md' \
  --allow 'DECISIONS.md' \
  --allow 'TRACEABILITY.md' \
  --allow 'docs/ops/ROLLING_OPERATIONS_JOURNAL.md'
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

Expected:

- READY_COUNT `1`, READY `NA-0258`.
- D-0483 exists once after implementation.
- D-0484 is absent before closeout.
- No duplicate decision IDs.
- No forbidden paths touched.
- No token/secret leakage detected.
- Public-safety remains required and green before PR.

## PR / Merge Expectations

- Branch: `na-0258-native-desktop-package-screenshot`.
- PR title: `NA-0258: add native desktop package screenshot proof`.
- PR body includes `Goals: G1, G4, G5`.
- Required checks pass normally.
- Merge uses merge commit only with `--match-head-commit`.
- No direct push, admin bypass, squash, rebase, public-safety weakening, or
  branch-protection exception.
- NA-0258 remains READY after the evidence PR until a separate closeout
  directive promotes exactly one successor.

## Post-Fix Hardening Review Checklist

- Correctness under stress: package and screenshot proof run from the packaged
  AppImage on the provisioned host; host prerequisites are explicit.
- Minimality: committed changes remain evidence/governance/testplan only unless
  a proven in-scope qsc-desktop readiness defect requires a bounded fix.
- Maintainability: repeat steps use documented qsc-desktop commands and store
  bulky artifacts outside the repository.
- Coverage quality: package build, packaged-app screenshot, desktop contract,
  QSP gate, send_commit, cargo audit, and scope/leak/link checks all exercise
  distinct failure surfaces.
- Cross-lane stability: Linux/macOS protected checks and public-safety remain
  unchanged; qsl-server, qsl-attachments, website, Cargo, and protocol lanes
  remain untouched.

## Related Evidence

- [Native desktop readiness](../docs/demo/NATIVE_DESKTOP_PACKAGE_SCREENSHOT_READINESS.md)
- [NA-0258 audit](../docs/governance/evidence/NA-0258_native_desktop_package_screenshot_audit.md)
