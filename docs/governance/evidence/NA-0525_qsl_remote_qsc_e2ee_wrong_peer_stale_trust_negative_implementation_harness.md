Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-23

# NA-0525 QSL Remote qsc E2EE Wrong-Peer / Stale-Trust Negative Implementation Harness

## Executive summary

NA-0525 stopped at the retained-qsc freshness gate with classification `REMOTE_E2EE_RETAINED_QSC_STALE_AFTER_SECURITY_REMEDIATION`.

D422 updated both affected lockfiles to `quinn-proto 0.11.15` after the retained remote qsc binary had originally been staged. A current clean local qsc build from `19c5bba14eef` produced SHA-256 `8292f3473909e499017912c3f814509b2a2b67d0fd8809375dad894f9d5858a2`. The retained remote binary at `/home/qslcodex/qsl-remote-test/bin/qsc` remained owned by `qslcodex` and still hashed to the old retained value `6f12ab5eec2468a4146012dfc247cef15be2337cb8e8e99cefa96a8c258d91ea`.

Because the retained binary is stale after dependency-security remediation, Codex did not run remote qsc E2EE, did not run qsc send/receive, did not create a remote E2EE root, and did not execute wrong-peer or stale-trust negatives. The selected successor is `NA-0526 -- QSL Remote qsc Prebuilt Binary Restaging After quinn-proto Remediation Implementation Harness`.

## Live NA-0525 scope

Startup proof and live repo checks showed:

- `READY_COUNT 1`.
- Sole READY item: `NA-0525 -- QSL Remote qsc E2EE Wrong-Peer / Stale-Trust Negative Implementation Harness`.
- `NA-0524`, `NA-0523`, and `NA-0522` were DONE.
- D-1037, D-1038, and D-1039 each existed once.
- D-1040 and D-1041 were absent before this patch.
- Duplicate decision count was zero.

Allowed checked-in mutation for this implementation evidence is limited to this evidence file, the NA-0525 testplan, `DECISIONS.md`, `TRACEABILITY.md`, and `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

The qwork proof files were present and copied into proof root:

- `/srv/qbuild/work/NA-0525/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0525/.qwork/startup.qsl-protocol.json`

Required qwork proof fields passed:

- `startup_result=OK`
- `lane=NA-0525`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0525/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0525`
- `requested_lane_status=READY`

The qwork proof HEAD and origin/main matched live pre-fetch refs at `19c5bba14eef`. Fetch was performed only after this match and disk usage remained below the 95% stop threshold.

## D423 / D422 / D419 inheritance

NA-0525 consumed the required inheritance:

- D423/NA-0524 closeout completed and restored NA-0525 READY.
- D423 verified D422 remediation and green current-main `public-safety` / `advisories`.
- D422 remediated both root `Cargo.lock` and nested `qsl/qsl-client/qsc/fuzz/Cargo.lock` from `quinn-proto 0.11.14` to `quinn-proto 0.11.15`.
- D422 made no Cargo.toml or qsc source mutation.
- D419/NA-0523 classified `REMOTE_E2EE_REPLAY_CORRUPT_NEGATIVES_PASS`.
- D419 proved replay and corrupt-delivery negatives failed closed, selected no-mutation checks passed, valid path remained usable, cleanup passed, retained remote qsc was preserved, and qsl-server/qsl-attachments were not used.
- NA-0525 was selected for wrong-peer/stale-trust negative implementation.
- The retained remote qsc binary was preserved before D422.
- D422 lockfile remediation means retained-qsc freshness must be revalidated before any new remote E2EE run.

## Retained-qsc freshness gate

The freshness gate compared a current clean local qsc build with the retained remote qsc binary before any remote E2EE.

Local current qsc:

- Source commit: `19c5bba14eef`
- Build command: `cargo build -p qsc --locked --bin qsc`
- Target dir: proof-root-local target directory
- Local binary path: proof-root-local `local_runtime/target/debug/qsc`
- SHA-256: `8292f3473909e499017912c3f814509b2a2b67d0fd8809375dad894f9d5858a2`
- Size: `102103920`
- Help smoke: passed

Retained remote qsc:

- Path: `/home/qslcodex/qsl-remote-test/bin/qsc`
- Owner: `qslcodex`
- UID/GID: `1003/1003`
- Mode: `700`
- Size: `102103920`
- SHA-256: `6f12ab5eec2468a4146012dfc247cef15be2337cb8e8e99cefa96a8c258d91ea`
- Help smoke: passed

