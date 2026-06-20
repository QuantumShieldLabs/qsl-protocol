Status: Supporting
Owner: QSL Governance / Core Assurance
Last-Updated: 2026-06-20

# NA-0506 Remote Test Account SSH Operator Setup Runbook

## Executive summary

NA-0506 consumes NA-0505/D393 and implements only an in-repo operator runbook
and proof checklist for a future least-privilege remote test account and SSH
boundary.

This runbook does not authorize Codex remote action. Codex did not create an
account, generate or install an SSH key, run SSH, run scp/sftp/rsync, mutate
local SSH config, mutate system SSH config, mutate known_hosts, mutate
authorized_keys, mutate a remote host, run sudo, install packages, run qwork on
a remote host, or run qsl-backup.

Default host alias:

- `qsl-remote-test`

Optional convenience alias:

- `remote`, only after an operator collision check and explicit operator
  approval.

Selected successor:

- `NA-0507 -- QSL Remote Test Account / SSH Operator Setup Readiness and Manual Action Authorization Plan`

## Scope and non-scope

In scope:

- document the future remote account model.
- document the future SSH key model.
- document the future host alias model.
- document local operator responsibilities.
- document Codex restrictions.
- document remote host restrictions.
- document the remote directory model.
- provide proof and cleanup checklists.
- record failure handling and paste-back boundaries.

Out of scope:

- remote account creation.
- SSH execution.
- scp/sftp/rsync execution.
- SSH key generation or installation.
- local or system SSH config mutation.
- known_hosts or authorized_keys mutation.
- remote host mutation.
- sudo/admin action.
- package installation.
- remote qwork/qstart/qresume.
- remote qsl-backup.
- qsc implementation, fuzz, Cargo, corpus, vector, workflow, dependency,
  formal, refimpl, service, public-doc, or backup mutation.

## Threat model

The remote test host is future test infrastructure, not a trusted secret store.
The remote account boundary must limit blast radius if the test account, host,
operator workstation, or SSH key path is mishandled.

Primary threats:

- private SSH key disclosure.
- personal SSH key reuse.
- accidental use of an existing broad host alias.
- accidental login as root or a primary operator account.
- sudo/admin exposure through the test account.
- production data exposure.
- backup material exposure.
- qwork or qsl-backup execution on remote infrastructure.
- host-key trust drift.
- durable test artifacts without cleanup or revocation proof.
- public/security claim drift beyond evidence.

Fail-closed response:

- if account, key, alias, host-key, backup, sudo, or production-data proof is
  ambiguous, the Director should stop before authorizing later Codex remote
  probes.

## Roles

Operator:

- owns future manual account setup.
- owns future SSH key generation and private key custody.
- owns future host-key verification.
- owns future local SSH config edits, if any.
- collects redacted proof outputs for the Director.
- never pastes private keys, passphrases, tokens, credentials, production
  endpoints, or backup private material to Codex.

Director:

- remains final authority for lane progression.
- authorizes any later manual action and any later Codex remote capability
  probe.
- decides whether proof is sufficient.

Codex:

- writes this runbook and governance evidence only in NA-0506.
- must not perform remote setup or remote tests in NA-0506.
- must not read or handle private key material.
- must not mutate local SSH config, known_hosts, authorized_keys, or a remote
  host in NA-0506.
- must not run qwork/qstart/qresume or qsl-backup on a remote host.

Future remote host:

- is limited to non-production test use.
- must not contain production data or backup material for this lane.
- must provide no sudo/admin path to the test user.
- must use a dedicated work root for QSL remote test artifacts.

Stewards:

- remain advisory only. The Lead Director remains final authority.

## NA-0505 / D393 inheritance

NA-0505 completed and NA-0506 was restored READY by D-1000.

Inherited classification:

- `REMOTE_SSH_OPERATOR_RUNBOOK_IMPLEMENTATION_READY`

Inherited account model:

- dedicated non-root project test user.
- no sudo.
- no primary operator account.

Inherited SSH model:

- dedicated per-project operator-owned key.
- key-only login if feasible.
- no personal key reuse.

Inherited alias model:

- default `qsl-remote-test`.
- optional `remote` only after collision check and explicit operator approval.

Inherited prohibitions:

- no remote action performed.
- no SSH execution.
- no account creation.
- no SSH key generation or installation.
- no SSH config mutation.
- no remote host mutation.
- no qwork or qsl-backup remote execution.
- no public-readiness claim, no production-readiness claim, no crypto-complete
  claim, no replay-proof claim, no downgrade-proof claim, and no
  secret-material-complete claim.

Selected NA-0506 purpose:

- implement this in-repo operator setup runbook and proof checklist only.

## Account model

Future account placeholder:

- `<qsl_remote_user>`

Required properties:

- dedicated to QSL remote test work.
- non-root.
- no sudo.
- not the operator's primary personal account.
- no production data access.
- no backup access.
- no shared secrets or private key material stored in the repo.
- removable or disableable by the operator during cleanup.

The test user should own only a dedicated test work root and synthetic test
artifacts. The account must not be used for production service administration,
personal operations, backups, or unrelated projects.

