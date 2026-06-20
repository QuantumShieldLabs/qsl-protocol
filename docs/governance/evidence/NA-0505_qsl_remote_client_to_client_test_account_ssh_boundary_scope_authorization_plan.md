Status: Supporting
Owner: QSL Governance / Core Assurance
Last-Updated: 2026-06-20

# NA-0505 Remote Client-to-Client Test Account SSH Boundary Scope Authorization Plan

## Executive summary

NA-0505 consumes the NA-0504 closeout evidence and authorizes only the
least-privilege boundary for a future remote/LAN client-to-client test setup.
No account was created, no SSH key was generated or installed, no SSH command
was run, no SSH config was changed, and no remote host was mutated.

Primary classification:
`REMOTE_SSH_OPERATOR_RUNBOOK_IMPLEMENTATION_READY`.

Selected successor:
`NA-0506 -- QSL Remote Test Account / SSH Operator Setup Runbook Implementation Harness`.

The next lane should write an in-repo operator runbook and proof checklist. It
must still avoid Codex-controlled remote setup, key handling, SSH execution,
local SSH config mutation, host mutation, and remote tests.

## Live NA-0505 scope

Allowed mutation paths:

- `docs/governance/evidence/NA-0505_qsl_remote_client_to_client_test_account_ssh_boundary_scope_authorization_plan.md`
- `tests/NA-0505_qsl_remote_client_to_client_test_account_ssh_boundary_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

NA-0505 is authorization-only. It decides the remote account, SSH key, host
alias, operator proof, and future scope boundaries. It does not implement the
remote setup.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

qwork proof files existed and were copied into the directive proof root:

- `/srv/qbuild/work/NA-0505/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0505/.qwork/startup.qsl-protocol.json`

Verified startup fields:

- `startup_result=OK`
- `lane=NA-0505`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0505/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0505`
- `requested_lane_status=READY`

Freshness proof:

- qwork HEAD matched live pre-fetch HEAD at `a79ab1cfd70b`.
- qwork origin/main matched live pre-fetch origin/main at `a79ab1cfd70b`.
- Fetch was performed only after proof/live ref match and disk proof below the
  stop threshold.
- `/` disk use was 78%; `/backup/qsl` disk use was 25%.
- qsl-backup was inspected read-only; installed helper digest matched expected
  prefix `e9ecff3d22ed`, and the Codex ops source inclusion count was exactly 1.

Recovered proof-parser issue:

- Failing command shape: initial queue parser expected ASCII `--` in
  `NEXT_ACTIONS.md` headings and returned zero READY.
- Classification: recoverable command-shape/proof parser issue.
- Corrective action: reran using the repository's actual `### NA-....`
  heading grammar and explicit `- **ID:** D-....` decision records.
- Final result: READY_COUNT 1; READY NA-0505; NA-0504, NA-0503, and NA-0502
  DONE; D-0997 once; D-0998 once; D-0999 absent; duplicate decision count zero.

Startup main-health caveat:

- Required protection context `public-safety` completed success on current
  main.
- `qsc-adversarial-smoke` completed success.
- `qsc-linux-full-suite` and `macos-qsc-full-serial` were skipped under the
  docs/governance closeout policy for current main.
- Three non-required check runs on current main were red:
  `remote-handshake`, `remote-relay`, and `relay-ui-integration`.
- Branch protection required contexts did not include those three red runs.
  They were recorded as a non-blocking main-health caveat for this
  authorization-only lane.

## D391 response-file recovery

The original D392-required exact response path remained absent:

- `/home/victor/work/qsl/codex/responses/NA0504_closeout_restore_na0505_20260620T024511Z_D391.md`

This directive authorized bounded discovery under the response directory. The
bounded discovery used only the permitted filename patterns at max depth 1 and
found exactly one plausible D391 closeout response:

- `/home/victor/work/qsl/codex/responses/NA0504_closeout_restore_na0505_20260620T044423Z_D391.md`

