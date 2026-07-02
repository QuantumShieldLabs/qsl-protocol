Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-01

# NA-0587 Local qsc / qsl-server Relay Integration Pivot Harness

## Executive Summary

NA-0587 consumed D-1163 and D-1164, recorded the operator-selected pivot away
from immediate Tailnet/GitHub-runner remediation, and used the build server as
the controlled local integration environment.

Required working classification:
`LOCAL_CLIENT_RELAY_INTEGRATION_PIVOT_SELECTED`.

Result classification:
`LOCAL_CLIENT_RELAY_E2EE_INTEGRATION_PASS`.

Selected successor:
`NA-0588 — QSL Local qsc / qsl-server Adversarial and Metadata Stress Harness`.

## qwork Proof Verification

- Fresh qwork proof files were copied into the proof root before fetch,
  repository mutation, qsl-server acquisition, qsl-server start, qsc command
  execution, or governance patching.
- qwork proof verified lane `NA-0587`, repo `qsl-protocol`, path
  `/srv/qbuild/work/NA-0587/qsl-protocol`, clean worktree/index/untracked
  state, READY_COUNT 1, queue top READY `NA-0587`, and shared cargo target
  readiness.
- Pre-fetch live `HEAD` and `origin/main` matched qwork proof at
  `48efc7278b87`.
- Root disk and `/backup/qsl` were below the 95 percent stop threshold;
  `/backup/qsl` was mounted.
- Codex did not run `qwork`, `qstart`, or `qresume`.

## D515 / D-1163 / D-1164 Inheritance

- D515 response was consumed.
- D-1163 exists once and is Accepted.
- D-1163 result classification:
  `REMOTE_RELAY_DIAGNOSTIC_VERIFICATION_DNS_TIMEOUT`.
- D-1164 exists once and is Accepted.
- D-1164 restored NA-0587 as the sole READY item.
- NA-0586 is DONE; NA-0587 is READY.
- D-1165 was absent before this implementation patch.

## Strategy Pivot

- D-1163/D-1164 showed the remote workflow failure reached the diagnostic
  surface and classified as DNS timeout.
- The DNS timeout is consistent with GitHub-hosted runners not being inside the
  operator's private Tailnet.
- The operator decided not to prioritize Tailnet/GitHub runner remediation yet.
- The selected immediate engineering path is local build-server integration:
  qsc client behavior plus qsl-server relay on loopback.
- Remote/Tailnet remediation is deferred, not rejected.
- Local integration success does not prove remote readiness, public readiness,
  production readiness, or Internet readiness.

## Authority Model Application

NA-0587 used the D-1161-expanded issue-investigation and safe-fix authority.
Source analysis, proof-root harness repair, local loopback qsl-server start,
local qsc command execution, and proof-root-only scans were in scope. No
project-owned source fix was selected.

## qsl-server Source Review

- qsl-server source was cloned into the NA-0587 workspace and reviewed at
  `6bf61d439fa2`.
- Worktree was clean before and after validation.
- Binary target: `qsl-server`.
- Route surface: canonical push and pull route shapes only; legacy path-token
  routes are retired.
- Route-token carriage: header-based.
- Optional bearer behavior: enabled by `RELAY_TOKEN`; missing or wrong bearer
  rejects before queue mutation.
- Default bind is loopback; the local harness also bound loopback only.
- qsl-server logs metadata classes only, including redacted route identifier,
  message id, and byte count.

## qsl-server Build / Audit / Test

qsl-server validation passed:

- `cargo metadata --locked --format-version=1`
- `cargo audit --deny warnings`
- `cargo fmt --check`
- `cargo test --locked`
- `cargo build --locked`

## qsc Source and Command Discovery

- qsc source review confirmed canonical relay transport push/pull helpers.
- qsc sends route tokens with the route-token header and optional bearer from
  configured test auth sources.
- Redacted relay-push diagnostic mode emits safe classes without publishing
  endpoint, token, Authorization, body, payload, or raw response values.
- qsc help output confirmed relay send, relay receive, relay inbox, and
  relay-backed handshake command surfaces.
- Focused qsc relay tests passed for auth/header carriage, redacted push
  diagnostics, and canonical transport contract.

## Local qsl-server Route-Shape Harness

qsl-server started locally on loopback with proof-root-only non-secret test
credentials and proof-root-only logs.

Classifications:

- `LOCAL_QSL_SERVER_ROUTE_SHAPE_PASS`
- `LOCAL_QSL_SERVER_AUTH_FAIL_CLOSED_PASS`

Checks:

- empty pull returned the expected no-content class;
- missing route token rejected;
- missing bearer rejected;
- wrong bearer rejected;
- canonical push accepted;
- wrong-route pull was isolated and did not cross-deliver;
- canonical pull returned the queued item;
- pull after drain returned the expected no-content class;
- qsl-server process cleanup completed.

## Local qsc Client / Relay Integration

qsc local relay integration ran against the local qsl-server process using
proof-root-only non-secret test credentials and proof-root-only local qsc state.

Classifications:

- `LOCAL_QSC_RELAY_PUSH_PULL_PASS`
- `LOCAL_QSC_RELAY_AUTH_FAIL_CLOSED_PASS`

qsc send reached qsl-server canonical push and qsc receive pulled from
qsl-server canonical pull. Raw command output, local qsc state, route token,
bearer value, loopback port, and payload bytes remain proof-root-only.

## Local E2EE Send / Receive

