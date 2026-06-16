Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-16

# NA-0489 Binding Fuzz Ephemeral Seed Strategy Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0489 consumes NA-0488/D351 and implements deterministic ephemeral seed
recipes inside `qsc_binding_semantics`.

The implementation remains target-local. It adds no checked-in corpus, reads no
manifest JSON at runtime, performs no file IO, adds no dependency, mutates no
Cargo or lockfile, and does not mutate qsc source/helpers, qsc-adversarial
scripts, workflows, vectors, inputs, formal models, refimpl, services, public
docs, backups, or qwork tooling.

The selected successor is:

`NA-0490 -- QSL Binding Fuzz Corpus Secret-Material Validator Authorization Plan`

## Live NA-0489 scope

Startup READY item:

`NA-0489 -- QSL Binding Fuzz Ephemeral Seed Strategy Implementation Harness`

Allowed implementation mutation used:

- `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs`

Allowed governance mutation used:

- this evidence doc
- `tests/NA-0489_qsl_binding_fuzz_ephemeral_seed_strategy_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No optional closeout mutation is included in this implementation evidence.

## qwork proof-file verification

Codex read the operator-provided qwork proof files and did not run qwork,
qstart, or qresume:

- `/srv/qbuild/work/NA-0489/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0489/.qwork/startup.qsl-protocol.json`

Required qwork fields matched:

- `startup_result=OK`
- `lane=NA-0489`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0489/qsl-protocol`
- `head_equals_origin_main=yes`
- clean worktree, index, and untracked state
- `ready_count=1`
- `queue_top_ready=NA-0489`
- `requested_lane_status=READY`

Freshness proof:

- proof HEAD and live pre-fetch HEAD both matched `fdc67c8b8526`
- proof `origin/main` and live pre-fetch `origin/main` both matched
  `fdc67c8b8526`
- fetch was performed only after proof/live refs matched
- `origin/main` equals or descends from `fdc67c8b8526`
- PR #1248 was verified merged with merge commit `fdc67c8b8526`

Startup queue and decision proof:

- READY_COUNT 1
- READY NA-0489
- NA-0488 DONE
- NA-0487 DONE
- D-0965 exists once
- D-0966 exists once
- D-0967 absent before this patch
- duplicate decision count 0

Startup health proof:

- public-safety on `fdc67c8b8526`: success
- qsc-adversarial-smoke on `fdc67c8b8526`: success
- root cargo audit: PASS
- nested qsc fuzz lock audit: PASS
- `rustls-webpki` and `ml-kem` inverse dependency probes: present
- installed qsl-backup SHA256 matched the expected digest
- installed qsl-backup source list includes the Codex ops source exactly once

## NA-0488 / D351 inheritance

Consumed inheritance sources:

- D351 response:
  `/home/victor/work/qsl/codex/responses/NA0488_20260616T191626Z_D351.md`
- NA-0488 evidence doc:
  `docs/governance/evidence/NA-0488_qsl_binding_fuzz_corpus_seed_strategy_authorization_plan.md`
- NA-0488 testplan:
  `tests/NA-0488_qsl_binding_fuzz_corpus_seed_strategy_authorization_testplan.md`
- D-0965 and D-0966 in `DECISIONS.md`
- NA-0489 block in `NEXT_ACTIONS.md`

Inherited facts:

- PR #1247 authorization merged at `84dae09ba17e`.
- PR #1248 closeout merged at `fdc67c8b8526`.
- `BINDING_FUZZ_EPHEMERAL_SEED_STRATEGY_READY` was selected.
- No checked-in binding corpus was selected.
- The NA-0483 manifest remains traceability-only.
- Runtime JSON manifest reading was rejected.
- Likely target-only mutation was selected.
- Forbidden split conditions include qsc source/helper mutation, Cargo/script
  mutation, workflow mutation, dependency or lockfile mutation, vector/input or
  corpus mutation, runtime manifest consumption, formal/refimpl/service/public
  mutation, backup mutation, or public-claim expansion.
