Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-23

# NA-0528 QSL Remote qsc E2EE Reverse-Forwarding Diagnostic / Retry Scope Authorization Plan

## Executive summary

NA-0528 is an authorization-only governance/security lane. Codex did not run SSH, did not run remote commands, did not run qsc send/receive, did not run remote E2EE, did not use qsl-server, and did not use qsl-attachments.

NA-0528 consumed D427 / D426 / D425 / D414 / D413 inheritance and selected the next narrow diagnostic implementation lane:

```text
REMOTE_FORWARDING_DIAGNOSTIC_IMPLEMENTATION_READY
```

Selected successor:

```text
NA-0529 -- QSL Remote qsc E2EE Reverse-Forwarding Diagnostic Implementation Harness
```

The selected successor combines known-good NA-0520 marker-forwarding reproduction, D427/NA-0520 command/config comparison, sanitized SSH verbose-log diagnostic if reproduction fails, and bounded local/remote loopback state checks. It defers operator key/sshd proof-review unless the diagnostic evidence points to server-side policy.

## Live NA-0528 scope

Startup proof and live repo checks showed:

- `READY_COUNT 1`.
- Sole READY item: `NA-0528 -- QSL Remote qsc E2EE Reverse-Forwarding Diagnostic / Retry Scope Authorization Plan`.
- `NA-0527`, `NA-0526`, and `NA-0525` were DONE.
- D-1044 and D-1045 each existed once.
- D-1046 and D-1047 were absent before this patch.
- Duplicate decision count was zero.

Allowed checked-in mutation for this authorization evidence is limited to this evidence file, the NA-0528 testplan, `DECISIONS.md`, `TRACEABILITY.md`, and `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

The qwork proof files were present and copied into proof root:

- `/srv/qbuild/work/NA-0528/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0528/.qwork/startup.qsl-protocol.json`

Required qwork proof fields passed:

- `startup_result=OK`
- `lane=NA-0528`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0528/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0528`
- `requested_lane_status=READY`

The qwork proof HEAD and origin/main matched live pre-fetch refs at `11f06ba6caec`. Fetch was performed only after this match. Disk usage before fetch was below the stop threshold: `/` at 89% and `/backup/qsl` at 27%.

## D427 / D426 / D425 / D414 / D413 inheritance

NA-0528 consumed the required inheritance:

- D427 confirmed NA-0527 completed and closed out.
- D427 confirmed NA-0527 implementation PR #1327 merged at `7743e3926a52`.
- D427 confirmed NA-0527 closeout PR #1328 merged at `11f06ba6caec`.
- D427 confirmed NA-0528 was restored READY.
- NA-0527 classification was `REMOTE_E2EE_FORWARDING_RECHECK_FAILURE`.
- NA-0527 retained qsc recheck passed for path `$HOME/qsl-remote-test/bin/qsc`, owner `qslcodex`, mode `700`, size `102103920`, SHA-256 `6bf9e59fdae397c2e0f88538d700cccbee80d229c6a979cc79555e39fea2b4f7`, and remote `--help` success.
- NA-0527 remote account and boundary checks passed.
- NA-0527 local qsc relay started on loopback port `39176`.
- NA-0527 source inspection confirmed the relay bind to `127.0.0.1`.
- NA-0527 failed at reverse-forwarding before E2EE.
- NA-0527 did not run baseline E2EE.
- NA-0527 did not run wrong-peer or stale-trust negatives.
- NA-0527 did not run qsc send/receive.
- NA-0527 cleanup passed.
- D426 confirmed NA-0526 DONE and NA-0527 restored READY after NA-0526 closeout.
- D425 confirmed NA-0526 restaged the retained remote qsc binary after the quinn-proto remediation and retained the fresh binary for NA-0527.
- D414 / NA-0520 previously proved dedicated-key reverse forwarding succeeded with marker traversal and ACK on `127.0.0.1:39176`.
- D413 / NA-0519 accepted the dedicated forwarding key proof, fingerprint `SHA256:7guT/Hd72vuu+u1Jrwpuy0hSFETVrxYaXHwupuFfaPM`, public key comment `qsl-inspiron-qslcodex-forward-20260622`, loopback-only `permitlisten` and `permitopen` for `127.0.0.1:39176`, no PTY, no agent forwarding, and no X11 forwarding.
- D413 recorded the `command="/bin/false"` compatibility caveat, and NA-0520 later proved the forwarding path worked.
- qsl-server and qsl-attachments remained out of scope throughout the inherited lanes.

