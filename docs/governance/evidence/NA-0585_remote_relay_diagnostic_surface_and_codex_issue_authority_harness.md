Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-01

# NA-0585 Remote Relay Diagnostic Surface and Codex Issue Authority Harness

## Executive Summary

NA-0585 consumed D-1159 and D-1160, verified fresh qwork proof from
`2026-07-01T16:46:24Z`, hardened the project-wide Codex issue-investigation
authority model, completed source analysis for qsc relay diagnostics, and
implemented a safe diagnostic-surface improvement.

Result classification:
`REMOTE_RELAY_DIAGNOSTIC_SURFACE_SAFE_FIX_IMPLEMENTED`.

Selected successor:
`NA-0586 -- QSL Remote Relay Diagnostic Verification and Timeout Phase Triage Harness`.

## qwork Proof Verification

- qwork proof files were copied into the proof root and parsed before fetch,
  repository mutation, GitHub action, workflow action, artifact/log retrieval,
  source-analysis publication, or proof publication.
- Required qwork values matched lane `NA-0585`, repo `qsl-protocol`, clean
  `main`, READY_COUNT 1, queue top READY `NA-0585`, shared cargo target ready,
  and proof timestamp at or after `2026-07-01T16:46:24Z`.
- Pre-fetch live `HEAD` and `origin/main` matched qwork proof.
- Root disk usage was below the stop threshold and `/backup/qsl` was mounted.
- Codex did not run `qwork`, `qstart`, or `qresume`.

## D-1159 / D-1160 Inheritance

- D-1159 exists once and is Accepted.
- D-1160 exists once and is Accepted.
- D-1161 was absent before this implementation patch.
- D-1159 result `REMOTE_RELAY_REACHABILITY_QSC_TIMEOUT_PHASE_ONLY` was
  consumed.
- D-1159 selected diagnostic surface improvement as successor.
- D-1160 restored NA-0585 as the sole READY successor.

## Authority Model Application

NA-0585 applied the lane-active issue-resolution authority model. Codex used
Tier 0 read-only project analysis, Tier 1 proof-root scanner/parser recovery,
and Tier 2 minimal project-owned diagnostic improvements. No privileged,
remote-host mutation, dependency, qsl-server source, qsl-attachments, or public
claim authority was used.

## Codex Issue-Investigation Authority Hardening

`START_HERE.md`, `AGENTS.md`, and
`docs/ops/CODEX_BOUNDED_OPERATIONAL_AUTHORITY.md` now durably record:

- failure-triggered investigation is mandatory while safe in-scope evidence
  remains available;
- project-owned source/tests/workflows/scripts/artifacts/logs may be inspected
  by default for failure classification;
- proof-root scripts, scanners, parsers, classifiers, manifests, fixtures, and
  local harnesses may be safely repaired and retried;
- lane-opted-in diagnostic fixes and minimal bug fixes may be applied inside
  strict path and safety boundaries;
- hard stops remain for secrets, private-material publication, privileged/root
  action, destructive action, dependency changes, workflow weakening,
  branch/settings mutation, public/security overclaims, and Codex execution of
  qwork/qstart/qresume.

## Current Main Required-Check Classification

Before implementation, current main at `8a376e30e92a` had public-safety
success, advisories success, suite2-vectors success, no failed visible
check-runs, no pending visible check-runs, root cargo audit success, nested qsc
fuzz cargo audit success, locked metadata success, and no Cargo drift.

## qsc Relay Transport Source Review

qsc relay push normalizes the relay base and appends the v1 push path. qsc
relay pull normalizes the same base and appends the v1 pull path with max
query. Both use `X-QSL-Route-Token`. Optional bearer auth is added only when
configured. Existing push diagnostics emitted safe status/error classes but
collapsed reqwest timeout context to a generic timeout class; pull still drops
reqwest error context. NA-0585 adds safe push diagnostic enum fields without
changing request construction or returned qsc errors.

## qsc Timeout/Error Plumbing Review

The reqwest error object is available only inside qsc transport before it is
reduced to static qsc errors. NA-0585 adds bounded enum classification at that
point. Exact DNS/TCP/TLS/HTTP phase is best-effort because the HTTP client does
not guarantee a structured phase value, so unknown cases remain
`unknown_timeout` or `diagnostic_unavailable`.

## Remote Workflow/Harness Review

The remote-handshake and remote-relay workflows build qsc, invoke
`scripts/demo/qsc_remote_handshake_smoke.sh` or
`scripts/demo/qsc_remote_relay_smoke.sh`, and upload their output directories.
Those helpers already set `QSC_RELAY_PUSH_DIAGNOSTIC=redacted` and collect qsc
markers. NA-0585 adds safe summary/count artifact fields for diagnostic class,
timeout phase class, and status class.