- NA-0489 objective is deterministic ephemeral seed recipes in
  `qsc_binding_semantics`, with no checked-in corpus.
- D351 wait accounting recorded 10.99 CI minutes, 10.99 productive CI minutes,
  0.00 idle CI minutes, 9.76 local long-command minutes, and 0.00 potentially
  wasted wait minutes.

## Read-only SSD disk pressure inventory

Read-only inventory was collected under the NA-0489 proof root.

Root filesystem:

- `/` was 92% used, with 37G available.
- `/backup/qsl` was 6% used.
- The hard STOP threshold of 95% was not hit.

Largest read-only candidates:

- `/srv/qbuild/work`: 335G total; older lane workspaces dominate.
- `/srv/qbuild/work/NA-0380`: 12G.
- `/srv/qbuild/work/NA-0487`: 6.1G.
- `/srv/qbuild/tmp`: 17G total.
- `/srv/qbuild/tmp/NA0482_binding_negative_vector_scope_20260615T060409Z`: 5.4G.
- `/srv/qbuild/tmp/NA0467_closeout_restore_na0468_20260613T004103Z`: 3.7G.
- `/home/victor/.cargo`: 831M.
- `/tmp`: 34M.

Disk pressure is recorded as a P1 local-ops residual. A dedicated archival lane
should inspect nightly/local-ops scripts before moving files, because those
scripts reference qbuild work/tmp roots, response paths, and backup paths.

## Pre-mutation review

Preimage copied:

- `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs`

Pre-mutation state:

- worktree was clean
- no tracked diff
- no untracked files
- target contained NA-0487 markers and direct arbitrary-byte category routing
- target had a special `0xff` vector-traceability selector
- target did not contain NA-0489 seed recipes
- `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/` was absent

## Ephemeral seed strategy implementation

The target now runs two bounded exercises per fuzz input:

1. The inherited arbitrary-input path remains in place, preserving existing
   fuzzer-driven coverage.
2. A deterministic ephemeral seed recipe is generated from the same input and
   routed to the selected category.

All generated bytes are public/synthetic and process-local. The recipes use
static byte labels, public handshake framing constants, public provider byte
lengths, fuzzer input bytes, and deterministic xor/mutation operations. They do
not use private keys, KEM secret keys, signing keys, passphrases, runtime keys,
backup keys, operator data, user data, live service data, or private endpoint
material.

Required NA-0489 markers are present in the fuzz target:

- `NA0489_CORPUS_STRATEGY_CONSUMED_OK`
- `NA0489_EPHEMERAL_SEED_GENERATION_ONLY_OK`
- `NA0489_NO_CHECKED_IN_CORPUS_OK`
- `NA0489_VECTOR_MANIFEST_TRACEABILITY_ONLY_OK`
- `NA0489_NO_SECRET_MATERIAL_IN_CORPUS_OK`
- `NA0489_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0489_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0489_NO_FUZZ_COMPLETE_CLAIM_OK`
- `NA0489_NO_CORPUS_COMPLETE_CLAIM_OK`
- `NA0489_NO_VECTOR_COMPLETE_CLAIM_OK`
- `NA0489_NO_REPLAY_PROOF_CLAIM_OK`
- `NA0489_NO_DOWNGRADE_PROOF_CLAIM_OK`

## Category coverage proof

Seed category selection is deterministic and fuzzer-input-driven:

- selector 0: A1 mutation seed recipe
- selector 1: B1 mutation seed recipe
- selector 2: A2 mutation seed recipe
- selector 3: suite-confusion seed recipe
- selector 4: replay seed recipe
- selector 5: stale public-record / trusted-pin seed recipe
- selector 6: vector-manifest traceability seed recipe

Recipe mapping:

- A1 mutation builds a synthetic Suite-2 A1-shaped frame and mutates payload
  bytes in the public KEM binding area.
