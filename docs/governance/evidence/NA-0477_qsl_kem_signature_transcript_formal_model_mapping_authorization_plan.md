Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-14

# NA-0477 QSL KEM / Signature / Transcript Formal Model Mapping Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0477 consumes the bounded qsc negative-test evidence from NA-0476 and
authorizes the next formal-model implementation lane.

Selected classification:

`FORMAL_MAPPING_QSC_BINDING_MODEL_IMPLEMENTATION_READY`

Selected successor:

`NA-0478 -- QSL qsc KEM / Signature / Transcript Binding Formal Model Implementation Harness`

The selected future model should be a new bounded Python model:

`formal/model_qsc_kem_signature_transcript_binding_bounded.py`

The model should use opaque tokens for KEM keys, KEM ciphertext, signature keys,
signature domains, transcript contexts, suite contexts, public records, replay
state, pending state, and completed session state. It must not model
cryptographic internals, timing, side channels, provider RNG failure, TUI
bootstrap transactionality, refimpl behavior, vectors, fuzz targets, or runtime
implementation changes.

This NA-0477 directive is authorization only. It does not mutate formal model
files, runtime code, crypto code, dependencies, Cargo manifests, lockfiles,
workflows, executable tests, fuzz targets, vectors, refimpl, services, qshield,
qshield-cli, website, public docs, README, START_HERE, qwork/qstart/qresume/
qshell, qsl-backup, backup status, backup plan, rollback paths, or backup tree
paths.

## Live NA-0477 scope

Live READY item at startup:

`NA-0477 -- QSL KEM / Signature / Transcript Formal Model Mapping Authorization Plan`

Allowed mutation paths for this evidence PR:

- this evidence doc
- `tests/NA-0477_qsl_kem_signature_transcript_formal_model_mapping_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Read-only inspection covered qwork proof files, queue/governance files,
NA-0476 evidence and testplan files, qsc handshake and identity source, qsc
tests, qsc fuzz metadata, refimpl, formal models, inputs, Cargo files, scripts,
workflows, and local backup evidence paths authorized by the directive.

Forbidden mutation scope preserved:

- formal model files
- runtime or crypto implementation
- dependencies, Cargo manifests, lockfiles, workflows
- executable tests, fuzz targets, vectors
- refimpl
- qsl-server, qsl-attachments, qshield runtime, qshield-cli
- website, public docs, README, START_HERE
- qwork, qstart, qresume, qshell
- backup, restore, qsl-backup, backup status, backup plan, rollback, backup
  tree, systemd, timers, fstab
- public technical paper content
- durable Director State Index output

Acceptance criteria for NA-0477:

- existing formal models are inventoried without mutation;
- qsc implementation/test behavior from NA-0476 is mapped to candidate formal
  abstractions;
- model scope boundary is selected without overclaiming;
- one exact NA-0478 successor is selected;
- no public claim expansion occurs;
- exactly one READY item remains.

## qwork proof-file verification

Codex read the qwork proof files:

- `/srv/qbuild/work/NA-0477/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0477/.qwork/startup.qsl-protocol.json`

Verified required startup facts:

- `startup_result=OK`
- lane `NA-0477`
- repo `qsl-protocol`
- path `/srv/qbuild/work/NA-0477/qsl-protocol`
- clean worktree, index, and untracked state
- `READY_COUNT 1`
- sole READY item: NA-0477
- requested lane status: READY
- proof HEAD and proof `origin/main`: `fcf434b1746b`

Freshness proof:

- proof HEAD matched live HEAD before fetch;
- proof `origin_main` matched live `origin/main` before fetch;
- fetch did not advance `origin/main`;
- `origin/main` equals and descends from PR #1223 merge commit
  `fcf434b1746b`;
- PR #1223 was verified MERGED;
- current main public-safety completed success.

Codex did not run qwork, qstart, or qresume.

Recovered startup proof issues:

- Failing command: first queue/decision assertion wrapper.
  Classification: recoverable command-shape mistake because the compact helper
  output did not enumerate every historical NA status or D-0940 line. Corrective
  action: direct bounded checks against `NEXT_ACTIONS.md` and `DECISIONS.md`.
  Final result: READY NA-0477, NA-0476 DONE, D-0940 once, D-0941 once, D-0942
  absent, duplicate decision count zero.
- Failing command: first backup source-list count.
  Classification: recoverable check-scope mistake because a broad historical
  text scan counted old logs/status prose instead of the current manifest source
  list. Corrective action: count the latest scheduled manifest source entry.
  Final result: `/home/victor/work/qsl/codex/ops` appeared exactly once in the
  latest scheduled manifest; qsl-backup SHA matched the expected value.

## NA-0476 inheritance

NA-0476 implemented bounded internal qsc negative-test evidence in:

`qsl/qsl-client/qsc/tests/kem_signature_transcript_binding_negative.rs`

Inherited markers and behavior:

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

NA-0476 did not make completion or public-readiness claims:

- NA-0476 did not claim KEM completeness.
- NA-0476 did not claim signature completeness.
- NA-0476 did not claim identity completeness.
- NA-0476 did not claim transcript completeness.
- NA-0476 did not claim downgrade proof.
- NA-0476 did not claim replay proof.
- NA-0476 did not claim formal-proof completeness.
- NA-0476 did not claim crypto completeness.
- NA-0476 did not claim public readiness.
- NA-0476 did not claim production readiness.
- NA-0476 did not claim public-internet readiness.
- NA-0476 did not claim side-channel freedom.
- NA-0476 did not claim vulnerability freedom.
- NA-0476 did not claim bug freedom.
- NA-0476 did not claim perfect crypto.
- NA-0476 did not claim external-review completion.
- NA-0476 did not claim backup or restore completion.

## Applicable Stewardship and Assurance Review

1. Crypto / Protocol Steward: qsc binding behavior is security-sensitive but
   NA-0477 is authorization-only. The selected future model must treat crypto
   operations as opaque accept/reject tokens and must not alter protocol, wire,
   crypto, auth, state-machine, or runtime semantics.

2. CI / Dependency / Release Health Steward: root cargo audit, nested qsc fuzz
   lock audit, inherited qsc tests, refimpl pqkem768, formal checks, and
   public-safety are health evidence only. Cargo audit green is dependency-health
   evidence only.

3. Public Claims / External Review Steward: this lane is internal governance
   evidence. Claim boundaries:
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
   - No formal-proof-complete claim is made.
   - No side-channel-free claim is made.
   - No vulnerability-free claim is made.
   - No bug-free claim is made.
   - No perfect-crypto claim is made.

4. Product / Demo / Service Boundary Steward: qsl-server, qsl-attachments,
   qshield runtime, qshield-cli, website, public docs, README, and START_HERE
   remain untouched. qshield/demo evidence is not production evidence.

5. Local Ops / Backup / Restore Steward: qwork proof files were read only.
   qsl-backup was hashed read-only; latest manifest source inclusion was
   counted read-only. No backup, restore, qsl-backup mutation, status mutation,
   plan mutation, rollback mutation, or backup tree mutation occurred.

6. Best-Known-Method Review: `BEST_KNOWN_METHOD_FOR_SCOPE`. A new bounded model
   is the best-known next step because NA-0476 provides direct behavior witnesses
   and the existing qsc suite-id model is too narrow for KEM/signature/identity
   binding.

7. Hostile Cryptographer Review: select opaque tokens and explicit reject/no-
   mutation invariants. Do not model KEM decapsulation, ML-DSA verification,
   KDF outputs, or side channels as cryptographic proofs. Treat wrong-token
   acceptance, replay acceptance, or session mutation after reject as modeled
   failures.

8. Red-Team Review: include adversary-controlled message mutation, cross-message
   signature replay, stale public-record replay, suite-confusion, and replayed
   A1/A2-style contexts. Keep the model bounded and deterministic.

9. Production SRE Review: model completed-session and pending-store mutation
   explicitly. Reject paths must leave completed session state unchanged. Pending
   cleanup may be modeled only where the implementation intentionally clears
   explicit suite-bound pending state; it must not be overclaimed as broad
   rollback safety.

10. Side-Channel Caveat: side-channel analysis remains out of scope. The future
    model must not claim timing, memory-access, cache, branch, power, provider
    error-oracle, or secret-material lifetime coverage.

11. Formal-Model Mapping Residual: `FORMAL_MODEL_MAPPING_RESIDUAL_ACTIVE` until
    NA-0478 implements and validates the selected model.

12. External-Review Readiness: `EXTERNAL_REVIEW_READINESS_INCREMENTAL`. The
    mapping selection improves internal review readiness, but external-review
    completion remains blocked until implementation, vectors/fuzz/refimpl
    residuals, side-channel caveats, and public-claim boundaries are handled.

13. Release-Claim Boundary: all release/public claims remain bounded by internal
    evidence. The selected model would be one G4 gate, not release readiness by
    itself.

14. Assurance Gap Review Trigger:
    `HIGHER_PRIORITY_RESIDUAL_SUPERSEDES_ASSURANCE_REVIEW`, because an exact
    formal model implementation successor is selected. Assurance Gap Review
    remains required later unless a later directive identifies another higher-
    priority residual.

Level 1 stewardship is active. Level 2 and Level 3 remain future-gated. No
separate Directors are created. No independent READY promotion is created. No
independent merge authority is created. Lead Director final authority is
preserved.

## Existing formal model inventory

| Formal path | Purpose and state variables | Transitions and invariants | Direct binding coverage | Safe future extension |
|---|---|---|---|---|
| `formal/README.md` | Describes current formal checks as bounded Python state-space models for SCKA, Suite-2 negotiation, and future qsc suite-id semantics. | Documents G4 fail-closed model-check gate and limitations. | No direct KEM/signature/identity binding. It describes transcript/key-context binding for suite-id only. | Update only when a future model lands. |
| `formal/model_scka_bounded.py` | Models SCKA `Party` and `World` state: peer ADV ids, local keys, tombstones, network, seen messages. | `send_adv`, `send_ctxt`, and delivery transitions assert ADV monotonicity, one-time CTXT consumption, tombstone reject, reject no state change, and transactional commit. | No direct KEM binding, signature binding, transcript/KDF/confirm binding, identity/public-record binding, suite confusion, or stale public-record behavior. Replay is represented only as ADV/CTXT stale/tombstone handling. | Do not extend for qsc handshake binding; it is a different protocol slice. |
| `formal/model_suite2_negotiation_bounded.py` | Models `NegotiationState` and `NegotiationAttempt`: support booleans, committed capabilities, negotiated suite, transcript suite views. | Applies negotiation attempts and asserts Suite-2 accept only when commitments and transcript views match; rejects mutate no accepted/durable state. | Direct for Suite-2 downgrade/capability commitment only. No KEM, signature, identity, stale public-record, qsc pending/session, or concrete transcript/KDF/confirm binding. | Keep as negotiation slice. Do not overload with qsc handshake tokens. |
| `formal/model_qsc_handshake_suite_id_bounded.py` | Models qsc suite-id semantics with `ModelState`, `Scenario`, `ParamBlock`, transcript context, key context, accept/reject outcomes. | Valid/legacy/downgrade/unsupported/stripped/mismatch/duplicate/unknown/noncanonical/malformed/transcript/key-context scenarios assert deterministic reject, no accepted-state mutation, no output, no recv_commit, no secret leak, and no downgrade path. | Direct for suite-id, downgrade, suite-confusion, and abstract transcript/key-context coverage. Not direct for KEM public-key binding, KEM ciphertext binding, signature public-record binding, cross-message signature replay, identity/public-record stale records, or qsc completed-session mutation. | A small extension could add transcript/replay/suite-only cases, but KEM/signature/identity binding is broad enough to justify a new model. |
| `formal/run_model_checks.py` | CI entry point imports the three current models and emits stable pass/fail output. | Runs SCKA, Suite-2 negotiation, and qsc suite-id model checks fail-closed. | No direct binding model is registered today. | Future NA-0478 should add the new model to this runner after its standalone checks pass. |

## qsc implementation / test behavior to map

| Property | Direct qsc evidence | Formal abstraction candidate | State variables / transitions | Model complexity | Overclaim risk | Priority |
|---|---|---|---|---|---|---|
| Wrong KEM public key reject | NA-0476 wrong Alice identity emits `peer_mismatch`/`identity_mismatch`, no B1, no session. | `kem_pk_token != pinned_peer_kem_token` rejects before responder session or B1 output. | peer record token, A1 KEM token, responder pending/session state. | Low | Low if tokenized. | P1 |
| Stale KEM public record reject | NA-0476 rotates Alice identity while Bob retains stale record; Bob rejects and existing session bytes are unchanged. | stale public-record token rejects new A1 and leaves completed session unchanged. | current record token, stale trusted token, completed session snapshot. | Medium | Medium; avoid claiming rollback completeness. | P1 |
| Wrong KEM ciphertext reject | NA-0476 mutates B1 KEM ciphertext; Alice rejects with decap or transcript-context reason, no A2/session. | ciphertext token not bound to pending KEM secret rejects before A2/session. | pending KEM secret token, ciphertext token, transcript token, session state. | Medium | Medium; no decapsulation proof. | P1 |
| Wrong signature identity reject | NA-0476 wrong signature pin rejects after B1 verification, no A2/session. | signature public-record token mismatch rejects after signature-valid abstraction. | signature key token, signature pin token, transcript token. | Medium | Low if public-record token is explicit. | P1 |
| Cross-message signature replay reject | NA-0476 replays an A2 signature into B1; B1 verification rejects with `sig_invalid`. | signature domain token mismatch (`B1` vs `A2`) rejects. | signature domain, message role, transcript token, session id token. | Low | Low if no cryptographic claim. | P1 |
| Transcript mutation reject | NA-0476 mutates B1 transcript-bound field; Alice rejects with transcript-context reason. | transcript token mismatch rejects before A2/session. | A1/B1/A2 transcript token, KDF/confirm token. | Medium | Medium; no KDF security proof. | P1 |
| Transcript replay reject | NA-0476 replays A1 while responder pending state exists; reject, no second B1/session. | replayed message context rejects when pending key/session token already exists. | seen/pending state, message id token, output flag. | Medium | Low if bounded. | P1 |
| Suite confusion reject | NA-0476 downgrades suite block; suite-required Bob rejects, no B1/session. | suite token mismatch/downgrade rejects. | suite context token, mode, output flag. | Low | Low; existing suite-id model already covers related slice. | P1 |
| Stale public-record reject | Same as stale KEM public record with identity rotation and existing session stability. | current vs stale record token rejects and preserves completed session. | trusted record token, active identity token, session snapshot. | Medium | Medium; do not claim identity completeness. | P1 |
| No session mutation on selected rejects | NA-0476 asserts no qsp session for pre-session rejects and unchanged existing Bob session for stale-record reject. | reject outcome leaves `completed_session` unchanged; selected outputs false. | completed session, pending store, B1/A2 output flags. | Medium | Low if scoped to selected rejects. | P1 |
| Provider RNG failure no partial state | NA-0458/0461/0463/0465/0467/0469/0472 tests cover KEM, B1, A2, lazy identity, legacy/public-record, CLI rotation, and TUI bootstrap forced RNG failures. | Keep out of first binding model; already executable test evidence. | Would require provider failure events and durable pre-generation/write ordering. | High | High; could dilute binding model. | P2 later |
| Suite-id fail-closed/no-mutation | NA-0309/0313-era suite-id model/tests and current formal suite-id model. | Reference existing model or integrate suite token only in new binding model. | suite token, mode, output flag. | Low | Low if not overclaimed. | P1 |

## Formal mapping option review

| Option | Select / reject | Evidence and rationale | Future path | Future command | Invariant and caveat |
|---|---|---|---|---|---|
| Option 1 - Extend existing qsc suite-id bounded model | Reject as primary | It already models suite-id, downgrade, transcript/key-context, and no-mutation. Extending it with KEM/signature/identity tokens would overload a previously narrow model. | Could remain a reference dependency only. | `python3 formal/model_qsc_handshake_suite_id_bounded.py` | Good for suite-id only; no KEM/signature completeness claim. |
| Option 2 - Add new qsc binding negative model | Select | NA-0476 provides direct qsc behavior witnesses for KEM, signature, transcript, replay, suite, stale-record, and no-session mutation. Opaque tokens can model them without crypto internals. | `formal/model_qsc_kem_signature_transcript_binding_bounded.py`; update `formal/run_model_checks.py`. | `python3 formal/model_qsc_kem_signature_transcript_binding_bounded.py`; `python3 formal/run_model_checks.py` | Reject wrong/mismatched tokens and preserve completed session state; no crypto-proof claim. |
| Option 3 - Add identity/public-record stale-record model | Reject as primary | Important, but only one part of NA-0476. Splitting identity first would delay direct KEM/signature/transcript mapping. | Possible later if stale-record state grows beyond the combined binding model. | Future standalone command if authorized. | Avoid identity completeness claim. |
| Option 4 - Add combined qsc binding model | Select only as bounded combined token model, not broad all-assurance model | Combined KEM/signature/transcript/identity/replay/suite is appropriate if bounded to NA-0476 tokens and selected no-session invariants. Provider RNG, TUI, side-channel, vectors, fuzz, and refimpl stay out. | Same as Option 2. | Same as Option 2. | Combined does not mean formal-proof complete. |
| Option 5 - Formal mapping documentation-only | Reject | Enough concrete behavior exists from NA-0476 to implement an exact bounded model next. | n/a | n/a | Documentation-only would leave G4 residual unconsumed. |
| Option 6 - Negative vector suite before formal model | Reject for current successor | Vectors remain useful but are not required before an opaque-token formal model. | Future vector lane remains residual. | n/a | No vector completeness claim. |
| Option 7 - qsc/refimpl mapping before formal model | Reject for current successor | qsc/refimpl mapping remains useful, but current formal abstraction does not depend on refimpl internals. | Future qsc/refimpl lane remains residual. | n/a | No qsc/refimpl equivalence claim. |

## Model scope boundary review

Future NA-0478 model should include:

- roles: initiator and responder;
- messages: abstract A1, B1, A2;
- mode and suite context tokens;
- session id token;
- KEM public key token and KEM ciphertext token;
- signature public key token and signature domain token;
- public-record token, trusted-pin token, and stale/current record token;
- transcript token and confirm/KDF context token;
- pending store state;
- completed session state;
- output flags for B1, A2, recv_commit, and handshake_complete;
- replay/seen-message state for selected bounded cases;
- deterministic reason labels for selected rejects.

Future transitions should include:

- start initiator pending and emit A1;
- responder consumes A1 and either emits B1 with responder pending/session
  snapshot or rejects;
- initiator consumes B1 and either stores completed session/emits A2 or rejects;
- responder consumes A2 and either stores completed session or rejects;
- mutate KEM public key token, KEM ciphertext token, signature token, transcript
  token, suite token, public-record token, or replay state;
- compare pre/post completed-session snapshots for reject cases.

Future invariants should include:

- wrong KEM public key rejects with no B1 and no completed session;
- stale KEM/public-record token rejects and preserves existing completed session;
- wrong KEM ciphertext rejects before A2/completed session;
- wrong signature public-record token rejects before A2/completed session;
- cross-message signature-domain replay rejects;
- transcript mutation rejects before A2/completed session;
- replay rejects without duplicate output/session mutation;
- suite confusion rejects without output/session mutation;
- all selected reject outcomes leave completed session state unchanged;
- accepted baseline path exists so the model checks both accept and reject paths.

Explicit non-goals:

- cryptographic security of KEM, ML-DSA, KDF, AEAD, hash, or transcript MAC;
- byte parser memory safety;
- side-channel analysis;
- provider RNG failure and no-partial-state transactionality;
- TUI account bootstrap pre-generation;
- qsc/refimpl equivalence;
- negative vector generation;
- fuzz coverage;
- no public-readiness claim and no external-review completion claim.

## Formal successor readiness review

Selected classification:

`FORMAL_MAPPING_QSC_BINDING_MODEL_IMPLEMENTATION_READY`

Reasons:

- NA-0476 provides enough concrete behavior for abstraction.
- The future model can represent cryptography as opaque token equality,
  freshness, domain, and context checks.
- Existing formal files do not directly represent KEM, signature, identity, or
  stale public-record binding.
- The existing suite-id model should not be overloaded.
- A new model file can keep scope auditable and CI integration minimal.

Rejected classifications:

- `FORMAL_MAPPING_SUITE_ID_MODEL_EXTENSION_READY`: rejected because suite-id is
  only one part of the binding evidence.
- `FORMAL_MAPPING_IDENTITY_STALE_RECORD_MODEL_READY`: rejected because stale
  record is important but not dominant.
- `FORMAL_MAPPING_COMBINED_MODEL_TOO_BROAD_SPLIT_NEEDED`: rejected because a
  combined opaque-token model is bounded enough if provider RNG, TUI, refimpl,
  vectors, fuzz, and side channels stay out.
- `FORMAL_MAPPING_DOCUMENTATION_ONLY`: rejected because implementation is ready.
- `NEGATIVE_VECTOR_SUITE_FIRST`: rejected because vectors are residual, not a
  prerequisite for this model.
- `QSC_REFIMPL_MAPPING_FIRST`: rejected because refimpl mapping is residual, not
  a prerequisite for this model.
- `FORMAL_MAPPING_AMBIGUOUS`: rejected because the model boundary is exact.

## Future scope bundle

Future NA-0478 title:

`QSL qsc KEM / Signature / Transcript Binding Formal Model Implementation Harness`

Allowed future paths:

- `formal/model_qsc_kem_signature_transcript_binding_bounded.py`
- `formal/run_model_checks.py`
- `docs/governance/evidence/NA-0478_qsl_qsc_kem_signature_transcript_binding_formal_model_implementation_harness.md`
- `tests/NA-0478_qsl_qsc_kem_signature_transcript_binding_formal_model_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future forbidden unless exact later directive authorizes:

- qsc runtime/source mutation;
- qsc executable test mutation;
- dependency, Cargo, lockfile, or workflow mutation;
- fuzz target or vector mutation;
- refimpl mutation;
- qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public
  docs, README, START_HERE mutation;
- backup, restore, qsl-backup, backup status, backup plan, rollback, backup tree
  mutation;
- no public-readiness claim and no crypto-complete claim.

## Future validation / marker plan

Common NA-0478 markers:

- `NA0478_FORMAL_MAPPING_SCOPE_CONSUMED_OK`
- `NA0478_NO_RUNTIME_CHANGE_OK`
- `NA0478_NO_DEPENDENCY_CHANGE_OK`
- `NA0478_NO_WORKFLOW_CHANGE_OK`
- `NA0478_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0478_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0478_NO_KEM_COMPLETE_CLAIM_OK`
- `NA0478_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0478_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0478_NO_TRANSCRIPT_COMPLETE_CLAIM_OK`
- `NA0478_NO_DOWNGRADE_PROOF_CLAIM_OK`
- `NA0478_NO_REPLAY_PROOF_CLAIM_OK`
- `NA0478_ONE_READY_INVARIANT_OK`

Binding model markers:

- `NA0478_BINDING_MODEL_WRONG_KEM_REJECT_OK`
- `NA0478_BINDING_MODEL_WRONG_SIGNATURE_REJECT_OK`
- `NA0478_BINDING_MODEL_TRANSCRIPT_MUTATION_REJECT_OK`
- `NA0478_BINDING_MODEL_REPLAY_REJECT_OK`
- `NA0478_BINDING_MODEL_SUITE_CONFUSION_REJECT_OK`
- `NA0478_BINDING_MODEL_STALE_PUBLIC_RECORD_REJECT_OK`
- `NA0478_BINDING_MODEL_NO_SESSION_MUTATION_OK`

