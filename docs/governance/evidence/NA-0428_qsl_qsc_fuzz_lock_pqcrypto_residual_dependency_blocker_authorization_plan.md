Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0428 QSL qsc Fuzz Lock pqcrypto Residual Dependency Blocker Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0428 confirms that the pqcrypto residual is confined to the separate
`qsl/qsl-client/qsc/fuzz` cargo-fuzz workspace lock context and is not present
in the root workspace locked graph. Root `cargo audit --deny warnings` remains
green, root `rustls-webpki` remains `v0.103.13`, root `ml-kem` remains active,
and root `pqcrypto-mlkem`, `pqcrypto-traits`, and `pqcrypto-internals` package
IDs remain absent.

The nested fuzz lock remains an active blocker because `qsl/qsl-client/qsc/fuzz`
is a committed cargo-fuzz workspace, `scripts/ci/qsc_adversarial.sh` invokes its
targets, `.github/workflows/qsc-adversarial.yml` wires that smoke path into CI
for non-docs scopes, and `cargo audit --deny warnings --file
qsl/qsl-client/qsc/fuzz/Cargo.lock` reports denied advisories against the
separate lock.

A temporary proof-root simulation refreshed only the nested fuzz lock in an
untracked copy. That simulation removed all pqcrypto packages, raised
`rustls-webpki` to `0.103.13`, and produced a green nested lock audit. Because
the current fuzz `Cargo.toml` does not directly pin pqcrypto packages and no
workflow, runtime, crypto, harness, test, vector, or root dependency change was
needed in the simulation, NA-0428 authorizes the next lane as lockfile-only:

`FUZZ_LOCK_REMEDIATION_AUTHORIZED_LOCKFILE_ONLY`

Selected successor:

`NA-0429 -- QSL qsc Fuzz Lock pqcrypto Residual Lockfile Cleanup Implementation Harness`

NA-0428 does not remediate the lock and does not mutate any Cargo manifest,
lockfile, dependency, workflow, fuzz target, test, vector, runtime, crypto,
service, public, qwork, backup, or qsl-backup path.

## Live NA-0428 scope

Live `NEXT_ACTIONS.md` lists:

`NA-0428 -- QSL qsc Fuzz Lock pqcrypto Residual Dependency Blocker Authorization Plan`

Status: READY.

Goals: G1, G2, G3, G4, G5.

Allowed mutation paths for the NA-0428 evidence PR:

- `docs/governance/evidence/NA-0428_qsl_qsc_fuzz_lock_pqcrypto_residual_dependency_blocker_authorization_plan.md`
- `tests/NA-0428_qsl_qsc_fuzz_lock_pqcrypto_residual_dependency_blocker_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Allowed read-only inspection covered qwork proof files, the live queue,
DECISIONS, TRACEABILITY, rolling journal, NA-0426/NA-0427 evidence and
testplans, the stewardship canon, root and qsc Cargo manifests and locks, the
separate qsc fuzz workspace, qsc adversarial script/workflow linkage, root
dependency metadata, formal roots, and input/vector roots.

Forbidden mutation scope:

- no runtime code mutation;
- no crypto implementation mutation;
- no dependency, Cargo manifest, or lockfile mutation;
- no workflow mutation;
- no fuzz target mutation;
- no tests or vectors mutation outside this governance testplan;
- no qsl-server or qsl-attachments mutation;
- no qshield runtime mutation;
- no website, public docs, README, or START_HERE mutation;
- no qwork, qstart, qresume, or qshell execution or mutation by Codex;
- no backup execution;
- no restore execution;
- no qsl-backup mutation;
- no backup status or backup plan mutation;
- no rollback subtree or `/backup/qsl` mutation;
- no public technical paper content;
- no secret material handling;
- no unsupported public assurance claim.

Acceptance criteria:

- qsc fuzz-lock pqcrypto residual dependency blocker is classified using current evidence;
- exact future remediation authorization scope is selected or explicitly deferred;
- public-claim caveats are explicit;
- no runtime, crypto, dependency, Cargo, workflow, fuzz target, test, vector,
  service, public, backup, or local-ops mutation occurs;
- root cargo audit remains green;
- public-safety is green before merge and after merge;
- exactly one READY item remains.

Stop conditions preserved:

- qwork proof files missing, malformed, stale, or inconsistent;
- qwork, qstart, or qresume run by Codex;
- PR #1124 not merged at the expected merge commit lineage;
- `origin/main` not equal to or descended from `311a93ea9a47`;
- queue not READY NA-0428 at start;
- D-0843 absent or D-0844 already present at start;
- root cargo audit not green;
- nested fuzz lock cannot be inspected;
- successor cannot be selected safely;
- qsl-backup source-list regression;
- backup or restore execution by Codex;
- forbidden runtime, crypto, dependency, Cargo, lockfile, workflow, fuzz target,
  test, vector, public, service, qwork, backup, restore, qsl-backup,
  status/plan, rollback, README, START_HERE, or website mutation;
- public-safety red or missing;
- more than one READY item;
- unsupported public assurance claim introduced.

## qwork proof-file verification

Codex read, but did not run, qwork proof files:

- `/srv/qbuild/work/NA-0428/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0428/.qwork/startup.qsl-protocol.json`

The `.kv` proof reported the required values:

- `startup_result=OK`
- `lane=NA-0428`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0428/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0428`
- `requested_lane_status=READY`

