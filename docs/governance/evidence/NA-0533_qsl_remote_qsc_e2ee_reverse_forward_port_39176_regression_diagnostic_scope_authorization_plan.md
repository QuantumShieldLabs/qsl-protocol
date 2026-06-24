Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-06-24

# NA-0533 QSL Remote qsc E2EE Reverse-Forward Port 39176 Regression Diagnostic Scope Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0533 is an authorization-only governance/security lane. It consumes the NA-0532/D436 forwarding precheck failure and the prior forwarding success chain, then selects the exact next diagnostic implementation lane for the reverse-forward port 39176 regression.

Selected classification: `REMOTE_FORWARD_PORT_39176_REGRESSION_DIAGNOSTIC_IMPLEMENTATION_READY`.

Selected successor: `NA-0534 -- QSL Remote qsc E2EE Reverse-Forward Port 39176 Regression Diagnostic Implementation Harness`.

This lane does not diagnose the host live. It runs no SSH, no remote commands, no qsc E2EE, no qsc send/receive, no qsl-server, and no qsl-attachments. It changes only governance evidence, testplan, decision, traceability, and rolling journal files.

## Live NA-0533 scope

Allowed checked-in mutation paths for this authorization PR are this evidence file, `tests/NA-0533_qsl_remote_qsc_e2ee_reverse_forward_port_39176_regression_diagnostic_scope_authorization_testplan.md`, `DECISIONS.md`, `TRACEABILITY.md`, and `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Proof root: `/srv/qbuild/tmp/NA0533_reverse_forward_port_39176_regression_scope_authorization_20260624T135353Z`.

This lane selects future diagnostic command families and stop rules only. It does not mutate `NEXT_ACTIONS.md`, runtime source, qsc source/test/fuzz/Cargo paths, dependency/lockfile paths, workflow/script/helper paths, corpus/vector/input paths, formal/refimpl/service/public/backup paths, qsl-server, qsl-attachments, SSH keys, authorized_keys, known_hosts, SSH config, remote hosts, or qsl-backup.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

The required qwork proof files existed and were copied into the proof root:

- `/srv/qbuild/work/NA-0533/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0533/.qwork/startup.qsl-protocol.json`

The `.kv` and `.json` proofs matched the required startup state: `startup_result=OK`, lane `NA-0533`, repo `qsl-protocol`, path `/srv/qbuild/work/NA-0533/qsl-protocol`, clean worktree/index/untracked state, `head_equals_origin_main=yes`, `ready_count=1`, queue top READY `NA-0533`, and requested lane status READY.

Proof HEAD and proof `origin/main` both matched the live pre-fetch refs at `93d90d4657a8`. Fetch occurred only after that match and after disk proof showed `/` usage below the 95% stop threshold.

Startup queue/decision proof on current main:

- READY_COUNT 1.
- READY `NA-0533 -- QSL Remote qsc E2EE Reverse-Forward Port 39176 Regression Diagnostic / Retry Scope Authorization Plan`.
- NA-0532 DONE.
- NA-0531 DONE.
- NA-0530 DONE.
- D-1054 exists once.
- D-1055 exists once.
- D-1056 absent before patch.
- D-1057 absent before patch.
- Duplicate decision count 0.

Current main health before patch:

- `origin/main` equals `93d90d4657a8` and descends from `93d90d4657a8`.
- `public-safety` completed success.
- `advisories` completed success.
- Retrieved check-runs had no completed failure, action-required, timed-out, or cancelled conclusion.
- Root `Cargo.lock` and nested qsc fuzz `Cargo.lock` both retained `quinn-proto 0.11.15`.
- Cargo manifests had no drift.
- qsl-backup was checked read-only: installed helper digest matched `e9ecff3d22ed...f6232`, the Codex ops source entry count was exactly 1, and qsl-backup was not executed.

## D437 / D436 / D435 / D414 / D413 inheritance

D437 recorded NA-0532 DONE and NA-0533 READY after PR #1338 merged at `93d90d4657a8`. It also recorded that D-1055 exists once, D-1056 was absent, post-closeout public-safety and advisories were green, and no remote action occurred in D437.

D436 recorded PR #1337 merged at `aaf4d7f03d5c`, D-1054 exists once, and classification `REMOTE_E2EE_INTEGRATED_FORWARDING_PRECHECK_FAILURE`. It rechecked retained qsc, wrote the command manifest, passed local/remote boundary checks, compiled and rehearsed the local trigger, and bound the local listener to `127.0.0.1:39176`.

D436 reverse-forward command used the dedicated-key proof-root SSH config, `ssh -N -T`, `ExitOnForwardFailure=yes`, and `-R 127.0.0.1:39176:127.0.0.1:39176`. It exited before remote trigger execution with `remote port forwarding failed for listen port 39176`.

D436 therefore recorded no marker traversal, no ACK, no qsc relay start, no remote E2EE root creation, no qsc E2EE, no qsc send/receive, no baseline E2EE, no wrong-peer negative, no stale-trust negative, no qsl-server, and no qsl-attachments.

D435 recorded classification `REMOTE_TRIGGER_QUOTING_REMEDIATION_MARKER_TRAVERSAL_PASS`. Its stdin-script trigger shape worked; the integrated listener / dedicated-key reverse-forward / remote trigger lifetime passed marker traversal and ACK, and D435 recorded no qsc E2EE and no qsc send/receive.

D414 recorded classification `SSH_FORWARDING_CAPABILITY_PROBE_PASS`. NA-0520 proved that the dedicated-key reverse-forward to `127.0.0.1:39176` could carry a synthetic marker and return an ACK, without qsc E2EE or qsc send/receive.

D413 recorded that the dedicated forwarding key proof was accepted with fingerprint `SHA256:7guT/Hd72vuu+u1Jrwpuy0hSFETVrxYaXHwupuFfaPM`, public key comment `qsl-inspiron-qslcodex-forward-20260622`, loopback-only `permitlisten` and `permitopen` on `127.0.0.1:39176`, no PTY, no agent/X11 forwarding, qslcodex non-sudo status, and no backup/qwork/qsl-backup exposure.

qsl-server and qsl-attachments are out of scope throughout this chain.

## Regression inventory

Success cases:

| Case | Result | Meaning |
|---|---|---|
| NA-0520 | Dedicated-key reverse-forward to `127.0.0.1:39176` succeeded; marker traversal and ACK succeeded. | The constrained forwarding key and loopback `-R` shape previously worked for a synthetic transport probe. |
| Manual integrated proof | Marker traversal and ACK succeeded. | The integrated listener/forward/trigger shape could succeed outside an implementation lane, but still needed in-lane proof before E2EE. |
| NA-0531 | Stdin-script trigger shape succeeded; marker traversal and ACK succeeded. | Trigger quoting was remediated and the integrated marker gate passed without qsc E2EE. |

Failure cases:

| Case | Result | Meaning |
|---|---|---|
| D427 | First config had `ClearAllForwardings yes`; corrected retries failed with `remote port forwarding failed for listen port 39176`. | Config shape was one cause, but correcting it did not settle the port-allocation failure. |
| D430 | Reverse-forward started; remote trigger failed. | The exact D427 startup failure did not reproduce there; trigger/target timing or shape remained suspect. |
| D433 | Trigger quoting failed. | The forwarding path stayed alive, but the remote `python3 -c` payload was split before marker traversal. |
| D436 | Reverse-forward failed before remote trigger with `remote port forwarding failed for listen port 39176`. | The port-allocation symptom returned after D435 trigger remediation. |

Potential cause categories remain open:

- stale remote listener/session.
- stale local process or ControlMaster/session reuse.
- remote loopback port already bound.
- remote bind host syntax.
- proof-root SSH config drift.
- default alias hazards.
- dedicated-key `permitlisten` / `permitopen` drift.
- forced-command compatibility regression.
- sshd policy drift.
- remote address-family IPv4/IPv6 ambiguity.
- Tailscale/DNS alias drift.
- local listener readiness/timing.
- cleanup race.

No single cause is selected because the evidence does not prove one.

## Option review

Option 1 - Authorization for bounded port-state diagnostic implementation: selected. It directly addresses D436 by checking local and remote port state, proof-root SSH config, dedicated-key command shape, and known-good marker/ACK behavior before any E2EE retry.

Option 2 - Authorization for operator-side proof review: held as a later successor if diagnostics implicate key or sshd policy. Codex must not read authorized_keys or sshd_config directly.

Option 3 - Authorization for stale-session cleanup diagnostic: partially included as local-only cleanup authority in NA-0534. Future work may identify and terminate local proof-root SSH/listener processes only; it must not kill remote sshd/system processes.

Option 4 - Authorization for alternate remote listen port: rejected unless a later operator-action lane changes key restrictions. The current key is constrained to `127.0.0.1:39176`.

Option 5 - Immediate E2EE retry: rejected until port allocation and marker/ACK traversal are re-proven.

Option 6 - qsl-server/qsl-attachments integration: deferred and out of the direct qsc sprint.

Option 7 - Broad sshd/key remediation: rejected unless diagnostics prove a key/sshd policy issue and a later operator action authorizes remediation.

Option 8 - Abandon remote sprint / cleanup retained qsc: rejected unless the remote host becomes unavailable or diagnostics become irrecoverable.

Best-Known-Method Review: the smallest safe next step is the diagnostic lane that preserves the proven marker/ACK gate and distinguishes port-state, config, policy, alias, and stale-session causes before any qsc E2EE retry.

## Selected future diagnostic implementation design

NA-0534 should be:

`NA-0534 -- QSL Remote qsc E2EE Reverse-Forward Port 39176 Regression Diagnostic Implementation Harness`

Status after closeout: READY.

Goals: G1, G2, G3, G4, G5.

Objective: diagnose the reverse-forward port 39176 regression without running qsc E2EE or qsc send/receive by comparing prior success/failure evidence, checking local and remote loopback port state, confirming proof-root SSH config and dedicated-key constraints, running one known-good integrated marker/ACK probe only if port state is safe, capturing sanitized SSH/port diagnostics, cleaning all local processes, and selecting exactly one successor.

Allowed future checked-in scope:

- `docs/governance/evidence/NA-0534_qsl_remote_qsc_e2ee_reverse_forward_port_39176_regression_diagnostic_implementation_harness.md`
- `tests/NA-0534_qsl_remote_qsc_e2ee_reverse_forward_port_39176_regression_diagnostic_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Allowed future proof-root-only scope:

- command manifest/logs.
- local listener and SSH forward logs.
- safe `ssh -G` parsing.
- read-only local port checks.
- bounded read-only remote account/boundary checks.
- bounded read-only remote port checks.
- optional transient remote loopback bind probe if no listener is detected, no remote files are written, and the probe binds only `127.0.0.1:39176` and exits immediately.
- one known-good integrated marker/ACK probe if port state permits.
- sanitized SSH debug logs if forward fails.
- cleanup proof.

Forbidden future scope:

- qsc E2EE.
- qsc send/receive.
- qsc identity/contact/handshake/relay protocol commands.
- qsl-server/qsl-attachments.
- package installation.
- sudo/admin action except negative `sudo -n true` probe if explicitly needed.
- key generation/installation.
- authorized_keys mutation.
- authorized_keys reading.
- sshd_config mutation.
- sshd_config reading unless operator supplies redacted proof in a later lane.
- SSH config mutation outside proof root.
- known_hosts mutation.
- remote file write.
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

## Future command family

NA-0534 may run only under its own directive:

- qwork proof reading only; Codex must not run qwork.
- safe `ssh -G` parsing.
- local process and port checks for `127.0.0.1:39176`.
- proof-root SSH config with dedicated forwarding key and no `ClearAllForwardings yes`.
- bounded operational SSH for read-only remote boundary checks.
- read-only remote port check using safe commands such as `ss` or equivalent if present.
- optional transient remote loopback bind probe that binds only `127.0.0.1:39176`, exits immediately, writes no files, and runs only if selected by the future directive.
- local listener on `127.0.0.1:39176`.
- dedicated-key reverse forward: `ssh -N -T -o ExitOnForwardFailure=yes -R 127.0.0.1:39176:127.0.0.1:39176 ...`.
- stdin-script remote trigger only after forwarding starts.
- sanitized SSH debug logs if needed.
- cleanup of local proof-root listener and SSH processes.