## Forwarding failure review

D427 records a real forwarding blocker, not a qsc protocol result.

Required findings:

- D427 first failure had proof-root SSH config with `ClearAllForwardings yes`.
- D427 corrected that once as a recoverable proof-root command/config mistake.
- D427 subsequent attempts failed with `remote port forwarding failed for listen port 39176`.
- D427 read-only bind probe showed remote `127.0.0.1:39176` available and no listener in the retrieved socket table.
- D427 bounded recovery was exhausted.
- NA-0520 known-good proof succeeded with the dedicated key, the same loopback port, marker traversal, and ACK.

The evidence does not prove a single root cause. Future NA-0529 must distinguish:

- proof-root config error;
- bind-host syntax mismatch;
- dedicated-key restriction or policy issue;
- forced-command compatibility regression;
- stale session or port state;
- remote sshd policy drift;
- local listener or relay setup issue;
- insufficient diagnostic logging.

## NA-0520 successful forwarding comparison

NA-0520 used:

- local listener bound only to `127.0.0.1:39176`;
- reverse forward command with `-N -T`, `ExitOnForwardFailure=yes`, and `-R 127.0.0.1:39176:127.0.0.1:39176`;
- proof-root SSH config with `User qslcodex`, `IdentitiesOnly yes`, `PreferredAuthentications publickey`, `PasswordAuthentication no`, `BatchMode yes`, `StrictHostKeyChecking yes`, `ForwardAgent no`, `ForwardX11 no`, and `RequestTTY no`;
- one remote loopback trigger that wrote no files, required no PTY, required no sudo, ran no qsc, ran no qwork, and ran no qsl-backup;
- synthetic marker traversal and ACK `NA0520_TUNNEL_ACK_OK`.

D427 used the same intended loopback reverse-forward target but first suppressed the explicit reverse forward via `ClearAllForwardings yes`; after correction, the remote port-forward request failed even though a read-only bind probe showed the remote port was available. Future NA-0529 must compare the D427 command/config evidence against the NA-0520 known-good command/config before attempting reproduction.

## Option review

Option 1 - Known-good NA-0520 marker-forwarding reproduction: selected. It directly retests the exact path that previously worked without qsc E2EE or qsc send/receive.

Option 2 - D427 command/config diff diagnostic: selected. It should run before reproduction so proof-root config drift is found early.

Option 3 - SSH verbose-log diagnostic: selected if reproduction fails. Logs must stay under the proof root and be summarized only after redaction.

Option 4 - Remote bind / stale session diagnostic: selected in bounded read-only form. It should check local listener/process state and remote loopback port state without remote file writes.

Option 5 - Operator proof review for key/sshd policy: deferred until diagnostics point to server-side policy. Codex must not read authorized_keys or sshd_config.

Option 6 - Immediate E2EE retry: rejected until forwarding is proven again.

Option 7 - qsl-server/qsl-attachments integration: deferred and out of the current direct qsc sprint.

Option 8 - Broad SSH/key/sshd remediation: rejected/deferred unless diagnostics prove it is needed and a later operator-action authorization lane permits it.

Option 9 - Abandon remote sprint / cleanup retained qsc: rejected unless the remote host becomes unavailable or forwarding diagnosis fails irrecoverably.

## Selected future diagnostic implementation design

Future NA-0529 should be:

```text
NA-0529 -- QSL Remote qsc E2EE Reverse-Forwarding Diagnostic Implementation Harness
Status: READY
Goals: G1, G2, G3, G4, G5
```

Objective:

Diagnose the NA-0527 reverse-forwarding failure without running qsc E2EE or qsc send/receive, by comparing D427 and NA-0520 command/config evidence, reproducing the known-good NA-0520 synthetic marker-forwarding probe with the dedicated forwarding key, capturing sanitized SSH debug logs if the reproduction fails, checking local/remote loopback port state without mutating remote files, cleaning all local processes, and selecting either a corrected E2EE retry successor or an operator-action remediation/proof-review successor.

Allowed future checked-in paths:

- `docs/governance/evidence/NA-0529_qsl_remote_qsc_e2ee_reverse_forwarding_diagnostic_implementation_harness.md`
- `tests/NA-0529_qsl_remote_qsc_e2ee_reverse_forwarding_diagnostic_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Allowed future proof actions:

- proof-root-local listener and logs;
- one known-good reverse-forwarding reproduction attempt;
- at most one corrected proof-root config retry if the first attempt reveals a local config-shape error;
- sanitized SSH verbose log capture under proof root;
- read-only remote boundary and port-state checks;
- one synthetic marker traversal if forwarding starts successfully;
- cleanup proof.

## Future command family

Future NA-0529 may run only under its own directive:

- qwork proof reading only; Codex must not run qwork.
- safe `ssh -G` parsing.
- local listener on `127.0.0.1:39176`.
- dedicated-key reverse forward with `ssh -N -T -o ExitOnForwardFailure=yes -R 127.0.0.1:39176:127.0.0.1:39176 ...`.
- proof-root SSH config only.
- sanitized `ssh -vvv` or `ssh -E "$PROOF_DIR/..."` logs if needed.
- one bounded remote trigger through the operational path only after forwarding starts, for synthetic marker traversal.
- read-only remote port checks.
- local process cleanup.

Future NA-0529 must not run qsc E2EE, qsc send/receive, qsc identity/contact/handshake/relay protocol commands, mutate remote files, read authorized_keys, mutate keys/config, or use qsl-server/qsl-attachments.

## Future proof / redaction rules

Future proof must include no private keys, passphrases, tokens, passwords, production endpoints, backup material, unrelated known_hosts/authorized_keys material, or full SSH config.

Checked-in evidence should summarize:

- safe command families;
- safe proof-root SSH fields;
- marker traversal result or fail reason;
- redacted debug-log conclusions;
- cleanup status;
- successor selection.

Raw debug logs must stay under proof root and must be scanned/redacted before any summary is checked in.

## Future stop conditions

Future NA-0529 must stop for:

- qwork stale proof;
- unsafe SSH config;
- forwarding binds non-loopback;
- dedicated key not used;
- private key/passphrase/token in output;
- remote file write need;
- qsc E2EE need;
- qsc send/receive command need;
- qsl-server/qsl-attachments requirement;
- authorized_keys read or mutation need;
- sshd_config read or mutation need;
- remote package install need;
- qwork/qsl-backup appearance;
- cleanup failure;
- public/production claim pressure.

## Hostile Cryptographer Review

Best-Known-Method Review: the narrowest next step is to re-establish the transport precondition using the previously proven NA-0520 marker traversal pattern before retrying qsc E2EE.

Hostile Cryptographer Review: forwarding diagnostics prove only transport setup behavior, not qsc protocol correctness. A successful marker traversal would not prove E2EE, identity correctness, trust correctness, replay resistance, downgrade resistance, secret-material completeness, or side-channel safety. qsc E2EE must remain deferred until forwarding is proven and a separate retry lane authorizes it.

Side-Channel Caveat: NA-0528 and future NA-0529 do not make any side-channel-free claim. Debug logging must avoid exposing secrets, route material, private paths beyond approved proof-root summaries, or production endpoint data.

Formal-Model Mapping Residual: this lane maps to governance and operational precondition evidence only. It does not add or prove formal protocol model coverage.

External-Review Readiness: the evidence improves reviewability of the remote diagnostic path, but no external-review-complete claim is made.

Assurance Gap Review Trigger: if NA-0529 cannot reproduce the NA-0520 marker-forwarding path, the next lane must select either operator proof-review/remediation or a narrower server-side policy diagnostic rather than retrying E2EE.

## Red-Team Review

Red-team conclusion:

- Overbroad SSH forwarding, non-loopback bind, private-key exposure, authorized_keys leakage, sshd_config mutation, or remote file writes are rejection conditions.
- If forced-command or key-level restrictions are implicated, future work should use operator-proof review, not Codex mutation.
- If forwarding succeeds with corrected proof-root config, the next successor can be a narrow E2EE retry.
- If forwarding fails with known-good config, the next successor should be operator-action remediation/proof review or a narrower server-side policy diagnostic.

## Production SRE Review

Production SRE conclusion:

- Diagnosing forwarding before E2EE is the right operational step.
- Logs must remain proof-root-local and redacted.
- Cleanup proof must cover local listener and SSH processes.
- qwork, qsl-backup, production data, qsl-server, and qsl-attachments remain isolated.
- This does not imply public-readiness, production-readiness, or public-internet-readiness.

## Release-Claim Boundary Review

Release-claim boundary:

- no public-readiness claim is made.
- no production-readiness claim is made.
- no public-internet-readiness claim is made.
- no external-review-complete claim is made.
- no crypto-complete claim is made.
- no identity-complete claim is made.
- no trust-complete claim is made.
- no replay-proof claim is made.
- no downgrade-proof claim is made.
- no secret-material-complete claim is made.
- no side-channel-free claim is made.
- no vulnerability-free claim is made.
- no bug-free claim is made.
- no perfect-crypto claim is made.

## Prioritization matrix

| Candidate | Risk reduced | Directness | Speed while host is available | Security risk | Operator burden | Implementation feasibility | Scope risk | Remote mutation risk | Secret/key risk | Public-claim risk | Disposition | Next-lane yes/no |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Known-good NA-0520 marker-forwarding reproduction | Rechecks proven forwarding capability | High | High | Low if loopback-only | Low | High | Low | Low | Medium, due dedicated key use | Low | Selected | Yes |
| D427 command/config diff diagnostic | Reduces local config-shape ambiguity | High | High | Low | Low | High | Low | None | Low | Low | Selected | Yes |
| SSH verbose-log diagnostic | Reduces opaque SSH failure ambiguity | Medium | Medium | Medium, needs redaction | Low | High | Medium | None | Medium | Low | Selected if reproduction fails | Yes |
| Remote bind / stale session diagnostic | Reduces stale port/process ambiguity | Medium | Medium | Low if read-only | Low | Medium | Medium | None | Low | Low | Selected in bounded read-only form | Yes |
| Operator key/sshd proof-review lane | Reduces server-side policy ambiguity | Medium | Lower | Low for Codex, higher operator handling | Medium | Medium | Medium | None by Codex | Medium | Low | Deferred until evidence points there | No |
| Immediate E2EE retry | None until forwarding works | Low | Medium | High for truthfulness | Low | Medium | High | Possible if E2EE roots used | Medium | High | Rejected | No |
| qsl-server/qsl-attachments integration | Does not answer direct qsc forwarding gap | Low | Low | Medium | Medium | Medium | High | Medium | Medium | Medium | Deferred/out of scope | No |
| Broad SSH/key/sshd remediation | Could fix root cause if known | Low before diagnosis | Low | High | High | Low/medium | High | High | High | Medium | Rejected/deferred | No |
| Abandon remote sprint / cleanup retained qsc | Reduces retained-state exposure if host unavailable | Low now | Medium | Low/medium | Medium | Medium | Medium | Medium | Low | Low | Rejected unless diagnostics fail irrecoverably | No |

## Authorization decision

Selected classification:

```text
REMOTE_FORWARDING_DIAGNOSTIC_IMPLEMENTATION_READY
```

Basis:

- D427 consumed.
- D414 consumed.
- D413 consumed.
- Failure review completed.
- Option review completed.
- Future command family selected.
- Proof/redaction/stop rules selected.
- Hostile cryptographer, red-team, SRE, release-claim, side-channel, formal residual, external-review, and assurance-gap reviews completed.
- Prioritization matrix completed.
- Exact NA-0529 successor selected.
- No remote action in NA-0528.
- No SSH execution in NA-0528.
- No qsc send/receive in NA-0528.
- No qsl-server/qsl-attachments selected.
- No qsc source/test/fuzz/Cargo mutation.
- No public claim expansion.
- Exactly one READY successor remains mandatory.

## Selected NA-0529 successor

```text
NA-0529 -- QSL Remote qsc E2EE Reverse-Forwarding Diagnostic Implementation Harness
Status: READY
Goals: G1, G2, G3, G4, G5
```

## Future scope bundle

Future NA-0529 allowed scope:

- `docs/governance/evidence/NA-0529_qsl_remote_qsc_e2ee_reverse_forwarding_diagnostic_implementation_harness.md`
- `tests/NA-0529_qsl_remote_qsc_e2ee_reverse_forwarding_diagnostic_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- proof-root-local listener and logs;
- one known-good reverse-forwarding reproduction attempt;
- at most one corrected proof-root config retry if the first attempt reveals a local config-shape error;
- sanitized SSH verbose log capture under proof root;
- read-only remote boundary and port-state checks;
- one synthetic marker traversal if forwarding starts successfully;
- cleanup proof.

