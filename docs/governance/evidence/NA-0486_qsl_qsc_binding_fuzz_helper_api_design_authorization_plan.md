Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-15

# NA-0486 QSL qsc Binding Fuzz Helper API Design Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0486 consumes NA-0485 and records the helper/API design authorization for
semantic qsc binding fuzz.

Primary decision:

- NA-0485 is consumed.
- qwork proof files were read and copied; Codex did not run qwork, qstart, or
  qresume.
- Existing qsc fuzz targets remain parser-boundary oriented for binding.
- Existing qsc integration tests prove the target semantic cases through CLI
  and temp-root/process patterns, but those helpers are test-local and are not
  reusable from a fuzz target without source/test mutation.
- A process harness is safer for internals exposure but too heavy and flaky for
  the first libFuzzer-oriented binding target.
- A target-local simplified oracle is rejected as primary evidence because it
  would not exercise real qsc binding rejection paths.
- The selected future implementation shape is a cfg-gated adversarial helper
  module plus a semantic fuzz target plus qsc fuzz Cargo metadata plus
  qsc-adversarial script inclusion.
- Primary classification:
  `QSC_BINDING_FUZZ_HELPER_PLUS_TARGET_PLUS_CARGO_PLUS_SCRIPT_IMPLEMENTATION_READY`.
- Helper design safety classification:
  `HELPER_DESIGN_SAFE_TEST_FUZZ_ONLY`.
- Cargo/script/CI classification:
  `HELPER_PLUS_TARGET_PLUS_CARGO_PLUS_SCRIPT_READY`.
- Corpus/seed classifications:
  `FUZZ_SEED_NO_CORPUS_FIRST`,
  `FUZZ_SEED_EPHEMERAL_GENERATION_ONLY`,
  `FUZZ_SEED_MANIFEST_TRACEABILITY_ONLY`, and
  `FUZZ_SEED_CORPUS_SEPARATE_LANE`.
- Selected successor:
  `NA-0487 -- QSL qsc Binding Fuzz Helper and Target Implementation Harness`.

No implementation mutation is performed in NA-0486. This lane changes no qsc
source, fuzz target, fuzz Cargo, qsc-adversarial script, workflow, corpus,
vector/input, runtime, crypto, dependency, lockfile, executable test, formal
model, refimpl, qsl-server, qsl-attachments, qshield runtime, qshield-cli,
website, public documentation, backup, restore, qsl-backup, qwork, qstart,
qresume, or qshell path.

## Live NA-0486 scope

Live READY item at startup:

`NA-0486 -- QSL qsc Binding Fuzz Helper / API Design Authorization Plan`

Allowed NA-0486 mutation paths:

- `docs/governance/evidence/NA-0486_qsl_qsc_binding_fuzz_helper_api_design_authorization_plan.md`
- `tests/NA-0486_qsl_qsc_binding_fuzz_helper_api_design_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Read-only inspection included:

- qwork proof files for NA-0486;
- `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`, and the rolling
  journal;
- NA-0485 evidence and testplans;
- `qsl/qsl-client/qsc/src/lib.rs`;
- `qsl/qsl-client/qsc/src/adversarial/`;
- `qsl/qsl-client/qsc/src/handshake/mod.rs`;
- `qsl/qsl-client/qsc/src/identity/mod.rs`;
- qsc integration tests and common helpers;
- qsc fuzz targets and qsc fuzz `Cargo.toml` / `Cargo.lock`;
- `scripts/ci/qsc_adversarial.sh`;
- `.github/workflows/qsc-adversarial.yml`;
- internal negative binding vector manifest;
- formal binding model files;
- refimpl boundary tests.

Forbidden mutation scope preserved:

- qsc source and runtime behavior;
- qsc fuzz targets, fuzz corpus, qsc fuzz Cargo files, and fuzz lockfile;
- scripts and workflows;
- inputs, vectors, and formal models;
- qsc executable tests and refimpl source/tests;
- qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public
  docs, README, and START_HERE;
- qwork/qstart/qresume/qshell;
- qsl-backup, backup status, backup plan, rollback, and backup tree paths.

Acceptance criteria:

- qwork proof-file verification recorded;
- NA-0485 inheritance consumed;
- applicable stewardship and assurance review completed;
- helper/API candidate inventory completed;
- process-harness versus in-process fuzz review completed;
- Cargo/script/CI scope decision recorded;
- helper design safety reviewed;
- corpus/seed strategy decided;
- implementation readiness classification selected;
- selected NA-0487 successor recorded;
- no implementation mutation;
- no public claim expansion;
- exactly one READY item remains.

Stop conditions:

- stale or inconsistent qwork proof;
- PR #1241 not merged at expected merge prefix;
- queue not READY NA-0486 at start;
- D-0959 absent or D-0960 present at start;
- helper/API, process-harness, Cargo/script/CI, helper safety, or corpus
  strategy review omitted;
- successor cannot be selected safely;
- any qsc source, fuzz, Cargo, script, workflow, corpus, vector, formal,
  refimpl, service, public, backup, or qwork-tool mutation in NA-0486;
- root or nested fuzz lock audit failure;
- public-safety red or missing;
- more than one READY item.

## qwork proof-file verification

Codex read and copied:

- `/srv/qbuild/work/NA-0486/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0486/.qwork/startup.qsl-protocol.json`

Verified fields:

- `startup_result=OK`;
- lane `NA-0486`;
- repo `qsl-protocol`;
- path `/srv/qbuild/work/NA-0486/qsl-protocol`;
- clean worktree, index, and untracked state;
- `READY_COUNT 1`;
- READY item `NA-0486`;
- requested lane status READY.

The proof HEAD and proof `origin/main` both matched live pre-fetch state at
`6264f37bf0db`. Fetch did not advance `origin/main`. Clean `main` was checked
out at the same commit. PR #1241 was verified merged with merge commit
`6264f37bf0db`.

Codex did not run qwork, qstart, qresume, qshell, sudo, backup, restore,
cargo update, or cargo generate-lockfile.

## NA-0485 inheritance

NA-0485 is DONE. D-0958 selected
`FUZZ_BINDING_HELPER_API_DESIGN_NEEDED` after finding that current qsc fuzz
targets cannot safely reach semantic A1/B1/A2 binding checks through target
files alone.

Inherited facts:

- existing qsc fuzz targets are `qsc_route_http`, `qsc_payload_boundaries`, and
  `qsc_vault_envelope`;
- those targets call public `qsc::adversarial::*` parser helpers only;
- `qsl/qsl-client/qsc/fuzz/Cargo.toml` enumerates all fuzz targets as
  `[[bin]]`;
- `scripts/ci/qsc_adversarial.sh` explicitly invokes each fuzz target and
  copies `fuzz/corpus/<target>` into a temp run directory;
- `.github/workflows/qsc-adversarial.yml` invokes the script generically and
  does not enumerate individual fuzz targets;
- the NA-0483 manifest remains internal metadata only;
- existing qsc binding negative tests and formal model remain supporting
  evidence, not fuzz implementation.

Inherited semantic binding surfaces:

- A1, B1, and A2 admission;
- wrong KEM public key;
- stale KEM/public record;
- wrong KEM ciphertext;
- wrong signature identity or public record;
- cross-message signature replay;
- transcript mutation;
- replay;
- suite confusion or downgrade-style suite block mutation;
- stale trusted pin/public-record mismatch;
- no completed-session mutation and no success output on reject.

## Applicable Stewardship and Assurance Review

Level 1 stewardship is active. Level 2 and Level 3 remain future-gated. No
separate Directors, independent READY promotion, independent merge authority,
or stewardship-canon mutation is created. Lead Director final authority is
preserved.

1. Crypto / Protocol Steward: semantic fuzz must exercise real qsc rejection
   paths for A1/B1/A2 binding and no-mutation boundaries. A target-local oracle
   alone is insufficient. A helper must not bypass validation or normalize bad
   inputs into accepted paths.
2. CI / Dependency / Release Health Steward: a new CI-visible target requires
   qsc fuzz `Cargo.toml` and qsc-adversarial script changes. Workflow mutation
   is not needed because the workflow calls the script. Dependency and lockfile
   mutation are not selected.
3. Public Claims / External Review Steward: this lane is internal governance
   evidence only. Classification:
   `EXTERNAL_REVIEW_READINESS_INCREMENTAL`.
4. Product / Demo / Service Boundary Steward: no qsl-server, qsl-attachments,
   qshield runtime, qshield-cli, demo, service, website, or public-doc behavior
   is in scope.
5. Local Ops / Backup / Restore Steward: qwork proof files were read and
   copied. qsl-backup SHA was read-only verified as `e9ecff3d22ed`; the codex
   ops source path appears exactly once in the installed qsl-backup source
   list. No backup or restore was run.
6. Best-Known-Method Review: classification
   `BEST_KNOWN_METHOD_FOR_SCOPE`. The best method is a narrow cfg-gated
   adversarial helper plus semantic fuzz target, not a broad public API and not
   parser-only fuzz.
7. Hostile Cryptographer Review: the selected design must fail if it only
   replays the formal model, tests parser bytes, or claims replay/downgrade
   proof. The helper must use real qsc reject paths.
8. Red-Team Review: priority attack classes are wrong KEM public key, wrong
   ciphertext, wrong signature identity, cross-message replay, transcript
   mutation, suite confusion, replay, stale public record, stale trusted pin,
   and no-mutation regressions.
9. Production SRE Review: normal qsc builds must not compile or expose the
   helper. The qsc-adversarial script may enable the helper through an explicit
   custom cfg for fuzz runs only.
10. Side-Channel Caveat: this lane and successor are not timing, cache,
    allocation, traffic-shape, or side-channel evidence. no side-channel-free
    claim is made.
11. Formal-Model Mapping Residual: classification
    `FORMAL_MODEL_MAPPING_SUPPORTING_ONLY`. The NA-0478 model guides expected
    classes but does not replace real qsc-path fuzz.
12. External-Review Readiness: classification
    `EXTERNAL_REVIEW_READINESS_INCREMENTAL`. This improves internal evidence
    organization only.
13. Release-Claim Boundary: no public-readiness claim, no crypto-complete
    claim, no fuzz-complete claim, no vector-complete claim, no replay-proof
    claim, no downgrade-proof claim, no formal-proof-complete claim, no
    qsc/refimpl-equivalence-complete claim, no provider-boundary-complete claim,
    no vulnerability-free claim, and no perfect-crypto claim is made.
14. Assurance Gap Review Trigger: classification
    `HIGHER_PRIORITY_RESIDUAL_SUPERSEDES_ASSURANCE_REVIEW` because an exact
    helper-plus-target successor is selected. A separate assurance-gap review
    remains future-gated until this residual is consumed or blocked.

## Binding helper / API candidate inventory

| Candidate | Current visibility and usability | Binding reach | Risk | Priority / disposition |
|---|---|---|---|---|
| Existing `qsc/src/adversarial/` helpers | Public module exported by `qsc::adversarial`; current helpers parse route, payload, and vault-envelope formats. Usable from fuzz today without source mutation. | Cannot express A1/B1/A2 semantic binding, replay, suite confusion, stale public record, trusted pin, or no-mutation paths today. | Existing helpers do not change production behavior, but adding ungated semantic helpers here would expand production public API. | High if cfg-gated. Select `qsl/qsl-client/qsc/src/adversarial/binding_fuzz.rs` plus guarded export in `adversarial/mod.rs`. |
| Existing handshake encode/decode/frame helpers | Private functions in `handshake/mod.rs`; not usable from fuzz target today. | Can express A1/B1/A2, transcript, KEM, signature, replay, suite, and no-mutation if exposed through a narrow wrapper. | Direct public exposure risks API drift and validation bypass. | Use only through a cfg-gated adversarial wrapper that calls real reject paths. |
| Existing identity/public-record/trusted-pin helpers | Mostly `pub(super)` and private within `identity/mod.rs`; not reusable from fuzz today. | Relevant for stale public-record and trusted-pin mismatch setup. | Direct exposure of identity internals could leak secret-handling assumptions. | Use indirectly through helper-created ephemeral temp state; no secret output. |
| Existing qsc integration test helper patterns | Test-local helpers in `tests/common/mod.rs` and binding negative tests; process-based CLI/temp-root patterns. | Already express KEM/signature/transcript/replay/suite/stale-record/no-mutation semantics. | Not reusable from fuzz without test mutation; slow and process-heavy. | Supporting design evidence only. Do not depend on test helper reuse for fuzz. |
| Existing mock relay/temp-root setup patterns | Test-only mock relay and temp roots. | Can drive real CLI handshakes. | Heavy filesystem/network state, libFuzzer instability, and process overhead. | Reject as primary fuzz approach; retain as integration-test evidence. |
| Existing fuzz target helper patterns | Current targets call public adversarial parsers. | Parser-only for binding. | Extending these repeats parser-only limitation. | Reject for semantic binding target. |
| Existing formal/vector metadata | NA-0478 model and NA-0483 manifest metadata. | Defines classes and expected reject/no-mutation outcomes. | Runtime JSON consumption would add IO/dependency risk and not exercise qsc path by itself. | Use as traceability-only seed classes, not runtime input. |
| Process-harness CLI/TUI invocation | Possible with qsc CLI and temp roots. | Exercises real qsc paths. | Slow, flaky, filesystem/network cleanup complexity, poor libFuzzer fit. | Reject as first implementation; future fallback if helper becomes unsafe. |
| New qsc internal helper module behind test/fuzz/adversarial cfg | Not present today; would require exact source mutation in successor. | Can reach real semantic reject paths while hiding from normal builds. | Must be narrowly cfg-gated and no secret output; otherwise production API drift. | Selected. |
| Fuzz-target-local simplified semantic checker | Can be written inside target without source helper. | Can express model categories but not real qsc behavior. | Drift from runtime, false confidence. | Reject as primary; may supplement only as assertions around helper outputs. |

The selected helper surface must be test/fuzz/adversarial only, cfg-gated,
call real reject paths, assert no success output and no selected session
mutation on rejects where feasible, and avoid exposing secrets.

## Process-harness vs in-process fuzz review

Option 1 -- In-process fuzz helper/API:

- Selected.
- Evidence: current qsc fuzz targets are in-process and CI already runs
  cargo-fuzz through `scripts/ci/qsc_adversarial.sh`.
- Future paths:
  `qsl/qsl-client/qsc/src/adversarial/binding_fuzz.rs`,
  `qsl/qsl-client/qsc/src/adversarial/mod.rs`,
  `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs`,
  `qsl/qsl-client/qsc/fuzz/Cargo.toml`, and
  `scripts/ci/qsc_adversarial.sh`.
- Required cfg: a custom fuzz/test cfg such as `qsc_binding_fuzz_helper`, with
  `#![allow(unexpected_cfgs)]` in the new helper module if needed, following
  the existing qsc cfg-seam pattern.
