Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-22

# NA-0519 QSL remote qsc E2EE SSH forwarding operator action proof review

## Executive summary

NA-0519 is a proof-review-only governance/security lane. It consumes NA-0518 / D412 inheritance and operator-provided redacted proof that a dedicated qslcodex SSH forwarding key was generated on Build and installed on Inspiron with loopback-only forwarding constraints.

Primary classification: `SSH_FORWARDING_OPERATOR_PROOF_ACCEPTED_WITH_COMPATIBILITY_CAVEAT`.

The operator proof is sufficient to authorize only a future bounded forwarding capability probe. It does not prove SSH transport success, qsc E2EE success, protocol correctness, production readiness, or public readiness.

Selected successor: `NA-0520 -- QSL Remote qsc E2EE SSH Forwarding Capability Probe Implementation Harness`.

Required marker summary:

- `NA0519_QWORK_PROOF_FILES_VERIFIED_OK`
- `NA0519_D412_INHERITANCE_CONSUMED_OK`
- `NA0519_OPERATOR_PROOF_CONSUMED_OK`
- `NA0519_DEDICATED_FORWARDING_KEY_PROOF_ACCEPTED_OK`
- `NA0519_AUTHORIZED_KEYS_OPTIONS_REVIEWED_OK`
- `NA0519_NO_PTY_AGENT_X11_OK`
- `NA0519_PRIVILEGE_BACKUP_QWORK_QSLBACKUP_REVIEWED_OK`
- `NA0519_CLEANUP_REVOCATION_REVIEWED_OK`
- `NA0519_SSH_FORWARDING_OPERATOR_PROOF_ACCEPTED_WITH_COMPATIBILITY_CAVEAT`
- `NA0519_SELECTED_NA0520_SUCCESSOR_OK`
- `NA0519_NO_REMOTE_ACTION_BY_CODEX_OK`
- `NA0519_NO_SSH_EXECUTION_BY_CODEX_OK`
- `NA0519_NO_AUTHORIZED_KEYS_MUTATION_OK`
- `NA0519_NO_KEY_GENERATION_INSTALLATION_OK`
- `NA0519_NO_QSC_SOURCE_TEST_FUZZ_CARGO_MUTATION_OK`
- `NA0519_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0519_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0519_ONE_READY_INVARIANT_OK`

## Live NA-0519 scope

Allowed repository mutation paths for this proof-review evidence PR:

- `docs/governance/evidence/NA-0519_qsl_remote_qsc_e2ee_ssh_forwarding_operator_action_proof_review.md`
- `tests/NA-0519_qsl_remote_qsc_e2ee_ssh_forwarding_operator_action_proof_review_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

The live queue at startup had READY_COUNT 1 and sole READY item `NA-0519 -- QSL Remote qsc E2EE SSH Forwarding Operator Action Proof Review Harness`.

This lane is proof-review only. Codex did not run SSH, did not test forwarding, did not run scp/sftp/rsync, did not execute qsc send/receive, did not run remote E2EE, did not generate or install keys, did not edit authorized_keys, did not mutate SSH config or known_hosts, did not mutate remote host state, and did not use qsl-server or qsl-attachments.

## qwork proof-file verification

Codex read and copied the qwork proof files without running qwork, qstart, or qresume:

- `/srv/qbuild/work/NA-0519/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0519/.qwork/startup.qsl-protocol.json`

Required proof fields passed:

- `startup_result=OK`
- `lane=NA-0519`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0519/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0519`
- `requested_lane_status=READY`

Freshness passed before fetch: proof HEAD and proof origin/main matched live refs at `0e976a45d3b3`.

Disk proof passed before fetch: `/` usage was below the 95% stop threshold and `/backup/qsl` was checked read-only below the stop threshold.

qsl-backup boundary was read-only: installed helper digest matched expected short digest `e9ecff3d22ed`, the Codex ops source inclusion count was exactly 1, and qsl-backup was not executed.

Current main health before mutation: `public-safety` completed success, `qsc-adversarial-smoke` completed success, `qsc-linux-full-suite` and `macos-qsc-full-serial` were accepted skipped by existing docs/governance-only policy, and no required red check was present in the retrieved check-run set.

## NA-0518 / D412 inheritance

