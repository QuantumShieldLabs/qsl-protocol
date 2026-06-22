Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-22

# NA-0520 qsl remote qsc E2EE SSH forwarding capability probe implementation harness

## Executive summary

NA-0520 executed the bounded SSH loopback reverse-forwarding capability probe authorized by NA-0519 / D413. The probe used the dedicated qslcodex forwarding key through a proof-root-local SSH config, started one Build-local listener on `127.0.0.1:39176`, started one reverse-forward session with `ExitOnForwardFailure=yes`, ran one remote loopback trigger command through the existing operational `inspiron` SSH path, proved one synthetic marker traversed the tunnel and returned an ACK, and cleaned up the local listener and SSH process.

Result classification: `SSH_FORWARDING_CAPABILITY_PROBE_PASS`.

Selected successor: `NA-0521 -- QSL Build-to-Inspiron Remote qsc E2EE Retry Implementation Harness`.

This is SSH transport capability evidence only. It is not qsc E2EE proof, not qsc send/receive proof, not qsl-server proof, not qsl-attachments proof, and not a public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim.

## Live NA-0520 scope

NA-0520 was the sole READY item at startup. Its live scope authorized a forwarding-capability probe only:

- one proof-root-local listener bound to Build loopback `127.0.0.1:39176`;
- one dedicated-key SSH reverse-forward session;
- one bounded remote trigger command that connected to Inspiron loopback `127.0.0.1:39176` and sent a synthetic marker;
- local cleanup of the listener and SSH process;
- governance evidence, testplan, D-1029, TRACEABILITY, and rolling journal updates.

NA-0520 did not authorize qsc E2EE, qsc send/receive, qsl-server, qsl-attachments, package installation, sudo/admin action, SSH key generation/installation, authorized_keys mutation, known_hosts mutation, remote file writes, remote host mutation, qwork/qstart/qresume, qsl-backup, qsc source/test/fuzz/Cargo mutation, workflow/script/helper mutation, dependency/lockfile mutation, corpus/vector/input mutation, or formal/refimpl/service/public/backup mutation.

Markers recorded in this evidence:

- `NA0520_FORWARDING_PROOF_REVIEW_CONSUMED_OK`
- `NA0520_DEDICATED_FORWARDING_KEY_USED_OK`
- `NA0520_LOOPBACK_LISTENER_STARTED_OK`
- `NA0520_REVERSE_FORWARD_STARTED_OK`
- `NA0520_EXIT_ON_FORWARD_FAILURE_OK`
- `NA0520_REMOTE_TRIGGER_STARTED_OK`
- `NA0520_TUNNEL_MARKER_SENT_OK`
- `NA0520_TUNNEL_ACK_RECEIVED_OK`
- `NA0520_TUNNEL_MARKER_TRAVERSED_OK`
- `NA0520_NO_PTY_REQUIRED_OK`
- `NA0520_NO_AGENT_X11_FORWARDING_OK`
- `NA0520_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0520_NO_REMOTE_E2EE_OK`
- `NA0520_NO_REMOTE_FILE_WRITE_OK`
- `NA0520_CLEANUP_COMPLETED_OK`
- `NA0520_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0520_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0520_ONE_READY_INVARIANT_OK`

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

The operator-created qwork proof files existed and were copied into the proof root:

- `/srv/qbuild/work/NA-0520/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0520/.qwork/startup.qsl-protocol.json`

Required proof fields passed:

- `startup_result=OK`
- `lane=NA-0520`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0520/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0520`
- `requested_lane_status=READY`

Freshness proof passed before fetch:

- proof HEAD matched live HEAD at `32273a1d7ca8`;
- proof `origin/main` matched live `origin/main` at `32273a1d7ca8`;
- disk usage before fetch was below the 95% stop threshold.

After fetch, `origin/main` still equaled and descended from `32273a1d7ca8`.

`NA0520_FORWARDING_PROOF_REVIEW_CONSUMED_OK`

## NA-0519 / D413 inheritance

NA-0519 completed and NA-0520 was restored READY by D413 / D-1028. The D413 response was discovered exactly once at `/home/victor/work/qsl/codex/responses/NA0519_20260622T021059Z_D413.md` and copied into the proof root.

Inherited facts consumed:

- classification `SSH_FORWARDING_OPERATOR_PROOF_ACCEPTED_WITH_COMPATIBILITY_CAVEAT`;
- dedicated forwarding key proof accepted;
- forwarding key fingerprint `SHA256:7guT/Hd72vuu+u1Jrwpuy0hSFETVrxYaXHwupuFfaPM`;
- public key comment `qsl-inspiron-qslcodex-forward-20260622`;
- loopback-only `permitlisten` and `permitopen` on `127.0.0.1:39176` accepted;
- PTY absent;
- agent forwarding absent;
- X11 forwarding absent;
- qslcodex non-sudo;
- no backup, qwork, or qsl-backup exposure accepted from operator proof;
- cleanup/revocation command accepted;
- forwarding itself was untested before NA-0520;
- `command="/bin/false"` compatibility caveat remained open before this probe;
- qsl-server and qsl-attachments remained out of scope;
- NA-0520 was selected as the forwarding capability probe.

The NA-0520 probe closed only the forwarding-capability caveat. It did not prove qsc protocol behavior.

## Forwarding probe command manifest

Proof root: `/srv/qbuild/tmp/NA0520_ssh_forwarding_capability_probe_impl_20260622T030104Z`

Proof ID: `NA0520_20260622T030407Z_8553`

Synthetic marker: `QSL_FORWARDING_PROBE_SYNTHETIC_NA0520_NA0520_20260622T030407Z_8553`

Manifest artifacts:

- `$PROOF_DIR/forward_probe/na0520_forwarding_manifest.md`
- `$PROOF_DIR/forward_probe/na0520_forwarding_manifest.json`

Local listener script:

```text
/srv/qbuild/tmp/NA0520_ssh_forwarding_capability_probe_impl_20260622T030104Z/forward_probe/listener_na0520.py
```

Proof-root SSH config:

```text
/srv/qbuild/tmp/NA0520_ssh_forwarding_capability_probe_impl_20260622T030104Z/forward_probe/ssh_forward_config
```

Exact reverse-forward command:

```bash
ssh -F "/srv/qbuild/tmp/NA0520_ssh_forwarding_capability_probe_impl_20260622T030104Z/forward_probe/ssh_forward_config" -N -T -o ExitOnForwardFailure=yes -R 127.0.0.1:39176:127.0.0.1:39176 qsl-forward-inspiron
```

Exact remote trigger command:

```bash
ssh -T -o BatchMode=yes -o PasswordAuthentication=no -o ConnectTimeout=10 inspiron 'bash -s' < "/srv/qbuild/tmp/NA0520_ssh_forwarding_capability_probe_impl_20260622T030104Z/forward_probe/remote_trigger.sh"
```

Manifest boundaries:

- no qsc E2EE;
- no qsc send/receive;
- no qsl-server or qsl-attachments;
- no remote file write;
- no PTY;
- no agent forwarding;
- no X11 forwarding.

## Local listener proof

Before starting the listener, the local port precheck proved `LOCAL_PORT_39176_FREE=yes`.

The listener:

- bound only to `127.0.0.1:39176`;
- accepted exactly one connection;
- read exactly one line marker;
- required the marker to equal the expected synthetic marker;
- returned `NA0520_TUNNEL_ACK_OK`;
- wrote its result JSON under the proof root;
- exited after one connection.

Listener proof markers:

- `NA0520_LOOPBACK_LISTENER_STARTED_OK`
- `NA0520_TUNNEL_MARKER_TRAVERSED_OK`

Listener result summary:

- `bind_host=127.0.0.1`
- `bind_port=39176`
- `marker_match=True`
- `ack_sent=True`
- `received_marker_length=66`

## Dedicated-key SSH reverse-forward proof

The proof-root-local SSH config used host alias `qsl-forward-inspiron`, user `qslcodex`, `IdentityFile /home/victor/.ssh/qslcodex_forward_ed25519`, `IdentitiesOnly yes`, public-key-only authentication, `BatchMode yes`, `StrictHostKeyChecking yes`, `ForwardAgent no`, `ForwardX11 no`, `RequestTTY no`, and `ClearAllForwardings no`.

Codex verified the dedicated forwarding key path existed and was readable without reading private key contents.

The reverse-forward process stayed alive after the startup wait, which proves `ExitOnForwardFailure=yes` did not abort the selected loopback reverse-forward path.

Proof markers:

- `NA0520_DEDICATED_FORWARDING_KEY_USED_OK`
- `NA0520_REVERSE_FORWARD_STARTED_OK`
- `NA0520_EXIT_ON_FORWARD_FAILURE_OK`
- `NA0520_NO_PTY_REQUIRED_OK`
- `NA0520_NO_AGENT_X11_FORWARDING_OK`

The prior forced no-shell command compatibility caveat did not block this `-N -T -R` forwarding session.

## Remote trigger proof

Codex ran exactly one bounded remote trigger command through the existing operational `inspiron` SSH path, not through the dedicated forwarding key.

The remote trigger script:

- used bash `/dev/tcp/127.0.0.1/39176`;
- wrote no remote files;
- required no PTY;
- required no sudo/admin action;
- ran no qsc;
- ran no qwork;
- ran no qsl-backup;
- inspected no remote secrets;
- emitted only fixed proof markers.

Remote trigger stdout:

```text
NA0520_REMOTE_TRIGGER_STARTED_OK
NA0520_TUNNEL_MARKER_SENT_OK
NA0520_TUNNEL_ACK_RECEIVED_OK
NA0520_REMOTE_TRIGGER_DONE_OK
```

## Tunnel marker traversal proof

The remote trigger sent the synthetic marker through the reverse-forward tunnel and received `NA0520_TUNNEL_ACK_OK` from the Build-local listener.

Traversal verification:

- listener result `ok=True`;
- listener bound to `127.0.0.1:39176`;
- marker matched;
- ACK was sent;
- remote trigger received the ACK.

Marker proof:

- `NA0520_TUNNEL_MARKER_SENT_OK`
- `NA0520_TUNNEL_ACK_RECEIVED_OK`
- `NA0520_TUNNEL_MARKER_TRAVERSED_OK`

This proves only the selected SSH loopback reverse-forwarding path carried one synthetic marker. It does not prove qsc E2EE, relay semantics, replay resistance, downgrade resistance, side-channel properties, or secret lifecycle completeness.

## Cleanup proof

Cleanup ran at the end of the probe:

- SSH forward PID `1014460` was terminated;
- listener had already exited after one connection;
- no proof-root listener process remained;
- no SSH process using the proof-root forwarding config remained.

Cleanup markers:

- `NA0520_CLEANUP_COMPLETED_OK`

No remote cleanup was required because the trigger wrote no remote files.

## Result classification

Selected classification:

```text
SSH_FORWARDING_CAPABILITY_PROBE_PASS
```

Basis:

- qwork proof files verified without rerunning qwork;
- NA-0519 / D413 inheritance consumed;
- dedicated forwarding key path used through proof-root SSH config;
- local listener bound to Build loopback only;
- reverse-forward session started with `ExitOnForwardFailure=yes`;
- remote trigger connected to Inspiron loopback only;
- synthetic marker traversed the tunnel and returned ACK;
- cleanup completed with no proof-root listener or SSH forward process remaining;
- no qsc E2EE, qsc send/receive, qsl-server, qsl-attachments, remote file write, package install, sudo/admin, key generation, authorized_keys mutation, known_hosts mutation, qwork, qsl-backup, qsc source/test/fuzz/Cargo mutation, workflow/script/helper mutation, dependency/lockfile mutation, corpus/vector/input mutation, or formal/refimpl/service/public/backup mutation occurred.

## Hostile Cryptographer Review

Marker traversal proves only that the selected SSH transport path can carry one synthetic line from the remote loopback endpoint to a Build-local listener and return one ACK. It does not prove qsc protocol correctness, qsc session establishment, message encryption, identity binding, transcript binding, replay rejection, downgrade rejection, mailbox semantics, or secret lifecycle behavior.

qsc E2EE remains unproven until a later lane runs the retained qsc binary through a bounded send/receive and reply flow over this transport path with synthetic messages and separate cleanup/retention proof.

Remaining unproven areas include:

- qsc relay semantics under the forwarded path;
- qsc send/receive and reply behavior;
- replay and downgrade resistance;
- side-channel behavior;
- secret lifecycle and zeroization completeness;
- formal-model correspondence for the remote E2EE flow;
- public claim readiness.

The correct cryptographic interpretation is narrow: the forwarding path is available for a later test. No crypto-complete, replay-proof, downgrade-proof, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim is made.

## Red-Team Review

If forwarding were to bind beyond loopback, the risk would be exposing a later local qsc relay beyond the intended host boundary. NA-0520 reduced that risk by specifying `-R 127.0.0.1:39176:127.0.0.1:39176` and by using a remote trigger that connected only to `127.0.0.1:39176`; future NA-0521 should preserve explicit loopback binding proof.

If the forced command broke forwarding, NA-0520 would have selected a remediation successor. It did not: the reverse-forward session stayed alive and carried the marker. This does not prove shell access; the probe used `-N -T` and required no shell on the dedicated-key session.

If the tunnel stayed alive after cleanup, it would preserve an unintended forwarding path. Cleanup verification found no proof-root listener process and no SSH process using the proof-root forwarding config.

If the synthetic marker were misread or replayed, the listener would reject any non-matching marker and exit nonzero. This is a simple transport marker check only; it is not qsc replay protection.

If the remote trigger required PTY, sudo, or file writes, the lane would fail its scope. The trigger used `ssh -T`, bash `/dev/tcp`, no sudo, and no remote file writes.

Before the E2EE retry, NA-0521 should recheck the retained qsc hash/path/owner, recheck forwarding, isolate local and remote roots, use synthetic messages only, redact proof, and preserve cleanup/retention evidence.

## Production SRE Review

The forwarding probe is operationally useful because it proves the specific blocked transport primitive before spending another lane on remote qsc E2EE. It is bounded because it starts only one listener, one SSH reverse-forward session, and one remote trigger command.

Started processes:

- one Build-local Python listener bound to `127.0.0.1:39176`;
- one dedicated-key SSH reverse-forward process.

Cleaned up processes:

- the listener exited after one accepted connection;
- the SSH process was terminated;
- process checks found no proof-root listener or proof-root SSH config process remaining.

Logged material:

- fixed proof markers;
- command paths;
- process PIDs;
- synthetic marker length and traversal result;
- no private key contents, passphrases, tokens, passwords, authorized_keys dump, known_hosts dump, production endpoint secret, backup private material, qsc secret material, or user data.

Failure isolation:

- qwork proof files were read-only inputs and were not regenerated;
- qsl-backup helper was checked by digest/source-count only and was not run;
- no backup or restore ran;
- no qsl-server or qsl-attachments service was used;
- no qsc runtime was invoked.

This does not imply public-readiness, production-readiness, or public-internet-readiness.

## Release-Claim Boundary Review

NA-0520 preserves these boundaries:

- no public-readiness claim is made;
- no production-readiness claim is made;
- no public-internet-readiness claim is made;
- no external-review-complete claim is made;
- no crypto-complete claim is made;
- no replay-proof claim is made;
- no downgrade-proof claim is made;
- no secret-material-complete claim is made;
- no side-channel-free claim is made;
- no vulnerability-free claim is made;
- no bug-free claim is made;
- no perfect-crypto claim is made.

## Successor selection

Because forwarding capability passed, the selected successor is:

```text
NA-0521 -- QSL Build-to-Inspiron Remote qsc E2EE Retry Implementation Harness
Status: READY
Goals: G1, G2, G3, G4, G5
```

Objective:

Retry the bounded Build-to-Inspiron qsc client-to-client E2EE flow using the retained remote qsc binary and the now-proven loopback reverse-forwarding path, with synthetic messages, isolated local/remote roots, redacted proof, retained-qsc hash recheck, send/receive and reply flow, cleanup/retention proof, and no qsl-server/qsl-attachments or public/production readiness claims.

## Future scope bundle

Future NA-0521 allowed scope should include:

- `docs/governance/evidence/NA-0521_qsl_build_to_inspiron_remote_qsc_e2ee_retry_implementation_harness.md`
- `tests/NA-0521_qsl_build_to_inspiron_remote_qsc_e2ee_retry_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- proof-root-local listener/relay;
- dedicated-key reverse forwarding;
- existing operational SSH trigger commands needed for qsc remote commands;
- retained remote qsc binary at `$HOME/qsl-remote-test/bin/qsc`;
- synthetic messages only;
- remote artifacts under `$HOME/qsl-remote-test/e2ee/<PROOF_ID>` only;
- cleanup/retention proof.

