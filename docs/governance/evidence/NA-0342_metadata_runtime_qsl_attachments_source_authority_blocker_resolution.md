Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-23

# NA-0342 Metadata Runtime qsl-attachments Source Authority Blocker Resolution

## Executive summary

NA-0342 is a qsl-protocol governance/evidence lane. It performs read-only
qsl-attachments source, freshness, permission, protection, and CI inspection so
that the source/authority blocker found by NA-0341 is either resolved or
preserved exactly.

Result: `COMPLETE_SOURCE_AUTHORITY`.

The selected qsl-attachments source remains:

- repository: `QuantumShieldLabs/qsl-attachments`;
- URL: `https://github.com/QuantumShieldLabs/qsl-attachments`;
- selected local path: `/srv/qbuild/work/NA-0237D/qsl-attachments`;
- selected local ref state: detached `HEAD`;
- selected local HEAD: `320be68fe632`;
- remote default branch: `main`;
- remote default branch SHA: `320be68fe632`;
- freshness classification: `FRESH_SOURCE`;
- viewer permission: `ADMIN`;
- mutation authority classification: `COMPLETE_MUTATION_AUTHORITY`;
- CI/protection classification: `COMPLETE_CI_AUTHORITY`;
- final source/authority classification: `COMPLETE_SOURCE_AUTHORITY`.

This resolves the NA-0341 source/authority blocker for planning purposes only.
NA-0342 does not authorize qsl-attachments mutation, qsl-server mutation,
runtime mitigation implementation, qsl-attachments production object-size
padding implementation, deployment, public-copy changes, or any stronger
privacy/readiness claim.

Selected successor:

`NA-0343 -- Metadata Runtime qsl-attachments Production Size-Class Implementation Authorization Plan`

## Live NA-0342 scope

The live `NEXT_ACTIONS.md` entry on `origin/main` was:

- `NA-0342 -- Metadata Runtime qsl-attachments Source / Authority Blocker
  Resolution`;
- Status: READY;
- Goals: G1, G2, G3, G4, G5;
- Objective: prove exact qsl-attachments latest source freshness plus
  mutation/PR/merge authority, or stop with exact blocker evidence.

The live scope required:

- exact qsl-attachments repository URL/path and local path;
- exact qsl-attachments base branch/ref and base SHA, with freshness proof;
- exact qsl-attachments mutation, PR creation, and merge authority proof, or
  exact blocker evidence;
- exact current required qsl-attachments CI/build/test/lint entrypoints and
  protected-check expectations;
- exact successor selection.

The live scope forbade unsupported production, public-internet,
external-review, anonymity, unsupported metadata-free, unsupported untraceable,
attachment-size-hidden, timing-hidden, traffic-hidden, qsl-server,
qsl-attachments, qshield, qsc/qsp/protocol/crypto/key-schedule, dependency,
workflow, website, README, START_HERE, docs/public, branch-protection, and
public-safety configuration changes unless a future exact directive authorizes
them.

## Inherited NA-0341 partial source/authority result

NA-0341 recorded `PARTIAL_SOURCE_AUTHORITY`.

Known from NA-0341:

- strongest local qsl-attachments source:
  `/srv/qbuild/work/NA-0237D/qsl-attachments`;
- local observed HEAD: `320be68fe632`;
- local observed state: detached clean checkout with local `origin/main` also
  at `320be68fe632`;
- older clean local qsl-attachments checkouts existed at `1e1ae272a4cb`;
- qsl-attachments production object-size padding remained unimplemented and
  unproven;
- qsl-server production timing/storage behavior remained unimplemented and
  unproven;
- qshield embedded relay/demo evidence remained reference evidence only.

Unresolved by NA-0341:

- latest remote freshness;
- qsl-attachments mutation authority;
- qsl-attachments branch/PR creation authority;
- qsl-attachments merge authority;
- protected CI/required-check expectations.

## Sources inspected

qsl-protocol sources inspected:

