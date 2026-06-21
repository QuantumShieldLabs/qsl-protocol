Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-21

# NA-0513 Remote qsc Staging Strategy Authorization Plan

## Executive summary

NA-0513 is authorization-only. Codex did not run SSH, remote commands, scp,
sftp, rsync, binary transfer, remote E2E, package installation, key generation,
SSH config mutation, known_hosts mutation, sudo/admin action, qwork, qstart,
qresume, qsl-backup, or remote qsc.

Packet A triaged the D404 closeout red remote checks on merge commit
`72994b2882e7`. `remote-handshake`, `remote-relay`, and
`relay-ui-integration` were completed failures, but they were scheduled
non-required workflows, were not branch-protection contexts, were not inputs to
the green `public-safety` aggregate, and matched existing remote scenario /
staging residuals. They are not ignored, passed, or treated as release evidence.

Primary classification:

- `REMOTE_PREBUILT_QSC_BINARY_STAGING_SMOKE_READY`

Selected successor:

- `NA-0514 -- QSL Remote qsc Prebuilt Binary Staging and Smoke Implementation Harness`

The next lane may stage a locally built qsc binary under
`$HOME/qsl-remote-test/bin/qsc`, verify its hash, run a non-protocol
`qsc --help` or `qsc --version` smoke, and preferably retain the binary only if
retention proof is satisfied. Remote E2E remains deferred.

## Live NA-0513 scope

Allowed mutation paths for this evidence PR:

- `docs/governance/evidence/NA-0513_qsl_remote_qsc_staging_strategy_authorization_plan.md`
- `tests/NA-0513_qsl_remote_qsc_staging_strategy_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden in this lane:

- remote SSH execution.
- scp, sftp, rsync, or binary transfer.
- remote command execution.
- remote E2E.
- remote qsc send/receive.
- remote source checkout/build.
- package installation.
- sudo/admin action.
- key generation or installation.
- local SSH config or known_hosts mutation.
- remote host mutation.
- qwork/qstart/qresume mutation or execution.
- qsl-backup execution or mutation.
- qsc source/test/fuzz/Cargo mutation.
- workflow/script/helper/dependency mutation.
- corpus/vector/input mutation.
- formal/refimpl/service/public/backup mutation.
- no public-readiness claim is made.
- no production-readiness claim is made.

## qwork proof-file verification

Codex read these proof files and did not run qwork, qstart, or qresume:

- `/srv/qbuild/work/NA-0513/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0513/.qwork/startup.qsl-protocol.json`

Required qwork fields passed:

- `startup_result=OK`
- `lane=NA-0513`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0513/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0513`
- `requested_lane_status=READY`

Freshness passed before fetch:

- proof HEAD matched live HEAD at `72994b2882e7`.
- proof origin/main matched live origin/main at `72994b2882e7`.
- `.json` proof mirrored `.kv` for required fields.

Disk proof before fetch:

- `/` usage was 82%, below the 95% stop threshold.
- `/backup/qsl` usage was checked read-only.

qsl-backup boundary proof:

- installed helper digest matched `e9ecff3d22ed`.
- Codex ops source inclusion count was exactly one.
- Codex did not run backup, restore, or qsl-backup.

Recovered startup parser issue:

- failing command: initial queue parser in the startup proof script.
- classification: recoverable command-shape/parser issue.
- cause: parser expected ASCII hyphen separators while queue headings use a
  different separator glyph.
- corrective action: reran line-position heading parsing independent of
  separator glyph.
- final result: READY_COUNT 1; READY NA-0513; NA-0510, NA-0511, and NA-0512
  DONE; D-1013 once; D-1014 once; D-1015 absent; duplicate decision count 0.

## D404 red check triage

Target commit:

- `72994b2882e7`

Packet A collected:

- REST check-runs for the target commit.
- PR #1297 metadata and changed files.
- branch-protection required status checks.
- workflow run/job metadata and logs for `remote-handshake`, `remote-relay`,
  and `relay-ui-integration`.
- workflow names and event types.

Branch protection required contexts were:

- `ci-4a`
- `ci-4b`
- `ci-4c`
- `ci-4d`
- `ci-4d-dur`
- `demo-cli-build`
- `demo-cli-smoke`
- `formal-scka-model`
- `goal-lint`
- `metadata-conformance-smoke`
- `suite2-vectors`
- `CodeQL`
- `macos-qsc-qshield-build`
- `public-safety`

The three red checks are not required branch-protection contexts.

PR #1297 changed only closeout governance/testplan paths:

- `DECISIONS.md`
- `NEXT_ACTIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0512_closeout_restore_na0513_testplan.md`

Classification table:

| Check | Workflow | Event | Evidence | Classification |
|---|---|---|---|---|
| `remote-handshake` | `remote-handshake-tests` | `schedule` | non-required scheduled remote scenario smoke failed at vault initialization | `REMOTE_CHECK_NONREQUIRED_REMOTE_RESIDUAL` |
| `remote-relay` | `remote-relay-tests` | `schedule` | non-required scheduled relay scenario smoke failed with local qsc contact-store state invalid for the expected happy path | `REMOTE_CHECK_NONREQUIRED_REMOTE_RESIDUAL` |
| `relay-ui-integration` | `relay-ui-integration` | `schedule` | non-required scheduled relay UI integration readiness check returned HTTP 404 before ignored integration tests | `REMOTE_CHECK_NONREQUIRED_REMOTE_RESIDUAL` |

Continuation criteria:

- not required branch-protection contexts: satisfied.
- not part of the public-safety aggregate: satisfied.
- not required by NA-0512 closeout acceptance: satisfied.
- logs or metadata show residual remote scenario/staging gaps: satisfied.
- no evidence of qsc source regression from PR #1297: satisfied because PR
  #1297 changed only closeout governance/testplan paths.
- no evidence of unsafe remote host mutation by Codex: satisfied.
- no evidence of backup/qwork/qsl-backup exposure: satisfied.
- no public/production readiness claim drift: satisfied.

No rerun was selected because the failures were not classified as infrastructure
transient. They remain residual red remote scenario checks and must not be
treated as passing.

## D404 closeout public-safety gate interpretation

For merge commit `72994b2882e7`, `public-safety` completed success. The
`public-safety` job completed before the three scheduled remote checks and its
workflow is separate from the scheduled remote workflows. `public-safety` is a
required branch-protection context; the three red remote checks are not.

Interpretation:

- D404 closeout public-safety was green.
- The red scheduled remote checks are residual evidence, not a public-safety
  input and not a branch-protection blocker.
- NA-0513 may proceed only because Packet A explicitly classified them.

## NA-0512 / D404 inheritance

NA-0512 completed and D404 restored NA-0513 as READY.

Inherited facts:

- classification `REMOTE_MARKER_PROBE_PASS_TOOLCHAIN_ABSENT`.
- exactly one bounded SSH command occurred in NA-0512.
- marker content was synthetic.
- marker write/read/delete passed.
- marker deletion was verified by absent-after-delete proof.
- `git` was absent remotely.
- `cargo` was absent remotely.
- `rustc` was absent remotely.
- `qsc` was absent remotely.
- toolchain and qsc absence are evidence for staging strategy selection.
- no remote E2E was run.
- no qsc send/receive was run remotely.
- no remote source checkout/build was run.
- no package installation was run.
- no backup exposure was accepted.
- qwork and qsl-backup were absent remotely.
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

D404 final state:

- implementation PR #1296 merged at `d20e700fcb2c`.
- closeout PR #1297 merged at `72994b2882e7`.
- final queue: READY_COUNT 1; READY NA-0513.
- D404 red remote checks were triaged under Packet A in this lane.

## Staging options review

