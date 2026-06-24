Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-06-24

# NA-0530 QSL Remote qsc E2EE Wrong-Peer / Stale-Trust Retry with Integrated Forwarding Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0530 recovered the D432 retained-qsc smoke command-shape stop, verified the two exact D432 remote `/tmp` residue paths were absent, rechecked the retained Inspiron `qsc` binary against the NA-0526/D425 hash, wrote a command manifest from current qsc CLI/test surfaces, and ran the mandatory integrated listener / dedicated-key reverse-forward / remote trigger precheck before any qsc E2EE.

The integrated precheck stopped before qsc E2EE. The local listener bound to `127.0.0.1:39176`, and the dedicated-key reverse-forward process stayed alive with `ExitOnForwardFailure=yes`, but the remote trigger command was quoted incorrectly: the remote shell split the `python3 -c` payload before it could connect to the forwarded loopback port. No marker reached the listener, no ACK was sent, and no qsc E2EE, send, receive, baseline, wrong-peer, or stale-trust negative ran.

Result classification: `REMOTE_E2EE_INTEGRATED_TRIGGER_QUOTING_FAILURE`.

## Live NA-0530 scope

Allowed checked-in mutation paths for this implementation PR are this evidence file, `tests/NA-0530_qsl_remote_qsc_e2ee_wrong_peer_stale_trust_retry_with_integrated_forwarding_testplan.md`, `DECISIONS.md`, `TRACEABILITY.md`, and `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

The proof root is `/srv/qbuild/tmp/NA0530_recover_retry_wrong_peer_stale_trust_integrated_forwarding_impl_20260624T040457Z`.

No qsc source/test/fuzz/Cargo path, workflow/script/helper path, dependency/lockfile path, corpus/vector/input path, formal/refimpl/service/public/backup path, qsl-server path, or qsl-attachments path is changed.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`. It read and copied:

- `/srv/qbuild/work/NA-0530/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0530/.qwork/startup.qsl-protocol.json`

The proof files recorded `startup_result=OK`, lane `NA-0530`, repo `qsl-protocol`, path `/srv/qbuild/work/NA-0530/qsl-protocol`, clean worktree/index/untracked state, `head_equals_origin_main=yes`, `ready_count=1`, queue top READY `NA-0530`, and requested lane status READY. Proof HEAD and proof `origin/main` matched the live refs before fetch at `023b26452786`.

Disk proof before fetch stayed below the 95% stop threshold.

## D432 / D431 / D430 / D427 / D425 / D419 inheritance

D432 stopped before integrated forwarding precheck, qsc E2EE, governance patch, PR, merge, and closeout with operational stop label `REMOTE_SCOPE_BOUNDARY_VIOLATION_STOP`. The stop was caused by a retained-qsc smoke command that briefly redirected remote `qsc --help` output through two proof-ID-named remote `/tmp` files. D432 made no repo mutation, opened no branch or PR, added no D-1050, and left NA-0530 READY.

D431 closed out NA-0529, accepted the operator-supplied manual integrated forwarding proof, and restored NA-0530 READY. The manual proof showed `REMOTE_ACK=MANUAL_TUNNEL_ACK_OK`, `INTEGRATED_MARKER_TRAVERSED_OK`, `"marker_match": true`, `"ack_sent": true`, and `"ok": true`, but NA-0530 still had to re-prove marker traversal and ACK in-lane before qsc E2EE.

D430 recorded `REMOTE_FORWARDING_DIAGNOSTIC_REMOTE_TRIGGER_FAILURE` for NA-0529 and did not run qsc E2EE or qsc send/receive. D427 recorded `REMOTE_E2EE_FORWARDING_RECHECK_FAILURE` before E2EE. D425 recorded the retained remote qsc hash `6bf9e59fdae397c2e0f88538d700cccbee80d229c6a979cc79555e39fea2b4f7`. D419 recorded replay/corrupt negatives passed in its bounded synthetic lane. No inherited lane used qsl-server or qsl-attachments for this retry.

## D432 /tmp residue recovery

Before any other remote work, Codex ran one bounded remote cleanup command that checked only:

- `/tmp/NA0530_20260624T022844Z_D432_qsc_help_disallowed.tmp`
- `/tmp/NA0530_20260624T022844Z_D432_qsc_help_disallowed.err`

Both were absent, so no removal was needed. Required markers were present:

- `NA0530_D432_TMP_RESIDUE_CHECK_STARTED_OK`
- `NA0530_D432_TMP_RESIDUE_TMP_ABSENT_OR_REMOVED_OK`
- `NA0530_D432_TMP_RESIDUE_ERR_ABSENT_OR_REMOVED_OK`
- `NA0530_D432_TMP_RESIDUE_CLEANUP_OK`

