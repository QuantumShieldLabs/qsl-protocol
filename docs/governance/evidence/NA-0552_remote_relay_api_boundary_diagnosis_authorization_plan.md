Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-28

# NA-0552 Remote Relay API Boundary Diagnosis Authorization Plan

## Executive Summary

NA-0552 is accepted as an authorization-only boundary diagnosis. It consumed
D-1092 and D-1093, verified fresh qwork proof, reviewed current-main required
checks, reviewed the stopped NA-0551 branch and D465/D466 run evidence, and
inspected qsc relay-push semantics read-only.

Result classification:
`REMOTE_RELAY_API_BOUNDARY_DIAGNOSTIC_INSTRUMENTATION_READY`.

Selected successor:
`NA-0553 -- QSL Remote Relay API Boundary Diagnostic Instrumentation
Authorization Plan`.

No rerun, workflow dispatch, local reproduction, script remediation, workflow
mutation, qsc/runtime/dependency mutation, qsl-server/qsl-attachments command,
public-site mutation, Cloudflare mutation, raw-log commit, or private-material
publication occurred.

## qwork Proof Verification

Fresh qwork proof was copied from `/srv/qbuild/work/NA-0552/.qwork/` into the
proof root and verified from `.kv`, JSON, and cargo-target env files.

- qwork proof timestamp: `2026-06-28T00:45:05Z`
- lane: `NA-0552`
- repo: `qsl-protocol`
- startup head/origin-main/main: `4cdfaf14ab47`
- startup worktree/index/untracked state: clean
- READY_COUNT: 1
- READY: `NA-0552`
- shared cargo target: ready

Codex did not run qwork, qstart, or qresume.

## D-1092 / D-1093 Inheritance

D-1092 exists once and is Accepted. It records
`REMOTE_SMOKE_DEMO_SCRIPT_REMEDIATION_RELAY_API_BOUNDARY_STOP_ACCEPTED`, keeps
the D465/D466 stopped branch preserved and unmerged, and selects NA-0552.

D-1093 exists once and is Accepted. It records NA-0551 closeout as a terminal
stop with no remediation merge, and restores NA-0552 as exactly one READY
authorization-only successor.

## Current Main Required-Check Classification

Current main was verified at `4cdfaf14ab47`, matching origin/main.
public-safety completed success on current main. advisories completed success
on current main. No failed required check was observed.

Branch-protection required contexts were classified. Direct current-main
check-runs satisfied the main required contexts. The associated merged PR #1376
head check-runs supplied exact success evidence for PR-head-only `goal-lint` and
aggregate `CodeQL` contexts.

## NA-0551 Terminal Stop Inheritance

NA-0551 is DONE as a terminal stop. The stopped remediation branch remains
preserved and unmerged:

- branch: `na-0551-remote-smoke-demo-script-remediation`
- HEAD: `2b897d658416`
- commits: `ab4ab9bba14f`, `2b897d658416`

No NA-0552 implementation occurred before this directive.

## Stopped Branch Review

The stopped branch exists on origin at the expected HEAD and is not an ancestor
of current main. Its diff against its original base is limited to:

- `scripts/demo/qsc_remote_handshake_smoke.sh`
- `scripts/demo/qsc_remote_relay_smoke.sh`

No open PR exists for the stopped branch.

## D465 / D466 Evidence Review

D465/D466 run metadata and redacted summaries were reviewed for:

- remote-handshake attempt 1: `28304536771`
- remote-relay attempt 1: `28304537372`
- remote-handshake attempt 2: `28304699022`
- remote-relay attempt 2: `28304701270`

D465 first reached missing-contact-route evidence, then D465/D466 evidence
converged on `relay_inbox_push_failed`. D466 records that deterministic
vault/contact setup and qsc payload construction completed before the remaining
bounded failure.

Exact HTTP status/body was not safely visible in the available redacted
evidence. Raw logs and raw artifacts were not copied into repository docs.

## qsc Relay Push Semantics Review

Read-only qsc source, docs, and tests identify these relay-push semantics:

- qsc constructs the relay push request internally from the relay base.
- qsc uses the canonical v1-path pattern for push.
- qsc uses route-token header carriage.
- qsc may add optional bearer auth from environment, account secret, or token
  file sources.
- qsc owns the payload body.
- qsc treats HTTP 200 as push success.
- qsc maps unauthorized, payload-too-large, and queue-full statuses to specific
  error codes.
- qsc maps send errors and unclassified statuses to
  `relay_inbox_push_failed`.
- qsc output does not expose exact generic status/body for this failure class.

No script-owned input or normalization bug was proven by read-only review.

## Relay Push Status / Body Visibility Review

Status/body visibility classification:
`RELAY_PUSH_STATUS_BODY_NOT_LOGGED`.

