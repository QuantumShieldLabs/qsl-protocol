Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-11

# NA-0459 qsc Signature / Identity Provider RNG Failure Scope Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate NA-0459 as an authorization-only qsc signature / identity provider RNG
failure scope lane. The lane must consume NA-0458, classify the signature /
identity residual scope, select exactly one NA-0460 successor, and avoid all
runtime, crypto, dependency, workflow, executable test, fuzz, vector, formal,
service, backup, restore, and public-surface mutation.

## Protected invariants

- NA-0458 KEM-only forced-seam evidence remains bounded.
- signature/identity provider RNG work is not implemented in NA-0459.
- verification / `sig_invalid` remains background only because it is not
  RNG-relevant.
- X25519/ephemeral generation remains a separate residual unless later exact
  scope authorizes it.
- cargo audit green remains dependency-health evidence only.
- No public-readiness claim is allowed. No production-readiness claim is allowed.
- No crypto-complete claim is allowed. No RNG-failure-complete claim is allowed.
- No provider-RNG-complete claim is allowed. No signature-complete claim is allowed.
- No identity-complete claim is allowed. No vulnerability-free claim is allowed. No perfect-crypto claim is allowed.
- Exactly one READY item remains.

## Allowed scope

Changed paths must be limited to:

- `docs/governance/evidence/NA-0459_qsl_qsc_signature_identity_provider_rng_failure_scope_authorization_plan.md`
- `tests/NA-0459_qsl_qsc_signature_identity_provider_rng_failure_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test, fuzz
target, vector, formal model, refimpl, qsl-server, qsl-attachments, qshield
runtime, qshield-cli, website, public docs, README, START_HERE,
qwork/qstart/qresume/qshell, backup, restore, qsl-backup, backup status, backup
plan, rollback, or backup tree mutation is allowed.

## Required evidence checks

- qwork proof files are read and verified without rerunning qwork.
- PR #1186 is merged at `a2712d05b1e2`.
- Queue proof shows READY_COUNT 1 and READY NA-0459 before mutation.
- Decision proof shows D-0903 once, D-0904 once, D-0905 absent, and duplicate
  decision count zero before mutation.
- NA-0458 evidence and testplan inheritance are consumed.
- Signature signing, identity bootstrap, verification, and X25519 surfaces are
  inventoried read-only.
- Primary classification is recorded.
- Selected NA-0460 successor is recorded.

## Classification expectation

Expected NA-0459 classification:

`QSC_SIGNATURE_IDENTITY_SPLIT_FURTHER_NEEDED`

Expected NA-0460 successor:

`NA-0460 -- QSL qsc Signature / Identity Provider RNG Failure Split-Scope Authorization Plan`

## Validation commands

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file <pr-body> --scan-overclaims
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_residual_surfaces -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test rng_failure_residual_surfaces -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
cargo fmt --check
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

If local cargo-fuzz is unavailable during `scripts/ci/qsc_adversarial.sh`,
record the exact output and rely on PR CI qsc-adversarial-smoke.

## Scope guard

Before commit, after commit, and before merge, changed paths must be exactly
the five allowed NA-0459 paths. Any runtime, crypto, dependency, Cargo,
lockfile, workflow, executable test, fuzz target, vector, formal, refimpl,
service, public-surface, backup, restore, qsl-backup, status, plan, rollback,
qwork, qstart, qresume, or qshell mutation fails this testplan.

## Public claim boundary

NA-0459 is bounded internal governance evidence only. No public-readiness claim is allowed.
No production-readiness claim is allowed. No public-internet readiness claim is allowed.
No external-review-complete claim is allowed. No crypto-complete claim is allowed.
No signature-complete claim is allowed. No identity-complete claim is allowed.
No RNG-failure-complete claim is allowed. No provider-RNG-complete claim is allowed.
No side-channel-free claim is allowed. No vulnerability-free claim is allowed.
No bug-free claim is allowed. No perfect-crypto claim is allowed.

## Closeout prerequisite

Closeout to NA-0460 may run only after the NA-0459 evidence PR merges and
post-merge public-safety is green. Closeout must restore exactly one READY item
and must not implement NA-0460.
