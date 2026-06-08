Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-08

# NA-0445 QSL qsc Key Lifecycle Secret Cleanup / Zeroization Test Scope Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0445 consumes the NA-0444 key-lifecycle cleanup / zeroization evidence
policy and authorizes exact future qsc test scope. It does not implement tests
or change runtime behavior.

Primary classification:

`QSC_ZEROIZATION_TEST_SCOPE_IMPLEMENTATION_READY`

Selected successor:

`NA-0446 -- QSL qsc Key Lifecycle Secret Cleanup / Zeroization Test Implementation Harness`

Exact future qsc test path:

`qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs`

Reason: existing qsc CLI/test APIs and mock-vault test helpers are sufficient
to implement bounded integration tests for observable cleanup/no-mutation,
session-store insertion, encrypted-at-rest, peer-bound session state, and
redaction invariants. The future test must not claim direct memory overwrite,
allocator behavior, side-channel behavior, or all-material coverage. Those
remain residual runtime-hook or documentation-only boundaries unless a later
exact directive authorizes them.

## Live NA-0445 scope

Live `NEXT_ACTIONS.md` lists:

`NA-0445 -- QSL qsc Key Lifecycle Secret Cleanup / Zeroization Test Scope Authorization Plan`

Status: READY.

Allowed NA-0445 mutation paths:

- `docs/governance/evidence/NA-0445_qsl_qsc_key_lifecycle_secret_cleanup_zeroization_test_scope_authorization_plan.md`
- `tests/NA-0445_qsl_qsc_key_lifecycle_secret_cleanup_zeroization_test_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Read-only inspection included qwork proof files, the live NA-0445 queue block,
NA-0441 through NA-0444 evidence and testplans, D-0869 through D-0876,
TRACEABILITY, the rolling journal, the domain stewardship canon, qsc source and
tests, qsc fuzz metadata, refimpl and qshield-cli boundary paths, formal paths,
inputs, Cargo manifests and lockfiles, scripts, workflows, qsl-backup boundary
proof, backup status/plan proof, and prior response files.

Forbidden mutation scope includes runtime code, crypto code, dependencies,
Cargo manifests, lockfiles, workflows, executable tests, fuzz targets, vectors,
formal model files, qsl-server, qsl-attachments, qshield runtime, website,
public docs, README, START_HERE, qwork/qstart/qresume/qshell, qsl-backup,
backup status files, backup plan files, rollback subtree paths, `/backup/qsl`,
public technical paper content, branch protection, and public claim surfaces.

Acceptance criteria:

- qwork proof files are verified without running qwork, qstart, or qresume;
- NA-0444 evidence policy is consumed;
- qsc testable surfaces are classified;
- exact future qsc test path is selected if implementation is authorized;
- material requiring runtime hooks remains out of scope;
- one exact NA-0446 successor is selected;
- no implementation mutation occurs in NA-0445;
- root cargo audit remains green;
- nested qsc fuzz lock audit remains green;
- provider-error no-mutation test and qsc adversarial script anchors remain
  healthy;
- formal checks remain green;
- public-safety is green before merge and after merge;
- exactly one READY item remains.

Stop conditions include missing or inconsistent qwork proof, qwork/qstart/
qresume execution by Codex, PR #1158 not merged, stale origin/main after fetch,
queue drift from READY NA-0445, D-0876 absence, D-0877 preexistence, audit
failure, unconsumable NA-0444 policy, unsafe surface classification, unsafe
successor selection, qsl-backup source-list regression, backup or restore
execution, forbidden mutation, public overclaim, or more than one READY item.

## qwork proof-file verification

Codex read, but did not run, qwork proof files:

- `/srv/qbuild/work/NA-0445/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0445/.qwork/startup.qsl-protocol.json`

Required `.kv` markers passed:

- `startup_result=OK`
- `lane=NA-0445`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0445/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0445`
- `requested_lane_status=READY`

The JSON proof parsed successfully and mirrored the `.kv` proof for lane, repo,
path, HEAD, origin/main, ready count, top READY item, requested lane status,
and clean-state fields.

