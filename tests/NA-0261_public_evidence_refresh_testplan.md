Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-10
Replaces: n/a
Superseded-By: n/a

# NA-0261 Public Evidence Refresh Test Plan

## Objective

Verify that public and demo evidence summaries reflect NA-0259 KT-negative demo
proof and NA-0260 attachment demo proof without introducing production
readiness overclaims or implementation drift.

## Protected Invariants

- Public evidence remains truthful and evidence-bound.
- Demo evidence remains explicitly non-production.
- KT-negative proof is demo-only and verifier-backed.
- Attachment proof is demo-only and preserves the opaque-ciphertext boundary.
- Known production and release gaps remain visible.
- `public-safety` remains required and green.

## Allowed Scope

- `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md`
- `docs/public/EXTERNAL_REVIEW_PACKAGE.md`
- `docs/public/WEBSITE_IMPLEMENTATION_HANDOFF.md`
- `docs/demo/PUBLIC_DEMO_TOUCH_AND_FEEL_READINESS.md`
- `docs/demo/CROSS_HOST_PUBLIC_DEMO_REPRODUCIBILITY.md`
- `docs/demo/KT_NEGATIVE_PUBLIC_DEMO_READINESS.md`
- `docs/demo/ATTACHMENT_PUBLIC_DEMO_READINESS.md`
- `docs/governance/evidence/NA-0261_public_evidence_refresh_audit.md`
- `tests/NA-0261_public_evidence_refresh_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` if used for operational evidence.

## Forbidden Scope

- `.github/**`, `scripts/**`, `Cargo.toml`, `Cargo.lock`, `qsp/**`, `qsc/**`,
  `qsl/**`, `qsl-client/**`, `apps/**`, `tools/**`, `inputs/**`, `formal/**`,
  `qsc-desktop/**`, `qsl-server/**`, `qsl-attachments/**`, `website/**`, any
  external website repository, runtime/protocol/crypto/demo/service code,
  branch-protection settings, and public-safety/check configuration.

## Stale KT Wording Proof

- Search refreshed docs for stale claims that KT-negative demo readiness is
  still open.
- Expected result: remaining KT gaps are production KT deployment, live qshield
  KT evidence ingestion, and cross-host KT-negative behavior, not the local
  NA-0259 demo-runner verifier proof.

## Stale Attachment Wording Proof

- Search refreshed docs for stale claims that attachment demo readiness is
  still open.
- Expected result: remaining attachment gaps are production attachment
  readiness, qsl-server/qsl-attachments hardening, and
  cross-host/private-network attachment proof, not the local NA-0260 qshield
  descriptor/fetch/decrypt proof.

## No Production-Overclaim Proof

- Run direct phrase scans for high-risk overclaims.
- Expected result: any matches are confined to prohibited-copy lists,
  "do-not-claim" sections, or boundary statements, not positive claims.

## Link / Leak / Goal-Lint Expectations

- `python3 scripts/ci/qsl_evidence_helper.py link-check` passes.
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main` passes.
- Goal-lint passes for the Packet B PR body with a standalone `Goals: G1, G3, G4, G5` line.
- Queue parser reports exactly one READY item, `NA-0261`.
- Decision parser reports D-0489 once, D-0490 absent, and no duplicate IDs.

## CI Expectations

- Required checks pass normally.
- `public-safety` remains required and green before PR creation, on the Packet B
  PR, and after merge.
- CodeQL neutral/skipped outcomes are acceptable only when branch protection and
  workflow policy accept them.
- No branch-protection exception, admin bypass, squash merge, rebase merge, or
  direct push is used.
