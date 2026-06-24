Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-24

# NA-0535 Remote qsc E2EE Wrong-Peer / Stale-Trust Retry After Port Diagnostic Implementation Harness

## Executive summary

NA-0535 consumed the NA-0534 port 39176 diagnostic success and retried the bounded remote qsc E2EE identity/trust negative lane. The result classification is `REMOTE_E2EE_WRONG_PEER_STALE_TRUST_NEGATIVES_PASS`.

The lane rechecked qwork proof files without rerunning qwork, verified NA-0535 as the sole READY item, rechecked the retained remote qsc binary against the NA-0526/D425 hash, built a current local qsc from clean `main`, wrote a command manifest before integrated forwarding or E2EE, proved marker traversal and ACK through the dedicated-key reverse-forward path, then created isolated local and remote qsc roots. It established a synthetic Build-to-Inspiron qsc E2EE baseline, executed one wrong-peer negative, executed one stale/replaced-peer negative, proved no unexpected selected-state mutation for both negatives, proved the valid path remained usable, scanned qsc/SSH outputs for secret material, and cleaned up local and remote runtime artifacts.

## Live NA-0535 scope

Allowed repository mutations were limited to this evidence file, `tests/NA-0535_qsl_remote_qsc_e2ee_wrong_peer_stale_trust_retry_after_port_diagnostic_implementation_testplan.md`, `DECISIONS.md`, `TRACEABILITY.md`, and `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Runtime proof root: `/srv/qbuild/tmp/NA0535_wrong_peer_stale_trust_retry_after_port_diagnostic_impl_20260624T163514Z`.

Remote runtime root after precheck: `/home/qslcodex/qsl-remote-test/e2ee/NA0535_20260624T163514Z/`.

No qsc source, qsc test, qsc fuzz, Cargo, workflow, helper script, dependency, lockfile, corpus, vector, formal, refimpl, service, public, backup, qsl-server, or qsl-attachments repository path was mutated.

## qwork proof-file verification

Codex did not run qwork, qstart, or qresume. It read and copied the expected qwork proof files from `/srv/qbuild/work/NA-0535/.qwork/`.

The `.kv` and `.json` proofs both recorded `startup_result=OK`, lane `NA-0535`, repo `qsl-protocol`, path `/srv/qbuild/work/NA-0535/qsl-protocol`, clean worktree/index/untracked state, `ready_count=1`, `queue_top_ready=NA-0535`, and requested lane status READY. Proof HEAD and proof origin/main matched live pre-fetch refs at `4fbbfaf04ecd`. Fetch happened only after that match and disk usage was below the 95% stop threshold.

## D440 / D439 / D436 / D435 / D425 / D419 inheritance

D440 recorded NA-0534 DONE, NA-0535 READY, D-1059 once, D-1060 absent, no remote action in closeout, and public-safety/advisories green.

D439 recorded classification `REMOTE_FORWARD_PORT_39176_DIAGNOSTIC_MARKER_TRAVERSAL_PASS`; the remote loopback bind probe passed, the corrected integrated probe kept the SSH forward alive, remote `ss` showed `127.0.0.1:39176` listening during the forward, the remote trigger received ACK, listener `ok`, `marker_match`, and `ack_sent` were true, and cleanup passed. No qsc E2EE, qsc send/receive, qsl-server, or qsl-attachments occurred in NA-0534.

D436's prior NA-0532 forwarding failure was consumed and resolved by the D439 diagnostic pass. D435's stdin-script trigger shape and marker/ACK remediation passed without qsc E2EE. D425's retained remote qsc hash was `6bf9e59fdae397c2e0f88538d700cccbee80d229c6a979cc79555e39fea2b4f7`. D419's replay and corrupt delivery negatives passed with no-mutation and cleanup proof. No qsl-server or qsl-attachments inheritance was used as a bypass.

## Retained-qsc freshness recheck

Local qsc was built from clean source commit `4fbbfaf04ecd` using `cargo build -p qsc --locked --bin qsc` with `CARGO_TARGET_DIR` under the proof root. Local qsc path was proof-root-local, size `102103920`, and SHA-256 `38735046a90088ef99e7e6647e6e0ae9e14c80e430e38090083b8449d244a8ba`. Local `--help` succeeded.

The retained remote qsc at `/home/qslcodex/qsl-remote-test/bin/qsc` was rechecked read-only: owner `qslcodex`, mode `700`, size `102103920`, and SHA-256 `6bf9e59fdae397c2e0f88538d700cccbee80d229c6a979cc79555e39fea2b4f7`. Remote `--help` succeeded with local-only output capture and no remote redirection.

The current local hash differed from the retained D425 hash, but the qsc runtime/dependency diff from D425 source commit `2cff954de589` to current `main` was empty, so provenance was accepted without restaging. The retained remote binary was not mutated.

## Command surface inspection and manifest

Read-only inspection covered the required qsc tests and source command surfaces, including `same_host_client_to_client_e2e.rs`, `receive_e2e.rs`, `identity_binding.rs`, `identity_ux.rs`, `trust_onboarding_mainstream_flow_na0187.rs`, binding negative tests, `qsl/qsl-client/qsc/src/cmd/mod.rs`, `identity`, `contacts`, `handshake`, `transport`, `relay`, and protocol-state modules.

The exact command manifest was written before integrated forwarding and before qsc E2EE at:

- `$PROOF_DIR/command_manifest/na0535_command_manifest.md`
- `$PROOF_DIR/command_manifest/na0535_command_manifest.json`

The manifest used only existing qsc CLI/test-visible surfaces: vault init/unlock, identity rotate/show, contacts add/device list/device trust, relay inbox-set, local qsc relay serve, handshake init/poll, send, receive, wrong-peer receive reject, and replaced-peer identity mismatch reject. Synthetic passphrases and route tokens were handled through sensitive runtime files and were not written into checked-in evidence.

Recovered harness/verifier issues were recorded under the proof root: an overbroad manifest verifier, local precheck script-generation quoting mistakes, use of the operational identity instead of the D439 dedicated forwarding key in the first proof-root SSH config, and one E2EE flow script-generation quoting mistake. These were local harness issues before qsc E2EE, did not require scope expansion, and did not weaken fail-closed behavior.

## Local / remote boundary rechecks

Safe `ssh -G inspiron` parsing recorded only allowed fields. The operational alias still reported `clearallforwardings=yes`, so the reverse-forward used a proof-root SSH config with the D439 dedicated forwarding key, publickey-only authentication, no agent/X11 forwarding, no PTY, no ControlMaster, and `ClearAllForwardings no`.

Remote boundary checks passed: user `qslcodex`, nonzero UID, no privileged groups, negative `sudo -n true`, `/backup/qsl` absent or not readable, qwork absent, qsl-backup absent, and directive-specific remote E2EE root absent before creation. Local and remote port `39176` were free before the integrated precheck.

## Integrated forwarding marker/ACK precheck

The integrated precheck used a local listener on `127.0.0.1:39176`, the D439 dedicated-key proof-root SSH config, reverse forward `127.0.0.1:39176:127.0.0.1:39176`, and the D435/D439 stdin-script trigger shape:

`ssh -T inspiron 'python3 -' < $PROOF_DIR/integrated_forwarding/remote_trigger_na0535.py`

Local trigger syntax check and marker rehearsal passed. The corrected integrated attempt kept the SSH forward alive, remote `ss` showed `127.0.0.1:39176` listening during the forward, the local listener received exact marker `QSL_INTEGRATED_FORWARDING_PRECHECK_NA0535_NA0535_20260624T163514Z`, the listener sent `NA0535_TUNNEL_ACK_OK`, and the remote trigger received that ACK. No qsc E2EE command ran before this precheck passed.

## Local qsc provenance

Local qsc was built under the proof root from current clean checkout `4fbbfaf04ecd`. The current local binary was used for Build-side qsc commands. The retained remote D425 binary was used for Inspiron-side qsc commands. No remote source checkout/build, remote cargo, remote rustup, remote git, package install, or retained-binary mutation occurred.

## Forwarding / relay setup

After the marker/ACK precheck passed, qsc relay was started locally with `qsc relay serve --port 39176 --seed 535 --max-messages 40`. The implementation surface binds qsc relay to `127.0.0.1:<port>`. The dedicated-key SSH reverse forward exposed only remote loopback `127.0.0.1:39176` to the local loopback relay. No qsl-server, qsl-attachments, public service deployment, or public internet exposure was used.

## Isolated runtime roots and synthetic data

Local Build qsc runtime root was under `$PROOF_DIR/sensitive_runtime/local_build_qsc_root`. Remote Inspiron qsc runtime root was under `/home/qslcodex/qsl-remote-test/e2ee/NA0535_20260624T163514Z/remote_qsc_root`.

Synthetic labels:

- baseline: `QSL_REMOTE_E2EE_IDTRUST_BASELINE_NA0535_20260624T163514Z`
- wrong-peer: `QSL_REMOTE_E2EE_WRONG_PEER_NEGATIVE_NA0535_20260624T163514Z`
- stale-trust: `QSL_REMOTE_E2EE_STALE_TRUST_NEGATIVE_NA0535_20260624T163514Z`

No production data, personal data, private keys, passphrases, passwords, route-token values, or secret message text is included in checked-in evidence.

## Baseline E2EE setup to identity/trust negative test point

The baseline initialized isolated Alice, Alice2, and Bob qsc roots, rotated/showed synthetic identities, exchanged public fingerprints through existing qsc contact surfaces, trusted peer devices through `contacts device trust`, configured synthetic relay inbox route tokens through sensitive files, established Alice/Bob handshake state using relay-backed `handshake init` and `handshake poll`, then sent and received a baseline Alice-to-Bob synthetic message through the loopback qsc relay. Baseline receive emitted `recv_commit`.

## Wrong-peer negative boundary proof or deferral

Wrong-peer negative executed and passed. After a valid Alice/Bob session existed, Alice sent a synthetic message to Bob. Bob attempted to receive from the Bob mailbox with `--from charlie`. The command failed closed with an expected qsc fail-closed marker class (`protocol_inactive` or `qsp_hdr_auth_failed`), did not create a new receive artifact, did not mutate Bob's selected Alice session digest, and did not output plaintext or sensitive values.

## Stale-trust negative boundary proof or deferral

Stale/replaced-peer negative executed and passed. After Bob had pinned original Alice and had a valid Alice session, a separate local Alice2 root rotated a new identity under label `alice` and initiated a handshake to Bob. Bob's poll for peer `alice` surfaced `identity_mismatch` / `peer_mismatch`, left the selected Bob Alice-session digest unchanged, and did not output sensitive values. This is a bounded replaced-peer/stale-trust fail-closed proof, not a full identity/trust completeness claim.

## No-mutation checks

For wrong-peer, Bob's selected Alice session digest before and after the negative matched, and Bob output-file count was unchanged.

For stale/replaced-peer, Bob's selected Alice session digest before and after the negative matched.

State summaries used hashes/counts/metadata only. Raw qsc private state was not copied into checked-in evidence.

## Valid-path usability proof or deferral

After both negatives, original Alice sent another synthetic message to Bob and Bob received it successfully with `recv_commit`. This proved the valid path remained usable after the executed negative boundaries.

## No-secret-output review

The flow verifier scanned qsc/SSH stdout and stderr for the generated synthetic passphrase values, generated route-token values, private key block markers, API/bearer marker fixtures, and qsc private/passphrase marker strings. The scan passed. Checked-in evidence does not include private keys, passphrases, tokens, passwords, raw qsc private material, or plaintext receive files.

## Cleanup / retention proof

Cleanup removed `/home/qslcodex/qsl-remote-test/e2ee/NA0535_20260624T163514Z/`, removed local `$PROOF_DIR/sensitive_runtime`, stopped qsc relay/listener/SSH forward processes, and verified local and remote port `39176` were closed. The retained remote qsc at `/home/qslcodex/qsl-remote-test/bin/qsc` remained unchanged.

## Result classification

`REMOTE_E2EE_WRONG_PEER_STALE_TRUST_NEGATIVES_PASS`

## Hostile Cryptographer Review

The integrated marker/ACK precheck proves only a transport precondition: remote loopback can traverse the dedicated reverse-forward tunnel and return an ACK. It does not prove qsc protocol correctness.

The wrong-peer and stale/replaced-peer negatives prove bounded fail-closed behavior for the exercised identity/trust surfaces only. They do not prove identity-complete or trust-complete status, and they do not close authentication, replay, downgrade, side-channel, or secret-material lifecycle gaps. Synthetic identities and trust records avoided production and personal data exposure.

## Red-Team Review

If port 39176 regression recurs, qsc E2EE must remain blocked until a fresh marker/ACK precheck passes. If wrong-peer or stale-trust cannot be staged, the lane must record exact surface limits rather than claim broader assurance. If stale-trust mutation occurs, the lane must stop as a no-mutation failure. Route/capability metadata must stay out of checked-in evidence and final responses. Cleanup proof must remain explicit because stale remote roots could contaminate repeated runs.

Next hardening should repeat the run for freshness, exercise cleanup robustness, and keep pre-run retained-qsc verification mandatory.

## Production SRE Review

This lane is operationally useful because it ties qsc remote negative testing to a freshly proven loopback forwarding precondition, isolated roots, bounded command lifetimes, and cleanup proof. Logs stayed under the proof root; checked-in evidence records summaries and avoids route-token/passphrase/private-state contents. Failures stayed isolated from qwork, qsl-backup, backup paths, and production data. qsl-server and qsl-attachments remain deferred boundaries.

This does not imply public, production, or public-internet readiness.

## Release-Claim Boundary Review

No public-ready claim is made. No production-ready claim is made. No public-internet-ready claim is made. No external-review-complete claim is made. No crypto-complete claim is made. No identity-complete claim is made. No trust-complete claim is made. No replay-proof claim is made. No downgrade-proof claim is made. No secret-material-complete claim is made. No side-channel-free claim is made. No vulnerability-free, bug-free, or perfect-crypto claim is made.

## Successor selection

Selected successor after merge and green post-merge public-safety: `NA-0536 -- QSL Remote qsc E2EE Repeated-Run / Cleanup / Freshness Scope Authorization Plan`.

The implementation PR does not edit `NEXT_ACTIONS.md`; closeout must restore the selected successor only after merge and required protection checks.

## Future scope bundle

Future NA-0536 should authorize repeated-run freshness, cleanup robustness, retained-qsc freshness before each run, forwarding cleanup, synthetic proof redaction, no qsl-server/qsl-attachments, and no public/production readiness claims.

## Future validation / marker plan

Every future remote qsc E2EE lane should first recheck retained qsc metadata/hash and prove integrated marker traversal and ACK on port `39176` in one controlled lifetime before any qsc E2EE command.

## No qsl-server / no qsl-attachments boundary

No qsl-server command, process, endpoint, or repository path was used. No qsl-attachments command, process, endpoint, or repository path was used. qsc relay was local loopback only and was stopped during cleanup.

## No public/production readiness boundary

This is internal remote qsc hardening evidence only. It is not public readiness, production readiness, public-internet readiness, external-review completion, crypto completion, identity completion, trust completion, replay proof, downgrade proof, secret-material completion, side-channel-free proof, vulnerability-free proof, bug-free proof, or perfect-crypto proof.

## Backup-impact statement

No backup or restore was run. qsl-backup was checked read-only by digest and source-inclusion count. `/backup/qsl` was not mutated, and the remote account had no readable backup exposure.

## Rejected alternatives

- Run qsc E2EE before marker/ACK precheck: rejected by directive gate.
- Restage remote qsc: rejected because the retained remote qsc matched the D425 hash and no qsc runtime/dependency paths changed since D425.
- Use qsl-server or qsl-attachments: rejected as out of scope.
- Use a public service or public relay: rejected because loopback-only proof was required.
- Claim broad protocol completeness: rejected because the evidence is bounded to the executed identity/trust negative surfaces.

## Next recommendation

Merge the NA-0535 implementation PR only after required checks pass, then close out to NA-0536 only if post-merge public-safety is green inside the short attach/early-failure window.
