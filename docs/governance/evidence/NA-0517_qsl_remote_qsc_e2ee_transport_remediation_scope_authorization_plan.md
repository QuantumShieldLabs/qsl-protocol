Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-21

# NA-0517 QSL remote qsc E2EE transport remediation scope authorization plan

## Executive summary

NA-0517 consumed NA-0516 / D410 transport-failure evidence and completed a read-only review of current qsc transport surfaces and SSH forwarding remediation options. No remote action was performed in this directive. Codex did not run SSH, scp, sftp, rsync, qsc send/receive, remote E2EE, key generation, SSH config mutation, authorized_keys mutation, known_hosts mutation, package installation, sudo/admin action, qwork/qstart/qresume, qsl-backup, qsl-server, or qsl-attachments.

The current qsc command surface does not provide a safe forwarding-free Build-to-Inspiron E2EE path. qsc send, receive, and handshake require an HTTP relay endpoint reachable by both clients. `qsc relay serve` binds loopback and stores its inbox in memory. The receive-side `--file` path is a legacy fail-closed parse/reject path, not a mailbox or artifact transport. A forwarding-free retry would require qsc source/test changes, a new helper/proxy, qsl-server/qsl-attachments, public exposure, or package/build work; all are out of scope for this lane.

Primary classification: `SSH_FORWARDING_OPERATOR_ACTION_AUTHORIZATION_READY`.

Selected successor: `NA-0518 -- QSL Remote qsc E2EE SSH Forwarding Operator Action Authorization Plan`.

NA-0518 should authorize an operator-owned, manual, redacted SSH forwarding remediation. The safer default is a separate dedicated forwarding key, not broadening the existing operational qslcodex key. The selected first forwarding model is loopback-only remote reverse forwarding for a Build-local qsc relay, constrained to one remote loopback listen endpoint, with no PTY, no agent forwarding, no X11 forwarding, no sudo/admin, no backup exposure, no qwork/qsl-backup exposure, and no qsl-server/qsl-attachments use.

Required marker summary:

- `NA0517_QWORK_PROOF_FILES_VERIFIED_OK`
- `NA0517_D410_TRANSPORT_FAILURE_CONSUMED_OK`
- `NA0517_D409_RESIDUE_CLEANUP_CONSUMED_OK`
- `NA0517_QSC_NATIVE_FORWARDING_FREE_REVIEWED_OK`
- `NA0517_QSC_NATIVE_FORWARDING_FREE_REJECTED_OK`
- `NA0517_SSH_FORWARDING_POLICY_REVIEWED_OK`
- `NA0517_FORWARDING_MODEL_SELECTED_OK`
- `NA0517_DEDICATED_FORWARDING_KEY_SELECTED_OK`
- `NA0517_OPERATOR_ACTION_ONLY_SELECTED_OK`
- `NA0517_NO_REMOTE_ACTION_BY_CODEX_OK`
- `NA0517_NO_SSH_EXECUTION_BY_CODEX_OK`
- `NA0517_NO_AUTHORIZED_KEYS_MUTATION_OK`
- `NA0517_NO_QSC_SOURCE_TEST_FUZZ_CARGO_MUTATION_OK`
- `NA0517_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0517_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0517_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0517_SSH_FORWARDING_OPERATOR_ACTION_AUTHORIZATION_READY`
- `NA0517_SELECTED_NA0518_SUCCESSOR_OK`
- `NA0517_ONE_READY_INVARIANT_OK`

## Live NA-0517 scope

Allowed repository mutation paths for this authorization evidence PR:

- `docs/governance/evidence/NA-0517_qsl_remote_qsc_e2ee_transport_remediation_scope_authorization_plan.md`
- `tests/NA-0517_qsl_remote_qsc_e2ee_transport_remediation_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

The live queue at startup had READY_COUNT 1 and sole READY item `NA-0517 -- QSL Remote qsc E2EE Transport Remediation Scope Authorization Plan`.

This lane is authorization-only. It does not implement NA-0518, does not retry remote E2EE, does not run SSH, does not transfer artifacts, and does not mutate qsc implementation code.

## qwork proof-file verification

Codex read and copied the qwork proof files without running qwork, qstart, or qresume:

- `/srv/qbuild/work/NA-0517/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0517/.qwork/startup.qsl-protocol.json`

Required proof fields passed:

- `startup_result=OK`
- `lane=NA-0517`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0517/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0517`
- `requested_lane_status=READY`

