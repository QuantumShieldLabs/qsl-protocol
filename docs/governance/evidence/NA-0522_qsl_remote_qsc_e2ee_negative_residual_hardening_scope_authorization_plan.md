Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-22

# NA-0522 QSL Remote qsc E2EE Negative / Residual Hardening Scope Authorization Plan

## Executive summary

NA-0522 is authorization-only. It consumes NA-0521 / D416, inventories the remaining direct remote qsc E2EE negative and residual hardening surfaces, and selects the exact next implementation lane.

Authorization decision: `REMOTE_E2EE_REPLAY_CORRUPT_NEGATIVE_IMPLEMENTATION_READY`.

Selected successor: `NA-0523 -- QSL Remote qsc E2EE Replay / Corrupt Delivery Negative Boundary Implementation Harness`.

No remote action occurred in NA-0522. No SSH execution occurred in NA-0522. No qsc send/receive occurred in NA-0522. No remote E2EE occurred in NA-0522. No qsl-server or qsl-attachments work is selected. No public-readiness claim is made. No production-readiness claim is made. No public-internet-readiness claim is made. No external-review-complete claim is made. No crypto-complete claim is made. No replay-proof claim is made. No downgrade-proof claim is made. No secret-material-complete claim is made. No side-channel-free claim is made. No vulnerability-free, bug-free, or perfect-crypto claim is made.

## Live NA-0522 scope

NA-0522 was the sole READY item at startup. This lane is limited to governance evidence, a testplan, D-1033, TRACEABILITY, and the rolling operations journal.

Allowed mutation paths:
- `docs/governance/evidence/NA-0522_qsl_remote_qsc_e2ee_negative_residual_hardening_scope_authorization_plan.md`
- `tests/NA-0522_qsl_remote_qsc_e2ee_negative_residual_hardening_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No qsc source/test/fuzz/Cargo path, workflow/script/helper path, dependency/lockfile, corpus/vector/input path, formal/refimpl/service/public/backup path, qsl-server path, qsl-attachments path, qshield path, qsl-backup path, backup path, public docs path, or website path is mutated.

## qwork proof-file verification

Codex did not run qwork, qstart, or qresume. It read the existing qwork proof files under `/srv/qbuild/work/NA-0522/.qwork/` and copied them into the directive proof root.

The `.kv` and `.json` proofs mirrored the required state:
- `startup_result=OK`
- `lane=NA-0522`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0522/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0522`
- `requested_lane_status=READY`

Proof HEAD and proof origin/main matched live pre-fetch refs at short SHA `057c4cb27c2e`. Fetch occurred only after that match and after disk usage was below the 95% stop threshold.

Startup queue and decision state:
- READY_COUNT 1.
- READY NA-0522.
- NA-0521 DONE.
- NA-0520 DONE.
- NA-0519 DONE.
- D-1031 exists once.
- D-1032 exists once.
- D-1033 was absent before this patch.
- Duplicate decision ID count was zero.

Startup main health:
- `public-safety` completed success.
- `qsc-adversarial-smoke` completed success.
- `qsc-linux-full-suite` completed skipped under policy.
- `macos-qsc-full-serial` completed skipped under policy.
- No required red check was present in the retrieved main check-run set.

qsl-backup boundary:
- `/usr/local/sbin/qsl-backup` was inspected read-only by hash/source-list proof only.
- The expected helper digest matched, recorded in proof by full hash and in this evidence by prefix `e9ecff3d22ed`.
- The Codex ops source inclusion count was exactly 1.
- qsl-backup, backup, or restore was not executed.

## NA-0521 / D416 inheritance

NA-0521 completed and NA-0522 was restored READY by the D416 closeout. The inherited classification is `REMOTE_BUILD_TO_INSPIRON_E2EE_PASS_WITH_NEGATIVE_BOUNDARY`.

Consumed facts:
- NA-0521 completed.
- NA-0522 restored READY.
- D415 residue recovery succeeded.
- The two D415 `/tmp` residue paths were absent.
- Retained remote qsc was rechecked.
- Dedicated-key reverse forwarding path was rechecked.
- Local qsc provenance was recorded from a clean checkout.
- Build-to-Inspiron synthetic E2EE send/receive succeeded.
- Inspiron-to-Build synthetic E2EE reply succeeded.
- Wrong-mailbox negative/no-mutation boundary passed.
- Remote E2EE root cleanup passed.
- Local sensitive runtime cleanup passed.
- Retained remote qsc binary was preserved.
- No qsl-server was used.
- No qsl-attachments was used.
- No package install occurred.
- No remote source checkout/build occurred.
- No qwork/qsl-backup execution occurred.
- No qsc source/test/fuzz/Cargo mutation occurred.
- No workflow/dependency/corpus/formal/refimpl/service/public/backup mutation occurred.
- No public-readiness, production-readiness, crypto-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim was made.