- Validation: normal no-cfg qsc builds/tests prove helper inactive; cfg-enabled
  helper/fuzz build proves target reaches A1/B1/A2 mutation classes.
- Expected CI behavior: qsc-adversarial-smoke includes the new target through
  the script; public-safety remains authoritative.
- Public-claim caveat: no fuzz-complete claim and no replay-proof claim.

Option 2 -- Process-harness fuzz target:

- Rejected as first implementation.
- Evidence: qsc integration tests already show process/temp-root patterns are
  viable, but they are slow and have filesystem/network cleanup complexity.
- Future fallback: if the helper/API is found unsafe during implementation,
  stop and select a process-harness authorization lane.
- Public-claim caveat: process-harness proof would still be internal evidence
  only.

Option 3 -- Fuzz-target-local simplified binding oracle:

- Rejected as primary.
- Evidence: it can mirror the formal model but would not exercise qsc runtime
  rejection.
- Public-claim caveat: no qsc behavior claim from target-local oracle alone.

Option 4 -- Extend parser fuzz only:

- Rejected.
- Evidence: NA-0484 and NA-0485 already found parser-only fuzz insufficient.

Option 5 -- Design no helper; defer to vector-consumer tests:

- Rejected as primary.
- Evidence: vector-consumer tests would improve deterministic coverage but
  leave the fuzz binding residual open.

