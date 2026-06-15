Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-15

# NA-0484 QSL Fuzz Binding Coverage Scope Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0484 consumes NA-0483 internal negative binding vector evidence and
inventories current qsc fuzz coverage before any fuzz implementation work.

Current qsc fuzz coverage is classified as
`FUZZ_BINDING_CURRENTLY_PARSER_ONLY`. The active qsc fuzz targets exercise HTTP
route parsing, message/control payload parsing, and vault envelope parsing.
They do not directly exercise A1/B1/A2 handshake frame mutation, KEM public-key
binding, KEM ciphertext binding, signature identity/context binding,
transcript mutation, replay rejection, suite-confusion rejection, stale
public-record rejection, or trusted-pin mismatch behavior.

The internal NA-0483 vector manifest can safely inform future fuzz scope and
seed strategy because it is metadata-only and records no secret/private
material. It should not be treated as public/conformance vector evidence and
should not be consumed by a fuzz target until a later exact directive
authorizes that consumption.

Primary NA-0484 decision:

- Current fuzz classification: `FUZZ_BINDING_CURRENTLY_PARSER_ONLY`.
- Cargo/workflow/CI classification: `FUZZ_IMPLEMENTATION_SPLIT_NEEDED`.
- Secret/corpus classification: `FUZZ_CORPUS_GENERATED_EPHEMERAL_ONLY`.
- Primary authorization classification: `FUZZ_BINDING_SPLIT_SCOPE_NEEDED`.
- Selected successor: `NA-0485 -- QSL Fuzz Binding Coverage Split-Scope Authorization Plan`.

No fuzz implementation is performed in NA-0484. No runtime, crypto,
dependency, Cargo, lockfile, workflow, executable test, fuzz target, corpus,
vector/input, formal model, qsc source, refimpl source/test, service, public
surface, qsl-backup, backup status, backup plan, rollback, or backup tree path
is mutated.

## Live NA-0484 scope

Live READY item at startup:

`NA-0484 -- QSL Fuzz Binding Coverage Scope Authorization Plan`

Allowed NA-0484 evidence paths:

- `docs/governance/evidence/NA-0484_qsl_fuzz_binding_coverage_scope_authorization_plan.md`
- `tests/NA-0484_qsl_fuzz_binding_coverage_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Allowed read-only inspection surfaces:

- qwork proof files for NA-0484;
- `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`, and the rolling journal;
- NA-0483 evidence, testplan, README, and internal manifest;
- qsc source/tests/fuzz workspace;
- refimpl crypto source/tests;
- formal model files;
- inputs and vectors;
- scripts and workflows for CI placement review;
- backup status/plan/script/log paths as read-only boundary evidence.

Forbidden mutation scope preserved:

- implementation, runtime, crypto, dependency, Cargo, lockfile, workflow,
  executable test, fuzz target, fuzz corpus, vector/input, formal model, qsc
  source/test, refimpl source/test, qsl-server, qsl-attachments, qshield
  runtime, qshield-cli, website, public docs, README, START_HERE,
  qwork/qstart/qresume/qshell, qsl-backup, backup status, backup plan,
  rollback, `/backup/qsl`, durable Director State Index output, and public
  technical paper content.

Acceptance criteria:

- current fuzz surface inventoried;
- binding fuzz candidate matrix recorded;
- fuzz strategy reviewed;
- Cargo/workflow/CI impact reviewed;
- secret/corpus risk reviewed;
- selected successor recorded;
- no implementation mutation;
- no public claim expansion;
- exactly one READY item remains.

Stop conditions used:

- qwork proof stale or inconsistent;
- PR #1237 not merged at required merge prefix;
- queue not READY NA-0484;
- D-0955 absent or D-0956 present at startup;
- current fuzz inventory or candidate matrix omitted;
- successor cannot be selected safely;
- any forbidden path mutation;
- root or nested fuzz lock audit failure;
- public-safety red or missing;
- more than one READY item.

## qwork proof-file verification

Codex read and copied:

- `/srv/qbuild/work/NA-0484/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0484/.qwork/startup.qsl-protocol.json`

Verified fields:

- `startup_result=OK`;
- lane `NA-0484`;
- repo `qsl-protocol`;
- path `/srv/qbuild/work/NA-0484/qsl-protocol`;
- clean worktree, index, and untracked state;
- `READY_COUNT 1`;
- READY item `NA-0484`;
- requested lane status READY.

The proof HEAD and proof `origin/main` both matched live pre-fetch state at
`8aa9ae192764`. Fetch did not advance `origin/main`. Clean `main` was checked
out at the same commit.

PR #1237 was verified merged with merge commit
`8aa9ae1927647a8669f7557036405ada9a9c1ae2`.

Codex did not run qwork, qstart, qresume, sudo, backup, restore, cargo update,
or cargo generate-lockfile.

## NA-0483 inheritance

NA-0483 is DONE. D-0954 added:

- `inputs/suite2/internal_negative_binding_vectors/README.md`;
- `inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json`.

The manifest contains 34 metadata-only internal vector entries:

- KEM binding: 4 entries;
- signature binding: 5 entries;
- transcript/replay/suite binding: 8 entries;
- stale identity/rollback: 4 entries;
- refimpl signature provider-boundary: 6 entries;
- formal-token mapping: 7 entries.

The manifest records no private keys, signing keys, KEM secret keys,
passphrases, runtime keys, backup keys, operator data, user data, live service
data, or private production endpoint data. Future fuzz or test harnesses must
generate any needed secret material ephemerally.

Public/conformance vectors under `inputs/suite2/vectors/` remain separate and
untouched. NA-0483 remains internal negative evidence only.
No public-readiness evidence is claimed. No vector-complete evidence is claimed.
No crypto-complete evidence is claimed. No external-review-complete evidence is
claimed. No replay-proof evidence is claimed. No downgrade-proof evidence is
claimed. No side-channel-free evidence is claimed. No vulnerability-free
evidence is claimed. No bug-free evidence is claimed. No perfect-crypto
evidence is claimed.

## Applicable Stewardship and Assurance Review

Level 1 stewardship is active. Level 2 and Level 3 remain future-gated. No
separate Directors, independent READY promotion, independent merge authority,
or independent public-claim authority is created. Lead Director final authority
is preserved.

1. Crypto / Protocol Steward:
   Current qsc fuzzing does not reach KEM/signature/transcript binding
   semantics. The internal vector manifest is useful evidence for selecting
   future fuzz surfaces, but direct semantic fuzz implementation is not ready
   without additional exact scope for qsc adversarial helper exposure or a
   process/file harness. Classification:
   `FUZZ_BINDING_SPLIT_SCOPE_NEEDED`.

2. CI / Dependency / Release Health Steward:
   qsc-adversarial-smoke runs `scripts/ci/qsc_adversarial.sh`, and that script
   enumerates only `qsc_route_http`, `qsc_payload_boundaries`, and
   `qsc_vault_envelope`. A new CI-visible fuzz target would need exact scope
   for at least qsc fuzz Cargo metadata and likely script inclusion. Workflow
   mutation is not required if the existing workflow keeps calling the script.
   Classification: `FUZZ_IMPLEMENTATION_SPLIT_NEEDED`.

3. Public Claims / External Review Steward:
   This lane is internal governance evidence only. It improves external-review
   readiness incrementally by identifying the next fuzz gap, but it is not
   external-review-complete evidence and does not expand public claims.
   Classification: `EXTERNAL_REVIEW_READINESS_INCREMENTAL`.

4. Product / Demo / Service Boundary Steward:
   qsc binding fuzz scope is client-local. No qsl-server, qsl-attachments,
   qshield runtime, qshield-cli, website, public docs, public demo, or
   production service path is mutated or authorized.

5. Local Ops / Backup / Restore Steward:
   qwork proof files were read and copied; qwork/qstart/qresume were not run.
   qsl-backup SHA was read-only verified as
   `e9ecff3d22ed`, and the codex ops source path appears exactly once in the
   installed qsl-backup source list. No backup, restore, status, plan,
   rollback, or backup tree mutation is performed.

6. Best-Known-Method Review:
   Classification: `BEST_KNOWN_METHOD_FOR_SCOPE`. The best method in this
   authorization-only lane is not to force a fuzz implementation through a
   private or CI-invisible route. The best method is to select a split-scope
   successor that decides the exact helper/API/Cargo/script boundary first.

7. Hostile Cryptographer Review:
   A hostile reviewer would reject parser-only fuzz as evidence for binding
   semantics. Binding fuzz must exercise fail-closed rejection for wrong KEM
   public key, wrong ciphertext, wrong signature public record, cross-message
   signature replay, transcript mutation, replay, suite confusion, and stale
   public record or trusted pin mismatch. Current fuzz does not do that.

8. Red-Team Review:
   Red-team residuals remain around replayed A1/B1/A2 frames, suite block
   mutation, stale identity records, trusted-pin mismatch, and transcript field
   mutation. The internal vector manifest describes these cases but does not
   execute them under fuzzing.

9. Production SRE Review:
   No production operation changes are made. Future fuzz should use bounded
   runtimes and ephemeral temp directories if process/file harnessing is used,
   and must avoid checked-in secrets or persistent runtime data.

10. Side-Channel Caveat:
   This lane and any future fuzz lane selected here provide no side-channel-free
   evidence. Timing, memory-access, branching, and provider
   side-channel review remain residual.

11. Formal-Model Mapping Residual:
   Classification: `FORMAL_MODEL_MAPPING_SUPPORTING_ONLY`. NA-0478 formal
   tokens support scenario selection but are not implementation equivalence
   proof. They are not fuzz proof. They are not formal-proof-complete evidence.
   They are not public security completion evidence.

12. External-Review Readiness:
   Classification: `EXTERNAL_REVIEW_READINESS_INCREMENTAL`. The lane prepares
   clearer future fuzz scope and caveats for reviewers, but does not claim
   external review has occurred or completed.

13. Release-Claim Boundary:
   No public-readiness claim is made. No production-readiness claim is made.
   No public-internet-readiness claim is made. No external-review-complete
   claim is made. No crypto-complete claim is made. No fuzz-complete claim is
   made. No vector-complete claim is made. No KEM-complete claim is made. No
   signature-complete claim is made. No identity-complete claim is made. No
   transcript-complete claim is made. No qsc/refimpl-equivalence-complete claim
   is made. No provider-boundary-complete claim is made. No provider-RNG-complete
   claim is made. No formal-proof-complete claim is made. No replay-proof claim
   is made. No downgrade-proof claim is made. No side-channel-free claim is
   made. No vulnerability-free claim is made. No bug-free claim is made.
   No perfect-crypto claim is made.

14. Assurance Gap Review Trigger:
   Classification: `HIGHER_PRIORITY_RESIDUAL_SUPERSEDES_ASSURANCE_REVIEW`.
   The exact successor is a fuzz split-scope authorization plan. If NA-0485
   cannot authorize an exact implementation path, the next decision should
   trigger `ASSURANCE_GAP_REVIEW_REQUIRED_NOW`.

## Current fuzz surface inventory

Current qsc fuzz workspace:

- `qsl/qsl-client/qsc/fuzz/Cargo.toml`
- `qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_route_http.rs`
- `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_payload_boundaries.rs`
- `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_vault_envelope.rs`
- corpus directories for those three target names.

Current qsc-adversarial CI path:

- `.github/workflows/qsc-adversarial.yml` installs cargo-fuzz and runs
  `sh scripts/ci/qsc_adversarial.sh`.
- `scripts/ci/qsc_adversarial.sh` runs stable qsc adversarial tests and then
  calls `run_fuzz_target` for only `qsc_route_http`,
  `qsc_payload_boundaries`, and `qsc_vault_envelope`.

Existing fuzz target inventory:

| Target | Path | Purpose | Parser or semantic state coverage | A1/B1/A2 reach | Binding/replay/downgrade/stale reach | Corpus/seeds | Future Cargo/script impact |
|---|---|---|---|---|---|---|---|
| `qsc_route_http` | `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_route_http.rs` | Parses HTTP request bytes, target path, and route-token headers. | Parser boundary only. | No. | No KEM/signature/transcript/replay/suite/stale binding reach. | Uses `qsl/qsl-client/qsc/fuzz/corpus/qsc_route_http/`. | New binding target would need separate Cargo bin and script inclusion. |
| `qsc_payload_boundaries` | `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_payload_boundaries.rs` | Parses receipt, file-confirm, file-transfer, attachment-descriptor, and attachment-confirm payloads. | Parser boundary only. | No. | No KEM/signature/transcript/replay/suite/stale binding reach. | Uses `qsl/qsl-client/qsc/fuzz/corpus/qsc_payload_boundaries/`. | New binding target would need separate Cargo bin and script inclusion. |
| `qsc_vault_envelope` | `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_vault_envelope.rs` | Parses qsc vault envelope bytes. | Parser boundary only. | No. | No KEM/signature/transcript/replay/suite/stale binding reach. | Uses `qsl/qsl-client/qsc/fuzz/corpus/qsc_vault_envelope/`. | New binding target would need separate Cargo bin and script inclusion. |

Current fuzz classification:

- `FUZZ_BINDING_CURRENTLY_PARSER_ONLY`

Supporting note:

- qsc handshake encode/decode and binding logic live in `qsl/qsl-client/qsc/src/handshake/mod.rs`.
- The qsc library exports `adversarial` and `envelope`; it does not export a
  binding/handshake adversarial helper surface for fuzz targets.
- The current fuzz crate can call public qsc adversarial parsers but cannot
  directly call private A1/B1/A2 handshake encode/decode/binding helpers.

## Binding fuzz candidate surface inventory

| Group | Candidate | Source evidence | Feasibility | Target path candidate | Cargo/fuzz metadata | Corpus | Secrets | Existing deterministic coverage | Expected invariant | Public-claim risk | Complexity | Priority | Disposition |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| qsc frame parser / mutation | A1 frame mutation | `handshake/mod.rs` A1 encode/decode; NA-0476 tests; NA-0483 manifest | Medium only if helper/API or process harness is authorized. | Candidate `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_frames.rs` | Required for new target. | Avoid first lane. | Ephemeral generation only. | NA-0476 deterministic rejects. | malformed or mutated A1 rejects fail-closed and no session mutation. | Must not imply replay-proof or fuzz-complete. | Medium/high. | P1. | Needs split-scope. |
| qsc frame parser / mutation | B1 frame mutation | `handshake/mod.rs` B1 decode; NA-0476 tests; formal model | Medium only if helper/API or process harness is authorized. | Candidate `qsc_binding_frames.rs` | Required. | Avoid first lane. | Ephemeral generation only. | NA-0476 wrong ciphertext/transcript rejects. | mutated B1 rejects fail-closed. | Must not imply KEM-complete. | Medium/high. | P1. | Needs split-scope. |
| qsc frame parser / mutation | A2 frame mutation | `handshake/mod.rs` A2 decode; NA-0476 tests; formal model | Medium only if helper/API or process harness is authorized. | Candidate `qsc_binding_frames.rs` | Required. | Avoid first lane. | Ephemeral generation only. | NA-0476 signature/cross-message rejects. | mutated A2 rejects fail-closed. | Must not imply signature-complete. | Medium/high. | P1. | Needs split-scope. |
| qsc frame parser / mutation | Suite block mutation | `hs_parse_parameter_block`; D-0955 residual | High for parser/admission if helper is exposed; lower for full semantic state. | Candidate `qsc_handshake_suite_context.rs` or combined target | Required. | Optional public byte seeds later. | No. | Existing deterministic suite-confusion tests. | unsupported/downgrade tuples reject. | Must not imply downgrade-proof. | Medium. | P1. | Needs split-scope. |
| qsc frame parser / mutation | Transcript field mutation | NA-0476 and NA-0478 | Medium only with binding-state harness. | Candidate combined target | Required. | Avoid first lane. | Ephemeral generation only. | NA-0476 transcript mutation rejects. | transcript mismatch rejects and no session mutation. | Must not imply transcript-complete. | High. | P1. | Needs split-scope. |
| binding semantic fuzz | Wrong KEM public key token | NA-0483 `kem_wrong_peer_public_key`; formal model | Medium with generated identities and helper/process harness. | Candidate `qsc_binding_semantics.rs` | Required. | No checked-in corpus first. | Ephemeral generation only. | NA-0476 and formal model. | no completed session on wrong public key. | Must not imply KEM-complete. | High. | P0/P1. | Needs split-scope. |
| binding semantic fuzz | Wrong KEM ciphertext token | NA-0483 `kem_wrong_ciphertext`; formal model | Medium with generated handshake. | Candidate `qsc_binding_semantics.rs` | Required. | No checked-in corpus first. | Ephemeral generation only. | NA-0476 wrong ciphertext reject. | no completed session on wrong ciphertext. | Must not imply KEM-complete. | High. | P1. | Needs split-scope. |
| binding semantic fuzz | Wrong signature public record / identity token | NA-0483 signature entries | Medium with generated identities. | Candidate `qsc_binding_semantics.rs` | Required. | No checked-in corpus first. | Ephemeral generation only. | NA-0476 signature wrong identity reject. | wrong identity/signature binding rejects. | Must not imply signature-complete. | High. | P1. | Needs split-scope. |
| binding semantic fuzz | Cross-message signature replay | NA-0483 and formal model | Medium/high complexity. | Candidate `qsc_binding_semantics.rs` | Required. | No checked-in corpus first. | Ephemeral generation only. | NA-0476 cross-message replay reject. | B1/A2 signature context mismatch rejects. | Must not imply replay-proof. | High. | P1. | Needs split-scope. |
| binding semantic fuzz | Transcript mutation | NA-0483 and formal model | Medium/high complexity. | Candidate `qsc_binding_semantics.rs` | Required. | No checked-in corpus first. | Ephemeral generation only. | NA-0476 transcript mutation reject. | reject without session mutation. | Must not imply transcript-complete. | High. | P1. | Needs split-scope. |
| binding semantic fuzz | Replayed messages | NA-0483 replay entries | Medium/high complexity. | Candidate `qsc_binding_semantics.rs` | Required. | No checked-in corpus first. | Ephemeral generation only. | NA-0476 replay reject. | duplicate/replayed frame rejects. | Must not imply replay-proof. | High. | P1. | Needs split-scope. |
| binding semantic fuzz | Suite confusion | NA-0483 suite entries; `hs_parse_parameter_block` | Medium. | Candidate combined target | Required. | Optional safe public bytes later. | No. | NA-0476 suite-confusion reject. | wrong suite block rejects. | Must not imply downgrade-proof. | Medium. | P1. | Needs split-scope. |
| binding semantic fuzz | Stale public record / trusted pin | NA-0483 stale entries | Medium/high complexity. | Candidate `qsc_binding_semantics.rs` | Required. | No checked-in corpus first. | Ephemeral generation only. | NA-0476 stale public-record reject. | stale record rejects and no state mutation. | Must not imply identity-complete. | High. | P1. | Needs split-scope. |
| vector-seeded fuzz | Use internal negative manifest metadata as seed descriptions | NA-0483 manifest | Feasible after exact parser/semantic target chosen. | Candidate target reads no checked-in secret data. | No dependency change expected; target and Cargo bin needed. | Metadata-only first. | No checked-in secrets. | Manifest JSON validation. | vectors guide cases, not public vectors. | Must not imply vector-complete. | Medium. | P1. | Select for later, not current implementation. |
| vector-seeded fuzz | Generate ephemeral messages inside fuzz harness | NA-0483 secret policy | Feasible if helper/process harness authorized. | Candidate semantic target. | Required. | No checked-in corpus. | Ephemeral only. | Existing deterministic tests. | no secret material in fuzz inputs. | Must not imply secret-material-complete. | High. | P1. | Recommended after split-scope. |
| corpus / seed strategy | No corpus in first lane | Current corpus directories are target-specific parser seeds | Feasible. | No corpus path. | None beyond target/Cargo. | None. | No. | N/A. | start with generated data. | Low. | Low. | P0. | Recommended. |
| corpus / seed strategy | Metadata-only seed mapping | NA-0483 manifest | Feasible. | Evidence/testplan only until implementation. | None. | None. | No. | Manifest validation. | trace cases without checked-in secret data. | Low. | Low. | P1. | Recommended. |
| corpus / seed strategy | Checked-in public/mutated bytes only if safe | Future corpus path | Feasible later after exact review. | `qsl/qsl-client/qsc/fuzz/corpus/<target>/` | None. | Requires exact safety proof. | No. | None yet. | byte seeds contain no private material. | Medium. | Medium. | P2. | Defer. |
| corpus / seed strategy | Generated ephemeral seed corpus | Future proof-root only | Feasible for CI/local transient evidence. | proof root or temp run dir only. | None. | Temporary only. | Ephemeral only. | Existing tests. | no checked-in secrets. | Low. | Medium. | P1. | Recommended. |
| refimpl boundary fuzz | Signature malformed lengths | NA-0481 tests | Feasible but lower priority than qsc binding split. | Future refimpl fuzz target if selected later. | New refimpl fuzz scope likely needed. | No corpus first. | Ephemeral only. | NA-0481 tests. | Err versus false classification stable. | Must not imply signature-complete. | Medium. | P2. | Defer. |
| refimpl boundary fuzz | Invalid signatures / wrong public key | NA-0481 tests | Feasible later. | Future refimpl target. | Separate scope. | No corpus first. | Ephemeral only. | NA-0481 tests. | invalid returns false or Err as specified. | Must not imply provider-boundary-complete. | Medium. | P2. | Defer. |
| refimpl boundary fuzz | KEM malformed lengths / tamper | refimpl `pqkem768` tests | Feasible later. | Future refimpl target. | Separate scope. | No corpus first. | Ephemeral only. | `pqkem768`. | malformed rejects; tamper does not silently succeed. | Must not imply KEM-complete. | Medium. | P2. | Defer. |

## Fuzz strategy option review

Option 1 -- New qsc binding fuzz target:

- Selection: not implementation-ready in NA-0485 without split-scope.
- Evidence: current qsc fuzz targets are public parser helpers only; binding
  helpers are private to `handshake/mod.rs`; qsc-adversarial script enumerates
  targets explicitly.
- Candidate future paths if later authorized:
  `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_frames.rs` or
  `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs`,
  `qsl/qsl-client/qsc/fuzz/Cargo.toml`, and possibly
  `scripts/ci/qsc_adversarial.sh`.
- Cargo/fuzz metadata: required for a new registered cargo-fuzz target.
- Corpus: avoid checked-in corpus in first implementation lane.
- Public-claim caveat: no fuzz-complete, replay-proof, or downgrade-proof
  claim.

Option 2 -- Extend existing qsc fuzz target:

- Selection: rejected for the next lane.
- Evidence: existing targets are route, payload, and vault parser scopes.
  Extending one to semantic binding would mix unrelated invariants and reduce
  evidence clarity.
- Public-claim caveat: extension would not make current parser fuzz binding
  semantic evidence.

Option 3 -- Vector-seeded fuzz target using internal negative manifest metadata:

- Selection: promising but not direct next implementation.
- Evidence: NA-0483 manifest safely lists metadata-only cases and no secrets.
- Required caveat: the manifest is not public/conformance vector evidence.
- Disposition: should inform the split-scope successor and later target design.

Option 4 -- Corpus-only seed authorization:

- Selection: rejected.
- Evidence: no existing target consumes binding corpus, so corpus-only work
  would not reach binding semantics.

Option 5 -- Refimpl boundary fuzz first:

- Selection: rejected for immediate successor.
- Evidence: NA-0481 deterministic refimpl signature provider-boundary tests
  already cover the strongest refimpl signature residual selected in the prior
  lane. qsc binding fuzz remains the stronger residual.

Option 6 -- Split-scope authorization:

- Selection: selected.
- Evidence: exact implementation needs a decision on qsc helper/API exposure,
  process/file harness viability, fuzz Cargo metadata, and qsc-adversarial
  script inclusion before it is safe to authorize implementation.
- Selected future title:
  `NA-0485 -- QSL Fuzz Binding Coverage Split-Scope Authorization Plan`.

Option 7 -- Side-channel / secret-material scope next:

- Selection: rejected for immediate successor.
- Evidence: side-channel and secret-material residuals remain real, but current
  live residual is fuzz binding scope exactness after NA-0483.

Option 8 -- External review readiness next:

- Selection: rejected for immediate successor.
- Evidence: external-review readiness benefits from first resolving whether
  binding fuzz can be implemented cleanly and caveated.

## Cargo / workflow / CI impact review

Questions answered:

- If a new fuzz target is added later, qsc fuzz `Cargo.toml` must add a new
  `[[bin]]` entry unless the target is not a cargo-fuzz registered binary.
- A new target using only existing direct dependencies should not require
  root dependency or lockfile mutation. The qsc fuzz `Cargo.lock` might remain
  unchanged if no new dependency is added, but this must be proven in the
  implementation lane.
- Workflow mutation is not inherently required because
  `.github/workflows/qsc-adversarial.yml` delegates to
  `scripts/ci/qsc_adversarial.sh`.
- qsc-adversarial-smoke will not pick up a new target automatically because
  `scripts/ci/qsc_adversarial.sh` explicitly calls each target.
- Local cargo-fuzz absence is not a blocker for this authorization lane; if
  implementation occurs later, PR CI can supply cargo-fuzz-backed evidence
  while local absence is recorded.
- Future implementation should avoid dependency changes.

Classification:

- `FUZZ_IMPLEMENTATION_SPLIT_NEEDED`

Split-scope questions for NA-0485:

- Can a binding fuzz target use only public qsc APIs, or is a qsc
  adversarial helper seam required?
- If a helper seam is required, can it be limited to an adversarial/test-facing
  API without changing runtime behavior?
- Should qsc-adversarial script mutation be authorized to include the new
  target, or should a separate equivalent CI proof path be selected?
- Can the target run without checked-in corpus and without secret material?
- Can `qsl/qsl-client/qsc/fuzz/Cargo.lock` remain unchanged?

## Secret material / corpus risk review

Future fuzz seeds or corpus must not require checked-in private keys, signing
keys, KEM secret keys, passphrases, runtime keys, backup keys, operator data,
user data, live service data, or private production endpoint data.

Safe strategy:

- avoid checked-in corpus in the first binding fuzz implementation;
- use NA-0483 metadata only as case descriptions and traceability;
- generate ephemeral identities, KEM material, signatures, transcripts, and
  mutated messages inside the harness or temp proof root;
- store no secret material in repository fuzz inputs;
- preserve public-claim caveats in evidence and PR body.

Classification:

- `FUZZ_CORPUS_GENERATED_EPHEMERAL_ONLY`

No unsafe corpus path is selected. If a future lane proposes checked-in corpus
bytes, it must prove the bytes are public/mutated-only and no-secret before
adding them.

## Authorization decision

NA-0483 is consumed. Current fuzz surfaces are inventoried. Binding fuzz
candidates are inventoried. Fuzz strategy options are reviewed.
Cargo/workflow/CI impact is reviewed. Secret/corpus risk is reviewed.

Primary classification:

- `FUZZ_BINDING_SPLIT_SCOPE_NEEDED`

Selected successor:

- `NA-0485 -- QSL Fuzz Binding Coverage Split-Scope Authorization Plan`

Rationale:

- Current qsc fuzz is parser-only and does not reach binding semantics.
- Direct semantic binding fuzz is valuable but not exact enough to authorize
  as implementation yet because the current public fuzz API does not expose
  A1/B1/A2 binding helpers, and CI target enumeration would need exact script
  or equivalent CI scope.
- The internal negative manifest is safe to use as metadata for future target
  design, but not as checked-in secret or public/conformance vector material.
- The next safest lane is a split-scope authorization plan that decides the
  exact API/helper, target, Cargo, script/CI, and corpus boundaries before
  implementation.

No implementation mutation is authorized or performed by NA-0484.

## Future scope bundle

Selected NA-0485:

`NA-0485 -- QSL Fuzz Binding Coverage Split-Scope Authorization Plan`

Future NA-0485 objective:

- Decide the exact qsc binding fuzz implementation shape after resolving the
  public API/helper, Cargo metadata, qsc-adversarial script/CI, and corpus
  boundaries.

Future NA-0485 allowed mutation scope:

- `docs/governance/evidence/NA-0485_qsl_fuzz_binding_coverage_split_scope_authorization_plan.md`
- `tests/NA-0485_qsl_fuzz_binding_coverage_split_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0485 read-only inspection scope should include:

- `qsl/qsl-client/qsc/fuzz/`
- `qsl/qsl-client/qsc/fuzz/fuzz_targets/`
- `qsl/qsl-client/qsc/fuzz/Cargo.toml`
- `qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `scripts/ci/qsc_adversarial.sh`
- `.github/workflows/qsc-adversarial.yml`
- `qsl/qsl-client/qsc/src/adversarial/`
- `qsl/qsl-client/qsc/src/handshake/`
- `qsl/qsl-client/qsc/tests/kem_signature_transcript_binding_negative.rs`
- `inputs/suite2/internal_negative_binding_vectors/`
- formal binding model and refimpl provider-boundary tests.

Candidate later implementation paths for a future implementation lane, not
authorized by NA-0484:

- `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_frames.rs`
- `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs`
- `qsl/qsl-client/qsc/fuzz/Cargo.toml`
- `scripts/ci/qsc_adversarial.sh`
- a tightly bounded qsc adversarial helper path only if a later exact
  directive authorizes it and proves no runtime behavior change.

Future forbidden unless exact scope authorizes:

- qsc runtime/source mutation outside an explicitly selected adversarial helper
  seam;
- dependency, Cargo, lockfile, workflow, or script mutation;
- qsc executable test mutation;
- refimpl source/test mutation;
- formal model mutation;
- vector/input mutation;
- fuzz corpus mutation;
- service/public/qshield/qsl-server/qsl-attachments mutation;
- backup/restore/qsl-backup/status/plan/rollback mutation;
- no public readiness or crypto completion claims.

## Future validation / marker plan

Common future NA-0485 no-overclaim markers:

- `NA0485_FUZZ_BINDING_SCOPE_CONSUMED_OK`
- `NA0485_NO_SECRET_MATERIAL_IN_FUZZ_INPUTS_OK`
- `NA0485_NO_RUNTIME_CHANGE_OK`
- `NA0485_NO_DEPENDENCY_CHANGE_OK` or
  `NA0485_EXACT_FUZZ_CARGO_SCOPE_AUTHORIZED_OK`
- `NA0485_NO_WORKFLOW_CHANGE_OK` or
  `NA0485_EXACT_CI_SCRIPT_SCOPE_AUTHORIZED_OK`
- `NA0485_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0485_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0485_NO_FUZZ_COMPLETE_CLAIM_OK`
- `NA0485_NO_VECTOR_COMPLETE_CLAIM_OK`
- `NA0485_NO_REPLAY_PROOF_CLAIM_OK`
- `NA0485_NO_DOWNGRADE_PROOF_CLAIM_OK`
- `NA0485_ONE_READY_INVARIANT_OK`

Candidate future fuzz markers if a later implementation is authorized:

- `NA0485_FUZZ_A1_MUTATION_TARGET_OK`
- `NA0485_FUZZ_B1_MUTATION_TARGET_OK`
- `NA0485_FUZZ_A2_MUTATION_TARGET_OK`
- `NA0485_FUZZ_SUITE_CONFUSION_TARGET_OK`
- `NA0485_FUZZ_REPLAY_TARGET_OK`
- `NA0485_FUZZ_STALE_PUBLIC_RECORD_TARGET_OK`
- `NA0485_FUZZ_VECTOR_MANIFEST_TRACEABILITY_OK`

Candidate validation commands for future split-scope authorization:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
rg -n "qsc_route_http|qsc_payload_boundaries|qsc_vault_envelope" qsl/qsl-client/qsc/fuzz/Cargo.toml scripts/ci/qsc_adversarial.sh
rg -n "hs_decode_init|hs_decode_resp|hs_decode_confirm|hs_parse_parameter_block" qsl/qsl-client/qsc/src/handshake/mod.rs
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
```