The JSON proof parsed successfully and mirrored the required `.kv` fields for
lane, repo, path, head, origin/main, clean state, READY count, queue-top READY,
and requested-lane status.

After `git fetch --all --prune`, live `HEAD` and `origin/main` still matched
the qwork proof at `311a93ea9a47`. PR #1124 was verified MERGED with merge
commit `311a93ea9a47`. Public-safety on that commit completed success.

Proof root:

`/srv/qbuild/tmp/NA0428_qsc_fuzz_lock_pqcrypto_blocker_authorization_20260605T165135-0500`

The qwork proof files were copied into the proof root under `qwork/`. A
temporary untracked repo copy under that proof root was used only to simulate a
nested fuzz lock refresh without mutating the real worktree.

## NA-0427 inheritance

NA-0427 consumed the NA-0426 provider-boundary findings matrix and selected
NA-0428 because F-0426-04 was classified as an active qsc fuzz-lock pqcrypto
residual dependency blocker candidate.

Inherited facts:

- root `cargo audit --deny warnings` was green;
- root `rustls-webpki` was `v0.103.13`;
- root `ml-kem v0.2.1` was active through `quantumshield_refimpl`;
- root `pqcrypto-mlkem`, `pqcrypto-traits`, and `pqcrypto-internals` package
  IDs were absent from the locked graph;
- the separate committed qsc fuzz lock still recorded pqcrypto packages;
- `qsl/qsl-client/qsc/fuzz/Cargo.toml` was a real cargo-fuzz workspace;
- `scripts/ci/qsc_adversarial.sh` ran fuzz/adversarial smoke;
- `.github/workflows/qsc-adversarial.yml` wired qsc adversarial checks into CI;
- NA-0427 did not remediate the nested fuzz lock.

NA-0428 inherits the blocker but not authority to change the lock. Its job is
to authorize the exact future remediation scope or stop on ambiguity.

## Stewardship template application

### Crypto / Protocol Steward

Review question: Does the nested fuzz lock residual indicate a runtime provider
regression or require crypto implementation mutation?

Evidence reviewed: `qsl/qsl-client/qsc/Cargo.toml`,
`tools/refimpl/quantumshield_refimpl/Cargo.toml`, root `Cargo.lock`,
`cargo metadata --locked --format-version=1`, root inverse trees, qsc fuzz
manifest and lock, and NA-0426/NA-0427 provider-boundary evidence.