NA-0534 must not run qsc E2EE, qsc send/receive, qsc identity/contact/handshake/relay protocol commands, write remote files, read authorized_keys, read sshd_config, mutate keys/config, or use qsl-server/qsl-attachments.

## Future proof / redaction rules

Future proof must include no private keys, passphrases, tokens, passwords, production endpoints, backup material, unrelated known_hosts/authorized_keys material, full SSH config, or sshd_config.

Checked-in evidence should summarize only safe command families, safe fields, port-state results, marker result or failure reason, redacted debug-log conclusions, cleanup status, and successor selection.

Raw debug logs must stay under the proof root and be scanned/redacted before any summary is checked in.

## Future stop conditions

Future NA-0534 must stop for:

- stale qwork proof.
- unsafe SSH config.
- `ClearAllForwardings yes` in proof-root forwarding config.
- non-loopback bind.
- dedicated key not used.
- private key/passphrase/token/password in output.
- remote file write need.
- qsc E2EE need.
- qsc send/receive need.
- qsl-server/qsl-attachments requirement.
- authorized_keys or sshd_config read/mutation need.
- package install need.
- qwork/qsl-backup appearance.
- cleanup failure.
- public/production claim pressure.

## Hostile Cryptographer Review

Port diagnostics prove only SSH forwarding setup behavior, not qsc protocol correctness.