Classification:
`LOCAL_QSC_E2EE_OVER_RELAY_PASS`.

The local qsc harness packed a Suite-2 envelope, pushed it through local
qsl-server, pulled it back through qsc receive, unpacked it, and validated the
received payload against the proof-root-only fixture. The plaintext fixture is
not published.

## Selected Negative Tests

Selected negatives passed:

- wrong bearer send rejected without send-commit or accepted-by-relay marker;
- empty pull after drain produced the no-item class;
- wrong-route pull produced the no-item class without cross-route delivery.

Malformed relay response and broader replay/stale/concurrency stress are
deferred to NA-0588.

## Issue Investigation and Safe Fix

Failure investigation was skipped because the route-shape harness, qsc
push/pull, E2EE send/receive, and selected negatives passed.

Safe fix classification:
`SAFE_FIX_SKIPPED_NOT_NEEDED`.

Selected source/test fix paths: none.

## Recovered Failures

- qwork queue proof initially used a zero-match-unsafe D-1165 count. This was
  a valid zero-match proof outcome; the parser was rerun with zero-match-safe
  counting and completed with D-1165 absent.
- qwork queue proof initially paired a historical queue header with the sole
  READY status because the proof-root parser crossed sections. This was a
  proof-root parser bug; the parser was replaced with section-bounded parsing.
- qsl-server route search included a non-existent examples directory. This was
  a command-shape issue; the search was rerun over existing paths.
- qsc discovery included a non-existent examples directory and an incorrect
  binary-entry path. This was a command-shape issue; discovery was rerun over
  existing paths and the actual qsc entrypoint.

## Private-Material Review

Aggregate private-material review passed. Test route tokens, bearer values,
payload fixture, passphrase fixture, loopback port, raw qsl-server logs, raw qsc
outputs, and response bodies remain proof-root-only.

Repository evidence publishes only classes, short SHAs, and pass/fail results.
No endpoint value beyond loopback class, private port, token, bearer,
Authorization value, payload, response body, plaintext message, process
identity, topology, key material, secret environment value, or raw private log
is published.

## Metadata Review

Local metadata review passed at class level.

- qsl-server observed metadata classes: redacted route identifier, relay
  message id, byte count, status class, empty/non-empty queue behavior.
- qsc observed metadata classes: local peer label, redacted/hashed mailbox
  class, envelope length bucket/count classes, and redacted diagnostic classes.
- This is local loopback evidence only and does not prove remote or public
  metadata readiness.

## Result Classification

`LOCAL_CLIENT_RELAY_E2EE_INTEGRATION_PASS`.

## Selected Successor

Option B selected:

### NA-0588 — QSL Local qsc / qsl-server Adversarial and Metadata Stress Harness
Status: READY
Goals: G1, G2, G3, G4, G5

Objective:
Stress the working local qsc/qsl-server setup with repeated runs, wrong peer,
wrong token, stale state, replay-like duplicates, malformed relay responses,
server restart, client restart, concurrency, timeout, empty queue, and metadata
visibility checks. qsl-attachments remains deferred unless selected later.

## Boundaries

- No remote action occurred.
- No Tailscale action occurred.
- No GitHub workflow dispatch/rerun occurred.
- No qsl-attachments command or mutation occurred.
- No qsl-server source mutation occurred.
- No qsl-protocol qsc source mutation occurred.
- No dependency or lockfile mutation occurred.
- No workflow mutation occurred.
- No public-readiness claim is made.
- No production-readiness claim is made.
- No vulnerability-free claim is made.
- No bug-free claim is made.
- No perfect-build or perfect-crypto claim is made.

## Required Markers

- NA0587_D1163_DNS_TIMEOUT_CONSUMED_OK
- NA0587_D1164_CLOSEOUT_CONSUMED_OK
- NA0587_LOCAL_INTEGRATION_PIVOT_RECORDED_OK
- NA0587_FRESH_QWORK_PROOF_OK
- NA0587_QSL_SERVER_SOURCE_REVIEW_OK
- NA0587_QSL_SERVER_AUDIT_BUILD_TEST_OK_OR_CLASSIFIED
- NA0587_QSL_SERVER_LOCAL_ROUTE_SHAPE_CLASSIFIED_OK
- NA0587_QSC_COMMAND_DISCOVERY_OK
- NA0587_QSC_LOCAL_RELAY_INTEGRATION_CLASSIFIED_OK
- NA0587_QSC_E2EE_EXECUTED_OR_DEFERRED_OK
- NA0587_SELECTED_NEGATIVES_EXECUTED_OR_DEFERRED_OK
- NA0587_METADATA_REVIEW_OK
- NA0587_ISSUE_INVESTIGATION_EXECUTED_OR_SKIPPED_OK
- NA0587_SAFE_FIX_APPLIED_OR_SKIPPED_OK
- NA0587_PRIVATE_MATERIAL_SCAN_OK
- NA0587_NO_REMOTE_ACTION_OK
- NA0587_NO_TAILSCALE_OK
- NA0587_NO_WORKFLOW_DISPATCH_OK
- NA0587_NO_QSL_ATTACHMENTS_OK
- NA0587_NO_PUBLIC_READINESS_CLAIM_OK
- NA0587_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0587_RESULT_CLASSIFICATION_SELECTED_OK
- NA0587_SUCCESSOR_SELECTED_OK
- NA0587_ONE_READY_INVARIANT_OK