## Residual inventory

| Residual | Classification | Current evidence | NA-0522 disposition |
|---|---|---|---|
| Replayed delivery/message boundary | Direct remote negative candidate | Local receive code emits replay rejection markers; relay can duplicate HTTP inbox pushes; NA-0521 did not test replay | Select for NA-0523 |
| Corrupt delivery artifact boundary | Direct remote negative candidate | Receive path fail-closes on unpack/auth failures; local tests cover corrupt/tamper classes for related transfer surfaces | Select for NA-0523, with deferral if artifact staging requires internals |
| Wrong-peer boundary | Supporting local evidence exists | `receive_e2e` covers wrong peer/no output mutation locally; NA-0521 covered wrong mailbox remotely | Defer behind replay/corrupt unless replay/corrupt are impossible |
| Stale public/trust material boundary | Supporting local evidence exists | Identity/trust tests exist; remote stale trust was not exercised | Defer to later identity/trust lane |
| Repeated-run safety and cleanup after multiple runs | Process/tooling residual | NA-0521 cleanup passed once; repeated-run cleanup not proven | Include as future acceptance support, not primary next lane |
| Retained remote qsc freshness before each run | Process/tooling residual | NA-0521 rechecked hash/path/owner/help before use | Require in NA-0523 |
| Forwarding tunnel cleanup / stale process risk | Process/tooling residual | NA-0521 stopped relay/forwarding and verified exact process absence | Require in NA-0523 cleanup proof |
| Route-token / capability metadata redaction | Direct evidence hygiene residual | NA-0521 scanned checked-in evidence and proof outputs for route-token leakage | Require in NA-0523 proof/redaction rules |
| qsc output no-secret review | Direct evidence hygiene residual | NA-0521 scan passed | Require in NA-0523 |
| Remote root cleanup/retention policy | Process/tooling residual | NA-0521 removed remote E2EE root and preserved retained qsc | Require in NA-0523 |
| Residual scheduled remote CI checks from earlier lanes | Out of current direct qsc sprint | Earlier non-required/scheduled red-check diagnostics remain separate from direct two-host qsc evidence | Defer to scheduled diagnostic lane |
| qsl-server/qsl-attachments deferred boundary | Future service/attachment integration only | NA-0521 used qsc local relay transport, not qsl-server/qsl-attachments | Defer |
| Public/production claim boundary | Process/tooling residual | All prior lanes preserved no-claim wording | Preserve in NA-0523 |

## Option review

### Option 1 - Remote replay + corrupt delivery negative implementation

Disposition: selected.

Risk reduced: active replay/tamper delivery surfaces in the direct qsc client-to-client path.

Evidence gap addressed: NA-0521 proved positive remote E2EE and one wrong-mailbox no-mutation boundary, but did not prove replayed delivery or corrupt artifact rejection on the remote path.

Implementation feasibility: high enough for the next lane. The local qsc relay has deterministic duplicate behavior, the receive path emits replay/auth failure markers, and synthetic artifact staging can be attempted under the remote E2EE root. If corrupt artifact staging requires qsc internals, NA-0523 must defer that subcase with exact rationale and still complete the feasible replay path.

Scope risk: medium, controlled by forbidding qsc source/test/fuzz/Cargo mutation and service integration.

Remote mutation risk: medium, limited to the allowed remote E2EE runtime root and cleanup.

Secret/key risk: medium, controlled by synthetic messages, no private material transfer, redaction scan, and checked-in summary only.

Public-claim risk: low if no-claim language is retained.

Likely future allowed paths: NA-0523 evidence, NA-0523 testplan, DECISIONS, TRACEABILITY, rolling journal, proof-root-local output.

Likely future forbidden paths: qsc source/test/fuzz/Cargo, workflows, dependencies, corpora/vectors/inputs, formal/refimpl/service/public/backup paths, qsl-server, qsl-attachments.

