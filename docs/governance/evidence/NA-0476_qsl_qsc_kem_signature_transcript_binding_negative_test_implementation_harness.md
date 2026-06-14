Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-14

# NA-0476 QSL qsc KEM / Signature / Transcript Binding Negative Test Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0476 consumes the NA-0475 authorization scope and adds one qsc
integration-test harness:

`qsl/qsl-client/qsc/tests/kem_signature_transcript_binding_negative.rs`

The harness uses existing qsc public/test-visible behavior only: qsc CLI
handshake init/poll, mock relay queues, temporary qsc config roots, mock vault
helpers already used by qsc tests, frame mutation in relay-held A1/B1/A2
messages, and session-path/pending-state assertions. It does not mutate qsc
runtime/source code, crypto code, dependencies, Cargo manifests, lockfiles,
workflows, refimpl, fuzz targets, vectors, formal models, qsl-server,
qsl-attachments, qshield, qshield-cli, website, public docs, README,
START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status files,
backup plan files, rollback paths, or backup tree paths.

Implemented negative coverage is bounded internal qsc evidence only.

- No public readiness claim is made.
- No production readiness claim is made.
- No public-internet readiness claim is made.
- No external review completion claim is made.
- No crypto completion claim is made.
- No KEM completion claim is made.
- No signature completion claim is made.
- No identity completion claim is made.
- No transcript completion claim is made.
- No downgrade proof claim is made.
- No replay proof claim is made.
- No side-channel freedom claim is made.
- No vulnerability freedom claim is made.
- No bug freedom claim is made.
- No perfect crypto claim is made.

## Live NA-0476 scope

Live READY item at startup:

`NA-0476 -- QSL qsc KEM / Signature / Transcript Binding Negative Test Implementation Harness`

Allowed implementation path:

- `qsl/qsl-client/qsc/tests/kem_signature_transcript_binding_negative.rs`

Allowed governance paths:

