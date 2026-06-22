Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-22

# NA-0521 Build-to-Inspiron Remote qsc E2EE Retry Implementation Harness

## Executive summary

NA-0521 recovered the D415 boundary-recheck stop, verified the two legacy D415 `/tmp` residue paths were absent, rechecked the retained Inspiron qsc binary, rechecked the dedicated-key reverse-forwarding path, and executed a bounded Build-to-Inspiron qsc client-to-client E2EE workflow over loopback relay transport.

Result classification: `REMOTE_BUILD_TO_INSPIRON_E2EE_PASS_WITH_NEGATIVE_BOUNDARY`.

This is bounded workflow-interoperability evidence only. No public-readiness claim is made. No production-readiness claim is made. No public-internet-readiness claim is made. No external-review-complete claim is made. No crypto-complete claim is made. No replay-proof claim is made. No downgrade-proof claim is made. No secret-material-complete claim is made. No side-channel-free claim is made. No vulnerability-free, bug-free, or perfect-crypto claim is made.

Proof marker bundle:
- `NA0521_D415_RESIDUE_CLEANUP_OK`
- `NA0521_REMOTE_BOUNDARY_RECHECK_OK`
- `NA0521_RETAINED_QSC_HELP_OK`
- `NA0521_FORWARDING_RECHECK_OK`
- `NA0521_LOCAL_QSC_PROVENANCE_OK`
- `NA0521_E2EE_REVERSE_FORWARD_STARTED_OK`
- `NA0521_HANDSHAKE_ESTABLISHED_BOTH_SIDES_OK`
- `NA0521_BUILD_TO_INSPIRON_SEND_RECEIVE_OK`
- `NA0521_INSPIRON_TO_BUILD_REPLY_RECEIVE_OK`
- `NA0521_REMOTE_NEGATIVE_REJECT_NO_MUTATION_OK`
- `NA0521_NO_SECRET_OUTPUT_SCAN_OK`
- `NA0521_REMOTE_E2EE_ROOT_REMOVED_OK`
- `NA0521_LOCAL_SENSITIVE_RUNTIME_REMOVED_OK`
- `NA0521_E2EE_CLEANUP_OK`
- `REMOTE_BUILD_TO_INSPIRON_E2EE_PASS_WITH_NEGATIVE_BOUNDARY`

## Live NA-0521 scope

NA-0521 was the sole READY item. The lane was limited to governance evidence/testplan/decision/traceability/journal mutation, proof-root-local runtime, retained remote qsc execution, the remote E2EE root, the exact D415 residue cleanup check, and synthetic messages only.

No qsc source/test/fuzz/Cargo path, workflow/script/helper path, dependency/lockfile, corpus/vector/input path, formal/refimpl/service/public/backup path, qsl-server path, or qsl-attachments path was mutated.

## qwork proof-file verification

Codex did not run qwork, qstart, or qresume. It read the qwork proof files from `/srv/qbuild/work/NA-0521/.qwork/`, verified `startup_result=OK`, `lane=NA-0521`, `repo=qsl-protocol`, clean worktree/index/untracked state, `ready_count=1`, and `queue_top_ready=NA-0521`.

The proof HEAD and proof origin/main matched live pre-fetch refs at `56521dced009`, and fetch occurred only after that freshness proof and disk usage below the 95% stop threshold.

## D415 stop inheritance and recovery

D415 was consumed as a recovered stop, not as an E2EE attempt. D415 stopped before repo mutation with classification `REMOTE_E2EE_BOUNDARY_RECHECK_FAILURE`.

Inherited D415 facts:
- no implementation PR;
- no local qsc build;
- no forwarding process;
- no E2EE roots;
- no qsc send/receive;
- no qsl-server or qsl-attachments use;
- no D-1031;
- retained qsc path/owner proof partially reached;
- retained hash check did not complete;
- remote sudo negative probe redirected to `/tmp/na0521_sudo_probe_out` and `/tmp/na0521_sudo_probe_err`;
- later command shape failed with `$1` unbound.

Recovery hardening used `/dev/null` for the sudo probe and generated remote scripts with no shell positional parameters.

## NA-0520 / D414 inheritance

NA-0520 / D414 was consumed as SSH forwarding capability evidence only. The inherited result was `SSH_FORWARDING_CAPABILITY_PROBE_PASS`: the dedicated qslcodex forwarding key established loopback reverse forwarding for `127.0.0.1:39176`, a synthetic marker traversed the tunnel, and ACK returned.

NA-0520 did not run qsc E2EE, qsc send/receive, qsl-server, qsl-attachments, or remote file writes.

## D415 residue cleanup proof

Codex ran exactly one bounded SSH command before other remote work to check the two D415 residue paths. The command removed nothing because both files were absent.

Markers:
- `NA0521_D415_RESIDUE_CHECK_OK`
- `NA0521_D415_RESIDUE_ABSENT_OUT`
- `NA0521_D415_RESIDUE_ABSENT_ERR`
- `NA0521_D415_RESIDUE_CLEANUP_OK`

