Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-09

# NA-0448 QSL qsc RNG Failure Test Seam Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0448 consumes F-0441-03 and the NA-0447 classification
`RNG_FAILURE_SCOPE_QSC_TEST_SEAM_NEXT`. It reviews qsc RNG-dependent surfaces
and authorizes a narrow future qsc implementation lane for a test-only cfg seam
that can force bounded RNG failure evidence without changing normal production
build behavior.

Selected primary classification:

`QSC_RNG_TEST_SEAM_IMPLEMENTATION_READY`

Selected successor:

`NA-0449 -- QSL qsc RNG Failure Test Seam Implementation Harness`

NA-0448 does not implement tests, test seams, runtime behavior, crypto
behavior, dependency changes, Cargo manifest changes, lockfile changes,
workflow changes, fuzz targets, vectors, formal models, qsl-server,
qsl-attachments, qshield runtime, qshield-cli, website, public docs, README,
START_HERE, backup/restore/local-ops state, or qwork tooling.

## Live NA-0448 scope

Live `NEXT_ACTIONS.md` lists:

`NA-0448 -- QSL qsc RNG Failure Test Seam Authorization Plan`

Status: READY.

Allowed mutation paths for this evidence PR:

- `docs/governance/evidence/NA-0448_qsl_qsc_rng_failure_test_seam_authorization_plan.md`
- `tests/NA-0448_qsl_qsc_rng_failure_test_seam_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden mutation scope includes runtime code, crypto code, dependency
metadata, Cargo manifests, lockfiles, workflows, executable tests, fuzz target
source, vectors, formal model files, qsl-server, qsl-attachments, qshield
runtime, qshield-cli, website, public docs, README, START_HERE,
qwork/qstart/qresume/qshell, qsl-backup, backup status files, backup plan
files, rollback subtree paths, `/backup/qsl`, public technical paper content,
branch protection, and public claim surfaces.

Acceptance criteria:

- qwork proof files are verified without running qwork, qstart, or qresume;
- F-0441-03 and NA-0447 are consumed;
- qsc RNG-dependent surfaces are classified;
- existing APIs are judged for RNG failure forcing;
- test-only seam strategy is accepted, rejected, or refined from evidence;
- an exact NA-0449 successor is selected;
- no implementation mutation occurs in NA-0448;
- no public claim expansion occurs;
- root and nested fuzz lock cargo audits remain green;
- inherited qsc key lifecycle and provider-error tests remain green;
- public-safety remains green;
- exactly one READY item remains mandatory.

Stop conditions:

- qwork proof files are missing, malformed, stale, or inconsistent with live
  repo state;
- PR #1164 is not merged at the expected merge commit;
- queue state is not READY NA-0448 with READY_COUNT 1;
- D-0882 is absent or D-0883 exists before patching;
- F-0441-03 / NA-0447 cannot be consumed truthfully;
- qsc RNG failure test seam strategy cannot be safely classified;
- exact successor cannot be selected safely;
- any forbidden path would need mutation;
- public-safety, dependency health, inherited qsc tests, or backup-boundary
  proof fail conclusively.

## qwork proof-file verification

Codex read, but did not run, qwork proof files:

- `/srv/qbuild/work/NA-0448/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0448/.qwork/startup.qsl-protocol.json`

Required `.kv` markers passed:

- `startup_result=OK`
- `lane=NA-0448`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0448/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0448`
- `requested_lane_status=READY`

The JSON proof parsed successfully and mirrored the `.kv` proof for lane, repo,
path, HEAD, origin/main, clean-state fields, READY count, top READY item, and
requested lane status.

After `git fetch --all --prune`, live `HEAD` and `origin/main` still matched
the qwork proof at `a5f93b100d00`. PR #1164 was verified MERGED with merge
commit `a5f93b100d00`.

Recorded timestamps:

- Local: `2026-06-08T22:01:37-05:00`
- UTC: `2026-06-09T03:01:37+00:00`

