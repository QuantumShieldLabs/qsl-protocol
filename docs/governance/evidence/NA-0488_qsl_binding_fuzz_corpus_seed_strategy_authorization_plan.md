Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-16

# NA-0488 Binding Fuzz Corpus / Seed Strategy Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0488 consumes NA-0487 and D350, inventories the current qsc fuzz corpus and
seed surface, and authorizes the next binding fuzz seed strategy. The selected
classification is `BINDING_FUZZ_EPHEMERAL_SEED_STRATEGY_READY`.

No checked-in corpus is authorized now. The first follow-on should keep
`qsc_binding_semantics` no-corpus and add deterministic ephemeral seed recipes
inside the fuzz harness. The NA-0483 internal negative vector manifest remains
traceability-only; future code may use a static category mapping derived from
the same category names, but must not read the JSON manifest at runtime.

Selected successor:

`NA-0489 -- QSL Binding Fuzz Ephemeral Seed Strategy Implementation Harness`

This authorization adds no implementation mutation, no corpus mutation, no
vector/input mutation, no qsc source mutation, no qsc fuzz target mutation, no
qsc fuzz Cargo mutation, no qsc-adversarial script mutation, no workflow
mutation, no dependency mutation, no lockfile mutation, no formal mutation, no
refimpl mutation, no service mutation, no public-doc mutation, and no backup or
restore action.

## Live NA-0488 scope

Startup READY item:

`NA-0488 -- QSL Binding Fuzz Corpus / Seed Strategy Authorization Plan`

Allowed mutation paths used by this evidence PR:

- this evidence doc
- `tests/NA-0488_qsl_binding_fuzz_corpus_seed_strategy_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Read-only inventory paths used:

- qwork startup proof files under `/srv/qbuild/work/NA-0488/.qwork/`
- D349 and D350 response files under `/home/victor/work/qsl/codex/responses/`
- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- qsc fuzz target, Cargo, corpus, and script paths
- internal negative binding vector manifest paths
- formal/refimpl/qsc validation paths
- qsl-backup status/plan/helper paths

No implementation, runtime, crypto, dependency, Cargo, lockfile, workflow,
corpus, vector/input, formal, refimpl, service, public-doc, backup, qsl-backup,
qwork, qstart, qresume, or qshell mutation is authorized by this lane.

## qwork proof-file verification

Codex read the operator-provided qwork proof files and did not run qwork,
qstart, or qresume:

- `/srv/qbuild/work/NA-0488/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0488/.qwork/startup.qsl-protocol.json`

Required qwork fields matched:

- `startup_result=OK`
- `lane=NA-0488`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0488/qsl-protocol`
- `head_equals_origin_main=yes`
- clean worktree, index, and untracked state
- `ready_count=1`
- `queue_top_ready=NA-0488`
- `requested_lane_status=READY`

Freshness proof:

- proof HEAD and live pre-fetch HEAD both matched `d78d163535ff`
- proof `origin/main` and live pre-fetch `origin/main` both matched
  `d78d163535ff`
- fetch did not advance `origin/main`
- `origin/main` equals or descends from `d78d163535ff`
- PR #1246 was verified merged with merge commit `d78d163535ff`

Startup queue and decision proof:

- READY_COUNT 1
- READY NA-0488
- NA-0487 DONE
- NA-0486 DONE
- NA-0485 through NA-0436 DONE where expected
- NA-0434 BLOCKED
- NA-0429 BLOCKED
- D-0963 exists once
- D-0964 exists once
- D-0965 absent before this patch
- D-0966 absent before this patch
- duplicate decision count 0

Startup health proof:

- public-safety on `d78d163535ff`: success
- qsc-adversarial-smoke on `d78d163535ff`: success
- root cargo audit: PASS
- nested qsc fuzz lock audit: PASS
- `rustls-webpki` and `ml-kem` inverse dependency probes: present
- optional pqcrypto inverse probes completed under directive-approved
  failure-tolerant command shape
