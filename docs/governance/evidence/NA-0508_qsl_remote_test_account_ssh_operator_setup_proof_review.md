Status: Supporting
Owner: QSL Governance / Core Assurance
Last-Updated: 2026-06-20

# NA-0508 Remote Test Account SSH Operator Setup Proof Review

## Executive summary

NA-0508 consumes NA-0507/D396 inheritance and reviews only the operator-provided
redacted proof embedded in directive D397. Codex performed no remote action,
did not run SSH, did not generate or install keys, did not mutate SSH config or
known_hosts, and did not mutate any remote host.

Primary classification:

- `OPERATOR_REMOTE_SETUP_PROOF_ACCEPTED`

Selected successor:

- `NA-0509 -- QSL Remote Host Capability Probe Scope Authorization Plan`

The accepted proof is sufficient to proceed to an authorization-only capability
probe planning lane. It is not remote E2E evidence and it makes no
public-readiness claim, no production-readiness claim, no public-internet-
readiness claim, no external-review-complete claim, no crypto-complete claim,
no replay-proof claim, no downgrade-proof claim, no secret-material-complete
claim, no side-channel-free claim, no vulnerability-free claim, no bug-free
claim, and no perfect-crypto claim.

## Live NA-0508 scope

Allowed mutation paths for this evidence PR:

- `docs/governance/evidence/NA-0508_qsl_remote_test_account_ssh_operator_setup_proof_review.md`
- `tests/NA-0508_qsl_remote_test_account_ssh_operator_setup_proof_review_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Protected boundaries:

- proof review only.
- no remote action by Codex.
- no SSH execution by Codex.
- no scp, sftp, or rsync to remote by Codex.
- no remote account creation by Codex.
- no SSH key generation or installation by Codex.
- no local or system SSH config mutation by Codex.
- no known_hosts or authorized_keys mutation by Codex.
- no remote host mutation by Codex.
- no qwork, qstart, or qresume mutation.
- no qsl-backup execution or mutation.
- no qsc source/test/fuzz/Cargo mutation.
- no workflow/script/helper/dependency mutation.
- no corpus/vector/input mutation.
- no formal/refimpl/service/public/backup mutation.
- exactly one READY item remains mandatory.

## qwork proof-file verification

Codex did not run qwork, qstart, or qresume.

Read and copied proof files:

- `/srv/qbuild/work/NA-0508/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0508/.qwork/startup.qsl-protocol.json`

Verified required fields:

- `startup_result=OK`
- `lane=NA-0508`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0508/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0508`
- `requested_lane_status=READY`

Freshness:

- proof HEAD matched live pre-fetch HEAD at `70018694c589`.
- proof origin/main matched live pre-fetch origin/main at `70018694c589`.
- fetch occurred only after proof/live match and disk proof below the 95% stop
  threshold.

Recovery note:

- an initial qsl-backup source proof counted broad status-file narrative
  mentions and returned 6. This was classified as a recoverable proof-shape
  mistake because it did not inspect the actual installed source list. The
  corrected read-only parser inspected the installed helper source array and
  confirmed the Codex ops source appears exactly once.

## NA-0507 / D396 inheritance

Consumed:

- D396 response:
  `/home/victor/work/qsl/codex/responses/NA0507_closeout_restore_na0508_20260620T161558Z_D396.md`
- NA-0507 evidence:
  `docs/governance/evidence/NA-0507_qsl_remote_test_account_ssh_operator_setup_readiness_manual_action_authorization_plan.md`
- NA-0507 testplan:
  `tests/NA-0507_qsl_remote_test_account_ssh_operator_setup_readiness_manual_action_authorization_testplan.md`
- D-1003 and D-1004 in `DECISIONS.md`
- NA-0508 READY block in `NEXT_ACTIONS.md`
- NA-0506 runbook and testplan.

Inherited facts:

- NA-0507 completed.
- NA-0508 was restored READY.
- operator manual setup authorization completed.
- approved proof categories were selected.
- forbidden paste-back material was selected.
- NA-0508 was selected as proof-review-only.
- no remote action by Codex occurred.
- no SSH execution by Codex occurred.
- no account creation by Codex occurred.
- no SSH key generation or installation by Codex occurred.
- no SSH config mutation by Codex occurred.
- no remote host mutation by Codex occurred.
- no public-readiness claim is made.
- no production-readiness claim is made.
- no crypto-complete claim is made.
- no replay-proof claim is made.
- no downgrade-proof claim is made.
- no secret-material-complete claim is made.

Selected NA-0508 purpose:

- review operator-provided redacted proof only.

## Operator proof reviewed

Codex reviewed only the operator-provided redacted proof embedded in D397.
Codex did not rerun SSH, did not scan host keys, did not inspect private key
files, and did not read or mutate SSH configuration.

Reviewed operator proof findings:

- local host was documented as `ideacentre` and local user as `victor`.
- remote host was documented as `Inspiron`.
- remote alias was documented as `inspiron`.
- optional generic alias `remote` was not approved and not configured.
- remote account was documented as `qslcodex`.
- remote account UID/GID were documented as `1003` / `1003`.
- remote account shell was documented as `/bin/bash`.
- account password was documented as locked.
- remote account groups were documented as only `qslcodex`.
- remote account was documented absent from sudo, adm, docker, lxd, libvirt,
  wheel, and admin groups.
- sudo proof was documented as denied.
- qsl-remote-test workdir was documented as existing and writable by
  `qslcodex`.
- remote qwork was documented absent.
- remote qsl-backup was documented absent.
- `/backup/qsl` was documented absent or not readable by `qslcodex`.
- host key fingerprint match was documented.
- public key fingerprint match was documented.
- SSH config was documented as key-only, BatchMode, no password auth, strict
  host-key checking, no agent forwarding, no X11 forwarding, and clear
  forwardings.
- Step 10 proof-shape issue was documented and corrected by Step 10B.

Classification:

- `OPERATOR_REMOTE_SETUP_PROOF_ACCEPTED`

## Redaction review

The proof is redacted enough for this governance lane.

Accepted redaction findings:

- private key content was absent.
- passphrase was absent.
- token was absent.
- password was absent.
- production endpoint was absent.
- backup material was absent.
- private host key material was absent.
- Tailscale/private address detail was not needed for this evidence summary.

The proof includes public fingerprints and non-secret account/config metadata
only. The evidence preserves the operator-provided fingerprint facts without
treating them as remote E2E evidence.

## Private key / passphrase / token absence proof

The operator proof explicitly stated:

- private key included: no.
- passphrase included: no.
- password included: no.
- token included: no.
- production endpoint included: no.
- backup material included: no.

NA-0508 evidence and testplan intentionally include no private key block, no
passphrase, no token, no password, no production endpoint, and no backup
material.

## Account boundary proof

Reviewed account boundary facts:

- account: `qslcodex`.
- UID/GID: `1003` / `1003`.
- home: `/home/qslcodex`.
- shell: `/bin/bash`.
- account is non-root.
- account password is locked.
- groups are only `qslcodex`.
- account is absent from sudo, adm, docker, lxd, libvirt, wheel, and admin.
- `sudo -n true` returned denied in the operator proof.
- remote workdir `qsl-remote-test` exists and is writable by `qslcodex`.
- `/home/qslcodex`, `.ssh`, `authorized_keys`, and workdir permissions were
  documented as restrictive and account-owned.

This is enough to establish a bounded test-account setup proof for the next
authorization lane. It is not enough to prove future commands remain bounded if
the account, host, alias, host-key trust, or private key custody changes later.

## SSH config / alias proof

Reviewed alias/config facts:

- host alias: `inspiron`.
- `User qslcodex`.
- `HostName inspiron`.
- `IdentityFile ~/.ssh/qslcodex_ed25519`.
- `IdentitiesOnly yes`.
- `PasswordAuthentication no`.
- `BatchMode yes`.
- `StrictHostKeyChecking true`.
- `ForwardAgent no`.
- `ForwardX11 no`.
- `ClearAllForwardings yes`.
- optional alias `remote` was not configured and not approved.

