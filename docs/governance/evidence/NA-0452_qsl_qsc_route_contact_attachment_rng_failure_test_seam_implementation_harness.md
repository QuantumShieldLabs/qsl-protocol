Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-09

# NA-0452 QSL qsc Route / Contact / Attachment RNG Failure Test Seam Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0452 consumes NA-0451 / D-0889 and implements the bounded qsc
route/contact/attachment RNG failure test seam for the selected labels only.

Implemented scope:

- reused `QSC.VAULT.INIT.DEFAULT_ROUTE_TOKEN`;
- added cfg-only `QSC.CONTACT.ROUTE_TOKEN`;
- added cfg-only `QSC.TUI.CONTACT.ROUTE_TOKEN`;
- added cfg-only `QSC.TUI.RELAY_INBOX_ROUTE_TOKEN`;
- added cfg-only `QSC.ATTACHMENT.ID`;
- added cfg-only `QSC.ATTACHMENT.CEK`;
- added cfg-only `QSC.ATTACHMENT.NONCE_PREFIX`;
- added `qsl/qsl-client/qsc/tests/rng_failure_residual_surfaces.rs`.

The seam is active only with `--cfg qsc_rng_failure_test_seam`. Normal builds
without that cfg preserve the existing production `OsRng` behavior and ignore
`QSC_RNG_FAILURE_TEST_SEAM`.

No RNG-failure-complete claim is made. Cargo audit green is dependency-health
evidence only.

## Live NA-0452 scope

Live queue state before mutation:

- READY_COUNT 1.
- READY item: NA-0452.
- NA-0451 through NA-0435 DONE.
- NA-0434 and NA-0429 BLOCKED.
- Latest decision before patch: D-0890.
- D-0889 exists once.
- D-0890 exists once.
- D-0891 was absent before this patch.
- Duplicate decision count was zero.

Allowed implementation mutation paths were exactly:

- `qsl/qsl-client/qsc/tests/rng_failure_residual_surfaces.rs`
- `qsl/qsl-client/qsc/src/vault/mod.rs`
- `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs`
- `qsl/qsl-client/qsc/src/contacts/mod.rs`
- `qsl/qsl-client/qsc/src/tui/controller/commands/contacts.rs`
- `qsl/qsl-client/qsc/src/attachments/mod.rs`

Allowed governance mutation paths were exactly this evidence doc, the NA-0452
testplan, `DECISIONS.md`, `TRACEABILITY.md`, and
`docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

Read-only proof files:

- `/srv/qbuild/work/NA-0452/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0452/.qwork/startup.qsl-protocol.json`

Verified qwork proof values:

- `startup_result=OK`
- `lane=NA-0452`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0452/qsl-protocol`
- clean worktree, index, and untracked state
- `ready_count=1`
- `queue_top_ready=NA-0452`
- `requested_lane_status=READY`

Proof HEAD and proof `origin/main` matched live local refs before fetch at
`58e2377296d6`. Fetch did not advance `origin/main`.

PR #1172 was verified MERGED at `58e2377296d6`.

Proof root:

`/srv/qbuild/tmp/NA0452_qsc_route_contact_attachment_rng_impl_20260609T174543Z`

## NA-0451 authorization inheritance

NA-0451 / D-0889 selected:

`ROUTE_CONTACT_ATTACHMENT_RNG_IMPLEMENTATION_READY`

Inherited exact labels:

- reuse `QSC.VAULT.INIT.DEFAULT_ROUTE_TOKEN`;
- add `QSC.CONTACT.ROUTE_TOKEN`;
- add `QSC.TUI.CONTACT.ROUTE_TOKEN`;
- add `QSC.TUI.RELAY_INBOX_ROUTE_TOKEN`;
- add `QSC.ATTACHMENT.ID`;
- add `QSC.ATTACHMENT.CEK`;
- add `QSC.ATTACHMENT.NONCE_PREFIX`.

Inherited exact test path:

- `qsl/qsl-client/qsc/tests/rng_failure_residual_surfaces.rs`

Inherited caveats:

- TUI account verification seed remains deferred.
- qsc provider-dependent RNG remains deferred.
- refimpl/provider RNG remains deferred.
- qshield-cli demo RNG remains deferred.
- formal/model RNG remains deferred.
- fuzz/vector RNG remains deferred.

## Pre-mutation review and rollback setup

Before mutation, Codex recorded SHA256 and mode/owner/size/mtime for the
authorized source paths and copied rollback preimages under the proof root.

The residual-surface test path did not exist before mutation, so an absent
marker was recorded in the rollback bundle.

Pre-mutation `git diff --name-only` and untracked inventory were empty.

## Route/contact/attachment RNG failure seam implementation

The implementation keeps helper code local and cfg-gated:

- `contacts/mod.rs` adds `generate_route_token_with_label()` only under
  `qsc_rng_failure_test_seam`.
- `contacts/mod.rs` uses `QSC.CONTACT.ROUTE_TOKEN` only for generated CLI
  contact route tokens.
