Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-19

# NA-0498 QSL Core Assurance Checkpoint and Next Highest-Risk Security Lane Authorization Plan

## Executive summary

NA-0498 is a read-only core assurance checkpoint. It consumes the NA-0497 / D375 binding negative vector consumer closeout, inventories the current security evidence chain, and selects the next highest-value risk-reduction lane by security and external-review value rather than queue momentum.

Primary classification selected: `CORE_ASSURANCE_SIDE_CHANNEL_SECRET_MATERIAL_NEXT`.

Selected successor:

### NA-0499 -- QSL Side-Channel / Secret-Material Lifecycle Assurance Scope Authorization Plan
Status: READY
Goals: G1, G2, G3, G4, G5

This directive makes no implementation mutation, no qsc source/test/fuzz/Cargo mutation, no corpus/vector/input mutation, no workflow/script/helper mutation, no dependency/lockfile mutation, no formal/refimpl/service/public/backup mutation, and no public claim expansion.

## Live NA-0498 scope

Allowed mutation paths used by this evidence PR:

- `docs/governance/evidence/NA-0498_qsl_core_assurance_checkpoint_and_next_highest_risk_security_lane_authorization_plan.md`
- `tests/NA-0498_qsl_core_assurance_checkpoint_and_next_highest_risk_security_lane_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Read-only inspection covered qwork proof files, `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`, NA-0497 evidence/testplans, qsc source/tests/fuzz/corpus surfaces, internal negative binding vectors, validator and qsc-adversarial scripts, formal model surfaces, refimpl surfaces, workflow metadata, and read-only backup boundary evidence.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

The qwork proof files existed and were copied into the proof root:

- `/srv/qbuild/work/NA-0498/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0498/.qwork/startup.qsl-protocol.json`

Proof fields verified:

- `startup_result=OK`
- `lane=NA-0498`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0498/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0498`
- `requested_lane_status=READY`

The proof HEAD and proof origin/main matched live pre-fetch HEAD and origin/main at `ea3c24d99a7e`. Disk usage was below the hard stop before fetch: `/` was 76% and `/backup/qsl` was 23%.

The qwork `.kv` and `.json` proofs mirrored required values. One startup parser attempt was recovered after initially counting textual decision references rather than repository-native `- **ID:** D-####` decision entries. The final repository-native parser passed: READY count 1, READY `NA-0498`, `NA-0497` DONE, `NA-0496` DONE, D-0983 once, D-0984 once, D-0985 absent as a decision entry, and duplicate decision entry count zero.

After fetch, local `main` was fast-forward checked against `origin/main`. `origin/main` equals or descends from `ea3c24d99a7e`.

Read-only qsl-backup boundary:

- installed helper SHA-256 matched `e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`;
- the Codex ops source inclusion count was exactly 1;
- no backup or restore was run.

## NA-0497 / D375 inheritance

NA-0497 is closed and NA-0498 is restored READY as the sole READY item.

Inherited facts accepted from D375, D-0983, D-0984, NA-0497 evidence, NA-0497 testplan, and current main health:

- The vector manifest consumer test was implemented as `qsl/qsl-client/qsc/tests/binding_negative_vector_consumer.rs`.
- The consumer parses the internal negative binding vector manifest and validates schema, category, layer, group, mapping, no-secret material policy, and claim-boundary metadata.
- Manifest schema/category/layer/mapping evidence is now executable qsc integration-test evidence.
- qsc-frame vectors are qsc-facing evidence; refimpl and formal-token sections remain supporting-only.
- The internal-only/no-public-conformance-vector boundary is preserved.
- No public-readiness claim, no production-readiness claim, no public-internet-readiness claim, no crypto-complete claim, no fuzz-complete claim, no corpus-complete claim, no vector-complete claim, no replay-proof claim, no downgrade-proof claim, no side-channel-free claim, no vulnerability-free claim, no bug-free claim, and no perfect-crypto claim is inherited or introduced.
- qsc-adversarial validator integration is already green on current main.
- Current disk pressure is below the hard stop.
- This checkpoint is intentionally the next step to prevent process drift after the RNG, binding, formal, fuzz, corpus, validator, qsc-adversarial, and vector-consumer chain.

