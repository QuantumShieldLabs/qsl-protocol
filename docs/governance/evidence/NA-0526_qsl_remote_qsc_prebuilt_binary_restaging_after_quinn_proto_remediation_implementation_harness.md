Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-23

# NA-0526 QSL Remote qsc Prebuilt Binary Restaging After quinn-proto Remediation Implementation Harness

## Executive summary

NA-0526 restaged the approved Inspiron `qslcodex` retained `qsc` binary after the D422 `quinn-proto 0.11.15` remediation.

The old retained remote binary at `$HOME/qsl-remote-test/bin/qsc` hashed to `6f12ab5eec2468a4146012dfc247cef15be2337cb8e8e99cefa96a8c258d91ea`. A fresh local `qsc` build from clean source commit `2cff954de589` produced SHA-256 `6bf9e59fdae397c2e0f88538d700cccbee80d229c6a979cc79555e39fea2b4f7`, size `102103920`, and passed local `--help` smoke.

Codex staged the fresh binary to `$HOME/qsl-remote-test/bin/qsc.NA0526_20260623T122810Z.stage`, verified the staged hash matched the local hash, moved it to `$HOME/qsl-remote-test/bin/qsc`, and verified final owner `qslcodex`, mode `700`, size `102103920`, and the same SHA-256. Remote `qsc --help` passed. No remote E2EE, qsc send/receive, wrong-peer or stale-trust testing, qsl-server, or qsl-attachments were used.

Result classification: `REMOTE_PREBUILT_QSC_RESTAGING_AFTER_SECURITY_REMEDIATION_PASS_RETAINED`.

## Live NA-0526 scope

Startup proof and live repo checks showed:

- `READY_COUNT 1`.
- Sole READY item: `NA-0526 -- QSL Remote qsc Prebuilt Binary Restaging After quinn-proto Remediation Implementation Harness`.
- `NA-0525`, `NA-0524`, and `NA-0523` were DONE.
- D-1040 and D-1041 each existed once.
- D-1042 and D-1043 were absent before this patch.
- Duplicate decision count was zero.

Allowed checked-in mutation for this implementation evidence is limited to this evidence file, the NA-0526 testplan, `DECISIONS.md`, `TRACEABILITY.md`, and `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

The qwork proof files were present and copied into proof root:

- `/srv/qbuild/work/NA-0526/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0526/.qwork/startup.qsl-protocol.json`

Required qwork proof fields passed:

- `startup_result=OK`
- `lane=NA-0526`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0526/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0526`
- `requested_lane_status=READY`

The qwork proof HEAD and origin/main matched live pre-fetch refs at `2cff954de589`. Fetch was performed only after this match and disk usage remained below the 95% stop threshold.

## D424 / D423 / D422 inheritance

NA-0526 consumed the required inheritance:

- NA-0525 completed and NA-0526 was restored READY by D-1041.
- D424 classified NA-0525 as `REMOTE_E2EE_RETAINED_QSC_STALE_AFTER_SECURITY_REMEDIATION`.
- D424 recorded old retained remote qsc hash `6f12ab5eec2468a4146012dfc247cef15be2337cb8e8e99cefa96a8c258d91ea`.
- D424 recorded current local qsc hash `8292f3473909e499017912c3f814509b2a2b67d0fd8809375dad894f9d5858a2`.
- D422 remediated root `Cargo.lock` and nested `qsl/qsl-client/qsc/fuzz/Cargo.lock` to `quinn-proto 0.11.15`.
- D423 verified post-remediation public-safety and advisories green before restoring NA-0525.
- NA-0526 was selected to restage the current `qsc` binary before retrying wrong-peer/stale-trust remote E2EE.
- No remote E2EE was run in NA-0525.
- No qsl-server or qsl-attachments were used.

The fresh NA-0526 local hash differs from the D424 local hash, but the qsc runtime, qsc tests, qsc fuzz, Cargo manifest, and lockfile paths had no diff from the D424 source commit. This keeps the directive inside the allowed restaging lane.

## Local current qsc build/provenance

Build command:

```bash
cargo build -p qsc --locked --bin qsc
```

Build environment:

- Proof root: `/srv/qbuild/tmp/NA0526_remote_qsc_prebuilt_restaging_after_quinn_proto_impl_20260623T122810Z`
- `TMPDIR`: proof-root `local_tmp`
- `CARGO_TARGET_DIR`: proof-root `cargo_target`

Local current qsc:

- Source commit: `2cff954de589`
- Root `Cargo.lock` qsc dependency proof: `quinn-proto 0.11.15`
- Nested qsc fuzz lock proof: `quinn-proto 0.11.15`
- Binary path: proof-root `cargo_target/debug/qsc`
- Size: `102103920`
- SHA-256: `6bf9e59fdae397c2e0f88538d700cccbee80d229c6a979cc79555e39fea2b4f7`
- File type: ELF 64-bit Linux executable
- Local smoke: `qsc --help` passed

Post-build proof showed no checked-in source, Cargo, dependency, lockfile, qsc test, or qsc fuzz mutation.