Future NA-0529 forbidden scope:

- qsc E2EE;
- qsc send/receive;
- qsc identity/contact/handshake/relay protocol commands;
- qsl-server/qsl-attachments;
- package installation;
- sudo/admin action except negative `sudo -n true` probe if needed;
- key generation/installation;
- authorized_keys mutation;
- authorized_keys reading;
- sshd_config mutation;
- sshd_config reading unless operator supplies redacted proof in a later lane;
- SSH config mutation outside proof root;
- known_hosts mutation;
- remote file write;
- remote source checkout/build;
- qwork/qstart/qresume;
- qsl-backup;
- qsc source/test/fuzz/Cargo mutation;
- workflow/dependency mutation;
- corpus/vector/input mutation;
- formal/refimpl/service/public/backup mutation;
- production/user data;
- public/security/completion claim expansion.

## Future validation / marker plan

Future NA-0529 markers:

- `NA0529_D427_FORWARDING_FAILURE_CONSUMED_OK`
- `NA0529_NA0520_SUCCESSFUL_FORWARDING_CONSUMED_OK`
- `NA0529_COMMAND_CONFIG_DIFF_REVIEWED_OK`
- `NA0529_DEDICATED_KEY_USED_OK`
- `NA0529_LOOPBACK_ONLY_BIND_CHECKED_OK`
- `NA0529_EXIT_ON_FORWARD_FAILURE_USED_OK`
- `NA0529_SYNTHETIC_MARKER_TRAVERSAL_RESULT_RECORDED_OK`
- `NA0529_SSH_DEBUG_LOG_REDACTED_OK`
- `NA0529_NO_REMOTE_FILE_WRITE_OK`
- `NA0529_NO_QSC_E2EE_OK`
- `NA0529_NO_QSC_SEND_RECEIVE_OK`
- `NA0529_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0529_CLEANUP_COMPLETED_OK`
- `NA0529_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0529_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0529_ONE_READY_INVARIANT_OK`

