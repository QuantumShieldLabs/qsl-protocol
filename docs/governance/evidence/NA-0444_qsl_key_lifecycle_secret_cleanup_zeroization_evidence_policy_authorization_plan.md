Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-08

# NA-0444 QSL Key Lifecycle Secret Cleanup / Zeroization Evidence Policy Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0444 consumes the NA-0441, NA-0442, and NA-0443 key-lifecycle findings and
authorizes an internal evidence policy for key-material cleanup and zeroization
expectations across qsc, refimpl, and qshield-cli surfaces.

Primary classification:

`KEY_LIFECYCLE_ZEROIZATION_POLICY_TEST_SCOPE_NEXT`

Selected successor:

`NA-0445 -- QSL qsc Key Lifecycle Secret Cleanup / Zeroization Test Scope Authorization Plan`

Reason: the policy can now distinguish qsc runtime, refimpl/reference, and
qshield-cli demo-local boundaries, but runtime cleanup implementation remains
too broad to authorize directly. The next safe step is a qsc test-scope
authorization lane that identifies exact future executable-test paths,
invariants, negative/no-mutation expectations, and claim boundaries before any
runtime or crypto code is changed.

This policy is internal governance evidence only. It does not implement
cleanup, add zeroization, change runtime behavior, change crypto behavior,
change dependencies, change Cargo manifests, change lockfiles, change
workflows, change executable tests, change fuzz targets, change vectors, change
formal models, change qsl-server, change qsl-attachments, change qshield
runtime, change public docs, change README, or change START_HERE.

## Live NA-0444 scope

Live `NEXT_ACTIONS.md` lists:

`NA-0444 -- QSL Key Lifecycle Secret Cleanup / Zeroization Evidence Policy Authorization Plan`

Status: READY.

Allowed NA-0444 mutation paths:

- `docs/governance/evidence/NA-0444_qsl_key_lifecycle_secret_cleanup_zeroization_evidence_policy_authorization_plan.md`
- `tests/NA-0444_qsl_key_lifecycle_secret_cleanup_zeroization_evidence_policy_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Read-only inspection included qwork proof files, the live NA-0444 queue block,
NA-0441 through NA-0443 evidence and testplans, D-0869 through D-0874,
TRACEABILITY, the rolling journal, the domain stewardship canon, qsc/refimpl/
qshield-cli source and tests, formal models, inputs, Cargo manifests and locks,
scripts, workflows, qsl-backup boundary proof, backup source-list proof, and
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
- NA-0441, NA-0442, and NA-0443 findings are consumed;
- a policy matrix defines evidence expectations for qsc, refimpl, qshield-cli,
  and supporting formal/test/input surfaces;
- qsc/refimpl/qshield-cli boundaries are explicit;
- one exact NA-0445 successor is selected;
- no implementation mutation occurs;
- root cargo audit remains green;
- nested qsc fuzz lock audit remains green;
- provider-error test and qsc adversarial script evidence remain healthy;
- formal checks remain green;
- public-safety is green before merge and after merge;
- exactly one READY item remains.

Stop conditions include missing or inconsistent qwork proof, PR #1156 not
merged, origin/main not equal to or descending from PR #1156 merge commit,
queue drift from READY NA-0444, D-0874 absence, D-0875 preexistence, audit
failures, unconsumable NA-0441/NA-0442/NA-0443 findings, unsafe policy
definition, unsafe successor selection, qsl-backup source-list regression,
forbidden mutation, backup or restore execution, public overclaim, or more than
one READY item.

## qwork proof-file verification

Codex read, but did not run, qwork proof files:

- `/srv/qbuild/work/NA-0444/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0444/.qwork/startup.qsl-protocol.json`

Required `.kv` markers passed:

- `startup_result=OK`
- `lane=NA-0444`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0444/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0444`
- `requested_lane_status=READY`

The JSON proof parsed successfully and mirrored the `.kv` proof for lane, repo,
path, HEAD, origin/main, ready count, top READY item, requested lane status,
and clean-state fields.

Initial live `HEAD` and `origin/main` matched the qwork proof at
`fd5d0583e836`. After `git fetch --all --prune`, `origin/main` still matched
the proof. PR #1156 was verified MERGED with merge commit `fd5d0583e836`.