## Remote prep/pre-state

Remote boundary checks used bounded SSH to `inspiron` and only account, path, stat, hash, and allowed bin-directory commands.

Remote account/boundary proof:

- Remote user: `qslcodex`
- UID: `1003`
- Groups: `qslcodex`
- Negative sudo probe failed as expected.
- `/backup/qsl` was absent.
- `qwork` was absent.
- `qsl-backup` was absent.
- `$HOME/qsl-remote-test/bin` existed after verification/preparation.

Pre-state final qsc:

- Path: `$HOME/qsl-remote-test/bin/qsc`
- Owner/group: `qslcodex/qslcodex`
- Mode: `700`
- Size: `102103920`
- SHA-256: `6f12ab5eec2468a4146012dfc247cef15be2337cb8e8e99cefa96a8c258d91ea`

Stage path pre-state:

- `$HOME/qsl-remote-test/bin/qsc.NA0526_20260623T122810Z.stage` was absent.

## Transfer/stage proof

Transfer command shape:

```bash
scp -o BatchMode=yes -o PasswordAuthentication=no -o ConnectTimeout=10 <local-qsc> inspiron:qsl-remote-test/bin/qsc.NA0526_20260623T122810Z.stage
```

Transfer result:

- Exactly one bounded transfer command was used.
- Transfer exit code: `0`
- Local transfer size: `102103920`

Staged remote binary proof:

- Owner/group: `qslcodex/qslcodex`
- Mode after chmod: `700`
- Size: `102103920`
- Stage SHA-256: `6bf9e59fdae397c2e0f88538d700cccbee80d229c6a979cc79555e39fea2b4f7`
- Stage hash matched local SHA-256.

## Final move/hash verify proof

After staged hash verification, Codex moved the stage path to final:

- Stage path after move: absent.
- Final path: `$HOME/qsl-remote-test/bin/qsc`
- Final owner/group: `qslcodex/qslcodex`
- Final mode: `700`
- Final size: `102103920`
- Final SHA-256: `6bf9e59fdae397c2e0f88538d700cccbee80d229c6a979cc79555e39fea2b4f7`
- Final SHA-256 matched the local binary.
- Final SHA-256 differs from old stale hash `6f12ab5eec2468a4146012dfc247cef15be2337cb8e8e99cefa96a8c258d91ea`.

## Remote smoke proof

Remote smoke command:

```bash
$HOME/qsl-remote-test/bin/qsc --help
```

Result: passed.

Only help output was requested. No qsc send/receive, E2EE, identity, contact, handshake, relay, or trust command was run.

## Retention/cleanup decision

Decision: retain final `$HOME/qsl-remote-test/bin/qsc` for NA-0527.

Recorded cleanup command if the Director later chooses removal:

```bash
ssh -o BatchMode=yes -o PasswordAuthentication=no -o ConnectTimeout=10 inspiron 'rm -f -- "$HOME/qsl-remote-test/bin/qsc"'
```

Cleanup/absence proof:

- Stage path absent after final move.
- Proof-specific remote E2EE root `$HOME/qsl-remote-test/e2ee/NA0526_20260623T122810Z` absent.
- No local `sensitive_runtime` or local proof-specific E2EE root was created.
- No remote `qsc` process remained after smoke.
- No non-ancestor local process with the proof ID remained after the corrected process scan.

NA-0527 must recheck final path, owner, mode, and hash before any remote E2EE.

## Result classification

`REMOTE_PREBUILT_QSC_RESTAGING_AFTER_SECURITY_REMEDIATION_PASS_RETAINED`

## Hostile Cryptographer Review

Restaging proves only binary freshness/provenance and non-protocol startup for the retained remote `qsc` executable. It does not prove protocol correctness, identity correctness, trust correctness, replay resistance, downgrade resistance, side-channel behavior, or secret lifecycle behavior.

No E2EE or identity/trust claims are made because this lane intentionally did not run E2EE, send/receive, identity, contact, handshake, relay, wrong-peer, or stale-trust commands. Remote help smoke only proves the executable starts and exposes its command surface.

NA-0527 must recheck the retained hash before use because the retained binary is mutable remote state. A later operator must not rely on this evidence alone if the remote path, owner, mode, size, or hash has drifted.

Remaining unproven areas include identity/trust binding behavior, replay and downgrade resistance under this binary, side-channel characteristics, private material handling during remote E2EE, cleanup of future E2EE roots, and state no-mutation on identity/trust rejects.

## Red-Team Review

If staged binary hash mismatches, the correct action is to remove the stage path and stop with `REMOTE_PREBUILT_QSC_STAGE_HASH_MISMATCH`. That did not occur.

If final owner, path, mode, size, or hash drifts, the correct action is to stop before remote E2EE. Final verification passed for this run, but NA-0527 must recheck.

If the stale binary is accidentally reused, wrong-peer/stale-trust evidence becomes ambiguous and must be rejected. NA-0526 replaced the stale hash and records the old and new hashes to prevent silent reuse.

