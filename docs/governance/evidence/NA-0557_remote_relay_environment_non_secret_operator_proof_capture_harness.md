Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-28

# NA-0557 Remote Relay Environment Non-Secret Operator Proof Capture Harness

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0557 recovered the D476 qwork parser stop with a proof-root file-backed
parser, reviewed the operator-prepared non-secret proof package, and classified
the remaining remote relay boundary without publishing private material.

Result classification:
`REMOTE_RELAY_OPERATOR_PROOF_TARGETED_NON_SECRET_PROBE_AUTH_READY`.

Selected successor:
`NA-0558 -- QSL Remote Relay Targeted Non-Secret Operator Probe Authorization Plan`.

The operator proof is safe to summarize but insufficient to authorize retry,
runtime remediation, script remediation, workflow dispatch, or remote-service
claim. It preserves endpoint configuration, bearer configuration, route
configuration, and age classes as unknown, and DNS/TCP/TLS/service-health proof
as not_checked.

## qwork Proof Verification

Fresh NA-0557 qwork proof was copied from the lane workspace and parsed by a
proof-root file-backed parser. The parser verified the `.kv`, JSON, and
cargo-target env files, including lane `NA-0557`, repo `qsl-protocol`, branch
`main`, clean worktree/index/untracked proof, READY_COUNT 1, queue top READY
`NA-0557`, qwork version
`175b0ea1d5b9abc07bdab66e9b92446e2a3d533018468e94a95c26f8698f86cf`, and proof
timestamp `2026-06-28T16:33:05Z`.

Pre-fetch live HEAD and origin/main both matched
`fe17e458cef1`. Root disk usage was below the stop threshold and
`/backup/qsl` was mounted. Codex did not run qwork, qstart, or qresume.

## D-1102 / D-1103 Inheritance

D-1102 exists once with Status Accepted and selected
`REMOTE_RELAY_ENV_SECRET_BOUNDARY_OPERATOR_PROOF_CAPTURE_READY`, with exact
successor `NA-0557 -- QSL Remote Relay Environment Non-Secret Operator Proof
Capture Harness`.

D-1103 exists once with Status Accepted, marks NA-0556 DONE, and restores
NA-0557 as the sole READY successor. NA-0555 and NA-0556 are DONE. No NA-0557
implementation occurred before this directive.

Inherited NA-0555 safe diagnostic evidence:

- remote-handshake run/job `28325075419` / `83913585385` classified as
  `REMOTE_HANDSHAKE_DIAGNOSTIC_NETWORK_TLS_TIMEOUT`.
- remote-relay run/job `28325168201` / `83913828473` classified as
  `REMOTE_RELAY_DIAGNOSTIC_NETWORK_TLS_TIMEOUT`.
- Safe inherited facts include timeout class, unknown HTTP status/body,
  route-token header present true, bearer auth present true, endpoint label
  `relay_push_v1`, and qsc error `relay_inbox_push_failed`.

## D476 Parser Stop Recovery

D476 stopped before fetch, repository mutation, PR creation, merge, closeout,
remote action, probe execution, qwork/qstart/qresume, or private-material scan.
The stop reason was a local command-shape quoting failure while constructing
qwork proof verification output.

D477 recovered with proof-root file-backed parser
`recovery/verify_qwork_proof.py`. The recovery parser performed no shell
command execution, no network access, and no repository mutation. It wrote
`qwork/qwork_proof_verification.json` and
`qwork/qwork_proof_verification.md`.

Proof-root queue parser corrections were also recorded before repository
mutation: the parser was updated to handle the queue dash style, suffixed NA
IDs, and first-status-only parsing. Final queue proof passed.

## Current Main Required-Check Classification

Current main was verified at `fe17e458cef1`, equal to origin/main.
public-safety completed success, advisories completed success, suite2-vectors
completed success, and no failed required check was classified.

