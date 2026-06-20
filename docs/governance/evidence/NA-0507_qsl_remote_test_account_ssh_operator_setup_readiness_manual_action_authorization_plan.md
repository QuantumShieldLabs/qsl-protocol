Status: Supporting
Owner: QSL Governance / Core Assurance
Last-Updated: 2026-06-20

# NA-0507 Remote Test Account SSH Operator Setup Readiness Manual Action Authorization Plan

## Executive summary

NA-0507 consumes NA-0506/D394 inheritance, reviews the NA-0506 operator
runbook, and selects the operator-only manual setup authorization path for a
future least-privilege remote test account and SSH boundary.

Primary classification:

- `REMOTE_OPERATOR_MANUAL_SETUP_AUTHORIZATION_READY`

Selected successor:

- `NA-0508 -- QSL Remote Test Account / SSH Operator Manual Setup Proof Review Harness`

This lane performs no remote action. Codex does not run SSH, scp, sftp, rsync,
ssh-keygen, ssh-keyscan, sudo, qwork, qstart, qresume, or qsl-backup. Codex
does not create accounts, generate keys, install keys, mutate local SSH config,
mutate system SSH config, mutate known_hosts, mutate authorized_keys, mutate a
remote host, or run remote tests.

## Live NA-0507 scope

Allowed mutation paths for this evidence PR:

- `docs/governance/evidence/NA-0507_qsl_remote_test_account_ssh_operator_setup_readiness_manual_action_authorization_plan.md`
- `tests/NA-0507_qsl_remote_test_account_ssh_operator_setup_readiness_manual_action_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Protected boundaries:

- no remote setup.
- no SSH execution by Codex.
- no key generation or key installation by Codex.
- no local or system SSH config mutation by Codex.
- no known_hosts or authorized_keys mutation by Codex.
- no remote host mutation by Codex.
- no qsc source/test/fuzz/Cargo mutation.
- no workflow/script/helper/dependency mutation.
- no corpus/vector/input mutation.
- no formal/refimpl/service/public/backup mutation.
- exactly one READY item remains mandatory.

## qwork proof-file verification

Codex did not run qwork, qstart, or qresume.

Read and copied proof files:

- `/srv/qbuild/work/NA-0507/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0507/.qwork/startup.qsl-protocol.json`

Verified fields:

- `startup_result=OK`
- `lane=NA-0507`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0507/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0507`
- `requested_lane_status=READY`

Freshness:

- qwork proof HEAD matched live pre-fetch HEAD.
- qwork proof origin/main matched live pre-fetch origin/main.
- fetch occurred only after proof/live match and disk proof below the 95% stop
  threshold.

## NA-0506 / D394 inheritance

Consumed:

- D394 response:
  `/home/victor/work/qsl/codex/responses/NA0506_20260620T152835Z_D394.md`
- NA-0506 runbook:
  `docs/governance/evidence/NA-0506_qsl_remote_test_account_ssh_operator_setup_runbook.md`
- NA-0506 proof checklist/testplan:
  `tests/NA-0506_qsl_remote_test_account_ssh_operator_setup_testplan.md`
- D-1001 and D-1002 in `DECISIONS.md`
- NA-0507 READY block in `NEXT_ACTIONS.md`

Inherited facts:

- NA-0506 completed.
- NA-0507 was restored READY.
- the runbook path and proof checklist path are in-tree.
- the remote account model is documented.
- the SSH key model is documented.
- the host alias model is documented.
- the default alias is `qsl-remote-test`.
- optional alias `remote` is allowed only after collision check and explicit
  operator approval.
- no remote action occurred.
- no SSH execution occurred.
- no account creation occurred.
- no SSH key generation or installation occurred.
- no SSH config mutation occurred.
- no remote host mutation occurred.
- no qwork or qsl-backup remote execution occurred.
- no public-readiness claim is made.
- no production-readiness claim is made.
- no crypto-complete claim is made.
- no replay-proof claim is made.
- no downgrade-proof claim is made.
- no secret-material-complete claim is made.

