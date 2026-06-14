Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-14

# NA-0475 QSL qsc KEM / Signature / Transcript Binding Negative Test Scope Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0475 consumes NA-0474 and authorizes the exact future qsc negative-test
implementation scope for KEM, signature, transcript, identity, stale
public-record, replay, downgrade, and suite-confusion binding gaps.

Primary classification:

`BINDING_NEGATIVE_TEST_COMBINED_SCOPE_READY`

Selected successor:

`NA-0476 -- QSL qsc KEM / Signature / Transcript Binding Negative Test Implementation Harness`

The read-only inventory found that existing qsc integration tests already
exercise the needed public and test-visible surfaces: qsc CLI handshake
init/poll, suite-required mode, mock relay frame capture/reinjection,
temporary qsc config roots, encrypted qsc vault/session fixture readers used by
tests, identity/public-record files under test roots, and existing no-mutation
assertions. Future NA-0476 can therefore implement a single bounded qsc
integration test file without qsc runtime/source mutation, crypto mutation,
dependency mutation, Cargo/lockfile mutation, workflow mutation, refimpl
mutation, fuzz/vector/formal mutation, qsl-server mutation, qsl-attachments
mutation, qshield runtime mutation, qshield-cli mutation, website mutation, or
public-doc mutation.

Formal mapping, negative vectors, fuzz binding, and qsc/refimpl mapping remain
supporting-only residuals. They should not precede the selected qsc test-only
implementation lane because the P1 qsc negative-test surfaces are exact enough
and can be exercised through existing public/test-visible behavior.

NA-0475 is authorization-only. It does not implement tests and does not change
runtime behavior. No public-readiness claim is introduced. No
production-readiness claim is introduced. No public-internet-readiness claim is
introduced. No external-review-complete claim is introduced. No crypto-complete
claim is introduced. No KEM-complete claim is introduced. No signature-complete
claim is introduced. No identity-complete claim is introduced. No
transcript-complete claim is introduced. No downgrade-proof claim is introduced.
No replay-proof claim is introduced. No side-channel-free claim is introduced.
No vulnerability-free claim is introduced. No bug-free claim is introduced. No
perfect-crypto claim is introduced. Cargo audit green remains dependency-health
evidence only.

## Live NA-0475 scope

Live READY item:

`NA-0475 -- QSL qsc KEM / Signature / Transcript Binding Negative Test Scope Authorization Plan`

Allowed NA-0475 mutation paths:

- `docs/governance/evidence/NA-0475_qsl_qsc_kem_signature_transcript_binding_negative_test_scope_authorization_plan.md`
- `tests/NA-0475_qsl_qsc_kem_signature_transcript_binding_negative_test_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Read-only inspection included qwork proof files, governance evidence and
testplans, `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`, rolling
journal, qsc handshake/identity/source/test/fuzz surfaces, refimpl source/tests,
formal models, vector inputs, Cargo manifests and lockfiles, scripts,
workflows, qsl-backup hash evidence, backup status/plan files, and prior
response files.

Forbidden mutation scope was preserved for implementation, runtime, crypto,
dependency, Cargo, lockfile, workflow, executable qsc test, fuzz target,
vector, formal model, refimpl, qsl-server, qsl-attachments, qshield runtime,
qshield-cli, website, public docs, README, START_HERE, qwork, qstart, qresume,
qshell, qsl-backup, backup status, backup plan, rollback subtree, backup tree,
public technical paper, and durable Director State Index output paths.

Acceptance criteria:

- qwork proof files verified without rerunning qwork;
- NA-0474 consumed;
- KEM, signature, transcript/replay/suite, and stale public-record/identity
  rollback scope reviews completed;
- candidate negative-test surface inventory completed;
- combined vs split decision recorded;
- exact future implementation path and markers selected;
- no implementation mutation in NA-0475;
- no public overclaim;
- exactly one READY item remains mandatory.

Stop conditions preserved: stale qwork proof, PR #1219 not merged, unexpected
queue/decision state, omitted KEM/signature/transcript/stale-record review,
unsafe successor selection, root or nested audit failure, qsl-backup source-list
regression, public-safety red or missing, more than one READY item, any
forbidden mutation, or any prohibited readiness/completion/security claim.

## qwork proof-file verification

Codex read the qwork proof files:

- `/srv/qbuild/work/NA-0475/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0475/.qwork/startup.qsl-protocol.json`

Verified required startup facts:

- `startup_result=OK`
- lane `NA-0475`
- repo `qsl-protocol`
- path `/srv/qbuild/work/NA-0475/qsl-protocol`
- clean worktree, index, and untracked state
- READY_COUNT 1
- sole READY item: NA-0475
- requested lane status: READY
- proof HEAD and proof `origin/main`: `ea5b650502e8`

Freshness proof:

- proof HEAD matched live HEAD before fetch;
- proof `origin_main` matched live `origin/main` before fetch;
- fetch did not advance `origin/main`;
- `origin/main` equals and descends from PR #1219 merge commit
  `ea5b650502e8`;
- PR #1219 was verified MERGED;
- current main public-safety completed success.

Codex did not run `qwork`, `qstart`, or `qresume`.

Recovered startup and validation failures:

- Failing command: inline queue/decision checker over the compact
  `qsl_evidence_helper.py queue` and `decisions` output.
- Classification: recoverable command-shape/proof-script assumption; the queue
  helper output is not an all-item historical inventory.
- Corrective action: direct bounded parser over `NEXT_ACTIONS.md` and
  `DECISIONS.md`.
- Final result: READY_COUNT 1, READY NA-0475, NA-0474 through NA-0435 DONE,
  NA-0434 and NA-0429 BLOCKED, D-0936 once, D-0937 once, D-0938 absent, and
  duplicate decision count zero.
- Failing command: first staged scope guard using only `git diff --name-only`.
  Classification: recoverable command-shape issue because it did not prove
  staged ignored evidence files or untracked paths.
  Corrective action: used an explicit staged-path allowlist plus untracked-file
  proof and forced staging for the ignored evidence path.
  Final result: PASS; staged paths were exactly the five NA-0475 allowed
  governance files.
- Failing command: direct decision parser using heading-only decision detection
  after D-0938 was added.
  Classification: recoverable parser-shape issue because current decisions use
  `- **ID:**` records for recent entries.
  Corrective action: switched the direct parser to the current decision ID
  record shape.
  Final result: PASS; latest decision D-0938, D-0936/D-0937/D-0938 each once,
  D-0939 absent, duplicate decision count zero.
- Failing command: first added-line overclaim scan after the governance patch.
  Classification: recoverable wording/scan-hygiene issue because the matches
  were explicit no-claim boundaries or wrapped no-claim lines, not affirmative
  public claims.
  Corrective action: rewrote the new evidence and journal no-claim wording so
  each sensitive term has an explicit local no-claim sentence, then reran an
  added-line overclaim scan.
  Final result: PASS; added affirmative overclaim count zero.
- Failing command: local PR-body goal-lint preflight with a synthetic event
  missing pull-request base/head SHAs.
  Classification: recoverable command-shape issue; PR body text itself passed
  required field and wording checks, while goal-lint needs a committed head.
  Corrective action: record PR-body text preflight before commit and rerun
  goal-lint after commit/PR with real base/head context.
  Final result: PR-body text preflight PASS; post-commit/PR goal-lint remains a
  required validation step.
- Failing command: local `scripts/ci/qsc_adversarial.sh`.
  Classification: recoverable local tool availability issue under the directive
  allowance for local cargo-fuzz absence.
  Corrective action: recorded exact local output and preserved PR CI
  `qsc-adversarial-smoke` as required external validation.
  Final result: stable adversarial tests passed locally, including
  `NA0439_QSC_PROVIDER_ERROR_NO_MUTATION_ADVERSARIAL_STEP` and
  `handshake_provider_error_no_mutation`; local fuzz step stopped at missing
  `cargo fuzz`.

## NA-0474 inheritance

NA-0474 is DONE and D-0936/D-0937 are accepted.

Inherited classification:

- `BINDING_NEGATIVE_TEST_SCOPE_NEXT`

Inherited P1 findings:

- F-0474-01 KEM binding negative tests.
- F-0474-02 signature binding/domain-separation negative tests.
- F-0474-03 transcript mutation/replay tests.
- F-0474-04 stale public-record / identity rollback tests.
- F-0474-05 downgrade / suite-confusion tests.

Inherited supporting-only residuals:

- F-0474-06 qsc/refimpl mapping residual.
- F-0474-07 formal-model mapping residual.
- F-0474-08 negative vectors residual.
- F-0474-09 fuzz binding residual.
- F-0474-10 side-channel caveat.

NA-0474 selected a combined authorization lane because KEM, signature,
transcript, identity, stale public-record, replay, downgrade, and
suite-confusion gaps are adjacent and can share the same qsc integration
harness setup.

## Applicable Stewardship and Assurance Review

Level 1 stewardship is active. Level 2 and Level 3 remain future-gated. No
separate Directors are created. No steward has independent READY promotion,
merge authority, public-claim authority, or directive authority. Lead Director
final authority is preserved.

| Review item | Classification | Evidence | Disposition |
|---|---|---|---|
| Crypto / Protocol Steward | COMBINED_QSC_NEGATIVE_TEST_SCOPE_READY | qsc handshake/identity code, NA-0474 findings, existing qsc frame mutation tests | Select one qsc test-only implementation lane |
| CI / Dependency / Release Health Steward | DEPENDENCY_HEALTH_ACCEPTED_SUPPORTING_ONLY | root cargo audit, nested qsc fuzz lock audit, cargo tree probes, public-safety | Supporting only; no vulnerability-free claim |
| Public Claims / External Review Steward | EXTERNAL_REVIEW_READINESS_INCREMENTAL | evidence chain, residual formal/vector/fuzz/external review work | No public claim expansion |
| Product / Demo / Service Boundary Steward | SERVICE_BOUNDARY_UNCHANGED | no qsl-server, qsl-attachments, qshield, qshield-cli, website, README, or public-doc mutation | Service and demo boundaries unchanged |
| Local Ops / Backup / Restore Steward | LOCAL_OPS_READ_ONLY_OK | qwork proof, qsl-backup hash/source-list proof, response archive path | No backup/restore; no off-host or restore claim |
| Best-Known-Method Review | BEST_KNOWN_METHOD_FOR_SCOPE | direct source/test inventory shows existing qsc test API can construct the cases | Combined test-only successor is selected |
| Hostile Cryptographer Review | BINDING_NEGATIVE_TEST_COMBINED_SCOPE_READY | wrong keys, stale records, replay, transcript, role, suite, and identity confusion reviewed | Include all P1 binding negatives in NA-0476 |
| Red-Team Review | REPLAY_ROLLBACK_DOWNGRADE_INCLUDED_NEXT | replayed A1/B1/A2, stale record replay, identity rollback, and downgrade/suite confusion are exact | Bundle with KEM/signature/transcript tests |
| Production SRE Review | RECOVERY_AND_CLAIM_BOUNDARY_RESIDUAL | rollback and stale local state remain operationally relevant | Test-only lane first; SRE/runbook residual remains |
| Side-Channel Caveat | SIDE_CHANNEL_RESIDUAL_ACTIVE | no constant-time or complete memory-erasure proof in scope | Carry caveat; no side-channel-free claim |
| Formal-Model Mapping Residual | FORMAL_MODEL_MAPPING_RESIDUAL_ACTIVE | qsc suite-id model is crypto-agnostic and abstract | Formal mapping does not precede NA-0476 |
| External-Review Readiness | EXTERNAL_REVIEW_READINESS_INCREMENTAL | evidence is internal governance/test evidence only | External package remains future work |
| Release-Claim Boundary | RELEASE_CLAIM_BOUNDARY_PRESERVED | no public-doc/website/readiness edits | No readiness or completion claim |
| Assurance Gap Review Trigger | HIGHER_PRIORITY_RESIDUAL_SUPERSEDES_ASSURANCE_REVIEW | exact NA-0476 negative-test successor selected | No assurance-gap review required now |

## Candidate negative-test surface inventory

Future exact test path for selected rows:

`qsl/qsl-client/qsc/tests/kem_signature_transcript_binding_negative.rs`

| # | Candidate surface | Current coverage | Coverage class | Existing qsc public/test API can construct? | Future test-only helper needed? | Runtime source mutation needed? | Refimpl mutation needed? | Expected rejection marker | No-mutation expectation | Priority | Grouping |
|---:|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wrong peer KEM public key | primary identity mismatch tests; A1 KEM fingerprint check | Supporting | Yes, inject A1 from alternate identity | Yes, frame/fixture helper | No | No | `identity_mismatch` or `identity_pin_failed`; `NA0476_KEM_WRONG_PUBLIC_KEY_REJECT_OK` | no B1/session/pending mutation beyond reject rules | P1 | KEM |
| 2 | stale KEM public key / stale public record | identity mismatch and rotation tests support | Supporting | Yes, restore old public record in temp config | Yes, public-record snapshot helper | No | No | `identity_mismatch`; `NA0476_KEM_STALE_PUBLIC_RECORD_REJECT_OK` | existing session/contact state unchanged | P1 | KEM, identity |
| 3 | replayed KEM ciphertext | provider-error decap no-mutation covers corrupted pending secret; suite tests cover replay patterns | Supporting | Yes, replay captured B1 into wrong/old pending state | Yes, relay replay helper | No | No | `pq_decap_failed`, `bad_transcript`, or suite reject; `NA0476_TRANSCRIPT_REPLAY_REJECT_OK` | no new session and pending unchanged or cleared per existing reject contract | P1 | KEM, replay |
| 4 | corrupted KEM ciphertext | provider-error no-mutation covers decap failure path | Direct/supporting | Yes, mutate B1 KEM ciphertext bytes | Yes, B1 offset helper | No | No | `pq_decap_failed`; `NA0476_KEM_WRONG_CIPHERTEXT_REJECT_OK` | no session mutation and pending store preserved or rejected consistently | P1 | KEM |
| 5 | B1 signature replay as A2 | B1/A2 label separation exists; no exact replay test | None/direct code evidence | Yes, place B1 bytes in A2 channel or splice B1 signature into A2 frame | Yes, frame splice helper | No | No | `decode_failed`, `bad_confirm`, or `a2_verify`; `NA0476_SIGNATURE_CROSS_MESSAGE_REPLAY_REJECT_OK` | responder session remains absent/unchanged | P1 | Signature, replay |
| 6 | A2 signature replay as B1 | B1/A2 label separation exists; no exact replay test | None/direct code evidence | Yes, place A2 bytes in B1 channel or splice A2 signature into B1 frame | Yes, frame splice helper | No | No | `decode_failed`, `bad_transcript`, or `b1_verify` | initiator session/pending state not advanced | P1 | Signature, replay |
| 7 | wrong B1 signature public key | B1 signature tamper only | Supporting | Yes, splice B1 signature public key from alternate identity | Yes, B1 offset helper | No | No | `b1_verify` or optional sig pin reject; `NA0476_SIGNATURE_WRONG_IDENTITY_REJECT_OK` | no initiator session commit | P1 | Signature |
| 8 | wrong A2 signature public key | A2 signature tamper only | Supporting | Yes, use A2 from alternate identity or stale public record | Yes, fixture/helper | No | No | `a2_verify` or optional sig pin reject | responder session remains absent/unchanged | P1 | Signature |
| 9 | wrong identity / wrong public record | first-contact and primary pin mismatch tests | Direct for primary pin; broader supporting | Yes, alternate identity and public-record replacement | Yes, fixture helper | No | No | `identity_mismatch`; `NA0476_SIGNATURE_WRONG_IDENTITY_REJECT_OK` | accepted session unchanged | P1 | Identity |
| 10 | wrong role signature | labels `QSC.HS.SIG.B1` and `QSC.HS.SIG.A2` exist; no exact role test | None/direct code evidence | Yes, cross-role signature replay/splice | Yes, frame splice helper | No | No | `b1_verify` or `a2_verify` | no role-based accept or session mutation | P1 | Signature |
| 11 | wrong suite signature | suite mismatch tests exist; signature-specific suite confusion not exact | Supporting | Yes, alter suite block before signature verify | Yes, suite block helper exists in tests | No | No | `REJECT_QSC_HS_CONTEXT_MISMATCH` or verify failure | no session commit | P1 | Signature, suite |
| 12 | wrong session ID signature | session mismatch checks exist; signature-specific not exact | Supporting | Yes, mutate session ID in frame | Yes, session-id helper exists in tests | No | No | `session_id_mismatch` or verify failure | no session commit | P1 | Signature, transcript |
| 13 | mutated transcript before B1 verify | suite/transcript mismatch tests mutate B1 transcript field | Direct for selected field | Yes, mutate B1 transcript field | Yes, reuse offset helper | No | No | `REJECT_QSC_HS_TRANSCRIPT_CONTEXT`; `NA0476_TRANSCRIPT_MUTATION_REJECT_OK` | no initiator session, pending cleared/unchanged by contract | P1 | Transcript |
| 14 | mutated transcript before A2 verify | A2 confirm mismatch tests exist via suite/context | Supporting | Yes, mutate A2 MAC/signature bytes | Yes, A2 offset helper | No | No | `bad_confirm` or `a2_verify`; `NA0476_TRANSCRIPT_MUTATION_REJECT_OK` | responder session absent/unchanged | P1 | Transcript |
| 15 | transcript truncation | frame decode rejects malformed lengths | Supporting | Yes, truncate captured A1/B1/A2 bytes | Yes, truncation helper | No | No | `decode_failed` mapped to sanitized suite/decode reject | no output and no session mutation | P1 | Transcript |
| 16 | mixed-context transcript | suite context mismatch and key-context missing tests | Direct/supporting | Yes, mix A1/B1/A2 from different sessions | Yes, relay mix helper | No | No | `REJECT_QSC_HS_CONTEXT_MISMATCH`, `REJECT_QSC_HS_KEY_CONTEXT`, or transcript reject | no session mutation | P1 | Transcript, suite |
| 17 | stale public-record replay | primary pin mismatch supports stale record rejection | Supporting | Yes, save old record, rotate, replay old A1/record | Yes, public-record helper | No | No | `identity_mismatch`; `NA0476_STALE_PUBLIC_RECORD_REJECT_OK` | existing accepted session/contact state unchanged | P1 | Identity |
| 18 | public-record rollback | no direct rollback fixture; rotation tests support | Supporting | Yes, restore old identity/public-record files inside temp root | Yes, rollback fixture helper | No | No | `identity_mismatch` or signature/KEM reject | no new accepted stale session | P1 | Identity |
| 19 | identity rotation stale peer state | CLI rotation and identity mismatch support | Supporting | Yes, rotate one peer and keep stale contact/session | Yes, fixture helper | No | No | `identity_mismatch` or pin failure | old session unchanged; no silent re-pin | P1 | Identity |
| 20 | cross-suite downgrade / suite confusion | NA-0313 direct suite-required negative matrix | Direct for suite context | Yes, mutate suite block/version | Reuse existing helper | No | No | `REJECT_QSC_HS_DOWNGRADE`, context mismatch; `NA0476_SUITE_CONFUSION_REJECT_OK` | no output/no session mutation | P1 | Suite |
| 21 | wrong-role message replay | replayed A1/A2 direct; B1 role broader | Supporting | Yes, deliver A1/B1/A2 on wrong channel/order | Yes, relay ordering helper | No | No | `REJECT_QSC_HS_REPLAY`, decode reject, or verify reject | no session mutation | P1 | Replay |
| 22 | replayed A1 | NA-0313 direct replayed A1 with suite context | Direct | Yes, already patterned | Reuse existing helper | No | No | `REJECT_QSC_HS_REPLAY` | no B1/session mutation | P1 | Replay |
| 23 | replayed B1 | no exact replayed B1 row; B1 pending/session checks support | Supporting | Yes, replay captured B1 after pending consumed or into wrong pending | Yes, relay replay helper | No | No | `bad_transcript`, `session_id_mismatch`, `pq_decap_failed`, or replay reject | no duplicate session commit | P1 | Replay |
| 24 | replayed A2 | NA-0313 direct replayed A2 with suite context | Direct | Yes, already patterned | Reuse existing helper | No | No | `REJECT_QSC_HS_REPLAY`; `NA0476_TRANSCRIPT_REPLAY_REJECT_OK` | no duplicate responder commit | P1 | Replay |

## KEM negative-test scope review

Classification: `KEM_NEGATIVE_TEST_SCOPE_READY`

Wrong peer KEM public key can be tested without source mutation by using an
alternate identity's A1 or by replacing the KEM public key/public record in a
temporary test root. The responder-side primary identity pin compares the A1
KEM public-key fingerprint before encapsulation.

Stale KEM public record can be tested without source mutation by snapshotting
old `identities/self_*.json` material in a test root, rotating identity, then
replaying or restoring the stale record while peer contact pins remain current.

Replayed KEM ciphertext and wrong/corrupted ciphertext can be tested without
source mutation by capturing B1 from the mock relay, replaying it into an
incompatible pending state, or mutating its KEM ciphertext bytes. Existing test
helpers already parse B1 sizes/offsets and existing provider-error tests prove
the decap failure no-mutation path.

No-mutation invariant: rejection must not create a new qsp session, must not
silently advance peer confirmation, must not emit dependent A2/B1 output after
the rejected input, and must preserve or clear pending state only according to
the existing reject contract. No qsc runtime helper seam is required.

## Signature negative-test scope review

Classification: `SIGNATURE_NEGATIVE_TEST_SCOPE_READY`

B1/A2 cross-message replay can be tested without source mutation by capturing
B1 and A2 frames from the mock relay and replaying them in the wrong channel or
splicing wrong-role signature bytes into otherwise typed frames. qsc signature
messages use distinct labels for B1 and A2, so future tests should assert a
sanitized reject rather than any accept or session mutation.

Wrong signature public key can be tested without source mutation by splicing
the B1 or A2 signature public-key field from an alternate identity, or by using
stale public-record material in a temp root. Wrong role, wrong suite, and wrong
session ID signatures can be tested by reusing the existing suite block,
session-id, and frame mutation helpers already present in qsc integration
tests.

No-mutation/no-output invariant: rejection must not commit a qsp session, must
not emit `handshake_complete`, must not emit `recv_commit`, must not emit
dependent A2/B1 output, and must not leak route tokens, passphrases, secret
material, panic text, or stack traces. B1 and A2 should be covered in the same
future test file because they share setup, frame offsets, identity fixtures,
and label/domain separation assertions. No runtime source change is required.

## Transcript / replay / suite negative-test scope review

Classification: `TRANSCRIPT_REPLAY_NEGATIVE_TEST_SCOPE_READY`

Transcript mutation can be tested by mutating A1/B1/A2 bytes captured from the
mock relay. Existing NA-0313 helpers already mutate a B1 transcript field,
replace suite parameter blocks, set session IDs, and assert suite/transcript
reject markers.

Transcript truncation can be tested through existing frame parsing by
truncating captured frames before poll. Mixed-context transcript tests can use
A1, B1, and A2 captured from different test pairs/sessions, wrong session IDs,
wrong suite blocks, and wrong-role message ordering. Replayed A1 and A2 already
have direct patterns; replayed B1 can be added by delivering an old B1 after
the pending state was consumed or to a peer with a different pending KEM secret.

Downgrade/suite confusion is exact enough for the same future test file because
suite-required mode and suite block mutation helpers already exist. The future
file should bundle transcript/replay/suite cases with KEM and signature cases
because the mock relay setup and no-mutation assertions are shared. No runtime
source change is required.

## Stale public-record / identity rollback scope review

Classification: `STALE_PUBLIC_RECORD_NEGATIVE_TEST_SCOPE_READY`

Stale public-record replay can be tested without source mutation by saving an
old self public record, rotating identity, and replaying old A1/public-record
material against a peer that pins the current fingerprint. Identity rollback
can be simulated in a temporary qsc config root by restoring old identity files
and vault secret values using test-only fixture helpers patterned after
existing vault/session test readers. Stale peer/contact state can be tested by
keeping an old contact pin and injecting current or old peer frames.

The rejection/no-mutation invariant is no silent re-pin, no accepted stale qsp
session, no peer confirmation advancement, no rewrite of current contact pin on
reject, and no dependent handshake output after stale material is rejected.
Stale public-record tests should be included in the combined future test file
because they share the same identity setup, mock relay, and frame replay
helpers. No runtime source change is required.

## Combined vs split scope decision

| Option | Select/reject | Evidence | Future path if selected | Validation markers | Public-claim caveat |
|---|---|---|---|---|---|
| Option 1 - Combined KEM / Signature / Transcript Binding Negative Test Scope | SELECT | Most P1 surfaces share one qsc CLI/mock-relay/temp-config harness; test-only implementation is viable; no runtime/refimpl/dependency/workflow changes required | `qsl/qsl-client/qsc/tests/kem_signature_transcript_binding_negative.rs` | `NA0476_*_REJECT_OK`, `NA0476_NEGATIVE_TESTS_NO_SESSION_MUTATION_OK` | Internal test evidence only; no completion/readiness claims |
| Option 2 - KEM-only next | Reject | KEM is exact but shares setup with transcript/signature/stale-record tests | n/a | n/a | Would leave adjacent P1 gaps untested |
| Option 3 - Signature-only next | Reject | Signature replay/domain separation is exact but shares frames and suite/session context with KEM/transcript | n/a | n/a | Would delay KEM and transcript P1 cases |
| Option 4 - Transcript/replay/suite-only next | Reject | Existing suite helpers make this exact, but KEM/signature/stale-record cases can be bundled | n/a | n/a | Would over-focus on already stronger suite evidence |
| Option 5 - stale public-record/identity rollback next | Reject | Stale-record risk is P1 but can be tested in same qsc harness | n/a | n/a | Would delay KEM/signature/transcript P1 cases |
| Option 6 - path-specific split authorization | Reject | No runtime/source helper scope is needed; split would add process overhead | n/a | n/a | Split remains available only if NA-0476 proves unexpected source hooks are required |
| Option 7 - formal mapping first | Reject | Formal qsc suite-id model remains abstract, but exact qsc tests are actionable now | n/a | n/a | Formal mapping residual remains active |
| Option 8 - vector suite first | Reject | Negative vectors are useful later but qsc test-only evidence can precede them | n/a | n/a | Vector residual remains active |
| Option 9 - ambiguous triage | Reject | Successor can be selected safely with current evidence | n/a | n/a | No ambiguity stop needed |

## Authorization decision

Primary classification:

`BINDING_NEGATIVE_TEST_COMBINED_SCOPE_READY`

Required conclusions:

- NA-0474 is consumed.
- KEM negative-test scope is reviewed and ready.
- Signature negative-test scope is reviewed and ready.
- Transcript/replay/suite negative-test scope is reviewed and ready.
- Stale public-record/identity rollback scope is reviewed and ready.
- Combined implementation is selected over split, formal-first, vector-first,
  and ambiguous triage options.
- Future implementation can be test-only with no qsc runtime/source mutation.
- qsc/refimpl mapping remains supporting-only and does not need its own
  immediate successor.
- Formal mapping, vector, and fuzz binding remain residuals after NA-0476.
- Exactly one READY successor remains mandatory after optional closeout.

## Future scope bundle

Selected NA-0476:

`NA-0476 -- QSL qsc KEM / Signature / Transcript Binding Negative Test Implementation Harness`

Candidate future allowed paths:

- `qsl/qsl-client/qsc/tests/kem_signature_transcript_binding_negative.rs`
- `docs/governance/evidence/NA-0476_qsl_qsc_kem_signature_transcript_binding_negative_test_implementation_harness.md`
- `tests/NA-0476_qsl_qsc_kem_signature_transcript_binding_negative_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Candidate future forbidden paths unless a later exact directive stops and
authorizes otherwise:

- `qsl/qsl-client/qsc/src/**`
- `tools/refimpl/**`
- Cargo manifests and lockfiles
- `.github/workflows/**`
- qsc fuzz targets
- vectors
- formal models
- qsl-server
- qsl-attachments
- qshield runtime
- qshield-cli runtime
- website, public docs, README, and START_HERE
- qwork, qstart, qresume, qshell, qsl-backup, backup status, backup plan,
  rollback subtree, and backup tree paths

Future NA-0476 exact test expectations:

- use qsc CLI/test harness only;
- capture and mutate A1/B1/A2 bytes through the mock relay;
- use temporary qsc config roots only;
- use test-local helper code for frame parsing, public-record snapshots,
  vault/session snapshots, and no-mutation assertions;
- do not add runtime seams or source hooks;
- do not change cryptographic algorithms, provider APIs, dependencies,
  workflows, vectors, fuzz targets, formal models, or refimpl behavior.

## Future validation / marker plan

Common future NA-0476 markers:

- `NA0476_BINDING_NEGATIVE_SCOPE_CONSUMED_OK`
- `NA0476_NO_DEPENDENCY_CHANGE_OK`
- `NA0476_NO_WORKFLOW_CHANGE_OK`
- `NA0476_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0476_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0476_NO_KEM_COMPLETE_CLAIM_OK`
- `NA0476_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0476_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0476_NO_TRANSCRIPT_COMPLETE_CLAIM_OK`
- `NA0476_NO_DOWNGRADE_PROOF_CLAIM_OK`
- `NA0476_NO_REPLAY_PROOF_CLAIM_OK`
- `NA0476_ONE_READY_INVARIANT_OK`