## No remote action in NA-0528

NA-0528 performed no remote action. Codex did not run SSH, scp, sftp, rsync, remote commands, remote E2EE, qsc send/receive, qsl-server, or qsl-attachments.

## No qsc send/receive in NA-0528

NA-0528 did not run qsc send or qsc receive. It did not run qsc identity/contact/handshake/relay protocol commands. It did not mutate qsc source, tests, fuzz corpus, Cargo manifests, or lockfiles.

## No qsl-server / no qsl-attachments boundary

NA-0528 did not use, start, mutate, require, or route through qsl-server or qsl-attachments. They remain deferred and out of the current direct qsc sprint.

## Public claim / website / external review boundary

NA-0528 did not mutate public website, service, refimpl, formal model, or public docs paths. This lane does not create public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, identity-complete, trust-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claims.

## Backup-impact statement

Backup impact: none. NA-0528 did not run qsl-backup, did not mutate backup status/plan files, did not mutate `/backup/qsl`, and did not inspect backup private material. The installed qsl-backup helper digest and source-list count were checked read-only.

## Rejected alternatives

- Immediate qsc E2EE retry: rejected because D427 stopped at forwarding and a retry would produce untruthful evidence until forwarding is proven again.
- qsl-server/qsl-attachments integration: rejected/deferred because it does not answer the direct qsc reverse-forwarding diagnostic question.
- Broad SSH/key/sshd remediation: rejected/deferred because D427 does not prove the root cause and Codex must not mutate authorized_keys, sshd_config, or key policy.
- Abandon remote sprint / cleanup retained qsc: rejected now because the known-good NA-0520 path previously worked and a narrow diagnostic is still warranted.

## Next recommendation

Merge NA-0528 authorization after required checks pass. If post-merge public-safety is attached and green inside the short attach/early-failure window, close out NA-0528 and restore NA-0529 as the sole READY successor. NA-0529 must diagnose forwarding only and must not run qsc E2EE or qsc send/receive.
