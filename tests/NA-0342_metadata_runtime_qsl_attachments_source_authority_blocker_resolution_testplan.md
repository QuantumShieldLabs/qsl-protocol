Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-23

# NA-0342 Metadata Runtime qsl-attachments Source Authority Blocker Resolution Test Plan

## Objective

Verify that NA-0342 resolves or precisely preserves the qsl-attachments
source/authority blocker discovered by NA-0341 using qsl-protocol governance
changes plus read-only qsl-attachments source, remote freshness, authority,
protection, and CI evidence.

## Protected invariants

- NA-0342 is qsl-protocol governance/evidence only.
- qsl-attachments is inspected read-only and not mutated.
- qsl-server is not mutated.
- qshield runtime is not mutated.
- qsc/qsp/protocol/crypto/key-schedule implementation is not changed.
- Dependencies, Cargo manifests, workflows, branch protection, public-safety
  configuration, website, README, START_HERE, docs/public, qsc-desktop, and
  production-service paths are not changed.
- qshield embedded relay/demo evidence remains distinct from qsl-server and
  qsl-attachments production behavior.
- qsl-attachments production object-size padding remains unimplemented and
  unproven.
- qsl-server production timing/storage behavior remains unimplemented and
  unproven.
- No claim is added that attachment size, timing metadata, traffic shape, or
  all metadata is hidden.
- No anonymity, metadata-free, untraceable, production-readiness,
  public-internet-readiness, or external-review-complete claim is introduced.

## Allowed scope

