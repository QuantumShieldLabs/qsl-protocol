Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-23

# NA-0343 Metadata Runtime qsl-attachments Production Size-Class Implementation Authorization Test Plan

## Objective

Verify that NA-0343 refreshes qsl-attachments source/authority/CI proof and
produces an exact future qsl-attachments production size-class implementation
authorization plan, or records exact blocker evidence if any prerequisite
regressed.

## Protected invariants

- NA-0343 is qsl-protocol governance/evidence only.
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
  unproven by NA-0343.
- qsl-server production timing/storage behavior remains unimplemented and
  unproven by NA-0343.
- No claim is added that attachment size, timing metadata, traffic shape, or
  all metadata is hidden.
- No anonymity, metadata-free, untraceable, production-readiness,
  public-internet-readiness, or external-review-complete claim is introduced.

## Allowed scope

- `docs/governance/evidence/NA-0343_metadata_runtime_qsl_attachments_production_size_class_implementation_authorization.md`
- `tests/NA-0343_metadata_runtime_qsl_attachments_production_size_class_implementation_authorization_testplan.md`
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

- live NA-0343 queue scope;
- inherited NA-0342 `COMPLETE_SOURCE_AUTHORITY` result;
- NA-0341 source/authority bundle;
- NA-0340 cross-repo authorization/source-authority plan;
- NA-0339 qshield embedded relay/demo attachment size-class proof;
- qshield demo versus qsl-attachments production boundary;
- qsl-server production timing/storage boundary.

## Freshness/authority refresh requirements

The evidence must refresh and record:

- selected qsl-attachments local path;
- local HEAD SHA;
- local branch/ref state;
- local cleanliness;
- remote URL;
- remote default branch name;
- remote default branch SHA from read-only evidence;
- whether local selected source matches the remote default branch;
- stale non-active local refs, if any;
- authenticated viewer permission;
- branch protection status;
- required checks;
- latest qsl-attachments workflow run status;
- open qsl-attachments PR list.

Allowed source classifications:

- `FRESH_SOURCE`;
- `STALE_SOURCE`;
- `UNKNOWN_REMOTE_FRESHNESS`.

Allowed mutation-authority classifications:

- `COMPLETE_MUTATION_AUTHORITY`;
- `PARTIAL_MUTATION_AUTHORITY`;
- `BLOCKED_MUTATION_AUTHORITY`;
- `UNKNOWN_MUTATION_AUTHORITY`.

Allowed CI classifications:

- `COMPLETE_CI_AUTHORITY`;
- `PARTIAL_CI_AUTHORITY`;
- `BLOCKED_CI_AUTHORITY`;
- `UNKNOWN_CI_AUTHORITY`.

Allowed final gates:

- `IMPLEMENTATION_AUTHORIZATION_READY`;
- `IMPLEMENTATION_AUTHORIZATION_BLOCKED`.

## Implementation authorization decision requirements

If prerequisites remain ready, the evidence must define:

- repository;
- local path;
- base branch;
- base SHA;
- future qsl-attachments branch naming rule;
- future qsl-protocol companion branch naming rule;
- future allowed files;
- future forbidden files;
- future qsl-attachments commands;
- future qsl-protocol companion commands;
- PR strategy;
- merge strategy;
- post-merge verification;
- rollback/deploy boundary;
- secrets/env boundary;
- backup boundary;
- qsl-server boundary;
- qshield demo compatibility boundary.

If any prerequisite regresses, the evidence must not create an implementation
authorization plan and must select an exact blocker successor.

## Future qsl-attachments bundle requirements

The future qsl-attachments bundle must include:

- source/ref/authority proof refreshed at future start;
- exact base SHA and clean worktree proof;
- exact allowed implementation files;
- exact forbidden files;
- size-class policy name;
- deterministic table or deterministic table-generation rule;
- max object size;
- max overhead;
- invalid-config rejection;
- oversize rejection;
- malformed object/descriptor rejection;
- no accepted state or output on reject;
- retention/purge behavior;
- backup boundary;
- secrets/env boundary;
- qsl-server boundary;
- qshield demo compatibility boundary;
- markers and verification bundle.

## Future qsl-protocol companion requirements

The future qsl-protocol companion must include:

- qsl-attachments PR/head/CI evidence;
- qsl-attachments source freshness proof;
- qsl-attachments authority proof;
- implementation/result classification;
- D-0669 or later decision, depending on queue state;
- TRACEABILITY update;
- evidence and testplan;
- queue successor update only during closeout;
- no runtime, public-copy, dependency, workflow, branch-protection, or
  public-safety configuration changes unless separately authorized.