## SSH key model

Future key placeholder:

- `<qsl_remote_key_public>`

Required properties:

- dedicated per-project key.
- generated only by the operator outside Codex.
- private key never committed to this repo.
- private key never pasted to Codex.
- private key passphrase never pasted to Codex.
- personal key reuse is rejected.
- public key may be installed by the operator only after a later lane
  authorizes the manual setup step.
- key-only login is preferred if feasible.

Codex must not run a key-generation tool in this lane or install any key.

## Host alias model

Default alias:

- `qsl-remote-test`

Optional alias:

- `remote`, only after the operator proves no collision with existing local
  SSH aliases and explicitly approves the convenience alias.

Why `remote` is convenient:

- it is short and easy to type.
- it may simplify future operator instructions.

Why `remote` is riskier:

- it is generic.
- it may collide with an existing operator host.
- it may cause accidental command targeting if reused across projects.

The default alias remains `qsl-remote-test` unless a later authorization lane
records collision proof and explicit operator approval for `remote`.

## Local SSH config boundary

Local SSH config is operator-only future action.

NA-0506 Codex boundary:

- no local SSH config mutation.
- no system SSH config mutation.
- no known_hosts mutation.
- no host-key scanning.
- no SSH execution.

Recommended future config fields, placeholders only:

- alias: `qsl-remote-test`
- user: `<qsl_remote_user>`
- host: `<qsl_remote_host_placeholder>`
- identity file: `<operator_owned_qsl_remote_key_path>`
- identities-only: `<operator_selected_boolean>`
- host-key policy: `<operator_verified_host_key_policy>`

No real hostname, IP address, endpoint, or private SSH key path is recorded in
this runbook.

## Remote account setup boundary

Remote account setup is operator-only future action.

NA-0506 Codex boundary:

- no account creation.
- no sudo.
- no package installation.
- no authorized_keys mutation.
- no remote host mutation.
- no remote tests.

The operator must complete any future setup manually outside Codex, only after
a later directive authorizes exact operator-facing steps.

## Remote directory model

Future dedicated work root placeholder:

- `~/qsl-remote-test/`

Required properties:

- owned by `<qsl_remote_user>`.
- used only for synthetic QSL remote test artifacts.
- no `/backup/qsl`.
- no production data.
- no private keys.
- no passphrases.
- no tokens.
- no qwork on remote.
- no qstart/qresume on remote.
- no qsl-backup on remote.
- cleanup and revocation proof required before the lane family treats the
  setup as retired.

## Future setup checklist

This checklist is for a later operator-authorized lane. It is not executable by
Codex in NA-0506.

- confirm the Director authorized manual setup.
- confirm the remote host is non-production test infrastructure.
- check local alias collision for `qsl-remote-test`.
- if considering `remote`, check for alias collision and obtain explicit
  operator approval.
- generate a dedicated per-project SSH key as the operator outside Codex.
- create or select `<qsl_remote_user>` as a dedicated non-root test user.
- confirm `<qsl_remote_user>` has no sudo/admin capability.
- install only the dedicated public key as the operator.
- verify private key material remains outside repo and outside Codex.
- verify key-only login if feasible.
- verify host-key identity out of band before trusting the alias.
- create or verify `~/qsl-remote-test/`.
- verify no production data in the work root.
- verify no `/backup/qsl` access.
- verify qwork/qstart/qresume are not run on the remote host.
- verify qsl-backup is not run on the remote host.
- collect redacted proof outputs.

## Future proof checklist

Account identity proof:

- show `<qsl_remote_user>` identity without exposing credentials.
- show the account is not root.
- show the account is dedicated to QSL remote testing.

Home/work directory proof:

- show the home directory and `~/qsl-remote-test/` ownership.
- show the work root contains only synthetic test material or is empty.

No sudo proof:

- show the test user has no sudo/admin capability.

Key-only login proof:

- show login is bound to the dedicated operator-owned public key if feasible.
- do not include private key material.

Host-key verification proof:

- show the operator verified host identity out of band.
- do not paste private host material.

No backup access proof:

- show the test user cannot access `/backup/qsl`.
- show qsl-backup is not used on the remote host.

Cleanup/revocation proof:

- show public key removal or account disablement/removal when retiring the
  setup.
- show the dedicated work root cleanup decision and result.

## Allowed future Codex commands after later authorization

Codex may run only very limited remote capability probes after a later
directive explicitly authorizes exact command shapes, proof collection, and
failure handling.

Future probes must be:

- read-only unless the later directive expressly authorizes a bounded synthetic
  test artifact.
- scoped to `qsl-remote-test` unless `remote` was explicitly approved.
- free of private key, passphrase, token, backup, and production endpoint
  material.
- fail-closed on ambiguity.

No such command is authorized by NA-0506.

## Explicit forbidden future commands until later authorization

Until a later directive authorizes exact actions, Codex must not:

- run SSH/scp/sftp/rsync to remote.
- mutate a remote host.
- run sudo.
- create remote accounts.
- generate or install SSH keys.
- mutate local SSH config.
- mutate system SSH config.
- mutate known_hosts.
- mutate authorized_keys.
- run qwork/qstart/qresume.
- run qsl-backup.
- install services.
- start persistent daemons.
- perform remote tests.