| Option | Disposition | Risk reduced | Evidence gap addressed | Feasibility | Scope risk | Remote mutation risk | Secret/key risk | Public-claim risk | Future allowed paths | Future forbidden paths | P0/P1/P2 risks |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Local prebuilt qsc binary transfer + remote smoke, retain binary | Select | fastest path to executable qsc on remote host | binary provenance, hash match, non-protocol qsc execution | high | medium | medium, one artifact under qsl-remote-test | low if no keys/secrets | medium if overstated | NA-0514 evidence/testplan/decision/traceability/journal; proof-root logs | remote E2E, qsc send/receive, package install, remote build | P0 hash/smoke mismatch; P1 stale retained binary; P2 cleanup tracking |
| Local prebuilt qsc binary transfer + remote smoke, cleanup after smoke | Defer as fallback | reduces residue | same as retain minus future availability | high | medium | lower after cleanup | low | low/medium | same NA-0514 paths plus cleanup proof | retention without proof | P0 cleanup failure; P1 host window lost; P2 repeated transfer |
| Operator-managed remote toolchain setup | Defer | may enable remote builds later | git/cargo/rustc availability | medium, operator-dependent | medium/high | high outside Codex | medium | medium | future operator authorization docs | Codex package install/setup | P0 unauthorized system change; P1 delay; P2 toolchain drift |
| Remote source checkout/build | Reject now | would prove remote build if ready | source/build reproducibility | low because git/cargo/rustc absent | high | high | medium | high | none now | remote git clone/fetch/pull, cargo build/test/update | P0 package/toolchain need; P1 broad mutation; P2 noisy failures |
| Remote installed qsc binary smoke | Reject now | would prove existing qsc execution | installed binary presence | not feasible because qsc absent | low | low | low | medium if stale | none now | trusting unknown installed binary | P0 qsc absent; P1 stale binary if appears; P2 provenance gap |
| Remote E2E authorization now | Defer | would test protocol flow | full remote behavior | not feasible before qsc staging | very high | high | medium/high | high | later E2E authorization only | qsc send/receive now | P0 premature protocol claim; P1 state leakage; P2 test flake |
| Same-host E2E negative expansion | Defer | improves local coverage | local negative behavior | high | low | none | low | low | later qsc test lane | remote staging displacement | P0 host window lost; P1 less direct remote proof; P2 scope churn |
| Remote setup remediation | Reject unless safety gap appears | addresses boundary weakness | setup/account/toolchain uncertainty | not selected; Packet A and NA-0512 show no boundary weakness | medium | medium/high | medium | medium | future remediation if needed | remediation without blocker | P0 unnecessary host mutation; P1 setup drift; P2 delay |
| CI/tooling lane | Reject for now | could repair scheduled red checks | process health | medium | medium/high | none | low | medium | later process lane if required checks change | workflow mutation in NA-0513 | P0 distracts from staging; P1 masks residuals; P2 broad workflow churn |

## Recommended staging strategy

Recommended classification:

- `REMOTE_PREBUILT_QSC_BINARY_STAGING_SMOKE_READY`

Rationale:

NA-0512 proved the remote account/workdir boundary and marker cleanup, but it
also proved `git`, `cargo`, `rustc`, and `qsc` are absent remotely. A remote
source build would first require toolchain or package setup. A locally built
binary copied into the dedicated test directory is the fastest bounded path to
remote qsc execution evidence while the host is available.

The future lane must prove only staging and non-protocol smoke:

- local source commit recorded.
- local qsc binary path recorded.
- local qsc binary sha256 recorded.
- local qsc binary size recorded.
- one bounded transfer to `$HOME/qsl-remote-test/bin/qsc`.
- remote hash match.
- remote `qsc --help` or `qsc --version` smoke.
- no remote E2E.
- no qsc send/receive.
- no public-readiness claim is made.
- no production-readiness claim is made.

Retention policy:

- prefer retaining the staged binary under `$HOME/qsl-remote-test/bin/qsc` if
  hash and smoke pass, because remote E2E is a near-term follow-on and the host
  may be time-limited.
- retention must record owner `qslcodex`, path under qsl-remote-test only, hash,
  cleanup command, no production data, and no backup exposure.
- if any retention proof fails, future NA-0514 must select cleanup-after-smoke
  instead.

## Exact future command family

NA-0513 does not run these commands. They are future NA-0514 command families.

Local commands:

```bash
cargo build -p qsc --locked
sha256sum target/debug/qsc
stat -c '%s %n' target/debug/qsc
target/debug/qsc --help
```

Future NA-0514 may use a different unambiguous local qsc binary path if the
build output path differs, but it must record the path, source commit, sha256,
and size.

Remote directory creation if needed:

```bash
ssh -o BatchMode=yes -o PasswordAuthentication=no -o ConnectTimeout=10 inspiron 'mkdir -p "$HOME/qsl-remote-test/bin"'
```

Transfer command, exactly one bounded transfer:

```bash
scp -o BatchMode=yes -o PasswordAuthentication=no -o ConnectTimeout=10 target/debug/qsc inspiron:~/qsl-remote-test/bin/qsc
```

Remote smoke command family:

```bash
ssh -o BatchMode=yes -o PasswordAuthentication=no -o ConnectTimeout=10 inspiron 'bash -s'
```

The future remote script may:

- verify `id -un` is `qslcodex`.
- verify non-root and no sudo/admin boundary.
- verify no backup/qwork/qsl-backup exposure.
- verify `$HOME/qsl-remote-test/bin/qsc` exists.
- verify remote hash equals local hash.
- run `$HOME/qsl-remote-test/bin/qsc --help` or `--version`.
- emit fixed markers.

Forbidden future NA-0514 command families:

- remote cargo build/test/update.
- remote git clone/fetch/pull.
- remote rustup install/update.
- remote package manager.
- sudo/admin action except an explicitly authorized negative no-sudo probe.
- qwork/qstart/qresume.
- qsl-backup.
- qsc send/receive.
- remote E2E.
- account/key/SSH config mutation.
- writes outside `$HOME/qsl-remote-test`.

## Future redaction / stop rules

Future NA-0514 proof must:

- include no private key material.
- include no passphrase material.
- include no token material.
- include no password material.
- include no production endpoints.
- include no backup material.
- redact host/IP/disk topology in checked-in evidence.
- keep raw transfer/smoke logs under the proof root.
- record binary hash, not binary content.
- record source commit.
- record staged path, with qsl-remote-test-relative wording acceptable in final
  checked-in evidence.

Future NA-0514 must stop if:

- local build fails.
- qsc binary cannot be identified unambiguously.
- local qsc smoke fails.
- transfer fails.
- remote directory creation needs sudo or a path outside qsl-remote-test.
- remote hash mismatches local hash.
- remote qsc smoke fails.
- qsc smoke output includes secret-looking material.
- backup exposure appears.
- qwork/qsl-backup appears remotely.
- remote sudo succeeds.
- remote E2E is needed.
- package installation is needed.
- source checkout/build is needed remotely.
- public/production readiness claim pressure appears.

## Hostile Cryptographer Review

Does prebuilt binary staging reduce risk compared with remote toolchain
installation?

- Yes. It avoids remote package installation, remote rustup, remote git, remote
  source checkout, and remote dependency resolution. It narrows remote mutation
  to one artifact under the dedicated test directory.

What provenance weakness remains when copying a local binary?

- The remote host trusts the local build artifact. The future lane must record
  source commit, local path, binary hash, size, and remote hash match, but this
  remains provenance evidence for that artifact only.

Does qsc smoke prove only binary execution, not protocol correctness?

- Yes. `--help` or `--version` proves only that the staged binary executes and
  emits bounded non-protocol output. It proves no qsc send/receive, no remote
  E2E, no cryptographic completeness, no replay-proof property, and no
  no downgrade-proof property is claimed.

What could a malicious or stale remote binary/output fake?

- It could fake help/version output or preserve a stale artifact after staging.
  Hash verification against the freshly staged local binary and ownership/path
  proof reduce but do not eliminate that risk. Later E2E must reverify hash
  before use.

Why remote E2E remains deferred until staging/smoke passes?

- Without a proven qsc binary on the remote host, E2E would mix staging failure,
  network failure, protocol state setup, and qsc behavior. Smoke isolates the
  first dependency.

What claim boundaries remain mandatory?

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

## Red-Team Review

What if red remote checks indicate something beyond missing staging?

- Then future work must stop and remediate. Packet A found them non-required,
  scheduled, outside public-safety, and residual; this classification must be
  revisited if a required or repo-defect signal appears.

What if transfer writes outside qsl-remote-test?

- Stop. Future NA-0514 must constrain creation and transfer paths to
  `$HOME/qsl-remote-test/bin/qsc`.

What if staged binary is replaced later?

