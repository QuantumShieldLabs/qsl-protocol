Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-08

# NA-0443 QSL Key Lifecycle Secret Cleanup / Zeroization Scope Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0443 consumes the NA-0441 and NA-0442 key-lifecycle findings and selects the
next exact scope for secret cleanup and zeroization evidence.

Primary classification:

`KEY_LIFECYCLE_SECRET_CLEANUP_EVIDENCE_POLICY_NEXT`

Selected successor:

`NA-0444 -- QSL Key Lifecycle Secret Cleanup / Zeroization Evidence Policy Authorization Plan`

Reason: qsc, refimpl, and qshield-cli carry different secret-material roles and
claim boundaries. qsc has pending handshake material, encrypted session blobs,
vault-held secret values, and selected zeroization evidence. refimpl has
provider and Suite-2 material with narrow `ZeroizeOnDrop` evidence only for the
X25519 private wrapper. qshield-cli intentionally persists demo-local
establishment/session material. The exact implementation or test paths are not
safe enough to authorize directly until a policy lane defines cleanup and
zeroization expectations across these classes.

No implementation mutation is authorized by NA-0443. No runtime, crypto,
dependency, Cargo, lockfile, workflow, executable-test, fuzz-target, vector,
formal-model, qsl-server, qsl-attachments, qshield runtime, website, public-doc,
README, START_HERE, qwork/qstart/qresume/qshell, backup, restore, qsl-backup,
backup status, backup plan, rollback, or local-ops mutation is authorized or
performed.

## Live NA-0443 scope

Live `NEXT_ACTIONS.md` lists:

`NA-0443 -- QSL Key Lifecycle Secret Cleanup / Zeroization Scope Authorization Plan`

Status: READY.

Allowed NA-0443 mutation paths:

- `docs/governance/evidence/NA-0443_qsl_key_lifecycle_secret_cleanup_zeroization_scope_authorization_plan.md`
- `tests/NA-0443_qsl_key_lifecycle_secret_cleanup_zeroization_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Read-only inspection included qwork proof files, the live NA-0443 queue block,
NA-0441 and NA-0442 evidence/testplans, D-0869 through D-0872, TRACEABILITY,
the rolling journal, the domain stewardship canon, qsc/refimpl/qshield-cli
source and tests, formal models, inputs, Cargo manifests and locks, scripts,
workflows, qsl-backup boundary proof, backup manifest source-list proof, and
prior response files.

Forbidden mutation scope includes runtime code, crypto code, dependencies,
Cargo manifests, lockfiles, workflows, executable tests, fuzz targets, vectors,
formal model files, qsl-server, qsl-attachments, qshield runtime, website,
public docs, README, START_HERE, qwork/qstart/qresume/qshell, qsl-backup,
backup status, backup plan, rollback subtree, `/backup/qsl`, backup tree,
systemd/timer/fstab state, public technical paper content, and public claim
surfaces.

Acceptance criteria:

- qwork proof files are verified without running qwork, qstart, or qresume;
- NA-0441 and NA-0442 findings are consumed;
- F-0441-02 is selected as the current lane focus;
- qsc/refimpl/qshield-cli surfaces are classified separately;
- one exact NA-0444 successor is selected;
- no implementation mutation occurs;
- root cargo audit remains green;
- nested qsc fuzz lock audit remains green;
- provider-error test and qsc adversarial script evidence remain healthy;
- public-safety is green before merge and after merge;
- exactly one READY item remains.

Stop conditions include missing or inconsistent qwork proof, PR #1154 not
merged, queue drift from READY NA-0443, D-0872 absence, D-0873 preexistence,
audit failures, unconsumable NA-0441/NA-0442 findings, unsafe surface
classification, unsafe successor selection, qsl-backup source-list regression,
forbidden mutation, backup or restore execution, public overclaim, or more than
one READY item.

## qwork proof-file verification

Codex read, but did not run, qwork proof files:

- `/srv/qbuild/work/NA-0443/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0443/.qwork/startup.qsl-protocol.json`

Required `.kv` markers passed:

- `startup_result=OK`
- `lane=NA-0443`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0443/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0443`
- `requested_lane_status=READY`