The remote retained qsc hash equaled the old retained hash and differed from the current clean local qsc hash. The freshness result is therefore `REMOTE_E2EE_RETAINED_QSC_STALE_AFTER_SECURITY_REMEDIATION`.

## Command surface inspection and manifest

The fresh-path command manifest was not executed because the freshness gate failed.

Read-only command-surface inspection still confirmed the existing qsc surfaces that a future retry may use after restaging:

- vault init/unlock using passphrase-file handling;
- identity rotate/show;
- contacts add and device trust/list;
- relay inbox-set and relay serve;
- handshake init/poll/status;
- relay send/receive;
- existing no-secret-output and no-mutation test patterns.

No qsc send/receive, relay serve, handshake, wrong-peer negative, stale-trust negative, forwarding setup, or remote E2EE command was run in NA-0525 after the stale result.

## Local / remote boundary rechecks

Only the retained-qsc read/smoke boundary was run remotely because that action is explicitly allowed even if the freshness gate fails.

The remote retained-qsc recheck proved:

- remote user `qslcodex`;
- UID `1003`;
- retained qsc path exists;
- owner `qslcodex`;
- retained qsc hash equals old known hash;
- retained qsc help smoke works.

The broader fresh-path remote boundary recheck was not run because running retained stale qsc for E2EE is not acceptable after dependency-security remediation.

## Local qsc provenance

The local qsc binary was built from the current clean checkout:

- qsl-protocol HEAD: `19c5bba14eef`
- Rust: `rustc 1.95.0`
- Cargo: `cargo 1.95.0`
- qsc manifest: `qsl/qsl-client/qsc/Cargo.toml`
- qsc package version: `0.1.0`
- Cargo build exit code: `0`

The local qsc help smoke printed the expected qsc command surface without secret material.

## Forwarding / relay setup

Not run. The stale retained-qsc result occurred before the fresh-path forwarding packet.

## Isolated runtime roots and synthetic data

No remote E2EE runtime root was created. A read-only remote absence check confirmed the proof-specific remote E2EE root did not exist.

No synthetic E2EE message was generated. No production data and no personal data were used.

The proof-root `sensitive_runtime` directory was empty and removed after the stale-path classification. The local qsc build/hash proof remains under the proof root outside `sensitive_runtime`.

## Baseline E2EE setup to identity/trust negative test point

Not run. The retained-qsc freshness gate failed before baseline E2EE setup.

## Wrong-peer negative boundary proof or deferral

Wrong-peer negative testing was not run.

Deferral reason: the retained remote qsc binary is stale after D422 dependency-security remediation, so running remote qsc E2EE would use a pre-remediation binary and violate the freshness gate.

## Stale-trust negative boundary proof or deferral

Stale-trust/replaced-peer negative testing was not run.

Deferral reason: the retained remote qsc binary is stale after D422 dependency-security remediation, so running remote qsc E2EE would use a pre-remediation binary and violate the freshness gate.

## No-mutation checks

No E2EE negative state mutation was possible because no E2EE runtime was created and no qsc send/receive or handshake command was run.

Checked stale-path boundaries:

- no remote E2EE root existed for this proof ID;
- local `sensitive_runtime` was empty and removed;
- retained remote qsc was read/executed only for help/hash proof and was not modified;
- no qsl-server path was used;
- no qsl-attachments path was used;
- no qsc source/test/fuzz/Cargo path was mutated;
- no workflow/script/helper path was mutated;
- no dependency or lockfile path was mutated.

## Valid-path usability proof or deferral

Valid-path E2EE usability was not run because the stale retained-qsc gate prevents any new remote qsc E2EE run.

The valid-path proof is deferred to a later lane after the remote qsc binary is restaged from current post-D422 source and rechecked.

## No-secret-output review

Checked-in evidence includes only:

- command classifications and paths;
- qwork proof status;
- qsc binary hashes and sizes;
- non-secret qsc help output summaries;
- queue, decision, and validation state;
- no private key, passphrase, token, password, production data, personal data, or raw qsc private material.

The proof root contains raw command logs. The checked-in evidence does not include private key blocks, passphrases, tokens, passwords, or qsc private material.

## Cleanup / retention proof

Stale-path cleanup/retention result:

- Remote E2EE root: not created; read-only absence check passed.
- Local `sensitive_runtime`: empty and removed.
- Local qsc build/hash proof: retained under proof root for audit evidence.
- Retained remote qsc: unchanged.
- Local relay/listener: not started.
- SSH forwarding process: not started.

## Result classification

`REMOTE_E2EE_RETAINED_QSC_STALE_AFTER_SECURITY_REMEDIATION`

## Hostile Cryptographer Review

