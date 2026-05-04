Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-04
Replaces: n/a
Superseded-By: n/a

# NA-0250 External Review Release-Readiness Testplan

## Objective

Validate that NA-0250 creates a docs-only external-review and release-readiness evidence package that is truthful, reproducible, and bounded to current repository evidence.

## Protected Invariant

The package must not approve production release, claim a proven true Triple Ratchet, claim anonymity or metadata elimination, hide release gaps, or change protocol/runtime/crypto/demo/service behavior.

## Scope Guard

Allowed changed paths:

- `docs/public/EXTERNAL_REVIEW_PACKAGE.md`
- `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md`
- `docs/governance/evidence/NA-0250_external_review_release_readiness_audit.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `tests/NA-0250_external_review_release_readiness_testplan.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden changes:

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
- runtime, protocol, crypto, demo, or service implementation code
- branch-protection or public-safety helper/configuration changes

## External Review Package Validation

The package must include:

- Goals line.
- Status and owner metadata.
- Executive summary.
- Current posture: research-stage, not production-ready, not anonymity/metadata-free, demo/non-production.
- What is proven.
- What is not proven.
- Reproducible commands.
- Evidence artifact index.
- Recent PR table for #708, #727, #729, #731, #734, #736, #740, #742, #744, and #746.
- Review questions.
- Known gaps and recommended next work.
- Safe public wording and prohibited wording.

## Release-Readiness Map Validation

The evidence map must include:

- G1 through G5 matrix using `PROVEN`, `PARTIAL`, or `NOT_READY`.
- Release-readiness gate checklist.
- CI evidence map.
- Local reproduction map.
- Claim boundary map.
- Demo, GUI, and website readiness map.
- Metadata/privacy readiness map.
- Formal verification readiness map.
- External review readiness map.
- `Do not claim yet` section.

## Command Proof Validation

The audit must record exact commands and pass/fail summaries for:

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/run_model_checks.py`
- `scripts/ci/demo_cli_smoke.sh`
- `scripts/ci/metadata_conformance_smoke.sh`

If a command is infeasible in a future rerun, the audit must state why and must not pretend local proof exists.

## No Implementation Change Proof

Validate with:

```bash
git diff --name-only origin/main...HEAD
```

Expected result: only allowed docs/governance/testplan/journal paths are changed. No implementation, workflow, script, Cargo, website, service, qsc, qsl-client, formal, inputs, or tools paths are changed.

## Queue Parser Expectation

Run the canonical queue parser.

Expected result before and after Packet A:

- `READY_COUNT 1`
- `READY NA-0250 External Review and Release-Readiness Evidence Package`

Packet A must not edit `NEXT_ACTIONS.md`.

## Decision Parser Expectation

Run the canonical decision parser.

Expected result after Packet A:

- D-0110 exists once.
- D-0439 through D-0466 exist once each.
- D-0467 is absent.
- duplicate decision count is zero.

## CI Expectations

Required CI must attach and pass normally before merge:

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

`public-safety` must remain required and green before work, on the PR, and after merge.

## Local Validation Bundle

Run:

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
scripts/ci/demo_cli_smoke.sh
scripts/ci/metadata_conformance_smoke.sh
python3 tools/goal_lint.py
```

Also run the repository markdown inventory/link validation and added-line leak-safe scan using the established local patterns.