- Later lanes must recheck hash before use and treat mismatch as a stop.

What if hash mismatch occurs?

- Stop. Do not run smoke or E2E.

What if qsc smoke writes state unexpectedly?

- Stop and classify the smoke as unsafe. NA-0514 should prefer `--help` or
  `--version` only.

What if remote lacks exec permission or is mounted noexec?

- Stop. Do not remediate by package install, chmod outside staged artifact, or
  mount/admin changes unless a separate directive authorizes it.

What if disk is too low?

- Stop before transfer or retention. Do not force staging.

What cleanup/retention proof is needed before E2E?

- If retained: path, owner, hash, size, cleanup command, no backup exposure, and
  no production data. If cleaned: removal proof and absent-after-cleanup proof.

## Production SRE Review

Why prebuilt-binary staging is the fastest safe next step while host is
available:

- It avoids remote package/toolchain setup and narrows the action to one
  artifact plus one non-protocol smoke. It also preserves qbuild/qwork
  isolation by not running qwork or qsl-backup remotely.

Whether staged binary should be retained for near-term E2E:

- Prefer retention only after hash/smoke/path/owner/no-backup proof passes. The
  staged binary should be considered a test artifact, not a system install.

How retention should be documented and cleaned up later:

- Record qsl-remote-test path, owner, sha256, size, source commit, cleanup
  command, and the follow-on lane that either uses or deletes it.

How staging failures remain isolated from local qbuild/qwork:

- All raw logs stay in the proof root. Remote commands do not run qwork,
  qstart, qresume, qsl-backup, source checkout/build, or package install.

Why package/toolchain installation is deferred:

- It is broader, slower, and higher-risk than staging a single local binary.

## Release-Claim Boundary Review

NA-0513 and future NA-0514 must preserve:

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

| Candidate | Risk reduced | Directness of evidence | Speed while host is available | Security risk | Operator burden | Implementation feasibility | Scope risk | Remote mutation risk | Secret/key risk | Public-claim risk | Recommended disposition | Next-lane yes/no |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Local prebuilt qsc binary transfer + remote smoke, retain binary | high | high for staging | fastest | medium | low | high | medium | medium | low | medium if overstated | select | yes |
| Local prebuilt qsc binary transfer + remote smoke, cleanup after smoke | medium/high | high for staging | fast but less reusable | low/medium | low | high | medium | low after cleanup | low | low/medium | fallback | no |
| Operator-managed remote toolchain setup | medium | indirect until verified | slower | medium/high | high | medium | medium/high | high | medium | medium | defer | no |
| Remote source checkout/build | medium/high | high if available | slow | high | medium | low now | high | high | medium | high | reject now | no |
| Remote installed qsc binary smoke | medium | medium if binary existed | fast | medium from provenance | low | not feasible | low | low | low | medium | reject now | no |
| Remote E2E authorization now | high if ready | high but premature | low readiness | high | medium | low now | high | high | medium/high | high | defer | no |
| Same-host E2E negative expansion | medium | low for remote staging | medium | low | low | high | low | none | low | low | defer | no |
| Remote setup remediation | medium if boundary weak | medium | medium | medium/high | high | not needed | medium/high | high | medium | medium | reject unless blocker appears | no |
| CI/tooling lane | medium for process | indirect | medium | low/medium | medium | medium | medium/high | none | low | medium | reject unless remote checks become blockers | no |

## Authorization decision

Primary classification:

- `REMOTE_PREBUILT_QSC_BINARY_STAGING_SMOKE_READY`

Required decision points completed:

- Packet A red check triage completed.
- NA-0512/D404 consumed.
- D404 closeout public-safety success verified.
- completed red remote checks classified with evidence.
- marker probe passed.
- toolchain/qsc absent remotely.
- staging options reviewed.
- prebuilt qsc binary staging/smoke selected.
- exact future command family selected.
- retention/cleanup policy selected.
- redaction/stop rules selected.
- selected NA-0514 successor.
- no remote action in this directive.
- no SSH execution in this directive.
- no binary transfer in this directive.
- no package install.
- no remote source checkout/build.
- no remote E2E.
- no qsc source/test/fuzz/Cargo mutation.
- no workflow/script/helper mutation.
- no corpus/vector/input mutation.
- no dependency/lockfile mutation.
- no formal/refimpl/service/public/backup mutation.
- no public-readiness claim is made.
- no production-readiness claim is made.
- exactly one READY remains mandatory.

