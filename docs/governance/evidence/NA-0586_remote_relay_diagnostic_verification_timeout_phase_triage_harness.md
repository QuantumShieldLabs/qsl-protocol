Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-01

# NA-0586 Remote Relay Diagnostic Verification and Timeout Phase Triage Harness

## Executive Summary

NA-0586 consumed D-1161 and D-1162, verified fresh qwork proof from
`2026-07-01T20:57:45Z`, verified current main at `d4b8b9545b2a`, dispatched
the exact authorized remote-handshake and remote-relay workflows on `main`, and
classified both new diagnostic surfaces as present and parseable.

Result classification:
`REMOTE_RELAY_DIAGNOSTIC_VERIFICATION_DNS_TIMEOUT`.

Selected successor:
`NA-0587 -- QSL Remote Relay Network Path Remediation Harness`.

## qwork Proof Verification

- qwork proof files were copied into the proof root and parsed before fetch,
  repository mutation, GitHub workflow action, artifact/log retrieval, source
  analysis publication, or proof publication.
- Required qwork values matched lane `NA-0586`, repo `qsl-protocol`, clean
  `main`, READY_COUNT 1, queue top READY `NA-0586`, shared cargo target ready,
  and proof timestamp at or after `2026-07-01T20:57:45Z`.
- Pre-fetch live `HEAD` and `origin/main` matched qwork proof at
  `d4b8b9545b2a`.
- Root disk usage was below the stop threshold and `/backup/qsl` was mounted.
- Codex did not run `qwork`, `qstart`, or `qresume`.

## D-1161 / D-1162 Inheritance

- D-1161 exists once and is Accepted.
- D-1162 exists once and is Accepted.
- D-1163 was absent before this implementation patch.
- NA-0585 is DONE.
- NA-0586 is READY.
- D-1161 result `REMOTE_RELAY_DIAGNOSTIC_SURFACE_SAFE_FIX_IMPLEMENTED` was
  consumed.
- D-1162 restored NA-0586 as the sole READY successor.

## Authority Model Application

NA-0586 applied the D-1161-expanded issue-investigation authority model. Source
analysis, proof-root scanner/parser repair, and strict in-scope diagnostic fixes
were available. No diagnostic source fix was needed because both workflows
produced parseable safe diagnostic classes. Workflow action was limited to
`remote-handshake-tests.yml` and `remote-relay-tests.yml`.

## Current Main Required-Check Classification

Current main at `d4b8b9545b2a` had public-safety success, advisories success,
suite2-vectors success, no failed required checks, no required pending checks,
root cargo audit success, nested qsc fuzz cargo audit success, locked metadata
success, and no Cargo drift.

## Current Diagnostic Surface Review

The qsc relay-push diagnostic marker exists and includes safe fields for
diagnostic class, timeout phase class, status class, body presence/length class,
route-header presence, auth presence, and qsc error class. The remote helper
scripts summarize relay-push diagnostic count, diagnostic classes, timeout phase
classes, and status classes into artifacts. Pull-side diagnostics remain
coarser. Helper extraction is key-based rather than event-scoped, but NA-0586
artifact parsing used event-scoped extraction for the relay-push diagnostic
review.

## Workflow Metadata and Diagnostic Action

Both authorized workflow files exist and expose `workflow_dispatch`.
`remote-relay-tests.yml` has required inputs with defaults, so dispatch on
`main` was valid without extra inputs.

Executed exact allowlisted commands:

- `gh workflow run remote-handshake-tests.yml --ref main`
- `gh workflow run remote-relay-tests.yml --ref main`

Run IDs:

- remote-handshake: `28548002907`
- remote-relay: `28548012640`

Both runs used `workflow_dispatch` on `main` at `d4b8b9545b2a`.

## Workflow Polling

No watch mode was used. Bounded polling showed:

- remote-handshake run `28548002907`: completed failure.
- remote-relay run `28548012640`: completed failure.

The workflow failures are not treated as success. Their diagnostic artifacts are
used only to classify the safe timeout phase.

## Workflow Artifact Review

Artifacts were downloaded proof-root-only and scanned before parsing.

- remote-handshake artifacts: diagnostic surface present and parseable;
  diagnostic class `dns_timeout`; timeout phase `dns_timeout`.
