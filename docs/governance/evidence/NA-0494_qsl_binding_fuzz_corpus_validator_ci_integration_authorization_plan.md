Status: Supporting
Owner: QSL Director
Last-Updated: 2026-06-18

# NA-0494 QSL Binding Fuzz Corpus Validator CI Integration Authorization Plan

## Executive summary

NA-0494 consumes NA-0493/D364 and authorizes a future CI integration for the
binding fuzz corpus secret-material validator. The selected primary
classification is
`BINDING_FUZZ_VALIDATOR_QSC_ADVERSARIAL_INTEGRATION_READY`.

The validator should be integrated now as the next implementation successor, but
not by this lane. The future implementation should add validator invocations to
`scripts/ci/qsc_adversarial.sh`, because that script is already the qsc fuzz
corpus CI surface and already runs `qsc_binding_semantics`.

This lane does not mutate workflows, scripts, helpers, validators, corpus files,
vectors, inputs, qsc source, qsc fuzz targets, Cargo files, lockfiles,
dependencies, formal models, refimpl, services, public docs, backup tooling, or
qwork tooling. It records the authorization and future scope only.

## Live NA-0494 scope

Allowed evidence paths:

- `docs/governance/evidence/NA-0494_qsl_binding_fuzz_corpus_validator_ci_integration_authorization_plan.md`
- `tests/NA-0494_qsl_binding_fuzz_corpus_validator_ci_integration_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Read-only surfaces inspected:

- qwork proof files under `/srv/qbuild/work/NA-0494/.qwork/`.
- NA-0493/D364 response and evidence.
- `NEXT_ACTIONS.md`, `DECISIONS.md`, and `TRACEABILITY.md`.
- `scripts/audit/validate_binding_fuzz_corpus_no_secrets.py`.
- `scripts/ci/qsc_adversarial.sh`, `scripts/ci/qsl_evidence_helper.py`,
  `scripts/ci/classify_ci_scope.sh`, qshield 4D scripts, goal-lint helper, and
  workflow files.
- qsc fuzz corpus directories and the `qsc_binding_semantics` fuzz target.

All implementation, workflow, script, helper, corpus, source, Cargo,
dependency, formal, refimpl, service, public, backup, and qwork paths remain
read-only in this lane.

## qwork proof-file verification

Codex did not run qwork, qstart, or qresume. It read:

- `/srv/qbuild/work/NA-0494/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0494/.qwork/startup.qsl-protocol.json`

The proofs recorded `startup_result=OK`, lane `NA-0494`, repo `qsl-protocol`,
path `/srv/qbuild/work/NA-0494/qsl-protocol`,
`head_equals_origin_main=yes`, clean worktree/index/untracked state,
`ready_count=1`, `queue_top_ready=NA-0494`, and
`requested_lane_status=READY`. JSON mirrored the required fields.

Proof `HEAD` and proof `origin/main` both matched live local refs before fetch:
`1c4fb02158e23`. Fetch occurred only after that match. `origin/main` still
equaled `1c4fb02158e23` and descended from the required D364 closeout merge.

Startup disk status was below the stop threshold: `/` was 93% used and
`/backup/qsl` was 11% used.

## NA-0493 / D364 inheritance

NA-0493 is DONE and NA-0494 is READY. D-0975 and D-0976 each exist exactly
once. D-0977 was absent at startup. Duplicate decision count was zero.

Inherited facts accepted by NA-0494:

- PR #1257 merged at `b5f140e5bd3a`.
- PR #1258 closeout merged at `1c4fb02158e2`.
- D363's pointer-file stop was recovered in D364.
- `ci-4d-evidence` initially failed because of a transient Cargo registry fetch
  for `aead`; D364 reran the failed job and it completed success.
- The checked-in `qsc_binding_semantics` corpus is present.
- Exactly seven seed files are present.
- Every seed file is 8 bytes.
- The validator passes the binding corpus and the full qsc fuzz corpus with
  zero findings.
- No implementation, corpus, vector, input, qsc source, qsc fuzz target, Cargo,
  lockfile, script, workflow, dependency, formal, refimpl, service, public, or
  backup mutation occurred after NA-0493 closeout.
- NA-0494 was restored as authorization-only work.
- No public-readiness claim is inherited. no crypto-complete claim is inherited.
  no fuzz-complete claim is inherited. no corpus-complete claim is inherited.
  no validator-complete claim beyond bounded internal evidence is inherited.
  no vector-complete claim is inherited. no replay-proof claim is inherited.
  no downgrade-proof claim is inherited. no side-channel-free claim is
  inherited. no vulnerability-free claim is inherited. no bug-free claim is
  inherited. no perfect-crypto claim is inherited.

Current corpus inventory:

- `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics`: 7 files, 56 bytes.
- `qsl/qsl-client/qsc/fuzz/corpus/qsc_payload_boundaries`: 5 files, 810 bytes.
- `qsl/qsl-client/qsc/fuzz/corpus/qsc_route_http`: 3 files, 296 bytes.
- `qsl/qsl-client/qsc/fuzz/corpus/qsc_vault_envelope`: 2 files, 76 bytes.

Current validator proof:

- Binding corpus scan: 7 files, 56 bytes, finding count 0, result `pass`.
- Full qsc fuzz corpus scan: 17 files, 1238 bytes, finding count 0, result
  `pass`.

## Applicable Stewardship and Assurance Review

Level-1 stewardship was applied. Lead Director remains final authority, and
stewards remain advisory only.

Best-Known-Method Review: placing the validator next to qsc-adversarial fuzz
corpus execution is the least broad CI integration surface. It keeps the guard
with the corpus consumer and avoids helper-wide or workflow-wide behavior
changes in the first implementation lane.

Hostile Cryptographer Review: the validator is a secret-material guard, not a
cryptographic proof. The future qsc-adversarial step must fail on findings and
must not allow a missing binding corpus now that NA-0493 checked it in.

Red-Team Review: local-only evidence is insufficient against accidental future
corpus regressions. The first automated gate should inspect the exact binding
corpus and the full qsc fuzz corpus before fuzz execution continues.

Production SRE Review: qsc-adversarial already installs cargo-fuzz and runs
qsc fuzz smoke. The validator is dependency-free Python and should be low
latency compared with cargo-fuzz. Keeping the initial output in CI logs avoids
workflow artifact plumbing in the first implementation.

Side-Channel Caveat: this lane and the selected future validator integration do
not make side-channel claims. The validator scans repository corpus bytes for
disallowed secret material only.

Formal-Model Mapping Residual: formal model checks remain supporting evidence
for binding behavior. They do not prove corpus completeness or validator
completeness.

External-Review Readiness: this authorization improves internal CI guardrails,
but no external-review-complete claim is made.

Release-Claim Boundary: no public claim is expanded. no release claim is
expanded. no production claim is expanded. no crypto claim is expanded. no fuzz
claim is expanded. no corpus claim is expanded. no vector claim is expanded. no
replay claim is expanded. no downgrade claim is expanded. no side-channel claim
is expanded. no vulnerability claim is expanded. no bug claim is expanded.
no perfect-crypto claim is expanded.

Assurance Gap Review Trigger: if validator findings appear, if the binding
corpus disappears, or if qsc corpus layout changes enough that the selected
paths are ambiguous, the future implementation must stop rather than weaken the
validator or silently skip the scan.

## Current validator / CI surface inventory

Current state classification: `VALIDATOR_LOCAL_ONLY`.

The validator script exists at
`scripts/audit/validate_binding_fuzz_corpus_no_secrets.py`. It supports
dependency-free JSON and text output, accepts one or more `--path` arguments,
rejects missing paths by default, and exits nonzero when findings exist. It is
not currently called by workflow files, `scripts/ci/qsc_adversarial.sh`,
`scripts/ci/qsl_evidence_helper.py`, goal-lint, or qshield 4D scripts.

Surface inventory:

| Surface | Current role | Runs validator now | Mutation required | PR/main behavior | Artifact/log value | First integration fit |
|---|---|---:|---|---|---|---|
| Local testplan only | Manual directive validation | No | None | No automatic PR/main gate | Local proof only | Rejected as insufficient |
| PR body preflight | Metadata check for Goals/Impact/No-regression/Tests | No | Helper or script mutation | PR helper only | Text output | Rejected; not a corpus scanner |
| `qsl_evidence_helper.py` leak-scan extension | Central governance and CI evidence helper | No | Helper mutation | Potentially broad PR/main impact | Useful but broad | Deferred |
| public-safety aggregate | Required protection gate | No | Workflow/helper behavior change | PR/main gate | Useful central signal | Deferred |
| `scripts/ci/qsc_adversarial.sh` | qsc adversarial Rust tests and cargo-fuzz smoke | No | Script mutation only | PR/main for non-docs scope | Text log now; JSON later if workflow mutates | Selected |
| qshield `ci-4d-evidence` | 4D evidence lane and artifacts | No | 4D script or workflow mutation | PR/main except docs-only | Artifact-friendly | Rejected for first integration |
| New standalone CI job | Dedicated validator signal | No | Workflow mutation | PR/main gate | Clear signal and artifacts | Rejected for first integration |
| goal-lint | PR body goal metadata | No | Goal-lint mutation | PR only | Not corpus-relevant | Rejected |
| release/public-safety aggregate | Higher-level release gate | No | Workflow/helper mutation | Main/release-oriented | Useful later | Deferred |

## Integration option review

Option 1 - Local/testplan only for now: rejected. It avoids mutation but does
not automatically block future corpus regressions.

Option 2 - Integrate into qsc-adversarial script: selected. It is closest to
the qsc fuzz corpus consumer, requires no workflow mutation, requires no new
dependencies, and can fail closed before cargo-fuzz runs. Future allowed path is
`scripts/ci/qsc_adversarial.sh` plus governance files. Future forbidden paths
remain workflow files, qsc source, qsc fuzz target code, Cargo files, lockfiles,
dependencies, corpus files, vectors, inputs, formal models, refimpl, services,
public docs, and backup paths unless a later directive authorizes them.

Option 3 - Integrate into public-safety helper or evidence helper: deferred.
The central policy gate is valuable, but helper mutation can affect many lanes.
It should follow a narrower qsc-adversarial integration if still needed.

Option 4 - Integrate into `ci-4d-evidence`: rejected for first integration.
The lane is artifact-friendly, but D364 already showed recovery friction around
4D dependency fetches. The corpus validator is better kept near qsc fuzz corpus
execution first.

Option 5 - Add standalone CI job: rejected for first integration. It gives a
clear signal, but it requires workflow mutation and adds a new check context.

Option 6 - Require validator only in future corpus implementation directives:
rejected as primary. It remains a directive-level requirement, but it does not
protect against accidental corpus changes outside corpus lanes.

Option 7 - CI integration authorization split: rejected as unnecessary. This
lane has enough evidence to authorize exact qsc-adversarial implementation
scope for NA-0495.

Risk review:

- P0 risk: silently accepting secret material or a missing binding corpus. The
  future implementation must fail PRs on findings and missing binding corpus.
- P1 risk: broad helper or workflow mutation causing unrelated CI behavior
  drift. The selected successor avoids workflow/helper changes.
- P2 risk: CI log-only output is less convenient than JSON artifacts. The
  initial implementation accepts text log output; JSON artifact upload can be a
  later workflow lane if needed.

## CI impact / failure-mode review

Validator findings should fail CI immediately.

Missing `qsc_binding_semantics` corpus should fail CI now that NA-0493 checked
it in. The future implementation must not use `--allow-missing` for the binding
corpus unless a later directive removes the corpus first.

The future qsc-adversarial integration should scan both:

- `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics`
- `qsl/qsl-client/qsc/fuzz/corpus`

This gives an exact binding-corpus existence gate plus full current qsc fuzz
corpus coverage. Future corpus directories under `qsl/qsl-client/qsc/fuzz/corpus`
are covered by the full-corpus scan automatically, but non-qsc corpus roots are
not authorized by this lane.

Initial output should be text log. JSON proof should continue in local
testplans and may be added as a CI artifact only in a later workflow-authorized
lane. Adding uploaded JSON artifacts now would require workflow mutation.

Classifications:

- `VALIDATOR_CI_FAIL_CLOSED_READY`
- `VALIDATOR_CI_SCAN_ALL_CORPUS_READY`
- `VALIDATOR_CI_SCAN_BINDING_ONLY_READY`
- `VALIDATOR_CI_NO_DEPENDENCY_CHANGE_READY`
- `VALIDATOR_CI_SCRIPT_SCOPE_REQUIRED`
- `VALIDATOR_CI_WORKFLOW_SCOPE_REQUIRED` for JSON artifact upload only, deferred
- `VALIDATOR_CI_HELPER_SCOPE_REQUIRED` for public-safety/evidence-helper
  integration only, deferred

Transient validator execution failures should be treated like other in-scope CI
validation failures: classify first, recover only if the cause is understood
and the corrective action does not weaken fail-closed behavior. A missing Python
runtime on GitHub-hosted runners would be unexpected and should stop rather
than skip the validator.

## Authorization decision

Primary classification:
`BINDING_FUZZ_VALIDATOR_QSC_ADVERSARIAL_INTEGRATION_READY`.

Decision answers:

1. The binding fuzz corpus validator should be integrated into CI now, as the
   next implementation successor.
2. First integration should be in qsc-adversarial smoke via
   `scripts/ci/qsc_adversarial.sh`.
3. Initial integration should not require workflow mutation.
4. Integration requires qsc-adversarial script mutation in future NA-0495.
5. Integration does not require `qsl_evidence_helper.py` mutation now.
6. Integration can use the existing validator script unchanged.
7. Integration should scan the exact binding corpus and the full qsc fuzz
   corpus root. Future qsc corpus directories under that root are included by
   the full-root scan.
8. Validator findings should fail PRs.
9. Missing binding corpus should not be accepted in CI now that the corpus
   exists.
10. Initial CI output should be text log. JSON proof remains local/testplan
   evidence; uploaded JSON artifacts require a later workflow lane.
11. CI integration should be immediate implementation successor, not another
   authorization lane.
12. Exact successor scope, validation commands, and public-claim caveats are
   listed below.

No implementation mutation is performed in NA-0494. No workflow, script,
helper, validator, corpus, vector, input, qsc source, qsc fuzz target, Cargo,
dependency, lockfile, formal, refimpl, service, public, backup, qsl-backup,
qwork, qstart, or qresume mutation is performed in NA-0494.

## Future scope bundle

Selected successor:

```md
### NA-0495 -- QSL Binding Fuzz Corpus Validator qsc-Adversarial Integration Implementation Harness

