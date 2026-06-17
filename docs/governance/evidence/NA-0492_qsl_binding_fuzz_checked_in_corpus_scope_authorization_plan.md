Status: Supporting
Owner: QSL Director
Last-Updated: 2026-06-17

# NA-0492 QSL Binding Fuzz Checked-In Corpus Scope Authorization Plan

## Executive summary

NA-0492 is authorization-only. It recovers the D359 proof-output path stop,
consumes the NA-0491 validator implementation evidence, inventories the current
binding fuzz corpus state, and authorizes a narrowly bounded future
`qsc_binding_semantics` checked-in corpus implementation lane.

Primary classification: `BINDING_FUZZ_MINIMAL_CORPUS_IMPLEMENTATION_READY`.
Current corpus state: `BINDING_CORPUS_ABSENT_VALIDATOR_READY`.
Validator gate state: `VALIDATOR_GATE_READY_FOR_CORPUS_SCOPE`.
CI and dependency impact: `CORPUS_DATA_ONLY_NO_CARGO_SCRIPT_WORKFLOW_READY`.

Selected successor:
`NA-0493 -- QSL Binding Fuzz Checked-In Corpus Implementation Harness`.

The future corpus is limited to exactly seven small raw libFuzzer seed files
under `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/`. This directive
does not add those files. It does not mutate corpus, vectors, inputs, qsc source,
qsc fuzz target code, qsc fuzz Cargo files, lockfiles, qsc-adversarial scripts,
workflows, dependencies, formal models, refimpl, services, public docs, website,
backup tooling, qwork tooling, or runtime behavior.

## Live NA-0492 scope

Allowed mutation paths for this evidence lane:

- `docs/governance/evidence/NA-0492_qsl_binding_fuzz_checked_in_corpus_scope_authorization_plan.md`
- `tests/NA-0492_qsl_binding_fuzz_checked_in_corpus_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

All other qsl-protocol paths were read-only. The only local recovery mutation
authorized by the directive was the exact file
`/tmp/na0492_cargo_fuzz_version.out`, if present.

## qwork proof-file verification

Codex did not run qwork, qstart, or qresume. It read the qwork proof files:

- `/srv/qbuild/work/NA-0492/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0492/.qwork/startup.qsl-protocol.json`

The `.kv` proof recorded `startup_result=OK`, lane `NA-0492`, repo
`qsl-protocol`, path `/srv/qbuild/work/NA-0492/qsl-protocol`,
`head_equals_origin_main=yes`, clean worktree/index/untracked state,
`ready_count=1`, `queue_top_ready=NA-0492`, and
`requested_lane_status=READY`. The JSON proof mirrored the required fields.

Proof freshness was checked before fetch: proof HEAD and proof `origin/main`
matched the live local refs. Fetch happened only after that match.

## D359 stop recovery

D359 stopped before qsl-protocol repo mutation because a local cargo-fuzz
availability probe wrote output to `/tmp/na0492_cargo_fuzz_version.out`, outside
the approved proof root. D359 partial observations are treated as inherited
observations only, not as authorization decisions.

For D360, Codex created proof root:

`/srv/qbuild/tmp/NA0492_checked_in_corpus_scope_authorization_recovery_20260617T140221Z`

The stray file existed. Codex recorded `ls -l`, `sha256sum`, copied it into the
proof root as `local_recovery/na0492_cargo_fuzz_version.out`, summarized it
without trusting it as proof, deleted exactly that `/tmp` path, and verified the
path was absent. No other `/tmp` path was deleted or modified.

After the recovery guard, `PROOF_DIR` and `TMPDIR` pointed into the D360 proof
root. Command output was written under the proof root, allowed repo files, or
the final response file.

## NA-0491 / D358 inheritance

NA-0491 implemented
`scripts/audit/validate_binding_fuzz_corpus_no_secrets.py` and was closed by
D358. D-0971 records the validator implementation, and D-0972 records NA-0491
closeout plus NA-0492 restoration.

Inherited validator behavior:

- deterministic JSON and text output.
- recursive scanning of supplied corpus paths.
- explicit `--allow-missing` required for absent paths.
- no symlink following.
- private-key marker rejection.
- secret-label rejection for passphrase, KEM secret-key, signature secret-key,
  identity secret-key, backup/recovery key, runtime/service secret, private
  endpoint, operator/user data, and qsc secret filename/path label classes.
- high-entropy unallowlisted material rejection.
- safe synthetic public bytes and manifest/category labels accepted.
- current qsc fuzz corpus passes with zero findings.
- missing `qsc_binding_semantics` corpus passes only with explicit
  `--allow-missing`.

Startup disk status for this directive was below the stop threshold:
`/` was about 85.44% used, and `/backup/qsl` was checked read-only. The
installed qsl-backup helper matched the expected SHA-256 digest, and the Codex
ops source appeared exactly once in the helper source list. Codex did not run
backup or restore.

No public-readiness claim is made. no crypto-complete claim is made. no
fuzz-complete claim is made. no corpus-complete claim is made. no
vector-complete claim is made. no replay-proof claim is made. no
downgrade-proof claim is made. no side-channel-free claim is made. no
vulnerability-free claim is made. no bug-free claim is made. no perfect-crypto
claim is made. no external-review-complete claim is made.

## Applicable Stewardship and Assurance Review

Level-1 stewardship and D328 assurance requirements were applied as advisory
review gates. Lead Director authority remains final.

- Best-Known-Method Review: use a small raw libFuzzer corpus only after the
  no-secret validator exists, keep traceability in governance evidence, and keep
  the corpus itself data-only.
- Hostile Cryptographer Review: corpus seeds must be public/synthetic selector
  inputs only; they must not embed private keys, secret labels, production-like
  identifiers, endpoint fragments, live user/operator data, or high-entropy
  unallowlisted material.
- Red-Team Review: the future PR must prove the validator passes every corpus
  file and must preserve fail-closed behavior if any validator finding appears.
- Production SRE Review: no workflow, dependency, Cargo, script, runtime,
  service, backup, restore, or public-doc change is needed for the selected
  future corpus lane.
- Side-Channel Caveat: the corpus can improve deterministic fuzz starting
  coverage, but no side-channel-free claim is made.
- Formal-Model Mapping Residual: vector/formal mapping remains supporting
  evidence only; the corpus is not a formal proof.
- External-Review Readiness: this is incremental internal governance evidence,
  not external-review-complete evidence.
- Release-Claim Boundary: cargo audit green is dependency-health evidence only.
  Corpus authorization evidence is internal governance evidence only.
- Assurance Gap Review Trigger: if the future corpus needs source, target,
  script, workflow, dependency, vector, input, formal, refimpl, service,
  public-doc, or backup mutation, the future lane must stop or split.

## Current binding corpus scope inventory

Existing qsc fuzz corpus directories:

- `qsc_payload_boundaries`: 5 files.
- `qsc_route_http`: 3 files.
- `qsc_vault_envelope`: 2 files.

`qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/` is absent.

The binding fuzz target exists at
`qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs`. It already
routes direct input categories and also generates target-local ephemeral seeds
for A1 mutation, B1 mutation, A2 mutation, suite-confusion, replay, stale
public-record, and vector-manifest traceability.

The helper category mapping in qsc source maps first-byte selectors 0 through 4
to A1, B1, A2, suite-confusion, and replay; other direct values map to stale
public-record. The fuzz target treats first byte `0xff` as the direct
vector-traceability selector, while the target-local ephemeral seed routing uses
first byte modulo 7.

The internal negative vector manifest exists at
`inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json`
and includes qsc binding categories such as wrong peer public key, stale public
record, wrong ciphertext, signature context mismatch, transcript mutation,
replay, suite confusion, downgrade-like suite block, and stale trusted pin. The
manifest remains traceability-only and is not read by the fuzz target at runtime.

`scripts/ci/qsc_adversarial.sh` includes `qsc_binding_semantics` and applies
`--cfg qsc_binding_fuzz_helper` for that target. Local `cargo fuzz` availability
was not present in this environment; the probe exited 101 and was recorded under
the D360 proof root. That is local availability evidence only.

## Checked-in corpus option review

Option 1, small checked-in public/synthetic corpus now: selected for the future
NA-0493 implementation lane. Evidence: target categories exist, deterministic
ephemeral generation exists, validator exists and passes current corpus, and a
small data-only corpus can improve deterministic starting coverage. Future
allowed paths are the exact seed files listed in the future scope bundle plus
NA-0493 governance files. Future forbidden paths include qsc source, fuzz target
code, Cargo files, lockfiles, scripts, workflows, dependencies, vectors, inputs,
formal models, refimpl, services, public docs, backup tooling, and qwork
tooling. Validation requires validator pass, scope guard, leak scan, overclaim
scan, local tests, root audit, nested qsc fuzz lock audit, and PR CI. Public
claim caveat: no public-readiness claim, no fuzz-complete claim, no
corpus-complete claim, and no crypto-complete claim. P0 risk is secret-like
material in corpus; validator gate mitigates. P1 risk is overclaiming corpus
meaning; governance caveats mitigate. P2 risk is corpus upkeep; exact seven-file
scope mitigates.

Option 2, no checked-in corpus yet: rejected as the primary path because the
validator now resolves the blocker that previously favored ephemeral-only
seeding. It remains the fallback if NA-0493 validation fails.

Option 3, corpus validator CI integration first: rejected as a prerequisite for
NA-0493 because workflow/script mutation is outside this lane and the future
implementation directive can require local validator proof before commit and PR
body evidence. Validator CI integration remains a later hardening candidate.

Option 4, vector-consumer tests before corpus: rejected as primary because it
does not exercise libFuzzer corpus behavior. It remains a separate residual.

Option 5, category-split corpus authorization: rejected because seven small
files are reviewable in one implementation lane. Splitting would add governance
overhead without clear safety benefit after validator readiness.

Option 6, fuzz stabilization or run-budget lane first: rejected as a prerequisite
because the target and qsc-adversarial inclusion already exist. It remains a
later performance/stability residual if PR CI shows instability.

## Corpus format / file strategy review

Future corpus format: raw binary libFuzzer input files, not text fixtures.

Future corpus path:

`qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/`

Exact future files:

- `seed_00_a1_mutation.bin`
- `seed_01_b1_mutation.bin`
- `seed_02_a2_mutation.bin`
- `seed_03_suite_confusion.bin`
- `seed_04_replay.bin`
- `seed_05_stale_public_record.bin`
- `seed_ff_vector_traceability.bin`

Exact future file count: 7 files. No extra corpus files or corpus README are
authorized by this plan.

Maximum file size: 64 bytes per file. Each file must encode only a selector byte
and short synthetic bytes. The selector bytes must be `0x00`, `0x01`, `0x02`,
`0x03`, `0x04`, `0x05`, and `0xff` for the categories listed above.

No file may contain strings that look secret-like. No file may contain private
keys, secret labels, passphrases, production-like endpoints, live identifiers,
operator data, user data, backup/recovery keys, high-entropy unallowlisted
material, or long hex/base64-like blobs. Traceability must live in NA-0493
governance evidence and testplan text, not in corpus metadata.

Every future file must pass
`scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics`
before commit.

## Validator-gate review

Validator proof in D360:

- current corpus scan passed: 10 files, 1182 bytes, zero findings.
- missing binding corpus with `--allow-missing` passed as `missing_allowed`.
- missing binding corpus without `--allow-missing` exited nonzero.

Classification: `VALIDATOR_GATE_READY_FOR_CORPUS_SCOPE`.

Future NA-0493 must run the validator before commit and record JSON evidence in
the proof root. The future PR body should mention the validator output summary.
Validator CI integration may follow after the minimal corpus implementation; it
is not required before the seven-file data-only corpus lane because workflow
mutation is not part of the selected implementation scope.

## CI / workflow / dependency impact review

Future checked-in corpus implementation should be data-only plus governance. It
does not require Cargo changes, qsc-adversarial script changes, workflow
changes, dependency changes, or lockfile changes. The existing
`qsc_adversarial.sh` script already includes `qsc_binding_semantics` and will
copy the target corpus directory into the fuzz run directory if present.

Validator CI integration would require workflow or script scope and is therefore
not selected for NA-0493. If future implementation discovers that corpus
behavior requires qsc source, fuzz target, qsc fuzz Cargo, qsc fuzz lockfile,
script, workflow, dependency, vector/input, formal, refimpl, service, public-doc,
or backup mutation, it must stop or split.

Classification: `CORPUS_DATA_ONLY_NO_CARGO_SCRIPT_WORKFLOW_READY`.

## Authorization decision

D359 stop is recovered. NA-0491/D358 evidence is consumed. Current binding
corpus scope is inventoried. Corpus options, corpus format/file strategy,
validator gate, and CI/workflow/dependency impacts are reviewed.

Primary classification:
`BINDING_FUZZ_MINIMAL_CORPUS_IMPLEMENTATION_READY`.

Selected successor:
`NA-0493 -- QSL Binding Fuzz Checked-In Corpus Implementation Harness`.

This directive performs no implementation mutation and no corpus/vector/input
mutation. It performs no runtime, crypto, dependency, Cargo, lockfile, workflow,
test, fuzz target, formal, refimpl, service, public-doc, website, backup,
restore, or qwork tooling mutation. It expands no public claims. Exactly one
READY item remains mandatory.

## Future scope bundle

Future NA-0493 may mutate only:

- `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/seed_00_a1_mutation.bin`
- `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/seed_01_b1_mutation.bin`
- `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/seed_02_a2_mutation.bin`
- `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/seed_03_suite_confusion.bin`
- `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/seed_04_replay.bin`
- `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/seed_05_stale_public_record.bin`
- `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/seed_ff_vector_traceability.bin`
- `docs/governance/evidence/NA-0493_qsl_binding_fuzz_checked_in_corpus_implementation_harness.md`
- `tests/NA-0493_qsl_binding_fuzz_checked_in_corpus_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0493 must not mutate qsc source, qsc fuzz target code, qsc fuzz Cargo
files, qsc fuzz lockfiles, root Cargo files, qsc-adversarial scripts, workflows,
dependencies, vectors, inputs, formal models, refimpl, qsl-server,
qsl-attachments, qshield runtime, qshield-cli, services, public docs, README,
START_HERE, website, backup tooling, backup plans/status, rollback trees,
qwork/qstart/qresume/qshell, or public claim surfaces unless a later exact
directive authorizes a narrower split.