## Core evidence inventory

| Evidence chain | Current evidence classification | Direct/supporting/absent summary | Residual |
|---|---|---|---|
| qsc KEM provider RNG | DIRECT_IMPLEMENTATION_EVIDENCE | KEM provider RNG forced-seam evidence exists for selected keypair/encap labels, with no-cfg production behavior preserved. | No provider-RNG-complete claim; broader provider and refimpl RNG remain residual. |
| Signature provider RNG | DIRECT_IMPLEMENTATION_EVIDENCE | B1 and A2 signing paths have bounded qsc provider RNG failure evidence with distinct no-output/no-mutation boundaries. | No signature-complete or provider-RNG-complete claim. |
| Lazy identity provider RNG | DIRECT_IMPLEMENTATION_EVIDENCE | Lazy identity KEM/signature generation forced failures are covered by bounded qsc seam evidence. | Identity-complete remains unclaimed. |
| Legacy/public-record provider RNG | DIRECT_IMPLEMENTATION_EVIDENCE | Legacy/public-record upgrade provider RNG failures are covered by bounded qsc seam evidence. | No provider-RNG-complete claim. |
| CLI rotation provider RNG | DIRECT_IMPLEMENTATION_EVIDENCE | CLI identity rotation provider RNG failure evidence covers selected identity, vault, public-record, and peer-reset surfaces. | No identity-complete claim. |
| TUI bootstrap pre-generation transactionality | DIRECT_IMPLEMENTATION_EVIDENCE | TUI bootstrap now pre-generates identity material before durable account/default writes; forced provider failures leave no selected durable identity state. | In-memory secret lifetime caveat remains. |
| Provider-error no-mutation | DIRECT_NEGATIVE_TEST_EVIDENCE | Provider-error no-mutation tests remain inherited bounded qsc evidence. | Does not model every provider/RNG failure. |
| X25519/ephemeral residual | RESIDUAL_OPEN | Evidence chain repeatedly records X25519/ephemeral provider RNG as separate residual. | Candidate later lane; not selected over secret-material lifecycle in NA-0498. |
| refimpl provider RNG residual | RESIDUAL_OPEN | refimpl provider RNG remains deferred and trait-level changes may be needed for failure injection. | Future provider-boundary scope. |
| qshield-cli demo RNG residual | RESIDUAL_OPEN | qshield-cli demo RNG remains product/demo-local residual, not qsc core. | Not next core protocol assurance lane. |
| Formal/fuzz/vector RNG residuals | SUPPORTING_ONLY_EVIDENCE | Formal, fuzz, and vector surfaces support assurance but do not directly model RNG failure behavior. | Keep as explicit residual. |
| qsc KEM/signature/transcript negative tests | DIRECT_NEGATIVE_TEST_EVIDENCE | qsc negative tests cover wrong KEM public key/ciphertext, wrong signature public record, cross-message signature replay, transcript mutation, A1 replay, suite confusion, stale public-record, and no completed-session mutation. | No replay-proof or downgrade-proof claim. |
| stale public-record / rollback negative tests | DIRECT_NEGATIVE_TEST_EVIDENCE | qsc binding negative tests and manifest metadata cover stale public-record / public-record rollback evidence classes. | Local storage rollback and relay stale delivery remain deeper residuals. |
| suite confusion / replay negative tests | DIRECT_NEGATIVE_TEST_EVIDENCE | qsc negative tests and fuzz target cover selected suite-confusion and replay surfaces. | No complete replay/downgrade proof. |
| qsc binding fuzz helper/target | DIRECT_IMPLEMENTATION_EVIDENCE | qsc binding fuzz target and helper exist with checked-in seeds. | Local cargo-fuzz is absent, so local parity is process residual. |
| checked-in binding corpus | DIRECT_IMPLEMENTATION_EVIDENCE | Seven small public/synthetic seeds exist under the binding corpus. | Corpus-complete not claimed. |
| corpus validator | DIRECT_NEGATIVE_TEST_EVIDENCE | Validator rejects secret-like corpus content and is used for binding and all qsc fuzz corpus scans. | Validator-complete not claimed. |
| qsc-adversarial validator integration | DIRECT_NEGATIVE_TEST_EVIDENCE | qsc-adversarial runs validator gates before fuzz phases and is green in PR/main evidence. | Local cargo-fuzz absence remains process residual. |
| vector consumer test | DIRECT_NEGATIVE_TEST_EVIDENCE | qsc integration test consumes manifest metadata and enforces internal-only/no-public-claim boundaries. | Dynamic execution from manifest metadata remains intentionally absent. |
| formal binding model | DIRECT_FORMAL_BOUNDED_EVIDENCE | Bounded model covers KEM/signature/transcript token binding but explicitly avoids crypto proof and qsc/refimpl equivalence claims. | Formal-model mapping residual remains active. |
| refimpl signature provider-boundary test | SUPPORTING_ONLY_EVIDENCE | refimpl evidence supports provider-boundary reasoning. | qsc/refimpl equivalence not proven. |
| qsc/refimpl mapping residual | RESIDUAL_OPEN | Multiple rows preserve qsc/refimpl/formal mapping residual. | Candidate later lane. |
| zeroization evidence | DIRECT_NEGATIVE_TEST_EVIDENCE | qsc key lifecycle zeroization tests and refimpl X25519 zeroize traits exist. | Comprehensive zeroization/wipe expectations remain incomplete. |
| no-secret-output evidence | DIRECT_NEGATIVE_TEST_EVIDENCE | Validator, qsc-adversarial, output/diagnostic tests, and no-secret manifest policy enforce bounded output discipline. | Diagnostic/logging review remains cross-cutting. |
| TUI in-memory secret lifetime caveat | RESIDUAL_OPEN | TUI bootstrap pre-generation improves durability transactionality but carries an explicit in-memory lifetime caveat. | Selected next lane. |
| side-channel non-claim | GOVERNANCE_ONLY_EVIDENCE | Existing governance repeatedly preserves no side-channel-free claim. | Side-channel and memory lifetime need scoped assurance review. |
| secret-material-complete residual | RESIDUAL_OPEN | F-0441-02 records incomplete KEM/PQ signature/shared secret/pending/session wipe expectations. | Selected next lane. |
| memory-erasure-complete residual | RESIDUAL_OPEN | Current tests are bounded and do not prove comprehensive memory erasure. | Selected next lane. |
| constant-time residual | NOT_COVERED | Current chain does not establish constant-time behavior across operations. | Keep as side-channel caveat, not claim. |
| provider secret output discipline | SUPPORTING_ONLY_EVIDENCE | Provider returns and diagnostic surfaces are bounded by tests and sanitizer expectations. | Needs cross-cutting lifecycle/diagnostic inventory. |
| specs/evidence/testplans | SUPPORTING_ONLY_EVIDENCE | Internal governance evidence is extensive and traceable. | External-review package not complete. |
| threat-model coverage | SUPPORTING_ONLY_EVIDENCE | Threat-model and red-team style reviews exist across recent lanes. | Public package and cryptographer-ready summary remain residual. |
| formal model coverage | DIRECT_FORMAL_BOUNDED_EVIDENCE | Bounded formal evidence exists. | Do not overinterpret as cryptographic proof. |
| fuzz/corpus coverage | DIRECT_IMPLEMENTATION_EVIDENCE | qsc fuzz/corpus/validator/qsc-adversarial chain exists. | Fuzz-complete and corpus-complete not claimed. |
| dependency-health evidence | SUPPORTING_ONLY_EVIDENCE | `cargo audit --deny warnings` is dependency-health evidence only. | Does not prove protocol security. |
| release-claim boundary | GOVERNANCE_ONLY_EVIDENCE | Repeated rows preserve release and public-claim boundaries. | Needs continued enforcement. |
| disk pressure | PROCESS_DEFER | Current disk pressure below hard stop. | Not next core assurance blocker. |
| local cargo-fuzz absent | PROCESS_DEFER | Local missing cargo-fuzz is recorded but CI qsc-adversarial-smoke is green. | Defer unless it blocks core assurance. |
| CI long-wait behavior | PROCESS_DEFER | Wait policy is bounded and long waits belong to closeout/evidence directives. | Not next core assurance blocker. |
| GitHub connector PR permissions | PROCESS_DEFER | Local `gh` auth is available; connector/CLI flow is adequate. | Not core security. |
| proof-root / response-writing pitfalls | PROCESS_DEFER | Proof root and response-writing rules are known. | Maintain guardrails. |
| queue/governance parser friction | PROCESS_DEFER | One parser recovery was needed because decision entries use `- **ID:**`. | Improve local proof scripts later if it recurs. |

