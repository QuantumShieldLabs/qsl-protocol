Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-27

# NA-0547 Remote/Relay Non-Required CI Failure Bounded Reproduction Log Capture Harness

## Executive Summary

NA-0547 executed the exact D-1082-authorized GitHub Actions reproduction
commands for the three non-required remote/relay targets and captured
proof-root-only run/job/log evidence.

Result:

`REMOTE_RELAY_REPRODUCTION_PARTIAL_MORE_EVIDENCE_REQUIRED`

All three targets reproduced on current `main` at `c37fa6111e46`:

- remote-handshake: `REMOTE_HANDSHAKE_REPRODUCED_CURRENT`
- remote-relay: `REMOTE_RELAY_REPRODUCED_CURRENT`
- relay-ui-integration: `RELAY_UI_REPRODUCED_CURRENT`

Selected successor:

`NA-0548 -- QSL Remote/Relay Non-Required CI Failure Follow-Up Evidence Authorization Plan`

NA-0547 did not execute local reproduction, mutate workflows, mutate runtime or
qsc source, update dependencies or lockfiles, run qsl-server/qsl-attachments
locally, run qwork/qstart/qresume, run qsl-backup, mutate public-site content,
or mutate Cloudflare configuration.

## qwork Proof Verification

Fresh qwork proof files from `2026-06-27T07:04:10Z` were copied from
`/srv/qbuild/work/NA-0547/.qwork/` into the proof root and verified from
file-backed `.kv`, JSON, and cargo-target env parsing.

Verified fields included lane `NA-0547`, repo `qsl-protocol`, path
`/srv/qbuild/work/NA-0547/qsl-protocol`, clean worktree/index/untracked state,
HEAD/origin/main `c37fa6111e46`, READY_COUNT 1, READY NA-0547, shared Cargo
target mode, toolchain key `rustc-1.95.0-x86_64-unknown-linux-gnu`,
`explicit_target_preserved=no`, and `shared_target_ready=yes`.

Codex did not run qwork, qstart, or qresume.

## D-1082 / D-1083 Inheritance

D-1082 and D-1083 were consumed and verified Accepted once.

D-1082 authorized only these GitHub Actions commands for NA-0547:

- `gh run rerun 28222737830 --failed`
- `gh run rerun 28221877145 --failed`
- `gh run rerun 28221488004 --failed`
- `gh workflow run remote-handshake-tests.yml --ref main`
- `gh workflow run remote-relay-tests.yml --ref main -f scenario=happy-path -f seed=1`
- `gh workflow run relay-ui-integration.yml --ref main`

D-1082 did not authorize local reproduction.

## Current Main Required-Check Classification

Current main was verified at `c37fa6111e46`.

Required-check classification passed:

- public-safety completed success.
- advisories completed success.
- no failed required checks were observed.
- branch-protection contexts were classified.
- PR-only `goal-lint` and aggregate `CodeQL` were satisfied by PR #1366 rollup
  and current CodeQL analyze check-runs.

The target `remote-handshake`, `remote-relay`, and `relay-ui-integration`
scheduled check-runs failed on current main, but they are non-required
reproduction targets and not required-check blockers.

## Reproduction Command Gate

The command gate passed before rerun or dispatch execution:

- exact D-1082 commands matched the directive byte-for-byte.
- GitHub CLI authentication was present with workflow scope.
- target workflow files existed and retained `workflow_dispatch`.
- no workflow mutation was pending.
- local HEAD, origin/main, and GitHub main matched `c37fa6111e46`.
- raw log proof root existed outside repository tracked paths.
- redaction and private-material scan policy were written.
- no local reproduction command was prepared or invoked.

Recovered failure: `gh workflow view <workflow>.yml --json ...` is unsupported
by the installed GitHub CLI. This was a recoverable read-only command-shape
issue before any rerun/dispatch/repo mutation. Corrective action used the
allowed read-only REST workflow endpoint. Final result: workflow metadata was
captured for all targets.

## Historical Rerun Results

| Target | Historical run | Rerun attempt | Result | Classification |
| --- | --- | --- | --- | --- |
| remote-handshake | `28222737830` | 2 | failure | `HISTORICAL_REPRODUCED` |
| remote-relay | `28221877145` | 2 | failure | `HISTORICAL_REPRODUCED` |
| relay-ui-integration | `28221488004` | 2 | failure | `HISTORICAL_REPRODUCED` |

Failed steps after historical rerun:

- remote-handshake: `Run remote handshake smoke (happy-path seed=1)`.
- remote-relay: `Run remote relay smoke (manual/nightly)`.
- relay-ui-integration: `Start local relay and run ignored relay UI integration tests`.

## Current-Main Workflow Dispatch Results

| Target | Command | New run | Result | Classification |
| --- | --- | --- | --- | --- |
| remote-handshake | `gh workflow run remote-handshake-tests.yml --ref main` | `28298341119` | failure | `CURRENT_REPRODUCED` |
| remote-relay | `gh workflow run remote-relay-tests.yml --ref main -f scenario=happy-path -f seed=1` | `28298371731` | failure | `CURRENT_REPRODUCED` |
| relay-ui-integration | `gh workflow run relay-ui-integration.yml --ref main` | `28298405239` | failure | `CURRENT_REPRODUCED` |

New dispatch run identification used before/after workflow run lists, workflow
file/name, branch `main`, event `workflow_dispatch`, and head SHA
`c37fa6111e46`. Each new run was uniquely identified.

## remote-handshake Result

Classification:

`REMOTE_HANDSHAKE_REPRODUCED_CURRENT`

Current-main dispatch run `28298341119` failed in step `Run remote handshake
smoke (happy-path seed=1)`. The redacted bounded extract shows qsc built
successfully, then the smoke harness failed at vault initialization for the
Alice actor before handshake assertions ran.

## remote-relay Result

Classification:

`REMOTE_RELAY_REPRODUCED_CURRENT`

Current-main dispatch run `28298371731` failed in step `Run remote relay smoke
(manual/nightly)`. The redacted bounded extract shows qsc relay send reached
`contacts_store_invalid`, then the smoke harness failed happy-path count
expectations.

## relay-ui-integration Result

Classification:

`RELAY_UI_REPRODUCED_CURRENT`

Current-main dispatch run `28298405239` failed in step `Start local relay and
run ignored relay UI integration tests`. The redacted bounded extract shows the
workflow built qsl-server, started the local relay, then an HTTP health request
returned 404 before the ignored qsc relay UI tests ran.

## Log Capture Inventory

Raw logs, metadata JSON, poll snapshots, and redacted extracts were stored under
the proof root:

`/srv/qbuild/tmp/NA0547_remote_relay_reproduction_log_capture_20260627T183232Z/`

Repository docs contain summaries only. Raw logs were not copied into repository
docs.

## Redaction and Private-Material Review

Raw logs contain masked secret/authorization fields and public CI token-like
identifiers such as commit hashes, action SHAs, artifact digests, and runner
paths. Raw logs remain proof-root-only.

Redacted historical and current-main extracts passed the private-material scan.
No private material was published in repository docs.

## Per-Target Classification

| Target | Classification |
| --- | --- |
| remote-handshake | `REMOTE_HANDSHAKE_REPRODUCED_CURRENT` |
| remote-relay | `REMOTE_RELAY_REPRODUCED_CURRENT` |
| relay-ui-integration | `RELAY_UI_REPRODUCED_CURRENT` |

## Overall Result Classification

`REMOTE_RELAY_REPRODUCTION_PARTIAL_MORE_EVIDENCE_REQUIRED`

All targets reproduced on current main, but exact future implementation
ownership and mutation path bundle cannot be selected safely from this summary
evidence alone. The remote smoke failures involve secret-backed relay boundaries
and qsc runtime/harness behavior. The relay UI failure includes workflow
transitive qsl-server behavior.

## Selected Successor

Selected successor:

`NA-0548 -- QSL Remote/Relay Non-Required CI Failure Follow-Up Evidence Authorization Plan`

Objective:
Authorize a bounded follow-up evidence lane for target remote/relay CI failures
that remained partially reproduced or ambiguous after NA-0547. Define exact
additional GitHub Actions metadata/log capture or rerun permissions, exact log
redaction boundaries, and exact stop conditions before any implementation lane.
No workflow/runtime/dependency/qsc/qsl-server/qsl-attachments mutation is
authorized.

## Required-Check Boundary

Target remote/relay workflows are non-required. public-safety and advisories
completed success, and no failed required check was observed before
reproduction.

## Workflow Mutation Boundary

No workflow file was modified. NA-0547 used only exact D-1082-authorized
GitHub Actions rerun and dispatch commands.

## Runtime / qsc / Dependency Boundary

No runtime/source path, qsc source/test/fuzz/Cargo path, dependency manifest, or
lockfile was modified. Focused qsc runtime tests were skipped because NA-0547
authorized GitHub Actions black-box reproduction only and no local qsc runtime
reproduction or source mutation occurred.

## qsl-server / qsl-attachments Boundary

No local qsl-server or qsl-attachments command was executed by Codex. No
qsl-server or qsl-attachments source was mutated. relay-ui-integration used
qsl-server only through the existing GitHub Actions workflow.

## Remote-Action Boundary

The only remote actions performed were the exact D-1082-authorized GitHub
Actions rerun and workflow-dispatch commands. No SSH, scp, sftp, rsync, qsc
send/receive, E2EE reproduction, branch-protection mutation, cancellation, or
deletion occurred.

## Public-Site / Cloudflare Boundary

NA-0547 is not a public-site lane. No README public-progress content,
`docs/public`, `public`, `website`, deployment setting, public-site content, or
Cloudflare configuration was mutated.

## Claim Boundary

NA-0547 makes none of these claims:

- no public-readiness claim
- no production-readiness claim
- no public-internet-readiness claim
- no external-review-complete claim
- no reproducibility-complete claim
- no backup/restore-complete claim
- no vulnerability-free claim
- no bug-free claim
- no perfect-build claim
- no perfect-crypto claim

## Validation

Validation proof is recorded under the proof root validation directory. Required
pre-PR local validation passed for `git diff --check`, exact five-path scope
guard, queue/decision proof, marker proof, link-check, private-material scans,
overclaim scan, docs/governance-only classifier, PR body preflight, root cargo
audit, nested qsc fuzz lock cargo audit, `cargo fmt --check`, `sh -n
scripts/ci/qsc_adversarial.sh`, and `bash -n scripts/ci/qsc_adversarial.sh`.

Focused qsc runtime tests were skipped because NA-0547 uses GitHub Actions
rerun/dispatch evidence only, no local qsc runtime reproduction was authorized,
and no qsc source/runtime/dependency/workflow mutation occurred.

## Recommendation

Proceed to the selected NA-0548 follow-up evidence authorization lane before
any implementation lane. The next lane should define exact additional evidence
permissions and exact stop conditions, then select exact mutation paths only if
the additional evidence supports them.
