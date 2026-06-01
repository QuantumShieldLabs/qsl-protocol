Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-01

# NA-0397 QSL Code / Crypto Research Watch and Audit Follow-Up Plan Testplan

Goals: G1, G2, G3, G4, G5

Directive: QSL-DIR-2026-06-01-217

## Objective

Validate that NA-0397 records a qsl-protocol code/crypto research-watch and
audit-follow-up plan without performing remediation, changing runtime behavior,
changing cryptography, changing dependencies, mutating sibling repositories,
publishing public docs, changing backup configuration, mutating response
archives, or expanding public claims.

## Protected Invariants

- READY_COUNT remains exactly one.
- READY remains NA-0397 until closeout.
- NA-0396 is DONE.
- D-0774 exists once.
- D-0775 exists once.
- D-0776 exists once after this PR.
- D-0777 is absent before closeout.
- No runtime, service, protocol, crypto, dependency, workflow, public docs,
  website, backup, response archive, qsl-server, qsl-attachments, qshield
  runtime, qstart/qresume, or secret-bearing path is changed.
- No code/crypto research-watch finding automatically promotes READY.
- No production, public-internet, metadata-free, anonymity, untraceable,
  vulnerability-free, bug-free, perfect-crypto, compliance, certification,
  side-channel-free, constant-time-guaranteed, audit-complete, or
  external-review-complete claim is introduced.

## Allowed Scope