- installed qsl-backup SHA256 matched the expected digest
- installed qsl-backup source list includes the Codex ops source exactly once

## NA-0487 / D350 inheritance

Consumed inheritance sources:

- D350 response:
  `/home/victor/work/qsl/codex/responses/NA0487_closeout_restore_na0488_20260616T165533Z_D350.md`
- D349 response:
  `/home/victor/work/qsl/codex/responses/NA0487_20260616T163857Z_D349.md`
- `docs/governance/evidence/NA-0487_qsl_qsc_binding_fuzz_helper_and_target_implementation_harness.md`
- `tests/NA-0487_qsl_qsc_binding_fuzz_helper_and_target_implementation_testplan.md`
- D-0963 and D-0964 in `DECISIONS.md`
- NA-0488 block in `NEXT_ACTIONS.md`

Inherited facts:

- PR #1245 implemented the recovered qsc binding fuzz helper and target and
  merged at `ea1af60e84f0`.
- PR #1246 closed NA-0487, restored NA-0488, and merged at `d78d163535ff`.
- The cfg-gated qsc binding fuzz helper exists behind exact cfg
  `qsc_binding_fuzz_helper`.
- The semantic fuzz target `qsc_binding_semantics` exists.
- qsc fuzz Cargo metadata includes the target.
- `scripts/ci/qsc_adversarial.sh` invokes the target with target-specific cfg.
- NA-0487 added no checked-in corpus.
- NA-0487 added no dependency, lockfile, workflow, vector/input, formal,
  refimpl, service, public-doc, backup, or qsl-backup mutation.
- D350 recorded CI wait accounting:
  `CI_WAIT_TOTAL_MINUTES 19.93`, `CI_PRODUCTIVE_WAIT_MINUTES 19.93`,
  `CI_IDLE_WAIT_MINUTES 0.00`, and
  `TOTAL_POTENTIALLY_WASTED_WAIT_MINUTES 0.00`.
- D350 verified post-merge public-safety and qsc-adversarial-smoke success for
  the inherited implementation and closeout path.

Why NA-0488 is the correct lane now:

- The helper and target exist, so corpus/seed strategy is no longer abstract.
- The new target has no checked-in corpus, so the immediate decision is whether
  that should remain true.
- The internal vector manifest exists and validates, but its usage boundary must
  be selected before code reads it, maps it, or turns it into seeds.
- Secret/private-material and public/conformance claim risk now dominate the
  checked-in corpus decision.

Inherited caveats:

- local cargo-fuzz may be unavailable; PR CI qsc-adversarial-smoke supplies
  cargo-fuzz-backed smoke evidence when required
- qsc-adversarial smoke is smoke evidence, not no fuzz-complete claim
- cargo audit green is dependency-health evidence only
- vector manifest evidence is internal governance evidence only
- no public-readiness claim, no crypto-complete claim, no fuzz-complete claim,
  no corpus-complete claim, no vector-complete claim, no replay-proof claim, no
  downgrade-proof claim, no side-channel-free claim, no vulnerability-free
  claim, and no perfect-crypto claim is made

## Applicable Stewardship and Assurance Review

Level-1 stewardship conclusion:

- The safest next step is no checked-in corpus and deterministic ephemeral
  seed recipes for the existing target.
- Lead Director authority and exactly-one-READY queue discipline remain
  unchanged.

Best-Known-Method Review:

- Keep seed material hermetic, deterministic, and generated from small public
  recipe classes.
- Do not check in binding corpus bytes until a no-secret validator exists and a
  future lane proves the corpus is target-specific, public/mutated-only, and
  not presented as conformance material.

Hostile Cryptographer Review:

- A checked-in binding corpus could be mistaken for public conformance vectors,
  could overfit fuzz evidence to hand-picked examples, and could smuggle
  representational assumptions about keys or transcripts.