- remote-relay artifacts: diagnostic surface present and parseable; diagnostic
  class `dns_timeout`; timeout phase `dns_timeout`.

Raw artifact contents remain proof-root-only.

## Workflow Log Review

Workflow logs were captured proof-root-only and scanned before use. The decision
uses only safe class summaries extracted from artifacts and redacted markers.
Raw logs are not committed or published.

## Diagnostic Surface Verification

Remote-handshake diagnostic availability:
`REMOTE_RELAY_DIAGNOSTIC_SURFACE_PRESENT_AND_PARSEABLE`.

Remote-relay diagnostic availability:
`REMOTE_RELAY_DIAGNOSTIC_SURFACE_PRESENT_AND_PARSEABLE`.

Timeout phase classification:
`REMOTE_RELAY_TIMEOUT_PHASE_DNS_TIMEOUT`.

## Automatic Failure-Cause Investigation

Phase 8 source/fix investigation was skipped because the trigger condition did
not occur: diagnostics were not missing, malformed, conflicting, or generic
only. The safe defect class is
`DIAGNOSTIC_VERIFY_DEFECT_NOT_REPRODUCIBLE`.

## Diagnostic Fix, If Any

No diagnostic source/helper fix was applied. Selected diagnostic mutation paths:
none.

## Optional Remote Postcheck

`REMOTE_POSTCHECK_NOT_RUN_NOT_NEEDED`. The workflow diagnostics already
classified the timeout phase, so no SSH command was run and no remote mutation
occurred.

## Private-Material Review

Private-material review passed. No endpoint value, private port value, token
value, Authorization value, route-token/capability value, payload, response
body, process identity, private topology, authorized_keys content, public key
material, private key material, or secret value is published. Raw logs and raw
artifacts remain proof-root-only.

## No-Semantics-Change Review

No crypto, protocol, wire, auth, state-machine, request construction,
fail-closed, dependency, lockfile, workflow, qsl-server, qsl-attachments,
public-site, or Cloudflare semantics changed.

## Result Classification

`REMOTE_RELAY_DIAGNOSTIC_VERIFICATION_DNS_TIMEOUT`.

## Selected Successor

Option A was selected.

### NA-0587 -- QSL Remote Relay Network Path Remediation Harness
Status: READY
Goals: G1, G2, G3, G4, G5

Objective:
Remediate the artifact-backed network path failure class selected by NA-0586
without publishing endpoint values, private ports, topology, tokens,
Authorization values, payloads, response bodies, process identities, command
lines, or key material. Codex may use D-1163-authorized redacted diagnostics and
bounded project-owned fixes only. Remote host, tunnel, DNS, or secret actions
remain operator-owned unless a later exact directive authorizes them.

## Required-Check Boundary

Required checks were not weakened. Failed or pending checks must not be treated
as success. The implementation PR must pass public-safety and advisories before
merge.

## Source / Script Mutation Boundary

No qsc source, qsc test, or helper script mutation was applied in NA-0586.

## Workflow Mutation Boundary

No workflow file was mutated. Workflow dispatch was evidence only.

## Runtime / qsc Boundary

No manual qsc send/receive was run. qsc runtime semantics were not changed.

## qsl-server / qsl-attachments Boundary

No qsl-server source mutation, qsl-server runtime mutation, qsl-attachments
command, or qsl-attachments mutation occurred.

## Remote-Action Boundary

No SSH, scp, Tailscale, remote command, qsl-server start, qsl-server stop,
qsl-server cleanup, or remote mutation occurred.

## Public-Site / Cloudflare Boundary

No public-site or Cloudflare mutation occurred.

## Evidence / Decision / Traceability

D-1163 records the NA-0586 verification and DNS-timeout classification.
TRACEABILITY maps the classification to this evidence doc and testplan.

## Validation

Validation must include diff/scope/queue/marker proof, link check,
private-material scans, overclaim scan, PR body preflight, goal-lint, cargo
audits, locked metadata, cargo fmt, qsc adversarial shell syntax checks, and no
forbidden path mutation. Focused qsc runtime tests are skipped because no qsc
source/runtime path changed.

## Recommendation

Merge NA-0586 after required checks pass, then close out NA-0586 and restore the
selected NA-0587 network-path remediation successor.