Findings: root runtime provider configuration continues to use the
`quantumshield_refimpl` `pqcrypto` historical feature name mapped to `pqkem`
and `ml-dsa`; `pqkem` maps to RustCrypto `ml-kem`. Root metadata includes
`ml-kem`, `qsc`, and `rustls-webpki`, and does not include `qsc-fuzz` or
pqcrypto package IDs. The residual pqcrypto packages are confined to the
separate nested fuzz lock.

Risk classification: LOW for runtime regression; MEDIUM dependency-health
blocker for active fuzz evidence.

Public-claim impact: no crypto-complete claim, no external-review-complete
claim, no side-channel-free claim, no bug-free claim, no vulnerability-free
claim, and no perfect-crypto claim is supported.

Scope impact: no runtime or crypto implementation mutation is authorized.

Recommended action: authorize lockfile-only fuzz lock cleanup and preserve
provider-error/no-mutation evidence work for a later exact lane.

### CI / Dependency / Release Health Steward

Review question: Is the nested fuzz lock active in CI/adversarial tooling, and
what is the minimal safe remediation surface?

Evidence reviewed: `qsl/qsl-client/qsc/fuzz/Cargo.toml`,
`qsl/qsl-client/qsc/fuzz/Cargo.lock`, `scripts/ci/qsc_adversarial.sh`,
`.github/workflows/qsc-adversarial.yml`, root cargo audit, nested lock audit,
root metadata, and the proof-root lock refresh simulation.

Findings: the fuzz workspace is separate from the root workspace but active in
the qsc adversarial smoke path for non-docs scopes. The committed nested lock
records `pqcrypto-mlkem 0.1.1`, `pqcrypto-traits 0.3.5`,
`pqcrypto-internals 0.2.11`, and `rustls-webpki 0.103.10`. The nested lock
audit reports denied advisories. The proof-root simulation refreshed only
`qsl/qsl-client/qsc/fuzz/Cargo.lock`, removed all pqcrypto packages, set
`rustls-webpki` to `0.103.13`, and passed nested lock audit.

Risk classification: ACTIVE SECURITY BLOCKER for the nested fuzz lock.

Public-claim impact: root cargo audit green is dependency-health evidence only
and not a broad public assurance claim.

Scope impact: future remediation can be limited to
`qsl/qsl-client/qsc/fuzz/Cargo.lock`.

Recommended action: select
`FUZZ_LOCK_REMEDIATION_AUTHORIZED_LOCKFILE_ONLY` and reject workflow/script
scope unless later implementation proof contradicts the simulation.

### Public Claims / External Review Steward

Review question: Does nested fuzz-lock triage create public security,
readiness, or external-review claims?

Evidence reviewed: stewardship canon, NA-0426/NA-0427 claim caveats,
`NEXT_ACTIONS.md`, DECISIONS, TRACEABILITY, root and nested audit output, and
workflow linkage.

Findings: NA-0428 is internal governance evidence and authorization only. It
does not create public docs, website content, README text, START_HERE text,
public technical paper content, an external-review package, public deployment
readiness, cryptographic completeness, defect-absence proof, or vulnerability
absence proof.

Risk classification: CLAIM_BOUNDARY.

Public-claim impact: public assurance claims remain unsupported by this lane.

Scope impact: no public-surface mutation is authorized.

Recommended action: preserve explicit claim caveats in NA-0429 and keep root
and nested audit results framed as dependency-health evidence only.

### Product / Demo / Service Boundary Steward

Review question: Does nested fuzz-lock cleanup change qsl-server,
qsl-attachments, qshield runtime, demo, or service-readiness state?

Evidence reviewed: qsc fuzz/adversarial script and workflow linkage, service
boundary notes in prior evidence, and forbidden scope for sibling repositories.

Findings: the qsc fuzz workspace is dev/test/CI tooling in qsl-protocol, not
production service runtime. It can affect adversarial confidence but provides
no service deployment readiness proof and no public-internet readiness proof.

Risk classification: CI/TOOLING BLOCKER; service boundary unchanged.

Public-claim impact: no production-readiness or public-internet-readiness claim
is supported.

