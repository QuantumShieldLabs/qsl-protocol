Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-09

# NA-0451 QSL qsc Route / Contact / Attachment RNG Failure Scope Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Validate the NA-0451 governance-only route/contact/attachment RNG failure scope
authorization lane.

This testplan verifies that NA-0451 consumes NA-0450, classifies the selected
qsc route/contact/attachment RNG surfaces, chooses an exact NA-0452 successor,
and preserves no-runtime/no-crypto/no-dependency/no-public-claim boundaries.

## Required markers

- `NA0451_QWORK_PROOF_FILES_VERIFIED_OK`
- `NA0451_NA0450_INHERITANCE_CONSUMED_OK`
- `NA0451_ROUTE_RNG_SCOPE_CLASSIFIED_OK`
- `NA0451_CONTACT_RNG_SCOPE_CLASSIFIED_OK`
- `NA0451_ATTACHMENT_RNG_SCOPE_CLASSIFIED_OK`
- `NA0451_ROUTE_CONTACT_ATTACHMENT_IMPLEMENTATION_READY_OK`
- `NA0451_EXACT_FUTURE_PATHS_SELECTED_OK`
- `NA0451_ACCOUNT_BOOTSTRAP_SEED_DEFERRED_OK`
- `NA0451_NO_RUNTIME_CHANGE_OK`
- `NA0451_NO_DEPENDENCY_CHANGE_OK`
- `NA0451_NO_WORKFLOW_CHANGE_OK`
- `NA0451_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0451_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0451_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0451_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0451_ONE_READY_INVARIANT_OK`

## qwork proof-file gate

Verify Codex did not run `qwork`, `qstart`, or `qresume`.

Verify these files were read:

- `/srv/qbuild/work/NA-0451/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0451/.qwork/startup.qsl-protocol.json`

Required result:

- `.kv` proof has `startup_result=OK`.
- JSON proof parses and mirrors the required `.kv` keys.
- proof `HEAD` equals live `HEAD` before fetch.
- proof `origin/main` equals live `origin/main` before fetch.
- fetch does not advance `origin/main` beyond the qwork proof.
- PR #1170 is merged at `5b15748c0aec`.
- `NA0451_QWORK_PROOF_FILES_VERIFIED_OK`.

## Queue and decision proof

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Required result before patch:

- `READY_COUNT 1`.
- READY item is NA-0451.
- latest decision is D-0888.
- D-0887 exists once.
- D-0888 exists once.
- D-0889 is absent.
- duplicate decision count is zero.

Required result after patch:

- `READY_COUNT 1`.
- READY item remains NA-0451 before optional closeout.
- D-0889 exists once.
- D-0890 is absent before optional closeout.
- duplicate decision count is zero.
- `NA0451_ONE_READY_INVARIANT_OK`.

## Scope guard

Changed paths must be exactly:

- `docs/governance/evidence/NA-0451_qsl_qsc_route_contact_attachment_rng_failure_scope_authorization_plan.md`
- `tests/NA-0451_qsl_qsc_route_contact_attachment_rng_failure_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No runtime, crypto, dependency, Cargo manifest, lockfile, workflow, executable
test, fuzz target, vector, formal model, qsl-server, qsl-attachments, qshield
runtime, qshield-cli, website, public doc, README, START_HERE, qwork,
qstart, qresume, qshell, qsl-backup, backup status, backup plan, rollback, or
backup tree path may be changed.

Required markers:

- `NA0451_NO_RUNTIME_CHANGE_OK`
- `NA0451_NO_DEPENDENCY_CHANGE_OK`
- `NA0451_NO_WORKFLOW_CHANGE_OK`

## Surface classification check

Verify the evidence doc includes:

- route/default-route/relay token RNG scope;
- contact route-token RNG scope;
- attachment ID RNG scope;
- attachment CEK RNG scope;
- attachment nonce-prefix RNG scope;
- provider-dependent RNG residual;
- refimpl provider RNG residual;
- qshield-cli demo RNG residual;
- formal/fuzz/vector residual.

Required classifications:

- route: `ROUTE_RNG_IMPLEMENTATION_READY`;
- contact: `CONTACT_RNG_IMPLEMENTATION_READY`;
- attachment: `ATTACHMENT_RNG_IMPLEMENTATION_READY`;
- primary: `ROUTE_CONTACT_ATTACHMENT_RNG_IMPLEMENTATION_READY`.

Required result:

- selected successor is
  `NA-0452 -- QSL qsc Route / Contact / Attachment RNG Failure Test Seam Implementation Harness`;
- exact future paths are listed;
- TUI account verification seed is deferred from the selected implementation
  successor;
- `NA0451_ROUTE_RNG_SCOPE_CLASSIFIED_OK`;
- `NA0451_CONTACT_RNG_SCOPE_CLASSIFIED_OK`;
- `NA0451_ATTACHMENT_RNG_SCOPE_CLASSIFIED_OK`;
- `NA0451_ROUTE_CONTACT_ATTACHMENT_IMPLEMENTATION_READY_OK`;
- `NA0451_EXACT_FUTURE_PATHS_SELECTED_OK`;
- `NA0451_ACCOUNT_BOOTSTRAP_SEED_DEFERRED_OK`.

## Inherited qsc proof

Run:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
```

Required result:

- cfg-gated RNG failure test passes.
- normal no-cfg RNG failure test passes.
- key lifecycle zeroization test passes.
- provider-error no-mutation test passes.
- send_commit test passes.
- refimpl pqkem768 test passes.
- `NA0451_NA0450_INHERITANCE_CONSUMED_OK`.

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

- `NA0451_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0451_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0451_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`

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
- `NA0451_STEWARD_REVIEW_TEMPLATE_USED_OK`.

## Backup / restore boundary

Verify:

- Codex did not run backup.
- Codex did not run restore.
- Codex did not run sudo.
- qsl-backup SHA proof is read-only boundary evidence.
- qsl-backup source-list inclusion count remains exactly one.
- no qsl-backup, backup status, backup plan, rollback subtree, or backup tree
  path changed.

## Public-safety and PR checks

Before merge:

- PR body includes `Goals:`, `Impact:`, `No-regression:`, and
  `Tests/Vectors:`.
- goal-lint passes.
- public-safety completes success.
- no required check is failing.

After merge:

- public-safety completes success on the merge commit.
- D-0889 exists on main.
- queue remains READY NA-0451 until optional closeout.
