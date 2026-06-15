Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-15

# NA-0485 QSL Fuzz Binding Coverage Split-Scope Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0485 consumes NA-0484 and answers the remaining split-scope questions before
any qsc binding fuzz implementation.

Primary decision:

- NA-0484 inheritance: consumed.
- Current qsc fuzz inventory: `qsc_route_http`, `qsc_payload_boundaries`, and
  `qsc_vault_envelope` remain parser-boundary oriented.
- Target strategy: do not extend an existing target for semantic binding.
- New target strategy: a later semantic target remains the right shape, but it
  is not implementation-ready from fuzz target files alone.
- Helper/API access classification:
  `FUZZ_HELPER_ACCESS_REQUIRES_RUNTIME_SOURCE_DESIGN`.
- Cargo/script/CI exact scope classification:
  `FUZZ_EXACT_SCOPE_SPLIT_REQUIRED`.
- Corpus/secret strategy classification:
  `FUZZ_CORPUS_EPHEMERAL_GENERATION_READY` with
  `FUZZ_CORPUS_METADATA_ONLY_READY` guidance from the internal manifest.
- Implementation readiness classification:
  `FUZZ_BINDING_HELPER_API_DESIGN_NEEDED`.
- Selected successor:
  `NA-0486 -- QSL qsc Binding Fuzz Helper / API Design Authorization Plan`.

No fuzz implementation is performed in NA-0485. No fuzz target, fuzz corpus,
Cargo manifest, lockfile, workflow, script, runtime, crypto, dependency,
executable test, vector/input, formal model, qsc source, refimpl source/test,
qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public doc,
README, START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status,
backup plan, rollback, durable Director State Index, public technical paper, or
backup tree path is mutated.

## Live NA-0485 scope

Live READY item at startup:

`NA-0485 -- QSL Fuzz Binding Coverage Split-Scope Authorization Plan`

Allowed NA-0485 mutation paths:

- `docs/governance/evidence/NA-0485_qsl_fuzz_binding_coverage_split_scope_authorization_plan.md`
- `tests/NA-0485_qsl_fuzz_binding_coverage_split_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Read-only inspection surfaces included qwork proof files, `NEXT_ACTIONS.md`,
`DECISIONS.md`, `TRACEABILITY.md`, rolling journal state, NA-0484 evidence,
current qsc fuzz targets and corpora, qsc fuzz Cargo metadata and lockfile,
`scripts/ci/qsc_adversarial.sh`, `.github/workflows/qsc-adversarial.yml`,
the internal negative binding vector manifest, qsc binding negative tests,
qsc handshake and identity source, formal model files, and refimpl boundary
tests.

Forbidden mutation scope preserved:

- fuzz target, fuzz corpus, qsc runtime/source, qsc executable test,
  dependency, Cargo manifest, lockfile, workflow, script, vector/input, formal
  model, refimpl source/test, qsl-server, qsl-attachments, qshield runtime,
  qshield-cli, website, public docs, README, START_HERE,
  qwork/qstart/qresume/qshell, qsl-backup, backup status, backup plan,
  rollback, `/backup/qsl`, durable Director State Index output, and public
  technical paper content.

Acceptance criteria:

- qwork proof-file verification recorded.
- NA-0484 inheritance recorded.
- fuzz target/Cargo/script/CI inventory recorded.
- helper/API access classified.
- Cargo/script/CI exact scope classified.
- corpus/secret-material strategy classified.
- implementation readiness decision selected.
- exact NA-0486 successor selected.
- no implementation mutation.
- no public claim expansion.
- exactly one READY item remains.

Stop conditions used:

- stale or inconsistent qwork proof;
- PR #1239 not merged at expected merge prefix;
- queue not READY NA-0485 at start;
- D-0957 absent or D-0958 present at start;
- fuzz/Cargo/script/CI inventory omitted;
- helper/API, Cargo/script/CI, or corpus/secret review omitted;
- successor cannot be selected safely;
- any forbidden path mutation;
- root or nested fuzz lock audit failure;
- public-safety red or missing;
- more than one READY item.

## qwork proof-file verification

Codex read and copied:

- `/srv/qbuild/work/NA-0485/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0485/.qwork/startup.qsl-protocol.json`

Verified fields:

- `startup_result=OK`;
- lane `NA-0485`;
- repo `qsl-protocol`;
- path `/srv/qbuild/work/NA-0485/qsl-protocol`;
- clean worktree, index, and untracked state;
- `READY_COUNT 1`;
- READY item `NA-0485`;
- requested lane status READY.

The proof HEAD and proof `origin/main` both matched live pre-fetch state at
`228434165c2b`. Fetch did not advance `origin/main`. Clean `main` was checked
out at the same commit.

PR #1239 was verified merged with merge commit
`228434165c2b220339b642ce79a33632ffd3a27a`.

Codex did not run qwork, qstart, qresume, sudo, backup, restore, cargo update,
or cargo generate-lockfile.

Recovered proof-shape issues:

- First direct decision-count command used a decision heading pattern that did
  not match this repository's `- **ID:** D-xxxx` format. Classification:
  recoverable command-shape proof mistake. Corrective action: reran the count
  with the repository's actual decision ID line format. Final result:
  D-0956 count 1, D-0957 count 1, D-0958 count 0.
- First qsl-backup source count scanned status/plan narrative prose as well as
  the script, yielding narrative references instead of source-list proof.
  Classification: recoverable evidence-selector mistake. Corrective action:
  counted the installed qsl-backup script source list. Final result: codex ops
  source path count 1 in `/usr/local/sbin/qsl-backup`.
- One read-only `rg` inventory command included nonexistent
  `qsl/qsl-client/qsc/src/adversarial.rs`. Classification: recoverable
  command-shape inventory mistake. Corrective action: inventoried the actual
  `qsl/qsl-client/qsc/src/adversarial/` directory. Final result: parser helper
  modules identified.

## NA-0484 inheritance

NA-0484 is DONE. D-0956 selected
`FUZZ_BINDING_SPLIT_SCOPE_NEEDED` after finding that current qsc fuzz coverage
is parser-boundary oriented for binding purposes.

Inherited current fuzz targets:

- `qsc_route_http.rs`
- `qsc_payload_boundaries.rs`
- `qsc_vault_envelope.rs`

Inherited uncovered binding surfaces:

- A1/B1/A2 handshake frame semantics;
- KEM public-key binding;
- KEM ciphertext binding;
- signature identity/context binding;
- transcript binding;
- replay behavior;
- downgrade and suite-confusion behavior;
- stale public-record and trusted-pin behavior.

Inherited caveats preserved:

- no public-readiness claim;
- no production-readiness claim;
- no public-internet-readiness claim;
- no external-review-complete claim;
- no crypto-complete claim;
- no fuzz-complete claim;
- no vector-complete claim;
- no KEM-complete claim;
- no signature-complete claim;
- no identity-complete claim;
- no transcript-complete claim;
- no qsc/refimpl-equivalence-complete claim;
- no provider-boundary-complete claim;
- no provider-RNG-complete claim;
- no formal-proof-complete claim;
- no replay-proof claim;
- no downgrade-proof claim;
- no side-channel-free claim;
- no vulnerability-free claim;
- no bug-free claim;
- no perfect-crypto claim.

Cargo audit remains dependency-health evidence only.

## Applicable Stewardship and Assurance Review

Level 1 stewardship is active. Level 2 and Level 3 remain future-gated. No
separate Directors, independent READY promotion, independent merge authority,
or independent public-claim authority is created. Lead Director final authority
is preserved.

1. Crypto / Protocol Steward:
   Existing fuzz target files can call public `qsc::adversarial::*` parser
   helpers, but A1/B1/A2 encode/decode, transcript MAC/hash, signature
   message, suite-context admission, KEM decapsulation handling, trusted-pin,
   and session no-mutation behavior are internal to `handshake/mod.rs`.
   A target-only semantic fuzz implementation would either fail to compile or
   test only parser-shaped data. Classification:
   `FUZZ_HELPER_ACCESS_REQUIRES_RUNTIME_SOURCE_DESIGN`.

2. CI / Dependency / Release Health Steward:
   qsc fuzz targets are enumerated in `qsl/qsl-client/qsc/fuzz/Cargo.toml`.
   `scripts/ci/qsc_adversarial.sh` explicitly runs each target and copies
   `fuzz/corpus/<target>` into a temp run directory. The qsc-adversarial
   workflow invokes the script and does not enumerate target names itself.
   Future implementation therefore needs exact Cargo and script scope after
   helper/API design. Workflow mutation is not expected. Classification:
   `FUZZ_EXACT_SCOPE_SPLIT_REQUIRED`.

3. Public Claims / External Review Steward:
   This lane is internal governance evidence only. It improves external-review
   readiness incrementally by identifying the helper/API blocker, but it does
   not claim external review happened or completed. Classification:
   `EXTERNAL_REVIEW_READINESS_INCREMENTAL`.

4. Product / Demo / Service Boundary Steward:
   qsc binding fuzz remains a client-local assurance lane. It does not mutate
   qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public
   docs, demos, or production service paths.

5. Local Ops / Backup / Restore Steward:
   qwork proof files were read and copied; qwork/qstart/qresume were not run.
   qsl-backup SHA was read-only verified as
   `e9ecff3d22ed`, and the codex ops source path appears exactly once in the
   installed qsl-backup source list. No backup, restore, status, plan,
   rollback, or backup tree mutation is performed.

6. Best-Known-Method Review:
   Classification: `BEST_KNOWN_METHOD_FOR_SCOPE`. The best method for this
   scope is to stop short of implementation and authorize a helper/API design
   lane that can decide whether to expose a narrow adversarial handshake
   binding helper, use a process/file harness, or choose a smaller parser
   admission target without changing runtime semantics.

7. Hostile Cryptographer Review:
   A hostile reviewer would reject a parser-only or target-only fuzz lane as
   binding evidence. Future semantic fuzz must exercise fail-closed behavior
   for wrong KEM public key, wrong KEM ciphertext, wrong signature identity or
   public record, cross-message signature replay, transcript mutation, replay,
   suite confusion, stale public record, and trusted-pin mismatch.

8. Red-Team Review:
   Red-team priority remains A1/B1/A2 replay, suite block mutation, stale
   identity records, trusted-pin mismatch, and transcript-field mutation. These
   remain fuzz residuals until a helper/API or equivalent harness can reach the
   semantic binding checks.

9. Production SRE Review:
   Future fuzz must stay bounded, use ephemeral temp roots or in-memory state,
   avoid durable secret material, avoid live relay/service data, and avoid
   long-running process harnesses that would make CI flaky or too expensive.

10. Side-Channel Caveat:
   This lane gives no side-channel-free evidence. Future fuzz would still give
   no side-channel-free evidence. Timing, branching, memory-access, provider,
   and misuse-boundary reviews remain residual.

11. Formal-Model Mapping Residual:
   Classification: `FORMAL_MODEL_MAPPING_SUPPORTING_ONLY`. NA-0478 formal
   tokens can guide scenario selection, but they are not implementation
   equivalence proof, not fuzz proof, and not formal-proof-complete evidence.

12. External-Review Readiness:
   Classification: `EXTERNAL_REVIEW_READINESS_INCREMENTAL`. The lane makes the
   future fuzz prerequisite clearer for reviewers, but no external-review-
   complete claim is made.

13. Release-Claim Boundary:
   no public-readiness claim is made. no production-readiness claim is made.
   no public-internet-readiness claim is made. no external-review-complete
   claim is made. no crypto-complete claim is made. no fuzz-complete claim is
   made. no vector-complete claim is made. no KEM-complete claim is made. no
   signature-complete claim is made. no identity-complete claim is made. no
   transcript-complete claim is made. no qsc/refimpl-equivalence-complete claim
   is made. no provider-boundary-complete claim is made. no provider-RNG-
   complete claim is made. no formal-proof-complete claim is made. no replay-
   proof claim is made. no downgrade-proof claim is made. no side-channel-free
   claim is made. no vulnerability-free claim is made. no bug-free claim is
   made. no perfect-crypto claim is made.

14. Assurance Gap Review Trigger:
   Classification: `ASSURANCE_GAP_REVIEW_REQUIRED_NOW`. NA-0485 selects an
   exact helper/API design successor, but it does not select an exact fuzz
   implementation successor because the target-only implementation path cannot
   safely reach qsc binding semantics from the current public fuzz API.

## Current fuzz target / Cargo / script / CI inventory

Existing fuzz targets:

| Target file | Cargo bin | Input structure | Coverage class | Helper/API imports | Corpus | qsc-adversarial invocation |
|---|---|---|---|---|---|---|
| `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_route_http.rs` | `qsc_route_http` | raw bytes parsed as HTTP request bytes; target and route-token extraction if request parses | parser-only for binding purposes | `qsc::adversarial::route::{parse_http_request_bytes, parse_http_route_token_from_request, parse_http_target}` | `qsl/qsl-client/qsc/fuzz/corpus/qsc_route_http/` | `run_fuzz_target qsc_route_http` |
| `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_payload_boundaries.rs` | `qsc_payload_boundaries` | raw bytes parsed as receipt, file confirm, file transfer, attachment descriptor, and attachment confirm JSON payloads | parser-only for binding purposes | `qsc::adversarial::payload::{parse_receipt_payload, parse_file_confirm_payload, parse_file_transfer_payload, parse_attachment_descriptor_payload, parse_attachment_confirm_payload}` | `qsl/qsl-client/qsc/fuzz/corpus/qsc_payload_boundaries/` | `run_fuzz_target qsc_payload_boundaries` |
| `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_vault_envelope.rs` | `qsc_vault_envelope` | raw bytes parsed as vault envelope bytes | parser-only for binding purposes | `qsc::adversarial::vault_format::parse_vault_envelope` | `qsl/qsl-client/qsc/fuzz/corpus/qsc_vault_envelope/` | `run_fuzz_target qsc_vault_envelope` |

Cargo and CI findings:

- `qsl/qsl-client/qsc/fuzz/Cargo.toml` has one `[[bin]]` stanza per target.
- Adding a new target requires a `[[bin]]` stanza in that Cargo manifest.
- Adding a new target without new dependencies should not require root
  `Cargo.lock` mutation.
- Adding only a new `[[bin]]` without dependency changes should not require
  `qsl/qsl-client/qsc/fuzz/Cargo.lock` mutation.
- `scripts/ci/qsc_adversarial.sh` enumerates target names explicitly, so a
  CI-visible new target requires script inclusion.
- `.github/workflows/qsc-adversarial.yml` invokes the script generically and
  does not enumerate target names; workflow mutation is not expected.
- The script currently expects a checked-in corpus directory for each target.
  A future no-corpus/ephemeral-seed target must either create an empty safe
  corpus directory under exact future scope or adjust the script under exact
  future scope to support generated seed material.

Required classification results:

- `EXISTING_FUZZ_TARGETS_ENUMERATED_OK`.
- `FUZZ_TARGET_ADD_REQUIRES_CARGO_TOML`.
- `FUZZ_TARGET_ADD_REQUIRES_SCRIPT_CHANGE`.
- `FUZZ_TARGET_ADD_REQUIRES_WORKFLOW_CHANGE` is not selected because the
  workflow calls the script.
- `FUZZ_TARGET_ADD_REQUIRES_LOCKFILE` is not selected if dependencies remain
  unchanged.
- `FUZZ_TARGET_ADD_AMBIGUOUS` is not selected for Cargo/script/workflow shape,
  but helper/API access remains the implementation blocker.

## qsc binding fuzz implementation shape review

Option 1 - New qsc binding fuzz target with generated ephemeral state:

- Selection: deferred.
- Reason: correct long-term implementation shape, but current public fuzz API
  does not expose semantic A1/B1/A2 binding helpers.
- Future path candidate after helper/API design:
  `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs`.
- Needs fuzz Cargo.toml: yes after helper/API design.
- Needs qsc_adversarial script: yes after helper/API design.
- Needs workflow: no expected workflow change.
- Needs corpus: no checked-in secret corpus; use generated ephemeral state and
  either empty safe seed directory or script support for seedless execution.
- Needs qsc source helper/API: yes, subject to NA-0486 design.
- Secret-material risk: manageable only with ephemeral generation and no
  durable private material.
- Expected validation after implementation: cargo-fuzz run for the target,
  qsc-adversarial-smoke, manifest JSON validation, formal checks, qsc binding
  negative tests, root audit, nested fuzz lock audit.
- Public-claim caveat: no fuzz-complete claim and no crypto-complete claim.

Option 2 - New qsc binding fuzz target using internal negative vector manifest
metadata:

- Selection: deferred as a seed-class guide.
- Reason: the manifest gives useful case classes, but should not be loaded as
  runtime secret material or treated as public/conformance vectors. Concrete
  inputs should be generated ephemerally.
- Future path candidate after helper/API design:
  `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs`.
- Needs fuzz Cargo.toml: yes after helper/API design.
- Needs script: yes after helper/API design.
- Needs workflow: no expected workflow change.
- Needs corpus: no checked-in corpus first; metadata-only guidance is safe.
- Needs qsc source helper/API: yes.
- Public-claim caveat: no vector-complete claim.

Option 3 - Extend an existing qsc fuzz target:

- Selection: rejected.
- Reason: current targets are route, payload, and vault parser targets. Extending
  one would mix unrelated input grammars and still not reach private handshake
  binding semantics without helper/API design.
- Public-claim caveat: no parser target extension should be represented as
  semantic binding fuzz proof.

Option 4 - Add corpus only:

- Selection: rejected.
- Reason: corpus-only mutation cannot reach semantic binding if the target
  code does not call semantic binding paths. Existing targets would consume the
  corpus as route, payload, or vault parser inputs.
- Public-claim caveat: no corpus-only lane is fuzz binding evidence.

Option 5 - Add helper/API design authorization before fuzz implementation:

- Selection: selected.
- Future successor:
  `NA-0486 -- QSL qsc Binding Fuzz Helper / API Design Authorization Plan`.
- Reason: an exact design lane is needed to choose a narrow adversarial helper
  seam or reject helper exposure before implementation.
- Needs fuzz Cargo.toml in NA-0486: no; NA-0486 is governance-only design.
- Needs qsc_adversarial script in NA-0486: no; NA-0486 is governance-only
  design.
- Needs workflow in NA-0486: no.
- Needs corpus in NA-0486: no.
- Needs qsc source helper/API: NA-0486 must decide this, not implement it.
- Public-claim caveat: no implementation, fuzz-complete, or crypto-complete
  claim.

Option 6 - qsc adversarial script/CI scope first:

- Selection: rejected as first successor.
- Reason: script shape is known enough; the unresolved blocker is helper/API
  access to semantic binding code. Script authorization alone would not make a
  semantic target compile or execute useful binding logic.

Option 7 - side-channel or external review next:

- Selection: rejected as immediate successor.
- Reason: side-channel and external-review residuals remain important, but
  qsc binding fuzz helper/API design is the direct blocker left by NA-0484 and
  is more actionable as the next queue item. no side-channel-free claim is
  made. no external-review-complete claim is made.

Priority ordering:

- qsc binding fuzz helper/API design should outrank side-channel review,
  external-review packaging, refimpl boundary fuzz, and vector-consumer test
  expansion for the immediate successor because it is the direct blocker for
  semantic qsc binding fuzz.
- side-channel review remains residual and cannot be closed by fuzz.
- vector-consumer tests remain useful later, but the internal manifest already
  provides metadata-only seed-class guidance and should not outrank the helper
  API prerequisite.
- refimpl boundary fuzz remains a later supporting lane; it does not replace
  qsc A1/B1/A2 semantic binding fuzz.
- external-review packaging remains later because this lane does not produce
  implementation evidence and makes no external-review-complete claim.

Future fuzz focus after helper/API design:

- parser mutation remains covered by the existing targets;
- semantic binding should be the primary new target;
- vector-manifest metadata should guide seed classes;
- replay, suite-confusion, and stale-record behavior should be first-class
  mutation classes;
- checked-in corpus should remain out of the first implementation unless a
  later exact corpus lane proves public/mutated-only data and no secret
  material.

## helper / API access review

Questions answered:

- Can a future qsc fuzz target call existing parser/handshake frame functions
  directly? It can call current parser helpers under `qsc::adversarial::*`, but
  cannot call private `handshake/mod.rs` helpers such as A1/B1/A2
  encode/decode, transcript MAC/hash, signature-message, suite-context, KEM,
  trusted-pin, or session mutation logic.
- Can it construct enough qsc state without qsc source changes? Not for
  semantic binding fuzz. It could mutate parser-shaped bytes, but that would
  not reach the binding checks.
- Can it stay inside fuzz target code only? Not for semantic binding fuzz under
  current exports.
- Would it need test-only helpers from qsc executable tests? Reusing
  executable-test helpers from a fuzz crate is not selected because those
  helpers are integration-test local and process/fixture heavy.
- Would it need runtime/source helper extraction? Yes, if the future lane wants
  in-process semantic binding fuzz.
- Would it need mock relay/temp-root setup that is too expensive for fuzz?
  Full CLI/mock-relay process harnessing is likely too heavy for cargo-fuzz and
  should be treated as a fallback or separate adversarial harness design.
- Can it use simplified parser-level binding checks rather than full handshake
  state? It can only after NA-0486 defines a narrow adversarial helper that
  preserves runtime semantics and fail-closed behavior.
- Is a process-harness style fuzz target needed? Not selected now; classified
  as too heavy for first binding fuzz design unless NA-0486 proves otherwise.

Selected classification:

- `FUZZ_HELPER_ACCESS_REQUIRES_RUNTIME_SOURCE_DESIGN`.

Rejected classifications:

- `FUZZ_HELPER_ACCESS_TARGET_ONLY_READY`.
- `FUZZ_HELPER_ACCESS_REQUIRES_QSC_TEST_HELPER_REUSE`.
- `FUZZ_HELPER_ACCESS_PROCESS_HARNESS_TOO_HEAVY` as primary strategy, though
  it remains a fallback concern for NA-0486.
- `FUZZ_HELPER_ACCESS_AMBIGUOUS`.

## Cargo / script / CI exact scope review

Questions answered:

- Is `qsl/qsl-client/qsc/fuzz/Cargo.toml` mutation required for a new target?
  Yes, because existing targets are explicitly listed as `[[bin]]`.
- Would adding only a target file compile without Cargo.toml change? No for the
  current cargo-fuzz target enumeration pattern.
- Is `qsl/qsl-client/qsc/fuzz/Cargo.lock` expected to change? No, not if the
  future target uses existing dependencies.
- Is dependency change avoidable? Yes, expected avoidable.
- Is `scripts/ci/qsc_adversarial.sh` mutation required to run the target in CI?
  Yes, because the script enumerates target names.
- Is workflow mutation required? No expected workflow mutation; the workflow
  invokes the script.
- Can future implementation rely on existing PR qsc-adversarial-smoke if the
  script is updated? Yes, subject to PR CI evidence.
- Can local lack of cargo-fuzz be handled by PR CI evidence? Yes, with exact
  local output recorded and PR qsc-adversarial-smoke required.

Selected classification:

- `FUZZ_EXACT_SCOPE_SPLIT_REQUIRED`.

Implementation-scope implication after helper/API design, if selected later:

- likely target plus Cargo plus script;
- no workflow mutation expected;
- no dependency mutation expected;
- no lockfile mutation expected.

## corpus / secret-material exact scope review

Selected strategy:

- no checked-in corpus in the first semantic binding fuzz implementation lane;
- generate identities, KEM material, signatures, transcript materials, and
  mutated messages ephemerally;
- use NA-0483 manifest metadata as seed-class guidance only;
- if a script requires a target corpus directory, use only empty/minimal
  non-secret seeds or adjust the script under exact future scope;
- checked-in corpus, if ever needed, should be a separate future lane with
  explicit proof that it contains public/mutated-only data and no private
  material.

Selected classifications:

- `FUZZ_CORPUS_EPHEMERAL_GENERATION_READY`.
- `FUZZ_CORPUS_METADATA_ONLY_READY`.

Rejected classifications:

- `FUZZ_CORPUS_CHECKED_IN_SAFE_READY` for the first semantic binding lane.
- `FUZZ_CORPUS_UNSAFE_STOP`.

Secret-material boundary:

- no private keys;
- no signing keys;
- no KEM secret keys;
- no passphrases;
- no runtime keys;
- no backup keys;
- no operator data;
- no user data;
- no live service data;
- no private production endpoint data.

## implementation readiness decision

Primary classification:

`FUZZ_BINDING_HELPER_API_DESIGN_NEEDED`

Decision:

NA-0485 does not authorize qsc binding fuzz implementation yet. Current
public fuzz API access is insufficient for semantic binding fuzz, and a
target-only implementation would risk parser-only evidence drift. The next
lane should design the exact helper/API boundary first.

NA-0484 is consumed. Current fuzz/Cargo/script/CI inventory is complete.
Helper/API access is classified. Cargo/script/CI exact scope is classified.
Corpus/secret strategy is classified. Selected successor is exact:

`NA-0486 -- QSL qsc Binding Fuzz Helper / API Design Authorization Plan`

No implementation mutation occurs in NA-0485.

## future scope bundle

Selected NA-0486 successor:

### NA-0486 -- QSL qsc Binding Fuzz Helper / API Design Authorization Plan

Future NA-0486 objective:

Authorize or reject the narrow qsc helper/API design needed for semantic
binding fuzz to reach A1/B1/A2 frame admission, KEM/signature/transcript
binding, replay/suite/stale-record behavior, and no-mutation checks without
changing runtime behavior or expanding public claims.

Future NA-0486 allowed mutation paths:

- `docs/governance/evidence/NA-0486_qsl_qsc_binding_fuzz_helper_api_design_authorization_plan.md`
- `tests/NA-0486_qsl_qsc_binding_fuzz_helper_api_design_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0486 read-only candidate paths:

- `qsl/qsl-client/qsc/src/lib.rs`
- `qsl/qsl-client/qsc/src/adversarial/mod.rs`
- `qsl/qsl-client/qsc/src/adversarial/`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/fuzz/Cargo.toml`
- `qsl/qsl-client/qsc/fuzz/fuzz_targets/`
- `scripts/ci/qsc_adversarial.sh`
- `.github/workflows/qsc-adversarial.yml`
- `inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json`
- `qsl/qsl-client/qsc/tests/kem_signature_transcript_binding_negative.rs`
- `formal/model_qsc_kem_signature_transcript_binding_bounded.py`

Future NA-0486 forbidden unless later exact scope authorizes:

- qsc runtime/source mutation;
- fuzz target mutation;
- fuzz corpus mutation;
- Cargo manifest or lockfile mutation;
- qsc adversarial script mutation;
- workflow mutation;
- dependency mutation;
- qsc executable test mutation;
- refimpl source/test mutation;
- formal/vector mutation;
- qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public
  docs, backup/restore/qsl-backup, qwork/qstart/qresume/qshell mutation;
- public-readiness, production-readiness, external-review-complete,
  crypto-complete, fuzz-complete, vector-complete, replay-proof, downgrade-
  proof, side-channel-free, vulnerability-free, bug-free, or perfect-crypto
  claim.

Candidate implementation shape after NA-0486, not authorized now:

- possible helper path:
  `qsl/qsl-client/qsc/src/adversarial/handshake_binding.rs`;
- possible module export:
  `qsl/qsl-client/qsc/src/adversarial/mod.rs`;
- possible fuzz target:
  `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs`;
- possible Cargo metadata:
  `qsl/qsl-client/qsc/fuzz/Cargo.toml`;
- possible CI script inclusion:
  `scripts/ci/qsc_adversarial.sh`;
- no workflow mutation expected;
- no dependency mutation expected;
- no checked-in secret corpus.

## future validation / marker plan

Common future NA-0486 no-overclaim markers:

- `NA0486_FUZZ_BINDING_SPLIT_SCOPE_CONSUMED_OK`
- `NA0486_NO_SECRET_MATERIAL_IN_FUZZ_INPUTS_OK`
- `NA0486_NO_RUNTIME_CHANGE_OK`
- `NA0486_NO_DEPENDENCY_CHANGE_OK`
- `NA0486_NO_WORKFLOW_CHANGE_OK`
- `NA0486_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0486_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0486_NO_FUZZ_COMPLETE_CLAIM_OK`
- `NA0486_NO_VECTOR_COMPLETE_CLAIM_OK`
- `NA0486_NO_REPLAY_PROOF_CLAIM_OK`
- `NA0486_NO_DOWNGRADE_PROOF_CLAIM_OK`
- `NA0486_ONE_READY_INVARIANT_OK`

Candidate future fuzz markers, if a later implementation lane is authorized:

- `NA0486_FUZZ_A1_MUTATION_TARGET_OK`
- `NA0486_FUZZ_B1_MUTATION_TARGET_OK`
- `NA0486_FUZZ_A2_MUTATION_TARGET_OK`
- `NA0486_FUZZ_SUITE_CONFUSION_TARGET_OK`
- `NA0486_FUZZ_REPLAY_TARGET_OK`
- `NA0486_FUZZ_STALE_PUBLIC_RECORD_TARGET_OK`
- `NA0486_FUZZ_VECTOR_MANIFEST_TRACEABILITY_OK`
- `NA0486_FUZZ_CI_ADVERSARIAL_TARGET_INCLUDED_OK`

Future validation commands for NA-0486 design:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
git diff --check
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed docs/governance/evidence/NA-0486_qsl_qsc_binding_fuzz_helper_api_design_authorization_plan.md --allowed tests/NA-0486_qsl_qsc_binding_fuzz_helper_api_design_authorization_testplan.md --allowed DECISIONS.md --allowed TRACEABILITY.md --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py leak-scan --base origin/main
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
python3 formal/model_qsc_kem_signature_transcript_binding_bounded.py
python3 formal/run_model_checks.py
cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test signature_provider_boundary -- --test-threads=1 --nocapture
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
```