- Runtime JSON manifest consumption would add IO/path assumptions and reduce
  fuzz hermeticity.

Red-Team Review:

- The target should not depend on repository-relative runtime files.
- Corpus files should not be used as an accidental channel for private keys,
  passphrases, operator data, user data, live endpoints, or production-like
  identifiers.
- No corpus is the lowest-risk first step.

Production SRE Review:

- qsc-adversarial already invokes `qsc_binding_semantics`; no workflow or script
  update is needed for a no-corpus ephemeral recipe lane.
- Target-only recipe code keeps CI blast radius smaller than corpus validators,
  workflow changes, or dependency/tooling changes.

Side-Channel Caveat:

- Fuzz seeds and corpus strategy are not side-channel review.
- This lane makes no side-channel-free claim and does not reduce the need for a
  later side-channel or secret-material assurance lane.

Formal-Model Mapping Residual:

- The NA-0483 manifest and formal-token entries support traceability only.
- This lane makes no formal-proof-complete claim and does not consume formal
  artifacts as runtime seed inputs.

External-Review Readiness:

- Internal seed strategy improves review preparation but is not external
  review completion.
- External reviewers should see caveated internal evidence, not public vector
  or conformance claims from fuzz corpus material.

Release-Claim Boundary:

- This lane is internal governance evidence only.
- It makes no public-readiness claim, no production-readiness claim, no
  crypto-complete claim, no fuzz-complete claim, no corpus-complete claim, no
  vector-complete claim, no replay-proof claim, no downgrade-proof claim, no
  side-channel-free claim, no vulnerability-free claim, and no perfect-crypto
  claim.

Assurance Gap Review Trigger:

- STOP or split if future seed work requires qsc source mutation outside the
  exact selected fuzz target, runtime manifest IO, dependency or lockfile
  changes, workflow changes, checked-in corpus files, vector/input mutation,
  secret-like material, or public-claim expansion.

## Current fuzz corpus / seed surface inventory

Current qsc fuzz targets:

| Target | Corpus path | Exists | File count | Checked in | Content type | qsc-adversarial |
|---|---|---:|---:|---:|---|---|
| `qsc_route_http` | `qsl/qsl-client/qsc/fuzz/corpus/qsc_route_http` | yes | 3 | yes | `.http` text fixtures | yes |
| `qsc_payload_boundaries` | `qsl/qsl-client/qsc/fuzz/corpus/qsc_payload_boundaries` | yes | 5 | yes | `.json` text fixtures | yes |
| `qsc_vault_envelope` | `qsl/qsl-client/qsc/fuzz/corpus/qsc_vault_envelope` | yes | 2 | yes | small `.bin` fixtures | yes |
| `qsc_binding_semantics` | `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics` | no | 0 | n/a | none | yes |

Inventory classifications:

- `FUZZ_CORPUS_EXISTING_PARSER_TARGETS_ONLY`
- `FUZZ_CORPUS_BINDING_TARGET_NONE`

Not selected:

- `FUZZ_CORPUS_BINDING_TARGET_PRESENT`
- `FUZZ_CORPUS_SECRET_RISK_FOUND`
- `FUZZ_CORPUS_AMBIGUOUS`

Additional inventory notes:

- `qsl/qsl-client/qsc/fuzz/artifacts/` is absent.
- Existing checked-in corpus files are small synthetic fixtures for older
  parser/format targets.
- The existing corpus file names and types show no obvious secret/private
  material risk, but that is not a deep secret scan for future corpus work.
- The new binding target currently relies on arbitrary libFuzzer input plus
  category selection in the target/helper.
- `scripts/ci/qsc_adversarial.sh` copies a target-specific corpus directory if
  one exists, but currently no binding corpus directory exists.

## Corpus / seed strategy options

Option 1 -- No checked-in corpus for now:

- Selected as the immediate corpus policy.
- Evidence: `qsc_binding_semantics` is already invoked by qsc-adversarial and
  has no corpus directory.
