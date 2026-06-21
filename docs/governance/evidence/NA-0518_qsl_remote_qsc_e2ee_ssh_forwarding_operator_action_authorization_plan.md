Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-21

# NA-0518 QSL remote qsc E2EE SSH forwarding operator action authorization plan

## Executive summary

NA-0518 is an authorization-only governance/security lane. It consumes NA-0517 / D411 transport-remediation inheritance plus the operator-supplied PTY root-cause context, selects the exact future operator-owned SSH forwarding remediation, and restores no transport capability by itself.

Primary classification: `SSH_FORWARDING_DEDICATED_KEY_OPERATOR_ACTION_READY`.

Selected operator action: use a separate dedicated qslcodex forwarding key constrained to loopback-only TCP forwarding on `127.0.0.1:39176`, keep PTY disabled, keep agent forwarding disabled, keep X11 forwarding disabled, keep no sudo/admin, keep no backup exposure, keep no qwork/qsl-backup exposure, and keep qsl-server/qsl-attachments out of scope.

Selected future successor: `NA-0519 -- QSL Remote qsc E2EE SSH Forwarding Operator Action Proof Review Harness`.

Codex did not run SSH, scp, sftp, rsync, qsc send/receive, remote E2EE, key generation, key installation, authorized_keys mutation, SSH config mutation, known_hosts mutation, sshd_config mutation, remote host mutation, sudo/admin action, package installation, qwork/qstart/qresume, qsl-backup, qsl-server, or qsl-attachments in NA-0518.

Required marker summary:

- `NA0518_QWORK_PROOF_FILES_VERIFIED_OK`
- `NA0518_D411_INHERITANCE_CONSUMED_OK`
- `NA0518_OPERATOR_PTY_CONTEXT_CONSUMED_OK`
- `NA0518_PTY_BROADENING_REJECTED_OK`
- `NA0518_FORWARDING_KEY_OPTIONS_REVIEWED_OK`
- `NA0518_DEDICATED_FORWARDING_KEY_SELECTED_OK`
- `NA0518_AUTHORIZED_KEYS_TEMPLATE_SELECTED_OK`
- `NA0518_OPERATOR_ACTION_CHECKLIST_SELECTED_OK`
- `NA0518_FUTURE_PROOF_OUTPUTS_SELECTED_OK`
- `NA0518_PROOF_REJECTION_RULES_SELECTED_OK`
- `NA0518_QSL_SERVER_ATTACHMENTS_BOUNDARY_OK`
- `NA0518_NO_REMOTE_ACTION_BY_CODEX_OK`
- `NA0518_NO_SSH_EXECUTION_BY_CODEX_OK`
- `NA0518_NO_AUTHORIZED_KEYS_MUTATION_OK`
- `NA0518_NO_KEY_GENERATION_INSTALLATION_OK`
- `NA0518_NO_QSC_SOURCE_TEST_FUZZ_CARGO_MUTATION_OK`
- `NA0518_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0518_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0518_SSH_FORWARDING_DEDICATED_KEY_OPERATOR_ACTION_READY`
- `NA0518_SELECTED_NA0519_SUCCESSOR_OK`
- `NA0518_ONE_READY_INVARIANT_OK`

## Live NA-0518 scope

Allowed repository mutation paths for this authorization evidence PR:

- `docs/governance/evidence/NA-0518_qsl_remote_qsc_e2ee_ssh_forwarding_operator_action_authorization_plan.md`
- `tests/NA-0518_qsl_remote_qsc_e2ee_ssh_forwarding_operator_action_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

The live queue at startup had READY_COUNT 1 and sole READY item `NA-0518 -- QSL Remote qsc E2EE SSH Forwarding Operator Action Authorization Plan`.

This lane is authorization-only. It does not perform the operator action, does not retry remote E2EE, does not run SSH, does not create or install keys, and does not mutate qsc implementation code.

## qwork proof-file verification

Codex read and copied the qwork proof files without running qwork, qstart, or qresume:

- `/srv/qbuild/work/NA-0518/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0518/.qwork/startup.qsl-protocol.json`

Required proof fields passed:

- `startup_result=OK`
- `lane=NA-0518`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0518/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0518`
- `requested_lane_status=READY`

