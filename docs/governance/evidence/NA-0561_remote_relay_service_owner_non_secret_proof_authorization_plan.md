Goals: G1, G2, G3, G4, G5
Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-06-28

# NA-0561 Remote Relay Service Owner Non-Secret Proof Authorization Plan

## Executive Summary

NA-0561 is authorization-only. It consumes D-1110/D-1111 and the NA-0560 proof that SSH command execution and SSH local port-forward setup succeeded, while the forwarded relay TCP probe was refused before a HEAD response could be checked.

Selected result classification: `REMOTE_RELAY_SERVICE_OWNER_LISTENER_PROOF_CAPTURE_READY`.

Selected successor: `NA-0562 -- QSL Remote Relay Service Listener Non-Secret Proof Capture Harness`.

## qwork Proof Verification

Fresh qwork proof files from `2026-06-28T22:13:16Z` were copied proof-root-only and verified before repository mutation.

- lane: `NA-0561`
- repo: `qsl-protocol`
- startup result: `OK`
- repo result: `OK`
- qwork HEAD: `402616e9e2bc`
- qwork origin/main: `402616e9e2bc`
- qwork main: `402616e9e2bc`
- worktree/index/untracked proof: clean
- READY count: 1
- queue top READY: `NA-0561`
- shared cargo target proof: ready

Live pre-fetch HEAD and origin/main matched the qwork proof. Disk proof remained below the stop threshold and `/backup/qsl` was mounted. Codex did not run qwork, qstart, or qresume.

## D-1110 / D-1111 Inheritance

D-1110 exists once and is Accepted. D-1111 exists once and is Accepted. D-1112 and D-1113 were absent before this implementation patch. NA-0560 is DONE and NA-0561 is READY.

D-1110 accepted NA-0560 Codex-executed non-secret probe evidence. D-1111 accepted NA-0560 closeout and restored NA-0561 as authorization-only.

NA-0560 selected `CODEX_PROBE_SSH_TUNNEL_RELAY_UNREACHABLE_SERVICE_OWNER_PROOF_REQUIRED`. No NA-0561 implementation existed before this directive. No private material was published by the inherited evidence.

## Current Main Required-Check Classification

Current main was verified at `402616e9e2bc` and equals origin/main. origin/main descends from the expected D-1111 closeout merge.

- public-safety: completed success
- advisories: completed success
- suite2-vectors: completed success
- no failed required checks: yes
- no pending required checks: yes
- branch-protection required contexts: green on the merge commit or conclusively satisfied by PR #1394 head checks that merged into `402616e9e2bc`
- no Cargo.toml drift: yes
- no Cargo.lock drift: yes
- no qsc fuzz Cargo.lock drift: yes

## NA-0560 Probe Evidence Review

NA-0560 coarse facts consumed:

- SSH command execution: success
- SSH port-forward setup: success
- forwarded relay TCP class: refused
- forwarded HEAD class: not checked because TCP was refused
- tunnel cleanup: success
- remote command-execution environment: endpoint/auth/route candidates absent in that command-execution context
- private-material scans: passed

No endpoint values, route-token/capability values, bearer values, Authorization headers, private topology, payloads, response bodies, authorized_keys content, public key material, private key material, or secret environment values were published.

## Service Owner Boundary Analysis

Boundary classifications:

- SSH access boundary: `SSH_ACCESS_OK`
- SSH tunnel boundary: `SSH_TUNNEL_SETUP_OK`
- loopback listener boundary: `LOOPBACK_LISTENER_ABSENT_OR_PORT_MISMATCH_LIKELY`
- service deployment boundary: `SERVICE_DEPLOYMENT_DOWN_OR_NOT_LISTENING_LIKELY`, `SERVICE_DEPLOYMENT_PORT_MISMATCH_LIKELY`, `SERVICE_DEPLOYMENT_HEALTH_UNKNOWN`
- qsc runtime boundary: `QSC_RUNTIME_NOT_PRIMARY_SUSPECT`
- GitHub runner boundary: `GITHUB_RUNNER_BOUNDARY_DEFERRED_UNTIL_SERVICE_LISTENER_PROOF`

