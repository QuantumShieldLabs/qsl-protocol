Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-30

# NA-0575 Remote qsl-server Start / Bind Operator Proof Capture Harness

## Executive Summary

NA-0575 consumed the D500 stop, D-1138, D-1139, and the D498/D499 inheritance trail, then used the D501 bounded Codex operational authority pivot to inspect the operator-owned `inspiron` test workspace. The inspection was redacted-by-construction and published only coarse classes.

Result classification: `QSL_SERVER_BIND_START_INSUFFICIENT_PROOF`.

Selected successor: `NA-0576 -- QSL Remote qsl-server Start / Bind Proof Completion Authorization Plan`.

## qwork Proof Verification

Fresh qwork proof files were copied from the NA-0575 lane workspace before fetch, remote action, or repository mutation. Required qwork values passed: `startup_result=OK`, lane `NA-0575`, repo `qsl-protocol`, branch `main`, upstream `origin/main`, clean worktree/index/untracked state, READY count 1, queue top READY `NA-0575`, requested lane status READY, shared cargo target mode, and shared target ready.

Live pre-fetch `HEAD` and `origin/main` both matched `f159415ffc8a`. Codex did not run `qwork`, `qstart`, or `qresume`.

## D500 / D499 / D498 Inheritance

D500 stopped only because no operator proof package existed. D500 did not mutate the repository, did not create D-1140, did not run remote action, and did not start qsl-server.

D499 and D-1138 selected the NA-0575 start/bind proof model. D-1139 restored NA-0575 as the sole READY item. D498 proved qsl-server was staged under the qslcodex test workspace, qsl-server start was not attempted, and the inherited postcheck classified binary ready with listener not ready.

## Bounded Codex Operational Authority Model

D501 records the reusable authority model:

- Tier 0 is governance/read-only default authority.
- Tier 1 permits redacted diagnostics on named test hosts/workspaces with raw output proof-root-only.
- Tier 2 permits bounded no-secret, non-root, reversible test action in named workspaces after preflight and postcheck.
- Tier 3 remains operator/admin action for sudo, systemd, firewall, Tailscale, account, shell, authorized_keys, root-owned service, and backup actions unless a later directive explicitly authorizes a privileged lane.
- Tier 4 forbids secret publication, destructive unbounded mutation, workflow weakening, out-of-scope protocol/crypto/security semantic changes, and public/production/security overclaims.

For NA-0575, D501 applied Tier 1 and, only if gates passed, Tier 2 to host label `inspiron` and workspace `/home/qslcodex/qsl-remote-test/`. The gate did not pass for bounded start.

## Current Main Required-Check Classification

Current main `f159415ffc8a` satisfied public-safety, advisories, and suite2-vectors. Required branch-protection contexts had no failed or pending required checks after D498 visibility recovery for `goal-lint` and `CodeQL`.

Non-required remote demo check-runs remained failed and were retained as evidence of the remote-relay blocker. Root cargo audit, nested qsc fuzz cargo audit, and Cargo manifest/lock drift checks passed.

## Remote Script Design and Static Review

Proof-root-only scripts were generated for inspection, bounded start, postcheck, and cleanup. Static review passed after correcting a reviewer false positive that matched directive-required field names instead of actual forbidden commands.

The final scripts used Python stdlib only, no `shell=True`, no sudo/systemctl/service/Tailscale/firewall/account/authorized_keys access, no qsc send/receive, no qsl-attachments, no endpoint/token/body/topology/process identity publication, and no raw private port publication.

## SSH Readiness

The single authorized SSH readiness command executed once and exited 0. The stdout marker used the known directive-form literal trailing `n` shape; it was classified as `SSH_QSL_SERVER_START_BIND_READY` without rerun. Private-material scan passed.

## Remote Start / Bind Inspection

The single authorized inspection command ran through SSH stdin and returned JSON only. Classification:

`QSL_SERVER_START_BIND_INSPECTION_INSUFFICIENT`

Key safe classes:

- staged binary present/executable class: present/executable
- bind target available class: unavailable
- listener already ready class: not checked because bind was unavailable
- Codex bounded start safe: no
- redaction review: passed by construction

No endpoint value, private port value, topology, route-token/capability value, bearer/Authorization value, process identity, payload, response body, authorized_keys content, or key material was published.

