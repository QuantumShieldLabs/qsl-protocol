Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-04

# NA-0599 remote / Tailnet full-stack reintroduction readiness harness

## Executive Summary

NA-0599 consumed D530, D-1187, and D-1188, then separated local correctness
evidence from remote/Tailnet reachability and operator-owned setup state.

Result classification:
`REMOTE_TAILNET_REINTRODUCTION_READINESS_OPERATOR_SETUP_REQUIRED`.

Selected successor:
`NA-0600 -- QSL Remote / Tailnet Operator Setup Proof Review Harness`.

Local qsc/qsl-server/qsl-attachments correctness and fail-closed evidence is
sufficient to plan reintroduction. It remains local-only evidence. Existing
GitHub workflow metadata shows remote relay and remote handshake workflows using
secret-name backed relay endpoint inputs, but no Tailnet join setup, no remote
qsl-attachments workflow surface, and no proof that GitHub-hosted runners can
reach private Tailnet services. No remote, Tailscale, GitHub secret, workflow,
deployment, DNS, Cloudflare, or public-site mutation occurred in NA-0599.

## qwork Proof Verification

Fresh qwork proof from `2026-07-04T01:41:11Z` was copied and verified before
fetch, GitHub metadata review, repository mutation, PR creation, source-review
publication, or proof publication. Codex did not run qwork, qstart, or qresume.

Verified values included lane NA-0599, repo qsl-protocol, worktree path
`/srv/qbuild/work/NA-0599/qsl-protocol`, branch main, upstream origin/main,
clean worktree/index/untracked state, READY_COUNT 1, queue top READY NA-0599,
requested lane status READY, shared cargo target mode, and shared target ready.

Live pre-fetch `HEAD` and `origin/main` matched `2209126e3952`. Root disk usage
was below the stop threshold and `/backup/qsl` was mounted. After fetch,
origin/main still matched `2209126e3952` and descended from the expected D530
base.

Proof-root files:

- `qwork/qwork_proof_verification.md`
- `qwork/qwork_proof_verification.json`
- `startup/pre_implementation_queue_decision_proof.md`
- `startup/pre_implementation_queue_decision_proof.json`
- `startup/current_main_required_check_classification.md`
- `startup/current_main_required_check_classification.json`

## D-1187 / D-1188 Inheritance

D530 response, D-1187, and D-1188 were consumed.

- D-1187 exists once and is Accepted.
- D-1188 exists once and is Accepted.
- NA-0598 is DONE.
- NA-0599 is READY.
- D-1189 was absent before the NA-0599 patch.
- D-1187 result classification:
  `QSL_SERVER_EXACT_4MIB_RELAY_BOUNDARY_FIX_PASS`.
- D-1187 selected NA-0599.
- D-1188 restored NA-0599.
- qsl-server PR #59 merged at `544edfd213ea`.
- qsl-protocol PR #1470 merged at `e35a61ccea19`.
- qsl-protocol closeout PR #1471 merged at `2209126e3952`.

Inherited facts remain unchanged: exact 4 MiB stays legacy in-message,
qsl-attachments remains `not_used` for exact 4 MiB, qsl-server now supports the
bounded 256 data chunks plus manifest shape, greater-than-4-MiB qsl-attachments
controls passed, qsc threshold semantics did not change, qsl-attachments path
semantics did not change, and no remote/Tailnet/workflow/deployment work
occurred in NA-0598.

## Authority Model Application

NA-0599 used only:

- Tier 0 read-only analysis.
- Tier 1 proof-root analysis tooling.
- Tier 2 governance/readiness documentation.

NA-0599 did not run remote/Tailnet diagnostics, SSH, scp, Tailscale commands,
workflow dispatch, workflow reruns, GitHub secret/variable mutation, DNS,
Cloudflare, public-site, qsl-server deployment, qsl-attachments deployment, or
source/workflow/dependency mutation.

## Local Full-Stack Evidence Summary

