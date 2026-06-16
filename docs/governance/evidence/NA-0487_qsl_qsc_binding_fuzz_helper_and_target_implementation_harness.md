Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-16

# NA-0487 qsc Binding Fuzz Helper and Target Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0487 implements the recovered qsc binding fuzz helper and semantic fuzz
target selected by D-0962 after the D347 source-boundary stop. The helper is
visible only when Rust is compiled with exact cfg `qsc_binding_fuzz_helper`.
Normal no-cfg qsc builds do not export the helper and do not need it.

The implementation adds a bounded helper facade under `qsc::adversarial`, a
semantic binding fuzz target, one qsc fuzz Cargo bin entry, and qsc-adversarial
script inclusion. The cfg-on qsc handshake path routes suite-context parsing,
frame-header parsing, A1 replay candidate matching, and stale/trusted-pin
comparison through the helper-facing routines so the target is tied to real qsc
reject-path code rather than a target-local fake oracle.

This lane adds no dependency, lockfile, workflow, vector/input, corpus, formal,
refimpl, service, public-doc, backup, restore, or qsl-backup mutation. It makes
no directive-forbidden readiness, completion, proof, side-channel, or
vulnerability claim.

## Live NA-0487 scope

Startup READY item:

`NA-0487 -- QSL qsc Binding Fuzz Helper and Target Implementation Harness`

Allowed implementation paths used:

- `qsl/qsl-client/qsc/src/lib.rs`
- `qsl/qsl-client/qsc/src/adversarial/mod.rs`
- `qsl/qsl-client/qsc/src/adversarial/binding_fuzz.rs`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs`
- `qsl/qsl-client/qsc/fuzz/Cargo.toml`
- `scripts/ci/qsc_adversarial.sh`

Allowed governance paths used:

- this evidence doc
- `tests/NA-0487_qsl_qsc_binding_fuzz_helper_and_target_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## qwork proof-file verification

Codex read the operator-provided qwork proof files and did not run qwork,
qstart, or qresume.

Verified proof files:

- `/srv/qbuild/work/NA-0487/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0487/.qwork/startup.qsl-protocol.json`

Required fields matched:

- `startup_result=OK`
- `lane=NA-0487`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0487/qsl-protocol`
- `head_equals_origin_main=yes`
- clean worktree, index, and untracked state
- `ready_count=1`
- `queue_top_ready=NA-0487`
- `requested_lane_status=READY`

Freshness proof:

- proof HEAD and live pre-fetch HEAD both matched `4f0f56df1f55`
- proof `origin/main` and live pre-fetch `origin/main` both matched
  `4f0f56df1f55`
- fetch did not advance `origin/main`
- PR #1244 was verified merged with merge commit `4f0f56df1f55`
- `origin/main` equals or descends from `4f0f56df1f55`

Startup queue and decision proof:

- READY_COUNT 1
- READY NA-0487
- NA-0485 DONE
- NA-0486 DONE
- D-0960 exists once by decision ID entry
- D-0961 exists once by decision ID entry
- D-0962 exists once by decision ID entry
- D-0963 absent at startup
- duplicate decision count 0

Startup health proof:

- public-safety on `4f0f56df1f55`: success
- qsc-adversarial-smoke on `4f0f56df1f55`: success
- root `cargo audit --deny warnings`: PASS
- nested qsc fuzz lock audit: PASS
- `rustls-webpki` and `ml-kem` inverse dependency probes: PASS
- optional pqcrypto inverse probes: absent-package output under directive
  `|| true` allowance
- installed qsl-backup digest: matched expected short SHA `e9ecff3d22ed`
- installed qsl-backup source list: Codex ops source appears exactly once

## D347 / D348 recovery inheritance

D347 stop condition:

- D347 stopped before mutation because the helper-only path could not reach
  real qsc semantic handshake reject paths within the then-allowed scope.

Why fake oracle is forbidden:

- A helper that classifies mutated frames without sharing qsc reject-path code
  would test a separate model and could drift from the real qsc implementation.
- Such output would be misleading semantic binding evidence and would dilute
  fail-closed proof.

D348 selected source-boundary recovery:

- D348 selected `SOURCE_BOUNDARY_RECOVERY_MINIMAL_READY`.
- The selected recovery authorizes a narrow cfg-gated qsc library/source
  boundary rather than a target-local oracle or process-heavy harness.

Future paths authorized by D-0962 and consumed here:

- `qsl/qsl-client/qsc/src/lib.rs`
- `qsl/qsl-client/qsc/src/adversarial/mod.rs`
- `qsl/qsl-client/qsc/src/adversarial/binding_fuzz.rs`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs` only for stale-public-record /
  trusted-pin reachability
