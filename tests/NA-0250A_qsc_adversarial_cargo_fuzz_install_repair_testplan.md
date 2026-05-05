Goals: G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-05
Replaces: n/a
Superseded-By: n/a

# NA-0250A qsc-adversarial cargo-fuzz Install Repair Testplan

## Objective

Validate that NA-0250A repairs the qsc-adversarial-smoke cargo-fuzz install path on current GitHub runners while preserving adversarial smoke enforcement and public-safety posture.

## Protected Invariant

qsc-adversarial-smoke must remain enforced, cargo-fuzz must remain version-pinned, fuzz targets must still run through `scripts/ci/qsc_adversarial.sh`, public-safety must remain required, and no protocol/runtime/service behavior may change.

## Scope Guard

Allowed changed paths:

- `.github/workflows/qsc-adversarial.yml`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0250A_qsc_adversarial_cargo_fuzz_install_repair_testplan.md`

Forbidden changes include `Cargo.toml`, `Cargo.lock`, `.github/workflows/public-ci.yml`, `scripts/**`, qsc/qsl/app/runtime/test paths, tools/refimpl, inputs, formal, qsc-desktop, qsl-server, qsl-attachments, website, public-safety helper/configuration, and branch-protection settings.

## Failure Evidence

Required evidence before applying the repair:

- `public-safety` on `origin/main` commit `98c631a5dc18` is failed.
- `qsc-adversarial-smoke` attempt 2 is failed.
- `Install cargo-fuzz` ran `cargo install cargo-fuzz --locked --version 0.13.1`.
- The install resolved rustix v0.36.5 and failed with reserved `rustc_*` attribute errors.
- `Run qsc adversarial smoke` was skipped only because cargo-fuzz installation failed.
- `qsc-linux-full-suite` is green on the same SHA.

## Install Repair Proof

Local isolated proof should run:

```bash
TMP="$(mktemp -d)"
CARGO_HOME="$TMP/cargo-home" CARGO_TARGET_DIR="$TMP/target" cargo +nightly install cargo-fuzz --locked --version 0.13.1 --root "$TMP/root"
```

Expected current result: failure with rustix v0.36.5 reserved `rustc_*` attribute errors.

Fallback proof should run:

```bash
TMP="$(mktemp -d)"
CARGO_HOME="$TMP/cargo-home" CARGO_TARGET_DIR="$TMP/target" cargo +nightly install cargo-fuzz --version 0.13.1 --root "$TMP/root"
"$TMP/root/bin/cargo-fuzz" --version
rm -rf "$TMP"
```

Expected result: `cargo-fuzz 0.13.1`.

## qsc-adversarial-smoke Proof

The repaired workflow must retain:

- job name `qsc-adversarial-smoke`;
- the `Install cargo-fuzz` step;
- a pinned `cargo-fuzz --version 0.13.1` install;
- the `Run qsc adversarial smoke` step;
- `sh scripts/ci/qsc_adversarial.sh`.

The repair PR CI must show `qsc-adversarial-smoke` reaches and runs `scripts/ci/qsc_adversarial.sh`.

## No Weakening / No Skip Proof

Validate with:

```bash
rg -n "cargo-fuzz|qsc_adversarial|qsc-adversarial-smoke|skip|continue-on-error" .github/workflows/qsc-adversarial.yml
```

Expected result:

- cargo-fuzz remains pinned to `0.13.1`;
- `scripts/ci/qsc_adversarial.sh` remains present;
- no `continue-on-error` is added;
- no skip condition is added for qsc-adversarial-smoke;
- public-safety workflow/configuration is untouched.

## Queue Parser Expectation

Run the canonical queue parser.

Expected result after NA-0250A repair:

- `READY_COUNT 1`
- `READY NA-0250 External Review and Release-Readiness Evidence Package`
- `NA-0250` remains READY pending closeout.

NA-0250A must not edit `NEXT_ACTIONS.md`.

## Decision Parser Expectation

Run the canonical decision parser.

Expected result after NA-0250A repair:

- D-0110 exists once.
- D-0439 through D-0467 exist once each.
- D-0468 is absent.
- duplicate decision count is zero.

## CI Expectations

Required CI and relevant qsc-adversarial CI must attach and pass normally before merge:

- `ci-4a`
- `ci-4b`
- `ci-4c`
- `ci-4d`
- `ci-4d-dur`
- `demo-cli-build`
- `demo-cli-smoke`
- `formal-scka-model`
- `goal-lint`
- `metadata-conformance-smoke`
- `suite2-vectors`
- `CodeQL`
- `macos-qsc-qshield-build`
- `public-safety`
- `qsc-adversarial-smoke`

`public-safety` must remain required. No branch-protection exception, admin bypass, direct push, squash merge, or rebase merge is allowed.

## Local Validation Bundle

Run:

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check
python3 - <<'PY'
import yaml
from pathlib import Path
yaml.safe_load(Path(".github/workflows/qsc-adversarial.yml").read_text())
PY
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
```

Also run goal-lint, the canonical queue parser, the canonical decision parser, markdown inventory/link validation, leak-safe added-line scan, and branch-protection required-check proof.
