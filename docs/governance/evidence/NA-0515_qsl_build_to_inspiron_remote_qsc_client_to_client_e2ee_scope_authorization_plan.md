Status: Supporting
Owner: QSL Governance / Director
Last-Updated: 2026-06-21

Goals: G1, G2, G3, G4, G5

# NA-0515 Build-to-Inspiron remote qsc client-to-client E2EE scope authorization plan

## Executive summary

NA-0515 is an authorization-only governance and security lane. It consumes NA-0514 / D407, reviews the retained Inspiron qsc binary evidence, reviews Build-to-Inspiron E2EE options, and selects the exact future NA-0516 implementation scope.

Primary classification: `REMOTE_BUILD_TO_INSPIRON_E2EE_IMPLEMENTATION_READY`.

Selected successor: `NA-0516 -- QSL Build-to-Inspiron Remote qsc Client-to-Client E2EE Implementation Harness`.

NA-0515 does not run remote commands, does not execute SSH, does not transfer files, does not run qsc send/receive, does not execute remote E2EE, and does not mutate qsc source, qsc tests, fuzz inputs, Cargo files, workflows, dependencies, formal assets, reference implementations, services, public documents, backup assets, or remote host state.

Markers:
- `NA0515_QWORK_PROOF_FILES_VERIFIED_OK`
- `NA0515_NA0514_D407_INHERITANCE_CONSUMED_OK`
- `NA0515_RETAINED_REMOTE_QSC_REVIEWED_OK`
- `NA0515_E2EE_OPTIONS_REVIEWED_OK`
- `NA0515_EXACT_FUTURE_COMMAND_FAMILY_SELECTED_OK`
- `NA0515_REDACTION_STOP_RULES_SELECTED_OK`
- `NA0515_STEWARDSHIP_REVIEWS_COMPLETED_OK`
- `NA0515_PRIORITY_MATRIX_COMPLETED_OK`
- `NA0515_REMOTE_BUILD_TO_INSPIRON_E2EE_IMPLEMENTATION_READY`
- `NA0515_SELECTED_NA0516_SUCCESSOR_OK`
- `NA0515_NO_REMOTE_ACTION_BY_CODEX_OK`
- `NA0515_NO_SSH_EXECUTION_BY_CODEX_OK`
- `NA0515_NO_QSC_SEND_RECEIVE_OK`
- `NA0515_NO_REMOTE_E2EE_OK`
- `NA0515_NO_IMPLEMENTATION_MUTATION_OK`
- `NA0515_NO_QSC_SOURCE_TEST_FUZZ_CARGO_MUTATION_OK`
- `NA0515_NO_WORKFLOW_DEPENDENCY_MUTATION_OK`
- `NA0515_NO_CORPUS_VECTOR_INPUT_MUTATION_OK`
- `NA0515_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0515_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0515_ONE_READY_INVARIANT_OK`

## Live NA-0515 scope