Proof root:

`/srv/qbuild/tmp/NA0448_qsc_rng_failure_test_seam_auth_20260609T030252Z`

## F-0441-03 / NA-0447 inheritance

F-0441-03 records that inspected runtime cryptographic randomness sites use
OS/provider randomness, while RNG failure behavior is not directly modeled,
not directly injectable, and not executable evidence.

NA-0447 selected:

`RNG_FAILURE_SCOPE_QSC_TEST_SEAM_NEXT`

Inherited evidence:

- qsc RNG failure cannot currently be forced through existing public APIs.
- qsc session ID generation, vault/session nonce generation, salts, route
  tokens, attachment IDs, and provider-dependent generation are RNG-related
  surfaces.
- refimpl RNG failure remains provider-boundary backlog.
- qshield-cli RNG surfaces remain demo/claim-boundary backlog.
- formal/model/fuzz/vector RNG failure evidence remains supporting or backlog.
- NA-0447 selected NA-0448 as authorization-only.

NA-0448's objective is to decide whether exact future qsc test-seam
implementation can proceed without changing normal production build semantics
or making broad RNG-failure claims.

## Applicable Stewardship Review

Level 1 stewardship is active in this evidence lane. Level 2 and Level 3 remain
future-gated. Stewards remain advisory only: no separate Directors, no
independent READY promotion, no independent merge authority, and Lead Director
final authority is preserved.

Crypto / Protocol Steward:

- A qsc test seam must be bounded and must not alter normal production build
  semantics.
- A test seam is separate from runtime error propagation; it can force bounded
  internal evidence but does not prove all OS/provider RNG failure handling.
- The future seam must enable deterministic failure only where safe, with
  fail-closed behavior and no deterministic fallback values.

CI / Dependency / Release Health Steward:

- Root `cargo audit --deny warnings` is green.
- Nested qsc fuzz lock audit is green.
- `key_lifecycle_zeroization` and `handshake_provider_error_no_mutation` are
  green.
- qsc adversarial script marker and provider-error command remain present.
- public-safety is required and green on current `origin/main`.
- Cargo audit green is dependency-health evidence only.

Public Claims / External Review Steward:

- No RNG-failure-complete claim is made.
- No crypto-complete claim is made.
- No side-channel-free claim is made.
- No vulnerability-free claim is made.
- No bug-free claim is made.
- No perfect-crypto claim is made.
- No public-readiness claim is made.
- No production-readiness claim is made.
- No external-review-complete claim is made.
- Test seam authorization is internal engineering evidence only.

Product / Demo / Service Boundary Steward:

- qshield-cli remains demo-local.
- qsc runtime RNG failure and refimpl/provider randomness are separate.
- No qsl-server readiness claim is made. No qsl-attachments readiness claim is
  made. No qshield or website public-service readiness claim is made.

Local Ops / Backup / Restore Steward:

- No backup, restore, or local-ops mutation is authorized or performed.
- qsl-backup proof remains boundary evidence only.
- qsl-backup checksum matched the expected boundary value from the directive,
  and the Codex ops source-list inclusion count was exactly one.

## qsc RNG failure candidate surface review