- B1 mutation builds a synthetic Suite-2 B1-shaped frame and mutates payload
  bytes in the public KEM ciphertext area.
- A2 mutation builds a synthetic Suite-2 A2-shaped frame and mutates payload
  bytes in the signature/transcript binding area.
- Suite confusion builds an A1-shaped frame with deterministic wrong suite
  tuple variants.
- Replay builds a Suite-2 A1-shaped frame suitable for the helper replay
  classifier.
- Stale public-record / trusted-pin emits deterministic bytes whose first byte
  selects the helper reject branch.
- Vector-manifest traceability emits static NA-0483 category names as
  traceability labels only.

## Vector manifest traceability proof

The target includes static comments/category names mapping to NA-0483 manifest
classes:

- `kem_wrong_peer_public_key`
- `kem_wrong_ciphertext`
- `signature_wrong_identity_public_record`
- `signature_cross_message_replay_b1_as_a2`
- `transcript_mutation`
- `suite_confusion_wrong_suite_token`
- `stale_public_record_replay`
- `formal_token_mapping`

This is traceability-only. The target does not read the manifest JSON at
runtime, does not call file IO, and does not use `include_bytes!` or
`include_str!`.

## No checked-in corpus proof

No directory or file was added under:

- `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/`
- `qsl/qsl-client/qsc/fuzz/corpus/`

No `inputs/` path changed. No
`inputs/suite2/internal_negative_binding_vectors/` path changed.

## No secret/private material proof

Changed target lines contain only:

- public string markers
- public category names
- public wire constants
- public provider byte-length constants
- deterministic synthetic byte-generation logic

They contain no checked-in private key, no KEM secret key, no signing key, no
passphrase, no runtime key, no backup key, no operator data, no user data, no
live service data, no private endpoint, no PEM private header, and no
production-like secret string.

## No manifest runtime consumption proof

The target has no runtime manifest path reference, no JSON parser, no
`std::fs`, no `File::`, no `include_bytes!`, and no `include_str!`.

Manifest category names are static traceability labels only.

## No qsc source / Cargo / script / workflow mutation proof

Implementation mutation is limited to:

- `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs`

No qsc source/helper file changed. No qsc fuzz Cargo file changed. No qsc fuzz
`Cargo.lock` changed. No root `Cargo.lock` changed. No qsc-adversarial script
changed. No workflow changed. No dependency changed. No vector/input/corpus,
formal, refimpl, service, public-doc, backup, qsl-backup, or qwork tooling path
changed.

## Validation

Target-local validation:

- `cargo fmt --check`: PASS
- `RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo check --manifest-path qsl/qsl-client/qsc/fuzz/Cargo.toml --locked --bin qsc_binding_semantics`: PASS

Full local validation:

- `git diff --check`: PASS
- exact six-path scope guard: PASS
- link-check: `TOTAL_MISSING 0`
- leak-scan: `SECRET_FINDING_COUNT 0`
- added-line overclaim scan: `OVERCLAIM_FINDING_COUNT 0`
- classifier: `runtime_critical`, with no workflow-security path
- queue/decision proof: READY_COUNT 1; READY NA-0489; D-0965 once;
  D-0966 once; D-0967 once; D-0968 absent; duplicate decision count 0
- internal negative vector manifest JSON validation: PASS
- formal binding model: PASS
- formal model runner: PASS
- qsc binding negative test, no cfg: PASS
- qsc binding negative test, `qsc_binding_fuzz_helper` cfg: PASS
- refimpl signature provider-boundary test: PASS
- qsc inherited RNG/provider transactionality tests, cfg and no-cfg variants:
  PASS
- qsc key lifecycle zeroization: PASS
- qsc provider-error no-mutation: PASS
- stable qsc `send_commit`: PASS
- refimpl `pqkem768`: PASS
- root cargo audit: PASS
- nested qsc fuzz lock audit: PASS
- `rustls-webpki` and `ml-kem` dependency probes: recorded
- qsc-adversarial shell syntax: PASS
- local qsc-adversarial stable phases: PASS before local cargo-fuzz caveat