Selected NA-0507 purpose:

- authorize exact operator-owned manual setup steps and exact redacted proof
  outputs for a later Director-led manual setup sequence.

## Runbook review

Reviewed:

- `docs/governance/evidence/NA-0506_qsl_remote_test_account_ssh_operator_setup_runbook.md`
- `tests/NA-0506_qsl_remote_test_account_ssh_operator_setup_testplan.md`

Adequacy findings:

- account model: present.
- SSH key model: present.
- host alias model: present.
- future setup checklist: present.
- proof checklist: present.
- cleanup/revocation checklist: present.
- forbidden action list: present.
- paste-back boundary: present.
- claim boundary: present.

Required NA-0506 markers were present:

- `NA0506_REMOTE_BOUNDARY_SCOPE_CONSUMED_OK`
- `NA0506_OPERATOR_RUNBOOK_IMPLEMENTED_OK`
- `NA0506_REMOTE_ACCOUNT_MODEL_DOCUMENTED_OK`
- `NA0506_SSH_KEY_MODEL_DOCUMENTED_OK`
- `NA0506_HOST_ALIAS_MODEL_DOCUMENTED_OK`
- `NA0506_NO_REMOTE_ACTION_OK`
- `NA0506_NO_SSH_KEY_GENERATION_OK`
- `NA0506_NO_SSH_CONFIG_MUTATION_OK`
- `NA0506_NO_REMOTE_HOST_MUTATION_OK`
- `NA0506_NO_SUDO_ADMIN_SCOPE_OK`
- `NA0506_NO_BACKUP_EXPOSURE_OK`
- `NA0506_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0506_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0506_ONE_READY_INVARIANT_OK`

Runbook secret-boundary findings:

- no private key block was present.
- no passphrase, token, real production endpoint, or real operator secret was
  identified in the runbook/testplan review.
- the runbook does not instruct Codex to run SSH.
- the runbook does not instruct Codex to generate keys.
- the runbook does not instruct Codex to mutate remote hosts.

The runbook is adequate for Option 1 manual setup authorization and proof
collection handoff.

## Manual action authorization checklist

The following checklist is authorized for a future operator-owned manual setup
sequence after NA-0507 closeout. Codex must not execute these steps.

1. Confirm the target host is non-production test infrastructure and contains
   no production or user data.
2. Confirm target-host backup boundaries and confirm no `/backup/qsl` exposure
   to the future test user.
3. Choose the remote account placeholder `<qsl_remote_user>`.
4. Create or select a dedicated non-root, no-sudo project test user.
5. Create or verify the dedicated remote work directory placeholder
   `~/qsl-remote-test/`.
6. Generate a dedicated per-project SSH key as the operator outside this repo
   and outside Codex; retain only public-key fingerprint proof for paste-back.
7. Install only the dedicated public key for `<qsl_remote_user>`.
8. Confirm key-only login if feasible.
9. Choose the default local alias `qsl-remote-test`.
10. If the operator wants `ssh remote`, first prove no existing `remote` alias
    collision and explicitly approve that alias.
11. Add any local SSH config entry only as the operator.
12. Confirm host-key trust manually and out of band.
13. Confirm `<qsl_remote_user>` has no sudo/admin capability.
14. Confirm `<qsl_remote_user>` has no access to `/backup/qsl`.
15. Confirm the host and work root contain no production data.
16. Confirm qwork, qstart, qresume, and qsl-backup are not run on the remote
    host.
17. Collect only the approved redacted proof outputs.
18. Record cleanup and revocation steps before remote tests are considered.

## Approved proof outputs

The operator may paste back only redacted, non-secret proof outputs:

- redacted username and selected host alias.
- `ssh -G <alias>` relevant non-secret fields, redacted to omit sensitive
  hostnames, private paths, and unrelated config.