- `NEXT_ACTIONS.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `tests/NA-0341_closeout_restore_na0342_testplan.md`;
- `docs/governance/evidence/NA-0341_metadata_runtime_qsl_attachments_source_authority_bundle.md`;
- `tests/NA-0341_metadata_runtime_qsl_attachments_source_authority_bundle_testplan.md`;
- `docs/governance/evidence/NA-0340_metadata_runtime_qsl_attachments_production_size_class_cross_repo_authorization.md`;
- `docs/governance/evidence/NA-0339_metadata_runtime_qshield_demo_attachment_size_class_harness.md`;
- qsl-protocol canonical/design qsl-attachments and metadata-runtime
  references linked by traceability.

Read-only qsl-attachments local paths found:

- `/srv/qbuild/work/NA-0237D/qsl-attachments` at `320be68fe632`;
- `/srv/qbuild/work/NA-0237C/qsl-attachments` at `1e1ae272a4cb`;
- `/srv/qbuild/work/NA-0237/qsl-attachments` at `1e1ae272a4cb`;
- `/srv/qbuild/work/NA-0237A/qsl-attachments` at `1e1ae272a4cb`;
- `/srv/qbuild/work/NA-0237B/qsl-attachments` at `1e1ae272a4cb`.

Read-only GitHub evidence inspected:

- `git ls-remote --heads origin`;
- `git ls-remote origin HEAD`;
- `gh repo view QuantumShieldLabs/qsl-attachments`;
- `gh api /repos/QuantumShieldLabs/qsl-attachments/branches/main/protection`;
- `gh pr list --repo QuantumShieldLabs/qsl-attachments --state open`;
- `gh run list --repo QuantumShieldLabs/qsl-attachments`.

No qsl-attachments checkout was fetched, cloned, checked out, branched,
committed, pushed, merged, rebased, dependency-installed, built, tested,
deployed, or mutated.

## Local qsl-attachments source status

Selected local source:

| Field | Result |
| --- | --- |
| local path | `/srv/qbuild/work/NA-0237D/qsl-attachments` |
| repository | `qsl-attachments` |
| remote URL | `https://github.com/QuantumShieldLabs/qsl-attachments.git` |
| mirror remote | `/srv/qbuild/mirrors/qsl-attachments.git` |
| worktree status | clean |
| local ref state | detached `HEAD` |
| local HEAD | `320be68fe632` |
| local `origin/main` | `320be68fe632` |
| local `main` | `1e1ae272a4cb` |
| package/build system | Rust/Cargo package `qsl-attachments` |
| top-level workflow | `.github/workflows/rust.yml` |
| primary runtime files | `src/lib.rs`, `src/main.rs` |
| primary local tests | `tests/*.rs` including service, retention, recovery, quota, abuse, and backup/restore harnesses |

The selected local source is fresh against the live remote default branch. The
other local checkouts are historical and not selected.

## Remote freshness resolution

Remote freshness evidence:

- `git ls-remote origin HEAD` returned `320be68fe632` for `HEAD`;
- `git ls-remote --heads origin` returned `320be68fe632` for
  `refs/heads/main`;
- GitHub repository metadata reported default branch `main`;
- selected local HEAD is `320be68fe632`;
- selected local `origin/main` is `320be68fe632`;
- latest listed qsl-attachments workflow run on `main` used
  `320be68fe632` and completed `success`.

Freshness classification: `FRESH_SOURCE`.

The source is not stale. No remote fetch was performed into any
qsl-attachments checkout.

## Mutation / PR / merge authority resolution

GitHub authority evidence:

- repository: `QuantumShieldLabs/qsl-attachments`;
- viewer permission: `ADMIN`;
- open PR list: empty at inspection time;
- default branch: `main`;
- branch protection exists on `main`;
- required status checks are strict;
- required check context: `rust`;
- force pushes disabled;
- deletions disabled;
- required signatures disabled;
- required linear history disabled;
- admins are not enforced by branch protection.

Authority interpretation:

- current authentication has sufficient permission for a future directive to
  create a qsl-attachments branch and PR if that future directive explicitly
  authorizes mutation;
- current authentication appears sufficient to merge a qsl-attachments PR after
  required checks pass if that future directive explicitly authorizes merge;
- direct push/admin bypass remains forbidden by qsl-protocol operating policy
  and must be prohibited again by any future qsl-attachments implementation
  directive, even though repository permission is `ADMIN`;
- the known protected path is normal PR review/check flow with the `rust`
  check green.

Mutation authority classification: `COMPLETE_MUTATION_AUTHORITY`.

