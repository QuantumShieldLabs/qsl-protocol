Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0425 QSL Code / Crypto Audit Follow-Up Resumption Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0425 resumes the deferred QSL code/crypto audit follow-up stream after the
backup/log-code chain completed through NA-0422 and the advisory domain
stewardship canon was implemented through NA-0424.

This lane is governance planning and read-only inventory only. It records the
next bounded audit domains, applies the NA-0424 stewardship template, and
selects the first future audit lane without mutating runtime, crypto,
dependencies, workflows, qsl-server, qsl-attachments, qshield runtime, website,
public docs, README, START_HERE, qwork/qstart/qresume/qshell, qsl-backup,
backup status/plan files, rollback subtree paths, or `/backup/qsl`.

Selected successor:

`NA-0426 -- QSL Crypto API / Provider Boundary Read-Only Audit Plan`

Rationale: NA-0418/D257 recently replaced the runtime-reachable pqcrypto
ML-KEM provider with RustCrypto `ml-kem` while preserving the `PqKem768`
boundary. A read-only provider-boundary audit is the highest-leverage first
code/crypto follow-up and creates a clean pattern for later nonce/key/RNG,
fail-closed, formal, side-channel, fuzz/vector, dependency-family, and
demo/service-boundary lanes.

## Live NA-0425 scope

Live `NEXT_ACTIONS.md` lists:

`NA-0425 -- QSL Code / Crypto Audit Follow-Up Resumption Plan`

Status: READY.

Goals: G1, G2, G3, G4, G5.

Allowed mutation paths for the NA-0425 evidence PR:

- `docs/governance/evidence/NA-0425_qsl_code_crypto_audit_follow_up_resumption_plan.md`
- `tests/NA-0425_qsl_code_crypto_audit_follow_up_resumption_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Allowed read-only inspection covered governance evidence, tests, `qsl`,
`tools/refimpl`, `apps`, `formal`, `inputs`, `Cargo.toml`, and `Cargo.lock`.
The requested top-level `qsp` and `qsc` roots are absent in this checkout;
`qsc` is represented under `qsl/qsl-client/qsc` and was scanned through the
`qsl` root.

Forbidden mutation scope:

- no runtime mutation;
- no crypto implementation mutation;
- no dependency or Cargo mutation;
- no workflow mutation;
- no qsl-server, qsl-attachments, qshield runtime, website, public docs,
  README, or START_HERE mutation;
- no qwork, qstart, qresume, or qshell mutation or execution by Codex;
- no backup execution;
- no restore execution;
- no qsl-backup mutation;
- no backup status or backup plan mutation;
- no rollback subtree or `/backup/qsl` mutation;
- no public technical paper content;
- no secret material handling;
- no public-readiness, no production-readiness, and no public-internet-readiness
  claim;
- no external-review-complete, no crypto-complete, and no side-channel-free
  claim;
- no vulnerability-free, no bug-free, and no perfect-crypto claim;
- no backup-complete, no off-host-backup-complete, no disaster-recovery-complete,
  and no restore-proven claim;
- no metadata-free, no anonymity, and no untraceability claim.

Acceptance criteria:

- audit domains are explicitly scoped;
- stewardship canon is used as advisory structure;
- no runtime/crypto/dependency/workflow mutation occurs;
- no public claim expansion occurs;
- cargo audit remains green;
- public-safety is green before merge and after merge;
- exactly one READY item remains.

Stop conditions:

- qwork proof files missing, malformed, stale, or inconsistent;
- qwork, qstart, or qresume is run by Codex;
- PR #1118 not merged;
- `origin/main` not equal to or descended from PR #1118 merge commit;
- queue not READY NA-0425 at start;
- D-0837 absent or D-0838 already present at start;
- cargo audit not green;
- qsl-backup source-list regression;
- backup or restore executed by Codex;
- any forbidden runtime/crypto/dependency/workflow/public/service/local-ops
  mutation;
- more than one READY item;
- unsupported public assurance claim introduced.

## qwork proof-file verification

Codex read, but did not run, qwork proof files:

- `/srv/qbuild/work/NA-0425/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0425/.qwork/startup.qsl-protocol.json`

The `.kv` proof reported the required values:

- `startup_result=OK`
- `lane=NA-0425`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0425/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0425`
- `requested_lane_status=READY`

