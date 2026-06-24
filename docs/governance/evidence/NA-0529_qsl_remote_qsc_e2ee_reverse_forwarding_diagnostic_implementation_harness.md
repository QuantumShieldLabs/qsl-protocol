Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-24

# NA-0529 - QSL Remote qsc E2EE Reverse-Forwarding Diagnostic Implementation Harness

## Executive summary

NA-0529 executed the bounded reverse-forwarding diagnostic authorized by NA-0528. The run verified qwork proof files, consumed D429/D428/D427/D414/D413 inheritance, compared the D427 failed forwarding shape with the NA-0520 known-good marker-forwarding shape, built a proof-root command manifest, parsed only safe SSH config fields, created a proof-root SSH config with the dedicated forwarding key and no `ClearAllForwardings yes`, ran read-only remote boundary checks, started a local loopback synthetic listener, and started the dedicated-key reverse-forward session with `ExitOnForwardFailure=yes`.

The dedicated-key reverse-forward session started and remained alive, so the D427 `remote port forwarding failed for listen port 39176` symptom was not reproduced in this run. The single authorized remote trigger did not complete marker traversal: the trigger output was `n`, the remote shell reported a connection reset while reading the ACK, and the SSH forward stderr recorded `connect_to 127.0.0.1 port 39176: failed.` A corrected remote trigger was not run because this directive allowed exactly one remote loopback trigger. Result classification: `REMOTE_FORWARDING_DIAGNOSTIC_REMOTE_TRIGGER_FAILURE`.

No qsc E2EE ran. No qsc send/receive ran. No qsl-server or qsl-attachments path was used. No remote file was written. Cleanup completed and no proof-root listener or SSH process remained.

## Live NA-0529 scope