Future NA-0521 forbidden scope should preserve:

- no qsl-server/qsl-attachments;
- no package installation;
- no sudo/admin action;
- no SSH key generation/installation;
- no authorized_keys mutation;
- no SSH config mutation;
- no known_hosts mutation;
- no remote host mutation outside the scoped qsl remote test E2EE root;
- no remote source checkout/build;
- no qwork/qstart/qresume;
- no qsl-backup;
- no qsc source/test/fuzz/Cargo mutation;
- no workflow/dependency mutation;
- no corpus/vector/input mutation;
- no formal/refimpl/service/public/backup mutation;
- no public-readiness or production-readiness claims.

Future NA-0521 acceptance criteria should require:

- retained qsc hash/path/owner rechecked;
- forwarding path rechecked;
- Build-to-Inspiron synthetic send/receive succeeds;
- Inspiron-to-Build synthetic reply succeeds;
- no secret material in proof;
- cleanup/retention result recorded;
- no public/production readiness claim;
- exactly one READY item remains after closeout.

## Future validation / marker plan

NA-0520 markers:

- `NA0520_FORWARDING_PROOF_REVIEW_CONSUMED_OK`
- `NA0520_DEDICATED_FORWARDING_KEY_USED_OK`
- `NA0520_LOOPBACK_LISTENER_STARTED_OK`
- `NA0520_REVERSE_FORWARD_STARTED_OK`
- `NA0520_EXIT_ON_FORWARD_FAILURE_OK`
- `NA0520_REMOTE_TRIGGER_STARTED_OK`
- `NA0520_TUNNEL_MARKER_SENT_OK`
- `NA0520_TUNNEL_ACK_RECEIVED_OK`
- `NA0520_TUNNEL_MARKER_TRAVERSED_OK`
- `NA0520_NO_PTY_REQUIRED_OK`
- `NA0520_NO_AGENT_X11_FORWARDING_OK`
- `NA0520_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0520_NO_REMOTE_E2EE_OK`
- `NA0520_NO_REMOTE_FILE_WRITE_OK`
- `NA0520_CLEANUP_COMPLETED_OK`
- `NA0520_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0520_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0520_ONE_READY_INVARIANT_OK`

