Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-22

# NA-0524 QSL Remote qsc E2EE Wrong-Peer / Stale-Trust Negative Scope Authorization Plan

## Executive summary

NA-0524 is authorization-only. It consumes NA-0523 / D419 replay and corrupt-delivery negative evidence, inventories the remaining direct remote qsc E2EE identity/trust residuals, reviews implementation options, and selects the exact next lane:

`NA-0525 -- QSL Remote qsc E2EE Wrong-Peer / Stale-Trust Negative Implementation Harness`

Primary classification: `REMOTE_E2EE_WRONG_PEER_STALE_TRUST_IMPLEMENTATION_READY`.

No remote action occurred in NA-0524. No SSH execution occurred in NA-0524. No qsc send/receive occurred in NA-0524. No remote E2EE occurred in NA-0524. qsl-server and qsl-attachments remain deferred. No public-readiness claim is made. No production-readiness claim is made. No public-internet-readiness claim is made. No external-review-complete claim is made. No crypto-complete claim is made. No identity-complete claim is made. No trust-complete claim is made. No replay-proof claim is made. No downgrade-proof claim is made. No secret-material-complete claim is made. No side-channel-free claim is made. No vulnerability-free, bug-free, or perfect-crypto claim is made.

## Live NA-0524 scope

NA-0524 is the sole READY item at startup and is limited to governance evidence, one testplan, D-1037, TRACEABILITY, and the rolling operations journal.

Allowed checked-in mutation paths:

- `docs/governance/evidence/NA-0524_qsl_remote_qsc_e2ee_wrong_peer_stale_trust_negative_scope_authorization_plan.md`
- `tests/NA-0524_qsl_remote_qsc_e2ee_wrong_peer_stale_trust_negative_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

NA-0524 does not authorize qsc source/test/fuzz/Cargo mutation, workflow/script/helper mutation, dependency/lockfile mutation, corpus/vector/input mutation, formal/refimpl/service/public/backup mutation, qsl-server mutation, qsl-attachments mutation, qshield or qshield-cli mutation, qsl-backup execution or mutation, public docs mutation, website mutation, archive/move/delete activity, remote action, SSH execution, qsc send/receive, or remote E2EE execution.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

The operator-provided qwork proof files were read and copied into the directive proof root:

- `/srv/qbuild/work/NA-0524/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0524/.qwork/startup.qsl-protocol.json`

Required qwork proof fields were verified:

- `startup_result=OK`
- `lane=NA-0524`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0524/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0524`
- `requested_lane_status=READY`

Freshness proof:

- Proof HEAD matched live HEAD before fetch: `f4b60a92089b`.
- Proof origin/main matched live origin/main before fetch: `f4b60a92089b`.
- Fetch occurred only after the proof/live ref match.
- `/` usage was below the 95% STOP threshold at startup.
- `/backup/qsl` was checked read-only and was below the 95% STOP threshold.

Post-fetch proof:

- `origin/main` equaled and therefore descended from `f4b60a92089b`.
- `main` was fast-forward clean from `origin/main`.
- READY_COUNT was 1.
- READY item was `NA-0524 -- QSL Remote qsc E2EE Wrong-Peer / Stale-Trust Negative Scope Authorization Plan`.
- NA-0523, NA-0522, and NA-0521 were DONE.
- D-1035 existed once.
- D-1036 existed once.
- D-1037 was absent before this patch.
- Duplicate decision ID count was zero using the repository-native `- **ID:** D-####` parser.

## NA-0523 / D419 inheritance

NA-0523 completed and NA-0524 was restored READY.

Inherited classification:

`REMOTE_E2EE_REPLAY_CORRUPT_NEGATIVES_PASS`

Inherited facts consumed:

- D418 startup parser stop was recovered; D418 made no repo mutation, opened no PR, built or ran no qsc, executed no SSH or remote command, created no E2EE runtime root, and used no qsl-server/qsl-attachments.
- Retained remote qsc was rechecked.
- Dedicated-key forwarding was rechecked.
- Baseline remote E2EE setup succeeded using synthetic data and isolated roots.
- Replay negative passed: duplicate delivery was rejected with `qsp_replay_reject` / `ratchet_replay_reject`, and no duplicate plaintext artifact was produced.
- Corrupt delivery negative passed: corrupt synthetic bytes were rejected with `qsp_env_decode_failed`, and no plaintext artifact was produced.
- Selected no-mutation checks passed for replay and corrupt-delivery rejects.
- Valid path remained usable after negative cases.
- Cleanup passed: local relay stopped, SSH forward stopped, remote E2EE root removed, local sensitive runtime removed, and port 39176 closed.
- Retained remote qsc was preserved.
- qsl-server was not used.
- qsl-attachments was not used.
- No public-readiness claim is inherited.
- No production-readiness claim is inherited.
- No replay-proof claim is inherited.
- No downgrade-proof claim is inherited.

## Residual inventory

| Residual | Classification | Current evidence | NA-0525 disposition |
|---|---|---|---|
| Wrong-peer receive / send boundary | Direct remote negative candidate | `receive_e2e` covers wrong peer/no output mutation locally; NA-0521 covered wrong-mailbox remotely; NA-0523 did not cover identity mismatch | Select for direct remote negative implementation |
| Stale-trust / stale public-record boundary | Direct remote negative candidate | `identity_binding` and `kem_signature_transcript_binding_negative` cover stale public record / peer mismatch with no session mutation locally | Select for direct remote negative implementation if qsc surfaces can stage it without internals |
| Wrong-device fingerprint or replaced peer identity boundary | Direct remote negative candidate | qsc contacts and device trust/revoke/primary commands exist; local trust tests cover changed/revoked/no-trusted-device blocks | Include as stale-trust/replaced-peer subcase or fallback |
| Contact/trust downgrade or missing-trust boundary | Supporting local evidence exists | trust-mode and device trust surfaces exist; local send blocks no-trusted-device without send mutation | Include as fallback if stale public-record staging is ambiguous |
| Repeated-run freshness and cleanup | Process/tooling residual | NA-0521 and NA-0523 cleanup passed once each; repeated identity/trust negative cleanup is not proven | Require cleanup/retention proof in NA-0525, not primary objective |
| Retained remote qsc freshness before each run | Process/tooling residual | NA-0521 and NA-0523 rechecked retained qsc before use | Require hash/path/owner/help recheck in NA-0525 |
| Forwarding tunnel cleanup / stale process risk | Process/tooling residual | NA-0521 and NA-0523 cleaned relay/forwarding and checked port closure | Require forwarding cleanup proof in NA-0525 |
| Route-token / capability metadata redaction | Process/tooling residual | Prior proof scans passed after redaction; future identity/trust negatives may add route-token output | Require proof-root and checked-in redaction scans in NA-0525 |
| qsc output no-secret review | Process/tooling residual | NA-0521 and NA-0523 no-secret-output scans passed | Require no-secret-output proof in NA-0525 |
| Remote root cleanup/retention policy | Process/tooling residual | NA-0521 and NA-0523 removed remote E2EE roots and preserved retained qsc | Require removal or explicit safe retention proof in NA-0525 |
| Residual scheduled remote CI checks from earlier lanes | Out of current direct qsc sprint | Earlier remote-handshake/remote-relay/relay-ui scheduled checks remain diagnostic-only residuals | Defer; not selected while host time favors direct qsc identity/trust negatives |
| qsl-server/qsl-attachments boundary | Future service/attachment integration only | Direct qsc relay path did not require either service | Defer outside direct qsc sprint |
| Public/production claim boundary | Process/tooling residual | Prior lanes preserve no-claim wording | Preserve in NA-0525 |

## Option review

