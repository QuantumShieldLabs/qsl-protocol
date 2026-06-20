Status: Supporting
Owner: QSL Governance / Core Assurance
Last-Updated: 2026-06-20

# NA-0509 Remote Host Capability Probe Scope Authorization Plan

## Executive summary

NA-0509 consumes NA-0508/D397 inheritance and authorizes the exact future
remote host capability probe scope for the approved `inspiron` / `qslcodex`
test account. This lane is authorization-only. Codex did not run SSH, did not
run remote commands, did not generate or install keys, did not mutate SSH
configuration, did not mutate known_hosts, and did not mutate the remote host.

Primary classification:

- `REMOTE_READ_ONLY_CAPABILITY_PROBE_IMPLEMENTATION_READY`

Selected successor:

- `NA-0510 -- QSL Remote Host Read-Only Capability Probe Implementation Harness`

The selected NA-0510 successor may execute one bounded read-only SSH capability
probe only after a future directive and fresh qwork proof authorize it. The
remote write-marker probe and remote client-to-client E2E remain deferred.

This lane makes no public-readiness claim, no production-readiness claim, no
public-internet-readiness claim, no external-review-complete claim, no
crypto-complete claim, no replay-proof claim, no downgrade-proof claim, no
secret-material-complete claim, no side-channel-free claim, no vulnerability-free
claim, no bug-free claim, and no perfect-crypto claim.

## Live NA-0509 scope

Allowed mutation paths for this evidence PR:

- `docs/governance/evidence/NA-0509_qsl_remote_host_capability_probe_scope_authorization_plan.md`
- `tests/NA-0509_qsl_remote_host_capability_probe_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Protected boundaries:

- authorization-only governance evidence.
- no remote action by Codex.
- no SSH execution by Codex.
- no scp, sftp, or rsync to remote by Codex.
- no ssh-keygen or ssh-keyscan by Codex.
- no remote account creation by Codex.
- no SSH key generation or installation by Codex.
- no local or system SSH config mutation by Codex.
- no known_hosts or authorized_keys mutation by Codex.
- no remote host mutation by Codex.
- no sudo/admin action by Codex.
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

- `/srv/qbuild/work/NA-0509/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0509/.qwork/startup.qsl-protocol.json`

Verified required fields:

- `startup_result=OK`
- `lane=NA-0509`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0509/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0509`
- `requested_lane_status=READY`

Freshness:

- proof HEAD matched live pre-fetch HEAD at `f6d142035911`.
- proof origin/main matched live pre-fetch origin/main at `f6d142035911`.
- fetch occurred only after proof/live match and disk proof below the 95% stop
  threshold.

Startup queue and decision proof:

- READY_COUNT was 1.
- READY item was NA-0509.
- NA-0508, NA-0507, and NA-0506 were DONE.
- D-1005 existed once.
- D-1006 existed once.
- D-1007 was absent before this patch.
- duplicate decision record count was zero.

Startup main health:

- `origin/main` equaled `f6d142035911`.
- `origin/main` descends from `f6d142035911`.
- `public-safety` completed success.
- `qsc-adversarial-smoke` completed success.
- `qsc-linux-full-suite` and `macos-qsc-full-serial` were skipped under the
  accepted docs/governance closeout policy.
- no completed red checks were present in the retrieved check-run set.

Read-only backup boundary proof:

- installed qsl-backup helper SHA-256 matched
  `e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`.
- the configured Codex ops source appeared exactly once in the installed helper
  source list.
- Codex did not run backup, restore, or qsl-backup.

## NA-0508 / D397 inheritance

Consumed:

- D397 response:
  `/home/victor/work/qsl/codex/responses/NA0508_20260620T211615Z_D397.md`
- NA-0508 evidence:
  `docs/governance/evidence/NA-0508_qsl_remote_test_account_ssh_operator_setup_proof_review.md`
- NA-0508 proof-review testplan:
  `tests/NA-0508_qsl_remote_test_account_ssh_operator_setup_proof_review_testplan.md`
- D-1005 and D-1006 in `DECISIONS.md`
- NA-0509 READY block in `NEXT_ACTIONS.md`
- same-host client-to-client E2E test path as inherited local-only evidence:
  `qsl/qsl-client/qsc/tests/same_host_client_to_client_e2e.rs`
- read-only qsl-backup boundary evidence.

Inherited facts:

- NA-0508 completed.
- NA-0509 was restored READY.
- operator setup proof was accepted as bounded setup/account proof only.
- the proof was not remote E2E evidence.
- remote alias was `inspiron`.
- remote account was `qslcodex`.
- optional alias `remote` was not approved/configured.
- `qslcodex` account boundary was accepted as non-root, non-sudo, and not in
  privileged groups.