- Future paths if selected alone: none beyond governance; this lane selects
  ephemeral seed implementation as the next active follow-up.
- No-secret proof strategy: no files means no checked-in secret material.
- Validation strategy: qsc-adversarial-smoke plus existing target build/test
  checks.
- Public-claim caveat: no corpus-complete claim and no fuzz-complete claim.
- Risks: P1 less deterministic initial coverage; P2 slower corpus discovery.

Option 2 -- Generated ephemeral seed recipes only:

- Selected for NA-0489.
- Evidence: deterministic recipes can improve starting coverage without adding
  public corpus files, runtime JSON IO, dependencies, lockfiles, or workflow
  changes.
- Exact future implementation path:
  `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs`.
- Future governance paths:
  `docs/governance/evidence/NA-0489_qsl_binding_fuzz_ephemeral_seed_strategy_implementation_harness.md`,
  `tests/NA-0489_qsl_binding_fuzz_ephemeral_seed_strategy_implementation_testplan.md`,
  `DECISIONS.md`, `TRACEABILITY.md`, and
  `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.
- No-secret proof strategy: recipes must use public constants, mutated public
  message bytes, short synthetic byte arrays, and ephemeral generated values
  only; no persisted private material.
- Validation strategy: target build under `RUSTFLAGS='--cfg qsc_binding_fuzz_helper'`,
  qsc-adversarial-smoke in PR CI, no-corpus scope guard, and overclaim scan.
- Public-claim caveat: no fuzz-complete claim and no vector-complete claim.
- Risks: P1 target-only recipes may be too shallow; if helper/source mutation is
  required, NA-0489 must STOP and split.

Option 3 -- Metadata-only seed-class manifest:

- Selected only as traceability/static category guidance.
- Evidence: NA-0483 manifest records categories and no-secret policy, but
  validation status remains metadata-only pending future test consumption.
- Future paths: same as Option 2 if implemented as static comments/category
  mapping inside the fuzz target.
- No-secret proof strategy: do not copy manifest JSON into code and do not
  consume it at runtime.
- Validation strategy: manifest JSON tool validation plus static mapping review.
- Public-claim caveat: no conformance/vector claim.
- Risks: P2 stale category mapping; mitigated by traceability updates.

Option 4 -- Checked-in public/mutated corpus for `qsc_binding_semantics`:

- Rejected for now.
- Evidence: no target-specific corpus exists and no corpus secret-material
  validator exists.
- Exact future path if later selected:
  `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/<exact files>`.
- Required first proof: corpus secret scanner/validator, public/mutated-only
  provenance, target-specific path, and public/conformance caveat.
- Public-claim caveat: checked-in corpus must be labeled internal fuzz seed
  material, not public vectors, conformance vectors, or completion evidence.
- Risks: P0 secret/private material leak if validator is missing; P1 public
  conformance confusion; P1 maintenance/provenance overhead.

Option 5 -- Split corpus by category:

- Rejected for the first follow-up.
- Evidence: category split is useful if a checked-in corpus is later selected,
  but it is unnecessary overhead for ephemeral recipes.
- Future paths if later selected: category-specific subfiles under the binding
  corpus directory after a validator-first lane.
- Risks: P1 governance overhead and public-claim confusion.

Option 6 -- Defer corpus; vector-consumer tests next:

- Rejected as the immediate successor but retained as a later candidate.
- Evidence: vector-consumer tests are valuable, but NA-0488's narrow decision
  can improve fuzz determinism first without consuming the manifest at runtime.
- Future successor if selected later:
  `NA-0490 -- QSL Binding Negative Vector Consumer Test Scope Authorization Plan`
  or a Director-approved equivalent.
- Risks: P2 fuzz coverage remains less deterministic if no ephemeral recipe
  work lands first.

Option 7 -- Fuzz stabilization/recovery next:

- Rejected now.
- Evidence: qsc-adversarial-smoke is green on current main and no instability
  was observed in the new target from D350/main evidence.
- Risk: P2 if future PR CI shows target instability, split to stabilization.

Option 8 -- Side-channel / secret-material assurance next:

- Rejected as immediate successor but retained as a residual.
- Evidence: no checked-in corpus is authorized, so corpus secret-material risk
  is lower than deterministic seed implementation.
- Risk: P1 side-channel and secret-material assurance remains incomplete and
  must not be represented as closed.

## Internal vector manifest usage review

Reviewed manifest:

`inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json`

Observed manifest summary:

- schema version 1
- status `internal-negative-evidence-only`
- 34 vector metadata entries
- groups include KEM binding, signature binding, transcript/replay/suite,
  stale identity/rollback, refimpl signature provider-boundary, and formal
  token mapping
- `contains_secret_material` true count: 0
- private-key true count: 0
- passphrase true count: 0
- user-data true count: 0
- validation status is metadata-only pending future test consumption
- manifest says ephemeral generation is required for sensitive validation

Selected classifications:

- `VECTOR_MANIFEST_TRACEABILITY_ONLY_SELECTED`
- `VECTOR_MANIFEST_STATIC_CATEGORY_MAPPING_READY`

Rejected classifications:

- `VECTOR_MANIFEST_RUNTIME_CONSUMPTION_READY`
- `VECTOR_MANIFEST_CORPUS_MAPPING_SPLIT_NEEDED`
- `VECTOR_MANIFEST_USE_UNSAFE_STOP`

Consumer-test classification:

- `VECTOR_MANIFEST_CONSUMER_TESTS_NEXT` is not selected as NA-0489 but remains a
  later candidate after the ephemeral seed strategy lands or if the Lead
  Director reprioritizes deterministic vector consumers.

Answers:

- Future seed code should not read the JSON manifest at runtime.
- Future seed code may hardcode a stable subset of category names, with comments
  tying the categories to NA-0483 traceability.
- Future corpus files should not embed vector IDs now because no corpus is
  authorized.
- Manifest use should remain traceability-only for NA-0489.
- Runtime JSON consumption would add IO, repository-relative path assumptions,
  fuzz non-hermeticity, and public/conformance confusion risk.

## Secret / private material review

Checked-in seeds must not require:

- private keys
- signing keys
- KEM secret keys
- passphrases
- runtime keys
- backup keys
- operator data
- user data
- live service data
- private endpoints
- production-like identifiers

Selected classification:

- `CORPUS_SECRET_MATERIAL_EPHEMERAL_ONLY`

Additional future requirement:

- `CORPUS_SECRET_MATERIAL_VALIDATOR_REQUIRED_FIRST` applies before any later
  checked-in binding corpus lane.

Rejected classifications:

- `CORPUS_SECRET_MATERIAL_SAFE_TO_CHECK_IN`
- `CORPUS_SECRET_MATERIAL_SPLIT_NEEDED`
- `CORPUS_SECRET_MATERIAL_UNSAFE_STOP`

Conclusions:

- All NA-0489 seed data should be public constants, mutated public message
  bytes, short synthetic byte arrays, or generated ephemerally.
- Checked-in binding corpus files are prohibited until a validator exists and a
  future lane proves no-secret/public-mutated-only provenance.
- First implementation should avoid corpus entirely to prevent hidden secret
  material risk.

## CI / Cargo / workflow impact review

Selected classifications for the chosen no-corpus ephemeral strategy:

- `CORPUS_NO_CARGO_SCRIPT_WORKFLOW_IMPACT`
- `CORPUS_FUZZ_TARGET_CHANGE_NEEDED`

Deferred classification for future checked-in corpus:

- `CORPUS_VALIDATOR_SCRIPT_NEEDED`
- `CORPUS_SCOPE_SPLIT_NEEDED`

Rejected for NA-0489:

- `CORPUS_QSC_ADVERSARIAL_SCRIPT_CHANGE_NEEDED`
- `CORPUS_WORKFLOW_CHANGE_NEEDED`

Impact answers:

- Adding corpus files would not inherently require Cargo changes, but is not
  authorized now.
- Adding deterministic ephemeral seed-generation code requires mutation to the
  qsc binding fuzz target.
- qsc-adversarial script updates are not needed for the selected target-only
  no-corpus strategy because the target is already invoked.
- workflow updates are not needed.
- fuzz `Cargo.lock` changes are not needed.
- corpus validation would require a new validator/script only if a future
  checked-in corpus lane is selected.
- qsc-adversarial-smoke currently provides enough target-attach fuzz smoke
  without corpus; it is not no fuzz-complete claim.

## Authorization decision

Primary classification selected:

`BINDING_FUZZ_EPHEMERAL_SEED_STRATEGY_READY`

Supporting classifications:

- `BINDING_FUZZ_NO_CHECKED_IN_CORPUS_READY`
- `BINDING_FUZZ_METADATA_ONLY_SEED_STRATEGY_READY`

Rejected primary classifications:

- `BINDING_FUZZ_CHECKED_IN_CORPUS_IMPLEMENTATION_READY`
- `BINDING_FUZZ_CORPUS_VALIDATOR_FIRST_READY`
- `BINDING_NEGATIVE_VECTOR_CONSUMER_TESTS_NEXT`
- `BINDING_FUZZ_STABILIZATION_NEXT`
- `SIDE_CHANNEL_SCOPE_NEXT`
- `CORPUS_STRATEGY_SPLIT_NEEDED`
- `CORPUS_STRATEGY_UNSAFE_STOP`
- `CORPUS_STRATEGY_AMBIGUOUS`

Decision:

- Do not add checked-in binding fuzz corpus now.
- Keep first binding fuzz seed work no-corpus and ephemeral-generation-only.
- Keep NA-0483 manifest usage traceability-only.
- Allow static category mapping in the fuzz target.
- Do not read manifest JSON at runtime.
- Do not mutate inputs/vectors/corpus in NA-0489.
- Do not mutate qsc source/helper code, qsc fuzz Cargo, qsc-adversarial script,
  workflows, dependencies, lockfiles, formal models, refimpl, services, public
  docs, qsl-backup, or backup paths in NA-0489 unless a later exact directive
  changes scope.

## Future scope bundle

Selected successor block:

```md
### NA-0489 -- QSL Binding Fuzz Ephemeral Seed Strategy Implementation Harness
Status: READY
Goals: G1, G2, G3, G4, G5