Scope impact: no qsl-server, qsl-attachments, qshield runtime, demo, public
docs, website, README, or START_HERE mutation is authorized.

Recommended action: keep NA-0429 limited to nested fuzz lock cleanup and
validation.

### Local Ops / Backup / Restore Steward

Review question: Were qwork proof files, qsl-backup, backup state, and local
ops boundaries preserved?

Evidence reviewed: qwork proof files, live git state, qsl-backup checksum,
source inclusion count, prior response file, proof root, and rolling journal.

Findings: qwork proof files were read and copied to the proof root. Codex did
not run qwork, qstart, qresume, sudo, backup, or restore. qsl-backup checksum
matched the directive-required value, and the Codex ops source inclusion count
was exactly one. Codex did not mutate backup status, backup plan, qsl-backup,
rollback subtree paths, or `/backup/qsl`.

Risk classification: INFO.

Public-claim impact: no backup-complete claim, no off-host-backup-complete
claim, no disaster-recovery-complete claim, and no restore-proven claim is
supported.

Scope impact: no local mutable path changed except the directive-authorized
temporary proof root.

Recommended action: continue no-backup/no-restore discipline.

## Nested fuzz lock presence and content review

Read-only evidence:

- `qsl/qsl-client/qsc/fuzz/Cargo.toml` exists.
- `qsl/qsl-client/qsc/fuzz/Cargo.lock` exists.
- `qsl/qsl-client/qsc/fuzz/Cargo.toml` declares package `qsc-fuzz`, sets
  `cargo-fuzz = true`, contains `[workspace]`, depends on `libfuzzer-sys`,
  `ml-dsa = "=0.1.0-rc.7"`, and `qsc = { path = ".." }`.
- Root `Cargo.toml` workspace members do not include `qsl/qsl-client/qsc/fuzz`.
- Root `cargo metadata --locked --format-version=1` reported selected packages
  `ml-kem,qsc,rustls-webpki`, five workspace members, and
  `QSC_FUZZ_IN_METADATA no`.

Package versions in the committed nested fuzz lock:

- `pqcrypto-mlkem 0.1.1`
- `pqcrypto-traits 0.3.5`
- `pqcrypto-internals 0.2.11`
- `rustls-webpki 0.103.10`
- `ml-kem`: absent from the committed nested fuzz lock

Package versions in the committed root lock:

- `ml-kem 0.2.1`
- `rustls-webpki 0.103.13`
- root `pqcrypto-mlkem`: absent
- root `pqcrypto-traits`: absent
- root `pqcrypto-internals`: absent

Nested fuzz lock audit status:

- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
  reported a red audit for the separate lock.
- Advisory IDs observed:
  - `RUSTSEC-2026-0098` for `rustls-webpki 0.103.10`
  - `RUSTSEC-2026-0099` for `rustls-webpki 0.103.10`
  - `RUSTSEC-2026-0104` for `rustls-webpki 0.103.10`
  - `RUSTSEC-2026-0161` for `pqcrypto-mlkem 0.1.1`
  - `RUSTSEC-2026-0162` for `pqcrypto-traits 0.3.5`
  - `RUSTSEC-2026-0163` for `pqcrypto-internals 0.2.11`
  - `RUSTSEC-2026-0097` for `rand` versions in the nested dependency graph

Proof-root lock refresh simulation:

- Command shape: create an untracked copy under the NA-0428 proof root and run
  `cargo generate-lockfile` from the copied `qsl/qsl-client/qsc/fuzz`
  directory.
- Result: simulated nested lock contained `ml-kem 0.2.3`,
  `rustls-webpki 0.103.13`, `rand 0.9.4`, and zero entries for
  `pqcrypto-mlkem`, `pqcrypto-traits`, and `pqcrypto-internals`.
- `cargo audit --deny warnings --file <proof-root>/qsl/qsl-client/qsc/fuzz/Cargo.lock`
  passed on the simulated lock.