Branch-protection required contexts were classified using main-attached
check-runs and associated PR-head proof for PR-only `goal-lint` and aggregate
`CodeQL` contexts. Required contexts were green or conclusively satisfied:
`ci-4a`, `ci-4b`, `ci-4c`, `ci-4d`, `ci-4d-dur`, `demo-cli-build`,
`demo-cli-smoke`, `formal-scka-model`, `goal-lint`,
`metadata-conformance-smoke`, `suite2-vectors`, `CodeQL`,
`macos-qsc-qshield-build`, and `public-safety`.

No `Cargo.toml`, root `Cargo.lock`, or qsc fuzz `Cargo.lock` drift was present.

## Operator Proof Package Inventory

The operator proof package was copied proof-root-only from the operator
provided directory. Expected files were present:

- `00_manifest.kv`
- `01_workflow_and_script_name_inventory.json`
- `02_repo_secret_names_metadata.json`
- `03_operator_review_summary.json`
- `04_operator_non_secret_boundary_proof.json`
- `05_operator_non_secret_boundary_scan.json`
- `06_safe_to_paste_summary.json`
- `collect_non_secret_boundary_proof.py`

Workflow inventory contained names only. The workflow secret-reference names
were `RELAY_URL` and `RELAY_TOKEN`; no values were accessed. Repository secret
metadata exposed names/update metadata only; no secret values were visible or
accessed.

## Operator Proof Private-Material Review

Operator-supplied scan pass: true. Codex scan agreed.

The copied package scan found no route-token values, capability values, bearer
values, Authorization headers, private endpoint hosts, URLs, IP addresses,
private topology, payload content, response body content, passphrases, private
keys, secret environment values, GitHub tokens, API keys, SSH private material,
backup material, or Cloudflare tokens requiring quarantine.

The operator review summary reported:

- secret values accessed: no;
- route token values accessed: no;
- bearer values accessed: no;
- private endpoint values accessed: no;
- operator review required before sharing files: yes.

## Operator Proof Field Classification

| Field | Value | Classification |
|---|---|---|
| `relay_endpoint_configured` | `unknown` | `present_unknown` |
| `relay_endpoint_label` | `private_relay` | `present_known` |
| `endpoint_host_publicly_disclosable` | `no` | `present_known` |
| `dns_resolution_class` | `not_checked` | `present_not_checked` |
| `tcp_connect_class` | `not_checked` | `present_not_checked` |
| `tls_handshake_class` | `not_checked` | `present_not_checked` |
| `health_endpoint_class` | `not_checked` | `present_not_checked` |
| `bearer_secret_configured` | `unknown` | `present_unknown` |
| `bearer_secret_value_disclosed` | `no` | `present_known` |
| `bearer_secret_age_class` | `unknown` | `present_unknown` |
| `route_secret_configured` | `unknown` | `present_unknown` |
| `route_secret_value_disclosed` | `no` | `present_known` |
| `route_secret_age_class` | `unknown` | `present_unknown` |
| `github_secret_names_checked` | `yes` | `present_known` |
| `github_secret_values_accessed` | `no` | `present_known` |
| `github_actions_runner_specific_probe` | `no` | `present_known` |
| `local_operator_probe_only` | `no` | `present_known` |
| `raw_output_contains_private_material` | `no` | `present_known` |
| `redaction_review` | `pass` | `present_known` |
| `operator_asserts_no_secret_values_included` | `yes` | `present_known` |

Unknown and not_checked values were preserved as known unknowns.

## Endpoint Boundary Classification

`ENDPOINT_CONFIGURED_UNKNOWN`.

The proof does not publish an endpoint host and does not prove whether the
endpoint value is configured correctly. The endpoint host was marked not
publicly disclosable.

## DNS Boundary Classification

`DNS_RESOLUTION_NOT_CHECKED`.

No DNS proof was provided and no DNS probe was executed by Codex.

## TCP Boundary Classification

`TCP_CONNECT_NOT_CHECKED`.