Initial live `HEAD` and `origin/main` matched the qwork proof at
`dc0adc8aebd6`. After `git fetch --all --prune`, `origin/main` still matched
the proof. PR #1158 was verified MERGED with merge commit `dc0adc8aebd6`.

Recorded timestamps:

- Local: `2026-06-08T12:41:19-05:00`
- UTC: `2026-06-08T17:41:19+00:00`

Proof root:

`/srv/qbuild/tmp/NA0445_qsc_key_lifecycle_zeroization_test_scope_20260608T174211Z`

## NA-0444 evidence policy inheritance

NA-0444 selected:

`KEY_LIFECYCLE_ZEROIZATION_POLICY_TEST_SCOPE_NEXT`

Inherited policy:

- qsc runtime is the immediate test-scope bucket.
- refimpl remains included in evidence-policy scope but is not immediate qsc
  test implementation scope.
- qshield-cli remains demo-local and is not qsc runtime assurance.
- qsc runtime tests must distinguish pending handshake material, session/shared
  secret store material, vault/passphrase/runtime key material,
  redaction/logging, reject/no-mutation paths, and encrypted-at-rest evidence.
- no public claim may imply secret-material-complete status;
- no public claim may imply crypto-complete status;
- no public claim may imply side-channel-free status;
- no public claim may imply vulnerability-free status;
- no public claim may imply bug-free status;
- no public claim may imply perfect-crypto status;
- no public claim may imply public-readiness status;
- no public claim may imply production-readiness status;
- no public claim may imply external-review-complete status.

Inherited residuals:

- qsc runtime cleanup/zeroization evidence is bounded and not comprehensive.
- refimpl cleanup/zeroization remains a future separate evidence-policy/test
  scope if prioritized.
- qshield-cli demo-local storage remains claim-boundary/backlog.
- F-0441-03 RNG failure behavior remains the next candidate after qsc
  cleanup/zeroization test-scope work.

## Applicable Stewardship Review

### Crypto / Protocol Steward

qsc test scope must target the NA-0444 evidence-policy requirements, not change
runtime implementation. Future tests may prove only observable bounded
invariants through existing APIs: pending-store cleanup, reject no-mutation,
session-store insertion after success, encrypted-at-rest state, peer-bound AAD,
and redaction. They must not imply secret-material-complete coverage.

Pending, session, vault, redaction, and no-mutation surfaces are separate
material classes. Runtime memory overwrite, allocator behavior, side-channel
properties, and exhaustive key-material lifecycle coverage require future
authorization and are not covered by the implementation-ready scope.

### CI / Dependency / Release Health Steward

Current main evidence:

- public-safety completed success on `dc0adc8aebd6`;
- qsc-adversarial-smoke completed success on `dc0adc8aebd6`;
- root `cargo audit --deny warnings` passed;
- nested qsc fuzz lock audit passed;
- `rustls-webpki v0.103.13` is present;
- root pqcrypto inverse probes reported expected package-ID absence;
- nested qsc fuzz lock pqcrypto residual scan returned zero matches;
- qsc provider-error no-mutation test passed;
- `scripts/ci/qsc_adversarial.sh` contains the inherited NA-0439 marker and
  provider-error no-mutation command.

Cargo audit green is dependency-health evidence only.
It is not public-readiness proof.
It is not production-readiness proof.
It is not external-review-complete proof.
It is not crypto-complete proof.
It is not secret-material-complete proof.
It is not side-channel-free proof.
It is not vulnerability-free proof.
It is not bug-free proof.
It is not perfect-crypto proof.

### Public Claims / External Review Steward

This test-scope authorization is internal governance evidence only.

No secret-material-complete claim is made.
No crypto-complete claim is made.
No side-channel-free claim is made.
No vulnerability-free claim is made.
No bug-free claim is made.
No perfect-crypto claim is made.
No public-readiness claim is made.
No production-readiness claim is made.
No public-internet-readiness claim is made.
No external-review-complete claim is made.

Future tests, if implemented, must be described as bounded internal evidence
for named paths and invariants only.

### Product / Demo / Service Boundary Steward

qshield-cli remains demo-local. qsc runtime test scope must not become a
qshield-cli service-readiness claim.