- Real repo worktree remained clean after the simulation.

## Fuzz / adversarial tooling linkage review

`scripts/ci/qsc_adversarial.sh`:

- sets `FUZZ_DIR="qsl/qsl-client/qsc/fuzz"`;
- runs stable qsc adversarial tests with `--locked`;
- changes into the fuzz workspace and runs:
  - `cargo +nightly fuzz run qsc_route_http`
  - `cargo +nightly fuzz run qsc_payload_boundaries`
  - `cargo +nightly fuzz run qsc_vault_envelope`

`.github/workflows/qsc-adversarial.yml`:

- defines `qsc-adversarial`;
- runs on pull requests, pushes to `main`, and manual dispatch;
- classifies docs-only scope;
- skips adversarial smoke only for docs-only pull requests;
- installs stable and nightly Rust;
- installs `cargo-fuzz`;
- runs `sh scripts/ci/qsc_adversarial.sh`.

Determinations:

- The nested fuzz lock is used by active dev/test/CI adversarial tooling.
- The fuzz/adversarial path is not runtime production code.
- Stale lock entries can reduce CI/adversarial dependency-health confidence.
- Current evidence supports lockfile-only remediation.
- Future NA-0429 does not need `qsl/qsl-client/qsc/fuzz/Cargo.toml` mutation.
- Future NA-0429 does not need workflow or script mutation.
- Future NA-0429 does not need fuzz target, test, vector, runtime, crypto, or
  root workspace mutation.

## Remediation option matrix

| Option | Evaluation | Decision |
|---|---|---|
| Option 1 - Fuzz lockfile refresh only | The fuzz manifest does not directly pin pqcrypto packages; root and refimpl manifests already map the historical feature to `ml-kem`/`ml-dsa`; a proof-root lock refresh removed pqcrypto packages and passed nested lock audit. | Selected. |
| Option 2 - Fuzz Cargo.toml dependency cleanup plus lockfile refresh | No direct pqcrypto dependency or stale provider pin was found in the fuzz manifest. | Rejected as unnecessary. |
| Option 3 - Fuzz harness dependency/provider alignment | Fuzz target code was not shown to depend on old provider assumptions, and the lock refresh simulation needed no harness changes. | Rejected as unsupported by current evidence. |
| Option 4 - Workflow/script adjustment | Current workflow/script correctly points at the active fuzz workspace; the problem is the stale committed nested lock. | Rejected unless future implementation proof contradicts NA-0428. |
| Option 5 - Waiver/ignore | The nested lock is active in adversarial tooling and audit-red. | Rejected. |
| Option 6 - Stop / ambiguity | Proof-root simulation identified a safe minimal scope. | Not selected. |

## Authorization decision

Classification:

`FUZZ_LOCK_REMEDIATION_AUTHORIZED_LOCKFILE_ONLY`

Authorized future mutable path:

- `qsl/qsl-client/qsc/fuzz/Cargo.lock`

Not authorized by NA-0428:

- root `Cargo.toml` or `Cargo.lock`;
- `qsl/qsl-client/qsc/fuzz/Cargo.toml`;
- fuzz target source files;
- qsc runtime source;
- crypto implementation source;
- tests or vectors;
- workflows or CI scripts;
- qsl-server, qsl-attachments, qshield runtime, website, public docs, README,
  or START_HERE;
- backup, restore, qsl-backup, backup status, backup plan, rollback subtree,
  `/backup/qsl`, qwork, qstart, qresume, or qshell;
- public technical paper content or public assurance claims.

Rationale:

- The root workspace is clean and audit-green.
- The residual is confined to the separate fuzz lock.
- The nested fuzz lock is active in CI/adversarial tooling.
- The current manifests already resolve away from pqcrypto in a proof-root lock
  refresh.
- Lockfile-only remediation is the smallest evidence-backed future mutation.

## Successor selection

Selected exact NA-0429 successor:

`NA-0429 -- QSL qsc Fuzz Lock pqcrypto Residual Lockfile Cleanup Implementation Harness`

