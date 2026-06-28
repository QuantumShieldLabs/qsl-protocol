Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-28

# NA-0555 Remote Relay API Boundary Diagnostic Evidence Capture Harness

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0555 captured bounded evidence from the two Director-authorized main-branch
workflow dispatches for the remote-handshake and remote-relay relay-push
boundary. Both runs reached `relay_inbox_push_failed` and emitted the NA-0554
redacted qsc relay-push diagnostic marker. Both targets classified as network
or TLS timeout at the remote endpoint boundary, with no HTTP status or response
body visible to qsc.

Overall result classification:
`REMOTE_RELAY_DIAGNOSTIC_EVIDENCE_REMOTE_ENV_SECRET_BOUNDARY_READY`.

Selected successor:
`NA-0556 -- QSL Remote Relay Environment and Secret Boundary Review Authorization Plan`.

## qwork Proof Verification

Fresh qwork proof was copied from the NA-0555 lane workspace and parsed from the
`.kv`, JSON, and cargo-target env files. The proof timestamp was
`2026-06-28T14:03:22Z`, lane `NA-0555`, repo `qsl-protocol`, and startup
HEAD/origin/main/main was `e5f49a17c357`. Codex did not run qwork, qstart, or
qresume.

Proof-root qwork verification result: PASS.

## D-1098 / D-1099 Inheritance

D-1098 and D-1099 were consumed and each exists once with Status Accepted.
D-1098 accepted the NA-0554 redacted diagnostic instrumentation implementation.
D-1099 accepted NA-0554 closeout and restored NA-0555 as the sole READY
successor. NA-0554 is DONE, NA-0555 is READY, D-1100 and D-1101 were absent
before the NA-0555 patch, and duplicate decision count was zero.

Inherited NA-0554 result classification:
`REMOTE_RELAY_DIAGNOSTIC_INSTRUMENTATION_IMPLEMENTATION_PASS`.

## Current Main Required-Check Classification

Current main was verified at `e5f49a17c357`, equal to origin/main. The
branch-protection required contexts were classified. public-safety completed
success, advisories completed success, suite2-vectors completed success or was
conclusively satisfied, no failed required checks were observed, and no
required checks were pending.

The initial classifier did not find `CodeQL` and `goal-lint` on the merge
commit check-run list. That was recovered as a required-check visibility issue
by capturing the associated PR-head check-runs for the PR-only contexts. Final
classification result: PASS.

## Dispatch Gate

Dispatch gate checks passed before each workflow dispatch:

- clean local `main`;
- local `main` equaled `origin/main`;
- HEAD descended from `e5f49a17c357`;
- `.github/workflows/remote-handshake-tests.yml` existed;
- `.github/workflows/remote-relay-tests.yml` existed;
- qsc redacted diagnostics were present on main;
- both demo scripts enabled `QSC_RELAY_PUSH_DIAGNOSTIC=redacted`;
- both dispatch commands matched the allowlist exactly.

## Workflow Dispatches

Exactly two workflow dispatches were executed:

- `gh workflow run remote-handshake-tests.yml --ref main`
- `gh workflow run remote-relay-tests.yml --ref main -f scenario=happy-path -f seed=1`

No rerun, cancel, delete, or additional workflow dispatch was executed.

## Run / Job / Artifact Metadata

remote-handshake:

- run ID: `28325075419`
- job ID: `83913585385`
- workflow: `remote-handshake-tests`
- ref: `main`
- head SHA: `e5f49a17c357`
- event: `workflow_dispatch`
- attempt: `1`
- conclusion: `failure`
- failed step: `Run remote handshake smoke (happy-path seed=1)`
- artifact count: `1`

remote-relay:

- run ID: `28325168201`
- job ID: `83913828473`
- workflow: `remote-relay-tests`
- ref: `main`
- head SHA: `e5f49a17c357`
- event: `workflow_dispatch`
- attempt: `1`
- conclusion: `failure`
- failed step: `Run remote relay smoke (manual/nightly)`
- artifact count: `1`

## Log Capture Inventory

Raw Actions logs were captured proof-root-only for both exact runs. The raw
Actions logs did not contain the redacted relay-push diagnostic marker, so the
exact run artifacts were downloaded proof-root-only to classify the diagnostic
boundary.

Raw logs and artifacts were not copied into repository docs.

## Redaction and Private-Material Review

Private-material scans were run over raw logs, raw artifacts, redacted
summaries, diagnostic extracts, and candidate repository summaries before this
document was written.

Results:

- raw-log scan: PASS with proof-root quarantine;
- artifact scan: PASS with proof-root quarantine;
- redacted-summary scan: PASS;
- raw artifacts remain proof-root-only;
- candidate repository summaries contain only safe diagnostic fields.

The remote-relay artifact bundle included a request-payload artifact file. That
content remains proof-root-only and is not summarized here.

## remote-handshake Diagnostic Evidence

remote-handshake reached relay push and emitted one redacted diagnostic marker
from the consolidated artifact marker file.

Safe observed fields:

- status class: `unknown`
- status code: `unknown`
- error class: `timeout`
- response body present: `unknown`
- response body length: `unknown`
- route-token header present: `true`
- bearer auth present: `true`
- endpoint label: `relay_push_v1`
- qsc error: `relay_inbox_push_failed`
- qsc attempt count: `1`

## remote-relay Diagnostic Evidence

remote-relay reached relay push and emitted two redacted diagnostic markers from
the consolidated artifact marker file.

Safe observed fields:

- status class: `unknown`
- status code: `unknown`
- error class: `timeout`
- response body present: `unknown`
- response body length: `unknown`
- route-token header present: `true`
- bearer auth present: `true`
- endpoint label: `relay_push_v1`
- qsc error: `relay_inbox_push_failed`
- qsc attempt count: `1` for each send attempt

## Per-Target Diagnostic Classification

remote-handshake classification:
`REMOTE_HANDSHAKE_DIAGNOSTIC_NETWORK_TLS_TIMEOUT`.

remote-relay classification:
`REMOTE_RELAY_DIAGNOSTIC_NETWORK_TLS_TIMEOUT`.

Both targets point to network or remote endpoint environment ownership. The
evidence does not prove script-owned remediation, qsc runtime remediation, or
remote API status-shape remediation.

## Overall Result Classification

`REMOTE_RELAY_DIAGNOSTIC_EVIDENCE_REMOTE_ENV_SECRET_BOUNDARY_READY`.

Rationale: both authorized main-branch runs reached relay push and emitted
redacted timeout diagnostics with unknown status/body, route-token header
present, and bearer auth present. That points to remote endpoint, environment,
or secret-boundary review rather than local script, qsc runtime, or API
status-shape remediation.

## Selected Successor

Selected successor:
`NA-0556 -- QSL Remote Relay Environment and Secret Boundary Review Authorization Plan`.

The successor is authorization-only. It must define exact non-secret operator
proof requirements, redaction rules, classification rules, and successor
selection. It must not request or publish secret values.

## Required-Check Boundary

Current-main required checks were classified before dispatch. The diagnostic
workflow failures are target evidence from the two authorized workflow_dispatch
runs and are not treated as branch-protection required-check regressions for
main.

## Source / Script Mutation Boundary

No qsc source, qsc tests, qsc fuzz files, Cargo files, demo scripts, or workflow
files were mutated by NA-0555.

## Workflow Mutation Boundary

No workflow file was changed. Only the two exact authorized workflow dispatches
were executed. No rerun was executed.

## Runtime / qsc / Dependency Boundary

No local qsc send/receive, local qsc E2EE, qsc runtime reproduction, dependency
update, manifest update, or lockfile update occurred.

## qsl-server / qsl-attachments Boundary

No qsl-server or qsl-attachments command, clone, build, run, or mutation
occurred.

## Public-Site / Cloudflare Boundary

No README public-progress content, docs/public content, website path, public
path, public-site content, deployment setting, or Cloudflare configuration was
changed.

## Raw-Log / Artifact Boundary

Raw logs and artifacts remain proof-root-only. Repository docs include only
bounded redacted summaries and run/job identifiers.

## Claim Boundary

No public-readiness claim was made. No production-readiness claim was made. No
public-internet-readiness claim was made. No external-review-complete claim was
made. No reproducibility-complete claim was made. No backup/restore-complete
claim was made. No vulnerability-free claim was made. No bug-free claim was
made. No perfect-build or perfect-crypto claim was made.

## Validation

Validation covered qwork proof, queue proof, D-1098/D-1099 inheritance,
current-main required-check classification, exact dispatch gate proof, workflow
run/job/artifact metadata capture, raw log capture, artifact download only when
needed for classification, private-material scanning, per-target
classification, overall classification, and successor selection.

Focused qsc runtime tests are intentionally not part of NA-0555 because this
lane is evidence-capture/governance-only and no qsc runtime/source/dependency
or workflow mutation occurred.

## Recommendation

Proceed to close out NA-0555 only after the implementation PR merges and
post-merge checks are green. The next READY item should be the selected
NA-0556 remote relay environment and secret-boundary review authorization lane.
