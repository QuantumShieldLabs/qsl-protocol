Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0566 Remote Relay Listener Deployment Proof Authorization Plan

## Executive Summary

NA-0566 is authorization-only. It consumes D-1120/D-1121 and the NA-0565
missing-candidate proof, then selects the next safe proof lane for remote relay
deployment/listener state.

Selected result classification:
`REMOTE_RELAY_LISTENER_DEPLOYMENT_OPERATOR_PROOF_READY`.

Selected successor:
`NA-0567 -- QSL Remote Relay Listener Deployment Non-Secret Operator Proof Capture Harness`.

The selected model is operator/service-owner proof capture only. Codex does not
run probes, SSH, Tailscale, service commands, workflow dispatches, reruns, qsc
send/receive, or remote commands in NA-0566. NA-0566 does not mutate source,
scripts, workflows, dependencies, lockfiles, qsl-server, qsl-attachments,
accounts, service state, public-site content, or Cloudflare configuration.

## qwork Proof Verification

Fresh NA-0566 qwork proof files were copied from the lane workspace into the
proof root and parsed from files before fetch or repository mutation.

- startup_result: `OK`
- lane: `NA-0566`
- repo: `qsl-protocol`
- branch: `main`
- upstream: `origin/main`
- qwork proof timestamp: `2026-06-29T03:58:53Z`
- HEAD before fetch: `0de8707462b4`
- origin/main before fetch: `0de8707462b4`
- worktree/index/untracked: clean
- READY_COUNT: `1`
- READY: `NA-0566`
- shared cargo target metadata: verified
- `/backup/qsl`: mounted
- root disk use: below stop threshold

Codex did not run qwork, qstart, or qresume.

## D-1120 / D-1121 Inheritance

D-1120 exists once and is Accepted. It accepted the NA-0565 candidate
confirmation proof and selected this NA-0566 authorization-only listener
deployment proof lane.

D-1121 exists once and is Accepted. It marked NA-0565 DONE and restored NA-0566
as the exactly one READY successor.

Inherited NA-0565 result classification:
`LOOPBACK_CANDIDATE_MISSING_SERVICE_DEPLOYMENT_PROOF_REQUIRED`.

No NA-0566 implementation evidence or testplan existed before this directive,
and inherited publishable evidence records no private-material publication.

## Current Main Required-Check Classification

Current main was verified at `0de8707462b4`. The required-check classifier used
file-backed GitHub metadata for current main and merged PR #1404 head checks.

- public-safety: completed success
- advisories: completed success
- suite2-vectors: completed success
- failed required checks: none
- pending required checks: none
- branch-protection required contexts: green or conclusively satisfied
- CodeQL: conclusively satisfied by successful analysis jobs
- goal-lint: conclusively satisfied by merged PR #1404 head check-run success

Recovered proof issue: a read-only workflow-run API query first used an invalid
query shape and returned 404. It was corrected once by putting the query in the
GET URL, and the corrected query succeeded.

Recovered classifier issue: the first file-backed required-check classifier
treated `goal-lint` as missing because it was not attached to the merge commit.
Merged PR #1404 head checks show `goal-lint` completed success, so the context
is classified conclusively satisfied.

## NA-0565 Evidence Review

NA-0565 published only coarse classes:

- SSH candidate readiness: `ready`
- candidate listener count class: `none`
- candidate listener class: `absent`
- candidate TCP connect class: `not_checked`
- candidate v1 push HEAD class: `not_checked`
- candidate v1 pull HEAD class: `not_checked`
- candidate relay shape class: `unknown`
- candidate confirmation class: `unknown`
- private-material scans: passed

NA-0565 did not publish endpoint values, private port values, route-token or
capability values, bearer values, Authorization headers, private topology,
process identity, payloads, response bodies, authorized_keys material, public
key material, or private material.

## Listener Deployment Boundary Analysis

Boundary classifications:

- deployment state boundary: `DEPLOYMENT_STATE_PROOF_REQUIRED`
- listener binding boundary: `LISTENER_BINDING_PROOF_REQUIRED`
- service installation boundary: `SERVICE_INSTALLATION_PROOF_REQUIRED`
- operator/service-owner action boundary: `OPERATOR_SERVICE_OWNER_PROOF_REQUIRED`
- Codex action boundary: `CODEX_ACTION_NOT_AUTHORIZED`
- qsc runtime boundary: `QSC_RUNTIME_NOT_PRIMARY_SUSPECT`
- GitHub runner boundary: `GITHUB_RUNNER_DEFERRED_UNTIL_DEPLOYMENT_PROOF`

Direct operator action is premature because NA-0565 confirmed no usable
candidate listener. The next safe step is non-secret proof of deployment and
listener state.

## NA-0567 Proof Model Design

Selected model: Model A, operator/service-owner proof capture only.

Codex-executed remote deployment proof is not selected. Exact commands that
would prove installed/running/bound state risk exposing service identity,
process identity, private ports, configuration, secret paths, or private
topology. Hybrid proof is also rejected for this successor because it would add
remote proof surface without removing the need for operator/service-owner proof.

Future NA-0567 should review operator-provided or service-owner-provided
safe-to-paste summaries and approved proof files only.

## Exact NA-0567 Proof Requirements

Minimum future proof fields:

- `proof_origin`: operator, service-owner, codex-read-only, or unknown
- `deployment_state_class`: running, listening, installed_not_running,
  not_installed, or unknown
- `listener_state_class`: expected_listener_present, other_listener_present,
  no_listener, or unknown