## Highest-risk residual review

| Residual | Classification | Assessment |
|---|---|---|
| Side-channel / secret-material lifecycle assurance scope | CORE_SECURITY_HIGH | Cross-cuts every cryptographic path. Current evidence is bounded zeroization/no-secret-output evidence plus caveats. The open gap affects external cryptographer review and public-claim honesty. Feasible next lane is authorization-only and can define exact future inspection/test scope without code mutation. |
| X25519 / ephemeral RNG/provider boundary | CORE_SECURITY_MEDIUM | Real provider/RNG residual with attack relevance, but narrower than secret-material lifecycle and partly downstream of provider-boundary strategy. It remains active, not selected. |
| qsc/refimpl KEM/signature/provider mapping deeper implementation | ASSURANCE_HIGH | Important for equivalence and external review, but recent binding/formal chain already advanced mapping. Scope risk is higher than a lifecycle authorization checkpoint. |
| stale identity / replay / rollback deeper test expansion | CORE_SECURITY_MEDIUM | Attack-relevant and partially directly tested. Deeper local storage rollback/relay stale delivery remains useful but less cross-cutting than secret lifecycle. |
| qsc/refimpl/formal mapping gap | ASSURANCE_HIGH | Formal model is bounded and qsc/refimpl equivalence is unproven. Candidate later lane. |
| external-review readiness package | ASSURANCE_HIGH | Valuable, but should not outrank closing the secret-material lifecycle scope that external reviewers would ask for first. |
| local cargo-fuzz parity / qsc-adversarial local environment | PROCESS_DEFER | Useful operations parity; CI evidence is currently green and the gap does not block NA-0499 authorization. |
| CI watcher / wait-reduction tooling | PROCESS_DEFER | Reduces wait waste but does not reduce core cryptographic risk. |
| disk/archive routine | PROCESS_DEFER | Current disk state is below hard stop. Defer unless pressure returns. |
| backup/off-host/restore/key-custody residual | GOVERNANCE_ONLY | Important governance residual, but not a substitute for protocol assurance. |
| public technical paper/external claim boundary | GOVERNANCE_ONLY | Boundary should remain strict; no public paper work should outrank core assurance. |
| D132 cleanup timing | NOT_NEXT | Not core security and not blocking current assurance work. |