The JSON proof parsed successfully and mirrored the required `.kv` fields for
lane, repo, path, head, origin/main, clean state, READY count, queue-top READY,
and requested-lane status.

After `git fetch --all --prune`, live `HEAD` and `origin/main` still matched
the qwork proof at `cd88811051a7`. PR #1118 was verified MERGED with merge
commit `cd88811051a7`.

Proof root:

`/srv/qbuild/tmp/NA0425_code_crypto_audit_resumption_20260605T134025-0500`

The qwork proof files were copied into the proof root under `qwork/`.

Host timestamp note: local host `date --iso-8601=seconds` reported
`2026-06-05T13:38:38-05:00` and UTC `2026-06-05T18:38:38+00:00` during
startup capture, earlier than the embedded directive begin time. This is
recorded as operational friction and is not used as authority over qwork proof
files or live repo state.

## NA-0424 stewardship canon inheritance

NA-0424/D-0836 implemented the internal advisory stewardship canon at:

`docs/governance/DOMAIN_STEWARDSHIP_AND_DIRECTOR_WORKFLOW.md`

Inherited rules used by NA-0425:

- Lead Director remains final authority for directive issuance, READY
  promotion, queue order, PR merge recommendation, public-claim boundary,
  conflict resolution, and stop/retry decisions.
- Domain stewards are advisory reviewers only.
- Exactly one READY item remains mandatory.
- Steward input cannot override `NEXT_ACTIONS.md`, `DECISIONS.md`,
  `TRACEABILITY.md`, public-safety, branch protection, Project Goal canon,
  live repo state, or explicit directive scope.
- Steward recommendations may identify evidence gaps, risks, public-claim
  implications, validation needs, and future queue candidates, but they do not
  independently create queue state.

Stewardship domains applied:

- Crypto / Protocol Steward shaped the audit domain matrix.
- CI / Dependency / Release Health Steward shaped validation gates.
- Public Claims / External Review Steward shaped no-overclaim boundaries.
- Local Ops / Backup / Restore Steward preserved no-backup/no-restore and
  qwork proof-file boundaries.
- Product / Demo / Service Boundary Steward preserved demo, refimpl, qshield,
  qsl-server, and qsl-attachments boundaries.

## Prior code/crypto evidence intake

Prior evidence reviewed:

- NA-0394 PQC standards alignment and migration evidence mapping.
- NA-0395 IETF/CFRG RFC and draft boundary mapping.
- NA-0396 dependency/advisory watch trigger policy.
- NA-0397 code/crypto research watch and audit follow-up plan.
- NA-0418 RustSec pqcrypto dependency-health blocker remediation.
- D-0836 and D-0837 stewardship decisions.
- `TRACEABILITY.md` rows for NA-0394 through NA-0424.
- `docs/governance/PROJECT_GOAL_AND_OPERATING_PRINCIPLES.md`.
- `docs/governance/DOMAIN_STEWARDSHIP_AND_DIRECTOR_WORKFLOW.md`.

Previously identified audit domains:

- crypto API misuse / provider boundary;
- nonce / key / RNG lifecycle;
- KEM / signature / key schedule / transcript binding;
- panic / unwrap / expect / abort / fail-closed behavior;
- unsafe / memory safety / FFI;
- side-channel / timing / secret-dependent behavior caveats;
- fuzz / property / differential / vector testing;
- formal model / implementation alignment;
- dependency duplication / crypto dependency family;
- demo / refimpl / service boundary.

Prior no-go claims:

- no code/crypto audit-complete claim;
- no external-review-complete claim;
- no production-readiness or public-internet-readiness claim;
- no standards conformance, certification, FIPS validation, TLS compliance,
  HPKE compliance, or MLS compliance claim;
- no side-channel-free or constant-time-guaranteed claim;
- no vulnerability-free, bug-free, or perfect-crypto claim;
- no metadata-free, anonymity, or untraceability claim;
- no qshield-demo-as-production claim;
- no qsl-server/qsl-attachments service-local evidence as public-internet proof.

Known evidence gaps:

- ML-KEM and ML-DSA are implemented but evidence-incomplete for public claims.
- Provider boundary needs post-remediation review after the pqcrypto to
  `ml-kem` change.
