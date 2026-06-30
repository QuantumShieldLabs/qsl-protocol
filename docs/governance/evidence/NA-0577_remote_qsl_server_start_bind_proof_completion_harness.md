Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-30

# NA-0577 Remote qsl-server Start / Bind Proof Completion Harness

## Executive Summary

NA-0577 consumed the D-1142/D-1143 bounded Codex operational authority model and the D-1144 NA-0577 restoration. Codex verified fresh qwork proof, current main health, queue state, and inherited D501 start/bind insufficiency before any remote action.

Codex then ran only the directive-authorized SSH readiness, remote inspection, bounded start, and cleanup commands against the approved `inspiron` / `/home/qslcodex/qsl-remote-test/` workspace. The remote inspection classified temporary loopback smoke start as safe, but the bounded start failed before any successful listener/postcheck proof. Cleanup completed for NA-0577-owned state.

Result classification: `QSL_SERVER_START_BIND_INSUFFICIENT_PROOF`.

Selected successor: `NA-0578 -- QSL Remote qsl-server Start / Bind Proof Completion Follow-Up Harness`.

## qwork Proof Verification

- Fresh qwork proof files were copied before fetch, remote action, or repository mutation.
- Required qwork values matched: `startup_result=OK`, lane `NA-0577`, repo `qsl-protocol`, clean worktree/index/untracked state, READY_COUNT 1, queue top READY `NA-0577`, shared cargo target mode, and shared target ready.
- qwork proof timestamp was verified at or after `2026-06-30T22:01:51Z`.
- Live pre-fetch `HEAD` and `origin/main` matched qwork proof at `bff5d8379100`.
- Codex did not run `qwork`, `qstart`, or `qresume`.

## D-1142 / D-1143 / D-1144 Inheritance

- D-1142 exists once and is Accepted.
- D-1143 exists once and is Accepted.
- D-1144 exists once and is Accepted.
- D-1142 defines the bounded Codex operational authority model.
- D-1143 defines continuous CI wait-work and read-only forward-audit policy.
- D-1144 restored NA-0577 as the sole READY successor.
- D501 result was `QSL_SERVER_BIND_START_INSUFFICIENT_PROOF`.
- D501 did not start qsl-server.

## Authority Model Application

- Tier 1 redacted diagnostics were authorized for host label `inspiron` and workspace `/home/qslcodex/qsl-remote-test/`.
- Tier 2 bounded test action was authorized only after inspection proved no-secret, non-root, loopback-only, non-sudo, non-systemd, no-firewall, no-Tailscale, no-account/authorized_keys mutation, no qsc send/receive, no workflow dispatch/rerun, no qsl-attachments, no private publication, and workspace-only write gates.
- Tier 3 operator/admin action remained forbidden.
- Tier 4 forbidden boundaries remained forbidden.
- Continuous CI wait-work policy applies to this lane and later waits.

## Current Main Required-Check Classification

- Current main: `bff5d8379100`.
- public-safety: completed success.
- advisories: completed success.
- Visible check-runs: no failed and no pending.
- Commit statuses: no failed and no pending.
- Root cargo audit: success.
- Nested qsc fuzz cargo audit: success.
- `cargo metadata --locked --format-version=1`: success.
- Cargo manifest/lock drift: absent.

## Remote Script Design and Static Review

- Generated proof-root-only scripts for inspection, bounded start, bounded postcheck, and cleanup.
- Static review passed for syntax, JSON-only stdout shape, absence of `shell=True`, absence of forbidden command invocations, no qsc send/receive, no qsl-attachments, and workspace-bound write design.
- Recovered static-review false positives were recorded for required field-name text and HTTP route literals used only as no-secret route-shape probes.

## SSH Readiness

- SSH readiness command executed exactly once.
- Classification: `SSH_QSL_SERVER_START_BIND_COMPLETION_READY`.
- The known directive-form literal trailing marker shape was classified without rerunning SSH.
- Private-material scan: PASS.

## Remote Start / Bind Completion Inspection

