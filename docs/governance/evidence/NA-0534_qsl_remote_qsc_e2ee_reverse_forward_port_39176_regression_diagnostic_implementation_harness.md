Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-06-24

# NA-0534 QSL Remote qsc E2EE Reverse-Forward Port 39176 Regression Diagnostic Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0534 diagnosed the reverse-forward port `39176` regression without running qsc E2EE, qsc send, qsc receive, or any qsc identity/contact/handshake/relay protocol command. It compared D438/D437/D436/D435/D414/D413 inheritance, checked local and remote loopback port state, built a proof-root SSH config with the dedicated forwarding key, ran a transient remote loopback bind probe, and then ran one corrected single-lifetime integrated listener / reverse-forward / stdin-script trigger probe.

Result classification: `REMOTE_FORWARD_PORT_39176_DIAGNOSTIC_MARKER_TRAVERSAL_PASS`.

Selected successor: `NA-0535 -- QSL Remote qsc E2EE Wrong-Peer / Stale-Trust Retry After Port Diagnostic Implementation Harness`.

The first integrated marker attempt was a local harness lifetime failure: listener and forward processes started in separate command invocations did not survive to the trigger. That failure was recovered in place by rerunning the bounded listener / reverse-forward / remote port-state / stdin trigger / cleanup sequence in one shell lifetime with trap-based cleanup. The corrected attempt proved the remote listener on `127.0.0.1:39176`, marker traversal, ACK receipt, and cleanup.

## Live NA-0534 scope

Allowed checked-in mutation paths for this implementation PR are this evidence file, `tests/NA-0534_qsl_remote_qsc_e2ee_reverse_forward_port_39176_regression_diagnostic_implementation_testplan.md`, `DECISIONS.md`, `TRACEABILITY.md`, and `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Proof root: `/srv/qbuild/tmp/NA0534_reverse_forward_port_39176_regression_diagnostic_impl_20260624T150000Z`.

This lane used only proof-root-local runtime files, local loopback checks, bounded read-only remote boundary and port-state checks, one optional transient remote loopback bind probe, one integrated marker/ACK probe, and local cleanup proof. It did not mutate `NEXT_ACTIONS.md`, runtime source, qsc source/test/fuzz/Cargo paths, dependency/lockfile paths, workflow/script/helper paths, corpus/vector/input paths, formal/refimpl/service/public/backup paths, qsl-server, qsl-attachments, SSH keys, authorized_keys, known_hosts, SSH config outside the proof root, remote files, remote temp files, remote source, or qsl-backup.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

The required qwork proof files existed and were copied into the proof root:

- `/srv/qbuild/work/NA-0534/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0534/.qwork/startup.qsl-protocol.json`

The `.kv` and `.json` proofs matched the required startup state: `startup_result=OK`, lane `NA-0534`, repo `qsl-protocol`, path `/srv/qbuild/work/NA-0534/qsl-protocol`, clean worktree/index/untracked state, `head_equals_origin_main=yes`, `ready_count=1`, queue top READY `NA-0534`, and requested lane status READY.

Proof HEAD and proof `origin/main` both matched the live pre-fetch refs at `8ba332e07f84`. Fetch occurred only after that match and after disk proof showed `/` usage 92%, below the 95% stop threshold.

Startup queue/decision proof on current main:

- READY_COUNT 1.
- READY NA-0534.
- NA-0533 DONE.
- NA-0532 DONE.
- NA-0531 DONE.
- D-1056 exists once.
- D-1057 exists once.
- D-1058 absent before patch.
- D-1059 absent before patch.
- Duplicate decision ID count 0 using the `- **ID:** D-####` parser.

Current main health proof on `8ba332e07f84`:

- `public-safety`: completed success.
- `advisories`: completed success.
- no completed red required check was present in the check-runs payload.

Lockfile and backup boundary proof:

- root `Cargo.lock` retained `quinn-proto 0.11.15`.
- nested `qsl/qsl-client/qsc/fuzz/Cargo.lock` retained `quinn-proto 0.11.15`.
- no local `Cargo.toml` drift existed.
- `/usr/local/sbin/qsl-backup` matched SHA-256 `e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`.
- `/home/victor/work/qsl/codex/ops` appeared exactly once in the installed qsl-backup source list.
- qsl-backup was not executed.

## D438 / D437 / D436 / D435 / D414 / D413 inheritance