Allowed NA-0515 mutations are limited to this evidence file, the NA-0515 testplan, `DECISIONS.md`, `TRACEABILITY.md`, and `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

The lane is authorization-only. It may read qwork proof, governance files, D407/D406/D404 response files, prior NA evidence, current qsc test patterns, qsc source surfaces, CI/audit scripts, formal model runner metadata, and backup status/source-list files listed by the directive. It must not inspect private keys, passphrases, tokens, credentials, production endpoint secrets, or backup private material.

## qwork proof-file verification

Codex verified the operator-created proof files without rerunning qwork:

- `/srv/qbuild/work/NA-0515/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0515/.qwork/startup.qsl-protocol.json`

Required proof values were present: `startup_result=OK`, `lane=NA-0515`, `repo=qsl-protocol`, `path=/srv/qbuild/work/NA-0515/qsl-protocol`, clean worktree/index/untracked state, `head_equals_origin_main=yes`, `ready_count=1`, `queue_top_ready=NA-0515`, and `requested_lane_status=READY`.

Before fetch, live `HEAD` and live `origin/main` matched the qwork proof commit `6e12e1849737bbf243f6645db86784a3836368d9`. The `.json` proof mirrored the `.kv` proof. The proof files were copied into the directive proof root for evidence retention.

## NA-0514 / D407 inheritance

NA-0514 completed and restored NA-0515 READY. D407 was consumed from `/home/victor/work/qsl/codex/responses/NA0514_20260621T152257Z_D407.md`.

Inherited facts:

- Classification: `REMOTE_PREBUILT_QSC_STAGING_SMOKE_PASS_RETAINED`.
- Implementation PR #1300 merged at `9567f845c1c5`.
- Closeout PR #1301 merged at `6e12e1849737`.
- Local source commit for staged qsc: `6e0796de79c9abb4d3c5e18b46b004b5bd585167`.
- Local/remote qsc sha256: `6f12ab5eec2468a4146012dfc247cef15be2337cb8e8e99cefa96a8c258d91ea`.
- Retained remote qsc path, qsl-remote-test-relative: `qsl-remote-test/bin/qsc`.
- Remote qsc owner: `qslcodex`.
- Remote qsc smoke used only `--help`.
- Remote qsc smoke exit code: 0.
- Cleanup command recorded: `ssh -o BatchMode=yes -o PasswordAuthentication=no -o ConnectTimeout=10 inspiron 'rm -f -- "$HOME/qsl-remote-test/bin/qsc"'`.
- No remote E2EE was executed.
- No qsc send/receive was executed.
- No remote source checkout/build was executed.
- No package installation was performed.
- no public-readiness claim is made.
- no production-readiness claim is made.
- no crypto-complete claim is made.
- no replay-proof claim is made.
- no downgrade-proof claim is made.
- no secret-material-complete claim is made.
- no side-channel-free claim is made.
- no vulnerability-free claim is made.

## Retained remote qsc binary review

The retained binary evidence is accepted for authorizing the next bounded implementation lane, not for skipping NA-0516 boundary rechecks.

Accepted retained-artifact scope:

- Path: `qsl-remote-test/bin/qsc`, relative to the remote qsl-remote-test root.
- Owner: `qslcodex`.
- Expected sha256: `6f12ab5eec2468a4146012dfc247cef15be2337cb8e8e99cefa96a8c258d91ea`.
- Provenance: local qsc binary built/selected from source commit `6e0796de79c9abb4d3c5e18b46b004b5bd585167`, staged during NA-0514, remote hash matched local hash, and only a non-protocol `--help` smoke was executed remotely.

Residual risk: the remote binary may be replaced or removed before NA-0516. Therefore NA-0516 must recheck path, owner, hash, remote account, privilege, backup/qwork/qsl-backup boundaries, and a retained-binary help/version smoke before any E2EE command.

## Remote E2EE scope goal

The future NA-0516 goal is one bounded Build-to-Inspiron qsc client-to-client E2EE implementation using synthetic messages and isolated roots:

- Build / ideacentre side uses local qsc from the current clean checkout or a proof-root-local build.
- Inspiron side uses retained qsc at `$HOME/qsl-remote-test/bin/qsc`.
- Local Build root lives under `$PROOF_DIR/remote_e2ee/local_build_root`.
- Remote Inspiron root lives under `$HOME/qsl-remote-test/e2ee/<PROOF_ID>/`.
- Default roles: Build is Alice/local sender; Inspiron is Bob/remote receiver/replier.
- A role reversal is allowed only if NA-0516 command-surface evidence shows it is materially cleaner and no weaker.

Future flow:

1. Recheck retained remote qsc hash/path/owner/provenance.
2. Set up isolated local and remote qsc roots.
3. Create or rotate synthetic identities as needed.
4. Exchange synthetic public/trust material through proof-root and remote-test artifacts only.
5. Send synthetic plaintext `QSL_REMOTE_E2EE_SYNTHETIC_BUILD_TO_INSPIRON_<PROOF_ID>` from Build to Inspiron.
6. Receive/decrypt on Inspiron.
7. Send synthetic reply `QSL_REMOTE_E2EE_SYNTHETIC_INSPIRON_TO_BUILD_<PROOF_ID>` from Inspiron to Build.
8. Receive/decrypt on Build.
9. Run one negative/no-mutation boundary if feasible without broadening qsc source or host scope.
10. Cleanup transient message/artifact paths or document explicit retention and cleanup command.

No production data, personal data, private keys, passphrases, tokens, passwords, backup material, or credential material may be transferred or stored in checked-in evidence.

## Build-to-Inspiron E2EE options review

| Option | Disposition | Risk reduced | Evidence gap addressed | Feasibility | Scope risk | Remote mutation risk | Secret/key risk | Public-claim risk | Likely future allowed paths | Likely future forbidden paths | P0/P1/P2 risks |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1. Remote Build-to-Inspiron E2EE authorization now, implementation next | Select | Reduces uncertainty about real two-host qsc workflow | Direct send/receive/reply evidence gap | High, using retained qsc and observed qsc tests | Medium, controlled by NA-0516 stop rules | Medium, limited to qsl-remote-test E2EE root | Medium, reduced by synthetic roots and redaction | Low if claim boundary is explicit | NA-0516 evidence/testplan/decision/trace/journal and proof root | qsc source, workflow, dependency, corpus, formal, public docs, backup | P0 retained binary mismatch; P1 relay/tunnel ambiguity; P2 cleanup residue |
| 2. Identity/trust-only remote smoke first | Defer | Reduces identity setup uncertainty | Identity/trust exchange only | Medium | Low | Low | Low to medium | Low | Smaller NA-0516 identity/trust evidence | Message send/receive, qsc source, workflows | P0 over-splitting while host is available; P1 incomplete evidence; P2 later duplicate setup |
| 3. Remote qsc binary re-smoke / retained artifact hardening | Defer | Reduces retained artifact drift | Hash/smoke freshness | High | Low | Low | Low | Low | Hash/path/owner proof only | E2EE execution, broad mutation | P0 lost host window; P1 redundant with NA-0514; P2 cleanup delay |
| 4. Remote relay/transport diagnostic lane | Defer unless NA-0516 rechecks reveal transport blocker | Reduces relay/tunnel uncertainty | Transport reachability | Medium | Medium | Medium | Low | Low | Diagnostic evidence, bounded remote probe | qsc source, package install, public exposure | P0 transport false-negative; P1 tunnel complexity; P2 extra lane delay |
| 5. Operator toolchain setup | Reject | None now | No current gap; retained qsc exists | Not needed | Medium | Medium | Medium | Low | None selected | package install, remote source build | P0 unauthorized host mutation; P1 host time wasted; P2 provenance churn |
| 6. Remote E2EE implementation immediately in NA-0515 | Reject | Would reduce E2EE gap but violates lane | Full flow | Technically possible but unauthorized | High | High | High | High | None in NA-0515 | SSH, transfer, qsc send/receive, remote E2EE | P0 directive violation; P1 unreviewed output; P2 governance contradiction |
| 7. Same-host E2E negative expansion | Defer | Reduces local negative-case uncertainty | Local no-mutation evidence | High | Low | None | Low | Low | Future qsc tests only if separately authorized | Remote work, source mutation in NA-0515 | P0 misses remote host window; P1 less direct evidence; P2 duplicate local coverage |
| 8. CI/tooling lane | Reject unless current CI blocks | Reduces validation infrastructure risk | CI health only | High if needed | Medium | None | Low | Low | CI docs/evidence only | workflow mutation unless authorized | P0 delays remote evidence; P1 unrelated churn; P2 false confidence |

## Exact future command family

NA-0516 may run bounded local qsc commands on Build, bounded SSH commands to `inspiron`, bounded file-transfer commands for synthetic public/trust/message artifacts, and retained remote qsc commands at `$HOME/qsl-remote-test/bin/qsc`.

NA-0516 must start with boundary rechecks:

- local `ssh -G inspiron` safe-field parse.
- remote account is `qslcodex`.
- remote account is non-root.
- remote account is not in privileged groups.
- negative `sudo -n true` probe fails.
- no backup exposure.
- qwork absent remotely.
- qsl-backup absent remotely.
- retained qsc exists at `qsl-remote-test/bin/qsc`.
- retained qsc owner is `qslcodex`.
- retained qsc sha256 equals `6f12ab5eec2468a4146012dfc247cef15be2337cb8e8e99cefa96a8c258d91ea`.
- retained qsc `--help` or `--version` smoke succeeds.

Observed qsc command family from same-host tests:

- `vault init --non-interactive --key-source passphrase --passphrase-file <synthetic-passphrase-file>`
- `identity rotate --as <alice-or-bob> --confirm`
- `identity show --as <alice-or-bob>`
- `contacts add --label <peer> --fp <peer-fingerprint> --route-token <synthetic-route-token>`
- `contacts device list --label <peer>`
- `contacts device trust --label <peer> --device <device-id> --confirm`
- `relay inbox-set --token <synthetic-mailbox-token>`
- `relay serve --port <local-loopback-port> --seed <synthetic-seed> --drop-pct 0 --dup-pct 0 --reorder-window 0 --fixed-latency-ms 0 --jitter-ms 0 --max-messages <bounded-count>`
- `handshake init --as alice --peer bob --relay <relay-url>`
- `handshake poll --as bob --peer alice --relay <relay-url> --max <bounded-count>`
- `handshake poll --as alice --peer bob --relay <relay-url> --max <bounded-count>`
- `send --transport relay --relay <relay-url> --to <peer> --file <synthetic-plaintext-path>`
- `receive --transport relay --relay <relay-url> --mailbox <mailbox-token> --from <peer> --max 1 --out <output-dir>`

Because the qsc test relay binds loopback, NA-0516 may use an ephemeral loopback-only SSH reverse tunnel only if boundary checks pass and the command is explicitly bounded, for example `ssh -N -o BatchMode=yes -o PasswordAuthentication=no -o ConnectTimeout=10 -R 127.0.0.1:<remote-loopback-port>:127.0.0.1:<local-loopback-port> inspiron`. If relay reachability is ambiguous or would require public exposure, package installation, config mutation, or source mutation, NA-0516 must stop or select a relay/transport diagnostic successor.

Future transfers are limited to synthetic public/trust/message artifacts through bounded `scp` or SSH stdin transfer. No private keys, passphrases, tokens, passwords, backups, production data, or personal data may be transferred. `rsync` is not selected by default.

Selected negative boundary: wrong mailbox or wrong peer no-mutation case, matching the same-host test pattern. If no negative boundary can be run without broadening scope, NA-0516 must record the rationale and select a split successor.

Cleanup default: remove transient remote E2EE root under `$HOME/qsl-remote-test/e2ee/<PROOF_ID>` when safe. If retained for debugging, record reason, retained path, expected owner, and cleanup command. The retained qsc binary may remain unless a stop condition or cleanup decision says otherwise.

## Future proof / redaction rules

NA-0516 proof must include no private keys, passphrases, tokens, passwords, production endpoints, backup material, personal data, or production data. Synthetic messages only are allowed.

Raw command output may be retained under the local proof root. Checked-in evidence must summarize commands, markers, synthetic labels, hash/path checks, send/receive/reply outcomes, negative boundary outcome, and cleanup/retention status. Host/IP/home/path details must be redacted unless needed for exact safety proof. Secret-looking command output must cause a stop before checked-in evidence is updated.

## Future stop conditions

NA-0516 must stop if any of these occur:

- qwork proof is stale.
- retained qsc hash, owner, or path mismatch.
- remote account, privilege, backup, qwork, or qsl-backup boundary fails.
- local qsc build/selection is ambiguous.
- sudo/admin action is needed.
- package installation is needed.
- remote source checkout/build is needed.
- qsc output emits private key, passphrase, token, password, or secret material.
- production or personal data is involved.
- send/receive/reply fails.
- negative boundary mutates state unexpectedly.
- cleanup fails and cannot be safely documented.
- pressure appears to make a public-readiness, production-readiness, public-internet-readiness, crypto-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim.

## Hostile Cryptographer Review

The future Build-to-Inspiron E2EE flow would show only bounded workflow behavior: two qsc clients using synthetic identities and messages can complete one send/receive/reply sequence across Build and Inspiron with a retained remote binary. It does not establish crypto completeness, replay resistance, downgrade resistance, side-channel freedom, secret-material completeness, or vulnerability freedom.

Replay and downgrade resistance remain unproven after one remote E2EE flow unless specific replay/downgrade tests are separately authorized and executed. The selected negative case is a no-mutation guard, not a full replay or downgrade proof.

Retained remote qsc provenance must be rechecked by path, owner, sha256, source-commit linkage, and smoke before E2EE. Synthetic messages avoid secret exposure by using directive-generated labels and no production, personal, credential, backup, or private-key material.

## Red-Team Review

If the retained qsc binary is replaced before NA-0516, the hash/path/owner recheck must stop the lane. If the remote root contains stale state, NA-0516 must use a fresh `$HOME/qsl-remote-test/e2ee/<PROOF_ID>` root and verify isolation. If transfer selection would include private material, the lane must stop before transfer.

If qsc output leaks secret-looking material, raw output must remain in proof-root quarantine and checked-in evidence must not include it. If the negative boundary fails open, NA-0516 must record the failure, stop before broadening behavior, and avoid cleanup that hides evidence unless cleanup itself is safe and documented. Cleanup proof must include removed or retained paths, final owner/path status when retained, and cleanup command. If scheduled remote residual checks remain red after E2EE, they remain residual unless required by protection or public-safety policy; a follow-up diagnostic lane may be selected.

## Production SRE Review

Proceeding to one bounded remote E2EE implementation next is operationally reasonable because NA-0514 retained a smoked qsc binary and the remote host may be time-limited. NA-0516 should log command family, timestamps, hash checks, qsc provenance, synthetic labels, transfer inventory, negative boundary result, cleanup/retention status, and check-run evidence.

Checked-in evidence should redact host/IP/home/path details when not essential. Temporary debugging artifacts may be retained under the proof root and, only if necessary, under `$HOME/qsl-remote-test/e2ee/<PROOF_ID>` with an explicit cleanup command. Failures must remain isolated from qwork, qsl-backup, backup trees, production data, user data, and public services.

This is not production readiness or public readiness. It is one bounded remote workflow implementation target.

## Release-Claim Boundary Review

no public-readiness claim is made.
no production-readiness claim is made.
no public-internet-readiness claim is made.
no external-review-complete claim is made.
no crypto-complete claim is made.
no replay-proof claim is made.
no downgrade-proof claim is made.
no secret-material-complete claim is made.
no side-channel-free claim is made.
no vulnerability-free claim is made.
no bug-free claim is made.
no perfect-crypto claim is made.

## Best-Known-Method Review

Best-known method is to use the retained remote binary only after rechecking hash/path/owner/provenance, use current same-host qsc test patterns for command selection, isolate local and remote roots, use synthetic messages, capture raw output under proof root, redact checked-in summaries, run one negative/no-mutation boundary if feasible, and clean up or explicitly retain remote E2EE artifacts.

## Formal-Model Mapping Residual

NA-0516 can be mapped to formal-model expectations only as workflow evidence around setup, send, receive, reply, and negative no-mutation behavior. It does not close formal proof residuals around replay, downgrade, side channels, endpoint compromise, or production threat models.

## External-Review Readiness

NA-0516 should produce evidence that is easier for later external review to inspect: command family, artifact inventory, redaction policy, synthetic labels, hash checks, and stop decisions. no external-review-complete claim is made.

## Assurance Gap Review Trigger

If NA-0516 passes, the next successor should still explicitly decide whether to pursue remote negative expansion, remote cleanup/remediation, relay/transport diagnostics, or assurance documentation. If NA-0516 fails, the successor should target the first failing boundary without weakening fail-closed behavior.

## Prioritization matrix

| Candidate | Risk reduced | Directness of evidence | Speed while host is available | Security risk | Operator burden | Implementation feasibility | Scope risk | Remote mutation risk | Secret/key risk | Public-claim risk | Recommended disposition | Next-lane yes/no |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Build-to-Inspiron remote E2EE implementation next | Highest: real two-host send/receive/reply | Direct | Fast | Medium, bounded | Medium | High | Medium | Medium | Medium | Low with boundaries | Select | Yes |
| Identity/trust-only remote smoke first | Moderate | Partial | Fast | Low | Medium | Medium | Low | Low | Low | Low | Defer | No |
| Remote qsc binary re-smoke / retained artifact hardening | Low to moderate | Indirect | Fast | Low | Low | High | Low | Low | Low | Low | Defer | No |
| Remote relay/transport diagnostic lane | Moderate if transport blocks | Indirect | Medium | Medium | Medium | Medium | Medium | Medium | Low | Low | Conditional defer | No |
| Operator toolchain setup | Low | Indirect | Slow | Medium | High | Low need | Medium | High | Medium | Low | Reject | No |
| Same-host E2E negative expansion | Moderate local-only | Indirect for remote | Medium | Low | Low | High | Medium if source changes | None | Low | Low | Defer | No |
| CI/tooling lane | Low unless CI blocks | Indirect | Medium | Low | Low | High | Medium | None | Low | Low | Reject unless blocked | No |
| Remote cleanup/remediation lane | Moderate only if residue exists | Indirect | Fast | Low | Medium | High | Low | Medium | Low | Low | Defer until needed | No |

## Authorization decision

Primary classification selected: `REMOTE_BUILD_TO_INSPIRON_E2EE_IMPLEMENTATION_READY`.

Rationale: NA-0514/D407 staged and smoked the retained qsc binary on Inspiron, recorded hash/path/owner/provenance and cleanup evidence, and restored NA-0515 as the sole READY item. The highest-value next lane is therefore one bounded Build-to-Inspiron E2EE implementation with strict rechecks, synthetic data, redaction, negative/no-mutation boundary, and cleanup/retention rules.

## Selected NA-0516 successor

`NA-0516 -- QSL Build-to-Inspiron Remote qsc Client-to-Client E2EE Implementation Harness`

Status: READY

Goals: G1, G2, G3, G4, G5

## Future scope bundle

Objective: Execute one bounded Build-to-Inspiron qsc client-to-client E2EE test using local Build qsc capability and the retained Inspiron qsc binary, with synthetic messages, isolated local and remote roots, retained-binary hash recheck, send/receive and reply flow, one negative/no-mutation boundary if feasible, redacted proof capture, and cleanup/retention proof, without public or production readiness claims.

Allowed future NA-0516 scope:

- `docs/governance/evidence/NA-0516_qsl_build_to_inspiron_remote_qsc_client_to_client_e2ee_implementation_harness.md`
- `tests/NA-0516_qsl_build_to_inspiron_remote_qsc_client_to_client_e2ee_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- proof-root-local capture of local/remote E2EE command output.
- bounded SSH commands to `inspiron` as `qslcodex`.
- bounded transfer of synthetic public/trust/message artifacts only.
- retained remote qsc binary at `$HOME/qsl-remote-test/bin/qsc`.
- local Build qsc binary from current clean checkout or proof-root build.
- synthetic messages only.
- remote artifacts only under `$HOME/qsl-remote-test/e2ee/<PROOF_ID>`.
- cleanup or documented retention under qsl-remote-test only.
- ephemeral loopback-only relay tunnel only if explicitly bounded and required.