- remote workdir `qsl-remote-test` exists and is writable.
- qwork is absent remotely.
- qsl-backup is absent remotely.
- `/backup/qsl` is absent or not readable remotely.
- host key fingerprint match was accepted.
- public key fingerprint match was accepted.
- no private key, passphrase, token, password, production endpoint, or backup
  material was included.
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

Selected NA-0509 purpose:

- authorize exact future remote capability probe commands and redaction
  boundaries without running remote commands.

## Prior operator proof boundary

The accepted operator proof is a setup-boundary proof. It supports the next
read-only capability probe because it established a least-privilege test account
and key-only SSH boundary, but it does not prove qsc behavior across hosts.

Accepted bounded proof:

- alias `inspiron`.
- account `qslcodex`.
- optional alias `remote` not approved/configured.
- non-root account.
- no sudo/admin capability.
- no privileged groups.
- `$HOME/qsl-remote-test` exists and is writable.
- qwork absent remotely.
- qsl-backup absent remotely.
- `/backup/qsl` absent or not readable remotely.
- host key fingerprint matched the operator-approved fingerprint.
- public key fingerprint matched the installed public key fingerprint.

Not accepted as proof:

- remote qsc send/receive.
- remote qsl-protocol checkout/build.
- remote daemon/service behavior.
- public-readiness.
- production-readiness.
- public-internet-readiness.
- crypto-complete, replay-proof, downgrade-proof, or secret-material-complete
  status.

## Remote capability probe scope design

The future NA-0510 probe should be the smallest useful remote check:

1. Locally verify the `inspiron` effective SSH configuration from `ssh -G`
   redacted output only.
2. Locally verify that optional alias `remote` has not become an approved
   qslcodex route unless a future directive explicitly approves it.
3. Execute one non-interactive SSH invocation against `inspiron` with
   `BatchMode=yes`, `PasswordAuthentication=no`, and `ConnectTimeout=10`.
4. Run only a bounded read-only remote script that checks identity, UID, groups,
   workdir existence/writability, negative sudo capability, no readable backup
   path, and absent qwork/qsl-backup.
5. Save raw proof only under the future directive proof root and paste back only
   redacted summaries and fixed markers.

The selected future probe checks remote workdir writability with `test -w` only.
It does not write a marker file. The write-marker probe is deferred because
read-only capability evidence is enough before authorizing any remote file
mutation.

## Exact future command list

NA-0509 authorizes the following future command family for NA-0510 only. These
commands were not run in NA-0509.

### Local pre-SSH proof for `inspiron`

Future NA-0510 may run a local effective-config command and save only the
redacted output:

```bash
ssh -G inspiron | python3 -c '
import os, sys
allowed = {
    "hostname",
    "user",
    "batchmode",
    "passwordauthentication",
    "stricthostkeychecking",
    "forwardagent",
    "forwardx11",
    "clearallforwardings",
    "identitiesonly",
    "identityfile",
}
for raw in sys.stdin:
    parts = raw.strip().split(None, 1)
    if len(parts) != 2:
        continue
    key, value = parts[0].lower(), parts[1]
    if key not in allowed:
        continue
    if key == "identityfile":
        print(f"{key}=<redacted-basename:{os.path.basename(value)}>")
    elif key == "hostname":
        print(f"{key}=<redacted-or-approved>")
    else:
        print(f"{key}={value}")
'
```

Expected required effective fields:

- `user=qslcodex`
- `batchmode=yes`
- `passwordauthentication=no`
- `stricthostkeychecking=yes` or `stricthostkeychecking=true`
- `forwardagent=no`
- `forwardx11=no`
- `clearallforwardings=yes`
- `identitiesonly=yes`

### Local optional-alias proof for `remote`

Future NA-0510 may run local effective-config inspection for `remote` and save
only redacted `hostname` and `user` fields:

```bash
ssh -G remote | python3 -c '
import sys
for raw in sys.stdin:
    parts = raw.strip().split(None, 1)
    if len(parts) != 2:
        continue
    key, value = parts[0].lower(), parts[1]
    if key == "hostname":
        print("hostname=<redacted-or-literal-remote>")
    elif key == "user":
        print(f"user={value}")
'
```

Expected result:

- `remote` must not resolve as an approved qslcodex remote test route unless a
  future directive explicitly approves it.
- If `remote` appears to target the approved host or use `qslcodex`, future
  NA-0510 must stop before remote execution.