No qsl-server readiness claim is made or authorized.
No qsl-attachments readiness claim is made or authorized.
No qshield runtime readiness claim is made or authorized.
No website readiness claim is made or authorized.
No public docs readiness claim is made or authorized.
No public-service readiness claim is made or authorized.
No production-readiness claim is made or authorized.
No public-internet-readiness claim is made or authorized.

### Local Ops / Backup / Restore Steward

No backup, restore, sudo, qwork, qstart, or qresume was run. qsl-backup proof
remains boundary evidence only: the qsl-backup checksum matched the required
value and the script source-list inclusion count for the Codex ops path was
exactly one.

No qsl-backup, backup status, backup plan, rollback subtree, backup tree,
systemd/timer/fstab, or local-ops mutation is authorized or performed.

Level 1 stewardship is active. Level 2 and Level 3 remain future-gated.
Stewards remain advisory only: no separate Directors, no independent READY
promotion, no independent merge authority, and Lead Director final authority is
preserved.

## qsc testable surface inventory

| Surface | Existing APIs enough? | Runtime hook needed? | Test-only fixtures needed? | Observable material without exposing secrets | Assertion type | Candidate exact future test path | Expected markers |
|---|---|---|---|---|---|---|---|
| `qsl/qsl-client/qsc/src/handshake/mod.rs` pending store | Yes for encrypted vault secret presence/absence and byte no-mutation; no for memory overwrite | No for store cleanup/no-mutation; yes for direct memory overwrite | Yes, mock-vault decrypt/rewrite helper already used by integration tests | encrypted vault entry name, empty/absent value, session file presence, reject markers | cleanup on success, no-mutation on bounded reject, redaction | `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs` | `NA0446_PENDING_HANDSHAKE_CLEANUP_OK`, `NA0446_PENDING_REJECT_NO_MUTATION_OK` |
| `qsl/qsl-client/qsc/src/protocol_state/mod.rs` session store | Yes for session file creation/absence, AEAD envelope, tamper reject, peer-bound AAD behavior | No for storage invariants; yes for plaintext lifetime/memory overwrite | Existing seeded session and mock-vault patterns | `.qsv` existence, legacy tombstone, encrypted bytes, peer-bound decrypt failure markers | insertion only after success, encrypted-at-rest, tamper/no-mutation | `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs` | `NA0446_SESSION_STORE_BOUNDARY_OK`, `NA0446_SESSION_ENCRYPTED_AT_REST_OK` |
| `qsl/qsl-client/qsc/src/vault/mod.rs` vault/passphrase/runtime key | Partial. CLI and mock-vault APIs can test encrypted-at-rest, redaction, no-mutation, and destroy behavior; direct drop/zeroize observation needs hooks | Yes for direct runtime key/passphrase memory overwrite or `Drop` observation | Existing passphrase-file/stdin and mock-vault helpers | vault file bytes, output markers, file absence/presence, no plaintext passphrase in file/output | redaction, encrypted-at-rest, no-mutation on reject, destroy cleanup | `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs` for bounded assertions only | `NA0446_VAULT_REDACTION_BOUNDARY_OK`, `NA0446_RUNTIME_MEMORY_ZEROIZATION_RESIDUAL_OK` |
| qsc redaction/logging outputs | Yes for command stdout/stderr and marker scans | No for output assertions | Existing `output_text` and marker-scan helpers | deterministic markers and absence of test sentinel/passphrase/route-token strings | redaction/no leak/panic | `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs` | `NA0446_REDACTION_BOUNDARY_OK` |
| `qsl/qsl-client/qsc/tests/handshake_provider_error_no_mutation.rs` | Already exists and passes | No for existing decap no-mutation evidence | Existing test fixture | pending secret value, session absence, vault bytes | inherited reject no-mutation | Read-only current evidence; future implementation may reference, not mutate unless exact scope later changes | existing NA0436 markers |
| Existing vault/session/redaction/tamper/no-mutation tests | Yes for current bounded evidence | No for current assertions | Existing integration helpers | output strings, session blob bytes, vault file bytes | supporting current evidence | Read-only current evidence; future implementation may consolidate only in exact NA-0446 file | existing test-specific markers |
| `scripts/ci/qsc_adversarial.sh` | Yes for inherited provider-error command presence and CI smoke | No | None | marker and command text | adversarial anchor | Read-only in NA-0446 unless later exact scope changes | `NA0439_QSC_PROVIDER_ERROR_NO_MUTATION_ADVERSARIAL_STEP` |

