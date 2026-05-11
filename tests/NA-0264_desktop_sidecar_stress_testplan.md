Goals: G1, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-11
Replaces: n/a
Superseded-By: n/a

# NA-0264 Desktop Sidecar Stress Test Plan

## Objective

Validate bounded desktop/sidecar adversarial stress proof for the
non-production `qsc-desktop` prototype, including bad sidecar paths, unavailable
sidecar, malformed sidecar output, invalid local store, relay unavailable,
protocol inactive, no secret leakage, no panic, and truthful package/screenshot
evidence where host prerequisites permit.

## Protected Invariants

- Desktop remains non-production.
- No production-ready desktop claim.
- No production sidecar, relay, qsl-server, or qsl-attachments claim.
- Fake sidecar fixtures are test-only.
- Sidecar failures do not leak passphrases, route tokens, or plaintext.
- GUI/desktop/sidecar error paths do not panic on adversarial inputs.
- Protocol inactive states fail closed.
- Unsupported categories are recorded and not faked.
- No protocol, wire, crypto, or state-machine change.
- No qsl-server or qsl-attachments production hardening.
- No website/external website, `.github`, branch-protection, public-safety,
  Cargo manifest, or Cargo lockfile change.

## Allowed Scope

- `qsl/qsl-client/qsc-desktop/**`
- `qsl/qsl-client/qsc/tests/**` only for desktop/sidecar contract tests
- `scripts/ci/**` only for a local bounded desktop stress helper
- `docs/demo/**`
- `docs/governance/evidence/NA-0264_desktop_sidecar_stress_audit.md`
- `tests/NA-0264_desktop_sidecar_stress_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- `.github/**`
- `Cargo.toml`
- `Cargo.lock`
- `qsp/**`
- qsc protocol-core or crypto state-machine implementation
- `qsl-server/**`
- `qsl-attachments/**`
- `website/**`
- external website repository
- branch-protection or public-safety configuration
- production service implementation

## Sidecar Error Tests

Run:

```bash
NA0264_ARTIFACT_DIR=/srv/qbuild/tmp/NA-0264_desktop_sidecar_stress_artifacts_20260511T104314Z \
  scripts/ci/desktop_sidecar_stress_na0264.sh
```

Expected sidecar markers:

- `NA0264_SIDECAR_MISSING_REJECT_OK`
- `NA0264_SIDECAR_BAD_PATH_REJECT_OK`
- `NA0264_SIDECAR_NONEXEC_REJECT_OK`
- `NA0264_SIDECAR_MALFORMED_OUTPUT_REJECT_OK`
- `NA0264_SIDECAR_NONZERO_REJECT_OK`
- `NA0264_SIDECAR_TIMEOUT_REJECT_OK`
- `NA0264_NO_SECRET_LEAK_OK`
- `NA0264_NO_PANIC_OK`
- `NA0264_DESKTOP_SIDECAR_STRESS_OK`

## Local Store / Relay / Protocol Tests

Expected markers:

- `NA0264_MISSING_STORE_REJECT_OK`
- `NA0264_INVALID_STORE_REJECT_OK`
- `NA0264_RELAY_UNAVAILABLE_REJECT_OK`
- `NA0264_PROTOCOL_INACTIVE_FAIL_CLOSED_OK`

The relay-unavailable proof is qsc sidecar contract proof, not production relay
or qsl-server production hardening.

## Package / Screenshot Proof

Run from `qsl/qsl-client/qsc-desktop`:

```bash
npm ci
npm run build
npm run prepare:sidecar
npm run tauri:build
```

Run the packaged AppImage under Xvfb with Wayland unset and GTK forced to X11.
Capture `xwininfo` and `scrot` output into the artifact directory.

Expected:

- AppImage emits under the release bundle directory.
- The package copy is stored under `/srv/qbuild/tmp/`.
- The counted screenshot is nonblank and shows the QSC Desktop Prototype UI.
- Screenshot proof is labeled as local AppImage/Xvfb evidence only.

## Contract / Dependency Checks

Run:

```bash
cargo test --manifest-path qsl/qsl-client/qsc-desktop/src-tauri/Cargo.toml --locked
cargo test -p qsc --locked --test desktop_gui_contract_na0215b -- --test-threads=1
cargo test -p qsc --locked --test qsp_protocol_gate -- --test-threads=1
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
```

If Rust source changes occurred, also run:

```bash
cargo fmt --check --manifest-path qsl/qsl-client/qsc-desktop/src-tauri/Cargo.toml
cargo build --manifest-path qsl/qsl-client/qsc-desktop/src-tauri/Cargo.toml --locked
cargo clippy --manifest-path qsl/qsl-client/qsc-desktop/src-tauri/Cargo.toml --locked -- -D warnings
```

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
  --allow 'qsl/qsl-client/qsc-desktop/**' \
  --allow 'scripts/ci/desktop_sidecar_stress_na0264.sh' \
  --allow 'docs/demo/**' \
  --allow 'docs/governance/evidence/NA-0264_desktop_sidecar_stress_audit.md' \
  --allow 'tests/NA-0264_desktop_sidecar_stress_testplan.md' \
  --allow 'DECISIONS.md' \
  --allow 'TRACEABILITY.md' \
  --allow 'docs/ops/ROLLING_OPERATIONS_JOURNAL.md'
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

Expected:

- READY_COUNT `1`, READY `NA-0264`.
- D-0498 exists once after implementation.
- D-0499 absent before closeout.
- No duplicate decision IDs.
- No forbidden paths touched.
- No token/secret/plaintext leaks detected.
- `public-safety` remains required and green before PR.

## CI / Public-Safety Expectations

- qsc-desktop/script/runtime-adjacent changes classify as runtime-critical under
  NA-0262A cost control.
- Required checks must pass normally.
- Merge must use a merge commit with `--match-head-commit`.
- No direct push, admin bypass, squash, rebase, branch-protection exception, or
  public-safety weakening.
- NA-0264 remains READY after the evidence PR until a separate closeout promotes
  exactly one successor.

## Post-Fix Hardening Review Checklist

- Correctness under stress: adversarial sidecar path/output/timeout conditions
  reject explicitly and boundedly.
- Minimality: changes are limited to desktop bridge error handling, local stress
  helper, and evidence/governance docs.
- Maintainability: helper uses local commands and stable markers; fake sidecars
  remain test fixtures only.
- Coverage quality: tests fail for sidecar missing/bad path, non-executable,
  malformed output, nonzero, timeout, invalid/missing store, relay unavailable,
  protocol inactive, no leak, and no panic.
- Cross-lane stability: Linux/macOS required checks and public-safety remain
  authoritative; qsl-server, qsl-attachments, website, Cargo, and protocol lanes
  remain untouched.

## Related Evidence

- [Desktop sidecar adversarial stress](../docs/demo/DESKTOP_SIDECAR_ADVERSARIAL_STRESS.md)
- [NA-0264 audit](../docs/governance/evidence/NA-0264_desktop_sidecar_stress_audit.md)