The JSON proof parsed successfully and mirrored the `.kv` proof for lane, repo,
path, HEAD, origin/main, ready count, top READY item, requested lane status,
and clean-state fields.

Initial live `HEAD` and `origin/main` matched the qwork proof at
`ada372a1998b`. After `git fetch --all --prune`, `origin/main` still matched
the proof. PR #1154 was verified MERGED with merge commit `ada372a1998b`.

Recorded timestamps:

- Local: `2026-06-08T07:16:17-05:00`
- UTC: `2026-06-08T12:16:17+00:00`

Proof root:

`/srv/qbuild/tmp/NA0443_key_lifecycle_secret_cleanup_scope_20260608T121739Z`

## NA-0441 / NA-0442 findings inheritance

NA-0441 selected:

`NONCE_KEY_RNG_EVIDENCE_GAPS_FOUND`

NA-0442 selected:

`NONCE_KEY_RNG_TRIAGE_SECRET_CLEANUP_SCOPE_NEXT`

Inherited finding focus:

- F-0441-02: selected zeroize/redaction evidence exists, but no comprehensive
  cleanup/wipe expectation exists for all pending/session/shared-secret
  material.
- F-0441-03: RNG failure behavior remains the next candidate after this lane.
- F-0441-06: qshield-cli demo-local material remains claim-boundary/backlog.

Provider-error evidence is complete enough for background only and is not
reopened by NA-0443. `pq_decap_failed` no-mutation evidence remains bounded to
the existing deterministic test and qsc adversarial script integration.
`pq_encap_failed` remains defensive-branch documentation only.

## Applicable Stewardship Review

### Crypto / Protocol Steward

Secret cleanup and zeroization is a high-sensitivity evidence scope. The current
state shows cleanup evidence gaps, not a confirmed runtime vulnerability. qsc
runtime pending/session/vault material, refimpl provider material, and
qshield-cli demo-local material must remain separate. Side-channel caveats
remain open. No secret-material-complete claim is made.

### CI / Dependency / Release Health Steward

Root `cargo audit --deny warnings` passed. Nested qsc fuzz lock audit passed.
`rustls-webpki` is `v0.103.13`. Root pqcrypto inverse probes produced expected
package-absence output. Nested qsc fuzz lock pqcrypto residual scan returned
zero matches. The provider-error no-mutation test passed and
`scripts/ci/qsc_adversarial.sh` still carries the NA-0439 marker and command.
Public-safety was required and green on current main.

Cargo audit green is dependency-health evidence only. No vulnerability-free
claim is made.

### Public Claims / External Review Steward

This scope authorization is internal governance evidence only.
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
Evidence gaps are called gaps, not completions.

### Product / Demo / Service Boundary Steward

qshield-cli evidence remains demo-local. qsc runtime secret cleanup and
qshield-cli demo-local persistence are not conflated. No qsl-server,
qsl-attachments, qshield runtime, website, or public-service readiness claim is
made. No public-readiness claim is made. No production-readiness claim is made.

### Local Ops / Backup / Restore Steward

No backup, restore, sudo, qwork, qstart, or qresume was run. qsl-backup proof
remains boundary evidence only: the qsl-backup checksum matched the expected
boundary value and the Codex ops source-list inclusion count was exactly one.
No qsl-backup, backup status, backup plan, rollback subtree, backup tree, or
local-ops mutation is authorized or performed.

Level 1 stewardship is active. Level 2 and Level 3 remain future-gated.
Stewards remain advisory only: no separate Directors, no independent READY
promotion, no independent merge authority, and Lead Director final authority is
preserved.

## Secret-material surface inventory

