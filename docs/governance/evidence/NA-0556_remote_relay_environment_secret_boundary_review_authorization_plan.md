Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-28

# NA-0556 Remote Relay Environment Secret Boundary Review Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0556 consumed the NA-0555 redacted diagnostic evidence and authorized the
next safe boundary step. Both remote-handshake and remote-relay reached the
relay-push boundary and timed out before HTTP status/body visibility, with
route-token header presence and bearer auth presence already proven as redacted
diagnostic facts.

Result classification:
`REMOTE_RELAY_ENV_SECRET_BOUNDARY_OPERATOR_PROOF_CAPTURE_READY`.

Selected successor:
`NA-0557 -- QSL Remote Relay Environment Non-Secret Operator Proof Capture Harness`.

## qwork Proof Verification

Fresh NA-0556 qwork proof was copied from the lane workspace and parsed from
the `.kv`, JSON, and cargo-target env files. The proof timestamp was
`2026-06-28T15:00:09Z`, lane `NA-0556`, repo `qsl-protocol`, and startup
HEAD/origin/main/main was `9bd9f14c7c69`.

Codex did not run qwork, qstart, or qresume.

## D-1100 / D-1101 Inheritance

D-1100 and D-1101 were consumed and each exists once with Status Accepted.
D-1100 recorded the NA-0555 result classification
`REMOTE_RELAY_DIAGNOSTIC_EVIDENCE_REMOTE_ENV_SECRET_BOUNDARY_READY`. D-1101
closed NA-0555 and restored NA-0556 as the sole READY successor.

NA-0555 is DONE. NA-0556 is READY. No NA-0556 implementation evidence or
testplan file existed before this directive.

## Current Main Required-Check Classification

Current main was verified at `9bd9f14c7c69`, equal to origin/main.
public-safety completed success, advisories completed success, suite2-vectors
completed success, and no failed required check was classified. Branch
protection required contexts were classified using main-attached check runs and
associated PR-head proof for PR-only `CodeQL` and `goal-lint` contexts.

## NA-0555 Diagnostic Evidence Inheritance

NA-0555 executed exactly two workflow dispatches:

- `remote-handshake-tests.yml` on main.
- `remote-relay-tests.yml` on main with `scenario=happy-path` and `seed=1`.

No rerun executed. No source, script, workflow, dependency, qsl-server,
qsl-attachments, public-site, or Cloudflare mutation occurred.

## remote-handshake Environment Boundary Review

Run/job: `28325075419` / `83913585385`.

Safe diagnostic fields consumed:

- diagnostic marker count: `1`;
- status class/code: unknown;
- error class: timeout;
- response body presence/length: unknown;
- route-token header present: true;
- bearer auth present: true;
- endpoint label: `relay_push_v1`;
- qsc error: `relay_inbox_push_failed`;
- attempt count: `1`.

Classification set:

- `HANDSHAKE_ENV_SECRET_BOUNDARY_ENDPOINT_REACHABILITY_REQUIRED`
- `HANDSHAKE_ENV_SECRET_BOUNDARY_AUTH_ROUTE_PROOF_REQUIRED`
- `HANDSHAKE_ENV_SECRET_BOUNDARY_RELAY_SERVICE_HEALTH_REQUIRED`
- `HANDSHAKE_ENV_SECRET_BOUNDARY_NETWORK_TLS_TIMEOUT_REQUIRED`
- `HANDSHAKE_ENV_SECRET_BOUNDARY_QSC_RUNTIME_STILL_POSSIBLE`

The timeout occurred before HTTP status/body visibility.

## remote-relay Environment Boundary Review

Run/job: `28325168201` / `83913828473`.

Safe diagnostic fields consumed:

- diagnostic marker count: `2`;
- status class/code: unknown;
- error class: timeout;
- response body presence/length: unknown;
- route-token header present: true;
- bearer auth present: true;
- endpoint label: `relay_push_v1`;
- qsc error: `relay_inbox_push_failed`;
- attempt count: `1` for each send marker.

Classification set:

- `RELAY_ENV_SECRET_BOUNDARY_ENDPOINT_REACHABILITY_REQUIRED`
- `RELAY_ENV_SECRET_BOUNDARY_AUTH_ROUTE_PROOF_REQUIRED`
- `RELAY_ENV_SECRET_BOUNDARY_RELAY_SERVICE_HEALTH_REQUIRED`
- `RELAY_ENV_SECRET_BOUNDARY_NETWORK_TLS_TIMEOUT_REQUIRED`
- `RELAY_ENV_SECRET_BOUNDARY_QSC_RUNTIME_STILL_POSSIBLE`

The timeout occurred before HTTP status/body visibility.

## Workflow Environment and Secret-Wiring Review

Both remote workflows reference the same endpoint/auth secret names:
`RELAY_URL` and `RELAY_TOKEN`. The remote-relay workflow additionally accepts
workflow input names `scenario` and `seed`. No repository variable references
are used by these workflow files.

The demo scripts pass the bearer material into `QSC_RELAY_TOKEN` and enable
`QSC_RELAY_PUSH_DIAGNOSTIC=redacted` for qsc subprocesses. Route-token material
is generated per run by the scripts and passed into qsc contact/inbox setup; it
is not a GitHub Actions secret reference in the reviewed workflows.

NA-0555 diagnostics already prove route-token header presence and bearer auth
presence. They do not prove endpoint value correctness, bearer value
correctness, route/capability correctness, service health, or runner network
reachability.

## Secret / Variable Metadata Visibility

Secret/variable metadata classification:

- `SECRET_VARIABLE_NAMES_VISIBLE_BUT_OPERATOR_PROOF_REQUIRED`
- `SECRET_VARIABLE_METADATA_API_VISIBLE_SAFE`

Read-only GitHub metadata exposed repository secret names and update metadata
without values. The required secret names `RELAY_URL` and `RELAY_TOKEN` are
present. No secret values were accessed or visible.

Metadata is safe but insufficient. It cannot prove endpoint reachability,
DNS/TLS path, relay service health, bearer value correctness, route capability
validity, GitHub-runner network behavior, or private deployment state.

## Endpoint / Network / TLS / Service Boundary Design

NA-0556 does not run probes. It defines what a future proof lane may collect:

- relay endpoint configured: `OPERATOR_NON_SECRET_PROOF`;
- endpoint reachability class: `OPERATOR_NON_SECRET_PROOF`;
- relay service health class: `REMOTE_SERVICE_OWNER_PROOF`;
- auth/bearer configuration: `OPERATOR_NON_SECRET_PROOF`;
- route-token/capability configuration: `OPERATOR_NON_SECRET_PROOF`;
- relay deployment state: `REMOTE_SERVICE_OWNER_PROOF`;
- GitHub Actions runner boundary: `GITHUB_ACTIONS_PROOF`;
- expiration/rotation boundary: `OPERATOR_NON_SECRET_PROOF`.

Future proof must use yes/no/unknown or coarse classes. It must not publish a
private endpoint host, IP, private topology, bearer value, route-token value,
capability value, Authorization header, payload, response body, or secret
environment value.

## Operator Non-Secret Proof Requirements

The future operator package must include these proof-root-only files:

- `operator_remote_relay_non_secret_proof.json`;
- `operator_remote_relay_redaction_attestation.md`;
- `operator_remote_relay_probe_summary.md`.

Required structured fields include:

- `proof_generated_at_utc`;
- `proof_operator`;
- `proof_target`;
- `relay_endpoint_configured`;
- `relay_endpoint_label`;
- `endpoint_host_publicly_disclosable`;
- `dns_resolution_class`;
- `tcp_connect_class`;
- `tls_handshake_class`;
- `health_endpoint_class`;
- `bearer_secret_configured`;
- `bearer_secret_value_disclosed`;
- `bearer_secret_age_class`;
- `route_secret_configured`;
- `route_secret_value_disclosed`;
- `route_secret_age_class`;
- `github_secret_names_checked`;
- `github_secret_values_accessed`;
- `github_actions_runner_specific_probe`;
- `local_operator_probe_only`;
- `raw_output_contains_private_material`;
- `redaction_review`;
- `operator_asserts_no_secret_values_included`.

