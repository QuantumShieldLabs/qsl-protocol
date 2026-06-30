Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-30

# NA-0578 qsl-server Failed Start Cause Classification and Corrected Loopback Proof Harness

## Executive Summary

NA-0578 consumed D-1145 and D-1146, verified fresh qwork proof from `2026-06-30T23:09:08Z`, verified current main at `75cd8e8b572c`, and applied the D-1142/D-1143 bounded Codex operational authority model.

The NA-0577 failed start cause was confirmed as a generated harness bind-argument bug. The NA-0577 script selected qsl-server's bind flag as a generic full-address flag, while qsl-server source/help define bind as host/address only with a separate port flag and compose the socket address internally.

Remote follow-up inspection classified the corrected temporary loopback smoke start as safe and confirmed the prior failed-start class. The corrected bounded start command was executed once through the authorized SSH stdin path, but the generated corrected start script stopped before attempting a process due to a harness gating defect. No listener or route-shape proof was produced, postcheck was skipped, and cleanup was not needed.

Result classification: `QSL_SERVER_FAILED_START_INSUFFICIENT_PROOF`.

Selected successor: `NA-0579 -- QSL Remote qsl-server Failed Start Proof Completion Follow-Up Harness`.

## qwork Proof Verification

- qwork proof files were copied before fetch, repository mutation, or remote action.
- Required qwork values matched lane `NA-0578`, repo `qsl-protocol`, path `/srv/qbuild/work/NA-0578/qsl-protocol`, clean worktree/index/untracked state, READY_COUNT 1, queue top READY `NA-0578`, shared cargo target mode, and shared target ready.
- qwork proof timestamp was verified at or after `2026-06-30T23:09:08Z`.
- Live pre-fetch `HEAD` and `origin/main` matched qwork proof at `75cd8e8b572c`.
- Codex did not run `qwork`, `qstart`, or `qresume`.

## D-1145 / D-1146 Inheritance

- D-1145 exists once and is Accepted.
- D-1146 exists once and is Accepted.
- D-1145 result classification was `QSL_SERVER_START_BIND_INSUFFICIENT_PROOF`.
- NA-0577 bounded start was attempted and failed.
- NA-0577 bounded postcheck was skipped.
- NA-0577 cleanup completed.
- NA-0577 is DONE and NA-0578 is the sole READY item.
- D-1147 was absent before this implementation patch.

## Authority Model Application

- Tier 1 redacted diagnostics were limited to host label `inspiron` and `/home/qslcodex/qsl-remote-test/`.
- Tier 2 corrected temporary loopback smoke action was allowed only after safety gates.
- Tier 3 operator/admin action remained forbidden.
- Tier 4 forbidden action remained forbidden.
- Continuous CI wait-work and read-only forward-audit policy applies to PR and post-merge waits.

## Current Main Required-Check Classification

- Current main: `75cd8e8b572c`.
- public-safety: completed success.
- advisories: completed success.
- suite2-vectors: success or conclusively satisfied by the visible required-check boundary.
- No failed or pending visible check-runs/statuses were classified.
- Root cargo audit: success.
- Nested qsc fuzz cargo audit: success.
- `cargo metadata --locked --format-version=1`: success.
- Cargo manifest/lock drift: absent.

## NA-0577 Failed Start Review

- NA-0577 generated scripts were reviewed proof-root-only.
- The inspection script mapped qsl-server bind support to a generic full-address style.
- The bounded start script then used a loopback host-plus-selected-port value as the bind flag value.
- This confirmed the generated harness shape mismatch for qsl-server's actual CLI.
- No raw stderr, command line, selected port, bind value, or process identity is published.

## qsl-server CLI Source / Help Review

- qsl-server source was acquired proof-root-only from the public source repository and reviewed without upstream mutation.
- Source/help confirm bind is host/address only and port is separate.
- Source composes the socket address internally from bind plus port before parsing.
- Source emits redacted bind parse/listen error classes on startup failure.
- README/source confirm relay bearer auth is optional for startup when the bearer environment variable is absent or empty.
- Route use still requires the route-token header; NA-0578 did not publish or use any private route token.

## Failure-Cause Classification

Classification: `QSL_SERVER_FAILED_START_CAUSE_HARNESS_BIND_ARG_BUG_CONFIRMED`.

The prior failed-start child class was `ERR_BIND_PARSE`. The likely invalid composed shape came from the generated harness passing a host-plus-port value as the bind argument while qsl-server separately appended its configured/default port.

## Remote Script Design and Static Review

- Four proof-root-only Python stdlib scripts were generated for follow-up inspection, corrected start, postcheck, and cleanup.
- Static review passed for syntax, JSON-only stdout, `shell=False`, workspace-bounded writes, no forbidden subprocess command use, no qsc send/receive, no qsl-attachments, no secret-file or authorized_keys access, and corrected host-only bind plus separate port flag shape.
- The later corrected start result exposed a logic defect not caught by static review: the start script read an inspection-summary field before the inspection script's persisted summary reflected the safe classification.