Marker traversal, if achieved in NA-0534, would not prove E2EE, identity correctness, trust correctness, replay resistance, downgrade resistance, or side-channel safety.

qsc E2EE remains deferred until forwarding is stable and a later retry lane authorizes it.

Side-Channel Caveat: a marker/ACK transport probe says nothing about timing leakage, metadata minimization, traffic analysis, padding behavior, or qsc side-channel posture.

Formal-Model Mapping Residual: this authorization changes no formal model and adds no protocol transition. Formal verification coverage remains relevant to qsc protocol behavior, but NA-0534 will still be a transport diagnostic unless a later lane explicitly authorizes protocol execution.

## Red-Team Review

Overbroad SSH forwarding, non-loopback bind, private-key exposure, authorized_keys leakage, sshd_config mutation, and remote file writes are rejection conditions.

If key/sshd policy is implicated, use operator-proof review rather than Codex mutation.

If stale local proof-root processes are implicated, clean local processes only.

If remote system/session state is implicated, select an operator-assisted diagnostic or proof-review successor.

## Production SRE Review

Diagnosing the port 39176 regression before E2EE is the right operational step because the D436 failure happened before remote trigger execution and before any qsc protocol command.

Logs must remain proof-root-local and redacted. Cleanup proof must cover local listener and SSH processes. qwork, qsl-backup, production data, qsl-server, and qsl-attachments remain isolated.

This does not imply public, production, or public-internet readiness.

External-Review Readiness: NA-0533 and NA-0534 are not external review completion evidence. They can produce a cleaner transport diagnostic record for later review, but they do not close protocol assurance gaps.

Assurance Gap Review Trigger: if NA-0534 indicates policy drift, key restriction drift, remote service drift, or inability to reproduce prior success, the next successor must explicitly record the gap and choose operator-proof review, corrected retry, or sprint cleanup instead of silently proceeding.

## Release-Claim Boundary Review

