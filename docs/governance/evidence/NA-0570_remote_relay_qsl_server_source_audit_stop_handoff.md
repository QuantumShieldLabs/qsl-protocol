Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0570 qsl-server Source-Audit Stop Handoff

## Executive Summary

NA-0570 consumed D-1128 and D-1129, verified a fresh NA-0570 qwork proof,
reviewed D493's qsl-server source-audit stop, and accepts the stop as the
correct fail-closed outcome for this lane.

D493 acquired qsl-server source proof-root-only at `d40e6003fdf0`, ran
`cargo audit --deny warnings`, and stopped because `RUSTSEC-2026-0185` affects
`quinn-proto 0.11.14` through the qsl-server dev dependency path via
reqwest/quinn. qsl-server build, transfer, remote staging, remote start, SSH,
scp, qsc send/receive, qsl-attachments work, workflow dispatch, and qsl-protocol
runtime/source/dependency mutation were not attempted.

Result classification:

`QSL_SERVER_RECOVERY_SOURCE_AUDIT_STOP_ACCEPTED`

Selected successor:

`NA-0571 -- QSL Remote qsl-server Source / Build Recovery Authorization Plan`

## qwork Proof Verification

Fresh qwork proof files were copied from the NA-0570 lane workspace into the
proof root and parsed with a file-backed parser before fetch or repository
mutation.

Required qwork values passed:

- `startup_result=OK`
- `lane=NA-0570`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0570/qsl-protocol`
- `head=eb0e6a24dad8`
- `origin_main=eb0e6a24dad8`
- `main=eb0e6a24dad8`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0570`
- `requested_lane_status=READY`

Codex did not run `qwork`, `qstart`, or `qresume`.

Pre-fetch live state matched qwork proof: `HEAD` and `origin/main` were both
`eb0e6a24dad8`, and worktree, index, and untracked state were clean. Root disk
usage was below the 95 percent stop threshold, and `/backup/qsl` was mounted.

After proof/live/disk/mount gates passed, origin main was fetched, local main
was verified at `eb0e6a24dad8`, and that commit equals the D492 expected
baseline.

## D-1128 / D-1129 Inheritance

D-1128 exists once and is Accepted. It consumed D-1126/D-1127 and selected
qsl-server, not qsc, as the relay/server deployment recovery target. It selected
NA-0570 as the exact successor.

D-1129 exists once and is Accepted. It closed NA-0569 after PR #1411 merged,
verified post-merge public-safety and advisories success, and restored NA-0570
as the sole READY successor without implementing NA-0570.

NA-0569 is DONE. NA-0568 is DONE. D-1130 and D-1131 were absent before this
patch. Duplicate decision count was zero.

## D493 Source-Audit Stop Review

D493 stopped before repository mutation because qsl-server cargo audit failed.
The D493 response and proof-root artifacts were reviewed and were consistent:

- D493 did not add D-1130.
- D493 did not create a branch, commit, or PR.
- D493 did not build qsl-server.
- D493 did not run qsl-server help or version commands.
- D493 did not generate or run remote scripts.
- D493 did not run SSH or scp.
- D493 did not mutate qsl-server.
- D493 did not mutate qsl-protocol.
- D493 did not run qsc.
- D493 did not touch qsl-attachments.
- D493 private-material review passed.
- Final local queue remained READY NA-0570.

D493 result classification:

`QSL_SERVER_RECOVERY_SOURCE_AUDIT_STOP`

## qsl-server Audit Finding

The source-audit finding consumed from D493 is:

- qsl-server source commit: `d40e6003fdf0`
- package: `qsl-server 0.1.0`
- command: `cargo audit --deny warnings`
- result: failed
- advisory: `RUSTSEC-2026-0185`
- affected crate/version: `quinn-proto 0.11.14`
- dependency path class: qsl-server dev dependency path via reqwest/quinn

This lane records the finding only. It does not remediate qsl-server and does
not select a patched dependency version.

## qsl-server Build / Remote Recovery Boundary

D493 skipped qsl-server build because audit failed first. No qsl-server binary
was built, no binary manifest was available, and no qsl-server runtime command
was run.

D494 preserves that boundary. It records the stop and successor only. It does
not clone, build, run, mutate, transfer, stage, or start qsl-server.

## Private-Material Review