That discovered response was copied into the proof root and consumed. The exact
path absence remains a residual evidence item, but it is not a stop condition
because D-0998, PR #1281, NEXT_ACTIONS, TRACEABILITY, the rolling journal, and
the D392 response also prove the NA-0504 closeout and NA-0505 restoration.

## NA-0504 / D391 / D392 inheritance

Inherited facts:

- D392 stopped only because the exact D391 response path was absent.
- No repository mutation occurred in D392.
- D391 response-file recovery succeeded through bounded discovery, as recorded
  above.
- NA-0504 completed and was closed by PR #1281.
- NA-0505 was restored READY as the sole READY item.
- Same-host client-to-client E2E test was implemented at
  `qsl/qsl-client/qsc/tests/same_host_client_to_client_e2e.rs`.
- Selected local Alice/Bob surfaces were tested: two independent temp roots,
  identity setup, public-record/trust exchange, send/receive, reply,
  wrong-mailbox reject/no-mutation, and stdout/stderr no-secret-output checks.
- Remote/LAN/two-machine testing was not executed.
- No remote accounts, SSH keys, SSH commands, SSH config, or remote host
  mutation occurred.
- No qsc source, dependency, workflow, corpus, vector, input, formal, refimpl,
  service, public, backup, qwork, qstart, qresume, or qsl-backup mutation
  occurred in this lane.

Claim boundaries:

- no public-readiness claim is made.
- no production-readiness claim is made.
- no public-internet-readiness claim is made.
- no external-review-complete claim is made.
- no crypto-complete claim is made.
- no replay-proof claim is made.
- no downgrade-proof claim is made.
- no secret-material-complete claim is made.
- no zeroization-complete claim is made.
- no memory-erasure-complete claim is made.
- no side-channel-free claim is made.
- no vulnerability-free, bug-free, or perfect-crypto claim is made.

Selected NA-0505 purpose:

- authorize the remote test account and SSH boundary for future work only.

## Remote test account / SSH boundary inventory

| Topic | Authorized boundary | Rationale |
| --- | --- | --- |
| Remote account role | Dedicated per-project remote test user, not root and not the operator's primary personal account | Keeps test artifacts, permissions, and revocation scoped to QSL remote test work. |
| Local operator role | Operator performs setup manually and records proof | Operator owns host trust, account creation, key creation, and host-key verification. |
| Codex role | Codex may write runbooks and later consume operator-supplied proof; Codex must not perform setup in NA-0505/NA-0506 | Prevents early exposure of SSH authority and secret-bearing material. |
| Future test command role | Future exact directive may authorize limited qsc commands only after account/key/alias proof exists | Keeps remote execution separated from authorization and setup. |
| Host alias model | Default `qsl-remote-test`; optional `remote` only after collision check and operator approval | Specific alias avoids accidental reuse of existing operator hosts. |
| SSH key model | Dedicated per-project key, generated and kept by operator outside Codex; key-only login | Avoids personal key reuse and keeps private material out of repo/proof/chat. |
| known_hosts / host-key model | Operator verifies host key out-of-band and pins it before future Codex use | Avoids Codex `ssh-keyscan` trust-on-first-use and known_hosts mutation. |
| Remote directory model | Dedicated work directory owned by the test user, with no production data or backup material | Restricts artifact blast radius and simplifies cleanup. |
| qsl-protocol checkout model | If needed later, operator-owned checkout or artifact placement under the dedicated work directory | Future tests should not assume shared developer home state. |
| Build/dependency model on remote | Operator-prepared dependencies only, documented by proof; no package install by Codex | Avoids sudo/admin and environmental drift by automation. |
| qwork/qbuild relationship | qwork/qbuild remain local qbuild authority; do not run qwork on the remote host | Keeps queue proof and governance execution tied to local qbuild state. |
| qsl-backup relationship | No qsl-backup execution and no access to `/backup/qsl` or backup private material | Prevents backup/key-custody exposure. |
| Logs/artifacts | Synthetic test artifacts only; proof must avoid secrets, route tokens, private keys, and long hex dumps | Maintains leak-safe evidence. |
| sudo/admin boundary | No sudo for the test account; no system service install; no privileged ports | Prevents remote test user from becoming an admin channel. |
| Production/user data boundary | No production data, personal home data, credentials, or backup material | Keeps remote testing non-production and claim-safe. |
| Firewall/network boundary | No public service exposure; future tests limited to explicit host/port paths authorized later | Avoids public-internet claim drift. |
| Remote host trust boundary | Remote host is treated as test infrastructure, not a trusted secret store | A compromised host should not expose personal keys or backups. |
| Cleanup/retention boundary | Operator documents cleanup and retention; future lanes must not leave durable secrets | Enables revocation and evidence hygiene. |
| Revocation boundary | Operator can remove authorized key and disable/remove test account | Provides a concrete stop path if key or host trust is lost. |

