Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-08

# NA-0442 QSL Nonce / Key / RNG Lifecycle Findings Triage Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0442 consumes the NA-0441 nonce/key/RNG lifecycle findings and selects the
highest-priority exact successor without implementing runtime or crypto
changes.

Primary classification:

`NONCE_KEY_RNG_TRIAGE_SECRET_CLEANUP_SCOPE_NEXT`

Selected successor:

`NA-0443 -- QSL Key Lifecycle Secret Cleanup / Zeroization Scope Authorization Plan`

Reason: F-0441-02 and F-0441-03 are both medium evidence gaps, but
secret-material cleanup and zeroization expectations have the higher immediate
claim-boundary and key-lifecycle leverage. F-0441-02 also needs exact path and
scope discovery before any implementation lane would be truthful. RNG failure
modeling remains the next candidate, not bundled into the selected successor.

No implementation mutation is authorized by NA-0442. No runtime, crypto,
dependency, Cargo, lockfile, workflow, executable-test, fuzz-target, vector,
formal-model, qsl-server, qsl-attachments, qshield runtime, website, public-doc,
README, START_HERE, qwork/qstart/qresume/qshell, backup, restore, qsl-backup,
backup status, backup plan, rollback, or local-ops mutation is authorized or
performed.

## Live NA-0442 scope

Live `NEXT_ACTIONS.md` lists:

`NA-0442 -- QSL Nonce / Key / RNG Lifecycle Findings Triage Authorization Plan`

Status: READY.

Allowed NA-0442 mutation paths:

- `docs/governance/evidence/NA-0442_qsl_nonce_key_rng_lifecycle_findings_triage_authorization_plan.md`
- `tests/NA-0442_qsl_nonce_key_rng_lifecycle_findings_triage_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Read-only inspection included the qwork proof files, the NA-0441 evidence and
testplans, D-0869 and D-0870, TRACEABILITY, the rolling journal, the domain
stewardship canon, qsc/refimpl/qshield/formal/input evidence references,
Cargo manifests and locks, scripts, workflows, qsl-backup boundary proof, and
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
- NA-0441 findings F-0441-01 through F-0441-06 are consumed;
- F-0441-02 and F-0441-03 are triaged explicitly;
- a single exact NA-0443 successor is selected;
- no implementation mutation occurs;
- root cargo audit remains green;
- nested qsc fuzz lock audit remains green;
- provider-error test and qsc adversarial script evidence remain healthy;
- public-safety is green before merge and after merge;
- exactly one READY item remains.

Stop conditions include missing or inconsistent qwork proof, PR #1152 not
merged, queue drift from READY NA-0442, D-0870 absence, D-0871 preexistence,
audit failures, unconsumable NA-0441 findings, unsafe triage, unsafe successor
selection, qsl-backup source-list regression, forbidden mutation, backup or
restore execution, public overclaim, or more than one READY item.

## qwork proof-file verification

Codex read, but did not run, qwork proof files:

- `/srv/qbuild/work/NA-0442/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0442/.qwork/startup.qsl-protocol.json`

Required `.kv` markers passed:

- `startup_result=OK`
- `lane=NA-0442`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0442/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0442`
- `requested_lane_status=READY`

The JSON proof parsed successfully and mirrored the `.kv` proof for lane, repo,
path, HEAD, origin/main, ready count, top READY item, requested lane status,
and clean-state fields.

Initial live `HEAD` and `origin/main` matched the qwork proof at
`a4d7616048b6`. After `git fetch --all --prune`, `origin/main` still matched
the proof and equals PR #1152 merge commit `a4d7616048b6`. PR #1152 was
verified MERGED.

Recorded timestamps:

- Local: `2026-06-08T05:28:50-05:00`
- UTC: `2026-06-08T10:28:50+00:00`

Proof root:

`/srv/qbuild/tmp/NA0442_nonce_key_rng_findings_triage_20260608T103028Z`

## NA-0441 findings inheritance

NA-0442 inherits NA-0441 classification:

`NONCE_KEY_RNG_EVIDENCE_GAPS_FOUND`

Inherited findings:

- F-0441-01: nonce lifecycle evidence exists but is spread across
  implementation and tests rather than captured as one invariant.
- F-0441-02: selected zeroize/redaction evidence exists, but no comprehensive
  cleanup/wipe expectation exists for all pending/session/shared-secret
  material.
- F-0441-03: inspected cryptographic paths use OS/provider randomness, but RNG
  failure behavior is not modeled/tested as a distinct invariant.
