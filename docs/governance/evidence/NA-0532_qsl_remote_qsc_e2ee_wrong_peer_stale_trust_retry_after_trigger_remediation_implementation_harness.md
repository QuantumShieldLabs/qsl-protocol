Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-06-24

# NA-0532 QSL Remote qsc E2EE Wrong-Peer / Stale-Trust Retry After Trigger Remediation Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0532 consumed D435/D434/D433/D431/D425/D419 inheritance, rechecked the retained remote `qsc` binary against the NA-0526/D425 hash, built local `qsc` from the clean current checkout, inspected the existing qsc command surfaces, wrote an exact command manifest, and reran the mandatory integrated listener / dedicated-key reverse-forward / stdin-script remote trigger precheck before any qsc E2EE.

The retained remote qsc binary matched the expected path, owner/group, mode, size, and SHA-256 from NA-0526/D425. The local current qsc build had the same size but a different hash; qsc runtime/dependency diff from the D425 source commit to current HEAD was empty, so the retained remote provenance was accepted.

The integrated forwarding precheck failed before remote trigger execution and before any qsc E2EE. The proof-root SSH reverse-forward process exited with `remote port forwarding failed for listen port 39176`. No marker traversed, no ACK was sent, no remote E2EE root was created, and no qsc send/receive or qsc protocol command ran.

Result classification: `REMOTE_E2EE_INTEGRATED_FORWARDING_PRECHECK_FAILURE`.

## Live NA-0532 scope