Freshness passed before fetch: proof HEAD and proof origin/main matched live refs at `5be44390c6c5`.

Disk proof passed before fetch: `/` usage was below the 95% stop threshold, and `/backup/qsl` was checked read-only below the stop threshold.

qsl-backup boundary was read-only: installed helper digest matched expected short digest `e9ecff3d22ed`, the Codex ops source inclusion count was exactly 1, and qsl-backup was not executed.

## NA-0516 / D409 / D410 inheritance

NA-0516 completed. NA-0517 was restored READY by NA-0516 closeout. D-1021 and D-1022 exist once. D-1023 was absent at startup.

D409 inheritance consumed:

- D409 stopped before governance patch, branch, commit, PR, merge, or closeout.
- D409 classification was `REMOTE_E2EE_AMBIGUOUS_STOP`.
- The stop cause was a literal-dollar-HOME remote path mistake.
- No qsl-server or qsl-attachments was used.
- No send/receive, reply, or wrong-mailbox negative boundary completed.

D410 cleanup and retry inheritance consumed:

- D410 removed the D409 remote literal-dollar-HOME tree.
- D410 removed the D409 local sensitive runtime root.
- D410 hardened retry path handling around resolved absolute remote paths.
- D410 rejected literal `$HOME`, `..`, and outside-prefix remote executable paths.
- D410 preserved and rechecked retained remote qsc under `/home/qslcodex/qsl-remote-test/bin/qsc`.
- Retained qsc owner remained `qslcodex`.
- Retained qsc digest matched the prior short digest prefix `6f12ab5eec24`.
- Local qsc provenance was recorded.
- D410 did not complete Build-to-Inspiron send/receive.
- D410 did not attempt Inspiron-to-Build reply after the positive path failed.
- D410 deferred wrong-mailbox negative/no-mutation proof.
- D410 introduced no custom proxy.
- D410 used no qsl-server and no qsl-attachments.
- D410 mutated no qsc source/test/fuzz/Cargo path.
- D410 mutated no workflow/script/helper, dependency/lockfile, corpus/vector/input, formal/refimpl/service/public/backup path.
- D410 classification was `REMOTE_E2EE_TRANSPORT_FAILURE`.

Accepted D410 transport failure details:

- SSH reverse forwarding was refused.
- SSH local forwarding was administratively prohibited.

This lane accepts D410 as bounded transport-block evidence, not remote E2EE success evidence.

## Operator SSH observation

Operator-supplied context, not Codex-executed proof:

- Manual command reported: `ssh qslcodex@inspiron`.
- Observed text: `PTY allocation request failed on channel 0`, followed by the Ubuntu login banner.

Interpretation for this lane:

- SSH key/account access appears to reach the remote account.
- PTY is disabled.
- PTY disablement is acceptable and should remain the default.
- Do not authorize broad PTY enablement.
- The current transport problem is TCP forwarding, not interactive shell access.

Codex did not run SSH to confirm this observation.

## qsc native transport / forwarding-free review

Read-only review covered current qsc tests and command/transport modules, including:

- `qsl/qsl-client/qsc/tests/same_host_client_to_client_e2e.rs`
- `qsl/qsl-client/qsc/tests/receive_e2e.rs`
- `qsl/qsl-client/qsc/tests/common/mod.rs`
- `qsl/qsl-client/qsc/tests/mock_relay_transport_na0173.rs`
- `qsl/qsl-client/qsc/src/cmd/mod.rs`
- `qsl/qsl-client/qsc/src/transport/mod.rs`
- `qsl/qsl-client/qsc/src/relay/mod.rs`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`

Findings:

- qsc `send` exposes only `SendTransport::Relay`.
- qsc `send` requires `--transport relay`, `--relay`, `--to`, and `--file`.
- qsc `receive` for the active client-to-client path requires `--transport relay`, `--relay`, `--mailbox`, `--from`, `--max`, and `--out`.
- qsc `handshake init` and `handshake poll` require a relay base URL.
- qsc relay push/pull functions use HTTP `/v1/push` and `/v1/pull` with route-token addressing.
- qsc `relay serve` binds `127.0.0.1` and keeps the relay inbox in process memory.
- qsc `receive --file` is a legacy receive path that rejects empty, oversize, or malformed inputs; it is not an offline mailbox import path.
- No file/inbox/artifact export command exists that can replace a reachable relay endpoint.
- No qsc-native forwarding-free Build-to-Inspiron path exists in the retained qsc surface.

Answers to the required questions:

- Can Build-to-Inspiron E2EE be executed by transferring mailbox/message artifacts without TCP port forwarding? No, not with the current retained qsc surface.
- Is there a file/inbox transport surface that can work across scp-only artifact transfer? No. The file receive surface is reject-only legacy input handling and no send-side offline artifact output exists.
- Does qsc send/receive require a relay endpoint reachable by both parties? Yes.
- Does the current retained qsc binary have a built-in transport that avoids SSH port forwarding? No.
- Would a forwarding-free path require qsc source/test modification? Yes.
- Would it require qsl-server or qsl-attachments? Not inherently, but using those services would be out of scope and unnecessary for this direct qsc sprint.
- Would it require public network exposure? A reachable service without SSH forwarding would require either public or LAN exposure; that is rejected for this lane.
- Would it require package installation or remote build? Not if the relay path remains as-is, but a new forwarding-free transport would require implementation work outside NA-0517.

Conclusion: `QSC_NATIVE_FORWARDING_FREE_E2EE_RETRY_READY` is rejected for this lane.

## SSH forwarding policy review

This review is policy design only. Codex did not edit SSH files, did not read private keys, did not read remote authorized_keys, and did not mutate local or remote SSH configuration.

Option A, existing qslcodex key with narrowly enabled forwarding:

- Status: Deferred / fallback only.
- Benefit: avoids a second public key.
- Risk: changes the current operational key boundary and could accidentally broaden a key used by prior remote lanes.
- Acceptable only if a future directive explicitly proves the existing key can be constrained equivalently, with no PTY, no agent forwarding, no X11 forwarding, no user rc, no sudo/admin, no backup exposure, no qwork/qsl-backup exposure, and exact forwarding host/port constraints.

Option B, separate dedicated forwarding key:

- Status: Selected default.
- Benefit: isolates forwarding authorization from the current qslcodex operational key and makes revocation/rollback easier.
- Required posture: operator-owned manual creation/installation only, public-key proof only, redacted authorized_keys option summary only, no private material, no Codex key generation, no Codex installation.
- Required key-level constraints: no PTY, no agent forwarding, no X11 forwarding, no user rc, no shell-like broadening, no sudo/admin, no backup exposure, exact loopback forwarding host/port constraints, and no qwork/qsl-backup exposure.

Option C, qsc-native forwarding-free path:

- Status: Rejected for current qsc surface.
- Reason: no current qsc send/handshake/offline mailbox path can replace a reachable relay endpoint.

Option D, operator-managed temporary SSH config or sshd policy:

- Status: Rejected unless key-level constraints are proven impossible in a later lane.
- Reason: broader account or daemon settings have larger blast radius than a dedicated constrained key.

Option E, qsl-server / qsl-attachments transport:

- Status: Rejected for this sprint.
- Reason: these components are protected architecture boundaries and remain out of scope for the direct qsc Build-to-Inspiron path.

Option F, defer remote E2EE:

- Status: Rejected for now.
- Reason: a safe next authorization step exists: operator-only, key-level SSH forwarding remediation.

Selected policy direction: separate dedicated forwarding key, key-level constraints, loopback-only forwarding, no PTY, no broad account or daemon-level `AllowTcpForwarding yes` change, no qsl-server/qsl-attachments.

## Forwarding model design

Shape 1, local relay on Build plus remote reverse forwarding:

- Build runs the qsc relay locally under a proof root.
- SSH reverse forwarding exposes that Build relay to Inspiron on remote loopback only.
- Future remote qsc commands use `http://127.0.0.1:39176` as the relay endpoint.
- Required operator authorization: key-level remote-forward authorization constrained to `127.0.0.1:39176` or an explicitly recorded single replacement port if the default is unavailable.
- Preferred authorized_keys option family: a dedicated public key with restrictive defaults plus port forwarding re-enabled only for `permitlisten="127.0.0.1:39176"` or equivalent.
- Benefit: no public exposure and no long-running remote relay process.
- Cleanup: stop local relay and close the SSH tunnel; no remote relay process cleanup required.
- D410 status: reverse forwarding was refused under existing settings.