### One bounded read-only remote probe

Future NA-0510 may run exactly one remote SSH invocation:

```bash
ssh -o BatchMode=yes -o PasswordAuthentication=no -o ConnectTimeout=10 inspiron 'bash -s' <<'REMOTE_PROBE'
set -u

printf '%s\n' 'NA0510_REMOTE_PROBE_BEGIN'

remote_hostname="$(hostname)"
remote_user="$(id -un)"
remote_uid="$(id -u)"
remote_groups="$(id -Gn)"
remote_pwd="$(pwd)"
remote_home="$HOME"

printf 'REMOTE_HOSTNAME=%s\n' "$remote_hostname"
printf 'REMOTE_USER=%s\n' "$remote_user"
printf 'REMOTE_UID=%s\n' "$remote_uid"
printf 'REMOTE_GROUPS=%s\n' "$remote_groups"
printf 'REMOTE_PWD=%s\n' "$remote_pwd"
printf 'REMOTE_HOME=%s\n' "$remote_home"

if [ "$remote_user" = "qslcodex" ]; then
  printf '%s\n' 'NA0510_REMOTE_ACCOUNT_QSLCODEX_OK'
else
  printf '%s\n' 'STOP_REMOTE_ACCOUNT_NOT_QSLCODEX'
  exit 20
fi

if [ "$remote_uid" != "0" ]; then
  printf '%s\n' 'NA0510_REMOTE_NOT_ROOT_OK'
else
  printf '%s\n' 'STOP_REMOTE_UID_ROOT'
  exit 21
fi

case " $remote_groups " in
  *" sudo "*|*" adm "*|*" admin "*|*" wheel "*|*" docker "*|*" lxd "*|*" libvirt "*")
    printf '%s\n' 'STOP_REMOTE_PRIVILEGED_GROUP'
    exit 22
    ;;
  *)
    printf '%s\n' 'NA0510_REMOTE_GROUPS_NON_PRIVILEGED_OK'
    ;;
esac

if test -d "$HOME/qsl-remote-test"; then
  printf '%s\n' 'NA0510_REMOTE_WORKDIR_EXISTS_OK'
else
  printf '%s\n' 'STOP_REMOTE_WORKDIR_MISSING'
  exit 23
fi

if test -w "$HOME/qsl-remote-test"; then
  printf '%s\n' 'NA0510_REMOTE_WORKDIR_WRITABLE_OK'
else
  printf '%s\n' 'STOP_REMOTE_WORKDIR_NOT_WRITABLE'
  exit 24
fi

if sudo -n true >/dev/null 2>&1; then
  printf '%s\n' 'STOP_REMOTE_SUDO_AVAILABLE'
  exit 25
else
  printf '%s\n' 'NA0510_REMOTE_NO_SUDO_OK'
fi

if test -e /backup/qsl; then
  printf '%s\n' 'REMOTE_BACKUP_PATH_EXISTS'
else
  printf '%s\n' 'REMOTE_BACKUP_PATH_ABSENT'
fi

if test -r /backup/qsl; then
  printf '%s\n' 'STOP_REMOTE_BACKUP_READABLE'
  exit 26
else
  printf '%s\n' 'NA0510_REMOTE_NO_BACKUP_EXPOSURE_OK'
fi

if command -v qwork >/dev/null 2>&1; then
  printf '%s\n' 'STOP_REMOTE_QWORK_PRESENT'
  exit 27
else
  printf '%s\n' 'NA0510_REMOTE_QWORK_ABSENT_OK'
fi

if command -v qsl-backup >/dev/null 2>&1; then
  printf '%s\n' 'STOP_REMOTE_QSL_BACKUP_PRESENT'
  exit 28
else
  printf '%s\n' 'NA0510_REMOTE_QSL_BACKUP_ABSENT_OK'
fi

printf '%s\n' 'NA0510_NO_REMOTE_FILE_WRITE_OK'
printf '%s\n' 'NA0510_NO_REMOTE_E2E_OK'
printf '%s\n' 'NA0510_REMOTE_READ_ONLY_PROBE_EXECUTED_OK'
printf '%s\n' 'NA0510_REMOTE_PROBE_END'
REMOTE_PROBE
```

No other remote command family is authorized by NA-0509.

## Expected future outputs

Expected local pre-SSH outputs:

- redacted `hostname=<redacted-or-approved>` for `inspiron`.
- `user=qslcodex`.
- `batchmode=yes`.
- `passwordauthentication=no`.
- `stricthostkeychecking=yes` or `stricthostkeychecking=true`.
- `forwardagent=no`.
- `forwardx11=no`.
- `clearallforwardings=yes`.
- `identitiesonly=yes`.
- redacted identity file basename only, if identityfile is printed.
- optional alias `remote` output summarized as not approved/configured for the
  qslcodex route.

Expected remote stdout markers:

- `NA0510_REMOTE_PROBE_BEGIN`
- `REMOTE_HOSTNAME=<redacted-or-approved>`
- `REMOTE_USER=qslcodex`
- `REMOTE_UID=<nonzero>`
- `REMOTE_GROUPS=<redacted-or-non-privileged-summary>`
- `REMOTE_PWD=<redacted-or-approved>`
- `REMOTE_HOME=<redacted-or-approved>`
- `NA0510_REMOTE_ACCOUNT_QSLCODEX_OK`
- `NA0510_REMOTE_NOT_ROOT_OK`
- `NA0510_REMOTE_GROUPS_NON_PRIVILEGED_OK`
- `NA0510_REMOTE_WORKDIR_EXISTS_OK`
- `NA0510_REMOTE_WORKDIR_WRITABLE_OK`
- `NA0510_REMOTE_NO_SUDO_OK`
- either `REMOTE_BACKUP_PATH_ABSENT` or `REMOTE_BACKUP_PATH_EXISTS`
- `NA0510_REMOTE_NO_BACKUP_EXPOSURE_OK`
- `NA0510_REMOTE_QWORK_ABSENT_OK`
- `NA0510_REMOTE_QSL_BACKUP_ABSENT_OK`
- `NA0510_NO_REMOTE_FILE_WRITE_OK`
- `NA0510_NO_REMOTE_E2E_OK`
- `NA0510_REMOTE_READ_ONLY_PROBE_EXECUTED_OK`
- `NA0510_REMOTE_PROBE_END`

Expected remote exit status:

- zero only when all read-only capability checks pass.
- nonzero on any stop marker.

## Redaction rules

Future NA-0510 must apply these rules before any evidence, PR body, or response
paste-back:

- Never include private keys, passphrases, tokens, passwords, credentials,
  production endpoints, backup material, private host material, or long
  secret-like dumps.
- Do not paste full `ssh -G` output.
- Do not paste full private key paths; identityfile output may include only a
  basename or `<redacted-basename:...>`.
- Redact hostnames and IP addresses unless the Director explicitly marks the
  value as safe to publish.
- Redact `$HOME`, `pwd`, and remote host labels when they reveal sensitive
  local naming or topology.
- Summarize groups as non-privileged unless exact group names are needed to
  prove a failure.
- Preserve fixed markers exactly.
- Save raw command output only under the future proof root; durable repo docs
  may contain only redacted summaries.
- If any output cannot be confidently redacted, stop before PR creation.

## Stop conditions

Future NA-0510 must stop before or during the probe if any condition occurs:

- qwork proof is stale, missing, or not for the expected lane.
- `inspiron` effective config does not show `User qslcodex`.
- `BatchMode` is not yes.
- `PasswordAuthentication` is not no.
- `StrictHostKeyChecking` is not yes/true.
- `ForwardAgent` is not no.
- `ForwardX11` is not no.
- `ClearAllForwardings` is not yes.
- `IdentitiesOnly` is not yes.
- optional alias `remote` appears to target the approved host or `qslcodex`
  without explicit future approval.
- SSH host-key validation fails, changes, or becomes ambiguous.
- remote command cannot connect with the approved strict options.
- remote `id -un` is not `qslcodex`.
- remote UID is 0.
- remote groups include sudo, adm, admin, wheel, docker, lxd, libvirt, or other
  privileged groups.
- `$HOME/qsl-remote-test` is missing.
- `$HOME/qsl-remote-test` is not writable by `qslcodex`.
- `sudo -n true` returns success.
- `/backup/qsl` is readable.
- qwork is present remotely.
- qsl-backup is present remotely.
- any remote file write occurs.
- any remote E2E, qsc protocol send/receive, checkout, build, daemon, or service
  action is attempted.
- private key, passphrase, token, password, credential, production endpoint, or
  backup material appears in output.
- redaction is ambiguous.
- any public/security/completion overclaim would be required to explain the
  result.

## No remote E2E boundary

NA-0509 authorizes no remote E2E. Future NA-0510 also remains no remote E2E.
The future probe must not run qsc send/receive, must not build or checkout
qsl-protocol on the remote host, must not run a daemon, must not install a
service, and must not treat account capability proof as remote protocol proof.

## Option review