- proof that `<qsl_remote_user>` has no sudo/admin capability, redacted.
- proof of `~/qsl-remote-test/` existence/ownership, redacted.
- proof of key-only login if feasible, redacted.
- proof that `<qsl_remote_user>` cannot access `/backup/qsl`, redacted.
- public key fingerprint only.
- host key fingerprint confirmation only, not private host material.
- cleanup/revocation checklist status.

Proof review must be redaction-first. If the operator is unsure whether an
output is safe to paste, stop and ask the Director before paste-back.

## Forbidden paste-back material

The operator must never paste:

- private SSH keys.
- SSH key passphrases.
- API tokens.
- session tokens.
- passwords.
- credentials.
- full private hostname/IP if sensitive.
- production endpoints.
- personal data.
- backup material.
- authorized_keys content if it exposes unrelated keys.
- known_hosts content if it exposes unrelated infrastructure.
- private host material.
- long secret-like dumps.

If any forbidden material is pasted, the lane family must stop for Director
triage and cleanup/revocation review.

## Alias policy

Default alias:

- `qsl-remote-test`

Optional alias:

- `remote`, only after an operator collision check proves no existing local
  alias collision and the operator explicitly approves the convenience alias.

Policy:

- NA-0507 does not perform the collision check.
- Codex does not read or mutate local SSH config for alias setup.
- Future proof review may consume redacted alias proof.
- If alias targeting is ambiguous, stop before any remote capability probe.

## Stop-and-ask-Director gates

Stop and ask the Director if any of the following occurs:

- the target host might be production or contain production/user data.
- the target host might expose `/backup/qsl` or backup material.
- the account is root, primary personal, shared, or sudo-capable.
- private key, passphrase, token, password, credential, personal data, or
  production endpoint material appears in any proof.
- the operator cannot verify host-key identity out of band.
- the operator wants alias `remote` but cannot prove no collision.
- known_hosts or authorized_keys output would reveal unrelated infrastructure.
- key-only login cannot be confirmed and the reason is unclear.
- qwork, qstart, qresume, or qsl-backup would need to run on the remote host.
- cleanup/revocation proof cannot be produced.
- any proof would expand public/security/completion claims.

## Cleanup/revocation requirements

Before remote tests are authorized in a later lane, proof must state how the
operator will retire the setup.

Required cleanup/revocation proof:

- whether the dedicated public key was removed or remains temporarily
  authorized for a bounded follow-on lane.
- whether `<qsl_remote_user>` was disabled, removed, or remains temporarily
  enabled for a bounded follow-on lane.
- whether `~/qsl-remote-test/` was removed, retained, or cleaned of synthetic
  artifacts.
- whether any synthetic artifacts are retained for evidence.
- revocation trigger proof for suspected private key disclosure, host-key
  mismatch, sudo/admin exposure, backup exposure, production data exposure, or
  alias-targeting ambiguity.

## Option review

| Option | Disposition | Risk reduced | Evidence gap addressed | Feasibility | Scope risk | Secret/key risk | Operator burden | Public-claim risk | Likely future paths | Likely forbidden paths | P0/P1/P2 risks |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1. Operator Manual Setup Authorization and Proof Collection Handoff | Select | account/key/alias ambiguity | exact operator steps and proof outputs | high | low | low if redaction holds | moderate | low | NA-0508 proof review evidence/testplan plus governance files | SSH execution by Codex, key generation, config mutation | P0 secret paste; P1 sudo/backup exposure; P2 alias confusion |
| 2. Runbook Hardening Before Manual Action | Reject | would reduce runbook gaps | no material gap found | high | low | low | low | low | a hardening evidence/testplan lane if needed | remote setup and SSH actions | P0 unsafe omission if review were wrong; P1 delay; P2 duplicate docs |
| 3. Local SSH Alias Collision Check Authorization | Defer | alias targeting ambiguity | only alias proof | medium | medium | medium if config exposed | low | low | future operator proof review may inspect redacted collision proof | Codex SSH config reads/mutations | P0 config leak; P1 wrong target; P2 convenience drift |
| 4. Remote Capability Probe Authorization | Defer | remote reachability ambiguity | capability proof | medium after setup proof | high now | medium | low after setup | medium | later capability-probe authorization evidence | remote tests before proof | P0 unauthorized remote action; P1 host mutation; P2 premature claim |
| 5. Remote Client-to-Client E2E Authorization | Defer | two-host behavioral gap | remote E2E proof | low now | high | medium | high | medium | later remote E2E scope/implementation | remote E2E before setup/probe | P0 unsafe host; P1 flaky remote state; P2 overclaim |
| 6. Same-host E2E negative expansion | Defer | local negative coverage | not the current blocker | high | low | low | low | low | qsc test plus governance paths if remote lane blocks | remote setup paths | P0 none identified; P1 priority drift; P2 less direct evidence |
| 7. CI/tooling lane | Reject | CI ambiguity | no tooling blocker found | high | medium | low | low | low | CI docs/governance only if blocker appears | workflow/script mutation now | P0 enforcement weakening; P1 churn; P2 delay |