Objective:
Implement deterministic ephemeral seed recipes inside the existing
`qsc_binding_semantics` fuzz target without adding checked-in corpus files,
runtime manifest consumption, dependency changes, lockfile changes, workflow
changes, or public-claim expansion.

Allowed scope:
- `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs`
- `docs/governance/evidence/NA-0489_qsl_binding_fuzz_ephemeral_seed_strategy_implementation_harness.md`
- `tests/NA-0489_qsl_binding_fuzz_ephemeral_seed_strategy_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden unless later exact scope authorizes:
- checked-in corpus files
- vector/input mutation
- qsc source/helper mutation outside the fuzz target
- qsc fuzz Cargo mutation
- qsc-adversarial script mutation
- workflow mutation
- dependency mutation
- lockfile mutation
- runtime/crypto behavior mutation outside selected fuzz-target recipe code
- refimpl mutation
- formal mutation
- service/public/backup mutation
- no public-readiness claim, no crypto-complete claim, no fuzz-complete claim,
  no corpus-complete claim, no vector-complete claim, no replay-proof claim, no
  downgrade-proof claim, no side-channel-free claim, no vulnerability-free
  claim, or no perfect-crypto claim
```

Future validation commands likely needed:

```bash
git diff --check
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo check --manifest-path qsl/qsl-client/qsc/fuzz/Cargo.toml --locked --bin qsc_binding_semantics
cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
```