Status: READY

Goals: G1, G2, G3, G4, G5

Summary:
Implement the NA-0494 authorization by adding the existing binding fuzz corpus
secret-material validator to `scripts/ci/qsc_adversarial.sh` without workflow,
helper, dependency, lockfile, qsc source, qsc fuzz target, corpus, vector, input,
formal, refimpl, service, public-doc, backup, or qwork mutation.

Scope:
- Add fail-closed validator invocation(s) to `scripts/ci/qsc_adversarial.sh`.
- Scan `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics`.
- Scan `qsl/qsl-client/qsc/fuzz/corpus`.
- Do not use `--allow-missing` for the binding corpus.
- Use the existing `scripts/audit/validate_binding_fuzz_corpus_no_secrets.py`
  unchanged.
- Emit text log output in the qsc-adversarial CI log.
- Keep JSON proof in local validation/testplan evidence unless a later workflow
  lane authorizes artifact upload.

Allowed mutation paths:
- `scripts/ci/qsc_adversarial.sh`
- `docs/governance/evidence/NA-0495_qsl_binding_fuzz_corpus_validator_qsc_adversarial_integration_implementation_harness.md`
- `tests/NA-0495_qsl_binding_fuzz_corpus_validator_qsc_adversarial_integration_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden:
- workflow mutation
- `qsl_evidence_helper.py` mutation
- validator script mutation
- corpus/vector/input mutation
- qsc source/fuzz target/Cargo/lockfile mutation
- dependency mutation
- formal/refimpl/service/public/backup/qwork mutation
- public claim expansion
```

