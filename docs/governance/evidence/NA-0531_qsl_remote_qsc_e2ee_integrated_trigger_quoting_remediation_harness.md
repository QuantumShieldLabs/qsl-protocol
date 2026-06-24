Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-06-24

# NA-0531 QSL Remote qsc E2EE Integrated Trigger Quoting Remediation Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0531 consumed the D434/D433/D432/D431/D430 inheritance for the remote integrated trigger quoting failure and executed a bounded marker/ACK remediation harness. The lane did not run qsc E2EE, qsc send, qsc receive, qsc identity/contact/handshake/relay protocol commands, qsl-server, or qsl-attachments.

The retained remote qsc binary was rechecked by metadata/hash only at `/home/qslcodex/qsl-remote-test/bin/qsc`: owner/group `qslcodex/qslcodex`, mode `700`, size `102103920`, and SHA-256 `6bf9e59fdae397c2e0f88538d700cccbee80d229c6a979cc79555e39fea2b4f7`. The binary was not executed.

The corrected trigger used an argv-safe stdin script shape: `ssh -T inspiron 'python3 -' < <proof-root trigger script>`. A single integrated listener / dedicated-key reverse-forward / operational trigger lifetime passed: listener `marker_match` true, `ack_sent` true, and remote trigger stdout included `NA0531_TRIGGER_ACK_RECEIVED_OK`.

Result classification: `REMOTE_TRIGGER_QUOTING_REMEDIATION_MARKER_TRAVERSAL_PASS`.

## Live NA-0531 scope

