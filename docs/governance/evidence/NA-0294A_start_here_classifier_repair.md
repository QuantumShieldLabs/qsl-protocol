Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-15

# NA-0294A START_HERE Classifier Repair Evidence

## Executive Summary

NA-0294A repairs the CI scope classifier mismatch that treated `START_HERE.md` as runtime-critical. `START_HERE.md` is now treated as a docs/front-door governance file, matching README and other root governance docs.

The repair is deliberately narrow: runtime, workflow/script, Cargo, dependency, qsp/qsc/qsl, app/tool/input/formal, service, desktop, website, unknown, empty, and mixed docs+runtime scopes remain non-docs-only.

## Root Cause

`scripts/ci/classify_ci_scope.sh` explicitly allowed README and selected governance root files as docs paths but omitted `START_HERE.md`. That omission caused a README/START_HERE/docs/public governance bundle for NA-0294 to classify as `runtime_critical`.

## Changed Files

- `scripts/ci/classify_ci_scope.sh`
- `scripts/ci/public_safety_gate.py`
- `scripts/ci/qsl_evidence_helper.py`
- `docs/governance/evidence/NA-0294A_start_here_classifier_repair.md`
- `tests/NA-0294A_start_here_classifier_repair_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Classifier Positive Cases

Required positive cases:

```bash
bash scripts/ci/classify_ci_scope.sh START_HERE.md
bash scripts/ci/classify_ci_scope.sh README.md START_HERE.md docs/public/INDEX.md docs/public/RELEASE_READINESS_EVIDENCE_MAP.md docs/public/EXTERNAL_REVIEW_PACKAGE.md docs/governance/evidence/NA-0294_public_evidence_navigation_refresh_audit.md tests/NA-0294_public_evidence_navigation_refresh_testplan.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/public_safety_gate.py selftest-full-suite-cost-control
```

Expected result: docs-only classification for START_HERE and for the intended NA-0294 public navigation bundle.

## Classifier Negative Cases

Required negative cases:

```bash
bash scripts/ci/classify_ci_scope.sh
bash scripts/ci/classify_ci_scope.sh Cargo.toml
bash scripts/ci/classify_ci_scope.sh Cargo.lock
bash scripts/ci/classify_ci_scope.sh scripts/ci/classify_ci_scope.sh
bash scripts/ci/classify_ci_scope.sh .github/workflows/public-ci.yml
bash scripts/ci/classify_ci_scope.sh qsp/example
bash scripts/ci/classify_ci_scope.sh qsc/example
bash scripts/ci/classify_ci_scope.sh qsl/example
bash scripts/ci/classify_ci_scope.sh qsl-client/example
bash scripts/ci/classify_ci_scope.sh apps/example
bash scripts/ci/classify_ci_scope.sh tools/example
bash scripts/ci/classify_ci_scope.sh inputs/example
bash scripts/ci/classify_ci_scope.sh formal/example
bash scripts/ci/classify_ci_scope.sh qsl-server/example
bash scripts/ci/classify_ci_scope.sh qsl-attachments/example
bash scripts/ci/classify_ci_scope.sh qsc-desktop/example
bash scripts/ci/classify_ci_scope.sh website/example
```

Expected result: each case is non-docs-only, either `runtime_critical` or `workflow_security` as appropriate.

## Mixed-Path Fail-Closed Proof

Required mixed case:

```bash
bash scripts/ci/classify_ci_scope.sh README.md Cargo.toml
```

Expected result: non-docs-only `runtime_critical`.

## public-safety Behavior

The repair updates `public_safety_gate.py` selftest coverage for:

- `START_HERE.md` docs-only positive.
- Intended NA-0294 public navigation bundle docs-only positive.
- Existing runtime/workflow/Cargo/mixed/unknown/empty negatives unchanged.

No public-safety configuration, workflow, branch-protection, or required-context settings are changed.

## Scope Guard

The Packet A scope is limited to the classifier, helper consistency, evidence, testplan, decision, traceability, and rolling journal files. It does not edit `.github`, Cargo, runtime implementation, qsp/qsc/qsl implementation, qsl-server, qsl-attachments, qsc-desktop, website, README, START_HERE, or docs/public content.

## No Workflow or Branch-Protection Change

This repair changes no `.github/**` files and uses no branch-protection mutation. Existing required contexts, including `public-safety`, remain required.

## No Runtime or Implementation Change

This repair does not change protocol, cryptography, state machines, handshake/key schedule, QSP wire format, qsl runtime, qsc runtime, qsl-client runtime, qsl-server, qsl-attachments, qsc-desktop, apps, tools/refimpl, inputs, formal models, Cargo files, dependencies, website, or external website content.

## How This Unblocks NA-0294

After NA-0294A merges and is closed out, the intended NA-0294 README/START_HERE/docs/public/governance/testplan bundle can classify as docs-only. NA-0294 can then improve public evidence navigation without triggering the full runtime-critical path solely because `START_HERE.md` was omitted from root-doc classification.