Allowed checked-in mutation paths for the implementation PR are this evidence file, `tests/NA-0529_qsl_remote_qsc_e2ee_reverse_forwarding_diagnostic_implementation_testplan.md`, `DECISIONS.md`, `TRACEABILITY.md`, and `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

The proof root was `/srv/qbuild/tmp/NA0529_reverse_forwarding_diagnostic_impl_20260624T003732Z`. Runtime artifacts remained under that proof root. No qsc source/test/fuzz/Cargo path, workflow/script/helper path, dependency/lockfile path, corpus/vector/input path, formal/refimpl/service/public/backup path, qsl-server path, or qsl-attachments path was mutated.

Markers:

- `NA0529_QWORK_PROOF_VERIFIED_OK`
- `NA0529_D427_NA0520_DIFF_RECORDED_OK`
- `NA0529_COMMAND_MANIFEST_RECORDED_OK`
- `NA0529_SAFE_SSH_CONFIG_VERIFIED_OK`
- `NA0529_REMOTE_BOUNDARY_CHECKED_OK`
- `NA0529_LOOPBACK_LISTENER_STARTED_OK`
- `NA0529_REVERSE_FORWARD_STARTED_OK`
- `NA0529_REMOTE_TRIGGER_FAILURE_RECORDED_OK`
- `NA0529_CLEANUP_COMPLETED_OK`
- `NA0529_NO_QSC_E2EE_OK`
- `NA0529_NO_QSC_SEND_RECEIVE_OK`
- `NA0529_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0529_NO_REMOTE_FILE_WRITE_OK`

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

The following qwork proof files existed and were copied into the proof root:

- `/srv/qbuild/work/NA-0529/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0529/.qwork/startup.qsl-protocol.json`

The `.kv` proof passed the required fields: `startup_result=OK`, lane `NA-0529`, repo `qsl-protocol`, path `/srv/qbuild/work/NA-0529/qsl-protocol`, clean worktree/index/untracked state, `head_equals_origin_main=yes`, `ready_count=1`, `queue_top_ready=NA-0529`, and `requested_lane_status=READY`.

The proof HEAD and proof `origin/main` both matched the live pre-fetch refs at `43d7a728a57f`. Fetch occurred only after that match. Disk usage before fetch was `/` 89% and `/backup/qsl` 27%, below the 95% stop threshold.

## D429 / D428 / D427 / D414 / D413 inheritance

D429 confirmed NA-0528 DONE, NA-0529 READY, D-1047 present once, D-1048 absent, and no remote action in the D429 closeout.

D428 classified NA-0528 as `REMOTE_FORWARDING_DIAGNOSTIC_IMPLEMENTATION_READY`, selected NA-0529, and authorized no remote SSH and no qsc execution in NA-0528.

D427 classified NA-0527 as `REMOTE_E2EE_FORWARDING_RECHECK_FAILURE`. The retained remote qsc recheck passed for `$HOME/qsl-remote-test/bin/qsc`, owner `qslcodex`, mode `700`, size `102103920`, SHA-256 `6bf9e59fdae397c2e0f88538d700cccbee80d229c6a979cc79555e39fea2b4f7`, and remote `--help` success. Remote boundary checks passed. The local qsc relay started on loopback port `39176`. The first forwarding attempt failed because proof-root SSH config had `ClearAllForwardings yes`; after correction, the next attempts failed with `remote port forwarding failed for listen port 39176`. A read-only remote bind probe showed remote `127.0.0.1:39176` available. No baseline E2EE, no qsc send/receive, no qsl-server, and no qsl-attachments use occurred.

D414 classified NA-0520 as `SSH_FORWARDING_CAPABILITY_PROBE_PASS`. NA-0520 used the dedicated forwarding key, a local listener bound to `127.0.0.1:39176`, `ssh -N -T`, `ExitOnForwardFailure=yes`, and `-R 127.0.0.1:39176:127.0.0.1:39176`; a synthetic marker traversed and returned an ACK. No qsc E2EE or qsc send/receive occurred.

D413 accepted the dedicated forwarding key proof with fingerprint `SHA256:7guT/Hd72vuu+u1Jrwpuy0hSFETVrxYaXHwupuFfaPM`, public key comment `qsl-inspiron-qslcodex-forward-20260622`, loopback-only `permitlisten` and `permitopen` on `127.0.0.1:39176`, no PTY, no agent forwarding, no X11 forwarding, qslcodex non-sudo, no backup/qwork/qsl-backup exposure, and a `command="/bin/false"` compatibility caveat that NA-0520 later proved compatible for forwarding.

qsl-server and qsl-attachments remain out of scope. qsc E2EE is not permitted in NA-0529.

## D427 vs NA-0520 command/config diff

Proof files:

- `command_diff/d427_vs_na0520_forwarding_diff.md`
- `command_diff/d427_vs_na0520_forwarding_diff.json`

Recorded differences:

- D427 first failure: proof-root SSH config had `ClearAllForwardings yes`.
- D427 corrected failure: `remote port forwarding failed for listen port 39176`.
- D427 final failure: same remote port-forwarding error.
- D427 remote bind proof: remote `127.0.0.1:39176` appeared available.
- NA-0520 known-good command family: `ssh -N -T -o ExitOnForwardFailure=yes -R 127.0.0.1:39176:127.0.0.1:39176 ...`.
- NA-0520 marker traversal succeeded and ACK returned.
- NA-0520 used a synthetic one-connection marker listener, while D427 used a qsc relay and stopped before qsc E2EE after forwarding failed.
- NA-0529 therefore used the NA-0520 synthetic marker shape and did not run qsc E2EE.

The diff proved that using the operational alias config shape directly for a forwarding session is unsafe because `ssh -G inspiron` reported `clearallforwardings yes`. NA-0529 created a separate proof-root SSH config from the NA-0520 known-good shape and verified effective `clearallforwardings no`.

## Command manifest

Proof files:

- `command_diff/na0529_command_manifest.md`
- `command_diff/na0529_command_manifest.json`

The manifest was written before SSH execution and included:

- safe `ssh -G inspiron` parsing of hostname, user, identityfile basename only, identitiesonly, passwordauthentication, batchmode, stricthostkeychecking, forwardagent, forwardx11, and clearallforwardings;
- read-only remote boundary check command;
- local loopback listener command;
- dedicated-key reverse-forward command with `-N -T`, `ExitOnForwardFailure=yes`, and loopback-only `-R`;
- remote trigger command only after forwarding starts;
- optional sanitized debug logging if forwarding fails;
- cleanup commands and stop conditions;
- explicit no qsc E2EE, no qsc send/receive, no qsl-server/qsl-attachments, and no remote file write boundaries.

## Safe SSH config and remote boundary check

Safe `ssh -G inspiron` parsing recorded:

- user: `qslcodex`
- hostname: `inspiron`
- batchmode: `yes`
- operational alias clearallforwardings: `yes`
- forwardx11: `no`
- identitiesonly: `yes`
- passwordauthentication: `no`
- stricthostkeychecking: `true`
- identityfile basename: `qslcodex_ed25519`
- forwardagent: `no`

The proof-root forwarding config used host alias `proof-root-local`, host `inspiron`, user `qslcodex`, dedicated identityfile basename `qslcodex_forward_ed25519`, `IdentitiesOnly yes`, public-key-only authentication, `PasswordAuthentication no`, `BatchMode yes`, `StrictHostKeyChecking yes`, `ForwardAgent no`, `ForwardX11 no`, and `RequestTTY no`. It did not include `ClearAllForwardings yes`.

Effective proof-root config parsing recorded:

- user: `qslcodex`
- hostname: `inspiron`
- batchmode: `yes`
- clearallforwardings: `no`
- forwardx11: `no`
- identitiesonly: `yes`
- passwordauthentication: `no`
- requesttty: `false`
- stricthostkeychecking: `true`
- identityfile basename: `qslcodex_forward_ed25519`
- forwardagent: `no`

Remote boundary proof through bounded operational SSH recorded:

- remote user `qslcodex`;
- remote UID `1003`, not root;
- no privileged group overlap;
- `sudo -n true` failed as expected;
- `/backup/qsl` not readable;
- `qwork` absent from the remote PATH;
- `qsl-backup` absent from the remote PATH;
- remote port-state tool `ss` found no listener lines for `39176`.

No remote file was written. No qsc command was run.

## Local listener proof

Local port precheck proved `LOCAL_PORT_39176_FREE=yes`.

The proof-root listener bound only to `127.0.0.1:39176`, accepted at most one connection, expected marker `QSL_FORWARDING_DIAGNOSTIC_SYNTHETIC_NA0529_20260624T003732Z`, and would return `NA0529_TUNNEL_ACK_OK` on an exact match. It printed `NA0529_LOOPBACK_LISTENER_STARTED_OK`.

The listener did not record a marker traversal summary because the single remote trigger failed before a valid marker/ACK exchange completed. The missing listener summary is part of the failure evidence, not a success marker.

## Reverse-forward attempt proof

The dedicated-key reverse-forward command family was:

`ssh -F <proof-root-config> -N -T -o ExitOnForwardFailure=yes -R 127.0.0.1:39176:127.0.0.1:39176 proof-root-local`

Proof recorded:

- `NA0529_REVERSE_FORWARD_STARTED_OK`
- `NA0529_EXIT_ON_FORWARD_FAILURE_OK`
- dedicated key basename `qslcodex_forward_ed25519`
- loopback-only remote listen and local target
- no PTY, no agent forwarding, no X11 forwarding
- no qsc E2EE and no qsc send/receive

Because the reverse-forward process stayed alive after the startup window, D427's corrected remote-port-forwarding failure was not reproduced by this attempt.

## Remote trigger / marker traversal proof or failure

The single authorized remote trigger was run only after the reverse-forward session was alive. It attempted to connect to remote loopback `127.0.0.1:39176`, send the synthetic marker, and read one ACK.

Failure evidence:

- trigger stdout was the unexpected fixed string `n`;
- trigger stderr reported `read error: Connection reset by peer`;
- reverse-forward stderr recorded `connect_to 127.0.0.1 port 39176: failed.`;
- no listener JSON marker-traversal summary was produced;
- no `NA0529_TUNNEL_ACK_RECEIVED_OK` marker was recorded.

This is classified as `REMOTE_FORWARDING_DIAGNOSTIC_REMOTE_TRIGGER_FAILURE`. A corrected trigger was not run because this directive allowed exactly one remote loopback trigger. The next lane must diagnose the trigger command shape and listener/target connection timing before retrying qsc E2EE.

## Debug-log redaction proof

SSH verbose debug logging was not captured because the reverse-forward session itself started successfully; Packet F required debug capture only if forwarding failed. Proof-root logs were scanned for private key block markers and high-likelihood token prefixes. Finding count was zero.

Checked-in evidence includes only redacted command families, safe config fields, key basenames, and failure summaries. It does not include private key contents, passphrases, tokens, passwords, raw authorized_keys, full SSH config, known_hosts dumps, or sshd_config.

## Cleanup proof

Cleanup terminated the reverse-forward SSH process and verified the listener PID was already absent. A first cleanup parser incorrectly counted the `ss` header row as a live listener; that was recorded as a recoverable proof-parser failure and corrected. The corrected cleanup summary found:

- no listener line for port `39176`;
- no proof-root process line after cleanup;
- cleanup result `ok=true`.

No remote cleanup was needed because no remote files were written.

## Result classification

Selected classification:

`REMOTE_FORWARDING_DIAGNOSTIC_REMOTE_TRIGGER_FAILURE`

The reverse-forward path started with the dedicated key and `ExitOnForwardFailure=yes`, but the single remote trigger did not produce a marker traversal. This result does not prove the D427 forwarding failure is fixed and does not authorize qsc E2EE retry.

## Hostile Cryptographer Review

Marker traversal would prove only SSH forwarding transport for one synthetic line. This run did not even prove marker traversal because the trigger failed. It does not prove qsc protocol correctness, E2EE, identity correctness, trust correctness, replay resistance, downgrade resistance, side-channel safety, or secret lifecycle.

qsc E2EE remains a later lane after the forwarding and trigger path is proven.

## Red-Team Review

Non-loopback bind was rejected by construction: all listener and forwarding endpoints were `127.0.0.1:39176`.

Private key exposure, authorized_keys leakage, sshd_config mutation, known_hosts mutation, remote file writes, and qsl-server/qsl-attachments use remained forbidden. The dedicated-key session started, so this run does not justify weakening key policy. The failed remote trigger should be diagnosed through a narrower proof-root command-shape lane, not by broadening SSH policy or rerunning E2EE.

## Production SRE Review

The diagnostic was operationally useful because it narrowed D427's blocker: the dedicated-key reverse-forward startup succeeded in this run, while the remote trigger/target connection path failed. Logs stayed proof-root-local and were summarized with redaction. Cleanup proof covers local listener and SSH processes. qwork, qsl-backup, production data, qsl-server, and qsl-attachments stayed isolated.

No public-readiness claim, no production-readiness claim, and no public-internet-readiness claim is made.

## Release-Claim Boundary Review

This evidence makes no external-review-complete claim, no crypto-complete claim, no identity-complete claim, no trust-complete claim, no replay-proof claim, no downgrade-proof claim, no secret-material-complete claim, no side-channel-free claim, no vulnerability-free claim, no bug-free claim, and no perfect-crypto claim.

## Successor selection

Because classification is `REMOTE_FORWARDING_DIAGNOSTIC_REMOTE_TRIGGER_FAILURE`, the selected successor is:

`NA-0530 -- QSL Remote qsc E2EE Reverse-Forwarding Remote Trigger Command-Shape Diagnostic Harness`

This successor should diagnose the remote trigger command quoting/shape, listener readiness, and local target connection timing without qsc E2EE, without qsc send/receive, without qsl-server/qsl-attachments, and without remote file writes. It should not be a public/production readiness lane.

## Future scope bundle

Recommended future scope:

- proof-root-only command manifest for the corrected trigger shape;
- local loopback synthetic listener;
- dedicated-key reverse forward using the same safe proof-root config rule: effective `clearallforwardings no`;
- one corrected remote trigger after forwarding starts;
- cleanup proof;
- no qsc E2EE and no qsc send/receive unless a later successor explicitly authorizes it after marker traversal passes.

Out of scope for that successor: qsl-server, qsl-attachments, qsc source/test/fuzz/Cargo mutation, workflow/helper/dependency mutation, remote file writes, authorized_keys reads/mutations, sshd_config reads/mutations, known_hosts mutation, and package installation.

## Future validation / marker plan

Future validation should record:

- qwork proof verified;
- D427/NA-0520/NA-0529 inheritance consumed;
- proof-root effective SSH config still has `clearallforwardings no`;
- listener readiness proof before remote trigger;
- remote trigger command-shape proof using safe quoting;
- marker sent, marker received, ACK returned, or exact failure;
- cleanup proof;
- no qsc E2EE;
- no qsc send/receive;
- no qsl-server/qsl-attachments;
- no remote file write;
- no public-readiness claim and no production-readiness claim.

## No qsc E2EE

No qsc E2EE command was run. The retained qsc binary was not executed in this directive. The diagnostic stopped at SSH forwarding and remote trigger behavior.

## No qsc send/receive

No qsc send command and no qsc receive command was run. No qsc identity/contact/handshake/relay protocol command was run.

## No qsl-server / no qsl-attachments boundary

No qsl-server command, source path, workflow, endpoint, or runtime artifact was used or modified.

No qsl-attachments command, source path, workflow, endpoint, or runtime artifact was used or modified.

## No public/production readiness boundary

This evidence is a diagnostic failure record only. It does not claim public readiness, production readiness, public-internet readiness, crypto completion, identity completion, trust completion, replay proof, downgrade proof, side-channel freedom, vulnerability freedom, bug freedom, or perfect crypto.

## Backup-impact statement

qsl-backup was not executed. Backup/restore was not run. `/backup/qsl` was not mutated. The installed qsl-backup helper was checked read-only during startup and matched the expected digest; the Codex ops source inclusion count was exactly 1.

## Rejected alternatives

- Rerun the malformed remote trigger immediately: rejected because the directive allowed exactly one remote loopback trigger.
- Retry qsc E2EE after reverse-forward startup succeeded: rejected because marker traversal did not pass.
- Broaden SSH policy or inspect authorized_keys/sshd_config directly: rejected because this run did not prove a server-side policy failure and those reads/mutations are out of scope.
- Use qsl-server or qsl-attachments to bypass the diagnostic: rejected as out of scope.
- Treat reverse-forward startup as success for E2EE purposes: rejected because no marker/ACK traversal proof was obtained.

## Next recommendation

Close out NA-0529 only with a successor that matches `REMOTE_FORWARDING_DIAGNOSTIC_REMOTE_TRIGGER_FAILURE`. If no approved successor block is provided for closeout, stop before editing `NEXT_ACTIONS.md` and request explicit successor direction.