## Pending handshake material test scope

Pending material exists in `HandshakePending`:

- `session_id`;
- `kem_sk`;
- `kem_pk`;
- `dh_sk`;
- `dh_pub`;
- `sig_pk`;
- `peer_fp`;
- `peer_sig_fp`;
- `peer_sig_pk`;
- `confirm_key`;
- `transcript_hash`;
- `pending_session`;
- `suite_context`.

Sensitive material includes `kem_sk`, `dh_sk`, `confirm_key`,
`transcript_hash`, and `pending_session`. The persisted pending record is stored
as a vault secret under a `handshake.pending.<self>.<peer>` key and is cleared
through `hs_pending_clear`, which stores an empty value and removes any legacy
pending file.

Current evidence:

- `hs_pending_store` persists pending material via the encrypted vault secret
  store.
- `hs_pending_clear` is called on successful initiator and responder handshake
  completion and on selected fail-closed context/key-context rejects.
- `handshake_provider_error_no_mutation.rs` proves the `pq_decap_failed`
  reject path does not mutate sessions, pending state, or vault bytes.
- `na_0303_handshake_activation_negotiation.rs` proves several admission
  rejects do not create sessions and that replay preserves pending vault bytes.

Future exact tests can assert:

- pending state is present after A1/B1 setup where expected;
- pending state is cleared after successful completion for both peers;
- bounded reject paths preserve pending/session/vault state when fail-closed
  semantics require no mutation;
- secret sentinels, route tokens, and passphrase material are not emitted.

Classification:

`PENDING_SECRET_TEST_SCOPE_READY`

Residual:

`PENDING_SECRET_REQUIRES_RUNTIME_HOOK` for direct in-memory overwrite/drop proof.

Exact candidate future test file:

`qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs`

## Session / shared secret store test scope

Session/shared-secret artifacts are persisted as encrypted qsp session blobs:

- `.qsv` session files under `qsp_sessions`;
- AEAD envelope with `QSP_SESSION_BLOB_MAGIC`;
- peer-bound AAD built from session blob version and peer label;
- session store key loaded from vault or test fallback;
- Suite-2 session snapshots containing chain keys, root/handshake material, and
  receive/send state.

Current evidence:

- `qsp_session_store` writes encrypted `.qsv` blobs after successful session
  establishment.
- `qsp_session_load_encrypted` decrypts and rejects tampered state.
- `session_state_at_rest.rs` proves session state is not plaintext on disk,
  tamper rejects without mutation, migration is idempotent, migration blocked
  without vault is no-mutation, and outputs avoid broad secret-like markers.
- handshake activation tests prove session files remain absent on selected
  rejects and appear after success.

Future exact tests can assert:

- session files are absent before success and on bounded rejects;
- session files are inserted only after handshake success;
- session blobs do not contain plaintext field names or test payload markers;
- tampered or peer-mismatched session blobs reject without mutation;
- output remains redacted.

Classification:

`SESSION_SECRET_TEST_SCOPE_READY`

Residual: direct cleanup of in-memory shared-secret buffers requires runtime
hooks and is not part of the implementation-ready test scope.

Exact candidate future test file:

`qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs`

## Vault / passphrase / runtime key test scope

Vault/passphrase/runtime key material includes:

- passphrase buffers from stdin/file/env compatibility paths;
- process passphrase cache;
- Argon2 passphrase bytes;
- vault runtime key bytes;
- vault payload secret values;
- session store key secret;
- route token secret.

Current evidence:

- `vault_init` zeroizes passphrase and key buffers after success and on many
  fail-closed paths.
- `unlock_with_passphrase_file`, `unlock_with_passphrase_env`, `vault_unlock`,
  and `set_process_passphrase` zeroize selected buffers or replaced process
  passphrase values.
- `VaultSession` `Drop` zeroizes the runtime key and vault payload secret
  values before clearing the map.