| Option | Disposition | Risk reduced | Evidence gap addressed | Feasibility | Scope risk | Remote mutation risk | Secret/key risk | Public-claim risk | Likely future allowed paths | Likely future forbidden paths | P0 / P1 / P2 risks |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1. Remote wrong-peer + stale-trust negative implementation | Selected | Identity/trust misuse across the direct qsc remote path | Wrong peer, stale public record, replaced peer/device behavior | High enough: qsc identity/contact/device/handshake/send/receive surfaces and local negative tests exist | Medium; must avoid internals if staging is ambiguous | Medium; bounded remote E2EE root only | Medium; synthetic identities/trust records only | Low if no-claim text is preserved | NA-0525 evidence/testplan, DECISIONS, TRACEABILITY, journal, proof-root output, bounded SSH/forwarding under future directive | qsc source/test/fuzz/Cargo, workflows, dependencies, corpora, formal/refimpl/service/public/backup, qsl-server, qsl-attachments | P0 fail-open accept or state mutation; P1 ambiguous staging; P2 verbose proof output |
| 2. Remote wrong-peer-only negative implementation | Fallback | Wrong recipient/contact/device misuse | Remote wrong-peer reject/no-output/no-mutation | High | Lower than Option 1 | Medium | Medium | Low | Same governance/proof-root family as Option 1 | Same forbidden family as Option 1 | P0 wrong peer accepted; P1 stale-trust still untested; P2 redundant setup |
| 3. Remote stale-trust-only negative implementation | Fallback | Stale or replaced trust material misuse | Stale public record / peer mismatch / replaced device reject | Medium-high | Medium | Medium | Medium | Low | Same governance/proof-root family as Option 1 | Same forbidden family as Option 1 | P0 stale trust accepted; P1 wrong-peer still untested; P2 command-surface ambiguity |
| 4. Remote repeated-run cleanup/freshness implementation | Defer | Stale process/artifact interference | Multiple-run cleanup and freshness | High | Low-medium | Medium | Low-medium | Low | Future cleanup/freshness evidence/testplan if identity/trust staging fails | qsc source mutation, service integration, public claims | P0 cleanup failure; P1 host unavailable after process lane; P2 less direct security evidence |
| 5. Remote output/redaction hardening implementation | Defer | Proof leakage and evidence hygiene | Route-token/capability/no-secret-output review | High | Low | Low | Medium if raw proof is mishandled | Low | Evidence/testplan, proof scans, no runtime beyond existing proof | runtime/service/code mutation | P0 secret/capability leak; P1 no identity/trust behavior tested; P2 scan false positives |
| 6. Scheduled red-check diagnostic lane | Defer | Earlier scheduled remote CI residuals | remote-handshake, remote-relay, relay-ui diagnostic status | Medium | Medium-high because it shifts away from direct qsc sprint | Medium | Low-medium | Low | Diagnostic evidence only | direct qsc source/service mutation unless separately authorized | P0 directs host time away from identity/trust negatives; P1 stale diagnostics; P2 unclear owner |
| 7. qsl-server/qsl-attachments integration | Defer | Service integration risk | Server/attachment path behavior | Not selected for direct qsc sprint | High | High | Medium | Medium | Future service-specific lanes only | NA-0525 direct qsc lane | P0 broadens transport/auth semantics; P1 requires different repos/services; P2 premature coupling |
| 8. Public/production readiness package | Rejected | None appropriate now | Unsupported claim packaging | Not appropriate | High | Low | Medium | High | None in this sprint | public docs/website/claim expansion | P0 unsupported public/security claim; P1 external review not complete; P2 distracts from negative testing |
| 9. Cleanup/remediation lane | Reject unless blocker appears | Remediates unsafe leftover state | D419 cleanup or retained qsc safety | Not needed because D419 cleanup passed | Low if selected only for real blocker | Medium | Low | Low | Cleanup/remediation evidence only if future proof finds unsafe state | direct implementation if root cause unclear | P0 unsafe retained state; P1 false remediation need; P2 host time loss |

## Selected future implementation design

### NA-0525 -- QSL Remote qsc E2EE Wrong-Peer / Stale-Trust Negative Implementation Harness

Status: READY

Goals: G1, G2, G3, G4, G5

Objective:

Execute a bounded remote qsc E2EE identity/trust negative hardening run after NA-0521 and NA-0523 success, using retained remote qsc and the proven loopback reverse-forwarding path to rerun synthetic Build-to-Inspiron E2EE setup, then prove wrong-peer and stale-trust or replaced-peer public-record conditions fail closed without unexpected state mutation, preserving synthetic data, isolated local/remote roots, cleanup, no qsl-server/qsl-attachments, and no public/production readiness claims.

Allowed future scope:

- `docs/governance/evidence/NA-0525_qsl_remote_qsc_e2ee_wrong_peer_stale_trust_negative_implementation_harness.md`
- `tests/NA-0525_qsl_remote_qsc_e2ee_wrong_peer_stale_trust_negative_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- proof-root-local local/remote command output.
- bounded SSH to Inspiron as qslcodex.
- dedicated-key reverse forwarding.
- retained remote qsc binary.
- synthetic messages only.
- synthetic public/trust records only.
- remote artifacts only under `$HOME/qsl-remote-test/e2ee/<PROOF_ID>`.
- local sensitive runtime under proof root.
- cleanup/retention proof.

Forbidden future scope:

- qsl-server/qsl-attachments.
- package installation.
- sudo/admin action except negative `sudo -n true` probe.
- key generation/installation outside isolated qsc runtime.
- authorized_keys mutation.
- SSH config mutation outside proof root.
- known_hosts mutation.
- remote host mutation outside qsl-remote-test E2EE root.
- remote source checkout/build.
- qwork/qstart/qresume.
- qsl-backup.
- qsc source/test/fuzz/Cargo mutation.
- workflow/dependency mutation.
- corpus/vector/input mutation.
- formal/refimpl/service/public/backup mutation.
- production/user data.
- no public-readiness claim and no production-readiness claim.

Acceptance criteria:

- retained qsc hash/path/owner rechecked.
- forwarding path rechecked.
- baseline remote E2EE setup reaches identity/trust negative test point.
- wrong-peer negative fails closed or is deferred with exact rationale.
- stale-trust/replaced-peer negative fails closed or is deferred with exact rationale.
- selected state no-mutation checks pass for executed negatives.
- valid path remains usable if applicable.
- no secret material in checked-in proof.
- cleanup/retention result recorded.
- exactly one READY item remains after closeout.

## Future command family

Future NA-0525 may run, only under its own directive:

- qwork proof reading only; Codex must not run qwork.
- retained remote qsc hash/path/owner recheck.
- local qsc build/selection from clean checkout.
- dedicated-key reverse forwarding.
- local qsc relay serve.
- qsc vault/identity/contact/relay/handshake/send/receive commands already used in NA-0521 and NA-0523.
- synthetic public/trust record manipulation only under allowed runtime roots.
- one wrong-peer negative attempt.
- one stale-trust/replaced-peer negative attempt.
- cleanup.

Future NA-0525 must not:

- use qsl-server or qsl-attachments.
- install packages.
- mutate qsc source.
- run remote source checkout/build.
- use production/personal data.
- make no identity-complete claim, no trust-complete claim, no replay-proof claim, no downgrade-proof claim, no crypto-complete claim, no public-ready claim, and no production-ready claim.

Future wrong-peer negative design:

- create a synthetic third identity or mismatched contact using existing qsc identity/contact surfaces if feasible.
- attempt send/receive/handshake against the wrong peer/contact/device.
- expected result: reject/fail-closed/no plaintext output/no unexpected state mutation.
- if a wrong-peer case cannot be staged without internals, record deferral and use stale-trust if safe.

Future stale-trust/replaced-peer design:

- establish a valid synthetic trust relationship.
- create a second synthetic public record/device fingerprint for the same peer label or stale trust state if existing qsc surfaces allow.
- attempt to proceed with stale or replaced trust material.
- expected result: reject/fail-closed/no plaintext output/no unexpected state mutation.
- if stale-trust cannot be staged without internals, record deferral and use wrong-peer if safe.

## Future proof / redaction rules

Future NA-0525 proof must:

- include no private keys.
- include no passphrases.
- include no tokens.
- include no passwords.
- include no production endpoints.
- include no backup material.
- include no personal data.
- use synthetic identities/messages only.
- keep raw local/remote qsc output under proof root.
- summarize in checked-in evidence:
  - commands run;
  - synthetic labels;
  - hash/path checks;
  - wrong-peer result;
  - stale-trust result;
  - no-mutation checks;
  - valid-path usability result;
  - cleanup/retention status.

## Future stop conditions

Future NA-0525 must stop for:

- retained qsc mismatch.
- forwarding failure.
- command surface ambiguity.
- private material exposure.
- production/personal data.
- qsl-server/qsl-attachments requirement.
- package install requirement.
- source mutation requirement.
- wrong-peer/stale-trust negative fail-open result without selecting remediation.
- cleanup failure.
- pressure to weaken the no public-readiness claim or no production-readiness claim boundary.

## Hostile Cryptographer Review

Wrong-peer/stale-trust negative testing proves only bounded identity/trust failure behavior for the staged synthetic cases. It does not prove identity-complete or trust-complete status.

Remaining unproven areas include broader authentication composition, replay classes beyond NA-0523's staged duplicate, downgrade resistance beyond existing local tests, side channels, retained-binary replacement races, secret-material lifecycle completeness, formal model coverage for these exact remote cases, and external review.

Synthetic identities and trust records avoid production/personal data exposure because the future lane must create isolated local/remote qsc roots, synthetic labels, synthetic messages, and synthetic public/trust records only under proof-root-local and `$HOME/qsl-remote-test/e2ee/<PROOF_ID>` runtime paths.

No public-readiness or production-readiness claim is made because this is a bounded remote negative harness, not broad assurance, public deployment, independent review, or release qualification.

## Red-Team Review

If wrong-peer cannot be staged because qsc contact surfaces prevent it, future NA-0525 must record exact deferral rationale and execute stale-trust/replaced-peer if safe.

If stale-trust or replaced-peer surfaces require internals, future NA-0525 must not mutate qsc source or private stores unsafely; it must either use existing qsc identity/contact/device surfaces or record deferral.

If a stale-trust negative mutates valid state unexpectedly, future NA-0525 must stop and select remediation or diagnostic scope rather than treating the negative as passed.

If route-token/capability metadata leaks into evidence, future NA-0525 must stop publication, redact proof-root surfaces, rerun scans, and record the recovery before proceeding.

If cleanup leaves artifacts, future NA-0525 must stop or explicitly record safe retention with exact paths and no secret material.

After wrong-peer/stale-trust, the next hardening candidates should be repeated-run freshness, output/redaction hardening, or service/attachment integration only after direct qsc residuals are stronger.

## Production SRE Review

Wrong-peer/stale-trust negative testing is the right next remote hardening step because NA-0521 proved bounded two-host positive E2EE plus wrong-mailbox no-mutation, and NA-0523 proved replay/corrupt-delivery rejects. Identity/trust misuse remains the next direct qsc client-to-client risk while the remote host may still be available.

Future NA-0525 should log command manifests, retained qsc hash/path/owner checks, forwarding checks, qsc stdout/stderr under proof root, selected state digests/counts before and after rejects, no-secret-output scan summaries, and cleanup/retention proof. Checked-in evidence should summarize only sanitized labels, hashes, markers, and outcomes.

Cleanup proof must include local relay stop, SSH forward stop, remote E2EE root removal or explicit safe retention, local sensitive runtime cleanup, and port/process checks scoped to the future proof.

Failures remain isolated because qwork and qsl-backup are not run by Codex, production data is forbidden, qsl-server/qsl-attachments remain deferred, package install is forbidden, and future remote writes stay under the allowed remote E2EE root.

qsl-server and qsl-attachments remain deferred because the selected risk is direct qsc identity/trust misuse in the retained qsc path, not service integration.

This does not imply public readiness or production readiness.

## Release-Claim Boundary Review

No public-ready claim is made.

No production-ready claim is made.

No public-internet-ready claim is made.

No external-review-complete claim is made.

No crypto-complete claim is made.

No identity-complete claim is made.

No trust-complete claim is made.

No replay-proof claim is made.

No downgrade-proof claim is made.

No secret-material-complete claim is made.

No side-channel-free claim is made.

No vulnerability-free, bug-free, or perfect-crypto claim is made.

## Best-Known-Method Review

The selected next lane reuses the best-known remote qsc method from NA-0521 and NA-0523: fresh qwork proof-file verification by the operator, retained qsc hash/path/owner recheck, proof-root-local local runtime, dedicated-key loopback reverse forwarding, local qsc relay serve, synthetic identities/messages/trust records, explicit no-secret-output review, selected no-mutation checks, valid-path usability where applicable, cleanup/retention proof, and strict no qsl-server/qsl-attachments boundaries.

## Side-Channel Caveat

NA-0524 does not evaluate timing, cache behavior, memory access patterns, traffic analysis, or other side-channel classes. Future NA-0525 may record deterministic command outcomes and metadata-minimization observations, but no side-channel-free claim is made.

## Formal-Model Mapping Residual

Local formal and vector evidence includes binding and stale public-record classes, and NA-0524 maps the future remote lane to those existing invariants. The exact two-host remote wrong-peer/stale-trust execution remains a formal-model mapping residual until a later lane records whether model coverage exists or should be expanded. NA-0524 does not mutate formal models.

## External-Review Readiness

NA-0524 evidence can become input to a future external review package because it preserves scope, proof, redaction, and no-claim boundaries. It does not claim external review completion, public readiness, production readiness, crypto completeness, identity completeness, or trust completeness.

## Assurance Gap Review Trigger

Future NA-0525 must select remediation or diagnostics rather than claim expansion if it finds fail-open behavior, unexpected state mutation, secret/capability leakage, retained qsc mismatch, forwarding failure, command-surface ambiguity, cleanup failure, qsl-server/qsl-attachments requirement, source mutation requirement, or pressure to weaken the no public-readiness claim or no production-readiness claim boundary.

## Prioritization matrix

| Candidate | Risk reduced | Directness of evidence | Speed while host is available | Security risk | Operator burden | Implementation feasibility | Scope risk | Remote mutation risk | Secret/key risk | Public-claim risk | Recommended disposition | Next-lane yes/no |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Wrong-peer + stale-trust negative implementation | High identity/trust misuse risk | High | High | Medium | Medium | Medium-high | Medium | Medium | Medium | Low | Select | Yes |
| Wrong-peer-only negative implementation | Medium-high wrong-peer misuse | High | High | Medium | Low-medium | High | Low-medium | Medium | Medium | Low | Fallback | No |
| Stale-trust-only negative implementation | Medium-high stale trust misuse | High | Medium-high | Medium | Medium | Medium-high | Medium | Medium | Medium | Low | Fallback | No |
| Repeated-run cleanup/freshness implementation | Medium stale process/artifact risk | Medium | Medium | Low-medium | Medium | High | Low-medium | Medium | Low-medium | Low | Defer | No |
| Output/redaction hardening implementation | Medium proof leakage risk | Medium | Medium | Low | Low | High | Low | Low | Medium | Low | Defer | No |
| Scheduled red-check diagnostic lane | Medium older scheduled residual risk | Low-medium for direct qsc | Low-medium | Low-medium | Medium | Medium | Medium-high | Medium | Low-medium | Low | Defer | No |
| qsl-server/qsl-attachments integration | Service/attachment risk | Low for direct qsc | Low | High | High | Medium | High | High | Medium-high | Medium | Defer | No |
| Public/production readiness package | None appropriate | Unsupported | Low | High | High | Low | High | Low | Medium | High | Reject | No |
| Cleanup/remediation lane | High only if cleanup unsafe | High if blocker appears | Medium | Low-medium | Medium | High only for known issue | Low-medium | Medium | Low | Low | Reject unless blocker appears | No |

## Authorization decision

Primary classification:

`REMOTE_E2EE_WRONG_PEER_STALE_TRUST_IMPLEMENTATION_READY`

Required authorization conditions are satisfied:

- NA-0523 / D419 was consumed.
- Residual inventory was completed.
- Option review was completed.
- Future command family was selected.
- Redaction and stop rules were selected.
- Hostile Cryptographer, Red-Team, Production SRE, and Release-Claim reviews were completed.
- Prioritization matrix was completed.
- Exact NA-0525 successor was selected.
- No remote action occurred in NA-0524.
- No SSH execution occurred in NA-0524.
- No qsc send/receive occurred in NA-0524.
- qsl-server/qsl-attachments were not selected.
- No qsc source/test/fuzz/Cargo mutation occurred.
- No public claim expansion occurred.
- Exactly one READY successor remains mandatory.

## Selected NA-0525 successor

`NA-0525 -- QSL Remote qsc E2EE Wrong-Peer / Stale-Trust Negative Implementation Harness`

NA-0525 should be restored by a separate closeout after the NA-0524 authorization PR merges and post-merge public-safety is green inside the permitted attach/early-failure window.

## Future scope bundle

Future NA-0525 bundle:

- inherited NA-0524 scope evidence.
- retained qsc hash/path/owner recheck.
- forwarding path recheck.
- baseline remote E2EE setup to identity/trust negative test point.
- wrong-peer reject/no-mutation proof or exact deferral.
- stale-trust/replaced-peer reject/no-mutation proof or exact deferral.
- valid path usability proof if applicable.
- no-secret-output proof.
- cleanup/retention proof.
- no qsl-server/qsl-attachments proof.
- no public-readiness or production-readiness claim proof.
- one-READY invariant proof after closeout.

## Future validation / marker plan

Future NA-0525 markers:

- `NA0525_REMOTE_E2EE_IDENTITY_TRUST_NEGATIVE_SCOPE_CONSUMED_OK`
- `NA0525_RETAINED_QSC_HASH_RECHECKED_OK`
- `NA0525_FORWARDING_PATH_RECHECKED_OK`
- `NA0525_BASELINE_REMOTE_E2EE_READY_FOR_NEGATIVE_OK`
- `NA0525_WRONG_PEER_NEGATIVE_REJECTED_OK`
- `NA0525_STALE_TRUST_NEGATIVE_REJECTED_OK`
- `NA0525_NEGATIVE_NO_MUTATION_OK`
- `NA0525_VALID_PATH_REMAINS_USABLE_OK`
- `NA0525_NO_SECRET_OUTPUT_OK`
- `NA0525_CLEANUP_COMPLETED_OK`
- `NA0525_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0525_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0525_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0525_ONE_READY_INVARIANT_OK`

## No remote action in NA-0524

NA-0524 did not execute SSH, scp, sftp, rsync, qsc send/receive, remote qsc, remote E2EE, qsl-server, qsl-attachments, qshield, qshield-cli, package installation, sudo/admin action, key generation, key installation, authorized_keys mutation, SSH config mutation, known_hosts mutation, sshd_config mutation, remote host mutation, qwork/qstart/qresume, qsl-backup, or backup/restore.

## No qsl-server / no qsl-attachments boundary

qsl-server and qsl-attachments are deferred. NA-0524 selects direct qsc client-to-client wrong-peer/stale-trust hardening only. It does not authorize qsl-server or qsl-attachments use, setup, test execution, source mutation, dependency mutation, workflow mutation, or service integration in NA-0525.

## Public claim / website / external review boundary

NA-0524 does not mutate public docs, README, START_HERE, website, public technical paper material, external-review artifacts, service docs, or claim surfaces. NA-0525 must preserve the same no-claim boundary unless a later explicit directive changes scope.

## Backup-impact statement

No backup or restore ran. qsl-backup was inspected read-only by hash/source-list proof only. `/backup/qsl` was not mutated. The NA-0524 checked-in scope remains under tracked qsl-protocol governance/testplan/traceability/journal files and does not require backup-plan mutation. Future NA-0525 must not run qsl-backup or mutate backup state.

## Rejected alternatives

- qsl-server/qsl-attachments integration: rejected for the direct qsc sprint because direct identity/trust misuse remains untested.
- Public/production readiness package: rejected as unsupported by current evidence.
- Cleanup/remediation lane: rejected because D419 cleanup and retained qsc state were safe; select only if future evidence finds unsafe state.
- Scheduled red-check diagnostic lane: deferred because the limited remote-host window favors direct qsc wrong-peer/stale-trust hardening.
- Output/redaction-only lane: deferred because redaction remains mandatory support work inside the selected NA-0525 lane.

## Next recommendation

After the NA-0524 authorization PR merges and closeout restores the successor, proceed directly to NA-0525 while the remote host may still be available. NA-0525 should attempt wrong-peer and stale-trust/replaced-peer negatives using only existing qsc surfaces and stop rather than fabricate evidence if staging requires internals, source mutation, services, package installation, production data, or public/production claim expansion.