| Surface | Exact source path(s) | Randomness role | Current API behavior | Failure representation | Test seam possible without normal production semantic change? | Runtime propagation required? | Candidate future test path(s) | Candidate future source path(s) | Risk | Immediate NA-0449 scope? |
|---|---|---|---|---|---|---|---|---|---|---|
| qsc session ID generation | `qsl/qsl-client/qsc/src/handshake/mod.rs` | `hs_session_id` derives the A1 handshake session ID from `OsRng` bytes | Existing public CLI/API can run handshake init, but cannot force RNG failure | Hidden dependency through infallible `fill_bytes`; actual OS failure is not represented as qsc `Result` | Yes, with a custom test-only cfg seam such as `qsc_rng_failure_test_seam` plus test env selector, absent from normal builds | No for bounded test seam; yes only if future claims require production OS RNG failure as an explicit runtime `Result` | `qsl/qsl-client/qsc/tests/rng_failure_behavior.rs` | `qsl/qsl-client/qsc/src/handshake/mod.rs` | Medium evidence gap; high claim risk if overextended | Yes |
| qsc vault nonce/salt generation | `qsl/qsl-client/qsc/src/vault/mod.rs` | Vault init/write nonces, passphrase salt, keychain fallback bytes, default route token | Existing commands can initialize/write vault, but cannot force RNG failure | Hidden dependency through `generate_nonce` / `fill_bytes`; vault/provider errors are separate | Yes, with the same test-only cfg seam and fail-closed write-abort checks | No for bounded test seam; yes if production RNG errors must become first-class API errors | `qsl/qsl-client/qsc/tests/rng_failure_behavior.rs` | `qsl/qsl-client/qsc/src/vault/mod.rs` | Medium evidence gap; write-mutation safety matters | Yes |
| qsc session/protocol-state nonce or key generation | `qsl/qsl-client/qsc/src/protocol_state/mod.rs` | Session-store encryption key and AEAD nonce for encrypted session blobs | Existing send/handshake paths call session store, but cannot force RNG failure | Hidden dependency through infallible `fill_bytes`; storage errors are separate | Yes, with the same test-only cfg seam and no-session-write invariant | No for bounded test seam; yes if production RNG errors must be exposed distinctly | `qsl/qsl-client/qsc/tests/rng_failure_behavior.rs` | `qsl/qsl-client/qsc/src/protocol_state/mod.rs` | Medium evidence gap; state mutation safety matters | Yes |
| qsc route/contact token generation | `qsl/qsl-client/qsc/src/contacts/mod.rs`; `qsl/qsl-client/qsc/src/vault/mod.rs`; `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs` | Route tokens, default route token, account verification seed | Existing commands can create tokens/seeds, but cannot force RNG failure | Hidden dependency through `fill_bytes` | Technically yes, but adding this to first seam broadens scope | Maybe later if route/token APIs need first-class errors | future exact qsc test only if authorized later | source paths listed in this row only if authorized later | Medium claim/UX boundary risk | No |
| qsc attachment token or ID generation | `qsl/qsl-client/qsc/src/attachments/mod.rs` | Attachment IDs, CEKs, nonce prefixes | Existing file transfer paths cannot force RNG failure | Hidden dependency through `fill_bytes` | Technically yes, but attachment/file-transfer scope is broader | Maybe later if attachment staging must expose RNG errors | future exact qsc attachment RNG test only if authorized later | `qsl/qsl-client/qsc/src/attachments/mod.rs` only if authorized later | Medium; file-transfer blast radius | No |
| qsc identity/key/provider-dependent generation | `qsl/qsl-client/qsc/src/identity/mod.rs`; `qsl/qsl-client/qsc/src/handshake/mod.rs`; `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`; `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs` | KEM/signature keypairs, X25519 keypairs, KEM encapsulation | Some provider errors already return `Result`; OS RNG failure and some keypair helpers remain infallible | Mixed provider `Result`, infallible helper, and hidden dependency | Only partly within qsc; refimpl trait/provider work is separate | Yes if trait contracts must expose RNG failure | future provider-boundary test only if authorized later | refimpl/qsc provider paths only if authorized later | Medium to high semantic risk | No |
| qsc provider-dependent boundaries not selected | same as identity/provider row | Provider randomness and crypto operations | Existing provider-error no-mutation tests cover selected rejects, not RNG failure | Partly `Result`; partly hidden | Not for immediate qsc test seam | Likely separate runtime/provider scope if broadened | future exact provider test only | future exact provider source only | Medium | No |

Operations to test first:

1. qsc handshake session ID RNG failure: prove a forced failure aborts handshake
   init before pending/session mutation.
2. qsc vault RNG failure: prove a forced vault salt/nonce/key-source failure
   aborts without writing a vault or secret update.
3. qsc session-store RNG failure: prove a forced session-store key/nonce
   failure aborts without writing a session blob or storing session state.

Route/contact tokens, attachments, identity key generation, and refimpl
provider randomness remain real surfaces but should follow after the first seam
proves the test-only mechanism and no-mutation invariants.

## Test seam strategy options

| Option | Recommendation | Evidence | Future exact paths if known | Validation required | Public-claim caveats |
|---|---|---|---|---|---|
| Option 1: Test-only seam in qsc test support/helper layer | Reject as insufficient for immediate RNG failure forcing | Existing integration helpers can set env, fixtures, relay behavior, vault state, and deterministic seeds, but cannot force private `OsRng` call sites to fail | No source-only helper path selected | N/A | Existing helper success would not prove RNG failure behavior |
| Option 2: Feature-gated test seam in qsc runtime source | Recommend | Direct RNG call sites are private and infallible at the API boundary; a custom cfg seam can be absent from normal builds and selected only under `RUSTFLAGS='--cfg qsc_rng_failure_test_seam'` | `qsl/qsl-client/qsc/src/handshake/mod.rs`; `qsl/qsl-client/qsc/src/protocol_state/mod.rs`; `qsl/qsl-client/qsc/src/vault/mod.rs`; `qsl/qsl-client/qsc/tests/rng_failure_behavior.rs` | Run seam-specific test under custom cfg; rerun ordinary qsc tests without cfg; prove no Cargo/lockfile/workflow changes | Bounded internal evidence only; no RNG-failure-complete claim |
| Option 3: Trait/interface extraction for RNG dependency | Reject for immediate successor | Would likely alter broader runtime/provider contracts and increase semantic risk | None selected | Would require separate runtime/provider authorization | No crypto-complete or provider-complete claim |
| Option 4: Documentation-only boundary | Reject | NA-0447 already documents the gap; this would not improve executable fail-closed evidence | None selected | N/A | Would preserve caveat only |
| Option 5: Formal/model scope first | Reject for immediate successor | Current models do not define RNG health semantics; model-first work would be more useful after scoped qsc behavior exists | None selected | Future formal scope only after exact semantics are defined | No formal-complete claim |
| Option 6: Runtime propagation scope | Defer | Needed only if future work must represent real OS RNG failure as production API error rather than bounded test-only seam failure | None selected for NA-0449 | Separate authorization if source changes exceed cfg-only seam or require production API contracts | No production RNG failure coverage claim |
| Option 7: Split-scope triage | Reject for immediate successor | Surfaces are separable enough to select a narrow first qsc seam while recording residuals | None selected | Future split if NA-0449 finds blast radius too broad | No all-surface RNG claim |

## Exact future scope candidate

The future implementation successor is authorization-ready only for this exact
path set:

Exact future qsc test path:

- `qsl/qsl-client/qsc/tests/rng_failure_behavior.rs`

Exact future runtime source seam paths:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/protocol_state/mod.rs`
- `qsl/qsl-client/qsc/src/vault/mod.rs`

Exact future governance paths:

- `docs/governance/evidence/NA-0449_qsl_qsc_rng_failure_test_seam_implementation_harness.md`
- `tests/NA-0449_qsl_qsc_rng_failure_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Required seam strategy:

- use a custom test-only cfg guard named `qsc_rng_failure_test_seam` or an
  equally explicit custom cfg recorded by NA-0449;
- activate the seam only in the future seam test command, for example:
  `RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture`;
- use an env selector only inside the cfg-guarded code path;
- do not add fallback random bytes, deterministic replacement bytes, or
  fallback-to-success behavior;