## Account capability table

| Capability | Allowed for future test account? | Boundary |
| --- | --- | --- |
| SSH login | Yes, after operator setup proof | Key-only, dedicated user, pinned host identity. |
| Password login | No, if feasible to disable | Password auth should not be required for the test account. |
| sudo/admin | No | Must not be in sudoers or privileged groups. |
| Read/write dedicated work directory | Yes | Only under operator-approved test path. |
| Read production/user data | No | No home sharing, prod mounts, or live user data. |
| Access `/backup/qsl` | No | No backup material or backup private data. |
| Run qwork/qstart/qresume | No | qbuild authority remains local. |
| Run qsl-backup | No | Backups stay out of remote test scope. |
| Bind privileged ports | No | Use only unprivileged ports if later authorized. |
| Install packages/system services | No for Codex; operator-only if later needed | No Codex sudo/admin path. |
| Run daemon/persistent service | No unless separately authorized | Remote E2E should start from bounded foreground commands. |
| Store private SSH key | No | Private key remains local operator material. |
| Store test artifacts | Yes | Synthetic, under dedicated work directory, with cleanup proof. |

## SSH key / host alias model

| Item | Selected model | Notes |
| --- | --- | --- |
| Default host alias | `qsl-remote-test` | Selected as the safe default. |
| Convenience host alias | `remote` only if operator explicitly approves after proving no local collision | Not selected as default. Future setup must check existing SSH config behavior first. |
| SSH private key | Dedicated per-project key, generated by the operator outside Codex | Codex must not run `ssh-keygen` or read private key material. |
| SSH public key | Installed by operator only | Codex must not mutate `authorized_keys`. |
| IdentityFile path | Operator-owned local path outside repo and proof text | Runbook should use placeholders, not private paths. |
| Strict host-key policy | Require explicit host-key verification and pinned known_hosts before future Codex use | Codex must not run `ssh-keyscan` in this lane. |
| Local config mutation | Operator-only, future runbook may document candidate entries | NA-0505 and NA-0506 must not mutate `~/.ssh/config`. |
| System config mutation | Forbidden | No `/etc/ssh` mutation. |
| Future `ssh remote` convenience | Deferred | Allowed only with collision proof and operator confirmation. |

## Future operator proof checklist

The NA-0506 runbook should require operator-collected proof for:

- dedicated remote username and host identifier, redacted as needed;
- no sudo/admin membership for the test account;
- key-only login posture for the test account, if feasible;
- dedicated public key installed only for that account;
- private key path not pasted into evidence if it reveals sensitive local layout;
- host key fingerprint verified out-of-band and pinned by the operator;
- `qsl-remote-test` alias configured by the operator;
- optional `remote` alias collision check and explicit operator approval before use;
- dedicated remote work directory ownership and permissions;
- no access to `/backup/qsl` or backup private material;
- no production data in the remote work directory;
- no qwork, qstart, qresume, or qsl-backup execution on the remote host;
- cleanup/retention and revocation steps documented;
- proof that no Codex remote command was run during setup.