- `secret_set` and `secret_set_with_passphrase` re-encrypt vault payloads and
  zeroize the runtime key after write.
- existing vault tests prove encrypted-at-rest behavior, no plaintext
  passphrase in vault bytes, redacted output, and no mutation on selected
  rejects.

Future exact tests can assert bounded behavior through existing APIs:

- vault file and output do not contain a test passphrase or sentinel secret;
- passphrase/profile rejects do not mutate vault bytes;
- destroy cleanup removes or erases the vault file as currently implemented;
- route-token and passphrase sentinels do not appear in command output.

Not testable without runtime hooks:

- direct proof that `VaultSession::drop` zeroized memory;
- compiler/allocator overwrite persistence;
- side-channel behavior;
- all process-passphrase lifetime paths.

Classification:

`VAULT_SECRET_TEST_SCOPE_PARTIAL`

Residual:

`VAULT_SECRET_REQUIRES_RUNTIME_HOOK` for direct runtime-memory zeroization
proof.

Exact candidate future test file for bounded assertions:

`qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs`

## Redaction / logging test scope

Current qsc tests already assert absence of selected secret-like values in
stdout/stderr and markers across handshake, vault, session-state, TUI, and
message-state surfaces. Markers are deterministic and generally report event
state, peer labels, result codes, and redacted paths rather than secret values.

Future exact tests should include a consolidated sentinel scan for the new
key-lifecycle fixture:

- route tokens;
- passphrase sentinel;
- malformed handshake sentinel;
- payload sentinel;
- panic/backtrace strings;
- process passphrase environment key name where applicable.

Classification:

`REDACTION_TEST_SCOPE_READY`

Exact candidate future test file:

`qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs`

## Test scope decision matrix

| Surface | Material type | Current evidence | Gap | Candidate future test | Exact future test path | Existing APIs enough? | Runtime hook needed? | Risk | Priority | Public-claim implication |
|---|---|---|---|---|---|---|---|---|---|---|
| Pending handshake secret material | `kem_sk`, `dh_sk`, `confirm_key`, `transcript_hash`, `pending_session` | `hs_pending_store`, `hs_pending_clear`, NA0436 no-mutation, NA0303 replay no-mutation | no unified success-clear plus reject-boundary test | `pending_handshake_secret_cleanup_success_and_reject_boundaries` | `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs` | yes for vault/store state | no for store assertions; yes only for memory overwrite | medium | P0 | bounded internal evidence only |
| Session/shared secret store material | Suite-2 snapshot, chain keys, root/session state | `session_state_at_rest.rs`, `qsp_session_store`, handshake success/reject tests | no single key-lifecycle test tying insertion to success and absence on reject | `session_secret_store_inserted_only_after_success_and_encrypted_at_rest` | `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs` | yes | no for storage; yes for memory overwrite | medium | P0 | no crypto-complete claim |
| Vault/passphrase/runtime key material | passphrase buffers, vault key, secret map values | vault zeroize calls, `VaultSession::drop`, vault redaction/no-mutation tests | direct drop/overwrite not externally observable | `vault_passphrase_redaction_and_no_plaintext_boundary` | `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs` | partial | yes for direct memory/drop proof | medium | P1 | no secret-material-complete or side-channel claim |
| Redaction/logging | markers, stdout/stderr, diagnostics | bounded redaction tests across qsc | not consolidated for key-lifecycle fixture | `key_lifecycle_output_redaction_sentinel_scan` | `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs` | yes | no | low/medium | P0 | no leak-free public claim |
| Reject/no-mutation paths | decap failure, malformed/admission rejects, replay | NA0436, NA0303, vault/session tests | no unified cleanup scope test across pending/session/vault | `reject_paths_preserve_pending_session_vault_state` | `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs` | yes | no | medium | P0 | no exhaustive-testing claim |
| Encrypted-at-rest | vault `.qsv`, qsp session `.qsv` | vault tests, `session_state_at_rest.rs` | no single qsc cleanup evidence file combines both | `session_and_vault_encrypted_at_rest_boundaries` | `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs` | yes | no | medium | P0 | dependency/audit evidence remains separate |
| qshield-cli demo-local excluded | demo `pq_init_ss_hex`, demo session material | NA0444 policy boundary | not qsc runtime assurance | excluded; no NA-0446 qshield-cli mutation | none | n/a | n/a | claim-boundary | excluded | no service-readiness claim |
| refimpl excluded/backlog | provider/private/session material | NA0444 evidence-policy scope | separate runtime/reference boundary | excluded from immediate qsc test implementation | none | n/a | possible future exact scope | medium | backlog | no qsc runtime assurance from refimpl |