- fail closed before pending/session/vault mutation for selected cases;
- keep ordinary qsc tests green without the custom cfg;
- do not mutate Cargo manifests, lockfiles, dependencies, or workflows.

Required invariants:

- normal builds without the custom cfg do not read the seam env selector and do
  not change behavior;
- selected qsc RNG failure can be forced only in the test-only seam build;
- failure is an abort/reject path, not a downgrade or silent fallback;
- no pending handshake, session blob, session-store secret, vault file, or
  vault secret is created by the selected forced-failure cases;
- no public claim expansion is made.

Required markers:

- `NA0449_RNG_FAILURE_TEST_SEAM_IMPLEMENTED_OK`
- `NA0449_RNG_FAILURE_FORCED_BY_TEST_ONLY_SEAM_OK`
- `NA0449_PRODUCTION_SEMANTICS_UNCHANGED_OK`
- `NA0449_QSC_HANDSHAKE_SESSION_ID_RNG_FAILURE_NO_PENDING_OK`
- `NA0449_QSC_SESSION_STORE_RNG_FAILURE_NO_SESSION_WRITE_OK`
- `NA0449_QSC_VAULT_RNG_FAILURE_NO_VAULT_WRITE_OK`
- common NA-0449 governance markers listed in the future marker plan below.

Stop if future implementation needs any path outside the exact path set above,
needs Cargo/workflow mutation, needs dependency mutation, needs fuzz/vector/formal
mutation, or changes production API/runtime semantics beyond the cfg-only seam.

## Authorization matrix

| Surface | Failure condition to simulate | Existing API enough? | Seam strategy | Future mutable paths | Production semantics risk | Test invariant | Marker(s) | Priority | Selected for successor? | Public-claim implication |
|---|---|---|---|---|---|---|---|---|---|---|
| qsc session ID generation | force failure before `hs_session_id` returns A1 session ID bytes | no | custom cfg seam in `handshake/mod.rs` | `qsl/qsl-client/qsc/src/handshake/mod.rs`; `qsl/qsl-client/qsc/tests/rng_failure_behavior.rs` | Low if cfg-only; medium if call signatures drift | handshake init aborts before pending/session mutation | `NA0449_QSC_HANDSHAKE_SESSION_ID_RNG_FAILURE_NO_PENDING_OK` | P0 | yes | bounded qsc evidence only; no RNG-failure-complete claim |
| qsc vault nonce generation | force failure during vault init/write salt/nonce/key-source bytes | no | custom cfg seam in `vault/mod.rs` | `qsl/qsl-client/qsc/src/vault/mod.rs`; `qsl/qsl-client/qsc/tests/rng_failure_behavior.rs` | Low if cfg-only; medium because vault writes are stateful | no vault file/secret update is written by forced failure | `NA0449_QSC_VAULT_RNG_FAILURE_NO_VAULT_WRITE_OK` | P0 | yes | no production-readiness or crypto-complete claim |
| qsc session/protocol-state nonce/salt generation | force failure during session-store key or blob nonce generation | no | custom cfg seam in `protocol_state/mod.rs` | `qsl/qsl-client/qsc/src/protocol_state/mod.rs`; `qsl/qsl-client/qsc/tests/rng_failure_behavior.rs` | Low if cfg-only; medium because session writes are stateful | no session blob/session-store secret is written by forced failure | `NA0449_QSC_SESSION_STORE_RNG_FAILURE_NO_SESSION_WRITE_OK` | P0 | yes | no RNG-failure-complete claim |
| qsc route/contact/attachment token or ID generation | force failure for route token, account seed, attachment ID, CEK, or nonce prefix | no | later exact qsc seam only if separately authorized | none for NA-0449 | Medium because command/file-transfer blast radius broadens | future no-token/no-stage-write invariant | future exact marker | P1 backlog | no | residual gap remains |
| qsc identity/key/provider-dependent generation | force provider/OS RNG failure for KEM/signature/X25519/encap | partly provider errors only | later provider/runtime scope | none for NA-0449 | Medium/high because provider traits and crypto APIs may change | future no identity/session mutation invariant | future exact marker | P1 backlog | no | no provider-complete claim |
| qsc provider-dependent boundaries not selected | force failures beyond selected provider-error tests | partly | separate authorization if needed | none for NA-0449 | Medium/high | future bounded provider invariant | future exact marker | P2 backlog | no | no vulnerability-free or perfect-crypto claim |
| refimpl/provider RNG backlog | force refimpl RNG failure in `StdCrypto`/traits | no for infallible helpers | separate refimpl provider-boundary scope | none for NA-0449 | High if trait contracts change | future provider-boundary invariant | future exact marker | backlog | no | no refimpl RNG failure proof claim |
| qshield-cli demo RNG boundary | force qshield-cli demo or `/dev/urandom` fallback failure | not relevant to qsc | product/demo boundary only | none for NA-0449 | Low to medium claim risk | future demo-local invariant only | future exact marker | backlog | no | demo-local; no public-service readiness claim |