Allowed checked-in mutation paths for this implementation PR are this evidence file, `tests/NA-0531_qsl_remote_qsc_e2ee_integrated_trigger_quoting_remediation_testplan.md`, `DECISIONS.md`, `TRACEABILITY.md`, and `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Proof root: `/srv/qbuild/tmp/NA0531_integrated_trigger_quoting_remediation_20260624T053219Z`.

No qsc source/test/fuzz/Cargo path, workflow/script/helper path, dependency/lockfile path, corpus/vector/input path, formal/refimpl/service/public/backup path, qsl-server path, qsl-attachments path, public-doc path, backup path, or service path is changed.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

The required qwork proof files existed and were copied into the proof root:

- `/srv/qbuild/work/NA-0531/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0531/.qwork/startup.qsl-protocol.json`

The `.kv` and `.json` proofs matched the required startup state: `startup_result=OK`, lane `NA-0531`, repo `qsl-protocol`, path `/srv/qbuild/work/NA-0531/qsl-protocol`, clean worktree/index/untracked state, `head_equals_origin_main=yes`, `ready_count=1`, queue top READY `NA-0531`, and requested lane status READY.

Proof HEAD and proof `origin/main` both matched the live pre-fetch refs at `1b4ad9278d13`. Fetch occurred only after that match and after disk proof showed `/` usage below the 95% stop threshold.

## D434 / D433 / D432 / D431 / D430 inheritance

NA-0530 was DONE and NA-0531 was READY at startup.

D434 consumed D433 and restored NA-0531 as the sole READY item. It recorded D-1051 count 1, D-1052 count 0, and classification `REMOTE_E2EE_INTEGRATED_TRIGGER_QUOTING_FAILURE`.

D433 recorded that D432 residue cleanup passed, retained qsc recheck passed, the listener bound to `127.0.0.1:39176`, the reverse-forward process stayed alive with `ExitOnForwardFailure=yes`, and the trigger failed before connecting because the `python3 -c` payload was split by remote shell quoting. D433 listener result was `marker_match false`, `ack_sent false`, and `ack_received false`. D433 recorded no qsc E2EE and no qsc send/receive.

D431 manual integrated forwarding proof passed with `REMOTE_ACK=MANUAL_TUNNEL_ACK_OK`, `INTEGRATED_MARKER_TRAVERSED_OK`, `"marker_match": true`, `"ack_sent": true`, and `"ok": true`, while still requiring an in-lane marker/ACK proof before future qsc E2EE work.

D430 recorded `REMOTE_FORWARDING_DIAGNOSTIC_REMOTE_TRIGGER_FAILURE`.

NA-0531 therefore proved only trigger shape, marker traversal, and ACK receipt before any future qsc E2EE retry.

## Retained qsc metadata/hash recheck

Remote metadata/hash recheck used operational SSH and a stdin shell script with no remote write and no qsc execution.

Result:

- path: `/home/qslcodex/qsl-remote-test/bin/qsc`
- owner/group: `qslcodex/qslcodex`
- mode: `700`
- size: `102103920`
- type: regular file
- SHA-256: `6bf9e59fdae397c2e0f88538d700cccbee80d229c6a979cc79555e39fea2b4f7`

The retained-qsc result matched the expected NA-0526/D425 metadata and hash. No `qsc --help`, qsc E2EE, qsc send/receive, or qsc protocol command ran in this lane.

## Command manifest

Command manifest artifacts were written before SSH execution:

- `/srv/qbuild/tmp/NA0531_integrated_trigger_quoting_remediation_20260624T053219Z/command_manifest/na0531_command_manifest.md`
- `/srv/qbuild/tmp/NA0531_integrated_trigger_quoting_remediation_20260624T053219Z/command_manifest/na0531_command_manifest.json`

The trigger script SHA-256 was `2e14ce3e1e8422f95c302c2efac296c235fb9af75d5d1ba8d9ca07ca60fe61db`.

The safe remote trigger command was:

`ssh -T inspiron 'python3 -' < "/srv/qbuild/tmp/NA0531_integrated_trigger_quoting_remediation_20260624T053219Z/trigger/remote_trigger_na0531.py"`

The dedicated-key reverse-forward command was:

`ssh -F "/srv/qbuild/tmp/NA0531_integrated_trigger_quoting_remediation_20260624T053219Z/forwarding/ssh_config" -N -T -o ExitOnForwardFailure=yes -R 127.0.0.1:39176:127.0.0.1:39176 proof-root-local`

The manifest records no qsc E2EE, no qsc send/receive, no qsc protocol commands, no qsl-server/qsl-attachments, no remote file writes, and no remote temp files.

## Local-only trigger syntax / argv rehearsal

The local trigger script was written under the proof root and compiled with `python3 -m py_compile`.

Local rehearsal command:

`python3 "/srv/qbuild/tmp/NA0531_integrated_trigger_quoting_remediation_20260624T053219Z/trigger/remote_trigger_na0531.py" --rehearse`

Rehearsal output:

- `NA0531_LOCAL_TRIGGER_REHEARSAL_OK`
- `QSL_TRIGGER_QUOTING_REMEDIATION_SYNTHETIC_NA0531_20260624T053219Z_D435`

The marker used only ASCII letters, digits, and underscores, and contained no newline or shell-sensitive splitting characters.

## Safe SSH config and remote boundary check

Safe `ssh -G inspiron` parsing recorded the operational alias hazard `clearallforwardings yes`; Codex did not use that alias for reverse forwarding. The proof-root forwarding config used dedicated key basename `qslcodex_forward_ed25519`, `IdentitiesOnly yes`, `PreferredAuthentications publickey`, `PasswordAuthentication no`, `BatchMode yes`, `StrictHostKeyChecking yes`, `ForwardAgent no`, `ForwardX11 no`, `RequestTTY no`, and `ClearAllForwardings no`.

Effective proof-root config parsing confirmed `clearallforwardings no`.

The dedicated forwarding key is restricted for forwarding. A no-write minimal remote command through that key exited `1` with no output, consistent with the inherited `command="/bin/false"` compatibility caveat. Remote command execution for the boundary check, retained qsc metadata/hash check, and trigger used the operational `inspiron` alias; the reverse-forward remained on the dedicated proof-root config.

Remote boundary check passed:

- remote user `qslcodex`
- UID `1003`, not root
- no privileged group overlap
- `sudo -n true` failed as expected
- `/backup/qsl` absent
- `qwork` absent from PATH
- `qsl-backup` absent from PATH

No authorized_keys, sshd_config, private key material, or full SSH config was read or mutated.

## Integrated listener / forward / trigger precheck

The integrated harness used one controlled lifetime:

1. Started a local listener on `127.0.0.1:39176`.
2. Started the dedicated-key reverse forward with `ExitOnForwardFailure=yes`.
3. Confirmed the forward process was alive before the trigger.
4. Ran the remote trigger as a stdin Python script over the operational alias.
5. Cleaned up listener and forward processes.

Listener readiness used a proof-root ready flag after bind, not a probe connection that would consume the one-shot listener.

## Marker traversal / ACK proof or failure

Primary attempt passed.

Listener result:

- marker: `QSL_TRIGGER_QUOTING_REMEDIATION_SYNTHETIC_NA0531_20260624T053219Z_D435`
- ack: `NA0531_TUNNEL_ACK_OK`
- host/port: `127.0.0.1:39176`
- `marker_match`: true
- `ack_sent`: true
- `ok`: true

Remote trigger stdout:

- `NA0531_REMOTE_TRIGGER_STARTED_OK`
- `NA0531_TRIGGER_MARKER_SENT_OK`
- `NA0531_TRIGGER_ACK_RECEIVED_OK`
- `NA0531_REMOTE_TRIGGER_DONE_OK`

No corrected trigger retry was used because the primary attempt passed.

## Cleanup proof

Harness cleanup reported:

- reverse-forward process terminated
- listener process exited
- `port_held_after_cleanup`: false
- `cleanup_ok`: true

An independent filtered cleanup proof found no concrete NA-0531 listener, harness, or reverse-forward process, and no listening socket on `39176`.

One cleanup proof method attempted an immediate rebind and failed because the recent TCP close left the port temporarily non-rebindable. That was classified as a recoverable proof-method issue. The corrected `ss` listener-state proof passed.

## Result classification

`REMOTE_TRIGGER_QUOTING_REMEDIATION_MARKER_TRAVERSAL_PASS`

This classification means only the integrated remote trigger quoting/argv shape and marker/ACK traversal gate passed. It does not prove qsc protocol correctness.

## Hostile Cryptographer Review

This marker traversal proves only transport/trigger harness correctness for the synthetic marker path. It does not prove E2EE, identity correctness, trust correctness, replay resistance, downgrade resistance, side-channel safety, or secret lifecycle.

qsc E2EE remains a later lane. No qsc E2EE, qsc send, qsc receive, or qsc protocol command ran here.

## Red-Team Review

Remote temp files and remote file writes remained rejected. Private-key exposure, authorized_keys leakage, sshd_config mutation, non-loopback bind, SSH policy weakening, qsl-server use, and qsl-attachments use remained rejected.

The operational command alias was used only for bounded no-write remote commands and the stdin trigger. The dedicated key remained the reverse-forward key.

## Production SRE Review

The trigger remediation is operationally necessary after D433. Logs remained proof-root-local and checked-in evidence summarizes only command shapes, safe fields, markers, hashes, statuses, and classifications.

Cleanup proof covers the local listener and SSH forward processes. qwork, qsl-backup, production data, qsl-server, and qsl-attachments stayed isolated.

## Release-Claim Boundary Review

No public-readiness, production-readiness, public-internet-readiness, crypto-complete, identity-complete, trust-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim is introduced.

Best-Known-Method Review: use proof-root command manifest first, local syntax/argv rehearsal, dedicated-key loopback reverse forwarding, operational stdin trigger for remote command execution, and marker/ACK gate before future qsc E2EE.

Side-Channel Caveat: this lane does not evaluate timing leakage, traffic shape, side channels, or secret lifecycle.

Formal-Model Mapping Residual: no formal model semantic change is made; formal checks remain a validation gate, not evidence that this marker path proves protocol security.

External-Review Readiness: evidence is organized for later review, but no external-review-complete claim is made.

Assurance Gap Review Trigger: future qsc E2EE work must continue to treat identity, trust, replay, downgrade, cleanup, and secret-material boundaries as open until separately tested.

## Successor selection

Selected successor after a passing marker/ACK classification:

`NA-0532 -- QSL Remote qsc E2EE Wrong-Peer / Stale-Trust Retry After Trigger Remediation Implementation Harness`

This implementation PR does not edit `NEXT_ACTIONS.md`. Exactly one READY item remains mandatory until a closeout directive restores the selected NA-0532 block.

## Future scope bundle

Future NA-0532 may use the retained qsc binary only after a fresh hash recheck, integrated listener/forward/trigger precheck, synthetic identities/messages/trust records, isolated local/remote roots, cleanup/retention proof, and the same no qsl-server/no qsl-attachments boundary.

Future NA-0532 must not claim public/production readiness and must preserve fail-closed negative behavior.

## Future validation / marker plan

Future validation should record:

- qwork proof verified without rerunning qwork
- retained qsc metadata/hash rechecked before use
- local trigger rehearsal passed
- proof-root forwarding config effective `clearallforwardings no`
- listener bound to `127.0.0.1:39176`
- dedicated-key reverse forward alive with `ExitOnForwardFailure=yes`
- synthetic marker matched
- ACK sent and received
- cleanup verified
- no qsl-server/no qsl-attachments
- no qsc E2EE before the marker/ACK gate

## No qsc E2EE

No qsc E2EE ran in NA-0531. No baseline E2EE, wrong-peer negative, stale-trust negative, qsc relay, qsc send, qsc receive, qsc identity/contact/handshake/relay protocol command, or qsc protocol command ran.

## No qsc send/receive

No qsc send or qsc receive command ran. The retained qsc binary was read by `stat` and `sha256sum` only.

## No qsl-server / no qsl-attachments boundary

NA-0531 did not use qsl-server or qsl-attachments. Those remain protected out-of-scope architecture boundaries.

## No remote file write boundary

No remote file write, remote temp file, remote source checkout/build, remote package installation, remote cargo/rustup/git operation, key generation, key installation, authorized_keys read/mutation, sshd_config read/mutation, or known_hosts mutation occurred.

Remote scripts were delivered over SSH stdin and did not write remote files.

## Backup-impact statement

The local qsl-backup helper was read-only hashed and matched SHA-256 `e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`. The Codex ops source appears exactly once in the installed helper source list. qsl-backup was not executed. `/backup/qsl` was checked read-only for disk state locally and was absent on the remote boundary check.

## Recovered failures

Recovered proof-query failures: initial inheritance assertions used wording that was too exact for D434/D433. Classification: recoverable proof-query wording issue. Corrective action: extracted key lines and reran semantic checks. Final result: inheritance fact check passed.

Recovered command-shape failure: initial no-write remote command through the dedicated forwarding key exited `1` with no output. Classification: recoverable command-shape issue because the inherited key is forwarding-restricted and no remote write or qsc execution occurred. Corrective action: retained the dedicated key for reverse forwarding and used operational alias `inspiron` for bounded remote commands and the stdin trigger. Final result: remote boundary, retained qsc recheck, and integrated trigger passed.

Recovered harness issue: listener readiness initially would have used a local probe connection that could consume the one-shot listener. Corrective action: changed the proof-root harness to use a ready flag after bind before the primary integrated attempt. Final result: local rehearsal and integrated marker/ACK pass.

Recovered cleanup-proof issue: immediate bind probe after cleanup failed due recent TCP state, and one broad process scan assertion self-matched proof text. Corrective action: used filtered concrete process scan plus listener-state scan. Final result: cleanup proof passed.

## Rejected alternatives

- Rerun the D433 `python3 -c` payload: rejected because it was the failure shape.
- Write a remote temp script or remote heredoc file: rejected because remote writes and remote temp files are forbidden.
- Use the dedicated forwarding key for remote command execution: rejected after the no-write probe and inherited command restriction showed it is forwarding-only.
- Weaken SSH policy, inspect authorized_keys, or inspect sshd_config: rejected as out of scope and unnecessary.
- Use qsl-server or qsl-attachments to bypass the trigger issue: rejected as out of scope.
- Treat marker/ACK traversal as qsc E2EE proof: rejected because it proves only the trigger/transport precondition.

## Next recommendation

After merge and green required checks, close out NA-0531 to the selected NA-0532 retry successor only if post-merge public-safety is green inside the short attach/early-failure window. Otherwise hand off with NA-0531 still READY and D-1052 recorded.
