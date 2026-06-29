Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0571 qsl-server Source / Build Recovery Authorization Plan

## Executive Summary

NA-0571 consumed D-1130 and D-1131, verified fresh qwork proof, reacquired
qsl-server source under the proof root, reproduced the `RUSTSEC-2026-0185`
audit failure on qsl-server current main, and proved the narrow recovery model
in a scratch clone.

Baseline qsl-server source at `d40e6003fdf0` still locks `quinn-proto 0.11.14`
through the qsl-server dev dependency path `reqwest -> quinn -> quinn-proto`.
The scratch recovery changed only qsl-server `Cargo.lock`, updating
`quinn-proto` from `0.11.14` to `0.11.15`; `quinn` stayed `0.11.9` and
`reqwest` stayed `0.12.28`. Locked metadata, cargo audit, cargo build, and
cargo test all passed in the scratch clone.

Result classification:

`QSL_SERVER_SOURCE_BUILD_RECOVERY_LOCKFILE_ONLY_IMPLEMENTATION_READY`

Selected successor:

`NA-0572 -- QSL qsl-server Dependency Audit Recovery Implementation Harness`

## qwork Proof Verification

Fresh qwork proof files were copied from the NA-0571 lane workspace into the
proof root and parsed from disk before fetch, qsl-server source acquisition, or
repository mutation.

Required qwork values passed:

- `startup_result=OK`
- `lane=NA-0571`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0571/qsl-protocol`
- `head=03166146d7c`
- `origin_main=03166146d7c`
- `main=03166146d7c`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0571`
- `requested_lane_status=READY`
- `proof_written_at_utc=2026-06-29T22:11:25Z`
- `cargo_target_mode=shared`
- `shared_target_ready=yes`

Codex did not run `qwork`, `qstart`, or `qresume`.

Pre-fetch live state matched qwork proof: `HEAD` and `origin/main` were both
`03166146d7c`, and worktree, index, and untracked state were clean. Root disk
usage was below the 95 percent stop threshold, and `/backup/qsl` was mounted.

## D-1130 / D-1131 Inheritance

D-1130 exists once and is Accepted. It accepted the D493 qsl-server
source-audit stop, recorded `RUSTSEC-2026-0185` for `quinn-proto 0.11.14`, and
selected NA-0571 as the exact source/build recovery authorization successor.

D-1131 exists once and is Accepted. It closed NA-0570 after PR #1413 merged,
verified post-merge public-safety and advisories success, and restored NA-0571
as the sole READY successor without implementing it.

## D493 / D494 Source-Audit Stop Review

D493 stopped before repository mutation because qsl-server cargo audit failed.
It did not build qsl-server, did not generate remote scripts, did not run SSH or
scp, did not perform remote recovery, and did not mutate qsl-server.

D494 accepted that fail-closed stop, added D-1130 and D-1131 through merged
qsl-protocol PRs, and restored NA-0571. D-1132 was absent before this patch.
NA-0571 evidence/testplan files were not present before this directive.

## Current Main Required-Check Classification

Current qsl-protocol main was verified at `03166146d7c`; local main and
`origin/main` matched. `origin/main` equals the expected D494 closeout merge
commit.

Required-check classification passed:

- public-safety completed success.
- advisories completed success.
- suite2-vectors completed success.
- no failed check-runs were present.
- no incomplete check-runs were present.
- CodeQL was classified through successful merge-commit analysis check-runs.
- `goal-lint` was classified through the merged PR #1414 head context.

Root `cargo audit --deny warnings`, nested qsc fuzz `cargo audit --deny
warnings`, and `cargo metadata --locked --format-version=1` passed. There was no
`Cargo.toml`, root `Cargo.lock`, or nested qsc fuzz `Cargo.lock` drift.

## qsl-server Source Acquisition

qsl-server source was acquired proof-root-only with `gh repo clone
QuantumShieldLabs/qsl-server`. The analyzed source commit was `d40e6003fdf0`.
`Cargo.toml` and `Cargo.lock` were hashed in proof-root evidence, and
`cargo metadata --locked --format-version=1` passed on the baseline clone.

No qsl-server source-of-truth mutation occurred. No qsl-server branch was
pushed. No qsl-server PR was opened.

## Baseline qsl-server Audit / Dependency Path

Baseline command:

`cargo audit --deny warnings`

Baseline result:

`failed`, classified.

Finding:

- advisory: `RUSTSEC-2026-0185`
- affected crate/version: `quinn-proto 0.11.14`
- locked related versions: `quinn-proto 0.11.14`, `quinn 0.11.9`,
  `reqwest 0.12.28`
- dependency path class: qsl-server dev dependency path via reqwest/quinn

The local advisory/project history identifies `quinn-proto 0.11.15` as the
patched lower-bound candidate.

## Scratch Recovery Attempts

A proof-root scratch copy was created from the qsl-server baseline clone. All
scratch commands ran only in that scratch copy.

Attempt 1:

`cargo update -p quinn-proto --precise 0.11.15`

Result:

- changed files: `Cargo.lock` only
- changed package: `quinn-proto 0.11.14 -> 0.11.15`
- unchanged related packages: `quinn 0.11.9`, `reqwest 0.12.28`
- `Cargo.toml` changed: no
- qsl-server source code changed: no
- `cargo metadata --locked --format-version=1`: pass
- `cargo audit --deny warnings`: pass
- `cargo build --locked`: pass
- `cargo test --locked`: pass

Scratch feasibility classification:

`QSL_SERVER_SCRATCH_LOCKFILE_ONLY_RECOVERY_PASS`

## Dependency / Manifest Scope Analysis

The recovery model is lockfile-only. No manifest requirement change is needed.
No runtime dependency requirement change is needed. No qsl-server source code
change is needed.

The future implementation lane must mutate only qsl-server `Cargo.lock` unless
a later directive explicitly records a different exact path.

## Build / Test Feasibility

Scratch `cargo build --locked` passed after the lockfile-only remediation.
Scratch `cargo test --locked` passed after the lockfile-only remediation.
Build and test outputs remain proof-root-only.

## Private-Material Review

The qsl-server scratch-output private-material scan passed over proof-output
files. The scan excluded cloned source trees and build artifacts from the
publication surface and found zero blocking hits.

Recovered scan-scope issue: the first scan root included proof-root Cargo
target build artifacts. It was terminated before repository mutation and rerun
over proof-output files only. Final result: PASS.

## Option Review

Option A, current qsl-server main already green, was rejected because baseline
audit still failed.

Option B, lockfile-only qsl-server recovery ready, was selected because scratch
proof changed only `Cargo.lock` and passed locked metadata, audit, build, and
test.

Option C, dev-dependency manifest update required, was rejected because
`Cargo.toml` was unchanged.

Option D, runtime dependency update required, was rejected because no runtime
dependency requirement changed.

Option E, no patch available, was rejected because `quinn-proto 0.11.15` passed.

Option F, ambiguous or scope too broad, was rejected because the changed package
and file set were narrow and classified.

## Result Classification

`QSL_SERVER_SOURCE_BUILD_RECOVERY_LOCKFILE_ONLY_IMPLEMENTATION_READY`

## Selected Successor

`NA-0572 -- QSL qsl-server Dependency Audit Recovery Implementation Harness`

The successor should implement the exact qsl-server `Cargo.lock` remediation
proven here and open the qsl-server PR in that later lane. NA-0571 itself does
not mutate qsl-server source-of-truth.

## Required-Check Boundary

NA-0571 classified current-main and local dependency-health gates before
mutation. It did not dispatch or rerun workflows.

## Source / Script Mutation Boundary

No qsl-protocol source, script, workflow, dependency, or lockfile path changed.
No qsl-server source-of-truth path changed. Scratch qsl-server mutation was
limited to proof-root `Cargo.lock` only.

## Workflow Mutation Boundary

No workflow files were changed. No workflow dispatch occurred. No workflow rerun
occurred.

## Runtime / qsc Boundary

No qsc command was executed. No qsc send/receive or E2EE occurred.

## qsl-server / qsl-attachments Boundary

qsl-server was cloned proof-root-only and mutated only in a proof-root scratch
copy for dependency feasibility. qsl-server was not deployed or run. No
qsl-server PR was opened.

No qsl-attachments command, clone, build, run, mutation, or PR occurred.

## Remote-Action Boundary

No SSH, scp, Tailscale, remote command, remote probe, sudo, systemctl, service,
firewall, account, shell, authorized_keys, root-owned path, or remote workspace
action occurred.

## Public-Site / Cloudflare Boundary

No README public-progress, docs/public, website/public, public-site,
Cloudflare configuration, or deployment setting was mutated.

## Claim Boundary

- No public-readiness claim was made.
- No production-readiness claim was made.
- No public-internet-readiness claim was made.
- No external-review-complete claim was made.
- No vulnerability-free claim was made.
- No bug-free claim was made.
- No perfect-build claim was made.

## Validation

Validation evidence is required before PR:

- `git diff --check`
- exact five-path implementation scope guard
- no qsl-protocol source/script/workflow/dependency path change
- no qsl-server source-of-truth path change
- queue/decision proof
- marker proof
- deterministic markdown link check
- added-line/new-file private-material scan
- qsl-server scratch-output private-material scan proof
- overclaim scan
- docs/governance-only classifier
- PR body preflight
- goal-lint if available
- root `cargo audit --deny warnings`
- nested qsc fuzz `cargo audit --deny warnings`
- `cargo metadata --locked --format-version=1`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Focused qsc runtime tests may be skipped because NA-0571 is authorization-only
governance/testplan work with no qsl-protocol source/runtime/dependency/workflow
mutation and no qsc send/receive authorization.

## Recommendation

Close NA-0571 only after D-1132 is merged and post-merge gates are green, then
restore the exact Successor B block as NA-0572. Do not retry qsl-server remote
deployment until the qsl-server lockfile-only recovery lane has landed and
qsl-server audit/build proof is green in that repository.
