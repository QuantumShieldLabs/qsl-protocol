Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-27

# NA-0551 Remote Smoke Relay API Boundary Stop Handoff

## Executive Summary

NA-0551 is accepted as a terminal stop, not a completed remediation. D465
attempted exact script-only remediation on branch
`na-0551-remote-smoke-demo-script-remediation`, but branch validation still
failed at `relay_inbox_push_failed`. D466 reviewed the stopped branch and
selected `SCRIPT_ONLY_FIX_NOT_SUPPORTED_RELAY_API_BOUNDARY`.

The stopped remediation branch remains preserved as evidence and is not merged.
No PR was opened for that branch. This handoff records the relay API boundary
stop and selects `NA-0552 -- QSL Remote Relay API Boundary Diagnosis
Authorization Plan` as the next successor.

## qwork Proof Verification

NA-0551 qwork proof was verified from:

- `/srv/qbuild/work/NA-0551/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0551/.qwork/startup.qsl-protocol.json`
- `/srv/qbuild/work/NA-0551/.qwork/cargo-target.qsl-protocol.env`

The proof recorded `startup_result=OK`, lane `NA-0551`, repo `qsl-protocol`,
path `/srv/qbuild/work/NA-0551/qsl-protocol`, startup
head/origin-main/main `2ff3dd6c915c`, `head_equals_origin_main=yes`, clean
worktree/index/untracked state, READY_COUNT 1, READY `NA-0551`, and shared
cargo target readiness. The active stopped branch being ahead of that startup
main by the D465 remediation commits is expected evidence and is not a qwork
mismatch.

Codex did not run qwork, qstart, or qresume.

## D-1090 / D-1091 Inheritance

D-1090 and D-1091 were consumed. D-1090 selected NA-0551 as the exact
remote-handshake and remote-relay demo-script remediation lane. D-1091 restored
NA-0551 as the sole READY successor after NA-0550 closeout.

## D465 Attempt Summary

D465 attempted exact script-only remediation on branch
`na-0551-remote-smoke-demo-script-remediation`.

Stopped branch commits:

- `ab4ab9bba14f`
- `2b897d658416`

Branch validation attempts:

- Attempt 1 remote-handshake run `28304536771`: failed at missing contact route.
- Attempt 1 remote-relay run `28304537372`: failed at
  `relay_inbox_push_failed` after setup.
- Attempt 2 remote-handshake run `28304699022`: failed at
  `relay_inbox_push_failed`.
- Attempt 2 remote-relay run `28304701270`: failed at
  `relay_inbox_push_failed`.

D465 stopped before PR creation and before merge.

## D466 Recovery Diagnosis

D466 reviewed the stopped branch, D465 response, D465 proof-root artifacts, and
tracked qsc source/docs/tests read-only. D466 classified the remaining script
recovery as `SCRIPT_ONLY_FIX_NOT_SUPPORTED_RELAY_API_BOUNDARY`.

The diagnosis recorded:

- deterministic vault/contact setup occurs before the remaining failure;
- setup now reaches qsc payload creation;
- final bounded failure is `relay_inbox_push_failed`;
- exact HTTP status/body was not safely visible enough to prove a script-only
  fix;
- qsc source indicates a relay push v1-path pattern with a route-token header,
  optional bearer auth, and qsc-owned payload body;
- qsc maps send errors and non-classified statuses to
  `relay_inbox_push_failed`;
- further script patching would risk guessing at relay API, qsc runtime, secret,
  environment, or remote relay behavior.

## Branch State and Preservation

The stopped branch remains preserved as pushed evidence:

- branch: `na-0551-remote-smoke-demo-script-remediation`
- HEAD: `2b897d658416`
- base `origin/main`: `2ff3dd6c915c`
- diff limited to:
  - `scripts/demo/qsc_remote_handshake_smoke.sh`
  - `scripts/demo/qsc_remote_relay_smoke.sh`

This governance handoff branch is created from clean `origin/main` and does not
include those script changes.

## relay_inbox_push_failed Boundary

The failure is accepted as a relay API boundary stop. The scripts can supply the
relay base URL, auth-token environment, route tokens, and qsc command sequence,
but they do not own qsc's HTTP path, header/body construction, status mapping,
or remote relay behavior. Without safe visibility into the exact HTTP
status/body and endpoint behavior, script-only remediation cannot be proven.

## Script-Only Fix Decision

Selected decision: `SCRIPT_ONLY_FIX_NOT_SUPPORTED_RELAY_API_BOUNDARY`.

No additional script patch is authorized by this handoff. No recovery branch
validation is run. No script changes are merged.

## Private-Material Handling

Raw logs and artifact ZIPs remain proof-root-only. Repository evidence uses only
bounded redacted summaries and does not publish raw endpoints, route
capabilities, passphrase contents, token hashes, raw fingerprints, bearer
tokens, authorization headers, or private relay material.

## Scope Guard

This handoff changes only governance/evidence paths:

- this evidence document;
- the NA-0551 stop handoff testplan;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

No script, workflow, qsc source, dependency, lockfile, qsl-server,
qsl-attachments, public-site, or Cloudflare path is changed.

## Boundary Proof

This handoff performs no rerun, workflow dispatch, local reproduction, qsc
send/receive, qsc local/runtime reproduction, qsl-server command, qsl-attachments
command, dependency update, lockfile update, qsl-backup execution, backup
mutation, public-site mutation, Cloudflare mutation, qwork/qstart/qresume
execution, or private-material publication.

## Result Classification

`REMOTE_SMOKE_DEMO_SCRIPT_REMEDIATION_RELAY_API_BOUNDARY_STOP_ACCEPTED`

## Selected Successor

`NA-0552 -- QSL Remote Relay API Boundary Diagnosis Authorization Plan`

NA-0552 is authorization-only. It must inspect D465/D466 proof-root evidence,
read-only GitHub run/job/log metadata, preserved stopped-branch scripts, and
qsc relay push source/docs/tests enough to decide whether a later lane should
be qsc runtime review, remote relay environment/secret-boundary review,
diagnostic instrumentation, or renewed exact script remediation.

## No-Merge / No-PR Proof

No PR was opened for `na-0551-remote-smoke-demo-script-remediation`. The branch
was not merged. The branch remains preserved and unmodified by this handoff.

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

Validation for this handoff is governance-only. Focused qsc runtime tests are
skipped because no qsc source/runtime/dependency/workflow mutation occurred and
local reproduction is not authorized. Static governance validation, scope guard,
marker proof, link-check, private-material scan, overclaim scan, goal-lint where
available, cargo audits, cargo fmt, and qsc-adversarial shell syntax checks are
the relevant gates.

## Recommendation

Proceed with NA-0552 as a bounded authorization lane for the remote relay API
boundary. Do not resume script remediation until the relay/API/status/body,
secret/environment, or qsc runtime ownership question is resolved by future
authorized evidence.
