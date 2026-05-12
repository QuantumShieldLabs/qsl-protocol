# NA-0269 Server / Attachment Production-Boundary Plan Testplan

Goals: G1, G3, G4, G5

## Objective

Verify that NA-0269 produces a planning-only production-boundary hardening plan
for qsl-server and qsl-attachments, preserves current demo-only evidence
boundaries, and avoids implementation or public-claim drift.

## Protected invariants

- Exactly one READY item remains during the planning PR: NA-0269.
- D-0508 exists once after the planning patch.
- D-0509 remains absent in the planning PR.
- No qsl-server implementation changes.
- No qsl-attachments implementation changes.
- No qsp protocol-core, qsc/qsl runtime, qsc-desktop, website, workflow,
  branch-protection, public-safety, Cargo, dependency, protocol, wire, crypto,
  auth, or state-machine changes.
- Demo evidence remains bounded and non-production.
- Known service gaps remain explicit.

## Allowed scope

- `docs/public/**`
- `docs/demo/**`
- `docs/governance/evidence/NA-0269_server_attachment_production_boundary_plan_audit.md`
- `tests/NA-0269_server_attachment_production_boundary_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- `qsl-server/**`
- `qsl-attachments/**`
- `qsp/**`
- `qsc/**`
- `qsl/**`
- `qsl-client/**`
- `apps/**`
- `scripts/**`
- `.github/**`
- `Cargo.toml`
- `Cargo.lock`
- `tools/**`
- `inputs/**`
- `formal/**`
- `website/**`
- runtime, protocol, crypto, service, desktop, public-safety, or branch
  protection configuration paths

## Inventory requirements

Planning evidence must record:

- qsl-server repo/path/status, role, evidence found, and gaps.
- qsl-attachments repo/path/status, role, evidence found, and gaps.
- Missing sibling repo notes for `/home/victor/work/qsl` paths, if missing.
- qsl-protocol demo relay, attachment demo, opaque-ciphertext, private-network,
  metadata, and production-boundary evidence.

## Plan requirements

The production-boundary plan must include:

- Executive summary.
- Current evidence baseline.
- qsl-server role and current boundary.
- qsl-attachments role and current boundary.
- What current demo evidence proves.
- What current demo evidence does not prove.
- Threat model categories.
- Hardening domains.
- Required production gates.
- Non-goals.
- Acceptance criteria for future production-capable claims.
- Recommended NA sequence after planning.
- Safe public wording.
- Prohibited public wording.
- Rollback/stop conditions for future implementation.

## Overclaim scan

Scan added/changed docs for:

- `production-ready`
- `deployment-ready`
- `production relay ready`
- `production attachment ready`
- `qsl-server production ready`
- `qsl-attachments production ready`
- `external review complete`
- `metadata-free`
- `anonymity`
- `untraceable`
- `quantum-proof`
- `proven true Triple Ratchet`

Allowed uses must be explicitly negated, listed as future/unproven, or placed
inside prohibited wording sections. No heading may turn these phrases into an
affirmative claim.

## Link, leak, and goal-lint expectations

- Relative markdown links must resolve.
- Added-line leak scan must not report secrets, auth headers, route tokens,
  secret-bearing URLs, or long secret-like hex dumps.
- Goal metadata must include G1, G3, G4, and G5 where required.
- PR body must include the exact standalone `Goals: G1, G3, G4, G5` line near
  the top.

## CI expectations

Local validation should include:

- `git diff --check`
- overclaim scan over changed docs
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/run_model_checks.py`
- goal-lint

Required CI must pass normally before merge. The expected PR scope is
docs/governance/testplan only, so docs-only cost control may skip full suites
where policy allows.