- Nonce/key/RNG lifecycle evidence remains incomplete.
- KEM/signature/key schedule/transcript binding needs focused audit mapping.
- Panic/unwrap/fail-closed classification remains incomplete.
- Unsafe/FFI/memory-safety classification remains incomplete.
- Side-channel/timing caveats remain incomplete, and no public side-channel-free
  assurance claim is supported.
- Fuzz/property/differential/vector expansion remains future work.
- Formal model to implementation correspondence remains bounded evidence only.
- Duplicate crypto dependency families remain an audit-planning signal.
- Demo/refimpl/service evidence remains bounded and non-production.

Recently remediated pqcrypto provider issue:

- NA-0418/D257 classified the pqcrypto RustSec chain as
  runtime/security-critical reachable because `qsc` enabled
  `quantumshield_refimpl` with `features = ["pqcrypto"]`, and `StdCrypto`
  implemented `PqKem768` through the affected provider boundary.
- Remediation replaced the KEM provider with RustCrypto `ml-kem 0.2.1` while
  preserving the existing `PqKem768` trait and runtime helper API.
- Current `cargo audit --deny warnings` is green.
- Current inverse trees for `pqcrypto-mlkem`, `pqcrypto-traits`, and
  `pqcrypto-internals` report package absence under zero-failure-safe command
  shape.

Current validation used for qsc/formal/qshield family evidence:

- `cargo audit --deny warnings`;
- `cargo tree -i rustls-webpki --locked`;
- pqcrypto inverse-tree absence checks;
- `cargo fmt --check`;
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`;
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`;
- `python3 formal/run_model_checks.py`;
- qshield CLI build/test evidence in prior lanes when exact scope required it.

Residual public-claim constraints:

- cargo audit green is dependency-health evidence, not a no-vulnerability
  proof;
- formal checks are bounded model evidence, not full implementation proof;
- search counts are planning signals, not bug findings;
- internal governance evidence is not external review;
- read-only audit planning is not remediation.

## Stewardship template application

### Crypto / Protocol Steward

Review question: Which code/crypto audit domain should start the resumed stream
without mutating runtime or crypto behavior?

Evidence reviewed: NA-0394 through NA-0397 evidence, NA-0418 remediation
evidence, `PqKem768` / `StdCrypto` / `ml-kem` inventory, qsc/formal validation
history, and the stewardship canon.

Findings: The provider boundary is the best first domain because it was
recently changed by NA-0418, is central to QSL's PQ KEM abstraction, and can be
audited read-only before any runtime changes.

Risk classification: MEDIUM and EVIDENCE_INCOMPLETE.

Public-claim impact: no crypto-complete, no externally reviewed, no standards
conformance, no vulnerability-free, no bug-free, and no perfect-crypto claim is
supported.

Scope impact: future NA-0426 should be read-only unless a later exact scope
authorizes remediation.

Recommended action: add the normal NA-0426 provider-boundary read-only audit
successor.

### CI / Dependency / Release Health Steward

Review question: Does current dependency health force an alternative
dependency-blocker successor?

Evidence reviewed: public-safety status on `cd88811051a7`,
`cargo audit --deny warnings`, `cargo tree -i rustls-webpki --locked`, pqcrypto
inverse-tree checks, and `cargo tree -d --locked`.

Findings: public-safety is green, cargo audit is green, `rustls-webpki` is
`v0.103.13`, and pqcrypto blocker packages are absent. Duplicate dependency
families remain audit-planning material, not an active blocker.

Risk classification: INFO and CLAIM_BOUNDARY.

Public-claim impact: cargo audit green remains necessary health evidence only;
it is no vulnerability-free proof.

Scope impact: no dependency mutation is authorized or needed for NA-0425.

Recommended action: keep normal NA-0426; do not choose the dependency-blocker
alternative.

### Public Claims / External Review Steward

Review question: Does resuming code/crypto audit planning create any public
claim, website, README, START_HERE, public-doc, or paper obligation?

Evidence reviewed: Project Goal canon, stewardship canon, NA-0394/0395/0397
claim boundaries, and current NA-0425 scope.

Findings: NA-0425 is internal governance only. It must not create public
technical paper content, public docs, website edits, or public-readiness
wording. No external-review-complete wording may appear outside explicit
no-claim boundaries.

Risk classification: CLAIM_BOUNDARY.

Public-claim impact: all public assurance claims remain future-gated.

Scope impact: no public surface changes are allowed.

Recommended action: record explicit no-overclaim boundaries and keep NA-0426
internal/read-only.

