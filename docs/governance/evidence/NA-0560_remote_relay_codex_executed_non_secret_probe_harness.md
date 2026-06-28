Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-06-28

# NA-0560 Remote Relay Codex-Executed Non-Secret Probe Harness

## Executive Summary

NA-0560 executed the D-1108/D-1109-authorized Codex-run non-secret relay probe harness. The probe result is `CODEX_PROBE_SSH_TUNNEL_RELAY_UNREACHABLE_SERVICE_OWNER_PROOF_REQUIRED`.

The discriminating result is that SSH command execution succeeded and the SSH local port-forward setup succeeded, but the forwarded relay TCP probe was refused before any HEAD response. The selected successor is `NA-0561 -- QSL Remote Relay Service Owner Non-Secret Proof Authorization Plan`.

## qwork Proof Verification

Fresh qwork proof files from `2026-06-28T20:41:09Z` or later were copied proof-root-only and verified before probe execution or repository mutation.

- lane: `NA-0560`
- repo: `qsl-protocol`
- startup result: `OK`
- repo result: `OK`
- qwork HEAD: `e74f9c211690`
- qwork origin/main: `e74f9c211690`
- qwork main: `e74f9c211690`
- worktree/index/untracked proof: clean
- READY count: 1
- queue top READY: `NA-0560`
- shared cargo target proof: ready

Live pre-fetch HEAD and origin/main matched the qwork proof. Disk proof remained below the stop threshold and `/backup/qsl` was mounted.

## D-1108 / D-1109 Inheritance

D-1108 and D-1109 were each present once and Accepted. NA-0559 was DONE. NA-0560 was READY. No NA-0560 implementation evidence or testplan existed before this directive.

D-1108/D-1109 authorized Codex-executed, non-secret, non-mutating probes and required fail-closed private-material handling. NA-0560 consumed the prior NA-0555 diagnostics that remote-handshake and remote-relay had reached relay push and failed at the `relay_inbox_push_failed` timeout boundary without HTTP status/body proof.

## Operator-Supplied SSH Tunnel Context

The operator-supplied `inspiron` and `qslcodex` context was consumed only as non-secret access context. Raw authorized_keys content, public key material, private key material, endpoint values, private topology, route-token/capability values, bearer values, Authorization headers, payloads, and response bodies were not published.

SSH command execution and SSH port-forward access were classified separately.

## Current Main Required-Check Classification

Current main was verified at `e74f9c211690` and equals origin/main. `origin/main` descends from the expected D479/D-1109 closeout merge.

- public-safety: completed success
- advisories: completed success
- suite2-vectors: completed success
- no failed required checks: yes
- no pending required checks: yes
- branch-protection required contexts: green on the merge commit or conclusively satisfied by PR #1392 head checks that merged into `e74f9c211690`

## Probe Script Design and Static Review

Two proof-root-only Python scripts were generated:

- `redacted_relay_probe.py`
- `forwarded_relay_probe.py`

Static review passed before execution:

- Python syntax check passed.
- Output is JSON only.
- No subprocess import.
- No file mutation calls.
- No qsc execution.
- No POST or payload send.
- HEAD-only `/v1/push` behavior where applicable.
- No auth headers.
- No response body read or printed.
- No hardcoded endpoint, token, bearer, Authorization, public-key, authorized_keys, or private-key material.

## Local qbuild Host Probe

Local qbuild host probe classification:

- local result: `LOCAL_PROBE_ENDPOINT_ABSENT_OR_UNKNOWN`
- endpoint configured: no
- bearer secret candidate present: no
- route secret candidate present: no
- private-material scan: pass

## SSH Command Execution Reachability to inspiron

SSH command-execution reachability classification:

- result: `SSH_COMMAND_EXECUTION_SUCCESS`
- private-material scan: pass

The exact authorized command exited 0. The output contained the ready marker with the remote shell's literal `n` behavior from the unquoted command form; no rerun was performed.

## SSH Port-Forward Tunnel to inspiron

SSH local port-forward setup classification:

- result: `SSH_PORT_FORWARD_SETUP_SUCCESS`
- private-material scan: pass

No remote shell command was executed for the tunnel setup and no remote state was mutated.

## Forwarded Relay TCP / HEAD Probe

Forwarded relay probe classification:

- result: `SSH_PORT_FORWARD_RELAY_TCP_REFUSED`
- TCP class: refused
- HEAD class: not checked
- response body disclosed: no
- private-material scan: pass