## Hostile Cryptographer Review

A hostile cryptographer would attack the evidence chain first at its boundaries: bounded formal models, qsc/refimpl non-equivalence, provider RNG failure seams that do not prove every provider path, and side-channel/secret-material lifecycle gaps that are easy to overstate.

The proof most likely to be mistaken as stronger than it is is the bounded formal binding model. It is useful state-machine evidence, but it does not prove cryptographic authentication, AEAD security, side-channel behavior, provider behavior, refimpl behavior, or qsc/refimpl equivalence.

The binding vector consumer is also easy to overinterpret. It consumes manifest metadata and enforces schema/mapping/claim boundaries; it does not dynamically execute every negative vector as an end-to-end cryptographic test.

The crypto/protocol boundary most under-evidenced after the recent chain is the lifecycle of secret material across generation, in-memory use, diagnostic/output surfaces, zeroization/wipe expectations, and side-channel caveats. That is the selected next lane.

## Red-Team Review

The most plausible under-tested attacker behaviors after the inherited chain are:

- secret material leakage through in-memory lifetime, diagnostics, logs, crash artifacts, or test artifacts;
- side-channel and metadata leakage that could be incorrectly implied as addressed;
- local storage rollback or stale delivery beyond bounded negative tests;
- relay/server stale delivery outside qsc-only harnesses;
- X25519/ephemeral provider RNG failure outside current selected seams;
- qsc/refimpl/formal mismatch at implementation boundaries.

