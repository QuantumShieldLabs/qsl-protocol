Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-01

# NA-0397 QSL Code / Crypto Research Watch and Audit Follow-Up Plan

Goals: G1, G2, G3, G4, G5

Directive: QSL-DIR-2026-06-01-217

## Executive Summary

NA-0397 maps authoritative code/crypto research-watch source categories and
current read-only QSL evidence into future code/crypto audit follow-up
candidates.

This lane is planning and governance evidence only. It does not perform a full
code/crypto audit, remediate findings, change cryptography, change runtime
behavior, change dependencies, mutate workflows, mutate qsl-server or
qsl-attachments, publish public docs, create a durable audit report outside this
governance evidence, handle secrets, or expand public claims.

Current startup posture:

- `cargo audit --deny warnings` completed successfully for the current
  lockfile.
- `cargo tree -i rustls-webpki --locked` reported `rustls-webpki v0.103.13`
  through `rustls v0.23.36`.
- qsl-server PR #56 remains merged at `d40e6003fdf0` and was inspected
  read-only.
- qsl-attachments PR #37 remains merged at `96b9352bd63` and was inspected
  read-only.
- NA-0380 read-only overall and code/crypto audit reports were present locally
  and matched their expected checksums.

Selected successor:

`NA-0398 -- QSL Metadata Privacy / Secure Messaging Claim Boundary Plan`

No immediate code/crypto critical blocker is selected by NA-0397. The carried
forward code/crypto themes remain future audit candidates, not completed audit
results. Cargo audit green, formal checks, model checks, read-only scan hits,
and source verification do not prove the project is bug-free,
vulnerability-free, externally reviewed, or perfectly secure.

## Live NA-0397 Scope

Live `NEXT_ACTIONS.md` shows:

- Status: READY.
- Goals: G1, G2, G3, G4, G5.
- Objective: create a qsl-protocol governance plan that maps code/crypto
  research watch sources and prior audit findings into future audit follow-up
  candidates, including crypto API misuse, nonce/key/RNG lifecycle,
  panic/unwrap, unsafe, side-channel, fuzz/property, formal-model, and
  dependency-duplication themes.

Allowed mutation for this lane is limited to:

- `docs/governance/evidence/NA-0397_qsl_code_crypto_research_watch_audit_follow_up_plan.md`
- `tests/NA-0397_qsl_code_crypto_research_watch_audit_follow_up_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Allowed source verification:

- Targeted read-only verification of authoritative code/crypto research venues,
  preprint sources, official project guidance, and local audit evidence.

Allowed read-only repo scans:

- Bounded `rg` inventory of code/crypto planning terms.
- `cargo audit --deny warnings`.
- dependency-tree inspection.
- existing targeted tests and formal checks already used by prior directives.

Forbidden scope includes runtime code, qsc/qsp/qsl implementation paths,
qshield runtime, Cargo files, dependencies, workflows, public docs, website,
README, START_HERE, qsl-server, qsl-attachments, qsc-desktop, backup
scripts/timers/fstab/services, qstart/qresume tooling, response archive
mutation except the final D217 response file, secret handling, off-host setup,
and durable audit reports outside this governance evidence.

Acceptance criteria:

1. READY_COUNT remains 1.
2. READY remains NA-0397 until closeout.
3. NA-0396 is DONE.
4. D-0774 and D-0775 exist once.
5. D-0776 is added once.
6. Research source categories are cited and classified.
7. NA-0380 findings are carried forward if present.
8. Read-only scans are recorded as planning evidence only.
9. Future code/crypto audit candidate groups are recorded.
10. Exact NA-0398 successor is selected.
11. Required CI is green before merge.

Stop conditions include source-verification failure that prevents useful
planning, code/crypto mutation, dependency or Cargo mutation, workflow mutation,
sibling-repo mutation, secret handling, public-claim expansion, multiple READY
items, or treating this plan as remediation, external review, or completed
audit.

## Inherited NA-0396 Rationale

NA-0396 selected NA-0397 because the dependency/advisory trigger policy was
complete enough to move from advisory-source handling into code/crypto research
watch and audit-follow-up planning.

NA-0396 established:

- PR #1055 merged at `4fd4f7e31803`.
- PR #1056 merged at `6df7de1063ba`.
- READY_COUNT 1 and READY NA-0397.
- NA-0396 DONE.
- D-0774 and D-0775 present once.
- D-0776 absent before NA-0397.
- cargo audit green against the current lockfile.
- `rustls-webpki v0.103.13`.
- No active dependency/advisory blocker selected.
- No dependency, Cargo, runtime, crypto, workflow, public-doc, backup, or
  sibling-repo mutation.

Inherited successor rationale:

- Advisory watch policy is necessary but not sufficient for code/crypto
  assurance.
- Prior read-only audit evidence identified code/crypto follow-up themes that
  require future scoped audit lanes.
- The next lane should classify future audit work without implementing fixes.

The future Project Goal / Operating Principles canon request remains a future
governance candidate only. It must not override live NA-0397 scope or the
selected NA-0398 successor.

## Authoritative Research / Audit Source Verification

Source verification used targeted official source locations. This is source
category verification only. It is not a literature review, code audit,
external review, or public-claim basis.

| Source category | Authority / publisher | URL | Access date | Source tier | Classification | QSL relevance | Audit-follow-up implication | Public-claim implication |
|---|---|---|---|---|---|---|---|---|
| IACR ePrint Archive | International Association for Cryptologic Research | https://eprint.iacr.org/ | 2026-06-01 | 1 | PREPRINT_SOURCE | Early cryptography implementation, protocol, and analysis papers can signal future audit themes. | Watch for papers affecting KEM/signature integration, transcript binding, side channels, or implementation assumptions. | Preprint evidence is not final peer review and must not support settled public claims alone. |
| Real World Crypto | IACR Real World Crypto | https://rwc.iacr.org/ | 2026-06-01 | 1 | RESEARCH_WORKSHOP | Practical cryptography engineering venue relevant to API misuse, deployment, protocols, and implementation pitfalls. | Use for future audit theme discovery and source-cited engineering cautions. | Workshop source discovery is not external review of QSL. |
| USENIX Security | USENIX Association | https://www.usenix.org/conferences/byname/108 | 2026-06-01 | 1 | RESEARCH_CONFERENCE | Systems security venue relevant to implementation flaws, fuzzing, side channels, and real-world crypto misuse. | Watch accepted papers for implementation-audit and security-testing themes. | Venue relevance does not imply QSL is audited or production-ready. |
| IEEE Symposium on Security and Privacy | IEEE Computer Society Technical Community on Security and Privacy | https://sp2026.ieee-security.org/ | 2026-06-01 | 1 | RESEARCH_CONFERENCE | Top security venue relevant to protocol analysis, implementation security, privacy, and side-channel research. | Watch for formal/implementation alignment, privacy, and protocol-state-machine themes. | Source category supports caution, not current assurance claims. |
| ACM CCS | ACM SIGSAC | https://www.sigsac.org/ccs/CCS2026/ | 2026-06-01 | 1 | RESEARCH_CONFERENCE | Top security conference relevant to cryptographic protocols, software security, and privacy. | Use for future audit candidate refinement. | Conference monitoring is not certification or compliance evidence. |
| NDSS Symposium | Internet Society / NDSS | https://www.ndss-symposium.org/ndss-symposium/ | 2026-06-01 | 1 | RESEARCH_CONFERENCE | Network and distributed-system security venue relevant to secure messaging, protocol state, and implementation attacks. | Watch for service-boundary, messaging, and traffic-analysis implications. | Does not support public-internet readiness claims. |
| CHES | IACR CHES | https://ches.iacr.org/ | 2026-06-01 | 1 | RESEARCH_CONFERENCE | Cryptographic hardware/software implementation venue relevant to side channels and implementation leakage. | Use for side-channel and constant-time follow-up planning. | No side-channel-free or constant-time-guaranteed claim is supported. |
| TCHES | IACR Transactions on Cryptographic Hardware and Embedded Systems | https://tches.iacr.org/ | 2026-06-01 | 1 | PEER_REVIEWED_RESEARCH_VENUE | Peer-reviewed implementation-security source for leakage, masking, and measurement methods. | Watch for side-channel methods and implementation audit standards. | Venue monitoring is not QSL external review. |
| CRYPTO | IACR CRYPTO | https://crypto.iacr.org/ | 2026-06-01 | 1 | RESEARCH_CONFERENCE | Core cryptography venue relevant to assumptions, reductions, and protocol design context. | Watch for primitives or proof assumptions that may affect future QSL analysis. | Does not validate QSL implementation. |
| Eurocrypt | IACR Eurocrypt | https://eurocrypt.iacr.org/ | 2026-06-01 | 1 | RESEARCH_CONFERENCE | Core cryptography venue relevant to primitives, signatures, KEMs, and protocols. | Watch for primitive/proof developments and migration implications. | Does not create compliance or assurance claims. |
| arXiv | Cornell University | https://arxiv.org/ | 2026-06-01 | 2 | PREPRINT_SOURCE | Broad preprint source for security, cryptography, formal methods, and systems papers. | Use only with explicit PREPRINT classification and later venue reconciliation where possible. | Preprint-only support must not be treated as settled evidence. |
| libFuzzer | LLVM Project | https://llvm.org/docs/LibFuzzer.html | 2026-06-01 | 3 | OFFICIAL_PROJECT_GUIDANCE | Official fuzzing engine documentation relevant to future fuzz-target planning. | Supports future fuzz/property/vector expansion design. | Fuzz planning is not proof of exhaustive testing. |
| cargo-fuzz | rust-fuzz project | https://github.com/rust-fuzz/cargo-fuzz | 2026-06-01 | 3 | OFFICIAL_PROJECT_GUIDANCE | Rust fuzzing tooling source relevant to future Rust fuzz harness planning. | Candidate source for future malformed-input and state-machine fuzz lanes. | Tool availability does not imply QSL fuzz coverage is complete. |
| proptest | proptest project / docs.rs | https://docs.rs/proptest/ | 2026-06-01 | 3 | OFFICIAL_PROJECT_GUIDANCE | Property-test framework documentation relevant to randomized invariant testing. | Supports future property-test expansion for fail-closed and state invariants. | Property tests remain bounded evidence. |
| TLA+ | Leslie Lamport / TLA+ project | https://lamport.azurewebsites.net/tla/tla.html | 2026-06-01 | 3 | OFFICIAL_PROJECT_GUIDANCE | Formal specification language source relevant to model/implementation alignment planning. | Supports future formal-model scope refinement. | Model presence is not full implementation proof. |
| ProVerif | Inria ProVerif project | https://proverif.inria.fr/ | 2026-06-01 | 3 | OFFICIAL_PROJECT_GUIDANCE | Protocol verification tool source relevant to future symbolic analysis consideration. | Candidate reference for future formal-methods feasibility review. | Tool reference is not QSL proof. |
| Tamarin Prover | Tamarin Prover project | https://tamarin-prover.com/ | 2026-06-01 | 3 | OFFICIAL_PROJECT_GUIDANCE | Protocol verification source relevant to future symbolic protocol analysis consideration. | Candidate reference for future formal-methods feasibility review. | Tool reference is not QSL proof. |
| Rust Book unsafe chapter | Rust Project | https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html | 2026-06-01 | 3 | OFFICIAL_PROJECT_GUIDANCE | Official explanation of unsafe Rust obligations and boundaries. | Supports future unsafe/memory-safety review criteria. | Rust safety guidance is not a QSL-specific audit result. |
| Rustonomicon | Rust Project | https://doc.rust-lang.org/nomicon/ | 2026-06-01 | 3 | OFFICIAL_PROJECT_GUIDANCE | Unsafe Rust reference for advanced unsafe-code invariants. | Supports future unsafe and FFI boundary review if unsafe is present. | Reference guidance does not prove memory safety. |
| NA-0380 overall read-only audit | Local QSL audit evidence | `/srv/qbuild/tmp/NA0380_post_completion_audit_20260529T005653-0500/NA0380_overall_project_readonly_audit.md` | 2026-06-01 | 4 | LOCAL_AUDIT_EVIDENCE | Local read-only audit carry-forward for whole-project residuals. | Carries forward audit themes and boundaries. | Local read-only audit is not external review. |
| NA-0380 code/crypto read-only audit | Local QSL audit evidence | `/srv/qbuild/tmp/NA0380_post_completion_audit_20260529T005653-0500/NA0380_code_crypto_readonly_audit.md` | 2026-06-01 | 4 | LOCAL_AUDIT_EVIDENCE | Local read-only code/crypto audit carry-forward. | Primary local basis for future code/crypto audit candidate grouping. | Read-only audit did not find CRITICAL/HIGH at that time, but does not prove absence of all issues. |

Citation gaps and uncertainty:

- Source categories are verified, but NA-0397 does not rank or summarize all
  current papers in those venues.
- Preprints can conflict with later peer-reviewed versions. Record conflicts
  rather than over-resolving them.
- Official tool guidance can inform future validation shape, but it does not
  prove current QSL coverage.
- Local NA-0380 audit evidence is internal read-only evidence, not independent
  external review.

## Prior Audit and Local Evidence Intake

NA-0380 reports:

- Overall audit report present.
- Overall audit checksum matched
  `66dd26c0b35b97113f160e4dd67fdc9992bd3be91c72452359fbef74dcef0913`.
- Code/crypto audit report present.
- Code/crypto audit checksum matched
  `70c21179e7a57dd168dff77e2d5bb18ac2ad1c7c285b216da7875ca712d1c099`.

Carried-forward NA-0380 code/crypto findings:

- Duplicate dependency families need review.
- Broad all-workspace test timed out.
- All-target clippy has test-only lints.
- qsc panic-demo utility should remain clearly non-production.
- refimpl actor has deterministic/test-boundary unwrap/RNG assumptions.
- Crypto API misuse review needed.
- Nonce/key lifecycle proof needed.
- Runtime panic/unwrap hardening review needed.
- Fuzz/property expansion needed.
- Formal-model gap review needed.
- Side-channel and external-review readiness remain incomplete.
- CRITICAL/HIGH issues: none found at that time.
- EVIDENCE_INCOMPLETE items remain.

Other local evidence:

- NA-0392 established external standards/source watch as supporting evidence.
- NA-0393 triaged external-watch sources and kept code/crypto research watch as
  a future lane.
- NA-0394 mapped PQC standards and migration posture; ML-KEM and ML-DSA remain
  implemented-but-evidence-incomplete for public claims.
- NA-0395 mapped RFC/draft boundaries and preserved that QSL does not claim
  TLS, HPKE, or MLS implementation/compliance.
- NA-0396 mapped dependency/advisory trigger policy and selected NA-0397.

What cannot be concluded:

- No bug-free, vulnerability-free, external-review-complete, audit-complete,
  formally proven, side-channel-free, production-ready, public-internet-ready,
  metadata-free, anonymity, or untraceable claim is supported.
- Search hits are not defects by themselves.
- Green advisory and local test evidence do not prove future advisory absence.

## Read-Only Code / Crypto Surface Inventory

Read-only inventory covered qsc/qsp/qsl protocol and crypto surfaces,
qshield-cli and demo boundaries, formal models, reference implementation and
oracle/vector surfaces, metadata runtime harnesses, and tests/scripts/docs
relevant to crypto/protocol planning.

Read-only file inventory by top-level area:

- `apps`: 30 tracked paths in scanned scope.
- `docs`: 273 tracked paths in scanned scope.
- `formal`: 6 tracked paths in scanned scope.
- `inputs`: 168 tracked paths in scanned scope.
- `qsl`: 203 tracked paths in scanned scope.
- `scripts`: 67 tracked paths in scanned scope.
- `tests`: 418 tracked paths in scanned scope.
- `tools`: 48 tracked paths in scanned scope.

Read-only scan result counts:

- panic/unwrap/expect/todo/unimplemented/unsafe term set: 2274 matches.
- RNG/nonce/key/transcript/KDF/secret/zeroize term set: 3258 matches.
- KEM/signature/suite/protocol/draft term set: 3358 matches.
- fuzz/property/differential/oracle/refimpl/formal/model term set: 2710
  matches.

These counts are planning evidence only. They are not proof of a bug, exploit,
secret leak, or completed audit.

## Crypto API Misuse / API-Boundary Follow-Up

| Field | Classification / plan |
|---|---|
| QSL state | AUDIT_PLANNED; IMPLEMENTED_BUT_EVIDENCE_INCOMPLETE for relevant crypto surfaces |
| Current evidence | qsc/qsl/qshield/refimpl surfaces exist; NA-0380 carried forward API misuse review; targeted tests and formal models exist for some protocol invariants. |
| Future audit candidate | Review crypto API construction sites, unsafe API combinations, key material boundaries, public/private API separation, misuse-resistant defaults, test-only/demo-only boundaries, sanitized errors, and fail-closed rejection behavior. |
| Claim boundary | No public claim that QSL crypto APIs are misuse-proof or externally reviewed. |

## Nonce / Key / RNG Lifecycle Follow-Up

| Field | Classification / plan |
|---|---|
| QSL state | AUDIT_PLANNED; TEST_ONLY_BOUNDARY for deterministic fixtures; IMPLEMENTED_BUT_EVIDENCE_INCOMPLETE for runtime lifecycle evidence |
| Current evidence | Read-only scans show RNG, nonce, key, transcript, KDF, secret, and zeroize surfaces across implementation, tests, docs, and reference code. NA-0380 carried forward nonce/key lifecycle proof need. |
| Future audit candidate | Review nonce generation and uniqueness, RNG sources, deterministic test boundaries, key generation, key derivation, transcript/key-schedule binding, key erasure/zeroization, and state mutation on reject. |
| Claim boundary | No claim that lifecycle evidence is complete, that key erasure is comprehensive, or that all RNG/nonces are proven correct. |

## KEM / Signature / Key Schedule / Transcript Follow-Up

| Field | Classification / plan |
|---|---|
| QSL state | AUDIT_PLANNED; FORMAL_EVIDENCE_EXISTS for bounded model checks; REFERENCE_ORACLE_EXISTS for selected oracle/vector surfaces; IMPLEMENTED_BUT_EVIDENCE_INCOMPLETE for integration claims |
| Current evidence | NA-0394 carried forward ML-KEM/ML-DSA as implemented-but-evidence-incomplete; qsc send_commit and formal models provide bounded evidence; scans show KEM, signature, suite-id, handshake, transcript, and draft/source terms. |
| Future audit candidate | Review ML-KEM and signature boundaries, key schedule, transcript binding, suite-id/domain separation, downgrade/cross-suite confusion, formal-model coverage alignment, and vector/oracle coverage. |
| Claim boundary | No claim of protocol proof completion, standards compliance, or full implementation correctness. |

## Panic / Unwrap / Expect / Abort Behavior Follow-Up

| Field | Classification / plan |
|---|---|
| QSL state | AUDIT_PLANNED; TEST_ONLY_BOUNDARY for expected test unwraps; DEMO_ONLY for panic-demo utility |
| Current evidence | Read-only scan found panic/unwrap/expect/unsafe term-set matches across implementation, tests, docs, and reference/demo paths. NA-0380 carried forward panic-demo and refimpl deterministic/test-boundary assumptions. |
| Future audit candidate | Review production-path panic/unwrap/expect, panic-demo isolation, test-only unwraps, refimpl assumptions, error propagation, fail-closed behavior, and no-mutation-on-reject behavior. |
| Claim boundary | Search hits are not defects; no claim that all abort behavior has been hardened. |

## Unsafe / Memory Safety / FFI Follow-Up

| Field | Classification / plan |
|---|---|
| QSL state | AUDIT_PLANNED; BLOCKED_PENDING_CODE_CRYPTO_AUDIT for conclusive memory-safety boundary classification |
| Current evidence | Rust official unsafe guidance was verified; read-only term scans include unsafe-related matches, but NA-0397 did not classify each match. |
| Future audit candidate | Review every unsafe block if present, FFI boundary if present, dependency-driven unsafe assumptions, zeroization/secret-memory boundaries, and documentation of invariants. |
| Claim boundary | No memory-safety-complete or unsafe-free claim is made. |

## Side-Channel / Timing / Secret-Dependent Behavior Follow-Up

| Field | Classification / plan |
|---|---|
| QSL state | AUDIT_PLANNED; BLOCKED_PENDING_EXTERNAL_REVIEW for strong side-channel assurance; IMPLEMENTED_BUT_EVIDENCE_INCOMPLETE for constant-time assumptions |
| Current evidence | CHES/TCHES categories were verified; NA-0380 carried forward side-channel/external-review readiness incomplete. |
| Future audit candidate | Review constant-time assumptions, secret-dependent branches, timing limits, traffic/timing/metadata distinctions, side-channel test feasibility, and external-review prerequisites. |
| Claim boundary | No side-channel-free, constant-time-guaranteed, timing-hidden, traffic-hidden, metadata-free, anonymity, or untraceable claim is supported. |

## Fuzz / Property / Differential / Vector Testing Follow-Up

| Field | Classification / plan |
|---|---|
| QSL state | AUDIT_PLANNED; HARNESS_EVIDENCE_EXISTS for existing targeted harnesses; REFERENCE_ORACLE_EXISTS where refimpl/oracle surfaces apply |
| Current evidence | cargo-fuzz, libFuzzer, and proptest sources were verified; read-only scans show fuzz/property/oracle/refimpl/formal/model surfaces. NA-0380 carried forward fuzz/property expansion need. |
| Future audit candidate | Define fuzz targets, property tests, differential tests, oracle/refimpl checks, KAT/conformance vectors, malformed input vectors, negative/fail-closed vectors, and long-running suite boundaries. |
| Claim boundary | Existing harnesses and future fuzzing are bounded evidence, not exhaustive correctness proof. |

## Formal Model / Implementation Alignment Follow-Up

| Field | Classification / plan |
|---|---|
| QSL state | FORMAL_EVIDENCE_EXISTS for bounded models; AUDIT_PLANNED for implementation-to-model alignment; IMPLEMENTED_BUT_EVIDENCE_INCOMPLETE for full assurance |
| Current evidence | qsc formal model checks exist and passed in prior/current validation; NA-0380 carried forward formal-model gap review. |
| Future audit candidate | Review model assumptions, bounded limits, implementation-to-model correspondence, qshield demo/model separation, protocol state-machine coverage, and cross-suite/domain separation. |
| Claim boundary | Formal checks are not a full proof of implementation correctness. |

## Dependency Duplication / Crypto Dependency Family Follow-Up

| Field | Classification / plan |
|---|---|
| QSL state | AUDIT_PLANNED; BLOCKED_PENDING_DEPENDENCY_POLICY only if future source-cited advisory/version-skew evidence appears |
| Current evidence | `cargo audit` was green; `cargo tree -i rustls-webpki --locked` showed `v0.103.13`; read-only duplicate dependency inspection showed duplicate families that require review. NA-0380 carried forward duplicate dependency family review. |
| Future audit candidate | Review duplicate crypto dependency families, version skew, transitive crypto crates, RustSec/GHSA/NVD linkage, cargo audit relation, and NA-0396 trigger policy alignment. |
| Claim boundary | Cargo audit green is advisory-health evidence only, not proof of dependency safety or absence of all vulnerabilities. |

## Demo / Refimpl / Service Boundary Follow-Up

| Field | Classification / plan |
|---|---|
| QSL state | DEMO_ONLY for qshield demo boundaries; TEST_ONLY_BOUNDARY for deterministic refimpl assumptions; BLOCKED_PENDING_SERVICE_PRODUCTION_EVIDENCE for qsl-server/qsl-attachments public-service claims |
| Current evidence | qshield-cli build/test passed; NA-0380 carried forward qsc panic-demo and refimpl assumptions; qsl-server PR #56 and qsl-attachments PR #37 were inspected read-only. |
| Future audit candidate | Review qshield demo/non-production boundary, qshield-cli boundary, reference implementation/oracle boundary, deterministic refimpl assumptions, qsl-server service-local evidence boundary, qsl-attachments service-local evidence boundary, and public-internet/production claim boundary. |
| Claim boundary | qshield demo evidence is not production proof; service-local sibling-repo proof is not public-internet or external-review proof. |

## Code / Crypto Audit Candidate Grouping

| Group | Evidence basis | Urgency | Likely allowed scope | Forbidden scope | Queue recommendation | Claim-boundary implication |
|---|---|---|---|---|---|---|
| A. QSL Crypto API Misuse and Boundary Audit Plan | NA-0380 carry-forward; RWC/USENIX/IEEE/CCS/NDSS source categories; read-only API/key/error surfaces | Medium-high | governance evidence, scoped read-only scans, targeted tests if authorized | code/crypto changes unless future lane authorizes exact files | Backlog candidate after NA-0398 | Prevents misuse-resistant or external-review overclaims. |
| B. QSL Nonce / Key / RNG Lifecycle Audit Plan | NA-0380 nonce/key proof need; RNG/nonce/key scans | Medium-high | read-only lifecycle map and future test plan | dependency/Cargo/runtime changes without exact scope | Backlog candidate | Prevents key/nonce lifecycle completeness overclaims. |
| C. QSL KEM / Signature / Key Schedule / Transcript Binding Audit Plan | NA-0394 PQC mapping; qsc/formal/refimpl evidence; IACR/CHES/TCHES categories | High | governance map and targeted existing validations | crypto implementation changes without exact scope | Backlog candidate, promote if new blocker evidence appears | Prevents standards/compliance/proof overclaims. |
| D. QSL Panic / Unwrap / Fail-Closed Runtime Boundary Audit Plan | NA-0380 panic/unwrap carry-forward; read-only term scans | Medium | classify panic/unwrap contexts and fail-closed evidence | broad remediation without exact scope | Backlog candidate | Prevents hardening-complete overclaims. |
| E. QSL Unsafe / Memory Safety / FFI Boundary Audit Plan | Rust unsafe guidance; read-only unsafe planning scan | Medium | inventory unsafe/FFI boundaries | code mutation unless authorized | Backlog candidate | Prevents memory-safety-complete overclaims. |
| F. QSL Side-Channel / Timing / Secret-Dependent Behavior Audit Plan | CHES/TCHES categories; NA-0380 side-channel gap | High for public claims; medium for queue ordering | source map, audit plan, possible test feasibility review | side-channel claims or implementation changes | Backlog candidate before public technical paper | Prevents side-channel-free and timing/traffic-hidden overclaims. |
| G. QSL Fuzz / Property / Differential / Vector Expansion Plan | libFuzzer/cargo-fuzz/proptest guidance; existing harnesses | Medium | future harness/test expansion plan | adding tests/code unless future lane authorizes | Backlog candidate | Prevents exhaustive-testing overclaims. |
| H. QSL Formal Model / Implementation Alignment Plan | qsc formal checks; TLA+/ProVerif/Tamarin source categories | Medium-high | model assumption map and correspondence plan | claiming proof completeness | Backlog candidate | Prevents formally-proven implementation overclaims. |
| I. QSL Dependency Duplication / Crypto Dependency Family Review Plan | NA-0396 policy; cargo audit; cargo tree duplicate evidence | Medium | dependency inventory and policy mapping | cargo update or dependency changes | Backlog candidate, promote on source-cited advisory blocker | Prevents dependency-safe/vulnerability-free overclaims. |
| J. QSL Demo / Refimpl / Service Boundary Audit Plan | qshield/refimpl/qsl-server/qsl-attachments read-only evidence | Medium | boundary plan and claim map | sibling-repo or runtime changes | Backlog candidate | Prevents demo-as-production and service-local-as-public claims. |

## Selected Successor

Selected exact successor:

`NA-0398 -- QSL Metadata Privacy / Secure Messaging Claim Boundary Plan`

Rationale:

- Source verification completed with citation caveats.
- NA-0380 code/crypto evidence was available and carried forward.
- No immediate CRITICAL/HIGH code/crypto blocker was selected.
- The next external-watch group is metadata privacy and secure messaging claim
  boundaries, which is required before public technical paper or public-doc work
  and complements the code/crypto audit candidates.

Rejected alternatives:

- `NA-0398 -- QSL Code / Crypto Critical Audit Finding Blocker Resolution`
  because NA-0397 did not identify an immediate critical blocker.
- `NA-0398 -- QSL Code / Crypto Research Watch Source / Evidence Gap
  Resolution` because required source categories and NA-0380 local evidence were
  available enough to complete this planning lane.

## Future Path / Scope Bundle

If the selected normal successor is restored, future NA-0398 allowed paths are:

- `docs/governance/evidence/NA-0398_qsl_metadata_privacy_secure_messaging_claim_boundary_plan.md`
- `tests/NA-0398_qsl_metadata_privacy_secure_messaging_claim_boundary_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future forbidden scope:

- dependency changes.
- `Cargo.toml` or `Cargo.lock` changes.
- runtime code.
- crypto implementation.
- qsc/qsp/qsl implementation.
- qshield runtime.
- qsl-server.
- qsl-attachments.
- workflows.
- public docs or website.
- backup scripts.
- response archives.
- external claims.

Future NA-0398 may use targeted web verification only for metadata privacy and
secure messaging sources if live NA-0398 scope authorizes it.

## Public Claim / External Review / Website Boundary

NA-0397 preserves these boundaries:

- Code/crypto research watch is not code audit completion.
- Code/crypto planning is not remediation.
- Code/crypto planning is not external review.
- Cargo audit green is not bug-free proof.
- Formal model checks are not full proof of implementation correctness.
- No website or public docs update is made.
- No production or public-internet readiness claim is made.
- No metadata-free, anonymity, or untraceable claim is made.
- No bug-free or perfect-crypto claim is made.

## Future Validation / Marker Plan

Future NA-0398 markers if the normal successor is restored:

- `NA0398_METADATA_PRIVACY_CLAIM_BOUNDARY_PLAN_OK`
- `NA0398_SECURE_MESSAGING_SOURCE_REFERENCE_OK`
- `NA0398_TRAFFIC_ANALYSIS_CAVEAT_OK`
- `NA0398_TIMING_METADATA_CAVEAT_OK`
- `NA0398_COVER_TRAFFIC_BATCHING_PADDING_BOUNDARY_OK`
- `NA0398_NO_METADATA_FREE_CLAIM_OK`
- `NA0398_NO_ANONYMITY_CLAIM_OK`
- `NA0398_NO_UNTRACEABLE_CLAIM_OK`
- `NA0398_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0398_NO_PUBLIC_INTERNET_READY_CLAIM_OK`
- `NA0398_NO_RUNTIME_CHANGE_OK`
- `NA0398_NO_CRYPTO_IMPLEMENTATION_CHANGE_OK`
- `NA0398_NO_DEPENDENCY_CHANGE_OK`
- `NA0398_NO_WORKFLOW_CHANGE_OK`
- `NA0398_NO_SECRET_MATERIAL_OK`

