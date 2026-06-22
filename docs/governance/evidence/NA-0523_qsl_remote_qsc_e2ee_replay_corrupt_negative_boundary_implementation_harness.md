Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-22

# NA-0523 QSL Remote qsc E2EE Replay / Corrupt Delivery Negative Boundary Implementation Harness

## Executive summary

NA-0523 consumed the D418 startup stop, consumed NA-0522 / D417 inheritance, rechecked the retained Inspiron `qsc` binary and the dedicated reverse-forwarding path, then executed a bounded remote qsc E2EE replay/corrupt-delivery negative harness using synthetic data only.

Result classification: `REMOTE_E2EE_REPLAY_CORRUPT_NEGATIVES_PASS`.

Replay result: one duplicated synthetic delivery was accepted once, rejected on replay with `qsp_replay_reject`, and did not change the selected remote session/output state.

Corrupt-delivery result: one synthetic corrupt delivery artifact was rejected with `qsp_env_decode_failed`, produced no plaintext output, and did not change the selected remote session/output state.

Valid-path result: the Inspiron-to-Build reply succeeded after the replay negative, and a Build-to-Inspiron valid message succeeded after the corrupt-delivery negative.

No qsl-server was used. No qsl-attachments was used. No package install occurred. No remote source checkout/build occurred. No qwork/qstart/qresume was run by Codex. No qsl-backup was executed. No qsc source/test/fuzz/Cargo path, workflow/script/helper path, dependency/lockfile, corpus/vector/input, formal/refimpl/service/public/backup path, qsl-server path, or qsl-attachments path was mutated.

This is bounded synthetic negative-path evidence only. No public-readiness claim is made. No production-readiness claim is made. No public-internet-readiness claim is made. No external-review-complete claim is made. No crypto-complete claim is made. No replay-proof claim is made. No downgrade-proof claim is made. No secret-material-complete claim is made. No side-channel-free claim is made. No vulnerability-free, bug-free, or perfect-crypto claim is made.

Proof marker bundle:
- `NA0523_REMOTE_E2EE_NEGATIVE_SCOPE_CONSUMED_OK`
- `NA0523_RETAINED_QSC_HASH_RECHECKED_OK`
- `NA0523_FORWARDING_PATH_RECHECKED_OK`
- `NA0523_BASELINE_REMOTE_E2EE_READY_FOR_NEGATIVE_OK`
- `NA0523_REPLAY_NEGATIVE_REJECTED_OK`
- `NA0523_CORRUPT_DELIVERY_REJECTED_OK`
- `NA0523_NEGATIVE_NO_MUTATION_OK`
- `NA0523_VALID_PATH_REMAINS_USABLE_OK`
- `NA0523_NO_SECRET_OUTPUT_OK`
- `NA0523_CLEANUP_COMPLETED_OK`
- `NA0523_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0523_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0523_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0523_ONE_READY_INVARIANT_OK`

## Live NA-0523 scope

NA-0523 was the sole READY item at startup. It was scoped to a bounded remote qsc E2EE negative implementation harness, proof-root-local runtime, the retained remote qsc binary, the remote E2EE root, and exactly these checked-in paths:

- `docs/governance/evidence/NA-0523_qsl_remote_qsc_e2ee_replay_corrupt_negative_boundary_implementation_harness.md`
- `tests/NA-0523_qsl_remote_qsc_e2ee_replay_corrupt_negative_boundary_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

The lane did not authorize qsl-server, qsl-attachments, package installation, public service deployment, remote source checkout/build, qwork/qstart/qresume execution, qsl-backup execution, qsc source/test/fuzz/Cargo mutation, workflow/script/helper mutation, dependency mutation, corpus/vector/input mutation, formal/refimpl/service/public/backup mutation, production data, personal data, or public/production readiness claims.

## qwork proof-file verification

Codex did not run qwork, qstart, or qresume. It read the operator-created qwork proof files:

- `/srv/qbuild/work/NA-0523/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0523/.qwork/startup.qsl-protocol.json`

Required proof values matched in both `.kv` and `.json`:

- `startup_result=OK`
- `lane=NA-0523`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0523/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0523`
- `requested_lane_status=READY`