Because TCP was refused, no HEAD response was observed.

## Remote inspiron Probe

Remote redacted probe classification:

- result: `REMOTE_PROBE_SECRET_PRESENCE_CLASSIFIED`
- endpoint configured: no
- bearer secret candidate present: no
- route secret candidate present: no
- DNS/TCP/TLS/HEAD classes: not checked because no endpoint was available in the non-interactive SSH environment
- private-material scan: pass

The remote script was provided over SSH standard input only. No file was written to the remote host.

## GitHub Metadata Review

Read-only GitHub metadata review used current main check-runs, branch-protection required status checks, and merged PR #1392 head check-runs. No repository secret values were accessed. No repository variable values were printed. No GitHub mutation was performed.

## Private-Material Review

All probe and SSH raw captures remained proof-root-only. Scans passed for:

- local probe output
- SSH command reachability output
- SSH port-forward setup output
- forwarded probe output
- SSH tunnel cleanup output
- remote probe output

No endpoint values, route-token/capability values, bearer values, Authorization headers, private topology, payloads, response bodies, authorized_keys content, public key material, private key material, secret environment values, Cloudflare tokens, or API keys were published.

## Root-Cause Classification

Selected classification: `CODEX_PROBE_SSH_TUNNEL_RELAY_UNREACHABLE_SERVICE_OWNER_PROOF_REQUIRED`.

Rationale: SSH command execution worked and SSH local port-forward setup worked, but the forwarded relay TCP probe was refused before any HEAD response. That points the next non-secret discriminator at service owner/deployer proof for listener/service availability and path/method behavior, not Codex account repair or qsc runtime review.

## Selected Successor

Selected successor: `NA-0561 -- QSL Remote Relay Service Owner Non-Secret Proof Authorization Plan`.

The successor is authorization-only. It must define exact non-secret service-owner proof fields for service availability, deployment, loopback listener, path/method class, and health proof without publishing private topology, private endpoints, tokens, bearer material, Authorization headers, payloads, response bodies, or secret values.

## Required-Check Boundary

The implementation PR must keep required checks green. The pre-probe current-main check proof classified public-safety and advisories as green and no failed required checks as present.

## Source / Script Mutation Boundary

No repository source file was changed. No repository script was changed. The only generated scripts were proof-root-only probe scripts, and they are not committed.

## Workflow Mutation Boundary

No workflow was dispatched, rerun, canceled, deleted, or mutated.

## Runtime / qsc / Dependency Boundary

No qsc send/receive occurred. No qsc E2EE occurred. No qsc source, qsc tests, qsc fuzz files, dependencies, Cargo manifests, or lockfiles were mutated.

## qsl-server / qsl-attachments Boundary

No qsl-server or qsl-attachments command, clone, build, run, local use, or mutation occurred.

## Remote-Action Boundary

Remote action was limited to the exact authorized SSH command-execution check, exact authorized SSH local port-forward setup/cleanup, and the exact authorized remote stdin Python probe after command execution succeeded. No account, shell, authorized_keys, sudo, Tailscale, firewall, service, file, or environment mutation occurred.

## Public-Site / Cloudflare Boundary

No README public-progress, public-site, docs/public, website, public path, Cloudflare configuration, deployment setting, or public content mutation occurred.

## Raw Output Boundary

Raw stdout/stderr captures remain proof-root-only. Repository docs include only coarse classifications and private-material scan outcomes.

## Claim Boundary

No public-readiness claim was made. No production-readiness claim was made. No public-internet-readiness claim was made. No external-review-complete claim was made. No backup/restore-complete claim was made. No vulnerability-free claim was made. No bug-free claim was made. No perfect-build or perfect-crypto claim was made.

## Validation

Validation is recorded in proof-root-only artifacts and summarized by `tests/NA-0560_remote_relay_codex_executed_non_secret_probe_testplan.md`.

Focused qsc runtime tests are skipped for this governance-only change because no qsc source/runtime/dependency/workflow mutation occurred and qsc local send/receive was not authorized.

## Recommendation

Proceed to the selected NA-0561 service-owner non-secret proof authorization lane after implementation merge and closeout. The next lane should request only coarse, non-secret proof of remote relay service availability, listener class, deployment status, path/method class, and timeout class, with the same private-material publication boundary.