- this evidence doc
- `tests/NA-0476_qsl_qsc_kem_signature_transcript_binding_negative_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden mutation scope preserved:

- qsc runtime/source outside the exact test file
- qsc crypto implementation
- dependencies, Cargo manifests, lockfiles, workflows
- fuzz targets, vectors, formal models
- refimpl
- qsl-server, qsl-attachments, qshield, qshield-cli
- website, public docs, README, START_HERE
- qwork, qstart, qresume, qshell
- backup, restore, qsl-backup, backup status, backup plan, rollback, backup
  tree, systemd, timers, fstab
- public technical paper content
- durable Director State Index output

Acceptance criteria:

- KEM wrong public key, stale public record, and mutated ciphertext reject.
- Signature wrong public-record and cross-message signature replay reject.
- Transcript mutation, replay, and suite-confusion reject.
- Stale public-record / identity rollback behavior rejects.
- Selected negative cases do not create completed sessions or mutate protected
  session state unexpectedly.
- No runtime/source, crypto, dependency, workflow, refimpl, fuzz, vector, or
  formal mutation occurs.
- Exactly one READY item remains.

## qwork proof-file verification

Codex read the qwork proof files:

- `/srv/qbuild/work/NA-0476/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0476/.qwork/startup.qsl-protocol.json`

Verified required startup facts:

- `startup_result=OK`
- lane `NA-0476`
- repo `qsl-protocol`
- path `/srv/qbuild/work/NA-0476/qsl-protocol`
- clean worktree, index, and untracked state
- READY_COUNT 1
- sole READY item: NA-0476
- requested lane status: READY
- proof HEAD and proof `origin/main`: `15504bca439c`

Freshness proof:

- proof HEAD matched live HEAD before fetch;
- proof `origin_main` matched live `origin/main` before fetch;
- fetch did not advance `origin/main`;
- `origin/main` equals and descends from PR #1221 merge commit
  `15504bca439c`;
- PR #1221 was verified MERGED;
- current main public-safety completed success.

Codex did not run `qwork`, `qstart`, or `qresume`.

Recovered startup/implementation failures:

- Failing command: `cargo fmt --check` after adding the new Rust test file.
  Classification: recoverable formatting failure isolated to the newly added
  allowed test file. Corrective action: ran `cargo fmt` once and reran
  `cargo fmt --check`. Final result: PASS.
- Failing command: first pre-PR staged scope/link wrapper using
  `printf '---\n'`.
  Classification: recoverable command-shape mistake in the validation wrapper.
  Corrective action: reran the same staged proof with `printf '%s\n' '---'`.
  Final result: PASS; staged scope guard reported six allowed paths, zero
  forbidden paths, and link-check reported zero missing links.
- Failing command: first staged added-line overclaim scan.
  Classification: recoverable governance wording/scan-hygiene issue; matches
  were no-claim boundaries or testplan prohibition text, but wrapped lines
  separated sensitive phrases from the local no-claim wording.
  Corrective action: rewrote the affected allowed governance text so each
  sensitive phrase is locally attached to `No` or `must not`.
  A subsequent rerun caught a literal token in this recovery note; that note
  was rewritten. Final result: PASS; affirmative overclaim count zero.
- Failing command: first PR-body preflight over the draft PR body.
  Classification: recoverable PR-body wording issue; the draft no-claim line
  used a helper-prohibited readiness token.
  Corrective action: rewrote the draft PR body to use the repo-standard
  readiness no-claim wording.
  A subsequent staged overclaim rerun caught this recovery note's literal
  token; that note was rewritten.
  Final result: PASS on rerun; required fields present and prohibited phrase
  count zero.

## NA-0475 inheritance

NA-0475 completed D-0938 and selected
`BINDING_NEGATIVE_TEST_COMBINED_SCOPE_READY`.

Inherited exact future implementation path:

`qsl/qsl-client/qsc/tests/kem_signature_transcript_binding_negative.rs`

Inherited required surfaces:

- KEM wrong public key and stale public record;
- KEM wrong or corrupted ciphertext where feasible through existing qsc APIs;
- signature wrong identity / wrong public record;
- B1/A2 cross-message replay or wrong message-context signature rejection;
- transcript mutation;
- transcript replay;
- suite confusion / downgrade-style wrong-suite behavior;
- stale public-record replay / identity rollback where feasible through qsc
  temp roots and fixture state;
- no unintended completed-session mutation on rejected negative cases.

NA-0475 did not authorize runtime/source hooks, refimpl mutation, vector
mutation, fuzz mutation, or formal mutation. NA-0476 preserves those
boundaries.

## Negative test implementation summary

The new qsc integration file implements five tests:

- `common_na0476_markers`
- `kem_wrong_public_key_and_stale_record_reject_without_session_mutation`
- `kem_ciphertext_and_transcript_mutation_reject_without_completed_session`
- `signature_wrong_identity_and_cross_message_replay_reject_without_session_mutation`
- `replay_and_suite_confusion_reject_without_session_mutation`

The harness uses:

- `common::TestIsolation` for deterministic temp roots;
- `common::init_mock_vault` for encrypted mock vaults;
- qsc CLI `identity`, `contacts`, `relay`, and `handshake` commands;
- `common::start_inbox_server` plus mock relay `drain_channel` and
  `replace_channel`;
- local frame helpers for QHSM v2 parameter blocks, payload offsets, B1 KEM
  ciphertext mutation, B1 transcript-field mutation, B1 signature splicing,
  A2 signature capture, and suite-block replacement;
- local mock-vault JSON helpers only for disposable temp-root fixture state.

No external network is used. The only relay is the in-process qsc test server.

## KEM negative binding proof

Implemented cases:

- wrong peer KEM public key: Bob pins Alice's original identity, then receives
  A1 from a different Alice identity with a different KEM public key. Bob emits
  `peer_mismatch` / `identity_mismatch`, no session is created, and no B1 is
  emitted.
- stale KEM public record / stale trusted public record: Alice and Bob first
  complete a valid session. Alice then rotates identity, while Bob keeps the
  old trusted public record. Bob rejects the new A1 with `peer_mismatch`, and
  Bob's existing session bytes remain unchanged.
- wrong/corrupted KEM ciphertext: the test mutates the captured B1 KEM
  ciphertext before Alice consumes it. Alice rejects fail-closed, accepting
  either `pq_decap_failed` or explicit transcript-context rejection depending
  on provider decapsulation behavior, and no A2 or completed session is
  created.

Markers:

- `NA0476_KEM_WRONG_PUBLIC_KEY_REJECT_OK`
- `NA0476_KEM_STALE_PUBLIC_RECORD_REJECT_OK`
- `NA0476_KEM_WRONG_CIPHERTEXT_REJECT_OK`

## Signature negative binding proof

Implemented cases:

- wrong signature public record: Alice's temp-root contact record for Bob is
  given an intentionally wrong signature fingerprint inside the encrypted mock
  vault. Bob emits a valid B1, Alice verifies the B1 signature, then rejects
  the stale/wrong signature public record with `peer_mismatch`. No A2 or
  completed Alice session is created.
- cross-message signature replay: the test captures an A2 signature from a
  separate qsc exchange and splices it into the B1 signature field for a fresh
  exchange. The B1 transcript MAC remains structurally testable while the B1
  signature verifier rejects with `sig_invalid`. No completed session or A2 is
  created for the mutated exchange.

Markers:

- `NA0476_SIGNATURE_WRONG_IDENTITY_REJECT_OK`
- `NA0476_SIGNATURE_CROSS_MESSAGE_REPLAY_REJECT_OK`

## Transcript / replay / suite negative binding proof

Implemented cases:

- transcript mutation: the test mutates a B1 transcript-bound field after Bob
  emits B1 and before Alice consumes it. Alice rejects with
  `REJECT_QSC_HS_TRANSCRIPT_CONTEXT`.
- transcript replay: Bob receives a replayed A1 while responder pending state
  for the same session exists. Bob rejects with `REJECT_QSC_HS_REPLAY` and no
  session is created.
- suite confusion / downgrade-style wrong-suite behavior: the test replaces
  A1's QHSM v2 suite parameter block with the legacy tuple while Bob is in
  `suite-required` mode. Bob rejects with `REJECT_QSC_HS_DOWNGRADE`.

Markers:

- `NA0476_TRANSCRIPT_MUTATION_REJECT_OK`
- `NA0476_TRANSCRIPT_REPLAY_REJECT_OK`
- `NA0476_SUITE_CONFUSION_REJECT_OK`

## Stale public-record / identity rollback proof

The stale-record case exercises identity rollback-like risk with temp roots:
Alice rotates after a valid session while Bob retains the stale trusted public
record. Bob rejects the new A1, preserves the existing session bytes, and does
not emit B1.

Marker:

- `NA0476_STALE_PUBLIC_RECORD_REJECT_OK`

## No session mutation proof

The harness asserts that rejected negative cases do not create completed
sessions. For the stale-record case with a pre-existing accepted session, the
test captures Bob's session bytes before rejection and compares them after
rejection. For pre-session negative cases, the relevant `qsp_sessions/*.qsv`
paths remain absent and relay output channels that would carry B1/A2 remain
empty.

Marker:

- `NA0476_NEGATIVE_TESTS_NO_SESSION_MUTATION_OK`

## Caveats and bounded evidence

These tests are bounded qsc integration evidence only.

KEM ciphertext mutation accepts either provider-level decapsulation rejection or
explicit transcript-context rejection because the public qsc API exposes only
the fail-closed outcome, and provider decapsulation behavior may reject at
different internal points.

Cross-message signature replay is implemented by splicing an A2 signature into
a B1 signature field. Parser-level B1-as-A2 and A2-as-B1 frame-type confusion is
already rejected by QHSM framing, so this test selects a signature verifier
wrong-context replay that reaches the signature check without source hooks.

The tests do not claim complete KEM, signature, identity, transcript,
downgrade, replay, side-channel, secret-material, vulnerability, or crypto
assurance.

## No source/runtime mutation proof

Changed implementation path is exactly:

- `qsl/qsl-client/qsc/tests/kem_signature_transcript_binding_negative.rs`

No qsc runtime/source file under `qsl/qsl-client/qsc/src/**` is modified.

## No dependency/workflow/refimpl/qshield-cli mutation proof

No dependency, Cargo manifest, lockfile, workflow, refimpl, qshield runtime,
qshield-cli, qsl-server, qsl-attachments, fuzz, vector, formal, website,
README, START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status,
backup plan, rollback, or backup tree path is changed by the implementation
lane.

## qsc/refimpl mapping residual

qsc/refimpl mapping remains supporting-only. The harness imports no refimpl
behavior for the negative path and does not alter refimpl. A future mapping lane
can relate qsc state transitions and refimpl/formal evidence without blocking
this bounded qsc test lane.

## Formal/vector/fuzz residual

Formal model mapping remains a residual. Negative vectors remain a residual.
Fuzz binding remains a residual. This PR does not mutate formal models,
vectors, fuzz targets, or fuzz lockfiles.

## Side-channel / secret-material caveat

The harness checks observable qsc fail-closed behavior and basic no-secret
output boundaries in rejection output. It does not prove side-channel freedom,
complete secret-material cleanup, complete memory erasure, allocator behavior,
or all-material coverage.

## External-review readiness residual

This evidence improves internal qsc binding validation but does not complete
external review readiness. External-review packaging remains future work after
formal/model mapping and other residuals are addressed.

## Release-claim boundary

- No public-readiness claim is made.
- No production-readiness claim is made.
- No public-internet-readiness claim is made.
- No external-review-complete claim is made.
- No crypto-complete claim is made.
- No KEM-complete claim is made.
- No signature-complete claim is made.
- No identity-complete claim is made.
- No transcript-complete claim is made.
- No downgrade-proof claim is made.
- No replay-proof claim is made.
- No RNG-failure-complete claim is made.
- No provider-RNG-complete claim is made.
- No secret-material-complete claim is made.
- No side-channel-free claim is made.
- No vulnerability-free claim is made.
- No bug-free claim is made.
- No perfect-crypto claim is made.
- Cargo audit green is dependency-health evidence only.

## Validation

Startup validation completed:

- qwork proof-file verification: PASS.
- PR #1221 merged at `15504bca439c`: PASS.
- queue proof: READY_COUNT 1, READY NA-0476.
- decision proof: latest D-0939, D-0940 absent at startup, duplicate count 0.
- public-safety on current main: PASS.
- root `cargo audit --deny warnings`: PASS.
- nested qsc fuzz lock audit: PASS.
- inherited qsc cfg/no-cfg provider RNG, TUI bootstrap, legacy/public-record,
  lazy identity, A2, B1, KEM, key lifecycle, and provider-error tests: PASS.

Implementation validation completed:

- `cargo fmt --check`: PASS after one formatting recovery.
- `cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture`: PASS, 5 passed.
- required NA0476 markers were present in source and output.
- inherited qsc cfg/no-cfg provider RNG tests: PASS.
- `cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture`: PASS.
- `cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture`: PASS.
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`: PASS.
- `cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768`: PASS.
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`: PASS.
- `python3 formal/run_model_checks.py`: PASS.
- root `cargo audit --deny warnings`: PASS.
- nested qsc fuzz lock audit: PASS.
- dependency probes: `rustls-webpki` and `ml-kem` present; pqcrypto inverse
  probes returned package-ID absence.
- `sh -n scripts/ci/qsc_adversarial.sh`: PASS.
- `bash -n scripts/ci/qsc_adversarial.sh`: PASS.
- local `sh scripts/ci/qsc_adversarial.sh`: adversarial and provider-error
  steps passed, then local execution stopped at missing `cargo fuzz`. This is
  recorded as a local tool-availability caveat; PR CI `qsc-adversarial-smoke`
  remains required.
- staged scope guard: PASS.
- `git diff --cached --check`: PASS.
- link-check: PASS.
- staged added-line overclaim scan: PASS after wording recovery.
- PR body preflight: PASS after draft wording recovery.

Full CI validation and post-merge validation are recorded in the response
evidence bundle and rolling journal.

## Scope guard

Allowed NA-0476 changed paths:

- `qsl/qsl-client/qsc/tests/kem_signature_transcript_binding_negative.rs`
- `docs/governance/evidence/NA-0476_qsl_qsc_kem_signature_transcript_binding_negative_test_implementation_harness.md`
- `tests/NA-0476_qsl_qsc_kem_signature_transcript_binding_negative_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Optional closeout, if executed after merge and green post-merge public-safety,
may update only the separately allowed closeout paths.

## Backup-impact statement

No backup is run. No restore is run. No qsl-backup path is mutated. Backup
status and backup plan files remain read-only. The qsl-backup script SHA and
script-local ops source inclusion count were verified at startup.

## Successor selection

Default successor selected:

`NA-0477 -- QSL KEM / Signature / Transcript Formal Model Mapping Authorization Plan`

Rationale: NA-0476 implements direct qsc negative tests. The next highest-value
residual is formal-model mapping for KEM, signature, transcript, identity,
suite, replay, downgrade, and stale public-record binding evidence before any
stronger assurance or external review claim.

## Next recommendation

Merge the NA-0476 implementation PR only after local validation and required PR
checks pass. If post-merge public-safety is green, close out NA-0476 and
restore NA-0477 as the sole READY item without implementing NA-0477.
