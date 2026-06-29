Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0563 Remote Relay Loopback Port Alignment Authorization Plan

## Executive Summary

NA-0563 is authorization-only. It consumes D-1114/D-1115 and the NA-0562
listener proof that the expected remote loopback listener was absent, another
loopback listener class was present, TCP to the expected target was refused, and
the v1 push HEAD class was not checked.

Selected result classification:
`REMOTE_RELAY_LOOPBACK_ALIGNMENT_ACTION_AUTH_READY`.

Selected successor:
`NA-0564 -- QSL Remote Relay Loopback Port Alignment Action Authorization Harness`.

NA-0563 ran no probes, SSH, Tailscale, remote commands, workflow dispatches,
reruns, qsc send/receive, qsc E2EE, local reproduction, service commands, or
qsl-server/qsl-attachments commands. It mutates only governance/evidence
records and does not publish endpoint values, route-token/capability values,
bearer values, Authorization headers, private endpoint hosts, private topology,
private port values, process identities, payloads, response bodies,
authorized_keys material, public SSH key material, private keys, secret
environment values, Cloudflare tokens, API keys, raw logs, raw artifacts, or
private material.

## qwork Proof Verification

Fresh NA-0563 qwork proof files were copied into the proof root and parsed with
a file-backed parser before fetch or repository mutation.

- qwork proof timestamp: `2026-06-29T00:18:11Z` or later
- lane: `NA-0563`
- repo: `qsl-protocol`
- qwork HEAD: `7775babf409e`
- qwork origin/main: `7775babf409e`
- READY_COUNT: `1`
- READY: `NA-0563`
- worktree/index/untracked in qwork proof: clean
- cargo target mode: shared
- shared target ready: yes

The live pre-fetch checkout matched the qwork proof. Root disk usage was below
the stop threshold and `/backup/qsl` was mounted. Codex did not run qwork,
qstart, or qresume.

## D-1114 / D-1115 Inheritance

D-1114 exists once and is Accepted. It accepted NA-0562 remote relay service
listener non-secret proof capture and selected NA-0563 as a loopback port
alignment authorization lane.

D-1115 exists once and is Accepted. It marked NA-0562 DONE and restored NA-0563
as the exactly one READY successor.

Inherited NA-0562 result classification:
`SERVICE_OWNER_PORT_MISMATCH_REMEDIATION_READY`.

No NA-0563 implementation existed before this directive, and no private
material was published by inherited repository evidence.

## Current Main Required-Check Classification

Current main was verified at `7775babf409e`.

- public-safety: completed success
- advisories: completed success
- suite2-vectors: completed success or conclusively satisfied
- failed required checks: none
- pending required checks: none
- branch-protection required contexts: classified green or conclusively
  satisfied
- Cargo.toml drift: none
- Cargo.lock drift: none
- nested qsc fuzz Cargo.lock drift: none

Recovered proof-tooling note: the first local current-main classifier treated
PR-head `goal-lint` and aggregate CodeQL contexts as pending. This was a
recoverable classifier issue because the captured GitHub metadata showed
`goal-lint` success in PR #1398 rollup and successful CodeQL analysis runs.
The classifier was rerun once with those repository-local rules and passed.

## NA-0562 Evidence Review

NA-0562 evidence consumed these coarse facts:

- readiness marker recovery completed
- SSH readiness classification: `SSH_LISTENER_PROBE_READY`
- remote listener probe classification:
  `REMOTE_LISTENER_OTHER_LOOPBACK_PRESENT_EXPECTED_ABSENT`
- listener_present_class: `no`
- listener_bind_class: `not_available`
- other_loopback_listener_presence_class: `yes`
- tcp_connect_class: `refused`
- v1_push_head_class: `not_checked`
- private-material scan: pass

Repository evidence published no endpoint values, route-token/capability values,
bearer values, Authorization headers, private topology, process identity,
payloads, response bodies, authorized_keys content, public SSH key material, or
private material.

## Loopback Port Alignment Analysis

Boundary classifications:

- Expected tunnel target boundary:
  `EXPECTED_TUNNEL_TARGET_STALE_LIKELY`
- Actual relay listener boundary:
  `ACTUAL_RELAY_LISTENER_PRESENT_BUT_NOT_IDENTIFIED`
- Port-alignment boundary:
  `LOOPBACK_PORT_ALIGNMENT_REQUIRED`
- SSH authorized_keys/permitopen boundary:
  `SSH_PERMITOPEN_TARGET_MAY_BE_STALE`
- Service deployment boundary:
  `SERVICE_DEPLOYMENT_ALIGNMENT_PROOF_REQUIRED`
- qsc runtime boundary:
  `QSC_RUNTIME_NOT_PRIMARY_SUSPECT`
- GitHub runner boundary:
  `GITHUB_RUNNER_DEFERRED_UNTIL_LOOPBACK_ALIGNMENT`

Rationale: the expected target was absent and refused TCP while another
loopback listener class existed. That makes stale or mismatched loopback
alignment the primary next root-cause boundary. Actual listener identity and
private port values remain unpublished, so NA-0564 must prove only non-secret
classes and keep any action operator-owned.

