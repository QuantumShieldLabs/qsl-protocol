Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-14

# NA-0478 QSL qsc KEM / Signature / Transcript Binding Formal Model Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0478 consumes the NA-0477 formal mapping authorization and adds the selected
bounded qsc binding model:

`formal/model_qsc_kem_signature_transcript_binding_bounded.py`

The model is integrated into:

`formal/run_model_checks.py`

The model uses opaque tokens for KEM public keys, KEM ciphertexts, signature
identities, signature public keys, signature message contexts, transcript
tokens, confirmation tokens, suite tokens, public-record/trusted-pin tokens,
replay state, pending state, completed-session state, and output flags.

This is bounded internal assurance evidence only. It is not cryptographic
analysis, not a side-channel review, not qsc/refimpl equivalence, not vector
coverage, not fuzz coverage, and not public or release readiness evidence.

## Live NA-0478 scope

Live READY item at startup:

`NA-0478 -- QSL qsc KEM / Signature / Transcript Binding Formal Model Implementation Harness`

Allowed implementation paths:

- `formal/model_qsc_kem_signature_transcript_binding_bounded.py`
- `formal/run_model_checks.py`

Allowed governance paths:

- this evidence doc
- `tests/NA-0478_qsl_qsc_kem_signature_transcript_binding_formal_model_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden mutation scope preserved:

- qsc runtime/source files
- qsc executable tests
- crypto implementation
- dependencies, Cargo manifests, lockfiles, workflows
- fuzz targets and vectors
- refimpl
- qsl-server, qsl-attachments, qshield runtime, qshield-cli
- website, public docs, README, START_HERE
- qwork, qstart, qresume, qshell
- backup, restore, qsl-backup, backup status, backup plan, rollback, backup
  tree, systemd, timers, and fstab
- public technical paper content
- durable Director State Index output

Acceptance criteria:

- the selected model runs standalone;
- the formal runner runs existing models plus the selected model;
- all required NA0478 markers are emitted;
- the model is explicitly bounded and opaque-token based;
- no public overclaim is introduced;
- exactly one READY item remains before optional closeout.

## qwork proof-file verification

Codex read the qwork proof files:

- `/srv/qbuild/work/NA-0478/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0478/.qwork/startup.qsl-protocol.json`

Verified required startup facts:

- `startup_result=OK`
- lane `NA-0478`
- repo `qsl-protocol`
- path `/srv/qbuild/work/NA-0478/qsl-protocol`
- clean worktree, index, and untracked state
- `READY_COUNT 1`
- sole READY item: NA-0478
- requested lane status: READY
- proof HEAD and proof `origin/main`: `bd00c82be608`

Freshness proof:

- proof HEAD matched live HEAD before fetch;
- proof `origin_main` matched live `origin/main` before fetch;
- fetch did not advance `origin/main`;
- PR #1225 was verified MERGED at `bd00c82be608`;
- current main public-safety completed success.

Codex did not run qwork, qstart, or qresume.

## NA-0477 inheritance

NA-0477 selected `FORMAL_MAPPING_QSC_BINDING_MODEL_IMPLEMENTATION_READY` and
authorized a new bounded qsc binding model at:

`formal/model_qsc_kem_signature_transcript_binding_bounded.py`

Inherited required model scope:

- valid A1/B1/A2 baseline trace;
- wrong KEM public-key rejection;
- stale KEM/public-record rejection;
- wrong KEM ciphertext rejection;
- wrong signature identity and wrong message-context rejection;
- cross-message signature replay rejection;
- transcript mutation rejection;
- replay rejection;
- suite confusion rejection;
- stale public-record rejection;
- no completed-session mutation on selected rejected traces;
- no success output on rejected traces.

NA-0477 did not authorize qsc runtime/source mutation, qsc executable-test
mutation, crypto/dependency/Cargo/lockfile/workflow mutation, vector/fuzz
mutation, refimpl mutation, service/public-doc mutation, backup/restore
mutation, or any public/release/crypto-completion claim.

## Formal model implementation summary

The model defines:

- `Role` tokens for initiator and responder;
- `MessageType` tokens for A1, B1, and A2;
- abstract public-record and trusted-pin records for Alice and Bob;
- `Message` records carrying suite, session, KEM, signature, transcript,
  confirm, and public-record tokens;
- `Pending` state for in-progress initiator/responder handshake state;
- `CompletedSession` state for accepted initiator/responder sessions;
- `OutputFlag` state for emitted A1/B1/A2 and success-output evidence;
- deterministic reject reasons for each modeled invalid trace.

The model explores an explicit bounded scenario table. It exits nonzero on any
invariant failure and prints deterministic NA0478 markers on success.

## Opaque-token abstraction

All cryptographic inputs and outputs are modeled as opaque strings. Equality
and mismatch are the only operations over these tokens.

The model does not derive, parse, encrypt, decrypt, decapsulate, sign, verify,
hash, or KDF any material. It does not model provider behavior, memory layout,
timing, power, cache, branch behavior, or secret lifetime.

## Modeled variables and transitions

Modeled variables:

- role;
- A1/B1/A2 message type;
- suite token;
- session token;
- KEM public-key token;
- KEM ciphertext token;
- signature identity token;
- signature public-key token;
- signature message-context token;
- transcript token;
- confirm token;
- public-record token;
- trusted-pin token;
- seen/replay state;
- pending state;
- completed-session state;
- output flags.

Modeled transitions:

- A1 accepted by the responder creates responder pending state and emits B1
  output only if suite, public record, KEM public key, signature identity,
  message context, transcript, and replay state match.
- B1 accepted by the initiator consumes initiator pending state, checks
  ciphertext binding, records initiator completed-session state, and emits A2
  success output.
- A2 accepted by the responder consumes responder pending state, checks confirm
  binding, and records responder completed-session state.
- Every rejected transition returns the exact pre-reject state and emits no
  output or success output.

## Valid trace proof

Standalone model output includes:

- `NA0478_BINDING_MODEL_VALID_TRACE_OK`

The valid baseline trace accepts A1, B1, and A2 and records two completed
sessions, one for the initiator role and one for the responder role.

## Wrong KEM rejection proof

Standalone model output includes:

- `NA0478_BINDING_MODEL_WRONG_KEM_REJECT_OK`

The wrong-KEM scenario mutates the A1 KEM public-key token while keeping the
trusted public record current. The responder rejects with
`REJECT_QSC_BINDING_WRONG_KEM_PUBLIC_KEY`.

## Stale KEM/public-record rejection proof

Standalone model output includes:

- `NA0478_BINDING_MODEL_STALE_KEM_PUBLIC_RECORD_REJECT_OK`

The stale-KEM/public-record scenario supplies stale public-record and trusted-pin
tokens plus a stale KEM public-key token. The responder rejects with
`REJECT_QSC_BINDING_STALE_KEM_PUBLIC_RECORD`.

## Wrong ciphertext rejection proof

Standalone model output includes:

- `NA0478_BINDING_MODEL_WRONG_CIPHERTEXT_REJECT_OK`

The wrong-ciphertext scenario accepts the A1 setup, then mutates the B1 KEM
ciphertext token. The initiator rejects with
`REJECT_QSC_BINDING_WRONG_CIPHERTEXT`.

## Wrong signature rejection proof

Standalone model output includes:

- `NA0478_BINDING_MODEL_WRONG_SIGNATURE_REJECT_OK`

The wrong-signature scenario accepts the A1 setup, then mutates B1 signature
identity and signature public-key tokens. The initiator rejects with
`REJECT_QSC_BINDING_WRONG_SIGNATURE`.

## Cross-message signature replay rejection proof

Standalone model output includes:

- `NA0478_BINDING_MODEL_CROSS_MESSAGE_SIGNATURE_REPLAY_REJECT_OK`

The cross-message replay scenario accepts the A1 setup, then uses a B1 signature
message-context field copied from another message type. The initiator rejects
with `REJECT_QSC_BINDING_CROSS_MESSAGE_SIGNATURE_REPLAY`.

## Transcript mutation rejection proof

Standalone model output includes:

- `NA0478_BINDING_MODEL_TRANSCRIPT_MUTATION_REJECT_OK`

The transcript mutation scenario accepts the A1 setup, then mutates the B1
transcript token. The initiator rejects with
`REJECT_QSC_BINDING_TRANSCRIPT_MUTATION`.

## Replay rejection proof

Standalone model output includes:

- `NA0478_BINDING_MODEL_REPLAY_REJECT_OK`

The replay scenario delivers the same A1 transcript token twice for the same
session. The first A1 creates pending state; the replayed A1 rejects with
`REJECT_QSC_BINDING_REPLAY` and leaves the pre-reject state unchanged.

## Suite confusion rejection proof

Standalone model output includes:

- `NA0478_BINDING_MODEL_SUITE_CONFUSION_REJECT_OK`

The suite-confusion scenario changes the A1 suite token away from Suite-2. The
responder rejects with `REJECT_QSC_BINDING_SUITE_CONFUSION`.

## Stale public-record rejection proof

Standalone model output includes:

- `NA0478_BINDING_MODEL_STALE_PUBLIC_RECORD_REJECT_OK`

The stale-public-record scenario accepts the A1 setup, then supplies stale B1
public-record and trusted-pin tokens. The initiator rejects with
`REJECT_QSC_BINDING_STALE_PUBLIC_RECORD`.

## No session mutation proof

Standalone model output includes:

- `NA0478_BINDING_MODEL_NO_SESSION_MUTATION_OK`

Each rejected scenario asserts the rejected transition returns the exact
pre-reject completed-session set. The model also checks a selected rejected
trace after a completed baseline and verifies the completed-session set is
unchanged.

## No success output on reject proof

Standalone model output includes:

- `NA0478_BINDING_MODEL_NO_SUCCESS_OUTPUT_ON_REJECT_OK`

Each rejected transition emits no output and no success output flag.

## Runner integration proof

`formal/run_model_checks.py` imports and runs:

`emit_qsc_kem_signature_transcript_binding_model_report`

Local runner output includes all existing model reports plus:

- `NA-0478 qsc KEM/signature/transcript binding bounded model checks`
- `OK: qsc KEM/signature/transcript binding formal model checks passed`

## Bounded evidence / no completion claim proof

The model emits no completion/public claim. It also emits explicit boundary
markers:

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
- `NA0478_NO_FORMAL_PROOF_COMPLETE_CLAIM_OK`
- `NA0478_ONE_READY_INVARIANT_OK`

## qsc/refimpl mapping residual

qsc/refimpl provider and semantic mapping remain residual. The recommended
successor is:

`NA-0479 -- QSL qsc/refimpl KEM / Signature Binding Mapping Authorization Plan`

## Negative vectors residual

Negative vectors remain residual. This model does not add or mutate vector
files.

## Fuzz binding residual

Fuzz binding remains residual. This model does not add or mutate fuzz targets,
fuzz corpora, or fuzz lockfiles.

## Side-channel / secret-material caveat

Side-channel and secret-material lifetime analysis remain residual. This model
does not claim timing, cache, branch, power, allocator, memory overwrite,
provider-oracle, or all-secret-material coverage.

## External-review readiness residual

External-review readiness remains incremental and incomplete. This model is a
bounded internal assurance artifact only.

## Release-claim boundary

No public readiness claim is made.
No production readiness claim is made.
No public-internet readiness claim is made.
No external-review completion claim is made.
No crypto completion claim is made.
No KEM completion claim is made.
No signature completion claim is made.
No identity completion claim is made.
No transcript completion claim is made.
No downgrade proof claim is made.
No replay proof claim is made.
No formal-proof completion claim is made.
No side-channel freedom claim is made.
No vulnerability freedom claim is made.
No bug freedom claim is made.
No perfect-crypto claim is made.

Cargo audit green remains dependency-health evidence only.

## Level-1 stewardship and D328 assurance review

Best-Known-Method Review: a new bounded opaque-token model is the best-known
method for this exact scope because NA-0477 selected a model path that existing
formal models do not cover.

Hostile Cryptographer Review: the model treats cryptographic material as opaque
tokens and checks only explicit binding/reject/no-mutation invariants. It does
not infer cryptographic security from token equality.

Red-Team Review: modeled adversary actions include wrong KEM public key, stale
KEM/public-record material, wrong ciphertext, wrong signature identity,
cross-message signature replay, transcript mutation, transcript replay, suite
confusion, and stale public-record use.

Production SRE Review: pending state, completed-session state, replay state,
and output flags are explicit so rejected traces can be checked for no completed
session mutation and no success output.

Side-Channel Caveat: side-channel analysis is out of scope and remains
residual.

Formal-Model Mapping Residual: the NA-0477 selected formal model is now
implemented, but qsc/refimpl mapping, vectors, fuzz, side-channel, and external
review residuals remain.

External-Review Readiness: this model improves internal review material but is
not external-review completion.

Release-Claim Boundary: this model is not release readiness and does not change
public claim posture.

Assurance Gap Review Trigger: the next strongest residual is qsc/refimpl KEM and
signature mapping unless post-merge evidence proves a higher-priority residual.

Lead Director authority remains final. Stewardship remains advisory only.
Exactly one READY item remains mandatory.

## Validation

Startup validation:

- qwork proof files parsed and matched live HEAD/origin before fetch.
- Fetch did not advance origin/main.
- PR #1225 verified MERGED at `bd00c82be608`.
- READY_COUNT 1 and READY NA-0478 verified.
- D-0942 and D-0943 existed once; D-0944 absent at startup.
- Current main public-safety completed success.
- Root cargo audit passed.
- Nested qsc fuzz lock audit passed.
- Inherited qsc tests passed.
- Existing formal checks passed.
- qsl-backup SHA matched the expected value; current Codex ops source-list
  inclusion count was exactly one.

Implementation validation:

```bash
python3 formal/model_qsc_kem_signature_transcript_binding_bounded.py
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