The fields `bearer_secret_value_disclosed`, `route_secret_value_disclosed`, and
`github_secret_values_accessed` must be `no`.

## Private-Material Policy

Immediate STOP is required if any future proof includes secret values, route
token or capability values, bearer values, Authorization headers, private
endpoint hosts, private topology, payloads, response bodies, secret environment
values, raw unreviewed output, or a failed redaction review.

Future commands, if any, must be provided by the Director one step at a time.
The operator must review output before sharing it.

## Option Review

Option A, operator non-secret proof capture, is selected.

Option B, GitHub metadata-only review, is rejected because metadata cannot
prove values or reachability. Option C may be part of the operator package if a
service owner supplies non-secret deployment/health proof. Option D, qsc
runtime review, is deferred until environment proof is clean. Option E, script
remediation, is rejected because timeout evidence does not prove script
ownership. Option F, stop/ambiguous, is rejected because exact safe proof
requirements can be defined.

## Result Classification

`REMOTE_RELAY_ENV_SECRET_BOUNDARY_OPERATOR_PROOF_CAPTURE_READY`.

## Selected Successor

`NA-0557 -- QSL Remote Relay Environment Non-Secret Operator Proof Capture Harness`.

The successor must capture and review only redacted/non-secret operator proof
fields selected by D-1102. It must not request or publish secret values, route
token or capability values, bearer values, Authorization headers, private
endpoints, private topology, payloads, response bodies, or secret environment
values.

## Required-Check Boundary

Current-main required checks were classified before mutation. This authorization
lane does not dispatch workflows or rerun jobs.

## Source / Script Mutation Boundary

No qsc source, qsc tests, qsc fuzz files, Cargo files, or demo scripts were
mutated by NA-0556.

## Workflow Mutation Boundary

No workflow file was changed. No workflow dispatch, rerun, cancel, or delete
occurred.

## Runtime / qsc / Dependency Boundary

No local qsc send/receive, local qsc E2EE, qsc runtime reproduction,
dependency update, manifest update, or lockfile update occurred.

## qsl-server / qsl-attachments Boundary

No qsl-server or qsl-attachments command, clone, build, run, local use, or
mutation occurred.

## Remote-Action Boundary

No SSH, scp, sftp, rsync, remote command, sudo/admin action, systemctl action,
backup command, or remote-service mutation occurred. Only read-only GitHub API
metadata was used.

## Public-Site / Cloudflare Boundary

No README public-progress content, docs/public content, website path, public
path, public-site content, deployment setting, or Cloudflare configuration was
changed.

## Claim Boundary

- No public-readiness claim was made.
- No production-readiness claim was made.
- No public-internet-readiness claim was made.
- No external-review-complete claim was made.
- No reproducibility-complete claim was made.
- No backup/restore-complete claim was made.
- No vulnerability-free claim was made.
- No bug-free claim was made.
- No perfect-build or perfect-crypto claim was made.

## Validation

Validation covered qwork proof, queue/decision proof, D-1100/D-1101
inheritance, current-main required-check classification, NA-0555 diagnostic
evidence review, workflow environment/secret-name review, GitHub
secret/variable metadata review, endpoint/network/TLS/service boundary design,
operator non-secret proof requirements, option review, result classification,
selected successor, private-material policy, scope guards, marker proof, link
checks, private-material scans, overclaim scans, docs/governance classifier, PR
body preflight, goal-lint, cargo audits, cargo fmt, and qsc-adversarial shell
syntax.

Focused qsc runtime tests were skipped because NA-0556 is authorization-only
and no qsc source/runtime/dependency/workflow mutation was authorized or made.

## Recommendation

Proceed to NA-0557 only after NA-0556 merges and closeout restores the exact
successor. NA-0557 should review operator-supplied non-secret proof read-only,
scan it for private material, classify the environment/secret boundary, and
select a later exact remediation, qsc runtime review, remote-service owner
review, confirmation-evidence lane, or stop successor.
