Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-09
Replaces: n/a
Superseded-By: n/a

# NA-0259 Closeout and NA-0260 Restoration Test Plan

## Objective

Close NA-0259 only after the KT-negative demo readiness proof has merged and
post-merge main public-safety is green, then restore exactly one successor:
NA-0260, attachment demo readiness and opaque-ciphertext fetch/decrypt proof.

## Protected Invariants

- Exactly one READY item exists after closeout.
- NA-0259 is DONE.
- NA-0260 is READY.
- D-0485 exists once before closeout.
- D-0486 exists once after closeout.
- Public-safety remains required and green.
- Closeout does not implement NA-0260.
- Demo remains non-production.
- Attachment demo successor preserves opaque-ciphertext boundaries.
- No protocol/crypto state-machine, qsl-server, qsl-attachments production,
  website, workflow, public-safety, branch-protection, Cargo, qsc-desktop,
  runtime/service, or production-readiness drift is introduced.

## Required Pre-Closeout Proof

Expected:

- PR #768 merged normally as `f32585f8d8f2`.
- PR #768 validated head was `df059e3fcba7`.
- Artifact directory exists outside the repo:
  `/srv/qbuild/tmp/NA-0259_kt_negative_demo_artifacts_20260510T002546Z/`.
- Post-merge main `public-safety`, `qsc-linux-full-suite`,
  `macos-qsc-full-serial`, and `qsc-adversarial-smoke` completed success on
  `f32585f8d8f2`.
- Queue proof before closeout: `READY_COUNT 1`, READY `NA-0259`.
- Decision proof before closeout: D-0485 once, D-0486 absent, no duplicates.

## Allowed Scope

- `NEXT_ACTIONS.md`.
- `DECISIONS.md`.
- `TRACEABILITY.md`.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.
- `tests/NA-0259_closeout_restore_na0260_testplan.md`.

## Forbidden Scope

Forbidden paths include `.github/**`, `scripts/**`, Cargo manifests/lockfiles,
qsp/protocol-core, qsc/qsl runtime implementation, apps, tools, inputs, formal,
qsc-desktop, qsl-server, qsl-attachments, website, external website,
runtime/protocol/crypto/demo/service code, branch-protection settings,
public-safety/check configuration, and NA-0260 implementation.

## NEXT_ACTIONS Expectations

After patch:

- NA-0259 status is DONE.
- NA-0259 implementation note records PR #768 head, merge, artifacts, proof
  mode, D-0485, D-0486, and post-merge public-safety evidence.
- NA-0260 exists as the sole READY item.
- NA-0260 scope is bounded to attachment demo evidence or explicit prerequisite
  stop.
- NA-0260 forbids crypto/state-machine changes and qsl-server/qsl-attachments
  production hardening unless separately authorized.

## Decision / Traceability Expectations

- DECISIONS.md contains D-0486 exactly once.
- D-0486 states that NA-0259 produced KT-negative demo readiness evidence.
- D-0486 states that NA-0260 addresses attachment demo readiness.
- D-0486 states that the demo remains non-production.
- TRACEABILITY.md links NA-0259 closeout and NA-0260 successor evidence.

## Local Validation Commands

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed 'NEXT_ACTIONS.md' \
  --allowed 'DECISIONS.md' \
  --allowed 'TRACEABILITY.md' \
  --allowed 'docs/ops/ROLLING_OPERATIONS_JOURNAL.md' \
  --allowed 'tests/NA-0259_closeout_restore_na0260_testplan.md'
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
```

Also run goal-lint using the established synthetic pull-request event body with
`Goals: G1, G3, G4, G5`.

Expected:

- READY_COUNT `1`, READY `NA-0260`.
- NA-0259 DONE.
- D-0486 exists once.
- D-0487 absent.
- no duplicate decision IDs.
- no forbidden paths touched.
- no token/secret leakage.

## PR / Merge Expectations

- Branch: `na-0259-closeout-restore-na0260`.
- PR title: `NA-0259: closeout and restore NA-0260`.
- PR body includes `Goals: G1, G3, G4, G5`.
- Required checks pass normally.
- Merge uses merge commit only with `--match-head-commit`.
- No direct push, admin bypass, squash, rebase, public-safety weakening,
  branch-protection exception, or check spoofing.

## Post-Merge Expectations

- `origin/main` advances to the closeout merge commit.
- Queue proof: READY_COUNT `1`, READY `NA-0260`, NA-0259 DONE.
- Decision proof: D-0486 once.
- Public-safety remains required and green on the final main commit.

## Post-Fix Hardening Review Checklist

- Correctness under stress: closeout depends on merged Packet B evidence and
  green post-merge main public-safety, not local-only proof.
- Minimality: changed paths are governance/testplan/journal only.
- Maintainability: NA-0260 successor block is explicit about allowed and
  forbidden surfaces.
- Coverage quality: parser, scope, leak, link, cargo audit, dependency tree,
  send_commit, and required CI checks cover distinct closeout risks.
- Cross-lane stability: Linux/macOS required checks remain green and no runtime,
  service, website, Cargo, workflow, or public-safety configuration path changes.
