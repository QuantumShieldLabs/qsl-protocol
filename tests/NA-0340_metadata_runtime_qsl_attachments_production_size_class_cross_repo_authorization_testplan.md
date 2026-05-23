Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-23

# NA-0340 Metadata Runtime qsl-attachments Production Size-Class Cross-Repo Authorization Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the NA-0340 qsl-protocol governance-only authorization plan for a
future qsl-attachments production size-class lane. The plan must preserve the
qshield demo versus qsl-server/qsl-attachments production boundary, identify
source/authority prerequisites, and select an exact NA-0341 successor without
implementing NA-0341.

## Protected Invariants

- Exactly one READY item remains during NA-0340: NA-0340.
- NA-0339 is DONE.
- D-0660 and D-0661 each exist once before the patch.
- D-0662 is added once by NA-0340.
- qsl-attachments production object-size padding remains unimplemented.
- qsl-server production timing/storage behavior remains unimplemented and
  unproven.
- qshield embedded relay/demo proof remains distinct from production service
  proof.
- No claim is added that attachment size, timing metadata, traffic shape, all
  metadata, or padding residuals are hidden.
- No anonymity, metadata-free, untraceable, production-readiness,
  public-internet-readiness, or external-review-complete claim is introduced.

## Allowed Scope

- `docs/governance/evidence/NA-0340_metadata_runtime_qsl_attachments_production_size_class_cross_repo_authorization.md`
- this testplan
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- qsl-attachments implementation changes.
- qsl-server implementation changes.
- qshield runtime implementation changes.
- qsc/qsp/protocol/crypto/state-machine/key-schedule implementation changes.
- Dependency, Cargo manifest, or lockfile changes.
- Workflow, branch-protection, or public-safety configuration changes.
- Website, README, START_HERE, docs/public, qsc-desktop, formal, input,
  tool/refimpl, service, or public-copy changes.
- qsl-attachments production object-size padding implementation.
- qsl-server production timing/storage implementation.
- Public-internet attachment behavior.
- NA-0341 implementation.

## Prior qshield Demo Proof Review Requirements

The evidence must review:

- live NA-0340 scope in `NEXT_ACTIONS.md`;
- NA-0339 qshield embedded relay/demo attachment size-class proof;
- NA-0338 attachment size-class authorization;
- NA-0337 padding bucket proof;
- NA-0335 cover prototype proof;
- qshield attachment source boundary;
- canonical qsl-attachments descriptor/service/encryption-context docs;
- `TRACEABILITY.md` and `DECISIONS.md`.

## qsl-attachments Source / Authority Inventory Requirements

The evidence must record:

- whether qsl-attachments source repo/path is known;
- whether a local checkout exists;
- remote URL if local checkout exists;
- current branch/ref and SHA if local checkout exists;
- worktree cleanliness if local checkout exists;
- CI/test entrypoints known or unknown;
- package/build system known or unknown;
- allowed future files known or unknown;
- production storage/retention model known or unknown;
- deployment/public-internet status known or unknown;
- merge authority known or unknown;
- cross-repo PR permission known or unknown;
- rollback/deploy boundary known or unknown;
- secrets/env status known or unknown;
- backup impact known or unknown;
- conclusion: `READY_FOR_AUTHORIZATION`, `BLOCKED_SOURCE_AUTHORITY`, or
  `DOCS_ONLY_PLANNING`.

## Threat / Value Model Requirements

The evidence must distinguish:

- object-size observability;
- upload/fetch size correlation;
- attachment type or size-class inference;
- queue/storage object distribution;
- retry/fetch timing correlation;
- endpoint compromise;
- production log exposure;
- route/contact relationship leakage;
- content/key compromise;
- broad metadata-free behavior;
- public-internet readiness.

## Cross-Repo Authorization Requirements

The evidence must require a future implementation directive to state:

- exact qsl-attachments repository URL/path;
- exact branch/base SHA;
- exact allowed files;
- exact forbidden files;
- build/test commands;
- CI expectations;
- storage/object model;
- migration requirement;
- retention/purge requirement;
- backup-plan requirement;
- rollback plan;
- production deploy boundary;
- qsl-server integration boundary;
- qshield demo compatibility boundary;
- secret/env handling;
- public-claim boundary;
- external-review recommendation;
- stop conditions.

## Storage / Retention / Purge / Backup Requirements