## CI / test / protection resolution

Local qsl-attachments workflow:

- workflow name: `rust`;
- trigger: pull requests and pushes to `main`;
- job name/context: `rust`;
- runner: `ubuntu-latest`;
- toolchain: stable with rustfmt and clippy;
- commands:
  - `cargo fmt --all -- --check`;
  - `cargo clippy --all-targets -- -D warnings`;
  - `cargo build --locked`;
  - `cargo test --locked`.

Remote protection and run evidence:

- branch protection requires strict `rust`;
- latest run on `main` at `320be68fe632` completed `success`;
- recent PR and main `rust` runs also completed `success`.

Local test/build discovery:

- `Cargo.toml` defines package `qsl-attachments`;
- local tests include service-contract, retention cleanup/recovery/logging,
  reject taxonomy, disk pressure/quota/abuse, capability-scope/logging, and
  backup/restore recovery/logging coverage;
- docs define current deployment, authn/authz, durability/recovery, backup,
  and reference-host boundaries.

qsl-attachments tests were not run by NA-0342 because the directive was
source/authority evidence only and qsl-attachments was inspected read-only.
Running Cargo tests would write build artifacts under that checkout and is left
for a future directive that explicitly authorizes qsl-attachments mutation or
test artifact creation.

CI/protection classification: `COMPLETE_CI_AUTHORITY`.

## Source / authority final classification

Combined classifications:

| Dimension | Classification | Basis |
| --- | --- | --- |
| source presence | present | selected local path exists and is clean |
| freshness | `FRESH_SOURCE` | local selected source matches live remote `main` |
| mutation/PR/merge authority | `COMPLETE_MUTATION_AUTHORITY` | viewer permission is `ADMIN`, protection and required checks are known |
| CI/protection | `COMPLETE_CI_AUTHORITY` | required check and workflow commands are known; latest main run is green |
| deploy/rollback/secrets/backup authority for planning | sufficient for authorization planning | current docs and runtime surfaces identify the boundaries; future implementation directive must freeze exact deploy/non-deploy and backup impact |

Final classification: `COMPLETE_SOURCE_AUTHORITY`.

This classification resolves the source/authority blocker for selecting an
implementation-authorization planning successor. It is not production
implementation proof and does not authorize mutation.

## Future authorization bundle

Any future NA-0343 or later implementation authorization directive must contain
all of the following before qsl-attachments mutation:

- exact qsl-attachments repository URL:
  `https://github.com/QuantumShieldLabs/qsl-attachments`;
- exact qsl-attachments local path;
- exact default branch/ref, currently `main`;
- exact full base SHA from live remote proof, with narrative short SHA expected
  in qsl-protocol evidence;
- source freshness proof taken immediately before edits;
- clean qsl-attachments worktree proof;
- viewer permission proof;
- branch protection proof;
- branch/PR policy with no direct push, no admin bypass, no squash, no rebase,
  and no delete-branch flag;
- exact allowed files;
- exact forbidden files;
- build, test, fmt, and clippy commands;
- CI expectations and required status checks;
- storage/object model;
- descriptor, ciphertext, and object lifecycle map;
- size-class table and cap;
- migration requirement for existing objects/descriptors;
- retention and purge requirements;
- backup-plan requirement;
- rollback plan;
- production deploy or no-deploy boundary;
- qsl-server integration boundary;
- qshield demo compatibility boundary;
- secrets/env handling;
- public-claim boundary;
- external-review recommendation;
- verification bundle requirements;
- stop conditions.

NA-0342 does not authorize implementation. NA-0342 does not authorize
qsl-attachments mutation. A future implementation directive must explicitly
authorize qsl-attachments mutation and name the exact source, base, branch,
allowed files, CI, rollback, deploy/non-deploy boundary, backup boundary,
secrets boundary, and public-claim boundary. A future implementation directive
must include qsl-protocol governance updates plus a qsl-attachments
PR/verification bundle, or explicitly stop on a blocker.

## Backup / secrets / deploy / rollback resolution

NA-0342 backup impact:

- qsl-protocol changes stay under the existing qsl-protocol worktree in
  `/srv/qbuild/work`;
