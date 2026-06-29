Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0568 Remote Relay Inspiron Codex-Executed Deployment Recovery Harness

## Executive Summary

NA-0568 executed the D-1124/D-1125-authorized bounded Codex remote recovery
lane for the operator-owned `inspiron` QSL/QSC test host.

Result classification:
`REMOTE_RECOVERY_QSC_RELAY_COMMAND_AUTH_REQUIRED`.

Codex verified qwork proof, current main health, queue/decision state, and
inherited governance before running the exact SSH allowlist. SSH readiness
succeeded. Remote inventory classified the qslcodex test workspace as repairable
within the allowed workspace boundary. Remote repair created non-secret
workspace support directories and rollback/repair manifests under the qslcodex
test workspace. Remote postcheck then classified the workspace and qsc binary as
ready, but the expected listener was still not ready.

The qsc test binary exists and is usable for metadata, but no safe local qsc
relay listener start command was discoverable from qsc help output within the
NA-0568 allowlist. NA-0568 therefore selects the exact successor:

`NA-0569 -- QSL Remote qsc Relay Command Discovery Authorization Plan`.

Raw SSH and remote script outputs remain proof-root-only. Repository evidence
publishes only coarse classifications. No endpoint values, private port values,
route-token/capability values, bearer values, Authorization headers, private
topology, process identity, payloads, response bodies, authorized_keys content,
SSH key material, secret values, or private material are published.

## qwork Proof Verification

Fresh qwork proof files were copied from
`/srv/qbuild/work/NA-0568/.qwork/` into the proof root and parsed from files
before any SSH command or repository mutation.

Required values passed:

- `startup_result=OK`
- `lane=NA-0568`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0568/qsl-protocol`
- `head=b4e0b5a52ca4`
- `origin_main=b4e0b5a52ca4`
- `main=b4e0b5a52ca4`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0568`
- `requested_lane_status=READY`
- `cargo_target_mode=shared`
- `shared_target_ready=yes`

The qwork proof timestamp was at or after `2026-06-29T06:38:16Z`. Codex did
not run `qwork`, `qstart`, or `qresume`.

Pre-fetch live state matched the qwork proof. The worktree, index, and
untracked state were clean. Root disk usage was below the 95 percent stop
threshold and `/backup/qsl` was mounted. Fetch occurred only after those gates
passed. Local main was fast-forwarded to `origin/main`, and `origin/main`
descended from `b4e0b5a52ca4`.

## D-1124 / D-1125 Inheritance

D-1124 existed once and was Accepted. D-1124 authorized NA-0568 as the first
bounded Codex-executed remote recovery lane for the qslcodex test workspace.

D-1125 existed once and was Accepted. D-1125 restored NA-0568 as the sole READY
successor after NA-0567 closeout.

NA-0567 was DONE. NA-0568 was READY. D-1126 and D-1127 were absent before the
NA-0568 implementation patch. Duplicate decision count was zero.

Inherited technical findings consumed:

- remote-handshake and remote-relay remain one shared remote relay issue;
- qsc runtime is not currently the primary suspect;
- demo script logic is not currently the primary suspect;
- GitHub runner proof remains deferred until remote deployment/listener state is
  repaired or conclusively classified;
- the strongest current boundary is remote relay listener/deployment state on
  `inspiron`;
- NA-0565 found no usable candidate listener;
- the old NA-0564 operator action bundle remains review-only and was not run.

## Current Main Required-Check Classification

Current main was verified at `b4e0b5a52ca4`.

Required-check classification:

- `public-safety`: completed success
- `advisories`: completed success
- `suite2-vectors`: completed success
- failed required checks: none
- pending required checks: none
- branch-protection required contexts: green or conclusively satisfied

Recovered classifier note: the first current-main classifier treated aggregate
or PR-only contexts as missing literal check-run names. This was corrected by
classifying CodeQL through successful aggregate analysis/workflow evidence and
goal-lint as a pull-request-only required check for current-main health. Final
classification passed.

## Remote Script Design and Static Review

Three proof-root-only scripts were generated:

- `remote_recovery_inventory.py`
- `remote_recovery_repair.py`
- `remote_recovery_postcheck.py`

