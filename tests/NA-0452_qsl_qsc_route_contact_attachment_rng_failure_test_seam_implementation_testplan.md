Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-09

# NA-0452 QSL qsc Route / Contact / Attachment RNG Failure Test Seam Implementation Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Validate the NA-0452 bounded qsc route/contact/attachment RNG failure
test-only seam implementation.

## Required markers

- `NA0452_RNG_RESIDUAL_AUTHORIZATION_CONSUMED_OK`
- `NA0452_ROUTE_CONTACT_ATTACHMENT_RNG_IMPLEMENTED_OK`
- `NA0452_ROUTE_RNG_FORCED_BY_TEST_ONLY_SEAM_OK`
- `NA0452_CONTACT_RNG_FORCED_BY_TEST_ONLY_SEAM_OK`
- `NA0452_ATTACHMENT_RNG_FORCED_BY_TEST_ONLY_SEAM_OK`
- `NA0452_ROUTE_RNG_FAILURE_NO_PARTIAL_STATE_OK`
- `NA0452_CONTACT_RNG_FAILURE_NO_PARTIAL_STATE_OK`
- `NA0452_ATTACHMENT_ID_RNG_FAILURE_NO_PARTIAL_STATE_OK`
- `NA0452_ATTACHMENT_CEK_RNG_FAILURE_NO_PARTIAL_STATE_OK`
- `NA0452_ATTACHMENT_NONCE_PREFIX_RNG_FAILURE_NO_PARTIAL_STATE_OK`
- `NA0452_TUI_RELAY_ROUTE_RNG_FAILURE_NO_PARTIAL_STATE_OK`
- `NA0452_TUI_CONTACT_RNG_FAILURE_NO_PARTIAL_STATE_OK`
- `NA0452_TUI_ACCOUNT_VERIFICATION_SEED_DEFERRED_OK`
- `NA0452_PROVIDER_RNG_DEFERRED_OK`
- `NA0452_PRODUCTION_SEMANTICS_UNCHANGED_OK`
- `NA0452_NO_DEPENDENCY_CHANGE_OK`
- `NA0452_NO_WORKFLOW_CHANGE_OK`
- `NA0452_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0452_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0452_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0452_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0452_ONE_READY_INVARIANT_OK`

## qwork proof-file gate

Verify Codex did not run `qwork`, `qstart`, or `qresume`.

Verify read-only proof files:

- `/srv/qbuild/work/NA-0452/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0452/.qwork/startup.qsl-protocol.json`

Required result:

- proof parse succeeds;
- proof HEAD equals live HEAD before fetch;
- proof `origin/main` equals live `origin/main` before fetch;
- fetch does not advance `origin/main`;
- PR #1172 is merged at `58e2377296d6`;
- READY_COUNT 1 and READY NA-0452.

## Implementation checks

Run:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_residual_surfaces -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test rng_failure_residual_surfaces -- --test-threads=1 --nocapture
```

Required result:

- cfg test emits all required NA-0452 markers;
- no-cfg test emits `NA0452_PRODUCTION_SEMANTICS_UNCHANGED_OK`;
- normal no-cfg build ignores `QSC_RNG_FAILURE_TEST_SEAM`;
- no Cargo feature is added.

## Selected-surface checks

Route/default-route:

- `QSC.VAULT.INIT.DEFAULT_ROUTE_TOKEN` forced failure leaves no vault file and
  no temp vault file.

Contact:

- `QSC.CONTACT.ROUTE_TOKEN` forced failure occurs before `contacts_entry_upsert`.
- `QSC.TUI.CONTACT.ROUTE_TOKEN` forced failure occurs before TUI contact-cache
  insertion or persistence.

TUI relay:

- `QSC.TUI.RELAY_INBOX_ROUTE_TOKEN` forced failure occurs before the generated
  relay inbox route-token write. The vault-init default route token may already
  exist and must not be overwritten by the generated TUI route-token path.

Attachment:

- `QSC.ATTACHMENT.ID` forced failure occurs before staging, journal, service,
  upload, commit, descriptor send, or relay output.
- `QSC.ATTACHMENT.CEK` forced failure occurs before staging, journal, service,
  upload, commit, descriptor send, or relay output.
- `QSC.ATTACHMENT.NONCE_PREFIX` forced failure occurs before staging, journal,
  service, upload, commit, descriptor send, or relay output.

## Deferred-scope checks

Required:

- no TUI account verification seed label is added;
- provider-dependent qsc RNG remains deferred;
- refimpl/provider RNG remains deferred;
- qshield-cli demo RNG remains deferred;
- formal/model RNG remains deferred;
- fuzz/vector RNG remains deferred.

## Validation commands

Run:

```bash
git diff --check
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_residual_surfaces -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test rng_failure_residual_surfaces -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
cargo fmt --check
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

If local cargo-fuzz is unavailable during qsc adversarial smoke, record exact
output and rely on PR CI qsc-adversarial-smoke.

## Scope guard

Changed paths must be limited to:

- `qsl/qsl-client/qsc/tests/rng_failure_residual_surfaces.rs`
- `qsl/qsl-client/qsc/src/vault/mod.rs`
- `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs`
- `qsl/qsl-client/qsc/src/contacts/mod.rs`
- `qsl/qsl-client/qsc/src/tui/controller/commands/contacts.rs`
- `qsl/qsl-client/qsc/src/attachments/mod.rs`
- `docs/governance/evidence/NA-0452_qsl_qsc_route_contact_attachment_rng_failure_test_seam_implementation_harness.md`
- `tests/NA-0452_qsl_qsc_route_contact_attachment_rng_failure_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No dependency, Cargo, lockfile, workflow, fuzz target, vector, formal model,
qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public
docs, README, START_HERE, qwork/qstart/qresume/qshell, backup, restore,
qsl-backup, backup status, backup plan, rollback, or `/backup/qsl` mutation is
allowed.

## Public claim boundary

NA-0452 is bounded internal evidence only.

No public-readiness claim is allowed.

No production-readiness claim is allowed.

No public-internet-readiness claim is allowed.

No external-review-complete claim is allowed.

No crypto-complete claim is allowed.

No RNG-failure-complete claim is allowed.

No side-channel-free claim is allowed.

No vulnerability-free claim is allowed.

No bug-free claim is allowed.

No perfect-crypto claim is allowed.

Cargo audit green remains dependency-health evidence only.

## Expected result

NA-0452 implements the exact D-0889 route/contact/attachment test-only seam,
D-0891 exists once, D-0892 remains absent before optional closeout, READY_COUNT
remains one with READY NA-0452, and selected NA-0453 successor scope is
provider-boundary authorization only.
