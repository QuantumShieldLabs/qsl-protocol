Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-24

# NA-0536 Remote qsc E2EE Repeated-Run / Cleanup / Freshness Scope Authorization Plan

## Executive summary

NA-0536 is authorization-only. It consumes the NA-0535/D441 wrong-peer and stale-trust remote qsc E2EE negative evidence and selects the next bounded implementation lane for repeated-run freshness, cleanup robustness, retained-qsc freshness before each run, port 39176 marker/ACK gating before each run, synthetic proof redaction, and explicit no qsl-server / no qsl-attachments boundaries.

Selected classification: `REMOTE_E2EE_REPEATED_RUN_CLEANUP_FRESHNESS_IMPLEMENTATION_READY`.

Selected successor: `NA-0537 -- QSL Remote qsc E2EE Repeated-Run / Cleanup / Freshness Implementation Harness`.

No remote action, SSH execution, qsc send/receive, qsc E2EE, qsl-server use, qsl-attachments use, qwork/qstart/qresume execution, qsl-backup execution, package install, key/config/host mutation, qsc source/test/fuzz/Cargo mutation, workflow/script/helper mutation, corpus/vector/input mutation, dependency/lockfile mutation, formal/refimpl/service/public/backup mutation, or public/production/security-completion claim is authorized or performed by NA-0536.

## Live NA-0536 scope

Allowed repository mutations for this authorization PR are limited to:

- `docs/governance/evidence/NA-0536_qsl_remote_qsc_e2ee_repeated_run_cleanup_freshness_scope_authorization_plan.md`
- `tests/NA-0536_qsl_remote_qsc_e2ee_repeated_run_cleanup_freshness_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

NA-0536 does not edit `NEXT_ACTIONS.md`. NA-0536 remains READY until a separate closeout PR, after this authorization PR merges and post-merge public-safety/advisories are green, marks it DONE and restores exactly one successor.

Proof root for this directive: `/srv/qbuild/tmp/NA0536_repeated_run_cleanup_freshness_scope_authorization_20260624T174125Z`.

## qwork proof-file verification

Codex did not run qwork, qstart, or qresume. It read and copied the expected proof files from `/srv/qbuild/work/NA-0536/.qwork/`.

The `.kv` and `.json` proofs both recorded `startup_result=OK`, lane `NA-0536`, repo `qsl-protocol`, path `/srv/qbuild/work/NA-0536/qsl-protocol`, clean worktree/index/untracked state, `ready_count=1`, `queue_top_ready=NA-0536`, and requested lane status READY. Proof HEAD and proof origin/main matched live pre-fetch refs at `9e51b9a35e8`.

Fetch happened only after proof/live ref match and disk usage was below the 95% stop threshold. After fetch, `origin/main` still equaled `9e51b9a35e8` and descended from the required D441 closeout merge `9e51b9a35e8`.

Startup queue and decision proof:

- READY_COUNT 1.
- READY `NA-0536 -- QSL Remote qsc E2EE Repeated-Run / Cleanup / Freshness Scope Authorization Plan`.
- NA-0535 DONE.
- NA-0534 DONE.
- D-1060 exists once.
- D-1061 exists once.
- D-1062 was absent before this patch.
- D-1063 was absent before this patch.
- Duplicate decision count 0.

Current main health at startup:

- `public-safety` completed success.
- `advisories` completed success.
- No retrieved check-run was red or in progress.
- Root `Cargo.lock` and nested qsc fuzz `Cargo.lock` both retained `quinn-proto 0.11.15`.
- Cargo.toml drift check was empty.
- qsl-backup was checked read-only by SHA and installed source-list count; qsl-backup was not executed.

## D441 / D440 / D439 / D435 / D419 inheritance

D441 recorded NA-0535 DONE, NA-0536 READY, D-1060 once, D-1061 once, D-1062 absent, and final main at `9e51b9a35e8`. The NA-0535 classification was `REMOTE_E2EE_WRONG_PEER_STALE_TRUST_NEGATIVES_PASS`.

D441 retained-qsc freshness recheck passed for `/home/qslcodex/qsl-remote-test/bin/qsc`: owner/group `qslcodex/qslcodex`, mode `700`, size `102103920`, and SHA-256 `6bf9e59fdae397c2e0f88538d700cccbee80d229c6a979cc79555e39fea2b4f7`.

D441 consumed the D439 port 39176 diagnostic and proved the integrated marker/ACK precheck before qsc E2EE. The baseline remote qsc E2EE setup passed with synthetic data only. The wrong-peer negative failed closed. The stale/replaced-peer negative failed closed. Selected-state no-mutation checks passed. The valid path remained usable after the negative checks. Cleanup passed: remote E2EE root removed, local sensitive runtime removed, relay/listener/SSH forward stopped, local and remote port 39176 closed, stale proof-root process count zero, and retained remote qsc unchanged.

D440 recorded the NA-0534 closeout, restored NA-0535, and verified public-safety/advisories green without remote action, SSH execution, qsc send/receive, remote E2EE, qsl-server, or qsl-attachments work.

D439 recorded port 39176 diagnostic success: the remote loopback bind probe passed, the corrected single-lifetime integrated probe kept the reverse-forward alive, the remote trigger received ACK, listener `marker_match` and `ack_sent` were true, and cleanup passed. No qsc E2EE occurred in NA-0534.

D435 recorded trigger remediation success using the stdin-script trigger shape and marker/ACK traversal without qsc E2EE.

D419 recorded replay and corrupt delivery negative tests passing with no-mutation checks, valid-path usability, cleanup, and no qsl-server/qsl-attachments use.

No inherited evidence authorizes qsl-server or qsl-attachments integration. Both remain deferred architecture boundaries.

## Residual inventory

| Category | Residual item | NA-0536 disposition |
|---|---|---|
| Repeated-run freshness | Retained qsc hash/path/owner/mode/size must be rechecked before each run. | Authorize NA-0537 to recheck before run 1 and run 2. |
| Repeated-run freshness | Source/runtime drift must be checked before using the retained qsc. | Require stop if qsc runtime/dependency drift requires restage. |
| Repeated-run freshness | Port 39176 marker/ACK must pass before each run. | Require marker/ACK gate before run 1 and run 2. |
| Cleanup robustness | Remote E2EE root cleanup after success. | Require cleanup proof after each run. |
| Cleanup robustness | Local sensitive runtime cleanup. | Require deletion proof after each run. |
| Cleanup robustness | Relay/listener/SSH process cleanup. | Require process scans and stop on stale children. |
| Cleanup robustness | Local and remote port closure. | Require port 39176 closed after cleanup. |
| Cleanup robustness | Stale proof-root process detection. | Require proof-root process scans. |
| Repeated-run determinism | Run A / run B must use unique proof IDs. | Require two unique proof-run roots. |
| Repeated-run determinism | No stale state reuse across runs. | Require selected no-stale-state summaries. |
| Repeated-run determinism | Valid path remains usable after repeated runs. | Require valid-path result or explicit selected negative repeat result. |
| Negative-path repeatability | Wrong-peer repeat run. | Preferred if command surface remains safe. |
| Negative-path repeatability | Stale/replaced-peer repeat run. | Preferred if command surface remains safe. |
| Negative-path repeatability | Replay/corrupt negative repeat. | Optional or deferred because D419 already covered it and NA-0537 should stay bounded. |
| Redaction/no-secret-output | Proof-root raw output scan. | Require scan before summaries are checked in. |
| Redaction/no-secret-output | Checked-in evidence added-line scan. | Require no private keys, passphrases, tokens, passwords, route-token values, or raw private qsc material. |
| Redaction/no-secret-output | Route-token/capability metadata redaction. | Require summaries only; raw values stay out of repo evidence. |
| Retained-binary policy | Preserve retained qsc if still needed. | Keep retained binary unless mismatch/drift requires stop and restage authorization. |
| Retained-binary policy | Cleanup command documented. | Require future command manifest and cleanup proof. |
| Retained-binary policy | Restage if runtime/dependency drift occurs. | Stop condition, not an implicit NA-0537 action. |
| Public evidence sync | Future public/repo evidence sync has communication value. | Recommend separate future lane after repeated-run checkpoint. |
| qsl-server/qsl-attachments | Integration remains deferred. | No integration until direct qsc evidence checkpoint is stable and separately authorized. |

## Option review

Option 1, authorize repeated-run cleanup/freshness implementation, is selected. It directly reduces stale retained-binary, stale runtime-root, stale process, stale port, and stale proof-state risks while the remote host is still available. It should run two bounded synthetic remote qsc E2EE cycles with fresh proof IDs, retained-qsc hash recheck before each cycle, marker/ACK gate before each cycle, cleanup after each cycle, selected no-stale-state checks, and either one repeated wrong-peer/stale-trust negative or a lightweight valid-path repeat depending on command-surface risk.

Option 2, cleanup-only implementation, is rejected as lower directness. It could confirm cleanup after prior lanes but would not prove repeatability through a fresh E2EE cycle.

Option 3, repeated valid-path-only implementation, is rejected as weaker than Option 1. It would reduce freshness risk but would not continue identity/trust negative hardening.

Option 4, repeated negative-only implementation, is rejected as riskier because the baseline setup still has to exist and should not be treated as incidental.

Option 5, public website / repo evidence sync authorization, is recommended soon but not before repeated-run cleanup/freshness evidence unless the Director explicitly reprioritizes public communication.

Option 6, qsl-server/qsl-attachments integration, is deferred. Direct qsc evidence should be stabilized first.

Option 7, retained qsc cleanup / sprint shutdown, is rejected unless the remote host becomes unavailable or the operator chooses to stop.

Option 8, broader security/release readiness package, is rejected as premature.

## Selected future implementation design

Future lane:

`NA-0537 -- QSL Remote qsc E2EE Repeated-Run / Cleanup / Freshness Implementation Harness`

Status: READY after NA-0536 closeout only.

Goals: G1, G2, G3, G4, G5.

Objective: execute a bounded repeated-run cleanup/freshness harness after NA-0535 success, using retained remote qsc with fresh hash recheck before each run, integrated port 39176 marker/ACK gate before each run, synthetic qsc E2EE runtime roots with unique proof IDs, cleanup after each run, local/remote port and process cleanup checks, no-secret-output review, no qsl-server/qsl-attachments, and no public/production readiness claims.

Allowed future NA-0537 repository scope:

- `docs/governance/evidence/NA-0537_qsl_remote_qsc_e2ee_repeated_run_cleanup_freshness_implementation_harness.md`
- `tests/NA-0537_qsl_remote_qsc_e2ee_repeated_run_cleanup_freshness_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Allowed future NA-0537 runtime scope:

- proof-root-local command manifest/logs.
- bounded SSH to inspiron as qslcodex.
- retained remote qsc hash/path/owner/mode/size recheck before each run.
- integrated listener/forward/trigger marker/ACK precheck before each run.
- local qsc binary from clean checkout.
- synthetic messages only.
- synthetic identities/trust records only.
- remote artifacts only under `$HOME/qsl-remote-test/e2ee/<PROOF_ID>/run-<N>`.
- local sensitive runtime under proof root.
- cleanup/retention proof after each run.
- optional repeated wrong-peer/stale-trust negatives if command surface remains safe.

Forbidden future NA-0537 scope:

- qsl-server/qsl-attachments.
- package installation.
- sudo/admin action except negative `sudo -n true` probe.
- key generation/installation.
- authorized_keys mutation or reading.
- sshd_config mutation or reading unless operator supplies redacted proof in a later lane.
- SSH config mutation outside proof root.
- known_hosts mutation.
- remote file write outside `$HOME/qsl-remote-test/e2ee/<PROOF_ID>`.
- remote temp file write.
- remote source checkout/build.
- qwork/qstart/qresume.
- qsl-backup.
- qsc source/test/fuzz/Cargo mutation.
- workflow/dependency mutation.
- corpus/vector/input mutation.
- formal/refimpl/service/public/backup mutation.
- production/user data.
- no public-readiness, production-readiness, public-internet-readiness, crypto-complete, identity-complete, trust-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim.

Acceptance criteria:

- qwork proof verified without rerunning qwork.
- retained qsc hash/path/owner/mode/size rechecked before each run.
- qsc runtime/dependency drift check passed.
- port 39176 marker/ACK gate passed before each run.
- two unique proof-run roots used.
- valid remote qsc E2EE path succeeds in both runs, or a selected negative repeat succeeds when included.
- no stale state reuse across runs.
- cleanup after each run removes remote E2EE root and local sensitive runtime.
- local and remote port 39176 closed after cleanup.
- proof-root process scans clean.
- no secret material in checked-in evidence.
- no qsl-server/qsl-attachments.
- no public/production readiness claim.
- exactly one successor selected.