## Cargo / script / CI exact scope decision

If the selected target is implemented, future NA-0487 must mutate
`qsl/qsl-client/qsc/fuzz/Cargo.toml` with a new `[[bin]]` entry because current
qsc fuzz targets are explicitly enumerated.

Future target can avoid dependency changes by using existing qsc/fuzz
dependencies and the qsc crate path dependency. Therefore no dependency
mutation and no qsc fuzz `Cargo.lock` mutation are selected.

Future qsc-adversarial script mutation is selected because the script
explicitly invokes each target. The script should include the new target and
set the custom helper cfg only for that target if needed.

Workflow mutation is not selected because `.github/workflows/qsc-adversarial.yml`
invokes `sh scripts/ci/qsc_adversarial.sh` generically.

Local cargo-fuzz absence may be recorded as a local tool caveat only if PR CI
`qsc-adversarial-smoke` runs the target and completes success.

Classifications:

- `HELPER_PLUS_TARGET_PLUS_CARGO_PLUS_SCRIPT_READY`;
- no `WORKFLOW_SCOPE_REQUIRED`;
- no `CARGO_LOCK_SCOPE_REQUIRED`;
- no `DEPENDENCY_SCOPE_REQUIRED`;
- no `SCOPE_STILL_AMBIGUOUS`.