## Cleanup and revocation plan

Cleanup must be operator-owned and proof-backed.

Required cleanup decisions:

- whether to remove the dedicated public key.
- whether to disable or remove `<qsl_remote_user>`.
- whether to remove `~/qsl-remote-test/`.
- whether any synthetic artifacts are retained for evidence.

Required revocation events:

- suspected private key disclosure.
- host-key mismatch.
- sudo/admin exposure.
- backup exposure.
- production data exposure.
- operator uncertainty about alias targeting.

Fail-closed action:

- stop later remote work until revocation and proof are complete enough for the
  Director to evaluate.

## Failure handling

Stop and escalate if any future setup or proof indicates:

- alias collision cannot be ruled out.
- private key material was pasted, committed, or exposed.
- passphrase, token, credential, or production endpoint was exposed.
- host identity is ambiguous.
- login uses a personal key.
- account is root or has sudo/admin capability.
- test user can access `/backup/qsl`.
- remote work root contains production data.
- Codex would need to mutate remote host state without later authorization.
- public/security claim wording would exceed evidence.

Recoverable cases must record:

- failing command or proof step.
- why it is recoverable.
- corrective action.
- final result.

## What operator may paste back

Allowed paste-back material:

- redacted command outputs.
- account name placeholder or approved test account name.
- host alias proof.
- no-sudo proof with secret material removed.
- work-root ownership proof.
- host-key verification summary.
- cleanup/revocation proof.
- confirmation that private key material remained outside Codex.

## What operator must never paste back

Never paste:

- private SSH keys.
- SSH key passphrases.
- API tokens.
- session tokens.
- passwords.
- credentials.
- production endpoints.
- backup private material.
- authorized_keys contents containing unrelated real keys.
- long secret-like hex dumps.
- private host material.

## Claim boundaries

NA-0506 makes no public-readiness claim.

NA-0506 makes no production-readiness claim.

NA-0506 makes no public-internet-readiness claim.

NA-0506 makes no external-review-complete claim.

NA-0506 makes no crypto-complete claim.

NA-0506 makes no replay-proof claim.

NA-0506 makes no downgrade-proof claim.

NA-0506 makes no secret-material-complete claim.

NA-0506 makes no side-channel-free claim.

NA-0506 makes no vulnerability-free claim, no bug-free claim, and no
perfect-crypto claim.

## Stewardship and assurance reviews

Best-Known-Method Review:

- the safest next action is a human-readable runbook and proof checklist before
  any operator setup or Codex remote capability probe.

Hostile Cryptographer Review:

- private key custody, host-key trust, alias collision, backup exposure, and
  claim drift remain the highest-risk boundaries.

Red-Team Review:

- a generic `remote` alias, personal key reuse, sudo exposure, and unclear
  cleanup are treated as failure modes requiring stop/escalation.

Production SRE Review:

- future setup must not use production data, sudo/admin access, service
  installation, persistent daemons, package installation by Codex, or backup
  paths.

Side-Channel Caveat:

- this runbook does not prove side-channel resistance and makes no
  side-channel-free claim.

Formal-Model Mapping Residual:

- remote account and SSH operator setup are operational boundaries, not formal
  protocol-model proof.

External-Review Readiness:

- this runbook improves evidence hygiene for later review, but makes no
  external-review-complete claim.

Release-Claim Boundary:

- this lane is not public, production, public-internet, crypto, replay,
  downgrade, secret-material, side-channel, vulnerability, bug, or perfect
  crypto completion evidence.

Assurance Gap Review Trigger:

- if future proof exposes alias, key, host, sudo, backup, production-data, or
  private-material ambiguity, the next lane must refine the boundary before
  remote probing.

## Future lane sequence

Expected sequence:

1. NA-0507 operator setup readiness / manual action authorization.
2. later manual setup proof review and limited capability probe authorization.
3. later remote E2E authorization and implementation, if proof supports it.

NA-0506 does not implement NA-0507.

## Markers

- NA0506_REMOTE_BOUNDARY_SCOPE_CONSUMED_OK
- NA0506_OPERATOR_RUNBOOK_IMPLEMENTED_OK
- NA0506_REMOTE_ACCOUNT_MODEL_DOCUMENTED_OK
- NA0506_SSH_KEY_MODEL_DOCUMENTED_OK
- NA0506_HOST_ALIAS_MODEL_DOCUMENTED_OK
- NA0506_NO_REMOTE_ACTION_OK
- NA0506_NO_SSH_KEY_GENERATION_OK
- NA0506_NO_SSH_CONFIG_MUTATION_OK
- NA0506_NO_REMOTE_HOST_MUTATION_OK
- NA0506_NO_SUDO_ADMIN_SCOPE_OK
- NA0506_NO_BACKUP_EXPOSURE_OK
- NA0506_NO_PUBLIC_READINESS_CLAIM_OK
- NA0506_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0506_ONE_READY_INVARIANT_OK