- Remote inspection command executed exactly once through SSH stdin.
- Classification: `QSL_SERVER_START_BIND_COMPLETION_TEMP_LOOPBACK_SMOKE_SAFE`.
- Safe gate classes included staged binary executable, explicit loopback bind support, no-secret start command, non-privileged start command, loopback-only start command, no sudo/systemd, no Tailscale/firewall, no account/authorized_keys action, and no secret/endpoint action required.
- Private-material scan: PASS after allowing the directive-required safe field name `account_or_authorized_keys_required_class`.

## Bounded Start

- Bounded start command executed exactly once through SSH stdin.
- Classification: `QSL_SERVER_BOUNDED_START_FAILED`.
- Start mode class: temporary loopback smoke.
- Start was attempted, but no successful started-process class was published.
- A NA-0577-owned manifest was written under the approved qslcodex workspace and cleanup was required.
- Private-material scan: PASS.

## Bounded Postcheck

- Postcheck was skipped because bounded start failed and inspection did not classify an already-ready listener.
- Classification: `QSL_SERVER_BOUNDED_POSTCHECK_SKIPPED`.
- No qsc send/receive, payload, Authorization, bearer, route-token, or response-body action occurred.

## Cleanup / Rollback

- Cleanup command executed exactly once through SSH stdin because the start script marked NA-0577-owned cleanup required.
- Classification: `QSL_SERVER_BOUNDED_CLEANUP_DONE`.
- Cleanup affected only NA-0577-owned state.
- Private-material scan: PASS.

## Private-Material Review

- Raw SSH stdout/stderr, remote JSON, generated scripts, classifications, and tracked evidence were scanned.
- No endpoint value was published.
- No private port value was published.
- No route-token/capability value was published.
- No bearer value or Authorization header was published.
- No private topology was published.
- No process identity was published.
- No payload or response body was published.
- No authorized_keys content or key material was published.

## Result Classification

`QSL_SERVER_START_BIND_INSUFFICIENT_PROOF`

Reason: inspection proved a temporary loopback smoke attempt was safe, but bounded start failed, no listener/postcheck proof was produced, and cleanup completed.

## Selected Successor

`NA-0578 -- QSL Remote qsl-server Start / Bind Proof Completion Follow-Up Harness`

## Required-Check Boundary

Required-check handling was read-only. No workflow dispatch or rerun occurred. No failed required check was classified.

## Source / Script Mutation Boundary

No qsl-protocol source, repository script, workflow, dependency, lockfile, qsc runtime, qsl-server source, or qsl-attachments source mutation occurred.

## Workflow Mutation Boundary

No workflow file changed. No workflow dispatch or rerun occurred.

## Runtime / qsc Boundary

No qsc command was run. No qsc send/receive or E2EE action occurred.

## qsl-server / qsl-attachments Boundary

qsl-server source was not mutated and no qsl-server PR was opened. qsl-server was exercised only as a staged binary under the approved test workspace. qsl-attachments was not run, cloned, built, or mutated.

## Remote-Action Boundary

Remote action was limited to the exact authorized SSH readiness, inspection, bounded start, and cleanup commands. No scp, sudo, systemctl, service, firewall, Tailscale, account, shell, authorized_keys, qsl-backup, or root-owned path action occurred.

## Public-Site / Cloudflare Boundary

No public-site or Cloudflare mutation occurred.

## Raw Output Boundary

Raw outputs remain proof-root-only under the NA-0577 proof root. Tracked repository evidence contains only coarse classifications and safe boundary statements.

## Claim Boundary

No public-readiness claim is made. No production-readiness claim is made. No public-internet-readiness claim is made. No external-review-complete claim is made. No vulnerability-free claim is made. No bug-free claim is made. No perfect-build claim is made. No perfect-crypto claim is made.

## Validation

Validation includes qwork proof, queue/decision proof, current-main checks, inherited decision review, authority confirmation, remote script static review, SSH readiness scan, remote inspection scan, bounded start scan, cleanup scan, result/successor classification, scope guard, marker proof, link check, private-material scan, overclaim scan, PR body preflight, goal-lint, cargo audits, locked metadata, formatting, and shell syntax checks.

## Recommendation

Proceed with the selected NA-0578 follow-up harness. The follow-up should preserve the same private-material boundaries and focus on why the bounded temporary start did not produce listener proof, without publishing bind, endpoint, topology, token, payload, body, process identity, or key material.
