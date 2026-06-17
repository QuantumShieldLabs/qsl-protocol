Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-17

# NA-0490 Binding Fuzz Corpus Secret-Material Validator Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0490 consumes NA-0489/D355 and authorizes the next safest residual before
any future checked-in binding fuzz corpus: a dependency-free repo-local
secret-material validator script.

Primary classification selected:

`BINDING_FUZZ_CORPUS_VALIDATOR_SCRIPT_READY`

The selected successor is:

`NA-0491 -- QSL Binding Fuzz Corpus Secret-Material Validator Implementation Harness`

Future NA-0491 should implement a dedicated script, expected at
`scripts/audit/validate_binding_fuzz_corpus_no_secrets.py`. The script should
inspect all qsc fuzz corpus directories by default, with the future
`qsc_binding_semantics` corpus path treated as the immediate blocker. It should
reject private/secret markers, secret labels, high-entropy unallowlisted
material, operator/user/service data markers, and actual qsc vault secret
filenames or contents if detected. It should permit clearly synthetic public
bytes, mutated public messages, public keys when clearly synthetic/test/public,
vector IDs, manifest category names, expected reject reason strings, small
metadata, and comments documenting internal-only status.

NA-0490 does not implement the validator. NA-0490 adds no corpus, vector, input,
qsc source, qsc fuzz target, qsc fuzz Cargo, qsc fuzz lockfile, script,
workflow, dependency, lockfile, formal, refimpl, service, public-doc, website,
backup, qsl-backup, qwork, qstart, qresume, or qshell mutation.

Future checked-in binding corpus remains blocked until validator
implementation exists and passes. Cargo audit green remains dependency-health
evidence only. This evidence is internal governance evidence only.

## Live NA-0490 scope

Startup READY item:

`NA-0490 -- QSL Binding Fuzz Corpus Secret-Material Validator Authorization Plan`

Allowed mutation paths used by this evidence PR:

- this evidence doc
- `tests/NA-0490_qsl_binding_fuzz_corpus_secret_material_validator_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Read-only inventory paths used:

- qwork startup proof files under `/srv/qbuild/work/NA-0490/.qwork/`
- D352, D353, D354, and D355 response files under
  `/home/victor/work/qsl/codex/responses/`
- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `docs/governance/evidence/NA-0489_qsl_binding_fuzz_ephemeral_seed_strategy_implementation_harness.md`
- `tests/NA-0489_qsl_binding_fuzz_ephemeral_seed_strategy_implementation_testplan.md`
- `inputs/suite2/internal_negative_binding_vectors/`
- `qsl/qsl-client/qsc/fuzz/`
- `qsl/qsl-client/qsc/fuzz/fuzz_targets/`
- `qsl/qsl-client/qsc/fuzz/corpus/`
- `qsl/qsl-client/qsc/fuzz/Cargo.toml`
- `qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `qsl/qsl-client/qsc/src/`
- `qsl/qsl-client/qsc/tests/`
- `scripts/`
- `scripts/ci/`
- `scripts/audit/`
- `tools/`
- `tools/refimpl/`
- `formal/`
- `.github/workflows/`
- root and qsc Cargo files
- read-only local backup boundary paths named by the directive

Forbidden and not performed:

- validator implementation
- script or helper implementation
- corpus/vector/input mutation
- qsc source mutation
- qsc fuzz target mutation
- qsc fuzz Cargo or lockfile mutation
- qsc-adversarial script mutation
- workflow mutation
- dependency or lockfile mutation
- formal or refimpl mutation
- qsl-server, qsl-attachments, qshield, qshield-cli, service, public-doc,
  website, README, START_HERE, backup, qsl-backup, qwork, qstart, qresume,
  qshell, rollback, or backup tree mutation
- no public-readiness claim, no crypto-complete claim, no fuzz-complete claim,
  no corpus-complete claim, no vector-complete claim, no replay-proof claim,
  no downgrade-proof claim, no side-channel-free claim, no vulnerability-free
  claim, no bug-free claim, or no perfect-crypto claim

## qwork proof-file verification

Codex read the operator-provided qwork proof files and did not run qwork,
qstart, or qresume:

- `/srv/qbuild/work/NA-0490/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0490/.qwork/startup.qsl-protocol.json`

Required qwork fields matched:

- `startup_result=OK`
- `lane=NA-0490`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0490/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0490`
- `requested_lane_status=READY`

Freshness proof:

- proof HEAD and live pre-fetch HEAD both matched `feae11ac6c64`
- proof `origin/main` and live pre-fetch `origin/main` both matched
  `feae11ac6c64`
- fetch was performed only after proof/live refs matched
- `origin/main` equals or descends from
  `feae11ac6c64ba25613cc8d08e2698fe9ebb40e0`
- clean `main` was checked against `origin/main`

Startup queue and decision proof:

- READY_COUNT 1
- READY NA-0490
- NA-0489 DONE
- NA-0488 DONE
- NA-0487 DONE
- D-0967 exists once
- D-0968 exists once
- D-0969 absent before this patch
- D-0970 absent before this patch
- duplicate decision count 0

Startup health proof:

- public-safety on `feae11ac6c64`: success
- qsc-adversarial-smoke on `feae11ac6c64`: success
- `/` usage: 88 percent, below the 95 percent hard stop threshold
- `/backup/qsl` usage: 10 percent
- installed `/usr/local/sbin/qsl-backup` SHA256 matched the expected digest
- installed qsl-backup source inclusion count for the Codex ops source was
  exactly 1
- PR #1249 was verified merged at `6e1004e86b55`

## NA-0489 / disk-recovery inheritance

Consumed inheritance sources:

- D352 response:
  `/home/victor/work/qsl/codex/responses/NA0489_20260616T212856Z_D352.md`
- D353 response:
  `/home/victor/work/qsl/codex/responses/NA0489_closeout_restore_na0490_20260616T223437Z_D353.md`
- D354 response:
  `/home/victor/work/qsl/codex/responses/NA0489_disk_pressure_recovery_20260616T225916Z_D354.md`
- D355 response:
  `/home/victor/work/qsl/codex/responses/NA0489_closeout_restore_na0490_20260617T004737Z_D355.md`
- NA-0489 evidence doc and implementation testplan
- D-0967 and D-0968 in `DECISIONS.md`
- NA-0489 closeout entry in `TRACEABILITY.md`
- NA-0489 closeout entries in `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- NA-0490 block in `NEXT_ACTIONS.md`

Inherited facts:

- NA-0489 implementation PR #1249 merged at `6e1004e86b55`.
- NA-0489 implemented deterministic target-local ephemeral seed generation
  inside `qsc_binding_semantics`.
- NA-0489 kept the arbitrary-byte fuzz path and added public/synthetic seed
  recipes for A1 mutation, B1 mutation, A2 mutation, suite confusion, replay,
  stale public-record / trusted-pin, and vector-manifest traceability classes.
- NA-0489 added no checked-in binding corpus.
- NA-0489 added no vector/input mutation.
- NA-0489 added no qsc source/helper mutation outside the fuzz target.
- NA-0489 added no qsc fuzz Cargo, qsc fuzz lockfile, qsc-adversarial script,
  workflow, dependency, lockfile, formal, refimpl, service, public-doc, backup,
  qsl-backup, qwork, qstart, qresume, or qshell mutation.
- D353 verified PR #1249 public-safety success but stopped because `/` reached
  the disk stop threshold and because unsafe response writing invoked prohibited
  startup command names through unquoted heredoc substitution.
- D354 reduced `/` from 96 percent to 88 percent, preserved the active worktree
  and proof files, did not mutate qsl-protocol, did not run qwork/qstart/qresume
  or backup/restore, and used safe response writing.
- D355 closed NA-0489 in-tree, merged closeout PR #1250, verified post-merge
  public-safety and qsc-adversarial-smoke on `feae11ac6c64`, and restored
  NA-0490 as the sole READY item.
- No checked-in binding corpus currently exists.
- The validator residual is active because future checked-in binding corpus is
  blocked until validator implementation.
- No public-readiness claim, no crypto-complete claim, no fuzz-complete claim,
  no corpus-complete claim, no vector-complete claim, no replay-proof claim,
  no downgrade-proof claim, no side-channel-free claim, no vulnerability-free
  claim, no bug-free claim, or no perfect-crypto claim was made.

## Applicable Stewardship and Assurance Review

Level-1 stewardship conclusion:

- The correct next action is authorization for a validator implementation lane,
  not checked-in corpus and not broader assurance work.
- Lead Director authority and exactly-one-READY queue discipline remain
  unchanged.
