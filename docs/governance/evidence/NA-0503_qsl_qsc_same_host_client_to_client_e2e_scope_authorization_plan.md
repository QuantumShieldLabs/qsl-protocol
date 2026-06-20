Status: Supporting
Owner: QSL Governance / Core Assurance
Last-Updated: 2026-06-20

# NA-0503 qsc Same-Host Client-to-Client E2E Scope Authorization Plan

## Executive summary

NA-0503 is authorization-only. It consumes NA-0502/D388 inheritance and
selects the next exact qsc assurance lane: `NA-0504 -- QSL qsc Same-Host
Client-to-Client End-to-End Test Implementation Harness`.

Primary classification:
`SAME_HOST_CLIENT_TO_CLIENT_E2E_IMPLEMENTATION_READY`.

The future lane should implement a bounded same-host qsc integration test at
`qsl/qsl-client/qsc/tests/same_host_client_to_client_e2e.rs`. The test should
use two independent temp client roots, existing qsc CLI/test-visible helper
patterns, realistic Alice/Bob identity setup, public-record/trusted-pin
exchange, send, receive, reply, and at least one negative reject/no-mutation
case. It must not require remote SSH, two-machine setup, qsc source mutation,
helper mutation, dependency mutation, workflow mutation, or corpus/vector/input
mutation.

## Live NA-0503 scope