## Explicit forbidden remote actions

NA-0505 forbids, and NA-0506 should continue to forbid:

- creating remote users by Codex;
- generating SSH keys by Codex;
- installing SSH keys by Codex;
- running SSH, scp, sftp, or rsync to a remote host by Codex;
- mutating local SSH config by Codex;
- mutating system SSH config by Codex;
- mutating known_hosts by Codex;
- mutating authorized_keys by Codex;
- mutating remote hosts by Codex;
- sudo/admin action by Codex;
- package installation by Codex;
- remote qwork/qstart/qresume execution;
- remote qsl-backup execution;
- access to `/backup/qsl`;
- production data use;
- public service exposure;
- remote/LAN client-to-client E2E execution before a later directive.

## Option review

| Option | Disposition | Risk reduced | Evidence gap addressed | Feasibility | Scope/secret/public-claim risk |
| --- | --- | --- | --- | --- | --- |
| 1. Remote Test Account / SSH Operator Setup Runbook Authorization Plan | Reject as already satisfied by NA-0505 | Confirms boundary | This evidence doc completes the authorization | High | Low, but another authorization-only lane would add delay. |
| 2. Remote Test Account / SSH Operator Setup Runbook Implementation Harness | Select | Turns boundary into operator steps and proof checklist | Exact setup instructions without Codex remote action | High | Low if restricted to docs/governance paths. |
| 3. Local SSH Config Boundary Authorization Plan | Defer | Narrows alias/config uncertainty | Alias details are important but not the highest standalone uncertainty | High | Low, but too narrow to solve account setup. |
| 4. Remote Host Capability Probe Authorization Plan | Defer | Would prove remote reachability | Too early before account/key/runbook proof exists | Medium | Medium; introduces SSH execution risk. |
| 5. Remote Client-to-Client E2E Implementation Plan | Defer | Would prove two-host behavior | Too early before SSH/account boundary exists | Medium | High; risks remote action and overclaim. |
| 6. Continue same-host E2E negative expansion | Defer | Adds local negative coverage | Same-host evidence already landed; remote boundary is current blocker | High | Low, but less direct. |
| 7. CI/tooling lane | Reject | Could improve automation | No tooling blocker prevents this authorization | Medium | Medium if workflows/scripts change. |
| 8. Self-hosted runner setup | Reject | Not the needed test boundary | Broader CI runner trust problem, not remote client-to-client testing | Low | High. |

Best-Known-Method Review: split authorization, operator setup runbook, operator
proof, capability probe, and remote E2E into separate lanes. Do not combine
remote authority with implementation.

Side-Channel Caveat: remote testing can add timing, host, network, and logging
observations, but this lane makes no side-channel-free claim.

Formal-Model Mapping Residual: the remote SSH/account boundary is operational
assurance, not formal protocol evidence. Formal mapping remains bounded to
existing model checks until a later formal lane maps remote observations to
protocol invariants.

External-Review Readiness: a least-privilege runbook improves reviewability,
but no external-review-complete claim is made.

Assurance Gap Review Trigger: before any remote command is authorized, verify
account proof, alias proof, key boundary proof, host-key proof, cleanup proof,
and no-backup/no-production boundaries.

## Hostile Cryptographer Review

If Codex receives remote SSH too early, it could accidentally create an
unreviewed execution channel that bypasses local qwork proof, mixes local and
remote state, or leaves secret-bearing artifacts outside the checked-in
evidence system. A cautious reviewer would require a dedicated non-root user,
no sudo, per-project key, pinned host key, explicit alias policy, and
operator-owned setup proof before any remote E2E.

Leak paths include private SSH keys, local personal data, remote operator data,
backup material, route tokens, copied logs, shell history, and durable remote
artifacts. The test account must not see backup paths, production data, or
personal key material.