Running a pre-remediation retained binary after a dependency-security remediation would make the new negative evidence ambiguous. A wrong-peer or stale-trust reject from the stale binary would not prove the current post-D422 qsc executable behaves the same way, and a failure could mix implementation behavior with stale dependency provenance.

This lane therefore preserves fail-closed evidence discipline: stale retained binary means no new remote E2EE. It proves only that the retained qsc boundary is stale and that restaging is required before identity/trust negative testing can truthfully continue.

Remaining unproven items include authentication residuals, replay residuals beyond earlier bounded evidence, downgrade residuals beyond existing local/model gates, side-channel behavior, secret-material lifecycle residuals, and identity/trust residuals.

## Red-Team Review

The most likely misuse would be treating an old retained binary as good enough because the command surface still runs. NA-0525 rejects that shortcut. The binary still has the old known hash, so any new remote E2EE result would be suspect after D422.

Synthetic identities and trust records were not generated in this stale path. No production or personal data was involved.

## Production SRE Review

It is not operationally acceptable to continue remote E2EE with the retained qsc binary. Restaging must happen first.

Logged and redacted material:

- qwork status and repo refs;
- current local qsc build hash/size/help smoke;
- retained remote qsc owner/hash/size/help smoke;
- stale classification and no-run boundary.

Cleanup proof required and satisfied for this path:

- no remote E2EE root exists;
- local `sensitive_runtime` was removed;
- no relay/listener/forwarding process was started.

Failures remain isolated from qwork, qsl-backup, production data, qsl-server, and qsl-attachments because none of those systems were run or mutated.

## Release-Claim Boundary Review

This lane makes no public-readiness claim. It makes no production-readiness claim. It makes no public-internet-readiness claim. It makes no external-review-complete claim. It makes no crypto-complete claim. It makes no identity-complete claim. It makes no trust-complete claim. It makes no replay-proof claim. It makes no downgrade-proof claim. It makes no secret-material-complete claim. It makes no side-channel-free claim. It makes no vulnerability-free claim. It makes no bug-free claim. It makes no perfect-crypto claim.

## Successor selection

Selected successor:

`NA-0526 -- QSL Remote qsc Prebuilt Binary Restaging After quinn-proto Remediation Implementation Harness`

## Future scope bundle

Future NA-0526 should:

- build/select qsc from current post-D422 qsl-protocol source;
- record local qsc hash, size, and provenance;
- stage the binary to `$HOME/qsl-remote-test/bin/qsc` using the existing bounded staging pattern;
- verify remote owner/hash/provenance;
- run qsc help or version smoke only;
- record cleanup/retention proof;
- avoid qsc E2EE, qsc send/receive, wrong-peer/stale-trust testing, qsl-server, qsl-attachments, package installation, remote source checkout/build, and public/production readiness claims.

## Future validation / marker plan

Future NA-0526 markers should include:

- `NA0526_CURRENT_QSC_BUILD_PROVENANCE_OK`
- `NA0526_REMOTE_QSC_RESTAGED_HASH_MATCH_OK`
- `NA0526_REMOTE_QSC_OWNER_OK`
- `NA0526_REMOTE_QSC_HELP_SMOKE_OK`
- `NA0526_NO_REMOTE_E2EE_OK`
- `NA0526_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0526_NO_PUBLIC_PRODUCTION_CLAIM_OK`
- `NA0526_ONE_READY_INVARIANT_OK`

## No qsl-server / no qsl-attachments boundary

No qsl-server command, endpoint, service, file, or artifact was used or mutated.

No qsl-attachments command, endpoint, service, file, or artifact was used or mutated.

## No public/production readiness boundary

This is a stale-binary freshness gate result only. It preserves the no-claim boundary for public-readiness, production-readiness, public-internet-readiness, crypto-complete, identity-complete, trust-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, and perfect-crypto status.

## Backup-impact statement

No qsl-backup command was run. No backup or restore was run. `/usr/local/sbin/qsl-backup` was inspected read-only for hash/source-list boundary proof only. `/backup/qsl` was not mutated.

## Rejected alternatives

- Run remote E2EE anyway with the stale retained qsc: rejected because it would use a pre-remediation binary after D422.
- Restage qsc inside NA-0525: rejected because restaging is explicitly forbidden in this directive.
- Treat help smoke as freshness proof: rejected because help smoke proves command execution only, not post-remediation provenance.
- Select public/production readiness: rejected because the lane stopped before remote E2EE negative testing.

## Next recommendation

Merge the stale-gate evidence, then close out to the selected NA-0526 restaging implementation lane only after required checks are green.