## Helper design safety review

Selected helper requirements:

- compile only under an explicit custom fuzz/test cfg, not normal builds;
- be placed in `qsl/qsl-client/qsc/src/adversarial/binding_fuzz.rs`;
- be exported from `qsl/qsl-client/qsc/src/adversarial/mod.rs` only under the
  same cfg;
- avoid changing normal qsc public API in no-cfg builds;
- generate identities, keys, signatures, transcripts, temp state, and mutated
  frames ephemerally;
- return bounded public classifications and marker-friendly outcomes only;
- not return private keys, KEM secret keys, signing keys, passphrases, runtime
  keys, backup keys, operator data, user data, live service data, or private
  endpoint data;
- not bypass qsc validation logic;
- call real qsc reject paths for selected cases;
- assert no completed-session mutation and no success output on rejects where
  the selected scenario supports that assertion;
- use no unsafe code;
- require no dependency changes.

Classification: `HELPER_DESIGN_SAFE_TEST_FUZZ_ONLY`.

Stop conditions for future implementation:

- helper needs normal-build exposure;
- helper bypasses validation or accepts mutated bad input;
- helper emits secret material;
- helper needs dependency, lockfile, workflow, vector, formal, refimpl, service,
  public-doc, or backup changes outside exact scope;
- helper cannot prove inactive no-cfg behavior;
- helper cannot call real qsc reject paths.

## Corpus / seed strategy decision

Future implementation should include no checked-in corpus first. The target
should generate ephemeral inputs from fuzzer bytes, including selected seed
classes for A1/B1/A2 mutation, suite confusion, replay, stale public record,
and trusted-pin mismatch.