## Future Project Goal / Operating Principles Canon Carry-Forward Note

The operator-requested Project Goal / Operating Principles canon artifact
remains a future governance candidate only:

`QSL Project Goal and Operating Principles Canon Authorization Plan`

Purpose retained for future consideration:

- QSL north star.
- security before speed.
- evidence over vibes.
- code and crypto excellence.
- no public overclaiming.
- one-READY queue discipline.
- routine audits as operating rhythm.
- external awareness without hype.
- public technical paper timing.
- shorter and safer future directives.
- Director, Codex, and human role boundaries.

This candidate is not selected over NA-0398 because current evidence supports
the metadata privacy / secure messaging claim-boundary lane as the next
external-watch group.

## Rejected Alternatives

- Changing code/crypto now.
- Changing dependencies now.
- Changing workflows now.
- Writing a durable audit report outside governance evidence now.
- Starting the public technical paper now.
- Treating source discovery as external review.
- Treating planning as remediation.
- Treating cargo audit green as proof of dependency safety.
- Treating formal checks as full implementation proof.

## Backup-Plan Impact Statement

No backup-plan update is required for NA-0397 because durable changes are
limited to tracked qsl-protocol governance, testplan, traceability, and rolling
journal files under the normal repository working tree.

Future durable code/crypto audit reports, recurring external-watch report
stores, public technical paper evidence stores, real restore targets, backup
source-list changes, scripts, timers, fstab entries, services, key material,
recovery envelopes, monitoring artifacts, off-host targets, backup operations,
restore operations, deploy operations, rollback operations, and public-claim
mutations require separate backup-impact and local-ops authorization review.

The current `/backup/qsl` status remains same-host continuity only. NA-0397
does not call it complete disaster recovery.

## Next Recommendation

Proceed to close out NA-0397 after the NA-0397 planning PR merges and
post-merge public-safety is green. Restore exactly:

`NA-0398 -- QSL Metadata Privacy / Secure Messaging Claim Boundary Plan`

Do not implement NA-0398 during NA-0397 closeout.

## Source List

The source list is the table in "Authoritative Research / Audit Source
Verification." It includes the required title/category, authority, URL, access
date, source tier, source classification, QSL relevance, audit-follow-up
implication, and public-claim implication for each source category.
