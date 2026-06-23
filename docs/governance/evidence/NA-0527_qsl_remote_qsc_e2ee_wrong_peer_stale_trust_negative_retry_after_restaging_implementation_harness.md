Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-23

# NA-0527 QSL Remote qsc E2EE Wrong-Peer / Stale-Trust Negative Retry After Restaging Implementation Harness

## Executive summary

NA-0527 consumed the required D426 / D425 / D424 / D419 inheritance, rechecked the retained Inspiron `qsc` binary restaged by NA-0526, inspected the current `qsc` command surface, and attempted to establish the mandated dedicated-key loopback reverse-forwarding path before any remote E2EE send/receive.

The retained remote `qsc` at `$HOME/qsl-remote-test/bin/qsc` was rechecked as owner `qslcodex`, mode `700`, size `102103920`, SHA-256 `6bf9e59fdae397c2e0f88538d700cccbee80d229c6a979cc79555e39fea2b4f7`, and `--help` success. This matches the NA-0526 retained hash.

Remote E2EE did not run. The proof-root local relay started on loopback, but the required remote reverse-forwarding path to `127.0.0.1:39176` did not complete after bounded recovery. The first attempt was a recoverable proof-root SSH config mistake (`ClearAllForwardings yes` suppressed the explicit reverse forward). After correction, two bounded retries failed with `remote port forwarding failed for listen port 39176`; a read-only remote bind probe immediately before the final retry showed the port was available, so the exact forwarding root cause remains unresolved.

Result classification: `REMOTE_E2EE_FORWARDING_RECHECK_FAILURE`.

## Live NA-0527 scope

Startup proof and live repo checks showed:

- `READY_COUNT 1`.
- Sole READY item: `NA-0527 -- QSL Remote qsc E2EE Wrong-Peer / Stale-Trust Negative Retry After Restaging Implementation Harness`.
- `NA-0526` and `NA-0525` were DONE.
- D-1042 and D-1043 each existed once.
- D-1044 and D-1045 were absent before this patch.
- Duplicate decision count was zero.

Allowed checked-in mutation for this implementation evidence is limited to this evidence file, the NA-0527 testplan, `DECISIONS.md`, `TRACEABILITY.md`, and `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

The qwork proof files were present and copied into proof root:

- `/srv/qbuild/work/NA-0527/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0527/.qwork/startup.qsl-protocol.json`

Required qwork proof fields passed:

- `startup_result=OK`
- `lane=NA-0527`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0527/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0527`
- `requested_lane_status=READY`

The qwork proof HEAD and origin/main matched live pre-fetch refs at `bc8ed7e14834`. Fetch was performed only after this match. Disk usage before fetch was below the stop threshold: `/` at 88% and `/backup/qsl` at 27%.

## D426 / D425 / D424 / D419 inheritance

NA-0527 consumed the required inheritance:

- D426 confirmed NA-0526 DONE and NA-0527 restored READY.
- D426 confirmed D-1043 exists once and D-1044 was absent.
- D426 confirmed post-closeout public-safety and advisories were green.
- D426 ran no remote action.
- D425 classified NA-0526 as `REMOTE_PREBUILT_QSC_RESTAGING_AFTER_SECURITY_REMEDIATION_PASS_RETAINED`.
- D425 recorded PR #1325 merged at `d12385252c3d`.
- D425 recorded old stale retained remote qsc hash `6f12ab5eec2468a4146012dfc247cef15be2337cb8e8e99cefa96a8c258d91ea`.
- D425 recorded new retained remote qsc hash `6bf9e59fdae397c2e0f88538d700cccbee80d229c6a979cc79555e39fea2b4f7`.
- D425 recorded final retained path `$HOME/qsl-remote-test/bin/qsc`, owner/mode/size `qslcodex`/`700`/`102103920`, and remote `qsc --help` success.
- D424 classified NA-0525 as `REMOTE_E2EE_RETAINED_QSC_STALE_AFTER_SECURITY_REMEDIATION`; that stale retained-qsc block was resolved by NA-0526 restaging.
- D419 classified NA-0523 as `REMOTE_E2EE_REPLAY_CORRUPT_NEGATIVES_PASS`; replay and corrupt negatives passed, no-mutation checks passed, cleanup passed, and no qsl-server/qsl-attachments were used.
- NA-0527 therefore had to recheck the retained qsc before any E2EE.