P0/P1/P2 risks:
- P0: replay or corrupt delivery fails open and valid state mutates unexpectedly.
- P1: corrupt artifact setup requires internals or leaks capability metadata into evidence.
- P2: cleanup/retention proof is incomplete.

### Option 2 - Remote replay-only negative implementation

Disposition: fallback if corrupt-delivery staging is ambiguous.

Risk reduced: duplicate/replay delivery handling.

Evidence gap addressed: message replay rejection on remote transport path.

Implementation feasibility: high because duplicate relay behavior is directly visible in source.

Scope risk: low to medium.

Remote mutation risk: low to medium.

Secret/key risk: low to medium.

Public-claim risk: low.

Likely future allowed paths: same governance/proof-root paths as Option 1.

Likely future forbidden paths: same forbidden paths as Option 1.

P0/P1/P2 risks:
- P0: replay accepted or mutates receiver state unexpectedly.
- P1: duplicate semantics consume/remove artifacts in a way that makes replay proof ambiguous.
- P2: only one replay form is covered.

### Option 3 - Remote corrupt-delivery-only negative implementation

Disposition: fallback if replay is ambiguous but corrupt artifact staging is clean.

Risk reduced: tamper/authentication failure handling.

Evidence gap addressed: remote corrupt delivery rejection.

Implementation feasibility: medium; artifact mutation must stay under allowed runtime roots and must not require qsc internals.

Scope risk: medium.

Remote mutation risk: medium.

Secret/key risk: medium.

Public-claim risk: low.

Likely future allowed paths: same governance/proof-root paths as Option 1.

Likely future forbidden paths: same forbidden paths as Option 1.

P0/P1/P2 risks:
- P0: corrupt artifact is accepted or mutates receiver state unexpectedly.
- P1: artifact capture leaks capability metadata.
- P2: corruption setup is not representative.

### Option 4 - Remote wrong-peer/stale-trust negative implementation

Disposition: defer unless replay and corrupt delivery are both too broad.

Risk reduced: identity/trust misuse.

Evidence gap addressed: wrong-peer or stale-trust remote handling.

Implementation feasibility: medium.

Scope risk: medium because trust-state setup can become broader than delivery negative testing.

Remote mutation risk: medium.

Secret/key risk: medium.

Public-claim risk: low.

Likely future allowed paths: future evidence/testplan/decision/traceability/journal paths.

Likely future forbidden paths: qsc source mutation, key generation/installation outside isolated qsc runtime, qsl-server/qsl-attachments.

P0/P1/P2 risks:
- P0: stale trust accepts a peer unexpectedly.
- P1: test setup mutates trust state beyond the intended no-mutation check.
- P2: result overlaps too much with existing local coverage.

### Option 5 - Remote repeated-run cleanup/freshness hardening

Disposition: defer as a supporting acceptance criterion.

Risk reduced: stale process/root/binary drift.

Evidence gap addressed: repeated-run operational cleanup.

Implementation feasibility: high.

Scope risk: low.

Remote mutation risk: low to medium.

Secret/key risk: low to medium.

Public-claim risk: low.

Likely future allowed paths: future evidence/testplan/decision/traceability/journal paths and proof-root-local output.

Likely future forbidden paths: qsc source mutation, service integration, package install, qwork/qsl-backup.

P0/P1/P2 risks:
- P0: cleanup fails and leaves sensitive runtime.
- P1: retained qsc freshness is not rechecked before each run.
- P2: repeated positive runs do not add negative security evidence.

### Option 6 - Remote scheduled red-check diagnostic lane

Disposition: defer.

Risk reduced: scheduled CI health ambiguity.

Evidence gap addressed: earlier scheduled remote-handshake/relay/UI residuals.

Implementation feasibility: medium.

Scope risk: medium because it can drift away from direct qsc E2EE hardening.

Remote mutation risk: low to medium depending on diagnostics.

Secret/key risk: low if kept diagnostic-only.

Public-claim risk: low.

Likely future allowed paths: diagnostic evidence/testplan/decision/traceability/journal paths.

Likely future forbidden paths: direct remote E2EE implementation unless separately authorized.

P0/P1/P2 risks:
- P0: a required check is red and blocks merge.
- P1: diagnostic scope absorbs host-availability time better spent on direct qsc negatives.
- P2: low immediate security evidence yield.

### Option 7 - qsl-server/qsl-attachments integration

