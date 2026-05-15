Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-14
Replaces: n/a
Superseded-By: n/a

# NA-0289 External Review Package Refresh Audit

## Executive Summary

NA-0289 refreshes the public external review package and release readiness
evidence map after NA-0287 service production-gate mapping and NA-0288 metadata
phase-2 / external-review gap planning.

This is docs/governance evidence only. It does not implement protocol, crypto,
runtime, service, desktop, website, workflow, script, Cargo, dependency,
branch-protection, public-safety, production deployment, public internet
exposure, metadata phase-2 mitigations, or external review execution.

The refresh preserves conservative claim boundaries:

- external review completion remains `NOT_READY`;
- production readiness remains `NOT_READY`;
- public internet service readiness remains `NOT_READY`;
- metadata phase-2 completion remains `NOT_READY`;
- production backup/restore readiness remains `NOT_READY`;
- anonymity, metadata-free messaging, and untraceability remain unsupported
  claims.

## Baseline Documents Inspected

- `docs/public/EXTERNAL_REVIEW_PACKAGE.md`
- `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md`
- `docs/public/QSL_SERVER_ATTACHMENTS_PRODUCTION_BOUNDARY_PLAN.md`
- `docs/governance/evidence/NA-0287_service_production_gate_evidence_map.md`
- `docs/governance/evidence/NA-0288_metadata_phase2_external_review_gap_plan.md`
- `docs/governance/evidence/NA-0280_qsl_server_rate_global_cap_harness.md`
- `docs/governance/evidence/NA-0281_qsl_server_route_lifecycle_ttl_retention_harness.md`
- `docs/governance/evidence/NA-0282_qsl_attachments_retention_cleanup_recovery_harness.md`
- `docs/governance/evidence/NA-0283_qsl_attachments_disk_pressure_quota_abuse_harness.md`
- `docs/governance/evidence/NA-0284_qsl_attachments_capability_scope_abuse_logging_harness.md`
- `docs/governance/evidence/NA-0286_qsl_attachments_backup_restore_recovery_harness.md`
- `README.md`
- `START_HERE.md`
- `TRACEABILITY.md`
- `DECISIONS.md`
- `NEXT_ACTIONS.md`
- `formal/**`
- `inputs/**`
- `tools/refimpl/**`
- `tests/**`

## External Review Package Refresh Summary

The package already carried conservative public posture and NA-0288 gap-plan
language. NA-0289 refreshed it by:

- updating stale `public-safety` evidence from the PR #828-era main SHA to
  `origin/main` `28b2a98904e9` after PR #830;
- adding a package-refresh row that links D-0548, this audit, the testplan, the
  external review package, and the release evidence map;
- adding reviewer-ready checklist content that separates ready-to-review
  evidence from `NOT_READY` gates;
- adding expected reviewer outputs: accepted scope, findings, dispositions,
  and residual-risk / claim-wording feedback;
- adding recent PR evidence for the NA-0288 gap plan and closeout;
- converting the old "package needs refresh" gap into a stronger "reviewer
  findings and dispositions are not recorded" gap.

## Release Readiness Map Alignment Summary

The release map already preserved `NOT_READY` status for external review
completion and metadata phase-2. NA-0289 aligned it by:

- updating dependency and `public-safety` evidence to the NA-0289 start state;
- adding `DOCS_ONLY` and `FUTURE_GATE` meanings so planning/evidence-map rows do
  not look like completed readiness;
- adding explicit `NOT_READY` rows for public internet service readiness and
  production backup/restore readiness;
- adding an external-review-package-refresh row classified as `DOCS_ONLY`;
- adding claim-boundary rows for external review completion, public internet
  service readiness, and production backup/restore readiness;
- recording that NA-0289 does not change any `NOT_READY` gate.

## Claim-Boundary Scan Results

The changed public docs use prohibited phrases only as:

- explicitly negated statements;
- `NOT_READY` gate names;
- future-gate descriptions;
- prohibited wording / do-not-claim examples; or
- reviewer-scope boundaries.

No affirmative production-readiness, public-internet-readiness,
external-review-complete, anonymity, metadata-free, untraceable, quantum-proof,
or proven true Triple Ratchet claim was introduced.

## What Is Ready To Show Reviewers

- Governance spine and goal traceability.
- Canonical Suite-2 / SCKA specs and vector inventory.
- Bounded formal/model checks.
- KT verifier, SCKA, downgrade, skipped-key, and receive reject evidence.
- Non-production qshield demo evidence, KT-negative demo evidence, attachment
  demo evidence, desktop prototype evidence, clean-host reproduction evidence,
  and metadata conformance negatives.
- qsl-server and qsl-attachments local service-hardening evidence as
  production-gate input only.
- Metadata phase-2 gap classifications and residual metadata disclosures.

## What Remains NOT_READY

- External review completion.
- Reviewer scope acceptance, findings, dispositions, and residual-risk signoff.
- Production readiness.
- Public internet service readiness.
- Metadata phase-2 completion.
- Production backup/restore readiness.
- Production qsl-server or qsl-attachments operation.
- Production desktop release readiness.
- Anonymity, metadata-free messaging, and untraceability.
- Proven true Triple Ratchet or quantum-proof claims.

## What Changed

- `docs/public/EXTERNAL_REVIEW_PACKAGE.md` now references the current PR #830
  `public-safety` proof and includes an explicit reviewer checklist.
- `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md` now classifies NA-0289 as a
  `DOCS_ONLY` package refresh and keeps all stronger readiness gates
  `NOT_READY`.
- D-0548 records the decision boundary.
- `TRACEABILITY.md` links this audit and the NA-0289 testplan.
- `tests/NA-0289_external_review_package_refresh_testplan.md` records the
  validation expectations.

## What Did Not Change

- No protocol, wire, crypto, auth, negotiation, or state-machine semantics.
- No qsp protocol-core, qsc/qsl runtime, qsl-client, app, tool, input, formal,
  qsc-desktop, qsl-server, qsl-attachments, website, workflow, script, Cargo,
  dependency, branch-protection, or public-safety configuration behavior.
- No production readiness, public internet readiness, external review
  completion, anonymity, metadata-free, untraceable, quantum-proof, or proven
  true Triple Ratchet claim.
- No queue closeout; NA-0289 remains READY until a separate closeout packet
  runs after merge.

## No Implementation Changes

NA-0289 is limited to docs/governance evidence and public evidence-map
refreshes. It does not change executable implementation paths or service code.

## No External-Review-Complete Claim

The package refresh is not review completion. External review completion
requires later evidence recording reviewer scope, findings, dispositions,
residual risk, and updated claim boundaries.

## No Anonymity / Metadata-Free / Untraceable Claim

QSL remains a metadata-minimization project with explicit residual leakage.
Current evidence does not support anonymity, metadata-free messaging, or
untraceability.

## Next Recommended Lane

After NA-0289 merges and is closed out, the recommended successor is:

`NA-0290 - Metadata Phase-2 Identifier Rotation and Padding Defaults Design`

That lane should design identifier rotation / opaque-handle policy and
padding-default policy without implementation changes, anonymity claims,
metadata-free claims, untraceability claims, production-readiness claims, or
external-review-complete claims.