D438 was discovered exactly once at `/home/victor/work/qsl/codex/responses/NA0533_20260624T143842Z_D438.md`. It records NA-0533 DONE, NA-0534 READY, D-1056 and D-1057 each once, final public-safety/advisories green, no remote action in NA-0533, and classification `REMOTE_FORWARD_PORT_39176_REGRESSION_DIAGNOSTIC_IMPLEMENTATION_READY`.

D437 records NA-0532 closeout restoring NA-0533, with D436 classification `REMOTE_E2EE_INTEGRATED_FORWARDING_PRECHECK_FAILURE`.

D436 records the live regression symptom: the local listener was ready, the local trigger rehearsal passed, the dedicated-key reverse-forward failed before remote trigger execution with `remote port forwarding failed for listen port 39176`, no marker traversed, no ACK occurred, no qsc E2EE ran, no qsc send/receive ran, and cleanup passed.

D435 records classification `REMOTE_TRIGGER_QUOTING_REMEDIATION_MARKER_TRAVERSAL_PASS`. Its stdin-script trigger shape worked: `ssh -T inspiron 'python3 -' < <proof-root trigger script>`. Marker traversal and ACK passed with no qsc E2EE and no qsc send/receive.

D414 records classification `SSH_FORWARDING_CAPABILITY_PROBE_PASS`. The dedicated-key reverse-forward previously carried a synthetic marker and returned an ACK with no qsc E2EE, no qsc send/receive, no qsl-server, no qsl-attachments, and no remote file write.

D413 records accepted dedicated forwarding key proof:

- fingerprint `SHA256:7guT/Hd72vuu+u1Jrwpuy0hSFETVrxYaXHwupuFfaPM`;
- public key comment `qsl-inspiron-qslcodex-forward-20260622`;
- loopback-only `permitlisten` and `permitopen` on `127.0.0.1:39176`;
- PTY absent;
- agent forwarding absent;
- X11 forwarding absent;
- qslcodex non-sudo;
- no backup/qwork/qsl-backup exposure.

NA-0534 therefore inherited a narrow transport diagnostic only. It did not inherit authority to run qsc E2EE, qsc send/receive, qsc protocol commands, qsl-server, or qsl-attachments.

## Command manifest

The command manifest was written before remote diagnostics:

- `$PROOF_DIR/command_manifest/na0534_command_manifest.md`
- `$PROOF_DIR/command_manifest/na0534_command_manifest.json`

It records the exact safe `ssh -G` parse, proof-root SSH config fields, local port-state commands, remote boundary command, remote port-state command, optional remote bind probe command, reverse-forward command, stdin-script trigger command, debug command if a reverse-forward failure needed it, cleanup commands, and negative boundaries.

The manifest explicitly records no qsc E2EE, no qsc send/receive, no qsc protocol command, no qsl-server/qsl-attachments, no remote file write, no remote temp file, and no private-key content read.

Recovered manifest amendment: after the first integrated probe failed due local process lifetime across separate command invocations, the manifest was amended to record the corrected single-shell integrated probe driver: `timeout 60 $PROOF_DIR/forwarding_probe/integrated_probe_na0534_attempt2.sh`.

## Safe SSH config and local port state

Safe `ssh -G inspiron` parsing captured only allowed fields. The operational alias still reports `clearallforwardings=yes`, matching D436 inheritance, so NA-0534 did not use that alias for reverse forwarding.

The proof-root SSH config used:

- Host alias `proof-root-local`.
- `HostName inspiron`.
- `User qslcodex`.
- `IdentityFile /home/victor/.ssh/qslcodex_forward_ed25519`.
- `IdentitiesOnly yes`.
- `PreferredAuthentications publickey`.
- `PasswordAuthentication no`.
- `BatchMode yes`.
- `StrictHostKeyChecking yes`.
- `ForwardAgent no`.
- `ForwardX11 no`.
- `RequestTTY no`.
- `ControlMaster no`.
- `ControlPersist no`.
- `ClearAllForwardings no`.

`ClearAllForwardings yes` count in the proof-root SSH config was 0. The dedicated key path existed and was readable; Codex did not read private key contents.

Local preflight:

- local `127.0.0.1:39176` was free before the probe.
- no current proof-root listener/forward process was present before the probe.
- local bind rehearsal for the remote bind-probe script printed `NA0534_REMOTE_LOOPBACK_BIND_PROBE_OK`.

## Remote boundary and port state

Remote boundary checks were run through the operational alias by sending a local script to `sh -s`; no remote file was written.

Remote boundary result:

- `REMOTE_USER=qslcodex`.
- `REMOTE_UID=1003`.
- `REMOTE_GROUPS=qslcodex`.
- `REMOTE_SUDO_N_TRUE=EXPECTED_FAIL`.
- `REMOTE_BACKUP_QSL=ABSENT_OR_UNREADABLE`.
- `REMOTE_QWORK=ABSENT`.
- `REMOTE_QSL_BACKUP=ABSENT`.

Remote pre-probe port state used `ss` and showed no listener on port `39176`:

- `REMOTE_PORT_STATE_CHECK=START`.
- `REMOTE_PORT_TOOL=ss`.
- `REMOTE_PORT_STATE_CHECK=END`.

Remote port-state while the corrected reverse forward was active showed the expected loopback listener:

`LISTEN 0      128    127.0.0.1:39176 0.0.0.0:*`

Remote port-state after cleanup showed no listener.

## Optional transient remote loopback bind probe result or omission

The optional transient remote loopback bind probe ran because pre-probe remote port-state showed no listener. It used `python3 -` over SSH stdin, bound only `127.0.0.1:39176`, wrote no remote files, and exited immediately.

Remote bind probe result: `NA0534_REMOTE_LOOPBACK_BIND_PROBE_OK`.

## Known-good integrated marker/ACK probe result or failure

Marker: `QSL_PORT_39176_REGRESSION_DIAGNOSTIC_NA0534_20260624T150000Z`.

Expected ACK: `NA0534_TUNNEL_ACK_OK`.

First attempt result: recovered local harness lifetime failure. The listener and reverse-forward were started in separate command invocations; by the time the remote trigger ran, local background processes had exited, and the trigger saw `ConnectionRefusedError`. No marker result was produced in that attempt. This was not accepted as transport evidence.

Corrective action: reran the bounded sequence in one shell lifetime with trap-based cleanup:

- local listener started and wrote `ATTEMPT2_LISTENER_READY`;
- dedicated-key reverse-forward used the proof-root SSH config and `ExitOnForwardFailure=yes`;
- reverse-forward stayed alive after startup: `ATTEMPT2_SSH_FORWARD_ALIVE_AFTER_STARTUP`;
- remote `ss` showed `127.0.0.1:39176` listening during the forward;
- stdin-script remote trigger printed `NA0534_TRIGGER_ACK_RECEIVED_OK`;
- listener result was `{"ok": true, "marker_match": true, "ack_sent": true, "received_length": 60}`;
- corrected driver printed `ATTEMPT2_MARKER_ACK_PASS`.

## SSH debug log redaction proof if logs captured

No SSH `-vvv` debug log was captured because the corrected reverse-forward did not fail. The proof-root config and command output were still scanned later for private key blocks and token-like material.

## Cleanup proof

Cleanup was trap-based in the corrected integrated probe and verified after the run:

- `LOCAL_PORT_39176_CLOSED`.
- `LOCAL_PROOF_PROCESSES_ABSENT`.
- `REMOTE_PORT_39176_NOT_LISTENING_AFTER_CLEANUP`.

No remote cleanup was required because no remote files or remote temp files were written.

## Result classification

Selected classification: `REMOTE_FORWARD_PORT_39176_DIAGNOSTIC_MARKER_TRAVERSAL_PASS`.

This means port-state and marker/ACK diagnostics passed after the local harness lifetime issue was corrected. It does not prove qsc protocol correctness.

Evidence markers:

- `REMOTE_FORWARD_PORT_39176_DIAGNOSTIC_MARKER_TRAVERSAL_PASS`
- `NA0534_REMOTE_LOOPBACK_BIND_PROBE_OK`
- `NA0534_TRIGGER_ACK_RECEIVED_OK`
- `ATTEMPT2_MARKER_ACK_PASS`
- `NA0534_NO_QSC_E2EE_OK`
- `NA0534_NO_QSC_SEND_RECEIVE_OK`
- `NA0534_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0534_NO_REMOTE_FILE_WRITE_OK`
- `NA0534_CLEANUP_COMPLETED_OK`
- `NA0534_ONE_READY_INVARIANT_OK`

## Hostile Cryptographer Review

Marker traversal proves only transport setup behavior for one synthetic marker in this bounded harness. It does not prove E2EE, identity correctness, trust correctness, replay resistance, downgrade resistance, side-channel safety, secret lifecycle, or qsc protocol correctness.

qsc E2EE remains deferred until a later lane. qsc send/receive remains deferred until a later lane.

## Red-Team Review

The run rejected remote file writes, remote temp files, non-loopback binds, private-key exposure, authorized_keys leakage, sshd_config reading/mutation, and qsl-server/qsl-attachments use.

