Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-01

# NA-0584 Remote Relay Runner / Service Reachability Remediation Harness

## Executive Summary

NA-0584 consumed D-1157 and D-1158, verified fresh qwork proof from
`2026-07-01T09:01:48Z`, and applied the D-1142/D-1143 bounded authority model.
The local qsl-server precheck classified the recovered relay state as ready:
listener ready, `/v1/push` route shape ready, `/v1/pull?max=N` route shape
ready, and log visibility available.

The exact D512/D-1157 failed workflow runs were rerun with `--failed`:

- remote-handshake run `28498817017`: completed failure.
- remote-relay run `28498817988`: completed failure.

Both reruns reached relay push and emitted redacted qsc relay diagnostics with
timeout class. qsc source review and artifacts classify the timeout only as
`qsc_generic_timeout`; they do not separate DNS, TCP connect, TLS handshake, or
HTTP request phase. Postrun remote snapshot still showed local relay ready, but
safe logs did not provide a pre/post request-arrival delta.

Failure-cause classification:
`REMOTE_RELAY_REACHABILITY_QSC_TIMEOUT_PHASE_CLASSIFIED`.

Result classification:
`REMOTE_RELAY_REACHABILITY_QSC_TIMEOUT_PHASE_ONLY`.

Selected successor:
`NA-0585 -- QSL Remote Relay Diagnostic Surface Improvement Harness`.

## qwork Proof Verification

- qwork proof files were verified before fetch, repository mutation, GitHub
  action, workflow artifact/log retrieval, SSH, or proof publication.
- Required values matched lane `NA-0584`, repo `qsl-protocol`, clean `main`,
  `HEAD`/`origin/main` `d86f153fe72d`, READY_COUNT 1, and queue top READY
  `NA-0584`.
- qwork proof timestamp was at or after `2026-07-01T09:01:48Z`.
- Pre-fetch live `HEAD` and `origin/main` matched the qwork proof.
- Root disk usage was below the stop threshold and `/backup/qsl` was mounted.
- Codex did not run `qwork`, `qstart`, or `qresume`.

## D-1157 / D-1158 Inheritance

- D-1157 exists once and is Accepted.
- D-1158 exists once and is Accepted.
- D-1159 was absent before this implementation patch.
- NA-0583 is DONE and NA-0584 is READY.
- D-1157 result was
  `REMOTE_RELAY_TRIAGE_ARTIFACT_BACKED_SERVICE_UNREACHABLE_AFTER_LOCAL_READY`.
- D-1157 artifact-backed facts consumed here: both workflows reached relay
  push, timeout class was present, HTTP status/body classes were unknown, auth
  and route header classes were present, and no qsc runtime failure or workflow
  harness failure was supported.
- D-1158 restored NA-0584 as the sole READY successor.

## Authority Model Application

- NA-0584 applied D-1142/D-1143.
- Tier 1 redacted diagnostics were used for GitHub workflow evidence, qsc source
  expectation review, qsl-server route expectation review, SSH readiness, local
  postcheck, and postrun snapshot.
- No Tier 2 remote host mutation was authorized or performed.
- No workflow file mutation was authorized or performed.
- Diagnostic reruns were evidence only, not a fix claim.
- Continuous CI wait-work and automatic failure-cause investigation applied.

## Current Main Required-Check Classification

- Current main before implementation: `d86f153fe72d`.
- public-safety: completed success.
- advisories: completed success.
- No failed or pending required check was classified before implementation.
- Root cargo audit: success.
- Nested qsc fuzz cargo audit: success.
- `cargo metadata --locked --format-version=1`: success.
- Cargo manifest/lock drift: absent.

## qsc Relay Timeout Semantics Review

- Endpoint URL source class: workflow `RELAY_URL` secret passed to qsc
  `--relay`; value not accessed.
- qsc push construction: normalized relay base plus `/v1/push`.
- qsc pull construction: normalized relay base plus `/v1/pull?max=N`.
- Route-token header: `X-QSL-Route-Token` on push and pull.
- Optional bearer class: Authorization bearer when qsc finds a relay token; no
  value accessed.