| Surface | Material type | Lifecycle role | Current evidence | Future mutation type | Exact path scope obvious? |
|---|---|---|---|---|---|
| `qsl/qsl-client/qsc/src/handshake/mod.rs` | pending KEM secret key, KEM public key, DH private/public material, confirm key, transcript hash, pending session snapshot, PQ/DH shared-secret derivations | generation, pending state, session state, cleanup/clear | code; `hs_pending_store`; `hs_pending_clear`; provider-error no-mutation tests | policy first; later runtime/test only if authorized | No. Multiple pending and reject paths need expectation mapping. |
| `qsl/qsl-client/qsc/src/protocol_state/mod.rs` | session-store encryption key, encrypted session snapshot plaintext/ciphertext | generation, persistence, encrypted at rest, session state | code and tests; random session blob nonce; peer-bound AAD; fail-closed decrypt | policy first; possible future runtime/test | No. Key/plaintext wipe expectations are not yet selected. |
| `qsl/qsl-client/qsc/src/vault/mod.rs` | vault key, passphrase buffers, secret map values, vault ciphertext | vault/config, persistence, encryption, zeroization | code and tests; selected zeroize calls; `VaultSession` drop zeroizes key and secret values | policy first; possible future runtime/test | Partly. Vault evidence exists, but scope must separate existing evidence from gaps. |
| `qsl/qsl-client/qsc/tests/**` | no-mutation fixtures, vault/session proof, output redaction checks | test fixture, logging/redaction, reject no-mutation | executable tests | future test scope only after policy | No. Existing tests cover many adjacent properties but not a comprehensive cleanup standard. |
| `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs` | ML-KEM keys/shared secrets, ML-DSA seed/signing key, X25519 private key, AEAD keys/nonces | generation, transport, crypto provider | code and unit tests | policy first; possible future runtime/test | No. Provider material differs by algorithm and wrapper type. |
| `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs` | X25519 private wrapper | zeroization | code and unit tests via `Zeroize`/`ZeroizeOnDrop` | policy/documentation; possible future test | Partly. Existing trait evidence is narrow. |
| `tools/refimpl/quantumshield_refimpl/src/suite2/**` and `src/qsp/**` | session IDs, chain keys, skipped message keys, shared-secret contributions | session state, transport, test fixture | code, tests, vectors | policy first; possible future test | No. Session/ratchet material requires separate expectation mapping. |
| `apps/qshield-cli/src/store.rs` and `apps/qshield-cli/src/commands/establish.rs` | demo-local `pq_init_ss_hex`, `dh_init_hex`, session identifiers, demo relay records | demo-local store, persistence | code and tests | claim-boundary/backlog; not immediate runtime cleanup | Yes for claim boundary, no for runtime mutation. |
| `formal/*.py` and `formal/README.md` | model state and secret/sentinel leak flags | formal supporting evidence | formal/model | future model only after exact scope | No. Current models do not directly cover zeroization or cleanup. |
| `inputs/suite2/**` | KDF, SCKA, KEM, negotiation, suite-id vectors | vectors, transport/session binding | vector inputs | future vector only after exact scope | No. Inputs support binding/no-leak assertions, not cleanup completion. |
| `docs/governance/evidence/**` | prior findings, caveats, scope decisions | governance evidence | governance | governance-only | Yes for NA-0444 governance policy. |

## qsc runtime secret cleanup scope review

qsc pending/session/shared-secret material exists in at least these forms:

- initiator pending KEM secret key and DH private key in `HandshakePending`;
- responder pending confirm key, transcript hash, and pending session snapshot;
- PQ and DH shared-secret derivations during handshake processing;
- session-store encryption key stored through the vault;
- encrypted session snapshot plaintext during store/load;
- vault passphrase buffers, vault key, and secret map values.

Persistence:

- pending handshake records are stored as vault secret entries and legacy
  pending files may be migrated then removed;
- session snapshots are persisted as encrypted `.qsv` blobs;
- session-store key material is persisted through the vault;
- vault payloads are encrypted at rest.

Cleanup evidence:

- `hs_pending_clear` clears vault pending entries and removes legacy pending
  files after selected accept/reject paths;
- `VaultSession` drop zeroizes the vault key and secret values;
- selected passphrase/key buffers are explicitly zeroized;
- encrypted-at-rest and redaction tests exist.

Gaps:

- no comprehensive expectation says when each pending/session/shared-secret
  material class must be cleared, zeroized, or merely encrypted at rest;