Combined implementation markers:

- `NA0476_KEM_WRONG_PUBLIC_KEY_REJECT_OK`
- `NA0476_KEM_STALE_PUBLIC_RECORD_REJECT_OK`
- `NA0476_KEM_WRONG_CIPHERTEXT_REJECT_OK`
- `NA0476_SIGNATURE_WRONG_IDENTITY_REJECT_OK`
- `NA0476_SIGNATURE_CROSS_MESSAGE_REPLAY_REJECT_OK`
- `NA0476_TRANSCRIPT_MUTATION_REJECT_OK`
- `NA0476_TRANSCRIPT_REPLAY_REJECT_OK`
- `NA0476_SUITE_CONFUSION_REJECT_OK`
- `NA0476_STALE_PUBLIC_RECORD_REJECT_OK`
- `NA0476_NEGATIVE_TESTS_NO_SESSION_MUTATION_OK`

Required future validation:

- `cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture`
- inherited qsc cfg/no-cfg provider RNG suites remain green;
- `cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768`
- root and nested qsc fuzz lock cargo audit remain green;
- formal checks remain green;
- public-safety green before merge and after merge;
- exactly one READY item remains.

## Public claim / external review / website boundary

NA-0475 is internal governance authorization evidence only. It does not edit
public docs, website, README, START_HERE, public paper content, qsl-server,
qsl-attachments, qshield runtime, or qshield-cli runtime.

