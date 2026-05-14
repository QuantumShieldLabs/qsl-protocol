Goals: G1, G2, G3, G4, G5

# NA-0288 Metadata Phase-2 and External Review Gap Testplan

## Objective

Verify that NA-0288 produces a planning-only metadata phase-2 and external
review readiness gap plan without implementation changes or public-claim
overreach.

## Protected Invariants

- Metadata phase-2 gaps remain explicit.
- External review completion remains not proven.
- No anonymity claim is introduced.
- No metadata-free claim is introduced.
- No untraceable claim is introduced.
- No production-readiness or public-internet-readiness claim is introduced.
- qsl-protocol implementation paths remain untouched.
- qsl-server and qsl-attachments implementation paths remain untouched.
- public-safety remains required and green.

## Allowed / Forbidden Scope

Allowed paths:

- `docs/governance/evidence/NA-0288_metadata_phase2_external_review_gap_plan.md`
- `tests/NA-0288_metadata_phase2_external_review_gap_testplan.md`
- `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md`
- `docs/public/EXTERNAL_REVIEW_PACKAGE.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden paths:

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
- runtime, protocol, crypto, demo, service, workflow, public-safety, branch
  protection, and dependency paths.

## Metadata Inventory Requirements

The gap plan must classify metadata evidence into:

- `PROVEN_EXECUTABLE`
- `DOCS_ONLY`
- `NOT_READY`
- `FUTURE_GATE`
- `OUT_OF_SCOPE`

It must identify:

- executable demo metadata proof;
- service metadata/logging proof boundaries;
- docs-only threat-model/design text;
- explicitly prohibited metadata claims;
- remaining phase-2 gaps;
- evidence needed before public claims change.

## External Review Inventory Requirements

The gap plan must identify:

- reviewer package elements that are ready to show;
- stale or partial package elements;
- missing reviewer evidence;
- external review `NOT_READY` items;
- claim boundaries;
- the next package-refresh needs.

## Claim-Boundary Requirements

The plan and any public-doc updates must preserve explicit boundaries for:

- no anonymity claim;
- no metadata-free claim;
- no untraceable claim;
- no external-review-complete claim;
- no production-readiness claim;
- no public-internet-readiness claim;
- no proven true Triple Ratchet claim beyond already bounded terminology.

## No Implementation Proof

NA-0288 is successful only if the diff is docs/governance/testplan evidence and
does not touch implementation paths, workflow paths, scripts, Cargo metadata,
dependencies, service code, desktop code, website source, or protocol/crypto
state-machine files.

## No Production-Readiness Claim

Any phrase such as `production-ready`, `deployment-ready`, `production relay
ready`, `production attachment ready`, or `public internet ready` must appear
only as explicitly negated, prohibited, or future/unproven wording.

## No External-Review-Complete Claim

Any phrase such as `external review complete`, `externally reviewed`, or
`review complete` must appear only as explicitly negated, prohibited, or
future/unproven wording.

## No Anonymity / Metadata-Free / Untraceable Claim

Any phrase such as `metadata-free`, `anonymity`, `anonymous messaging`, or
`untraceable` must appear only as explicitly negated, prohibited, or
future/unproven wording.

## Link / Leak / Goal-Lint Expectations

Validation must include:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed docs/governance/evidence/NA-0288_metadata_phase2_external_review_gap_plan.md \
  --allowed tests/NA-0288_metadata_phase2_external_review_gap_testplan.md \
  --allowed docs/public/RELEASE_READINESS_EVIDENCE_MAP.md \
  --allowed docs/public/EXTERNAL_REVIEW_PACKAGE.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

The PR body must include a standalone `Goals: G1, G2, G3, G4, G5` line for
goal-lint.

## CI Expectations

Required local validation:

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
```

Required CI must pass normally. Since this is docs/governance-only work, the
NA-0262A cost-control path may skip heavyweight full suites when the path
classifier proves the scope is docs-only.

## Successor Handoff

After NA-0288 merges and post-merge public-safety is green, the recommended
successor is external review package refresh and claim-boundary alignment. That
successor must not implement metadata phase-2, claim external review
completion, or upgrade production/public claims without separate evidence.