| Evidence category | Lane / decision | Classification | Remote readiness support | Local-only limit |
|---|---|---|---|---|
| qsc/qsl-server local E2EE relay integration | NA-0587 / NA-0588 | local qsc/qsl-server relay integration and adversarial/metadata path advanced | yes | does not prove GitHub-runner reachability |
| qsc/qsl-server adversarial and metadata stress | NA-0588 | local fail-closed and metadata evidence advanced | yes | no Tailnet path exercised |
| qsl-attachments recovery/readiness | NA-0590 / D-1171 | `QSL_ATTACHMENTS_RECOVERY_VERIFICATION_READINESS_PASS_TRIPLE_RATCHET_VERIFY_REQUIRED` | yes | required triple-ratchet follow-up before full integration |
| qsc true triple-ratchet no-seed path verification | NA-0591 / D-1173 | `DYNAMIC_LOCAL_E2EE_PATH_PASS`, with seed-fallback finding recorded | yes | local-only and required hardening |
| qsc seed-fallback hardening | NA-0593 / D-1177 | `SEED_FALLBACK_HARDENING_IMPLEMENTATION_PASS_ATTACHMENT_DEFERRED` | yes | attachment integration deferred to NA-0594 |
| local qsl-attachments send/receive integration | NA-0594 / D-1179 | `LOCAL_QSL_ATTACHMENTS_SEND_RECEIVE_INTEGRATION_PASS_WITH_METADATA_LIMITS` | yes | local runtime only |
| local qsl-attachments adversarial and metadata stress | NA-0595 / D-1181 | `LOCAL_QSL_ATTACHMENTS_STRESS_RESOURCE_BOUNDARY_GAP` | yes | exact 4 MiB legacy boundary required follow-up |
| exact 4 MiB boundary diagnostics | NA-0596/NA-0597 / D-1183/D-1185 | `LOCAL_ATTACHMENT_EXACT_4MIB_QSL_SERVER_RELAY_BUG_FOUND` | yes | qsl-server relay bug identified, not fixed until NA-0598 |
| qsl-server exact 4 MiB relay-boundary fix | NA-0598 / D-1187 | `QSL_SERVER_EXACT_4MIB_RELAY_BOUNDARY_FIX_PASS` | yes | does not prove Tailnet/GitHub runner access or deployed service state |

## Prior Remote / Tailnet Failure Review

Prior remote/Tailnet evidence shows a separate reachability and setup chain:

- NA-0577: `QSL_SERVER_BOUNDED_START_FAILED`.
- NA-0578: qsl-server harness bind/port command-shape bug confirmed, corrected
  proof remained ambiguous.
- NA-0579: temporary loopback route shape passed, expected bind remained
  required.
- NA-0580: `QSL_SERVER_EXPECTED_BIND_ENDPOINT_VALUE_UNAVAILABLE`.
- NA-0581: qsl-server expected-bind remediation reached relay-testing-ready
  class.
- NA-0582: remote workflows failed after local qsl-server postcheck ready:
  `REMOTE_RELAY_RECOVERED_VERIFICATION_FAIL_SERVICE_UNREACHABLE_AFTER_LOCAL_READY`.
- NA-0583: artifact-backed failure classified service unreachable after local
  ready.
- NA-0584: qsc timeout phase was still generic.
- NA-0585: diagnostic surface authority was needed.
- NA-0586: diagnostics improved the phase to DNS timeout:
  `REMOTE_RELAY_DIAGNOSTIC_VERIFICATION_DNS_TIMEOUT`.

Key conclusion: GitHub-hosted runners cannot reach private Tailnet services
unless a Tailnet access mechanism is provided. Local correctness does not imply
GitHub-runner reachability. The NA-0598 local qsl-server fix supersedes the
exact 4 MiB local relay queue bug, but it does not supersede Tailnet access,
GitHub-runner access, service deployment, endpoint-source, or operator secret
setup requirements.

## Current Workflow Surface Review

Current workflow files were inspected read-only.

- `.github/workflows/remote-handshake-tests.yml` exists, uses
  `workflow_dispatch` and schedule, runs on GitHub-hosted `ubuntu-latest`,
  builds qsc, and uses secret names `RELAY_URL` and `RELAY_TOKEN`.
- `.github/workflows/remote-relay-tests.yml` exists, uses `workflow_dispatch`
  and schedule, runs on GitHub-hosted `ubuntu-latest`, builds qsc, and uses
  secret names `RELAY_URL` and `RELAY_TOKEN`.
- No remote qsl-attachments workflow surface was found.
- No Tailscale/Tailnet join step was found.
- Artifact upload exists for current remote relay/handshake workflows, but
  diagnostics are partial for full DNS/TCP/TLS-or-HTTP/relay/attachment phase
  classification.

Classification:

- remote-handshake workflow present: yes.
- remote-relay workflow present: yes.
- attachment remote workflow present: no.
- Tailscale / Tailnet setup present: no.
- workflow diagnostics sufficient: partial.
- artifact diagnostics sufficient: partial.
- workflow mutation needed before full remote verification: yes.
- workflow mutation allowed in NA-0599: no.

## GitHub Metadata / Secret-Name Review

GitHub Actions metadata was reviewed read-only. Only names were reviewed; no
secret values were requested or accessed.

- Repository secret names observed: `RELAY_URL`, `RELAY_TOKEN`.
- Repository variable names observed: none.
- qsl-server service endpoint source class: workflow secret name.
- qsl-attachments service endpoint source class: absent.
- required secret-name set discoverable: partial.
- secret-value access: none.

## Remote Access Model Matrix

| Model | Setup owner | Exposure | Diagnostic quality | Operator actions | Codex actions allowed | Selection |
|---|---|---|---|---|---|---|
| A. GitHub-hosted runner joins Tailnet for job duration | operator for Tailnet credentials/secrets; Codex later only if exact lane authorizes workflow/proof paths | private Tailnet | high after redacted diagnostics | Tailnet auth, GitHub secrets, service readiness, rollback proof | NA-0599 readiness docs only | selected preferred path pending operator setup |
| B. Self-hosted runner already inside Tailnet | operator | private Tailnet/runner network | high if runner isolation is proven | runner install/labels/isolation/lifecycle | proof review only unless later scoped | not selected now |
| C. Public internet endpoint for relay/attachments | operator | public internet | high for public reachability | DNS/TLS/Cloudflare/firewall/deployment | authorization only unless exact public lane exists | rejected without public endpoint authorization |
| D. Local build-server-only remote simulation | Codex/operator local only | local loopback/build-server only | low for GitHub reachability | none | local proof only if later authorized | rejected for remote/Tailnet readiness |
| E. Manual operator remote proof outside CI | operator | depends on operator environment | medium, weaker repeatability | manual redacted proof | proof review only | fallback stopgap, not primary |

## Selected Remote Reintroduction Model

Selected model: A, GitHub-hosted runner joins Tailnet for job duration, pending
operator-owned setup proof.

This model keeps relay and attachment endpoints private while making
GitHub-runner reachability testable. It requires operator-owned Tailnet
credential creation, GitHub secret/variable setup, service deployment/readiness
proof, and rollback proof before Codex can execute or implement a remote
verification lane.

## Operator / Codex Boundary

Operator-owned actions:

- Create Tailnet auth/OAuth/client mechanism.
- Set GitHub Actions secrets/variables.
- Approve Tailnet tags/ACLs.
- Start, stop, expose, or deploy remote qsl-server/qsl-attachments services.
- Install or manage a self-hosted runner if that path is chosen later.
- Change DNS, Cloudflare, firewall, or public exposure.
- Provide private endpoint values through approved secret channels.
- Run privileged local or remote commands.

Codex-owned actions in NA-0599:

- Read-only metadata review.
- Governance/readiness documentation.
- Redacted diagnostic design.
- Classification and private-material scans.
- Future tracked workflow/proof/testplan PR only if a later exact lane
  authorizes paths and boundaries.

NA-0599 performed no operator-owned actions.

## Redacted Diagnostic Plan

First bounded remote verification diagnostics must publish only classes, counts,
elapsed-time buckets, and redacted failure reasons for these phase classes:

- `runner_tailnet_join`: joined / not_configured / auth_failed / timeout /
  unknown.
- `endpoint_source`: secret_name_present / variable_name_present / missing /
  unknown.
- `dns_resolution`: success / nxdomain / timeout / unavailable /
  not_attempted / unknown.
- `tcp_connect`: success / refused / timeout / unreachable / not_attempted /
  unknown.
- `tls_handshake`: success / failed / not_applicable_http / not_attempted /
  unknown.
- `http_probe`: status_class_2xx / status_class_3xx / status_class_4xx /
  status_class_5xx / timeout / no_response / not_attempted / unknown.
- `qsl_server_route_shape`: ready / auth_fail_closed / timeout /
  not_attempted / unknown.
- `qsl_attachments_shape`: ready / auth_fail_closed / timeout /
  not_attempted / unknown.