Stop conditions for NA-0486 design:

- helper/API design cannot preserve runtime semantics;
- helper/API design would weaken fail-closed behavior;
- helper/API design would require secret/private material;
- design would require dependency/workflow changes outside exact scope;
- exact successor cannot be selected;
- public claim boundary cannot be preserved.

## public claim / external review / website boundary

NA-0485 is internal governance evidence only. It does not mutate website,
public docs, README, START_HERE, or public technical paper content.

no public-readiness claim is made. no production-readiness claim is made. no
public-internet-readiness claim is made. no external-review-complete claim is
made. no crypto-complete claim is made. no fuzz-complete claim is made. no
vector-complete claim is made. no KEM-complete claim is made. no signature-
complete claim is made. no identity-complete claim is made. no transcript-
complete claim is made. no qsc/refimpl-equivalence-complete claim is made. no
provider-boundary-complete claim is made. no provider-RNG-complete claim is
made. no formal-proof-complete claim is made. no replay-proof claim is made.
no downgrade-proof claim is made. no side-channel-free claim is made. no
vulnerability-free claim is made. no bug-free claim is made. no perfect-crypto
claim is made.

## rejected alternatives

- Extend `qsc_route_http`: rejected because route parsing does not reach
  handshake binding semantics.
- Extend `qsc_payload_boundaries`: rejected because control-payload parsing
  does not reach A1/B1/A2 binding semantics.
