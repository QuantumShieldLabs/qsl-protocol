Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-01

# NA-0579 qsl-server Failed Start Proof Completion Follow-Up Harness

## Executive Summary

NA-0579 consumed D-1147 and D-1148, verified fresh qwork proof from `2026-07-01T00:07:58Z`, verified current main at `dbaa462d6021`, and applied the D-1142/D-1143 bounded Codex operational authority model.

The NA-0578 proof-harness gating defect was classified as `QSL_SERVER_NA0578_GATING_DEFECT_INSPECTION_STATE_NOT_PERSISTED`: the inspection stdout reported corrected-start-safe, but the persisted manifest consumed by the start script was written before that safe classification was assigned. NA-0579 added a proof-root local dry-run assertion that proves inspection-safe state is visible to the corrected start script before any remote start attempt.

Remote inspection classified corrected start safe. Corrected bounded start created a temporary loopback smoke listener. No-secret/no-body postcheck proved route shape. Cleanup completed for NA-0579-owned temporary smoke state.

Result classification: `QSL_SERVER_PROOF_COMPLETION_TEMP_LOOPBACK_ROUTE_SHAPE_PASS_EXPECTED_BIND_REQUIRED`.

Selected successor: `NA-0580 -- QSL Remote qsl-server Expected Bind / Endpoint Alignment Harness`.

## qwork Proof Verification

- qwork proof files were copied before fetch, repository mutation, SSH, remote script generation, or proof publication.
- Required qwork values matched lane `NA-0579`, repo `qsl-protocol`, path `/srv/qbuild/work/NA-0579/qsl-protocol`, clean worktree/index/untracked state, READY_COUNT 1, queue top READY `NA-0579`, shared cargo target mode, and shared target ready.
- qwork proof timestamp was verified at or after `2026-07-01T00:07:58Z`.
- Live pre-fetch `HEAD` and `origin/main` matched qwork proof at `dbaa462d602`.
- Codex did not run `qwork`, `qstart`, or `qresume`.

## D-1147 / D-1148 Inheritance

- D-1147 exists once and is Accepted.
- D-1148 exists once and is Accepted.
- NA-0578 is DONE.
- NA-0579 was the sole READY item before implementation.
- D-1147 result classification was `QSL_SERVER_FAILED_START_INSUFFICIENT_PROOF`.
- D-1147 confirmed the original bind/port command-shape harness bug.
- D-1147 remote inspection classified corrected start safe.
- D-1147 corrected bounded start classified ambiguous stop due to a generated harness gating defect.
- D-1148 restored NA-0579.
- D-1149 was absent before this implementation patch.

## Authority Model Application

- Tier 1 redacted diagnostics were limited to host label `inspiron` and `/home/qslcodex/qsl-remote-test/`.
- Tier 2 corrected temporary loopback smoke action was used only after qwork, current-main, inheritance, failure-cause, local dry-run, source-review, static-review, SSH readiness, and remote-inspection gates passed.
- Tier 3 operator/admin action remained forbidden.
- Tier 4 forbidden action remained forbidden.
- Continuous CI wait-work and read-only forward-audit policy applies to PR and post-merge waits.

## Automatic Failure-Cause Investigation Policy

NA-0579 applied the automatic failure-cause investigation policy required by the directive. The lane inspected generated proof-root harnesses, compared harness assumptions against qsl-server source/help evidence, classified the likely failure cause, fixed proof-root-generated harness defects within authority, retried only after safety gates, and recorded the issue as a harness handoff bug rather than qsl-server runtime evidence.

Operational recommendation: a later governance lane should make this durable in `START_HERE.md`, `AGENTS.md`, and `docs/ops/CODEX_BOUNDED_OPERATIONAL_AUTHORITY.md`.

## Current Main Required-Check Classification

- Current main: `dbaa462d602`.
- public-safety: completed success.
- advisories: completed success.
- suite2-vectors: completed success.
- No failed required check was classified.
- No required pending check remained after D498-style visibility recovery for goal-lint and CodeQL.
- Root cargo audit: success.
- Nested qsc fuzz cargo audit: success.
- `cargo metadata --locked --format-version=1`: success.
- Cargo manifest/lock drift: absent.

## NA-0578 Gating Defect Review

Classification: `QSL_SERVER_NA0578_GATING_DEFECT_INSPECTION_STATE_NOT_PERSISTED`.

The NA-0578 inspection script wrote its handoff manifest before assigning the final corrected-start-safe classification. The remote inspection stdout contained safe state, but the start script read the persisted manifest, saw a stale insufficient classification, and stopped before qsl-server execution. This was a generated harness handoff bug, not qsl-server runtime failure evidence.

## Local Dry-Run Assertion

The proof-root local dry-run assertion passed before remote start. It proved:

- inspection output contains `corrected_temp_loopback_smoke_safe=yes`;
- start script can read inspection-safe state from a deterministic manifest;
- field names match;
- origin sentinel matches;
- start argv shape is host-only bind plus separate port;
- host-plus-port bind is not used;
- dry-run mode proceeds past gating without running qsl-server;
- sanitized environment omits `RELAY_TOKEN`;
- selected port, command line, and process identity are not printed;
- child exit/stderr classification is redacted.

## qsl-server CLI Source / Help Review