NA-0429 must implement only the NA-0428-authorized nested fuzz lock cleanup and
must not alter runtime, crypto, root dependency files, workflows, tests,
vectors, service paths, public surfaces, backup/local-ops paths, or public
claim boundaries.

## Future path/scope bundle

Future allowed paths for lockfile-only NA-0429:

- `qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `docs/governance/evidence/NA-0429_qsl_qsc_fuzz_lock_pqcrypto_residual_lockfile_cleanup_implementation_harness.md`
- `tests/NA-0429_qsl_qsc_fuzz_lock_pqcrypto_residual_lockfile_cleanup_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future read-only inspection should include:

- `qsl/qsl-client/qsc/fuzz/Cargo.toml`
- `scripts/ci/qsc_adversarial.sh`
- `.github/workflows/qsc-adversarial.yml`
- root `Cargo.toml`
- root `Cargo.lock`
- relevant NA-0426 through NA-0428 evidence docs and testplans

Future forbidden unless a later exact directive expands scope:

- runtime or crypto implementation changes outside fuzz/adversarial dependency scope;
- root `Cargo.toml` or `Cargo.lock` changes;
- `qsl/qsl-client/qsc/fuzz/Cargo.toml` changes;
- fuzz target source changes;
- tests or vectors;
- workflow or CI script changes;
- qsl-server or qsl-attachments changes;
- qshield runtime changes;
- public docs, website, README, or START_HERE changes;
- backup, restore, qsl-backup, backup status, backup plan, rollback subtree,
  `/backup/qsl`, qwork, qstart, qresume, or qshell changes;
- public assurance claims.

Future validation for lockfile-only NA-0429:

- `cargo audit --deny warnings`
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `rg` confirms pqcrypto residual removed from qsc fuzz lock or intentionally absent
- qsc adversarial smoke if feasible
- qsc send_commit test
- formal model checks
- public-safety green before merge and after merge

## Future validation/marker plan

Common NA-0429 markers:

- `NA0429_QSC_FUZZ_LOCK_REMEDIATION_OK`
- `NA0429_ROOT_CARGO_AUDIT_GREEN_OK`
- `NA0429_FUZZ_LOCK_AUDIT_GREEN_OK`
- `NA0429_PQCRYPTO_RESIDUAL_REMOVED_OR_EXPLAINED_OK`
- `NA0429_NO_RUNTIME_CHANGE_OK`
- `NA0429_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0429_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0429_NO_SECRET_MATERIAL_OK`
- `NA0429_ONE_READY_INVARIANT_OK`
- `NA0429_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0429_LOCKFILE_ONLY_SCOPE_OK`

## Public claim/external review/website boundary

NA-0428 is internal governance evidence only.

- Nested fuzz-lock triage is not production readiness.
- Nested fuzz-lock triage is not public-internet readiness.
- Nested fuzz-lock triage is not crypto-complete proof.
- Nested fuzz-lock triage is not side-channel-free proof.
- Nested fuzz-lock triage is not bug-free proof.
- Nested fuzz-lock triage is not vulnerability-free proof.
- Nested fuzz-lock triage is not perfect-crypto proof.
- Nested fuzz-lock triage is not public technical paper content.
- No README, START_HERE, public docs, docs-public, website, qsl-server, or
  qsl-attachments update is authorized.
- No public-readiness or public-security claim is authorized.
- Root cargo audit green is dependency-health evidence only.

## Rejected alternatives

- Treating root cargo audit green as resolving the nested fuzz lock: rejected
  because the fuzz workspace has its own committed lock and active CI linkage.
- Mutating the root workspace: rejected because root metadata and root inverse
  trees already show pqcrypto absence and root cargo audit is green.
- Editing `qsl/qsl-client/qsc/fuzz/Cargo.toml`: rejected because no direct
  pqcrypto pin exists and a lock-only simulation resolved the blocker.
- Editing workflow or script files: rejected because current wiring points at
  the correct active fuzz workspace.