No public-readiness claim is made. No production-readiness claim is made. No
public-internet-readiness claim is made. No external-review-complete claim is
made. No crypto-complete claim is made. No KEM-complete claim is made. No
signature-complete claim is made. No identity-complete claim is made. No
transcript-complete claim is made. No downgrade-proof claim is made. No
replay-proof claim is made. No side-channel-free claim is made. No
vulnerability-free claim is made. No bug-free claim is made. No
perfect-crypto claim is made. Cargo audit green is dependency-health evidence
only.

## Rejected alternatives

- KEM-only successor: rejected because signature, transcript, replay, suite,
  and stale-record P1 cases share setup and can be tested in the same bounded
  qsc file.
- Signature-only successor: rejected because KEM and transcript binding cases
  are equally P1 and use the same A1/B1/A2 frames.
- Transcript/replay/suite-only successor: rejected because existing suite-id
  evidence is already stronger than the broader KEM/signature/stale-record
  coverage.
- Stale public-record-only successor: rejected because stale-record tests are
  exact but do not require a separate lane.
- Formal mapping first: rejected because current qsc tests are actionable and
  formal mapping remains supporting-only.
- Vector suite first: rejected because vectors are not required before qsc
  test-only negative coverage.
- qsc/refimpl mapping successor: rejected for immediate next work; mapping
  remains supporting-only and should follow direct qsc binding tests.
- Ambiguous triage: rejected because an exact bounded successor is available.

## Backup-impact statement

Backup impact: none.

Codex did not run backup or restore. Codex did not mutate qsl-backup, backup
status files, backup plan files, rollback subtree paths, systemd, timers,
fstab, source lists, backup scripts, or backup tree paths. qsl-backup SHA
matched the expected value and the local ops source inclusion count remained
one. Same-host backup continuity remains a caveat; off-host backup, restore,
key custody, and disaster recovery remain residuals. No backup-complete claim is
made. No restore-proof claim is made. No off-host-backup-complete claim is made.
No disaster-recovery-complete claim is made.

## Next recommendation

Merge NA-0475 evidence, verify post-merge public-safety, then close out NA-0475
and restore exactly one READY successor:

`NA-0476 -- QSL qsc KEM / Signature / Transcript Binding Negative Test Implementation Harness`

NA-0476 should implement only the selected qsc test-only path unless a later
directive stops and explicitly authorizes a narrower exception.