- Timeout source class: no explicit qsc relay reqwest timeout or workflow
  `timeout-minutes` was found in reviewed push/pull/workflow files.
- Redacted diagnostics map reqwest timeout to `timeout`, TLS/certificate text
  to `tls_error`, reqwest connect to `network_error`, and other send errors to
  `transport_error`.
- Artifact timeout phase classification: `qsc_generic_timeout`.
- DNS/TCP/TLS/HTTP network-side phase remains unproven.

## qsl-server Route Review

- Canonical route shape: `POST /v1/push` and `GET /v1/pull?max=N`.
- Route usage requires `X-QSL-Route-Token`.
- Bearer auth is optional when configured; no value was accessed.
- Payload/body logging is not expected or published.
- Safe local route-shape postcheck used no secret and no body publication.

## Remote Script Design and Static Review

- Generated proof-root-only scripts:
  - `qsl_server_reachability_precheck.py`
  - `qsl_server_reachability_postrun_snapshot.py`
- Syntax review passed.
- Python stdlib only.
- JSON-only stdout.
- No `shell=True`.
- No sudo, systemctl, service, journalctl, Tailscale, firewall, qsc
  send/receive, qsl-attachments, authorized_keys access, secret-file access, or
  broad home-directory scan.
- No endpoint, private port, process identity, token, topology, command line,
  payload, response body, raw log, or key material is printed.
- No remote mutation calls are present.

## SSH Readiness

- The single authorized readiness command exited 0.
- Classification: `SSH_READINESS_READY`.
- Recovered classifier issue: the directive-form remote `printf` emitted the
  expected sentinel with a literal trailing marker character. The classifier was
  corrected without rerunning SSH.

## Local qsl-server Precheck

- Classification:
  `REMOTE_RELAY_REACHABILITY_PRECHECK_LOCAL_READY`.
- expected-bind listener ready class: ready.
- push route shape class: ready with no-secret/no-body 4xx response class.
- pull route shape class: ready with no-secret/no-body 4xx response class.
- local relay ready class: ready.
- log visibility class: available.
- Disclosure flags for endpoint, private port, process identity, token, response
  body, private topology, and raw private material were all false.

## Workflow Metadata and Diagnostic Action

- Reviewed exactly:
  - `.github/workflows/remote-handshake-tests.yml`
  - `.github/workflows/remote-relay-tests.yml`
- D512/D-1157 exact run IDs were accessible and unambiguous.
- Diagnostic action plan:
  - `gh run rerun 28498817017 --failed`
  - `gh run rerun 28498817988 --failed`
- No workflow file mutation was required.
- No secret value access was required.

## Workflow Polling

- No watch mode was used.
- remote-handshake run `28498817017`: completed failure.
- remote-relay run `28498817988`: completed failure.
- Both jobs completed failure.
- Productive wait-work was performed while runs were pending.

## Workflow Artifact Review

remote-handshake:

- reached relay push class: reached.
- timeout class present: yes.
- timeout phase class: `qsc_generic_timeout`.
- HTTP status class: present as unknown class.
- response body class: present as unknown class.
- auth header class: present.
- route-token header class: present.
- redaction review: pass.

remote-relay:

- reached relay push class: reached.
- timeout class present: yes.
- timeout phase class: `qsc_generic_timeout`.
- HTTP status class: present as unknown class.
- response body class: present as unknown class.
- auth header class: present.
- route-token header class: present.
- payload artifact file was quarantined proof-root-only and not published.
- redaction review: pass.

## Workflow Log Review

- Raw logs were fetched proof-root-only and scanned before summarization.
- Logs are summarized only as safe classes.
- remote-handshake: relay push diagnostic present, timeout class present, no
  unmasked Authorization value published.
- remote-relay: relay push diagnostic present, timeout class present, no
  unmasked Authorization value published.
- No endpoint value, private port value, token value, payload/body, or response
  body was published.

## Remote Postrun Snapshot

- Classification: `REMOTE_POSTRUN_CLASSIFIED`.
- local relay still ready class: ready.
- request arrival delta class: unavailable.
- request arrival time overlap class: not observed.
- response completion observed class: not observed.
- error class observed: present.
- log visibility class: available.
- Disclosure flags for endpoint, private port, process identity, token, response
  body, private topology, and raw private material were all false.