This lane preserves no public-ready claim, no production-ready claim, no public-internet-ready claim, no external-review-complete claim, no crypto-complete claim, no identity-complete claim, no trust-complete claim, no replay-proof claim, no downgrade-proof claim, no secret-material-complete claim, no side-channel-free claim, and no vulnerability-free, bug-free, or perfect-crypto claim.

## Prioritization matrix

| Candidate | Risk reduced | Directness | Speed while host is available | Security risk | Operator burden | Implementation feasibility | Scope risk | Remote mutation risk | Secret/key risk | Public-claim risk | Disposition | Next-lane yes/no |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---|
| Bounded port-state diagnostic implementation | Distinguishes port occupancy, stale session, config, and policy causes | High | High | Low | Low | High | Low | Low/read-only | Low | Low | Selected | Yes |
| Known-good integrated marker probe after port-state checks | Re-proves the marker/ACK gate if safe | High | High | Low | Low | High | Medium | Low if no files written | Low | Low | Included conditionally in NA-0534 | Yes |
| Operator key/sshd proof-review lane | Reduces policy/key ambiguity if diagnostics implicate it | Medium | Medium | Low | Medium | Medium | Low | None by Codex | Low if redacted | Low | Deferred successor option | No |
| Stale-session cleanup diagnostic | Reduces local stale process ambiguity | Medium | High | Low | Low | High | Medium | None remote | Low | Low | Local-only subset included | No standalone |
| Alternate remote listen port authorization | Avoids occupied port | Medium | Medium | Medium | High | Low under current key | High | Medium | Medium | Low | Rejected until operator key change | No |
| Immediate E2EE retry | Would test desired path if forwarding were stable | Low | Medium | Medium | Low | Medium | High | Medium | Medium | Medium | Rejected before marker/ACK reproved | No |
| qsl-server/qsl-attachments integration | Could bypass direct qsc tunnel question | Low | Low | Medium | Medium | Low | High | Medium | Medium | High | Deferred/out of direct sprint | No |
| Broad SSH/key/sshd remediation | Could fix policy drift if proven | Medium | Low | High | High | Low | High | High | High | Medium | Rejected without proof and operator action | No |
| Abandon remote sprint / cleanup retained qsc | Reduces operational exposure if host unavailable | Medium | Medium | Low | Medium | Medium | Medium | Low | Low | Low | Rejected unless diagnostics irrecoverable | No |

## Authorization decision

Classification selected: `REMOTE_FORWARD_PORT_39176_REGRESSION_DIAGNOSTIC_IMPLEMENTATION_READY`.

Required conditions satisfied:

- D436 consumed.
- D435 consumed.
- D414 consumed.
- D413 consumed.
- Regression inventory completed.
- Option review completed.
- Future command family selected.
- Proof/redaction/stop rules selected.
- Hostile cryptographer, red-team, SRE, release-claim, side-channel, formal residual, external-review, and assurance-gap reviews completed.
- Prioritization matrix completed.
- Exact NA-0534 successor selected.
- No remote action in NA-0533.
- No SSH execution in NA-0533.
- No qsc send/receive in NA-0533.
- No qsl-server/qsl-attachments selected.
- No qsc source/test/fuzz/Cargo mutation.
- No public claim expansion.
- Exactly one READY successor remains mandatory.

## Selected NA-0534 successor

Default successor text selected for future closeout:

### NA-0534 -- QSL Remote qsc E2EE Reverse-Forward Port 39176 Regression Diagnostic Implementation Harness

Status: READY

Goals: G1, G2, G3, G4, G5

Objective: diagnose the reverse-forward port 39176 regression without running qsc E2EE or qsc send/receive by comparing prior success/failure evidence, checking local and remote loopback port state, confirming proof-root SSH config and dedicated-key constraints, running one known-good integrated marker/ACK probe only if port state is safe, capturing sanitized SSH/port diagnostics, cleaning all local processes, and selecting either a corrected E2EE retry successor, a trigger/port retry successor, or an operator-proof review successor.