NA-0518 completed. NA-0519 was restored READY. D-1025 and D-1026 exist once. D-1027 was absent at startup.

D412 selected `SSH_FORWARDING_DEDICATED_KEY_OPERATOR_ACTION_READY`.

Inherited facts:

- PTY broadening was rejected and not selected.
- A separate dedicated qslcodex forwarding key was selected by default.
- Loopback-only forwarding was selected for `127.0.0.1:39176`.
- The selected authorized_keys template included `restrict`, explicit `port-forwarding`, loopback-only `permitlisten` and `permitopen`, and a forced no-shell command if compatible.
- Future proof review was selected before any Codex SSH forwarding or remote E2EE retry.
- NA-0518 performed no remote action, no SSH execution, no key generation or installation by Codex, no authorized_keys mutation by Codex, no qsc send/receive, no remote E2EE, no qsl-server/qsl-attachments use, and no public/production readiness claim.

The selected template inherited from NA-0518 was:

```text
restrict,port-forwarding,permitlisten="127.0.0.1:39176",permitopen="127.0.0.1:39176",command="/bin/false" ssh-ed25519 <PUBLIC_KEY> qsl-inspiron-qslcodex-forward-<date>
```

## Operator proof reviewed

The proof reviewed here was supplied by the operator in the directive body. It is operator-supplied proof, not Codex-executed proof.

The operator proof asserts:

- dedicated forwarding key generated on Build / ideacentre for user victor;
- public key installed for target Inspiron account qslcodex;
- relevant forwarding key fingerprint `SHA256:7guT/Hd72vuu+u1Jrwpuy0hSFETVrxYaXHwupuFfaPM`;
- relevant forwarding key comment `qsl-inspiron-qslcodex-forward-20260622`;
- private key content not pasted;
- passphrase not pasted;
- no password, token, or private material included;
- cleanup/revocation command documented.

The proof did not include the full authorized_keys file, unrelated keys, known_hosts output, private SSH key material, passphrase material, passwords, tokens, credentials, or backup private material.

## Dedicated forwarding key proof

Accepted dedicated-key conclusions:

- The operator reported a dedicated forwarding key exists locally on Build.
- The reported public key fingerprint is `SHA256:7guT/Hd72vuu+u1Jrwpuy0hSFETVrxYaXHwupuFfaPM`.
- The reported public key comment is `qsl-inspiron-qslcodex-forward-20260622`.
- The private key file metadata was reported as owned by victor with mode 600.
- The public key file metadata was reported as owned by victor with mode 644.
- Private key content was not pasted.
- Passphrase state is not inferred beyond the supplied proof; the passphrase itself was not pasted.

This establishes a key-management posture adequate for a later forwarding capability probe. It does not establish that the future SSH command will authenticate or that reverse forwarding will succeed.

## authorized_keys option summary proof

Accepted authorized_keys conclusions for the relevant forwarding key:

- `AUTHORIZED_KEYS_LINE_COUNT 2`
- `FORWARDING_KEY_MATCH_COUNT 1`
- relevant forwarding key fingerprint matches `SHA256:7guT/Hd72vuu+u1Jrwpuy0hSFETVrxYaXHwupuFfaPM`
- `restrict` present
- `port-forwarding` present
- `permitlisten` constrained to `127.0.0.1:39176`
- `permitopen` constrained to `127.0.0.1:39176`
- forced no-shell command present

The proof is scoped to the relevant key and does not expose unrelated authorized_keys content.

## PTY / agent / X11 proof

Accepted conclusions:

- PTY option absent.
- Agent forwarding not enabled.
- X11 forwarding not enabled.

PTY remains disabled because the next step is a noninteractive forwarding capability probe. Enabling PTY would broaden the key beyond the selected remediation and is not needed for NA-0520.

## qslcodex privilege / backup / qwork / qsl-backup proof

Accepted qslcodex privilege conclusions:

- qslcodex uid/gid/group proof identifies only qslcodex group membership.
- `QSLCODEX_GROUP_NAMES qslcodex`
- privileged groups absent: sudo, adm, docker, lxd, libvirt, wheel, admin.
- `QSLCODEX_SUDO_NOT_ALLOWED yes`

Accepted boundary conclusions as qslcodex:

- `/backup/qsl` absent or not readable by qslcodex.
- qwork absent for qslcodex.
- qsl-backup absent for qslcodex.

These conclusions reduce privilege and backup-exposure risk for a future forwarding capability probe. They do not prove remote execution safety beyond the scoped proof facts.

## Cleanup / revocation proof

The operator supplied a cleanup/revocation command that backs up the qslcodex authorized_keys file, removes the forwarding key line by its comment `qsl-inspiron-qslcodex-forward-20260622`, restores qslcodex ownership, and enforces mode 600.

The cleanup plan is accepted as documented proof for NA-0519. Future lanes should require post-action cleanup evidence and must stop if cleanup fails.

## Security classification

Selected proof-review classification:

`SSH_FORWARDING_OPERATOR_PROOF_ACCEPTED_WITH_COMPATIBILITY_CAVEAT`

Reasons:

- The proof satisfies the selected dedicated-key, loopback forwarding, no-PTY, no-agent, no-X11, no-sudo, no-backup, no-qwork, no-qsl-backup, no-private-material, and revocation-plan constraints.
- Forwarding capability itself has not yet been tested.
- `command="/bin/false"` may or may not be compatible with the future `-N -R` reverse-forward command.
- The future capability probe must test that compatibility and stop if incompatible.

## Future forwarding capability probe design

Selected future NA-0520 design:

1. Build starts a proof-root-local loopback listener on `127.0.0.1:39176`.
2. Codex starts one bounded SSH reverse forward using the dedicated forwarding key, with no PTY, no agent forwarding, no X11 forwarding, publickey-only authentication, strict host-key checking, identities-only, and `ExitOnForwardFailure`.
3. Codex uses an explicitly authorized bounded remote loopback trigger command only if needed.
4. A synthetic marker must traverse the reverse tunnel.
5. Codex kills the local listener and SSH forward.
6. Codex verifies no lingering local proof process.
7. NA-0520 runs no qsc E2EE, no qsc send/receive, and no qsl-server/qsl-attachments.

If the future probe needs the existing operational key only to trigger a remote loopback connection, NA-0520 must explicitly authorize that separate bounded command. It must not enable forwarding on the operational key, request PTY, use sudo, or mutate remote files.

If `command="/bin/false"` prevents the `-N -R` reverse-forwarding session, NA-0520 must stop and select remediation. It must not silently remove the forced command.

## Future proof outputs and stop rules

Future NA-0520 proof must include:

- forwarding key fingerprint;
- local listener bind proof;
- redacted SSH reverse-forward command proof;
- `ExitOnForwardFailure` proof;
- marker sent and received through the tunnel;
- no PTY proof where possible;
- no agent/X11 forwarding;
- no qsl-server/qsl-attachments;
- no remote E2EE;
- cleanup of listener and SSH process;
- no retained qsc mutation;
- no public/production readiness claim.

Future NA-0520 must stop if:

- SSH with the dedicated forwarding key fails;
- forwarding is administratively prohibited;
- `command="/bin/false"` breaks forwarding;
- the tunnel binds to a non-loopback address;
- the marker does not traverse;
- the remote trigger requires PTY, sudo, or a remote file write;
- qsl-server or qsl-attachments are needed;
- qwork or qsl-backup appear remotely;
- cleanup fails;
- public or production readiness claim pressure appears.

## Successor option review

Option 1 - Forwarding capability probe using dedicated key: selected. It reduces uncertainty around the exact SSH forwarding capability, addresses the untested-transport gap, is feasible with bounded scope, has low remote mutation risk if trigger commands are read-only and temporary, keeps secret/key risk constrained to the dedicated key, introduces no public-claim expansion, and likely future paths are NA-0520 evidence/testplan/decision/traceability/journal plus proof-root-local process evidence. P0 risk: tunnel binds beyond loopback or cleanup fails. P1 risk: forced command blocks forwarding. P2 risk: proof capture ambiguity.

Option 2 - Direct remote E2EE retry: deferred. It would skip the remaining forwarding capability gap and risks conflating SSH setup failure with qsc behavior. Scope and public-claim risk are higher than a capability probe.

