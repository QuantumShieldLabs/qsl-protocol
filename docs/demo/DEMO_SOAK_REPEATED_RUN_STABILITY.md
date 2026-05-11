Goals: G1, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-11
Replaces: n/a
Superseded-By: n/a

# Demo Soak and Repeated-Run Stability

## Purpose

NA-0266 adds a bounded repeated-run proof for the non-production public demo
and adversarial stress harness. The proof repeats the existing local demo smoke
and baseline stress checks with isolated per-run temp directories, per-run
artifact bundles, leak scans, panic scans, and a summary matrix.

This is not production-hardening evidence. It does not prove production relay
readiness, public internet exposure safety, qsl-server readiness,
qsl-attachments readiness, production desktop readiness, KT deployment
readiness, or release approval.

## Command

Default bounded run:

```bash
scripts/ci/demo_soak_repeated_run.sh
```

The default profile is:

```text
DEMO_SOAK_RUNS=5
DEMO_SOAK_PROFILE=baseline
DEMO_SOAK_MAX_RUNTIME_S=2700
```

The validation profile used for local CI-style proof may reduce the run count:

```bash
DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh
```

The helper accepts:

```text
DEMO_SOAK_RUNS
DEMO_SOAK_PROFILE
DEMO_SOAK_ARTIFACT_DIR
DEMO_SOAK_MAX_RUNTIME_S
```

Supported profiles:

```text
baseline
metadata-each
```

`baseline` runs `metadata_conformance_smoke.sh` once after all demo/stress
iterations. `metadata-each` runs it once per iteration. Both profiles run
`demo_cli_smoke.sh` and `demo_adversarial_stress.sh` baseline in every
iteration.

## Bounds

- `DEMO_SOAK_RUNS` defaults to `5`.
- `DEMO_SOAK_RUNS` is capped at `10`.
- `DEMO_SOAK_MAX_RUNTIME_S` defaults to `2700` seconds.
- The helper rejects invalid bounds and uses `timeout` for child commands.
- The stress child also keeps its own bounded runtime.
- The helper targets loopback-only child scripts and does not contact public
  internet targets for demo traffic.

## Artifacts

Default artifact path:

```text
/srv/qbuild/tmp/NA-0266_demo_soak_repeated_run_artifacts_<timestamp>/
```

Important files:

- `demo_soak_repeated_run_transcript.log`
- `demo_soak_repeated_run_markers.log`
- `summary_matrix.tsv`
- `run_state.tsv`
- `leak_scan.txt`
- `panic_scan.txt`
- `ARTIFACT_MANIFEST.txt`
- `run_<n>/demo_cli_smoke.log`
- `run_<n>/demo_adversarial_stress.log`
- `run_<n>/demo_adversarial_stress/`
- `metadata_once/metadata_conformance_smoke.log` for the baseline profile

The artifact directory must be absent or empty before the helper starts.

## Expected Markers

The helper emits:

```text
NA0266_SOAK_START
NA0266_SOAK_RUN_<n>_DEMO_OK
NA0266_SOAK_RUN_<n>_STRESS_OK
NA0266_SOAK_NO_STATE_BLEED_OK
NA0266_SOAK_NO_SECRET_LEAK_OK
NA0266_SOAK_NO_PANIC_OK
NA0266_SOAK_ARTIFACT_MANIFEST_OK
NA0266_DEMO_SOAK_REPEATED_RUN_OK
```

Each run also requires child proof markers from the existing scripts, including
`DEMO_ACCEPTANCE_OK`, `DEMO_POSITIVE_SEND_RECEIVE_DECRYPT_OK`,
`DEMO_NO_SECRET_LEAK_OK`, `DEMO_STRESS_NO_SECRET_LEAK_OK`,
`DEMO_STRESS_NO_PANIC_OK`, and `NA0262_DEMO_ADVERSARIAL_STRESS_OK`.

## What Is Proven

- The current non-production qshield demo smoke can pass repeatedly.
- The current baseline adversarial stress harness can pass repeatedly.
- Each run uses a distinct artifact directory and distinct temp directory.
- Metadata conformance still passes under the selected profile.
- Required child markers are present every run.
- The artifact bundle does not contain known relay-token or sentinel leakage
  patterns checked by the helper.
- The artifact bundle does not contain panic, backtrace, or unwrap panic
  markers checked by the helper.
- A manifest and summary matrix identify exactly what ran.

## What Is Not Proven

- Production readiness.
- Production relay readiness.
- Public internet exposure safety.
- qsl-server production hardening.
- qsl-attachments production hardening.
- Production desktop packaging or release readiness.
- Production KT service operation.
- Full relay-internal no-mutation beyond the existing observable checks.
- A fully cold dependency fetch.

## State-Bleed Boundary

The helper proves state separation by creating a unique per-run artifact
directory and a unique per-run `TMPDIR` before invoking the child scripts. The
child scripts create and clean their own demo stores inside that temp scope.
The helper verifies the temp scopes are unique and records them in
`run_state.tsv`.

The proof does not inspect private in-process relay internals. Where the demo
already exposes observable no-mutation checks, those remain covered by the
child smoke and stress scripts.

## Known Gaps

- Cross-host/private-network soak is not included in this helper.
- Remote thin-client proof remains a separate optional evidence mode.
- qsl-server and qsl-attachments production-hardening lanes remain separate.
- The helper does not add fuzzing, public internet testing, or unbounded soak.
