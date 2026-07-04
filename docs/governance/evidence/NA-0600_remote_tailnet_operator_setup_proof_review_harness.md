Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-04

# NA-0600 remote / Tailnet operator setup proof review harness

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0600 reviewed whether operator-provided setup/proof was sufficient to
proceed to a first bounded remote/Tailnet full-stack verification lane.

Result classification:
`REMOTE_TAILNET_OPERATOR_SETUP_PROOF_STILL_REQUIRED`.

Selected successor:
`NA-0601 -- QSL Remote / Tailnet Operator Setup Completion Harness`.

No operator setup proof artifact was supplied beyond qwork/startup proof and
inherited NA-0599 readiness evidence. qsl-server relay secret names are present,
but GitHub variables are absent, no Tailnet join workflow surface exists, no
qsl-attachments remote workflow surface exists, and no safe proof was provided
for Tailnet access, GitHub-hosted runner Tailnet join, remote service readiness,
or rollback/disable.

## qwork Proof Verification

Fresh qwork proof files were copied from `/srv/qbuild/work/NA-0600/.qwork/`
into the proof root and parsed with file-backed parsers before fetch, metadata
review, repository mutation, PR creation, source-analysis result publication, or
proof publication. Verified values included lane `NA-0600`, repo
`qsl-protocol`, path `/srv/qbuild/work/NA-0600/qsl-protocol`, clean
worktree/index/untracked state, READY_COUNT 1, queue top READY `NA-0600`,
`HEAD`/`origin/main`/`main` at `3f551c0735bb`, proof timestamp at or after
`2026-07-04T03:42:55Z`, shared cargo target mode, and shared target ready.

Codex did not run qwork, qstart, or qresume.

## D-1189 / D-1190 Inheritance

D531, D-1189, D-1190, NA-0599 evidence, and NA-0599 closeout state were
consumed.

- D-1189 exists once and is Accepted.
- D-1190 exists once and is Accepted.
- NA-0599 is DONE.
- NA-0600 is READY.
- D-1191 was absent before patch.
- D-1192 was absent before patch.
- D-1189 result classification was
  `REMOTE_TAILNET_REINTRODUCTION_READINESS_OPERATOR_SETUP_REQUIRED`.
- D-1189 selected NA-0600.
- D-1190 restored NA-0600.
- D-1189 selected GitHub-hosted runner Tailnet join for job duration, pending
  operator setup/proof.
- NA-0599 ran no remote verification and did not mutate remote, Tailnet,
  workflow, GitHub secret, DNS, Cloudflare, public-site, or deployment state.

## Authority Model Application

NA-0600 used Tier 0 read-only review, Tier 1 proof-root parsers/classifiers, and
Tier 2 governance/proof-review documentation. It did not use Tier 3 authority.
No remote/Tailnet diagnostics, SSH, scp, Tailscale commands, workflow
dispatch/rerun, GitHub secret or variable mutation, DNS/Cloudflare/public-site
mutation, deployment mutation, source mutation, workflow mutation, dependency
mutation, or lockfile mutation occurred.

## Operator Proof Inventory

Classification: `OPERATOR_PROOF_NOT_PROVIDED`.

No operator setup proof artifact was supplied in the allowed discovery
locations. No private-material stop was triggered because no operator proof
package with private material was supplied.

## GitHub Secret-Name Review

Only GitHub Actions secret names were reviewed. No secret values were requested,
read, printed, inferred, or published.

- RELAY_URL secret-name: present.
- RELAY_TOKEN secret-name: present.
- qsl-attachments endpoint secret-name: not_defined.
- qsl-attachments capability/token secret-name: not_defined.
- Tailscale OAuth/client secret-name set: not_defined.
- workflow uses secret names safely: yes.
- workflow exposes secret values: no observed workflow-level exposure.
- secret-value access: none.

## GitHub Variable-Name Review

Only GitHub Actions variable names were reviewed. No variable values or secret
values were accessed.

- GitHub variables: missing.
- repository variable-name count class: zero.
- variable-value access: none.

## Current Workflow Surface Review

- remote-handshake workflow: present.
- remote-relay workflow: present.
- remote-attachment workflow: absent.
- Tailnet join step: absent.
- redacted DNS/TCP/TLS/HTTP diagnostics: partial.
- qsl-server route-shape diagnostics: partial.
- qsl-attachments shape diagnostics: absent.
- qsc remote handshake: present.
- qsc remote relay E2EE: present.
- qsc remote attachment: absent.
- artifact/log redaction: partial.
- workflow mutation needed before remote verification: yes.

NA-0600 did not mutate workflow files.

## Tailnet Access Proof Review

- Tailnet access setup: missing.
- GitHub-hosted runner Tailnet join: not_configured.
- Tailscale OAuth/client proof: proof_missing.
- ACL/tag approval proof: proof_missing.
- rollback/disable proof: missing.
- private endpoint values: not_published.

No Tailscale command was run. No Tailnet mutation occurred.

## qsl-server Service Readiness Proof Review

- endpoint source: secret_backed.
- service readiness: operator_required.
- auth/route boundary: unknown.
- rollback: missing.

No qsl-server start, stop, deploy, remote check, or deployment mutation
occurred. No endpoint value, private port, route token, bearer value,
Authorization value, payload/body/plaintext, raw log, or topology was
published.

## qsl-attachments Service Readiness Proof Review

- endpoint source: absent.
- service readiness: operator_required.
- capability boundary: unknown.
- storage boundary: unknown.
- rollback: missing.

No qsl-attachments start, stop, deploy, remote check, or deployment mutation
occurred. No endpoint value, private port, capability value, payload/body/
plaintext, ciphertext body, storage path, raw log, or topology was published.