Option 3 - Forwarding remediation due `command="/bin/false"` risk: deferred. Current proof does not show compatibility failure. Remediation is selected only if NA-0520 proves incompatibility.

Option 4 - Use existing operational key for forwarding: rejected/deferred. Dedicated-key proof exists and is safer because it isolates forwarding authorization from the existing operational key.

Option 5 - Enable PTY: rejected. PTY is not needed for a forwarding probe and would broaden the key.

Option 6 - qsl-server/qsl-attachments integration: rejected/deferred. It is outside the direct qsc sprint and does not answer the immediate loopback reverse-forward capability question.

Option 7 - Cleanup and abandon remote sprint: rejected for now. The proof is accepted, so a bounded forwarding capability probe is the proportional next step.

## Hostile Cryptographer Review

This proof review establishes only SSH policy posture from operator-supplied redacted evidence. It does not prove transport success, qsc E2EE success, protocol correctness, crypto completeness, replay resistance, downgrade resistance, secret-material completeness, or side-channel freedom.

Forwarding capability must still be tested before any E2EE retry because the proof does not exercise the SSH server, the key options, the forced command, the loopback bind, or marker transit through the tunnel.

PTY remains disabled because the selected QSL forwarding path is noninteractive. Enabling PTY would increase authority without solving the E2EE transport need.

No public or production readiness claim is made because this lane reviewed only one redacted operator proof bundle for one future capability probe.

## Red-Team Review

If the operator proof was forged or incomplete, NA-0520 could fail safely at authentication, forwarding setup, loopback bind verification, marker transit, or cleanup. NA-0520 must treat any mismatch as a stop condition.

If `command="/bin/false"` blocks forwarding, NA-0520 must stop and select remediation rather than silently weakening the authorized_keys line.

If `permitlisten` or `permitopen` are too broad, ineffective, or misapplied, NA-0520 must stop before any E2EE retry. It must prove loopback bind behavior and reject non-loopback exposure.

If the dedicated key leaks, the revocation command must be used and proof should show the forwarding key line is removed. Key rotation or sprint abandonment may be required before further remote work.

Cleanup/revocation proof required after testing: removal of the forwarding-key authorized_keys line, ownership and mode restored, no lingering local listener or SSH process, and no qwork/qsl-backup exposure.

## Production SRE Review

The proof is sufficient for a forwarding capability probe, not for an E2EE retry.

Operational risk remains around key handling, host-key trust assumptions, forced-command compatibility, reverse-forward bind behavior, remote trigger mechanics, cleanup, and operator-proof fidelity.

Before any E2EE retry, NA-0520 should verify that the dedicated key authenticates, reverse forwarding starts with `ExitOnForwardFailure`, the listener remains loopback-only, a synthetic marker traverses the tunnel, no PTY is required, agent/X11 forwarding remain disabled, qsl-server/qsl-attachments remain unused, no remote file writes occur, and cleanup completes.

Key revocation after testing should remove the dedicated forwarding key line by comment, preserve authorized_keys ownership and mode, and record proof that the key no longer authorizes the test path if a later lane requests revocation verification.

qsl-server and qsl-attachments remain deferred because the selected sprint is direct qsc evidence and the current gap is SSH forwarding capability, not production service integration.

## Release-Claim Boundary Review

No public-readiness claim is made. No production-readiness claim is made. No public-internet-readiness claim is made. No external-review-complete claim is made. No crypto-complete claim is made. No replay-proof claim is made. No downgrade-proof claim is made. No secret-material-complete claim is made. No side-channel-free claim is made. No vulnerability-free claim is made. No bug-free claim is made. No perfect-crypto claim is made.

## Authorization decision

Primary classification: `SSH_FORWARDING_OPERATOR_PROOF_ACCEPTED_WITH_COMPATIBILITY_CAVEAT`.

Required assertions:

- NA-0518/D412 consumed.
- Operator proof consumed.
- Dedicated forwarding key proof accepted for a future forwarding capability probe.
- No private material included.
- Compatibility caveat around `command="/bin/false"` recorded.
- NA-0520 successor selected.
- No remote action in NA-0519.
- No SSH execution in NA-0519.
- No authorized_keys mutation in NA-0519.
- No key generation or installation in NA-0519.
- No qsc send/receive in NA-0519.
- No remote E2EE in NA-0519.
- No qsl-server/qsl-attachments.
- No public claim expansion.
- Exactly one READY successor remains mandatory.

