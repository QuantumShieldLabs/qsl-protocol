Goals: G1, G2, G3, G4, G5

# NA-0289 External Review Package Refresh Testplan

## Objective

Verify that NA-0289 refreshes the external review package and release readiness
evidence map after NA-0287/NA-0288 without implementation changes or public
claim overreach.

## Protected Invariants

- External review completion remains `NOT_READY`.
- Production readiness remains `NOT_READY`.
- Public internet service readiness remains `NOT_READY`.
- Metadata phase-2 completion remains `NOT_READY`.
- Production backup/restore readiness remains `NOT_READY`.
- No anonymity claim is introduced.
- No metadata-free claim is introduced.
- No untraceable claim is introduced.
- All readiness gaps remain explicit.
- public-safety remains required and green.

## Allowed / Forbidden Scope

Allowed paths:

- `docs/public/EXTERNAL_REVIEW_PACKAGE.md`
- `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md`
- `docs/governance/evidence/NA-0289_external_review_package_refresh_audit.md`
- `tests/NA-0289_external_review_package_refresh_testplan.md`
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
  protection, dependency, and external website paths.

## External Review Package Checks

The package must:

- reference NA-0287 service production-gate evidence where safe;
- reference NA-0288 metadata phase-2 and external-review gap planning;
- reference NA-0289 audit/testplan evidence;
- use current `origin/main` / `public-safety` proof;
- include reviewer-ready scope/checklist information;
- keep reviewer findings, dispositions, and external review completion separate
  from package existence.

## Release Evidence Map Checks

The release map must:

- align dependency and `public-safety` proof to current main;
- classify package refresh as `DOCS_ONLY`;
- preserve `NOT_READY` for production readiness, public internet service
  readiness, metadata phase-2 completion, external review completion,
  anonymity/metadata-free/untraceable claims, and production backup/restore
  readiness;
- not mark release readiness complete unless the evidence actually supports it.

## Claim-Boundary Checks

The following phrases may appear only as explicitly negated, prohibited,
`NOT_READY`, future-gate, or unproven wording:

- production-ready
- deployment-ready
- production relay ready
- production attachment ready
- public internet ready
- external review complete
- externally reviewed
- metadata-free
- anonymity
- anonymous messaging
- untraceable
- quantum-proof
- proven true Triple Ratchet
- review complete
- release ready
- ready for production

## Link / Leak / Goal-Lint Expectations

Validation must include:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed docs/public/EXTERNAL_REVIEW_PACKAGE.md \
  --allowed docs/public/RELEASE_READINESS_EVIDENCE_MAP.md \
  --allowed docs/governance/evidence/NA-0289_external_review_package_refresh_audit.md \
  --allowed tests/NA-0289_external_review_package_refresh_testplan.md \
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

NA-0289 must remain READY after this refresh PR until a separate closeout
promotes exactly one successor. The recommended successor is NA-0290 metadata
phase-2 identifier rotation and padding defaults design, without implementing
NA-0290 inside this refresh lane.