- Extend `qsc_vault_envelope`: rejected because vault envelope parsing does
  not reach handshake binding semantics.
- Corpus-only first lane: rejected because existing targets would not consume
  the corpus as binding frames.
- qsc-adversarial script-only first lane: rejected because helper/API access is
  the blocker.
- Workflow-first lane: rejected because the workflow already invokes the
  script generically.
- Side-channel or external-review lane first: rejected as immediate successor
  because helper/API design is the direct fuzz blocker; side-channel and
  external-review residuals remain active.

## backup-impact statement

Backup impact: none.

Codex did not run backup or restore. Codex did not mutate qsl-backup, backup
status, backup plan, rollback paths, `/backup/qsl`, systemd, timer, fstab, or
backup tree paths. qsl-backup SHA was read-only verified as `e9ecff3d22ed`;
the codex ops source path appears exactly once in the installed qsl-backup
source list.

Same-host continuity remains same-host continuity only. no off-host-backup-
complete claim is made. no disaster-recovery-complete claim is made. no
restore-proof claim is made. no backup-complete claim is made.

## next recommendation

After NA-0485 evidence merges and post-merge public-safety is green, close out
NA-0485 and restore exactly one successor:

`NA-0486 -- QSL qsc Binding Fuzz Helper / API Design Authorization Plan`

NA-0486 should decide whether a narrow qsc adversarial helper/API can be
designed without runtime behavior drift, or whether qsc binding fuzz should be
deferred in favor of a different harness or assurance-gap review.