Future NA-0521 markers:

- `NA0521_FORWARDING_CAPABILITY_CONSUMED_OK`
- `NA0521_RETAINED_QSC_HASH_RECHECKED_OK`
- `NA0521_BUILD_TO_INSPIRON_SEND_RECEIVE_OK`
- `NA0521_INSPIRON_TO_BUILD_REPLY_OK`
- `NA0521_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0521_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0521_ONE_READY_INVARIANT_OK`

## No qsc E2EE proof

NA-0520 did not run qsc E2EE and did not run qsc send/receive. It only proved SSH loopback reverse-forward marker traversal. Any later qsc E2EE claim requires a separate NA-0521 implementation lane and evidence.

`NA0520_NO_REMOTE_E2EE_OK`

## No qsl-server / no qsl-attachments boundary

NA-0520 did not use qsl-server and did not use qsl-attachments. No service deployment, public listener, attachment service, or qsl-server relay was invoked.

`NA0520_NO_QSL_SERVER_ATTACHMENTS_OK`

## Public claim / website / external review boundary

NA-0520 changed no website/public docs and made no public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim.

`NA0520_NO_PUBLIC_READINESS_CLAIM_OK`

`NA0520_NO_PRODUCTION_READINESS_CLAIM_OK`

## Backup-impact statement