If local cargo-fuzz is available, the future lane should also run:

```bash
cd qsl/qsl-client/qsc/fuzz
RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo fuzz run qsc_binding_semantics -- -runs=1
```

If local cargo-fuzz is unavailable, record the exact output and rely on PR CI
qsc-adversarial-smoke if green.

## Future validation / marker plan

Common NA-0489 markers:

- `NA0489_CORPUS_STRATEGY_CONSUMED_OK`
- `NA0489_NO_SECRET_MATERIAL_IN_CORPUS_OK`
- `NA0489_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0489_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0489_NO_FUZZ_COMPLETE_CLAIM_OK`
- `NA0489_NO_CORPUS_COMPLETE_CLAIM_OK`
- `NA0489_NO_VECTOR_COMPLETE_CLAIM_OK`
- `NA0489_NO_REPLAY_PROOF_CLAIM_OK`
- `NA0489_NO_DOWNGRADE_PROOF_CLAIM_OK`
- `NA0489_ONE_READY_INVARIANT_OK`

Ephemeral-strategy markers selected:

- `NA0489_EPHEMERAL_SEED_GENERATION_ONLY_OK`
- `NA0489_NO_CHECKED_IN_CORPUS_OK`
- `NA0489_VECTOR_MANIFEST_TRACEABILITY_ONLY_OK`