- TUI contacts use `QSC.TUI.CONTACT.ROUTE_TOKEN` before contact-cache insertion
  or persistence.
- TUI locked account defaults use `QSC.TUI.RELAY_INBOX_ROUTE_TOKEN` before the
  generated relay inbox route-token write.
- Attachments use local cfg-gated RNG fill helpers for ID, CEK, and nonce
  prefix generation before staging directory creation, ciphertext write,
  journal write, service session, upload, commit, or descriptor send.

`vault/mod.rs` already contained the cfg-gated
`QSC.VAULT.INIT.DEFAULT_ROUTE_TOKEN` label from the inherited seam pattern; the
new executable residual-surface test consumes that label.

## Route RNG failure proof

Cfg test command:

`RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_residual_surfaces -- --test-threads=1 --nocapture`

Passing markers:

- `NA0452_ROUTE_RNG_FORCED_BY_TEST_ONLY_SEAM_OK`
- `NA0452_ROUTE_RNG_FAILURE_NO_PARTIAL_STATE_OK`
- `NA0452_TUI_RELAY_ROUTE_RNG_FAILURE_NO_PARTIAL_STATE_OK`

The default-route forced failure leaves no `vault.qsv` and no temp vault file.
The TUI relay forced failure occurs before the generated relay-token write; the
pre-existing vault-init default token remains in place and is not overwritten by
the generated TUI route-token path.

## Contact RNG failure proof

Passing markers:

- `NA0452_CONTACT_RNG_FORCED_BY_TEST_ONLY_SEAM_OK`
- `NA0452_CONTACT_RNG_FAILURE_NO_PARTIAL_STATE_OK`
- `NA0452_TUI_CONTACT_RNG_FAILURE_NO_PARTIAL_STATE_OK`

CLI contact-add forced failure occurs before `contacts_entry_upsert`; the vault
bytes remain unchanged and `contacts.json` is absent. TUI contact-add forced
failure occurs before `state.contacts_records` insertion and before
`persist_contacts_cache`.

## Attachment RNG failure proof

Passing markers:

- `NA0452_ATTACHMENT_RNG_FORCED_BY_TEST_ONLY_SEAM_OK`
- `NA0452_ATTACHMENT_ID_RNG_FAILURE_NO_PARTIAL_STATE_OK`
- `NA0452_ATTACHMENT_CEK_RNG_FAILURE_NO_PARTIAL_STATE_OK`
- `NA0452_ATTACHMENT_NONCE_PREFIX_RNG_FAILURE_NO_PARTIAL_STATE_OK`

For each selected attachment label, the forced failure leaves no attachment
journal secret, no attachment staging directory, unchanged vault bytes, and no
relay descriptor output.

## Deferred TUI account verification seed proof

The account verification seed call in `locked.rs` remains unchanged:

- no `QSC.TUI.ACCOUNT_VERIFICATION_SEED` label was added;
- `ACCOUNT_VERIFICATION_SEED_SECRET_KEY` is still outside the selected NA-0452
  implementation labels;
- marker `NA0452_TUI_ACCOUNT_VERIFICATION_SEED_DEFERRED_OK` is emitted by the
  cfg residual-surface test.

## Provider RNG deferred proof

Provider-dependent qsc RNG and refimpl/provider RNG remain backlog scope. NA-0452
does not mutate provider traits, provider key generation, refimpl crypto, ML-KEM
or ML-DSA provider paths, qsc provider boundary code, qshield-cli demo code,
formal models, fuzz targets, or vectors.

Marker:

- `NA0452_PROVIDER_RNG_DEFERRED_OK`

## Production semantics unchanged proof

Normal no-cfg command:

`cargo test -p qsc --locked --test rng_failure_residual_surfaces -- --test-threads=1 --nocapture`

Passing marker:

- `NA0452_PRODUCTION_SEMANTICS_UNCHANGED_OK`

The no-cfg test sets `QSC_RNG_FAILURE_TEST_SEAM` and proves normal vault init
still succeeds, demonstrating the selector is inert without
`qsc_rng_failure_test_seam`.

## No dependency / no workflow mutation proof

NA-0452 changes no Cargo manifests, lockfiles, dependencies, workflows, fuzz
targets, vectors, formal models, qsl-server, qsl-attachments, qshield runtime,
qshield-cli, website, README, START_HERE, qwork/qstart/qresume/qshell,
qsl-backup, backup status, backup plan, rollback subtree, or `/backup/qsl`.

Markers:

- `NA0452_NO_DEPENDENCY_CHANGE_OK`
- `NA0452_NO_WORKFLOW_CHANGE_OK`

## No RNG-failure-complete claim proof

NA-0452 is bounded route/contact/attachment evidence only.

No RNG-failure-complete claim is made.

No crypto-complete claim is made.

No side-channel-free claim is made.

No vulnerability-free claim is made.

No bug-free claim is made.

No perfect-crypto claim is made.

No public-readiness claim is made.

No production-readiness claim is made.

No public-internet-readiness claim is made.

No external-review-complete claim is made.

No public technical paper content is created.

Markers:

- `NA0452_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0452_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0452_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`