- `bind_scope_class`: loopback, any, private_not_reported, unknown, or not_checked
- `service_health_class`: healthy, unhealthy, not_checked, or unknown
- `action_owner`: operator, service-owner, codex-not-authorized, or unknown
- `operator_action_required`: yes, no, or unknown
- `codex_mutation_authorized`: no

Required operator/service-owner proof package:

- `00_manifest.kv`
- `01_listener_deployment_summary.json`
- `02_service_health_summary.json`
- `03_alignment_candidate_summary.json`
- `04_private_material_scan.json`
- `05_safe_to_paste_summary.json`

Every future proof file must be reviewed by the operator before sharing. NA-0566
selects no exact Codex SSH command strings.

## NA-0567 Private-Material Policy

Future publishable proof must keep the following disclosure flags at `no`:

- endpoint value disclosed
- private port value disclosed
- process identity disclosed
- service name disclosed
- token value disclosed
- bearer value disclosed
- Authorization header disclosed
- response body disclosed
- private topology disclosed
- secret environment disclosed

If raw output contains private material or redaction review fails, NA-0567 must
stop before publication.

## NA-0567 Decision Tree

NA-0567 result classifications:

- `LISTENER_DEPLOYMENT_RUNNING_ALIGNMENT_ACTION_READY`
- `LISTENER_DEPLOYMENT_INSTALLED_NOT_RUNNING_ACTION_READY`
- `LISTENER_DEPLOYMENT_NOT_INSTALLED_OR_ABSENT_ACTION_READY`
- `LISTENER_DEPLOYMENT_HEALTH_UNHEALTHY_ACTION_READY`
- `LISTENER_DEPLOYMENT_UNKNOWN_SERVICE_OWNER_PROOF_REQUIRED`
- `LISTENER_DEPLOYMENT_PRIVATE_MATERIAL_STOP`
- `LISTENER_DEPLOYMENT_AMBIGUOUS_STOP`

Successor mapping:

- Running/listener confirmed: select alignment action authorization or runner
  proof depending on alignment.
- Installed but not running: select operator-owned service start/restart
  authorization.
- Not installed/absent: select deployment/install authorization or
  service-owner action lane.
- Unhealthy: select service health remediation authorization.
- Unknown: select narrower service-owner proof or stop.
- Private material or ambiguity: stop.

## Option Review

Option A, operator/service-owner proof authorization, is selected.

Option B, Codex-executed non-mutating deployment proof, is rejected because a
safe command set cannot be selected without disclosure risk.

Option C, hybrid proof plus operator checklist, is rejected because it adds
remote proof surface without avoiding operator/service-owner proof.

Option D, action authorization now, is rejected because D-1120 did not confirm a
usable candidate listener.

Option E, qsc runtime review, is deferred because qsc runtime is not the primary
suspect before deployment/listener proof.

Option F, GitHub runner proof, is deferred until deployment/listener proof is
resolved.

## Result Classification

`REMOTE_RELAY_LISTENER_DEPLOYMENT_OPERATOR_PROOF_READY`.

## Selected Successor

`NA-0567 -- QSL Remote Relay Listener Deployment Non-Secret Operator Proof Capture Harness`.

NA-0567 must review only operator/service-owner non-secret proof about remote
relay deployment and listener state after NA-0565 found no usable candidate.
Codex must perform read-only proof review only. No NA-0567 implementation is
performed by NA-0566.

## Required-Check Boundary

NA-0566 classifies current-main required checks before mutation and relies on
PR/post-merge checks for merge eligibility. No workflow dispatch or rerun is
performed.

## Source / Script Mutation Boundary

NA-0566 changes no source files and no repository scripts.

## Workflow Mutation Boundary

NA-0566 changes no workflow files and performs no workflow dispatch, rerun,
cancel, or deletion.

## Runtime / qsc / Dependency Boundary

NA-0566 performs no qsc send/receive, qsc E2EE, local qsc runtime reproduction,
dependency update, lockfile update, cargo dependency mutation, or qsc source/test
mutation.

## qsl-server / qsl-attachments Boundary

NA-0566 performs no qsl-server or qsl-attachments command, clone, build, run, or
mutation.

## Remote-Action Boundary

NA-0566 performs no probes, SSH, Tailscale, remote commands, sudo/admin actions,
service commands, account mutation, shell mutation, authorized_keys mutation,
firewall mutation, or service-state mutation.

## Public-Site / Cloudflare Boundary

NA-0566 changes no README public-progress content, docs/public content, public
paths, website paths, Cloudflare configuration, deployment settings, or
public-site content.

## Claim Boundary

NA-0566 makes:

- no public-readiness claim
- no production-readiness claim
- no public-internet-readiness claim
- no external-review-complete claim
- no backup/restore-complete claim
- no vulnerability-free claim
- no bug-free claim
- no perfect-build claim
- no perfect-crypto claim

## Validation

Validation must confirm:

- exact five-path implementation scope
- D-1122 exists once
- D-1123 absent
- duplicate decision entry count zero
- marker proof
- changed Markdown link-check
- added-line/new-file private-material scan
- prohibited-material scan
- added-line/new-file overclaim scan
- docs/governance-only classifier
- PR body preflight
- goal-lint if available and safe
- root cargo audit
- nested qsc fuzz lock cargo audit
- cargo fmt check
- qsc adversarial shell syntax checks

Focused qsc runtime tests may be skipped because NA-0566 is
authorization-only, changes only governance/evidence/testplan files, and
mutates no qsc source/runtime/dependency/workflow path.

## Recommendation

Proceed with implementation PR review for D-1122. After merge and healthy
post-merge checks, close out NA-0566 only if D-1122 remains accepted once and
the exact selected NA-0567 successor can be restored without placeholders.