Disposition: defer.

Risk reduced: service integration risk.

Evidence gap addressed: qsc-to-service path, not direct qsc path.

Implementation feasibility: not appropriate for the current remote-host window.

Scope risk: high.

Remote mutation risk: high.

Secret/key risk: medium to high.

Public-claim risk: medium.

Likely future allowed paths: future service integration lanes only.

Likely future forbidden paths: all service paths in NA-0523.

P0/P1/P2 risks:
- P0: service integration is mistaken for direct qsc negative proof.
- P1: service setup expands remote and secret surfaces.
- P2: delays direct negative hardening.

### Option 8 - Public/production readiness package

Disposition: rejected.

Risk reduced: none for the current evidence gap.

Evidence gap addressed: none; current evidence is not sufficient for public/production status.

Implementation feasibility: inappropriate.

Scope risk: high.

Remote mutation risk: variable.

Secret/key risk: high if rushed.

Public-claim risk: high.

Likely future allowed paths: none in this sprint.

Likely future forbidden paths: public docs, website, public-readiness claims, production-readiness claims.

P0/P1/P2 risks:
- P0: unsupported public or production claim.
- P1: external review readiness is overstated.
- P2: governance evidence is misused as release evidence.

### Option 9 - Cleanup/remediation lane

Disposition: reject for now; select only if D416 cleanup or retained qsc state is unsafe.

Risk reduced: cleanup/remediation risk.

Evidence gap addressed: none currently, because D416 cleanup passed.

Implementation feasibility: high if needed.

Scope risk: low to medium.

Remote mutation risk: medium if cleanup requires remote action.

Secret/key risk: low to medium.

Public-claim risk: low.

Likely future allowed paths: cleanup evidence/testplan/decision/traceability/journal paths.

Likely future forbidden paths: qsc source mutation, service integration, public claims.

P0/P1/P2 risks:
- P0: retained unsafe state is discovered.
- P1: cleanup requires out-of-scope remote mutation.
- P2: unnecessary cleanup lane consumes host availability.

## Selected future implementation design

### NA-0523 -- QSL Remote qsc E2EE Replay / Corrupt Delivery Negative Boundary Implementation Harness

Status: READY

Goals: G1, G2, G3, G4, G5

Objective:

Execute a bounded remote qsc E2EE negative hardening run after NA-0521 success, using retained remote qsc and the proven loopback reverse-forwarding path to rerun synthetic Build-to-Inspiron E2EE setup, then prove replayed delivery and corrupt delivery artifacts fail closed without unexpected state mutation, preserving synthetic data, isolated local/remote roots, cleanup, no qsl-server/qsl-attachments, and no public/production readiness claims.

