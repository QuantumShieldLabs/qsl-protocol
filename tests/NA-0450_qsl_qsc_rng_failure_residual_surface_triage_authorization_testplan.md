Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-09

# NA-0450 QSL qsc RNG Failure Residual Surface Triage Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Validate the NA-0450 governance-only residual qsc RNG failure triage lane.

This testplan verifies that NA-0450 consumes NA-0449 bounded evidence, classifies
residual route/contact/attachment and provider-dependent RNG surfaces, selects
an exact successor, and preserves no-runtime/no-crypto/no-dependency/no-public
claim boundaries.

## Required markers

- `NA0450_QWORK_PROOF_FILES_VERIFIED_OK`
- `NA0450_NA0449_INHERITANCE_CONSUMED_OK`
- `NA0450_RESIDUAL_QSC_SURFACES_TRIAGED_OK`
- `NA0450_ROUTE_CONTACT_ATTACHMENT_SCOPE_SELECTED_OK`
- `NA0450_PROVIDER_BOUNDARY_BACKLOG_OK`
- `NA0450_FORMAL_FUZZ_VECTOR_BACKLOG_OK`
- `NA0450_NO_RUNTIME_CHANGE_OK`
- `NA0450_NO_DEPENDENCY_CHANGE_OK`
- `NA0450_NO_WORKFLOW_CHANGE_OK`
- `NA0450_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0450_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0450_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0450_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0450_ONE_READY_INVARIANT_OK`

## qwork proof-file gate

Verify Codex did not run `qwork`, `qstart`, or `qresume`.

Verify these files were read:

- `/srv/qbuild/work/NA-0450/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0450/.qwork/startup.qsl-protocol.json`

Required result:

- `.kv` proof has `startup_result=OK`.
- JSON proof parses and mirrors the required `.kv` keys.
- proof `HEAD` equals live `HEAD` before fetch.
- proof `origin/main` equals live `origin/main` before fetch.
- fetch does not advance `origin/main` beyond the qwork proof.
- `NA0450_QWORK_PROOF_FILES_VERIFIED_OK`.

## Queue and decision proof

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Required result before patch:

- `READY_COUNT 1`.
- READY item is NA-0450.
- latest decision is D-0886.
- D-0887 is absent.
- duplicate decision count is zero.

Required result after patch:

- `READY_COUNT 1`.
- READY item remains NA-0450 before optional closeout.
- D-0887 exists once.
- D-0888 is absent before optional closeout.
- duplicate decision count is zero.
- `NA0450_ONE_READY_INVARIANT_OK`.

## Scope guard

Changed paths must be exactly:

- `docs/governance/evidence/NA-0450_qsl_qsc_rng_failure_residual_surface_triage_authorization_plan.md`
- `tests/NA-0450_qsl_qsc_rng_failure_residual_surface_triage_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No runtime, crypto, dependency, Cargo manifest, lockfile, workflow, executable
test, fuzz target, vector, formal model, qsl-server, qsl-attachments, qshield
runtime, qshield-cli, website, public doc, README, START_HERE, qwork,
qstart, qresume, qshell, qsl-backup, backup status, backup plan, rollback, or
backup tree path may be changed.

Required markers:

- `NA0450_NO_RUNTIME_CHANGE_OK`
- `NA0450_NO_DEPENDENCY_CHANGE_OK`
- `NA0450_NO_WORKFLOW_CHANGE_OK`

## Residual surface classification check

Verify the evidence doc includes:

- route/default-route/relay token RNG residual;
- contact RNG residual;
- attachment ID/CEK/nonce-prefix RNG residual;
- qsc provider-dependent RNG residual;
- refimpl provider RNG residual;
- qshield-cli demo RNG residual;
- formal/model RNG residual;
- fuzz/vector RNG residual.

Required classifications:

- route/contact/attachment: `QSC_ROUTE_CONTACT_ATTACHMENT_RNG_SCOPE_NEXT`;
- provider-dependent: `QSC_PROVIDER_DEPENDENT_RNG_BACKLOG`;
- formal: `RNG_RESIDUAL_FORMAL_BACKLOG`;
- fuzz: `RNG_RESIDUAL_FUZZ_BACKLOG`;
- vector: `RNG_RESIDUAL_VECTOR_BACKLOG`.

Required markers:

- `NA0450_RESIDUAL_QSC_SURFACES_TRIAGED_OK`
- `NA0450_ROUTE_CONTACT_ATTACHMENT_SCOPE_SELECTED_OK`
- `NA0450_PROVIDER_BOUNDARY_BACKLOG_OK`
- `NA0450_FORMAL_FUZZ_VECTOR_BACKLOG_OK`

## Inherited qsc proof

Run:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
```

Required result:

- cfg-gated RNG failure test passes.
- normal no-cfg RNG failure test passes.
- key lifecycle zeroization test passes.
- provider-error no-mutation test passes.
- send_commit test passes.
- `NA0450_NA0449_INHERITANCE_CONSUMED_OK`.

## Dependency, formal, and script checks

Run:

```bash
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Required result:

- root cargo audit passes.
- nested qsc fuzz lock audit passes.
- rustls-webpki and ml-kem inverse tree checks complete.
- pqcrypto inverse probes are either absent-package evidence or fail the lane if
  they show unexpected dependency reintroduction.
- cargo fmt passes.
- adversarial script syntax checks pass.
- formal checks pass.

## Public claim boundary

Verify the evidence doc and PR body do not create or imply:

- no public readiness claim;
- no production readiness claim;
- no public-internet readiness claim;
- no external-review completion claim;
- no crypto completion claim;
- no side-channel-free status claim;
- no RNG-failure-complete status claim;
- no bug-free status claim;
- no vulnerability-free status claim;
- no perfect-crypto status claim;
- no public technical paper content claim.

Required markers:

- `NA0450_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0450_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0450_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`

Cargo audit green must be described only as dependency-health evidence.

## Stewardship check

Verify the evidence doc includes Level 1 advisory summaries for:

- Crypto / Protocol Steward;
- CI / Dependency / Release Health Steward;
- Public Claims / External Review Steward;
- Product / Demo / Service Boundary Steward;
- Local Ops / Backup / Restore Steward.

Required result:

- Level 1 active.
- Level 2 and Level 3 future-gated.
- no separate Directors.
- no independent READY promotion.
- no independent merge authority.
- Lead Director final authority preserved.
- `NA0450_STEWARD_REVIEW_TEMPLATE_USED_OK`.

## Public-safety

Before merge and after merge, run:

```bash
python3 scripts/ci/qsl_evidence_helper.py public-safety-status --sha "$(git rev-parse origin/main)"
```

For PR checks, public-safety must be completed success before merge.

## Optional local qsc adversarial smoke

If feasible without environment drift:

```bash
scripts/ci/qsc_adversarial.sh
```

If local `cargo fuzz` is unavailable, record the exact output, classify it as a
local tooling limitation only after the stable Rust phases have passed, and rely
on PR CI `qsc-adversarial-smoke` if attached/required.

## PR body preflight

The PR body must include:

```md
Goals: G1, G2, G3, G4, G5
Impact:
No-regression:
Tests/Vectors:
```

The PR body must mention:

- residual surface triage;
- selected classification;
- selected successor;
- no implementation mutation;
- no runtime/crypto/dependency/Cargo/lockfile/workflow/test/fuzz/vector/formal
  mutation;
- no public overclaim.

## Expected successor

Selected NA-0451:

`NA-0451 -- QSL qsc Route / Contact / Attachment RNG Failure Scope Authorization Plan`

NA-0450 must not implement NA-0451.