- `qsc_remote_handshake`: pass / fail_class / not_attempted / unknown.
- `qsc_remote_relay_e2ee`: pass / fail_class / not_attempted / unknown.
- `qsc_remote_attachment`: pass / fail_class / not_attempted / unknown.

Diagnostics must not publish endpoint values, private port values, IP addresses
unless explicitly public and authorized, Tailnet hostnames, route tokens,
bearer/Authorization values, capabilities, payload/body/plaintext, ciphertext
bodies, seed/key material, raw topology, raw artifacts, or raw workflow logs.

## Security / Metadata Review

- Private endpoint risk: endpoint values must remain secret-backed and
  unpublished.
- Tailnet trust boundary: Tailnet access is operator-owned and should be
  short-lived and least-privilege for jobs.
- GitHub-hosted runner trust boundary: the runner can see job-scoped secrets;
  diagnostics must mask values and avoid raw log publication.
- Self-hosted runner trust boundary: requires a separate operator isolation and
  lifecycle review if selected later.
- Secret-name versus secret-value boundary: secret names may be reviewed; secret
  values must not be read, printed, inferred, or published.
- qsl-server route/auth boundary: route/auth failures must fail closed and
  publish only class outcomes.
- qsl-attachments capability boundary: capability values remain secret; publish
  only `auth_fail_closed`, timeout, or ready classes.
- Metadata exposure: remote runs can expose timing/status classes; publish only
  buckets/classes.
- Artifact/log redaction policy: raw logs/artifacts stay proof-root-only; repo
  evidence uses summaries.
- Rollback/disable strategy: revoke Tailnet job credentials, rotate/remove
  GitHub secrets, disable workflow/schedule if needed, and stop/unexpose
  services.

## Private-Material / Claim Boundary Review

NA-0599 publishes no endpoint value, private port value, token, Authorization
value, capability value, payload/body/plaintext, seed/key material, raw topology,
raw workflow log, or raw artifact.

NA-0599 claim boundaries:

- No public-readiness claim is made.
- No production-readiness claim is made.
- No public-internet-readiness claim is made.
- No vulnerability-free claim is made.
- No bug-free claim is made.
- No perfect-build claim is made.
- No perfect-crypto claim is made.
- No crypto-complete claim is made.
- No attachment-complete claim is made.
- No remote completion claim is made.
- No Tailnet completion claim is made.
- No side-channel-free claim is made.
- No formal-proof-complete claim is made.
- No triple-ratchet-complete claim is made.
- No external-review-complete claim is made.

## Readiness Matrix

| Field | Classification |
|---|---|
| local qsc/qsl-server evidence | pass |
| local qsl-attachments evidence | pass |
| exact 4 MiB qsl-server fix | pass |
| qsc seed fallback hardening | pass |
| workflow diagnostics | partial |
| Tailnet access | operator_required |
| GitHub secrets/variables | names_present |
| remote service deployment | operator_required |
| redaction policy | ready |
| first remote verification lane | operator_setup_required |

## Result Classification

`REMOTE_TAILNET_REINTRODUCTION_READINESS_OPERATOR_SETUP_REQUIRED`.

Rationale: existing workflow/secret names are insufficient for direct full-stack
Tailnet verification. No Tailnet join setup is present, remote qsl-attachments
workflow surface is absent, remote service deployment/readiness is
operator-owned or unknown, and workflow diagnostics are only partial.

## Selected Successor

Selected option B:

### NA-0600 — QSL Remote / Tailnet Operator Setup Proof Review Harness
Status: READY
Goals: G1, G2, G3, G4, G5

Objective:
Review operator-provided proof that the D-1189-selected remote/Tailnet access
setup was completed or intentionally declined. Verify only safe metadata:
secret-name presence, Tailnet access class, runner access class, service
readiness class, rollback/disable class, and redacted endpoint-source class.
Codex must not create or read secret values, must not run Tailscale commands,
must not mutate GitHub secrets, workflows, DNS, Cloudflare, remote hosts,
qsl-server deployment, or qsl-attachments deployment, and must not publish
private endpoint, private port, topology, token, Authorization, capability,
payload/body/plaintext, seed, or key material.

Prerequisite before Codex execution:

- Director provides one-step-at-a-time operator setup checklist from D-1189.
- Operator performs or declines the setup.
- Operator preserves proof outputs required by D-1189.
- Operator runs fresh qwork NA-0600 qsl-protocol only after setup/proof phase.