No file contents were read, no other `/tmp` path was touched, and no qsc command ran in this cleanup step.

## Retained-qsc freshness recheck with local-only output capture

Local qsc was built from clean source commit `023b26452786` with proof-root `CARGO_TARGET_DIR`. Local qsc path was under the proof root, size was `102103920`, and local SHA-256 was `f0e05f3439fa6ae75a509bb5ab904617fb8a81b0be0a912d86e6689fad4088d0`. Local `qsc --help` exited successfully.

The retained remote qsc recheck passed:

- Path: `/home/qslcodex/qsl-remote-test/bin/qsc`
- Owner/group: `qslcodex/qslcodex`
- Mode: `700`
- Size: `102103920`
- SHA-256: `6bf9e59fdae397c2e0f88538d700cccbee80d229c6a979cc79555e39fea2b4f7`

Remote `qsc --help` was rerun with local-only capture. The remote command string was exactly `/home/qslcodex/qsl-remote-test/bin/qsc --help`; a proof check found no `>`, `2>`, `>>`, `tee`, `mktemp`, `/tmp`, or `cat >` in the remote command string. Local shell redirection wrote stdout/stderr under the proof root.

The local hash differed from the retained remote hash, but the retained remote hash matched the NA-0526/D425 retained hash and qsc runtime/dependency diff since D425 was empty. Provenance was accepted as `remote_hash_matches_d425_retained_hash_local_differs_no_qsc_runtime_dependency_drift`.

## Command surface inspection and manifest

Codex inspected current qsc CLI/test surfaces before any send/receive or E2EE command:

- `qsl/qsl-client/qsc/tests/same_host_client_to_client_e2e.rs`
- `qsl/qsl-client/qsc/tests/receive_e2e.rs`
- `qsl/qsl-client/qsc/tests/common/mod.rs`
- `qsl/qsl-client/qsc/tests/mock_relay_transport_na0173.rs`
- `qsl/qsl-client/qsc/tests/identity_binding.rs`
- `qsl/qsl-client/qsc/tests/identity_ux.rs`
- `qsl/qsl-client/qsc/tests/trust_onboarding_mainstream_flow_na0187.rs`
- `qsl/qsl-client/qsc/tests/kem_signature_transcript_binding_negative.rs`
- `qsl/qsl-client/qsc/tests/binding_negative_vector_consumer.rs`
- command, handshake, transport, relay, identity, contacts, protocol-state, store, and vault modules under `qsl/qsl-client/qsc/src/`

The command manifest was written before integrated forwarding precheck:

- `/srv/qbuild/tmp/NA0530_recover_retry_wrong_peer_stale_trust_integrated_forwarding_impl_20260624T040457Z/command_manifest/na0530_command_manifest.md`
- `/srv/qbuild/tmp/NA0530_recover_retry_wrong_peer_stale_trust_integrated_forwarding_impl_20260624T040457Z/command_manifest/na0530_command_manifest.json`

The manifest used existing qsc surfaces only: vault init/unlock with passphrase files, identity rotate/show, contacts add/device list/device trust, relay inbox-set/serve, handshake init/poll/status, send, receive, and identity-binding negative patterns. Dynamic identity fingerprints were represented as proof-file inputs because they are generated at runtime. Passphrase values were path-only and were not recorded.

## Local / remote boundary rechecks

The advisory `ssh -G inspiron` parse recorded safe fields only and found the known default-host hazards: `clearallforwardings yes` and no dedicated forwarding identity basename. Codex did not rely on that default host for forwarding. It wrote a proof-root SSH config using the existing dedicated forwarding key basename `qslcodex_forward_ed25519`, `IdentitiesOnly yes`, `PasswordAuthentication no`, `BatchMode yes`, `ForwardAgent no`, `ForwardX11 no`, `ClearAllForwardings no`, and `RequestTTY no`.

Remote boundary recheck passed:

- user `qslcodex`
- UID nonzero
- no privileged groups
- `sudo -n true` failed as expected
- `/backup/qsl` absent or not readable
- qwork absent
- qsl-backup absent
- remote E2EE root absent
- D432 residue paths absent

Recovered failure: the first local `ssh -G` proof script incorrectly treated default-host forwarding hazards as a hard failure before writing the proof-root forwarding config. Classification: recoverable proof-script validation mistake. Corrective action: recorded the default hazard and generated the proof-root config with the dedicated forwarding key and `ClearAllForwardings no`. Final result: remote boundary recheck passed.