The proof also documented no pre-setup alias collision for `inspiron` or
`remote` in local and system SSH config. Codex did not inspect or mutate SSH
config; this lane consumes only the operator-provided redacted proof.

## Host key / public key fingerprint proof

Host key proof reviewed:

- preferred remote ED25519 host key fingerprint:
  `SHA256:w/WsyRVT77+VKCWG5b9eK7Jpak9F8EeWxaPsm3YyxEc`
- Build-side scanned ED25519 public host key fingerprint matched the same
  value.
- `HOST_KEY_MATCH yes` was documented.
- private host key material was not printed.

Public key proof reviewed:

- dedicated public key fingerprint:
  `SHA256:+dltU+DgwTGdPRV633NOsz0RVwVrPEC4HN0XDjFROMc`
- installed authorized key fingerprint matched the same value.
- authorized_keys line count was documented as 1.
- authorized_keys was documented as account-owned with mode 600.
- the installed line was documented as prefixed with an OpenSSH restrict
  option.
- private key content and passphrase were not printed.

## Step 10 proof-shape issue and Step 10B correction

Step 10 proof-shape issue:

- Step 10 printed `STOP_PUBLIC_KEY_FILE_NOT_READABLE` with a malformed path
  containing `/home/victor/~/...`.
- This is classified as a proof-script path-expansion bug, not an account,
  host-key, public-key, or SSH boundary failure.

Step 10B correction:

- effective identityfile raw value was documented as `~/.ssh/qslcodex_ed25519`.
- expanded identity path was documented as
  `/home/victor/.ssh/qslcodex_ed25519`.
- private key metadata was documented as owner `victor`, group `victor`,
  mode 600, regular file.
- public key metadata was documented as owner `victor`, group `victor`,
  mode 644, regular file.
- public key fingerprint was documented as
  `SHA256:+dltU+DgwTGdPRV633NOsz0RVwVrPEC4HN0XDjFROMc`.
- actual fingerprint matched expected.
- private key content printed: no.
- passphrase printed: no.

## No Codex remote action proof

This directive performed proof review only.

Codex did not run:

- SSH.
- scp.
- sftp.
- rsync to remote.
- ssh-keygen.
- ssh-keyscan.
- sudo/admin action.
- remote account creation.
- SSH key installation.
- local or system SSH config mutation.
- known_hosts mutation.
- authorized_keys mutation.
- remote host mutation.
- qwork, qstart, or qresume.
- qsl-backup.

## qwork / qsl-backup remote absence proof

Operator proof reviewed:

- qwork absent on Inspiron.
- qwork absent for `qslcodex`.
- qsl-backup absent on Inspiron.
- qsl-backup absent for `qslcodex`.

Local qsl-backup boundary was read-only:

- installed helper SHA-256 matched
  `e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`.
- configured daily source list includes Codex ops exactly once.
- no backup or restore was run.

## Backup exposure proof

Operator proof reviewed:

- `/backup/qsl` was absent in remote preflight.
- `/backup/qsl` was absent or not readable by `qslcodex` after account setup.
- no backup material was pasted.
- no remote qsl-backup was present.

Backup-impact statement:

- NA-0508 changes only governance evidence/testplan/decision/traceability/
  journal files.
- NA-0508 performs no backup or restore.
- NA-0508 changes no qsl-backup source, status, plan, timer, backup tree, or
  `/backup/qsl` content.

## Hostile Cryptographer Review

Question: Is this proof enough to establish a bounded remote account / SSH
boundary without treating it as remote E2E evidence?

- Yes for setup-boundary review: the proof documents a dedicated non-root
  no-sudo account, a dedicated public key fingerprint matching the installed
  key fingerprint, a matching host key fingerprint, key-only SSH config, no
  forwarding, no qwork/qsl-backup remote presence, and no backup exposure.
- No for remote E2E: it does not prove qsc remote send/receive behavior,
  transcript binding, downgrade resistance, replay resistance, remote artifact
  cleanup, side-channel behavior, or future host/alias/key stability.

Question: Does the proof avoid exposing private keys/passphrases?

- Yes. The proof records metadata and public fingerprints only; private key
  content and passphrase are absent.