- Waiving or ignoring the nested lock audit: rejected because the active fuzz
  lock is audit-red and can affect adversarial confidence.

## Backup-impact statement

Codex did not run backup, restore, sudo, qwork, qstart, or qresume. Codex did
not mutate qsl-backup, `/backup/qsl`, backup status files, backup plan files,
rollback subtree paths, systemd, timers, fstab, source lists, retention, or
backup scripts.

qsl-backup checksum matched:

`e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`

The Codex ops source inclusion count in qsl-backup was exactly `1`.

The only local proof mutation was the directive-authorized temporary proof root
under `/srv/qbuild/tmp/`.

## Recovered / classified non-zero commands

- Failing command: `python3 scripts/ci/qsl_evidence_helper.py goal-lint --help`.
  Classification: recoverable command-shape discovery error; the helper has no
  `goal-lint` subcommand. Corrective action: use
  `scripts/audit/run_goal_lint_pr.sh` for PR goal-lint after PR creation. Final
  result: validation plan uses the repo's actual goal-lint wrapper.
- Failing command: `python3 scripts/ci/qsl_evidence_helper.py scope-classifier --help`.
  Classification: recoverable command-shape discovery error; the helper has no
  `scope-classifier` subcommand. Corrective action: use
  `scripts/ci/classify_ci_scope.sh` for classifier proof. Final result:
  validation plan uses the repo's actual classifier script.
- Failing command: root inverse-tree probes for `pqcrypto-mlkem`,
  `pqcrypto-traits`, and `pqcrypto-internals`. Classification: recoverable
  zero-match dependency proof because package-ID absence is the required root
  locked-graph outcome. Corrective action: record package-ID absence and keep
  root cargo audit as the root dependency-health gate. Final result: root
  pqcrypto package IDs are absent.
- Failing command: `cargo audit --deny warnings --file
  qsl/qsl-client/qsc/fuzz/Cargo.lock`. Classification: recoverable nested
  audit evidence requested under a non-stopping command shape. Corrective
  action: record the audit-red state and advisory IDs as blocker evidence.
  Final result: nested fuzz lock blocker confirmed.
- Failing command: `cargo tree --manifest-path
  qsl/qsl-client/qsc/fuzz/Cargo.toml --locked -i pqcrypto-mlkem` and equivalent
  locked fuzz-workspace inverse-tree probes. Classification: recoverable
  stale-lock proof because Cargo attempted to update the stale nested lock but
  `--locked` correctly prevented mutation. Corrective action: ran the lock
  refresh only inside the allowed temporary proof root. Final result: proof-root
  simulation removed pqcrypto packages and passed nested lock audit.
- Failing command: proof-root `cargo tree --manifest-path ... --locked -i
  pqcrypto-mlkem` after simulated lock refresh. Classification: recoverable
  zero-match proof because package-ID absence was the expected outcome. Final
  result: pqcrypto package IDs absent from the simulated remediated nested lock.
- Failing command: added-line overclaim scan over staged diff. Classification:
  recoverable in-scope documentation validation failure because required
  negative claim-boundary phrases had line wraps that separated the restricted
  phrase from its local negation. Corrective action: tightened the affected
  caveat lines so each restricted claim term carries local negative wording.
  Final result: `OVERCLAIM_AFFIRMATIVE_FINDING_COUNT 0`.

## Next recommendation

Merge this NA-0428 authorization PR after required checks pass, then perform
the optional closeout to restore:

`NA-0429 -- QSL qsc Fuzz Lock pqcrypto Residual Lockfile Cleanup Implementation Harness`

NA-0429 should refresh only `qsl/qsl-client/qsc/fuzz/Cargo.lock`, prove root
and nested cargo audits green, prove pqcrypto residual removed or explicitly
explained, and preserve all no-runtime, no-crypto, no-root-dependency,
no-workflow, no-test/vector, no-service, no-public-surface, no-backup, and
no-public-overclaim boundaries.