- Stewards remain advisory only. Level 2 and Level 3 remain future-gated. No
  separate Directors, independent READY promotion, or independent merge
  authority is created.

Best-Known-Method Review:

- A future checked-in corpus needs a deterministic, repo-local, dependency-free
  validator before any corpus bytes land.
- The validator should be explicit about deny classes and safe classes rather
  than relying only on generic token scanning.
- The validator should default to inspecting all qsc fuzz corpora, while
  preserving the immediate binding-corpus blocker.

Hostile Cryptographer Review:

- A corpus can smuggle private keys, passphrases, service tokens,
  production-like endpoints, or secret-key labels under harmless-looking file
  names.
- High-entropy binary corpus material can hide secrets unless each file is
  clearly synthetic, public, or explicitly allowlisted.
- Internal negative vector IDs and manifest categories must remain
  traceability-only and must not become public conformance claims.

Red-Team Review:

- The validator should fail closed on private key headers, obvious tokens,
  passphrase labels, KEM/signature/identity secret-key names, backup/recovery
  key names, runtime secret labels, operator/user data markers, qsc vault
  secret file markers, and high-entropy unallowlisted blobs.
- The validator should report deterministic machine-readable output so future
  lanes can attach evidence without leaking matched secret values.

Production SRE Review:

- A dependency-free Python script under `scripts/audit/` is the lowest-blast
  implementation path.
- No Cargo, qsc fuzz Cargo, lockfile, workflow, qsc-adversarial script, qsc
  source, formal, refimpl, service, public-doc, backup, or qsl-backup mutation
  is needed for initial implementation.
- CI integration can be deferred until the script exists and a later directive
  authorizes the exact workflow or helper hook.

Side-Channel Caveat:

- Secret-material validation is not side-channel review.
- No side-channel-free claim is made. Side-channel and secret-lifecycle
  residuals remain future work.

Formal-Model Mapping Residual:

- The validator maps to corpus hygiene and evidence safety, not to formal
  protocol correctness.
- Existing formal checks remain supporting evidence only. No formal-proof
  completion claim is made.

External-Review Readiness:

- The validator would improve future internal evidence hygiene but is not
  external review.
- No external-review-complete claim is made.

Release-Claim Boundary:

- This lane does not change release readiness, protocol behavior, fuzz
  completeness, corpus completeness, or vector completeness.
- Cargo audit green remains dependency-health evidence only.

Assurance Gap Review Trigger:

- The validator remains higher priority than checked-in corpus work because it
  protects against secret ingress into tracked evidence.
- Vector-consumer tests, fuzz stabilization, side-channel review, and external
  review remain important residuals, but they do not replace the validator gate
  for future corpus bytes.

Assurance classifications:

- `BEST_KNOWN_METHOD_FOR_SCOPE`
- `FORMAL_MODEL_MAPPING_SUPPORTING_ONLY`
- `EXTERNAL_REVIEW_READINESS_INCREMENTAL`
- `HIGHER_PRIORITY_RESIDUAL_SUPERSEDES_ASSURANCE_REVIEW`

## Current corpus / validator surface inventory

Current checked-in qsc fuzz corpus directories:

| Corpus directory | Target association | File count | Current classification |
|---|---:|---:|---|
| `qsl/qsl-client/qsc/fuzz/corpus/qsc_payload_boundaries` | `qsc_payload_boundaries` | 5 | existing parser/payload corpus |
| `qsl/qsl-client/qsc/fuzz/corpus/qsc_route_http` | `qsc_route_http` | 3 | existing parser/route corpus |
| `qsl/qsl-client/qsc/fuzz/corpus/qsc_vault_envelope` | `qsc_vault_envelope` | 2 | existing vault-envelope corpus |

Binding target status:

- `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs` exists.
- `qsc_binding_semantics` is invoked by qsc-adversarial CI.
- `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/` is absent.
- Likely future binding corpus path, if later authorized:
  `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/`.

Current no-corpus status:

- `FUZZ_CORPUS_BINDING_TARGET_NONE`
- Future checked-in corpus remains blocked until validator implementation.

Existing validation and scanning tools:

| Path | Purpose | Arbitrary corpus paths | Private keys | Passphrases | API keys/tokens | High entropy | Deterministic output | CI-ready | Dependency changes needed | Suitable for future corpus validator | Gaps |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| `scripts/ci/qsl_evidence_helper.py leak-scan` | Added-line or full-path high-confidence secret scan | yes | yes, PEM private headers | no | yes | no | yes | yes | no | partial | no passphrase labels, qsc secret labels, corpus allowlist, entropy policy, or structured corpus report |
| `scripts/ci/qsl_evidence_helper.py scope-guard` | Path allow/deny diff guard | not a scanner | no | no | no | no | yes | yes | no | no | path-only |
| `scripts/ci/qsl_evidence_helper.py link-check` | Markdown relative link check | not a scanner | no | no | no | no | yes | yes | no | no | docs links only |
| `scripts/ci/qsl_evidence_helper.py pr-body-preflight` | PR body metadata and overclaim preflight | not a scanner | no | no | no | no | yes | yes | no | no | PR text only |
| `scripts/audit/run_goal_lint_pr.sh` and `tools/goal_lint.py` | Goals-line and governance lint | not a scanner | no | no | no | no | yes | yes | no | no | governance metadata only |
| `scripts/audit/audit_pr_delta.sh` | PR delta audit wrapper | PR-oriented | unknown | unknown | unknown | unknown | yes | yes | no | partial | not corpus-specific |
| `scripts/ci/classify_ci_scope.sh` | Scope classifier for CI behavior | not a scanner | no | no | no | no | yes | yes | no | no | classification only |
| `scripts/ci/public_safety_gate.py` | Required public-safety gate | not a scanner | no | no | no | no | yes | yes | no | no | CI gate orchestration only |
| `scripts/ci/qsc_adversarial.sh` | qsc adversarial tests and cargo-fuzz smoke | not a scanner | no | no | no | no | yes | yes | no | no | executes fuzz targets, does not validate corpus secrets |

Inventory conclusion:

- Existing helper leak-scan is useful but insufficient alone.
- Extending `qsl_evidence_helper.py` would broaden a shared CI/helper surface.
- A dedicated repo-local script is the best initial implementation target.

## Secret-material pattern matrix

Classification:

`SECRET_PATTERN_MATRIX_READY`

Material classes the future validator must reject or flag:

| Class | Required action | Notes |
|---|---|---|
| PEM private key headers | reject | includes common `BEGIN ... PRIVATE KEY` forms |
| OpenSSH private key headers | reject | includes OpenSSH private key marker |
| age, minisign, PGP, or private-key style markers | reject | if present in corpus file names or contents |
| obvious API tokens | reject | GitHub, Slack, AWS, Google, OpenAI, JWT-like, bearer-token-like patterns |
| passphrase labels | reject | `passphrase`, `password`, `secret phrase`, `mnemonic`, `seed phrase`, and similar labels |
| KEM secret key names | reject | `kem_secret`, `kem_sk`, `decapsulation_key`, private KEM labels |
| signature secret key names | reject | `sig_secret`, `signing_secret`, `signing_key`, private signature labels |
| identity secret key names | reject | identity private/secret key labels and private identity files |
| backup or recovery key names | reject | backup key, recovery key, recovery phrase, restore key labels |
| runtime/service secret labels | reject | auth token, bearer token, relay secret, service secret, vault secret, session secret labels |
| private endpoints or production-like identifiers | flag and reject unless later policy explicitly allowlists | corpus should not contain live/private endpoint data |
| high-entropy blobs above threshold | reject unless explicitly allowlisted | exact threshold and allowlist format belong to NA-0491 implementation; recommended starting point is length >=64 bytes plus entropy/long-hex/base64 heuristics |
| actual qsc vault secret filenames or contents | reject | if detected by path names or content markers |
| user/operator data markers | reject | names, local paths, operator IDs, hostnames, user data, or incident artifacts |

Material classes the future validator may permit or classify as safe:

| Class | Required action | Notes |
|---|---|---|
| short synthetic byte arrays | allow | bounded length and clearly synthetic/test/public |
| mutated public message bytes | allow | must not contain secret labels or unallowlisted high entropy |
| public keys | allow only when clearly synthetic/test/public | do not infer safety from key length alone |
| vector IDs | allow | traceability-only IDs, not public vector-complete evidence |
| manifest category names | allow | category names such as binding negative classes |
| non-secret labels | allow | public/synthetic labels and target names |
| expected reject reason strings | allow | no sensitive payload values |
| small structured JSON metadata | allow | if labels are safe and no secret values appear |
| comments documenting internal-only status | allow | internal governance caveats are safe |
| generated ephemeral seeds | not checked in | target-local generation remains preferred; no corpus file should claim generated seed bytes are production material |