Rationale: because SSH command execution and port-forward setup worked, but the forwarded relay TCP probe was refused before a HEAD class existed, the next discriminating proof is the service owner/listener boundary.

## Exact NA-0562 Command Allowlist

NA-0562 may run exactly one bounded SSH command-execution reachability check:

```bash
ssh -o BatchMode=yes -o ConnectTimeout=10 inspiron 'printf INSPIRON_SERVICE_OWNER_NON_SECRET_READY\n'
```

NA-0562 may run exactly one remote listener probe through SSH standard input:

```bash
ssh -o BatchMode=yes -o ConnectTimeout=10 inspiron 'python3 - --origin INSPIRON_SERVICE_OWNER_NON_SECRET' < "$PROOF_DIR/probe_scripts/remote_listener_probe.py"
```

The remote script must be generated proof-root-only, statically scanned before execution, and must output JSON only. It must not write files, run qsc, run sudo, run systemctl, import subprocess, read secret files, inspect authorized_keys, print endpoint values, print private hosts/IPs/topology, print token/bearer/Authorization material, print payloads or response bodies, or disclose process identity.

The preferred listener proof method is `proc_net_tcp_socket`: parse `/proc/net/tcp` and `/proc/net/tcp6` for LISTEN state on the D-1110 expected loopback relay target port, classify bind class without publishing private addresses, attempt a loopback TCP connect to that target, and perform a HEAD class check against the v1 push path only if TCP connects. The response body must never be read or printed.

## NA-0562 Proof Schema

Required NA-0562 output fields:

- `proof_origin = INSPIRON_SERVICE_OWNER_NON_SECRET`
- `listener_probe_method = proc_net_tcp_socket`
- `listener_present_class = yes|no|unknown|error`
- `listener_bind_class = loopback|any|private_not_reported|unknown|not_available`
- `tcp_connect_class = success|refused|timeout|error|not_checked`
- `v1_push_head_class = success|not_found|method_not_allowed|client_error|server_error|timeout|error|not_checked`
- `response_body_disclosed = no`
- `process_identity_disclosed = no`
- `endpoint_value_disclosed = no`
- `token_value_disclosed = no`
- `bearer_value_disclosed = no`
- `authorization_header_disclosed = no`
- `private_topology_disclosed = no`
- `service_state_mutated = no`
- `account_or_shell_mutated = no`
- `codex_asserts_no_private_material_published = yes|no`
- `raw_output_contains_private_material = yes|no`
- `redaction_review = pass|fail`

## NA-0562 Private-Material Policy

NA-0562 must stop before repository publication if raw output contains endpoint values, private hosts/IPs, private topology, route-token/capability values, bearer values, Authorization headers, payloads, response bodies, raw authorized_keys/public key material, private keys, secret environment values, Cloudflare tokens, or API tokens.

Repository docs and final responses may contain only coarse classes and scan outcomes. Raw remote output must remain proof-root-only.

## NA-0562 Root-Cause Decision Tree

NA-0562 must select exactly one classification:

- `SERVICE_OWNER_LISTENER_ABSENT_REMEDIATION_READY`: listener absent and TCP refused.
- `SERVICE_OWNER_PORT_MISMATCH_REMEDIATION_READY`: expected target is stale or port mismatch is proven by safe/non-secret class without private topology publication.
- `SERVICE_OWNER_LISTENER_PRESENT_BUT_PATH_UNHEALTHY_READY`: TCP connects but the v1 push HEAD class is not healthy/successful.
- `SERVICE_OWNER_LISTENER_PRESENT_GITHUB_RUNNER_PROOF_READY`: listener and v1 push HEAD class are clean through the tunnel context.
- `SERVICE_OWNER_PROOF_ACCESS_UNAVAILABLE_OPERATOR_ACTION_REQUIRED`: SSH command execution is unavailable.
- `SERVICE_OWNER_PROOF_PRIVATE_MATERIAL_STOP`: raw output or scan indicates private material.
- `SERVICE_OWNER_PROOF_AMBIGUOUS_STOP`: classes are inconsistent or insufficient.

