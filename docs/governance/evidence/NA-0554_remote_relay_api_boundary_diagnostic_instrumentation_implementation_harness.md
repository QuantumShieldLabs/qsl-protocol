Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-28

# NA-0554 Remote Relay API Boundary Diagnostic Instrumentation Implementation Harness

## Executive Summary

NA-0554 implements bounded qsc relay-push diagnostic instrumentation and enables it in the authorized remote smoke script harness with `QSC_RELAY_PUSH_DIAGNOSTIC=redacted`. D473 recovered branch validation reachability by adding only deterministic pre-relay setup repairs inside the two authorized demo scripts. Branch validation reached `relay_inbox_push_failed` and emitted safe redacted diagnostics for both remote-handshake and remote-relay.

## qwork Proof Verification

The NA-0554 qwork proof files were reused and verified from the lane worktree. The proof timestamp was `2026-06-28T02:48:33Z`, lane `NA-0554`, repo `qsl-protocol`, and startup main/origin/main was `8cd7468624bc`. Codex did not run qwork, qstart, or qresume.

## D-1096 / D-1097 / D470 / D471 / D472 / D473 Inheritance

D-1096 selected `DIAGNOSTIC_MODEL_QSC_PLUS_SCRIPT_HARNESS`, diagnostic gate `QSC_RELAY_PUSH_DIAGNOSTIC=redacted`, and the exact NA-0554 path bundle. D-1097 restored NA-0554 READY. D470 suite2-vectors recovery was consumed. D471 stopped before mutation due a testplan path mismatch. D472 consumed that mismatch, implemented local qsc diagnostics, and stopped before PR because branch validation failed before relay-push. D473 consumed D472 and recovered branch reachability without expanding beyond the harness scope.

## Current Main Required-Check Classification

Current main was verified at `8cd7468624bc`. public-safety and advisories completed success. suite2-vectors was green after the D470 recovery. No failed required check was classified.

## Inherited Relay Push Boundary

The inherited target failure remains `relay_inbox_push_failed`. The inherited visibility gap was that qsc did not expose enough safe status/body/error-class information to distinguish timeout, transport, auth, route, payload, endpoint, malformed response, or unexpected status causes without risking private material exposure.

## qsc Diagnostic Instrumentation

`qsl/qsl-client/qsc/src/transport/mod.rs` now emits a redacted diagnostic marker only when `QSC_RELAY_PUSH_DIAGNOSTIC=redacted`. Diagnostics are disabled by default. The output is bounded to a qsc marker with status class/code when available, error class, response body presence/length, route-header presence, bearer-auth presence, attempt count, and safe qsc error variant.

## Script Harness Enablement

Both authorized demo scripts export `QSC_RELAY_PUSH_DIAGNOSTIC=redacted` only inside qsc subprocess execution. D473 added deterministic pre-relay setup to use supported passphrase-file vault initialization, explicit unlock, isolated per-run qsc state, and synthetic contact/route setup needed to reach relay push. The scripts do not print route-token values, bearer values, Authorization headers, private endpoint hosts, payloads, or response bodies.

## Redaction and Output Policy

Allowed diagnostic fields are status class/code, error class, body presence/length, route-header presence, auth presence, qsc error variant, and attempt count. Forbidden fields remain route-token values, bearer values, Authorization headers, request payloads, response body content, full endpoint hosts, route capabilities, private topology, passphrases, private keys, and secret environment values.

## Local qsc Diagnostic Tests

Local validation passed:

- `cargo test -p qsc --locked --test relay_push_diagnostics -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked relay_push_diagnostic_tests -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test relay_dup_no_mutation -- --test-threads=1 --nocapture`
- `cargo fmt --check`

## Secret-Material Diagnostic Boundary Tests

`cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture` passed. The test proves diagnostic output omits token-like fixture strings, bearer text, Authorization text, private endpoint text, payload text, and response body text.

## Script Static Validation