Candidate validation commands for a later implementation lane only if exact
scope authorizes target/Cargo/script changes:

```bash
cargo +nightly fuzz run <selected-target> <temp-seed-dir> -- -max_total_time=10
sh scripts/ci/qsc_adversarial.sh
```

## Public claim / external review / website boundary

NA-0484 is internal governance evidence only.

No public-readiness claim is made. No production-readiness claim is made. No
public-internet-readiness claim is made. No external-review-complete claim is
made. No crypto-complete claim is made. No fuzz-complete claim is made. No
vector-complete claim is made. No KEM-complete claim is made. No
signature-complete claim is made. No identity-complete claim is made. No
transcript-complete claim is made. No qsc/refimpl-equivalence-complete claim is
made. No provider-boundary-complete claim is made. No provider-RNG-complete
claim is made. No formal-proof-complete claim is made. No replay-proof claim
is made. No downgrade-proof claim is made. No side-channel-free claim is made.
No vulnerability-free claim is made. No bug-free claim is made.
No perfect-crypto claim is made.

No website, public docs, public technical paper, qshield-cli, qshield runtime,
qsl-server, or qsl-attachments path is mutated or authorized.

## Rejected alternatives

- New qsc binding fuzz target now: rejected because exact helper/API/Cargo/CI
  scope is not ready.