- `docs/governance/evidence/NA-0397_qsl_code_crypto_research_watch_audit_follow_up_plan.md`
- `tests/NA-0397_qsl_code_crypto_research_watch_audit_follow_up_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

Forbidden changes include `.github/**`, workflows, `Cargo.toml`, `Cargo.lock`,
runtime/protocol/crypto implementation paths, qsc/qsp/qsl implementation,
qshield runtime, qsl-server, qsl-attachments, qsc-desktop, website, public docs,
README, START_HERE, backup scripts/timers/fstab/services, durable code/crypto
audit reports outside governance evidence, response archives, request/directive
history, qstart/qresume tooling, helper script mutations, and secret handling.

## NA-0396 Inheritance Requirements

Verify the evidence:

- Records PR #1055 and PR #1056 dependency.
- Preserves that NA-0396 selected NA-0397.
- Records D-0774 and D-0775 presence.
- Records cargo audit green and `rustls-webpki v0.103.13`.
- Preserves that advisory-health evidence is not bug-free or
  vulnerability-free proof.
- Carries forward the future Project Goal / Operating Principles canon lane as
  a future candidate only.

## Official Research Source Verification Requirements

Verify official source categories are cited for:

- IACR ePrint.
- Real World Crypto.
- USENIX Security.
- IEEE Symposium on Security and Privacy.
- ACM CCS.
- NDSS.
- CHES / TCHES.
- CRYPTO / Eurocrypt.
- arXiv with PREPRINT classification.
- fuzz/property/formal testing official project sources where used.
- Rust unsafe/security guidance where used.
- prior NA-0380 code/crypto audit report if locally present.

## Source Citation Requirements

Each citation must include:

- source title or source category.
- publisher/authority.
- URL or local path for local audit evidence.
- access date.
- source tier.
- source classification.
- relevance to QSL.
- audit-follow-up implication.
- public-claim implication.

## Prior Audit Intake Requirements

Verify the evidence records:

- NA-0380 overall report presence or absence.
- NA-0380 code/crypto report presence or absence.
- checksum status if reports are present.
- carried-forward findings.
- evidence gaps.
- what cannot be concluded.

## Read-Only Scan Requirements

Verify read-only scans cover:

- qsc/qsp/qsl protocol/crypto surfaces.
- qshield-cli and qshield demo boundaries.
- formal models.
- refimpl/oracle/vector surfaces.
- metadata runtime harnesses.
- tests and scripts relevant to crypto/protocol planning.

Search hits must be treated as planning evidence only, not proof of a bug.

## Crypto API Misuse Follow-Up Requirements

Verify the plan covers:

- crypto API misuse.
- unsafe API combinations.
- key material boundaries.
- public/private API separation.
- misuse-resistant defaults.
- test-only and demo-only boundaries.
- error behavior and sanitized error boundaries.

## Nonce / Key / RNG Follow-Up Requirements

Verify the plan covers:

- nonce generation and uniqueness.
- RNG source and test boundaries.
- deterministic test fixtures.
- key generation.
- key derivation.
- transcript/key schedule binding.
- key erasure or zeroization if applicable.
- state mutation on reject and fail-closed behavior.

## KEM / Signature / Key Schedule Follow-Up Requirements

Verify the plan covers:

- ML-KEM and KEM integration boundaries.
- ML-DSA and signature integration boundaries.
- key schedule.
- transcript binding.
- suite-id/domain separation.
- downgrade/cross-suite confusion.
- formal model coverage alignment.
- vector/oracle coverage.

## Panic / Unwrap / Fail-Closed Follow-Up Requirements

Verify the plan covers:

- production-path panic/unwrap/expect.
- panic-demo utilities.
- test-only unwraps.
- refimpl/test-boundary assumptions.
- fail-closed behavior.
- error propagation.
- no mutation on rejected input.

## Unsafe / Memory Safety Follow-Up Requirements

Verify the plan covers:

- unsafe usage if any.
- FFI boundary if any.
- memory-safety assumptions.
- zeroization/secret-memory boundary if applicable.
- dependency-driven unsafe assumptions.

## Side-Channel / Timing Follow-Up Requirements

Verify the plan covers:

- constant-time assumptions.
- secret-dependent branches.
- timing side-channel limits.
- traffic/timing/metadata distinctions.
- side-channel claims not made.
- future external review need.

## Fuzz / Property / Vector Follow-Up Requirements

Verify the plan covers:

- fuzz targets.
- property tests.
- differential tests.
- oracle/refimpl checks.
- KAT/conformance vectors.
- malformed input vectors.
- negative/fail-closed vectors.
- long-running suite boundaries.

## Formal / Model Alignment Follow-Up Requirements

Verify the plan covers:

- qsc formal models.
- implementation-to-model correspondence.
- model assumptions.
- bounded model limits.
- qshield demo/model separation.
- protocol state-machine coverage.
- cross-suite/domain separation.

## Dependency Duplication Follow-Up Requirements

Verify the plan covers:

- duplicate crypto dependency families.
- dependency tree review.
- RustSec/GHSA/NVD linkage.
- cargo audit relation.
- version skew.
- transitive crypto crates.
- NA-0396 dependency policy alignment.

## Demo / Refimpl / Service Boundary Follow-Up Requirements

Verify the plan covers:

- qshield demo/non-production boundary.
- qshield-cli boundary.
- reference implementation/oracle boundary.
- deterministic refimpl assumptions.
- qsl-server service-local evidence boundary.
- qsl-attachments service-local evidence boundary.
- public-internet/production claim boundary.

## Candidate Grouping Requirements

Verify the evidence groups future candidates:

- QSL Crypto API Misuse and Boundary Audit Plan.
- QSL Nonce / Key / RNG Lifecycle Audit Plan.
- QSL KEM / Signature / Key Schedule / Transcript Binding Audit Plan.
- QSL Panic / Unwrap / Fail-Closed Runtime Boundary Audit Plan.
- QSL Unsafe / Memory Safety / FFI Boundary Audit Plan.
- QSL Side-Channel / Timing / Secret-Dependent Behavior Audit Plan.
- QSL Fuzz / Property / Differential / Vector Expansion Plan.
- QSL Formal Model / Implementation Alignment Plan.
- QSL Dependency Duplication / Crypto Dependency Family Review Plan.
- QSL Demo / Refimpl / Service Boundary Audit Plan.

Each group must include evidence basis, urgency, likely allowed scope,
forbidden scope, queue recommendation, and claim-boundary implication.

## No Implementation / Remediation Requirements

Verify NA-0397 does not:

- implement code fixes.
- change crypto.
- change runtime.
- change dependencies.
- change Cargo files.
- change workflows.
- mutate qsl-server.
- mutate qsl-attachments.
- publish public docs.
- create a durable audit report outside governance evidence.

## Claim Boundary Requirements

Verify the evidence states:

- code/crypto research watch is not code audit completion.
- code/crypto planning is not remediation.
- code/crypto planning is not external review.
- cargo audit green is not bug-free proof.
- formal checks are not full implementation proof.
- no production/public-internet readiness claim.
- no metadata-free/anonymity/untraceable claim.
- no bug-free/perfect-crypto/vulnerability-free claim.

## Public Paper Boundary Requirements

Verify public technical position paper work remains future-gated and is not
started by NA-0397.

## Successor Selection Requirements

Verify exact successor selection:

`NA-0398 -- QSL Metadata Privacy / Secure Messaging Claim Boundary Plan`

Alternate successors must remain rejected unless the evidence shows an
immediate code/crypto blocker or source/evidence gap.

## Future Project Goal Canon Carry-Forward Requirements

Verify the future Project Goal / Operating Principles canon lane is carried
forward as a future governance candidate only and does not override NA-0397 or
the selected NA-0398 successor.

## Backup-Impact Requirements

Verify the evidence:

- records no backup-plan update is required for governance/testplan-only
  changes.
- records future durable code/crypto audit reports require separate backup
  impact review.
- does not call same-host continuity complete disaster recovery.

## Required Local Checks

Required checks include:

- `python3 scripts/ci/qsl_routine_audit_cadence.py --help`
- representative routine audit cadence fixture tests.
- `python3 scripts/ci/qsl_response_history_catalog.py --help`
- representative response history catalog fixture tests.
- `python3 scripts/ci/qsl_codex_response_writer.py --help`
- representative response writer fixture tests.
- `python3 scripts/ci/qsl_bounded_check_poll.py --help`
- representative bounded poll fixture tests.
- `python3 scripts/ci/qsl_directive_manifest_validate.py --help`
- representative manifest validator fixture tests.
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- metadata runtime JSON parse checks.
- targeted qsc send_commit.
- qsc NA-0313 harness if feasible.
- formal model checks.
- qshield-cli build/test if feasible.
- queue and decisions helper checks.
- scope guard.
- link-check.
- leak-scan.
- classifier or changed-path proof.
- goal-lint.

## CI Expectations

Before merge, required qsl-protocol PR checks must attach and pass normally.
`public-safety` must remain required and green. No admin bypass, direct push,
squash, rebase, force-push, amend, or branch deletion is permitted.

## Successor Handoff

After the NA-0397 planning PR merges and post-merge public-safety is green,
closeout may restore the selected NA-0398 successor as READY without
implementing NA-0398.