## qsl-server Route Expectation Review

Read-only qsl-server source/docs/tests confirm canonical v1 push and v1 pull
routes, required `X-QSL-Route-Token`, retired route-token-in-path shape, and
optional bearer auth when configured. Missing route token is a route-token
failure class; missing/wrong bearer under configured auth is a bearer failure
class. No qsl-server source was mutated.

## Selected Diagnostic Mutation Paths

- `qsl/qsl-client/qsc/src/transport/mod.rs`
- `qsl/qsl-client/qsc/tests/relay_push_diagnostics.rs`
- `qsl/qsl-client/qsc/tests/secret_material_diagnostic_boundary.rs`
- `scripts/demo/qsc_remote_handshake_smoke.sh`
- `scripts/demo/qsc_remote_relay_smoke.sh`

## Diagnostic Surface Design

The design adds only safe enum/count fields:

- qsc marker fields: `diagnostic_class`, `timeout_phase_class`;
- helper artifact fields: diagnostic count, diagnostic classes, timeout phase
  classes, and status classes;
- fallback class: `diagnostic_unavailable` when no marker exists.

Target safe classes include DNS timeout, TCP connect timeout, TLS handshake
timeout, HTTP request timeout, HTTP status received, route-token auth failed,
bearer auth failed, connection refused, connection reset, unknown timeout,
not-timeout, and diagnostic unavailable.

## Diagnostic Surface Implementation

qsc now maps redacted relay-push status/errors to bounded safe classes. Remote
helpers summarize those classes in uploaded artifacts. Existing relay URL,
header, payload, auth, response handling, returned qsc error strings, protocol
state, crypto, and fail-closed behavior are unchanged.

## Tests and Redaction Proof

Focused validation passed:

- `cargo fmt -p qsc`
- `bash -n scripts/demo/qsc_remote_relay_smoke.sh`
- `bash -n scripts/demo/qsc_remote_handshake_smoke.sh`
- `cargo test -p qsc --locked transport::relay_push_diagnostic_tests -- --test-threads=1`
- `cargo test -p qsc --locked --test relay_push_diagnostics -- --test-threads=1`
- `cargo test -p qsc --locked --test secret_material_diagnostic_boundary na0554_relay_push_diagnostic_boundary_is_value_free -- --test-threads=1`

## Private-Material Review

Added-line and captured-output scans passed. No endpoint values, private port
values, token values, Authorization values, route-token/capability values,
payloads, response bodies, private topology, process identity, command line,
authorized_keys content, public key material, private key material, or secret
values are published.

## No-Semantics-Change Review

No protocol, wire, crypto, auth, state-machine, route-token, bearer, qsl-server,
qsl-attachments, dependency, or lockfile semantics changed. The patch adds
diagnostic enum/count publication only.

## Result Classification

`REMOTE_RELAY_DIAGNOSTIC_SURFACE_SAFE_FIX_IMPLEMENTED`.

## Selected Successor

Option A:
`NA-0586 -- QSL Remote Relay Diagnostic Verification and Timeout Phase Triage Harness`.

## Required-Check Boundary

Pre-implementation main checks were green. PR checks remain authoritative before
merge; failed or pending required checks must not be treated as success.

## Source / Script Mutation Boundary

Mutations are limited to selected qsc diagnostic source/tests, selected remote
helper scripts, and governance/evidence/testplan paths authorized by NA-0585.

## Workflow Mutation Boundary

No workflow files were changed.

## Runtime / qsc Boundary

qsc runtime request/auth/protocol semantics are unchanged. qsc changes are
limited to redacted diagnostic classification and tests.

## qsl-server / qsl-attachments Boundary

qsl-server was read-only source review only. qsl-attachments was not touched.

## Remote-Action Boundary

No remote action, SSH, scp, Tailscale, remote command, qsl-server start/stop,
manual qsc send/receive, workflow dispatch, or workflow rerun occurred.

## Public-Site / Cloudflare Boundary

No public-site or Cloudflare mutation occurred.

## Claim Boundary

This evidence records diagnostic surface improvement only. It makes no
public-readiness, production-readiness, vulnerability-free, bug-free,
perfect-build, perfect-crypto, crypto-complete, or external-review-complete
claim.

## Validation

Validation artifacts include qwork proof, queue/decision proof, current-main
check classification, source-analysis reviews, diagnostic design/implementation
summary, focused qsc/helper test results, private-material scan, and
no-semantics-change review in the NA-0585 proof root.

## Recommendation

Merge NA-0585 after required checks pass, then restore NA-0586 to verify the new
safe diagnostic surface on the exact remote workflows and classify the timeout
phase from emitted safe artifacts.