Checked-in corpus markers are not selected for NA-0489.

Validator markers are not selected for NA-0489.

## Public claim / external review / website boundary

This evidence is internal governance evidence only.

It makes no public-readiness claim, no production-readiness claim, no
public-internet-readiness claim, no external-review-complete claim, no
crypto-complete claim, no fuzz-complete claim, no corpus-complete claim, no
vector-complete claim, no KEM-complete claim, no signature-complete claim, no
identity-complete claim, no transcript-complete claim, no replay-proof claim,
no downgrade-proof claim, no side-channel-free claim, no vulnerability-free
claim, no bug-free claim, and no perfect-crypto claim.

No website, README, START_HERE, public docs, public technical paper, or public
claim surface is modified.

## Rejected alternatives

- Runtime JSON manifest consumption is rejected because it adds IO/path
  assumptions, makes fuzz less hermetic, and risks public/conformance confusion.
- Checked-in binding corpus is rejected now because no target-specific corpus
  exists and no corpus secret-material validator exists.
- Category-split checked-in corpus is rejected now because it adds governance
  overhead before basic ephemeral recipes exist.
- Vector-consumer tests are deferred, not rejected permanently.
- Fuzz stabilization is deferred because current qsc-adversarial-smoke is
  green.
- Side-channel / secret-material assurance is retained as a residual but not
  selected as the immediate successor.

## Backup-impact statement

No backup or restore command was run.

No qsl-backup, backup status, backup plan, rollback, backup tree, or off-host
state path is mutated by NA-0488.

The installed qsl-backup helper SHA256 matched the expected digest, and the
installed daily source list includes the Codex ops source exactly once.

This lane makes no backup-complete claim, no restore-proof claim, no
off-host-backup-complete claim, and no disaster-recovery-complete claim.

## Next recommendation

Proceed to the selected closeout only after the NA-0488 evidence PR merges and
post-merge public-safety is green. The closeout should mark NA-0488 DONE and
restore:

`NA-0489 -- QSL Binding Fuzz Ephemeral Seed Strategy Implementation Harness`

NA-0489 should implement target-only deterministic ephemeral seed recipes for
`qsc_binding_semantics`, preserve no checked-in corpus, preserve manifest
traceability-only status, and stop if broader qsc source, Cargo, script,
workflow, dependency, lockfile, corpus, vector/input, formal, refimpl, service,
public-doc, backup, or public-claim scope is required.
