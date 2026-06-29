Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0567 Remote Relay Codex Recovery Authority Pivot

## Executive Summary

NA-0567 is accepted as an authorization-only governance pivot from the
D-1122/D-1123 operator-proof-only lane to a bounded Codex-executed recovery
successor. The selected classification is
`REMOTE_RELAY_CODEX_EXECUTED_RECOVERY_AUTH_READY`.

The exact successor is:

`NA-0568 -- QSL Remote Relay Inspiron Codex-Executed Deployment Recovery Harness`

NA-0567 did not run SSH, Tailscale, remote commands, qsc send/receive,
workflow dispatches, reruns, source changes, script changes, workflow changes,
dependency changes, account changes, service changes, public-site changes, or
Cloudflare changes. It records operator intent and the bounded authority model
for NA-0568 only.

## qwork Proof Verification

Fresh qwork proof files were copied from the NA-0567 lane workspace and parsed
with a file-backed parser before fetch or repository mutation.

Required qwork values passed:

- `startup_result=OK`
- `lane=NA-0567`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0567/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0567`
- `requested_lane_status=READY`

The qwork proof timestamp was `2026-06-29T05:49:41Z`. Codex did not run
`qwork`, `qstart`, or `qresume`.

Pre-fetch live state passed:

- `HEAD` and `origin/main`: `e26e57f58273`
- worktree/index/untracked state: clean
- root disk usage: below the 95 percent stop threshold
- `/backup/qsl`: mounted

After the proof/live/disk gates passed, `origin/main` was fetched and verified
to equal or descend from `e26e57f58273`.

## D-1122 / D-1123 Inheritance

D-1122 exists exactly once and is Accepted. D-1122 accepted NA-0566 listener
deployment proof authorization and selected NA-0567 only as an
operator/service-owner non-secret proof capture lane.

D-1123 exists exactly once and is Accepted. D-1123 closed NA-0566 after PR
#1405 merged at `64945f366851`, restored NA-0567 as the sole READY item, and
preserved the operator-proof-only boundary selected by D-1122.

NA-0566 is DONE. NA-0567 is READY. D-1124 and D-1125 were absent before this
patch. Duplicate decision count was zero.

## Operator Intent

The operator supplied a stronger remote recovery intent:

- `inspiron` is the operator's personal system.
- `inspiron` is set up specifically for QSL/QSC testing.
- The `qslcodex` account was intentionally created by the operator and the
  prior Director for remote test use.
- The operator wants Codex to log in during the next lane, gather required
  information, set up what is needed, and restart testing.
- The operator wants less circular proof gathering and more root-cause repair.
- Codex should be trusted more while preserving evidence, rollback,
  no-secret publication, and no public/security overclaim discipline.

Operator-provided visible state was recorded only at a coarse non-secret level:

- the `qslcodex` home exists;
- the qslcodex test workspace exists;
- the qsc test binary path exists and is executable by `qslcodex`;
- the qslcodex test workspace includes an E2EE-related subdirectory;
- prior SSH command execution and port-forward setup succeeded;
- prior expected target listener proof found no usable listener/candidate.

No raw key material, private endpoint, private port, private topology, token,
Authorization header, payload, response body, service process identity, or
secret environment value is published here.

## Current Main Required-Check Classification

Current main required checks were classified on `e26e57f58273`.

Results:

- `public-safety`: completed success
- `advisories`: completed success
- `suite2-vectors`: completed success
- failed current check runs: 0
- pending current check runs: 0
- branch-protection contexts: green or conclusively satisfied

The branch-protection `CodeQL` context was classified through successful CodeQL
workflow runs on the same main SHA. The branch-protection `goal-lint` context is
a PR-only workflow and was classified as satisfied by prior branch admission for
the current main commit, not as an attached main-push check run.

## Prior Remote Relay Evidence

The inherited evidence establishes one shared remote relay issue across
remote-handshake and remote-relay proof lanes. qsc runtime is not currently the
primary suspect. Demo script logic is not currently the primary suspect. GitHub
runner proof remains deferred until the remote deployment/listener state is
repaired or conclusively classified.

The strongest current boundary remains remote relay listener/deployment state on
the operator-owned test host. NA-0565 found no usable candidate listener. NA-0566
therefore selected operator/service-owner listener deployment proof. NA-0567 now
records the operator's request to move the next lane from proof-only to bounded
Codex-executed recovery.

## Authority Pivot Rationale

Accepted D-1122 and D-1123 make NA-0567 an operator-proof-only lane. Running SSH
or mutating the remote test host inside NA-0567 would contradict that accepted
authority.

The safe pivot is to record D-1124 and select NA-0568 as the first lane where
Codex may run bounded SSH commands and bounded qslcodex-workspace recovery
actions. This preserves truthful authority sequencing while responding to the
operator's request for root-cause repair.

## NA-0568 Recovery Model

NA-0568 may authorize Codex to:

- verify qwork/current-main gates;
- run bounded SSH readiness to `inspiron`;
- run proof-root-generated remote inventory scripts through SSH stdin;
- inspect only the qslcodex test workspace and coarse listener/deployment state;
- use the existing qsc test binary path if present;
- inspect coarse service/listener/deployment state without publishing secrets;
- create files only under the qslcodex test workspace if needed;
- repair missing test workspace structure under the qslcodex test workspace;
- start only user-owned, non-privileged test processes under `qslcodex` if no
  sudo, service, or systemd action is required;
- preserve rollback and cleanup artifacts;
- run bounded local loopback verification as `qslcodex`;
- stop before sudo, systemd, Tailscale, admin, firewall, or authorized_keys
  changes;
- stop before publishing private ports, endpoints, tokens, topology, or process
  identities;
- produce proof-root-only raw outputs and repository-safe summaries.

NA-0568 may not authorize Codex to:

- run sudo;
- run systemctl;
- mutate root-owned system paths or service state;
- mutate Tailscale or firewall state;
- edit authorized_keys;
- alter the qslcodex shell;
- publish secrets, private topology, private ports, or process identities;
- run GitHub workflow dispatch or rerun;
- mutate qsl-protocol source, scripts, workflows, or dependencies;
- run qsl-server or qsl-attachments outside explicitly discovered safe qslcodex
  test workspace evidence;
- make public, production, or security completion claims.

## NA-0568 Exact Command Allowlist

The selected future command families are exact and limited.

SSH readiness:

```bash
ssh -o BatchMode=yes -o ConnectTimeout=10 inspiron 'printf INSPIRON_REMOTE_RECOVERY_READY\n'
```

Remote inventory through SSH stdin:

```bash
ssh -o BatchMode=yes -o ConnectTimeout=10 inspiron 'python3 - --origin INSPIRON_QSLCODEX_REMOTE_RECOVERY_INVENTORY' < "$PROOF_DIR/probe_scripts/remote_recovery_inventory.py"
```

Remote repair through SSH stdin, only if inventory classifies the repair as
safe inside the qslcodex test workspace:

```bash
ssh -o BatchMode=yes -o ConnectTimeout=10 inspiron 'python3 - --origin INSPIRON_QSLCODEX_REMOTE_RECOVERY_REPAIR' < "$PROOF_DIR/probe_scripts/remote_recovery_repair.py"
```

Remote post-repair proof through SSH stdin:

```bash
ssh -o BatchMode=yes -o ConnectTimeout=10 inspiron 'python3 - --origin INSPIRON_QSLCODEX_REMOTE_RECOVERY_POSTCHECK' < "$PROOF_DIR/probe_scripts/remote_recovery_postcheck.py"
```

Optional qsc smoke proof is allowed only if inventory proves the qsc binary
exists, no secrets are needed, and output can be kept free of endpoints, tokens,
private topology, and process identity. Version/help/status-class proof is
preferred before any networked qsc action.

Remote scripts must be generated proof-root-only, sent through stdin, create a
rollback manifest before mutation, write only under the qslcodex test workspace
when repair is authorized, avoid sudo/systemctl/service calls, avoid
authorized_keys inspection, avoid reading secret files, and avoid printing
private ports, endpoints, tokens, process identities, response bodies, or
private material.

## NA-0568 Private-Material Policy

Repository docs must not publish:

- raw authorized_keys content;
- public keys;
- private keys;
- private ports;
- endpoints;
- private topology;
- tokens;
- bearer values;
- Authorization headers;
- payloads;
- response bodies;
- service process identities;
- secret environment values.

Repository-safe output is limited to coarse classifications, non-secret command
allowlist names, short SHA evidence, and summaries that have been redacted before
publication. Raw remote outputs remain proof-root-only.

## NA-0568 Stop Conditions

NA-0568 must stop before:

- sudo, systemctl, service, Tailscale, firewall, account, shell, or
  authorized_keys mutation;
- root-owned path mutation;
- remote writes outside the qslcodex test workspace;
- qsc send/receive unless a later exact safe phase authorizes it;
- secret, endpoint, private topology, private port, response body, or process
  identity publication;
- GitHub workflow dispatch or rerun;
- qsl-protocol source, script, workflow, dependency, or lockfile mutation;
- public-site or Cloudflare mutation;
- any forbidden claim; specifically no public-readiness claim, no
  production-readiness claim, no vulnerability-free claim, no bug-free claim,
  and no perfect-build claim;
- ambiguous root cause where continuing would risk behavior drift or untruthful
  evidence.

Potential NA-0568 classifications:

- `REMOTE_RECOVERY_QSLCODEX_WORKSPACE_REPAIRED_TESTING_READY`
- `REMOTE_RECOVERY_QSLCODEX_WORKSPACE_ALREADY_READY`
- `REMOTE_RECOVERY_NEEDS_OPERATOR_SUDO_SERVICE_ACTION`
- `REMOTE_RECOVERY_NEEDS_TAILSCALE_OR_FIREWALL_OPERATOR_ACTION`
- `REMOTE_RECOVERY_NEEDS_SECRET_OR_ENDPOINT_OPERATOR_ACTION`
- `REMOTE_RECOVERY_PRIVATE_MATERIAL_STOP`
- `REMOTE_RECOVERY_AMBIGUOUS_STOP`

## Selected Successor

Selected successor:

`NA-0568 -- QSL Remote Relay Inspiron Codex-Executed Deployment Recovery Harness`

Status: READY

Goals: G1, G2, G3, G4, G5

NA-0568 is the first lane authorized to let Codex run bounded SSH commands and
bounded non-secret recovery actions inside the qslcodex test workspace. Exactly
one READY item remains mandatory after closeout.

## Required-Check Boundary

NA-0567 required-check evidence is current-main classification only plus later PR
checks. It does not change branch protection, required checks, workflows, or CI
semantics.

## Source / Script Mutation Boundary

NA-0567 changes no source code, runtime code, scripts, generated remote scripts,
workflow files, dependencies, lockfiles, qsc source/test/fuzz/Cargo files,
qsl-server files, qsl-attachments files, website files, public docs, backup
state, or qwork/qstart/qresume tooling.

## Workflow Mutation Boundary

NA-0567 runs no workflow dispatch and no rerun. It mutates no workflow files and
does not change required-check configuration.

## Runtime / qsc Boundary

NA-0567 runs no qsc send/receive, no qsc E2EE, and no qsc runtime action. The
future NA-0568 qsc boundary is limited to non-secret smoke/status-class proof
only if inventory proves it is safe.

## qsl-server / qsl-attachments Boundary

NA-0567 runs no qsl-server or qsl-attachments command and mutates no
qsl-server/qsl-attachments state. NA-0568 must not run those tools outside
explicitly discovered safe qslcodex test workspace evidence.

## Remote-Action Boundary

NA-0567 performs no SSH, Tailscale, remote command, remote probe, service
mutation, account mutation, shell mutation, authorized_keys mutation, sudo,
systemctl, or remote repair. It only authorizes the future NA-0568 boundary.

## Public-Site / Cloudflare Boundary

NA-0567 mutates no public-site content and performs no Cloudflare action.

## Claim Boundary

NA-0567 makes no public-readiness claim.

NA-0567 makes no production-readiness claim.

NA-0567 makes no vulnerability-free claim.

NA-0567 makes no bug-free claim.

NA-0567 makes no perfect-build claim.

## Validation

Required validation for the implementation PR includes:

- exact five-path implementation scope guard;
- queue and decision proof;
- marker proof;
- markdown link check;
- added-line/new-file private-material scan;
- overclaim scan;
- docs/governance-only classifier;
- PR body preflight and goal-lint if available;
- root cargo audit;
- nested qsc fuzz lock cargo audit;
- cargo fmt check;
- qsc adversarial shell syntax checks.

Focused qsc runtime tests may be skipped because this is an
authorization-only governance/evidence/testplan change and mutates no qsc
source, runtime, dependency, or workflow path.

## Recommendation

Merge the NA-0567 authorization-only pivot after validation and required PR
checks pass. After merge and post-merge check proof, close out NA-0567 to the
exact NA-0568 successor without starting NA-0568 during closeout.
