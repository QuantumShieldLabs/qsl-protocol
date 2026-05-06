Goals: G1, G3, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-06
Replaces: n/a
Superseded-By: n/a

# NA-0251 Website Implementation Handoff Test Plan

## Objective

Validate that NA-0251 creates a qsl-protocol handoff package for later external website implementation without editing the external website repository, qsl-protocol website source, or any protocol/runtime/crypto/demo/service implementation path.

## Protected Invariant

The handoff must preserve evidence-boundary public copy:

- no production-readiness overclaim
- no "proven true Triple Ratchet" overclaim
- no quantum-proof overclaim
- no anonymity or metadata-elimination overclaim
- no external-product conflation with QSL protocol evidence
- public-safety remains required and green
- NA-0251 remains READY until a separate closeout packet

## Scope Guard

Allowed changed paths for Packet A:

- `docs/public/WEBSITE_IMPLEMENTATION_HANDOFF.md`
- `docs/governance/evidence/NA-0251_website_implementation_handoff_audit.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `tests/NA-0251_website_implementation_handoff_testplan.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden changed paths include website source, external website repo content, `.github/**`, `scripts/**`, Cargo files, qsp/qsc/qsl/qsl-client/apps/tools/inputs/formal paths, qsc-desktop, qsl-server, qsl-attachments, public-safety configuration, branch-protection settings, and protocol/runtime/crypto/demo/service code.

Validation commands:

```bash
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check
```

Expected result: changed paths are exactly Packet A allowed paths, with no forbidden-path match.

## Handoff Document Validation

Required checks:

- `docs/public/WEBSITE_IMPLEMENTATION_HANDOFF.md` exists.
- It includes a Goals line.
- It states the document is a handoff package only.
- It explicitly states no external website repo edits and no qsl-protocol website implementation edits.
- It lists source-of-truth artifacts.
- It includes page-by-page checklist coverage for homepage, protocol status, demo/GUI, metadata/privacy, Suite-2 / Triple-Ratchet wording, external product separation, healthcare/PQC consulting, evidence links, and roadmap/release readiness.
- It includes safe copy snippets.
- It includes prohibited copy snippets.
- It includes evidence link map, external repo checklist, suggested website PR title/body, future directive template, static scan recommendations, rollback/verification checklist, and known uncertainties.

Suggested command:

```bash
rg -n "handoff package only|Page-By-Page Checklist|Safe Copy Snippets|Prohibited Copy Snippets|Evidence Link Map|Future External Website Directive Template|Static Overclaim Phrase Scan|Rollback / Verification|Known Uncertainties" docs/public/WEBSITE_IMPLEMENTATION_HANDOFF.md
```

## Evidence Map Validation

Required sources should be referenced:

- `WEBSITE_CLAIM_MATRIX.md`
- `WEBSITE_UPDATE_PLAN.md`
- `SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md`
- `EXTERNAL_REVIEW_PACKAGE.md`
- `RELEASE_READINESS_EVIDENCE_MAP.md`
- `DEMO_ACCEPTANCE_CRITERIA.md`
- `CONFORMANCE_VECTOR_PRIORITIZATION.md`
- `DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md`
- `DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md`
- `NA-0245_website_truthfulness_audit.md`
- `NA-0250_external_review_release_readiness_audit.md`
- `GOALS.md`
- `ROADMAP.md`
- `TRACEABILITY.md`
- `DECISIONS.md`

Suggested command:

```bash
rg -n "WEBSITE_CLAIM_MATRIX|WEBSITE_UPDATE_PLAN|SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY|EXTERNAL_REVIEW_PACKAGE|RELEASE_READINESS_EVIDENCE_MAP|DEMO_ACCEPTANCE_CRITERIA|CONFORMANCE_VECTOR_PRIORITIZATION|DOC-G5-001|DOC-G5-003|NA-0245_website_truthfulness_audit|NA-0250_external_review_release_readiness_audit|GOALS.md|ROADMAP.md|TRACEABILITY.md|DECISIONS.md" docs/public/WEBSITE_IMPLEMENTATION_HANDOFF.md docs/governance/evidence/NA-0251_website_implementation_handoff_audit.md
```

## No Website / External Repo Change Proof

Validation command:

```bash
git diff --name-only origin/main...HEAD | rg '^(website/|\\.github/|scripts/|Cargo\\.toml|Cargo\\.lock|qsp/|qsc/|qsl/|qsl-client/|apps/|tools/|inputs/|formal/|qsc-desktop/|qsl-server/|qsl-attachments/)' || true
```

Expected result: no output.

Evidence text must state:

- no external website repo edits
- no qsl-protocol website implementation changes
- no protocol/runtime/crypto/demo/service changes
- no public-safety, branch-protection, `.github`, scripts, or Cargo changes

## Queue Parser Expectation

Run the canonical queue parser.

Expected result:

- `READY_COUNT 1`
- `READY NA-0251 Public Website Evidence-Boundary Implementation Handoff`
- NA-0251 remains `READY`
- NA-0250 through NA-0237 remain `DONE`

## Decision Parser Expectation

Run the canonical decision parser.

Expected result after Packet A:

- D-0110 exists once
- D-0439 through D-0469 exist once each
- D-0470 is absent
- duplicate count is zero

## CI Expectations

Local validation bundle:

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
scripts/ci/demo_cli_smoke.sh
scripts/ci/metadata_conformance_smoke.sh
```

Expected result: all pass.

Required PR checks:

- ci-4a
- ci-4b
- ci-4c
- ci-4d
- ci-4d-dur
- demo-cli-build
- demo-cli-smoke
- formal-scka-model
- goal-lint
- metadata-conformance-smoke
- suite2-vectors
- CodeQL
- macos-qsc-qshield-build
- public-safety

Expected result: required checks pass normally before merge. CodeQL may be accepted only under the repository's existing neutral/skipped policy if GitHub reports it that way for this PR.