- F-0441-04: transcript/key schedule/session binding evidence is covered as
  supporting evidence, with broader KEM/signature/transcript work still future
  scope.
- F-0441-05: formal/vector/fuzz/test evidence is strong in bounded areas but
  not comprehensive for lifecycle cleanup and RNG failure.
- F-0441-06: qshield-cli stores demo-local key material for reestablish flows;
  this is claim-boundary evidence and a backlog candidate.

No active blocker and no high runtime risk was identified by NA-0441. The
triage objective is to select the highest-priority exact follow-up while
preserving provider-error caveats as background, no public-claim expansion,
and no current implementation mutation.

Top candidate findings:

- F-0441-02 secret cleanup/zeroization evidence gap.
- F-0441-03 RNG-failure modeling/test evidence gap.

Stewardship review is required at Level 1. Level 2 and Level 3 remain
future-gated. Forbidden mutation scope remains the same governance-only
boundary listed above.

## Applicable Stewardship Review

### Crypto / Protocol Steward

F-0441-02 is a secret-material lifecycle evidence gap, not a proven active
runtime exposure. It should be scoped before any runtime or crypto remediation
because implicated surfaces include qsc pending/session/shared-secret material,
qsc vault/protocol_state, selected refimpl `ZeroizeOnDrop` evidence, and
qshield-cli demo-local shared-secret storage.

F-0441-03 is an RNG failure evidence gap, not a finding that inspected runtime
cryptographic paths use weak randomness. It should remain a next candidate
because qsc and refimpl use OS/provider randomness where inspected, but
fail-closed RNG failure behavior is not separately modeled or tested.

Secret-material caveats remain open. Side-channel caveats remain open. No
secret-material-complete claim is made. No side-channel-free claim is made.

### CI / Dependency / Release Health Steward

Root `cargo audit --deny warnings` passed. Nested qsc fuzz lock audit passed.
`rustls-webpki` is `v0.103.13`. Root pqcrypto residual inverse probes produced
expected package-absence output. Nested qsc fuzz lock pqcrypto residual scan
returned zero matches.

The qsc provider-error no-mutation test passed and emitted inherited NA-0436
markers. `scripts/ci/qsc_adversarial.sh` still contains the NA-0439 marker and
test command. Public-safety was required and green on current main.

Cargo audit green is dependency-health evidence only. It is not a
vulnerability-free claim and not public-readiness proof.

### Public Claims / External Review Steward

This triage is internal governance evidence only:

- no crypto-complete claim;
- no side-channel-free claim;
- no vulnerability-free claim;
- no bug-free claim;
- no perfect-crypto claim;
- no public-readiness claim;
- no production-readiness claim;
- no public-internet-readiness claim;
- no external-review-complete claim.

Evidence gaps are called gaps, not completions. F-0441-02 and F-0441-03 must
not be converted into public assurance statements.

### Product / Demo / Service Boundary Steward

qshield-cli evidence remains demo-local and claim-boundary scoped. It is not a
qsl-server, qsl-attachments, qshield runtime, website, public-service,
production-readiness claim, and it is not a public-internet-readiness claim.
qshield-cli stored demo material should stay separate from qsc runtime key
cleanup triage unless a future exact lane authorizes a demo-storage policy
review.

### Local Ops / Backup / Restore Steward

No backup, restore, sudo, qwork, qstart, or qresume was run. qsl-backup proof
remains boundary evidence only: the qsl-backup checksum matched the expected
boundary value and the Codex ops source-list inclusion count was exactly one.
No qsl-backup, backup status, backup plan, rollback subtree, or backup tree
mutation is authorized or performed.

Level 1 stewardship is active. Level 2 and Level 3 remain future-gated.
Stewards remain advisory only: no separate Directors, no independent READY
promotion, no independent merge authority, and Lead Director final authority is
preserved.

## Findings consumption and normalization