## Future validation / marker plan

Common future markers:

- `NA0493_CORPUS_SCOPE_CONSUMED_OK`
- `NA0493_VALIDATOR_GATE_PASSED_OK`
- `NA0493_NO_SECRET_MATERIAL_IN_CORPUS_OK`
- `NA0493_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0493_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0493_NO_FUZZ_COMPLETE_CLAIM_OK`
- `NA0493_NO_CORPUS_COMPLETE_CLAIM_OK`
- `NA0493_NO_VECTOR_COMPLETE_CLAIM_OK`
- `NA0493_NO_REPLAY_PROOF_CLAIM_OK`
- `NA0493_NO_DOWNGRADE_PROOF_CLAIM_OK`
- `NA0493_ONE_READY_INVARIANT_OK`
- `NA0493_CORPUS_QSC_BINDING_SEMANTICS_PATH_OK`
- `NA0493_CORPUS_FILES_PUBLIC_SYNTHETIC_ONLY_OK`
- `NA0493_CORPUS_A1_SEED_OK`
- `NA0493_CORPUS_B1_SEED_OK`
- `NA0493_CORPUS_A2_SEED_OK`
- `NA0493_CORPUS_SUITE_CONFUSION_SEED_OK`
- `NA0493_CORPUS_REPLAY_SEED_OK`
- `NA0493_CORPUS_STALE_PUBLIC_RECORD_SEED_OK`
- `NA0493_CORPUS_VALIDATOR_JSON_RECORDED_OK`