## Command surface inspection and manifest

Codex inspected current qsc CLI/test surfaces, including same-host client-to-client E2E, receive E2E, qsc common helpers, handshake tests, identity/trust tests, qsc command declarations, transport, relay, contacts, and vault code.

The command manifest was written under the proof root as:
- `command_manifest/na0521_command_manifest.md`
- `command_manifest/na0521_command_manifest.json`

The manifest used existing qsc surfaces only: passphrase-file vault init/unlock, identity rotate/show, contacts add/device list/device trust, relay inbox-set, local `relay serve`, handshake init/poll/status, relay send/receive, and the same-host wrong-mailbox negative boundary.

Raw route-token values were not placed in checked-in evidence. Proof logs record redacted/hash markers only.

## Local / remote boundary rechecks

Local `ssh -G inspiron` safe fields resolved to qslcodex, batch mode, identities-only, password auth off, strict host key checking, agent/X11 forwarding off, and clear forwardings enabled for the operational command path.

The remote boundary recheck verified:
- user `qslcodex`, UID nonzero;
- no privileged groups;
- `sudo -n true >/dev/null 2>&1` failed as expected;
- `/backup/qsl` absent or unreadable;
- qwork absent;
- qsl-backup absent;
- retained qsc executable present;
- planned E2EE root absent before creation.

## Retained remote qsc hash/path/owner recheck

Retained remote qsc path: qsl-remote-test-relative `qsl-remote-test/bin/qsc`.

Owner: `qslcodex`.

Digest: matched the directive-retained qsc digest, recorded in proof logs with prefix `6f12ab5eec24`.

Smoke: retained qsc `--help` completed successfully.

## Local qsc provenance

Local qsc was built from current clean qsl-protocol HEAD `56521dced009` using `cargo build -p qsc --locked --bin qsc` with `CARGO_TARGET_DIR` under the proof root.

Relevant qsc source, Cargo, lockfile, and selected E2E test paths had no changes between retained source commit `6e0796de79c9` and current HEAD, so local/remote command-surface compatibility remained within the retained binary basis.

Local qsc digest prefix: `baa81487058c`. Local qsc `--help` completed successfully.

## Forwarding / relay setup

Forwarding was rechecked before E2EE with a proof-root listener and then cleaned up.

The E2EE run then started local qsc `relay serve` on Build loopback `127.0.0.1:39176` and a dedicated-key SSH reverse forward with `-N -T`, `ExitOnForwardFailure=yes`, no PTY, no agent forwarding, and no X11 forwarding.

The relay was proof-root-local, not a daemon, and was stopped during cleanup.

## Isolated runtime roots and synthetic data

Local qsc private state, local passphrase file, local HOME/XDG/TMPDIR, message files, and output directories lived under the proof root `sensitive_runtime/` until cleanup.

Remote qsc private state, remote passphrase file, remote HOME/XDG/TMPDIR, message files, and output directories lived only under the remote E2EE root.

Synthetic message labels:
- `QSL_REMOTE_E2EE_SYNTHETIC_BUILD_TO_INSPIRON_<proof-id>`
- `QSL_REMOTE_E2EE_SYNTHETIC_INSPIRON_TO_BUILD_<proof-id>`

No production data or personal data was used.

## Build-to-Inspiron send/receive proof

The flow initialized isolated local and remote qsc roots, rotated identities, exchanged public fingerprints, trusted peer devices, established relay inboxes, completed the handshake sequence, sent the Build-to-Inspiron synthetic payload through qsc relay transport, and verified the remote received payload exactly.

Markers:
- `NA0521_BUILD_TO_INSPIRON_SEND_OK`
- `NA0521_BUILD_TO_INSPIRON_RECEIVE_OK`
- `NA0521_BUILD_TO_INSPIRON_SEND_RECEIVE_OK`

## Inspiron-to-Build reply proof

The retained remote qsc sent the synthetic reply to Build over the same loopback/reverse-forward relay path. Local qsc received and verified the exact reply payload.

Markers:
- `NA0521_INSPIRON_TO_BUILD_SEND_OK`
- `NA0521_INSPIRON_TO_BUILD_REPLY_RECEIVE_OK`

## Negative/no-mutation boundary proof or deferral

The lane executed one same-host-surface-derived wrong-mailbox negative boundary on the remote receiver before the valid Build-to-Inspiron receive.

Pre-state captured the remote output directory file count and remote Alice session digest. The invalid mailbox receive failed with the expected route-token/mailbox rejection marker, produced no receive artifacts, did not mutate the session artifact, and the valid receive then succeeded.

Marker: `NA0521_REMOTE_NEGATIVE_REJECT_NO_MUTATION_OK`.

## No-secret-output review