Forbidden future NA-0516 scope:

- package installation.
- sudo/admin action other than negative `sudo -n true` probe.
- key generation or installation outside qsc test-generated synthetic identities.
- SSH config mutation.
- known_hosts mutation.
- remote host mutation outside qsl-remote-test E2EE root.
- qwork/qstart/qresume mutation.
- qsl-backup execution.
- qsc source/test/fuzz/Cargo mutation.
- workflow/dependency mutation.
- corpus/vector/input mutation.
- formal/refimpl/service/public/backup mutation.
- production/user data.
- no public-readiness claim is made.
- no production-readiness claim is made.
- remote source checkout/build.

Deliverables: remote E2EE implementation evidence, testplan, decision, TRACEABILITY update, rolling journal update, and selected successor for remote negative expansion, remote cleanup, relay/transport diagnostic, or assurance follow-up.

Acceptance criteria: fresh qwork proof, retained remote qsc hash/path/owner rechecked, local qsc provenance recorded, Build-to-Inspiron synthetic send/receive succeeds, Inspiron-to-Build synthetic reply succeeds, one negative/no-mutation boundary passes or a justified split successor is selected, no secret/private material in proof, cleanup/retention decision recorded, no public/production readiness claim, and exactly one READY item remains after closeout.

## Future validation / marker plan

