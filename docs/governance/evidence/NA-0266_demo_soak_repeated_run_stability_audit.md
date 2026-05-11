Goals: G1, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-11
Replaces: n/a
Superseded-By: n/a

# NA-0266 Demo Soak and Repeated-Run Stability Audit

Directive: QSL-DIR-2026-05-11-066 / NA-0266

## Objective

Add a bounded repeated-run/soak proof for the non-production public demo and
baseline adversarial stress harness, with clean per-run temp scopes, artifacts,
leak checks, panic checks, and no production-hardening claim.

## Starting Authority Proof

- Starting `origin/main`: `a7dbfb2f9e13`.
- PR #784 merged as `a7dbfb2f9e13`.
- PRs #783 through #761 and PR #708: verified merged.
- PR #750 and PR #722: verified closed and unmerged.
- Branch protection required the expected contexts, including
  `public-safety`; force pushes and deletions were disabled; admin enforcement
  was enabled.
- Queue proof before edits: `READY_COUNT 1`, sole READY `NA-0266`.
- Decision proof before edits: D-0501 existed once, D-0502 absent, duplicate
  decision count zero.

## Red-Main Recovery Gate

The initial PR #784 merge commit had red `public-safety` because the
`advisories` job failed while fetching the RustSec advisory database. The job
log showed an external advisory database IO/fetch failure and did not report a
vulnerability.

Local proof on the exact merge commit passed:

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
```

The failure was classified as transient external fetch. The failed public-ci
jobs were rerun once with:

```bash
gh run rerun 25675241453 --repo QuantumShieldLabs/qsl-protocol --failed
```

The rerun completed with `advisories` success and `public-safety` success
before NA-0266 implementation began.

## Implementation Summary

- Added `scripts/ci/demo_soak_repeated_run.sh`.
- Added `docs/demo/DEMO_SOAK_REPEATED_RUN_STABILITY.md`.
- Added this audit.
- Added `tests/NA-0266_demo_soak_repeated_run_stability_testplan.md`.
- Added D-0502 and traceability links.
- Added a rolling operations journal entry for the directive.

No qshield CLI, protocol, crypto state-machine, qsl-server, qsl-attachments,
qsc-desktop implementation, website, workflow, branch-protection,
public-safety configuration, Cargo manifest, or Cargo lockfile path was changed.

## Helper Behavior

The helper:

- defaults to five runs and caps `DEMO_SOAK_RUNS` at ten;
- rejects invalid run counts and invalid max runtime values;
- creates a timestamped artifact root under `/srv/qbuild/tmp`;
- creates a distinct `run_<n>/tmp` temp scope for each run;
- runs `scripts/ci/demo_cli_smoke.sh` every iteration;
- runs `scripts/ci/demo_adversarial_stress.sh` with
  `DEMO_STRESS_PROFILE=baseline` every iteration;
- runs `scripts/ci/metadata_conformance_smoke.sh` once for the baseline profile
  or once per iteration for `metadata-each`;
- verifies expected child markers every run;
- scans artifacts for known token/secret/plaintext sentinels;
- scans artifacts for panic/backtrace/unwrap panic markers;
- writes `summary_matrix.tsv`, `run_state.tsv`, scan outputs, transcript, marker
  log, and `ARTIFACT_MANIFEST.txt`; and
- emits `NA0266_DEMO_SOAK_REPEATED_RUN_OK` only after all checks pass.

## Required Markers

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

## Counted Local Proof

The counted local proof command is:

```bash
DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh
```

Artifact directory:

```text
/srv/qbuild/tmp/NA-0266_demo_soak_repeated_run_artifacts_20260511T175224Z/
```

Run count:

```text
3
```

Total runtime:

```text
32 seconds
```

The local run emitted all required NA-0266 markers, including
`NA0266_SOAK_NO_STATE_BLEED_OK`, `NA0266_SOAK_NO_SECRET_LEAK_OK`,
`NA0266_SOAK_NO_PANIC_OK`, `NA0266_SOAK_ARTIFACT_MANIFEST_OK`, and
`NA0266_DEMO_SOAK_REPEATED_RUN_OK`.

## What Is Proven

- The demo smoke and baseline stress harness remain stable over repeated
  bounded local runs.
- Each run has a distinct artifact directory and temp scope.
- Metadata conformance passes under the selected profile.
- Required positive, negative, attachment, KT, leak, and panic markers remain
  present through the child scripts.
- No helper-scanned token/secret/plaintext sentinel leakage is present in the
  artifact bundle.
- No helper-scanned panic/backtrace/unwrap panic markers are present in the
  artifact bundle.
- The demo remains explicitly non-production.

## What Is Not Proven

- Production hardening.
- Production relay readiness.
- Public internet exposure safety.
- qsl-server production readiness.
- qsl-attachments production readiness.
- Production desktop readiness.
- Production KT deployment.
- Cross-host soak.
- Full relay-internal no-mutation beyond existing observable demo checks.

## Residual Gaps

- Advisories fetch resilience still depends on external RustSec/GitHub
  availability and needs a follow-on resilience lane.
- Cross-host/private-network soak remains a possible future evidence expansion.
- qsl-server and qsl-attachments production-hardening boundaries remain
  deliberately untouched by this lane.