## Authorization decision

Selected classification:

`QSC_ZEROIZATION_TEST_SCOPE_IMPLEMENTATION_READY`

The implementation-ready scope is bounded to observable qsc behavior through
existing public/test APIs. It authorizes a future NA-0446 test implementation
lane for the exact file:

`qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs`

Required future invariants:

1. Pending handshake secret store is cleared after successful handshake
   completion for the tested roles.
2. Bounded reject paths preserve pending/session/vault state when the existing
   fail-closed behavior requires no mutation.
3. qsp session store remains absent before success and appears only after
   successful session establishment in the tested flow.
4. qsp session blobs and vault files do not contain selected plaintext
   lifecycle sentinel values.
5. Tampered or peer-mismatched encrypted session state rejects without
   mutation where exercised.
6. qsc output does not emit selected route-token, passphrase, payload, pending,
   panic, or backtrace sentinels.
7. Runtime memory zeroization, allocator overwrite behavior, side-channel
   behavior, all-material coverage, refimpl coverage, and qshield-cli coverage
   remain explicit residuals.

No implementation mutation occurs in this directive. No runtime, crypto,
dependency, Cargo, lockfile, workflow, current test source, fuzz target, vector,
formal model, public, service, backup, qwork/qstart/qresume/qshell, qsl-backup,
status/plan, rollback, README, START_HERE, or website mutation is authorized by
NA-0445.

## Successor selection

Selected exact NA-0446 successor:

`NA-0446 -- QSL qsc Key Lifecycle Secret Cleanup / Zeroization Test Implementation Harness`

Rationale:

- exact future test path is known;
- exact bounded invariants are known;
- existing public/test APIs are sufficient for the bounded assertions;
- memory overwrite and side-channel residuals are explicitly out of scope;
- no runtime or crypto change is needed for the selected tests;
- qsc runtime remains higher priority than refimpl and qshield-cli residuals.

## Future path/scope bundle

Future NA-0446 allowed mutation paths:

- `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs`
- `docs/governance/evidence/NA-0446_qsl_qsc_key_lifecycle_secret_cleanup_zeroization_test_implementation_harness.md`
- `tests/NA-0446_qsl_qsc_key_lifecycle_secret_cleanup_zeroization_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0446 read-only inspection paths:

- `qsl/qsl-client/qsc/src/`
- `qsl/qsl-client/qsc/tests/`
- `formal/`
- `inputs/`
- `docs/governance/evidence/`
- `qsl/qsl-client/qsc/fuzz/`
- relevant scripts/workflows read-only.

Future forbidden unless a later exact directive authorizes it:

- runtime or crypto implementation changes;
- dependency changes;
- Cargo manifest or lockfile changes;
- workflow changes;
- test source changes outside `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs`;
- fuzz target source changes;
- vector changes;
- formal model changes;
- public docs, README, START_HERE, or website changes;
- qsl-server or qsl-attachments changes;
- qshield runtime changes;
- qshield-cli demo-local mutation;
- backup, restore, qsl-backup, backup status, backup plan, rollback, or backup
  tree changes;
- public claims.

## Future validation/marker plan

Common NA-0446 governance markers:

- `NA0446_QSC_ZEROIZATION_TEST_SCOPE_CONSUMED_OK`
- `NA0446_NEXT_SCOPE_SELECTED_OK`
- `NA0446_NO_RUNTIME_CHANGE_OK`
- `NA0446_NO_DEPENDENCY_CHANGE_OK`
- `NA0446_NO_WORKFLOW_CHANGE_OK`
- `NA0446_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0446_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0446_NO_SECRET_MATERIAL_COMPLETE_CLAIM_OK`
- `NA0446_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0446_ONE_READY_INVARIANT_OK`
- `NA0446_TEST_IMPLEMENTATION_SCOPE_OK`
- `NA0446_EXACT_TEST_PATHS_SELECTED_OK`

Expected qsc test markers for the future implementation:

- `NA0446_PENDING_HANDSHAKE_CLEANUP_OK`
- `NA0446_PENDING_REJECT_NO_MUTATION_OK`
- `NA0446_SESSION_STORE_BOUNDARY_OK`
- `NA0446_SESSION_ENCRYPTED_AT_REST_OK`
- `NA0446_VAULT_REDACTION_BOUNDARY_OK`
- `NA0446_REDACTION_BOUNDARY_OK`
- `NA0446_RUNTIME_MEMORY_ZEROIZATION_RESIDUAL_OK`

Expected NA-0446 validation:

- qwork proof-file verification without running qwork/qstart/qresume;
- queue and decision proof;
- exact path scope guard;
- `cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture`;
- inherited provider-error no-mutation test;
- qsc `send_commit`;
- refimpl `pqkem768`;
- adversarial script syntax checks;
- qsc-adversarial-smoke through CI;
- root cargo audit;
- nested qsc fuzz lock audit;
- cargo tree dependency probes;
- cargo fmt;
- formal model checks;
- link-check;
- leak-scan;
- overclaim scan;
- PR body preflight;
- goal-lint;
- public-safety before and after merge.

## Public claim/external review/website boundary

This test-scope authorization is internal governance evidence only.

This authorization is not production readiness.
This authorization is not public-internet readiness.
This authorization is not public readiness.
This authorization is not crypto-complete proof.
This authorization is not side-channel-free proof.
This authorization is not secret-material-complete proof.
This authorization is not bug-free proof.
This authorization is not vulnerability-free proof.
This authorization is not perfect-crypto proof.
This authorization is not public technical paper content.

No README, START_HERE, public docs, or website update is authorized.

Cargo audit green is dependency-health evidence only. Future tests, if
implemented, must be described as bounded internal evidence only.

## Rejected alternatives

Runtime cleanup implementation in NA-0445:

- Rejected because NA-0445 is test-scope authorization only and runtime/crypto
  mutation is forbidden.

NA-0446 runtime-hook authorization:

- Rejected for first position because the bounded observable qsc test scope is
  exact and implementable through existing APIs. Runtime hooks remain a
  residual for direct memory overwrite/drop proof only.

NA-0446 split-triage:

- Rejected because the qsc runtime test path and bounded invariants are exact
  enough for a future test implementation lane.

refimpl-first successor:

- Rejected because NA-0444 selected immediate qsc test-scope authorization and
  refimpl remains separate evidence-policy scope.

qshield-cli documentation successor:

- Rejected because qshield-cli remains demo-local and must not outrank qsc
  runtime evidence.

RNG failure successor:

- Rejected for first position because F-0441-03 remains the next candidate
  after qsc cleanup/zeroization test implementation scope is consumed.

No action/backlog:

- Rejected because exact bounded qsc tests can be implemented without runtime,
  crypto, dependency, workflow, vector, formal, public, service, or backup
  mutation.

## Backup-impact statement

No backup was run. No restore was run. No sudo was run. qsl-backup was not
mutated. Backup status files, backup plan files, rollback subtree paths,
systemd/timer/fstab state, source lists, retention settings, backup scripts,
and `/backup/qsl` were not mutated.

qsl-backup checksum and source-list inclusion proof remain boundary evidence
only.

No off-host backup completion is implied.
No disaster recovery completion is implied.
No restore proof is implied.
No backup completion is implied.
No key-custody completion is implied.

## Next recommendation

Proceed to:

`NA-0446 -- QSL qsc Key Lifecycle Secret Cleanup / Zeroization Test Implementation Harness`

The future lane should implement only
`qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs` plus its governance
evidence/testplan and traceability files. It should keep direct runtime memory
zeroization, side-channel, refimpl, qshield-cli, RNG failure, public-claim,
backup/restore, service, vector, formal, workflow, dependency, Cargo, and
lockfile changes out of scope unless a later exact directive authorizes them.
