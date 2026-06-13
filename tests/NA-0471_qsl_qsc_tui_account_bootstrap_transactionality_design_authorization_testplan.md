Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-13

# NA-0471 qsc TUI Account Bootstrap Transactionality Design Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the governance-only NA-0471 transactionality design authorization
lane. The lane must consume NA-0470, classify the best-known TUI account
bootstrap transactionality design, select exactly one successor, and avoid
implementation mutation or public overclaim.

## Protected invariants

- Exactly one READY item remains before optional closeout.
- NA-0471 is READY at start.
- NA-0470 is DONE at start.
- D-0927, D-0928, and D-0929 exist once at start.
- D-0930 is absent before patch and exists once after patch.
- D-0931 remains absent before optional closeout.
- qwork proof files are read and verified without rerunning qwork.
- NA-0470 evidence is consumed.
- D-0927 assurance recovery context is consumed.
- TUI bootstrap transactionality design is classified without runtime, crypto,
  dependency, Cargo, lockfile, workflow, executable test, fuzz target, vector,
  formal model, refimpl, qsl-server, qsl-attachments, qshield runtime,
  qshield-cli, website, public docs, README, START_HERE,
  qwork/qstart/qresume/qshell, backup, restore, qsl-backup, backup status,
  backup plan, rollback, or backup tree mutation.
- No public-readiness claim is introduced.
- No production-readiness claim is introduced.
- No public-internet-readiness claim is introduced.
- No external-review-complete claim is introduced.
- No crypto-complete claim is introduced.
- No KEM-complete claim is introduced.
- No signature-complete claim is introduced.
- No identity-complete claim is introduced.
- No RNG-failure-complete claim is introduced.
- No provider-RNG-complete claim is introduced.
- No secret-material-complete claim is introduced.
- No side-channel-free claim is introduced.
- No vulnerability-free claim is introduced.
- No bug-free claim is introduced.
- No perfect-crypto claim is introduced.

## Required classification checks

Required:

- primary classification is one of the directive's accepted classifications;
- selected classification is
  `TUI_BOOTSTRAP_PREGENERATION_IMPLEMENTATION_READY`;
- selected successor is `NA-0472 -- QSL qsc TUI Account Bootstrap
  Pre-Generation Transactionality Implementation Harness`;
- assurance gap trigger classification is
  `HIGHER_PRIORITY_RESIDUAL_SUPERSEDES_ASSURANCE_REVIEW`;
- pre-generation is selected over rollback, staged commit, identity-only
  invariant, path-specific split, documentation-only, and immediate Assurance
  Gap Review.

## Required source inventory checks

Read-only inspection must record:

- `/init` command entry;
- init wizard entry;
- `tui_try_vault_init(passphrase)`;
- `initialize_account_after_init(alias, passphrase)`;
- `init_account_defaults_with_passphrase`;
- autolock write;
- poll setting write;
- receipt/file setting write;
- verification seed write;
- relay endpoint/token write;
- TUI relay inbox token state write;
- `init_identity_with_passphrase`;
- identity KEM keypair generation;
- identity signature keypair generation;
- identity KEM secret write;
- identity signature secret write;
- self public record write;
- selected identity write/update if any;
- setup success output/diagnostics;
- CLI rotation completed background;
- lazy identity closed background;
- legacy/public-record completed background;
- X25519 / ephemeral excluded background;
- refimpl provider RNG excluded background.

Each row must include exact source path, provider/random operation,
initial-state requirement, state/write timing, existing marker/error, existing
coverage, whether existing APIs are enough, whether a future cfg-only seam is
enough, whether rollback/staging/pre-generation is needed, future test path,
truthful invariant, and priority.

## Required design checks

Evidence must evaluate:

- Option 1 - pre-generate identity material before durable account/default
  writes;
- Option 2 - stage all bootstrap writes, then atomic commit;
- Option 3 - roll back durable account/default writes after identity provider
  failure;
- Option 4 - narrow future invariant to identity/public-record-only;
- Option 5 - path-specific split;
- Option 6 - documentation-only;
- Option 7 - Assurance Gap Review now.

Required outcome:

- select Option 1;
- record in-memory secret lifetime caveat;
- reject rollback as primary cleanup;
- reject staged commit as too broad without existing bootstrap transaction
  support;