Recorded timestamps:

- Local: `2026-06-08T08:15:33-05:00`
- UTC: `2026-06-08T13:15:33+00:00`

Proof root:

`/srv/qbuild/tmp/NA0444_key_lifecycle_zeroization_evidence_policy_20260608T131711Z`

## NA-0441 / NA-0442 / NA-0443 findings inheritance

NA-0441 selected:

`NONCE_KEY_RNG_EVIDENCE_GAPS_FOUND`

NA-0442 selected:

`NONCE_KEY_RNG_TRIAGE_SECRET_CLEANUP_SCOPE_NEXT`

NA-0443 selected:

`KEY_LIFECYCLE_SECRET_CLEANUP_EVIDENCE_POLICY_NEXT`

Inherited classifications:

- qsc runtime classification:
  `QSC_SECRET_CLEANUP_EVIDENCE_POLICY_NEEDED`.
- refimpl classification:
  `REFIMPL_SECRET_CLEANUP_SCOPE_INCLUDED` for evidence-policy scope only.
- qshield-cli classification:
  `QSHIELD_DEMO_KEY_MATERIAL_CLAIM_BOUNDARY_ONLY`.

Inherited findings:

- F-0441-02: selected zeroize/redaction evidence exists, but no comprehensive
  cleanup/wipe expectation exists for all pending/session/shared-secret
  material.
- F-0441-03: RNG failure behavior remains the next candidate after cleanup
  policy and test-scope authorization work.
- F-0441-05: formal/vector/fuzz/test evidence is supporting and bounded; it is
  not direct cleanup completion proof.
- F-0441-06: qshield-cli demo-local material remains claim-boundary/backlog.

Provider-error evidence remains background. `pq_decap_failed` no-mutation
evidence remains bounded to the existing deterministic test and qsc
adversarial script integration. `pq_encap_failed` remains defensive-branch
documentation only.

## Applicable Stewardship Review

### Crypto / Protocol Steward

Cleanup/zeroization policy is evidence governance, not runtime proof. The
policy distinguishes qsc runtime material, refimpl/reference material, and
qshield-cli demo-local material because each surface has different ownership,
persistence, and claim boundaries.

The policy requires exact material classes and direct source/test evidence
before any future internal "covered" status. It does not make a
secret-material-complete claim. It does not make a side-channel-free claim.
Side-channel and misuse-boundary caveats remain open unless a future exact lane
closes them with specific evidence.

### CI / Dependency / Release Health Steward

Root `cargo audit --deny warnings` passed. Nested qsc fuzz lock audit passed.
`rustls-webpki` is `v0.103.13`. Root pqcrypto inverse probes returned package
absence for `pqcrypto-mlkem`, `pqcrypto-traits`, and `pqcrypto-internals`.
The nested qsc fuzz lock pqcrypto residual scan returned zero matches.

The qsc provider-error no-mutation test passed and emitted the inherited
NA-0436 markers. `scripts/ci/qsc_adversarial.sh` still carries the NA-0439
marker and test command. Public-safety was required and green on current main.

Cargo audit green is dependency-health evidence only.
It is not vulnerability-free proof.
It is not public-readiness proof.
It is not production-readiness proof.
It is not external-review-complete proof.
It is not crypto-complete proof.
It is not side-channel-free proof.
It is not secret-material-complete proof.

### Public Claims / External Review Steward

This evidence policy is internal governance only.

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

Evidence gaps are called gaps, not completions. The policy is not a public
technical paper, not a public security statement, and not public release
readiness evidence.

### Product / Demo / Service Boundary Steward

qshield-cli evidence remains demo-local. qsc runtime, refimpl/reference, and
qshield-cli demo-local material must not be conflated. qshield-cli demo-local
evidence cannot be reused as qsc runtime assurance.

No qsl-server readiness claim is made. No qsl-attachments readiness claim is
made. No qshield runtime readiness claim is made. No website or public-service
readiness claim is made.

### Local Ops / Backup / Restore Steward

