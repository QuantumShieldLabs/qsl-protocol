Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-01

# NA-0582 Remote Relay Recovered Test Verification Harness

## Executive Summary

NA-0582 consumed D-1153 and D-1154, verified fresh qwork proof from
`2026-07-01T06:12:58Z`, verified current main at `788689cbd86e`, and applied
the D511 bounded authority model.

The recovered qsl-server postcheck passed:
`REMOTE_RELAY_RECOVERED_QSL_SERVER_POSTCHECK_READY`.

Both exact recovered verification workflows were dispatched on `main` after the
postcheck:

- remote-handshake run `28498817017`: completed failure.
- remote-relay run `28498817988`: completed failure.

Failure-cause classification:
`REMOTE_RELAY_VERIFICATION_FAILURE_SERVICE_UNREACHABLE_AFTER_LOCAL_READY`.

Result classification:
`REMOTE_RELAY_RECOVERED_VERIFICATION_FAIL_SERVICE_UNREACHABLE_AFTER_LOCAL_READY`.

Selected successor:
`NA-0583 -- QSL Remote Relay Service Reachability After Local Ready Triage Harness`.

## qwork Proof Verification

- qwork proof files were copied before fetch, repository mutation, SSH, workflow
  action, workflow log retrieval, or repository proof publication.
- Required qwork values matched lane `NA-0582`, repo `qsl-protocol`, path
  `/srv/qbuild/work/NA-0582/qsl-protocol`, branch `main`, upstream
  `origin/main`, clean worktree/index/untracked state, READY_COUNT 1, queue top
  READY `NA-0582`, shared cargo target mode, and shared target ready.
- qwork proof timestamp was verified at `2026-07-01T06:12:58Z`.
- Live pre-fetch `HEAD` and `origin/main` matched qwork proof at
  `788689cbd86e`.
- Codex did not run `qwork`, `qstart`, or `qresume`.

## D-1153 / D-1154 Inheritance

- D-1153 exists once and is Accepted.
- D-1154 exists once and is Accepted.
- D-1155 was absent before the implementation patch.
- NA-0581 is DONE.
- NA-0582 is READY.
- D-1153 result classification was
  `QSL_SERVER_EXPECTED_BIND_REMEDIATION_RELAY_TESTING_READY`.
- D-1153 expected-bind start succeeded.
- D-1153 expected-bind postcheck passed.
- D-1153 cleanup was not needed.
- D-1153 left the recovered qsl-server state available for NA-0582.
- D-1154 restored NA-0582 as the sole READY item.

## Authority Model Application

- Tier 1 redacted diagnostics were limited to host label `inspiron` and
  workspace `/home/qslcodex/qsl-remote-test/`.
- Remote action was limited to the exact SSH readiness command and the exact
  recovered postcheck command through SSH stdin.
- No remote mutation, qsl-server start, qsl-server stop, or qsl-server cleanup
  occurred.
- Workflow action was limited to `remote-handshake-tests.yml` and
  `remote-relay-tests.yml`, after recovered postcheck passed.
- Workflow secret values were not accessed.
- Automatic failure-cause investigation was applied after both workflows failed.
- Continuous CI wait-work and read-only forward-audit policy was applied.

## Current Main Required-Check Classification

- Current main: `788689cbd86e`.
- public-safety: completed success.
- advisories: completed success.
- suite2-vectors: completed success.
- No failed required checks were classified before implementation.
- No required pending checks were classified before implementation.
- Root cargo audit: success.
- Nested qsc fuzz cargo audit: success.
- `cargo metadata --locked --format-version=1`: success.
- Cargo manifest/lock drift: absent.

## qsl-protocol / qsc Relay Expectation Review

- `remote-handshake-tests.yml` is present.
- `remote-relay-tests.yml` is present.
- Both workflows use the `RELAY_URL` and `RELAY_TOKEN` secret-name boundary.
- Secret values were not accessed.
- qsc push path expectation is `/v1/push`.
- qsc pull path expectation is `/v1/pull?max=N`.
- qsc route header expectation is `X-QSL-Route-Token`.
- Optional bearer auth uses Authorization bearer when a relay token is present.
- Prior failure class was network/TLS/timeout before HTTP status/body.
- NA-0582 verifies whether the recovered qsl-server setup resolves that class.

## Remote Script Design and Static Review

- Proof-root-only script:
  `remote_scripts/qsl_server_recovered_postcheck.py`.
- Python stdlib only.
- Syntax review passed.
- JSON-only stdout.
- No `shell=True`.
- No sudo, systemctl, service, Tailscale, firewall, journalctl, ps, ss, netstat,
  or lsof command execution.
- No authorized_keys access.
- No secret-file access.
- No broad home-directory scan.
- No qsc send/receive.
- No qsl-attachments.
- No endpoint, token, body, topology, process identity, raw port, or command
  line printing.
- No remote mutation, qsl-server start, qsl-server stop, or cleanup.

## SSH Readiness

Classification: `SSH_REMOTE_RELAY_RECOVERED_VERIFY_READY`.

The single authorized SSH readiness command exited 0 and emitted the directive
sentinel with the known literal trailing marker shape. It was not rerun.

Private-material scan: pass.

## Recovered qsl-server Postcheck

Classification: `REMOTE_RELAY_RECOVERED_QSL_SERVER_POSTCHECK_READY`.

- inherited NA-0581 state class: confirmed.
- expected-bind listener ready class: yes.
- push route shape class: canonical push route present with no-body rejection.
- pull route shape class: canonical pull route present.
- recovered relay postcheck ready class: yes.
- endpoint value disclosed: no.
- private port value disclosed: no.
- process identity disclosed: no.
- token value disclosed: no.
- response body disclosed: no.
- private topology disclosed: no.
- raw output contains private material: no.
- redaction review: pass.