Freshness passed before fetch: proof HEAD and proof origin/main matched live refs at `741262357e3f`.

Disk proof passed before fetch: `/` usage was below the 95% stop threshold, and `/backup/qsl` was checked read-only below the stop threshold.

qsl-backup boundary was read-only: installed helper digest matched expected short digest `e9ecff3d22ed`, the Codex ops source inclusion count was exactly 1, and qsl-backup was not executed.

Current main health before mutation: `public-safety` completed success, `qsc-adversarial-smoke` completed success, `qsc-linux-full-suite` and `macos-qsc-full-serial` were accepted skipped by existing docs/governance-only cost-control policy, and no required red check was present in the retrieved check-run set.

## NA-0517 / D411 inheritance

NA-0517 completed. NA-0518 was restored READY by NA-0517 closeout. D-1023 and D-1024 exist once. D-1025 was absent at startup.

D411 selected `SSH_FORWARDING_OPERATOR_ACTION_AUTHORIZATION_READY`.

Inherited facts:

- D410 transport failure was consumed.
- qsc-native forwarding-free path was reviewed and rejected for the current retained qsc surface.
- qsl-server and qsl-attachments were deferred and remain out of scope.
- Selected forwarding model: Build-local qsc relay plus remote reverse forwarding.
- Selected default: separate dedicated forwarding key.
- Selected policy: no PTY, no agent forwarding, no X11 forwarding, no sudo/admin, no backup exposure, no qwork/qsl-backup exposure, loopback-only forwarding constraints.
- Shape 1 default endpoint: `127.0.0.1:39176`.
- NA-0517 performed no remote action, no SSH execution, no authorized_keys mutation, no key generation/installation, no qsc send/receive, no remote E2EE, and no public/production readiness claim expansion.

This lane accepts D411 as transport-remediation authorization evidence, not as proof that the remote E2EE protocol path works.

## Operator PTY root-cause context

Operator-supplied context, not Codex-executed proof:

- qslcodex can allocate PTYs locally on Inspiron.
- SSH PTY failure is caused by the qslcodex authorized_keys line beginning with `restrict`.
- `restrict` disables PTY allocation unless `pty` is explicitly re-enabled.
- No changes were made by the operator during that investigation.
- A broad shell convenience change would be `restrict,pty`, but this project does not need an interactive PTY for the QSL remote qsc lanes.

NA-0518 conclusion:

- PTY remains disabled for the QSL project key by default.
- `pty` is not authorized for the E2EE forwarding remediation.
- The future operator action addresses TCP forwarding only.
- If an interactive shell is desired for separate operator convenience, it must be handled outside this Codex remote E2EE key path or through a separate authorization lane.

## Forwarding key strategy

Default future strategy: separate dedicated forwarding key for qslcodex.

Recommended operator-owned local private key path: `~/.ssh/qslcodex_forward_ed25519`.

Recommended public key comment: `qsl-inspiron-qslcodex-forward-<date>`.

Codex must never generate, read, receive, or install the private key.

Reasons for the dedicated key default:

- It reduces blast radius by keeping forwarding authorization off the existing operational qslcodex key.
- It keeps the existing no-forwarding operational key intact.
- It allows clear proof, cleanup, and revocation.
- It avoids introducing PTY to the existing key.
- It supports temporary sprint-scoped authorization that can be removed after remote testing.

Alternative existing-key update remains deferred. It may be considered only if the operator explicitly rejects a separate key later, and proof must show no PTY, no agent forwarding, no X11 forwarding, no broad shell behavior, no sudo/admin, no qwork/qsl-backup exposure, no backup exposure, and no unrelated keys exposed.

## authorized_keys option template

Preferred dedicated forwarding-key line template:

```text
restrict,port-forwarding,permitlisten="127.0.0.1:39176",permitopen="127.0.0.1:39176",command="/bin/false" ssh-ed25519 <PUBLIC_KEY> qsl-inspiron-qslcodex-forward-<date>
```

Requirements:

- `restrict` stays present.
- `port-forwarding` is explicitly enabled.
- `permitlisten` is constrained to loopback and selected port.
- `permitopen` is constrained to loopback and selected port.
- `command="/bin/false"` or equivalent no-shell forced command is included if compatible with future port forwarding use.
- `pty` is absent.
- Agent forwarding is absent.
- X11 forwarding is absent.
- Broad unqualified forwarding is absent.
- Unrelated keys are not exposed.
- Private key material is absent.
- Selected port default is `39176`.

OpenSSH option compatibility must be validated by the operator's manual proof. If `command="/bin/false"` prevents the required `-N -R` forwarding despite no session request, the operator must stop and report; it must not be silently removed without a new authorization.

If a different port is needed, future proof must stop and request alternate port authorization. A later E2EE retry must not choose an ad hoc port.

## Future operator action checklist

Authorized operator-owned manual actions after NA-0518 closeout:

1. Generate a dedicated forwarding key locally on Build with recommended name `~/.ssh/qslcodex_forward_ed25519` and comment `qsl-inspiron-qslcodex-forward-<date>`. Passphrase policy is operator choice, but future noninteractive use requires agent setup or no passphrase. Private key material must never be pasted to Codex.
2. Install only the public key into qslcodex authorized_keys on Inspiron with the selected key options.
3. Keep the existing operational qslcodex key unchanged unless the operator explicitly chooses existing-key update later.
4. Do not enable PTY.
5. Do not add sudo/admin membership.
6. Do not expose `/backup/qsl`.
7. Do not install qwork or qsl-backup remotely.
8. Do not alter sshd_config unless key-level options are proven insufficient and a later directive authorizes broader remediation.
9. Record cleanup/revocation commands: remove the forwarding-key authorized_keys line, delete local private/public forwarding key when no longer needed if desired, and verify forwarding no longer works when revoked.

## Future proof outputs

Future operator proof for NA-0519 must include only redacted/safe summaries:

- Key fingerprint for the forwarding key.
- Public key comment.
- Whether dedicated forwarding key or existing key was used.
- Relevant authorized_keys option summary for only that key.
- Confirmation `restrict` present.
- Confirmation `port-forwarding` present.
- Confirmation `permitlisten=127.0.0.1:39176` or approved equivalent present.
- Confirmation `permitopen=127.0.0.1:39176` or approved equivalent present.
- Confirmation `pty` absent.
- Confirmation agent forwarding absent.
- Confirmation X11 forwarding absent.
- Confirmation forced no-shell command present or compatibility issue reported.
- Confirmation qslcodex still non-sudo.
- Confirmation no backup exposure.
- Confirmation qwork absent.
- Confirmation qsl-backup absent.
- Confirmation no production data.
- Cleanup/revocation command documented.

Forbidden proof material:

- Private key material.
- Passphrase material.
- Full authorized_keys with unrelated keys.
- known_hosts dump.
- Unrelated host/IP inventory.
- Passwords, tokens, credentials, backup material, production endpoints, or personal data.

## Future forwarding proof / E2EE retry model

Recommended sequence:

1. NA-0519 proof review consumes operator-provided redacted proof only. Codex runs no SSH unless a later successor explicitly authorizes it.
2. NA-0520 forwarding capability probe uses the dedicated forwarding key, starts a Build-local relay or simple test listener if authorized, and tests only loopback reverse forwarding.
3. NA-0521 Build-to-Inspiron E2EE retry uses retained qsc, the proven forwarding path, synthetic messages only, cleanup/retention proof, and no qsl-server/qsl-attachments.

If time pressure justifies combining NA-0520 and NA-0521 later, the future directive must explicitly authorize the combination and preserve stop gates.

## Option review

Option 1, dedicated forwarding key operator action: selected. Risk reduced: least blast radius and clean revocation. Evidence gap addressed: exact key-level forwarding proof. Implementation feasibility: high for operator-owned manual action. Scope risk: low. Remote mutation risk: limited to one public-key line by operator only. Secret/key risk: private key remains outside Codex. Public claim risk: contained by no public-readiness claim and no production-readiness claim. Likely future allowed paths: NA-0519 proof-review evidence/testplan, DECISIONS, TRACEABILITY, journal. Likely future forbidden paths: SSH by Codex, qsc send/receive, qsl-server/qsl-attachments, private material. P0 risk: private key exposure. P1 risk: overbroad forwarding. P2 risk: forced-command compatibility.