Static review passed:

- Python syntax check passed for all scripts.
- Scripts print one JSON object only.
- No `shell=True` is used.
- No sudo, systemctl, service, Tailscale, firewall, scp, sftp, rsync, qsl-backup,
  journalctl, ps, ss, netstat, or lsof command is invoked.
- No authorized_keys path or key material is inspected.
- No secret file path is inspected.
- Inventory and postcheck do not write remote files.
- Repair writes are guarded under the qslcodex test workspace.
- The repair rollback manifest is the first remote write.
- qsc metadata commands are limited to the allowed version/help forms.
- qsc local relay start is limited to the NA-0568 allowlist and only if help
  output supports a safe pattern.
- The internal expected loopback target is used only for local comparison and
  is not printed.

Recovered static-review note: the first AST review used a fixed line number for
stdout writes. The review was corrected to verify that stdout writes are
enclosed by the JSON `emit()` helper. Final review passed.

## SSH Readiness

Codex ran the exact SSH readiness command once:

`ssh -o BatchMode=yes -o ConnectTimeout=10 inspiron 'printf INSPIRON_REMOTE_RECOVERY_READY\n'`

Classification: `SSH_REMOTE_RECOVERY_READY`.

The raw stdout/stderr captures remain proof-root-only. Private-material scan
passed.

## Remote Inventory

Codex ran the exact remote inventory command once through SSH stdin.

Classification: `REMOTE_INVENTORY_WORKSPACE_REPAIRABLE`.

Safe coarse fields:

- workspace exists: present
- workspace writable: writable
- qsc binary exists: present
- qsc binary executable: executable
- qsc metadata: available
- qsc help: available
- qsc relay start command discovery: `not_discovered`
- expected listener state: absent
- current listener candidate: no usable expected listener
- workspace repair needed: yes
- repairable within workspace: yes

No endpoint value, private port value, process identity, token value, response
body, private topology, raw private material, authorized key material, or secret
value was printed.

## Remote Repair

Codex ran the exact remote repair command once through SSH stdin because
inventory classified the workspace as repairable within scope.

Classification: `REMOTE_REPAIR_WORKSPACE_DIRS_REPAIRED`.

Safe coarse fields:

- rollback manifest created: yes
- workspace directories: repaired
- qsc binary usable: usable
- qsc relay start attempted: no
- qsc relay start class: command unavailable
- listener left running: no
- rollback available: yes
- remote write scope: qslcodex test workspace only
- mutation scope respected: yes

Repair created only non-secret workspace support files/directories under the
qslcodex test workspace. No sudo, systemctl, service, Tailscale, firewall,
account, shell, authorized_keys, root-owned path, qsl-server, qsl-attachments,
workflow, public-site, or Cloudflare mutation occurred.

## Remote Postcheck

Codex ran the exact remote postcheck command once through SSH stdin after
repair.

Classification: `REMOTE_POSTCHECK_WORKSPACE_READY_LISTENER_NOT_READY`.

Safe coarse fields:

- workspace ready: ready
- qsc binary ready: ready
- expected listener ready: not ready
- v1-path HEAD checks: not checked because listener was not ready
- relay testing ready: not ready
- rollback manifest present: yes
- repair manifest present: yes

Postcheck confirms the qslcodex test workspace is repaired enough for future
authorization work, but the relay listener still cannot be started from the
currently discoverable qsc help surface.

## Rollback and Cleanup Proof

The repair script wrote a rollback manifest before any remote directory
creation. A repair manifest was also written under the qslcodex recovery
workspace.

Rollback status:

- rollback manifest present: yes
- repair manifest present: yes
- remote write scope: qslcodex test workspace only
- cleanup class: remove only NA-0568-created files/directories if the operator
  requests cleanup

No cleanup was executed because the repaired directories and manifests are
non-secret support state for future proof.

## Private-Material Review

Private-material scans passed for:

- SSH readiness raw output;
- remote inventory raw output and parsed JSON;
- remote repair raw output and parsed JSON;
- remote postcheck raw output and parsed JSON;
- proof summaries used for repository docs.

