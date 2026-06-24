Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-06-24

# NA-0533 qsl remote qsc E2EE reverse-forward port 39176 regression diagnostic scope authorization testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Record validation for NA-0533, an authorization-only lane that consumes the D436 port 39176 reverse-forward failure, inventories prior forwarding successes and failures, and selects NA-0534 as the next bounded diagnostic implementation lane.

## Expected classification

`REMOTE_FORWARD_PORT_39176_REGRESSION_DIAGNOSTIC_IMPLEMENTATION_READY`

This classification authorizes only a future diagnostic lane. It does not prove SSH forwarding is currently healthy, qsc E2EE, qsc send/receive, identity correctness, trust correctness, replay resistance, downgrade resistance, side-channel safety, or secret lifecycle.

## Scope guard

Allowed changed paths:

- `docs/governance/evidence/NA-0533_qsl_remote_qsc_e2ee_reverse_forward_port_39176_regression_diagnostic_scope_authorization_plan.md`
- `tests/NA-0533_qsl_remote_qsc_e2ee_reverse_forward_port_39176_regression_diagnostic_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No `NEXT_ACTIONS.md`, qsc source/test/fuzz/Cargo, workflow/script/helper, dependency/lockfile, corpus/vector/input, formal/refimpl/service/public/backup, qsl-server, qsl-attachments, qshield, qshield-cli, qwork/qstart/qresume, qsl-backup, SSH key, authorized_keys, known_hosts, or SSH config path may change.

## qwork and queue checks

Expected startup proof:

- qwork proof files exist and are copied into the proof root.
- Codex does not run `qwork`, `qstart`, or `qresume`.
- qwork proof HEAD and `origin/main` match live refs before fetch.
- READY_COUNT is 1.
- READY item is NA-0533.
- NA-0532, NA-0531, and NA-0530 are DONE.
- D-1054 and D-1055 exist once.
- D-1056 and D-1057 are absent before patch.
- Duplicate decision count is 0.

## Main health checks

Expected current-main proof before patch:

- `origin/main` equals or descends from `93d90d4657a8`.
- `public-safety` completed success.
- `advisories` completed success.
- No required red check is present in retrieved check-runs.
- Root `Cargo.lock` has `quinn-proto 0.11.15`.
- `qsl/qsl-client/qsc/fuzz/Cargo.lock` has `quinn-proto 0.11.15`.
- Cargo manifest drift check passes.
- qsl-backup helper hash matches the expected read-only digest.
- Codex ops source inclusion count in qsl-backup is exactly 1.
- qsl-backup is not executed.

## Inheritance checks

Expected inherited facts:

- D437 records NA-0532 DONE and NA-0533 READY.
- D437 records PR #1338 merged at `93d90d4657a8`.
- D436 classification is `REMOTE_E2EE_INTEGRATED_FORWARDING_PRECHECK_FAILURE`.
- D436 reverse-forward failed before remote trigger with `remote port forwarding failed for listen port 39176`.
- D436 local listener was ready.
- D436 local trigger rehearsal passed.
- D436 remote trigger was never executed.
- D436 ran no qsc E2EE and no qsc send/receive.
- D435 classification is `REMOTE_TRIGGER_QUOTING_REMEDIATION_MARKER_TRAVERSAL_PASS`.
- D435 stdin-script trigger shape worked.
- D435 integrated marker traversal and ACK passed.
- D414 classification is `SSH_FORWARDING_CAPABILITY_PROBE_PASS`.
- D414 dedicated-key reverse-forward marker traversal and ACK passed.
- D413 accepted the dedicated forwarding key proof with loopback-only constraints.
- qsl-server and qsl-attachments remain out of scope.

## Regression inventory checks

Expected success cases:

- NA-0520 reverse-forward to `127.0.0.1:39176` succeeded.
- NA-0520 marker traversal and ACK succeeded.
- Manual integrated proof marker traversal and ACK succeeded.
- NA-0531 stdin-script trigger shape succeeded.
- NA-0531 marker traversal and ACK succeeded.

Expected failure cases:

- D427 first config had `ClearAllForwardings yes`.
- D427 corrected retry failed with `remote port forwarding failed for listen port 39176`.
- D430 reverse-forward started and remote trigger failed.
- D433 trigger quoting failed.
- D436 reverse-forward failed before trigger with `remote port forwarding failed for listen port 39176`.

Expected cause categories reviewed:

- stale remote listener/session.
- stale local process or ControlMaster/session reuse.
- remote loopback port already bound.
- remote bind host syntax.
- proof-root SSH config drift.
- default alias hazards.
- dedicated-key `permitlisten` / `permitopen` drift.
- forced-command compatibility regression.
- sshd policy drift.
- remote address-family IPv4/IPv6 ambiguity.
- Tailscale/DNS alias drift.
- local listener readiness/timing.
- cleanup race.

## Option review checks

Expected option result:

- Option 1, bounded port-state diagnostic implementation, is selected.
- Option 2, operator-side proof review, is deferred unless diagnostics implicate key/sshd policy.
- Option 3, stale-session cleanup diagnostic, is allowed only for local proof-root processes.
- Option 4, alternate remote listen port, is rejected without later operator-action key change.
- Option 5, immediate E2EE retry, is rejected.
- Option 6, qsl-server/qsl-attachments integration, is deferred/out of direct qsc sprint.
- Option 7, broad sshd/key remediation, is rejected without diagnostic proof and operator action.
- Option 8, abandon remote sprint / cleanup retained qsc, is rejected unless diagnostics become irrecoverable.

## Future command family checks

Expected selected future command family:

- qwork proof reading only.
- safe `ssh -G` parsing.
- local process and port checks for `127.0.0.1:39176`.
- proof-root SSH config with dedicated forwarding key and no `ClearAllForwardings yes`.
- bounded operational SSH for read-only remote boundary checks.
- read-only remote port checks.
- optional transient remote loopback bind probe, only if future directive selects it, binding only `127.0.0.1:39176`, exiting immediately, and writing no files.
- local listener on `127.0.0.1:39176`.
- dedicated-key reverse forward with `ssh -N -T`, `ExitOnForwardFailure=yes`, and loopback-only `-R 127.0.0.1:39176:127.0.0.1:39176`.
- stdin-script remote trigger only after forwarding starts.
- sanitized SSH debug logs if needed.
- cleanup of local proof-root listener and SSH processes.

Expected future forbids:

- no qsc E2EE.
- no qsc send/receive.
- no qsc identity/contact/handshake/relay protocol commands.
- no remote file write.
- no authorized_keys read.
- no sshd_config read.
- no key/config mutation.
- no qsl-server/qsl-attachments.

## Stewardship review checks

Expected completed reviews:

- Best-Known-Method Review.
- Hostile Cryptographer Review.
- Red-Team Review.
- Production SRE Review.
- Side-Channel Caveat.
- Formal-Model Mapping Residual.
- External-Review Readiness.
- Release-Claim Boundary.
- Assurance Gap Review Trigger.

## Marker checks

Expected NA-0533 checked-in markers or equivalent evidence statements:

- `REMOTE_FORWARD_PORT_39176_REGRESSION_DIAGNOSTIC_IMPLEMENTATION_READY`
- `NA0534_D436_PORT_FAILURE_CONSUMED_OK`
- `NA0534_NA0520_FORWARDING_SUCCESS_CONSUMED_OK`
- `NA0534_NA0531_TRIGGER_SUCCESS_CONSUMED_OK`
- `NA0534_PORT_STATE_CHECKED_OK`
- `NA0534_DEDICATED_KEY_USED_OK`
- `NA0534_PROOF_ROOT_CONFIG_SAFE_OK`
- `NA0534_EXIT_ON_FORWARD_FAILURE_USED_OK`
- `NA0534_REMOTE_FILE_WRITE_ABSENT_OK`
- `NA0534_MARKER_TRAVERSAL_RESULT_RECORDED_OK`
- `NA0534_SSH_DEBUG_LOG_REDACTED_OK`
- `NA0534_NO_QSC_E2EE_OK`
- `NA0534_NO_QSC_SEND_RECEIVE_OK`
- `NA0534_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0534_CLEANUP_COMPLETED_OK`
- `NA0534_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0534_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0534_ONE_READY_INVARIANT_OK`

## Validation commands

Required local validation:

- `git diff --check`
- exact five-path scope guard
- link-check
- leak-scan
- overclaim scan
- classifier
- PR body preflight
- goal-lint
- marker proof for NA-0533 evidence
- private-key block scan
- private key/passphrase/token/password material scan
- `cargo audit --deny warnings`
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo test -p qsc --locked --test same_host_client_to_client_e2e -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test receive_e2e -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test key_lifecycle_zeroization_expansion -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture`
- `python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics`
- `python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus`
- `python3 formal/run_model_checks.py`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

## Claim boundary

NA-0533 introduces no public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, identity-complete, trust-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim.

## Expected PR body facts

The PR body must include:

- standalone `Goals: G1, G2, G3, G4, G5`.
- authorization-only.
- D436 port 39176 forwarding failure consumed.
- D435 trigger success consumed.
- NA-0520 forwarding success consumed.
- selected NA-0534 successor.
- no remote action by Codex.
- no SSH execution.
- no qsc send/receive in NA-0533.
- no remote E2EE in NA-0533.
- qsl-server/qsl-attachments deferred.
- no qsc source/test/fuzz/Cargo mutation.
- no workflow/script/helper/dependency mutation.
- no corpus/vector/input mutation.
- no public-readiness, production-readiness, crypto-complete, identity-complete, trust-complete, replay-proof, downgrade-proof, or public-internet-readiness claim.

## Expected post-merge handling

After merge:

- READY remains NA-0533 until a separate closeout.
- D-1056 exists on main.
- If post-merge public-safety attaches and is green inside the short window, a separate closeout may restore NA-0534 READY.
- If public-safety is still running but healthy after the short attach window, stop and hand off for closeout.
- Codex must not run qwork post-merge.