The NA-0483 manifest should be referenced as traceability metadata or comments
only. It should not be consumed at runtime by the fuzz target because runtime
JSON consumption adds IO, dependency, determinism, and corpus/fixture coupling
without improving the first helper reachability proof.

Corpus work should remain a separate future lane after the helper plus target
is merged and stable.

Classifications:

- `FUZZ_SEED_NO_CORPUS_FIRST`;
- `FUZZ_SEED_EPHEMERAL_GENERATION_ONLY`;
- `FUZZ_SEED_MANIFEST_TRACEABILITY_ONLY`;
- `FUZZ_SEED_CORPUS_SEPARATE_LANE`.

Rejected:

- `FUZZ_SEED_MANIFEST_RUNTIME_CONSUMPTION_READY`;
- `FUZZ_SEED_UNSAFE_STOP`.

## Implementation readiness decision

Primary classification:

`QSC_BINDING_FUZZ_HELPER_PLUS_TARGET_PLUS_CARGO_PLUS_SCRIPT_IMPLEMENTATION_READY`

Required facts consumed:

- NA-0485 consumed;
- helper/API candidates inventoried;
- process versus in-process review completed;
- Cargo/script/CI exact scope decided;
- helper safety reviewed;
- corpus/seed strategy decided;
- exact successor selected;
- no implementation mutation performed in NA-0486.

Selected NA-0487 successor:

`NA-0487 -- QSL qsc Binding Fuzz Helper and Target Implementation Harness`

## Future scope bundle

Future NA-0487 allowed implementation paths, if the Director issues the exact
successor directive:

- `qsl/qsl-client/qsc/src/adversarial/binding_fuzz.rs`
- `qsl/qsl-client/qsc/src/adversarial/mod.rs`
- `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs`
- `qsl/qsl-client/qsc/fuzz/Cargo.toml`
- `scripts/ci/qsc_adversarial.sh`
- `docs/governance/evidence/NA-0487_qsl_qsc_binding_fuzz_helper_and_target_implementation_harness.md`
- `tests/NA-0487_qsl_qsc_binding_fuzz_helper_and_target_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0487 forbidden unless a later exact directive changes scope:

- qsc source outside the two exact adversarial helper paths;
- qsc fuzz files outside the exact target and `Cargo.toml`;
- qsc adversarial script behavior outside adding/running the selected target;
- dependency or lockfile mutation;
- workflow mutation;
- checked-in corpus;
- vector/input mutation;
- formal model mutation;
- refimpl mutation;
- executable qsc integration test mutation unless separately authorized;
- qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public
  docs, README, START_HERE;
- qwork/qstart/qresume/qshell;
- backup, restore, qsl-backup, status, plan, rollback, and backup tree paths;
- public claim expansion.

## Future validation / marker plan

Common NA-0487 markers:

- `NA0487_BINDING_FUZZ_HELPER_SCOPE_CONSUMED_OK`
- `NA0487_NO_SECRET_MATERIAL_IN_FUZZ_INPUTS_OK`
- `NA0487_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0487_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0487_NO_FUZZ_COMPLETE_CLAIM_OK`
- `NA0487_NO_VECTOR_COMPLETE_CLAIM_OK`
- `NA0487_NO_REPLAY_PROOF_CLAIM_OK`
- `NA0487_NO_DOWNGRADE_PROOF_CLAIM_OK`
- `NA0487_ONE_READY_INVARIANT_OK`

Helper/API markers:

- `NA0487_HELPER_API_TEST_FUZZ_ONLY_OK`
- `NA0487_HELPER_API_NO_PRODUCTION_BEHAVIOR_CHANGE_OK`
- `NA0487_HELPER_API_NO_SECRET_OUTPUT_OK`
- `NA0487_HELPER_API_REAL_REJECT_PATHS_OK`
- `NA0487_HELPER_API_VECTOR_TRACEABILITY_OK`

Fuzz target markers:

- `NA0487_FUZZ_A1_MUTATION_TARGET_OK`
- `NA0487_FUZZ_B1_MUTATION_TARGET_OK`
- `NA0487_FUZZ_A2_MUTATION_TARGET_OK`
- `NA0487_FUZZ_SUITE_CONFUSION_TARGET_OK`
- `NA0487_FUZZ_REPLAY_TARGET_OK`
- `NA0487_FUZZ_STALE_PUBLIC_RECORD_TARGET_OK`

Cargo/script markers:

- `NA0487_EXACT_FUZZ_CARGO_SCOPE_AUTHORIZED_OK`
- `NA0487_FUZZ_CI_ADVERSARIAL_TARGET_INCLUDED_OK`

Expected validation commands:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
python3 formal/model_qsc_kem_signature_transcript_binding_bounded.py
python3 formal/run_model_checks.py
cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test signature_provider_boundary -- --test-threads=1 --nocapture
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
```