## Future scope bundle

Future NA-0534 acceptance criteria:

- D436 port 39176 failure consumed.
- NA-0520 / manual / NA-0531 successes consumed.
- Local port state recorded.
- Remote port state recorded without remote file writes.
- Proof-root SSH config checked for no `ClearAllForwardings yes`.
- Dedicated key used for forwarding attempts.
- `ExitOnForwardFailure` used.
- If marker probe runs, marker/ACK result recorded.
- If forward fails, sanitized failure evidence and likely-cause classification recorded.
- No qsc E2EE.
- No qsc send/receive.
- No qsl-server/qsl-attachments.
- Cleanup completed.
- Exactly one successor selected.

## Future validation / marker plan

Future NA-0534 markers:

- `NA0534_D436_PORT_FAILURE_CONSUMED_OK`
- `NA0534_NA0520_FORWARDING_SUCCESS_CONSUMED_OK`
- `NA0534_NA0531_TRIGGER_SUCCESS_CONSUMED_OK`
- `NA0534_PORT_STATE_CHECKED_OK`
- `NA0534_DEDICATED_KEY_USED_OK`
- `NA0534_PROOF_ROOT_CONFIG_SAFE_OK`
- `NA0534_EXIT_ON_FORWARD_FAILURE_USED_OK`
- `NA0534_REMOTE_FILE_WRITE_ABSENT_OK`
- `NA0534_MARKER_TRAVERSAL_RESULT_RECORDED_OK`
- `NA0534_SSH_DEBUG_LOG_REDACTED_OK`
- `NA0534_NO_QSC_E2EE_OK`
- `NA0534_NO_QSC_SEND_RECEIVE_OK`
- `NA0534_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0534_CLEANUP_COMPLETED_OK`
- `NA0534_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0534_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0534_ONE_READY_INVARIANT_OK`

## No remote action in NA-0533

NA-0533 ran no SSH, scp, sftp, rsync, remote command execution, remote E2EE, ssh-keygen, ssh-keyscan, sudo/admin command, remote source checkout/build, key installation, authorized_keys read/mutation, sshd_config read/mutation, known_hosts mutation, SSH config mutation, qwork/qstart/qresume, qsl-backup, qsl-server, or qsl-attachments.

## No qsc send/receive in NA-0533

NA-0533 ran no qsc E2EE, no qsc send, no qsc receive, and no qsc identity/contact/handshake/relay protocol commands.

## No qsl-server / no qsl-attachments boundary

qsl-server and qsl-attachments remain deferred. NA-0533 does not use, mutate, depend on, or authorize qsl-server or qsl-attachments as a workaround for the direct qsc reverse-forwarding sprint.

## Public claim / website / external review boundary

NA-0533 does not change website/public docs and does not make public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, identity-complete, trust-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claims.

## Backup-impact statement

Backup impact: none. qsl-backup was checked read-only for helper hash and source inclusion count only. qsl-backup was not executed or mutated, and no backup/restore path was changed.

## Rejected alternatives

- Immediate qsc E2EE retry: rejected because D436 failed before marker/ACK traversal.
- qsl-server/qsl-attachments integration: rejected as out of direct qsc sprint scope.
- Alternate remote listen port: rejected because the current dedicated key is restricted to `127.0.0.1:39176`.
- Broad key/sshd remediation: rejected without diagnostic proof and explicit operator action.
- Remote cleanup/killing sessions by Codex: rejected because NA-0534 may clean only local proof-root processes unless a later directive authorizes operator-assisted remote action.
- Ignoring the regression because NA-0520/NA-0531 once passed: rejected because D436 is the current live failure on the same port.

## Next recommendation

Merge the NA-0533 authorization PR only after required checks pass. If post-merge public-safety is green inside the short attach/early-failure window, close out NA-0533 separately to restore NA-0534 READY. Otherwise stop with NA-0533 still READY and hand off for closeout after checks attach green.
