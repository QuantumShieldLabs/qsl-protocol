Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-04

# NA-0248 Suite-2 Triple-Ratchet Claim Boundary Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0248 adds a docs-only evidence-backed Suite-2 / Triple-Ratchet-style claim boundary without changing protocol, runtime, crypto, demo, service, website, CI, branch-protection, public-safety, or Cargo behavior.

## Protected Invariant

Public Suite-2 / Triple-Ratchet wording must not outrun repo evidence. The only safe short wording established by this lane is:

```text
Suite-2 is a research-stage Triple-Ratchet-style hybrid messaging design.
```

Unsupported production-ready, proven true Triple Ratchet, quantum-proof, metadata-free, anonymity, and production deployment ready claims must remain prohibited.

## Scope Guard

Allowed changed paths:

- `docs/public/SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md`
- `docs/governance/evidence/NA-0248_suite2_triple_ratchet_evidence_audit.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `tests/NA-0248_suite2_triple_ratchet_claim_boundary_testplan.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden changed paths include `.github/**`, scripts, Cargo metadata, qsp, qsc/qsl/qsl-client implementation paths, apps, tools/refimpl, tools/actors, inputs, qsc-desktop, qsl-server, qsl-attachments, website, runtime/protocol/crypto/demo/service code, branch-protection settings, and public-safety/check configuration.

## External-Source Check

Use only authorized external terminology sources:

- Signal Double Ratchet / Triple Ratchet specification.
- Signal ML-KEM Braid / SCKA specification.
- NIST FIPS 203 only for ML-KEM terminology.

Acceptance:

- external content is paraphrased;
- links and retrieval timestamps are recorded in the evidence audit;
- external sources are not used as QSL implementation proof.

## Repo Evidence-Map Check

The claim-boundary document and evidence audit must consult and map:

- `GOALS.md`
- `ROADMAP.md`
- `docs/conformance/CONFORMANCE_VECTOR_PRIORITIZATION.md`
- `docs/demo/DEMO_ACCEPTANCE_CRITERIA.md`
- `docs/public/WEBSITE_CLAIM_MATRIX.md`
- `docs/public/WEBSITE_UPDATE_PLAN.md`
- `docs/privacy/DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md`
- `docs/privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md`
- TRACEABILITY entries for PR #708, #727, #729, #731, #734, #736, #740, and #742.
- DECISIONS entries D-0440 through D-0461.
- PR summaries for #708, #727, #729, #731, #734, #736, #740, and #742.

## Claim-Boundary Document Validation

`docs/public/SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md` must contain:

- Goals line and classification header.
- Executive summary.
- Safe short wording.
- Unsafe wording examples.
- External definition summary.
- QSL evidence map.
- Current classification for SUPPORTED, PARTIALLY_SUPPORTED, and UNSUPPORTED claims.
- Public wording rules.
- Release-readiness gaps.
- Website/public-copy implications.
- Repo and external references.

## No Implementation Change Proof

Run:

```bash
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check origin/main...HEAD
```

Acceptance:

- changed paths are exactly inside the NA-0248 allowlist;
- no runtime/protocol/crypto/demo/service/website implementation files are changed;
- no `.github`, scripts, Cargo, qsc/qsl/qsl-client, apps, tools, inputs, qsc-desktop, qsl-server, or qsl-attachments files are changed.

## Queue Parser Expectation

The canonical queue parser must report:

```text
READY_COUNT 1
READY NA-0248 Suite-2 Triple-Ratchet Evidence and Claim Boundary
```

NA-0248 remains READY because Packet A does not edit `NEXT_ACTIONS.md`.

## Decision Parser Expectation

The canonical decision parser must report:

- D-0110 exists once.
- D-0439 through D-0462 exist once each.
- D-0463 is absent.
- No duplicate decision IDs exist.

## CI Expectations

Required local validation:

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 tools/goal_lint.py
```

Also run:

- canonical queue parser;
- canonical decision parser;
- markdown inventory and link validation runbook;
- leak/secret scan if established.

PR acceptance:

- all required CI contexts attach and pass normally;
- `public-safety` remains required and green;
- no admin bypass, direct push, check spoofing, branch-protection exception, squash merge, or rebase merge is used.