- no single test suite proves cleanup/zeroization across all pending/session
  paths;
- no formal/vector evidence directly models cleanup or wipe semantics.

qsc runtime classification:

`QSC_SECRET_CLEANUP_EVIDENCE_POLICY_NEEDED`

Direct runtime implementation is not safe to authorize here. A future policy
lane should decide whether later qsc work is documentation-only, test-only, or
runtime-scoped.

## refimpl secret cleanup scope review

Existing evidence:

- `X25519Priv` derives `Zeroize` and `ZeroizeOnDrop` when `stdcrypto` is
  enabled;
- tests assert the X25519 wrapper zeroize traits and clearing behavior;
- `StdCrypto` uses OS/provider randomness for ML-KEM, ML-DSA, X25519, KEM
  encapsulation, and random nonces;
- provider tests cover ML-KEM roundtrip, tamper behavior, and wrong-length
  fail-closed behavior.

Gaps:

- comprehensive cleanup expectations for ML-KEM secret keys, ML-KEM shared
  secrets, ML-DSA seed/signing material, AEAD plaintext/key use, Suite-2
  chainkeys, skipped message keys, and serialized session snapshots are not
  stated as one policy;
- refimpl is reference/runtime evidence for this repo, but qsc runtime and
  qshield-cli demo material have different boundaries.

refimpl classification:

`REFIMPL_SECRET_CLEANUP_SCOPE_INCLUDED`

This means included in the NA-0444 evidence-policy scope only. It does not
authorize refimpl runtime or test mutation in NA-0443.

## qshield-cli demo-local boundary review

qshield-cli is explicitly demo-local. Its README states the CLI is
non-production and local-demo only. `apps/qshield-cli/src/store.rs` persists a
`SessionEntry` containing `session_id_b64u`, `session_id_hex`, `dh_init_hex`,
`pq_init_ss_hex`, demo public-key IDs, and peer DH public keys. Establishment
derives deterministic demo material from local IDs and public key placeholders,
passes it to the refimpl actor, and records local demo state.

Classification:

`QSHIELD_DEMO_KEY_MATERIAL_CLAIM_BOUNDARY_ONLY`

qshield-cli should not be folded into immediate qsc runtime cleanup. It should
remain a claim-boundary/backlog surface unless a later exact lane authorizes a
demo-storage policy or implementation review.

## Evidence policy vs implementation scope decision

Selected option:

Option 1 - Evidence policy / documentation scope.

Rationale:

- qsc, refimpl, and qshield-cli are different classes of evidence;
- exact runtime/test paths are not yet safe enough for direct implementation;
- the gap is currently an evidence-policy gap, not a confirmed active security
  blocker;
- a policy lane can define cleanup/zeroization expectations without runtime,
  crypto, dependency, Cargo, lockfile, workflow, executable-test, fuzz-target,
  vector, formal-model, service, public-surface, or backup mutation.

Rejected alternatives:

- Option 2 - Test-scope authorization: rejected because exact future tests
  should follow a policy that names expected cleanup semantics.
- Option 3 - Runtime-scope authorization: rejected because exact runtime
  invariants and path scope are not yet clear enough.
- Option 4 - Split qsc runtime, refimpl, and qshield-cli into separate lanes:
  rejected for immediate successor because the first needed step is a shared
  evidence policy with explicit boundaries; split implementation lanes can
  follow if the policy requires them.
- Option 5 - Backlog/no immediate action: rejected because F-0441-02 is the
  selected immediate successor and has meaningful public-claim risk.
- Option 6 - Stop/ambiguity: rejected because evidence-policy scope is safe and
  exact enough.

Public-claim implication: evidence gaps must remain gaps. Partial zeroize or
redaction evidence must not be converted into a public assurance claim.

## Prioritization / scope matrix