Remote testing also risks claim drift: a successful LAN test is still no
public-readiness claim, no production-readiness claim, no
public-internet-readiness claim, no crypto-complete claim, no replay-proof
claim, no downgrade-proof claim, no secret-material-complete claim, and
no side-channel-free claim.

Alias decision: require `qsl-remote-test` by default. Permit `remote` only as a
secondary convenience alias after explicit collision check and operator
confirmation.

## Red-Team Review

An attacker would abuse the remote test account by pivoting through SSH,
reading residual artifacts, replacing test binaries, collecting logs, or
turning a convenience alias into a confused-deputy target. If the remote host
is compromised, assume remote worktree contents and artifacts are observable.
If the test key leaks, the operator must remove the public key, disable or
remove the test account, rotate any related host/alias proof, and invalidate
future use until a new proof bundle exists.

If Codex accidentally runs destructive commands remotely, lack of sudo, a
dedicated work directory, no production data, and no backup mounts reduce blast
radius. If qwork or backups are run remotely, queue/proof authority and backup
custody are blurred; those actions remain forbidden.

Cleanup/revocation evidence required:

- authorized key removed or rotated when needed;
- test account disabled/removed when no longer needed;
- dedicated work directory cleaned or retention justified;
- no backup material present;
- no production data present;
- no secret-shaped artifacts in copied evidence.

## Production SRE Review

Safe operator steps:

- create or identify a dedicated remote test user;
- create and protect a dedicated local SSH key outside Codex;
- install only the public key for that user;
- verify host key out of band;
- create a dedicated remote work directory;
- configure a specific local alias;
- collect proof using redacted, non-secret outputs.

Manual and explicit steps:

- account creation;
- key generation;
- public-key installation;
- host-key verification;
- local SSH config edits;
- package/build dependency preparation;
- cleanup and revocation.

Never automate by Codex in NA-0505 or NA-0506:

- sudo/admin;
- system SSH config edits;
- known_hosts edits;
- remote login/probe;
- package installation;
- qwork/qsl-backup on remote;
- production data handling.

Future remote test failures must not mutate local qbuild/qwork state. Remote
artifacts should remain under the dedicated remote work directory and be copied
into proof only by a later authorized, leak-safe process.

## Release-Claim Boundary Review

This lane preserves:

- no public-readiness claim.
- no production-readiness claim.
- no public-internet-readiness claim.
- no external-review-complete claim.
- no crypto-complete claim.
- no replay-proof claim.
- no downgrade-proof claim.
- no secret-material-complete claim.
- no side-channel-free claim.
- no vulnerability-free, bug-free, or perfect-crypto claim.

## Prioritization matrix

| Candidate | Risk reduced | Directness of evidence | Security risk | Operator burden | Feasibility | Scope risk | Secret/key risk | Public-claim risk | Recommended disposition | Next-lane |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Remote SSH operator setup runbook authorization | Medium | Medium | Low | Low | High | Low | Low | Low | Already completed by NA-0505 | No |
| Remote SSH operator setup runbook implementation | High | High | Low | Medium | High | Low | Low if no key material included | Low | Select | Yes |
| Local SSH config boundary authorization | Medium | Medium | Low | Low | High | Low | Low | Low | Defer | No |
| Remote host capability probe authorization | Medium | High | Medium | Medium | Medium | Medium | Medium | Medium | Defer | No |
| Remote client-to-client E2E implementation | High | High | High | High | Medium | High | High | High | Defer pending SSH boundary | No |
| Same-host E2E negative expansion | Medium | Medium | Low | Low | High | Low | Low | Low | Defer | No |
| CI/tooling lane | Low | Low | Medium | Medium | Medium | Medium | Low | Medium | Reject | No |
| Self-hosted runner setup | Low for this lane | Low | High | High | Low | High | High | High | Reject | No |

