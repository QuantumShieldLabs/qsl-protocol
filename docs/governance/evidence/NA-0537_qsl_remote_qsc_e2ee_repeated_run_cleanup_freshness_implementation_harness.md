Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-25

# NA-0537 Remote qsc E2EE Repeated-Run / Cleanup / Freshness Implementation Harness

## Executive summary

NA-0537 recovered the D445 baseline receive command-shape failure, proved the correct Bob receive peer label from existing qsc CLI/test/source surfaces, then executed two isolated synthetic remote qsc E2EE runs with retained-qsc freshness checks, generated-script compile gates, local listener self-test, integrated port 39176 marker/ACK gates, selected identity/trust negatives, cleanup, and no-secret-output review.

Result classification: `REMOTE_E2EE_REPEATED_RUN_CLEANUP_FRESHNESS_PASS`.

Selected successor after merge and green closeout protection checks: `NA-0538 -- QSL Website / Repository Public Evidence Sync Scope Authorization Plan`.

This lane uses direct qsc command surfaces only. It does not use qsl-server, qsl-attachments, public service deployment, package installation, remote source checkout/build, qwork/qstart/qresume, qsl-backup, production data, personal data, or qsc source/test/fuzz/Cargo mutation.

## Live NA-0537 scope

Allowed repository mutations are limited to this evidence file, `tests/NA-0537_qsl_remote_qsc_e2ee_repeated_run_cleanup_freshness_implementation_testplan.md`, `DECISIONS.md`, `TRACEABILITY.md`, and `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Proof root: `/srv/qbuild/tmp/NA0537_peer_label_recover_retry_repeated_run_cleanup_freshness_impl_20260625T021901Z`.

Remote runtime roots, created only after the relevant run's retained-qsc, generated-script, local self-test, boundary, and marker/ACK gates passed:

- `/home/qslcodex/qsl-remote-test/e2ee/NA0537_peer_label_recover_retry_repeated_run_cleanup_freshness_impl_20260625T021901Z/run-1`
- `/home/qslcodex/qsl-remote-test/e2ee/NA0537_peer_label_recover_retry_repeated_run_cleanup_freshness_impl_20260625T021901Z/run-2`

Local sensitive runtime roots were proof-root-local and were deleted after each run.

## qwork proof-file verification

Codex did not run qwork, qstart, or qresume. It read the proof files produced before execution:

- `/srv/qbuild/work/NA-0537/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0537/.qwork/startup.qsl-protocol.json`

The `.kv` proof recorded `startup_result=OK`, lane `NA-0537`, repo `qsl-protocol`, path `/srv/qbuild/work/NA-0537/qsl-protocol`, clean worktree/index/untracked state, `ready_count=1`, queue top READY `NA-0537`, and requested lane status READY. The `.json` proof mirrored those values. Proof HEAD and proof origin/main matched live HEAD and live origin/main before fetch at `836e62ca106c`.

The qwork proof written timestamp was `2026-06-25T02:17:51Z`, after the D445 response timestamp. Fetch happened only after proof/live ref match and disk usage was below the 95% stop threshold.

Startup queue and decision state:

- READY_COUNT 1.
- READY `NA-0537 -- QSL Remote qsc E2EE Repeated-Run / Cleanup / Freshness Implementation Harness`.
- NA-0536 DONE.
- NA-0535 DONE.
- D-1062 exists once.
- D-1063 exists once.
- D-1064 absent before this patch.
- D-1065 absent before this patch.
- Duplicate decision count 0.

Startup main health: `public-safety` and `advisories` were completed success on `836e62ca106c`, with no required red checks retrieved.

## D445 / D444 / D443 / D442 / D441 / D440 / D439 / D435 / D419 inheritance

D445 stopped with `REMOTE_E2EE_REPEATED_RUN1_BASELINE_FAILURE`. It had passed retained-qsc freshness, generated script compile, local listener self-test, run-1 marker/ACK precheck, setup, handshake, and Alice send commit. Bob baseline receive failed closed with `qsp_hdr_auth_failed` because the harness invoked Bob receive with `--from bob`; D445 suspected the correct peer/session label for Bob receiving Alice's message was Alice.

D445 cleanup passed, retained remote qsc was unchanged, the repo remained clean, and no qsl-server or qsl-attachments was used.

D444 stopped before qsc E2EE with `REMOTE_E2EE_REPEATED_AMBIGUOUS_STOP` after local qsc build succeeded but retained-qsc preflight command shape exceeded that directive's correction budget. D443 stopped before qsc E2EE with `REMOTE_E2EE_REPEATED_RUN_MARKER_ACK_FAILURE` because the generated listener script had a newline-escaping syntax bug; a response-writing safety violation happened after that stop. NA-0537 consumes both harness failures and the D443 response-writer violation through generated-script compile gates and the file-based response-writer policy.

D442 authorized NA-0537. D441 recorded `REMOTE_E2EE_WRONG_PEER_STALE_TRUST_NEGATIVES_PASS`, retained-qsc freshness, integrated port 39176 marker/ACK, baseline remote qsc E2EE setup, wrong-peer negative, stale/replaced-peer negative, no-mutation checks, valid path after negatives, cleanup, and no qsl-server/qsl-attachments. D440 restored NA-0535. D439 proved port 39176 marker/ACK diagnostics. D435 proved the stdin-script trigger remediation. D419 recorded replay/corrupt negatives passed.

## D443 / D444 / D445 residue and safety recovery

Before retry, Codex verified no D443/D444/D445 proof-root listener, trigger, forward, response-writer, qwork, qstart, qresume, or cargo process remained. The repo had no tracked diff and no untracked files.

Prior remote E2EE roots were absent:

- `/home/qslcodex/qsl-remote-test/e2ee/NA0537_20260624T195222Z`
- `/home/qslcodex/qsl-remote-test/e2ee/NA0537_20260624T202909Z`
- `/home/qslcodex/qsl-remote-test/e2ee/NA0537_20260624T205833Z_D445`

Prior local sensitive roots were absent or proof-root-local only. Local and remote port 39176 were free before retry.

## Enhanced self-recovery evidence

Recovered issues were logged under the proof root with failing command/check, classification, reason, corrective action, and final result:

- Startup proof parser used invalid local Python command shape; corrected to a safe parser and passed.
- Local port probe used invalid local Python command shape; corrected and passed.
- `ssh -G` safe-field parser used invalid local Python command shape; replaced by a compiled proof-root parser and passed.
- Run-1 marker precheck first used the operational SSH identity for reverse forwarding and failed before qsc E2EE or remote root creation; cleanup proved no stale root, then the harness was corrected to use the D439/D435 dedicated forwarding config for `ssh -N -T -R` and the operational config for remote commands. The rerun passed.
- No-secret review initially matched its own private-key block pattern constants; the scanner was corrected to skip its own source file and the rescan passed.

No recovery ran qwork, qstart, qresume, qsl-backup, qsl-server, qsl-attachments, remote source checkout/build, package installation, qsc E2EE before gates, or out-of-scope mutation.

## Response-writer safety gate

`$PROOF_DIR/response_safety/response_writer_policy.md` records:

- no unquoted heredoc for this directive.
- final response writing must use `$PROOF_DIR/response_safety/write_response.py`.
- final response writer must use `Path.write_text(...)`.
- final response writer must not shell-evaluate Markdown.
- final response writer must not execute qwork, qstart, qresume, cargo, or command substitutions.
- `RESPONSE_WRITER_FILE_BASED yes`.

## Local qsc build / provenance

Local qsc was built from clean checkout `836e62ca106c` with `cargo build -p qsc --locked --bin qsc`, using proof-root `CARGO_TARGET_DIR`. The local binary was proof-root-local, size `102103920`, and `qsc --help` succeeded.

Root `Cargo.lock` and nested `qsl/qsl-client/qsc/fuzz/Cargo.lock` both retained `quinn-proto 0.11.15`. The qsc runtime/dependency diff since D441 source commit `4fbbfaf04ecd` was empty for qsc source and Cargo lock surfaces, so retained-qsc restaging was not required.

## Peer-label correction proof

The peer-label proof is recorded under `$PROOF_DIR/peer_label_proof/`.

Existing surfaces prove Bob's receive context for Alice's message uses Alice's peer/session label:

- `same_host_client_to_client_e2e.rs` establishes sessions at `session_path(alice, "bob")` and `session_path(bob, "alice")`.
- The same test sends Alice to Bob with `--to bob` and receives on Bob with `--from alice`.
- The reply direction sends Bob to Alice and receives on Alice with `--from bob`.
- `src/cmd/mod.rs` defines receive `--from` as the protocol peer label/session key used for decrypt context.
- `transport` receive code checks active protocol state for the `--from` peer and unpacks using that peer context.
- Protocol state loads session material by peer/channel label.

Therefore Bob baseline receive in NA-0537 used `--from alice`. The wrong-peer repeat used a distinct synthetic wrong peer label, not Bob.

## Retained-qsc preflight hardening

The retained-qsc checker was generated by `$PROOF_DIR/retained_qsc_recheck/write_remote_retained_qsc_check.py`, which uses `Path.write_text(raw_script)`. The generated Python checker compiled locally and was streamed to the remote host through stdin with no remote redirection, no remote temp path, no `tee`, no `mktemp`, and no retained binary mutation.

The retained remote qsc remained `/home/qslcodex/qsl-remote-test/bin/qsc`, owner/group `qslcodex/qslcodex`, mode `700`, size `102103920`, SHA-256 `6bf9e59fdae397c2e0f88538d700cccbee80d229c6a979cc79555e39fea2b4f7`, with remote `--help` returning success under local-only output capture.

## Command surface inspection and manifest

Read-only inspection covered the required qsc tests and command modules, including same-host client-to-client E2E, receive E2E, common helpers, mock relay transport, identity binding/UX, trust onboarding, KEM/signature transcript binding negatives, binding vector consumers, and qsc command/source modules for identity, contacts, trust, handshake, transport, relay, send, receive, and protocol state.

The exact command manifest was written before remote E2EE:

- `$PROOF_DIR/command_manifest/na0537_command_manifest.md`
- `$PROOF_DIR/command_manifest/na0537_command_manifest.json`

The manifest records retained-qsc recheck commands for each run, generated script paths and compile commands, local listener self-test, integrated marker/ACK commands, Bob baseline receive `--from alice`, transfer commands, runtime roots, synthetic labels, passphrase-file handling, public/trust record paths, negative repeat plans, no-stale checks, cleanup commands, and the no qsl-server / no qsl-attachments boundary.

## Generated script safety gate

Generated proof-root scripts:

- `na0537_marker_listener.py`
- `remote_trigger_na0537.py`
- `remote_e2ee_harness_na0537.py`
- `local_run_harness_na0537.py`
- `remote_boundary_check_na0537.py`

All generated Python scripts passed `python3 -m py_compile` before use. The listener JSON writer used safe newline escaping and avoided the D443 raw-newline syntax bug.

Local-only listener/trigger self-test passed before any SSH marker precheck: marker matched, ACK was sent, ACK was received, and listener result JSON reported `ok=true`.

## Run structure

Run 1 used fresh local sensitive runtime `$PROOF_DIR/sensitive_runtime/run-1` and remote root `$HOME/qsl-remote-test/e2ee/<PROOF_ID>/run-1`. It rechecked retained qsc, passed marker/ACK, executed a valid Alice-to-Bob qsc E2EE path with Bob receive `--from alice`, executed the wrong-peer negative, then cleaned up.

Run 2 first proved run-1 roots were absent, then used fresh local sensitive runtime `$PROOF_DIR/sensitive_runtime/run-2` and remote root `$HOME/qsl-remote-test/e2ee/<PROOF_ID>/run-2`. It rechecked retained qsc, passed marker/ACK, executed a valid Alice-to-Bob qsc E2EE path with Bob receive `--from alice`, executed the stale/replaced-peer negative, then cleaned up.

## Retained-qsc freshness recheck per run

Run 1 retained-qsc recheck passed before qsc E2EE. Run 2 retained-qsc recheck passed before qsc E2EE. A final retained-qsc recheck after cleanup also passed with the same path, owner, mode, size, hash, and help success.

## Local / remote boundary rechecks per run

Safe `ssh -G` parsing recorded only allowed fields. The operational config used publickey-only, no agent/X11, no PTY, and no password authentication. Reverse-forwarding used the proof-root dedicated forwarding config with `ClearAllForwardings no`.

Before each run, remote boundary checks passed: user `qslcodex`, UID nonzero, no privileged groups, negative sudo probe, `/backup/qsl` absent or unreadable, qwork absent, qsl-backup absent, and the run root absent before creation. Local and remote port 39176 were closed before precheck.

## Integrated forwarding marker/ACK precheck per run

Run 1 and run 2 each used the compiled listener and trigger scripts, local trigger rehearsal, local listener on `127.0.0.1:39176`, dedicated-key reverse forward `127.0.0.1:39176:127.0.0.1:39176`, and stdin-script remote trigger. In both runs, the listener received the exact synthetic marker, sent the run-specific ACK, and the remote trigger received the ACK before any qsc E2EE command ran.

## Run 1 valid path and wrong-peer repeat proof or deferral

Run 1 valid path passed. Alice and Bob initialized isolated qsc roots, exchanged/trusted synthetic public identities, established relay-backed handshake state, Alice sent a synthetic message to Bob, and Bob received it with `--from alice`. Baseline receive returned success and matched the expected synthetic message digest. Raw plaintext/payload output remains proof-root-local and is not copied into checked-in evidence.

Run 1 wrong-peer repeat executed and passed fail-closed. Bob attempted to receive a valid Bob mailbox entry with `--from charlie`; qsc rejected before plaintext output, Bob's selected Alice session state remained unchanged, and no unexpected output artifact was created.

## Run 1 cleanup / no-stale check

Run 1 cleanup removed the remote run-1 root, removed local sensitive runtime for run 1, stopped listener/relay/SSH forward processes, closed local and remote port 39176, and left proof-root process count zero. The remote parent proof root was absent after run-1 cleanup.

## Run 2 valid path and stale-trust repeat proof or deferral

Run 2 valid path passed with fresh roots and unique synthetic labels. Bob baseline receive used `--from alice` and succeeded.

Run 2 stale/replaced-peer repeat executed and passed fail-closed. A second synthetic Alice identity attempted a handshake under peer label `alice`; Bob observed identity/peer mismatch and rejected it. Bob's selected original Alice session state remained unchanged, and no plaintext or secret material was copied into checked-in evidence.

## Run 2 cleanup / final cleanup

Run 2 cleanup removed the remote run-2 root, removed local sensitive runtime for run 2, stopped listener/relay/SSH forward processes, closed local and remote port 39176, and removed the remote parent proof root when empty. Final process scan found zero proof-root processes. Final retained-qsc recheck passed unchanged.

## No-stale-state proof

Run 2 started only after proving run-1 local sensitive runtime and remote run root were absent. Both runs used unique synthetic labels and isolated local/remote roots. Cleanup-state JSON and post-cleanup local/remote probes show no stale root, process, or port reuse across runs.

## No-secret-output review

The no-secret review scanned proof-root text outputs and produced `$PROOF_DIR/no_secret_review/no_secret_output_review.md`. After correcting the scanner self-hit, the review passed with zero private-key block files. Synthetic passphrase-file option names and route-token metadata remain in proof-root-local raw qsc command logs only. Checked-in evidence uses redacted booleans and does not include private keys, passphrases, tokens, passwords, raw qsc private state, or raw payload material.

## Cleanup / retention proof

Final cleanup proof:

- run-1 local sensitive runtime absent.
- run-2 local sensitive runtime absent.
- run-1 remote root absent.
- run-2 remote root absent.
- remote parent proof root absent.
- local port 39176 closed.
- remote port 39176 closed.
- proof-root process count zero.
- retained remote qsc unchanged.

## Result classification

`REMOTE_E2EE_REPEATED_RUN_CLEANUP_FRESHNESS_PASS`

## Hostile Cryptographer Review

Repeated-run cleanup/freshness proves bounded operational repeatability only. No crypto-complete claim is made. No identity-complete claim is made. No trust-complete claim is made. No replay-proof claim is made. No downgrade-proof claim is made. No side-channel-free claim is made. No secret-material-complete claim is made. No vulnerability-free claim is made. No bug-free claim is made. No perfect-crypto claim is made. Synthetic repeated runs reduce data exposure but do not substitute for external review or formal completeness.

## Red-Team Review

The primary risks were stale retained qsc, stale remote roots, stale local sensitive runtime, stale relay/SSH processes, port reuse, generated-script correctness, response-writer safety, peer-label command correctness, and accidental checked-in route/capability metadata. This lane gated qsc E2EE on retained-qsc freshness, script compile, local self-test, and marker/ACK; it stopped and recovered only pre-E2EE harness issues; and it kept raw qsc outputs proof-root-local.

## Production SRE Review

The repeated-run cleanup/freshness checkpoint is operationally useful after NA-0535 because it proves two bounded synthetic remote qsc E2EE cycles with per-run cleanup and freshness checks. Logs remain proof-root-local and redacted in governance evidence. qwork, qsl-backup, production data, qsl-server, and qsl-attachments remain isolated. This does not imply public or production readiness.

## Release-Claim Boundary Review

No public-ready claim is made. No production-ready claim is made. No public-internet-ready claim is made. No external-review-complete claim is made. No crypto-complete claim is made. No identity-complete claim is made. No trust-complete claim is made. No replay-proof claim is made. No downgrade-proof claim is made. No secret-material-complete claim is made. No side-channel-free claim is made. No vulnerability-free claim is made. No bug-free claim is made. No perfect-crypto claim is made.

## Successor selection

Because the classification is `REMOTE_E2EE_REPEATED_RUN_CLEANUP_FRESHNESS_PASS`, the selected successor is `NA-0538 -- QSL Website / Repository Public Evidence Sync Scope Authorization Plan`.

This implementation PR does not edit `NEXT_ACTIONS.md`. Queue closeout must happen in a separate closeout PR after this PR merges and post-merge public-safety is green inside the short attach/early-failure window.

## Future scope bundle

Future NA-0538 should authorize public-facing website and repository evidence-sync scope only. It should select exact public-doc/website/README paths for a later implementation lane, define safe public wording, define redaction/proof rules, and preserve all no public/production/security-completion claim boundaries.

## Future validation / marker plan

Future public evidence-sync work should not run remote qsc E2EE unless a later directive explicitly selects such work. Any later remote qsc lane must continue to recheck retained qsc, compile generated scripts, run local listener self-test, prove port 39176 marker/ACK, use unique synthetic roots, and prove cleanup before making new evidence claims.

## Public website / repo evidence sync recommendation

Public website/repo evidence sync is recommended as a separate authorization lane only. The lane should summarize achieved internal evidence, remaining limits, and review invitations without claiming public readiness, production readiness, public-internet readiness, external-review completion, crypto completeness, identity completeness, trust completeness, replay proof, downgrade proof, side-channel freedom, vulnerability freedom, bug freedom, or perfect crypto.

## No qsl-server / no qsl-attachments boundary

No qsl-server command, process, endpoint, file, or repository path was used. No qsl-attachments command, process, endpoint, file, or repository path was used. The only relay used was local qsc loopback relay through the dedicated reverse-forward tunnel, and it was stopped during cleanup.

## No public/production readiness boundary

This is internal direct-qsc evidence. No public-readiness claim is made. No production-readiness claim is made. No public-internet-readiness claim is made. No external-review-complete claim is made. No crypto-complete claim is made. No identity-complete claim is made. No trust-complete claim is made. No replay-proof claim is made. No downgrade-proof claim is made. No secret-material-complete claim is made. No side-channel-free claim is made. No vulnerability-free claim is made. No bug-free claim is made. No perfect-crypto claim is made.

## Backup-impact statement

qsl-backup was checked read-only by installed helper digest and source-inclusion count. qsl-backup was not executed. `/backup/qsl` was not mutated. Remote boundary checks showed no readable backup exposure for the remote account.

## Rejected alternatives

- Reuse the D445 failed root: rejected; retry used a fresh proof ID and fresh run roots.
- Retry Bob baseline receive with `--from bob`: rejected because current qsc source/tests prove Bob's receive peer label for Alice's message is Alice.
- Skip marker/ACK after run 1: rejected because each run must independently prove the tunnel gate.
- Defer cleanup until both runs finish: rejected because cleanup after each run is part of the freshness proof.
- Use qsl-server or qsl-attachments: rejected as out of scope.
- Treat repeated-run success as public/production/security completeness: rejected because the evidence is bounded internal synthetic qsc evidence.

## Next recommendation

Merge the NA-0537 implementation PR only after required validation and protection checks pass. If post-merge public-safety is green inside the short attach/early-failure window, close out NA-0537 and restore NA-0538 as the sole READY successor. Do not implement NA-0538 in the closeout.
