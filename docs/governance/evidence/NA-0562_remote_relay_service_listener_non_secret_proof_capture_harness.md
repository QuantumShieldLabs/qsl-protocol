Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-28

# NA-0562 Remote Relay Service Listener Non-Secret Proof Capture Harness

## Executive Summary

NA-0562 executed the D-1112-authorized service listener non-secret proof after
D482 stopped safely on a readiness-marker authority mismatch. D483 recovered by
using the D-1112 exact readiness marker.

Result classification:
`SERVICE_OWNER_PORT_MISMATCH_REMEDIATION_READY`.

Remote listener proof summary:
- SSH listener readiness: `SSH_LISTENER_PROBE_READY`
- listener presence class: `no`
- listener bind class: `not_available`
- other loopback listener presence class: `yes`
- TCP connect class: `refused`
- v1_push HEAD class: `not_checked`
- remote listener probe classification:
  `REMOTE_LISTENER_OTHER_LOOPBACK_PRESENT_EXPECTED_ABSENT`

The selected successor is
`NA-0563 -- QSL Remote Relay Loopback Port Alignment Authorization Plan`.

Raw SSH and remote probe outputs remain proof-root-only. No endpoint values,
private hosts/IPs, private topology, route-token/capability values, bearer
values, Authorization headers, payloads, response bodies, process identity,
authorized_keys content, public key material, private keys, secret environment
values, Cloudflare tokens, or API keys are published.

## qwork Proof Verification

Fresh NA-0562 qwork proof files were copied into the D483 proof root and parsed
with a file-backed parser before fetch, SSH, or repository mutation.

- qwork proof timestamp: `2026-06-28T23:03:37Z`
- lane: `NA-0562`
- repo: `qsl-protocol`
- qwork HEAD: `6972309858fa`
- qwork origin/main: `6972309858fa`
- READY_COUNT: `1`
- READY: `NA-0562`
- worktree/index/untracked in qwork proof: clean
- cargo target mode: shared
- shared target ready: yes

The live pre-fetch checkout matched the qwork proof. Root disk usage was below
the stop threshold and `/backup/qsl` was mounted.

## D-1112 / D-1113 Inheritance

D-1112 exists once and is Accepted. It selected the NA-0562 service listener
non-secret proof capture lane, exact SSH command-execution readiness check,
exact remote listener probe through SSH stdin, proof schema, private-material
policy, and root-cause decision tree.

D-1113 exists once and is Accepted. It marked NA-0561 DONE and restored NA-0562
as the sole READY successor.

Inherited NA-0560/D-1112 facts consumed:
- SSH command execution to the operator-selected host label succeeded.
- SSH port-forward setup succeeded.
- The forwarded relay TCP probe was refused.
- Forwarded HEAD was not checked because TCP was refused.
- The remote command-execution environment reported endpoint configured `no`,
  bearer candidate present `no`, and route candidate present `no`.
- Private-material scans passed.

## D482 Marker Mismatch Recovery

D482 stopped before script generation, SSH execution, repository mutation, PR
creation, merge, or closeout because D482 and D-1112 disagreed on the exact
readiness marker.

D483 resolved the mismatch by making D-1112 authoritative. The D483 SSH
readiness command used the D-1112 marker exactly and did not use the incorrect
D482 marker.

Recovered proof-parser issue: the first D482 recovery proof parser required an
overly narrow response phrase. The proof was rerun with D482 proof-root JSON and
the actual D482 stop wording. Final result: pass.

## Current Main Required-Check Classification

Current main was fetched only after the qwork/live/disk/mount gates passed.
Local main and origin/main matched `6972309858fa`, and origin/main descended
from the D481 closeout commit.

Required-check classification:
- public-safety: success
- advisories: success
- suite2-vectors: success
- failed required checks: `0`
- pending required checks: `0`
- branch-protection required contexts: classified green or conclusively
  satisfied

PR #1396 head check-runs were included for PR-head-only goal-lint and aggregate
CodeQL satisfaction while retaining merge-commit public-safety and advisories
evidence.

## Remote Listener Probe Script Design and Static Review

The proof-root-only script was generated at:
`/srv/qbuild/tmp/NA0562_service_listener_probe_marker_recovery_20260628T233212Z/probe_scripts/remote_listener_probe.py`.

Static review result: pass.

Review properties:
- Python syntax check passed.
- Forbidden import scan passed for subprocess, os.system, pathlib writes,
  open write modes, shutil, and tempfile.
- The script emits JSON only.
- The script does not run qsc or external commands.
- The script does not persist files on the remote host.
- The script reads only proc-net listener state and performs one TCP connect to
  the expected loopback target.
- The script sends only HEAD for the v1_push path if TCP connect succeeds.
- The script reads only the status line for classification and does not read or
  print response body content.
- The script does not transmit auth, route, bearer, payload, or body material.

## SSH Listener Readiness

The D-1112 exact SSH readiness command was executed once.