Replay, downgrade, stale public-record, and rollback have meaningful direct negative evidence, but no replay-proof or downgrade-proof claim is made. Provider RNG failure has meaningful qsc bounded evidence, but no provider-RNG-complete claim is made. The next lane that best reduces red-team risk is the side-channel / secret-material lifecycle scope authorization because it targets leakage and caveat boundaries that cut across every other assurance chain.

## Production SRE Review

Operational confusion risk is highest where bounded internal evidence could be interpreted as operational readiness. Secret-material lifecycle gaps can cause unsafe operator confidence if diagnostics, logs, temporary files, crash output, or in-memory lifetimes are not explicitly scoped.

Process-only residuals are not next:

- local cargo-fuzz parity is useful but CI qsc-adversarial-smoke is green;
- CI wait tooling reduces waste but does not reduce core cryptographic risk;
- disk/archive work is not urgent below the hard stop;
- backup/off-host/restore/key-custody remains governance residual, not protocol assurance.

The selected successor improves release-support language by making the no side-channel-free, no secret-material-complete, and no memory-erasure-complete boundaries reviewable without public overclaim.

## Side-Channel Caveat Review

Current evidence does not establish constant-time behavior, absence of side channels, comprehensive memory erasure, or complete secret lifecycle safety. The project must continue to say:

- no side-channel-free claim;
- no secret-material-complete claim;
- no memory-erasure-complete claim;
- no vulnerability-free, bug-free, or perfect-crypto claim.

The selected NA-0499 lane is authorization-only and must preserve those caveats while deciding exact future inspection/test scope.

## Formal-Model Mapping Residual Review

Formal evidence is bounded and intentionally narrow. The qsc KEM/signature/transcript binding model uses opaque tokens and explicitly excludes cryptographic proof, provider behavior, refimpl behavior, key schedule proof, and qsc/refimpl equivalence.

Formal mapping remains active residual evidence, not a blocker that outranks side-channel / secret-material lifecycle in this checkpoint.

## External-Review Readiness Review

External review readiness is incremental. The project now has useful internal evidence for identity/provider RNG, KEM/signature/transcript binding, formal bounded checks, fuzz/corpus validation, qsc-adversarial integration, and vector-manifest consumption.

The most serious external-review blocker is not absence of process evidence. It is the need to present a crisp, honest lifecycle and side-channel boundary: what secret material exists, where it lives, how long it lives, what is zeroized or not, what output is secret-safe, and what side-channel properties are not claimed.

NA-0499 should authorize that scope before any public external review package or public technical paper work.

## Release-Claim Boundary Review

NA-0498 preserves:

- no public-readiness claim;
- no production-readiness claim;
- no public-internet-readiness claim;
- no crypto-complete claim;
- no identity-complete claim;
- no KEM/signature/transcript-complete claim;
- no provider-RNG-complete claim;
- no replay-proof claim;
- no downgrade-proof claim;
- no side-channel-free claim;
- no vulnerability-free, bug-free, or perfect-crypto claim;
- no external-review-complete claim;
- no backup-complete or restore-proof claim.