## Access Model Recheck

D-1189 selected model A: GitHub-hosted runner joins Tailnet for job duration.
Selected model status:
`SELECTED_MODEL_OPERATOR_SETUP_STILL_REQUIRED`.

Self-hosted runner and public endpoint paths are not selected by NA-0600.

## Redacted Diagnostic Plan Refresh

Required future phase classes remain runner_tailnet_join, endpoint_source,
dns_resolution, tcp_connect, tls_handshake_or_http_class,
qsl_server_route_shape, qsl_attachments_shape, qsc_remote_handshake,
qsc_remote_relay_e2ee, qsc_remote_attachment_send_receive,
cleanup_or_teardown, and rollback_or_disable.

Next-lane classification: operator setup required.

NA-0600 refreshed the plan but did not run remote diagnostics. Future evidence
must publish only classes, counts, elapsed-time buckets, and redacted failure
reasons. Endpoint values, private ports, IP addresses unless explicitly public
and authorized, Tailnet hostnames, route tokens, bearer/Authorization values,
capabilities, payload/body/plaintext, ciphertext bodies, seed/key material, raw
topology, raw artifacts, and raw workflow logs remain unpublished.

## Security / Metadata / Claim Review

- Private endpoint risk: values were not published, but setup remains unproven.
- Tailnet trust boundary: operator setup/proof required.
- GitHub-hosted runner trust boundary: not configured for Tailnet join.
- Secret-name versus secret-value boundary: preserved.
- qsl-server route/auth boundary: endpoint secret name present, readiness
  unproven.
- qsl-attachments capability boundary: endpoint and capability names not
  defined.
- Artifact/log redaction: partial workflow surface only.
- Rollback/disable strategy: missing.
- Public/production claim boundary: preserved.

NA-0600 makes no public-readiness, production-readiness,
public-internet-readiness, vulnerability-free, bug-free, remote-ready,
Tailnet-ready, crypto-complete, attachment-complete, side-channel-free,
formal-proof-complete, or external-review-complete claim.

## Readiness Matrix

| Class | Result |
|---|---|
| operator proof package | absent |
| GitHub secret names | partial |
| GitHub variable names | missing |
| Tailnet access | operator_required |
| runner access | blocked |
| qsl-server remote service | operator_required |
| qsl-attachments remote service | operator_required |
| workflow diagnostics | partial |
| redaction policy | partial |
| first remote verification lane | operator_setup_required |

## Result Classification

Selected result:
`REMOTE_TAILNET_OPERATOR_SETUP_PROOF_STILL_REQUIRED`.

This is not a private-material stop and not an ambiguous stop. The evidence is
sufficient to choose a successor, but insufficient to proceed to remote
verification.

## Selected Successor

Selected successor:
`NA-0601 -- QSL Remote / Tailnet Operator Setup Completion Harness`.

Objective:
Complete the operator setup/proof cycle for the D-1189-selected GitHub-hosted
runner Tailnet access model. Director must provide one-step-at-a-time operator
instructions. Operator, not Codex, performs any Tailnet/GitHub-secret/service
setup. Codex may later review only safe proof classes. No Codex Tailnet command,
GitHub secret mutation, remote action, workflow dispatch, DNS/Cloudflare change,
deployment mutation, private-material publication, or public/production/
security-completion claim is authorized.

## Required-Check Boundary

Current main health passed before mutation: public-safety success, advisories
success, suite2-vectors success, no failed visible check-runs, no pending
visible check-runs, no missing/failed/pending required contexts after
D498-style visibility recovery for PR-head-only goal-lint and aggregate CodeQL,
root cargo audit success, nested qsc fuzz cargo audit success, locked cargo
metadata success, and Cargo drift absent.

## Source / Workflow Mutation Boundary

No qsl-protocol source, workflow, runtime, dependency, lockfile, script,
public-site, docs/public, qwork/qstart/qresume, qshield, qshield-cli, formal,
refimpl, or backup path mutation occurred.

## qsc Boundary

No qsc source, test, example, fuzz, script, or runtime path was mutated.

## qsl-server Boundary

No qsl-server source, test, deployment, runtime, service, remote, or workflow
path was mutated.

## qsl-attachments Boundary

No qsl-attachments source, test, deployment, runtime, service, remote, or
workflow path was mutated.

## Remote / Tailscale Boundary

No remote command, SSH, scp, Tailscale command, Tailnet setup, remote
diagnostic, workflow dispatch, workflow rerun, or GitHub runner Tailnet join was
performed.

## Public-Site / Cloudflare Boundary

No public-site, docs/public, website, DNS, Cloudflare, firewall, public
endpoint, TLS, public service exposure, or deployment mutation occurred.

## Evidence / Decision / Traceability

NA-0600 adds this evidence document, the NA-0600 testplan, D-1191 in
`DECISIONS.md`, a TRACEABILITY row, and a rolling operations journal entry.

## Validation

Planned validation covers git diff check, scope guard, queue/decision proof,
marker proof, link-check, private-material scans, overclaim scans, docs/
governance-only classifier, PR body preflight, goal-lint if available, cargo
audits, locked metadata, cargo fmt, and qsc-adversarial shell syntax.

Focused runtime tests may be skipped because NA-0600 is proof-review/readiness
only, mutates no qsc/qsl-server/qsl-attachments source or runtime surface,
mutates no workflow/dependency/lockfile surface, and performs no remote/Tailnet
execution.

## Recommendation

Do not run remote/Tailnet verification yet. Complete an operator setup/proof
cycle first under NA-0601 using one-step-at-a-time Director instructions and
operator-owned setup actions. Codex should later review only safe proof classes
unless a later exact directive authorizes additional work.