- Existing target extension: rejected because current parser targets are
  unrelated to binding semantics.
- Corpus-only work: rejected because no existing target consumes binding
  corpus.
- Refimpl boundary fuzz first: deferred because qsc binding fuzz scope is the
  stronger residual after NA-0483.
- Side-channel scope next: deferred; side-channel residual remains active but
  does not supersede the current fuzz binding exactness gap.
- External-review readiness next: deferred until fuzz binding scope is more
  exact.

## Backup-impact statement

NA-0484 changes only tracked qsl-protocol governance evidence/testplan,
decision, traceability, and rolling journal files. It does not mutate
qsl-backup, backup status, backup plan, rollback paths, `/backup/qsl`, backup
manifests, backup logs, systemd, timer, fstab, or restore state. It does not
run backup or restore.

Same-host continuity remains only same-host continuity. No off-host backup
claim is made. No restore-proof claim is made. No disaster-recovery claim is
made. No backup-complete claim is made. No key-custody claim is made. No
key-recovery claim is made.

## Next recommendation

Proceed to `NA-0485 -- QSL Fuzz Binding Coverage Split-Scope Authorization Plan`
after NA-0484 evidence PR merge, post-merge public-safety success, and
closeout. NA-0485 should decide whether the later implementation is:

- qsc parser/admission fuzz only;
- qsc semantic binding fuzz with an adversarial helper seam;
- qsc semantic binding fuzz through a process/file harness;
- vector-metadata-guided but corpus-free;
- split into parser/admission first and semantic binding later;
- or deferred in favor of another residual if exact implementation remains
  unsafe.