All three commands passed locally after implementation.

## Scope guard

Implementation changed only:

- `formal/model_qsc_kem_signature_transcript_binding_bounded.py`
- `formal/run_model_checks.py`
- this evidence doc
- `tests/NA-0478_qsl_qsc_kem_signature_transcript_binding_formal_model_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No qsc runtime/source, qsc executable-test, crypto, dependency, Cargo,
lockfile, workflow, fuzz, vector, refimpl, qsl-server, qsl-attachments,
qshield, qshield-cli, website, public-doc, README, START_HERE, qwork/qstart/
qresume/qshell, backup status, backup plan, rollback, or backup tree mutation
is introduced.

## Backup-impact statement

No backup or restore was run. qsl-backup was read and hashed only. Backup
status and backup plan files were read only and not mutated. This lane has no
backup-plan impact beyond normal repository governance and formal-file changes.

## Successor selection

Default successor selected:

`NA-0479 -- QSL qsc/refimpl KEM / Signature Binding Mapping Authorization Plan`

Rationale: NA-0478 implements the selected qsc binding formal model, and the
next strongest residual is mapping qsc assumptions to refimpl/provider KEM and
signature semantics.

## Next recommendation

If the implementation PR merges and post-merge public-safety remains green,
close NA-0478 and restore the default NA-0479 qsc/refimpl KEM/signature mapping
authorization plan. Do not implement NA-0479 in the NA-0478 closeout.