Cargo audit green remains dependency-health evidence only.

## Prioritization matrix

| Candidate | Risk reduced | Evidence gap | Attack relevance | Implementation uncertainty | Scope feasibility | Overclaim risk | External-review value | Process/tooling dependency | Recommended disposition | Next-lane candidate |
|---|---|---|---|---|---|---|---|---|---|---|
| Side-channel / secret-material lifecycle assurance scope | Secret leakage, diagnostic leakage, memory lifetime, side-channel caveat drift | Bounded zeroization/no-secret evidence; lifecycle incomplete | High | Medium | High for authorization-only scope | High if ignored | High | Low | Select as NA-0499 | Yes |
| X25519 / ephemeral provider RNG boundary | Provider RNG failure in ephemeral key generation | Residual open | Medium | Medium/high | Medium | Medium | Medium | Low | Defer as active residual | No |
| qsc/refimpl mapping implementation or scope | Implementation equivalence uncertainty | Residual open | Medium/high | High | Medium | High | High | Low | Defer after lifecycle scope | No |
| stale identity / replay / rollback deeper negative tests | Replay/stale/rollback behavior | Partial direct tests | Medium/high | Medium | Medium | Medium | Medium | Low | Defer; not next | No |
| qsc/refimpl/formal mapping checkpoint | Formal/refimpl/qsc boundary | Bounded model only | Medium | Medium | High for checkpoint | Medium/high | High | Low | Defer as later assurance lane | No |
| external-review readiness package | Review packaging | Public package absent | Medium | Low | High | High if premature | High | Low | Defer until lifecycle scope | No |
| local cargo-fuzz parity | Local validation parity | Local tool absent | Low core security | Low | High | Low | Low/medium | Medium | Process defer | No |
| CI watcher / wait tooling | Wait waste | Tooling gap | Low core security | Low | High | Low | Low | Medium | Process defer | No |
| disk/archive routine | Disk stop prevention | Current below threshold | Low current | Low | High | Low | Low | Low | Process defer | No |
| backup/off-host/restore/key-custody | Operational continuity | Governance residual | Low protocol assurance | High | Medium | High if conflated | Medium | High | Governance-only defer | No |
| public technical paper/external claim boundary | Public overclaim prevention | Claim boundary exists | Medium | Low | High | High | Medium | Low | Defer public work; preserve boundary | No |

## Authorization decision

Selected primary classification: `CORE_ASSURANCE_SIDE_CHANNEL_SECRET_MATERIAL_NEXT`.

Decision basis:

- NA-0497/D375 was consumed.
- Evidence inventory was completed.
- Highest-risk residual review was completed.
- Hostile cryptographer, red-team, production SRE, side-channel caveat, formal-model mapping residual, external-review readiness, and release-claim reviews were completed.
- Prioritization matrix was completed.
- Process/tooling lanes were rejected because they do not currently block core assurance.
- Backup/off-host/restore/key-custody was treated as governance residual, not protocol assurance substitute.

## Selected NA-0499 successor

### NA-0499 -- QSL Side-Channel / Secret-Material Lifecycle Assurance Scope Authorization Plan
Status: READY
Goals: G1, G2, G3, G4, G5

Objective:
Authorize the next exact assurance lane for QSL secret-material lifecycle, including in-memory lifetime caveats, zeroization evidence boundaries, secret-output discipline, diagnostic/logging boundaries, and side-channel non-claim preservation.

Allowed scope:

- governance evidence/testplan;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- rolling journal;
- read-only inspection of qsc identity/key lifecycle tests, TUI bootstrap pre-generation evidence, provider-error evidence, qsc/refimpl crypto helpers, formal evidence, and public-claim boundaries.

Forbidden:

- implementation mutation unless future exact lane authorizes it;
- qsc/refimpl/crypto/source mutation;
- dependency/workflow mutation;
- public readiness or side-channel-free claim.