No backup, restore, sudo, qwork, qstart, or qresume was run. qsl-backup proof
remains boundary evidence only: the qsl-backup checksum matched the expected
boundary value and the Codex ops source-list inclusion count was exactly one.

No qsl-backup, backup status, backup plan, rollback subtree, backup tree,
systemd/timer/fstab, or local-ops mutation is authorized or performed.

Level 1 stewardship is active. Level 2 and Level 3 remain future-gated.
Stewards remain advisory only: no separate Directors, no independent READY
promotion, no independent merge authority, and Lead Director final authority is
preserved.

## Policy surface boundaries

### qsc runtime

Covered material classes:

- pending handshake material;
- session/shared-secret store material;
- vault/passphrase/runtime key material;
- logging/redaction surfaces;
- reject/no-mutation paths.

Evidence required before internal "covered" status:

- observed source evidence that names exact material, ownership, persistence,
  and cleanup/drop behavior;
- deterministic tests for expected cleanup or zeroization where the behavior is
  observable without weakening security;
- negative/tamper/reject tests for fail-closed paths that must not mutate
  pending/session/vault state;
- logging/redaction tests for any operator-visible or test-visible output;
- CI proof that qsc targeted tests, public-safety, dependency-health checks,
  and relevant formal checks remain green.

Forbidden wording:

- no secret-material-complete claim;
- no side-channel-free claim;
- no vulnerability-free claim;
- no perfect-crypto claim;
- no public-readiness or production-readiness claim.

Future scope split:

- test-scope authorization first for qsc;
- implementation scope only after exact test paths, source paths, material
  types, and invariants are authorized.

Likely future validation:

- qsc provider-error no-mutation test;
- qsc send/commit targeted tests;
- vault/session/pending cleanup tests if later authorized;
- public-safety;
- qsc-adversarial-smoke through CI;
- root and nested audit checks.

### refimpl

Covered material classes:

- provider private/secret material;
- qsp/suite2 session and ratchet material;
- existing `ZeroizeOnDrop` / zeroize evidence;
- reference/demo boundary material where applicable.

Evidence required before internal "covered" status:

- observed source evidence for each provider and session material type;
- direct zeroize/drop evidence for wrappers that claim cleanup behavior;
- deterministic unit tests or provider tests where feasible;
- explicit statement when evidence is reference-only rather than qsc runtime
  assurance.

Forbidden wording:

- no broad refimpl zeroization-complete claim;
- no qsc runtime assurance based only on refimpl evidence;
- no crypto-complete, side-channel-free, vulnerability-free, or perfect-crypto
  claim.

Future scope split:

- refimpl-specific policy/test scope should be separate from qsc runtime
  implementation;
- refimpl evidence can support qsc policy comparison but cannot replace qsc
  runtime proof.

Likely future validation:

- `cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768`;
- targeted zeroize/drop tests if later authorized;
- refimpl feature-gated CI checks if relevant.

### qshield-cli

Covered material classes:

- demo-local stored session/shared-secret related material;
- reestablish flows;
- filesystem store;
- demo relay records and local demo state.

Evidence required before internal "covered" status:

- observed source evidence for demo-local storage and reestablish material;
- filesystem-store behavior evidence;
- explicit demo-local boundary text;
- tests or docs proving no public/product/service claim expansion if later
  authorized.

Forbidden wording:

- no qsc runtime cleanup assurance based on qshield-cli evidence;
- no production-readiness claim;
- no public-readiness or public-internet-readiness claim;
- no service-readiness claim.

Future scope split:

- qshield-cli demo boundary documentation or demo-storage policy should be
  separate from qsc runtime cleanup/test work.

Likely future validation:

- qshield-cli demo tests if later authorized;
- scope guard proving no qsc runtime or public-service path mutation.

### formal/tests/inputs

Covered material classes:

- formal/model supporting evidence;
- adversarial and negative tests;
- input vectors and deterministic fixtures.

Evidence required before internal "covered" status:

- exact mapping from model/test/vector assertion to a material class and
  invariant;
- clear statement whether evidence is direct cleanup proof or supporting
  fail-closed/no-mutation/redaction evidence.

Forbidden wording:

- no cleanup completion claim from models, vectors, or tests without exact
  mapping;
- no formal-proof-complete claim;
- no exhaustive-testing claim.

Future scope split:

- formal/model or vector changes require separate exact authorization.

Likely future validation:

- `python3 formal/model_qsc_handshake_suite_id_bounded.py`;
- `python3 formal/run_model_checks.py`;
- targeted qsc/adversarial tests if later authorized.

## Evidence standard definition

| Category | What it proves | What it does not prove | Required artifacts | Claim boundary | Example future validation |
|---|---|---|---|---|---|
| Observed code evidence | Exact source paths, material ownership, and implemented cleanup/drop/encryption behavior | Runtime behavior across all paths or memory overwrite completion | Source path list, material type list, code references, reviewer notes | Internal source evidence only | `rg -n "Zeroize|zeroize|drop|clear|pending|session" qsl/qsl-client/qsc/src` |
| Deterministic test evidence | A named invariant holds under deterministic test conditions | Unmodeled paths, side channels, or exhaustive key-material handling | Exact test path, assertion, command output | Covered only for tested path | `cargo test -p qsc --locked --test <test-name> -- --test-threads=1` |
| Negative / tamper / reject test evidence | Reject path fails closed for malformed or tampered input | Cleanup for accept paths unless directly asserted | Tamper fixture, expected reject marker, state comparison | Reject path assurance only | `cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture` |
| No-mutation evidence | Rejected operation does not mutate protected pending/session/vault state | Secret wipe or memory zeroization unless directly observed | Pre/post state proof, negative test, markers | No-mutation only | `cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture` |
| Redaction / logging evidence | Runtime/test-visible output does not expose named secret material | In-memory cleanup or storage cleanup | Output assertions, marker scans, log redaction tests | Output-safety evidence only | `cargo test -p qsc --locked --test <redaction-test>` |
| Explicit zeroization evidence | A specific type/buffer invokes zeroize/drop behavior | Compiler, allocator, side-channel, or all-material guarantees | Source references, type traits, focused tests where feasible | Direct only for named type/buffer | `cargo test -p quantumshield_refimpl --features pqcrypto --locked --test <zeroize-test>` |
| Drop / cleanup evidence | A drop/clear/remove path is invoked for a named store or file | Memory overwrite unless explicitly shown | Source clear/drop path, filesystem/store assertions | Cleanup-path evidence only | `cargo test -p qsc --locked --test <pending-cleanup-test>` |
| Persistence / encrypted-at-rest evidence | Stored material is encrypted or removed according to invariant | In-memory zeroization or all runtime cleanup | File format tests, tamper tests, AAD/nonce evidence | Storage evidence only | `cargo test -p qsc --locked --test session_state_at_rest` |
| Demo-local boundary evidence | qshield-cli material is intentionally local/demo scoped | qsc runtime assurance or service readiness | qshield-cli source/test evidence, caveat text | Demo-local only | `cargo test -p qshield-cli --locked <if later authorized>` |
| Formal/model supporting evidence | A bounded model supports fail-closed/no-mutation reasoning | Direct cleanup/zeroization completion unless modeled exactly | Model path, invariant mapping, check output | Supporting-only unless exact mapping exists | `python3 formal/run_model_checks.py` |
| Governance caveat evidence | Claim boundary is explicit and traceable | Runtime behavior or cleanup proof | Evidence doc, DECISIONS entry, TRACEABILITY row, testplan | Internal governance only | `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main` |

Policy:

- No "secret-material-complete" claim is allowed unless all applicable buckets
  have direct evidence and a future Director explicitly authorizes that claim
  boundary.
- No "side-channel-free" claim may be inferred from zeroization or cleanup
  evidence.
- No "vulnerability-free" or "perfect-crypto" claim may be inferred from
  cleanup evidence.
- qshield-cli demo-local evidence must not be used as qsc runtime assurance.
- Cargo audit green is dependency-health evidence only.
- Evidence gaps must remain named as gaps until direct evidence closes them.

## Future implementation gate policy

Before any future implementation lane may be authorized, the directive must
define:

1. Exact surface and file path list.
2. Exact material type.
3. Desired invariant.
4. Required test evidence.
5. Required negative/no-mutation evidence if applicable.
6. Required logging/redaction evidence if applicable.
7. Required rollback/restore plan for local files if any local mutation is
   authorized.
8. Public-claim boundary.
9. Scope guard.
10. CI gates.
11. Stop condition if runtime or crypto behavior is broader than expected.

For future runtime cleanup/zeroization implementation:

- list exact runtime source paths;
- list exact tests to add or update;
- prove no unwanted behavior change;
- prove public-safety green;
- preserve fail-closed behavior;
- do not touch dependencies unless explicitly authorized;
- do not mutate Cargo manifests or lockfiles unless explicitly authorized.

For future documentation-only work:

- avoid public-claim expansion;
- distinguish evidence gaps from completion;
- keep qsc runtime, refimpl/reference, and qshield-cli demo-local boundaries
  separate.

For future qshield-cli work:

- preserve the demo-local caveat unless a future service/product lane changes
  scope;
- do not represent demo-local evidence as qsc runtime assurance;
- keep qsl-server, qsl-attachments, qshield runtime, website, and public docs
  out of scope unless explicitly authorized.

## Policy matrix

| Bucket | Material examples | Required evidence before "covered" internal status | Current known evidence from NA-0441/NA-0443 | Current gap | Future lane type | Forbidden claim | Goals affected |
|---|---|---|---|---|---|---|---|
| qsc pending KEM secret / pending handshake material | initiator pending KEM secret key, DH private key, responder confirm key, transcript hash, pending session snapshot | exact source paths, pending-store lifecycle map, accept/reject cleanup tests, no-mutation rejects, redaction checks | `hs_pending_store`, `hs_pending_clear`, provider-error no-mutation test, adversarial script marker | no comprehensive pending-material cleanup/zeroization standard | qsc test-scope authorization, then possible runtime implementation | no secret-material-complete or side-channel-free claim | G1, G2, G3, G4 |
| qsc session/shared secret / session store material | encrypted session snapshots, session-store encryption key, shared-secret derivations | source map, encrypted-at-rest proof, plaintext lifetime expectations, tamper/no-mutation tests, cleanup/drop tests where observable | session-state encrypted-at-rest evidence and tamper/no-mutation patterns | plaintext/shared-secret cleanup expectations not fully mapped | qsc test-scope authorization | no vulnerability-free, perfect-crypto, or crypto-complete claim | G1, G2, G4 |
| qsc vault/passphrase/runtime key material | passphrase buffers, vault key, secret map values, vault ciphertext | source map, zeroize/drop evidence, unlock/reject no-mutation tests, output redaction tests | selected zeroize calls; `VaultSession` drop zeroizes key and secret values; vault tests | not comprehensive across all secret values and runtime buffers | qsc test-scope authorization | no secret-material-complete or side-channel-free claim | G2, G4, G5 |
| qsc logging/redaction surfaces | markers, CLI/TUI output, logs, diagnostics, test output | direct output tests and scan evidence for named secret classes | redaction evidence exists in bounded areas | not mapped to every cleanup bucket | qsc test-scope authorization or doc-boundary lane | no leak-free or public-readiness claim | G4, G5 |
| qsc reject/no-mutation paths | provider-error rejects, tamper rejects, invalid unlock/provider paths | negative tests proving reject and state preservation | provider-error no-mutation test green; related fail-closed tests | cleanup-after-reject standard still incomplete | qsc test-scope authorization | no exhaustive-testing claim | G2, G3, G4 |
| refimpl private/secret material with zeroization evidence | X25519 private wrapper | exact trait/source evidence and focused tests | `Zeroize`/`ZeroizeOnDrop` evidence is narrow | limited to named wrapper | refimpl-specific scope later if prioritized | no broad zeroization-complete claim | G1, G4 |
| refimpl private/secret material without comprehensive evidence | ML-KEM keys/shared secrets, ML-DSA seed/signing key, qsp/suite2 ratchet/session material | provider/session source map, material type list, tests or caveats per type | provider code and `pqkem768` test evidence | broad provider/session cleanup expectations missing | refimpl-specific policy/test scope later | no qsc runtime assurance from refimpl evidence | G1, G2, G4 |
| qshield-cli demo-local stored material | `pq_init_ss_hex`, `dh_init_hex`, session/reestablish store material | demo-local source map, filesystem-store evidence, explicit caveat text | qshield-cli demo material identified as claim-boundary/backlog | demo storage policy not selected for runtime cleanup | qshield-cli demo boundary documentation or policy later | no production/public/service-readiness claim | G3, G4, G5 |
| formal/model supporting evidence | SCKA, negotiation, qsc suite-id model state and no-mutation flags | exact invariant mapping to material class if used as direct evidence | formal checks support fail-closed/no-mutation discipline | no direct cleanup/zeroization model | formal/model authorization only after exact scope | no formal-proof-complete claim | G2, G4 |
| test/vector/adversarial supporting evidence | provider-error test, qsc adversarial smoke, suite2 vectors, inputs | exact mapping from test/vector to cleanup invariant | bounded test/adversarial/vector evidence exists | no unified cleanup standard or coverage map | qsc test-scope authorization first | no exhaustive-testing or cleanup-complete claim | G4, G5 |