## Required-Check Boundary

Before NA-0599 mutation, current main check metadata on `2209126e3952`
classified public-safety success, advisories success, suite2-vectors success,
no failed visible check-runs, and no pending visible check-runs. Root cargo
audit, nested qsc fuzz cargo audit, locked cargo metadata, and Cargo drift checks
passed.

## Source / Workflow Mutation Boundary

NA-0599 mutates only governance/readiness paths:

- `docs/governance/evidence/NA-0599_remote_tailnet_full_stack_reintroduction_readiness_harness.md`
- `tests/NA-0599_remote_tailnet_full_stack_reintroduction_readiness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No workflow file was mutated. No workflow was dispatched or rerun.

## qsc Boundary

No qsc source, test, example, fuzz, script, Cargo, lockfile, threshold,
negotiation, key schedule, state-machine, auth, wire, or runtime behavior was
mutated in NA-0599.

## qsl-server Boundary

No qsl-server source, test, docs, dependency, lockfile, deployment, service,
Docker, systemd, cloud, auth, route, storage, or runtime behavior was mutated in
NA-0599.

## qsl-attachments Boundary

No qsl-attachments source, test, docs, dependency, lockfile, deployment,
service, storage, capability, auth, or runtime behavior was mutated in NA-0599.

## Remote / Tailscale Boundary

No remote command, SSH, scp, Tailscale command, Tailnet mutation, GitHub secret
mutation, GitHub variable mutation, runner installation, service deployment, or
public exposure occurred in NA-0599.

## Public-Site / Cloudflare Boundary

No public-site, website, DNS, Cloudflare, firewall, TLS, or public-internet
exposure mutation occurred in NA-0599.

## Evidence / Decision / Traceability

D-1189 records NA-0599 readiness classification and selected successor.
TRACEABILITY maps D-1189 to this evidence document, the NA-0599 testplan, and
the rolling operations journal.

Proof-root supporting files include:

- `inheritance/d530_d1187_d1188_inheritance_review.*`
- `local_evidence_summary/local_full_stack_evidence_summary.*`
- `remote_history/remote_tailnet_prior_failure_review.*`
- `workflow_review/current_remote_workflow_surface_review.*`
- `workflow_review/github_metadata_secret_name_review.*`
- `access_model/remote_tailnet_access_model_matrix.*`
- `access_model/selected_remote_reintroduction_model.*`
- `operator_boundary/remote_tailnet_operator_codex_boundary.*`
- `operator_boundary/operator_setup_checklist_if_needed.md`
- `diagnostic_plan/redacted_remote_full_stack_diagnostic_plan.*`
- `security_review/remote_tailnet_security_metadata_review.*`
- `private_material_scan/na0599_private_material_and_claim_boundary_review.*`
- `readiness_matrix/remote_tailnet_reintroduction_readiness_matrix.*`
- `readiness_matrix/na0599_result_classification.*`
- `successor/selected_successor.*`

## Validation

Focused runtime tests are not required for NA-0599 because this is a
readiness/authorization-only governance lane. No qsc, qsl-server,
qsl-attachments, workflow, dependency, lockfile, remote, or Tailnet execution
surface was mutated.

Validation before PR includes scope guard, queue/decision proof, marker proof,
link-check, private-material scans, overclaim scans, docs/governance-only
classification, PR body preflight, goal-lint if available, root cargo audit,
nested qsc fuzz cargo audit, locked cargo metadata, cargo fmt check, and qsc
adversarial shell syntax checks.

Recovered validation issues:

- The first validation classifier treated the new ignored evidence file and new
  testplan visibility as a scope failure. This was classified as a recoverable
  proof-tooling issue, corrected by making the new allowed files visible to git
  diff with intent-to-add and rerunning the scope classifier. Final result:
  PASS.
- The first overclaim classifier flagged wrapped negated claim-boundary wording.
  This was classified as a recoverable content-safety wording issue, corrected
  by rewriting the NA-0599 claim-boundary list so each restricted claim is
  explicitly negated. Final result: PASS.

## Recommendation

Proceed to the selected operator setup proof review successor after the
implementation PR merges and optional closeout restores NA-0600. Do not run
remote/Tailnet verification until operator-owned Tailnet access, secret-name,
runner access, service readiness, endpoint-source, and rollback/disable proof
has been reviewed under the successor lane.