Proof logs were scanned for private key blocks, API-token-style material, bearer material, passphrase/private-key diagnostic markers, and route-token leaks. The scan passed.

Marker: `NA0521_NO_SECRET_OUTPUT_SCAN_OK`.

Checked-in evidence contains no private keys, passphrases, passwords, tokens, production endpoints, backup material, or qsc vault material.

## Cleanup / retention proof

Cleanup removed the remote E2EE root and local sensitive runtime root, stopped the local qsc relay, stopped the dedicated SSH forward, and verified no exact qsc relay/SSH forwarding process remained and port `39176` was not listening.

Markers:
- `NA0521_REMOTE_E2EE_ROOT_REMOVED_OK`
- `NA0521_LOCAL_SENSITIVE_RUNTIME_REMOVED_OK`
- `NA0521_E2EE_CLEANUP_OK`

The retained remote qsc binary was preserved.

## Result classification

`REMOTE_BUILD_TO_INSPIRON_E2EE_PASS_WITH_NEGATIVE_BOUNDARY`

## Hostile Cryptographer Review

This proves bounded workflow interoperability across Build and Inspiron using current qsc CLI surfaces, retained remote qsc, synthetic data, and loopback SSH forwarding. It does not prove replay resistance, downgrade resistance, crypto completeness, or side-channel safety.

Retained binary replacement remains a residual risk between recheck and use; this lane reduced but did not eliminate that risk by checking owner/hash/help immediately before E2EE. Synthetic messages avoid secret exposure, but this remains a synthetic harness, not external review or release proof.

## Red-Team Review

If the retained qsc changes after recheck, a future repeated-run lane should recheck immediately before each action or stage a fresh binary. If transfer accidentally includes private material, the no-secret-output and route-token scans should fail closed, but future work should further separate public fingerprint exchange from capability-bearing route metadata.

If qsc output leaks secret-looking material, evidence publication must stop. If relay/tunnel artifacts remain, cleanup must fail. If a negative boundary fails open, the selected successor should be remediation rather than hardening.

## Production SRE Review

The run is operationally useful because it proves a bounded two-machine qsc workflow with isolated roots, loopback relay, dedicated forwarding, synthetic messages, cleanup, and no qsl-server/qsl-attachments.

Logs were captured under the proof root and checked for secret-shaped material. Local sensitive runtime and remote E2EE state were removed. Failures remained isolated from qwork, qsl-backup, production data, qsl-server, qsl-attachments, and public services.

This does not imply public or production readiness.

## Release-Claim Boundary Review

No public-readiness claim is made. No production-readiness claim is made. No public-internet-readiness claim is made. No external-review-complete claim is made. No crypto-complete claim is made. No replay-proof claim is made. No downgrade-proof claim is made. No secret-material-complete claim is made. No side-channel-free claim is made. No vulnerability-free, bug-free, or perfect-crypto claim is made.

## Successor selection

Because the positive flow and one negative boundary passed, the selected successor is:

`NA-0522 -- QSL Remote qsc E2EE Negative / Residual Hardening Scope Authorization Plan`

## Future scope bundle

Future NA-0522 should authorize additional negative/no-mutation boundaries, repeated-run safety, relay residual checks, cleanup/retention hardening, retained-binary freshness checks, and route-token evidence redaction hardening.

It should not introduce qsl-server or qsl-attachments integration unless separately authorized, and it should not claim public/production readiness.

## Future validation / marker plan

Future markers should include repeated retained qsc hash rechecks, additional wrong-peer/replay/corrupt-delivery boundaries, post-negative valid-path usability, no route-token evidence leakage, remote root cleanup proof, local sensitive runtime cleanup proof, no qsl-server/qsl-attachments, no public/production readiness claims, and one-READY invariant proof.

## No qsl-server / no qsl-attachments boundary

No qsl-server or qsl-attachments command, endpoint, service, file path, or artifact was used or mutated. qsc used its own loopback demo relay transport through the SSH reverse-forward path.

## No public/production readiness boundary

This lane is not public readiness, not production readiness, and not public-internet readiness. It uses synthetic data, isolated roots, retained remote qsc, and proof-root-local runtime.

## Backup-impact statement

No backup or restore ran. `/usr/local/sbin/qsl-backup` was inspected read-only by hash/source-count proof only. `/backup/qsl` was not mutated. Remote `/backup/qsl` was absent or unreadable to qslcodex.

## Rejected alternatives

Rejected alternatives:
- treating D415 as a successful attempt;
- ignoring the D415 residue concern;
- using qsl-server or qsl-attachments;
- exposing the relay beyond loopback;
- mutating qsc source/tests/Cargo or workflows;
- transferring private qsc material;
- retaining remote E2EE runtime state after proof capture.

## Next recommendation

Proceed to a focused NA-0522 authorization lane for additional remote qsc E2EE negative/residual hardening while preserving synthetic data, isolated roots, cleanup proof, and no public/production readiness claims.
