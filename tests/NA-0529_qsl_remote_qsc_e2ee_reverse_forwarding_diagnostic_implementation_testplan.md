Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-24

# NA-0529 qsl remote qsc E2EE reverse-forwarding diagnostic implementation testplan

## Purpose

Record validation for NA-0529, which diagnosed the D427 reverse-forwarding failure without qsc E2EE, without qsc send/receive, without qsl-server, without qsl-attachments, and without remote file writes.

## Expected classification

`REMOTE_FORWARDING_DIAGNOSTIC_REMOTE_TRIGGER_FAILURE`

The dedicated-key reverse-forward session started with `ExitOnForwardFailure=yes`, but the single authorized remote trigger did not produce marker traversal or ACK proof.

## Static governance checks

Required checks:

- `git diff --check`
- exact five-path implementation scope guard
- repository-native decision parser for D-1048 once, D-1049 absent, and duplicate decision count zero
- READY proof remains exactly one READY item, NA-0529, before implementation PR
- link-check for markdown links
- leak scan for private key blocks and high-likelihood token patterns
- overclaim scan for public/production/security completion claims
- PR body preflight
- goal-lint preflight with standalone `Goals: G1, G2, G3, G4, G5`

## Runtime proof checks

Required proof artifacts:

- qwork proof files copied under the proof root
- D427 vs NA-0520 command/config diff markdown and JSON
- NA-0529 command manifest markdown and JSON
- safe `ssh -G inspiron` field parse
- proof-root effective SSH config field parse with `clearallforwardings no`
- remote boundary check summary
- local listener startup proof
- reverse-forward startup proof
- remote trigger failure classification JSON
- cleanup summary with no proof-root listener or SSH process remaining
- proof-root private-material scan with zero findings

## Required evidence markers

Evidence must contain:

- `NA0529_QWORK_PROOF_VERIFIED_OK`
- `NA0529_D427_NA0520_DIFF_RECORDED_OK`
- `NA0529_COMMAND_MANIFEST_RECORDED_OK`
- `NA0529_SAFE_SSH_CONFIG_VERIFIED_OK`
- `NA0529_REMOTE_BOUNDARY_CHECKED_OK`
- `NA0529_LOOPBACK_LISTENER_STARTED_OK`
- `NA0529_REVERSE_FORWARD_STARTED_OK`
- `NA0529_REMOTE_TRIGGER_FAILURE_RECORDED_OK`
- `NA0529_CLEANUP_COMPLETED_OK`
- `NA0529_NO_QSC_E2EE_OK`
- `NA0529_NO_QSC_SEND_RECEIVE_OK`
- `NA0529_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0529_NO_REMOTE_FILE_WRITE_OK`

## Boundary assertions

Validation must prove:

- no qsc E2EE command ran;
- no qsc send/receive command ran;
- no qsc identity/contact/handshake/relay protocol command ran;
- no qsl-server command/path was used or mutated;
- no qsl-attachments command/path was used or mutated;
- no remote file write occurred;
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

Run after the governance patch:

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

Forbidden validation commands:

- qsc E2EE
- qsc send/receive
- qsl-server
- qsl-attachments
- package install
- ssh-keygen
- ssh-keyscan
- local sudo
- qwork/qstart/qresume
- qsl-backup
- cargo update

## PR checks

Before merge:

- required GitHub checks must attach and pass;
- public-safety must not be red or missing;
- advisories must complete success;
- merge must use a merge commit;
- no squash, rebase, force-push, amend, or branch deletion flag.

## Closeout note

Because the selected classification is `REMOTE_FORWARDING_DIAGNOSTIC_REMOTE_TRIGGER_FAILURE`, any optional closeout must restore a successor matching that classification. If no approved successor NA block is available, stop before editing `NEXT_ACTIONS.md` and request explicit successor direction.