Allowed checked-in mutation paths for this implementation PR are this evidence file, `tests/NA-0532_qsl_remote_qsc_e2ee_wrong_peer_stale_trust_retry_after_trigger_remediation_implementation_testplan.md`, `DECISIONS.md`, `TRACEABILITY.md`, and `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Proof root: `/srv/qbuild/tmp/NA0532_wrong_peer_stale_trust_retry_after_trigger_remediation_impl_20260624T063939Z`.

No qsc source/test/fuzz/Cargo path, workflow/script/helper path, dependency/lockfile path, corpus/vector/input path, formal/refimpl/service/public/backup path, qsl-server path, qsl-attachments path, public-doc path, backup path, or service path is changed.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

The required qwork proof files existed and were copied into the proof root:

- `/srv/qbuild/work/NA-0532/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0532/.qwork/startup.qsl-protocol.json`

The `.kv` and `.json` proofs matched the required startup state: `startup_result=OK`, lane `NA-0532`, repo `qsl-protocol`, path `/srv/qbuild/work/NA-0532/qsl-protocol`, clean worktree/index/untracked state, `head_equals_origin_main=yes`, `ready_count=1`, queue top READY `NA-0532`, and requested lane status READY.

Proof HEAD and proof `origin/main` both matched the live pre-fetch refs at `21e63577e4ee`. Fetch occurred only after that match and after disk proof showed `/` usage below the 95% stop threshold.

## D435 / D434 / D433 / D431 / D425 / D419 inheritance

NA-0531 was DONE and NA-0532 was READY at startup.

D435 recorded classification `REMOTE_TRIGGER_QUOTING_REMEDIATION_MARKER_TRAVERSAL_PASS`. Its safe remote trigger shape was `ssh -T inspiron 'python3 -' < <proof-root trigger script>`. D435 proved integrated marker traversal and ACK inside one listener / forward / trigger lifetime, and recorded no qsc E2EE and no qsc send/receive.

D434 restored NA-0531 after D433 classification `REMOTE_E2EE_INTEGRATED_TRIGGER_QUOTING_FAILURE`.

D433 recorded D432 residue cleanup passed, retained qsc recheck passed, and retained qsc path `/home/qslcodex/qsl-remote-test/bin/qsc`, owner/group `qslcodex/qslcodex`, mode `700`, size `102103920`, and SHA-256 `6bf9e59fdae397c2e0f88538d700cccbee80d229c6a979cc79555e39fea2b4f7`.

D425 recorded the retained remote qsc hash `6bf9e59fdae397c2e0f88538d700cccbee80d229c6a979cc79555e39fea2b4f7`, size `102103920`, owner `qslcodex`, mode `700`, and remote help success after restaging.

D419 recorded replay and corrupt delivery negatives passed, no-mutation checks passed, cleanup passed, and no qsl-server/qsl-attachments use.

## Retained-qsc freshness recheck

Local current qsc was built with:

`cargo build -p qsc --locked --bin qsc`

The build used proof-root target dir only.

- source commit: `21e63577e4ee`
- local qsc path: proof-root `cargo_target/debug/qsc`
- local size: `102103920`
- local SHA-256: `184cbba58c29ff6afaf1f4f99e2f8d50fc913f662aadc66cb7f5021e6c4ba72f`
- local `--help`: success

Remote retained qsc recheck used local-only output capture. The remote command string contained no remote redirection, no remote temp path, and no remote file write.

- path: `/home/qslcodex/qsl-remote-test/bin/qsc`
- owner/group: `qslcodex/qslcodex`
- mode: `700`
- size: `102103920`
- SHA-256: `6bf9e59fdae397c2e0f88538d700cccbee80d229c6a979cc79555e39fea2b4f7`
- remote `--help`: success

The local hash differed from the retained remote hash, but qsc runtime/dependency diff from D425 source commit `2cff954de589` to current HEAD was empty. The retained remote hash still matched NA-0526/D425, so provenance was accepted and no restaging was attempted.

## Command surface inspection and manifest

Read-only inspection covered the required qsc tests and source modules, including:

- `qsl/qsl-client/qsc/tests/same_host_client_to_client_e2e.rs`
- `qsl/qsl-client/qsc/tests/receive_e2e.rs`
- `qsl/qsl-client/qsc/tests/common/mod.rs`
- `qsl/qsl-client/qsc/tests/mock_relay_transport_na0173.rs`
- `qsl/qsl-client/qsc/tests/identity_binding.rs`
- `qsl/qsl-client/qsc/tests/identity_ux.rs`
- `qsl/qsl-client/qsc/tests/trust_onboarding_mainstream_flow_na0187.rs`
- `qsl/qsl-client/qsc/tests/kem_signature_transcript_binding_negative.rs`
- `qsl/qsl-client/qsc/tests/binding_negative_vector_consumer.rs`
- qsc command, handshake, transport, identity, contacts, relay, protocol-state, store, and vault modules under `qsl/qsl-client/qsc/src/`

The manifest was written before integrated forwarding or E2EE execution:

- `/srv/qbuild/tmp/NA0532_wrong_peer_stale_trust_retry_after_trigger_remediation_impl_20260624T063939Z/command_manifest/na0532_command_manifest.md`
- `/srv/qbuild/tmp/NA0532_wrong_peer_stale_trust_retry_after_trigger_remediation_impl_20260624T063939Z/command_manifest/na0532_command_manifest.json`

The manifest used existing qsc surfaces only: vault init/unlock with passphrase files, identity rotate/show, contacts add/device list/device trust, relay inbox-set, local `qsc relay serve`, handshake init/poll/status, send, receive, wrong-peer receive reject, and replaced-peer identity mismatch reject.

Passphrase values were file-only and were not recorded in checked-in evidence. Synthetic route-token values were kept in proof-root manifest/log artifacts and are not copied into this checked-in evidence.

## Local / remote boundary rechecks

Safe `ssh -G inspiron` parsing recorded only safe fields. The operational alias has `clearallforwardings yes`, so NA-0532 used a proof-root SSH config with `ClearAllForwardings no` for reverse forwarding.

Remote boundary check passed through operational SSH:

- remote user `qslcodex`
- UID `1003`, not root
- no privileged group overlap
- `sudo -n true` failed as expected
- `/backup/qsl` absent or not readable
- `qwork` absent from PATH
- `qsl-backup` absent from PATH and absent at `/usr/local/sbin/qsl-backup`
- directive-specific remote E2EE root absent before precheck

No authorized_keys, sshd_config, private key material, known_hosts dump, or full SSH config was read or mutated.

## Integrated forwarding marker/ACK precheck

The precheck used the D435-proven stdin-script trigger style:

`ssh -T inspiron 'python3 -' < <proof-root trigger script>`

Local trigger script compilation and local marker rehearsal passed. The marker contained no newline or shell-sensitive splitting character.

The local listener bound to `127.0.0.1:39176` and printed `NA0532_LISTENER_READY`.

The proof-root forwarding config used:

- `IdentityFile` by basename only in evidence
- `IdentitiesOnly yes`
- `BatchMode yes`
- `PasswordAuthentication no`
- `StrictHostKeyChecking true`
- `ForwardAgent no`
- `ForwardX11 no`
- `RequestTTY no`
- `ClearAllForwardings no`

The reverse-forward command was:

`ssh -F <proof-root ssh_config> -N -T -o ExitOnForwardFailure=yes -R 127.0.0.1:39176:127.0.0.1:39176 na0532-inspiron-forward`

The SSH process exited before the remote trigger ran. Captured stderr:

`Error: remote port forwarding failed for listen port 39176`

Precheck result:

- marker match: false
- ACK sent: false
- ACK received: false
- remote trigger started: false
- listener ready: true
- qsc E2EE allowed: false

## Local qsc provenance

Local qsc was built from clean current checkout commit `21e63577e4ee` using the proof-root cargo target directory.

The local binary size was `102103920`. The local hash was recorded in proof-root provenance and differed from the retained remote D425 hash. The qsc runtime/dependency diff from D425 source commit `2cff954de589` to current HEAD was empty, so the retained remote qsc remained acceptable without restaging.

## Forwarding / relay setup

The qsc relay was not started. The integrated marker/ACK precheck failed before qsc relay or qsc E2EE was allowed.

No public bind occurred. The only listener started was the local precheck listener on `127.0.0.1:39176`, and cleanup verified port `39176` was free afterward.

## Isolated runtime roots and synthetic data

No qsc E2EE runtime roots were created because the integrated forwarding precheck failed.

- local sensitive runtime root: removed during cleanup
- remote E2EE root: never created and verified absent
- synthetic passphrase files: not created for qsc E2EE
- synthetic message files: not created

The manifest defined baseline, wrong-peer, and stale-trust/replaced-peer synthetic labels for the blocked E2EE phase, but those labels were not sent through qsc.

## Baseline E2EE setup to identity/trust negative test point

Deferred by the failed integrated forwarding precheck.

No vault initialization, identity rotation, contact/trust setup, handshake, qsc send, qsc receive, or baseline valid message ran.

## Wrong-peer negative boundary proof or deferral

Deferred by the failed integrated forwarding precheck.

Planned design from existing qsc surfaces: after a valid Build-to-Inspiron message was queued, remote receive would be attempted with the wrong peer label `charlie`, expecting fail-closed before relay pull because `protocol_active_or_reason_for_peer` checks peer state before pulling. Selected state and receive output directories would be compared before and after, then the valid path would receive the same message with the correct peer label.

Deferral marker: `REMOTE_E2EE_WRONG_PEER_NEGATIVE_DEFERRED_PRECHECK_FAILURE`.

## Stale-trust negative boundary proof or deferral

Deferred by the failed integrated forwarding precheck.

Planned design from existing qsc surfaces: establish a valid `build` to `inspiron` trust relationship, create a second local `build` identity with different public material, send a handshake init using the replaced identity, and require the remote side to emit `identity_mismatch` / `peer_mismatch` while selected remote qsc state remains unchanged.

Deferral marker: `REMOTE_E2EE_STALE_TRUST_NEGATIVE_DEFERRED_PRECHECK_FAILURE`.

## No-mutation checks

No qsc identity/trust negative executed, so selected qsc state no-mutation proof is not applicable.

Cleanup did prove no directive-specific remote E2EE root was created, local sensitive runtime was removed, no precheck listener/forward process remained, and local port `39176` was free.

## Valid-path usability proof or deferral

Deferred by the failed integrated forwarding precheck.

No baseline valid qsc path was established, so no post-negative valid-path check could run.

## No-secret-output review

Checked-in evidence does not include private key material, passphrase values, route-token values, password values, API tokens, raw private qsc material, authorized_keys, sshd_config, known_hosts dumps, or qsc private store contents.

No qsc E2EE output exists because qsc E2EE did not run. Retained-qsc help and metadata output were captured under the proof root and summarized here without private material.

## Cleanup / retention proof

Cleanup after the precheck failure passed:

- local listener process stopped
- reverse-forward process absent after failure
- local sensitive runtime root removed
- local port `39176` free
- no proof-root listener/forward/qsc relay child process remained
- remote E2EE root absent
- retained remote qsc unchanged

No long-lived remote runtime artifact was intentionally retained.

## Result classification

`REMOTE_E2EE_INTEGRATED_FORWARDING_PRECHECK_FAILURE`

The failure occurred before qsc E2EE, qsc send/receive, baseline setup, wrong-peer negative, and stale-trust/replaced-peer negative.

## Hostile Cryptographer Review

The retained-qsc hash and command-surface review prove only that the intended binary and command shapes were selected. They do not prove qsc protocol correctness.

The integrated forwarding marker/ACK precheck failed, so NA-0532 produced no E2EE evidence. Even if it had passed, marker traversal would prove only the transport precondition, not authentication, key agreement, replay resistance, downgrade resistance, side-channel safety, or secret lifecycle.

Wrong-peer and stale-trust testing did not execute. The planned tests would have proven only bounded fail-closed behavior for specific identity/trust surfaces, not identity-complete or trust-complete status.

No production or personal data was used. Synthetic identities, labels, passphrase files, and route tokens were scoped to proof-root or would have been scoped to the directive remote E2EE root after the precheck gate.

## Red-Team Review

If integrated precheck fails again, the next lane should diagnose why remote `127.0.0.1:39176` reverse-forward binding regressed after D435 proved marker traversal. It should not bypass the marker/ACK gate or proceed to qsc E2EE.

If wrong-peer cannot be staged safely after forwarding is restored, the successor should record exact command-surface limits rather than fabricating proof.

If stale-trust/replaced-peer requires internals, the successor should split a command-surface diagnostic lane instead of mutating qsc state files directly.

If stale-trust mutates valid state, the lane must stop and select remediation.

Route/capability metadata must stay out of checked-in evidence and final responses. Cleanup must continue to prove absence of local listener/forward processes and directive remote roots.

## Production SRE Review

This run is operationally useful because it caught a regression at the transport precondition before any qsc E2EE was attempted. It is bounded and synthetic, not service readiness.

Logs retained under the proof root include command outputs, check summaries, and cleanup proof. Checked-in evidence redacts synthetic route-token values and passphrase values.

Failures remained isolated from qwork, qsl-backup, backup storage, production data, qsl-server, and qsl-attachments.

qsl-server and qsl-attachments remain deferred and unused. This lane does not imply public, production, or public-internet readiness.

## Release-Claim Boundary Review

NA-0532 makes no public-ready claim, no production-ready claim, no public-internet-ready claim, no external-review-complete claim, no crypto-complete claim, no identity-complete claim, no trust-complete claim, no replay-proof claim, no downgrade-proof claim, no secret-material-complete claim, no side-channel-free claim, and no vulnerability-free, bug-free, or perfect-crypto claim.

## Successor selection

Selected successor for closeout, if this implementation PR merges and post-merge public-safety/advisories are green:

`NA-0533 -- QSL Remote qsc E2EE Reverse-Forward Port 39176 Regression Diagnostic / Retry Scope Authorization Plan`

The successor should diagnose the `remote port forwarding failed for listen port 39176` regression using bounded read-only remote checks and proof-root forwarding config, then authorize a retry only after marker traversal and ACK are re-proven. It must not select public/production readiness.

## Future scope bundle

Future exact scope should include proof-root SSH config review, remote loopback bind availability checks, bounded dedicated-key reverse-forward attempts, stdin-script trigger proof, cleanup proof, and no qsc E2EE until marker/ACK traversal passes.

Future remote mutation should remain absent until an explicit post-precheck E2EE retry is authorized.

## Future validation / marker plan

Future marker plan:

- re-use the D435 stdin-script remote trigger shape
- preserve one controlled listener / forward / trigger lifetime
- require listener `marker_match=true`, `ack_sent=true`, and remote ACK receipt before qsc E2EE
- record no qsc E2EE if the forwarding gate fails

## No qsl-server / no qsl-attachments boundary

No qsl-server command, source path, service, port, or deployment was used.

No qsl-attachments command, source path, service, port, or deployment was used.

## No public/production readiness boundary

This evidence is internal governance and operational hardening evidence only. It is not public-readiness, production-readiness, public-internet-readiness, external-review, crypto-completion, identity-completion, trust-completion, replay-proof, downgrade-proof, side-channel-free, vulnerability-free, bug-free, or perfect-crypto evidence.

## Backup-impact statement

qsl-backup was checked read-only by digest and source inclusion count. It was not executed. `/backup/qsl` was not mutated. Backup status and plan files were not mutated.

## Rejected alternatives

- Proceed to qsc E2EE after the failed precheck: rejected because marker traversal and ACK are mandatory before qsc E2EE.
- Retry forwarding repeatedly in this lane: rejected because Packet E requires stop before qsc E2EE on precheck failure.
- Use qsl-server or qsl-attachments to bypass the forwarding failure: rejected as out of scope.
- Modify SSH server configuration, authorized_keys, known_hosts, or installed keys: rejected as out of scope.
- Restage the retained remote qsc: rejected because the retained hash still matched NA-0526/D425 and qsc runtime/dependency diff was empty.

## Next recommendation

Merge this implementation evidence PR after required checks pass. If post-merge public-safety is green inside the short attach/early-failure window, close out NA-0532 and restore the selected NA-0533 forwarding regression diagnostic / retry authorization plan.