Shape 2, remote relay on Inspiron plus local forwarding:

- Inspiron runs qsc relay under the approved remote E2EE root.
- SSH local forwarding exposes the remote relay to Build on Build loopback only.
- Required operator authorization: key-level local-forward authorization constrained by `permitopen="127.0.0.1:<remote-relay-port>"` or equivalent.
- Benefit: local forwarding is conventional and can be tightly constrained.
- Cost: requires a long-running remote qsc relay process and remote process cleanup.
- D410 status: local forwarding was administratively prohibited under existing settings.

Design answers:

- Smaller forwarding permission: both shapes can be reduced to one loopback host/port. Shape 1 is selected because it avoids a remote relay process and keeps the relay lifecycle local, even though it requires remote-forward permission.
- Avoids public exposure: both shapes can avoid public exposure if constrained to loopback.
- Keeps relay bound to loopback: both shapes can keep relay traffic loopback-only.
- Minimizes long-running remote processes: Shape 1.
- Easiest cleanup: Shape 1, because the relay stays local and the SSH tunnel teardown removes remote exposure.
- Aligns with existing qsc tests: both shapes preserve the existing HTTP relay endpoint model; Shape 1 more closely matches the same-host local relay test pattern with the remote side reaching a loopback relay endpoint.
- Future NA-0518 should authorize first: Shape 1, loopback-only remote reverse forwarding to a Build-local relay, using a separate dedicated forwarding key.

Selected first model: Shape 1.

## Future operator proof rules

If operator SSH forwarding remediation is performed, future proof must include only redacted, relevant proof:

- Redacted authorized_keys option summary for the qslcodex project forwarding key only.
- Public key fingerprint for the forwarding key.
- Whether a separate dedicated forwarding key was used; selected default is yes.
- Whether PTY remains disabled; selected default is yes.
- Whether agent forwarding remains disabled; selected default is yes.
- Whether X11 forwarding remains disabled; selected default is yes.
- Whether user rc remains disabled.
- Whether forwarding is constrained to one loopback host/port.
- Exact allowed listen/connect host and port in redacted form; selected default for Shape 1 is `127.0.0.1:39176`.
- Proof no sudo/admin group was added.
- Proof no backup exposure was added.
- Proof no qwork/qsl-backup exposure was added.
- Cleanup/reversion command or operator runbook step documented.

Forbidden proof outputs:

- Private key material.
- Passphrases.
- Full authorized_keys content with unrelated keys.
- known_hosts dumps.
- Unrelated host/IP inventory.
- Production endpoints.
- Backup material.
- Passwords, tokens, credentials, or auth headers.

Future proof review must reject:

- PTY enabled without explicit justification.
- Broad forwarding with no loopback and host/port constraints unless a later directive explicitly justifies and bounds a temporary exception.
- Unrelated keys shown.
- Sudo/admin group added.
- Backup/qwork/qsl-backup exposure.
- Private material or passphrase material.

## Successor option review

Option 1, qsc-native forwarding-free E2EE retry implementation:

- Decision: Rejected.
- Risk reduced: none with current code, because no such path exists.
- Evidence gap addressed: qsc command/transport review proves the gap.
- Feasibility: not feasible without qsc implementation work or service exposure.
- Scope risk: high.
- Remote mutation risk: would require a later implementation lane.
- Secret/key risk: not the primary risk.
- Public-claim risk: high if misrepresented as current capability.
- Likely future allowed paths: none for NA-0518 under this option.
- Likely future forbidden paths: qsc source/test/fuzz/Cargo mutation in this lane, qsl-server/qsl-attachments, public exposure.
- P0/P1/P2 risks: P0 false E2EE success claim; P1 unreviewed transport semantics; P2 wasted retry.

Option 2, SSH forwarding operator action authorization:

- Decision: Selected.
- Risk reduced: converts D410 transport blocker into an operator-owned least-privilege authorization step.
- Evidence gap addressed: exact forwarding policy, proof, redaction, and rejection rules.
- Feasibility: feasible as a governance/manual-action authorization lane.
- Scope risk: bounded if key-level and loopback-only.
- Remote mutation risk: operator-only, not Codex.
- Secret/key risk: controlled by public-fingerprint-only proof and no private material.
- Public-claim risk: bounded by no public-readiness claim and no production-readiness claim.
- Likely future allowed paths: NA-0518 evidence/testplan, DECISIONS, TRACEABILITY, rolling journal.
- Likely future forbidden paths: Codex SSH, authorized_keys mutation by Codex, key generation by Codex, qsc send/receive, remote E2EE.
- P0/P1/P2 risks: P0 broad SSH forwarding; P1 private key exposure; P2 port conflict.

Option 3, SSH forwarding proof review harness:

- Decision: Deferred.
- Reason: operator has not yet performed and supplied redacted forwarding proof.

Option 4, remote E2EE retry with existing settings:

- Decision: Rejected.
- Reason: D410 proved existing forwarding settings are blocked.

Option 5, qsl-server/qsl-attachments integration authorization:

- Decision: Rejected for current direct qsc sprint.
- Reason: protected architecture boundaries and additional deployment complexity.

Option 6, remote cleanup/remediation:

- Decision: Rejected as primary successor.
- Reason: D410 cleanup completed and retained qsc preservation was accepted.

Option 7, CI/tooling lane:

- Decision: Rejected.
- Reason: CI is not the transport blocker.

## Authorization decision

Primary classification selected:

`SSH_FORWARDING_OPERATOR_ACTION_AUTHORIZATION_READY`

Decision:

- NA-0516 / D410 is consumed.
- D409 residue cleanup is consumed.
- D410 transport failure is accepted as a forwarding-policy blocker.
- qsc-native forwarding-free path is reviewed and rejected for the current qsc surface.
- SSH forwarding policy is reviewed.
- qsl-server and qsl-attachments are deferred and remain out of scope.
- A separate dedicated forwarding key is selected as the safer default over changing the existing qslcodex operational key.
- Shape 1 remote reverse forwarding to a Build-local relay is selected as the first future model.
- Future forwarding must be loopback-only and constrained to one host/port, default `127.0.0.1:39176`.
- No remote action occurs in NA-0517.
- No SSH execution occurs in NA-0517.
- No authorized_keys mutation occurs in NA-0517.
- No key generation or key installation occurs in NA-0517.
- No SSH config or known_hosts mutation occurs in NA-0517.
- No qsc send/receive or remote E2EE occurs in NA-0517.
- No qsc source/test/fuzz/Cargo mutation occurs in NA-0517.
- No workflow/script/helper/dependency/corpus/vector/input/formal/refimpl/service/public/backup mutation occurs in NA-0517.
- No public-readiness claim is made. no production-readiness claim is made. no public-internet-readiness claim is made.
- No crypto-complete claim is made. no replay-proof claim is made. no downgrade-proof claim is made. no secret-material-complete claim is made. no side-channel-free claim is made. no vulnerability-free claim is made. no bug-free claim is made. no perfect-crypto claim is made.
- Exactly one READY successor remains mandatory.

