# NA-0263 Closeout and NA-0264 Restoration Test Plan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-10

## Objective

Close NA-0263 after the real two-host Tailscale client/relay proof merged and
post-merge public-safety completed green, then restore NA-0264 as the sole
READY successor for desktop/sidecar adversarial stress and error-surface
hardening.

## Protected Invariants

- Exactly one READY item exists after the closeout.
- NA-0263 is DONE.
- NA-0264 is READY.
- D-0496 remains present exactly once.
- D-0497 is added exactly once.
- Public-safety remains required and green.
- Demo and desktop evidence remains non-production.
- The real two-host Tailscale proof is not described as production hardening,
  qsl-server production proof, qsl-attachments production proof, or production
  desktop readiness.
- No protocol, runtime, crypto, demo implementation, qsl-server,
  qsl-attachments, qsc-desktop implementation, website, workflow, Cargo,
  branch-protection, or public-safety configuration change is made.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0263_closeout_restore_na0264_testplan.md`

## Forbidden Scope

- `.github/**`
- `scripts/**`
- `Cargo.toml`
- `Cargo.lock`
- `qsp/**`
- `qsc/**`
- `qsl/**`
- `qsl-client/**`
- `apps/**`
- `tools/**`
- `inputs/**`
- `formal/**`
- `qsc-desktop/**`
- `qsl-server/**`
- `qsl-attachments/**`
- `website/**`
- external website source
- runtime/protocol/crypto/demo/service code
- branch-protection settings
- public-safety/check configuration

## Queue Proof

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
```

Expected:

- `READY_COUNT 1`
- `READY NA-0264 Desktop / Sidecar Adversarial Stress and Error-Surface Hardening`
- `NA-0263 DONE Cross-Host / Private-Network Demo Stress Reproducibility`

## Decision Proof

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Expected:

- D-0496 exists once.
- D-0497 exists once.
- No duplicate decision IDs.

## Scope Proof

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed NEXT_ACTIONS.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0263_closeout_restore_na0264_testplan.md
```

Expected:

- Changed paths are limited to the allowed closeout files.
- No forbidden implementation, workflow, Cargo, branch-protection,
  public-safety, qsl-server, qsl-attachments, qsc-desktop, website, or
  external website path appears.

## Closeout Evidence

Expected closeout evidence recorded in `NEXT_ACTIONS.md`:

- PR #779 head and merge SHAs.
- D-0496 and D-0497.
- Artifact directory:
  `/srv/qbuild/tmp/NA-0263_cross_host_demo_stress_artifacts_20260511T025100Z/`.
- Proof mode: `real two-host Tailscale client/relay proof`.
- Remote host alias and non-secret IP.
- Post-merge public-safety success on the PR #779 merge.
- Full-suite timing expectation: this closeout is docs/governance-only, so
  NA-0262A cost-control is expected to skip heavy full-suite waits/jobs for
  the closeout main push.

## Link / Leak / Goal-Lint Expectations

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
goal-lint
```

Expected:

- Link check passes.
- Added-line leak scan reports zero secret findings.
- Goal-lint passes with standalone `Goals: G1, G4, G5` in the PR body.

## Dependency / Main Health Expectations

Run:

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
```

Expected:

- Cargo audit passes.
- `rustls-webpki` remains at the patched locked version.
- `send_commit` tests pass.

## CI Expectations

- Required protected checks pass normally.
- CodeQL neutral is acceptable only under the repository's existing acceptance
  basis.
- Public-safety remains required on `main`.
- This closeout is docs/governance-only, so NA-0262A cost-control is expected
  to skip heavy full-suite waits/jobs for the closeout main push.