Successor mapping:

- Listener absent or port mismatch: service listener/deployment remediation authorization.
- Listener present but path unhealthy: service path/proxy boundary review.
- Listener healthy through tunnel but GitHub Actions still timeout: GitHub runner tunnel/reachability proof.
- Service proof clean and runner proof clean: qsc runtime review.
- Access unavailable: operator access action authorization.

## Option Review

Option A, service-owner listener proof capture, is selected because NA-0560 narrowed the boundary to listener/service availability after SSH and tunnel setup succeeded.

Option B, service-owner action authorization directly, is rejected because TCP refusal is not enough to choose a remediation without listener proof.

Option C, operator access action, is rejected because SSH command execution is currently proven available.

Option D, GitHub runner proof, is rejected because the tunnel-local service proof is not clean yet.

Option E, qsc runtime review, is rejected because service/listener/tunnel proof is not clean enough to shift suspicion to qsc runtime.

## Result Classification

Selected classification: `REMOTE_RELAY_SERVICE_OWNER_LISTENER_PROOF_CAPTURE_READY`.

## Selected Successor

Selected successor: `NA-0562 -- QSL Remote Relay Service Listener Non-Secret Proof Capture Harness`.

NA-0562 is a proof-capture lane for non-secret listener/service-owner classes only. It must not mutate accounts, shells, authorized_keys, Tailscale, firewall, services, qsl-server, qsl-attachments, source, scripts, workflows, dependencies, backup state, public-site content, or Cloudflare configuration.

## Required-Check Boundary

The implementation PR must keep public-safety and advisories green, preserve no failed required checks, and keep branch-protection required contexts classified green or conclusively satisfied.

## Source / Script Mutation Boundary

No source file or repository script is changed by NA-0561. Future NA-0562 remote probe scripts must remain proof-root-only and must not be committed.

## Workflow Mutation Boundary

No workflow was dispatched, rerun, canceled, deleted, or mutated.

## Runtime / qsc / Dependency Boundary

No qsc send/receive occurred. No qsc E2EE occurred. No qsc source, tests, fuzz files, Cargo manifests, dependencies, or lockfiles were mutated.

## qsl-server / qsl-attachments Boundary

No qsl-server or qsl-attachments command, clone, build, run, local use, or mutation occurred.

## Remote-Action Boundary

No probe, SSH command, Tailscale command, remote command, service command, sudo command, systemctl command, account mutation, shell mutation, authorized_keys mutation, firewall mutation, service mutation, or remote file mutation occurred in NA-0561.

## Public-Site / Cloudflare Boundary

No README public-progress, docs/public, public-site, website path, public path, Cloudflare configuration, deployment setting, or public content mutation occurred.

## Claim Boundary

No public-readiness claim was made. No production-readiness claim was made. No public-internet-readiness claim was made. No external-review-complete claim was made. No backup/restore-complete claim was made. No vulnerability-free claim was made. No bug-free claim was made. No perfect-build or perfect-crypto claim was made.

## Validation

Validation is recorded in proof-root-only artifacts and summarized by `tests/NA-0561_remote_relay_service_owner_non_secret_proof_authorization_testplan.md`.

Focused qsc runtime tests are skipped because this is an authorization-only governance/evidence/testplan change with no qsc source/runtime/dependency/workflow mutation and no local qsc execution authorized.

## Recommendation

Merge NA-0561 only after required checks remain green. Then close out NA-0561 to restore NA-0562 only if post-merge public-safety and advisories are green, no failed required check remains, and the exact NA-0562 successor block is restored without placeholders.