Question: Does the proof avoid public/production readiness claims?

- Yes. This evidence makes no public-readiness claim and no production-
  readiness claim.

Question: Does the proof preserve qwork/qsl-backup local-only boundaries?

- Yes. Remote qwork and qsl-backup absence are documented, and local
  qsl-backup was only inspected read-only.

Question: What remains unproven before a remote capability probe?

- Future command list authorization.
- Current alias target at probe time.
- Current host-key match at probe time.
- Current account no-sudo/no-backup/qwork/qsl-backup boundaries at probe time.
- Basic shell and workdir behavior under tightly bounded commands.
- Redaction rules for future command output.
- Cleanup/revocation proof.

## Red-Team Review

What if `qslcodex` later gains sudo?

- The capability probe must re-check no-sudo before any nontrivial command and
  stop if sudo/admin exposure appears.

What if the `inspiron` alias is retargeted?

- The capability probe must confirm the effective target, remote user, and
  host-key fingerprint before treating any output as valid.

What if known_hosts changes?

- The future lane must treat host-key mismatch or ambiguity as a stop condition;
  it must not loosen strict host-key checking.

What if the private key leaks later?

- Cleanup/revocation proof must remove the public key or disable the account
  before continuing. A later probe must not assume the key remained private.

What if remote artifacts accumulate?

- Future probes must use the dedicated workdir, keep artifacts synthetic, and
  record cleanup/retention state.

What if backup exposure appears later?

- Any `/backup/qsl` readability or qsl-backup presence on the remote host is a
  stop condition before remote E2E.

What must the future capability probe check before remote E2E?

- identity, host, UID/GID, groups, no sudo, no backup, no qwork, no qsl-backup,
  workdir existence/writability, shell capability, redaction, and cleanup
  posture.

## Production SRE Review

Is the operator proof safe enough to proceed to a later capability-probe
authorization lane?

- Yes. It is safe enough for authorization planning only. It is not enough for
  immediate remote E2E or broad remote execution.

What should the next lane test and not test?

- It should select exact future bounded commands for identity, host, account,
  workdir, no sudo, no backup, no qwork/qsl-backup, shell capability, output
  redaction, and cleanup posture.
- It should not run SSH, remote commands, remote E2E, key generation, key
  installation, config mutation, host mutation, or implementation code changes.

How should future remote command output be redacted?

- Keep usernames, aliases, UIDs/GIDs, modes, and fingerprints if needed.
- Omit private IPs unless necessary.
- Omit full unrelated config, known_hosts, authorized_keys lines, private key
  paths unrelated to this setup, tokens, passwords, passphrases, endpoint
  secrets, production paths, and backup material.

How should setup failure be isolated from qbuild/qwork?

- qwork, qstart, qresume, and qsl-backup remain local-only. Future remote
  failures must not trigger qbuild tool mutation or remote backup tooling.

What cleanup/revocation proof remains needed?

- Whether the public key remains installed only for a bounded follow-on lane,
  whether `qslcodex` remains enabled, whether the workdir is empty or contains
  only synthetic artifacts, and what revocation action occurs on key leak,
  host-key drift, sudo exposure, backup exposure, or alias ambiguity.

## Release-Claim Boundary Review

This evidence preserves:

- no public-readiness claim is made.
- no production-readiness claim is made.
- no public-internet-readiness claim is made.
- no external-review-complete claim is made.
- no crypto-complete claim is made.
- no replay-proof claim is made.
- no downgrade-proof claim is made.
- no secret-material-complete claim is made.
- no side-channel-free claim is made.
- no vulnerability-free claim is made.
- no bug-free claim is made.
- no perfect-crypto claim is made.

## Prioritization matrix