qsl-server source/help review reconfirmed:

- `--bind <BIND>` is host/address only;
- `--port <PORT>` is separate;
- qsl-server composes bind plus port internally;
- `RELAY_TOKEN` is optional for startup when absent/empty;
- route-token header checks still apply to route usage;
- with sanitized environment omitting `RELAY_TOKEN`, the temporary local no-secret smoke does not need bearer auth.

## Remote Script Design and Static Review

Four proof-root-only Python stdlib scripts were generated and static-reviewed:

- follow-up inspection;
- corrected bounded start;
- no-secret/no-body postcheck;
- cleanup.

Static review passed for syntax, JSON-only stdout, no `shell=True`, no forbidden commands, no secret-file or authorized_keys access, workspace-bounded writes, no qsc send/receive, no qsl-attachments action, no raw port/process/command publication, deterministic manifest handoff, and cleanup limited to NA-0579-owned state.

## SSH Readiness

Classification: `SSH_QSL_SERVER_PROOF_COMPLETION_FOLLOWUP_READY`.

The authorized SSH readiness command ran exactly once. A strict newline classifier mismatch was recovered without rerunning SSH by checking return code, expected sentinel prefix, empty stderr, and private-material scan pass.

## Remote Follow-Up Inspection

Classification: `QSL_SERVER_PROOF_COMPLETION_CORRECTED_START_SAFE`.

Remote inspection classified staged binary present/executable, qsl-server help/version available, bind host-only, separate port available, corrected temporary loopback smoke safe, secret-free start, non-privileged start, loopback-only start, no sudo/systemd, no Tailscale/firewall, no account/authorized_keys action, no secret/endpoint action, and inspection-safe manifest written.

## Corrected Bounded Start

Classification: `QSL_SERVER_PROOF_COMPLETION_BOUNDED_START_TEMP_SMOKE_STARTED`.

The corrected bounded start used the remote inspection-safe manifest, verified the origin sentinel and local-dry-run assertion ID, used host-only bind plus separate port shape, omitted `RELAY_TOKEN`, started only the staged qsl-server under the qslcodex test workspace, wrote only NA-0579-owned temporary state, and did not publish selected port, command line, or process identity.

## Corrected Bounded Postcheck

Classification: `QSL_SERVER_PROOF_COMPLETION_POSTCHECK_TEMP_LOOPBACK_ROUTE_SHAPE_PASS`.

The postcheck performed local loopback, no-secret/no-body route-shape probes, did not run qsc, did not use Authorization/bearer values, did not print or store response bodies, and output only safe status classes.

## Cleanup / Rollback

Classification: `QSL_SERVER_PROOF_COMPLETION_CLEANUP_DONE`.

Cleanup acted only on NA-0579-owned temporary smoke state from the start manifest. It did not print process identity and did not use sudo/systemctl/service.

## Private-Material Review

Aggregate private-material review passed.

No endpoint value was published. No private port value was published. No route-token/capability value was published. No bearer or Authorization value was published. No private topology was published. No process identity was published. No command line was published. No payload or response body was published. No authorized_keys content or key material was published.

## Result Classification

`QSL_SERVER_PROOF_COMPLETION_TEMP_LOOPBACK_ROUTE_SHAPE_PASS_EXPECTED_BIND_REQUIRED`

NA-0579 proved qsl-server can start and answer route-shape probes on a corrected temporary loopback bind. Expected remote bind / endpoint alignment remains unresolved.

## Selected Successor

`NA-0580 -- QSL Remote qsl-server Expected Bind / Endpoint Alignment Harness`

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

Remote action was limited to the authorized SSH readiness, follow-up inspection, corrected bounded start, postcheck, and cleanup commands. No scp, sudo, systemctl, service, firewall, Tailscale, account, shell, authorized_keys, qsl-backup, or root-owned path action occurred. No writes occurred outside `/home/qslcodex/qsl-remote-test/`.

## Public-Site / Cloudflare Boundary

No public-site or Cloudflare mutation occurred.

## Raw Output Boundary

Raw outputs, generated scripts, parsed remote JSON, source/help review artifacts, and private scans remain proof-root-only. Tracked repository evidence contains only coarse classifications and safe boundary statements.

## Claim Boundary

No public-readiness claim is made. No production-readiness claim is made. No public-internet-readiness claim is made. No external-review-complete claim is made. No vulnerability-free claim is made. No bug-free claim is made. No perfect-build claim is made. No perfect-crypto claim is made.

## Validation

Validation includes qwork proof, queue/decision proof, current-main checks, D-1147/D-1148 inheritance, authority confirmation, NA-0578 gating-defect review, local dry-run assertion, qsl-server CLI source/help review, remote script static review, SSH readiness scan, remote inspection scan, corrected start scan, postcheck scan, cleanup scan, aggregate private-material review, result/successor classification, scope guard, marker proof, link-check, private-material scan, overclaim scan, PR body preflight, goal-lint, cargo audits, locked metadata, formatting, and shell syntax checks.

## Recommendation

Proceed to closeout only after this implementation PR merges and post-merge public-safety/advisories/required-check gates are green. Closeout should mark NA-0579 DONE, record D-1150, and restore exactly one READY successor: NA-0580 expected bind / endpoint alignment. Do not implement NA-0580 during closeout.