## qshield-cli demo-local boundary proof

qshield-cli remains demo-local and out of scope. NA-0452 does not mutate
`apps/qshield-cli/` and does not use qshield-cli demo RNG as completion
evidence.

## refimpl/provider RNG backlog proof

refimpl/provider RNG remains a future boundary review topic. NA-0452 does not
mutate `tools/refimpl/` and does not claim provider RNG failure coverage.

## Validation

Startup validation completed before mutation:

- qwork proof parse and freshness: PASS.
- PR #1172 merged at `58e2377296d6`: PASS.
- queue helper: READY_COUNT 1, READY NA-0452.
- decision helper: latest D-0890, duplicate count zero.
- public-safety on current main: PASS.
- root `cargo audit --deny warnings`: PASS.
- nested qsc fuzz lock audit: PASS.
- inherited cfg/no-cfg `rng_failure_behavior`: PASS.
- `key_lifecycle_zeroization`: PASS.
- `handshake_provider_error_no_mutation`: PASS.
- qsl-backup SHA matched `e9ecff3d22ed`; same-host ops source-list inclusion
  count was exactly one.

Implementation validation:

- `git diff --check`: PASS.
- exact staged scope guard: PASS, ten changed paths and zero extra paths.
- link check: PASS, `TOTAL_MISSING 0`.
- added-line leak scan: PASS, `SECRET_FINDING_COUNT 0`.
- PR body preflight: PASS, missing field count zero and prohibited phrase count
  zero.
- classifier: `runtime_critical`, as expected for exact qsc runtime/test seam
  paths.
- affirmative added-line overclaim scan: PASS, zero affirmative overclaim
  findings.
- `cargo fmt --check`: PASS.
- cfg `rng_failure_residual_surfaces`: PASS, 8 tests.
- no-cfg `rng_failure_residual_surfaces`: PASS, 1 test.
- cfg `rng_failure_behavior`: PASS.
- no-cfg `rng_failure_behavior`: PASS.
- `key_lifecycle_zeroization`: PASS.
- `handshake_provider_error_no_mutation`: PASS.
- stable `send_commit`: PASS.
- refimpl `pqkem768`: PASS.
- root `cargo audit --deny warnings`: PASS.
- nested qsc fuzz lock audit: PASS.
- inverse dependency trees: `rustls-webpki` and `ml-kem` present as expected;
  pqcrypto inverse probes returned the expected not-present result.
- `sh -n scripts/ci/qsc_adversarial.sh`: PASS.
- `bash -n scripts/ci/qsc_adversarial.sh`: PASS.
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`: PASS.
- `python3 formal/run_model_checks.py`: PASS.
- local qsc adversarial script: stable Rust adversarial properties, miri-style
  adversarial tests, and the NA-0439 provider-error no-mutation step passed
  before local `cargo fuzz` unavailability stopped the script with `error: no
  such command: fuzz`. This is a local tooling caveat; PR CI
  `qsc-adversarial-smoke` remains the required cargo-fuzz-backed proof.

Recovered failures:

- Initial cfg residual test used the mock-vault passphrase for TUI vault
  assertions; fixed by adding passphrase-aware vault reads.
- First rerun expected no relay inbox secret, but vault init already creates a
  default route token under the same key; fixed by asserting the generated TUI
  route token did not overwrite the default-token shape.
- A broad added-line overclaim probe flagged required no-claim guardrail text.
  Classification: recoverable scanner-shape issue. Corrective action: reran a
  negation-aware affirmative overclaim scan. Final result: zero affirmative
  overclaim findings.
- The local qsc adversarial script exited nonzero only after passing the stable
  Rust adversarial phases and provider-error step, because local `cargo fuzz` is
  unavailable. Corrective action: recorded exact output and preserved PR CI
  `qsc-adversarial-smoke` as the authoritative cargo-fuzz-backed proof.

## Public claim/external review/website boundary

NA-0452 does not update README, START_HERE, website, public docs, public paper
content, external review materials, or public claim surfaces.

Cargo audit output remains dependency-health evidence only.

## Rejected alternatives

- Adding a Cargo feature: rejected because D-0889 requires the existing cfg seam
  pattern.
- Labeling the TUI account verification seed: rejected because D-0889 defers
  that account-bootstrap residual.
- Adding public APIs for tests: rejected because real CLI/TUI paths can prove
  the selected invariants.
- Treating the TUI relay generated-token failure as no-vault-secret proof:
  rejected because vault init already creates a default route token under the
  same secret key.

## Backup-impact statement

Codex did not run backup, restore, or sudo. qsl-backup was not mutated. Backup
status and plan files were read only. `/backup/qsl` was read only for manifest
source-list proof.

## Successor selection

Default selected successor after successful implementation:

`NA-0453 -- QSL refimpl / qsc Provider RNG Failure Boundary Authorization Plan`

NA-0452 does not implement NA-0453.

## Next recommendation

After NA-0452 merges and public-safety is green, close out NA-0452 and restore
the provider-boundary NA-0453 successor as the sole READY item.
