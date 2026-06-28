Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-28

# NA-0553 Remote Relay API Boundary Diagnostic Instrumentation Authorization Plan

## Executive Summary

NA-0553 is authorization-only. It reviewed the inherited `relay_inbox_push_failed` boundary from NA-0552, selected exact future diagnostic ownership, and did not implement instrumentation.

Result classification: `REMOTE_RELAY_DIAGNOSTIC_INSTRUMENTATION_IMPLEMENTATION_READY`.

Selected successor: `NA-0554 -- QSL Remote Relay API Boundary Diagnostic Instrumentation Implementation Harness`.

Selected instrumentation model: `DIAGNOSTIC_MODEL_QSC_PLUS_SCRIPT_HARNESS`.

Selected qsc diagnostic gate: `QSC_RELAY_PUSH_DIAGNOSTIC=redacted`.

## qwork Proof Verification

Fresh qwork proof was copied from the NA-0553 lane workspace and verified before any repository mutation.

- qwork proof timestamp: `2026-06-28T01:34:29Z`.
- lane: `NA-0553`.
- startup result: `OK`.
- repository: `qsl-protocol`.
- startup HEAD/origin-main/main: `0866f7da65fa`.
- startup worktree, index, and untracked state: clean.
- startup READY_COUNT: 1.
- startup READY item: `NA-0553`.
- cargo target mode: shared.
- cargo target dir: `/srv/qbuild/cache/targets/qsl-protocol/rustc-1.95.0-x86_64-unknown-linux-gnu/default`.

Codex did not run qwork, qstart, or qresume.

## D-1094 / D-1095 Inheritance

D-1094 and D-1095 were consumed.

- D-1094 exists once, is Accepted, and selected `REMOTE_RELAY_API_BOUNDARY_DIAGNOSTIC_INSTRUMENTATION_READY`.
- D-1095 exists once, is Accepted, marked NA-0552 DONE, and restored NA-0553 READY.
- NA-0552 is DONE.
- NA-0553 is READY.
- No NA-0553 implementation occurred before this directive.
- The stopped D465/D466 branch `na-0551-remote-smoke-demo-script-remediation` remains unmerged at `2b897d658416`; its diff is limited to the two remote demo scripts.
- Inherited evidence states that raw logs/artifacts were not committed to repository docs and private material was not published.

## Current Main Required-Check Classification

Current main was verified at `0866f7da65fa` after fetch. Branch protection required contexts were classified from current-main check runs, with associated PR #1378 head checks used for PR-head-only `goal-lint` and aggregate `CodeQL`.

- `public-safety`: success.
- `advisories`: success.
- required contexts: green or conclusively satisfied.
- no failed required checks.
- non-required `ci-4d-evidence` failure and aggregate `qshield-ci` failure were recorded as forward evidence only; required child contexts were green.

## Inherited Status / Body Gap

Inherited classification: `RELAY_PUSH_STATUS_BODY_NOT_LOGGED`.

Inherited secret/environment classification: `SECRET_ENV_BOUNDARY_POSSIBLE_BUT_NOT_PROVEN`.

Remote-handshake classification: `HANDSHAKE_RELAY_BOUNDARY_RELAY_API_STATUS_DIAGNOSTIC_REQUIRED`.

Remote-relay classification: `RELAY_RELAY_BOUNDARY_RELAY_API_STATUS_DIAGNOSTIC_REQUIRED`.

The current error string is insufficient because qsc maps both send errors and unclassified response statuses to `relay_inbox_push_failed`. Scripts cannot safely distinguish endpoint/path, auth, route, payload, queue, transient server, or unexpected-status causes without qsc exposing a redacted status/error class.

Needed future diagnostic fields:

- status class;
- exact HTTP status code;
- local error class;
- response body presence boolean;
- response body length only;
- route-token header presence boolean;
- bearer auth presence boolean;
- endpoint path label `relay_push`;
- retry attempt count;
- qsc error variant;
- optional non-secret run correlation id.

Forbidden diagnostic fields:

- route-token values;
- bearer token values;
- full Authorization headers;
- full private endpoint hosts;
- request payloads;
- response body content;
- route capabilities;
- private topology;
- passphrases;
- private keys;
- secret environment values.

## qsc Diagnostic Path Review

qsc instrumentation is required because qsc owns the relay push request and collapses the needed boundary before scripts can classify it.

Selected qsc mutation paths for NA-0554:

- `qsl/qsl-client/qsc/src/transport/mod.rs`
- `qsl/qsl-client/qsc/tests/relay_push_diagnostics.rs`
- `qsl/qsl-client/qsc/tests/secret_material_diagnostic_boundary.rs`