Allowed NA-0503 mutation paths are limited to this evidence doc,
`tests/NA-0503_qsl_qsc_same_host_client_to_client_e2e_scope_authorization_testplan.md`,
`DECISIONS.md`, `TRACEABILITY.md`, and
`docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

NA-0503 does not implement same-host E2E behavior. It records the current
surface inventory, option review, stewardship reviews, prioritization matrix,
authorization decision, future NA-0504 scope bundle, future marker plan, and
claim boundaries.

## qwork proof-file verification

- qwork was not run by Codex.
- Startup proof files were present and copied into the directive proof root:
  `/srv/qbuild/tmp/NA0503_same_host_client_to_client_e2e_scope_20260620T010016Z/`.
- `.kv` and `.json` proof agreed on `startup_result=OK`, `lane=NA-0503`,
  `repo=qsl-protocol`, path `/srv/qbuild/work/NA-0503/qsl-protocol`,
  `head_equals_origin_main=yes`, clean worktree/index/untracked state,
  `ready_count=1`, `queue_top_ready=NA-0503`, and
  `requested_lane_status=READY`.
- Proof HEAD and proof `origin_main` both matched live pre-fetch refs:
  `11f57c90a12e`.
- Fetch was performed only after proof/live ref match.
- `origin/main` equals or descends from `11f57c90a12e`; at authoring it is
  `11f57c90a12e`.
- Disk proof before fetch: `/` was 77% used and `/backup/qsl` was 24% used,
  below the 95% stop threshold.
- Startup queue proof: `READY_COUNT=1`, READY item `NA-0503`; `NA-0500`,
  `NA-0501`, and `NA-0502` are DONE.
- Startup decision proof: D-0993 exists once, D-0994 exists once, D-0995 and
  D-0996 are absent, and duplicate decision count is zero.
- D388 response exists at
  `/home/victor/work/qsl/codex/responses/NA0502_closeout_restore_na0503_20260619T215318Z_D388.md`.
- qsl-backup boundary proof: `/usr/local/sbin/qsl-backup` SHA-256 is
  `e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`;
  the Codex ops source inclusion count is exactly 1. Codex did not run backup
  or restore.

Recovered startup proof issue: a queue probe used a bare `rg` command for
expected-absent D-0995, and `rg` exited nonzero on zero matches. This was
classified as a recoverable zero-match proof outcome. The corrective action was
to rerun the probe with zero-match-safe guards and the decision-record grammar
used by `DECISIONS.md`. Final result: D-0995 count 0, D-0993 count 1,
D-0994 count 1, and duplicate decision count 0.

## NA-0502 / D388 inheritance

- NA-0502 completed and was closed by D388 / PR #1277.
- NA-0503 was restored as the sole READY item.
- NA-0502 implemented
  `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization_expansion.rs`.
- The NA-0502 test checked selected lifecycle/cleanup surfaces: identity
  KEM/signing secret rotation and public-record artifact boundaries, plus
  responder pending-confirm reject and session-artifact boundaries.
- NA-0502 introduced no qsc source, dependency, workflow, corpus/vector/input,
  formal, refimpl, service, public-doc, or backup mutation.
- NA-0502 selected same-host client-to-client E2E as the next broad,
  user-realistic assurance lane.
- No public-readiness claim is made. no production-readiness claim is made. no
  public-internet-readiness claim is made. no external-review-complete claim is
  made. no crypto-complete claim is made. no replay-proof claim is made. no
  downgrade-proof claim is made. no secret-material-complete claim is made. no
  zeroization-complete claim is made. no memory-erasure-complete claim is made.
  no side-channel-free claim is made. no vulnerability-free claim is made. no
  bug-free claim is made. no perfect-crypto claim is made.

## Current client-to-client / E2E surface inventory

| Surface | Existing path | Alice/Bob roots | Identity setup | Public-record/trusted-pin exchange | Send | Receive | Reply | Local relay/inbox | Negative reject/no-mutation | Diagnostics/no-secret output | Mutation needed | CI deterministic | Future use |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| qsc process/test isolation helpers | `qsl/qsl-client/qsc/tests/common/mod.rs` | Yes, through `TestIsolation` and per-command `QSC_CONFIG_DIR` | Supports process env and unlock args | Supports CLI process setup indirectly | Supports CLI process invocation | Supports CLI process invocation | Supports CLI process invocation | `start_inbox_server` and route-token channels | Supports output capture and artifact snapshots | stdout/stderr captured; helper env can force plain markers | No qsc helper mutation required | Yes | Base helper pattern for NA-0504 |
| Existing two-way local inbox E2E | `qsl/qsl-client/qsc/tests/receive_e2e.rs` | Yes, separate Alice/Bob cfg roots | Yes, `identity rotate` and `identity show` | Yes, contact add/route setup and authenticated contact setup | Yes, Alice sends to Bob | Yes, Bob receives | Yes, Bob sends reply and Alice receives | Yes, `start_inbox_server` | Partial; wrong mailbox/wrong peer no-output/no-mutation cases exist in same file | Yes, markers and secret-output assertions exist | No source/helper/dependency/workflow mutation required | Yes | Primary implementation template |
| Handshake MVP CLI | `qsl/qsl-client/qsc/tests/handshake_mvp.rs` | Yes | Yes | Yes, pinned contacts and trust | No message send, handshake only | No message receive, handshake only | No | Yes | Handshake reject behavior present in related tests | stdout/stderr captured | No | Yes | Identity/trust/handshake setup template |
| Identity binding negative | `qsl/qsl-client/qsc/tests/identity_binding.rs` | Yes | Yes | Yes, route-only and pinned contact paths | Blocks send/handshake when identity unknown | Not a full receive path | No | Yes | Route-only first contact rejects without silent TOFU/session mutation | stdout/stderr captured | No | Yes | Negative public-record/trust boundary template |
| Identity UX and trust onboarding | `qsl/qsl-client/qsc/tests/identity_ux.rs`, `qsl/qsl-client/qsc/tests/trust_onboarding_mainstream_flow_na0187.rs` | Mostly single-root plus contact device flows | Yes | Yes, verify/trust/contact-device states | Send is blocked until trust policy allows | Inbound request flows exist | No full Alice/Bob reply | Uses CLI/TUI test-visible flows | Mismatch and device-change send-block evidence | Explicit no-leak checks | No | Yes | Trust-policy setup and output-safety reference |
| Mock/local relay transport | `qsl/qsl-client/qsc/tests/mock_relay_transport_na0173.rs`, `qsl/qsl-client/qsc/tests/common/mod.rs` | Supports test roots | Not identity-specific | Route-token channels | Supports push/send behaviors | Supports pull/receive behaviors | Supports second send | Yes | Drop/dup/reorder can be combined with no-mutation tests | stdout/stderr and HTTP behavior observable | No | Yes | Local transport for same-host E2E |
| Send/receive no-mutation | `qsl/qsl-client/qsc/tests/send_semantics.rs`, `qsl/qsl-client/qsc/tests/receive_no_mutation.rs`, `qsl/qsl-client/qsc/tests/relay_drop_no_mutation.rs`, `relay_dup_no_mutation.rs`, `relay_reorder_no_mutation.rs` | Primarily single or scoped roots | Not full identity setup | Contact/route setup where needed | Yes | Yes | Some reply-like repeated sends | Yes | Direct drop/duplicate/reorder or failed receive no-mutation evidence | stdout/stderr captured | No | Yes | Negative case source for NA-0504 |
| Provider-error no-mutation | `qsl/qsl-client/qsc/tests/handshake_provider_error_no_mutation.rs` | Yes | Yes | Yes | No final message send | Handshake poll | No | Yes | PQ decap failure rejects and preserves selected session/pending/vault state | Reject markers and no-success/no-secret assertions | No | Yes | High-value negative boundary template |
| Binding negative tests | `qsl/qsl-client/qsc/tests/kem_signature_transcript_binding_negative.rs`, `binding_negative_vector_consumer.rs` | Yes | Yes | Yes | Handshake messages, not app send | Handshake poll | No | Yes | Wrong KEM, stale public-record, signature replay, transcript mutation, replay, suite confusion reject without session mutation | Reject markers and no-leak assertions | No | Yes | Negative stale/replay/corrupt boundary source |
| Diagnostic/no-output boundary | `qsl/qsl-client/qsc/tests/secret_material_diagnostic_boundary.rs` | Selected roots | Selected handshake errors | Identity unknown/diagnostic surfaces | No | No | No | No | Ensures reject diagnostics do not leak selected secret-like markers | Direct stdout/stderr/process-output scan | No | Yes | Required output hygiene pattern |
| Key lifecycle / zeroization | `key_lifecycle_zeroization.rs`, `key_lifecycle_zeroization_expansion.rs` | Test roots | Yes | Public-record artifact boundaries | No | Handshake reject artifacts | No | Yes for expansion | Reject/artifact no-mutation and lifecycle cleanup | Output and artifact checks | No | Yes | Inheritance and artifact snapshot pattern |
| Formal/corpus binding evidence | `formal/model_qsc_kem_signature_transcript_binding_bounded.py`, `scripts/audit/validate_binding_fuzz_corpus_no_secrets.py`, qsc fuzz corpus | Abstract/model or corpus, not qsc CLI roots | Abstract public-record/trusted-pin tokens | Yes, abstract/supporting only | No CLI app send | No CLI app receive | No | No | Bounded model/corpus stale/replay/corrupt reject evidence | No-secret corpus validation | No for NA-0504 | Yes | Supporting only, not a substitute for E2E |

Inventory result: current qsc surfaces can support a deterministic, hermetic,
same-host Alice/Bob E2E test with two independent client roots using existing
CLI/test-visible patterns. The future test should be integration-test driven
with process/CLI calls and the existing local inbox server. It does not require
qsc source, helper, dependency, Cargo, workflow, script, corpus/vector/input,
formal, refimpl, service, public-doc, or backup mutation.

Candidate future implementation shapes identified:

1. Process/CLI-driven qsc integration test using two temp roots and the local
   inbox server.
2. Smaller process/CLI smoke test using existing send/receive/reply flow if the
   full negative boundary must split.
3. Mock-relay-oriented split scope if a future implementation discovers that
   exact relay fault semantics need separate authorization.
4. Two-root identity/public-record exchange test if send/receive/reply proves
   infeasible during implementation.

## Option review

| Option | Disposition | Risk reduced | Evidence gap addressed | Feasibility | Scope risk | Public-claim risk | External-review value | Future allowed paths | Future forbidden paths | P0/P1/P2 risks |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1. Same-host client-to-client E2E implementation harness | Select | Broad user-realistic Alice/Bob flow | Two roots, identity/trust, send/receive/reply, negative boundary in one visible flow | High with current helpers | Medium, bounded by one new qsc integration test | Medium unless claims are caveated | High | `qsl/qsl-client/qsc/tests/same_host_client_to_client_e2e.rs` plus NA-0504 governance paths | Remote/two-machine, qsc source/helper/dependency/workflow/corpus/formal/refimpl/service/public/backup | P0 overclaim; P1 flaky relay wait; P2 duplicated helpers |
| 2. Same-host CLI smoke implementation | Reject as primary, keep fallback | Basic workflow confidence | Positive send/receive/reply | High | Low | Low | Medium | `qsl/qsl-client/qsc/tests/client_to_client_cli_smoke.rs` plus governance | Same forbidden paths as option 1 | P1 too superficial; P2 misses negative boundary |
| 3. Mock-relay scope split authorization | Defer | Relay semantics ambiguity | Exact mock-relay fault model | High if needed | Low | Low | Medium | Future authorization docs only | Implementation mutation unless separately authorized | P1 unnecessary delay if E2E is ready |
| 4. Two-root identity/public-record exchange implementation | Reject as primary, keep fallback | Identity/trust bootstrap | Public-record/trusted-pin exchange | High | Low | Low | Medium | New qsc test plus governance | Remote/two-machine and source/dependency/workflow mutation | P1 omits message exchange |
| 5. Negative E2E replay/stale/corrupt authorization | Reject as primary, include one negative in option 1 | Attack-path rejection | Stale/replay/corrupt no-mutation | Medium | Medium | Low if caveated | High | Future authorization docs or NA-0504 negative case | Same forbidden paths | P1 misses positive user flow |
| 6. Remote/LAN client-to-client authorization | Defer | Real network/operator realism | Remote host setup and LAN behavior | Not allowed in NA-0503 | High | High | Medium later | Separate future remote directive only | Current lane and NA-0504 | P0 unauthorized host mutation |
| 7. Continue internal lifecycle/mapping tests | Reject | Narrow internal assurance | More lifecycle or formal mapping evidence | High | Low | Low | Medium | Future narrow test lanes if later selected | Displacing ready E2E without blocker | P1 avoids user-realistic gap |
| 8. Process/tooling lane | Reject | CI/tool ergonomics | Tooling friction | High | Low | Low | Low | Tooling paths only if blocker appears | Core assurance displacement | P1 delays security evidence |

## Hostile Cryptographer Review

Same-host E2E meaningfully exercises real qsc protocol behavior beyond isolated
unit/integration invariants only if it uses two independent roots, real qsc CLI
processes, identity setup, trusted public records, the local inbox relay,
message send/receive, reply, and a negative reject/no-mutation boundary in the
same test file. The existing `receive_e2e.rs` positive flow is strong but can be
overread if presented as a complete client-to-client security proof. Existing
handshake, binding negative, lifecycle, diagnostic, and formal tests are direct
for their selected slices but are not substitutes for a user-realistic Alice/Bob
workflow.

A cryptographer would distrust first:

- deterministic seeded session fallback if it is mistaken for production
  randomness evidence;
- same-host local relay behavior if it is claimed as remote/LAN behavior;
- a positive-only send/receive/reply test if no stale/replay/corrupt or wrong
  identity boundary is included;
- output secrecy checks if they scan only success output and not reject output;
- formal/corpus evidence if it is overread as qsc/refimpl equivalence or full
  protocol proof.

The selected scope reduces a product workflow evidence gap more directly than a
formal/refimpl mapping gap. It still helps crypto review by making the actual
qsc client surfaces and state artifacts visible in one hermetic Alice/Bob flow.
The best next evidence is same-host local E2E with explicit claim caveats,
while remote/LAN and release-claim work remain out of scope.

## Red-Team Review

An attacker would target:

- public-record and trusted-pin exchange;
- wrong identity or stale trusted pin;
- stale, replayed, corrupt, duplicated, dropped, or reordered delivery;
- session/message mutation after reject;
- diagnostic/log output that leaks route tokens, secrets, endpoints, or
  operator/user data;
- temp-root artifact retention or cross-root state bleed;
- same-host isolation assumptions.

The first NA-0504 negative case should be a stale/wrong identity or corrupt
delivery reject that proves no message output and no selected session/output
artifact mutation. If implementation risk is lower, reusing the wrong
mailbox/wrong peer receive boundary from `receive_e2e.rs` is acceptable only if
the evidence explicitly records that replay/stale/corrupt coverage remains
separate.

## Production SRE Review

Same-host E2E helps future demo/client confidence because it tests what an
operator would recognize: Alice initializes, Bob initializes, each trusts the
other, Alice sends, Bob receives, Bob replies, and Alice receives. The test
should preserve hermetic local roots, loopback-only relay, deterministic
commands, bounded timeouts, stdout/stderr capture, and artifact snapshots.

Operator assumptions that must not be introduced:

- no remote SSH or remote account setup;
- no two-machine or LAN precondition;
- no production relay/service dependency;
- no live internet endpoint;
- no new dependency/tool installation;
- no branch protection or workflow mutation.

Artifacts/logs/diagnostics must be checked or caveated: qsc stdout/stderr,
plain markers, received files, selected session/output artifacts, relay inbox
behavior, temp-root separation, and secret-like output scans. Remote SSH and
two-machine testing remain deferred because they require separate operator
state, host trust, network assumptions, and public/service claim boundaries.

## Release-Claim Boundary Review

NA-0503 and selected NA-0504 preserve claim discipline:

- no public-readiness claim is made.
- no production-readiness claim is made.
- no public-internet-readiness claim is made.
- no external-review-complete claim is made.
- no crypto-complete claim is made.
- no KEM-complete claim is made.
- no signature-complete claim is made.
- no identity-complete claim is made.
- no provider-RNG-complete claim is made.
- no secret-material-complete claim is made.
- no zeroization-complete claim is made.
- no memory-erasure-complete claim is made.
- no replay-proof claim is made.
- no downgrade-proof claim is made.
- no side-channel-free claim is made.
- no vulnerability-free claim is made.
- no bug-free claim is made.
- no perfect-crypto claim is made.
- Cargo audit green is dependency-health evidence only.
- Formal models remain bounded evidence only.

## Prioritization matrix

| Candidate | Risk reduced | Directness of evidence | User-realism | Attack relevance | External-review value | Implementation feasibility | Scope risk | Overclaim risk | Dependency/workflow risk | Recommended disposition | Next-lane yes/no |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Same-host client-to-client E2E implementation | High | High | High | High | High | High | Medium | Medium | Low | Select | Yes |
| Same-host client-to-client CLI smoke implementation | Medium | Medium | Medium | Medium | Medium | High | Low | Low | Low | Fallback only | No |
| Mock-relay client-to-client split authorization | Medium | Medium | Medium | Medium | Medium | High | Low | Low | Low | Defer | No |
| Two-root identity/public-record exchange implementation | Medium | High for identity only | Medium | High for trust | Medium | High | Low | Low | Low | Fallback only | No |
| Negative E2E replay/stale/corrupt authorization | High for attack reject | Medium | Medium | High | High | Medium | Medium | Low | Low | Include one negative in selected lane | No |
| Remote/LAN client-to-client authorization | Medium later | Low for current lane | High | Medium | Medium | Low now | High | High | Medium | Defer | No |
| Continue internal lifecycle/mapping tests | Medium narrow | High for narrow slices | Low | Medium | Medium | High | Low | Low | Low | Reject for now | No |
| Process/tooling lane | Low for core assurance | Low | Low | Low | Low | High | Low | Low | Low | Reject | No |

## Authorization decision

Primary classification:
`SAME_HOST_CLIENT_TO_CLIENT_E2E_IMPLEMENTATION_READY`.

Reasons:

- NA-0502/D388 selected this as the next broad, user-realistic assurance lane.
- `receive_e2e.rs` already demonstrates two separate config roots, local inbox
  relay, send, receive, reply, received payload assertions, marker checks, and
  no selected secret output.
- `handshake_mvp.rs`, `identity_binding.rs`, `identity_ux.rs`, and
  `trust_onboarding_mainstream_flow_na0187.rs` provide existing identity,
  public-record, trusted-pin, route-token, trust-policy, and mismatch patterns.
- `handshake_provider_error_no_mutation.rs`,
  `kem_signature_transcript_binding_negative.rs`, and relay no-mutation tests
  provide existing negative reject/no-mutation patterns that can be reused as
  implementation references.
- `common/mod.rs` already provides process invocation, temp isolation, local
  inbox server, route-token channeling, stdout/stderr capture, and helper env
  patterns.
- No qsc source/helper/dependency/Cargo/workflow/script/corpus/formal/refimpl/
  service/public/backup mutation is required.

## Selected NA-0504 successor

### NA-0504 -- QSL qsc Same-Host Client-to-Client End-to-End Test Implementation Harness
Status: READY
Goals: G1, G2, G3, G4, G5

Objective:
Implement a bounded same-host qsc client-to-client E2E integration test using
two independent temp client roots and realistic Alice/Bob behavior, including
identity setup, public-record/trusted-pin exchange, message exchange, reply,
and at least one negative reject/no-mutation boundary, while preserving all
public/security/completion no-claim boundaries recorded by D-0995.

## Future scope bundle

Allowed scope for NA-0504:

- `qsl/qsl-client/qsc/tests/same_host_client_to_client_e2e.rs`
- `docs/governance/evidence/NA-0504_qsl_qsc_same_host_client_to_client_e2e_test_implementation_harness.md`
- `tests/NA-0504_qsl_qsc_same_host_client_to_client_e2e_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden scope for NA-0504:

- remote SSH or two-machine setup.
- qsc source mutation.
- qsc fuzz target or Cargo mutation.
- corpus/vector/input mutation.
- workflow/script/helper mutation.
- dependency/lockfile mutation.
- refimpl/formal/service/public/qshield/qsl-server/qsl-attachments mutation.
- backup/restore/qsl-backup mutation.
- any claim forbidden by the Release-Claim Boundary Review above.

Deliverables:

- qsc integration test.
- evidence doc.
- testplan.
- decision.
- TRACEABILITY update.
- rolling journal update.

Acceptance criteria:

- two independent client roots used.
- realistic Alice/Bob identity setup and trust/public-record exchange tested.
- Alice-to-Bob send/receive tested.
- Bob-to-Alice reply tested.
- at least one negative reject/no-mutation boundary included, or a concrete
  implementation-time defect forces STOP before weakening the scope.
- stdout/stderr/diagnostics are captured and scanned for selected secret-like
  markers.
- no qsc source/helper/dependency/workflow/Cargo/corpus/formal/refimpl/service/
  public/backup mutation.
- no public/production/security-completion overclaim.
- exactly one READY item remains after closeout.

## Future validation / marker plan

Common future markers for NA-0504:

- `NA0504_CLIENT_TO_CLIENT_SCOPE_CONSUMED_OK`
- `NA0504_TWO_INDEPENDENT_CLIENT_ROOTS_OK`
- `NA0504_ALICE_BOB_IDENTITY_SETUP_OK`
- `NA0504_PUBLIC_RECORD_TRUST_EXCHANGE_OK`
- `NA0504_SEND_RECEIVE_FLOW_OK`
- `NA0504_REPLY_FLOW_OK`
- `NA0504_NEGATIVE_REJECT_NO_MUTATION_OK`
- `NA0504_STDOUT_STDERR_NO_SECRET_OUTPUT_OK`
- `NA0504_NO_REMOTE_SSH_SCOPE_OK`
- `NA0504_NO_QSC_SOURCE_CHANGE_OK`
- `NA0504_NO_DEPENDENCY_CHANGE_OK`
- `NA0504_NO_WORKFLOW_CHANGE_OK`
- `NA0504_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0504_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0504_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0504_NO_REPLAY_PROOF_CLAIM_OK`
- `NA0504_NO_DOWNGRADE_PROOF_CLAIM_OK`
- `NA0504_ONE_READY_INVARIANT_OK`