The evidence must define future-required handling for:

- object lifecycle;
- descriptor lifecycle;
- ciphertext/object lifecycle;
- size-class padding object lifecycle;
- retention duration;
- purge trigger;
- stale object cleanup;
- failed upload cleanup;
- failed fetch cleanup;
- backup inclusion/exclusion;
- log redaction;
- artifact redaction;
- monitoring;
- alert thresholds;
- operator runbook;
- rollback;
- migration/compatibility;
- abuse/cost threshold.

If qsl-attachments source/authority is not exact enough for implementation,
fields must be marked as future prerequisites, not proven production behavior.

## Abuse / DoS / Quota / Latency Matrix Requirements

The matrix must cover:

- valid small production attachment;
- valid medium production attachment;
- valid large production attachment;
- oversized production object;
- malformed descriptor;
- malformed ciphertext/object;
- invalid size-class config;
- repeated invalid upload/fetch attempts;
- retention/purge failure;
- backup growth;
- storage quota exhaustion;
- public internet abuse;
- qsl-server integration mismatch;
- qshield demo compatibility;
- migration/legacy object compatibility;
- external-review-sensitive claim.

Each scenario must state risk, required future bound, future test, failure
mode, stop condition, compatibility impact, and claim boundary.

## qsl-server Integration Boundary Requirements

The evidence must state:

- whether qsl-server integration is needed before implementation;
- whether qshield demo proof can be used only as oracle/reference;
- what public-internet behavior remains unproven;
- whether service deploy/rollback is in scope later;
- which evidence must exist before any public claim.

## External-Review Boundary Requirements

The evidence must state:

- external review remains incomplete;
- production qsl-attachments size-class work is review-sensitive;
- no public claim should imply attachment sizes are hidden;
- no public claim should imply metadata-free, anonymity, or untraceable
  behavior;
- no website/public docs update occurs unless a future explicit directive
  authorizes it;
- stronger claims require implementation, service, deployment where relevant,
  and review evidence.

## Claim-Boundary Requirements

Changed lines must not introduce unsupported affirmative claims for:

- attachment size hidden;
- timing hidden;
- traffic shape hidden;
- padding hides all metadata;
- metadata-free behavior;
- anonymity;
- untraceable behavior;
- production readiness;
- public-internet readiness;
- external review completion;
- quantum-proof hype, unbreakable, guaranteed-secure, or military-grade
  properties.

Allowed matches must be negated, prohibited, future-gated, or classified as
unproven.

## Backup-Impact Requirements

The evidence must state whether the patch creates durable evidence outside the
current backup scope. Expected result: no backup-plan update is required for
NA-0340 because tracked qsl-protocol files stay under `/srv/qbuild/work`.

Future qsl-attachments source, deployment artifacts, large objects, or
non-rebuildable evidence outside current backup coverage must be recorded as a
future prerequisite.

## Required Local Checks

Run applicable qsl-protocol checks:

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- relevant qshield NA-0339, NA-0337, NA-0335, NA-0331, NA-0329, NA-0327,
  NA-0324, NA-0322, NA-0320, NA-0319, and NA-0318 harnesses if directly
  runnable
- `cargo +stable test -p qshield-cli --locked -- --test-threads=1` if feasible
- `cargo +stable build -p qshield-cli --locked` if feasible
- `scripts/ci/demo_cli_smoke.sh`
- `DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh`
- `DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh` if feasible
- metadata runtime plan and phase-2 harnesses if directly runnable
- metadata conformance smoke
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- JSON parse for `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`
- targeted refimpl NA-0310 oracle test
- full refimpl tests if feasible
- qsc NA-0313 harness if directly runnable
- queue and decision helper checks
- scope guard with exact allowed paths
- link-check
- leak-scan added lines
- changed-line overclaim scan
- classifier proof for changed paths
- goal-lint PR body proof

## CI Expectations

- Required PR checks complete green before merge.
- `public-safety` remains required by branch protection.
- Post-merge main `public-safety` completes success.
- No admin bypass, direct push, squash, rebase, branch deletion, or
  branch-protection mutation occurs.

## Successor Handoff

Selected successor must be exact:

`NA-0341 -- Metadata Runtime qsl-attachments Source / Authority Bundle`

The closeout, if executed, must restore exactly one READY item: NA-0341. It
must not implement NA-0341.