Option 2, modify existing operational qslcodex key: deferred unless operator rejects a separate key. Risk reduced: avoids adding a new key but increases blast radius. Evidence gap addressed: must prove existing key constraints remain narrow. Implementation feasibility: medium. Scope risk: medium. Remote mutation risk: existing key line changes. Secret/key risk: unrelated key exposure if proof is overbroad. Public claim risk: contained by no public-readiness claim and no production-readiness claim. Likely future allowed paths: proof-review docs only. Likely future forbidden paths: PTY, agent/X11, broad command shell, unrelated keys. P0 risk: broadening existing operational access. P1 risk: difficult revocation. P2 risk: proof ambiguity.

Option 3, enable PTY on existing key with `restrict,pty`: rejected for the QSL E2EE path. Risk reduced: none for transport. Evidence gap addressed: none for qsc relay reachability. Implementation feasibility: high but not needed. Scope risk: high because it broadens interactive access. Remote mutation risk: existing key broadening. Secret/key risk: increases shell access surface. Public claim risk: contained by rejection. Likely future allowed paths: none in this lane. Likely future forbidden paths: PTY enablement for E2EE remediation. P0 risk: unnecessary interactive access. P1 risk: operational confusion. P2 risk: future proof drift.

Option 4, sshd_config or account-wide forwarding change: rejected unless key-level options fail and a later directive authorizes broader host change. Risk reduced: could repair forwarding globally, but too broad. Evidence gap addressed: only after key-level proof fails. Implementation feasibility: operator/admin dependent. Scope risk: high. Remote mutation risk: daemon/account-wide. Secret/key risk: broader account exposure. Public claim risk: contained by deferral. Likely future allowed paths: separate remediation authorization only. Likely future forbidden paths: sshd_config mutation in NA-0518/NA-0519. P0 risk: account-wide forwarding exposure. P1 risk: host policy drift. P2 risk: cleanup complexity.

Option 5, qsc-native forwarding-free retry: rejected/deferred based on D411. Risk reduced: would avoid SSH forwarding, but current qsc surface lacks the path. Evidence gap addressed: none without implementation changes. Implementation feasibility: low in this lane. Scope risk: high because qsc source/test/Cargo mutation is out of scope. Remote mutation risk: none if implemented later, but not available now. Secret/key risk: lower if built later. Public claim risk: contained by no public-readiness claim and no production-readiness claim. Likely future allowed paths: separate qsc implementation lane only. Likely future forbidden paths: NA-0518 qsc mutation. P0 risk: false E2EE claim. P1 risk: ad hoc helper/proxy. P2 risk: schedule delay.

Option 6, qsl-server/qsl-attachments integration: deferred. Risk reduced: possible future service-backed path, not part of direct qsc sprint. Evidence gap addressed: service architecture is separate. Implementation feasibility: out of current scope. Scope risk: high. Remote mutation risk: service/deployment dependent. Secret/key risk: service trust boundary expansion. Public claim risk: contained by no public-readiness claim and no production-readiness claim. Likely future allowed paths: separate architecture lane. Likely future forbidden paths: qsl-server/qsl-attachments use in NA-0518/NA-0519. P0 risk: premature service coupling. P1 risk: public exposure. P2 risk: test complexity.

Option 7, defer all remote E2EE: rejected unless operator cannot safely perform forwarding action. Risk reduced: avoids SSH changes, but blocks the approved remote sprint. Evidence gap addressed: none. Implementation feasibility: high. Scope risk: low. Remote mutation risk: none. Secret/key risk: none. Public claim risk: contained by deferral. Likely future allowed paths: blocker closeout only. Likely future forbidden paths: remote E2EE execution without proof. P0 risk: no progress on G4 remote evidence. P1 risk: stale retained qsc. P2 risk: lost context.