## Workflow Metadata Review

remote-handshake:

- workflow file exists: yes.
- workflow_dispatch exists: yes.
- required inputs: none.
- latest failed current-main run before dispatch: none.
- dispatch possible: yes.
- secret values accessed: no.

remote-relay:

- workflow file exists: yes.
- workflow_dispatch exists: yes.
- required inputs: `scenario`, `seed`.
- required inputs have defaults: yes.
- latest failed current-main run before dispatch: none.
- dispatch possible: yes.
- secret values accessed: no.

## Workflow Rerun / Dispatch

- No suitable failed current-main run existed for either exact workflow.
- `remote-handshake-tests.yml` was dispatched on `main`.
- `remote-relay-tests.yml` was dispatched on `main`.
- No other workflow was dispatched or rerun.
- No watch mode was used.

## Workflow Polling

- Bounded REST polling was used.
- Interval: 20 seconds.
- Iterations used: 6.
- Elapsed polling time: about 110 seconds.
- remote-handshake run `28498817017`: completed failure.
- remote-relay run `28498817988`: completed failure.
- Job metadata and check-run IDs were captured proof-root-only.

## Workflow Log Review

- Raw logs were fetched proof-root-only for both failed runs.
- Raw logs remain quarantined and are not copied into repository evidence.
- Workflow log private-material scan passed for publishable summaries.
- No route-like values were found by the log scanner.
- No safe qsc relay diagnostic marker was present in raw logs.
- Safe redacted failure summary: both workflows reached their remote smoke step
  and exited nonzero.

## Failure-Cause Investigation

Classification:
`REMOTE_RELAY_VERIFICATION_FAILURE_SERVICE_UNREACHABLE_AFTER_LOCAL_READY`.

Basis:

- local recovered qsl-server postcheck passed immediately before workflow
  verification;
- both exact GitHub workflows were dispatched on current `main`;
- both workflow remote smoke steps failed;
- workflow logs displayed only masked secret-name environment lines;
- raw logs did not contain a safe HTTP status/body diagnostic marker.

Not classified as:

- qsl-server listener regression;
- endpoint/secret mismatch;
- auth or route-token failure;
- qsc client runtime failure;
- workflow harness issue;
- private-material stop.

## Private-Material Review

- SSH readiness scan: pass.
- remote postcheck scan: pass.
- workflow metadata scan: pass.
- workflow log scan: pass for publishable summaries, raw logs quarantined.
- added repository evidence scan: required before PR.
- No endpoint values are published.
- No private port values are published.
- No route-token/capability values are published.
- No bearer or Authorization values are published.
- No private topology is published.
- No process identity is published.
- No command lines are published.
- No payloads or response bodies are published.
- No authorized_keys content or key material is published.

## Result Classification

`REMOTE_RELAY_RECOVERED_VERIFICATION_FAIL_SERVICE_UNREACHABLE_AFTER_LOCAL_READY`.

## Selected Successor

`NA-0583 -- QSL Remote Relay Service Reachability After Local Ready Triage Harness`

Status: READY

Goals: G1, G2, G3, G4, G5

Objective: triage why exact GitHub remote-handshake and remote-relay verification
workflows still fail after the recovered qsl-server local postcheck is ready,
using redacted workflow logs, qsl-server postcheck metadata, and safe GitHub
metadata only unless a later directive authorizes additional action.

## Required-Check Boundary

NA-0582 did not weaken required checks, public-safety, advisories, goal-lint, or
branch protection. Failed remote verification workflows are classified as
evidence, not treated as success.

## Source / Script Mutation Boundary

No qsl-protocol source, repository script, dependency, lockfile, qsc runtime,
qsc source, qsc test, qsc fuzz, qsl-server source, or qsl-attachments source was
mutated.

## Workflow Mutation Boundary

No workflow file was mutated. No workflow was cancelled or deleted. Only the two
exact workflow_dispatch actions authorized by D511 were used.

## Runtime / qsc Boundary

No manual qsc send/receive was run. No local qsc reproduction was run. Runtime
evidence is limited to the recovered postcheck and the two GitHub workflows.

## qsl-server / qsl-attachments Boundary

No qsl-server start, stop, cleanup, deployment, source mutation, or PR occurred.
No qsl-attachments command, clone, build, run, or mutation occurred.

## Remote-Action Boundary

Remote commands were limited to the exact SSH readiness command and exact SSH
stdin postcheck command. No sudo, systemctl, service, firewall, Tailscale,
account/shell/authorized_keys mutation, ps, ss, netstat, lsof, qsc, or
qsl-attachments command was run.

## Public-Site / Cloudflare Boundary

No public-site or Cloudflare action occurred.

## Raw Output Boundary

Raw SSH output, remote postcheck output, workflow metadata, and raw workflow logs
remain proof-root-only. Repository evidence publishes only coarse classes, run
IDs, workflow names, check names, and redacted summaries.

## Claim Boundary

NA-0582 makes no public-readiness, production-readiness, public-internet-
readiness, external-review-complete, vulnerability-free, bug-free, perfect-build,
or perfect-crypto claim.

## Validation

Required pre-PR validation is recorded in the proof root. It includes scope
guard, queue/decision proof, marker proof, link-check, private-material scans,
overclaim scan, docs/governance-only classifier, PR body preflight, goal-lint,
cargo audits, locked metadata, formatting, and qsc-adversarial shell syntax.

## Recommendation

Proceed with the selected service-reachability-after-local-ready triage
successor. The next lane should determine why GitHub runner verification still
fails after a local recovered qsl-server postcheck passes, without publishing or
requesting private endpoint, token, Authorization, topology, payload, response
body, process, authorized_keys, or key material.