## Future validation / marker plan

Common NA-0495 markers:

- `NA0495_VALIDATOR_CI_SCOPE_CONSUMED_OK`
- `NA0495_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0495_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0495_NO_FUZZ_COMPLETE_CLAIM_OK`
- `NA0495_NO_CORPUS_COMPLETE_CLAIM_OK`
- `NA0495_NO_VECTOR_COMPLETE_CLAIM_OK`
- `NA0495_ONE_READY_INVARIANT_OK`

qsc-adversarial integration markers:

- `NA0495_VALIDATOR_QSC_ADVERSARIAL_STEP_INCLUDED_OK`
- `NA0495_VALIDATOR_FAILS_ON_FINDINGS_OK`
- `NA0495_VALIDATOR_SCANS_BINDING_CORPUS_OK`
- `NA0495_VALIDATOR_SCANS_ALL_QSC_CORPUS_OK`
- `NA0495_NO_WORKFLOW_CHANGE_OK`
- `NA0495_NO_DEPENDENCY_CHANGE_OK`

Future validation commands should include:

```bash
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Future negative proof should create proof-root-local synthetic disallowed corpus
fixtures and show the script/validator exits nonzero without modifying the
checked-in corpus.

## Public claim / external review / website boundary

No public-readiness claim is made. no production-readiness claim is made.
no public-internet-readiness claim is made. no external-review-complete claim is
made. no crypto-complete claim is made. no fuzz-complete claim is made.
no corpus-complete claim is made. no validator-complete claim beyond bounded
internal evidence is made. no vector-complete claim is made. no replay-proof
claim is made. no downgrade-proof claim is made. no side-channel-free claim is
made. no vulnerability-free claim is made. no bug-free claim is made.
no perfect-crypto claim is made.

No website, public README, public paper, release note, or marketing/public docs
work is authorized by NA-0494.

Cargo audit green remains dependency-health evidence only.

## Rejected alternatives

Public-safety/evidence-helper integration is rejected as first integration
because it mutates broad helper behavior. It remains a future hardening option.

Standalone CI job is rejected as first integration because it requires workflow
mutation and a new check context.

`ci-4d-evidence` integration is rejected as first integration because the
validator is qsc corpus-specific and should not be coupled to 4D evidence
artifacts first.

PR body/goal-lint integration is rejected because it does not inspect corpus
bytes.

Local-only continuation is rejected because checked-in corpus regressions should
be automatically gated.

## Backup-impact statement

Codex did not run backup or restore. Codex did not mutate qsl-backup, backup
status, backup plan, rollback material, archived path files, nightly/local-ops
scripts, or `/backup/qsl`.

Read-only qsl-backup boundary proof matched the expected helper SHA and the
expected Codex ops source inclusion count.

## Next recommendation

After NA-0494 evidence PR merge and public-safety success, close out NA-0494 and
restore `NA-0495 -- QSL Binding Fuzz Corpus Validator qsc-Adversarial
Integration Implementation Harness` as the sole READY successor. Do not
implement NA-0495 in NA-0494.