Classification:
- SSH listener readiness: `SSH_LISTENER_PROBE_READY`
- exit code: `0`
- readiness marker present: yes
- private-material scan: pass
- raw stdout/stderr: proof-root-only

No account, shell, authorized_keys, Tailscale, firewall, or service mutation
occurred.

## Remote Listener Proof

The D-1112 exact remote listener probe through SSH stdin was executed once after
readiness succeeded.

Remote listener proof classification:
- remote listener probe classification:
  `REMOTE_LISTENER_OTHER_LOOPBACK_PRESENT_EXPECTED_ABSENT`
- listener presence class: `no`
- listener bind class: `not_available`
- other loopback listener presence class: `yes`
- TCP connect class: `refused`
- v1_push HEAD class: `not_checked`
- response body disclosed: `no`
- process identity disclosed: `no`
- endpoint value disclosed: `no`
- token value disclosed: `no`
- bearer value disclosed: `no`
- authorization header disclosed: `no`
- private topology disclosed: `no`
- service state mutated: `no`
- account or shell mutated: `no`
- authorized_keys material disclosed: `no`
- raw output private-material scan: pass

The proof indicates the expected loopback listener was absent, TCP to the
expected target was refused, and some other loopback listener exists without
publishing any port, endpoint, topology, or process detail.

## GitHub Metadata Review

Read-only GitHub metadata reviewed:
- current main check-runs
- current main combined status
- branch-protection required checks
- PR #1396 metadata and head check-runs for required-context classification

No secret values were accessed. No repository variable values were printed.

## Private-Material Review

Private-material scans passed for:
- SSH listener readiness raw stdout/stderr
- remote listener probe raw stdout/stderr
- parsed remote listener JSON
- proof summaries used for repository docs

No endpoint values, private hosts/IPs, private topology, process identities,
route-token/capability values, bearer values, Authorization headers, payloads,
response bodies, raw authorized_keys content, public SSH key material, private
keys, secret environment values, Cloudflare tokens, API keys, or long opaque
token strings were found in publishable evidence.

## Root-Cause Classification

Selected root-cause classification:
`SERVICE_OWNER_PORT_MISMATCH_REMEDIATION_READY`.

Rationale: the expected loopback listener was absent and TCP was refused, while
another loopback listener was present. That suggests the expected loopback target
may be stale or mismatched, without publishing any port or topology details.

## Selected Successor

Selected successor:
`NA-0563 -- QSL Remote Relay Loopback Port Alignment Authorization Plan`.

Successor model: authorize a bounded non-secret remediation plan for likely
alignment between the SSH tunnel expected loopback target and the remote relay
service listener. NA-0563 is authorization-only and must not allow Codex service,
tunnel, account, source, workflow, dependency, qsl-server, qsl-attachments,
public-site, or Cloudflare mutation.

## Required-Check Boundary

public-safety and advisories were green before probe execution and before this
patch. No failed or pending required check was classified.

## Source / Script Mutation Boundary

No source file was changed. No repository script was changed. The remote listener
probe script was proof-root-only and was not committed to the repository.

## Workflow Mutation Boundary

No workflow files were changed. No workflow dispatch, rerun, cancel, deletion,
or mutation occurred.

## Runtime / qsc / Dependency Boundary

No qsc send/receive or E2EE command occurred. No local qsc reproduction
occurred. No dependency or lockfile was changed.

## qsl-server / qsl-attachments Boundary

No qsl-server or qsl-attachments command, clone, build, run, local use, or
mutation occurred.

## Remote-Action Boundary

Remote action was limited to exactly two D-1112-authorized SSH commands:
- the readiness command using the D-1112 marker
- the remote listener probe through SSH stdin

No scp, sftp, rsync, Tailscale command, sudo/admin action, account mutation,
shell mutation, authorized_keys mutation, firewall mutation, service mutation,
process inspection, service config inspection, or response-body inspection
occurred.

## Public-Site / Cloudflare Boundary

No public-site content, docs/public content, website path, public path,
Cloudflare configuration, or deployment setting was changed.

## Raw Output Boundary

Raw SSH and remote listener probe outputs remain proof-root-only. Repository docs
contain only coarse classifications and scan outcomes.

## Claim Boundary

No public-readiness claim was made. No production-readiness claim was made. No
public-internet-readiness claim was made. No external-review-complete claim was
made. No backup/restore-complete claim was made. No vulnerability-free claim was
made. No bug-free claim was made. No perfect-build or perfect-crypto claim was
made.

## Validation

Required validation is recorded in the proof root and testplan. Focused qsc
runtime tests were skipped because this is an evidence/governance-only change,
no qsc source/runtime/dependency/workflow mutation occurred, and no local qsc
send/receive was authorized.

## Recommendation

Proceed to NA-0563 loopback port alignment authorization after implementation
merge and closeout gates are green. NA-0563 should remain authorization-only and
must require non-secret operator/service-owner proof without publishing endpoint,
private topology, token, bearer, Authorization header, payload, response body,
process identity, or secret environment details.
