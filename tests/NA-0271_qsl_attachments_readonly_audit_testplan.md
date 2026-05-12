Goals: G1, G3, G4, G5

# NA-0271 qsl-attachments Read-Only Audit Testplan

## Objective

Verify that NA-0271 performs a read-only qsl-attachments code/security/ops
audit and designs the first executable qsl-attachments hardening test harness
without changing qsl-attachments implementation, qsl-server implementation,
protocol/crypto semantics, website source, workflows, scripts, Cargo files,
branch protection, or public-safety configuration.

## Protected Invariants

- Exactly one READY item remains during the audit/design PR: NA-0271.
- D-0512 exists once after the audit/design patch.
- D-0513 remains absent in the audit/design PR.
- No qsl-attachments implementation changes.
- No qsl-server implementation changes.
- No qsp protocol-core, qsc/qsl runtime, qsc-desktop, website, workflow,
  branch-protection, public-safety, Cargo, dependency, protocol, wire, crypto,
  auth, or state-machine changes.
- No production readiness claim.
- The opaque-ciphertext boundary remains explicit.
- Known qsl-attachments gaps remain explicit.
- Proven bugs, evidence gaps, recommendations, and non-issues remain separated.

## Allowed/Forbidden Scope

Allowed paths:

- `docs/governance/evidence/NA-0271_qsl_attachments_readonly_audit_test_harness_design.md`
- `tests/NA-0271_qsl_attachments_readonly_audit_testplan.md`
- `docs/public/QSL_SERVER_ATTACHMENTS_PRODUCTION_BOUNDARY_PLAN.md` only for
  handoff/reference updates that do not change claims
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` only if consistent

Forbidden paths:

- `qsl-attachments/**`
- `qsl-server/**`
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
- `qsc-desktop/**`
- `website/**`
- runtime, protocol, crypto, service, desktop, public-safety, or branch
  protection configuration paths

## Read-Only qsl-attachments Audit Proof

The audit must record:

- sibling repo path, status, HEAD, branch, and remotes;
- build system/language;
- service role and opaque-ciphertext boundary;
- API routes, request/response shapes, status codes, and error names;
- capability/auth model;
- descriptor/decrypt-context separation;
- fetch/decrypt/integrity boundary;
- retention/cleanup model;
- quota/disk-pressure model;
- logging/no-secret model;
- storage/recovery model;
- deployment/network assumptions;
- existing tests and docs;
- production-boundary gaps.

The sibling repo must remain clean after read-only inspection.

## Test-Harness Design Requirements

The design must include future executable coverage for:

- capability/auth tests: missing capability, wrong capability, wrong resource,
  replayed/stale capability, abuse escalation, and no capability logging;
- descriptor tests: malformed descriptor, tampered descriptor, wrong resource
  descriptor, and descriptor metadata boundary;
- opaque ciphertext tests: opaque store/fetch, no service decrypt, no plaintext
  logging, and client-side tamper handling;
- fetch/decrypt/integrity tests: valid fetch/decrypt path, tamper reject,
  missing object, duplicate fetch, unauthorized fetch, and range behavior;
- retention/cleanup tests: expiry, cleanup, delete/abort, and no stale object
  after cleanup;
- quota/disk tests: max size, deployment-global quota, disk-full simulation,
  and no unbounded growth;
- logging tests: capability absent, descriptor secrets absent, ciphertext and
  plaintext absent, and sanitized error output;
- restart/recovery tests: journal replay, partial/corrupt journal, missing
  file, and restart behavior aligned to the durability contract;
- config/startup tests: missing env, invalid root path, invalid quota, invalid
  bind, and safe bind config;
- health/ops tests: health endpoint if present, or explicit observability gap
  if absent;
- soak/stress tests: bounded upload/fetch bursts, cleanup during load, no
  panic, and no unbounded growth.

## Proven Bug Recording

The audit must explicitly investigate and record concrete qsl-attachments bugs
only when file/line evidence or behavior evidence supports the finding.
Evidence gaps and recommendations must not be overstated as proven bugs.

Known area requiring concrete review:

- malformed JSON/body-shape rejection versus canonical `reason_code` taxonomy;
- startup panic/config error surface;
- capability abuse tests;
- expiry/cleanup tests;
- disk-pressure tests;
- restart/recovery tests;
- secret-safe logs;
- opaque ciphertext boundary proof.

## Opaque-Ciphertext Boundary

The audit/design must preserve:

- qsl-attachments stores and returns ciphertext bytes only;
- qsl-attachments does not parse plaintext attachment content;
- qsl-attachments does not store or log `enc_ctx_*`, content keys, nonce
  prefixes, filenames, media types, or decrypt context;
- clients own descriptor authentication, fetched ciphertext integrity, decrypt,
  plaintext shape checks, local plaintext persistence, and confirmation.

## Overclaim Scan

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

## Link/Leak/Goal-Lint Expectations

- Relative markdown links must resolve.
- Added-line leak scan must not report secrets, auth headers, route tokens,
  secret-bearing URLs, or long secret-like hex dumps.
- Goal metadata must include G1, G3, G4, and G5 where required.
- PR body must include the exact standalone `Goals: G1, G3, G4, G5` line near
  the top.

## CI Expectations

Local validation should include:

- `git status --porcelain=v1 --branch`
- `git diff --name-only origin/main...HEAD`
- `git diff --stat origin/main...HEAD`
- `git diff --check`
- direct overclaim phrase scan
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

Required CI must pass normally before merge. The expected PR scope is docs,
governance, public handoff, and testplan only, so docs-only cost control may
skip full suites where policy allows.

## Successor Handoff

After NA-0271 merges and closeout is authorized separately, the next successor
must preserve exactly one READY item and must not implement qsl-attachments or
qsl-server behavior inside the closeout. Future implementation lanes should use
the audit/design document as the first qsl-attachments harness backlog.