| Candidate | Risk reduced | Directness of evidence | Security risk | Operator burden | Implementation feasibility | Scope risk | Secret/key risk | Public-claim risk | Recommended disposition | Next-lane yes/no |
|---|---|---|---|---|---|---|---|---|---|---|
| Remote capability probe authorization | Converts accepted setup proof into exact future no-mutation command scope | High for next safe step | Low because authorization-only | Low | High | Low | Low | Low | Select NA-0509 | Yes |
| Remote capability probe implementation | Proves live account/host/workdir/shell boundary | High but requires remote commands | Medium until scope is authorized | Medium | High after NA-0509 | Medium | Medium | Medium | Defer until authorization | No |
| Remote E2E authorization | Plans future qsc remote E2E | Medium, but premature without live capability proof | Medium | Medium | Medium | Medium | Medium | Medium | Defer pending capability probe | No |
| Remote E2E implementation | Proves remote qsc send/receive behavior | High after prerequisites | Higher | High | Medium | High | Medium | High | Reject for now | No |
| Setup remediation / proof-hardening | Fixes ambiguous or failed setup proof | Low because proof accepted | Low | Medium | High | Low | Low | Low | Not needed now | No |
| Same-host E2E negative expansion | Improves local qsc negative coverage | Useful but indirect for remote setup | Low | Low | High | Medium | Low | Low | Defer | No |
| CI/tooling lane | Improves automation reliability | Indirect for remote boundary | Low | Low | Medium | Medium | Low | Low | Defer | No |

## Authorization decision

Primary classification:

- `OPERATOR_REMOTE_SETUP_PROOF_ACCEPTED`

Required decision facts:

- NA-0507/D396 consumed.
- operator proof reviewed.
- redaction reviewed.
- account boundary reviewed.
- host-key fingerprint proof reviewed.
- public-key fingerprint proof reviewed.
- option/prioritization review completed.
- exact NA-0509 successor selected.
- no remote action by Codex.
- no key generation by Codex.
- no SSH config mutation by Codex.
- no implementation mutation.
- no public claim expansion.
- exactly one READY successor remains mandatory.

## Selected NA-0509 successor

Selected successor:

- `NA-0509 -- QSL Remote Host Capability Probe Scope Authorization Plan`

Rationale:

- The operator proof appears to establish the remote account / SSH boundary.
- The next safe step is not remote E2E.
- The next safe step is an authorization-only lane that selects exact future
  capability probe commands and stop conditions.

## Future scope bundle

### NA-0509 -- QSL Remote Host Capability Probe Scope Authorization Plan

Status: READY
Goals: G1, G2, G3, G4, G5

Objective:

Authorize the exact future remote capability probe scope for the `inspiron` /
`qslcodex` test account after redacted operator setup proof review, without
running SSH or remote commands in this lane and without performing remote E2E.

Allowed scope:

- `docs/governance/evidence/NA-0509_qsl_remote_host_capability_probe_scope_authorization_plan.md`
- `tests/NA-0509_qsl_remote_host_capability_probe_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- read-only review of NA-0508 proof-review evidence and prior setup proof.

Forbidden scope:

- running SSH/scp/sftp/rsync to remote.
- generating or installing SSH keys.
- mutating local SSH config.
- mutating known_hosts.
- mutating remote hosts.
- creating remote users.
- sudo/admin action.
- qwork/qstart/qresume mutation.
- qsc source/test/fuzz/Cargo mutation.
- workflow/dependency mutation.
- corpus/vector/input mutation.
- formal/refimpl/service/public/backup mutation.
- remote E2E.
- no public-readiness claim and no production-readiness claim.

Deliverables:

- capability probe scope authorization evidence.
- testplan.
- decision.
- TRACEABILITY update.
- rolling journal update.
- exact future probe command list or stop/no-action rationale.

Acceptance criteria:

- prior proof accepted or remediation selected.
- exact future remote commands selected.
- no remote commands run in this lane.
- no remote mutation authorized beyond bounded probe if future lane executes.
- no private key/passphrase/token material included.
- no public-readiness claim is made.
- no production-readiness claim is made.
- exactly one READY item remains after closeout.

## Future validation / marker plan

Future NA-0509 markers:

- `NA0509_OPERATOR_SETUP_PROOF_ACCEPTED_OK`
- `NA0509_REMOTE_CAPABILITY_PROBE_SCOPE_SELECTED_OK`
- `NA0509_NO_REMOTE_ACTION_IN_AUTHORIZATION_OK`
- `NA0509_NO_REMOTE_E2E_SCOPE_OK`
- `NA0509_NO_SSH_KEY_GENERATION_OK`
- `NA0509_NO_SSH_CONFIG_MUTATION_OK`
- `NA0509_NO_REMOTE_HOST_MUTATION_OK`
- `NA0509_NO_SUDO_ADMIN_SCOPE_OK`
- `NA0509_NO_BACKUP_EXPOSURE_OK`
- `NA0509_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0509_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0509_ONE_READY_INVARIANT_OK`

## Remote E2E deferral

Remote E2E remains deferred. NA-0508 does not prove qsc remote send/receive,
does not run any remote command, and does not authorize remote E2E. A future
capability probe must first re-check the live boundary before any E2E lane is
considered.

## Public claim / website / external review boundary

NA-0508 changes no website, public docs, README, START_HERE, public technical
paper, external-review artifact, or release-claim surface.

This evidence makes no public-readiness claim, no production-readiness claim,
no public-internet-readiness claim, no external-review-complete claim, no
crypto-complete claim, no replay-proof claim, no downgrade-proof claim, no
secret-material-complete claim, no side-channel-free claim, no vulnerability-
free claim, no bug-free claim, and no perfect-crypto claim.

## Rejected alternatives

- Immediate remote capability probe implementation: rejected because exact
  command scope must be authorized first.
- Immediate remote E2E authorization: rejected because live remote capability
  proof is still missing.
- Immediate remote E2E implementation: rejected because it would overrun the
  proof-review lane and require remote commands.
- Setup remediation: rejected because the operator proof was accepted.
- CI/tooling lane: rejected because it is indirect to this boundary.
- Same-host E2E negative expansion: rejected because it does not directly
  validate the remote setup proof.

## Next recommendation

After NA-0508 evidence merges and closeout is authorized, restore exactly one
READY item:

- `NA-0509 -- QSL Remote Host Capability Probe Scope Authorization Plan`

NA-0509 should remain authorization-only and must not run SSH or remote
commands.

## Marker proof

- `NA0508_OPERATOR_SETUP_PROOF_CONSUMED_OK`
- `NA0508_REDACTED_PROOF_ONLY_OK`
- `NA0508_PRIVATE_KEY_ABSENT_OK`
- `NA0508_PASSPHRASE_ABSENT_OK`
- `NA0508_TOKEN_ABSENT_OK`
- `NA0508_PASSWORD_ABSENT_OK`
- `NA0508_PRODUCTION_ENDPOINT_ABSENT_OK`
- `NA0508_BACKUP_MATERIAL_ABSENT_OK`
- `NA0508_REMOTE_ALIAS_INSPIRON_REVIEWED_OK`
- `NA0508_OPTIONAL_REMOTE_ALIAS_NOT_CONFIGURED_OK`
- `NA0508_ACCOUNT_BOUNDARY_REVIEWED_OK`
- `NA0508_NO_SUDO_ADMIN_PROOF_REVIEWED_OK`
- `NA0508_NO_BACKUP_EXPOSURE_PROOF_REVIEWED_OK`
- `NA0508_HOST_KEY_FINGERPRINT_MATCH_REVIEWED_OK`
- `NA0508_PUBLIC_KEY_FINGERPRINT_MATCH_REVIEWED_OK`
- `NA0508_STEP10_PATH_EXPANSION_CORRECTED_OK`
- `NA0508_NO_REMOTE_ACTION_BY_CODEX_OK`
- `NA0508_NO_SSH_EXECUTION_BY_CODEX_OK`
- `NA0508_NO_SSH_KEY_GENERATION_BY_CODEX_OK`
- `NA0508_NO_SSH_CONFIG_MUTATION_BY_CODEX_OK`
- `NA0508_NO_REMOTE_HOST_MUTATION_BY_CODEX_OK`
- `NA0508_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0508_NO_WORKFLOW_DEPENDENCY_MUTATION_OK`
- `NA0508_NO_CORPUS_VECTOR_INPUT_MUTATION_OK`
- `NA0508_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0508_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0508_ONE_READY_INVARIANT_OK`