## Authorization decision

Selected primary classification:

`QSC_RNG_TEST_SEAM_IMPLEMENTATION_READY`

Rationale:

- F-0441-03 and NA-0447 identify a real qsc evidence gap.
- Existing qsc public APIs and integration-test helpers cannot force private
  RNG call-site failure.
- A narrow custom cfg seam can be scoped exactly to qsc handshake session ID,
  vault RNG, and session-store RNG paths.
- The future implementation can prove bounded fail-closed/no-mutation behavior
  without Cargo, dependency, lockfile, workflow, fuzz, vector, formal, public,
  qsl-server, qsl-attachments, qshield-cli, backup, or qwork mutation.
- Runtime error propagation remains future-gated if implementation requires
  production API contract changes or claims broader than the cfg-only seam.

NA-0448 authorizes no implementation mutation. Future implementation remains
gated by the NA-0449 directive and exact path set selected by D-0883.

## Successor selection

Selected exact successor:

`NA-0449 -- QSL qsc RNG Failure Test Seam Implementation Harness`

NA-0449 should implement only the exact qsc test-only cfg seam and test path
listed above. It must not implement refimpl RNG failure, qshield-cli RNG
failure, attachment/contact/token RNG seams, formal RNG modeling, fuzz targets,
vectors, dependency changes, Cargo changes, workflows, public docs, website
content, service changes, backup/restore/local-ops changes, qwork tooling, or
public claim expansion.

Exactly one READY successor remains mandatory. NA-0448 does not implement
NA-0449.

## Future path/scope bundle

Future NA-0449 implementation allowed paths:

- `qsl/qsl-client/qsc/tests/rng_failure_behavior.rs`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/protocol_state/mod.rs`
- `qsl/qsl-client/qsc/src/vault/mod.rs`
- `docs/governance/evidence/NA-0449_qsl_qsc_rng_failure_test_seam_implementation_harness.md`
- `tests/NA-0449_qsl_qsc_rng_failure_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0449 may inspect read-only:

- `qsl/qsl-client/qsc/src/`
- `qsl/qsl-client/qsc/tests/`
- `docs/governance/evidence/`
- `qsl/qsl-client/qsc/fuzz/`
- `formal/`
- `inputs/`
- relevant scripts/workflows read-only

Future forbidden scope unless a later exact directive authorizes it:

- runtime/crypto implementation changes outside the exact qsc source paths;
- dependency changes;
- Cargo manifest or lockfile changes;
- workflow changes;
- executable test source changes outside the exact qsc test path;
- fuzz target source changes;
- vector changes;
- formal model changes;
- public docs, website, README, or START_HERE changes;
- qsl-server or qsl-attachments changes;
- qshield runtime or qshield-cli changes;
- backup, restore, qsl-backup, status, plan, rollback, or `/backup/qsl`
  mutation;