### Product / Demo / Service Boundary Steward

Review question: Does audit planning blur demo/refimpl/service boundaries?

Evidence reviewed: NA-0394/0395/0397 service-boundary maps, qshield demo
evidence references, qsl-server PR #56 and qsl-attachments PR #37 read-only
status in prior traceability rows, and current forbidden scope.

Findings: qshield remains demo-local; qsl-server and qsl-attachments evidence
remain service-local prerequisites only. Provider-boundary audit should inspect
refimpl/qsc surfaces read-only but must not mutate sibling repos or imply
production service readiness.

Risk classification: MEDIUM and CLAIM_BOUNDARY.

Public-claim impact: no production-readiness or public-internet-readiness claim
is supported.

Scope impact: qsl-server and qsl-attachments are boundary references only.

Recommended action: keep demo/refimpl/service boundaries explicit in NA-0426.

### Local Ops / Backup / Restore Steward

Review question: Does NA-0425 preserve qwork proof, backup, restore, and local
ops boundaries?

Evidence reviewed: qwork proof files, current worktree state, qsl-backup hash
proof, qsl-backup source inclusion count, NA-0422 status/plan refresh evidence,
and rolling journal state.

Findings: qwork proof files are present and consistent. Codex did not run
qwork, qstart, qresume, sudo, backup, or restore. qsl-backup hash matched the
required value and the source inclusion count in qsl-backup is exactly one.

Risk classification: INFO.

Public-claim impact: no off-host-backup-complete, no disaster-recovery-complete,
no restore-proven, and no backup-complete claim is supported.

Scope impact: no backup/local status/plan mutation is allowed.

Recommended action: continue with read-only planning and record the same-host
continuity caveat.

## Read-only code/crypto surface inventory

Command family used:

`rg --fixed-strings --count-matches --no-heading -e <terms> qsl tools/refimpl apps formal inputs tests docs/governance/evidence`

Requested roots:

- `qsl`
- `qsp`
- `qsc`
- `tools/refimpl`
- `apps`
- `formal`
- `inputs`
- `tests`
- `docs/governance/evidence`

Scanned roots:

- `qsl`
- `tools/refimpl`
- `apps`
- `formal`
- `inputs`
- `tests`
- `docs/governance/evidence`

Missing requested top-level roots:

- `qsp`
- `qsc`

Inventory proof files:

- `/srv/qbuild/tmp/NA0425_code_crypto_audit_resumption_20260605T134025-0500/code_inventory/inventory_summary.txt`
- `/srv/qbuild/tmp/NA0425_code_crypto_audit_resumption_20260605T134025-0500/code_inventory/inventory_summary.json`
- `/srv/qbuild/tmp/NA0425_code_crypto_audit_resumption_20260605T134025-0500/cargo_tree/cargo_tree_duplicates_locked.txt`

Counts are planning signals, not bug findings.

| Search group | Total matches | Files with matches | Top clusters |
|---|---:|---:|---|
| Crypto API / provider terms | 1935 | 352 | `docs/governance/evidence` 811; `qsl/qsl-client/qsc` 382; `tools/refimpl` 310 |
| Nonce / key / RNG / transcript terms | 4139 | 641 | `docs/governance/evidence` 1727; `qsl/qsl-client/qsc` 809; `tools/refimpl` 273 |
| Panic / fail-closed terms | 7643 | 595 | `qsl/qsl-client/qsc` 3109; `docs/governance/evidence` 2179; `apps/qshield-cli` 544 |
| Unsafe / memory / FFI terms | 434 | 116 | `docs/governance/evidence` 268; `apps/qshield-cli` 45; `qsl/qsl-client/qsc` 31 |
| Formal / fuzz / vector terms | 4005 | 557 | `docs/governance/evidence` 1565; `inputs/suite2` 184; `tools/refimpl` 165 |
| Side-channel / timing terms | 1274 | 234 | `docs/governance/evidence` 779; `apps/qshield-cli` 24; timing testplans clustered under `tests` |

Selected per-term counts:

- `PqKem768`: 25
- `StdCrypto`: 154
- `ml_kem`: 26
- `ML-KEM`: 79
- `ML-DSA`: 44
- `provider`: 124
- `encaps`: 4
- `decaps`: 8
- `sign`: 843
- `verify`: 628
- `nonce`: 292
- `rng`: 75
- `random`: 47
- `OsRng`: 51
- `transcript`: 484
- `kdf`: 139
- `hkdf`: 0
- `key_schedule`: 12
- `secret`: 2964
- `zeroize`: 75
- `unwrap(`: 474
- `expect(`: 1547
- `panic!`: 137
- `todo!`: 0
- `unimplemented!`: 0
- `Result<`: 497
- `error`: 1379
- `reject`: 3609
- `unsafe`: 114
- `extern "C"`: 3
- `FFI`: 307
- `MaybeUninit`: 10
- `raw pointer`: 0
- `formal`: 1121
- `model`: 1469
- `vector`: 1015
- `oracle`: 306
- `differential`: 7
- `proptest`: 10
- `fuzz`: 69
- `KAT`: 8
- `constant_time`: 0
- `timing`: 1236
- `side-channel`: 30
- `ct_eq`: 0
- `subtle`: 8

Top file clusters by domain:

- Provider/API terms: `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`,
  `tools/refimpl/quantumshield_refimpl/src/kt/canonical.rs`,
  `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs`, and
  `qsl/qsl-client/qsc/src/handshake/mod.rs` are representative code clusters.
- Nonce/key/RNG/transcript terms: `qsl/qsl-client/qsc/src/vault/mod.rs`,
  `qsl/qsl-client/qsc/src/identity/mod.rs`,
  `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs`, and
  `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json` are representative
  clusters.
- Panic/fail-closed terms: `qsl/qsl-client/qsc/src/handshake/mod.rs`,
  `qsl/qsl-client/qsc/src/transport/mod.rs`,
  `qsl/qsl-client/qsc/src/main.rs`, `apps/qshield-cli/src/commands/relay.rs`,
  and qsc tests are representative clusters.
- Unsafe/memory/FFI terms: most matches are governance evidence and testplan
  references; code clusters include `apps/qshield-cli`,
  `qsl/qsl-client/qsc`, and `tools/refimpl`.
- Formal/fuzz/vector terms: `formal/`, `inputs/suite2`, and
  `tools/refimpl/quantumshield_refimpl/tests` are representative clusters.
- Side-channel/timing terms: matches are mainly governance and metadata timing
  evidence; `constant_time` and `ct_eq` had zero matches in the scanned roots.

Dependency duplicate signal:

- `cargo tree -d --locked` shows duplicate crypto-adjacent families including
  `ml-dsa`, `sha3`, `digest`, `rand_core`, `getrandom`, `hybrid-array`,
  `signature`, and related utility crates.
- This is a planning signal for domain I only. With cargo audit green, it is
  not selected as an active blocker.

## Audit domain matrix