Future validation should include exact scope guard, validator JSON output, leak
scan, overclaim scan, internal vector manifest JSON validation, formal model
checks, qsc binding negative tests with and without `qsc_binding_fuzz_helper`,
refimpl signature provider-boundary test, refimpl `pqkem768`, root cargo audit,
nested qsc fuzz lock audit, cargo fmt, qsc-adversarial shell syntax, and PR CI.

## Public claim / external review / website boundary

This evidence is internal governance evidence only. It is not public website
content and does not authorize README, START_HERE, docs/public, website, or
external-review package changes.

No public-readiness claim is made. no production-readiness claim is made. no public-internet-readiness claim is made. no external-review-complete claim is made. no crypto-complete claim is made. no fuzz-complete claim is made. no corpus-complete claim is made. no vector-complete claim is made. no replay-proof claim is made. no downgrade-proof claim is made. no side-channel-free claim is made. no vulnerability-free claim is made. no bug-free claim is made. no perfect-crypto claim is made.

## Rejected alternatives

- Keeping ephemeral-only seeding as the primary path is rejected because the
  validator now exists and the seven-file data-only corpus can be bounded.
- Requiring validator CI integration first is rejected as a prerequisite because
  it requires workflow/script scope and is better handled as a later hardening
  lane.
- Vector-consumer tests first are rejected as primary because they do not
  exercise libFuzzer corpus behavior.
- Category-split corpus authorization is rejected because the exact seven-file
  scope is small enough to review at once.
- Fuzz stabilization first is rejected as a prerequisite because the target and
  qsc-adversarial inclusion already exist; instability remains a future stop or
  split trigger if observed.

## Backup-impact statement

No backup, restore, qsl-backup, backup status, backup plan, rollback, archive,
or `/backup/qsl` mutation is performed. The qsl-backup boundary was checked
read-only. The selected NA-0493 successor does not require backup-plan changes
because it is limited to tracked qsl-protocol corpus seed and governance files.

## Next recommendation

Merge this NA-0492 authorization evidence after validation and green PR checks.
If post-merge public-safety is green, close out NA-0492 and restore:

`NA-0493 -- QSL Binding Fuzz Checked-In Corpus Implementation Harness`

NA-0493 should add only the exact seven validator-gated raw corpus seed files
and governance evidence listed above.