Rejected qsc CLI flag candidate:

- `qsl/qsl-client/qsc/src/cmd/mod.rs` would be needed for `--diagnose-relay-push`, but a CLI flag would require broader command plumbing through send, handshake, and receipt paths. The selected model uses an env gate instead.

No dependency or lockfile mutation is required.

## Script Diagnostic Path Review

Script-only diagnostics are not sufficient now. The scripts can only see `relay_inbox_push_failed`; they cannot infer the status/body/error class safely.

Selected future script harness paths:

- `scripts/demo/qsc_remote_handshake_smoke.sh`
- `scripts/demo/qsc_remote_relay_smoke.sh`

Future script role:

- enable `QSC_RELAY_PUSH_DIAGNOSTIC=redacted` around qsc invocations;
- capture qsc `QSC_MARK/1` diagnostic lines;
- summarize safe classes/counts only;
- preserve relay URL/token redaction;
- avoid script-owned inference of relay API status.

## Workflow Diagnostic Boundary Review

Workflow classification: `WORKFLOW_DIAGNOSTIC_MUTATION_NOT_REQUIRED`.

The existing remote workflows pass secret names to the scripts and upload proof artifacts. NA-0554 can enable diagnostics inside the scripts, so workflow YAML does not need mutation.

Workflow paths remain non-mutation paths:

- `.github/workflows/remote-handshake-tests.yml`
- `.github/workflows/remote-relay-tests.yml`

## Redaction and Output Policy

Diagnostic output is disabled by default.

Future enablement is only `QSC_RELAY_PUSH_DIAGNOSTIC=redacted` in authorized validation lanes.

Allowed fields:

- status class;
- exact HTTP status code;
- local error class;
- response body presence boolean;
- response body length;
- route-token header presence boolean;
- bearer auth presence boolean;
- endpoint path label `relay_push`;
- retry attempt count;
- qsc error variant;
- optional non-secret run correlation id.

Forbidden fields:

- route-token values;
- bearer token values;
- full Authorization headers;
- request payloads;
- full private endpoint hosts;
- response body content;
- response body hashes;
- route capabilities;
- private topology;
- passphrases;
- private keys;
- secret environment values.

Raw diagnostic logs are proof-root-only. Repository docs must contain summaries only after private-material scans.

## Instrumentation Design

Selected model: `DIAGNOSTIC_MODEL_QSC_PLUS_SCRIPT_HARNESS`.

The future implementation should add an env-gated qsc `relay_push_diagnostic` marker from `relay_inbox_push` and update the two remote smoke scripts to enable and summarize that redacted output.

Workflow mutation is not required.

Dependency or lockfile mutation is not required.

## Exact Future Path Bundle

Future mutation paths:

- `qsl/qsl-client/qsc/src/transport/mod.rs`
- `qsl/qsl-client/qsc/tests/relay_push_diagnostics.rs`
- `qsl/qsl-client/qsc/tests/secret_material_diagnostic_boundary.rs`
- `scripts/demo/qsc_remote_handshake_smoke.sh`
- `scripts/demo/qsc_remote_relay_smoke.sh`
- `docs/governance/evidence/NA-0554_remote_relay_api_boundary_diagnostic_instrumentation_implementation_harness.md`
- `tests/NA-0554_remote_relay_api_boundary_diagnostic_instrumentation_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future non-mutation paths include:

- `.github/workflows/remote-handshake-tests.yml`
- `.github/workflows/remote-relay-tests.yml`
- `Cargo.toml`
- `Cargo.lock`
- `qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `qsl-server/**`
- `qsl-attachments/**`
- `docs/public/**`
- `public/**`
- `website/**`
- `README.md`

## Future Validation Policy

Required local validation for NA-0554:

- `git diff --check`
- `cargo test -p qsc --locked --test relay_push_diagnostics -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test relay_dup_no_mutation -- --test-threads=1 --nocapture`
- `bash -n scripts/demo/qsc_remote_handshake_smoke.sh scripts/demo/qsc_remote_relay_smoke.sh`
- `cargo fmt --check`
- `cargo audit`
- `cargo audit --manifest-path qsl/qsl-client/qsc/fuzz/Cargo.toml`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`
- added-line/new-file private-material scan
- raw-log private-material scan before summary use

Branch workflow validation commands for NA-0554, if authorized by the implementation directive:

- `gh workflow run remote-handshake-tests.yml --ref <branch>`
- `gh workflow run remote-relay-tests.yml --ref <branch> -f scenario=happy-path -f seed=1`
- `gh workflow run remote-relay-tests.yml --ref <branch> -f scenario=drop-reorder -f seed=7`

## Result Classification

`REMOTE_RELAY_DIAGNOSTIC_INSTRUMENTATION_IMPLEMENTATION_READY`

## Selected Successor

### NA-0554 — QSL Remote Relay API Boundary Diagnostic Instrumentation Implementation Harness

Status: READY

Goals: G1, G2, G3, G4, G5

Objective:
Implement bounded, redacted diagnostic instrumentation for the qsc relay push boundary selected by NA-0553. Expose enough status/error classification to diagnose `relay_inbox_push_failed` in remote-handshake and remote-relay validation without printing route-token values, bearer material, Authorization headers, private endpoint hosts, request payloads, response bodies, private topology, or secret environment values.

Allowed scope:

- `qsl/qsl-client/qsc/src/transport/mod.rs`
- `qsl/qsl-client/qsc/tests/relay_push_diagnostics.rs`
- `qsl/qsl-client/qsc/tests/secret_material_diagnostic_boundary.rs`
- `scripts/demo/qsc_remote_handshake_smoke.sh`
- `scripts/demo/qsc_remote_relay_smoke.sh`
- `docs/governance/evidence/NA-0554_remote_relay_api_boundary_diagnostic_instrumentation_implementation_harness.md`
- `tests/NA-0554_remote_relay_api_boundary_diagnostic_instrumentation_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- proof-root-only validation logs and redacted summaries

Forbidden scope:

- workflow mutation;
- dependency or lockfile mutation;
- qsc source/test paths outside the exact allowed qsc paths;
- qsc fuzz mutation;
- script remediation unrelated to consuming redacted qsc diagnostics;
- local qsc send/receive outside explicitly authorized tests or workflows;
- qsl-server/qsl-attachments command, clone, build, run, or mutation;
- public-site mutation;
- Cloudflare mutation;
- raw logs or raw artifacts committed to repository docs;
- private material publication;
- no public-readiness claim;
- no production-readiness claim;
- no vulnerability-free claim;
- no bug-free claim;
- no perfect-build claim.

## Required-Check Boundary

NA-0553 classified current main required checks and did not weaken branch protection or CI policy. Required checks were green or conclusively satisfied. public-safety and advisories completed success.

## Script Remediation Boundary

No script remediation occurred in NA-0553. Future script changes are limited to consuming redacted qsc diagnostics and summarizing safe classes/counts.

## Workflow Mutation Boundary

No workflow mutation occurred. Future workflow mutation is not required by the selected model.

## Runtime / qsc / Dependency Boundary

No runtime/source/qsc/dependency mutation occurred in NA-0553. NA-0554 may mutate only the exact qsc source/test paths listed in the future path bundle. Cargo and lockfile paths remain non-mutation.

## qsl-server / qsl-attachments Boundary

No qsl-server or qsl-attachments command, clone, build, run, source inspection beyond allowed inherited metadata, or mutation occurred. NA-0554 must preserve that boundary.

## Remote-Action Boundary

No rerun, workflow dispatch, local reproduction, qsc send/receive, E2EE run, or remote command was executed. NA-0553 used read-only GitHub API and run metadata only.

## Public-Site / Cloudflare Boundary

No public-site content, README public-progress content, docs/public content, website/public paths, Cloudflare configuration, or deployment setting was mutated.

## Private-Material Boundary

No route-token value, bearer token value, full Authorization header, private endpoint, request payload, response body, route capability, private topology, passphrase, private key, secret environment value, raw log, or raw artifact was committed to repository docs.

## Claim Boundary

No public-readiness claim was made. No production-readiness claim was made. No public-internet-readiness claim was made. No external-review-complete claim was made. No reproducibility-complete claim was made. No backup/restore-complete claim was made. No vulnerability-free claim was made. No bug-free claim was made. No perfect-build or perfect-crypto claim was made.

## Validation

NA-0553 validation is governance-only. Focused qsc runtime tests are intentionally deferred because this lane did not mutate qsc runtime/source/test/dependency/workflow paths and did not authorize local qsc send/receive.

Required validation evidence is recorded in the NA-0553 proof root and testplan.

## Recommendation

Proceed to NA-0554 implementation after closeout restores the exact successor. The next lane should implement qsc env-gated redacted diagnostics first, then update scripts to consume them, with no workflow, dependency, qsl-server, qsl-attachments, public-site, or Cloudflare mutation.