- public claim expansion.

## Future validation/marker plan

Common NA-0449 markers:

- `NA0449_RNG_FAILURE_AUTHORIZATION_CONSUMED_OK`
- `NA0449_NEXT_SCOPE_SELECTED_OK`
- `NA0449_NO_DEPENDENCY_CHANGE_OK`
- `NA0449_NO_WORKFLOW_CHANGE_OK`
- `NA0449_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0449_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0449_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0449_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0449_ONE_READY_INVARIANT_OK`

Implementation successor markers:

- `NA0449_RNG_FAILURE_TEST_SEAM_IMPLEMENTED_OK`
- `NA0449_RNG_FAILURE_FORCED_BY_TEST_ONLY_SEAM_OK`
- `NA0449_PRODUCTION_SEMANTICS_UNCHANGED_OK`
- `NA0449_QSC_HANDSHAKE_SESSION_ID_RNG_FAILURE_NO_PENDING_OK`
- `NA0449_QSC_SESSION_STORE_RNG_FAILURE_NO_SESSION_WRITE_OK`
- `NA0449_QSC_VAULT_RNG_FAILURE_NO_VAULT_WRITE_OK`

Exact future seam command:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
```

Required future validation also includes ordinary qsc tests without the custom
cfg, qsc adversarial syntax checks, root cargo audit, nested qsc fuzz lock
audit, cargo tree inverse probes, cargo fmt, formal checks, scope guard,
link-check, leak-scan, overclaim scan, classifier, PR body preflight,
goal-lint, public-safety before merge, and public-safety after merge.

## Public claim/external review/website boundary

qsc RNG test seam authorization is internal governance evidence only.

- It is not production readiness.
- It is not public-internet readiness.
- It is not crypto-complete proof.
- It is not side-channel-free proof.
- It is not RNG-failure-complete proof.
- It is not bug-free proof.
- It is not vulnerability-free proof.
- It is not perfect-crypto proof.
- It is not public technical paper content.
- No README, START_HERE, public-docs, or website update is made.
- No public-readiness or public-security claim is made.
- Cargo audit green is dependency-health evidence only.
- Future tests, if implemented, must be described as bounded evidence only.

## Rejected alternatives

Documentation-only boundary:

Rejected because NA-0447 already records the caveat and because executable
bounded evidence can be authorized without broad implementation scope.

Formal/model-first scope:

Rejected for the immediate successor because current models do not define RNG
health semantics; model work becomes more useful after bounded qsc behavior is
specified.

Trait/interface extraction:

Rejected for NA-0449 because it risks broader provider/runtime API changes and
should be separately authorized if future work requires it.

Runtime propagation scope:

Deferred. If NA-0449 cannot preserve a cfg-only seam or must alter normal
production runtime error contracts, it must stop and select a runtime
propagation authorization successor.

Split-scope triage:

Rejected for immediate successor because qsc handshake/session/vault RNG
failure can be scoped exactly, while route/contact/attachment/provider/refimpl
surfaces remain recorded residuals.

## Backup-impact statement

No backup, restore, or local-ops mutation is authorized or performed. Codex did
not run backup or restore. Codex did not mutate qsl-backup, backup status
files, backup plan files, rollback subtree paths, timers, fstab, source lists,
retention, backup scripts, or backup tree paths.

The qsl-backup checksum matched the expected directive value, and the Codex ops
source-list inclusion count was exactly one.

## Next recommendation

Open and merge the NA-0448 evidence PR after validation and public-safety. After
merge and post-merge public-safety success, close out NA-0448 and restore
exactly one READY successor:

`NA-0449 -- QSL qsc RNG Failure Test Seam Implementation Harness`

NA-0449 should implement the exact test-only cfg seam and bounded test path
authorized by D-0883. It must stop if implementation requires paths or
semantics outside the exact D-0883 scope.