Local cargo-fuzz caveat:

- failing command: `RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo fuzz run qsc_binding_semantics -- -runs=1`
- classification: recoverable local tool availability caveat
- exact output included `error: no such command: \`fuzz\``
- corrective action: record exact output and require PR CI qsc-adversarial-smoke
  for cargo-fuzz-backed evidence
- final local result: non-fuzz local adversarial phases PASS; cargo-fuzz-backed
  evidence deferred to PR CI

Cargo audit green remains dependency-health evidence only.

## Scope guard

Expected implementation PR changed paths:

- `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs`
- `docs/governance/evidence/NA-0489_qsl_binding_fuzz_ephemeral_seed_strategy_implementation_harness.md`
- `tests/NA-0489_qsl_binding_fuzz_ephemeral_seed_strategy_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Any other changed path is out of scope for this implementation evidence lane.

## Backup-impact statement

Backup impact: none. Codex did not run backup or restore and did not mutate
qsl-backup, backup status, backup plan, rollback paths, or backup trees.
qsl-backup boundary was read-only and intact at startup.

## Successor selection

Selected successor after successful NA-0489:

`NA-0490 -- QSL Binding Fuzz Corpus Secret-Material Validator Authorization Plan`

Rationale:

- NA-0488 rejected checked-in corpus for now.
- Future checked-in binding corpus requires a validator-first lane.
- NA-0489 implemented ephemeral seeds only and did not reveal a higher-priority
  target-stability or secret-material blocker.

NA-0490 must not be implemented by this evidence lane.

## Applicable stewardship and assurance review

Level-1 stewardship conclusion:

- Deterministic ephemeral seed generation is the narrowest safe implementation.
- The Lead Director authority and exactly-one-READY invariant remain unchanged.

Best-Known-Method Review:

- Keep generated data hermetic, deterministic, synthetic, and target-local.
- Do not add checked-in corpus until a validator lane exists.

Hostile Cryptographer Review:

- The recipes are not public vectors and make no conformance claim.
- No secret/private material is embedded.
- Runtime manifest IO is avoided.

Red-Team Review:

- There is no file path dependency, corpus material, or runtime JSON read.
- The raw arbitrary-byte path remains active to avoid narrowing fuzz input.

Production SRE Review:

- No workflow, script, dependency, Cargo, lockfile, or service mutation is
  needed.
- Local disk pressure is a P1 operations residual, not an implementation input.

Side-Channel Caveat:

- This lane makes no side-channel-free claim and does not replace later
  side-channel or secret-material assurance.

Formal-Model Mapping Residual:

- Static category names map back to NA-0483/NA-0478 evidence only as supporting
  traceability.
- This lane makes no formal-proof-complete claim.

External-Review Readiness:

- This improves internal review readiness but makes no external-review-complete
  claim.

Release-Claim Boundary:

- This lane makes no public-readiness claim, no production-readiness claim, no
  crypto-complete claim, no fuzz-complete claim, no corpus-complete claim, no
  vector-complete claim, no replay-proof claim, no downgrade-proof claim, no
  side-channel-free claim, no vulnerability-free claim, and no perfect-crypto
  claim.

Assurance Gap Review Trigger:

- STOP or split if future work requires qsc source/helper mutation, Cargo or
  script mutation, workflow mutation, dependency or lockfile mutation, checked-in
  corpus, vector/input mutation, runtime manifest consumption, formal/refimpl
  mutation, service/public/backup mutation, or public-claim expansion.

## Next recommendation

After implementation merge and post-merge public-safety plus
qsc-adversarial-smoke are green, close out NA-0489 and restore the selected
NA-0490 validator authorization plan. Do not implement NA-0490 in this lane.