## NA-0564 Action Model Design

Selected model: `Model C`, Codex-executed non-mutating proof plus operator
action bundle.

This model is safe because the exact future proof commands can be expressed
without private values in repository docs, and alignment actions can be
generated as an operator-owned bundle using placeholders.

Rejected models:

- Model A alone is too narrow because a safe non-mutating proof can be
  authorized first.
- Model B alone is too narrow because coarse alignment fields can be captured
  without identifying private listener values.

Codex mutation authority in NA-0564 remains `no`.

## Exact NA-0564 Command / Action Allowlist

Future NA-0564 Codex-executed proof commands:

```bash
ssh -o BatchMode=yes -o ConnectTimeout=10 inspiron 'printf INSPIRON_LOOPBACK_ALIGNMENT_PROBE_READY\n'
ssh -o BatchMode=yes -o ConnectTimeout=10 inspiron 'python3 - --origin INSPIRON_LOOPBACK_ALIGNMENT_NON_SECRET' < "$PROOF_DIR/probe_scripts/loopback_alignment_probe.py"
```

The proof-root-only remote proof script may:

- parse proc-net TCP tables directly;
- classify whether the expected target has a listener;
- classify whether other loopback listener classes exist;
- emit only count bands: none, one, multiple, or unknown;
- classify whether a candidate listener can be identified by class only.

The remote proof script must not:

- print private port values, endpoint values, private hosts/IPs, topology,
  process names, process identities, tokens, payloads, response bodies, secret
  environment values, authorized_keys material, public SSH key material, or
  private material;
- run external commands;
- inspect service configuration;
- inspect authorized_keys;
- mutate files or service state;
- run qsc.

Operator action bundles may include placeholder-only service listener
verification, service start/restart, authorized forwarding target adjustment,
tunnel target alignment, rollback, and post-action proof commands. Codex must
not execute those operator actions in NA-0564 unless a later directive
explicitly authorizes operator execution.

## NA-0564 Proof Schema

NA-0564 must emit non-secret fields only:

- expected_target_listener_present: yes/no/unknown
- other_loopback_listener_presence: yes/no/unknown
- loopback_listener_count_class: none/one/multiple/unknown
- expected_target_alignment_class: aligned/mismatched/stale/unknown
- action_owner: operator/service-owner/codex-not-authorized/unknown
- operator_action_required: yes/no/unknown
- codex_mutation_authorized: no
- endpoint_value_disclosed: no
- private_port_value_disclosed: no, unless already-authorized label only
- process_identity_disclosed: no
- token_value_disclosed: no
- response_body_disclosed: no
- private_topology_disclosed: no
- redaction_review: pass/fail
- raw_output_contains_private_material: yes/no

## NA-0564 Private-Material Policy

NA-0564 must stop before publication if output contains endpoint values, private
port values beyond an already-authorized label, private endpoint hosts/IPs,
private topology, route-token/capability values, bearer values, Authorization
headers, payloads, response bodies, process identities, secret environment
values, SSH private material, raw authorized_keys material, public SSH key
material, Cloudflare tokens, API keys, raw logs, raw artifacts, or private
material.

Repository docs may publish only coarse classes, marker names, pass/fail scan
results, and placeholder action bundles.

## NA-0564 Decision Tree

Future NA-0564 classifications:

- `LOOPBACK_ALIGNMENT_OPERATOR_ACTION_BUNDLE_READY`
- `LOOPBACK_ALIGNMENT_SERVICE_OWNER_PROOF_REQUIRED`
- `LOOPBACK_ALIGNMENT_CANDIDATE_PROOF_READY`
- `LOOPBACK_ALIGNMENT_ALREADY_CORRECT_GITHUB_RUNNER_PROOF_READY`
- `LOOPBACK_ALIGNMENT_ACCESS_UNAVAILABLE_OPERATOR_ACTION_REQUIRED`
- `LOOPBACK_ALIGNMENT_PRIVATE_MATERIAL_STOP`
- `LOOPBACK_ALIGNMENT_AMBIGUOUS_STOP`

Successor mapping:

- Operator action bundle ready: select operator action proof review or operator
  action execution lane.
- Service-owner proof required: select service-owner proof lane.
- Candidate proof ready: select candidate confirmation lane.
- Already correct: select GitHub runner tunnel/reachability proof authorization.
- Access unavailable: select operator access action lane.

## Option Review

Selected option: Option A, loopback alignment action authorization.

Rejected options:

- Option B, service-owner proof authorization, is narrower than necessary
  because bounded non-secret alignment proof can be authorized.
- Option C, GitHub runner proof authorization, is premature because loopback
  alignment is not proven clean.
- Option D, qsc runtime review, is premature because listener/tunnel alignment
  remains the primary boundary.
- Option E, stop/ambiguous, is rejected because a safe exact successor can be
  selected without private material publication.

## Result Classification

Selected result classification:
`REMOTE_RELAY_LOOPBACK_ALIGNMENT_ACTION_AUTH_READY`.