| Surface | Material type | Current evidence | Gap | Scope recommendation | Candidate future paths | Immediate successor? | Risk | Public-claim implication | Goals |
|---|---|---|---|---|---|---|---|---|---|
| qsc pending KEM secret / pending handshake material | KEM secret key, DH private key, confirm key, transcript hash, pending session snapshot | code; `hs_pending_store`; `hs_pending_clear`; provider-error no-mutation test | no comprehensive cleanup/wipe expectation across pending roles and rejects | Evidence policy first | `qsl/qsl-client/qsc/src/handshake/mod.rs`; qsc handshake tests if later authorized | Yes, policy only | Medium evidence gap | No secret-material-complete claim. | G1, G2, G3, G4 |
| qsc session/shared secret / session store material | session-store key, encrypted session snapshots, chain keys | encrypted `.qsv` blobs; AAD; session tests | plaintext/session-key wipe expectations not stated | Evidence policy first | `qsl/qsl-client/qsc/src/protocol_state/mod.rs`; session tests if later authorized | Yes, policy only | Medium evidence gap | No crypto-complete claim. | G1, G2, G4 |
| qsc vault/passphrase/runtime key material | passphrases, vault key, secret values | selected zeroize calls; `VaultSession` drop; vault tests | policy does not map all vault-held secret classes to cleanup expectations | Evidence policy first | `qsl/qsl-client/qsc/src/vault/mod.rs`; vault tests if later authorized | Yes, policy only | Medium evidence gap | No side-channel-free claim. | G2, G3, G4 |
| qsc logging/redaction surfaces | command output, markers, diagnostics | broad redaction/no-secret tests | redaction is not equivalent to in-memory cleanup | Preserve as supporting evidence | qsc output tests if later authorized | Yes, supporting policy row | Claim-boundary | No secret-material-complete claim from redaction. | G4, G5 |
| refimpl private/secret material with existing zeroize evidence | X25519 private wrapper | `Zeroize`/`ZeroizeOnDrop` traits and tests | narrow to X25519 wrapper | Include in policy | `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs`; refimpl tests if later authorized | Yes, policy only | Low/medium | No broad zeroization claim. | G1, G4 |
| refimpl private/secret material without comprehensive evidence | ML-KEM keys/shared secrets, ML-DSA signing material, AEAD/session material | provider code and tests | no comprehensive cleanup policy | Include in policy | `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`; `src/suite2/**`; tests if later authorized | Yes, policy only | Medium evidence gap | No perfect-crypto claim. | G1, G2, G4 |
| qshield-cli demo-local stored shared secret or related material | `pq_init_ss_hex`, `dh_init_hex`, session IDs, demo records | demo-local code and tests | demo persistence must not be overclaimed as production/runtime cleanup | Claim-boundary/backlog in policy | `apps/qshield-cli/src/store.rs`; `src/commands/establish.rs` if later authorized | Yes, policy boundary only | Claim-boundary | No production-readiness or public-readiness claim. | G3, G4, G5 |
| formal/model evidence | modeled state, no-leak/no-mutation flags | SCKA, negotiation, qsc suite-id model checks | no zeroization/cleanup model | Supporting-only in policy | `formal/*.py` if later authorized | No direct model change | Evidence incomplete | No formal-proof-complete claim. | G2, G4 |
| tests covering no-mutation/reject/redaction/zeroization | qsc/refimpl/qshield tests | many bounded tests; provider-error no-mutation green | no single cleanup/zeroization standard | Inventory in policy; future test scope after policy | relevant test files if later authorized | Yes, policy only | Medium evidence gap | No exhaustive-testing claim. | G4, G5 |

## Authorization decision

Primary classification:

`KEY_LIFECYCLE_SECRET_CLEANUP_EVIDENCE_POLICY_NEXT`

This directive authorizes only governance evidence for NA-0443. It does not
authorize runtime, crypto, dependency, Cargo, lockfile, workflow,
executable-test, fuzz-target, vector, formal-model, service, public-surface, or
backup mutation. It does not authorize a public claim expansion.

## Successor selection

Selected NA-0444 successor:

`NA-0444 -- QSL Key Lifecycle Secret Cleanup / Zeroization Evidence Policy Authorization Plan`

NA-0444 should define a bounded internal evidence policy for cleanup and
zeroization expectations across qsc/refimpl/qshield-cli surfaces. It should not
implement cleanup, add zeroization, add tests, change runtime behavior, change
crypto behavior, change dependencies, or change public surfaces unless a later
exact directive authorizes that work.