## Integrated forwarding marker/ACK precheck

The integrated precheck was run in one local runner process. It locally rehearsed the exact marker bytes for:

`QSL_INTEGRATED_FORWARDING_PRECHECK_NA0530_NA0530_20260624T040457Z_D433`

The local listener bound to `127.0.0.1:39176`. The dedicated-key reverse-forward command used `ssh -F <proof-root-config> -N -T -o ExitOnForwardFailure=yes -R 127.0.0.1:39176:127.0.0.1:39176 na0530-inspiron-forward`. The forward process remained alive.

The remote trigger failed before connecting to the forwarded loopback port. Remote stderr showed the `python3 -c` payload was split by the remote shell:

- `SyntaxError: Expected one or more names after 'import'`
- `bash: -c: line 2: syntax error near unexpected token '('`

Listener result:

- listener bound: true
- forward alive: true
- marker match: false
- ACK sent: false
- ACK received: false
- trigger return code: 2
- final ok: false

Because the marker did not traverse and ACK was not received, qsc E2EE did not run.

## Local qsc provenance

Local qsc source commit: `023b26452786`. Local qsc hash: `f0e05f3439fa6ae75a509bb5ab904617fb8a81b0be0a912d86e6689fad4088d0`. Root and nested qsc fuzz lockfiles both retain `quinn-proto 0.11.15`. No Cargo.toml drift was present at the start.

## Forwarding / relay setup

The qsc relay was not started. The integrated precheck listener was separate from qsc relay and failed before qsc E2EE was allowed. The proof-root reverse-forward process was terminated during cleanup, and no listener remained on `127.0.0.1:39176`.

## Isolated runtime roots and synthetic data

Planned local and remote E2EE roots were recorded in the manifest, but qsc E2EE roots were not created because the integrated precheck failed. Synthetic labels were recorded in the manifest only. No production data, personal data, private key material, passphrase value, token value, or secret message text was checked in.

## Baseline E2EE setup to identity/trust negative test point

Not executed. The mandatory integrated marker/ACK precheck failed before qsc E2EE was allowed.

## Wrong-peer negative boundary proof or deferral

Deferred because qsc E2EE was not allowed after the integrated trigger quoting failure.

Deferral marker: `REMOTE_E2EE_WRONG_PEER_NEGATIVE_DEFERRED_PRECHECK_FAILURE`.

## Stale-trust negative boundary proof or deferral

Deferred because qsc E2EE was not allowed after the integrated trigger quoting failure.

Deferral marker: `REMOTE_E2EE_STALE_TRUST_NEGATIVE_DEFERRED_PRECHECK_FAILURE`.

## No-mutation checks

No identity/trust negative executed, so selected identity/trust state no-mutation checks were not applicable. Cleanup proof showed no remote E2EE root was created and no proof-root listener/forwarding process remained.

## Valid-path usability proof or deferral

Deferred because baseline E2EE did not run. There was no valid qsc path established in this lane after the precheck failure.

## No-secret-output review

Checked-in evidence contains no private key contents, passphrase values, passwords, tokens, production endpoints, backup material, authorized_keys dump, known_hosts dump, sshd_config dump, or raw private qsc material. Proof-root qsc private runtime roots were not created. The only local qsc binary proof is hash/stat/help output.

## Cleanup / retention proof

Cleanup proof passed:

- local sensitive runtime root absent
- local port `39176` closed
- no proof-root listener or SSH forward process remained
- remote E2EE root absent
- D432 residue paths absent
- retained remote qsc unchanged

Recovered failure: the first cleanup process scan matched its own `rg`/shell command and reported a false `PROOF_PROCESS_PRESENT`. Classification: recoverable proof-parser false positive. Corrective action: reran the process scan while excluding scan self-matches. Final result: cleanup proof passed.

## Result classification

Selected classification:

`REMOTE_E2EE_INTEGRATED_TRIGGER_QUOTING_FAILURE`

This is a trigger quoting stop before qsc E2EE, not a qsc protocol or identity/trust negative result.

## Hostile Cryptographer Review

The integrated forwarding precheck would prove only a transport precondition: that one synthetic marker crossed the loopback reverse-forwarding path and returned an ACK. In this run, even that precondition failed because the remote trigger command was quoted incorrectly.

No qsc protocol correctness, authentication completeness, replay resistance, downgrade resistance, side-channel safety, or secret-material lifecycle property is proven. The retained-qsc hash proof establishes binary freshness against NA-0526/D425 only, not crypto completeness.