Suggested future validation commands:

```bash
cargo test -p qsc --locked --test same_host_client_to_client_e2e -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test receive_e2e -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
python3 formal/run_model_checks.py
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

## Remote/LAN deferral

Remote SSH, two-machine, LAN, public-internet, and production relay testing are
deferred. They require separate operator setup, host identity/trust decisions,
network assumptions, remote account handling, and service/public claim
boundaries. They are not needed to prove the next same-host client-to-client
assurance step.

## Public claim / website / external review boundary

NA-0503 does not authorize website, public-doc, README, START_HERE, public
technical paper, branch-protection, public-safety workflow, external review
work, release-claim expansion, or security/completion claim expansion. The
explicit no-claim boundary is the Release-Claim Boundary Review above.

## Rejected alternatives

- Remote/LAN client-to-client scope is rejected for this lane because it is
  explicitly outside NA-0503 and would require operator/host/network setup.
- Continuing internal lifecycle/mapping tests is rejected as the immediate next
  lane because no blocker prevents a same-host E2E implementation.
- A process/tooling lane is rejected because current tooling is sufficient for
  this authorization and no CI/tool blocker displaces core assurance.
- A positive-only CLI smoke test is rejected as primary because it would leave
  the requested negative reject/no-mutation boundary out of the first E2E
  implementation lane.
- A two-root identity-only test is rejected as primary because current surfaces
  support send/receive/reply without forbidden mutation.
- A mock-relay split authorization is deferred unless NA-0504 implementation
  finds a concrete relay-scope ambiguity.

## Backup-impact statement

No backup, restore, qsl-backup, backup plan/status, rollback, archive, or
`/backup/qsl` mutation is performed or authorized. qsl-backup was read only for
the required digest and source-list boundary proof.

## Next recommendation

Proceed to NA-0504 as a bounded qsc integration-test implementation harness
using current CLI/test-visible surfaces and the existing local inbox server.
Keep it same-host only, use two independent temp roots, include one negative
reject/no-mutation boundary, and preserve all claim boundaries above.
