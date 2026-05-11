Goals: G1, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-11
Replaces: n/a
Superseded-By: n/a

# NA-0266 Demo Soak and Repeated-Run Stability Testplan

## Objective

Validate a bounded repeated-run/soak helper for the non-production public demo
and baseline adversarial stress harness.

## Protected Invariants

- Runs are bounded.
- `DEMO_SOAK_RUNS` has a sane cap.
- Every iteration uses distinct temp and artifact directories.
- Demo smoke passes every iteration.
- Baseline adversarial stress passes every iteration.
- Metadata conformance passes at least once.
- No token/secret/plaintext sentinel leakage is accepted.
- No panic/backtrace/unwrap panic marker is accepted.
- The demo remains non-production.
- No production hardening claim is introduced.
- No protocol/crypto state-machine change is made.

## Allowed Scope

- `scripts/ci/demo_soak_repeated_run.sh`
- `docs/demo/DEMO_SOAK_REPEATED_RUN_STABILITY.md`
- `docs/governance/evidence/NA-0266_demo_soak_repeated_run_stability_audit.md`
- `tests/NA-0266_demo_soak_repeated_run_stability_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- `.github/**`
- `Cargo.toml`
- `Cargo.lock`
- `qsp/**`
- qsc/qsl protocol-core or crypto state-machine files
- `qsl-server/**`
- `qsl-attachments/**`
- qsc-desktop implementation
- `website/**`
- external website repo
- `tools/**` implementation changes
- `inputs/**`
- `formal/**`
- branch-protection settings
- public-safety/check configuration
- production service implementation

## Required Proof

Hard-start and preflight:

```bash
git status --porcelain=v1 --branch
git diff --name-only || true
git ls-files --others --exclude-standard || true
git fetch --all --prune
git rev-parse origin/main
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
gh api /repos/QuantumShieldLabs/qsl-protocol/branches/main/protection/required_status_checks
```

Red-main recovery gate:

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
```

Implementation validation:

```bash
git diff --check
bash -n scripts/ci/demo_soak_repeated_run.sh
DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh
scripts/ci/demo_cli_smoke.sh
DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh
scripts/ci/metadata_conformance_smoke.sh
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
python3 formal/run_model_checks.py
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

## Expected Soak Markers

```text
NA0266_SOAK_START
NA0266_SOAK_RUN_1_DEMO_OK
NA0266_SOAK_RUN_1_STRESS_OK
NA0266_SOAK_NO_STATE_BLEED_OK
NA0266_SOAK_NO_SECRET_LEAK_OK
NA0266_SOAK_NO_PANIC_OK
NA0266_SOAK_ARTIFACT_MANIFEST_OK
NA0266_DEMO_SOAK_REPEATED_RUN_OK
```

For `DEMO_SOAK_RUNS=3`, the run-specific demo and stress markers must exist
for runs 1, 2, and 3.

## Success Criteria

- Local soak proof exits zero.
- Required NA-0266 markers are present.
- `summary_matrix.tsv` records every run.
- `run_state.tsv` records unique per-run temp dirs.
- `ARTIFACT_MANIFEST.txt` is non-empty.
- Helper leak and panic scans pass.
- Queue helper reports `READY_COUNT 1`, READY `NA-0266`.
- D-0502 exists once and D-0503 is absent before closeout.
- Scope guard reports only allowed Packet C paths.
- Required PR checks and post-merge public-safety complete green.