## Future command family

Future NA-0537 may run only under its own directive:

- qwork proof reading only; Codex must not run qwork.
- retained qsc metadata/hash/smoke with local-only capture.
- local qsc build/selection from a clean checkout.
- safe `ssh -G` parsing.
- remote boundary checks.
- local/remote port-state checks.
- integrated marker/ACK precheck before each run.
- qsc vault/identity/contact/trust/relay/handshake/send/receive commands already used in NA-0535.
- synthetic artifact transfer only.
- wrong-peer/stale-trust repeats if selected and safe.
- cleanup after each run.

Future NA-0537 must not use qsl-server/qsl-attachments, install packages, mutate qsc source/Cargo, mutate workflows/dependencies/corpus/formal/refimpl/service/public/backup paths, use production/personal data, or claim public/production/security completeness.

## Future proof / redaction rules

Future proof must include no private keys, passphrases, tokens, passwords, production endpoints, backup material, personal data, or raw qsc private runtime material.

Checked-in evidence should summarize run IDs, retained qsc hash checks, marker/ACK prechecks, baseline/negative results, no-stale-state summaries, cleanup status, no-secret-output scan status, and successor selection. Raw qsc output stays proof-root-local and must be scanned/redacted before any summary is checked in.

## Future stop conditions

Future NA-0537 must stop for stale qwork proof, retained qsc mismatch, qsc runtime/dependency drift requiring restage, port 39176 marker/ACK gate failure, remote boundary failure, qsl-server/qsl-attachments requirement, package install requirement, qsc source/Cargo mutation need, private material exposure, production/personal data exposure, repeated-run stale state reuse, cleanup failure, route/capability metadata leakage in checked-in evidence, or pressure to make public/production/security-completion claims.

## Hostile Cryptographer Review

Repeated-run cleanup/freshness proves bounded operational repeatability only. It does not prove crypto-complete, identity-complete, trust-complete, replay-proof, downgrade-proof, side-channel-free, vulnerability-free, bug-free, or perfect-crypto status.

Synthetic repeated runs reduce data exposure but do not substitute for external review. Formal model evidence remains bounded and separate.

## Red-Team Review

Stale retained qsc, stale remote roots, stale local sensitive runtime, stale relay/SSH processes, and port reuse are the target risk areas. Cleanup failure must stop. Repeated success must not be overread as production readiness. Public/website sync should remain a separate no-claim lane.

## Production SRE Review

Repeated-run cleanup/freshness is operationally appropriate after NA-0535 success. Logs must remain proof-root-local and redacted. Cleanup proof must cover remote roots, local sensitive runtime, relay/listener/SSH processes, and ports after each run. qwork, qsl-backup, production data, qsl-server, and qsl-attachments remain isolated. This does not imply public/production readiness.

## Release-Claim Boundary Review

No public-ready claim is made. No production-ready claim is made. No public-internet-ready claim is made. No external-review-complete claim is made. No crypto-complete claim is made. No identity-complete claim is made. No trust-complete claim is made. No replay-proof claim is made. No downgrade-proof claim is made. No secret-material-complete claim is made. No side-channel-free claim is made. No vulnerability-free, bug-free, or perfect-crypto claim is made.

## Prioritization matrix