## SSH Readiness

Classification: `SSH_QSL_SERVER_FAILED_START_FOLLOWUP_READY`.

The authorized SSH readiness command executed exactly once and private-material scan passed.

## Remote Follow-Up Inspection

Classification: `QSL_SERVER_FAILED_START_FOLLOWUP_CORRECTED_START_SAFE`.

Remote inspection classified staged binary present/executable, help/version available, bind host-only, separate port available, corrected temporary loopback smoke safe, secret-free start, non-privileged start, loopback-only start, no sudo/systemd, no Tailscale/firewall, no account/authorized_keys action, no secret/endpoint action, and prior failed-start cause confirmed.

## Corrected Bounded Start

Classification: `QSL_SERVER_CORRECTED_BOUNDED_START_AMBIGUOUS_STOP`.

The authorized corrected bounded start command executed exactly once. The script returned before attempting start, wrote no start manifest, and reported cleanup not required. This is treated as insufficient proof for NA-0578, not as qsl-server listener failure evidence.

## Corrected Bounded Postcheck

Classification: `QSL_SERVER_CORRECTED_POSTCHECK_LISTENER_NOT_READY`.

Postcheck was skipped because the corrected start did not attempt a process and did not produce listener proof.

## Cleanup / Rollback

Classification: `QSL_SERVER_CORRECTED_CLEANUP_NOT_NEEDED`.

Cleanup was not run because the corrected start wrote no NA-0578-owned start manifest and reported no cleanup-required state.

## Private-Material Review

- Aggregate private-material review passed.
- No endpoint value was published.
- No private port value was published.
- No route-token/capability value was published.
- No bearer value or Authorization value was published.
- No private topology was published.
- No process identity was published.
- No command line was published.
- No payload or response body was published.
- No authorized_keys content or key material was published.

## Result Classification

`QSL_SERVER_FAILED_START_INSUFFICIENT_PROOF`

NA-0578 confirmed the NA-0577 failed-start cause, but did not complete corrected listener or route-shape proof.

## Selected Successor

`NA-0579 -- QSL Remote qsl-server Failed Start Proof Completion Follow-Up Harness`

## Required-Check Boundary

Required-check handling was read-only. No workflow dispatch or rerun occurred. No failed required check was classified.

## Source / Script Mutation Boundary

No qsl-protocol source, repository script, workflow, dependency, lockfile, qsc runtime, qsl-server source, qsl-server PR, or qsl-attachments source mutation occurred.

## Workflow Mutation Boundary

No workflow file changed. No workflow dispatch or rerun occurred.

## Runtime / qsc Boundary

No qsc command was run. No qsc send/receive or E2EE action occurred.

## qsl-server / qsl-attachments Boundary

qsl-server source was reviewed proof-root-only and was not mutated. qsl-server was not deployed. qsl-attachments was not run, cloned, built, or mutated.

## Remote-Action Boundary

Remote action was limited to the authorized SSH readiness, follow-up inspection, and corrected bounded start commands. No scp, sudo, systemctl, service, firewall, Tailscale, account, shell, authorized_keys, qsl-backup, or root-owned path action occurred.

## Public-Site / Cloudflare Boundary

No public-site or Cloudflare mutation occurred.

## Raw Output Boundary

Raw outputs, generated scripts, qsl-server source clone, qsl-server help/version raw output, and parsed JSON remain proof-root-only. Tracked repository evidence contains only coarse classifications and safe boundary statements.

## Claim Boundary

No public-readiness claim is made. No production-readiness claim is made. No public-internet-readiness claim is made. No external-review-complete claim is made. No vulnerability-free claim is made. No bug-free claim is made. No perfect-build claim is made. No perfect-crypto claim is made.

## Validation

Validation includes qwork proof, queue/decision proof, current-main checks, D-1145/D-1146 inheritance, failure-cause review, qsl-server CLI source/help review, authority confirmation, remote script static review, SSH readiness scan, remote inspection scan, corrected start scan, postcheck skip classification, cleanup not-needed classification, aggregate private-material review, result/successor classification, scope guard, marker proof, link-check, private-material scan, overclaim scan, PR body preflight, goal-lint, cargo audits, locked metadata, formatting, and shell syntax checks.

## Recommendation

Proceed to NA-0579 as a narrow proof-completion follow-up. It should preserve the confirmed bind/port failure-cause evidence, fix the corrected start gating defect, and produce corrected listener/postcheck/cleanup proof without publishing private bind, endpoint, topology, token, payload, body, process identity, command line, or key material.