Recovered scan note: the first aggregate scan flagged public GitHub metadata
URLs in proof-root-only API payloads. The scan was rerun with only GitHub-owned
public metadata URLs allowed while private endpoint, private IP, private port,
token, bearer, Authorization, key, and secret patterns remained blocked. Final
aggregate scan passed.

No endpoint values, private hosts/IPs, private topology, private port values,
route-token/capability values, bearer values, Authorization headers, payloads,
response bodies, process identities, raw authorized_keys content, public SSH key
material, private key material, secret environment values, Cloudflare tokens,
API keys, long opaque token strings, raw logs, raw artifacts, or private
material are published.

## Result Classification

Selected result classification:
`REMOTE_RECOVERY_QSC_RELAY_COMMAND_AUTH_REQUIRED`.

Rationale: qsc exists, is executable, and provides metadata/help output. Codex
repaired missing qslcodex workspace directories and left rollback/repair
manifests. Postcheck shows the workspace and qsc binary are ready, but the
expected listener is not ready. The qsc help surface did not reveal any safe
allowlisted local relay listener start command. NA-0568 must not guess.

## Selected Successor

Selected successor:
`NA-0569 -- QSL Remote qsc Relay Command Discovery Authorization Plan`.

Successor objective:
Authorize exact next proof needed to identify a safe qsc local relay listener
command without secrets or private material. This successor is authorization
only.

## Required-Check Boundary

NA-0568 classified current-main checks before mutation and will rely on PR and
post-merge checks for merge eligibility. No branch-protection configuration was
changed. No workflow dispatch or rerun was executed.

## Source / Script Mutation Boundary

No repository source path or repository script path was mutated. Generated
remote recovery scripts remained proof-root-only and were not committed.

## Workflow Mutation Boundary

No workflow file was mutated. No workflow dispatch, rerun, cancel, deletion, or
workflow run mutation occurred.

## Runtime / qsc Boundary

qsc was used only for allowed metadata/help classification. No qsc send,
receive, E2EE, relay payload, external endpoint connection, source mutation,
test mutation, fuzz mutation, Cargo manifest mutation, or lockfile mutation
occurred.

## qsl-server / qsl-attachments Boundary

No qsl-server or qsl-attachments command, clone, build, run, or mutation
occurred.

## Remote-Action Boundary

Remote action was limited to:

- exact SSH readiness;
- exact inventory through SSH stdin;
- exact repair through SSH stdin after repairable inventory classification;
- exact postcheck through SSH stdin.

Remote writes were limited to non-secret files/directories under the qslcodex
test workspace. No sudo, systemctl, service command, Tailscale mutation,
firewall mutation, account/shell mutation, authorized_keys mutation, root-owned
path mutation, qsl-server/qsl-attachments mutation, qsc send/receive, workflow
dispatch, rerun, public-site mutation, or Cloudflare mutation occurred.

## Public-Site / Cloudflare Boundary

No README public-progress content, docs/public content, website path, public
path, public-site content, deployment setting, or Cloudflare configuration was
mutated.

## Raw Output Boundary

Raw SSH and remote script stdout/stderr captures remain proof-root-only.
Repository docs include only coarse classifications and private-material scan
outcomes. Raw logs and raw artifacts were not committed.

## Claim Boundary

No public-readiness claim was made. No production-readiness claim was made. No
public-internet-readiness claim was made. No vulnerability-free claim was made.
No bug-free claim was made. No perfect-build claim was made.

## Validation

Validation is recorded in proof-root-only artifacts and summarized by
`tests/NA-0568_remote_relay_inspiron_codex_executed_deployment_recovery_testplan.md`.

Focused qsc runtime tests are skipped for NA-0568 because no qsc source,
runtime, dependency, workflow, executable test, fuzz target, or vector was
mutated and qsc send/receive was not authorized.

## Recommendation

Proceed to the selected NA-0569 authorization lane after NA-0568 implementation
merge and closeout, if post-merge gates allow closeout. The next lane should
authorize exact non-secret proof for discovering a safe qsc local relay listener
command and must not perform remote mutation or qsc send/receive.