Cleanup/revocation proof needed for future lanes includes stage absence, proof-specific E2EE root cleanup or retention rationale, no leftover qsc/relay/forwarding processes, and no checked-in private material.

NA-0527 must recheck retained qsc hash/path/owner/mode, forwarding path, synthetic-only data boundaries, no qsl-server/qsl-attachments boundary, and cleanup/retention state.

## Production SRE Review

Restaging was operationally necessary because D422 remediated a dependency advisory after the prior retained remote binary was staged. Running new remote E2EE against the old retained binary would have produced stale evidence.

Logged evidence includes qwork proof status, repo SHAs, local build hash/size, remote owner/mode/size/hash, transfer exit code, stage/final hash checks, smoke result, cleanup/absence checks, validation outputs, and the recovered proof-tooling failure. Private key, passphrase, token, password, production data, and personal data were not logged in checked-in evidence.

Retained state: final remote `$HOME/qsl-remote-test/bin/qsc` remains for NA-0527 with SHA-256 `6bf9e59fdae397c2e0f88538d700cccbee80d229c6a979cc79555e39fea2b4f7`.

qsl-server and qsl-attachments were not used because this lane only restages a prebuilt qsc client binary and must not attach service infrastructure. This evidence does not imply public readiness, production readiness, or public-internet readiness.

## Release-Claim Boundary Review

This lane makes no public-ready claim, no production-ready claim, no public-internet-ready claim, no external-review-complete claim, no crypto-complete claim, no identity-complete claim, no trust-complete claim, no replay-proof claim, no downgrade-proof claim, no secret-material-complete claim, no side-channel-free claim, and no vulnerability-free, bug-free, or perfect-crypto claim.

## Successor selection

Because the result classification is `REMOTE_PREBUILT_QSC_RESTAGING_AFTER_SECURITY_REMEDIATION_PASS_RETAINED`, the selected successor is:

`NA-0527 -- QSL Remote qsc E2EE Wrong-Peer / Stale-Trust Negative Retry After Restaging Implementation Harness`

## Future scope bundle

Future NA-0527 should use retained remote `qsc` only after rechecking:

- path `$HOME/qsl-remote-test/bin/qsc`;
- owner `qslcodex`;
- executable mode;
- size `102103920`;
- SHA-256 `6bf9e59fdae397c2e0f88538d700cccbee80d229c6a979cc79555e39fea2b4f7`;
- no stale stage residue;
- synthetic-only local and remote runtime roots;
- no qsl-server or qsl-attachments;
- cleanup/retention proof after execution.

## Future validation / marker plan

Future NA-0527 markers should include:

- `NA0527_RETAINED_QSC_HASH_RECHECKED_OK`
- `NA0527_FORWARDING_PATH_RECHECKED_OK`
- `NA0527_BASELINE_REMOTE_E2EE_READY_FOR_NEGATIVE_OK`
- `NA0527_WRONG_PEER_NEGATIVE_REJECTED_OK`
- `NA0527_STALE_TRUST_NEGATIVE_REJECTED_OK`
- `NA0527_NEGATIVE_NO_MUTATION_OK`
- `NA0527_VALID_PATH_REMAINS_USABLE_OK`
- `NA0527_NO_SECRET_OUTPUT_OK`
- `NA0527_CLEANUP_COMPLETED_OK`
- `NA0527_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0527_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0527_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0527_ONE_READY_INVARIANT_OK`

## No remote E2EE

No remote E2EE was attempted or run in NA-0526.

## No qsc send/receive

No qsc send or qsc receive command was attempted or run in NA-0526.

## No qsl-server / no qsl-attachments boundary

NA-0526 did not use, start, mutate, or require qsl-server or qsl-attachments.

## No public/production readiness boundary

This is a remote retained-binary restaging proof only. It does not establish public readiness, production readiness, public-internet readiness, external review completion, crypto completion, identity completion, trust completion, replay proof, downgrade proof, side-channel freedom, secret-material completion, vulnerability freedom, bug freedom, or perfect crypto.

## Backup-impact statement

Backup impact: none. NA-0526 did not execute qsl-backup, did not mutate backup status/plan files, and did not mutate `/backup/qsl`.

Read-only boundary proof showed the installed qsl-backup helper digest matched the expected value and the Codex ops source-list marker appeared exactly once.

## Rejected alternatives

- Run wrong-peer/stale-trust remote E2EE immediately after D424: rejected because the retained binary was stale.
- Treat D424 local hash as mandatory for NA-0526: rejected because current source commit may include docs-only closeout changes; the safe gate is no qsc runtime/dependency drift plus fresh local provenance.
- Use rsync or sftp: rejected because the directive required exactly one bounded transfer command, preferably scp.
- Clean the final retained qsc instead of retaining it: rejected because successful retained restaging is the intended predecessor for NA-0527.

## Next recommendation

Merge NA-0526 after required checks pass. If post-merge public-safety is attached and green inside the short attach/early-failure window, close out NA-0526 and restore the selected NA-0527 retry lane. NA-0527 must recheck the retained binary hash before remote E2EE.