Future NA-0487 must additionally run the new helper/target validation selected
by implementation and rely on PR CI for cargo-fuzz-backed proof if local
cargo-fuzz is unavailable.

## Public claim / external review / website boundary

This evidence is internal governance evidence only. It is not public docs,
website content, public technical paper content, external review, production
readiness, public-internet readiness, public conformance vectors, fuzz
completion, formal proof, downgrade proof, replay proof, or vulnerability
review.

no public-readiness claim is made. no production-readiness claim is made. no
public-internet-readiness claim is made. no external-review-complete claim is
made. no crypto-complete claim is made. no fuzz-complete claim is made. no
vector-complete claim is made. no KEM-complete claim is made. no
signature-complete claim is made. no identity-complete claim is made. no
transcript-complete claim is made. no qsc/refimpl-equivalence-complete claim is
made. no provider-boundary-complete claim is made. no provider-RNG-complete
claim is made. no formal-proof-complete claim is made. no replay-proof claim is
made. no downgrade-proof claim is made. no side-channel-free claim is made. no
vulnerability-free claim is made. no bug-free claim is made. no perfect-crypto
claim is made. Cargo audit green remains dependency-health evidence only.

## Rejected alternatives

- Ungated public adversarial helper: rejected because it would expand normal
  qsc public API and risk production behavior drift.
- New non-adversarial qsc source module: rejected because `adversarial/`
  already carries parser/fuzz helper precedent and a new module would be more
  invasive.
- Process harness first: rejected because it is slower, filesystem/network
  heavy, and less suitable for libFuzzer.
- Target-local oracle first: rejected because it would not exercise real qsc
  rejection paths.
- Extend parser fuzz only: rejected because it repeats the parser-only
  limitation.
- Runtime manifest JSON consumption: rejected because traceability metadata is
  enough for seed classes and runtime IO/dependency coupling is not justified.
- Checked-in corpus first: rejected to avoid secret material and stale corpus
  policy before the helper/target shape is proven.

## Backup-impact statement

Backup impact is none for NA-0486. Durable changes are tracked qsl-protocol
governance/testplan/decision/traceability/journal files only. Codex did not
run backup or restore and did not mutate qsl-backup, backup status, backup
plan, rollback, systemd, timer, fstab, or `/backup/qsl`.

Same-host continuity remains not disaster recovery. no off-host-backup-complete
claim is made. no restore-proof claim is made. no backup-complete claim is
made. no disaster-recovery-complete claim is made.

## Next recommendation

Close NA-0486 after merge and restore:

`NA-0487 -- QSL qsc Binding Fuzz Helper and Target Implementation Harness`

The NA-0487 directive should implement only the exact helper plus target plus
Cargo plus qsc-adversarial script scope selected here, prove no-cfg production
behavior is unchanged, prove no secret output, and stop if implementation needs
dependency, lockfile, workflow, corpus, vector, formal, refimpl, service,
public-doc, backup, or broader qsc source scope.