- `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs`
- `qsl/qsl-client/qsc/fuzz/Cargo.toml`
- `scripts/ci/qsc_adversarial.sh`
- NA-0487 governance evidence/testplan/decision/traceability/journal paths

Exact cfg required:

- `qsc_binding_fuzz_helper`

Still-forbidden paths:

- dependencies and lockfiles
- workflows
- vectors, inputs, and checked-in corpus
- formal and refimpl code
- qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, and
  public-doc paths
- backup, restore, qsl-backup, backup status, backup plan, rollback, qwork,
  qstart, qresume, and qshell mutation

No-cfg proof requirement:

- normal qsc tests must pass without `qsc_binding_fuzz_helper`
- helper module export must be absent unless the cfg is set

Real reject-path proof requirement:

- cfg-on qsc tests must pass while suite-context parsing, frame-header parsing,
  replay candidate matching, and trusted-pin mismatch logic route through the
  helper-facing code used by the fuzz target

## Pre-mutation review

Preimages were copied for existing allowed implementation files:

- `qsl/qsl-client/qsc/src/lib.rs`
- `qsl/qsl-client/qsc/src/adversarial/mod.rs`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/fuzz/Cargo.toml`
- `scripts/ci/qsc_adversarial.sh`

Absent markers were recorded for:

- `qsl/qsl-client/qsc/src/adversarial/binding_fuzz.rs`
- `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs`

Existing export and target style:

- `qsl/qsl-client/qsc/src/lib.rs` exported `adversarial` and `envelope`.
- `qsl/qsl-client/qsc/src/adversarial/mod.rs` exported parser helper modules.
- Existing qsc fuzz targets are small `libfuzzer_sys::fuzz_target!` wrappers.
- Existing qsc fuzz Cargo metadata uses one `[[bin]]` entry per target.
- Existing qsc-adversarial script uses POSIX shell, temp run dirs, and
  `cargo +nightly fuzz run` target calls.

## cfg-gated source-boundary implementation

Source-boundary changes:

- `qsl/qsl-client/qsc/src/lib.rs` records the no-production-behavior marker.
- `qsl/qsl-client/qsc/src/adversarial/mod.rs` exports
  `binding_fuzz` only under `#[cfg(qsc_binding_fuzz_helper)]`.
- `qsl/qsl-client/qsc/src/handshake/mod.rs` delegates cfg-on
  suite-context parsing, frame-header parsing, replay candidate matching, and
  trusted-pin comparison to helper-facing routines.
- `qsl/qsl-client/qsc/src/identity/mod.rs` remains behavior-equivalent; it was
  touched only to keep cfg-on stale-public-record helper validation warning-free.

Normal no-cfg builds do not compile or export the binding fuzz helper module.

## Binding fuzz helper implementation

New helper path:

- `qsl/qsl-client/qsc/src/adversarial/binding_fuzz.rs`

Helper properties:

- compiled only under exact cfg `qsc_binding_fuzz_helper`
- no unsafe code
- no filesystem/process harness behavior
- no secret or private material output
- bounded enum/marker/classification output only
- deterministic category selection support
- shared qsc frame/suite parsing routines used by cfg-on qsc handshake
- replay and stale-public-record/trusted-pin reachability helpers

Required source markers present:

- `NA0487_BINDING_FUZZ_HELPER_SCOPE_CONSUMED_OK`
- `NA0487_HELPER_API_TEST_FUZZ_ONLY_OK`
- `NA0487_HELPER_API_NO_PRODUCTION_BEHAVIOR_CHANGE_OK`
- `NA0487_HELPER_API_NO_SECRET_OUTPUT_OK`
- `NA0487_HELPER_API_REAL_REJECT_PATHS_OK`
- `NA0487_HELPER_API_VECTOR_TRACEABILITY_OK`

