Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0572 qsl-server Dependency Audit Recovery Implementation Harness

## Executive Summary

NA-0572 consumed D-1132 and D-1133, verified fresh qwork proof, implemented the
D-1132-authorized qsl-server lockfile-only recovery, merged qsl-server PR #57,
and recorded the evidence needed for a later closeout to NA-0573.

The qsl-server source-of-truth remediation changed `Cargo.lock` only, updating
`quinn-proto` from `0.11.14` to `0.11.15`. `Cargo.toml`, qsl-server source
paths, and qsl-server workflow paths were unchanged.

Result classification:

`QSL_SERVER_DEPENDENCY_AUDIT_RECOVERY_IMPLEMENTATION_PASS`

Selected successor:

`NA-0573 -- QSL Remote Relay qsl-server Inspiron Deployment Recovery Retry Harness`

## qwork Proof Verification

Fresh qwork proof files were copied from the NA-0572 lane workspace into the
proof root and parsed from disk before qsl-server clone, qsl-server mutation,
qsl-protocol governance mutation, or PR creation.

Required qwork values passed:

- `startup_result=OK`
- `lane=NA-0572`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0572/qsl-protocol`
- `head=a4df7fadfb83`
- `origin_main=a4df7fadfb83`
- `main=a4df7fadfb83`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0572`
- `requested_lane_status=READY`
- `proof_written_at_utc=2026-06-29T23:25:30Z`
- `cargo_target_mode=shared`
- `shared_target_ready=yes`

Codex did not run `qwork`, `qstart`, or `qresume`.

Pre-fetch live state matched qwork proof: qsl-protocol `HEAD` and
`origin/main` were both `a4df7fadfb83`, and worktree, index, and untracked
state were clean. Root disk usage was below the 95 percent stop threshold, and
`/backup/qsl` was mounted.

## D-1132 / D-1133 Inheritance

D-1132 exists once and is Accepted. It selected the exact qsl-server
lockfile-only recovery model for `RUSTSEC-2026-0185`: update qsl-server
`Cargo.lock` only, changing `quinn-proto 0.11.14` to `0.11.15`.

D-1133 exists once and is Accepted. It closed NA-0571, restored NA-0572 as the
sole READY item, and preserved the no-deployment/no-remote/no-qsl-attachments
boundaries.

Prior qsl-server PR searches for exact `NA-0572` and `RUSTSEC-2026-0185`
returned no PRs. A broader `quinn-proto` search returned only older unrelated
qsl-server PR #48.

## qsl-protocol Current Main Required-Check Classification

Current qsl-protocol main was verified at `a4df7fadfb83`; local main and
`origin/main` matched after fetch.

Required-check classification passed:

- public-safety completed success.
- advisories completed success.
- suite2-vectors completed success.
- no failed check-runs were present.
- no incomplete check-runs were present.
- CodeQL was classified through successful analysis check-runs.
- `goal-lint` was classified through the merged PR #1416 head context.

Root `cargo audit --deny warnings` and nested qsc fuzz
`cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock` passed
before qsl-server mutation.

## qsl-server Checkout and Baseline Audit

qsl-server was cloned into the lane workspace at
`/srv/qbuild/work/NA-0572/qsl-server`.

Checkout proof:

- repository: `QuantumShieldLabs/qsl-server`
- default branch: `main`
- starting HEAD: `d40e6003fdf0`
- `origin/main`: `d40e6003fdf0`
- clean worktree, index, and untracked state before remediation
- `Cargo.toml` and `Cargo.lock` present
- `Cargo.lock` contained `quinn-proto 0.11.14`

Baseline validation:

- `cargo metadata --locked --format-version=1`: passed
- `cargo audit --deny warnings`: failed as expected
- baseline classification:
  `QSL_SERVER_BASELINE_AUDIT_EXPECTED_RUSTSEC_2026_0185`

The baseline audit reported only `RUSTSEC-2026-0185`, affecting
`quinn-proto 0.11.14`, with solution `>=0.11.15`.

## qsl-server Lockfile-Only Remediation

Implementation branch:

`na-0572-quinn-proto-lockfile-audit-recovery`

Remediation command:

`cargo update -p quinn-proto --precise 0.11.15`

The command updated only the qsl-server lockfile package entry:

- `quinn-proto 0.11.14 -> 0.11.15`
- checksum updated for `quinn-proto`
- no `Cargo.toml` change
- no qsl-server source change
- no qsl-server workflow change

Implementation classification:

`QSL_SERVER_DEPENDENCY_AUDIT_RECOVERY_LOCKFILE_ONLY_PASS`

## qsl-server Diff Guard

qsl-server changed paths before commit:

- `Cargo.lock`

Diff guard passed:

- `Cargo.lock` only.
- `Cargo.toml` unchanged.
- qsl-server source paths unchanged.
- qsl-server workflow paths unchanged.
- no broad resolver churn beyond `quinn-proto`.
- generated `target/` build artifacts were cleaned before commit and were not
  included in the PR.

## qsl-server Validation

Pre-merge qsl-server validation passed on the implementation branch:

- `cargo metadata --locked --format-version=1`: passed
- `cargo audit --deny warnings`: passed
- `cargo build --locked`: passed
- `cargo test --locked`: passed
- `cargo fmt --check`: passed

## qsl-server PR / Merge Evidence

qsl-server commit:

`e32f48754ab0` - `NA-0572 recover quinn-proto audit advisory`

qsl-server PR:

- PR: #57
- title: `NA-0572: recover quinn-proto audit advisory`
- URL: `https://github.com/QuantumShieldLabs/qsl-server/pull/57`
- required check: `rust`
- required check result before merge: success
- merge method: merge commit
- merge commit: `6bf61d439fa2`

The qsl-server PR body recorded the required phrases for lockfile-only recovery,
the RustSec advisory, Cargo.lock-only scope, validation, no deployment, no
remote action, no private material publication, and no production/security
overclaim.

## qsl-server Post-Merge Verification

After qsl-server PR #57 merged, local qsl-server main was fast-forwarded to
`origin/main` at `6bf61d439fa2`.

Post-merge verification passed:

- local qsl-server main equals `origin/main`
- qsl-server worktree, index, and untracked state clean
- `Cargo.lock` contains `quinn-proto 0.11.15`
- changed path from starting qsl-server commit: `Cargo.lock` only
- `Cargo.toml` unchanged
- qsl-server source paths unchanged
- qsl-server workflow paths unchanged
- `cargo audit --deny warnings`: passed
- `cargo metadata --locked --format-version=1`: passed
- `cargo build --locked`: passed
- `cargo test --locked`: passed
- `cargo fmt --check`: passed
- qsl-server main `rust` check completed success

## qsl-protocol Governance Update

The qsl-protocol implementation patch records the qsl-server remediation and
selects NA-0573 as the exact successor for later closeout.

Allowed qsl-protocol implementation mutation paths:

- `docs/governance/evidence/NA-0572_qsl_server_dependency_audit_recovery_implementation_harness.md`
- `tests/NA-0572_qsl_server_dependency_audit_recovery_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No qsl-protocol source, script, workflow, dependency, or lockfile path is
mutated by this implementation patch.

## Private-Material Review

Repository evidence publishes public repository names, PR numbers, check names,
safe command names, package names, versions, and short SHAs only.

No secret values, endpoint values, private port values, route-token or bearer
values, Authorization headers, payloads, response bodies, process identities,
authorized_keys content, key material, raw logs, or private material are
published.

## Result Classification

`QSL_SERVER_DEPENDENCY_AUDIT_RECOVERY_IMPLEMENTATION_PASS`

## Selected Successor

### NA-0573 -- QSL Remote Relay qsl-server Inspiron Deployment Recovery Retry Harness

Status: READY
Goals: G1, G2, G3, G4, G5

Objective:
Retry the qsl-server-centered `inspiron` deployment recovery now that
qsl-server source/build audit has been remediated. Codex may acquire qsl-server
source under proof root, audit/build it, stage qsl-server under the qslcodex
test workspace, start only a non-privileged loopback qsl-server process if a
safe no-secret command and bind target are available, and verify canonical
qsl-server route shape with no secrets and no response-body publication. Codex
must stop before sudo, systemctl, Tailscale, firewall, account/shell/
authorized_keys, root-owned service, qsl-attachments, qsc send/receive,
workflow dispatch/rerun, source/workflow/dependency mutation, public-site
mutation, Cloudflare mutation, or private-material publication.

## Required-Check Boundary

NA-0572 classified qsl-protocol current-main required checks before mutation,
waited for the qsl-server PR required check before merge, verified the
qsl-server merge-commit `rust` check, and does not dispatch or rerun workflows.

## Source / Script Mutation Boundary

qsl-server source-of-truth mutation was limited to qsl-server `Cargo.lock`.
qsl-protocol source/script/workflow/dependency paths were not mutated.

## Workflow Mutation Boundary

No qsl-server workflow paths were changed. No qsl-protocol workflow paths were
changed. No workflow dispatch or rerun command was executed.

## Runtime / qsc Boundary

No qsc command was executed. No qsc send/receive or E2EE operation occurred.

## qsl-server / qsl-attachments Boundary

qsl-server was cloned, built, tested, patched in `Cargo.lock`, and merged
through PR #57. qsl-server was not deployed or run as a service.

No qsl-attachments command, clone, build, run, mutation, or PR occurred.

## Remote-Action Boundary

No SSH, scp, Tailscale, remote command, remote staging, remote start, sudo,
systemctl, firewall, account, shell, authorized_keys, root-owned path, or remote
workspace action occurred.

## Public-Site / Cloudflare Boundary

No public-site, `docs/public`, `public`, `website`, README, or Cloudflare path
or configuration was mutated.

## Claim Boundary

No public-readiness claim is made.
No production-readiness claim is made.
No public-internet-readiness claim is made.
No external-review-complete claim is made.
No vulnerability-free claim is made.
No bug-free claim is made.
No perfect-build claim is made.

## Validation

Validation expectations for the qsl-protocol governance patch are captured in
`tests/NA-0572_qsl_server_dependency_audit_recovery_implementation_testplan.md`.

Focused qsc runtime tests are intentionally skipped because NA-0572 does not
mutate qsl-protocol qsc source/runtime/dependency/workflow paths and qsc
send/receive is not authorized.

## Recommendation

After this implementation PR merges and post-merge qsl-protocol checks are
green, perform a separate closeout directive that marks NA-0572 DONE and
restores NA-0573 as exactly one READY successor. Do not start NA-0573 inside
the implementation PR.