## Selected NA-0520 successor

`NA-0520 -- QSL Remote qsc E2EE SSH Forwarding Capability Probe Implementation Harness`

Status after NA-0519 closeout should be READY. NA-0520 must execute only the bounded loopback reverse-forwarding capability probe and must not execute qsc E2EE.

## Future scope bundle

NA-0520 allowed scope:

- `docs/governance/evidence/NA-0520_qsl_remote_qsc_e2ee_ssh_forwarding_capability_probe_implementation_harness.md`
- `tests/NA-0520_qsl_remote_qsc_e2ee_ssh_forwarding_capability_probe_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- proof-root-local loopback listener
- one bounded SSH reverse-forward command using the dedicated forwarding key
- one bounded remote loopback trigger command if explicitly needed
- synthetic marker over the tunnel
- cleanup of local listener and SSH process

NA-0520 forbidden scope:

- qsc E2EE;
- qsc send/receive;
- qsl-server/qsl-attachments;
- package installation;
- sudo/admin action;
- key generation or installation;
- authorized_keys mutation;
- SSH config mutation;
- known_hosts mutation;
- remote host mutation;
- remote file write;
- qwork/qstart/qresume mutation;
- qsl-backup execution;
- qsc source/test/fuzz/Cargo mutation;
- workflow/dependency mutation;
- corpus/vector/input mutation;
- formal/refimpl/service/public/backup mutation;
- public-readiness or production-readiness claims.

## Future validation / marker plan

Future NA-0520 markers:

- `NA0520_FORWARDING_PROOF_REVIEW_CONSUMED_OK`
- `NA0520_DEDICATED_FORWARDING_KEY_USED_OK`
- `NA0520_LOOPBACK_LISTENER_STARTED_OK`
- `NA0520_REVERSE_FORWARD_STARTED_OK`
- `NA0520_EXIT_ON_FORWARD_FAILURE_OK`
- `NA0520_TUNNEL_MARKER_TRAVERSED_OK`
- `NA0520_NO_PTY_REQUIRED_OK`
- `NA0520_NO_AGENT_X11_FORWARDING_OK`
- `NA0520_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0520_NO_REMOTE_E2EE_OK`
- `NA0520_NO_REMOTE_FILE_WRITE_OK`
- `NA0520_CLEANUP_COMPLETED_OK`
- `NA0520_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0520_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0520_ONE_READY_INVARIANT_OK`

## No remote action in NA-0519

Codex performed no remote action in this lane. Codex did not run SSH, did not test forwarding, did not run scp/sftp/rsync, did not execute qsc send/receive, did not run remote E2EE, did not generate or install keys, did not inspect private SSH keys, did not mutate authorized_keys, did not mutate SSH config or known_hosts, did not mutate remote host state, did not use sudo/admin, did not run qwork/qstart/qresume, and did not execute qsl-backup.

## No qsl-server / no qsl-attachments boundary

qsl-server and qsl-attachments remain out of scope. They were not used, read for execution, modified, or required to reach the NA-0519 decision.

## Public claim / website / external review boundary

This lane changes no public docs, website, README, START_HERE, public service surface, formal model, refimpl, qsl-server, qsl-attachments, backup, qshield, or qshield-cli paths.

No public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim is introduced.

## Backup-impact statement

Backup impact: none. qsl-backup was checked read-only by digest and source inclusion count and was not executed or mutated. `/backup/qsl` was checked for disk watermark only and was not mutated.

## Rejected alternatives

Rejected or deferred alternatives:

- direct remote E2EE retry before forwarding capability proof;
- modifying the existing operational key for forwarding;
- enabling PTY;
- removing `command="/bin/false"` without proof;
- qsl-server/qsl-attachments integration;
- abandoning the remote sprint while proof is accepted.

## Next recommendation

Merge this proof-review evidence PR after required validation and checks pass. Then, if post-merge public-safety is green inside the short attach/early-failure window, close out NA-0519 and restore NA-0520 as the sole READY successor. NA-0520 should probe only loopback reverse-forwarding capability with the dedicated key and stop on any compatibility or cleanup failure.