Handling rules:

- Synthetic public bytes and mutated public messages are safe only when their
  provenance is explicit and they avoid deny-pattern labels.
- Vector IDs and manifest category names remain traceability-only.
- Generated ephemeral seeds from NA-0489 should stay generated in the fuzz
  target; if later serialized into corpus, they require validator pass plus
  explicit provenance.
- High-entropy material must fail closed unless the future lane defines an
  explicit allowlist with public/synthetic provenance.

## Validator strategy options

| Option | Decision | Evidence | Future path | Validation commands | Dependency/workflow impact | Public-claim caveat | P0/P1/P2 risks |
|---|---|---|---|---|---|---|---|
| Option 1 - extend `qsl_evidence_helper.py leak-scan` for corpus paths | reject as first implementation | helper already scans full paths and common secrets, but shared helper would broaden CI surface and still needs corpus-specific rules | possible later helper reuse after dedicated script stabilizes | `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode full --paths qsl/qsl-client/qsc/fuzz/corpus` | no dependency expected; workflow integration later only | no public-readiness or corpus-complete claim | P1 shared-helper surface creep |
| Option 2 - add dedicated repo-local corpus validator script | select | exact purpose, dependency-free Python, deterministic output, smaller blast radius | `scripts/audit/validate_binding_fuzz_corpus_no_secrets.py` | future `python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --root qsl/qsl-client/qsc/fuzz/corpus --json` | no Cargo, lockfile, dependency, workflow, qsc source, fuzz target, corpus, vector, formal, refimpl, service, public, or backup mutation required | no public-readiness, fuzz-complete, corpus-complete, or vector-complete claim | P0 future checked-in corpus remains blocked until implemented; P1 entropy allowlist design |
| Option 3 - use ad hoc proof-root validator only | reject | proof-root-only checks are not durable and would not protect future PRs | none selected | proof-root commands only | no repo mutation | no public claim | P0 non-durable gate |
| Option 4 - require no checked-in corpus indefinitely | reject for now | safest but loses future deterministic corpus benefits | retain as fallback if validator unsafe | no corpus commands | no impact | no corpus-complete claim | P2 may slow fuzz stabilization |
| Option 5 - vector-consumer tests before corpus validator | reject as successor | useful for deterministic coverage but does not solve secret ingress | future vector-consumer lane may follow validator | Rust/vector test commands | could need qsc/refimpl/test scope later | no vector-complete claim | P1 does not unblock corpus safely |
| Option 6 - external/side-channel review before corpus | reject as successor | broader assurance remains valuable but does not provide a repo-local corpus gate | future assurance/review lane remains residual | review evidence commands | no immediate implementation impact | no side-channel-free or external-review-complete claim | P1 validator blocker remains |

Selected path:

`BINDING_FUZZ_CORPUS_VALIDATOR_SCRIPT_READY`

Rejected alternatives are retained only as future residuals or fallback if the
dedicated script proves unsafe or too broad.

## CI / workflow / dependency impact review

Impact classifications:

- `VALIDATOR_NO_DEPENDENCY_NO_WORKFLOW_READY`
- `VALIDATOR_WORKFLOW_INTEGRATION_LATER`

Review answers:

- Validator implementation can be dependency-free Python using the standard
  library only.
- Validator implementation does not need workflow changes immediately.
- Validator can initially be local/testplan/future-corpus-lane validation.
- Later integration into public-safety, PR preflight, qsc-adversarial, or a
  helper wrapper should require a separate exact directive.
- Validator implementation does not require Cargo changes.
- Validator implementation does not require lockfile changes.
- Validator implementation should inspect all qsc fuzz corpus directories by
  default, while treating a future `qsc_binding_semantics` corpus as the
  immediate blocker and requiring explicit provenance/allowlist for any
  high-entropy corpus files.
- Future corpus lanes should require explicit allowlist/provenance for each
  corpus file that trips high-entropy heuristics.

Rejected classifications:

- `VALIDATOR_CI_INTEGRATION_REQUIRED_NOW`
- `VALIDATOR_SCOPE_SPLIT_NEEDED`
- `VALIDATOR_DEPENDENCY_OR_WORKFLOW_RISK_STOP`

## Authorization decision

Primary classification:

`BINDING_FUZZ_CORPUS_VALIDATOR_SCRIPT_READY`

Decision:

- A binding fuzz corpus secret-material validator should be built before any
  checked-in binding corpus.
- The validator should be a dedicated repo-local dependency-free script.
- The validator should inspect all qsc fuzz corpus directories by default.
- The immediate policy blocker is future
  `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/`.
- Initial run location should be local validation and future corpus
  implementation lanes.
- CI or public-safety integration is deferred until a later exact directive.
- No implementation mutation is authorized in NA-0490.
- No corpus/vector/input mutation is authorized in NA-0490.
- No qsc source, qsc fuzz target, qsc fuzz Cargo, qsc fuzz lockfile,
  qsc-adversarial script, workflow, dependency, lockfile, formal, refimpl,
  service, public-doc, backup, or qsl-backup mutation is authorized in NA-0490.
- Future checked-in corpus remains blocked until validator implementation.
- Vector-consumer tests, fuzz stabilization, side-channel review, and external
  review remain residuals; they do not outrank the validator for the checked-in
  corpus gate.

Required evidence consumed:

- NA-0489/D355 and in-tree D-0968 evidence consumed.
- Current corpus and validator surface inventoried.
- Secret-material pattern matrix defined.
- Validator options reviewed.
- CI/workflow/dependency impact reviewed.
- Selected NA-0491 successor defined.
- No public claim expansion is introduced.
- Exactly one READY remains mandatory until optional closeout.

## Future scope bundle

Selected successor:

`NA-0491 -- QSL Binding Fuzz Corpus Secret-Material Validator Implementation Harness`

Potential allowed future paths:

- `scripts/audit/validate_binding_fuzz_corpus_no_secrets.py`
- `tests/NA-0491_qsl_binding_fuzz_corpus_secret_material_validator_implementation_testplan.md`
- `docs/governance/evidence/NA-0491_qsl_binding_fuzz_corpus_secret_material_validator_implementation_harness.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Expected implementation requirements:

- standard-library Python only
- deterministic JSON report option
- stable non-zero exit for secret findings
- no matched secret value in output
- full-path scan over `qsl/qsl-client/qsc/fuzz/corpus/` by default
- explicit support for the future binding corpus path
- deny rules for private headers, tokens, passphrase labels, secret-key labels,
  backup/recovery/runtime secret labels, operator/user/service markers,
  qsc vault secret markers, and high-entropy unallowlisted content
- allow classification for synthetic public bytes, mutated public messages,
  synthetic/test/public public keys, vector IDs, manifest category names,
  non-secret labels, expected reject strings, small metadata, and
  internal-only comments
- no corpus, vector, input, qsc source, qsc fuzz target, qsc fuzz Cargo,
  qsc-adversarial script, workflow, dependency, lockfile, formal, refimpl,
  service, public-doc, backup, qsl-backup, qwork, qstart, qresume, or qshell
  mutation unless a later exact directive authorizes narrower scope

Forbidden unless later exact scope authorizes:

- corpus/vector/input mutation
- qsc source/fuzz target mutation
- qsc fuzz Cargo mutation
- workflow mutation
- dependency/lockfile mutation
- refimpl/formal/service/public/backup mutation
- checked-in corpus additions
- no public-readiness claim, no crypto-complete claim, no fuzz-complete claim,
  no corpus-complete claim, no vector-complete claim, no replay-proof claim,
  no downgrade-proof claim, no side-channel-free claim, no vulnerability-free
  claim, no bug-free claim, or no perfect-crypto claim

## Future validation / marker plan

Common future NA-0491 markers:

- `NA0491_VALIDATOR_SCOPE_CONSUMED_OK`
- `NA0491_SECRET_PATTERN_MATRIX_READY_OK`
- `NA0491_NO_DEPENDENCY_CHANGE_OK`
- `NA0491_NO_WORKFLOW_CHANGE_OK`
- `NA0491_NO_CORPUS_MUTATION_OK`
- `NA0491_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0491_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0491_NO_FUZZ_COMPLETE_CLAIM_OK`
- `NA0491_NO_CORPUS_COMPLETE_CLAIM_OK`
- `NA0491_NO_VECTOR_COMPLETE_CLAIM_OK`
- `NA0491_ONE_READY_INVARIANT_OK`

Script implementation markers:

- `NA0491_VALIDATOR_SCRIPT_IMPLEMENTED_OK`
- `NA0491_VALIDATOR_REJECTS_PRIVATE_KEY_MARKERS_OK`
- `NA0491_VALIDATOR_REJECTS_SECRET_LABELS_OK`
- `NA0491_VALIDATOR_REJECTS_HIGH_ENTROPY_UNALLOWLISTED_OK`
- `NA0491_VALIDATOR_ALLOWS_SYNTHETIC_PUBLIC_BYTES_OK`
- `NA0491_VALIDATOR_REPORTS_DETERMINISTIC_JSON_OK`

Future validation commands expected:

```bash
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --root qsl/qsl-client/qsc/fuzz/corpus --json
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --self-test
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode full --paths qsl/qsl-client/qsc/fuzz/corpus
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
```

Future negative self-test classes should include:

- private key marker rejection
- OpenSSH private key marker rejection
- token pattern rejection
- passphrase label rejection
- KEM/signature/identity secret-key label rejection
- backup/recovery/runtime secret label rejection
- qsc vault secret marker rejection
- operator/user marker rejection
- high-entropy unallowlisted rejection

Future safe self-test classes should include:

- short synthetic public byte arrays
- mutated public message bytes
- manifest category names
- vector IDs
- expected reject reason strings
- small structured JSON metadata

## Public claim / external review / website boundary

NA-0490 is internal governance evidence only.

This lane makes:

- no public-readiness claim
- no production-readiness claim
- no public-internet-readiness claim
- no external-review-complete claim
- no crypto-complete claim
- no fuzz-complete claim
- no corpus-complete claim
- no vector-complete claim
- no KEM-complete claim
- no signature-complete claim
- no identity-complete claim
- no transcript-complete claim
- no qsc/refimpl-equivalence-complete claim
- no provider-boundary-complete claim
- no provider-RNG-complete claim
- no formal-proof-complete claim
- no replay-proof claim
- no downgrade-proof claim
- no side-channel-free claim
- no vulnerability-free claim
- no bug-free claim
- no perfect-crypto claim

No public docs, website, README, START_HERE, public technical paper, external
website, service, qsl-server, qsl-attachments, qshield, or qshield-cli path is
mutated.

## Rejected alternatives

- Extending `qsl_evidence_helper.py` first is rejected because the helper is a
  shared evidence/CI surface and would still need corpus-specific matrix and
  high-entropy policy.
- Ad hoc proof-root-only validation is rejected because it is not durable.
- No checked-in corpus indefinitely is rejected as the selected successor
  because a validator script can preserve future deterministic corpus options.
- Vector-consumer tests are rejected as the immediate successor because they do
  not prevent secret ingress into a future corpus.
- External/side-channel review before the validator is rejected as the immediate
  successor because those reviews do not provide a repo-local checked-in corpus
  gate.
- Immediate CI/workflow integration is rejected because the script should exist
  and pass locally before a later directive authorizes workflow wiring.

## Backup-impact statement

No backup, restore, qsl-backup, backup status, backup plan, rollback, backup
tree, nightly/local-ops script, or archived path mutation is performed.

Read-only backup boundary evidence:

- installed `/usr/local/sbin/qsl-backup` SHA256 matched the expected digest
- the installed qsl-backup source inclusion count for the Codex ops source was
  exactly 1
- `/backup/qsl` usage was 10 percent

D354 disk pressure recovery remains a P1 local-ops residual. NA-0490 does not
authorize archive, move, delete, cleanup, backup, restore, or qsl-backup work.

## Next recommendation

Proceed to optional closeout only after this NA-0490 evidence PR merges and
post-merge public-safety is green.

The closeout should mark NA-0490 DONE and restore exactly one READY successor:

`NA-0491 -- QSL Binding Fuzz Corpus Secret-Material Validator Implementation Harness`

NA-0491 should implement the dedicated validator script and governance evidence
only. It should not add corpus files, mutate qsc source/fuzz targets/Cargo,
mutate workflows, change dependencies or lockfiles, mutate vectors/inputs,
modify formal/refimpl/service/public/backup paths, or expand public claims.