| Finding ID | Restated finding | Evidence | Classification | Disposition | Exact future mutable paths obvious? | Public-claim implication |
|---|---|---|---|---|---|---|
| F-0441-01 | Nonce/session-id generation and nonce length evidence exists, but no one cross-surface lifecycle invariant captures uniqueness, replay, and storage together. | qsc handshake/vault/protocol_state, refimpl Suite-2 ratchet, qsc nonce/replay tests | LOW / EVIDENCE_INCOMPLETE | BACKLOG | No. Another authorization lane would be needed before tests or policy changes. | No nonce-complete or crypto-complete claim. |
| F-0441-02 | Comprehensive cleanup/wipe expectations for KEM, PQ signature, shared secret, pending, and session snapshot material are incomplete. | qsc identity/vault/session state, refimpl `StdCrypto`, selected `ZeroizeOnDrop`, qsc vault/session tests | MEDIUM / EVIDENCE_INCOMPLETE | IMMEDIATE_SUCCESSOR | No. Scope authorization is needed to inventory exact paths and choose policy/test/runtime shape. | No secret-material-complete, side-channel-free, perfect-crypto, or vulnerability-free claim. |
| F-0441-03 | Runtime cryptographic randomness uses OS/provider sources where inspected, but RNG failure behavior is not directly modeled or injectable. | qsc `OsRng` sites, refimpl `StdCrypto`, qsc seed-fallback tests, fixed RNG test contexts | MEDIUM / EVIDENCE_INCOMPLETE | NEXT_CANDIDATE | No. Scope authorization is needed before any RNG seam, tests, or formal model extension. | No RNG-complete or crypto-complete claim. |
| F-0441-04 | Transcript/key schedule/session binding evidence is covered in bounded qsc/refimpl areas, with broader KEM/signature/transcript work still future scope. | qsc handshake KDF/transcript code, qsc suite-id tests/vectors, refimpl Suite-2 code/tests | COVERED / SUPPORTING_ONLY | ACCEPTED_NO_ACTION | None for NA-0442. Future domain audit can own broader work. | No transcript-complete or external-review-complete claim. |
| F-0441-05 | Formal/vector/fuzz/test evidence is strong in bounded areas but not comprehensive for lifecycle cleanup and RNG failure. | formal models, inputs, recent governance evidence | EVIDENCE_GAP | WATCH | No. Coverage changes would need future exact authorization. | Formal/model checks remain bounded supporting evidence only. |
| F-0441-06 | qshield-cli intentionally persists deterministic demo establishment material for reestablish flows. | `apps/qshield-cli/src/store.rs`, establish commands, qshield demo tests | CLAIM_BOUNDARY_ONLY / BACKLOG_CANDIDATE | CLAIM_BOUNDARY_ONLY | No. qshield runtime/demo storage mutation is not authorized here. | No production-readiness, public-readiness, or crypto-complete claim. |

## Secret cleanup / zeroization triage

F-0441-02 is primarily a missing evidence policy and scope-selection gap. It
may later reveal missing tests or runtime behavior, but NA-0442 does not
authorize that conclusion.

Concrete surfaces implicated by NA-0441:

- qsc pending/session/shared-secret material;
- qsc vault/protocol_state;
- selected refimpl `ZeroizeOnDrop` evidence;
- qshield-cli demo-local shared-secret storage.

Exact future mutable paths are not yet obvious enough for implementation. A
scope authorization lane is appropriate now because any direct implementation
could require runtime/crypto code changes and tests. Public claims are at risk
if the current partial zeroize/redaction evidence is described as
secret-material complete; NA-0442 makes no secret-material-complete claim.
qshield-cli demo-local material should be separated from qsc runtime key
cleanup triage.

Classification:

`SECRET_CLEANUP_SCOPE_AUTHORIZATION_NEEDED`

## RNG failure triage

F-0441-03 is about RNG failure behavior, tests, and possible formal modeling,
not about replacing the inspected RNG source choice. Runtime qsc/refimpl paths
use OS/provider randomness where inspected; deterministic/demo RNG or seed
surfaces are test, vector, or demo scoped.

Concrete surfaces implicated by NA-0441:

- qsc session ID generation;
- qsc vault/session nonce generation;
- refimpl provider randomness;
- deterministic/demo RNG or seed surfaces;
- formal/model gaps.

Exact future mutable paths are not yet obvious enough for implementation. A
future lane may need a provider or RNG injection seam, tests, or formal model
extension, each of which requires exact authorization. Public claims are at
risk if unmodeled RNG failure is treated as closed.

Classification:

`RNG_FAILURE_SCOPE_AUTHORIZATION_NEEDED`

## Nonce / transcript / demo boundary triage

F-0441-01 nonce lifecycle evidence does not need the immediate successor ahead
of key cleanup and RNG failure. It is classified:

`NONCE_LIFECYCLE_BACKLOG`

F-0441-04 transcript/key schedule/session binding moves naturally into the next
KEM/signature/transcript audit domain when that domain is authorized. It is
classified:

`TRANSCRIPT_BINDING_NEXT_DOMAIN`

F-0441-06 qshield-cli demo-local key material is a claim-boundary issue and
backlog candidate. It should not be folded into qsc runtime secret cleanup
unless a future exact lane authorizes that scope. It is classified:

`DEMO_KEY_MATERIAL_CLAIM_BOUNDARY_ONLY`

No active blocker and no high runtime risk is present from the NA-0441 read-only
evidence.

## Prioritization matrix

| Finding ID | Domain | Original classification | Triage disposition | Recommended action | Future exact paths if known | Future lane type | Risk | Public-claim implication | Goals affected |
|---|---|---|---|---|---|---|---|---|---|
| F-0441-02 | key lifecycle cleanup / zeroization | MEDIUM / EVIDENCE_INCOMPLETE | IMMEDIATE_SUCCESSOR | Select NA-0443 key lifecycle secret cleanup / zeroization scope authorization. | NA-0443 governance paths known; runtime/test paths not yet selected. | Scope authorization | Medium evidence gap, no active exposure proven | No secret-material-complete, side-channel-free, perfect-crypto, or vulnerability-free claim. | G1, G2, G3, G4, G5 |
| F-0441-03 | RNG failure behavior | MEDIUM / EVIDENCE_INCOMPLETE | NEXT_CANDIDATE | Keep as next or parallel future scope lane after secret cleanup scope selection. | None yet; possible qsc/refimpl/formal/test paths require authorization. | Scope authorization | Medium evidence gap, no weak RNG source finding | No RNG-complete or crypto-complete claim. | G1, G2, G4 |
| F-0441-01 | nonce lifecycle | LOW / EVIDENCE_INCOMPLETE | BACKLOG | Defer behind key/RNG medium gaps. | None yet. | Scope authorization if promoted later | Low auditability gap | No nonce-complete or crypto-complete claim. | G1, G2, G4 |
| F-0441-04 | transcript/key schedule/session binding | COVERED / SUPPORTING_ONLY | ACCEPTED_NO_ACTION / next domain | Preserve as supporting evidence; route broader work to KEM/signature/transcript audit domain. | None for NA-0442. | Future audit domain | Supporting-only residual | No transcript-complete or external-review-complete claim. | G1, G2, G4 |
| F-0441-05 | formal/vector/fuzz/test coverage | EVIDENCE_GAP | WATCH | Track as coverage residual tied to F-0441-02 and F-0441-03. | None yet. | Coverage/model authorization if promoted later | Evidence overclaim risk | Formal/model checks remain bounded supporting evidence. | G2, G4, G5 |
| F-0441-06 | qshield demo key material | CLAIM_BOUNDARY_ONLY / BACKLOG_CANDIDATE | CLAIM_BOUNDARY_ONLY | Keep demo-local boundary explicit; do not treat as runtime qsc cleanup. | None; qshield runtime mutation not authorized. | Backlog or claim-boundary authorization | Public/demo confusion risk | No production-readiness, public-readiness, or service-readiness claim. | G3, G4, G5 |

Prioritization rules applied:

- No blocker/high active runtime risk outranks the medium gaps.
- F-0441-02 outranks F-0441-03 because secret cleanup/zeroization is directly
  tied to key material lifecycle and public-claim risk.
- Exact path discovery is required, so the selected successor is scope
  authorization, not implementation.
- Secret cleanup and RNG failure are not bundled into one implementation lane.
- qshield-cli demo-local material remains separate from qsc runtime key cleanup.
- Public-claim caveats are preserved.

## Authorization decision

Primary classification:

`NONCE_KEY_RNG_TRIAGE_SECRET_CLEANUP_SCOPE_NEXT`

NA-0442 decides:

- NA-0441 findings F-0441-01 through F-0441-06 are consumed.
- F-0441-02 and F-0441-03 are triaged as the highest-priority medium evidence
  gaps.
- F-0441-02 is selected as the immediate successor because secret cleanup and
  zeroization expectations need exact scope discovery before implementation.
- No implementation mutation is authorized in NA-0442.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test,
  fuzz target, vector, formal model, service, public-surface, backup, restore,
  qwork, qstart, qresume, or qsl-backup mutation is authorized or performed.
- No public-claim expansion is authorized.
- Exactly one READY successor remains mandatory.

## Successor selection

Selected exact NA-0443 successor:

`NA-0443 -- QSL Key Lifecycle Secret Cleanup / Zeroization Scope Authorization Plan`