- the response file remains under the existing Codex response path;
- no non-rebuildable artifact outside current backup scope is created by this
  governance patch;
- no backup-plan update is required for NA-0342.

Future qsl-attachments prerequisites:

- the selected source path is under `/srv/qbuild/work`, but future
  qsl-attachments build/test artifacts and response/evidence paths must be
  covered explicitly if they become durable evidence;
- production storage paths such as `/var/lib/qsl-attachments/data`, if used in
  a future lane, require explicit backup-scope proof before relying on them;
- current qsl-attachments docs state cold full-root backup/restore plus
  matching service configuration as the supported backup shape;
- hot/live backup and partial restore remain unsupported by current
  qsl-attachments evidence;
- local config/env surfaces include `QATT_*` variables and `RUST_LOG`; no live
  secret values were inspected;
- deployment docs describe a reference binary, systemd, and Caddy path, but
  NA-0342 authorizes no deploy;
- a future implementation plan may be no-deploy/test-only, or must separately
  authorize any deployment/rollback activity.

## Public claim / external-review / production boundary

External review remains not complete.

qsl-attachments production size-class work remains review-sensitive. No public
or product claim should imply:

- attachment sizes are hidden;
- timing metadata is hidden;
- traffic shape is hidden;
- all metadata is hidden;
- padding hides all metadata;
- the runtime is metadata-free;
- anonymity or untraceability is achieved;
- production readiness or public-internet readiness is achieved;
- external review is complete.

qshield embedded relay/demo attachment size-class evidence remains a useful
reference/oracle for fail-closed local behavior. It is not qsl-attachments
production proof and is not qsl-server production timing/storage proof.

## Selected successor

Selected successor:

`NA-0343 -- Metadata Runtime qsl-attachments Production Size-Class Implementation Authorization Plan`

Rationale:

- selected qsl-attachments source is fresh against the live remote default
  branch;
- mutation/PR/merge authority is known enough for a future branch/PR/merge
  plan;
- required protected check and workflow commands are known;
- remaining work is not blocker discovery, but an exact implementation
  authorization plan with no runtime implementation in NA-0343 unless a later
  directive authorizes it.

## Rejected alternatives

- `NA-0343 -- Metadata Runtime qsl-attachments Source / Authority Blocker
  Continuation`: rejected because the source/authority blocker is resolved for
  planning; the local selected source matches live remote `main`, viewer
  permission is known, protection is known, and required CI is known.
- `NA-0343 -- Metadata Runtime qsl-attachments Test / CI Discovery Plan`:
  rejected because required CI and local commands are known enough for a future
  authorization plan.
- `NA-0343 -- Metadata Runtime qsl-attachments Backup / Retention Prerequisite
  Plan`: rejected as immediate successor because backup/retention boundaries
  are known enough to be bundled into implementation authorization planning,
  though future production storage may still require explicit backup proof.
- `NA-0343 -- Metadata Runtime Service Timing Cross-Repo Authorization`:
  rejected as immediate successor because qsl-attachments production
  size-class authorization remains the narrower next step; any qsl-server
  timing/storage change still needs separate future authorization.
- `NA-0343 -- Metadata Runtime External Review Readiness Gap Audit`: rejected
  as immediate successor because external review remains important but a
  reviewable implementation boundary should first be planned.
- Direct qsl-attachments implementation: rejected because NA-0342 is
  evidence/governance only and authorizes no qsl-attachments mutation.

## Backup-plan impact statement

No backup-plan update is required for NA-0342. The only tracked changes are
qsl-protocol governance/evidence/testplan/journal files under the existing
qsl-protocol worktree.

Future qsl-attachments implementation, production validation, deployment, or
durable artifact capture must re-check backup scope, especially for production
storage roots, build artifacts, logs, evidence bundles, and response paths.

## Next recommendation

Proceed to close out NA-0342 only after this evidence PR merges and
post-merge `public-safety` is green. Restore exactly one READY successor:

`NA-0343 -- Metadata Runtime qsl-attachments Production Size-Class Implementation Authorization Plan`

The successor should remain an authorization-plan lane unless a later
directive explicitly authorizes qsl-attachments mutation and names exact
allowed files, base SHA, CI, rollback, deploy/non-deploy, backup, secrets, and
public-claim boundaries.