## Semantic binding fuzz target implementation

New target path:

- `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs`

Target properties:

- follows existing `libfuzzer_sys::fuzz_target!` style
- builds under `RUSTFLAGS='--cfg qsc_binding_fuzz_helper'`
- derives the semantic category deterministically from input bytes
- calls `qsc::adversarial::binding_fuzz::exercise_binding_fuzz_case`
- does not read the vector manifest at runtime
- does not use checked-in corpus
- does not output secrets
- does not perform filesystem-heavy process harness behavior
- must not panic on arbitrary input

Required target markers present:

- `NA0487_FUZZ_A1_MUTATION_TARGET_OK`
- `NA0487_FUZZ_B1_MUTATION_TARGET_OK`
- `NA0487_FUZZ_A2_MUTATION_TARGET_OK`
- `NA0487_FUZZ_SUITE_CONFUSION_TARGET_OK`
- `NA0487_FUZZ_REPLAY_TARGET_OK`
- `NA0487_FUZZ_STALE_PUBLIC_RECORD_TARGET_OK`
- `NA0487_FUZZ_VECTOR_MANIFEST_TRACEABILITY_OK`
- `NA0487_NO_SECRET_MATERIAL_IN_FUZZ_INPUTS_OK`
- `NA0487_NO_FUZZ_COMPLETE_CLAIM_OK`
- `NA0487_NO_VECTOR_COMPLETE_CLAIM_OK`
- `NA0487_NO_REPLAY_PROOF_CLAIM_OK`
- `NA0487_NO_DOWNGRADE_PROOF_CLAIM_OK`

## qsc fuzz Cargo metadata proof

`qsl/qsl-client/qsc/fuzz/Cargo.toml` adds exactly one `[[bin]]` entry:

- `name = "qsc_binding_semantics"`
- `path = "fuzz_targets/qsc_binding_semantics.rs"`
- `test = false`
- `doc = false`
- `bench = false`

Required evidence marker present:

- `NA0487_EXACT_FUZZ_CARGO_SCOPE_AUTHORIZED_OK`

No dependency entry was added. No lockfile changed.

## qsc-adversarial script inclusion proof

`scripts/ci/qsc_adversarial.sh` now includes:

- `qsc_binding_semantics` in the smoke flow
- target-specific `RUSTFLAGS='--cfg qsc_binding_fuzz_helper'`
- the required marker `NA0487_FUZZ_CI_ADVERSARIAL_TARGET_INCLUDED_OK`

The script preserves:

- existing `qsc_route_http`
- existing `qsc_payload_boundaries`
- existing `qsc_vault_envelope`
- `NA0439_QSC_PROVIDER_ERROR_NO_MUTATION_ADVERSARIAL_STEP`
- `handshake_provider_error_no_mutation`

The script now tolerates a missing seed directory by using an ephemeral empty
run directory. This avoids adding checked-in corpus while preserving existing
seeded target behavior.

## cfg gating / no production behavior drift proof

Source proof:

- `binding_fuzz` is exported only under `#[cfg(qsc_binding_fuzz_helper)]`.
- handshake cfg changes are guarded by `#[cfg(qsc_binding_fuzz_helper)]`.
- no-cfg alternatives preserve the prior code paths.

Validation proof:

- no-cfg binding negative test passed
- cfg-on binding negative test passed
- cfg-on qsc cargo check passed cleanly after warning cleanup
- no production dependency, lockfile, workflow, vector, or corpus file changed

## No secret output / no corpus proof

The helper returns bounded classifications and static reason strings only. It
does not return frame bytes, keys, ciphertexts, signatures, route tokens,
passphrases, vault payloads, or session material.

The fuzz target uses libFuzzer input bytes only and does not persist or read
runtime manifests. No corpus directory or seed file is added.

## Real reject path / vector traceability proof

Real qsc reject-path reachability:

- cfg-on `hs_parse_parameter_block` delegates to helper-facing
  `parse_suite_context`
- cfg-on `hs_decode_header` delegates to helper-facing `decode_header`
- cfg-on responder replay detection delegates to helper-facing
  `replay_candidate_matches_pending_init`
- cfg-on trusted-pin comparison calls helper-facing
  `trusted_pin_matches_seen` while also exercising the canonical identity
  comparison path

Validation:

- cfg-on qsc binding negative tests passed for KEM ciphertext, transcript
  mutation, stale public record, wrong public key, replay, suite confusion,
  wrong signature identity, and cross-message signature replay rejects
- vector manifest traceability remains comment/static metadata only; the target
  does not read manifest JSON at runtime

Residual:

- this is bounded internal engineering evidence and does not claim
  directive-forbidden readiness, completion, proof, side-channel, or
  vulnerability properties

## No dependency / lockfile / workflow mutation proof

No changes were made to:

- root `Cargo.lock`
- qsc fuzz `Cargo.lock`
- root dependencies
- qsc dependencies
- qsc fuzz dependencies
- `.github/workflows/**`

The only Cargo metadata change is the single qsc fuzz bin entry for
`qsc_binding_semantics`.

## Validation

Local validation completed so far:

```bash
cargo fmt --check
cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo check -p qsc --locked
RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo check --manifest-path qsl/qsl-client/qsc/fuzz/Cargo.toml --locked --bin qsc_binding_semantics
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Local cargo-fuzz availability:

- unavailable locally with `error: no such command: fuzz`
- PR CI qsc-adversarial-smoke is required for cargo-fuzz-backed smoke proof

Full validation and PR/post-merge CI evidence are recorded in the response and
proof root for D349.

## Scope guard

Expected changed paths are limited to the allowed NA-0487 implementation and
governance paths. Forbidden mutation classes remain zero:

- dependency mutation
- lockfile mutation
- workflow mutation
- vector/input/corpus mutation
- formal mutation
- refimpl mutation
- service mutation
- public-doc mutation
- backup/qsl-backup mutation

## Backup-impact statement

No backup or restore command was run. qsl-backup, backup status, backup plan,
rollback, and `/backup/qsl` were not mutated. The installed qsl-backup helper
was inspected read-only for digest/source-list proof.

## Stewardship and assurance review

Best-Known-Method Review:

- Use a cfg-only helper module and existing qsc fuzz target style.
- Keep seed/corpus strategy out of this lane.

Hostile Cryptographer Review:

- The helper must not be treated as proof of complete binding security.
- The helper exercises selected qsc reject-path code but does not prove absence
  of downgrade, replay, identity, transcript, KEM, or signature defects.

Red-Team Review:

- Arbitrary fuzz input is bounded to parse/classification paths.
- No secret-bearing runtime state, vault, relay token, or session material is
  emitted.

Production SRE Review:

- No normal-build API exposure and no workflow/dependency churn.
- Local cargo-fuzz absence is explicitly caveated; CI is required for fuzz
  smoke evidence.

Side-Channel Caveat:

- This lane does not establish side-channel-free behavior.

Formal-Model Mapping Residual:

- Formal model checks remain required inherited evidence; the fuzz helper does
  not replace formal verification.

External-Review Readiness:

- This lane improves internal evidence only and does not complete external
  review readiness.

Release-Claim Boundary:

- No public-readiness or production-readiness claim is made.

Assurance Gap Review Trigger:

- NA-0488 should decide corpus/seed strategy after this helper and target land.

## Successor selection

Selected successor after successful NA-0487 closeout:

`NA-0488 -- QSL Binding Fuzz Corpus / Seed Strategy Authorization Plan`

Rationale:

- NA-0487 intentionally adds no checked-in corpus.
- The next strongest residual is whether to add checked-in public/mutated
  corpus, generated seed recipes, metadata-only seed classes, or no corpus.

## Next recommendation

Merge NA-0487 only after local validation and required PR checks pass. If
post-merge public-safety and qsc-adversarial-smoke are green, close out
NA-0487 and restore NA-0488 as the corpus/seed strategy authorization lane.