## Selected NA-0518 successor

Selected successor:

`NA-0518 -- QSL Remote qsc E2EE SSH Forwarding Operator Action Authorization Plan`

Status: READY after NA-0517 closeout only.

Goals: G1, G2, G3, G4, G5.

Objective:

Authorize the exact operator-owned SSH forwarding remediation needed for the approved `inspiron` / `qslcodex` remote qsc E2EE test. The selected default is a separate dedicated forwarding key constrained to loopback-only remote reverse forwarding for a Build-local qsc relay, preserving no PTY, no agent/X11 forwarding, no sudo/admin, no backup exposure, no qwork/qsl-backup, no qsl-server/qsl-attachments use, and no public/production readiness claims.

## Future scope bundle

Allowed NA-0518 scope:

- Governance evidence/testplan paths for NA-0518.
- `DECISIONS.md`.
- `TRACEABILITY.md`.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.
- Read-only review of D410 transport failure evidence.
- Read-only review of operator-provided redacted SSH boundary proof if supplied.
- Exact operator manual-action checklist and proof checklist.
- Exact rejection conditions for private material, broad forwarding, PTY broadening, sudo/admin, backup exposure, qwork/qsl-backup exposure, and qsl-server/qsl-attachments use.

Forbidden NA-0518 scope:

- Codex running SSH.
- Codex editing authorized_keys.
- Codex generating or installing keys.
- Codex reading private keys.
- SSH config mutation by Codex.
- known_hosts mutation by Codex.
- Remote host mutation by Codex.
- Remote E2EE execution.
- qsc send/receive.
- qsl-server/qsl-attachments.
- Package installation.
- sudo/admin action.
- qwork/qstart/qresume.
- qsl-backup.
- no public-readiness claim and no production-readiness claim.

NA-0518 deliverables:

- Operator action authorization evidence.
- Testplan.
- Decision.
- TRACEABILITY update.
- Rolling journal update.
- Exact future proof outputs.
- Exact rejection conditions.
- Selected proof-review or E2EE retry successor.

NA-0518 acceptance criteria:

- D410 transport failure consumed.
- qsc-native forwarding-free path rejection consumed.
- Exact forwarding model selected.
- Separate dedicated forwarding key selected by default or an explicit fallback rationale recorded.
- PTY policy preserved or explicitly justified.
- No private material included.
- No remote command run by Codex.
- Exactly one READY item remains.

## Future validation / marker plan

Future NA-0518 markers:

- `NA0518_D410_TRANSPORT_FAILURE_CONSUMED_OK`
- `NA0518_QSC_NATIVE_FORWARDING_FREE_REVIEWED_OK`
- `NA0518_SSH_FORWARDING_MODEL_SELECTED_OK`
- `NA0518_DEDICATED_FORWARDING_KEY_SELECTED_OK`
- `NA0518_NO_PTY_BROADENING_OK`
- `NA0518_NO_AGENT_X11_FORWARDING_OK`
- `NA0518_OPERATOR_ACTION_ONLY_OK`
- `NA0518_NO_PRIVATE_KEY_MATERIAL_OK`
- `NA0518_NO_REMOTE_ACTION_BY_CODEX_OK`
- `NA0518_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0518_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0518_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0518_ONE_READY_INVARIANT_OK`

Future E2EE retry markers after a separate proof-review/implementation lane, not NA-0518:

- retained qsc hash/path/owner/help rechecked.
- transport authorization proof accepted.
- Build-to-Inspiron send/receive exact synthetic message match.
- Inspiron-to-Build reply exact synthetic message match.
- wrong-mailbox no-mutation proof.
- no-secret-output proof.
- local/remote cleanup proof.

## No remote action in NA-0517

Codex performed no remote action in NA-0517. Specifically, Codex did not run SSH, scp, sftp, rsync, remote commands, qsc send/receive, remote E2EE, package installation, sudo/admin action, key generation, key installation, SSH config mutation, known_hosts mutation, authorized_keys mutation, remote host mutation, qwork/qstart/qresume, qsl-backup, qsl-server, or qsl-attachments.