The available evidence safely shows the bounded qsc error code but not the
exact relay push status or body. Current qsc behavior intentionally avoids
leaking bearer/header material, path details, route tokens, and long
secret-like material in normal output. That safety posture is correct, but the
remaining boundary now needs a bounded diagnostic plan that exposes only a safe
status/error class.

## Secret / Environment Boundary Review

Secret/environment classification:
`SECRET_ENV_BOUNDARY_POSSIBLE_BUT_NOT_PROVEN`.

Workflow variable names reviewed by shape only:

- `RELAY_URL`
- `RELAY_TOKEN`
- `SCENARIO`
- `SEED`

qsc auth variable names reviewed by shape only:

- `QSC_RELAY_TOKEN`
- `RELAY_TOKEN`

The failure could be caused by remote endpoint state, auth mode, route-token
state, bearer token state, or deployment/API-shape mismatch. The safe evidence
does not prove that as the primary cause, and NA-0552 has no authority to
inspect secret values or remote service internals.

## remote-handshake Boundary Classification

`HANDSHAKE_RELAY_BOUNDARY_RELAY_API_STATUS_DIAGNOSTIC_REQUIRED`

The remote-handshake target reaches qsc-owned relay push and exposes only
`relay_inbox_push_failed` for the remaining bounded failure.

## remote-relay Boundary Classification

`RELAY_RELAY_BOUNDARY_RELAY_API_STATUS_DIAGNOSTIC_REQUIRED`

The remote-relay target reaches qsc-owned relay push and exposes only
`relay_inbox_push_failed` for the remaining bounded failure.

## Overall Result Classification

`REMOTE_RELAY_API_BOUNDARY_DIAGNOSTIC_INSTRUMENTATION_READY`

Safe evidence supports a bounded diagnostic instrumentation authorization lane
before renewed script remediation, qsc runtime remediation, or remote
environment remediation.

## Selected Successor

Selected successor:
`NA-0553 -- QSL Remote Relay API Boundary Diagnostic Instrumentation
Authorization Plan`.

The successor is authorization-only. It must select exact future diagnostic
paths, redaction policy, validation commands, and stop conditions before any
instrumentation is implemented.

## Required-Check Boundary

Current main public-safety is green. Current main advisories is green.
Branch-protection required contexts are green or conclusively satisfied using
current-main checks first and associated merged-PR head checks for contexts not
emitted on the merge commit. No failed required check was observed.

## Script Remediation Boundary

No script remediation occurred in NA-0552. The stopped D465/D466 branch remains
preserved and unmerged. Renewed script remediation is not authorized until a
future lane proves a script-owned fix.

## Workflow Mutation Boundary

No workflow files were mutated. No workflow dispatch or rerun was executed.

## Runtime / qsc / Dependency Boundary

No qsc source, qsc tests, qsc fuzz, Cargo manifest, Cargo lockfile, dependency,
or runtime path was mutated. qsc was not run locally.

## qsl-server / qsl-attachments Boundary

No qsl-server or qsl-attachments command, clone, build, run, local use, or
mutation occurred.

## Remote-Action Boundary

No remote command occurred. GitHub access was limited to read-only API/log
metadata for the authorized runs, checks, branch, and PR context.

## Public-Site / Cloudflare Boundary

No README public-progress, docs/public, website, public path, deployment, or
Cloudflare configuration was mutated.

## Private-Material Boundary

Secret values were not requested, printed, or committed. Raw logs and raw
artifacts were not committed. Endpoint values, route tokens, bearer material,
payloads, private topology, passphrases, and token hashes remain outside repo
docs.

## Claim Boundary

No public-readiness claim is made.
No production-readiness claim is made.
No public-internet-readiness claim is made.
No external-review-complete claim is made.
No reproducibility-complete claim is made.
No backup/restore-complete claim is made.
No vulnerability-free claim is made.
No bug-free claim is made.
No perfect-build claim is made.
No perfect-crypto claim is made.

## Validation

Validation for this lane is governance-only. Focused qsc runtime tests are
skipped because NA-0552 changed only authorization evidence/governance paths,
did not mutate qsc source/runtime/dependencies/workflows, and did not authorize
local reproduction or qsc send/receive.

Required validation includes diff checks, exact five-path scope guards,
queue/decision proof, marker proof, link-check, private-material scan,
overclaim scan, docs/governance-only classifier, PR body preflight, goal-lint
when available, cargo audits, cargo fmt, and qsc-adversarial shell syntax.

## Recommendation

Restore NA-0553 as the diagnostic instrumentation authorization successor after
NA-0552 closeout, if PR and post-merge checks stay green. The next lane should
not implement diagnostics; it should define exact future diagnostic paths and
redaction rules needed to expose only safe relay push status/error class
evidence.