| Option | Disposition | Risk reduced | Evidence gap addressed | Feasibility | Scope risk | Remote mutation risk | Secret/key risk | Public-claim risk | Likely future allowed paths | Likely future forbidden paths | P0/P1/P2 risks |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Option 1 - Remote Host Read-Only Capability Probe Implementation Harness | Select | Verifies account/host boundary before any remote protocol work | Confirms current alias, identity, no sudo, no backup, no qwork/qsl-backup, and workdir state | High | Low | Low | Low if redacted | Medium if overread, mitigated by no-claim wording | NA-0510 evidence/testplan/decision/traceability/journal and proof-root captures | remote E2E, remote writes, source/build, key/config mutation | P0: host/account drift; P1: redaction miss; P2: operator rerun burden |
| Option 2 - Remote Host Read/Write Marker Capability Probe Implementation Harness | Defer | Would prove create/read/delete in workdir | Write proof is useful later but not necessary before read-only capability | Medium | Medium | Medium | Low if marker contains no secret | Medium if overread | Later evidence/testplan if read-only probe passes | any write before explicit later authorization | P0: accidental persistent file; P1: cleanup failure; P2: extra operator review |
| Option 3 - Remote Host Capability Probe Split Authorization | Reject for now | Could reduce ambiguity if commands were unclear | Not needed; command family and redaction rules are exact enough | High | Low | Low | Low | Low | none now | same as NA-0509 | P1: unnecessary delay; P2: duplicated governance |
| Option 4 - Remote Client-to-Client E2E Authorization | Defer | Would scope future remote protocol proof | Too early until capability probe passes | Medium | High | Medium | Medium | High if overread | future authorization docs only | implementation, remote writes, protocol run | P0: premature trust in remote; P1: broad scope; P2: unclear failures |
| Option 5 - Remote Client-to-Client E2E Implementation | Reject/defer | Would test remote protocol behavior | Premature and too broad before capability proof | Low now | High | High | Medium | High | none now | qsc remote execution, source checkout/build | P0: protocol evidence misread; P1: host mutation; P2: long debug cycle |
| Option 6 - Same-host E2E negative expansion | Defer | Adds local negative coverage | Useful if remote probe is blocked, but not the direct next gap | Medium | Medium | None | Low | Low | qsc test/evidence only if later selected | remote work | P1: displaces remote readiness proof; P2: redundant local coverage |
| Option 7 - Remote setup remediation | Reject unless future failure | Repairs weak setup | NA-0508 proof did not show boundary weakness requiring remediation | Medium | Medium | Medium | Medium | Medium | future runbook/remediation docs if failure occurs | Codex remote setup, key install, sudo | P0: privilege drift; P1: manual setup error; P2: proof churn |
| Option 8 - CI/tooling lane | Reject | Would fix process blocker | No process blocker prevents authorization | Medium | Medium | None | Low | Low | CI/tooling docs only if blocker appears | workflow/helper mutation now | P1: unnecessary process churn; P2: delays remote proof |

## Hostile Cryptographer Review

Does a read-only capability probe add useful assurance without being overread as
remote protocol evidence?

- Yes, if it is framed only as host/account capability evidence. It proves the
  configured alias reaches the expected least-privilege account and that known
  negative boundaries still hold. It does not prove qsc cryptographic behavior,
  transcript binding, downgrade resistance, replay behavior, or remote E2E.

Which future outputs could leak private network details or sensitive paths?

- `ssh -G` hostname output, identityfile paths, remote `hostname`, `$HOME`,
  `pwd`, and exact group names can reveal topology or local naming. Future
  evidence must redact or summarize those values.

Does checking `sudo -n true` risk privilege misuse?

- The risk is low because the command is noninteractive, bounded, and uses only
  `true`. It must be interpreted as a negative capability check. A zero exit is
  a hard stop and must not be used to perform any privileged action.

Does probe evidence risk becoming public/production readiness evidence?

- Yes, if wording drifts. NA-0510 must state that the probe is host/account
  capability evidence only and makes no public-readiness claim, no
  production-readiness claim, no public-internet-readiness claim, no
  crypto-complete claim, no replay-proof claim, and no downgrade-proof claim.

Is remote write proof necessary before remote E2E, or can it wait?

- It can wait. `test -w` gives enough pre-E2E capability signal without creating
  remote artifacts. A later write-marker lane can be authorized only if the
  read-only probe passes and the Director decides write proof is necessary.

## Red-Team Review

What if `inspiron` is retargeted?

- Future NA-0510 must stop if `ssh -G inspiron` output, strict host-key
  validation, or remote identity markers are inconsistent or ambiguous.