## Retained-qsc freshness recheck

Local current qsc was built from clean source commit `bc8ed7e14834` with:

```bash
cargo build -p qsc --locked --bin qsc
```

The isolated proof-root build produced size `102103920` and SHA-256 `5f7196475dd25833fd2354ea9e5966412c9abf193cfffcbe5f441302e57d92c8`. That hash differs from the NA-0526 retained hash, but qsc runtime/dependency inputs had no diff from the NA-0526 implementation commit `d12385252c3d`; only governance closeout files changed between D425 and current `main`. The mismatch was therefore recorded as an isolated-build artifact hash divergence, not as source/dependency drift.

The retained remote binary was rechecked before any E2EE:

- Path: `$HOME/qsl-remote-test/bin/qsc`
- Owner: `qslcodex`
- Mode: `700`
- Size: `102103920`
- SHA-256: `6bf9e59fdae397c2e0f88538d700cccbee80d229c6a979cc79555e39fea2b4f7`
- Smoke: remote `qsc --help` passed

Recovered failure: the first remote hash/stat/help probe used a remote awk field reference under `set -u`, producing an unbound-variable error after printing the matching hash/stat. Classification: recoverable command-shape mistake. Corrective action: reran once using `cut`; final result passed.

## Command surface inspection and manifest

Codex inspected the current qsc CLI/test surfaces before send/receive or remote E2EE:

- `qsl/qsl-client/qsc/tests/same_host_client_to_client_e2e.rs`
- `qsl/qsl-client/qsc/tests/receive_e2e.rs`
- `qsl/qsl-client/qsc/tests/common/mod.rs`
- `qsl/qsl-client/qsc/tests/mock_relay_transport_na0173.rs`
- `qsl/qsl-client/qsc/tests/identity_binding.rs`
- `qsl/qsl-client/qsc/tests/identity_ux.rs`
- `qsl/qsl-client/qsc/tests/trust_onboarding_mainstream_flow_na0187.rs`
- `qsl/qsl-client/qsc/src/cmd/mod.rs`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/transport/mod.rs`
- `qsl/qsl-client/qsc/src/protocol_state/mod.rs`

The command manifest was written under the proof root:

- `$PROOF_DIR/command_manifest/na0527_command_manifest.md`
- `$PROOF_DIR/command_manifest/na0527_command_manifest.json`

The manifest used only existing qsc CLI/test surfaces:

- passphrase-file vault init/unlock;
- identity rotate/show;
- contacts add, device list/trust, verify, and show;
- relay inbox-set and local `relay serve`;
- handshake init/poll/status;
- relay send/receive;
- synthetic public identity fingerprints and trust records.

The manifest explicitly excluded qsl-server and qsl-attachments, kept route-token/passphrase contents runtime-only, and restricted remote writes to `$HOME/qsl-remote-test/e2ee/<PROOF_ID>`.

## Local / remote boundary rechecks

Safe `ssh -G inspiron` fields were parsed and proof-root SSH config was created. Checked-in evidence records only safe fields:

- Hostname: `inspiron`
- User: `qslcodex`
- Identity file basename: `qslcodex_ed25519`
- `IdentitiesOnly yes`
- `PasswordAuthentication no`
- `BatchMode yes`
- `StrictHostKeyChecking yes`
- `ForwardAgent no`
- `ForwardX11 no`

Remote boundary recheck passed before remote E2EE root creation:

- Remote user `qslcodex`.
- UID `1003`, not root.
- Groups `qslcodex`; no privileged group was present.
- Negative `sudo -n true` probe failed as expected.
- `/backup/qsl` was not readable.
- `qwork` was absent.
- `qsl-backup` was absent.
- Remote E2EE root was absent before creation.

## Local qsc provenance

Local source commit was `bc8ed7e14834`. Root `Cargo.lock` and nested `qsl/qsl-client/qsc/fuzz/Cargo.lock` both contain `quinn-proto 0.11.15`.

The current local proof-root build hash did not match the retained NA-0526 binary hash, but qsc runtime/dependency paths had no diff from D425/PR #1325. NA-0527 therefore treated retained remote qsc freshness as passing only because the retained remote hash/path/owner/mode/size exactly matched NA-0526 and current runtime inputs had no source/dependency drift. The local isolated-build hash divergence is carried as a future reproducibility note, not as a protocol or wire claim.

## Forwarding / relay setup

The planned forwarding setup was:

- Build-local qsc relay: `qsc relay serve --port 39176`.
- Source inspection confirmed relay bind uses `127.0.0.1`.
- Dedicated-key proof-root SSH config.
- Reverse forward: `127.0.0.1:39176` on Inspiron loopback to `127.0.0.1:39176` on Build loopback.
- SSH options included `-N -T` and `ExitOnForwardFailure=yes`.
- No PTY, agent, or X11 forwarding.

Attempt history:

- Attempt 1: local relay started and remote connection to forwarded loopback was refused. Root cause was proof-root SSH config `ClearAllForwardings yes`, which suppressed the explicit command-line reverse forward. This was classified as recoverable command-shape/config mistake.
- Correction: proof-root SSH config changed to `ClearAllForwardings no`.
- Attempt 2: SSH exited with `remote port forwarding failed for listen port 39176`.
- Diagnostic: read-only remote bind probe showed `127.0.0.1:39176` available and no listener in the retrieved socket table.
- Attempt 3: SSH again exited with `remote port forwarding failed for listen port 39176`.

Bounded recovery was exhausted before any qsc send/receive or remote E2EE command. This triggered `REMOTE_E2EE_FORWARDING_RECHECK_FAILURE`.

## Isolated runtime roots and synthetic data

The planned roots were:

- Local Build qsc root: `$PROOF_DIR/sensitive_runtime/local_build_qsc_root`
- Local wrong-peer qsc root: `$PROOF_DIR/sensitive_runtime/local_wrong_peer_qsc_root`
- Remote Inspiron qsc root: `$HOME/qsl-remote-test/e2ee/<PROOF_ID>/remote_qsc_root`
- Local and remote passphrase files under sensitive/proof runtime roots only.

Synthetic labels selected by manifest:

- `QSL_REMOTE_E2EE_IDTRUST_BASELINE_<PROOF_ID>`
- `QSL_REMOTE_E2EE_WRONG_PEER_NEGATIVE_<PROOF_ID>`
- `QSL_REMOTE_E2EE_STALE_TRUST_NEGATIVE_<PROOF_ID>`

Because forwarding recheck failed, no baseline message was sent and no remote E2EE receive occurred.

## Baseline E2EE setup to identity/trust negative test point

Deferred. Baseline E2EE setup did not run because the dedicated reverse-forwarding recheck failed before qsc E2EE was allowed.

## Wrong-peer negative boundary proof or deferral

Deferred by forwarding failure. The planned wrong-peer negative was to create a third synthetic local qsc root using the peer label `build` with a different public identity, send handshake init to the remote inbox, and require retained remote qsc to reject while selected remote handshake/session state remained unchanged.

This was not executed. No wrong-peer pass or fail-open claim is made.

## Stale-trust negative boundary proof or deferral

Deferred by forwarding failure. The planned stale-trust/replaced-peer negative was to mark the remote contact state changed with an intentionally wrong verification code and then require remote send to fail closed without additional selected-state mutation.

This was not executed. No stale-trust pass or fail-open claim is made.

## No-mutation checks

No identity/trust negative was executed, so negative-path no-mutation checks were not applicable. The forwarding failure occurred before qsc E2EE state was established.

Cleanup/no-residue checks passed:

- Remote E2EE root was absent after cleanup.
- Local sensitive runtime root was absent after cleanup.
- Local relay was stopped.
- SSH forward process was absent.
- Local port `39176` was closed.
- Retained remote qsc was not modified.

## Valid-path usability proof or deferral

Deferred by forwarding failure. No valid baseline send/receive occurred, so no post-negative valid-path usability claim is made.

## No-secret-output review

No qsc E2EE send/receive ran. A post-stop proof-root scan over startup, inheritance, retained-qsc, command-manifest, forwarding, and remote-E2EE logs found zero private-key block markers, bearer/API-key-style markers, or explicit passphrase/password marker findings.

Recovered warning: the first scan wrote its output inside the scanned tree and produced benign self-scan warnings. Corrective action: reran with the output file excluded; final result was zero findings.

Checked-in evidence contains no private key contents, passphrase values, passwords, tokens, production endpoints, backup material, authorized_keys dump, known_hosts dump, or raw private qsc material.

## Cleanup / retention proof

Cleanup after failed forwarding attempts passed:

- Local relay process stopped.
- SSH forward process was stopped or already absent.
- Remote E2EE root `$HOME/qsl-remote-test/e2ee/<PROOF_ID>` was removed/absent.
- Local `$PROOF_DIR/sensitive_runtime` was removed/absent.
- Local loopback port `39176` was closed.
- Retained remote qsc at `$HOME/qsl-remote-test/bin/qsc` was left unchanged.

## Result classification

Selected classification:

```text
REMOTE_E2EE_FORWARDING_RECHECK_FAILURE
```

This is a forwarding recheck stop, not a qsc wrong-peer/stale-trust negative result.

## Hostile Cryptographer Review

This run proves only bounded operational preconditions: qwork handoff, retained-qsc freshness against NA-0526, command-surface availability, remote account boundary checks, and cleanup after forwarding failure. It does not prove identity-complete or trust-complete behavior because the wrong-peer and stale-trust negatives did not execute.

Authentication, replay/downgrade resistance, side channels, secret-material lifecycle, and formal identity/trust properties remain unproven by NA-0527. D419 replay/corrupt evidence remains inherited, but NA-0527 makes no replay-proof or downgrade-proof claim.

Synthetic identities and trust records were planned and partially prepared only under isolated proof roots. No production data or personal data was used.

Public/production readiness remains unclaimed because a prerequisite loopback forwarding boundary failed before E2EE execution.

## Red-Team Review

If wrong-peer cannot be staged because qsc contact surfaces prevent it, the next lane must record that as command-surface truth rather than fabricating an internal test. If stale-trust/replaced-peer surfaces require internals, the next lane should split a command-surface diagnostic before attempting E2EE.

The immediate red-team finding is lower-level: the required remote loopback reverse-forward failed even after the proof-root config correction. A future diagnostic should distinguish sshd policy, dedicated-key restrictions, bind-address syntax, stale master/session behavior, and local proof config shape before any qsc send/receive.

Route-token/capability metadata remains a leak risk for future E2EE runs and must stay runtime-only with checked-in redaction. Cleanup risk was controlled in this run; both local and remote sensitive roots were absent after cleanup.

## Production SRE Review

Wrong-peer/stale-trust remote testing remains operationally useful but bounded. NA-0527 did not reach that layer because transport precondition failed.

Logged proof includes qwork verification, queue/decision state, retained-qsc proof, command manifest, remote boundary checks, forwarding attempt logs, recovered-failure classification, and cleanup proof. Runtime secret values were not included in checked-in evidence.

Failures remained isolated from qwork, qsl-backup, `/backup/qsl`, production data, qsl-server, and qsl-attachments. No public service deployment, package installation, sudo/admin action beyond the authorized negative probe, remote source checkout/build, or retained-qsc mutation occurred.

This does not imply public-readiness, production-readiness, public-internet-readiness, external-review-complete status, crypto-complete status, identity-complete status, trust-complete status, replay-proof status, downgrade-proof status, secret-material-complete status, side-channel-free status, vulnerability-free status, bug-free status, or perfect-crypto status.

## Release-Claim Boundary Review

NA-0527 makes no public-readiness claim.
NA-0527 makes no production-readiness claim.
NA-0527 makes no public-internet-readiness claim.
NA-0527 makes no external-review-complete claim.
NA-0527 makes no crypto-complete claim.
NA-0527 makes no identity-complete claim.
NA-0527 makes no trust-complete claim.
NA-0527 makes no replay-proof claim.
NA-0527 makes no downgrade-proof claim.
NA-0527 makes no secret-material-complete claim.
NA-0527 makes no side-channel-free claim.
NA-0527 makes no vulnerability-free, bug-free, or perfect-crypto claim.

## Successor selection

Selected successor candidate:

```text
NA-0528 -- QSL Remote qsc E2EE Reverse-Forwarding Diagnostic / Retry Scope Authorization Plan
```

The successor should authorize a focused forwarding diagnostic before retrying wrong-peer/stale-trust E2EE. It should not run qsc send/receive until the loopback reverse-forward path is proven stable.

## Future scope bundle

Future scope should include:

- Recheck retained remote qsc hash/path/owner/mode/size before any E2EE.
- Recheck dedicated-key policy and proof-root SSH config shape.
- Diagnose why explicit remote forwarding to `127.0.0.1:39176` failed despite read-only bind availability proof.
- Compare only within authorized diagnostics whether remote bind syntax or key restrictions require an operator action.
- Preserve no qsl-server and no qsl-attachments.
- Preserve no package installation, no remote source checkout/build, no qsc source/test/fuzz/Cargo mutation, no workflow/script/helper/dependency mutation, no corpus/vector/input mutation, and no public/production readiness claims.

## Future validation / marker plan

Future validation markers should include:

- `NA0528_QWORK_PROOF_FILES_VERIFIED_OK`
- `NA0528_RETAINED_QSC_HASH_RECHECKED_OK`
- `NA0528_REMOTE_BOUNDARY_RECHECKED_OK`
- `NA0528_FORWARDING_FAILURE_ROOT_CAUSE_CLASSIFIED_OK`
- `NA0528_REVERSE_FORWARDING_PATH_PROVEN_OR_REMEDIATION_SELECTED_OK`
- `NA0528_NO_QSC_SEND_RECEIVE_BEFORE_FORWARDING_OK`
- `NA0528_REMOTE_E2EE_ROOT_CLEANED_OK`
- `NA0528_LOCAL_SENSITIVE_RUNTIME_CLEANED_OK`
- `NA0528_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0528_NO_PUBLIC_PRODUCTION_READINESS_CLAIM_OK`
- `NA0528_ONE_READY_INVARIANT_OK`

## No qsl-server / no qsl-attachments boundary

No qsl-server command, process, endpoint, source path, test path, workflow path, or runtime artifact was used or modified.

No qsl-attachments command, process, endpoint, source path, test path, workflow path, or runtime artifact was used or modified.

## No public/production readiness boundary

NA-0527 is failure evidence for a prerequisite forwarding recheck. It is not public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, identity-complete, trust-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto evidence.

## Backup-impact statement

No backup or restore was run. `/usr/local/sbin/qsl-backup` was read only for checksum/source-list proof and matched the required SHA-256. `/backup/qsl` was not mutated. Remote `/backup/qsl` was not readable by `qslcodex`.

## Rejected alternatives

- Continue retrying reverse-forwarding after the bounded retry budget (rejected: directive requires stop after bounded recovery).
- Run qsc E2EE over any transport other than the authorized loopback reverse-forwarding path (rejected: would leave scope).
- Use qsl-server or qsl-attachments to bypass the forwarding issue (rejected: explicitly out of scope).
- Mutate remote authorized_keys, known_hosts, SSH config, sshd config, retained qsc, source checkout, dependencies, or workflows (rejected: explicitly out of scope).
- Claim wrong-peer or stale-trust pass/defer on command-surface grounds (rejected: the actual blocker was forwarding before negative execution).

## Next recommendation

Run a narrow NA-0528 forwarding diagnostic / retry authorization lane. The lane should isolate sshd/key restriction behavior from proof-root SSH config shape, prove the remote loopback reverse-forward path before any qsc E2EE, and only then authorize retrying the wrong-peer/stale-trust negative harness.