Do not implement NA-0443 in NA-0442.

## Future path/scope bundle

Future allowed NA-0443 mutation paths:

- `docs/governance/evidence/NA-0443_qsl_key_lifecycle_secret_cleanup_zeroization_scope_authorization_plan.md`
- `tests/NA-0443_qsl_key_lifecycle_secret_cleanup_zeroization_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0443 may inspect read-only:

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

- runtime or crypto implementation changes;
- dependency changes;
- Cargo or lockfile changes;
- workflow changes;
- test source changes;
- fuzz target source changes;
- vector changes;
- formal model changes;
- public docs, README, START_HERE, or website changes;
- qsl-server or qsl-attachments changes;
- qshield runtime changes;
- backup/restore/qsl-backup changes;
- public claim expansion.

## Future validation/marker plan

Common NA-0443 markers:

- `NA0443_LIFECYCLE_TRIAGE_CONSUMED_OK`
- `NA0443_NEXT_SCOPE_SELECTED_OK`
- `NA0443_NO_RUNTIME_CHANGE_OK`
- `NA0443_NO_DEPENDENCY_CHANGE_OK`
- `NA0443_NO_WORKFLOW_CHANGE_OK`
- `NA0443_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0443_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0443_NO_SECRET_MATERIAL_COMPLETE_CLAIM_OK`
- `NA0443_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0443_ONE_READY_INVARIANT_OK`
- `NA0443_KEY_LIFECYCLE_SECRET_CLEANUP_SCOPE_OK`

Expected NA-0443 validation:

- consume NA-0441 and NA-0442 key lifecycle findings;
- select exact future scope from evidence;
- preserve no implementation mutation;
- keep root and nested dependency-health checks green;
- keep public-safety green before and after merge;
- keep exactly one READY item.

## Public claim/external review/website boundary

NA-0442 findings triage is internal governance evidence only.

- No production-readiness claim is made.
- No public-internet-readiness claim is made.
- No public-readiness claim is made.
- No external-review-complete claim is made.
- No crypto-complete claim is made.
- No side-channel-free claim is made.
- No bug-free claim is made.
- No vulnerability-free claim is made.
- No perfect-crypto claim is made.
- No public technical paper content is created.
- No README, START_HERE, public docs, or website update is made.
- Cargo audit green is dependency-health evidence only.
- No secret-material lifecycle finding is turned into a public assurance claim.
- Evidence gaps are called gaps, not completions.

## Rejected alternatives

Direct implementation now:

- Rejected because exact implementation scope for cleanup/zeroization and RNG
  failure behavior is not yet selected. Direct implementation could require
  runtime, crypto, or test changes outside NA-0442 scope.

RNG failure as the immediate successor:

- Rejected for first position because F-0441-02 has higher key-lifecycle and
  public-claim leverage. RNG failure remains the next candidate.

Nonce lifecycle as the immediate successor:

- Rejected because F-0441-01 is a low auditability gap and does not outrank
  medium key/RNG evidence gaps.

Documentation-only next:

- Rejected because the secret cleanup and RNG failure gaps need scope discovery,
  not only wording.

Next KEM/signature/transcript audit domain now:

- Rejected because F-0441-04 is supporting-only background and does not outrank
  the key lifecycle cleanup evidence gap.

Bundled secret cleanup plus RNG implementation:

- Rejected because the likely paths, validation strategy, and risk profile are
  distinct. Bundling would raise scope risk.

## Backup-impact statement

NA-0442 did not run backup, restore, sudo, qwork, qstart, or qresume. NA-0442
did not mutate qsl-backup, backup status files, backup plan files, rollback
subtree paths, timers, fstab, source lists, backup scripts, or backup tree
paths. Backup evidence remains boundary evidence only, with no off-host backup
claim, no restore-proven claim, no disaster-recovery claim, no backup-complete
claim, and no source-of-truth index claim.

## Next recommendation

Close NA-0442 only after the evidence PR merges and post-merge public-safety is
green. Restore exactly:

`NA-0443 -- QSL Key Lifecycle Secret Cleanup / Zeroization Scope Authorization Plan`

NA-0443 should authorize the exact future scope for investigating and, if
justified later, improving key-material cleanup and zeroization evidence across
qsc/refimpl/qshield-cli surfaces. NA-0443 should not implement runtime,
crypto, dependency, Cargo, lockfile, workflow, test, fuzz, vector, formal,
public, service, backup, restore, or qsl-backup changes unless a later exact
directive authorizes them.