## Selected NA-0514 successor

### NA-0514 -- QSL Remote qsc Prebuilt Binary Staging and Smoke Implementation Harness

Status: READY

Goals: G1, G2, G3, G4, G5

Objective:

Stage a locally built qsc binary from the current qsl-protocol checkout to the
approved `inspiron` / `qslcodex` remote test account under
`$HOME/qsl-remote-test/bin/qsc`, verify provenance and sha256, run a bounded
non-protocol qsc smoke command, and decide whether the binary is safely retained
for near-term remote E2E, without package installation, remote source
checkout/build, qsc send/receive, remote E2E, sudo/admin action, backup
exposure, or public/production readiness claims.

Allowed scope:

- `docs/governance/evidence/NA-0514_qsl_remote_qsc_prebuilt_binary_staging_smoke_implementation_harness.md`
- `tests/NA-0514_qsl_remote_qsc_prebuilt_binary_staging_smoke_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- proof-root-local capture of local build/smoke, transfer, remote hash, and
  remote smoke output.
- local qsc binary build from clean current checkout, if needed.
- one bounded remote transfer to `$HOME/qsl-remote-test/bin/qsc`.
- bounded remote directory creation only under `$HOME/qsl-remote-test/bin` if
  needed.
- bounded remote qsc `--help` or `--version` smoke only.
- retain staged binary under qsl-remote-test if hash/provenance/smoke pass and
  cleanup plan is documented.

Forbidden scope:

- remote E2E.
- qsc send/receive remotely.
- remote source checkout/build.
- package installation.
- sudo/admin action other than a negative no-sudo probe if explicitly included.
- key generation or installation.
- SSH config mutation.
- known_hosts mutation.
- remote host mutation outside staged qsc artifact.
- qwork/qstart/qresume mutation.
- qsl-backup execution.
- qsc source/test/fuzz/Cargo mutation.
- workflow/dependency mutation.
- corpus/vector/input mutation.
- formal/refimpl/service/public/backup mutation.
- no public-readiness claim and no production-readiness claim.
- writes outside `$HOME/qsl-remote-test`.

Deliverables:

- staging/smoke implementation evidence.
- testplan.
- decision.
- TRACEABILITY update.
- rolling journal update.
- selected future remote E2E authorization scope, remote smoke expansion, or
  remediation/no-action rationale.

Acceptance criteria:

- qwork proof fresh.
- local source commit recorded.
- local qsc binary hash recorded.
- remote staged binary hash matches local hash.
- remote qsc smoke passes.
- no remote E2E.
- no package installation.
- no source checkout/build on remote.
- no key material included.
- cleanup/retention plan recorded.
- exactly one READY item remains after closeout.

## Future scope bundle

Future NA-0514 must be staging/smoke only. It may use a locally built qsc
binary as the test artifact. It must not create identities, send or receive
messages, initialize protocol state beyond non-mutating help/version behavior,
or claim public/production readiness.

Future NA-0514 must record:

- source commit.
- local binary path.
- local binary sha256.
- local binary size.
- remote staged path.
- remote staged sha256.
- smoke command.
- retention or cleanup decision.
- cleanup command if retained.
- no remote E2E proof.
- no package install proof.
- no remote source build proof.

## Future validation / marker plan

Future NA-0514 markers:

- `NA0514_REMOTE_STAGING_SCOPE_CONSUMED_OK`
- `NA0514_LOCAL_QSC_BINARY_BUILT_OR_SELECTED_OK`
- `NA0514_LOCAL_QSC_BINARY_HASH_RECORDED_OK`
- `NA0514_REMOTE_QSC_BINARY_STAGED_OK`
- `NA0514_REMOTE_QSC_BINARY_HASH_MATCH_OK`
- `NA0514_REMOTE_QSC_SMOKE_OK`
- `NA0514_REMOTE_QSC_RETENTION_DECISION_OK`
- `NA0514_NO_REMOTE_E2E_OK`
- `NA0514_NO_REMOTE_SOURCE_BUILD_OK`
- `NA0514_NO_PACKAGE_INSTALL_OK`
- `NA0514_NO_SUDO_ADMIN_OK`
- `NA0514_NO_BACKUP_EXPOSURE_OK`
- `NA0514_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0514_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0514_ONE_READY_INVARIANT_OK`

NA-0513 markers:

- `NA0513_QWORK_PROOF_FILES_VERIFIED_OK`
- `NA0513_D404_RED_CHECK_TRIAGE_COMPLETED_OK`
- `NA0513_D404_PUBLIC_SAFETY_GREEN_OK`
- `NA0513_D404_RED_REMOTE_CHECKS_NONBLOCKING_RESIDUAL_OK`
- `NA0513_NA0512_D404_INHERITANCE_CONSUMED_OK`
- `NA0513_STAGING_OPTIONS_REVIEWED_OK`
- `NA0513_EXACT_FUTURE_COMMAND_FAMILY_SELECTED_OK`
- `NA0513_REDACTION_STOP_RULES_SELECTED_OK`
- `NA0513_STEWARDSHIP_REVIEWS_COMPLETED_OK`
- `NA0513_PRIORITY_MATRIX_COMPLETED_OK`
- `NA0513_REMOTE_PREBUILT_QSC_BINARY_STAGING_SMOKE_READY`
- `NA0513_SELECTED_NA0514_SUCCESSOR_OK`
- `NA0513_NO_REMOTE_ACTION_BY_CODEX_OK`
- `NA0513_NO_SSH_EXECUTION_BY_CODEX_OK`
- `NA0513_NO_BINARY_TRANSFER_OK`
- `NA0513_NO_REMOTE_E2E_OK`
- `NA0513_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0513_NO_WORKFLOW_DEPENDENCY_MUTATION_OK`
- `NA0513_NO_CORPUS_VECTOR_INPUT_MUTATION_OK`
- `NA0513_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0513_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0513_ONE_READY_INVARIANT_OK`

## No remote action in NA-0513

Codex did not run SSH, scp, sftp, rsync, remote command, remote E2E, remote
qsc, ssh-keygen, ssh-keyscan, sudo/admin action, package installation, qwork,
qstart, qresume, or qsl-backup in NA-0513.

## No remote E2E boundary

NA-0513 authorizes only a future staging/smoke lane. Future NA-0514 is not a
remote E2E lane. It must not run qsc send/receive or create protocol state.

## Public claim / website / external review boundary

NA-0513 changes no website, README, START_HERE, public technical paper, public
docs, or external review status. no public-readiness claim is made. no
production-readiness claim is made. no public-internet-readiness claim is made.
no external-review-complete claim is made. no crypto-complete claim is made. no
replay-proof claim is made. no downgrade-proof claim is made. no
secret-material-complete claim is made. no side-channel-free claim is made. no
vulnerability-free claim is made. no bug-free claim is made. no perfect-crypto
claim is made.

## Backup-impact statement

Backup impact is none. NA-0513 reads the installed qsl-backup helper only for
digest/source-list boundary proof. Codex did not run backup, restore, qsl-backup,
or mutate `/backup/qsl`, backup status, backup plans, or backup material.

Future NA-0514 must stop if backup exposure appears remotely and must not copy
qsc into backup paths.

## Rejected alternatives

- Remote source checkout/build is rejected now because remote git/cargo/rustc
  were absent and source checkout/build remains forbidden for the selected next
  lane.
- Remote installed qsc smoke is rejected now because qsc was absent remotely.
- Remote E2E authorization now is deferred because staging/smoke has not passed.
- Operator-managed toolchain setup is deferred because it is broader and slower
  than one staged binary smoke.
- CI/tooling repair is rejected for this lane because the red remote checks are
  non-required residuals, not branch-protection or public-safety blockers.

## Next recommendation

Proceed with NA-0513 evidence PR. After merge and a green public-safety attach,
perform closeout to restore the selected NA-0514 successor. NA-0514 should stage
and smoke a locally built qsc binary only, with remote E2E still deferred.