## Future path/scope bundle

Future allowed NA-0444 mutation paths:

- `docs/governance/evidence/NA-0444_qsl_key_lifecycle_secret_cleanup_zeroization_evidence_policy_authorization_plan.md`
- `tests/NA-0444_qsl_key_lifecycle_secret_cleanup_zeroization_evidence_policy_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0444 may inspect read-only:

- `qsl/qsl-client/qsc/src/`
- `qsl/qsl-client/qsc/tests/`
- `tools/refimpl/quantumshield_refimpl/src/`
- `tools/refimpl/quantumshield_refimpl/tests/`
- `apps/qshield-cli/`
- `formal/`
- `inputs/`
- `docs/governance/evidence/`
- `Cargo.toml`
- `Cargo.lock`
- `qsl/qsl-client/qsc/fuzz/`
- relevant scripts/workflows read-only.

Future forbidden unless exact scope authorizes:

- runtime/crypto implementation changes;
- dependency, Cargo, or lockfile changes;
- workflow changes;
- test source changes;
- fuzz target source changes;
- vector changes;
- formal model changes;
- public docs, README, START_HERE, or website changes;
- qsl-server or qsl-attachments changes;
- backup, restore, qsl-backup, backup status, backup plan, rollback, or
  backup tree changes;
- public claims.

## Future validation/marker plan

Common NA-0444 markers:

- `NA0444_KEY_LIFECYCLE_SCOPE_CONSUMED_OK`
- `NA0444_SECRET_CLEANUP_ZEROIZATION_SCOPE_SELECTED_OK`
- `NA0444_EVIDENCE_POLICY_AUTHORIZATION_OK`
- `NA0444_NO_RUNTIME_CHANGE_OK`
- `NA0444_NO_DEPENDENCY_CHANGE_OK`
- `NA0444_NO_WORKFLOW_CHANGE_OK`
- `NA0444_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0444_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0444_NO_SECRET_MATERIAL_COMPLETE_CLAIM_OK`
- `NA0444_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0444_ONE_READY_INVARIANT_OK`

## Public claim/external review/website boundary

Key lifecycle scope authorization is internal governance evidence only.

No production-readiness claim is made.
No public-internet-readiness claim is made.
No crypto-complete claim is made.
No side-channel-free claim is made.
No secret-material-complete claim is made.
No bug-free claim is made.
No vulnerability-free claim is made.
No perfect-crypto claim is made.
No public technical paper content is created.
No README, START_HERE, public docs, or website update is made.
Cargo audit green is dependency-health evidence only.
Evidence gaps are called gaps, not completions.

## Rejected alternatives

Direct qsc runtime cleanup implementation:

- Rejected because exact invariants and path scope are not yet selected.

Direct refimpl cleanup implementation:

- Rejected because refimpl evidence differs from qsc runtime evidence and needs
  a policy boundary first.

Direct qshield-cli demo material cleanup implementation:

- Rejected because qshield-cli is demo-local and claim-boundary scoped.

Direct test addition:

- Rejected because tests should follow a policy that names expected behavior.

Backlog/no action:

- Rejected because F-0441-02 is the inherited immediate successor and public
  claim risk remains meaningful.

## Backup-impact statement

No backup or restore was run. No sudo was run. qsl-backup was not mutated.
Backup status files, backup plan files, rollback subtree paths, timers, fstab,
source lists, retention, backup scripts, and `/backup/qsl` were not mutated.
qsl-backup proof is boundary evidence only and does not support an off-host
backup completion claim, restore proof claim, disaster recovery completion
claim, or backup completion claim.

## Next recommendation

After NA-0443 merges and post-merge public-safety is green, close out NA-0443
and restore exactly:

`NA-0444 -- QSL Key Lifecycle Secret Cleanup / Zeroization Evidence Policy Authorization Plan`

NA-0444 should define the evidence policy before any test or runtime lane is
authorized. F-0441-03 RNG failure behavior remains the next candidate after the
secret cleanup/zeroization evidence-policy path is bounded.