| Candidate | Risk reduced | Directness | Speed while host is available | Security risk | Operator burden | Implementation feasibility | Scope risk | Remote mutation risk | Secret/key risk | Public-claim risk | Disposition | Next-lane yes/no |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Repeated-run cleanup/freshness implementation | Stale qsc, stale roots, stale processes, stale ports, stale proof reuse | High | High | Medium but bounded | Medium | High | Medium | Medium, confined to E2EE proof roots | Medium, mitigated by synthetic data/redaction | Low if claim boundary enforced | Select | yes |
| Cleanup-only implementation | Stale residue after prior lane | Medium-low | High | Low | Low | High | Low | Low | Low | Low | Reject as less direct | no |
| Repeated valid-path-only implementation | Freshness/determinism without negatives | Medium | High | Medium | Medium | High | Medium | Medium | Medium | Low | Reject as weaker than Option 1 | no |
| Repeated negative-only implementation | Identity/trust repeatability | Medium | Medium | Medium-high | Medium | Medium | Medium-high | Medium | Medium | Low | Reject; baseline still needed | no |
| Public website / repo evidence sync authorization | Communication drift | Medium | Medium | Low | Low | High | Medium | None | Low | Medium | Recommend later | no |
| qsl-server/qsl-attachments integration | Service integration gap | High eventually | Low now | High | High | Low until direct qsc stabilizes | High | High | Medium | High | Defer | no |
| Retained qsc cleanup / remote sprint shutdown | Residual remote binary footprint | Low-to-medium | High | Low | Low | High | Low | Low | Low | Low | Reject unless host unavailable/operator stops | no |
| Broader security/release readiness package | Broad assurance gaps | Diffuse | Low | High | High | Low now | High | Variable | Variable | High | Reject as premature | no |

## Authorization decision

Classification selected: `REMOTE_E2EE_REPEATED_RUN_CLEANUP_FRESHNESS_IMPLEMENTATION_READY`.

Required conditions satisfied:

- D441 consumed.
- D439 consumed.
- D419 consumed.
- residual inventory completed.
- option review completed.
- future command family selected.
- proof/redaction/stop rules selected.
- hostile cryptographer, red-team, SRE, and release-claim reviews completed.
- prioritization matrix completed.
- exact NA-0537 successor selected.
- no remote action in NA-0536.
- no SSH execution in NA-0536.
- no qsc send/receive in NA-0536.
- no remote E2EE in NA-0536.
- no qsl-server/qsl-attachments selected.
- no qsc source/test/fuzz/Cargo mutation.
- no public claim expansion.
- exactly one READY successor remains mandatory after closeout.

## Selected NA-0537 successor

### NA-0537 -- QSL Remote qsc E2EE Repeated-Run / Cleanup / Freshness Implementation Harness
Status: READY
Goals: G1, G2, G3, G4, G5

Objective:
Execute a bounded repeated-run cleanup/freshness harness after NA-0535 success, using retained remote qsc with fresh hash recheck before each run, integrated port 39176 marker/ACK gate before each run, synthetic qsc E2EE runtime roots with unique proof IDs, cleanup after each run, local/remote port and process cleanup checks, no-secret-output review, no qsl-server/qsl-attachments, and no public/production readiness claims.

Allowed scope:
- docs/governance/evidence/NA-0537_qsl_remote_qsc_e2ee_repeated_run_cleanup_freshness_implementation_harness.md
- tests/NA-0537_qsl_remote_qsc_e2ee_repeated_run_cleanup_freshness_implementation_testplan.md
- DECISIONS.md
- TRACEABILITY.md
- docs/ops/ROLLING_OPERATIONS_JOURNAL.md
- proof-root-local command manifest/logs.
- bounded SSH to inspiron as qslcodex.
- retained remote qsc hash/path/owner/mode/size recheck before each run.
- integrated listener/forward/trigger marker/ACK precheck before each run.
- local qsc binary from clean checkout.
- synthetic messages only.
- synthetic identities/trust records only.
- remote artifacts only under `$HOME/qsl-remote-test/e2ee/<PROOF_ID>/run-<N>`.
- local sensitive runtime under proof root.
- cleanup/retention proof after each run.
- optional repeated wrong-peer/stale-trust negatives if command surface remains safe.

Forbidden scope:
- qsl-server/qsl-attachments.
- package installation.
- sudo/admin action except negative `sudo -n true` probe.
- key generation/installation.
- authorized_keys mutation or reading.
- sshd_config mutation or reading unless operator supplies redacted proof in a later lane.
- SSH config mutation outside proof root.
- known_hosts mutation.
- remote file write outside `$HOME/qsl-remote-test/e2ee/<PROOF_ID>`.
- remote temp file write.
- remote source checkout/build.
- qwork/qstart/qresume.
- qsl-backup.
- qsc source/test/fuzz/Cargo mutation.
- workflow/dependency mutation.
- corpus/vector/input mutation.
- formal/refimpl/service/public/backup mutation.
- production/user data.
- no public-readiness, production-readiness, public-internet-readiness, crypto-complete, identity-complete, trust-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim.