## Future scope bundle

Future NA-0499 should use exact markers such as:

- `NA0499_SECRET_MATERIAL_LIFECYCLE_SCOPE_AUTHORIZED`
- `NA0499_IN_MEMORY_LIFETIME_CAVEAT_RECORDED`
- `NA0499_ZEROIZATION_BOUNDARY_RECORDED`
- `NA0499_SECRET_OUTPUT_DISCIPLINE_RECORDED`
- `NA0499_DIAGNOSTIC_LOGGING_BOUNDARY_RECORDED`
- `NA0499_SIDE_CHANNEL_NON_CLAIM_PRESERVED`
- `NA0499_NO_IMPLEMENTATION_MUTATION`
- `NA0499_NO_PUBLIC_CLAIM_EXPANSION`

Future NA-0499 validation should include queue/decision proof, exact allowed-path scope guard, link check, leak scan, added-line overclaim scan, classifier, PR body preflight, goal-lint, root cargo audit, nested qsc fuzz lock audit, cargo fmt, formal model checks, binding corpus validator scan, all qsc fuzz corpus validator scan, and current public-safety/qsc-adversarial status evidence.

Future NA-0499 stop conditions should include stale qwork proof, more than one READY item, D-0986 already present at start, implementation mutation attempt, qsc/refimpl/crypto source mutation attempt, dependency/workflow mutation attempt, public claim expansion, side-channel-free claim, secret-material-complete claim, vulnerability-free claim, root or nested audit red, validator red, public-safety red/missing, or disk usage at or above the hard stop.

## Public claim / website / external review boundary

No public docs, README, START_HERE, website, public technical paper, public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, side-channel-free, secret-material-complete, vulnerability-free, bug-free, or perfect-crypto claim is authorized by NA-0498.

External-review readiness is improved only as an internal planning step. It is not represented as external review completion.

## Rejected alternatives

- `CORE_ASSURANCE_X25519_EPHEMERAL_PROVIDER_RNG_NEXT`: rejected as active but narrower residual after the broader identity/provider RNG chain and current secret-material lifecycle caveat.
- `CORE_ASSURANCE_STALE_IDENTITY_REPLAY_ROLLBACK_NEXT`: rejected because direct bounded negative evidence already exists and the residual is less cross-cutting than secret-material lifecycle.
- `CORE_ASSURANCE_QSC_REFIMPL_FORMAL_MAPPING_NEXT`: rejected as high-value but less immediate than lifecycle/side-channel scoping.
- `CORE_ASSURANCE_EXTERNAL_REVIEW_READINESS_NEXT`: rejected as premature until lifecycle and side-channel boundaries are clearer.
- `CORE_ASSURANCE_PROVIDER_RNG_RESIDUAL_NEXT`: rejected because recent provider RNG work already covered multiple qsc paths and remaining residuals are narrower or refimpl-scoped.
- `CORE_ASSURANCE_RELEASE_CLAIM_BOUNDARY_NEXT`: rejected because release-claim boundaries are preserved and not the highest security reducer.
- `CORE_ASSURANCE_BACKUP_RESTORE_KEY_CUSTODY_NEXT`: rejected because it is governance/ops residual, not core protocol assurance.
- Process blocker classifications: rejected because no process/tooling issue currently blocks core assurance.

## Backup-impact statement

NA-0498 has no backup impact beyond normal repository governance file changes. Codex did not run backup or restore, did not mutate qsl-backup, did not mutate backup status or backup plan, did not touch rollback trees, and did not write under `/backup/qsl`.

## Next recommendation

Merge the NA-0498 evidence PR after targeted validation and required checks. If post-merge public-safety attaches and is green within the short attach/early-failure window, close out NA-0498 and restore the selected NA-0499 side-channel / secret-material lifecycle authorization plan as the sole READY successor. If public-safety is still running but healthy after the short window, stop and hand off closeout as a separate directive.