Option 8, cleanup retained qsc and abandon remote sprint: rejected unless remote host becomes unavailable or proof boundaries fail. Risk reduced: removes retained remote artifact, but abandons current evidence path. Evidence gap addressed: cleanup only. Implementation feasibility: operator/SSH dependent and not authorized here. Scope risk: high for NA-0518 because remote action is forbidden. Remote mutation risk: cleanup action. Secret/key risk: low. Public claim risk: contained by no public-readiness claim and no production-readiness claim. Likely future allowed paths: separate cleanup lane. Likely future forbidden paths: Codex remote cleanup in NA-0518. P0 risk: unauthorized remote mutation. P1 risk: losing test asset prematurely. P2 risk: queue churn.

## Hostile Cryptographer Review

Enabling narrow SSH forwarding proves only transport reachability. It does not prove protocol correctness, key schedule correctness, E2EE message authenticity, replay resistance, downgrade resistance, side-channel safety, or production readiness.

No PTY remains the default because qsc needs TCP forwarding, not interactive shell allocation. PTY broadening adds an unnecessary execution surface and does not reduce the transport evidence gap.

A separate key reduces blast radius by isolating forwarding authority from the existing operational key and making revocation auditable.

Before any E2EE retry, future proof must establish loopback-only permitlisten/permitopen constraints, PTY absent, agent/X11 absent, no-shell forced command status, no sudo/admin, no backup exposure, no qwork/qsl-backup exposure, and redacted proof only.

No public-readiness claim is made. no production-readiness claim is made. no public-internet-readiness claim is made. no external-review-complete claim is made. no crypto-complete claim is made. no replay-proof claim is made. no downgrade-proof claim is made. no secret-material-complete claim is made. no side-channel-free claim is made. no vulnerability-free claim is made. no bug-free claim is made. no perfect-crypto claim is made.

## Red-Team Review

If authorized_keys proof is overbroad, NA-0519 must reject it and select remediation.

If PTY is enabled accidentally, NA-0519 must reject it for the QSL E2EE forwarding path.

If agent or X11 forwarding is enabled, NA-0519 must reject it because those features are unrelated to qsc relay transport and broaden the session surface.

If permitopen or permitlisten are missing or too broad, NA-0519 must reject the proof and require a narrower key-level constraint.

If `command="/bin/false"` breaks forwarding, the operator must stop and report a compatibility issue; a later authorization must decide whether an equivalent no-shell shape is acceptable.

If a forwarded port is exposed beyond loopback, NA-0519 must reject the proof.

If qslcodex gains sudo or backup access, NA-0519 must reject the proof and select remediation.

Cleanup/revocation proof must document how to remove the forwarding-key authorized_keys line, optionally delete local forwarding key files, and verify forwarding no longer works after revocation.

## Production SRE Review

A dedicated forwarding key is operationally reasonable for a short remote sprint because it is narrow, auditable, and revocable.

Before Codex can use it, proof must show the selected key fingerprint/comment, the redacted key-option summary, loopback-only forwarding constraints, no PTY, no agent/X11 forwarding, no sudo/admin, no backup exposure, no qwork/qsl-backup exposure, no production data, and a cleanup/revocation plan.

The key should be revoked by removing the dedicated authorized_keys line and deleting local key files when the sprint no longer needs them, if desired.

sshd_config and account-wide changes are deferred because key-level options are narrower and preserve host policy.

qsl-server and qsl-attachments remain out of the direct qsc path because D411 selected retained qsc plus SSH forwarding, not a service integration or attachment architecture lane.

This is not production/public readiness. It authorizes a bounded test transport proof path only.

## Release-Claim Boundary Review

No public-readiness claim is made. no production-readiness claim is made. no public-internet-readiness claim is made. no external-review-complete claim is made. no crypto-complete claim is made. no replay-proof claim is made. no downgrade-proof claim is made. no secret-material-complete claim is made. no side-channel-free claim is made. no vulnerability-free claim is made. no bug-free claim is made. no perfect-crypto claim is made.

## Authorization decision

Primary classification: `SSH_FORWARDING_DEDICATED_KEY_OPERATOR_ACTION_READY`.

Decision:

- NA-0517 / D411 is consumed.
- Operator PTY root-cause context is consumed as operator-supplied context only.
- PTY broadening is rejected/not selected for the QSL E2EE forwarding path.
- Dedicated forwarding key strategy is selected.
- authorized_keys template is selected.
- Loopback-only forwarding constraints are selected.
- Operator action checklist is selected.
- Future proof outputs and forbidden proof material are selected.
- Proof rejection rules are selected.
- qsl-server/qsl-attachments boundary is preserved.
- No remote action occurs in NA-0518.
- No key generation or installation occurs in NA-0518.
- No authorized_keys mutation occurs in NA-0518.
- No SSH config, known_hosts, sshd_config, or remote host mutation occurs in NA-0518.
- No qsc send/receive or remote E2EE occurs in NA-0518.
- No public claim expansion occurs.
- Exactly one READY successor remains mandatory.

## Selected NA-0519 successor

`NA-0519 -- QSL Remote qsc E2EE SSH Forwarding Operator Action Proof Review Harness`

Status after future closeout: READY.

Goals: G1, G2, G3, G4, G5.

This successor is proof-review only. It consumes operator-provided redacted proof and either accepts the forwarding key setup as input to a later forwarding probe or rejects unsafe proof. It does not itself authorize Codex SSH execution unless a later directive explicitly says so.

## Future scope bundle

### NA-0519 -- QSL Remote qsc E2EE SSH Forwarding Operator Action Proof Review Harness

Status: READY

Goals: G1, G2, G3, G4, G5

Objective:
Review operator-provided redacted proof that a dedicated qslcodex SSH forwarding key has been installed for the approved `inspiron` remote account with loopback-only forwarding constraints, PTY disabled, agent/X11 forwarding disabled, no sudo/admin, no backup exposure, no qwork/qsl-backup, no qsl-server/qsl-attachments, and cleanup/revocation documented, before any Codex SSH forwarding or remote E2EE retry is authorized.

Allowed scope:

- `docs/governance/evidence/NA-0519_qsl_remote_qsc_e2ee_ssh_forwarding_operator_action_proof_review.md`
- `tests/NA-0519_qsl_remote_qsc_e2ee_ssh_forwarding_operator_action_proof_review_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- Read-only review of operator-provided redacted proof pasted by the user.
- Read-only review of prior NA-0517/NA-0518 evidence.

Forbidden scope:

- Codex running SSH.
- Codex editing authorized_keys.
- Codex generating/installing keys.
- Codex reading private keys.
- Local SSH config mutation by Codex.
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

Deliverables:

- Proof-review evidence.
- Testplan.
- Decision.
- TRACEABILITY update.
- Rolling journal update.
- Selected forwarding capability probe or remote E2EE retry successor.

Acceptance criteria:

- Operator proof consumed.
- Private key/passphrase/token absent.
- Relevant key fingerprint recorded.
- No unrelated authorized_keys material included.
- `restrict` and port-forwarding constraints reviewed.
- PTY remains disabled.
- Agent/X11 forwarding disabled.
- Loopback-only permitlisten/permitopen or equivalent reviewed.
- No sudo/admin proof reviewed.
- No backup/qwork/qsl-backup exposure proof reviewed.
- Cleanup/revocation plan reviewed.
- Exactly one READY item remains.

## Future validation / marker plan

Future NA-0519 markers:

- `NA0519_OPERATOR_FORWARDING_PROOF_CONSUMED_OK`
- `NA0519_REDACTED_PROOF_ONLY_OK`
- `NA0519_DEDICATED_FORWARDING_KEY_REVIEWED_OK`
- `NA0519_PRIVATE_KEY_ABSENT_OK`
- `NA0519_PASSPHRASE_ABSENT_OK`
- `NA0519_RESTRICT_PRESENT_OK`
- `NA0519_PORT_FORWARDING_PRESENT_OK`
- `NA0519_PTY_DISABLED_OK`
- `NA0519_AGENT_X11_DISABLED_OK`
- `NA0519_LOOPBACK_FORWARDING_CONSTRAINT_REVIEWED_OK`
- `NA0519_NO_SUDO_ADMIN_PROOF_REVIEWED_OK`
- `NA0519_NO_BACKUP_EXPOSURE_PROOF_REVIEWED_OK`
- `NA0519_NO_QWORK_QSLBACKUP_PROOF_REVIEWED_OK`
- `NA0519_CLEANUP_REVOCATION_PLAN_REVIEWED_OK`
- `NA0519_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0519_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0519_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0519_ONE_READY_INVARIANT_OK`

## No remote action in NA-0518

Codex performed no remote action in NA-0518. Specifically, Codex did not run SSH, scp, sftp, rsync, remote commands, qsc send/receive, remote E2EE, key generation, key installation, SSH config mutation, known_hosts mutation, sshd_config mutation, authorized_keys mutation, remote host mutation, qwork/qstart/qresume, qsl-backup, qsl-server, or qsl-attachments.

## No PTY broadening boundary

The operator PTY root-cause context explains why interactive SSH PTY allocation failed, but it does not create a QSL need for PTY. NA-0518 authorizes TCP forwarding remediation only and rejects PTY enablement for this path.

## No qsl-server / no qsl-attachments boundary

qsl-server and qsl-attachments remain protected architecture boundaries. They are not used, required, or authorized by NA-0518.

If a later program decision intentionally moves to service-backed transport, it must be a separate explicit architecture lane after direct qsc E2EE either succeeds or is intentionally abandoned.

## Public claim / website / external review boundary

NA-0518 changes no public docs, website, README, START_HERE, external-review process, or public-facing claim surface.

No public-readiness claim is made. no production-readiness claim is made. no public-internet-readiness claim is made. no external-review-complete claim is made. no crypto-complete claim is made. no replay-proof claim is made. no downgrade-proof claim is made. no secret-material-complete claim is made. no side-channel-free claim is made. no vulnerability-free claim is made. no bug-free claim is made. no perfect-crypto claim is made.

## Backup-impact statement

No backup, restore, qsl-backup, backup status, backup plan, rollback subtree, `/backup/qsl`, or backup-private-material mutation occurs in NA-0518. The qsl-backup helper was inspected read-only for digest and source-list boundary proof only.

## Rejected alternatives

- PTY enablement on the existing key is rejected for this E2EE path.
- sshd_config/account-wide forwarding is rejected unless key-level options fail and a later directive authorizes broader remediation.
- qsc-native forwarding-free retry is rejected/deferred for the current retained qsc surface.
- qsl-server/qsl-attachments integration is deferred.
- Full remote E2EE deferral is rejected while a narrow operator action path remains available.
- Cleanup retained qsc and abandon remote sprint is rejected unless remote host availability or proof safety fails later.

## Next recommendation

Merge NA-0518 authorization evidence after required validation and PR checks pass. If post-merge public-safety is green inside the short attach/early-failure window, perform the separate closeout to mark NA-0518 DONE and restore NA-0519 READY. If not, stop and hand off with NA-0518 still READY.

## Level-1 stewardship and D328 assurance requirements

Best-Known-Method Review: key-level, loopback-only, dedicated-key forwarding is the least broad currently available remediation that preserves D411's direct qsc sprint path.

Hostile Cryptographer Review: included above and keeps transport proof separate from protocol correctness.

Red-Team Review: included above and defines rejection conditions for overbroad key options, PTY, agent/X11, public exposure, sudo/admin, backup exposure, and forced-command compatibility failure.

Production SRE Review: included above and treats the key as temporary, auditable, and revocable.

Side-Channel Caveat: NA-0518 does not evaluate timing, traffic-shape, endpoint metadata, or side-channel behavior; no side-channel-free claim is made.

Formal-Model Mapping Residual: no new protocol state-machine, key schedule, negotiation, or wire behavior is introduced, so formal models are unchanged; remote transport evidence remains outside formal protocol proof.

External-Review Readiness: this lane does not complete external review and does not prepare a public claim package.

Release-Claim Boundary: no public-readiness claim is made. no production-readiness claim is made. no public-internet-readiness claim is made.

Assurance Gap Review Trigger: if future NA-0519 proof shows overbroad forwarding, PTY enablement, agent/X11 enablement, private material, sudo/admin, backup/qwork/qsl-backup exposure, qsl-server/qsl-attachments dependency, or ad hoc port drift, the lane must stop and select remediation.