## Bounded Start

Bounded start was skipped because inspection did not classify `QSL_SERVER_START_BIND_INSPECTION_CODEX_START_SAFE`.

Classification: `QSL_SERVER_BOUNDED_START_SKIPPED`.

## Bounded Postcheck

Bounded postcheck was skipped because bounded start did not run and inspection did not classify listener already ready.

Classification: `QSL_SERVER_BOUNDED_POSTCHECK_SKIPPED`.

## Cleanup / Rollback

Cleanup was not needed because no NA-0575-owned process was started.

Classification: `QSL_SERVER_BOUNDED_CLEANUP_NOT_NEEDED`.

## Private-Material Review

Private-material scans passed for SSH readiness output, remote inspection output, skipped start/postcheck/cleanup artifacts, generated scripts, parsed JSON, and governance publication text.

No endpoint value, private port value, route-token/capability value, bearer value, Authorization header, private topology, process identity, payload, response body, authorized_keys content, public/private key material, secret env value, Cloudflare/API token, or long opaque token string is published.

## Result Classification

`QSL_SERVER_BIND_START_INSUFFICIENT_PROOF`

The redacted inspection did not prove a safe bind target or complete start command path, so qsl-server start remained fail-closed.

## Selected Successor

Selected successor:

`NA-0576 -- QSL Remote qsl-server Start / Bind Proof Completion Authorization Plan`

Objective: authorize the minimum additional non-secret proof needed after Codex-executed inspection preserved unknown/not_checked values and could not safely authorize start.

## Required-Check Boundary

Required-check handling was read-only. No workflow dispatch or rerun occurred. Failed non-required remote checks were not reclassified as required-green evidence.

## Source / Script Mutation Boundary

No qsl-protocol source, repository script, workflow, dependency, lockfile, qsc runtime, qsl-server source, or qsl-attachments source mutation occurred.

## Workflow Mutation Boundary

No workflow file changed. No workflow dispatch or rerun was executed.

## Runtime / qsc Boundary

No qsc command was run. No qsc send/receive or E2EE action occurred.

## qsl-server / qsl-attachments Boundary

qsl-server source was not mutated and no qsl-server PR was opened. qsl-server was inspected only as a staged binary under the approved test workspace. qsl-server was not started. qsl-attachments was not run, cloned, built, or mutated.

## Remote-Action Boundary

Remote actions were limited to the exact D501 allowlist: one SSH readiness command and one redacted inspection command through SSH stdin. No scp, sudo, systemctl, service, journalctl, ps, ss, netstat, lsof, Tailscale, firewall, account, shell, authorized_keys, root-owned path, qsl-backup, or backup mutation occurred.

## Public-Site / Cloudflare Boundary

No public-site or Cloudflare mutation occurred.

## Raw Output Boundary

Raw SSH output, raw remote JSON, generated scripts, and local proof artifacts remain proof-root-only. Repository docs publish only coarse classifications and safe labels.

## Claim Boundary

This evidence makes no public-readiness claim. It makes no production-readiness claim. It makes no public-internet-readiness claim. It makes no external-review-complete claim. It makes no vulnerability-free claim. It makes no bug-free claim. It makes no perfect-build claim. It makes no perfect-crypto claim.

## Validation

Validation bundle: qwork proof, queue/decision proof, current-main required-check proof, inheritance review, authority model record, remote script static review, SSH readiness/private-material scan, remote inspection/private-material scan, start/postcheck/cleanup skip proof, aggregate private-material review, and result/successor classification.

Post-fix hardening review:

1. Correctness under stress: the scripts fail closed when the bind target is unavailable or unknown.
2. Minimality: repository changes are governance/testplan/journal only and remote action stayed within the exact allowlist.
3. Maintainability: the authority model and successor selection are recorded in D-1140 and traceability.
4. Coverage quality: required markers cover proof gates, redaction, skip boundaries, and one-READY discipline.
5. Cross-lane stability: no qsc runtime, qsl-server source, workflow, dependency, or macOS/Linux behavior was changed.

## Recommendation

Proceed through NA-0576 to authorize the minimum additional non-secret proof needed to identify a safe private loopback bind/start path, without publishing bind values or widening remote authority.