D493 aggregate private-material review passed. D494 publishes only coarse
governance classifications, public advisory identifiers, public crate names and
versions, short source SHAs, and non-secret boundary statements.

This evidence does not publish endpoint values, private ports, private topology,
route-token or capability values, bearer values, Authorization headers, payloads,
response bodies, process identities, authorized_keys content, key material,
secret environment values, raw logs, raw artifacts, or private material.

## Result Classification

`QSL_SERVER_RECOVERY_SOURCE_AUDIT_STOP_ACCEPTED`

Rationale: D493 hit a qsl-server source-audit failure before qsl-server build or
remote action. Continuing into build, transfer, staging, runtime, or deployment
would dilute the fail-closed audit gate and exceed NA-0570's safe recovery
boundary.

## Selected Successor

The exact selected successor is:

`NA-0571 -- QSL Remote qsl-server Source / Build Recovery Authorization Plan`

NA-0571 must authorize qsl-server source/build recovery before any retry of
NA-0570-style deployment recovery. It must decide whether the qsl-server audit
failure can be resolved by a lockfile-only update, a dev-dependency manifest
update, a qsl-server repository remediation lane, or another explicitly
authorized operator/source staging action.

NA-0571 must not perform qsl-server mutation unless a later directive authorizes
the exact repository and paths.

## Required-Check Boundary

Current main was classified at `eb0e6a24dad8` before mutation.

- public-safety completed success.
- advisories completed success.
- suite2-vectors completed success.
- no failed required check was classified.
- no pending required check was classified after branch-protection context
  review.
- root `cargo audit --deny warnings` completed success.
- nested qsc fuzz `cargo audit --deny warnings --file
  qsl/qsl-client/qsc/fuzz/Cargo.lock` completed success.
- `cargo metadata --locked --format-version=1` completed success.
- no `Cargo.toml`, root `Cargo.lock`, or qsc fuzz `Cargo.lock` drift was
  detected before the governance patch.

`goal-lint` and `CodeQL` were classified through the merged PR #1412 rollup
where those aggregate contexts were not literal merge-commit check-run names.

## Source / Script Mutation Boundary

D494 does not mutate qsl-protocol source, repository scripts, workflows,
dependencies, or lockfiles. Its implementation mutation is limited to this
evidence document, the NA-0570 source-audit stop testplan, D-1130 in
DECISIONS.md, TRACEABILITY.md, and the rolling operations journal.

## Workflow Mutation Boundary

D494 does not change workflow files. It does not dispatch workflows and does not
rerun workflow jobs.

## Runtime / qsc Boundary

D494 does not execute qsc and does not perform qsc send/receive or E2EE. Focused
qsc runtime tests are not required for this governance-only source-audit stop
handoff because no qsl-protocol runtime, source, dependency, workflow,
executable test, fuzz target, or vector path is changed.

## qsl-server / qsl-attachments Boundary

D494 does not run qsl-server commands and does not clone, build, run, mutate, or
open a qsl-server PR.

D494 does not run qsl-attachments commands and does not clone, build, run, or
mutate qsl-attachments.

## Remote-Action Boundary

D494 does not run SSH, scp, Tailscale, qsc, remote commands, sudo, systemctl,
service commands, firewall commands, account mutation, shell mutation,
authorized_keys mutation, or remote workspace mutation.

## Public-Site / Cloudflare Boundary

D494 does not mutate README public-progress text, docs/public, website/public
paths, public-site content, Cloudflare configuration, or deployment settings.

## Claim Boundary

D494 makes no public readiness claim, no production readiness claim, no public
internet readiness claim, no external review complete claim, no vulnerability
free claim, no bug free claim, and no perfect build claim.

## Validation

Required validation for this governance-only handoff includes:

- `git diff --check`
- exact five-path implementation scope guard
- queue/decision proof
- marker proof
- deterministic markdown link check
- added-line/new-file private-material scan
- added-line overclaim scan
- docs/governance-only classifier
- PR body preflight
- goal-lint if available
- root cargo audit
- nested qsc fuzz cargo audit
- locked cargo metadata
- cargo fmt check
- qsc adversarial script shell syntax checks

## Recommendation

Proceed to PR and merge only if the D494 validation and required checks pass.
After merge, close out NA-0570 only if D-1130 exists once, post-merge checks are
green, and the exact NA-0571 successor block is restored without placeholders.