What if `qslcodex` gains sudo?

- Future NA-0510 must stop on `sudo -n true` success. A sudo-capable account is
  outside the accepted least-privilege boundary.

What if `/backup/qsl` appears or becomes readable?

- Existence alone is recorded, but readability is a hard stop because it creates
  backup exposure for the test account.

What if qwork/qsl-backup appears remotely?

- Future NA-0510 must stop. The remote test host must not become coupled to the
  qbuild/qwork or qsl-backup operational surfaces.

What if the future probe command writes files accidentally?

- Any remote file write is a hard stop and invalidates the read-only probe.
  NA-0509 authorizes no marker-file write.

What if command output contains hostnames/IPs or sensitive paths?

- Raw output remains proof-root local. Repo docs, PR body, and response text use
  redacted or summarized values only.

What cleanup/revocation proof is needed before remote E2E?

- Before remote E2E authorization, the Director should require fresh proof that
  the account remains non-root/no-sudo, no backup exposure exists, qwork and
  qsl-backup remain absent, the workdir remains bounded, and cleanup/revocation
  steps are documented for the key and account.

## Production SRE Review

What is the smallest useful remote probe?

- One local `ssh -G` redacted config proof plus one bounded read-only SSH
  invocation that checks identity, privileges, workdir, backup exposure, and
  forbidden tool presence.

Which future probe results should be hard stop conditions?

- Alias drift, host-key ambiguity, unexpected user, UID 0, privileged group,
  sudo success, missing/non-writable workdir, readable backup path, qwork or
  qsl-backup present, remote write, remote E2E, private material in output, or
  ambiguous redaction.

How should future remote output be redacted?

- Keep raw output under the proof root only. Replace hostnames, IPs, identity
  paths, `$HOME`, and `pwd` with approved labels or summaries. Preserve fixed
  pass/fail markers exactly.

Why should remote E2E remain deferred until after a capability probe?

- E2E failures are ambiguous unless the account, SSH, privilege, workdir, and
  backup boundaries are known current. The read-only probe isolates host/setup
  drift from protocol behavior.

How should future remote probe failures be isolated from local qbuild/qwork
state?

- Save all future proof under the directive proof root, never run qwork remotely,
  never run qsl-backup remotely, and treat remote failure as remote-boundary
  evidence rather than local queue or qbuild state.

## Release-Claim Boundary Review

This lane preserves the following claim boundaries:

- no public-ready claim is made.
- no production-ready claim is made.
- no public-internet-ready claim is made.
- no external-review-complete claim is made.
- no crypto-complete claim is made.
- no replay-proof claim is made.
- no downgrade-proof claim is made.
- no secret-material-complete claim is made.
- no side-channel-free claim is made.
- no vulnerability-free claim is made.
- no bug-free claim is made.
- no perfect-crypto claim is made.

Side-channel caveat:

- The future read-only capability probe does not measure timing, traffic
  fingerprinting, host side channels, SSH metadata leakage, or qsc protocol
  side-channel behavior.

Formal-model mapping residual:

- This authorization maps to operational capability gating, not to a formal
  protocol state-machine proof. Formal model coverage remains unchanged.

External-review readiness:

- The evidence is structured for reviewer inspection, but no external-review-
  complete claim is made.

Assurance gap review trigger:

- If future NA-0510 finds alias drift, privilege drift, backup exposure,
  qwork/qsl-backup presence, or redaction ambiguity, the queue should select
  remediation or split authorization before remote E2E.

## Prioritization matrix

| Candidate | Risk reduced | Directness of evidence | Security risk | Operator burden | Implementation feasibility | Scope risk | Remote mutation risk | Secret/key risk | Public-claim risk | Recommended disposition | Next-lane yes/no |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Read-only remote capability probe implementation | High for host/account drift | Direct for capability, not protocol | Low | Low | High | Low | Low | Low with redaction | Medium if overread | Select | Yes |
| Read/write marker remote capability probe implementation | Medium for actual write lifecycle | Direct for write capability | Medium | Medium | Medium | Medium | Medium | Low | Medium | Defer | No |
| Remote capability probe split authorization | Low incremental | Indirect | Low | Medium | High | Low | Low | Low | Low | Reject for now | No |
| Remote client-to-client E2E authorization | Future protocol scope | Indirect until probe passes | Medium | Medium | Medium | High | Medium | Medium | High | Defer | No |
| Remote client-to-client E2E implementation | High only after prerequisites | Direct protocol evidence | High now | High | Low now | High | High | Medium | High | Reject/defer | No |
| Same-host E2E negative expansion | Local negative coverage | Direct local evidence | Low | Low | Medium | Medium | None | Low | Low | Defer | No |
| Remote setup remediation | Fixes setup weakness | Direct if weakness exists | Medium | High | Medium | Medium | Medium | Medium | Medium | Reject unless future failure | No |
| CI/tooling lane | Process reliability | Indirect | Low | Medium | Medium | Medium | None | Low | Low | Reject | No |