## Successor strategy

Options considered:

- Option 1 - qsc runtime test scope authorization: selected. The policy now
  identifies qsc material classes and evidence standards strongly enough to
  authorize the next governance lane to choose exact future qsc tests before
  any runtime implementation.
- Option 2 - qsc runtime implementation scope authorization: rejected for now.
  Runtime implementation should wait until exact future test paths, invariants,
  and no-mutation/redaction requirements are authorized.
- Option 3 - refimpl-specific evidence policy / scope authorization: rejected
  for first position. refimpl remains important, but qsc runtime material is
  the higher-priority inherited runtime scope.
- Option 4 - qshield-cli demo boundary documentation: rejected for first
  position. qshield-cli remains demo-local claim-boundary evidence and must not
  outrank qsc runtime test-scope work.
- Option 5 - RNG failure behavior scope authorization: rejected for first
  position. F-0441-03 remains next candidate after cleanup/zeroization test
  scope is bounded.
- Option 6 - split-scope triage: rejected for first position. NA-0444 has now
  separated qsc/refimpl/qshield-cli enough to proceed with qsc test-scope
  authorization while preserving refimpl and qshield-cli residuals.

## Authorization decision

Selected classification:

`KEY_LIFECYCLE_ZEROIZATION_POLICY_TEST_SCOPE_NEXT`

Decision rationale:

- NA-0441/NA-0442/NA-0443 findings are consumable.
- The policy matrix defines qsc/refimpl/qshield-cli boundaries.
- qsc runtime material has the highest-priority cleanup evidence gap.
- A test-scope authorization lane is safer than runtime implementation.
- No implementation mutation is authorized by NA-0444.
- No runtime/crypto/dependency/Cargo/lockfile/workflow/test/fuzz/vector/formal
  mutation is authorized by NA-0444.
- No public-claim expansion is authorized.
- Exactly one READY successor remains mandatory.

## Successor selection

Selected successor:

`NA-0445 -- QSL qsc Key Lifecycle Secret Cleanup / Zeroization Test Scope Authorization Plan`

NA-0445 must not implement cleanup or zeroization. It must consume NA-0441
through NA-0444, identify exact future qsc test paths if implementation is
later authorized, define no-runtime/no-crypto/no-dependency/no-public-claim
boundaries, and select the next exact successor or no-action rationale.

## Future path/scope bundle

Future NA-0445 allowed mutation paths:

- `docs/governance/evidence/NA-0445_qsl_qsc_key_lifecycle_secret_cleanup_zeroization_test_scope_authorization_plan.md`
- `tests/NA-0445_qsl_qsc_key_lifecycle_secret_cleanup_zeroization_test_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0445 read-only inspection paths:

- `qsl/qsl-client/qsc/src/`
- `qsl/qsl-client/qsc/tests/`
- `formal/`
- `inputs/`
- `docs/governance/evidence/`
- `qsl/qsl-client/qsc/fuzz/`
- relevant scripts/workflows read-only.

Future forbidden unless exact scope authorizes:

- runtime or crypto implementation changes;
- dependency changes;
- Cargo manifest or lockfile changes;
- workflow changes;
- executable test source changes;
- fuzz target source changes;
- vector changes;
- formal model changes;
- public docs, README, START_HERE, or website changes;
- qsl-server or qsl-attachments changes;
- backup, restore, qsl-backup, backup status, backup plan, rollback, or backup
  tree changes;
- public claims.

## Future validation/marker plan

Common NA-0445 markers:

- `NA0445_ZEROIZATION_POLICY_CONSUMED_OK`
- `NA0445_NEXT_SCOPE_SELECTED_OK`
- `NA0445_NO_RUNTIME_CHANGE_OK`
- `NA0445_NO_DEPENDENCY_CHANGE_OK`
- `NA0445_NO_WORKFLOW_CHANGE_OK`
- `NA0445_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0445_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0445_NO_SECRET_MATERIAL_COMPLETE_CLAIM_OK`
- `NA0445_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0445_ONE_READY_INVARIANT_OK`
- `NA0445_QSC_TEST_SCOPE_AUTHORIZATION_OK`

Expected NA-0445 validation:

- qwork proof-file verification without running qwork/qstart/qresume;
- queue and decision proof;
- scope guard;
- link-check;
- leak-scan;
- overclaim scan;
- PR body preflight;
- goal-lint;
- adversarial script syntax checks;
- qsc provider-error no-mutation test;
- qsc send_commit test;
- refimpl pqkem768 test;
- root cargo audit;
- nested qsc fuzz lock audit;
- cargo tree dependency probes;
- cargo fmt;
- formal model checks;
- public-safety before and after merge.

## Public claim/external review/website boundary

This evidence policy is internal governance evidence only.

This evidence policy is not production readiness.
This evidence policy is not public-internet readiness.
This evidence policy is not public readiness.
This evidence policy is not crypto-complete proof.
This evidence policy is not side-channel-free proof.
This evidence policy is not secret-material-complete proof.
This evidence policy is not bug-free proof.
This evidence policy is not vulnerability-free proof.
This evidence policy is not perfect-crypto proof.
This evidence policy is not a public technical paper.

No README, START_HERE, public docs, or website update is authorized or made.
No public-readiness or public-security claim is authorized or made.
Cargo audit green is dependency-health evidence only.
Evidence gaps must be called gaps, not completions.

## Rejected alternatives

Direct qsc runtime cleanup implementation:

- Rejected because exact future tests, source paths, material classes, and
  no-mutation/redaction expectations must be authorized before runtime
  mutation.

Bundled qsc/refimpl/qshield-cli implementation:

- Rejected because the surfaces have different ownership and claim boundaries.
  Bundling them would risk behavior drift and public overclaim.

Refimpl-first scope:

- Rejected for first position because qsc runtime pending/session/vault
  material is the higher-priority inherited runtime evidence gap.

qshield-cli-first documentation:

- Rejected for first position because qshield-cli remains demo-local and cannot
  be used as qsc runtime assurance.

RNG failure behavior next:

- Rejected for first position only. F-0441-03 remains a valid future candidate
  after qsc cleanup/zeroization test-scope authorization is bounded.

No action:

- Rejected because F-0441-02 remains a meaningful medium evidence gap and the
  policy now identifies a safe next governance step.

## Backup-impact statement

No backup was run. No restore was run. No sudo was run. qsl-backup was not
mutated. Backup status files, backup plan files, rollback subtree paths,
systemd/timer/fstab state, and `/backup/qsl` were not mutated.

qsl-backup checksum proof remained:

`e9ecff3d22ed`

The Codex ops source-list inclusion count was exactly one. This is boundary
evidence only.
It is not off-host-backup-complete proof.
It is not disaster-recovery proof.
It is not restore-proven proof.
It is not backup-complete proof.

## Next recommendation

Proceed to the selected NA-0445 qsc test-scope authorization lane after the
NA-0444 evidence PR merges and post-merge public-safety is green. NA-0445
should identify exact future qsc executable-test paths and invariants before
any runtime cleanup or zeroization implementation is considered.