`bash -n` passed for both demo scripts. `sh -n` was classified bash-only because both scripts use existing Bash array syntax. Script safety scanning found no blocking eval, assertion bypass, secret echoing, passphrase echoing, route-token echoing, bearer echoing, broad destructive removal, qwork/qstart/qresume, qsl-backup, qsl-server/qsl-attachments, or workflow mutation command pattern.

## Branch GitHub Actions Diagnostic Validation

D472 branch validation runs `28309984363` and `28309984861` failed before relay push. D473 recovery runs reached relay push:

- remote-handshake run `28310659387`, job `83874845735`: `REMOTE_HANDSHAKE_RECOVERY_DIAGNOSTIC_OBSERVED_SAFE`
- remote-relay run `28310659797`, job `83874846680`: `REMOTE_RELAY_RECOVERY_DIAGNOSTIC_OBSERVED_SAFE`

Both emitted redacted relay-push diagnostics with `error_class=timeout`, `status_class=unknown`, `response_body_present=unknown`, `route_header_present=true`, `auth_present=true`, and `qsc_error=relay_inbox_push_failed`.

## Log Capture and Redaction

Raw workflow logs, job metadata, and downloaded artifacts remain proof-root-only under the D472 and D473 proof roots. Repository evidence includes only bounded summaries and run/job identifiers.

## Private-Material Review

The D473 private-material scan classification is `RECOVERY_BRANCH_PRIVATE_MATERIAL_SCAN_PASS`. Relay-push diagnostics contained no route-token value, bearer value, Authorization header, endpoint host, payload content, response body content, passphrase, private key, or secret environment value. Masked GitHub checkout metadata and commit/action/fingerprint identifiers were retained proof-root-only.

## Result Classification

`REMOTE_RELAY_DIAGNOSTIC_INSTRUMENTATION_IMPLEMENTATION_PASS`.

## Selected Successor

Selected successor: `NA-0555 -- QSL Remote Relay API Boundary Diagnostic Evidence Capture Harness`.

## Required-Check Boundary

Current-main required checks were classified before mutation. Branch validation succeeded for diagnostic visibility even though the remote workflows still failed at the inherited target boundary. Merge remains gated on PR checks, public-safety, advisories, and no failed required checks.

## Script Remediation Boundary

D473 did not treat `relay_inbox_push_failed` as fixed and did not broaden success criteria. It restored only pre-relay setup reachability needed to observe qsc diagnostics.

## Workflow Mutation Boundary

No workflow files were changed. Only the exact authorized branch workflow dispatches were run. No historical rerun, cancel, delete, or main workflow dispatch occurred.

## Runtime / qsc / Dependency Boundary

qsc request semantics, payload construction, route-token handling, bearer handling, success/failure behavior, dependencies, manifests, and lockfiles were not changed.

## qsl-server / qsl-attachments Boundary

No qsl-server or qsl-attachments command, clone, build, run, or mutation occurred.

## Public-Site / Cloudflare Boundary

No public-site, docs/public, website, public path, deployment, or Cloudflare mutation occurred.

## Claim Boundary

No public-readiness claim is introduced. No production-readiness claim is introduced. No public-internet-readiness claim is introduced. No external-review-complete claim is introduced. No reproducibility-complete claim is introduced. No backup/restore-complete claim is introduced. No vulnerability-free claim is introduced. No bug-free claim is introduced. No perfect-build claim is introduced. No perfect-crypto claim is introduced.

## Validation

Validation covered qwork proof, queue proof, current-main check classification, D471/D472/D473 recovery proof, qsc tests, secret-material tests, script syntax, script safety scan, branch diagnostic validation, private-material scan, scope guard, and corrected testplan path guard.

## Recommendation

Proceed to NA-0555 only after NA-0554 merges and closeout restores the successor. NA-0555 should use the redacted diagnostics to capture bounded evidence for the remote relay API boundary without mutating qsc source, demo scripts, workflows, dependencies, services, public-site content, or Cloudflare.