## Authorization decision

Primary classification:
`REMOTE_SSH_OPERATOR_RUNBOOK_IMPLEMENTATION_READY`.

Rationale:

- NA-0504/D391/D392 inheritance was consumed.
- D391 response-file recovery was handled through bounded discovery.
- Remote boundary inventory was completed.
- Option review was completed.
- Hostile Cryptographer, Red-Team, Production SRE, and Release-Claim reviews
  were completed.
- Prioritization matrix was completed.
- Remote probing and remote E2E remain premature.
- The safest next step is an in-repo operator setup runbook and proof checklist
  that keeps Codex away from remote systems and secrets.

## Selected NA-0506 successor

`NA-0506 -- QSL Remote Test Account / SSH Operator Setup Runbook Implementation Harness`

Status: READY

Goals: G1, G2, G3, G4, G5

## Future scope bundle

Objective:

Create an in-repo operator runbook and proof checklist for setting up a
least-privilege remote test account and SSH boundary for future remote
client-to-client testing, without Codex creating accounts, generating keys,
installing keys, running SSH, mutating SSH config, mutating remote hosts, or
running remote tests.

Allowed scope:

- `docs/governance/evidence/NA-0506_qsl_remote_test_account_ssh_operator_setup_runbook.md`
- `tests/NA-0506_qsl_remote_test_account_ssh_operator_setup_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden scope:

- creating remote users by Codex;
- generating or installing SSH keys by Codex;
- running SSH, scp, sftp, or rsync to remote;
- mutating local SSH config;
- mutating system SSH config;
- mutating known_hosts;
- mutating remote hosts;
- sudo/admin action;
- qwork/qstart/qresume mutation;
- qsc source/test/fuzz/Cargo mutation;
- workflow/dependency mutation;
- corpus/vector/input mutation;
- formal/refimpl/service/public/backup mutation;
- no public-readiness claim and no production-readiness claim.

Deliverables:

- operator setup runbook;
- proof checklist;
- decision;
- TRACEABILITY update;
- rolling journal update.

Acceptance criteria:

- remote account model documented;
- SSH key model documented;
- host alias model documented;
- future operator steps documented;
- proof checklist documented;
- no remote action performed;
- no key material included;
- no sudo/admin/backup exposure authorized;
- exactly one READY item remains after closeout.

## Future validation / marker plan

Common future NA-0506 markers:

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

## Remote/LAN deferral

Remote/LAN/two-machine client-to-client testing remains deferred. The next lane
must implement only the operator runbook and proof checklist. A later directive
must separately authorize any SSH probe or remote qsc command.

## Public claim / website / external review boundary

NA-0505 changes no website, README, START_HERE, public docs, or public-facing
claims. The evidence is internal governance support only. No external review is
completed by this lane.

## Backup-impact statement

Backup impact: none. NA-0505 mutates only tracked qsl-protocol governance,
testplan, traceability, and rolling-journal files. It does not run backup or
restore, mutate qsl-backup, access `/backup/qsl`, or expose backup private
material.

## Rejected alternatives

- `remote` as the default alias is rejected because it risks collision with
  existing operator config and ambiguous future instructions.
- Codex-generated keys are rejected because private key custody must remain
  operator-owned and outside Codex.
- Codex-installed keys are rejected because they mutate remote trust state.
- Capability probing is deferred because it would run SSH before setup proof.
- Remote E2E implementation is deferred because account/key/host proof does
  not yet exist.
- Self-hosted runner setup is rejected because it introduces a broader CI
  runner trust problem unrelated to this client-to-client test boundary.

## Next recommendation

Proceed to NA-0506 as a runbook implementation harness. The runbook should make
`qsl-remote-test` the default alias, include an explicit collision check before
any optional `remote` convenience alias, and require operator proof for account,
key, host-key, no-sudo, no-backup, no-production-data, cleanup, and revocation
boundaries.