## File-map requirements

The evidence must define exact future qsl-attachments allowed files and
forbidden files. It must stop a future lane if source inspection shows that
correct implementation requires files outside the allowed list.

## Build/test/CI requirements

The evidence must require qsl-attachments:

- `cargo fmt --all -- --check`;
- `cargo clippy --all-targets -- -D warnings`;
- `cargo build --locked`;
- `cargo test --locked`;
- remote `rust` required check green on PR head and post-merge `main`.

The evidence must require qsl-protocol:

- queue/decisions;
- scope guard;
- link-check;
- leak-scan;
- goal-lint or PR body preflight;
- dependency health;
- formal/qsc checks relevant to governance claims;
- public-safety required/green.

## Storage/retention/purge/backup requirements

The evidence must record:

- session lifecycle;
- descriptor/object lifecycle;
- ciphertext lifecycle;
- size-class object lifecycle;
- storage paths;
- retention duration boundary;
- purge triggers;
- stale cleanup;
- failed upload cleanup;
- failed fetch cleanup;
- backup inclusion/exclusion boundary;
- log redaction;
- artifact redaction;
- monitoring/alert unknowns;
- operator runbook unknowns;
- rollback boundary;
- migration/compatibility boundary;
- abuse/cost threshold boundary;
- secrets/env variable boundary;
- deployment/non-deployment boundary.

Unknowns that block deployment but not a bounded no-deploy implementation
harness must be marked `REQUIRED_BEFORE_DEPLOYMENT`.

## qsl-server integration boundary requirements

The evidence must state whether qsl-attachments production size-class work can
be implemented/tested without qsl-server changes.

If qsl-server integration is required for the next implementation, the evidence
must select a qsl-server integration successor instead of implementation.

If bounded service-local qsl-attachments tests do not require qsl-server, the
evidence must state the exact qsl-server boundary and preserve qshield demo
evidence as reference/oracle only.

## Claim-boundary requirements

The evidence must state:

- external review remains not complete;
- no claim should imply attachment sizes are hidden;
- no claim should imply timing, traffic shape, or all metadata is hidden;
- no claim should imply metadata-free, anonymity, or untraceable behavior;
- no claim should imply production or public-internet readiness;
- no website/public docs update is authorized;
- stronger future claims require implementation, service, deployment, and
  review evidence.

## Backup-impact requirements

The evidence must state whether NA-0343 itself changes important evidence
locations, response paths, source roots, excluded backup paths, or
non-rebuildable artifacts outside current backup scope.

Expected NA-0343 result: no backup-plan update required if only qsl-protocol
governance/evidence/testplan/decision/traceability/journal paths under
`/srv/qbuild/work` change.

Future qsl-attachments implementation must re-check backup scope before
mutation or deployment evidence.

## Required local checks

- Timestamp and disk watermark proof.
- Clean qsl-protocol worktree proof.
- qsl-protocol fetch and exact `origin/main` proof.
- PR state proof for PR #946/#947 and preservation checks.
- qsl-protocol branch protection and public-safety proof.
- `cargo audit --deny warnings`.
- `cargo tree -i rustls-webpki --locked`.
- START_HERE classifier repair checks.
- `python3 scripts/ci/qsl_evidence_helper.py queue`.
- `python3 scripts/ci/qsl_evidence_helper.py decisions`.
- Live NA-0343 scope quote.
- qsl-attachments read-only local source proof.
- qsl-attachments read-only remote freshness/authority/protection/CI proof.
- `cargo fmt --check`.
- relevant qshield/qsc/formal/metadata checks that are directly runnable.
- scope guard over changed files.
- link-check.
- leak-scan.
- changed-line overclaim scan.
- docs-only classifier proof for changed paths.
- PR body preflight or goal-lint.

## CI expectations

- qsl-protocol `public-safety` remains required by branch protection.
- PR checks attach and complete successfully.
- Post-merge main `public-safety` completes success.
- No admin bypass, squash, rebase, direct push, branch deletion command, or
  delete-branch flag is used.

## Successor handoff

If authorization planning completes, NA-0343 must select:

`NA-0344 -- Metadata Runtime qsl-attachments Production Size-Class Implementation Harness`

If a prerequisite regresses, NA-0343 must select the exact blocker successor
that matches the refreshed evidence. NA-0343 must not implement NA-0344.
