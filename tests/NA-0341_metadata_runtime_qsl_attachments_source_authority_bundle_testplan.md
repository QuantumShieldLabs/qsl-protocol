Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-23

# NA-0341 Metadata Runtime qsl-attachments Source Authority Bundle Test Plan

## Objective

Verify that NA-0341 records a qsl-attachments source/authority bundle before
any production size-class implementation authorization, or records exact
blockers when source/authority remains incomplete.

## Protected invariants

- NA-0341 is qsl-protocol governance/evidence only.
- qsl-attachments is inspected read-only and not mutated.
- qsl-server is not mutated.
- qshield runtime, qsc/qsp/protocol/crypto/key schedule, dependencies,
  workflows, branch protection, public-safety configuration, website, README,
  START_HERE, docs/public, and production-service paths are not changed.
- qshield embedded relay/demo evidence remains distinct from qsl-server and
  qsl-attachments production behavior.
- No claim is added that attachment size, timing metadata, traffic shape, or
  all metadata is hidden.
- No anonymity, metadata-free, untraceable, production-readiness,
  public-internet-readiness, or external-review-complete claim is introduced.

## Allowed scope

- `docs/governance/evidence/NA-0341_metadata_runtime_qsl_attachments_source_authority_bundle.md`
- `tests/NA-0341_metadata_runtime_qsl_attachments_source_authority_bundle_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsl-attachments implementation changes.
- qsl-server implementation changes.
- qshield runtime changes.
- qsc/qsp/protocol/crypto/key-schedule changes.
- `Cargo.toml`, `Cargo.lock`, dependency updates, and workflow changes.
- branch-protection or public-safety configuration changes.
- website, README, START_HERE, docs/public, qsc-desktop, formal, input,
  tool/refimpl, app runtime, service implementation, production-service, or
  public-copy changes.

## Prior source-authority review requirements

The evidence must review:

- live NA-0341 queue scope;
- NA-0340 cross-repo source/authority result;
- NA-0339 qshield embedded relay/demo attachment size-class proof;
- qshield demo versus qsl-attachments production boundary;
- qsl-server production timing/storage boundary.

## qsl-attachments source discovery requirements

The evidence must list all local qsl-attachments paths found, identify the
selected local source if one exists, and record:

- local path;
- remote URL(s);
- current branch/ref;
- current HEAD SHA;
- local cleanliness;
- detached status;
- local `origin/main` or equivalent;
- source freshness caveats;
- mutation authority status.

## Source/authority matrix requirements

The evidence must include a matrix covering:

- local path exists yes/no;
- repository name;
- remote URL(s);
- current HEAD SHA;
- current branch/ref;
- detached yes/no;
- clean yes/no;
- origin/main or equivalent ref present yes/no;
- branch freshness known yes/no;
- latest remote known yes/no without fetching;
- PR/merge authority known yes/no;
- cross-repo mutation authority known yes/no;
- future branch naming rule;
- future base SHA requirement;
- future allowed files known yes/no;
- future forbidden files known yes/no;
- build/test commands known yes/no;
- CI entrypoints known yes/no;
- secrets/env required yes/no/unknown;
- storage/retention/purge model known yes/no;
- deploy/rollback boundary known yes/no;
- backup impact known yes/no;
- qsl-server integration requirement known yes/no;
- public-claim boundary known yes/no;
- classification.

## Future authorization requirements

The evidence must state that any future qsl-attachments implementation
authorization requires exact repo/path, local path, base branch/ref, base SHA,
freshness proof, mutation authority, PR/merge authority, allowed files,
forbidden files, build/test/CI/lint commands, storage/object lifecycle,
descriptor/ciphertext lifecycle, size-class table/cap, migration requirement,
retention/purge requirement, backup-plan requirement, rollback plan, deploy
boundary, qsl-server boundary, qshield demo compatibility boundary,
secrets/env handling, public-claim boundary, external-review recommendation,
branch/PR policy, verification bundle, and stop conditions.

## File-map requirements

The evidence must identify probable future upload, fetch, object storage,
descriptor/ciphertext, tests, docs/evidence, and config surfaces from local
source when available. It must mark dependencies/manifests, workflows,
deployment/secrets, unrelated refactors, public website/public docs,
protocol/crypto/key schedule, qsl-server, qshield runtime, and qsc/qsp as
forbidden by default unless exact future authorization exists.

## Build/test/CI/secrets/deploy/rollback discovery requirements

The evidence must use only read-only qsl-attachments inspection. It must list
local manifests, workflow files, README/docs facts, build/test/lint commands
derived from local files, secrets/env references, deploy docs, rollback/update
docs, and backup/storage docs. It must not run networked tests, service tests
requiring secrets, destructive storage tests, dependency installation,
migrations, or deployment commands.

## Storage/retention/purge/backup requirements

The evidence must describe future prerequisites for object lifecycle,
descriptor lifecycle, ciphertext lifecycle, size-class object lifecycle,
retention duration, purge trigger, stale object cleanup, failed upload cleanup,
failed fetch cleanup, backup inclusion/exclusion, log redaction, artifact
redaction, monitoring, alert thresholds, operator runbook, rollback,
migration/compatibility, abuse/cost thresholds, and qsl-server interaction.

## Production-boundary requirements

The evidence must state that qsl-attachments production object-size padding and
qsl-server production timing/storage behavior remain unimplemented and
unproven. qshield demo proof must remain reference/oracle evidence only.

## Claim-boundary requirements

The changed lines must not introduce affirmative claims of attachment-size
hiding, timing hiding, traffic-shape hiding, all-metadata hiding,
metadata-free behavior, anonymity, untraceable behavior, production readiness,
public-internet readiness, external review completion, quantum-proof,
unbreakable, guaranteed secure, or military-grade behavior.

## Backup-impact requirements

The evidence must state whether NA-0341 changes durable evidence locations,
response paths, source roots, excluded backup paths, or non-rebuildable
artifacts outside backup scope. Expected NA-0341 result: no backup-plan update
required for qsl-protocol governance-only changes under `/srv/qbuild/work`,
with a future prerequisite for any qsl-attachments artifacts outside current
backup scope.

## Required local checks

- `date --iso-8601=seconds`
- `date -u --iso-8601=seconds`
- `df -BG /srv/qbuild`
- `df -h /backup/qsl || true`
- `git status --porcelain=v1 --branch`
- `git diff --name-only || true`
- `git ls-files --others --exclude-standard || true`
- `git fetch --all --prune` for qsl-protocol only
- `git rev-parse origin/main`
- PR-state checks for #943, #942, #941 through #827, #750, #722, and #708
- branch-protection and public-safety proof
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- START_HERE classifier proof commands from the directive
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- qsl-attachments read-only source inventory commands
- `cargo fmt --check`
- targeted qshield NA-0339, NA-0337, NA-0335, NA-0331, NA-0329, NA-0327,
  NA-0324, NA-0322, NA-0320, NA-0319, and NA-0318 harnesses if directly
  runnable
- `cargo +stable test -p qshield-cli --locked -- --test-threads=1`
- `cargo +stable build -p qshield-cli --locked`
- `scripts/ci/demo_cli_smoke.sh`
- `DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh`
- `DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh`
- metadata runtime plan and phase-2 harnesses
- `scripts/ci/metadata_conformance_smoke.sh`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- JSON parse for `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`
- targeted and full refimpl tests
- qsc NA-0313 harness if directly runnable
- queue/decisions/scope/link/leak/goal-lint validation before PR merge.

## CI expectations

- qsl-protocol protected branch keeps `public-safety` required.
- PR checks attach and complete successfully.
- `public-safety` remains green before merge and after merge.
- No admin bypass, squash, rebase, direct push, branch deletion command, or
  delete-branch flag is used.

## Successor handoff

If source/authority remains partial, restore:

`NA-0342 -- Metadata Runtime qsl-attachments Source / Authority Blocker Resolution`

NA-0342 must not implement production size-class behavior unless a later exact
directive separately authorizes implementation.