## Future scope bundle

Future NA-0537 should bind together retained-qsc freshness, per-run marker/ACK gates, isolated synthetic local/remote roots, optional repeated identity/trust negative checks, cleanup after each run, no-secret-output scans, and explicit qsl-server/qsl-attachments deferral. It should not include public website/repo sync, qsl-server/qsl-attachments integration, retained binary shutdown, broader release readiness, or any public/production/security-completion claim.

## Future validation / marker plan

Future NA-0537 markers:

- `NA0537_D441_SUCCESS_CONSUMED_OK`
- `NA0537_RETAINED_QSC_HASH_RECHECKED_RUN1_OK`
- `NA0537_RETAINED_QSC_HASH_RECHECKED_RUN2_OK`
- `NA0537_PORT_39176_MARKER_ACK_RUN1_OK`
- `NA0537_PORT_39176_MARKER_ACK_RUN2_OK`
- `NA0537_REMOTE_E2EE_RUN1_OK`
- `NA0537_REMOTE_E2EE_RUN2_OK`
- `NA0537_NO_STALE_STATE_REUSE_OK`
- `NA0537_REMOTE_ROOT_CLEANUP_RUN1_OK`
- `NA0537_REMOTE_ROOT_CLEANUP_RUN2_OK`
- `NA0537_LOCAL_SENSITIVE_RUNTIME_CLEANUP_RUN1_OK`
- `NA0537_LOCAL_SENSITIVE_RUNTIME_CLEANUP_RUN2_OK`
- `NA0537_LOCAL_REMOTE_PORTS_CLOSED_OK`
- `NA0537_NO_SECRET_OUTPUT_OK`
- `NA0537_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0537_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0537_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0537_ONE_READY_INVARIANT_OK`

## Public website / repo evidence sync recommendation

Public website or repository evidence synchronization has communication value and should be scheduled soon after the repeated-run cleanup/freshness checkpoint, unless the Director explicitly prioritizes public attention first. Any such lane must be separate, must not imply public/production readiness, and must not expand external-review or security-completion claims.

## No remote action in NA-0536

NA-0536 performs no remote action and executes no SSH. It reads local proof files, repository governance files, GitHub check metadata, and local read-only qsl-backup boundary evidence only.

## No qsc send/receive in NA-0536

NA-0536 does not run qsc send, qsc receive, qsc relay, qsc E2EE, or qsc protocol commands. It only authorizes a future implementation lane to reuse previously proven command surfaces under a new directive.

## No qsl-server / no qsl-attachments boundary

NA-0536 uses no qsl-server and no qsl-attachments. Future NA-0537 also keeps both out of scope. Direct qsc evidence must remain stable before either service boundary is reopened by a separate directive.

## Public claim / website / external review boundary

This plan is internal governance authorization. It is not public evidence sync, website work, production deployment, public-internet exposure, external review completion, or a release-readiness assertion.

## Backup-impact statement

No backup or restore is run. qsl-backup is checked read-only by installed helper digest and source-list count. `/backup/qsl` is not mutated. NA-0537 must also keep qwork, qsl-backup, backup data, and production data isolated.

## Rejected alternatives

- Cleanup-only implementation: rejected because it does not exercise repeated E2EE freshness.
- Valid-path-only repeated implementation: rejected because it does not carry forward identity/trust negative hardening.
- Negative-only repeated implementation: rejected because baseline setup still matters and should be explicit.
- Public website/repo evidence sync now: rejected as a preemption unless Director reprioritizes communication.
- qsl-server/qsl-attachments integration: deferred until direct qsc evidence is stable.
- Retained qsc cleanup / sprint shutdown: rejected unless the host becomes unavailable or the operator chooses to stop.
- Broader security/release readiness package: rejected as premature.

## Next recommendation

Merge the NA-0536 authorization PR only after required checks pass. Then, if post-merge public-safety is green inside the short attach/early-failure window, close out NA-0536 and restore NA-0537 as the sole READY successor. If post-merge public-safety is still running but healthy after the short window, stop and hand off for closeout.