NA-0520 did not run backup, restore, or qsl-backup and did not mutate backup paths. Read-only boundary proof confirmed the installed qsl-backup helper matched the expected digest by short digest prefix and that the Codex ops source inclusion count remained exactly one.

## Rejected alternatives

- Run qsc E2EE in NA-0520: rejected because NA-0520 is forwarding-capability probe only.
- Use qsl-server or qsl-attachments to bypass forwarding: rejected as out of scope.
- Use a public or non-loopback listener: rejected because the authorization is loopback-only.
- Retry the SSH forward or remote trigger after success: rejected because a single pass was sufficient and the directive bounded attempts.
- Mutate authorized_keys, known_hosts, sshd_config, or local SSH config outside proof root: rejected as out of scope.
- Write remote files for trigger proof: rejected because the trigger can use bash `/dev/tcp` without remote file writes.

## Next recommendation

Merge NA-0520 evidence after local validation and required PR checks pass. If post-merge public-safety is green inside the short attach/early-failure window, close out NA-0520 and restore NA-0521 as the sole READY item. NA-0521 should consume this forwarding capability proof and perform the bounded qsc E2EE retry without expanding public, production, crypto-complete, replay-proof, downgrade-proof, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claims.

`NA0520_ONE_READY_INVARIANT_OK`