Allowed scope:
- `docs/governance/evidence/NA-0523_qsl_remote_qsc_e2ee_replay_corrupt_negative_boundary_implementation_harness.md`
- `tests/NA-0523_qsl_remote_qsc_e2ee_replay_corrupt_negative_boundary_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- proof-root-local local/remote command output
- bounded SSH to inspiron as qslcodex
- dedicated-key reverse forwarding
- retained remote qsc binary
- synthetic messages only
- remote artifacts only under `$HOME/qsl-remote-test/e2ee/<PROOF_ID>`
- local sensitive runtime under proof root
- cleanup/retention proof

Forbidden scope:
- qsl-server/qsl-attachments
- package installation
- sudo/admin action except negative `sudo -n true` probe
- key generation/installation
- authorized_keys mutation
- SSH config mutation outside proof root
- known_hosts mutation
- remote host mutation outside the qsl remote E2EE root
- remote source checkout/build
- qwork/qstart/qresume
- qsl-backup
- qsc source/test/fuzz/Cargo mutation
- workflow/dependency mutation
- corpus/vector/input mutation
- formal/refimpl/service/public/backup mutation
- production/user data
- no public-readiness claim and no production-readiness claim

Acceptance criteria:
- retained qsc hash/path/owner rechecked.
- forwarding path rechecked.
- baseline synthetic remote E2EE setup reaches the negative test point.
- replayed delivery fails closed or is deferred with exact rationale.
- corrupt delivery fails closed or is deferred with exact rationale.
- selected state no-mutation checks pass for executed negatives.
- valid path remains usable if applicable.
- no secret material in checked-in proof.
- cleanup/retention result recorded.
- exactly one READY item remains after closeout.

## Future command family

Future NA-0523 may run:
- qwork proof reading only; Codex must not run qwork.
- retained remote qsc hash/path/owner recheck.
- local qsc build/selection from clean checkout.
- dedicated-key reverse forwarding.
- local qsc relay serve.
- qsc vault/identity/contact/relay/handshake/send/receive commands already used in NA-0521.
- synthetic artifact transfer only.
- one replay negative attempt.
- one corrupt delivery negative attempt.
- cleanup.

Future NA-0523 must not:
- use qsl-server or qsl-attachments.
- install packages.
- mutate qsc source.
- run remote source checkout/build.
- use production or personal data.
- make no replay-proof claim, no downgrade-proof claim, no crypto-complete claim, no public-ready claim, and no production-ready claim.

Future replay negative design:
- capture or identify a delivery artifact from the successful synthetic message path, or use the local qsc relay duplicate path where it preserves a duplicate synthetic delivery.
- attempt to receive/process the same delivery artifact again.
- expected result: reject/fail-closed/no duplicate output/no unexpected session mutation.
- if the delivery system consumes and removes the artifact such that replay cannot be reproduced without internals, record deferral and use corrupt delivery instead.

Future corrupt delivery design:
- create a copy of a synthetic delivery artifact under the allowed E2EE runtime root.
- mutate one byte or replace content with a synthetic corrupt marker, only under the allowed runtime root.
- attempt receive/process.
- expected result: reject/fail-closed/no output/no unexpected session mutation.
- no private material may appear in corrupt artifact evidence.

## Future proof / redaction rules

Future NA-0523 proof must:
- include no private keys.
- include no passphrases.
- include no tokens.
- include no passwords.
- include no production endpoints.
- include no backup material.
- include no personal data.
- use synthetic messages only.
- keep raw local/remote qsc output under proof root.
- summarize in checked-in evidence: commands run, synthetic labels, hash/path checks, replay result, corrupt result, no-mutation checks, and cleanup/retention status.

Checked-in evidence must redact route-token and capability-bearing metadata. If raw output contains values that could act as capability material, only short hashes or descriptive counts may be checked in.

## Future stop conditions

Future NA-0523 must stop if any of the following occurs:
- retained qsc mismatch.
- forwarding failure.
- command surface ambiguity.
- private material exposure.
- production/personal data exposure.
- qsl-server/qsl-attachments required.
- package install required.
- source mutation required.
- replay or corrupt negative fails open without selecting remediation.
- cleanup fails.
- public/production readiness claim pressure.

## Best-Known-Method Review

The best next direct qsc hardening step is to reuse the proven NA-0521 Build-to-Inspiron setup and add the smallest high-value negative cases that exercise active attacker delivery behavior. Replay and corrupt delivery are closer to the active-network threat model than service integration or unsupported public-readiness packaging, which is rejected, and they keep the limited remote-host window focused on direct qsc client-to-client evidence.

## Hostile Cryptographer Review

A replay/corrupt remote negative lane proves bounded failure behavior only. It does not prove replay-proof status, vulnerability-free status, crypto completeness, downgrade resistance, side-channel safety, or secret-lifecycle completeness.

What remains unproven after one or two negative cases:
- general replay resistance across all transports, epochs, and state-restoration cases.
- corrupt delivery behavior for every artifact format and every tamper position.
- stale-trust, wrong-peer, downgrade, rollback, and side-channel behavior.
- external review and formal model coverage for the exact remote harness.

Side-channel and secret-lifecycle caveats remain. Synthetic artifacts reduce exposure because they are generated under isolated qsc roots, use synthetic messages, and are cleaned up, but the future lane must still scan raw output and avoid checking in capability-bearing material.

Public/production readiness remains unclaimed because the evidence is a bounded harness run, not a release audit, not an external review, and not a comprehensive proof.

## Red-Team Review

If a replay attempt is impossible because artifact semantics consume/remove messages, NA-0523 must record exact deferral rationale and run the corrupt delivery negative if it can be staged safely.

If corrupt delivery accidentally mutates valid state, the lane must stop and select remediation rather than treating the result as hardening evidence.

If route-token or capability metadata leaks into evidence, publication must stop until evidence is redacted or regenerated safely.

If cleanup leaves artifacts, the lane must stop unless a bounded cleanup/remediation successor is selected.

After replay/corrupt boundaries, the next hardening candidates should be wrong-peer/stale-trust and repeated-run cleanup/freshness.

## Production SRE Review

Replay/corrupt negative testing is the right next remote hardening step because it reuses the known direct qsc path, adds attacker-oriented evidence, and avoids new services while the remote host may be briefly available.

Future NA-0523 should log:
- command family and timestamps.
- retained qsc path/owner/hash recheck.
- forwarding recheck.
- synthetic labels.
- negative result markers.
- selected state digests/counts before and after negatives.
- cleanup status.

Future NA-0523 should redact route-token/capability material, private key/passphrase/password/token material, production endpoints, backup material, and personal data.

Cleanup proof must show remote E2EE root cleanup or explicitly recorded retention under the allowed root, local sensitive runtime cleanup, and no stale relay/forwarding process for the proof root.

Failures remain isolated because qwork and qsl-backup are not run, production data is forbidden, qsl-server/qsl-attachments are deferred, and remote writes stay under the allowed qsl remote E2EE root.

This does not imply public or production readiness.

## Release-Claim Boundary Review

No public-readiness claim is made. No production-readiness claim is made. No public-internet-readiness claim is made. No external-review-complete claim is made. No crypto-complete claim is made. No replay-proof claim is made. No downgrade-proof claim is made. No secret-material-complete claim is made. No side-channel-free claim is made. No vulnerability-free, bug-free, or perfect-crypto claim is made.

## D328 assurance review

Level-1 stewardship and D328 assurance requirements are applied as advisory review, with the Lead Director retaining final authority.

Side-Channel Caveat: replay/corrupt delivery negatives do not prove side-channel freedom or constant-time behavior.

Formal-Model Mapping Residual: NA-0523 should record whether the observed replay/corrupt boundary maps to existing formal checks or remains a future formal-model gap. NA-0522 does not mutate formal models.

External-Review Readiness: NA-0523 evidence may become part of a future review package, but neither NA-0522 nor NA-0523 can claim external review completion.

Assurance Gap Review Trigger: if NA-0523 finds fail-open behavior, state mutation on reject, secret/capability leakage, cleanup failure, or command-surface ambiguity, the next lane should be remediation or diagnostic rather than claim expansion.

## Prioritization matrix

| Candidate | Risk reduced | Directness of evidence | Speed while host is available | Security risk | Operator burden | Implementation feasibility | Scope risk | Remote mutation risk | Secret/key risk | Public-claim risk | Recommended disposition | Next-lane yes/no |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Replay + corrupt delivery negative implementation | Replay/tamper active delivery risk | High | High | Medium | Medium | Medium-high | Medium | Medium | Medium | Low | Select | Yes |
| Replay-only negative implementation | Duplicate delivery risk | High | High | Medium | Low-medium | High | Low-medium | Low-medium | Low-medium | Low | Fallback | No |
| Corrupt-only negative implementation | Tamper/auth failure risk | High | Medium | Medium | Medium | Medium | Medium | Medium | Medium | Low | Fallback | No |
| Wrong-peer/stale-trust negative implementation | Identity/trust misuse risk | Medium-high | Medium | Medium | Medium | Medium | Medium | Medium | Medium | Low | Defer | No |
| Repeated-run cleanup/freshness hardening | Operational stale-state risk | Medium | High | Low-medium | Low-medium | High | Low | Low-medium | Low-medium | Low | Supporting criterion | No |
| Scheduled red-check diagnostic lane | CI diagnostic risk | Medium | Low-medium | Low | Medium | Medium | Medium | Low-medium | Low | Low | Defer | No |
| qsl-server/qsl-attachments integration | Service integration risk | Low for direct qsc | Low | High | High | Medium | High | High | Medium-high | Medium | Defer | No |
| Public/production readiness package | Claim packaging only | Low | Low | High | High | Low | High | Variable | High | High | Reject | No |
| Cleanup/remediation lane | Residual cleanup risk | Medium if unsafe state found | Medium | Low-medium | Medium | High if needed | Low-medium | Medium | Low-medium | Low | Reject unless unsafe state appears | No |

## Authorization decision

Primary classification: `REMOTE_E2EE_REPLAY_CORRUPT_NEGATIVE_IMPLEMENTATION_READY`.

Required selection proof:
- NA-0521/D416 consumed.
- Residual inventory completed.
- Option review completed.
- Future command family selected.
- Redaction/stop rules selected.
- Hostile Cryptographer Review completed.
- Red-Team Review completed.
- Production SRE Review completed.
- Release-Claim Boundary Review completed.
- Prioritization matrix completed.
- Exact NA-0523 successor selected.
- No remote action in NA-0522.
- No SSH execution in NA-0522.
- No qsc send/receive in NA-0522.
- No qsl-server/qsl-attachments selected.
- No qsc source/test/fuzz/Cargo mutation.
- No public claim expansion.
- Exactly one READY successor remains mandatory.

## Selected NA-0523 successor

`NA-0523 -- QSL Remote qsc E2EE Replay / Corrupt Delivery Negative Boundary Implementation Harness`

This successor is direct remote qsc negative hardening. It should not become a process/tooling lane, service integration lane, public-readiness package, or cleanup/remediation lane unless a future directive records an exact blocker.

## Future scope bundle

Future NA-0523 bundle:
- retained qsc freshness proof.
- forwarding recheck proof.
- baseline synthetic E2EE readiness proof.
- one replay negative attempt.
- one corrupt delivery negative attempt.
- no-mutation proof for executed negatives.
- valid path still usable proof where applicable.
- no-secret-output proof.
- cleanup/retention proof.
- no qsl-server/qsl-attachments proof.
- no public/production claim proof.
- one-READY invariant proof after closeout.

## Future validation / marker plan

Future markers:
- `NA0523_REMOTE_E2EE_NEGATIVE_SCOPE_CONSUMED_OK`
- `NA0523_RETAINED_QSC_HASH_RECHECKED_OK`
- `NA0523_FORWARDING_PATH_RECHECKED_OK`
- `NA0523_BASELINE_REMOTE_E2EE_READY_FOR_NEGATIVE_OK`
- `NA0523_REPLAY_NEGATIVE_REJECTED_OK`
- `NA0523_CORRUPT_DELIVERY_REJECTED_OK`
- `NA0523_NEGATIVE_NO_MUTATION_OK`
- `NA0523_VALID_PATH_REMAINS_USABLE_OK`
- `NA0523_NO_SECRET_OUTPUT_OK`
- `NA0523_CLEANUP_COMPLETED_OK`
- `NA0523_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0523_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0523_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0523_ONE_READY_INVARIANT_OK`

## No remote action in NA-0522

NA-0522 did not run SSH, scp, sftp, rsync, qsc send/receive, remote qsc, remote E2EE, remote source checkout/build, remote package installation, sudo/admin action, key generation/installation, authorized_keys mutation, SSH config mutation, known_hosts mutation, remote host mutation, qwork/qstart/qresume, qsl-backup, backup, or restore.

## No qsl-server / no qsl-attachments boundary

qsl-server and qsl-attachments are deferred. NA-0522 selects direct qsc client-to-client replay/corrupt delivery hardening only. It does not authorize qsl-server or qsl-attachments use, setup, test execution, source mutation, or service integration in NA-0523.

## Public claim / website / external review boundary

NA-0522 does not mutate public docs, README, START_HERE, website, public technical paper material, or external-review artifacts. NA-0523 must preserve the same no-claim boundary unless a later explicit directive changes scope.

No public-readiness claim is made. No production-readiness claim is made. No public-internet-readiness claim is made. No external-review-complete claim is made. No crypto-complete claim is made. No replay-proof claim is made. No downgrade-proof claim is made. No secret-material-complete claim is made. No side-channel-free claim is made. No vulnerability-free, bug-free, or perfect-crypto claim is made.

## Backup-impact statement

No backup or restore ran. qsl-backup was inspected read-only by hash/source-list proof only. `/backup/qsl` was not mutated. NA-0523 must not run qsl-backup or mutate backup state.

## Rejected alternatives

Rejected or deferred alternatives:
- public/production readiness package: rejected as premature.
- qsl-server/qsl-attachments integration: deferred outside the direct qsc sprint.
- scheduled red-check diagnostic lane: deferred unless required checks fail or a later directive selects diagnostics.
- cleanup/remediation lane: rejected unless unsafe D416 state appears.
- wrong-peer/stale-trust: deferred behind replay/corrupt direct delivery negatives.
- repeated-run cleanup/freshness: included as support but not primary.

## Next recommendation

After the NA-0522 authorization PR merges and closeout restores the successor, proceed directly to NA-0523 while the remote host may still be available.