If a later lane indicates key or sshd policy drift, that lane must select an operator-proof review rather than Codex reading authorized_keys or sshd_config directly. If stale local proof-root processes appear, Codex may clean only its own proof-root processes. If remote system/session state becomes implicated, a later operator-assisted diagnostic or proof-review successor is required.

## Production SRE Review

The port regression diagnostic was operationally necessary before more remote qsc E2EE attempts. Logs stayed proof-root-local and checked-in evidence uses summaries. Cleanup proof covers the local listener, SSH forward process, local port state, and remote loopback listener disappearance after cleanup.

qwork, qsl-backup, production data, qsl-server, and qsl-attachments remained isolated. No public-readiness, production-readiness, or public-internet-readiness claim is made.

## Release-Claim Boundary Review

This lane makes no public-readiness claim, no production-readiness claim, no public-internet-readiness claim, no external-review-complete claim, no crypto-complete claim, no identity-complete claim, no trust-complete claim, no replay-proof claim, no downgrade-proof claim, no secret-material-complete claim, no side-channel-free claim, no vulnerability-free claim, no bug-free claim, and no perfect-crypto claim.

Formal-Model Mapping Residual: NA-0534 changes no formal model and adds no protocol transition. Formal checks remain a validation gate, not evidence that this marker path proves protocol security.

Assurance Gap Review Trigger: if the selected NA-0535 retry later fails before qsc E2EE, the next lane must classify the exact failure instead of silently bypassing the marker/ACK gate.

## Successor selection

Because marker traversal passed, select:

`NA-0535 -- QSL Remote qsc E2EE Wrong-Peer / Stale-Trust Retry After Port Diagnostic Implementation Harness`

Do not select public/production readiness.

## Future scope bundle

The selected NA-0535 successor should consume this result before any qsc E2EE retry and should preserve:

- fresh qwork proof verification without running qwork/qstart/qresume;
- retained remote qsc hash and metadata recheck before qsc use;
- integrated listener / reverse-forward / stdin-trigger marker precheck before qsc E2EE;
- synthetic identities/messages/trust records only;
- isolated local/remote roots;
- no qsl-server/qsl-attachments;
- cleanup/retention proof;
- no public/production readiness claims.

## Future validation / marker plan

Future NA-0535 should record:

- qwork proof freshness;
- `NA0534_REMOTE_LOOPBACK_BIND_PROBE_OK` consumed;
- `NA0534_TRIGGER_ACK_RECEIVED_OK` consumed;
- proof-root SSH config with dedicated key and no `ClearAllForwardings yes`;
- local listener bind to `127.0.0.1:39176`;
- remote listener visible on `127.0.0.1:39176` during forward;
- marker match true;
- ACK sent and received;
- cleanup local and remote port-state proof;
- no qsc E2EE before marker/ACK gate;
- no qsl-server/qsl-attachments.

## No qsc E2EE

NA-0534 ran no qsc E2EE. It did not run baseline E2EE, wrong-peer negative, stale-trust negative, or any qsc protocol command.

## No qsc send/receive

NA-0534 ran no qsc send and no qsc receive.

## No qsl-server / no qsl-attachments boundary

NA-0534 did not use qsl-server or qsl-attachments. No qsl-server or qsl-attachments path was mutated.

## No remote file write boundary

Remote commands were sent over SSH stdin and wrote no remote files and no remote temp files. The remote bind probe bound loopback only and exited immediately. No remote cleanup was needed.

## Backup-impact statement

Backup impact: none. qsl-backup was checked read-only by digest and source inclusion count and was not executed or mutated. `/backup/qsl` was checked for disk watermark and remote unreadability only and was not mutated.

## Rejected alternatives

- Proceed directly to qsc E2EE after D436: rejected because marker/ACK had to be re-proven after the port regression.
- Use a non-loopback remote bind: rejected because the dedicated-key proof is loopback-only.
- Change the listen port: rejected because the dedicated key and evidence lane are scoped to `127.0.0.1:39176`.
- Read authorized_keys or sshd_config directly: rejected because the directive forbids it.
- Use qsl-server or qsl-attachments to bypass forwarding: rejected as out of scope.
- Treat the first failed local harness lifetime attempt as remote transport evidence: rejected because the local processes had exited before the trigger.

## Next recommendation

Merge the NA-0534 evidence PR after local validation and required checks pass. If post-merge public-safety is green inside the short attach/early-failure window, close out NA-0534 and restore the selected NA-0535 wrong-peer / stale-trust retry successor as the sole READY item. The closeout must not implement NA-0535.