Wrong-peer and stale-trust plans remain bounded identity/trust fail-closed tests for synthetic identities only. They would not prove identity-complete or trust-complete status even if later executed. Synthetic identities and route labels avoid production/personal data exposure.

## Red-Team Review

If D432 residue cleanup had failed, the lane would have stopped before any other remote work. It passed with both files absent.

If local-only retained-qsc smoke capture is replaced with remote redirection again, the lane must stop; this run proved the corrected remote command string contained no remote redirection and no `/tmp`.

If integrated precheck fails again, E2EE must remain blocked. That is what happened here.

If wrong-peer or stale-trust staging requires internals, a later lane must record the command-surface truth instead of fabricating evidence.

If stale-trust mutates valid state, a later lane must stop and select remediation. If route/capability metadata appears in evidence, redact or fail the evidence scan. If cleanup leaves artifacts, record exact paths/PIDs and do not claim cleanup success.

The next hardening item is a trigger command-shape remediation that rehearses and syntax-checks the remote `python3 -c` payload without splitting it across the remote shell.

## Production SRE Review

This run is operationally useful but bounded. It verified qwork handoff, current-main health, retained-qsc freshness, D432 residue cleanup, boundary checks, proof-root forwarding config, listener bind, forward process startup, trigger failure capture, and cleanup.

Logs are proof-root-local. Checked-in evidence summarizes paths, hashes, statuses, and error classes only. Failures stayed isolated from qwork, qsl-backup, production data, qsl-server, and qsl-attachments.

qsl-server and qsl-attachments remain deferred. This is not public, production, or public-internet readiness evidence.

## Release-Claim Boundary Review

No public-readiness claim is made. No production-readiness claim is made. No public-internet-readiness claim is made. No external-review-complete claim is made. No crypto-complete claim is made. No identity-complete claim is made. No trust-complete claim is made. No replay-proof claim is made. No downgrade-proof claim is made. No secret-material-complete claim is made. No side-channel-free claim is made. No vulnerability-free, bug-free, or perfect-crypto claim is made.

## Successor selection

Selected successor candidate:

`NA-0531 -- QSL Remote qsc E2EE Integrated Trigger Quoting Remediation Harness`

Because this directive did not provide a full approved failure-case successor block for closeout, this implementation PR does not edit `NEXT_ACTIONS.md`. A later closeout directive should provide explicit successor block text before queue mutation.

## Future scope bundle

Future work should keep the same D432 residue cleanup proof, retained-qsc local-only smoke capture, proof-root SSH config, local listener, loopback-only reverse forward, and no qsl-server/no qsl-attachments boundaries. It should fix the remote trigger quoting shape before attempting any qsc E2EE.

## Future validation / marker plan

Future markers should include qwork proof verified, one READY NA-0530 or successor READY, retained qsc D425 hash, remote smoke local-only capture, proof-root SSH config with dedicated key and `ClearAllForwardings no`, remote trigger command syntax proof, marker match, ACK sent, ACK received, cleanup, and no qsl-server/no qsl-attachments.

## No qsl-server / no qsl-attachments boundary

No qsl-server command, endpoint, service, file path, workflow, source, or runtime artifact was used or mutated. No qsl-attachments command, endpoint, service, file path, workflow, source, or runtime artifact was used or mutated.

## No public/production readiness boundary

This evidence is a pre-E2EE integrated trigger failure record. It does not claim public readiness, production readiness, public-internet readiness, crypto completion, identity completion, trust completion, replay proof, downgrade proof, side-channel freedom, vulnerability freedom, bug freedom, or perfect crypto.

## Backup-impact statement

qsl-backup was inspected read-only for checksum/source-list boundary proof and was not executed or mutated. `/backup/qsl` was checked read-only for remote account isolation and was absent or unreadable from the remote test account.

## Rejected alternatives

- Treat D432 as a successful E2EE attempt: rejected because D432 stopped before integrated forwarding and before qsc E2EE.
- Rerun the failed integrated trigger immediately: rejected because the mandatory precheck failed and the lane must stop before qsc E2EE.
- Use qsl-server or qsl-attachments as a transport bypass: rejected as out of scope.
- Weaken SSH or inspect authorized_keys/sshd_config directly: rejected as out of scope and unnecessary for the observed quoting failure.

## Next recommendation

Issue a narrow remediation directive that fixes remote trigger quoting while preserving one controlled listener/forward/trigger lifetime, dedicated-key reverse forwarding, loopback-only binding, local-only proof capture, no remote `/tmp` writes, and the hard qsc E2EE gate on marker traversal plus ACK receipt.
