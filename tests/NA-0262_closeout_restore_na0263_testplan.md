# NA-0262 Closeout and NA-0263 Restoration Test Plan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-10

## Objective

Close NA-0262 after the local demo adversarial stress harness merged and
post-merge public-safety completed green, then restore NA-0263 as the sole
READY successor for cross-host/private-network demo stress reproducibility.

## Protected Invariants

- Exactly one READY item exists after the closeout.
- NA-0262 is DONE.
- NA-0263 is READY.
- D-0494 remains present exactly once.
- D-0495 is added exactly once.
- Public-safety remains required and green.
- Demo evidence remains non-production.
- Local demo stress evidence is not described as production hardening.
- No protocol, runtime, crypto, demo implementation, qsl-server,
  qsl-attachments, qsc-desktop, website, workflow, Cargo, branch-protection,
  or public-safety configuration change is made.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0262_closeout_restore_na0263_testplan.md`

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
- `READY NA-0263 Cross-Host / Private-Network Demo Stress Reproducibility`
- `NA-0262 DONE Demo Adversarial Stress, Chaos, and Abuse Testing Harness`

## Decision Proof

Run the decision helper and exact parser:

```bash
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Expected:

- D-0494 exists once.
- D-0495 exists once.
- No duplicate decision IDs.

## Scope Proof

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed NEXT_ACTIONS.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0262_closeout_restore_na0263_testplan.md
```

Expected:

- Changed paths are limited to the allowed closeout files.
- No forbidden implementation, workflow, Cargo, branch-protection,
  public-safety, qsl-server, qsl-attachments, qsc-desktop, website, or
  external website path appears.

## Closeout Evidence

Expected closeout evidence recorded in `NEXT_ACTIONS.md`:

- PR #777 head and merge SHAs.
- D-0494 and D-0495.
- Artifact directory:
  `/srv/qbuild/tmp/NA-0262_demo_adversarial_stress_artifacts_20260510T213151Z/`.
- Post-merge public-safety success on the PR #777 merge.
- Full-suite timing expectation: this was a script/demo change, so
  `qsc-linux-full-suite`, `macos-qsc-full-serial`, and
  `qsc-adversarial-smoke` ran and passed.

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