## Failure-Cause Investigation

Classification:
`REMOTE_RELAY_REACHABILITY_QSC_TIMEOUT_PHASE_CLASSIFIED`.

Basis:

- local precheck was ready;
- postrun local relay was still ready;
- both workflow reruns reached relay push and failed with timeout class;
- qsc/artifact timeout phase was only `qsc_generic_timeout`;
- no safe evidence supported endpoint/secret mismatch;
- no safe evidence supported auth/route-token/bearer failure;
- no safe evidence supported qsl-server local regression;
- no safe evidence proved request arrival or response noncompletion;
- safe logs did not provide a pre/post request-arrival delta.

## Private-Material Review

- Raw SSH output, workflow logs, workflow artifacts, and generated scripts were
  scanned before use.
- Raw logs/artifacts remain proof-root-only.
- Repository evidence publishes only safe classes, run IDs, workflow names, job
  names, and no-disclosure flags.
- No endpoint values, private ports, route-token/capability values, bearer
  values, unmasked Authorization values, private topology, process identity,
  command lines, payloads, response bodies, authorized_keys content, public key
  material, private key material, secret values, Cloudflare tokens, API tokens,
  or private material are published.

## Result Classification

Selected result:
`REMOTE_RELAY_REACHABILITY_QSC_TIMEOUT_PHASE_ONLY`.

This is a partial diagnostic classification only. It does not claim runner
network nonreachability, endpoint/secret mismatch, auth/route-token failure,
service response noncompletion, or local qsl-server regression as root cause.

## Selected Successor

### NA-0585 -- QSL Remote Relay Diagnostic Surface Improvement Harness
Status: READY
Goals: G1, G2, G3, G4, G5

Objective:
Improve safe diagnostic surfacing for remote-handshake and remote-relay so future
runs classify DNS/TCP/TLS/HTTP timeout phase, request-arrival class, and redacted
relay result directly in logs/artifacts. Any workflow/test harness mutation must
be exact, minimal, no-secret, and preserve no-private-material publication.

## Required-Check Boundary

Required-check classification before implementation was green for public-safety
and advisories, with no failed or pending required checks. Workflow diagnostic
reruns are evidence and are not treated as required-check success.

## Source / Script Mutation Boundary

No qsl-protocol source, script, workflow, dependency, Cargo manifest, or lockfile
mutation occurred. The implementation patch is governance/testplan/journal only.

## Workflow Mutation Boundary

No workflow file was changed. The only workflow action was rerunning failed jobs
for the exact D512/D-1157 runs.

## Runtime / qsc Boundary

No qsc source/runtime/dependency mutation occurred. No manual qsc send/receive
was run.

## qsl-server / qsl-attachments Boundary

No qsl-server source mutation, start, stop, cleanup, deployment, or PR occurred.
No qsl-attachments command, clone, build, run, or mutation occurred.

## Remote-Action Boundary

Remote action was limited to the exact allowed SSH readiness, precheck, and
postrun commands. No remote mutation occurred.

## Public-Site / Cloudflare Boundary

No public-site and no Cloudflare action occurred.

## Raw Output Boundary

Raw SSH stdout/stderr, workflow logs, and downloaded workflow artifacts remain
proof-root-only. Repository evidence does not contain raw logs or raw artifacts.

## Claim Boundary

This lane makes no public-readiness, production-readiness, public-internet
readiness, vulnerability-free, bug-free, perfect-build, or perfect-crypto claim.

## Validation

Validation artifacts include qwork proof verification, queue/decision proof,
current-main checks, source expectation review, script static review, SSH
readiness, precheck/postrun private-material scans, workflow artifact/log scans,
failure-cause classification, result classification, selected successor, marker
proof, scope guard, link-check, private-material scan, overclaim scan, cargo
audits, locked metadata, fmt check, shell syntax checks, PR body preflight, and
goal-lint.

## Recommendation

Proceed to closeout after implementation merge if required checks are green and
restore the selected NA-0585 diagnostic-surface successor. Do not implement
NA-0585 in this lane.
