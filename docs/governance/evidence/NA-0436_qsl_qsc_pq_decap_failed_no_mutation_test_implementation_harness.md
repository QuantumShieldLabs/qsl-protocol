Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-06

# NA-0436 qsc pq_decap_failed No-Mutation Test Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0436 implements the NA-0435-authorized narrowed qsc integration test for
`pq_decap_failed` no-mutation behavior at:

`qsl/qsl-client/qsc/tests/handshake_provider_error_no_mutation.rs`

The test uses existing qsc CLI/test APIs, the existing mock relay, and the
existing encrypted mock-vault fixture pattern. It builds a normal A1/B1 exchange,
corrupts only Alice's test-local pending KEM secret in the mock vault, polls the
valid B1, observes `pq_decap_failed`, and proves session/pending state is not
mutated by the reject path.

Selected successor after merge and closeout:

`NA-0437 -- QSL qsc pq_encap_failed Defensive Branch Documentation / Evidence Plan`

## Live NA-0436 scope

Live `NEXT_ACTIONS.md` lists:

`NA-0436 -- QSL qsc pq_decap_failed No-Mutation Test Implementation Harness`

Status: READY.

Allowed implementation mutation path:

- `qsl/qsl-client/qsc/tests/handshake_provider_error_no_mutation.rs`

Allowed governance paths:

- `docs/governance/evidence/NA-0436_qsl_qsc_pq_decap_failed_no_mutation_test_implementation_harness.md`
- `tests/NA-0436_qsl_qsc_pq_decap_failed_no_mutation_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden mutation scope preserved: no runtime, crypto, dependency, Cargo,
lockfile, workflow, fuzz target, vector, qsl-server, qsl-attachments, qshield
runtime, website, public-doc, README, START_HERE, qwork/qstart/qresume/qshell,
backup, qsl-backup, backup status, backup plan, rollback, or `/backup/qsl`
mutation.

## qwork proof-file verification

Codex read, but did not run, qwork proof files:

- `/srv/qbuild/work/NA-0436/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0436/.qwork/startup.qsl-protocol.json`

Required proof markers were present:

- `startup_result=OK`
- `lane=NA-0436`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0436/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0436`
- `requested_lane_status=READY`

The JSON proof parsed successfully and mirrored the `.kv` proof for lane, repo,
path, head, origin/main, clean-state fields, READY count, queue top, and
requested lane status.

After `git fetch --all --prune`, live `HEAD` and `origin/main` still matched
the proof at `575c250cc24d`. PR #1140 was verified MERGED at
`575c250cc24d`.

Proof root:

`/srv/qbuild/tmp/NA0436_pq_decap_failed_no_mutation_test_impl_20260607T045500Z`

## NA-0435 authorization inheritance

NA-0435 selected:

`NARROW_DECAP_ONLY_TEST_AUTHORIZATION_READY`

Inherited constraints:

- implement only `pq_decap_failed` executable no-mutation coverage;
- preserve the `pq_encap_failed` caveat;
- do not claim executable coverage for `pq_encap_failed`;
- use only existing qsc APIs and test fixtures;
- do not add runtime hooks, provider fakes, provider trait seams, crypto changes,
  dependency changes, Cargo changes, lockfile changes, workflow changes, fuzz
  target changes, vector changes, public-surface changes, service changes, or
  backup/local-ops changes.

Inherited feasibility evidence: D278 showed malformed pending KEM secret
evidence can make provider decapsulation fail, while `pq_encap_failed` could not
be externally triggered under the current active provider and qsc external APIs.

## Pre-mutation test path review

The authorized test file was absent before mutation:

`AUTHORIZED_TEST_FILE_ABSENT_PREIMAGE_OK`

Rollback marker:

`$PROOF_DIR/rollback/handshake_provider_error_no_mutation.rs.absent`

Relevant source facts:

- `qsl/qsl-client/qsc/src/handshake/mod.rs` emits `pq_decap_failed` in the
  initiator pending path when `StdCrypto.decap(&pending.kem_sk, &resp.kem_ct)`
  fails.
- The decap failure branch emits `handshake_reject reason=pq_decap_failed` and
  returns before `qsp_session_store` and before pending clear.
- Pending handshake state is stored through the encrypted vault secret key
  `handshake.pending.<self>.<peer>`.
- qsc integration tests already use mock vault initialization, encrypted
  mock-vault JSON inspection, session-path inspection, and mock relay frame
  replacement.

Feasibility conclusion: exact narrowed `pq_decap_failed` testing was feasible
without runtime hooks.

## Implemented `pq_decap_failed` no-mutation test

Test:

`pq_decap_failed_reject_does_not_mutate_sessions_or_pending_state`

Implementation shape:

1. Create isolated Alice/Bob qsc configs.
2. Initialize mock passphrase vaults through existing test helpers.
3. Create authenticated contacts and relay inbox tokens through the qsc CLI.
4. Run Alice `handshake init` in `suite-required` mode.
5. Run Bob `handshake poll` to generate a structurally valid B1.
6. Decrypt Alice's mock-vault JSON in test-local state and set the pending
   `kem_sk` to a malformed one-byte array.
7. Snapshot Alice and Bob session/pending/vault state.
8. Run Alice `handshake poll` against the valid B1.
9. Assert `pq_decap_failed` is emitted.
10. Assert no session store or pending/vault state mutation occurred.

## `pq_decap_failed` marker proof

The exact test passed and emitted:

```text
NA0436_PQ_DECAP_FAILED_MARKER_OK
NA0436_NO_SESSION_MUTATION_ON_DECAP_REJECT_OK
NA0436_PENDING_STORE_NO_MUTATION_ON_DECAP_REJECT_OK
NA0436_PQ_ENCAP_FAILED_CAVEAT_PRESERVED_OK
NA0436_NO_RUNTIME_HOOK_USED_OK
```

The qsc output assertion requires `event=handshake_reject` and
`pq_decap_failed`, and rejects any `event=handshake_complete`,
`event=recv_commit`, or `event=qsp_unpack ok=true` output.

## State snapshot / no-mutation assertion proof

The test captures pre-reject state after the pending KEM secret is deliberately
malformed:

- Alice `qsp_sessions/bob.qsv`: absent before and after reject.
- Bob `qsp_sessions/alice.qsv`: absent before and after Alice reject.
- Alice pending secret `handshake.pending.alice.bob`: equal before and after
  reject.
- Bob responder pending secret `handshake.pending.bob.alice`: equal before and
  after Alice reject.
- Alice vault bytes: equal before and after reject.
- Bob vault bytes: equal before and after Alice reject.
- Bob relay channel: no A2 emitted after reject.

## `pq_encap_failed` caveat preservation proof

The test file includes an explicit decap-only caveat and does not assert
coverage for `pq_encap_failed`.

The inherited caveat remains: `pq_encap_failed` is a defensive branch under the
current active provider and qsc external API behavior. D278 showed wrong-length
A1 KEM public keys fail frame decode before provider encapsulation and
correct-length malformed public-key byte patterns did not make the active
provider's encapsulation fail.

## Existing API / no runtime hook proof

The implementation uses:

- `common::qsc_std_command()`;
- `common::start_inbox_server()`;
- qsc CLI commands for identity, contacts, relay inbox setup, and handshake
  init/poll;
- test-local encrypted mock-vault JSON read/write using the existing envelope
  format already decoded by neighboring qsc integration tests.

It does not add provider fakes, provider mocks, runtime hooks, trait seams,
runtime code, crypto code, dependency changes, Cargo changes, lockfile changes,
workflow changes, fuzz target changes, or vector changes.

## Test validation proof

Initial exact test validation passed:

`cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture`

Required full validation for the PR includes:

- exact new qsc test;
- qsc `send_commit`;
- provider `pqkem768`;
- root and nested cargo audits;
- root dependency tree probes;
- `cargo fmt --check`;
- formal model checks;
- qsc adversarial smoke locally if feasible, otherwise PR CI
  `qsc-adversarial-smoke`.

## Root and nested dependency health proof

Start-gate dependency health passed:

- `cargo audit --deny warnings`
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo tree -i rustls-webpki --locked` reported `rustls-webpki v0.103.13`.
- `cargo tree -i ml-kem --locked` reported `ml-kem v0.2.1`.
- root pqcrypto inverse-tree probes reported package-ID absence for
  `pqcrypto-mlkem`, `pqcrypto-traits`, and `pqcrypto-internals`.
- nested qsc fuzz lock pqcrypto residual scan returned zero matches.

Cargo audit green is dependency-health evidence only.

## No runtime / crypto / dependency / workflow / vector mutation proof

Implementation mutation is limited to:

- `qsl/qsl-client/qsc/tests/handshake_provider_error_no_mutation.rs`

Governance mutation is limited to the NA-0436 allowed evidence, testplan,
decision, traceability, and rolling journal paths.

No runtime, crypto, dependency, Cargo, lockfile, workflow, fuzz target, vector,
qsl-server, qsl-attachments, qshield runtime, website, public-doc, README,
START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status, backup plan,
rollback, or `/backup/qsl` path is mutated.

## Public claim/external review/website boundary

This is bounded internal qsc evidence only.

- It is not production readiness.
- It is not public-internet readiness.
- It is not external-review completion.
- It is not crypto-complete proof.
- It is not side-channel-free proof.
- It is not bug-free proof.
- It is not vulnerability-free proof.
- It is not perfect-crypto proof.

No README, START_HERE, public docs, website, or public technical paper content
is changed.

## Rejected alternatives

- Force `pq_encap_failed` in this test: rejected because NA-0435 preserved the
  D278 caveat and did not authorize hooks or provider fakes.
- Add a runtime provider seam: rejected as out of scope.
- Change crypto/provider behavior to force failure: rejected as out of scope.
- Change dependencies, Cargo files, workflows, fuzz targets, or vectors:
  rejected as out of scope.
- Weaken the assertion to any handshake reject: rejected because NA-0436
  requires the exact `pq_decap_failed` marker.

## Backup-impact statement

Backup impact: none. Codex did not run backup, restore, or sudo. Codex did not
mutate qsl-backup, `/backup/qsl`, backup status files, backup plan files,
rollback subtree paths, timers, fstab, source lists, retention, or backup
scripts.

qsl-backup preflight remained unchanged:

- installed qsl-backup SHA `e9ecff3d22ed`;
- codex ops source-list inclusion count exactly 1.

## Selected successor

Normal successor selected:

`NA-0437 -- QSL qsc pq_encap_failed Defensive Branch Documentation / Evidence Plan`

NA-0437 is not implemented by this PR. It should document the
`pq_encap_failed` defensive-branch status from D278/NA-0435/NA-0436 evidence
without executable coverage overclaim and without runtime, crypto, dependency,
workflow, test, fuzz, vector, public, service, or backup mutation unless future
exact scope authorizes otherwise.

## Next recommendation

After PR merge and post-merge public-safety, execute the optional NA-0436
closeout to mark NA-0436 DONE and restore NA-0437 as the sole READY successor.