Future validation commands:

```bash
python3 formal/model_qsc_kem_signature_transcript_binding_bounded.py
python3 formal/run_model_checks.py
cargo fmt --check
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
```

## Public claim / external review / website boundary

This evidence is internal governance evidence only.

No website, public docs, README, START_HERE, public technical paper, external
review package, public claim, qsl-server, qsl-attachments, qshield runtime, or
qshield-cli mutation is authorized or performed.

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
- No formal-proof-complete claim is made.
- No side-channel-free claim is made.
- No vulnerability-free claim is made.
- No bug-free claim is made.
- No perfect-crypto claim is made.
- No backup-complete claim is made.
- No restore-proof claim is made.

## Rejected alternatives

- Suite-id-only successor: rejected because KEM/signature/identity binding would
  remain unmapped.
- Identity stale-record-only successor: rejected because transcript and
  signature binding would remain unmapped.
- Documentation-only successor: rejected because implementation is ready with
  opaque tokens.
- Negative vectors first: rejected because the formal model can proceed without
  vectors while keeping vectors as residual.
- qsc/refimpl mapping first: rejected because qsc/refimpl equivalence is not
  required for the selected abstract model.
- Broad all-assurance model: rejected because it would overclaim provider RNG,
  side-channel, vectors, fuzz, refimpl, and TUI transactionality.

## Backup-impact statement

Backup impact: none.

Codex did not run backup or restore. Codex did not mutate qsl-backup, backup
status files, backup plan files, rollback paths, or backup tree paths.

Read-only local backup evidence:

- `/usr/local/sbin/qsl-backup` SHA matched the expected hash;
- the latest scheduled manifest included `/home/victor/work/qsl/codex/ops`
  exactly once;
- this remains same-host continuity evidence only;
- this is not off-host backup proof;
- this is not disaster recovery proof;
- this is not restore proof;
- this is not key custody proof;
- this is not backup-complete proof.

## Next recommendation

After the NA-0477 evidence PR merges and public-safety is green, close NA-0477
and restore:

`NA-0478 -- QSL qsc KEM / Signature / Transcript Binding Formal Model Implementation Harness`

NA-0478 should implement the new bounded opaque-token qsc binding model, add it
to `formal/run_model_checks.py`, and keep all runtime/crypto/dependency/
workflow/test/fuzz/vector/refimpl/service/public/backup boundaries intact.