## No qsl-server / no qsl-attachments boundary

qsl-server and qsl-attachments remain protected architecture boundaries. They are not used, required, or authorized by this direct qsc transport remediation lane.

The selected NA-0518 successor must not use qsl-server or qsl-attachments. If a later program decision intentionally moves to service-backed transport, it must be a separate explicit architecture lane after direct qsc E2EE either succeeds or is intentionally abandoned.

## Public claim / website / external review boundary

This lane is transport authorization only. It does not establish remote E2EE success, public readiness, production readiness, public-internet readiness, external review completion, crypto completeness, replay proof, downgrade proof, secret-material completeness, side-channel freedom, vulnerability freedom, bug freedom, or perfect crypto.

No public docs, website, README, or START_HERE path is changed.

## Backup-impact statement

No qsl-backup command was run. No backup or restore was run. No backup path was mutated. The installed qsl-backup helper was read only; digest and source-list inclusion were checked without execution. The selected successor must preserve no backup exposure and no qwork/qsl-backup exposure.

## Rejected alternatives

- qsc-native forwarding-free retry: rejected because current qsc lacks an offline/artifact mailbox transport.
- Remote E2EE retry with existing settings: rejected because D410 proved both tested forwarding shapes blocked.
- qsl-server/qsl-attachments integration: rejected for this direct qsc sprint.
- Broad sshd_config or account-wide forwarding: rejected in favor of key-level constraints.
- Existing qslcodex operational key broadening: deferred as fallback only; dedicated forwarding key is selected.
- Public relay exposure: rejected.
- Custom proxy/helper transport: rejected as behavior drift outside the current qsc surface.
- Remote cleanup/remediation: rejected as primary successor because D410 cleanup completed.
- CI/tooling lane: rejected because CI is not the blocker.

## Stewardship and assurance review

Best-Known-Method Review:

- Current qsc direct E2EE tests use an HTTP relay endpoint. The least-drift remediation is to make the existing loopback relay endpoint reachable across hosts through a narrowly authorized SSH tunnel, not to invent a new transport.

Hostile Cryptographer Review:

- This lane proves no new cryptographic property. It only selects a transport-remediation authorization path. Future E2EE success remains unclaimed until send/receive/reply and wrong-mailbox proof pass.

Red-Team Review:

- The largest risk is accidental broad SSH enablement. The selected mitigation is a separate dedicated forwarding key, loopback-only host/port constraints, no PTY, no agent/X11 forwarding, no sudo/admin, redacted proof, and rejection of private material.

Production SRE Review:

- Shape 1 keeps the relay process local to Build and reduces remote cleanup. The remote host receives only a loopback listener through SSH while the tunnel is active.

Side-Channel Caveat:

- This lane does not prove side-channel freedom. Timing, traffic-shape, and metadata residuals remain future assurance work.

Formal-Model Mapping Residual:

- No formal model semantics change in this lane. The transport-remediation decision maps to G4 verification gating but not to protocol state-machine correctness.

External-Review Readiness:

- This lane is not external-review complete. It prepares a safer operator action path that may later produce bounded remote E2EE evidence.

Release-Claim Boundary:

- No release-readiness, public-readiness, production-readiness, or public-internet-readiness claim is made.

Assurance Gap Review Trigger:

- If future proof shows broad forwarding, PTY enablement, private material, sudo/admin exposure, backup/qwork/qsl-backup exposure, qsl-server/qsl-attachments dependency, or successful E2EE without wrong-mailbox no-mutation proof, the lane must stop and select remediation.

## Next recommendation

Close NA-0517 only after this authorization PR merges and post-merge public-safety is green inside the short attach/early-failure window. The closeout should restore exactly one READY item:

`NA-0518 -- QSL Remote qsc E2EE SSH Forwarding Operator Action Authorization Plan`

NA-0518 should authorize operator-only manual SSH forwarding remediation using a separate dedicated forwarding key and loopback-only remote reverse forwarding to a Build-local relay as the selected default.