No TCP-connect proof was provided and no TCP probe was executed by Codex.

## TLS Boundary Classification

`TLS_HANDSHAKE_NOT_CHECKED`.

No TLS-handshake proof was provided and no TLS probe was executed by Codex.

## Service-Health Boundary Classification

`RELAY_SERVICE_HEALTH_NOT_CHECKED`.

No relay service-health proof was provided and no health endpoint was probed by
Codex.

## Auth / Route Configuration Boundary

`AUTH_ROUTE_CONFIGURED_UNKNOWN`.

Bearer and route configuration are unknown. The proof does confirm that bearer
values, route-token values, capability values, and Authorization headers were
not disclosed.

## Runner-Specific Proof Boundary

`GITHUB_RUNNER_PROOF_NOT_PERFORMED`.

The operator proof states that no GitHub Actions runner-specific probe was
performed.

## Overall Environment Boundary Classification

`REMOTE_RELAY_OPERATOR_PROOF_INSUFFICIENT_TARGETED_PROBE_REQUIRED`.

The package is safe and non-contradictory, but it does not classify
DNS/TCP/TLS/service health and does not prove endpoint/auth/route configuration
correctness. Retry, workflow confirmation, qsc runtime review, and script
remediation are premature.

## Result Classification

`REMOTE_RELAY_OPERATOR_PROOF_TARGETED_NON_SECRET_PROBE_AUTH_READY`.

## Selected Successor

`NA-0558 -- QSL Remote Relay Targeted Non-Secret Operator Probe Authorization
Plan`.

The successor must authorize one-step-at-a-time operator-run non-secret probes.
Codex may only review operator-provided proof. The lane must not request,
reveal, or publish route tokens, bearer values, Authorization headers, private
endpoints, private topology, payloads, response bodies, or secret environment
values.

## Required-Check Boundary

Current-main required checks were classified before mutation. NA-0557 executes
no workflow dispatch and no rerun.

## Source / Script Mutation Boundary

No qsc source, qsc tests, qsc fuzz files, Cargo files, or demo scripts were
mutated by NA-0557.

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

No SSH, scp, sftp, rsync, Tailscale, ping, nc, private endpoint curl, private
endpoint TLS probe, remote command, sudo/admin action, systemctl action, backup
command, or remote-service mutation occurred.

## Public-Site / Cloudflare Boundary

No README public-progress content, docs/public content, website path, public
path, public-site content, deployment setting, or Cloudflare configuration was
changed.

## Raw-Log / Artifact Boundary

No raw logs or raw artifacts were copied into repository docs. Operator proof
raw files remain proof-root-only; repository evidence includes only bounded
classification summaries.

## Claim Boundary

- No public-readiness claim was made.
- No production-readiness claim was made.
- No public-internet-readiness claim was made.
- No external-review-complete claim was made.
- No backup/restore-complete claim was made.
- No vulnerability-free claim was made.
- No bug-free claim was made.
- No perfect-build or perfect-crypto claim was made.

## Validation

Validation covers qwork proof, D476 recovery proof, queue/decision proof,
D-1102/D-1103 inheritance, current-main required-check classification, operator
proof private-material scan, field classification, boundary classification,
successor selection, implementation scope guard, marker proof, link-check,
private-material scan, prohibited-material scan, overclaim scan,
docs/governance-only classifier, PR body preflight, goal-lint when available,
cargo audits, cargo fmt, and qsc-adversarial shell syntax.

Focused qsc runtime tests are skipped because NA-0557 is evidence/governance
only, no qsc source/runtime/dependency/workflow mutation occurred, and local
qsc execution is not authorized.

## Recommendation

Proceed to implementation PR review and merge only after validation and required
checks are green. After NA-0557 merges and post-merge checks are healthy, a
separate optional closeout may restore NA-0558 using the selected targeted
non-secret operator probe authorization block. Do not start NA-0558 work in
NA-0557.