## Hostile Cryptographer Review

- Private key custody: the plan prevents Codex from seeing private keys by
  assigning key generation, private key storage, public key installation, and
  passphrase custody to the operator outside Codex.
- qwork/qsl-backup boundary: the plan keeps qwork, qstart, qresume, and
  qsl-backup off the remote host and requires proof if that boundary is later
  reviewed.
- Local versus remote proof: redacted proof is evaluated separately from any
  later remote capability probe, preserving separation between setup evidence
  and remote capability.
- Public claim boundary: remote setup proof is not public-readiness evidence.
- Proof sufficiency: approved proof outputs cover alias, account, no sudo,
  no backup access, work root, key-only login if feasible, fingerprints, and
  cleanup/revocation while excluding private material.

Residual:

- this is operational evidence only. It is not formal protocol proof and makes
  no side-channel-free claim.

## Red-Team Review

- If the operator pastes private key material: stop, revoke the key, remove or
  rotate installed public key material, document the incident, and require
  Director approval before resuming.
- If the remote test account has sudo/admin capability: stop and reject remote
  test authorization until the account is replaced or privileges are removed
  and proven.
- If the remote host contains production data: stop and reject this setup path.
- If alias `remote` already points somewhere sensitive: reject the alias and
  use only `qsl-remote-test`, or stop for Director approval.
- If known_hosts or authorized_keys output leaks unrelated infrastructure: do
  not paste it; replace it with redacted summary/fingerprint proof.
- Cleanup/revocation proof required before remote tests: account/key/work-root
  retention state, revocation triggers, and proof that backup/production data
  boundaries remain intact.

## Production SRE Review

- Checklist safety: the operator-facing checklist is explicit enough for
  non-expert use because each risky step has a stop gate and approved proof
  output boundary.
- Stop gates: production data, sudo/admin, backup exposure, alias collision,
  host-key ambiguity, private-material exposure, and unclear cleanup require
  Director escalation.
- Host assumptions: the host must be non-production, synthetic-test only, and
  outside backup/private data boundaries for this lane.
- Artifact retention: remote artifacts must stay under `~/qsl-remote-test/`;
  cleanup status must be recorded before later remote test authorization.
- qbuild isolation: setup failure must not mutate local qbuild/qwork state;
  Codex still does not run qwork/qstart/qresume on remote hosts.

## Release-Claim Boundary Review

NA-0507 preserves these boundaries:

- no public-readiness claim.
- no production-readiness claim.
- no public-internet-readiness claim.
- no external-review-complete claim.
- no crypto-complete claim.
- no replay-proof claim.
- no downgrade-proof claim.
- no secret-material-complete claim.
- no side-channel-free claim.
- no vulnerability-free claim.
- no bug-free claim.
- no perfect-crypto claim.

## Prioritization matrix