Future NA-0516 markers:

- `NA0516_REMOTE_E2EE_SCOPE_CONSUMED_OK`
- `NA0516_RETAINED_REMOTE_QSC_HASH_RECHECKED_OK`
- `NA0516_LOCAL_QSC_PROVENANCE_RECORDED_OK`
- `NA0516_RELAY_TRANSPORT_BOUNDARY_OK`
- `NA0516_BUILD_TO_INSPIRON_SEND_RECEIVE_OK`
- `NA0516_INSPIRON_TO_BUILD_REPLY_OK`
- `NA0516_REMOTE_E2EE_SYNTHETIC_MESSAGES_ONLY_OK`
- `NA0516_REMOTE_E2EE_NO_SECRET_OUTPUT_OK`
- `NA0516_REMOTE_E2EE_NEGATIVE_BOUNDARY_OK`
- `NA0516_REMOTE_E2EE_CLEANUP_OR_RETENTION_OK`
- `NA0516_NO_PACKAGE_INSTALL_OK`
- `NA0516_NO_REMOTE_SOURCE_BUILD_OK`
- `NA0516_NO_QWORK_QSLBACKUP_OK`
- `NA0516_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0516_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0516_ONE_READY_INVARIANT_OK`

## No remote action in NA-0515

Codex performs no remote action in NA-0515. No SSH, scp, sftp, rsync, qsc send/receive, remote E2EE, binary transfer, package installation, sudo/admin action, remote source checkout/build, SSH key generation, host-key scan, SSH config mutation, known_hosts mutation, or remote host mutation is authorized in this lane.

## No remote E2EE in NA-0515

NA-0515 authorizes future scope only. The first remote E2EE implementation remains NA-0516 or a later selected successor.

## Public claim / website / external review boundary

NA-0515 changes no public documents, website content, README, or START_HERE material. no public-readiness claim is made. no production-readiness claim is made. no public-internet-readiness claim is made. no external-review-complete claim is made.

## Backup-impact statement

Backup impact: none. NA-0515 does not run qsl-backup, does not mutate backup scripts or plans, does not inspect backup private material, and does not write under `/backup/qsl`.

## Rejected alternatives

Rejected alternatives are remote E2EE execution in NA-0515, operator toolchain setup, CI/tooling work absent a blocker, package installation, remote source checkout/build, public exposure of relay services, and any path that requires qsc source/test/fuzz/Cargo mutation before the remote E2EE implementation scope is executed.

## Next recommendation

Proceed to NA-0516 as the sole READY successor after NA-0515 closeout: one bounded Build-to-Inspiron remote qsc client-to-client E2EE implementation harness using retained Inspiron qsc, local Build qsc, synthetic messages, isolated roots, strict boundary rechecks, redacted proof, negative/no-mutation boundary, and cleanup/retention proof.