- `docs/governance/evidence/NA-0342_metadata_runtime_qsl_attachments_source_authority_blocker_resolution.md`
- `tests/NA-0342_metadata_runtime_qsl_attachments_source_authority_blocker_resolution_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsl-attachments implementation changes.
- qsl-attachments branch creation, checkout changes, fetch into a worktree,
  commits, pushes, PR creation, merge, rebase, build artifacts, dependency
  installation, deployment, or tests that mutate the checkout.
- qsl-server implementation changes.
- qshield runtime implementation changes.
- qsc/qsp/protocol/crypto/key-schedule changes.
- `Cargo.toml`, `Cargo.lock`, dependency updates, and workflow changes.
- branch-protection or public-safety configuration changes.
- website, README, START_HERE, docs/public, qsc-desktop, formal, input,
  tools/refimpl, app runtime, service implementation, production-service, or
  public-copy changes.

## Prior source-authority review requirements

The evidence must review:

- live NA-0342 queue scope;
- inherited NA-0341 `PARTIAL_SOURCE_AUTHORITY` result;
- NA-0340 cross-repo authorization/source-authority plan;
- NA-0339 qshield embedded relay/demo attachment size-class proof;
- qshield demo versus qsl-attachments production boundary;
- qsl-server production timing/storage boundary.

## Local source discovery requirements

The evidence must list all local qsl-attachments paths found and identify the
selected local source if one exists. For each relevant local source, record:

- local path;
- remote URL(s);
- current branch/ref;
- current HEAD SHA;
- local cleanliness;
- detached status;
- local `origin/main` or equivalent;
- source freshness caveats;
- mutation authority status.

## Remote freshness requirements

The evidence must use read-only remote inspection only and record:

- selected local qsl-attachments path;
- local HEAD SHA;
- local branch/ref state;
- local `origin/main` SHA if present;
- remote default branch name;
- remote default branch SHA from read-only evidence;
- whether local selected source matches the remote default branch;
- whether local selected source is stale;
- exact error and classification if remote evidence is unavailable.

Allowed freshness classifications:

- `FRESH_SOURCE`;
- `STALE_SOURCE`;
- `UNKNOWN_REMOTE_FRESHNESS`;
- `ABSENT_SOURCE`.

## Authority/permission requirements

The evidence must record:

- authenticated viewer permission for `QuantumShieldLabs/qsl-attachments`;
- whether permission is `ADMIN`, `MAINTAIN`, `WRITE`, `TRIAGE`, `READ`, or
  unknown;
- whether branch protection exists on the default branch;
- whether required checks are identifiable;
- whether PR creation appears possible from permissions;
- whether direct push is prohibited or unknown;
- whether merge authority appears present or absent;
- whether required reviews/status checks block merge;
- whether current auth could create a branch/PR if future scope explicitly
  authorizes it;
- whether current auth could merge if future scope explicitly authorizes it and
  checks pass;
- whether any authority remains unknown.

Allowed authority classifications:

- `COMPLETE_MUTATION_AUTHORITY`;
- `PARTIAL_MUTATION_AUTHORITY`;
- `BLOCKED_MUTATION_AUTHORITY`;
- `UNKNOWN_MUTATION_AUTHORITY`.

## CI/test/protection requirements

The evidence must record:

- local qsl-attachments top-level build/test files;
- workflow names and likely check contexts;
- package manager/build system;
- safe test commands known;
- risky or secret-dependent test commands;
- latest workflow runs from GitHub if available;
- required branch-protection checks if available;
- unknowns.

Allowed CI classifications:

- `COMPLETE_CI_AUTHORITY`;
- `PARTIAL_CI_AUTHORITY`;
- `BLOCKED_CI_AUTHORITY`;
- `UNKNOWN_CI_AUTHORITY`.

qsl-attachments build/test commands must not be run unless a future directive
explicitly authorizes qsl-attachments mutation or test artifact creation.

## Classification requirements

The evidence must include a final classification:

- `COMPLETE_SOURCE_AUTHORITY`;
- `PARTIAL_SOURCE_AUTHORITY`;
- `BLOCKED_SOURCE_AUTHORITY`.

`COMPLETE_SOURCE_AUTHORITY` requires local source, freshness against remote
default branch, future PR authority, branch protection/required checks known
enough for merge planning, CI/build/test entrypoints known enough for future
authorization, and no unresolved secrets/deploy/rollback blocker for planning.

## Future authorization-bundle requirements

The evidence must state that any future implementation authorization requires:

- exact qsl-attachments repo URL;
- exact local path;
- exact default branch/ref;
- exact base SHA;
- freshness proof;
- viewer permission proof;
- branch protection proof;
- allowed files;
- forbidden files;
- build/test commands;
- CI expectations;
- required status checks;
- lint/format commands;
- storage/object model;
- descriptor/ciphertext/object lifecycle;
- size-class table/cap;
- migration requirement;
- retention/purge requirement;
- backup-plan requirement;
- rollback plan;
- production deploy boundary;
- qsl-server integration boundary;
- qshield demo compatibility boundary;
- secrets/env handling;
- public-claim boundary;
- external-review recommendation;
- branch/PR policy;
- verification bundle requirements;
- stop conditions.

The evidence must state that NA-0342 authorizes no qsl-attachments
implementation or mutation.

## Backup/secrets/deploy/rollback requirements

The evidence must state:

- whether qsl-attachments source path is under current backup scope;
- whether future qsl-attachments work would create durable artifacts;
- whether future qsl-attachments production storage is under backup scope;
- whether secrets/env variables are required or unknown;
- whether deployment commands exist;
- whether rollback docs exist;
- whether future implementation can be no-deploy/test-only;
- whether backup plan must be updated before future work;
- whether NA-0342 itself requires a backup-plan update.

Expected NA-0342 result: no backup-plan update required if only qsl-protocol
governance/evidence/testplan/journal paths under `/srv/qbuild/work` change.

## Production-boundary requirements

The evidence must preserve:

- qsl-attachments production object-size padding remains unimplemented and
  unproven;
- qsl-server production timing/storage behavior remains unimplemented and
  unproven;
- qshield demo proof remains reference/oracle evidence only;
- no qsl-server or qsl-attachments runtime mutation is authorized.

## Claim-boundary requirements

Changed lines must not introduce affirmative claims of:

- attachment-size hiding;
- timing hiding;
- traffic-shape hiding;
- all-metadata hiding;
- metadata-free behavior;
- anonymity;
- untraceable behavior;
- production readiness;
- public-internet readiness;
- external review completion;
- quantum-proof behavior;
- unbreakable behavior;
- guaranteed secure behavior;
- military-grade behavior.

High-risk wording is allowed only when negated, prohibited, partial,
not-ready, future-gated, or exact bounded source/authority wording.

## Backup-impact requirements

The evidence must state whether NA-0342 changes durable evidence locations,
response paths, source roots, excluded backup paths, or non-rebuildable
artifacts outside backup scope. If a future qsl-attachments artifact or source
path falls outside current backup scope, record that as a future prerequisite.

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
- PR-state checks for #945, #944, #943 through #827, #750, #722, and #708
- D132 bundle existence proof
- qsl-protocol branch-protection and public-safety proof
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- START_HERE classifier proof commands from the directive
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- qsl-attachments read-only local source inventory commands
- qsl-attachments read-only remote freshness/authority/protection/CI commands
- `cargo fmt --check`
- qshield NA-0339, NA-0337, NA-0335, NA-0331, NA-0329, NA-0327, NA-0324,
  NA-0322, NA-0320, NA-0319, and NA-0318 harnesses if directly runnable
- `cargo +stable test -p qshield-cli --locked -- --test-threads=1`
- `cargo +stable build -p qshield-cli --locked`
- `scripts/ci/demo_cli_smoke.sh`
- `DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh`
- `DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh` if feasible
- metadata runtime plan and phase-2 harnesses
- `scripts/ci/metadata_conformance_smoke.sh`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- JSON parse for `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`
- targeted and full refimpl tests
- qsc NA-0313 harness if directly runnable
- `git diff --check`
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
  with exact allowed paths
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- goal-lint with a standalone `Goals: G1, G2, G3, G4, G5` line
- classifier proof for the changed path set.

## CI expectations

- qsl-protocol protected branch keeps `public-safety` required.
- PR checks attach and complete successfully.
- `public-safety` remains green before merge and after merge.
- No admin bypass, squash, rebase, direct push, branch deletion command, or
  delete-branch flag is used.

## Successor handoff

If NA-0342 classifies `COMPLETE_SOURCE_AUTHORITY`, the selected successor must
be:

`NA-0343 -- Metadata Runtime qsl-attachments Production Size-Class Implementation Authorization Plan`

If source/authority remains partial or blocked, select an exact blocker,
source-authority, test/CI, backup/retention, service-timing, or external-review
successor based on evidence. NA-0343 must not be implemented by NA-0342
closeout.