| Domain | Purpose | Representative files or roots | Inherited evidence | Current validation coverage | Evidence gaps | Public-claim caveats | Future lane type | Priority | Steward review | Must not happen |
|---|---|---|---|---|---|---|---|---|---|---|
| A. Crypto API misuse / provider boundary | Review `PqKem768`, `StdCrypto`, provider/helper boundaries, misuse-resistant defaults, and fail-closed reject behavior after provider replacement. | `tools/refimpl/quantumshield_refimpl/src/crypto/`, `tools/refimpl/quantumshield_refimpl/tests/`, `qsl/qsl-client/qsc/`, `Cargo.lock` read-only. | NA-0397 group A; NA-0418 pqcrypto remediation preserved `PqKem768` while switching to `ml-kem`. | cargo audit green; rustls-webpki safe line; pqcrypto absence; qsc send_commit; formal checks. | No fresh read-only provider-boundary audit after NA-0418. | No misuse-proof, external-review-complete, standards conformance, vulnerability-free, bug-free, or perfect-crypto claim. | Read-only audit plan. | Next | Crypto / Protocol plus CI / Dependency | No runtime, crypto, dependency, or workflow mutation in NA-0426. |
| B. Nonce / key / RNG lifecycle | Map nonce uniqueness, RNG sources, deterministic test boundaries, key generation, key derivation, transcript binding, secret handling, and zeroization. | `qsl/qsl-client/qsc/src/vault/`, `qsl/qsl-client/qsc/src/identity/`, `tools/refimpl/quantumshield_refimpl/src/suite2/`, `inputs/suite2/`. | NA-0397 group B; Project Goal canon code/crypto excellence. | Existing qsc/formal validations cover selected invariants only. | Lifecycle map incomplete; zeroization and deterministic fixture boundaries need review. | No lifecycle-complete, key-erasure-complete, or RNG/nonce-proof claim. | Read-only audit plan, later test/vector lane. | Backlog | Crypto / Protocol | No broad key/nonce remediation without exact future scope. |
| C. KEM / signature / key schedule / transcript binding | Review ML-KEM, ML-DSA, suite/domain separation, transcript binding, downgrade/cross-suite confusion, and vector/formal alignment. | `qsl/qsl-client/qsc/src/handshake/`, `tools/refimpl/quantumshield_refimpl/src/qsp/`, `formal/`, `inputs/suite2/`. | NA-0394 PQC map; NA-0395 RFC/draft boundary; NA-0397 group C. | qsc send_commit; formal model scripts; existing vectors and oracle tests. | Provider/signature/context-binding and implementation-model correspondence remain incomplete. | No proof-complete, standards conformance, FIPS validation, TLS/HPKE/MLS compliance, or external-review-complete claim. | Read-only audit plan, then focused vector/formal expansion. | Backlog | Crypto / Protocol | No wire, key schedule, transcript, or crypto semantic change without exact scope. |
| D. Panic / unwrap / expect / abort / fail-closed behavior | Classify panic/unwrap/expect contexts and ensure production security paths fail closed with no mutation on reject. | `qsl/qsl-client/qsc/src/`, `apps/qshield-cli/src/`, `tools/refimpl/`, qsc tests. | NA-0397 group D; fail-closed Project Charter and Goal canon. | qsc send_commit and formal models cover selected fail-closed invariants. | Production/test/demo context classification incomplete. | No hardening-complete, panic-free, bug-free, or vulnerability-free claim. | Read-only audit plan; promote if obvious blocker appears in future authorized audit. | Backlog | Crypto / Protocol plus Product / Demo / Service | No mechanical unwrap removal or behavior drift without exact scope. |
| E. Unsafe / memory safety / FFI | Inventory unsafe blocks, FFI, raw pointer assumptions, dependency-driven unsafe assumptions, and secret-memory caveats. | `apps/`, `qsl/qsl-client/qsc/`, `tools/refimpl/`, dependency tree read-only. | NA-0397 group E; Rust unsafe guidance source category. | No dedicated unsafe audit in current validation. | Code-level classification incomplete; scanned `constant_time`/`ct_eq` zero does not prove side-channel status. | No memory-safety-complete or unsafe-free claim. | Read-only audit plan. | Backlog | Crypto / Protocol plus CI / Dependency | No unsafe or FFI mutation without exact future scope. |
| F. Side-channel / timing / secret-dependent behavior caveats | Review constant-time assumptions, secret-dependent branches, timing caveats, and metadata/timing distinctions. | `tools/refimpl/`, `qsl/qsl-client/qsc/`, `apps/qshield-cli`, metadata timing evidence. | NA-0397 group F; CHES/TCHES source categories; metadata/timing lanes. | Existing validation is functional/model-focused, not side-channel proof. | No dedicated side-channel audit or measurement evidence for crypto paths. | No side-channel-free, constant-time-guaranteed, timing-hidden, traffic-hidden, metadata-free, anonymity, or untraceability claim. | Caveat/source map and read-only feasibility plan. | Backlog before public paper | Crypto / Protocol plus Public Claims | No public side-channel assurance claim. |
| G. Fuzz / property / differential / vector testing | Plan fuzz targets, property tests, differential/oracle checks, KATs, malformed-input vectors, and fail-closed vectors. | `tests/`, `inputs/suite2/`, `tools/refimpl/quantumshield_refimpl/tests/`, `formal/`. | NA-0397 group G; libFuzzer/cargo-fuzz/proptest source categories. | qsc send_commit, formal scripts, and existing vector/oracle surfaces. | Fuzz target inventory and property coverage are incomplete. | No exhaustive-testing, bug-free, or vulnerability-free claim. | Test/vector planning lane after read-only audit. | Backlog | Crypto / Protocol plus CI / Dependency | No new tests or harnesses unless future scope authorizes exact files. |
| H. Formal model / implementation alignment | Map model assumptions, bounded limits, implementation correspondence, and qshield demo/model separation. | `formal/`, `qsl/qsl-client/qsc/src/handshake/`, `inputs/suite2/`, `tools/refimpl/`. | NA-0397 group H; NA-0394/0395 formal evidence caveats. | `formal/model_qsc_handshake_suite_id_bounded.py` and `formal/run_model_checks.py`. | Model-to-implementation trace and proof limits remain incomplete. | No formally-proven implementation or protocol-proof-complete claim. | Read-only model-correspondence plan. | Backlog | Crypto / Protocol | No proof-complete wording from bounded models. |
| I. Dependency duplication / crypto dependency family | Review duplicate crypto dependency families, version skew, RustSec/GHSA/NVD linkage, and provider-family ownership. | `Cargo.toml`, `Cargo.lock`, `tools/refimpl/`, `qsl/qsl-client/qsc/`. | NA-0396 policy; NA-0418 pqcrypto remediation; `cargo tree -d --locked`. | cargo audit green; rustls-webpki safe line; pqcrypto blocker absent. | Duplicate family risk classification incomplete. | Cargo audit green is no dependency-safe or vulnerability-free proof. | Watch-only unless advisory appears; later read-only dependency-family audit. | Watch-only/backlog | CI / Dependency | No cargo update, dependency mutation, or audit waiver in NA-0425/NA-0426. |
| J. Demo / refimpl / service boundary | Preserve qshield demo, refimpl/oracle, qsl-server, qsl-attachments, and service-local evidence boundaries. | `apps/qshield-cli`, `tools/refimpl`, `qsl/qsl-client/qsc`, prior qsl-server/qsl-attachments evidence refs. | NA-0395/0397 service-boundary rows; Project Goal canon service/demo honesty. | qshield evidence from prior lanes; no current sibling repo mutation. | Production/service evidence incomplete; refimpl/test-only assumptions need classification. | No demo-as-production, public-internet-ready, production-ready, or external-review-complete claim. | Boundary plan; future production prerequisites if selected. | Backlog | Product / Demo / Service plus Public Claims | No sibling repo mutation or public service-readiness claim. |