- reject identity-only proof as too weak;
- reject immediate Assurance Gap Review because a higher-priority exact
  implementation successor is selected.

## Required stewardship and assurance checks

Evidence must include:

- Crypto / Protocol Steward review.
- CI / Dependency / Release Health Steward review.
- Public Claims / External Review Steward review.
- Product / Demo / Service Boundary Steward review.
- Local Ops / Backup / Restore Steward review.
- Best-Known-Method Review with `BEST_KNOWN_METHOD_FOR_SCOPE`.
- Hostile Cryptographer Review.
- Red-Team Review.
- Production SRE Review.
- Side-Channel Caveat.
- Formal-Model Mapping Residual classification
  `FORMAL_MODEL_MAPPING_RESIDUAL_ACTIVE`.
- External-Review Readiness classification
  `EXTERNAL_REVIEW_READINESS_INCREMENTAL`.
- Release-Claim Boundary.
- Assurance Gap Review Trigger classification
  `HIGHER_PRIORITY_RESIDUAL_SUPERSEDES_ASSURANCE_REVIEW`.

## Required markers

Evidence must record:

- `NA0471_QWORK_PROOF_FILE_VERIFIED_OK`
- `NA0471_NA0470_CONSUMED_OK`
- `NA0471_D0927_CONTEXT_CONSUMED_OK`
- `NA0471_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0471_TUI_BOOTSTRAP_WRITE_ORDER_REVIEW_OK`
- `NA0471_TUI_BOOTSTRAP_PREGENERATION_SELECTED_OK`
- `NA0471_NEXT_SCOPE_SELECTED_OK`
- `NA0471_NO_IMPLEMENTATION_MUTATION_OK`
- `NA0471_NO_DEPENDENCY_CHANGE_OK`
- `NA0471_NO_WORKFLOW_CHANGE_OK`
- `NA0471_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0471_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0471_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0471_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0471_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0471_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0471_ASSURANCE_TRIGGER_HIGHER_PRIORITY_RESIDUAL_OK`
- `NA0471_ONE_READY_INVARIANT_OK`

## Validation commands

Run:

```bash
git diff --check
```

Run exact scope guard:

```bash
python3 - <<'PY'
import subprocess, sys
allowed = {
    "docs/governance/evidence/NA-0471_qsl_qsc_tui_account_bootstrap_transactionality_design_authorization_plan.md",
    "tests/NA-0471_qsl_qsc_tui_account_bootstrap_transactionality_design_authorization_testplan.md",
    "DECISIONS.md",
    "TRACEABILITY.md",
    "docs/ops/ROLLING_OPERATIONS_JOURNAL.md",
}
changed = set(subprocess.check_output(["git", "diff", "--name-only"], text=True).splitlines())
bad = sorted(changed - allowed)
print("CHANGED_PATHS", sorted(changed))
print("SCOPE_GUARD_BAD_COUNT", len(bad))
if bad:
    print("\n".join(bad))
    sys.exit(1)
PY
```

Run link check, leak scan, overclaim scan, classifier, PR body preflight, and
goal-lint.

Run inherited validation:

```bash
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test cli_identity_rotation_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test cli_identity_rotation_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test legacy_identity_public_record_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test legacy_identity_public_record_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test lazy_identity_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test lazy_identity_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test a2_signature_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test a2_signature_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
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

Run qsc adversarial smoke locally if feasible:

```bash
scripts/ci/qsc_adversarial.sh
```

If local cargo-fuzz is unavailable, record the exact local output and rely on
PR CI `qsc-adversarial-smoke`.

## Expected results

- READY_COUNT is 1.
- The sole READY item is NA-0471 before optional closeout.
- NA-0470 is DONE.
- D-0930 exists exactly once.
- D-0931 is absent before optional closeout.
- Duplicate decision count is zero.
- Changed paths are limited to the five allowed NA-0471 governance paths.
- Root cargo audit is green.
- Nested qsc fuzz lock audit is green.
- Inherited qsc/provider/refimpl/formal checks are green.
- No implementation mutation occurs.
- No public overclaim is introduced.
- Public-safety is green before merge and after merge.
