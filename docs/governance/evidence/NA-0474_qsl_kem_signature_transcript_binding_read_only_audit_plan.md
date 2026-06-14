Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-13

# NA-0474 QSL KEM / Signature / Transcript Binding Read-Only Audit Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0474 consumes NA-0473 and performs a governance-only, read-only audit of qsc
and refimpl KEM, signature, transcript, identity, suite, public-record, replay,
downgrade, and state-transition binding evidence.

Primary classification:

`BINDING_NEGATIVE_TEST_SCOPE_NEXT`

Selected successor:

`NA-0475 -- QSL qsc KEM / Signature / Transcript Binding Negative Test Scope Authorization Plan`

The audit found meaningful supporting and direct evidence for selected qsc
binding behavior: A1 includes suite context, session ID, KEM public key,
signature public key, and DH public key; B1 includes suite context, session ID,
KEM ciphertext, transcript MAC, signature public key, signature, and DH public
key; A2 includes suite context, session ID, confirmation MAC, and signature.
KDF, transcript MAC/hash, confirmation key, and confirmation MAC inputs include
session ID and suite context where the qsc suite-context path is active. B1 and
A2 signatures use explicit label-prefixed messages and cover transcript hash,
with A2 also covering the confirmation MAC. Existing tests cover happy-path
handshake, B1/A2 signature tamper, generic tamper/out-of-order cases,
suite-context downgrade/mismatch/replay cases, primary identity pin mismatch,
provider-error no-mutation, and bounded provider RNG failure behavior.

The evidence is not complete enough to claim KEM, signature, identity, or
transcript completion. It is not public-readiness evidence, not
external-review-complete evidence, not vulnerability-free evidence, not
bug-free evidence, and not perfect-crypto completion evidence. It is not
downgrade-proof, not replay-proof, and not side-channel-free evidence. The
highest-value next lane is an authorization plan for exact qsc negative-test
scope across KEM public key/ciphertext, signature replay/domain separation,
transcript mutation/replay, stale identity/public-record rollback, downgrade,
suite confusion, and cross-role/cross-message cases.

NA-0474 mutates governance evidence only. It does not mutate runtime code,
crypto code, dependencies, Cargo manifests, lockfiles, workflows, executable
tests, fuzz targets, vectors, formal models, refimpl, qsl-server,
qsl-attachments, qshield runtime, qshield-cli, website, public docs, README,
START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status files,
backup plan files, rollback subtree paths, `/backup/qsl`, durable Director
State Index output, or public technical paper content.

## Live NA-0474 scope

Allowed NA-0474 mutation paths:

- `docs/governance/evidence/NA-0474_qsl_kem_signature_transcript_binding_read_only_audit_plan.md`
- `tests/NA-0474_qsl_kem_signature_transcript_binding_read_only_audit_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Read-only inspection included qwork proof files, governance evidence and
testplans, `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`, rolling
journal, qsc handshake/identity/source/test/fuzz surfaces, refimpl crypto and
Suite-2 source/tests, formal models, vector inputs, Cargo manifests and
lockfiles, scripts, workflows, qsl-backup hash evidence, backup status/plan
files, and prior response files.

Forbidden mutation scope was preserved for implementation, runtime, crypto,
dependency, Cargo, lockfile, workflow, executable test, fuzz target, vector,
formal model, refimpl, qsl-server, qsl-attachments, qshield runtime,
qshield-cli, website, public docs, README, START_HERE, qwork, qstart, qresume,
qshell, qsl-backup, backup status, backup plan, rollback subtree, `/backup/qsl`,
public technical paper, and durable Director State Index output paths.

Acceptance criteria:

- qwork proof files verified without rerunning qwork;
- NA-0473 consumed;
- KEM, signature, transcript/KDF/confirmation, identity/public-record,
  downgrade/replay/suite-confusion, qsc/refimpl mapping, formal/vector/fuzz,
  and threat scenario reviews completed;
- Level-1 stewardship and D328 assurance sections completed;
- findings matrix ranked;
- exactly one NA-0475 successor selected;
- no implementation mutation;
- no public overclaim;
- exactly one READY item remains mandatory.

Stop conditions preserved: stale qwork proof, PR #1217 not merged, unexpected
queue/decision state, omitted KEM/signature/transcript/stale-record/replay/
downgrade review, unsafe successor selection, root or nested audit failure,
qsl-backup source-list regression, public-safety red or missing, more than one
READY item, any forbidden mutation, or any prohibited public/readiness/complete
security claim.

## qwork proof-file verification

Codex read the qwork proof files:

- `/srv/qbuild/work/NA-0474/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0474/.qwork/startup.qsl-protocol.json`

Verified required startup facts:

- `startup_result=OK`
- lane `NA-0474`
- repo `qsl-protocol`
- path `/srv/qbuild/work/NA-0474/qsl-protocol`
- clean worktree, index, and untracked state
- READY_COUNT 1
- sole READY item: NA-0474
- requested lane status: READY
- proof HEAD and proof `origin/main`: `e5c1cc2ec455`

Freshness proof:

- proof HEAD matched live HEAD before fetch;
- proof `origin_main` matched live `origin/main` before fetch;
- fetch did not advance `origin/main`;
- `origin/main` equals and descends from PR #1217 merge commit
  `e5c1cc2ec455`;
- PR #1217 was verified MERGED;
- current main public-safety completed success.

Codex did not run `qwork`, `qstart`, or `qresume`.

## NA-0473 inheritance

NA-0473 is DONE and D-0934/D-0935 are accepted.

Inherited assurance:

- NA-0473 classified the next assurance gap as
  `CRYPTOGRAPHER_REVIEW_TRANSCRIPT_BINDING_NEXT`.
- The identity/provider RNG chain is bounded internal evidence for selected
  qsc KEM, B1, A2, lazy identity, legacy/public-record, CLI rotation, TUI
  bootstrap, key lifecycle, provider-error, qsc adversarial, formal, refimpl,
  root dependency, nested fuzz lock, public-safety, and backup/log-code
  surfaces.
- The chain is not identity-complete, not provider-RNG-complete, not
  signature-complete, not KEM-complete, and not transcript-complete evidence.
- The chain is not RNG-failure-complete, not side-channel-free, not
  external-review-complete, and not public-readiness evidence.

Residuals carried into NA-0474:

- transcript/signature/identity binding gap;
- qsc/refimpl/provider boundary residual;
- forced seam divergence risk;
- whole-label/domain-separation audit gap;
- downgrade/replay/stale public-record audit gap;
- side-channel and memory-erasure caveats;
- formal-model mapping residual;
- external-review readiness residual;
- release-claim boundary residual;
- supply-chain/provenance residual;
- public-safety / CI helper residual.

## Applicable Stewardship and Assurance Review

Level 1 stewardship is active. Level 2 and Level 3 remain future-gated. No
separate Directors are created. No steward has independent READY promotion,
merge authority, public-claim authority, or directive authority. Lead Director
final authority is preserved.

| Review item | Classification | Evidence | Disposition |
|---|---|---|---|
| Crypto / Protocol Steward | HIGH binding evidence gap | qsc `handshake/mod.rs`, `identity/mod.rs`, qsc handshake tests, refimpl crypto and Suite-2 helpers | Select qsc binding negative-test scope next |
| CI / Dependency / Release Health Steward | DEPENDENCY_HEALTH_ACCEPTED_SUPPORTING_ONLY | root cargo audit, nested qsc fuzz lock audit, cargo tree probes, public-safety | Supporting only; no vulnerability-free claim |
| Public Claims / External Review Steward | EXTERNAL_REVIEW_READINESS_INCREMENTAL | evidence chain, formal/vector/test residuals, public claim boundaries | No public claim expansion |
| Product / Demo / Service Boundary Steward | SERVICE_BOUNDARY_UNCHANGED | no qsl-server, qsl-attachments, qshield, qshield-cli, website, README, or public-doc mutation | Future work remains gated |
| Local Ops / Backup / Restore Steward | LOCAL_OPS_READ_ONLY_OK | qwork proof files, qsl-backup hash/source-list proof, response archive path | No backup/restore; no off-host or restore claim |
| Best-Known-Method Review | BEST_KNOWN_METHOD_FOR_SCOPE | read-only source/test/formal/vector inventory plus existing CI/dependency evidence | Audit is adequate for selecting successor |
| Hostile Cryptographer Review | BINDING_NEGATIVE_TEST_SCOPE_NEXT | direct code review of KEM/signature/transcript construction and negative-test gaps | Next scope should authorize exact qsc negative tests |
| Red-Team Review | STALE_REPLAY_ROLLBACK_INCLUDED_NEXT | attack scenarios for wrong key, stale record, replay, downgrade, cross-role | Include stale/replay/rollback in combined successor |
| Production SRE Review | RECOVERY_AND_CLAIM_BOUNDARY_RESIDUAL | local state, rollback, restore, and peer/contact stale state caveats | Future runbook lane remains residual |
| Side-Channel Caveat | SIDE_CHANNEL_RESIDUAL_ACTIVE | no constant-time or memory-erasure completeness proof | Carry forward as caveat, not P0 |
| Formal-Model Mapping Residual | FORMAL_MODEL_MAPPING_RESIDUAL_ACTIVE | formal qsc suite-id model is crypto-agnostic and abstract | Future mapping remains residual |
| External-Review Readiness | EXTERNAL_REVIEW_READINESS_INCREMENTAL | current evidence remains internal and bounded | External package not selected yet |
| Release-Claim Boundary | RELEASE_CLAIM_BOUNDARY_PRESERVED | no public docs/website/readiness edits | No readiness or completion claim |
| Assurance Gap Review Trigger | HIGHER_PRIORITY_RESIDUAL_SUPERSEDES_ASSURANCE_REVIEW | exact NA-0475 successor selected | No separate assurance-gap review required now |

## KEM binding review

Classification: `KEM_BINDING_NEGATIVE_TEST_SCOPE_NEXT`.

qsc KEM public keys are identity material and are carried in A1. qsc computes
the primary peer identity fingerprint from the A1 KEM public key and compares
it to the contact pin before the responder encapsulates. A1 includes suite
context, session ID, KEM public key, signature public key, and DH public key.
The initiator stores the generated KEM secret key and public key in pending
state before sending A1.

KEM ciphertext is carried in B1 and is transcript-bound through
`hs_encode_resp_no_auth`, `hs_transcript_mac`, and `hs_transcript_hash`. The
initiator decapsulates B1 with the pending KEM secret key, derives
`pq_init_ss` with session ID and suite context, validates the transcript MAC,
then verifies B1 signature over a label-prefixed B1 signature message carrying
session ID and transcript hash.

The KEM shared secret feeds `hs_pq_init_ss`, which includes session ID, a
purpose byte, and active suite context. It then feeds the Suite-2 session state
through `init_from_base_handshake` and feeds confirmation key derivation.

Direct supporting tests exist for KEM provider RNG failure, provider decap
failure no-mutation, pqkem768 roundtrip/tamper/wrong-length behavior, B1
transcript mutation, and suite-context mismatch. However, the current test
inventory does not directly cover all negative binding cases for swapped A1 KEM
public key, stale KEM public record, wrong peer KEM key with contact state, or
replayed KEM ciphertext as a named qsc binding matrix. Forced KEM provider RNG
tests prove selected failure behavior only; they do not prove KEM binding
correctness.

refimpl `StdCrypto` provides ML-KEM keypair, encap, decap, and length/fail-closed
tests. That is supporting provider evidence, not a complete qsc binding proof.

## Signature binding review

Classification: `SIGNATURE_BINDING_NEGATIVE_TEST_SCOPE_NEXT`.

B1 signs `QSC.HS.SIG.B1 || session_id || transcript_hash`. The transcript hash
is derived from A1 plus B1-without-auth using `pq_init_ss` as KMAC key, so B1
signature indirectly covers suite context, A1 KEM public key, A1 signature
public key, A1 DH public key, B1 KEM ciphertext, B1 signature public key, B1 DH
public key, and session ID. A2 signs `QSC.HS.SIG.A2 || session_id ||
transcript_hash || confirm_mac`.

Verification uses the B1 signature public key from B1 and the A2 signature
public key stored from A1 pending state. qsc also computes signature public-key
fingerprints and checks optional contact signature pins when present.

Existing tests directly cover B1 signature tamper rejection, A2 signature tamper
rejection, B1/A2 provider RNG failure behavior, primary identity mismatch, and
selected suite-context transcript mismatch. Current tests are supporting only
for full domain-separation and cross-message replay because there is no exact
negative matrix for B1 signature replayed as A2, A2 signature replayed as B1,
wrong role, wrong suite, mutated transcript with preserved signature length,
wrong peer signature public key, or stale signature public key after identity
rotation. Forced signature provider RNG tests prove signing failure behavior
only; they do not prove signature-binding correctness.

refimpl signature behavior covers ML-DSA sign/verify and tamper support in
provider tests. It does not duplicate the qsc B1/A2 transcript semantics and is
supporting only for this audit.

## Transcript / KDF / confirmation binding review

Classification: `TRANSCRIPT_NEGATIVE_TEST_SCOPE_NEXT`.

qsc transcript MAC/hash inputs are A1 and B1-without-auth. Because A1/B1
encoding includes the header and explicit suite context when active, transcript
binding covers suite context, message type, session ID, KEM public key, KEM
ciphertext, signature public keys, and DH public keys. Signature bytes are
excluded from the B1 transcript by construction and instead verify over the
transcript hash. A2 confirmation MAC covers session ID, transcript hash, an
`A2` role/message label, and suite context. Confirmation key derivation covers
session ID, transcript hash, and suite context.

Suite context is also included in `hs_pq_init_ss` and `hs_dh_init_from_shared`.
Suite-required mode rejects legacy and mismatched suite context. Existing qsc
suite-id tests cover legacy-required reject, unsupported/downgraded/stripped/
duplicate/unknown/noncanonical/malformed suite block, A1/B1 mismatch, B1/A2
mismatch, transcript-binding mismatch, key-context missing, replayed A1, and
replayed A2 cases.

Residual gaps remain because qsc negative tests do not yet present an explicit
combined binding matrix for transcript truncation, cross-role replay, cross-
message signature replay, wrong peer KEM/signature key, stale identity/public
record, or replayed ciphertext under otherwise plausible pending state. The
formal qsc suite-id model treats transcript and key contexts as abstract values,
not cryptographic proof.

## Identity / public record / stale record review

Classification: `IDENTITY_STALE_RECORD_NEGATIVE_TEST_SCOPE_NEXT`.

qsc self public records store KEM and signature public keys. The KEM public key
fingerprint is the primary identity fingerprint. Signature public-key
fingerprints are optional contact pins. During handshake, primary identity pin
checks are required and fail closed when unknown or mismatched; signature pin
checks are optional but fail closed when present and mismatched.

Existing tests cover route-only first-contact reject without silent TOFU and
primary pinned identity mismatch with no session mutation. CLI rotation and
legacy/public-record provider RNG tests prove selected no-partial-write behavior
under forced provider RNG failure.

Stale public-record and local rollback semantics remain a direct gap. There is
no dedicated qsc negative suite for an old self public record reintroduced after
rotation, stale peer/contact state after identity rotation, stale signature
public key with current KEM key, stale KEM public key with current signature
key, or local rollback creating accepted stale identity material. This is
supporting evidence only for identity binding and P1 for NA-0475.

## Downgrade / replay / suite-confusion review

Classification: `DOWNGRADE_REPLAY_NEGATIVE_TEST_SCOPE_NEXT`.

qsc suite-required mode uses explicit suite context and rejects legacy required
mode, unsupported suite IDs, downgraded legacy tuples, stripped suite parameter,
duplicate parameters, unknown parameters, noncanonical order, malformed length,
inconsistent tuple, A1/B1 context mismatch, B1/A2 context mismatch, missing key
context, and replayed A1/A2 with suite context. The formal qsc suite-id model
also asserts no mutation, no output on reject, no secret leak, and deterministic
reason labels for bounded abstract suite-id scenarios.

This is strong direct evidence for suite-context negative cases. It is not
downgrade-proof evidence and not replay-proof evidence across stale identity
records, stale session IDs, wrong-role replay, cross-protocol/domain confusion,
or mixed KEM/signature/transcript attacks. Those need exact qsc negative-test
authorization.

## qsc / refimpl mapping review

Classification: `QSC_REFIMPL_MAPPING_ACCEPTED_SUPPORTING_ONLY`.

qsc relies on refimpl `StdCrypto` for ML-KEM keypair/encap/decap, ML-DSA
sign/verify, X25519, KMAC, and Suite-2 establishment. refimpl has direct tests
for pqkem768 roundtrip, tamper, and wrong-length inputs; Suite-2 binding helpers
bind associated data to session ID, protocol version, suite ID, DH public key,
flags, and PQ binding.

The qsc handshake has its own framing, transcript, confirmation, and identity
pin semantics. Those semantics are not fully duplicated by refimpl tests or
formal models. qsc/refimpl mapping is therefore accepted as supporting evidence
for NA-0474 and should remain in the NA-0475 read-only candidate path set, but
the next lane should prioritize qsc negative-test scope before a standalone
mapping lane.

## Formal / vector / fuzz coverage review

Classification: `FORMAL_VECTOR_FUZZ_BINDING_SUPPORTING_ONLY`.

Formal coverage includes bounded SCKA, Suite-2 negotiation downgrade/no-mutation,
and qsc handshake suite-id semantics. The qsc suite-id model captures legacy
compatibility versus suite-required mode, canonical suite parameter blocks,
A1/B1/A2 context equality, abstract transcript/key-context binding, no mutation
on reject, no output on reject, and deterministic reason labels. It does not
prove cryptographic authentication, KEM binding, signature domain separation,
identity/public-record freshness, side-channel behavior, or implementation
memory erasure.

Vector coverage exists for Suite-2 transcript, downgrade, establish, SCKA,
replay, parse, boundary, E2E receive, and qsc handshake suite-id scenarios.
The qsc suite-id vector file explicitly marks older qsc harness expectations as
future-gated for several model-derived cases. There is no dedicated negative
vector suite for stale identity, wrong KEM, wrong signature public key, wrong
role, replayed KEM ciphertext, or stale public-record rollback across qsc
runtime.

Fuzz targets focus on route HTTP, payload boundaries, and vault envelope
boundaries. They support parser/boundary robustness, not full KEM/signature/
transcript binding checks.

## Threat / attack scenario review

| Scenario | Direct coverage | Supporting evidence | Future negative-test scope | Severity | Priority |
|---|---|---|---|---:|---|
| Stale public record replay | No | primary identity mismatch and provider RNG no-partial-write tests | Required | HIGH | P1 |
| Wrong peer signature public key | Partial | optional signature pin checks; B1/A2 signature tamper | Required | HIGH | P1 |
| Wrong peer KEM public key | Partial | primary identity pin mismatch | Required | HIGH | P1 |
| Cross-role message replay | Partial | A1/A2 replay and out-of-order tests | Required | MEDIUM | P1 |
| Cross-suite downgrade | Yes for suite context | qsc suite-id tests and formal model | Extend into combined binding matrix | HIGH | P1 |
| Replayed A1 | Yes for suite-context path | NA-0313 qsc suite-id test | Include in combined matrix | MEDIUM | P1 |
| Replayed B1 | Partial | session ID/transcript checks | Required | MEDIUM | P1 |
| Replayed A2 | Yes for selected cases | handshake_mvp and NA-0313 tests | Include in combined matrix | MEDIUM | P1 |
| Mixed transcript mutation | Yes for selected B1 mutation | NA-0313 transcript mismatch | Extend to truncation/cross-role | HIGH | P1 |
| Public-record rollback | No | CLI rotation no-partial-write and mismatch tests | Required | HIGH | P1 |
| Identity rotation race | Partial | CLI rotation no-partial-write | Required | MEDIUM | P2 |
| Relay/server stale message delivery | Partial | replay tests and transport replay evidence | Required for handshake stale messages | MEDIUM | P2 |
| Local storage rollback | Partial | protocol state stale messaging and backup caveats | Required | HIGH | P1 |
| Peer/contact stale state | Partial | primary identity mismatch | Required | HIGH | P1 |

## Findings matrix and prioritization

| ID | Domain | Finding | Evidence | Direct/supporting/none | Severity | Likelihood | Impact | Public-claim risk | Implementation risk | Recommended action | Candidate successor | Goals | Disposition |
|---|---|---|---|---|---:|---:|---:|---:|---:|---|---|---|---|
| F-01 | KEM | KEM binding lacks a named negative-test matrix for swapped/stale/wrong peer KEM public key and replayed KEM ciphertext cases | qsc A1/B1 code, `identity_binding`, provider-error tests | Supporting | HIGH | MEDIUM | HIGH | HIGH | MEDIUM | Authorize exact qsc negative-test scope | NA-0475 combined binding negative-test scope | G1, G3, G4 | P1 next |
| F-02 | Signature | Signature binding lacks exact cross-message, wrong-role, wrong-suite, stale signature public-key, and domain-separation negative cases | B1/A2 signature code and tamper tests | Supporting | HIGH | MEDIUM | HIGH | HIGH | MEDIUM | Authorize exact qsc negative-test scope | NA-0475 combined binding negative-test scope | G3, G4 | P1 next |
| F-03 | Transcript | Transcript mutation/replay coverage is strong for suite context but incomplete as a combined KEM/signature/transcript matrix | qsc suite-id tests, handshake_mvp tests | Direct selected, supporting overall | HIGH | MEDIUM | HIGH | HIGH | MEDIUM | Authorize exact qsc negative-test scope | NA-0475 combined binding negative-test scope | G3, G4 | P1 next |
| F-04 | Identity/public record | Stale public-record and identity rollback are not directly covered as runtime negative tests | identity public-record code, identity mismatch tests, CLI rotation evidence | Supporting | HIGH | MEDIUM | HIGH | HIGH | MEDIUM | Include stale-record and rollback cases in NA-0475 | NA-0475 combined binding negative-test scope | G2, G3, G4 | P1 next |
| F-05 | Downgrade/suite confusion | Suite-context downgrade coverage exists, but mixed-suite plus stale identity and replay combinations are not complete | NA-0313 qsc suite-id tests, formal model | Direct selected, supporting overall | HIGH | LOW | HIGH | HIGH | MEDIUM | Extend negative-test matrix | NA-0475 combined binding negative-test scope | G3, G4 | P1 next |
| F-06 | qsc/refimpl mapping | qsc handshake binding semantics are not fully mirrored by refimpl tests | qsc handshake source, refimpl crypto/Suite-2 helpers | Supporting | MEDIUM | MEDIUM | MEDIUM | MEDIUM | LOW | Keep read-only mapping in next lane, but do not outrank qsc tests | qsc/refimpl mapping lane later | G4 | P2 backlog |
| F-07 | Formal model | Formal qsc suite-id model is abstract and does not prove cryptographic binding | formal qsc model and README scope | Supporting | MEDIUM | MEDIUM | MEDIUM | MEDIUM | LOW | Preserve formal mapping residual | formal mapping lane later | G4 | P2 backlog |
| F-08 | Vectors | No dedicated qsc negative vector suite for stale identity, wrong KEM, wrong signature, wrong role, and replayed ciphertext | inputs suite2 and qsc suite-id vectors | Supporting | MEDIUM | MEDIUM | MEDIUM | MEDIUM | LOW | Consider vector suite after scope authorization | binding negative vector suite later | G4 | P2 backlog |
| F-09 | Fuzz | Existing fuzz targets exercise parser/boundary surfaces, not full binding checks | qsc fuzz targets | Supporting | LOW | MEDIUM | MEDIUM | MEDIUM | LOW | Backlog fuzz binding target design | fuzz binding backlog | G4 | P2 backlog |
| F-10 | Side-channel | No side-channel-free or memory-erasure completeness proof | NA-0473 caveats, key lifecycle evidence | Supporting | HIGH | UNKNOWN | HIGH | HIGH | HIGH | Carry caveat; do not select unless it rises to P0 | side-channel/secret-material lane later | G4 | accepted caveat |

## Successor selection

Selected exactly one NA-0475 successor:

`NA-0475 -- QSL qsc KEM / Signature / Transcript Binding Negative Test Scope Authorization Plan`

Rationale:

- multiple direct negative-test gaps exist across KEM, signature, transcript,
  identity/public-record, replay, downgrade, and suite-confusion binding;
- existing qsc code and tests make a future exact authorization lane practical;
- a narrower stale-record-only, transcript-only, or signature-only lane would
  leave equally important adjacent binding gaps unresolved;
- a formal mapping lane is valuable but lower priority than authorizing runtime
  negative tests for the directly observed qsc gap matrix;
- implementation should not be selected immediately because exact future test
  paths and acceptance criteria should be authorized first.

Assurance trigger classification:

`HIGHER_PRIORITY_RESIDUAL_SUPERSEDES_ASSURANCE_REVIEW`

## Future scope bundle

For the selected NA-0475 successor, future governance scope should use:

- `docs/governance/evidence/NA-0475_qsl_qsc_kem_signature_transcript_binding_negative_test_scope_authorization_plan.md`
- `tests/NA-0475_qsl_qsc_kem_signature_transcript_binding_negative_test_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future read-only/source paths for possible later implementation authorization:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/src/protocol_state/`
- `qsl/qsl-client/qsc/tests/`
- `tools/refimpl/`
- `formal/`
- `inputs/`

Future forbidden scope unless a later exact directive authorizes it:

- runtime or crypto implementation mutation;
- dependency, Cargo, lockfile, or workflow mutation;
- executable test, fuzz target, vector, formal model, or refimpl mutation
  outside exact future paths;
- qshield-cli, qsl-server, qsl-attachments, public docs, website, README,
  START_HERE, backup/restore/qsl-backup, or public-claim mutation.

## Public claim / external review / website boundary

NA-0474 is internal governance evidence only. It is not production readiness.
It is not public-internet readiness. It is not public readiness. It is not
crypto-complete proof. It is not KEM-complete proof. It is not
signature-complete proof. It is not identity-complete proof. It is not
transcript-complete proof. It is not downgrade-proof. It is not replay-proof.
It is not side-channel-free proof. It is not RNG-failure-complete proof. It is
not provider-RNG-complete proof. It is not vulnerability-free proof. It is not
bug-free proof. It is not perfect-crypto proof. It is not external-review-
complete proof. It is not public technical paper content.

No README, START_HERE, public docs, docs-public, website, or public claim update
is made. Cargo audit green is dependency-health evidence only. Future tests, if
authorized, must be described as bounded evidence only.

## Rejected alternatives

| Alternative | Disposition | Reason |
|---|---|---|
| Select stale/public-record only | Rejected | Important, but KEM/signature/transcript gaps are adjacent and equally material |
| Select signature domain separation only | Rejected | Domain separation is part of a broader binding matrix |
| Select transcript mutation/replay only | Rejected | Would not cover wrong KEM/signature key and stale identity/public-record cases |
| Select formal model mapping next | Rejected for immediate successor | Valuable P2, but qsc runtime negative-test scope is higher value now |
| Select qsc/refimpl mapping next | Rejected for immediate successor | Supporting residual, lower priority than qsc negative tests |
| Select side-channel / secret-material next | Rejected for immediate successor | Active caveat but not the highest immediate binding gap |
| Select external review readiness | Rejected | Core binding gaps are not yet sufficiently resolved |
| Select implementation directly | Rejected | Exact future test scope should be authorized before implementation |
| Select triage only | Rejected | Findings are clear enough to choose a successor |

## Backup-impact statement

No backup, restore, sudo, qsl-backup mutation, backup status mutation, backup
plan mutation, rollback subtree mutation, systemd/timer/fstab mutation, or
`/backup/qsl` mutation occurred in NA-0474. qsl-backup SHA matched the directive
value, and the qsl-backup script source-list inclusion count for the Codex ops
path was exactly one. This is same-host local-ops boundary evidence only. It is
not off-host backup proof, not restore proof, not disaster-recovery proof, not
backup-complete proof, and not key-custody proof.

## Next recommendation

Proceed to the selected NA-0475 authorization lane after NA-0474 evidence is
merged and, if public-safety remains green, NA-0474 is closed out. NA-0475
should authorize exact qsc negative-test scope for KEM public key/ciphertext,
signature key/signature replay/domain separation, transcript mutation/replay,
stale identity/public-record rollback, downgrade, suite confusion, wrong role,
wrong peer, and no-mutation expectations. It should not implement tests unless a
later directive authorizes exact implementation.