## Prioritization

Normal successor selected:

`NA-0426 -- QSL Crypto API / Provider Boundary Read-Only Audit Plan`

Selection rationale:

- NA-0418/D257 recently changed the provider implementation from pqcrypto to
  RustCrypto `ml-kem` while preserving the `PqKem768` boundary.
- Provider/API misuse review is the best first audit domain because it sits at
  the boundary between cryptographic provider details and supported runtime
  helper APIs.
- The future lane can remain read-only and governance/testplan scoped.
- It creates a repeatable template for later nonce/key/RNG, fail-closed,
  formal/model, side-channel, fuzz/vector, dependency-family, and
  demo/service-boundary lanes.

Rejected successor alternatives:

- `NA-0426 -- QSL Code Crypto Dependency Blocker Triage Plan`: rejected because
  cargo audit is green, rustls-webpki remains on the safe line, pqcrypto
  blocker packages are absent, and duplicate families are planning signals
  rather than an active blocker.
- `NA-0426 -- QSL Fail-Closed Crypto Error Boundary Blocker Triage Plan`:
  rejected because read-only inventory found many panic/fail-closed planning
  signals but did not establish an obvious active blocker within NA-0425 scope.
- `NA-0426 -- QSL Code / Crypto Audit Evidence Gap Resolution Plan`: rejected
  because evidence is sufficient to select the provider-boundary audit as the
  first focused lane.

## Selected successor

Exact selected successor:

`NA-0426 -- QSL Crypto API / Provider Boundary Read-Only Audit Plan`

NA-0426 objective:

Perform a bounded read-only audit of QSL crypto API and provider boundaries,
with emphasis on the `PqKem768` abstraction, the `ml-kem` provider replacement,
fail-closed reject behavior, provider misuse risks, test/vector/formal
coverage, and public-claim caveats, without changing runtime code or
dependencies.

## Future path/scope bundle

Future allowed mutation paths for normal NA-0426:

- `docs/governance/evidence/NA-0426_qsl_crypto_api_provider_boundary_read_only_audit_plan.md`
- `tests/NA-0426_qsl_crypto_api_provider_boundary_read_only_audit_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0426 may perform read-only audit of:

- `tools/refimpl/quantumshield_refimpl/src/crypto/`
- `tools/refimpl/quantumshield_refimpl/tests/`
- `qsl/qsl-client/qsc/`
- `qsp/` if present in future checkout
- `qsc/` if present in future checkout
- `formal/`
- `inputs/`
- `Cargo.toml`
- `Cargo.lock`
- relevant evidence docs

Future forbidden unless exact scope later authorizes it:

- no runtime or crypto implementation change;
- no dependency or Cargo change;
- no workflow change;
- no public docs, README, START_HERE, or website change;
- no qsl-server or qsl-attachments change;
- no backup, restore, qsl-backup, backup status, or backup plan change;
- no public technical paper content;
- no public-readiness, no production-readiness, and no public-internet-readiness
  claim;
- no external-review-complete, no crypto-complete, and no side-channel-free
  claim;
- no vulnerability-free, no bug-free, and no perfect-crypto claim;
- no secret material handling.

## Future validation/marker plan

Future NA-0426 markers:

- `NA0426_CRYPTO_API_PROVIDER_BOUNDARY_AUDIT_OK`
- `NA0426_PQKEM768_BOUNDARY_REVIEW_OK`
- `NA0426_MLKEM_PROVIDER_REVIEW_OK`
- `NA0426_FAIL_CLOSED_REJECT_REVIEW_OK`
- `NA0426_NO_RUNTIME_CHANGE_OK`
- `NA0426_NO_DEPENDENCY_CHANGE_OK`
- `NA0426_NO_WORKFLOW_CHANGE_OK`
- `NA0426_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0426_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0426_NO_SECRET_MATERIAL_OK`
- `NA0426_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0426_ONE_READY_INVARIANT_OK`

Future NA-0426 validation should include:

- qwork proof-file verification without rerunning qwork;
- queue helper and decision helper;
- public-safety status on current main;
- `cargo audit --deny warnings`;
- `cargo tree -i rustls-webpki --locked`;
- pqcrypto inverse-tree absence checks with zero-failure-safe command shape;
- `cargo fmt --check`;
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`;
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`;
- `python3 formal/run_model_checks.py`;
- link-check, leak-scan, overclaim scan, classifier, goal-lint, and exact scope
  guard.

## Public claim/external review/website boundary

NA-0425 records internal governance planning only.

Boundary statements:

- code/crypto audit resumption planning is not external review;
- code/crypto audit resumption planning is not production-readiness evidence;
- code/crypto audit resumption planning is not public-internet-readiness
  evidence;
- code/crypto audit resumption planning is not crypto-complete proof;
- code/crypto audit resumption planning is not side-channel-free proof;
- code/crypto audit resumption planning is not bug-free proof;
- code/crypto audit resumption planning is not vulnerability-free proof;
- code/crypto audit resumption planning is not perfect-crypto proof;
- code/crypto audit resumption planning is not public technical paper content;
- no README, START_HERE, public-doc, or website update is made;
- no public-readiness or public-security claim is made;
- cargo audit green is dependency-health evidence, not vulnerability-free proof.

## Rejected alternatives

- Implementing NA-0426 now.
- Mutating crypto/runtime/provider code now.
- Changing dependencies or Cargo files now.
- Changing workflows or required checks now.
- Updating public docs, README, START_HERE, website, or public paper content.
- Running backup or restore.
- Mutating qsl-backup, backup status, backup plan, rollback subtree paths, or
  `/backup/qsl`.
- Treating search hits as bug findings.
- Treating cargo audit green as no vulnerability-free proof.
- Treating bounded formal checks as full implementation proof.
- Treating internal audit planning as external review.

## Backup-impact statement

No backup-plan update is required for NA-0425 because durable qsl-protocol
changes are limited to tracked governance evidence, testplan, decision,
traceability, and rolling journal files.

NA-0425 did not run backup or restore. NA-0425 did not mutate qsl-backup,
`/backup/qsl`, backup status files, backup plan files, rollback subtree paths,
systemd, timers, fstab, source lists, retention, or backup scripts.

The backup/log-code chain remains complete through NA-0422. The current local
backup evidence remains same-host continuity only and is not off-host backup,
not disaster recovery, not restore proof, and not backup-complete evidence.

## Next recommendation

After the NA-0425 evidence PR merges and post-merge public-safety is green,
close out NA-0425 and restore exactly:

`NA-0426 -- QSL Crypto API / Provider Boundary Read-Only Audit Plan`

Do not implement NA-0426 during NA-0425 closeout.