| Candidate | Risk reduced | Directness of evidence | Security risk | Operator burden | Implementation feasibility | Scope risk | Secret/key risk | Public-claim risk | Recommended disposition | Next-lane yes/no |
|---|---|---|---|---|---|---|---|---|---|---|
| Operator manual setup authorization and proof collection handoff | high | high | low | moderate | high | low | low with redaction | low | Select | yes |
| Runbook hardening before manual action | low now | medium | low | low | high | low | low | low | Reject unless a concrete gap appears | no |
| Local SSH alias collision check authorization | medium | medium | medium | low | medium | medium | medium | low | Defer to operator proof review | no |
| Remote capability probe authorization | high later | medium | high now | low | medium after setup proof | high now | medium | medium | Defer | no |
| Remote client-to-client E2E authorization | high later | high later | high now | high | low now | high | medium | medium | Defer | no |
| Same-host E2E negative expansion | medium local | low for remote setup | low | low | high | low | low | low | Defer | no |
| CI/tooling lane | low now | indirect | medium | low | high | medium | low | low | Reject without blocker | no |

## Authorization decision

Primary classification:

- `REMOTE_OPERATOR_MANUAL_SETUP_AUTHORIZATION_READY`

Rationale:

- NA-0506/D394 inheritance was consumed.
- NA-0506 runbook is adequate for manual authorization.
- exact operator-owned manual setup steps are selected.
- exact approved proof outputs are selected.
- forbidden paste-back material is selected.
- alias policy is selected.
- setup stop gates and cleanup/revocation proof requirements are selected.
- Option 1 is the lowest-risk and most direct successor.
- no remote action, key generation, SSH config mutation, implementation code
  mutation, or public claim expansion is introduced.

## Selected NA-0508 successor

Selected successor:

- `NA-0508 -- QSL Remote Test Account / SSH Operator Manual Setup Proof Review Harness`

Reason:

- NA-0507 authorizes the Director/operator manual setup sequence.
- The next repo lane must review only redacted operator-provided proof.
- Codex should still not generate keys, install keys, run SSH, mutate config,
  mutate remote hosts, or run remote tests.

## Future scope bundle

### NA-0508 -- QSL Remote Test Account / SSH Operator Manual Setup Proof Review Harness

Status: READY
Goals: G1, G2, G3, G4, G5

Objective:
Review operator-provided redacted proof that the remote test account and SSH
boundary were set up according to the NA-0506/NA-0507 runbook, without Codex
generating keys, installing keys, running SSH, mutating SSH config, mutating
remote hosts, or running remote tests.

Allowed scope:

- `docs/governance/evidence/NA-0508_qsl_remote_test_account_ssh_operator_setup_proof_review.md`
- `tests/NA-0508_qsl_remote_test_account_ssh_operator_setup_proof_review_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- read-only review of operator-provided redacted proof pasted by the user.

Forbidden scope:

- creating remote users.
- generating or installing SSH keys.
- running SSH/scp/sftp/rsync to remote.
- mutating local SSH config.
- mutating system SSH config.
- mutating known_hosts.
- mutating remote hosts.
- sudo/admin action.
- qwork/qstart/qresume mutation.
- qsc source/test/fuzz/Cargo mutation.
- workflow/dependency mutation.
- corpus/vector/input mutation.
- formal/refimpl/service/public/backup mutation.
- no public-readiness claim and no production-readiness claim.

Deliverables:

- proof review evidence.
- testplan.
- decision.
- TRACEABILITY update.
- rolling journal update.
- selected future capability-probe authorization scope or stop/no-action
  rationale.

Acceptance criteria:

- operator proof consumed and checked for redaction.
- private key/passphrase/token absence verified.
- no sudo/admin proof reviewed.
- no backup exposure proof reviewed.
- alias collision proof reviewed.
- no remote action performed by Codex.
- no key material included.
- exactly one READY item remains after closeout.

Manual action note:

- After NA-0507 closeout, the Director should step the operator through the
  approved manual setup checklist outside Codex execution and collect redacted
  proof. NA-0508 should not proceed until proof is available.

## Future validation / marker plan

Future NA-0508 markers:

- `NA0508_OPERATOR_SETUP_PROOF_CONSUMED_OK`
- `NA0508_REDACTED_PROOF_ONLY_OK`
- `NA0508_PRIVATE_KEY_ABSENT_OK`
- `NA0508_PASSPHRASE_ABSENT_OK`
- `NA0508_TOKEN_ABSENT_OK`
- `NA0508_NO_REMOTE_ACTION_BY_CODEX_OK`
- `NA0508_ALIAS_COLLISION_PROOF_REVIEWED_OK`
- `NA0508_NO_SUDO_ADMIN_PROOF_REVIEWED_OK`
- `NA0508_NO_BACKUP_EXPOSURE_PROOF_REVIEWED_OK`
- `NA0508_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0508_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0508_ONE_READY_INVARIANT_OK`

NA-0507 evidence markers:

- `NA0507_QWORK_PROOF_FILES_VERIFIED_OK`
- `NA0507_D394_INHERITANCE_CONSUMED_OK`
- `NA0507_NA0506_RUNBOOK_REVIEWED_OK`
- `NA0507_MANUAL_ACTION_AUTHORIZATION_READY_OK`
- `NA0507_APPROVED_PROOF_OUTPUTS_SELECTED_OK`
- `NA0507_FORBIDDEN_PASTE_BACK_SELECTED_OK`
- `NA0507_ALIAS_POLICY_SELECTED_OK`
- `NA0507_NO_REMOTE_ACTION_OK`
- `NA0507_NO_SSH_EXECUTION_OK`
- `NA0507_NO_ACCOUNT_CREATION_OK`
- `NA0507_NO_SSH_KEY_GENERATION_OK`
- `NA0507_NO_SSH_CONFIG_MUTATION_OK`
- `NA0507_NO_REMOTE_HOST_MUTATION_OK`
- `NA0507_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0507_NO_WORKFLOW_DEPENDENCY_MUTATION_OK`
- `NA0507_NO_CORPUS_VECTOR_INPUT_MUTATION_OK`
- `NA0507_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0507_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0507_ONE_READY_INVARIANT_OK`

## Remote action deferral

Remote capability probes and remote client-to-client E2E tests remain deferred
until after redacted operator setup proof is reviewed and accepted in a later
lane.

NA-0507 authorizes operator manual setup steps only after closeout. It does not
authorize Codex remote setup or remote probes.

## Public claim / website / external review boundary

This lane changes no website, public docs, README, START_HERE,
no public-readiness surface, no production-readiness surface, or
no external-review-complete posture.

Boundary:

- no public-readiness claim.
- no production-readiness claim.
- no public-internet-readiness claim.
- no external-review-complete claim.
- no crypto-complete claim.
- no replay-proof claim.
- no downgrade-proof claim.
- no secret-material-complete claim.
- no side-channel-free claim.
- no vulnerability-free claim.
- no bug-free claim.
- no perfect-crypto claim.

## Backup-impact statement

NA-0507 reads qsl-backup boundary proof only.

- installed qsl-backup helper digest matched the expected value.
- Codex ops source inclusion count remained exactly 1.
- no qsl-backup execution occurred.
- no backup or restore occurred.
- no `/backup/qsl` mutation occurred.
- future remote account setup must prove no `/backup/qsl` exposure.

## Rejected alternatives

- runbook hardening successor: rejected because the NA-0506 runbook is adequate.
- local alias collision-only successor: deferred because alias proof belongs in
  operator proof review unless it becomes the only blocker.
- remote capability probe: deferred until operator setup proof exists.
- remote client-to-client E2E: deferred until setup proof and capability probe
  evidence exist.
- same-host negative expansion: deferred because it does not directly reduce
  the operator setup boundary risk.
- CI/tooling lane: rejected because no tooling blocker exists.

## Next recommendation

Merge NA-0507 authorization only after required checks pass. If post-merge
public-safety is green inside the short attach/early-failure window, close out
NA-0507 and restore the selected NA-0508 proof-review successor. After closeout,
the Director should step the operator through the approved manual setup
checklist outside Codex execution and collect only redacted proof for NA-0508.