Proof HEAD and proof origin/main matched live pre-fetch refs at short SHA `a11535f37754`. Fetch occurred only after the proof/live match and disk usage was below the 95% stop threshold.

Startup queue and decision state:
- READY_COUNT 1.
- READY NA-0523.
- NA-0522 DONE.
- NA-0521 DONE.
- D-1033 exists once.
- D-1034 exists once.
- D-1035 was absent before this patch.
- Duplicate decision ID count was zero using the corrected `- **ID:** D-####` parser.

Startup main health:
- `public-safety` completed success.
- `qsc-adversarial-smoke` completed success.
- `qsc-linux-full-suite` completed skipped under policy.
- `macos-qsc-full-serial` completed skipped under policy.
- No completed red checks were present in the retrieved main check-run set.

qsl-backup boundary:
- `/usr/local/sbin/qsl-backup` was inspected read-only by hash/source-list proof only.
- The expected helper digest matched, recorded in proof by full hash and in this evidence by prefix `e9ecff3d22ed`.
- The Codex ops source inclusion count was exactly 1.
- qsl-backup, backup, or restore was not executed.

## D418 stop inheritance and recovery

D418 stopped during startup validation only. It did not mutate the repository, open a PR, create a commit, build or run qsc, execute SSH or remote commands, create an E2EE runtime root, or use qsl-server/qsl-attachments.

Root cause: the D418 startup decision parser assumed the wrong `DECISIONS.md` format. This run used the deterministic parser keyed to decision entries matching `- **ID:** D-####`, counted only those entries, and detected duplicates only from that extracted list.

D418 is not an E2EE result. It is consumed here as a procedural startup stop.

## NA-0522 / D417 inheritance

NA-0522 completed and restored NA-0523 READY. The inherited classification is `REMOTE_E2EE_REPLAY_CORRUPT_NEGATIVE_IMPLEMENTATION_READY`.

Consumed facts:
- NA-0521 remote E2EE success with wrong-mailbox boundary was consumed.
- Replayed delivery/message was selected as a direct remote negative candidate.
- Corrupt delivery artifact was selected as a direct remote negative candidate.
- Wrong-peer and stale-trust were deferred.
- Repeated-run cleanup/freshness was deferred.
- qsl-server and qsl-attachments were deferred.
- Public/production readiness was rejected.
- NA-0522 ran no remote action, no SSH execution, and no qsc send/receive.
- NA-0522 made no public/production readiness claim.

## Command surface inspection and manifest

Codex inspected the current qsc CLI/test surfaces before E2EE send/receive:

- `qsl/qsl-client/qsc/tests/same_host_client_to_client_e2e.rs`
- `qsl/qsl-client/qsc/tests/receive_e2e.rs`
- `qsl/qsl-client/qsc/tests/common/mod.rs`
- `qsl/qsl-client/qsc/tests/mock_relay_transport_na0173.rs`
- `qsl/qsl-client/qsc/src/cmd/mod.rs`
- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/src/transport/mod.rs`
- `qsl/qsl-client/qsc/src/contacts/mod.rs`
- `qsl/qsl-client/qsc/src/vault/mod.rs`

The command manifest was written under the proof root:

- `command_manifest/na0523_command_manifest.md`
- `command_manifest/na0523_command_manifest.json`

The manifest uses existing qsc surfaces only:
- passphrase-file vault init/unlock;
- identity rotate/show;
- contacts add/device list/device trust;
- relay inbox-set;
- local qsc `relay serve`;
- handshake init/poll;
- relay send/receive;
- qsc relay duplicate delivery via `--dup-pct 100`;
- qsc relay HTTP inbox push/pull for the synthetic corrupt delivery artifact.

Route-token values and passphrase contents were treated as runtime-only material. Checked-in evidence records only redacted descriptions and short hashes where needed.

## Local / remote boundary rechecks

Safe `ssh -G inspiron` fields resolved to qslcodex, batch mode, identities-only, password authentication off, strict host key checking enabled, agent/X11 forwarding off, and clear forwardings enabled on the operational host entry.

The remote boundary recheck verified:
- remote user `qslcodex`;
- non-root UID;
- no privileged groups;
- `sudo -n true >/dev/null 2>&1` failed as expected;
- `/backup/qsl` absent or unreadable;
- qwork absent;
- qsl-backup absent;
- retained qsc executable present;
- planned remote E2EE root absent before creation.

## Retained remote qsc hash/path/owner recheck

Retained remote qsc path: `/home/qslcodex/qsl-remote-test/bin/qsc`.

Owner: `qslcodex`.

Digest: matched the expected retained qsc digest; proof logs record the full digest and this evidence records the prefix `6f12ab5eec24`.

Smoke: retained qsc `--help` completed successfully.

Marker: `NA0523_RETAINED_QSC_HASH_RECHECKED_OK`.

## Local qsc provenance

Relevant qsc source, Cargo, lockfile, fuzz, and selected test paths had no changes between retained source commit `6e0796de79c9` and current clean HEAD `a11535f37754`.

Local qsc was built with:

```bash
cargo build -p qsc --locked --bin qsc
```

The build used `CARGO_TARGET_DIR` under the proof root. Local qsc `--help` completed successfully. Local qsc size was recorded as `102103920` bytes and digest prefix `267410f7d6ec` was recorded in proof.

## Forwarding / relay setup

The dedicated forwarding path was rechecked before E2EE with a proof-root listener and then cleaned up. The marker traversed remote loopback `127.0.0.1:39176` through the reverse-forward to Build-local loopback `127.0.0.1:39176` and returned an ACK.

The E2EE run used:
- Build-local qsc relay bound to `127.0.0.1:39176`;
- dedicated-key SSH reverse forward with `-N -T`;
- `ExitOnForwardFailure=yes`;
- no PTY;
- no agent forwarding;
- no X11 forwarding;
- proof-root-local SSH config only.

Marker: `NA0523_FORWARDING_PATH_RECHECKED_OK`.

## Isolated runtime roots and synthetic data

Local qsc private state, local passphrase file, local HOME/XDG/TMPDIR, local message files, copied corrupt artifact, and local output directories lived under:

- `$PROOF_DIR/sensitive_runtime/`

Remote qsc private state, remote passphrase file, remote HOME/XDG/TMPDIR, remote message files, and remote output directories lived only under:

- `/home/qslcodex/qsl-remote-test/e2ee/<PROOF_ID>/`

Synthetic message labels:
- Baseline Build-to-Inspiron: `QSL_REMOTE_E2EE_NEG_BASELINE_BUILD_TO_INSPIRON_<PROOF_ID>`
- Validity reply: `QSL_REMOTE_E2EE_NEG_VALID_REPLY_INSPIRON_TO_BUILD_<PROOF_ID>`
- Replay label: `QSL_REMOTE_E2EE_REPLAY_NEGATIVE_<PROOF_ID>`
- Corrupt label: `QSL_REMOTE_E2EE_CORRUPT_NEGATIVE_<PROOF_ID>`

No production data or personal data was used.

## Baseline E2EE setup to negative test point

The harness initialized isolated local and remote qsc roots, rotated/showed identities, exchanged public fingerprints, added/trusted peer devices, set relay inboxes, completed the handshake sequence over the loopback reverse-forwarding path, and reached the replay/corrupt negative test points.

Baseline Build-to-Inspiron delivery was generated through a qsc relay running duplicate mode. The first copy was received and decrypted successfully by retained remote qsc. This established the valid baseline needed for replay testing.

Marker: `NA0523_BASELINE_REMOTE_E2EE_READY_FOR_NEGATIVE_OK`.

## Replay negative boundary proof or deferral

Replay was executed, not deferred.

Method:
- Start qsc relay with `--dup-pct 100`.
- Send one synthetic Build-to-Inspiron payload.
- Receive the first duplicate successfully on Inspiron.
- Attempt to receive the second duplicate using the same retained remote qsc state.

Observed result:
- First receive returned success and wrote one output artifact.
- Replay receive returned nonzero.
- Replay output included `qsp_replay_reject` and `ratchet_replay_reject`.
- No duplicate plaintext artifact was created.
- Selected remote session/output state did not mutate.

Marker: `NA0523_REPLAY_NEGATIVE_REJECTED_OK`.

## Corrupt delivery negative boundary proof or deferral

Corrupt delivery was executed, not deferred.

Method:
- Run normal qsc relay.
- Enqueue synthetic corrupt bytes to the remote mailbox through the qsc relay HTTP inbox surface.
- Attempt normal remote `qsc receive` against that mailbox.

Observed result:
- Corrupt receive returned nonzero.
- Corrupt receive output included `qsp_env_decode_failed`.
- No plaintext output artifact was created.
- Selected remote session/output state did not mutate.

Marker: `NA0523_CORRUPT_DELIVERY_REJECTED_OK`.

## No-mutation checks

Selected state was the remote qsp session artifact for Build plus the relevant remote output directory file count/listing digest. qsp status diagnostics were not selected state because receive rejects intentionally record status.

Replay selected-state proof:
- Before replay negative: output count 1, session digest prefix `d1fd7b1984b5`.
- After replay negative: output count 1, same session digest prefix and same output listing digest.

Corrupt selected-state proof:
- Before corrupt negative: output count 0, session digest prefix `a0d75a5caf0e`.
- After corrupt negative: output count 0, same session digest prefix and same output listing digest.

Marker: `NA0523_NEGATIVE_NO_MUTATION_OK`.

## Valid-path usability proof or deferral

Valid path was executed, not deferred.

After replay negative, retained remote qsc sent a synthetic Inspiron-to-Build reply and local qsc received/decrypted the exact reply payload.

After corrupt negative, local qsc sent a new synthetic Build-to-Inspiron message and retained remote qsc received/decrypted the exact payload.

Marker: `NA0523_VALID_PATH_REMAINS_USABLE_OK`.

## No-secret-output review

Runtime stdout/stderr logs were scanned for private key block headers, API-token-style material, bearer material, raw synthetic route-token values, and synthetic passphrase contents. The scan covered 76 stdout/stderr files and found zero findings.

After execution, the saved runner source was redacted to remove synthetic passphrase and route-token literals. A proof-root scan then found zero matches for the raw synthetic tokens, synthetic passphrase strings, or private-key block headers.

Checked-in evidence contains no private keys, passphrases, passwords, tokens, production endpoints, backup material, qsc vault material, authorized_keys dumps, known_hosts dumps, or raw private qsc material.

Marker: `NA0523_NO_SECRET_OUTPUT_OK`.

## Cleanup / retention proof

Cleanup removed:
- remote E2EE root under `/home/qslcodex/qsl-remote-test/e2ee/<PROOF_ID>/`;
- local sensitive runtime root under `$PROOF_DIR/sensitive_runtime/`;
- local qsc relay process;
- dedicated SSH reverse-forward process.

Cleanup verification:
- remote root absent;
- local sensitive runtime absent;
- port `39176` closed;
- relay stopped;
- SSH forward stopped.

The retained remote qsc binary under `/home/qslcodex/qsl-remote-test/bin/qsc` was preserved.

Marker: `NA0523_CLEANUP_COMPLETED_OK`.

## Result classification

`REMOTE_E2EE_REPLAY_CORRUPT_NEGATIVES_PASS`

## Hostile Cryptographer Review

This proves only bounded negative behavior for two synthetic remote qsc cases. It does not prove replay-proof status, vulnerability-free status, downgrade-proof status, side-channel freedom, secret-material lifecycle completeness, crypto completeness, public readiness, or production readiness.

Remaining unproven areas include broader replay classes, concurrency/reordering cases, stale trust, wrong-peer behavior, retained-binary replacement races, downgrade paths, side channels, long-run secret lifecycle behavior, and external review.

Synthetic artifacts reduce exposure risk because they contain no production data, no personal data, no private key material, and no qsc vault material in checked-in evidence. Route-token and passphrase values were treated as runtime-only and redacted from saved runner source after use.

## Red-Team Review

Replay staging risk was controlled by the existing qsc relay duplicate mode. If duplicate artifacts had been consumed or unavailable, the lane would have recorded a replay deferral instead of fabricating proof.

Corrupt delivery risk was controlled by enqueuing synthetic corrupt bytes through the qsc relay HTTP inbox surface and checking selected state before and after the reject. If corrupt delivery had mutated selected state, the lane would have stopped and selected remediation.

Route-token/capability metadata risk was controlled by checked-in redaction and proof scans. Cleanup risk was controlled by explicit remote/local absence verification. The next hardening target should be wrong-peer/stale-trust negative behavior.

## Production SRE Review

The run is operationally useful because it proves bounded two-host qsc replay/corrupt negative behavior over the retained qsc binary and the dedicated loopback reverse-forwarding path.

Logged proof includes command manifests, startup checks, boundary checks, qsc stdout/stderr, selected state summaries, cleanup summaries, and redaction scans. Checked-in evidence summarizes only redacted/hash-safe details.

Failures remained isolated from qwork, qsl-backup, production data, qsl-server, and qsl-attachments. Remote mutation stayed under the E2EE root, and that root was removed.

This does not imply public or production readiness.

## Release-Claim Boundary Review

No public-readiness claim is made. No production-readiness claim is made. No public-internet-readiness claim is made. No external-review-complete claim is made. No crypto-complete claim is made. No replay-proof claim is made. No downgrade-proof claim is made. No secret-material-complete claim is made. No side-channel-free claim is made. No vulnerability-free, bug-free, or perfect-crypto claim is made.

## Successor selection

Because both replay and corrupt delivery negatives passed, the selected successor is:

`NA-0524 -- QSL Remote qsc E2EE Wrong-Peer / Stale-Trust Negative Scope Authorization Plan`

Exactly one READY remains mandatory until closeout.

## Future scope bundle

Future NA-0524 should authorize, but not yet implement unless directed, a wrong-peer/stale-trust remote qsc E2EE hardening lane. It should consume this replay/corrupt evidence, recheck retained qsc freshness, recheck forwarding cleanup, preserve synthetic proof redaction, require cleanup/retention proof, use no qsl-server/qsl-attachments, and make no public/production readiness claims.

## Future validation / marker plan

Future marker plan:
- `NA0524_NA0523_REPLAY_CORRUPT_CONSUMED_OK`
- `NA0524_WRONG_PEER_SCOPE_AUTHORIZED_OK`
- `NA0524_STALE_TRUST_SCOPE_AUTHORIZED_OK`
- `NA0524_RETAINED_QSC_FRESHNESS_REQUIRED_OK`
- `NA0524_FORWARDING_CLEANUP_REQUIRED_OK`
- `NA0524_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0524_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0524_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0524_ONE_READY_INVARIANT_OK`

## No qsl-server / no qsl-attachments boundary

No qsl-server command, endpoint, service, file path, or artifact was used or mutated.

No qsl-attachments command, endpoint, service, file path, or artifact was used or mutated.

Marker: `NA0523_NO_QSL_SERVER_ATTACHMENTS_OK`.

## No public/production readiness boundary

This lane is not public readiness, not production readiness, and not public-internet readiness. It used synthetic data, isolated roots, retained remote qsc, and proof-root-local runtime.

Markers:
- `NA0523_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0523_NO_PRODUCTION_READINESS_CLAIM_OK`

## Backup-impact statement

No backup or restore ran. `/usr/local/sbin/qsl-backup` was inspected read-only by hash/source-count proof only. `/backup/qsl` was not mutated. Remote `/backup/qsl` was absent or unreadable to qslcodex.

## Rejected alternatives

Rejected alternatives:
- treating D418 as an E2EE attempt;
- skipping the corrected decision parser;
- using qsl-server or qsl-attachments;
- exposing the relay beyond loopback;
- mutating qsc source/tests/Cargo or workflows;
- transferring private qsc material;
- retaining remote E2EE runtime state after proof capture;
- making any forbidden public/security/completion claim outside the explicit no-claim boundary.

## Next recommendation

Merge this evidence if validation and CI pass, then close out NA-0523 to restore NA-0524 only after post-merge public-safety is green inside the allowed short attach/early-failure window.
