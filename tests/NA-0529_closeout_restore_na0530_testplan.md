Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-24

# NA-0529 closeout and NA-0530 restoration testplan

## Purpose

Record validation for the NA-0529 closeout-only governance patch that accepts D-1048, reviews the operator-supplied manual integrated forwarding proof, marks NA-0529 DONE, and restores NA-0530 READY with a mandatory in-lane integrated marker traversal and ACK precheck before qsc E2EE.

## Expected classification

`NA0529_CLOSEOUT_RESTORE_NA0530_INTEGRATED_FORWARDING_REVIEW_PASS`

## Static governance checks

Required checks:

- `git diff --check`
- exact five-path closeout scope guard
- repository-native decision parser for D-1048 once, D-1049 once, D-1050 absent, and duplicate decision count zero
- READY proof with exactly one READY item, NA-0530, after the patch
- link-check for markdown links
- leak scan for private key blocks and high-likelihood token patterns
- overclaim scan for public/production/security completion claims
- classifier proof for docs/governance closeout scope
- PR body preflight
- goal-lint preflight with standalone `Goals: G1, G2, G3, G4, G5`

## Required closeout markers

Evidence must contain:

- `NA0529_CLOSEOUT_PR1331_MERGED_OK`
- `NA0529_CLOSEOUT_D1048_ACCEPTED_OK`
- `NA0529_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0529_CLOSEOUT_ADVISORIES_GREEN_OK`
- `NA0529_CLOSEOUT_D430_TRIGGER_FAILURE_CONSUMED_OK`
- `NA0529_CLOSEOUT_MANUAL_INTEGRATED_MARKER_TRAVERSED_OK`
- `NA0529_CLOSEOUT_MANUAL_ACK_OK`
- `NA0529_CLOSEOUT_MANUAL_PROOF_SECRET_SCAN_OK`
- `NA0529_CLOSEOUT_D1049_RESTORED_NA0530_OK`
- `NA0529_CLOSEOUT_NA0529_DONE_OK`
- `NA0529_CLOSEOUT_NA0530_READY_OK`
- `NA0529_CLOSEOUT_NO_NA0530_IMPLEMENTATION_OK`
- `NA0529_CLOSEOUT_NO_REMOTE_ACTION_OK`
- `NA0529_CLOSEOUT_NO_SSH_EXECUTION_OK`
- `NA0529_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK`
- `NA0529_CLOSEOUT_NO_REMOTE_E2EE_OK`
- `NA0529_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0529_CLOSEOUT_NO_DEPENDENCY_LOCKFILE_MUTATION_OK`
- `NA0529_CLOSEOUT_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0529_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Manual proof review checks

Required proof findings:

- manual proof root exists at `/srv/qbuild/tmp/manual_reverse_forward_marker_integrated_20260624T012231Z`
- `REMOTE_ACK=MANUAL_TUNNEL_ACK_OK`
- `INTEGRATED_MARKER_TRAVERSED_OK`
- `"marker_match": true`
- `"ack_sent": true`
- `"ok": true`
- local listener bound to `127.0.0.1:39176`
- remote trigger connected to remote `127.0.0.1:39176`
- marker crossed the tunnel
- local listener returned ACK
- remote trigger received ACK
- cleanup left no local or remote listener row on `39176`
- no private key block, passphrase, token, password value, credential, production endpoint, backup material, or raw authorized_keys/sshd_config/known_hosts dump
- post-manual qwork proof corroborates clean repo/no code or governance mutation

## Boundary assertions

Validation must prove:

- no NA-0530 implementation occurred;
- no remote action occurred;
- no SSH command ran;
- no scp, sftp, or rsync command ran;
- no qsc E2EE command ran;
- no qsc send/receive command ran;
- no qsc protocol command ran;
- no qsl-server command/path was used or mutated;
- no qsl-attachments command/path was used or mutated;
- no package install occurred;
- no key generation or key installation occurred;
- no authorized_keys read or mutation occurred;
- no sshd_config read or mutation occurred;
- no known_hosts mutation occurred;
- no qwork/qstart/qresume ran;
- no qsl-backup execution occurred;
- no qsc source/test/fuzz/Cargo mutation occurred;
- no workflow/script/helper mutation occurred;
- no dependency/lockfile mutation occurred;
- no corpus/vector/input mutation occurred;
- no formal/refimpl/service/public/backup mutation occurred;
- no public-readiness claim and no production-readiness claim is introduced.

## Required local validation bundle

Run before and after the governance patch:

- `git diff --check`
- `cargo audit --deny warnings`
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Preferred if runtime allows:

- `cargo test -p qsc --locked --test same_host_client_to_client_e2e -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test receive_e2e -- --test-threads=1 --nocapture`

Forbidden validation commands:

- qwork/qstart/qresume
- SSH
- scp/sftp/rsync
- qsc E2EE
- qsc send/receive
- qsl-server
- qsl-attachments
- qsl-backup
- cargo update

## PR checks

Before merge:

- required GitHub checks must attach and pass;
- public-safety must not be red or missing;
- advisories must complete success;
- merge must use a merge commit;
- no squash, rebase, force-push, amend, or branch deletion flag.

After merge:

- final queue has exactly one READY item, NA-0530;
- NA-0529 remains DONE;
- D-1049 exists once;
- D-1050 remains absent;
- post-closeout public-safety completes success;
- advisories complete success or accepted not-applicable status;
- no required red checks remain.