This classification authorizes only the future NA-0564 non-mutating proof model
and placeholder-only operator action bundle generation. It does not authorize
Codex to mutate remote hosts, accounts, authorized_keys, services, firewalls,
Tailscale, qsl-server, qsl-attachments, source, scripts, workflows,
dependencies, backup state, public-site content, or Cloudflare configuration.

## Selected Successor

### NA-0564 -- QSL Remote Relay Loopback Port Alignment Action Authorization Harness

Status: READY
Goals: G1, G2, G3, G4, G5

Objective:
Use the D-1116-authorized non-secret alignment model to determine whether the
remote relay SSH tunnel target, authorized forwarding target, and actual relay
listener should be aligned through an operator-owned action bundle. Codex may
perform only exact non-mutating proof commands authorized by D-1116, generate a
non-secret operator action bundle if alignment is required, and classify the
next proof-review or service-owner lane. Codex must not mutate `inspiron`,
authorized_keys, service configuration, Tailscale, firewall, qslcodex account
state, qsl-server, qsl-attachments, source, scripts, workflows, dependencies,
backup state, public-site content, or Cloudflare configuration.

Allowed scope:

- `docs/governance/evidence/NA-0564_remote_relay_loopback_port_alignment_action_authorization_harness.md`
- `tests/NA-0564_remote_relay_loopback_port_alignment_action_authorization_testplan.md`
- `docs/ops/NA-0564_remote_relay_loopback_alignment_operator_action_bundle.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- proof-root-only alignment probe outputs
- proof-root-only proposed operator action artifacts
- read-only NA-0555 through NA-0563 evidence
- exact SSH readiness command authorized by D-1116
- exact remote alignment probe through SSH stdin authorized by D-1116
- private-material scan/redaction proof
- successor selection

Forbidden scope:

- qwork/qstart/qresume execution by Codex;
- any SSH command outside exact D-1116 allowlist;
- scp/sftp/rsync;
- sudo/admin action by Codex;
- account, shell, authorized_keys, Tailscale, firewall, or service mutation by
  Codex;
- qsc send/receive;
- qsc E2EE;
- payload push;
- workflow dispatch;
- rerun;
- source/script/qsc mutation;
- workflow mutation;
- dependency/lockfile mutation;
- qsl-server/qsl-attachments command, clone, build, run, or mutation;
- qsl-backup;
- backup mutation;
- public-site mutation;
- Cloudflare mutation;
- endpoint value publication;
- route-token/capability value publication;
- bearer value publication;
- Authorization header publication;
- private endpoint or private topology publication;
- process identity publication;
- payload or response body publication;
- secret environment value publication;
- no public-readiness, production-readiness, vulnerability-free, bug-free, or
  no perfect-build claim.

## Required-Check Boundary

NA-0563 classified current-main required checks before mutation and must rely on
protected PR checks before merge. A failed, missing, or pending required check is
a stop condition unless conclusively satisfied by branch-protection metadata and
PR-head evidence.

## Source / Script Mutation Boundary

NA-0563 changes no source files and no repository scripts. The future NA-0564
remote proof script is proof-root-only and must not be committed unless a later
directive explicitly authorizes that path.

## Workflow Mutation Boundary

NA-0563 changes no workflow files and performs no workflow dispatch, rerun,
cancel, deletion, or workflow mutation.

## Runtime / qsc / Dependency Boundary

NA-0563 performs no qsc send/receive, qsc E2EE, qsc runtime review, local qsc
reproduction, dependency mutation, Cargo.toml mutation, Cargo.lock mutation, or
nested qsc fuzz lockfile mutation. qsc runtime remains not the primary suspect
until loopback alignment proof/action is clean.

## qsl-server / qsl-attachments Boundary

NA-0563 performs no qsl-server or qsl-attachments command, clone, build, run,
local use, or mutation.

## Remote-Action Boundary

NA-0563 performs no probes, SSH, Tailscale, remote commands, sudo, service
commands, systemctl commands, account mutation, shell mutation, authorized_keys
mutation, firewall mutation, service mutation, or remote file mutation.

## Public-Site / Cloudflare Boundary

NA-0563 changes no public-site content, README public-progress content,
docs/public content, public path, website path, Cloudflare configuration, or
deployment setting.

## Claim Boundary

No public-readiness claim is made. No production-readiness claim is made. No
public-internet-readiness claim is made. No external-review-complete claim is
made. No backup/restore-complete claim is made. No vulnerability-free claim is
made. No bug-free claim is made. No perfect-build claim is made. No
perfect-crypto claim is made.

## Validation

Required validation for this authorization patch includes:

- `git diff --check`
- exact implementation scope guard
- queue/decision proof
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
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Focused qsc runtime tests may be skipped because NA-0563 is
authorization-only, changes only governance/evidence/testplan files, mutates no
qsc source/runtime/dependency/workflow paths, and authorizes no local qsc
execution.

## Recommendation

Proceed to NA-0564 only after D-1116 is merged and closeout restores NA-0564 as
the sole READY item. NA-0564 should run only the exact D-1116 allowlisted
non-mutating proof commands, publish only coarse proof fields, generate
placeholder-only operator action artifacts if alignment is required, and stop on
private material or ambiguous ownership.