## Authorization decision

Primary classification:

- `REMOTE_READ_ONLY_CAPABILITY_PROBE_IMPLEMENTATION_READY`

Decision:

- NA-0508/D397 is consumed.
- operator setup proof is accepted as setup-boundary proof only.
- remote probe scope is designed.
- exact future command family is selected.
- expected outputs are selected.
- redaction rules are selected.
- stop conditions are selected.
- read-only remote capability probe is selected for NA-0510.
- remote write-marker probe is deferred.
- remote E2E is deferred.
- no remote action occurs in NA-0509.
- no key generation, key installation, SSH config mutation, known_hosts
  mutation, or remote host mutation occurs in NA-0509.
- no qsc source/test/fuzz/Cargo mutation occurs in NA-0509.
- no workflow/script/helper/dependency mutation occurs in NA-0509.
- no corpus/vector/input mutation occurs in NA-0509.
- no formal/refimpl/service/public/backup mutation occurs in NA-0509.
- exactly one READY successor remains mandatory.

## Selected NA-0510 successor

### NA-0510 -- QSL Remote Host Read-Only Capability Probe Implementation Harness

Status: READY
Goals: G1, G2, G3, G4, G5

Objective:

Execute a single bounded read-only SSH capability probe against the approved
`inspiron` / `qslcodex` remote test account to verify account identity, no
sudo/admin access, workdir existence/writability, no backup exposure, no remote
qwork/qsl-backup presence, and no alias/host drift, without remote E2E, without
remote source checkout/build, without file writes, and without public/production
readiness claims.

Allowed scope:

- `docs/governance/evidence/NA-0510_qsl_remote_host_read_only_capability_probe_implementation_harness.md`
- `tests/NA-0510_qsl_remote_host_read_only_capability_probe_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- proof-root-local capture of the exact future SSH command output.
- one bounded SSH command to `inspiron` as `qslcodex` using the preconfigured
  local alias, if and only if qwork proof and lane scope authorize it.

Forbidden scope:

- remote E2E.
- qsc protocol execution remotely.
- remote source checkout/build.
- remote file creation/write/delete.
- remote package install.
- sudo/admin action other than negative `sudo -n true` probe.
- key generation or installation.
- SSH config mutation.
- known_hosts mutation.
- remote host mutation.
- qwork/qstart/qresume mutation.
- qsl-backup execution.
- qsc source/test/fuzz/Cargo mutation.
- workflow/dependency mutation.
- corpus/vector/input mutation.
- formal/refimpl/service/public/backup mutation.
- public-readiness or production-readiness claims.

Deliverables:

- remote capability probe implementation evidence.
- testplan.
- decision.
- TRACEABILITY update.
- rolling journal update.
- selected future remote write-probe or remote E2E authorization scope, or
  remediation/no-action rationale.

Acceptance criteria:

- qwork proof fresh.
- exact remote command captured.
- redaction rules applied.
- account identity `qslcodex` verified.
- non-root verified.
- no sudo/admin verified.
- workdir exists and writable verified.
- no backup exposure verified.
- no qwork/qsl-backup presence verified.
- no remote file writes.
- no remote E2E.
- no key material included.
- exactly one READY item remains after closeout.

## Future scope bundle

Allowed future remote command family:

- one non-interactive SSH invocation:
  `ssh -o BatchMode=yes -o PasswordAuthentication=no -o ConnectTimeout=10 inspiron 'bash -s'`
- remote script limited to read-only identity/boundary commands:
  - `hostname`
  - `id -un`
  - `id -u`
  - `id -Gn`
  - `pwd`
  - `printf` of `$HOME`
  - `test -d "$HOME/qsl-remote-test"`
  - `test -w "$HOME/qsl-remote-test"`
  - `sudo -n true` as negative capability check only
  - `test -e /backup/qsl`
  - `test -r /backup/qsl`
  - `command -v qwork`
  - `command -v qsl-backup`
  - `printf` fixed markers

Future proof artifacts:

- qwork startup proof copies.
- pre-fetch live ref proof.
- redacted `ssh -G inspiron` effective-config proof.
- redacted `ssh -G remote` optional-alias proof.
- exact remote command text.
- raw stdout/stderr under proof root only.
- redacted stdout/stderr summaries for repo evidence.
- remote exit-code proof.
- marker proof.
- no-private-key/no-token/no-password scan proof.
- no remote file-write proof by fixed marker and command review.
- public-safety and required CI proof after PR merge.

## Future validation / marker plan

Common future NA-0510 markers:

- `NA0510_REMOTE_PROBE_SCOPE_CONSUMED_OK`
- `NA0510_REMOTE_READ_ONLY_PROBE_EXECUTED_OK`
- `NA0510_REMOTE_ACCOUNT_QSLCODEX_OK`
- `NA0510_REMOTE_NOT_ROOT_OK`
- `NA0510_REMOTE_NO_SUDO_OK`
- `NA0510_REMOTE_WORKDIR_EXISTS_OK`
- `NA0510_REMOTE_WORKDIR_WRITABLE_OK`
- `NA0510_REMOTE_NO_BACKUP_EXPOSURE_OK`
- `NA0510_REMOTE_QWORK_ABSENT_OK`
- `NA0510_REMOTE_QSL_BACKUP_ABSENT_OK`
- `NA0510_NO_REMOTE_E2E_OK`
- `NA0510_NO_REMOTE_FILE_WRITE_OK`
- `NA0510_NO_SSH_KEY_GENERATION_OK`
- `NA0510_NO_SSH_CONFIG_MUTATION_OK`
- `NA0510_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0510_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0510_ONE_READY_INVARIANT_OK`

NA-0509 evidence markers:

- `NA0509_QWORK_PROOF_FILES_VERIFIED_OK`
- `NA0509_D397_INHERITANCE_CONSUMED_OK`
- `NA0509_OPERATOR_PROOF_BOUNDARY_RESTATED_OK`
- `NA0509_REMOTE_PROBE_SCOPE_DESIGNED_OK`
- `NA0509_EXACT_FUTURE_COMMANDS_SELECTED_OK`
- `NA0509_READ_ONLY_PROBE_SELECTED_OK`
- `NA0509_REMOTE_WRITE_PROBE_DEFERRED_OK`
- `NA0509_NO_REMOTE_E2E_OK`
- `NA0509_REDACTION_RULES_SELECTED_OK`
- `NA0509_STOP_CONDITIONS_SELECTED_OK`
- `NA0509_NO_REMOTE_ACTION_BY_CODEX_OK`
- `NA0509_NO_SSH_EXECUTION_BY_CODEX_OK`
- `NA0509_NO_SSH_KEY_GENERATION_OK`
- `NA0509_NO_SSH_CONFIG_MUTATION_OK`
- `NA0509_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0509_NO_WORKFLOW_DEPENDENCY_MUTATION_OK`
- `NA0509_NO_CORPUS_VECTOR_INPUT_MUTATION_OK`
- `NA0509_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0509_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0509_ONE_READY_INVARIANT_OK`

## Public claim / website / external review boundary

NA-0509 changes no public docs, website, README, START_HERE, or release
materials. It does not expand any public claim. It records only an internal
governance authorization for a future read-only capability probe.

No external-review-complete claim is made. No public-readiness claim is made.
No production-readiness claim is made. No public-internet-readiness claim is
made.

## Backup-impact statement

Backup impact is none. NA-0509 mutates only tracked qsl-protocol governance,
testplan, traceability, and rolling-journal files. It does not run backup,
restore, or qsl-backup. It does not mutate `/backup/qsl`, backup status, backup
plan, qsl-backup helper code, timers, fstab, manifests, or local ops scripts.

The future NA-0510 read-only probe must stop if `/backup/qsl` is readable by
`qslcodex`.

## Rejected alternatives

- Read/write marker probe now: rejected because read-only capability evidence is
  the safer first remote step and `test -w` is enough to avoid a remote write.
- Remote E2E authorization now: deferred because host/account capability must be
  current before interpreting protocol behavior.
- Remote E2E implementation now: rejected as premature and too broad.
- Setup remediation now: rejected because NA-0508 accepted the setup proof and
  no current weakness requires remediation.
- CI/tooling lane now: rejected because no process blocker prevents selecting
  the next probe scope.

## Next recommendation

After NA-0509 evidence merges and closeout restores NA-0510, execute NA-0510 as
a read-only remote capability probe only. If NA-0510 passes, the next decision
should choose between a narrow remote write-marker proof and a remote E2E
authorization lane. If NA-0510 fails, select remediation or split authorization
based on the exact stop marker.
