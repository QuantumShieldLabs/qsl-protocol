Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-25
Replaces: n/a
Superseded-By: n/a

# Release Readiness Evidence Map

## Summary

This map classifies current QSL release evidence by goal. It is intentionally
conservative: evidence can be strong for a bounded property while overall
release readiness remains partial or not ready. This is an evidence map, not a
release certificate, release approval, production-readiness statement, or public
internet readiness statement.

Status meanings:

- `PROVEN`: current repo evidence proves the stated bounded property and a reviewer can reproduce it.
- `PARTIAL`: meaningful evidence exists, but release readiness still depends on open gates or broader reproduction.
- `NOT_READY`: current evidence does not yet support the release claim.
- `DOCS_ONLY`: the artifact or boundary is documented, but it is planning,
  orientation, or classification evidence rather than executable proof of a
  stronger claim.
- `FUTURE_GATE`: a future lane must add evidence before the status can improve.

## G1 Through G5 Status Matrix

| Goal | Status | Evidence | Commands | Gaps | Next action |
| --- | --- | --- | --- | --- | --- |
| G1 - Always-hybrid per-message keys | PARTIAL | [GOALS.md](../../GOALS.md), [TRACEABILITY.md](../../TRACEABILITY.md), Suite-2 vectors, D-0462 claim-boundary mapping. | Existing Suite-2 CI/vector jobs; local NA-0250 proof included `cargo audit`, `send_commit`, formal, demo, metadata. | External release review and cross-host conformance reproduction remain open. | Keep public wording at research-stage Suite-2 / Triple-Ratchet-style until all G1-G5 release gates are green and reviewed. |
| G2 - Explicit SCKA with epoch monotonicity and persistence safety | PARTIAL | D-0445, [NA-0240 evidence](../governance/evidence/NA-0240_scka_persistence_monotonicity_audit.md), [formal README](../../formal/README.md). | `python3 formal/run_model_checks.py`; Suite-2 SCKA vector runners in CI. | Current evidence is bounded to model/refimpl surfaces and does not prove every future SCKA implementation path. | Extend reproducible vector map and keep no-mutation proofs tied to durable snapshots. |
| G3 - Fail-closed downgrade resistance | PARTIAL | D-0447, D-0464, [NA-0241 evidence](../governance/evidence/NA-0241_demo_downgrade_no_mutation_audit.md), [NA-0249 evidence](../governance/evidence/NA-0249_formal_downgrade_no_mutation_audit.md). | `python3 formal/run_model_checks.py`; demo smoke; Suite-2 downgrade/transcript vectors in CI. | Formal model abstracts authenticated transcript details; public demo downgrade surface is bounded. | Add more stateful reject no-mutation vectors where implementation surfaces expose safe harnesses. |
| G4 - Verification as a release gate | PARTIAL | Formal model checks, goal-lint, protected required checks, recent evidence audits, testplans. | `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`; `python3 formal/run_model_checks.py`; required CI contexts. | External review is not complete; cross-host reproduction remains uneven. | Package reviewer commands and capture external findings as separate evidence. |
| G5 - Metadata minimization lane | PARTIAL | [DOC-G5-001](../privacy/DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md), [DOC-G5-003](../privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md), D-0454, [NA-0244 evidence](../governance/evidence/NA-0244_metadata_conformance_negative_expansion_audit.md), [NA-0288 gap plan](../governance/evidence/NA-0288_metadata_phase2_external_review_gap_plan.md), [NA-0290 identifier/padding design](../governance/evidence/NA-0290_metadata_phase2_identifier_padding_design.md), [NA-0291 identifier/padding harness](../governance/evidence/NA-0291_metadata_phase2_identifier_padding_harness.md), [NA-0292 sanitized-error/retention design](../governance/evidence/NA-0292_metadata_phase2_sanitized_errors_retention_design.md), and [NA-0293 sanitized-error/retention harness](../governance/evidence/NA-0293_metadata_phase2_sanitized_errors_retention_harness.md). | `scripts/ci/metadata_conformance_smoke.sh`; `scripts/ci/demo_cli_smoke.sh`; `scripts/ci/metadata_phase2_identifier_padding_harness.sh`; `scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh`. | Stable runtime ids, timing, size, relay-visible metadata, deployment metadata, contact graph, IP-level metadata, broader sanitized errors, and production retention/purge metadata remain observable or future-gated. The NA-0291 and NA-0293 harnesses prove policy fixtures only. They do not prove runtime anonymity or metadata elimination. | Close out NA-0293, then improve public evidence navigation without anonymity claims. |

## Release-Readiness Gate Checklist

| Gate | Current status | Evidence / note |
| --- | --- | --- |
| Dependency/advisory scan clean | PROVEN for current lockfile at validation time | `cargo audit --deny warnings` passed locally during NA-0541 validation on 2026-06-25, and the root plus nested qsc fuzz lockfiles retained `quinn-proto 0.11.15`. Advisory posture remains time-sensitive. |
| Required `public-safety` present | PROVEN | Branch protection required contexts include `public-safety`. |
| Latest main `public-safety` and `advisories` green | PROVEN at NA-0541 start | `origin/main` `9e7e389b6c42` completed `public-safety` and `advisories` successfully after PR #1354. |
| G1-G5 evidence mapped | PARTIAL | This document and [external review package](EXTERNAL_REVIEW_PACKAGE.md). |
| External cryptographic review complete | NOT_READY | No external review completion is recorded; [NA-0288](../governance/evidence/NA-0288_metadata_phase2_external_review_gap_plan.md) keeps package existence separate from review completion. |
| Production relay / service hardening complete | NOT_READY | Local qsl-server/qsl-attachments hardening evidence is mapped by [NA-0287](../governance/evidence/NA-0287_service_production_gate_evidence_map.md), but production operation remains future-gated. |
| Public internet service readiness | NOT_READY | NA-0287 records public ingress, TLS/proxy, firewall/ACL, source-IP, and public abuse proof as future gates. |
| Production backup/restore readiness | NOT_READY | NA-0286 proves local qsl-attachments stopped/quiesced full-root recovery boundaries only; production automation, restore drills, hot/live backup, partial restore support, and cross-node recovery remain future gates. |
| Metadata phase-2 complete | NOT_READY | [NA-0288](../governance/evidence/NA-0288_metadata_phase2_external_review_gap_plan.md) maps remaining gaps, [NA-0290](../governance/evidence/NA-0290_metadata_phase2_identifier_padding_design.md) designs the identifier/padding lane, [NA-0291](../governance/evidence/NA-0291_metadata_phase2_identifier_padding_harness.md) proves deterministic identifier/padding policy fixtures only, [NA-0292](../governance/evidence/NA-0292_metadata_phase2_sanitized_errors_retention_design.md) designs sanitized-error/retention-purge policy, and [NA-0293](../governance/evidence/NA-0293_metadata_phase2_sanitized_errors_retention_harness.md) proves deterministic sanitized-error/retention-purge policy fixtures only. Runtime rotation/default padding, broader runtime sanitized-error coverage, production retention/purge behavior, and broader phase-2 mitigations remain open. |
| External review package refreshed | DOCS_ONLY | [External review package](EXTERNAL_REVIEW_PACKAGE.md), [NA-0289 audit](../governance/evidence/NA-0289_external_review_package_refresh_audit.md), and [NA-0289 testplan](../../tests/NA-0289_external_review_package_refresh_testplan.md) align evidence references and claim boundaries after NA-0287/NA-0288. Refresh is not review completion. |
| Attachment demo readiness complete | PROVEN for non-production qshield demo only | [Attachment demo readiness](../demo/ATTACHMENT_PUBLIC_DEMO_READINESS.md) and [NA-0260 evidence](../governance/evidence/NA-0260_attachment_demo_readiness_audit.md) prove descriptor/fetch/decrypt/integrity behavior on the local demo surface. |
| KT-negative demo acceptance complete | PROVEN for non-production demo verifier path only | [KT-negative demo readiness](../demo/KT_NEGATIVE_PUBLIC_DEMO_READINESS.md) and [NA-0259 evidence](../governance/evidence/NA-0259_kt_negative_demo_readiness_audit.md) prove bounded verifier rejects and accepted-state no-mutation. |
| Native desktop package proof complete | PROVEN for bounded Linux AppImage/screenshot proof only | [NA-0258 evidence](../governance/evidence/NA-0258_native_desktop_package_screenshot_audit.md) records provisioned-host package and screenshot proof; it is not production desktop approval. |
| Public website evidence-boundary implemented | NOT_READY | Website audit and plan exist; implementation handoff is a recommended successor. |
| Public repository evidence sync | PROVEN for selected repository docs only | D-1068 and [NA-0539 evidence](../governance/evidence/NA-0539_qsl_website_repository_public_evidence_sync_implementation_harness.md) update README and public docs with bounded evidence, limits, and review invitation. This is not website implementation and not release readiness. |
| Public Progress cadence and site-wide accuracy sweep | PROVEN for repository public docs only | D-1070, D-1071, D-1072, [Progress index](PROGRESS.md), and [June 25 Progress entry](progress/2026-06-25.md) create the first Progress log and correction ledger. This is not website implementation, public readiness, production readiness, public internet readiness, or external review completion. |
| No production-readiness overclaim | PROVEN for this package | Safe/unsafe wording is explicit. |

## CI Evidence Map

| Required context | What it contributes | Release-readiness interpretation |
| --- | --- | --- |
| `ci-4a`, `ci-4b`, `ci-4c`, `ci-4d`, `ci-4d-dur` | Core CI shards for current repo behavior. | Necessary but not sufficient for production readiness. |
| `demo-cli-build` | Builds the demo CLI surface. | Demo build proof only. |
| `demo-cli-smoke` | Runs bounded one-command demo smoke. | Non-production demo proof only. |
| `formal-scka-model` | Runs formal/model checks. | Bounded model proof only. |
| `goal-lint` | Enforces goal metadata. | Governance consistency proof only. |
| `metadata-conformance-smoke` | Runs metadata conformance smoke. | Metadata minimization baseline proof only; no anonymity claim. |
| `suite2-vectors` | Runs Suite-2 vectors. | Conformance proof for covered categories. |
| `CodeQL` | Static-analysis security signal. | Required security gate; neutral/skipped cases must be interpreted by workflow policy. |
| `macos-qsc-qshield-build` | macOS build signal for qsc/qshield. | Cross-platform build signal, not desktop package proof. |
| `public-safety` | Branch-protection and scope/advisory gate. | Required integrity gate; not a replacement for release review. |

## Local Reproduction Map

| Command | Reviewer use |
| --- | --- |
| `cargo audit --deny warnings` | Confirms current advisory posture. |
| `cargo tree -i rustls-webpki --locked` | Confirms locked dependency path for `rustls-webpki`. |
| `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1` | Confirms qsc send/commit regression surface. |
| `python3 formal/run_model_checks.py` | Confirms bounded SCKA and Suite-2 negotiation model checks. |
| `scripts/ci/demo_cli_smoke.sh` | Confirms current one-command non-production demo acceptance. |
| `scripts/ci/metadata_conformance_smoke.sh` | Confirms current metadata conformance negative baseline. |
| `scripts/ci/metadata_phase2_identifier_padding_harness.sh` | Confirms deterministic metadata phase-2 identifier/opaque-handle and padding policy fixtures; design-only markers preserve runtime claim boundaries. |
| `scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh` | Confirms deterministic metadata phase-2 sanitized-error and retention/purge policy fixtures; harness-only markers preserve runtime and production-retention claim boundaries. |
| `docs/governance/evidence/NA-0288_metadata_phase2_external_review_gap_plan.md` | Reviewer map for metadata phase-2 and external-review readiness gaps; planning evidence only. |
| `docs/governance/evidence/NA-0292_metadata_phase2_sanitized_errors_retention_design.md` | Reviewer map for sanitized-error and retention/purge metadata design; planning evidence only. |

## Claim Boundary Map

| Claim | Status | Safe boundary |
| --- | --- | --- |
| Research-stage Suite-2 / Triple-Ratchet-style design | PROVEN | Safe with non-production and release-gated wording. |
| Always-hybrid per-message design | PARTIAL | Safe as current architecture/evidence claim; not production assurance. |
| Fail-closed downgrade resistance | PARTIAL | Safe for covered vectors/model/tests. |
| Metadata minimization | PARTIAL | Safe only with residual-leakage disclaimer. |
| One-command demo readiness | PARTIAL | Safe only as local non-production demo acceptance; now includes bounded KT-negative and attachment proof markers. |
| KT-negative demo readiness | PROVEN for non-production demo verifier path | Safe only as canonical verifier/vector reject and no-mutation proof inside the demo runner; not production KT deployment. |
| Attachment demo readiness | PROVEN for non-production qshield demo path | Safe only as encrypted descriptor/payload fetch/decrypt/integrity proof through the local demo relay; not production attachment service readiness. |
| Desktop GUI guided demo readiness | PARTIAL | Safe only as bounded prototype readiness with local package/screenshot proof; not production desktop readiness. |
| Production-ready protocol | NOT_READY | Do not claim. |
| Public internet service readiness | NOT_READY | Do not claim. |
| Production backup/restore readiness | NOT_READY | Do not claim. |
| External review completion | NOT_READY | Do not claim; package refresh, package existence, and reviewer orientation are not reviewer findings or dispositions. |
| Proven true Triple Ratchet | NOT_READY | Do not claim. |
| Anonymity or metadata-free messaging | NOT_READY | Do not claim. |

## Demo / GUI / Website Readiness Map

| Surface | Current status | Evidence | Gap |
| --- | --- | --- | --- |
| qshield demo | PARTIAL | [NA-0246 evidence](../governance/evidence/NA-0246_one_command_demo_acceptance_audit.md), [NA-0259 evidence](../governance/evidence/NA-0259_kt_negative_demo_readiness_audit.md), and [NA-0260 evidence](../governance/evidence/NA-0260_attachment_demo_readiness_audit.md). | Demo proof is still non-production; live qshield KT evidence input, cross-host/private-network attachment proof, and production relay/service hardening remain open. |
| qsc desktop GUI | PARTIAL | [NA-0247 evidence](../governance/evidence/NA-0247_desktop_gui_public_demo_readiness_audit.md) and [NA-0258 evidence](../governance/evidence/NA-0258_native_desktop_package_screenshot_audit.md). | Keychain active ops, handshake/session-establish UI, production packaging/release approval, and production desktop readiness remain open. |
| Public website | NOT_READY for implementation | [WEBSITE_CLAIM_MATRIX.md](WEBSITE_CLAIM_MATRIX.md), [WEBSITE_UPDATE_PLAN.md](WEBSITE_UPDATE_PLAN.md), and [Suite-2 claim boundary](SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md). | Implementation handoff and external website repo changes remain future work. |
| Public repository evidence sync | PROVEN for selected repository docs only | D-1068, [NA-0539 evidence](../governance/evidence/NA-0539_qsl_website_repository_public_evidence_sync_implementation_harness.md), and [NA-0539 testplan](../../tests/NA-0539_qsl_website_repository_public_evidence_sync_implementation_testplan.md). | No website source, public/ path, website/ path, production service, public internet, or external-review-complete claim is added. |

## What Changed After NA-0259 And NA-0260

- NA-0259 added bounded KT-negative public demo proof through the demo runner invoking canonical KT verifier vectors, deterministic reject checks, accepted-state no-mutation proof, and an explicit non-production disabled-shape boundary.
- NA-0260 added bounded attachment public demo proof through encrypted descriptor and encrypted payload messages, authenticated local relay fetch, descriptor-bound ciphertext validation, tampered-ciphertext reject proof, and no checked token/sentinel leakage.
- These proofs update demo evidence status only. They do not prove production KT deployment, production attachment service readiness, qsl-server hardening, qsl-attachments hardening, or external cryptographic review completion.

## What Changed After NA-0287 And NA-0288

- NA-0287 mapped qsl-server and qsl-attachments local executable hardening
  evidence into explicit production-gate boundaries. It did not approve
  production relay operation, production attachment service operation, public
  internet exposure, or external review completion.
- NA-0288 maps metadata phase-2 and external review readiness gaps. It keeps
  metadata residuals explicit and preserves no anonymity, no metadata-free, no
  untraceable, no external-review-complete, and no production-readiness claims.
- NA-0289 refreshes the external review package and this release map against
  NA-0287/NA-0288 evidence. It does not change any `NOT_READY` gate, complete
  metadata phase-2, complete external review, or approve production/public
  service claims.

## What Changed After NA-0290

- NA-0290 adds a docs/governance design for the first metadata phase-2
  identifier rotation / opaque handle and padding-default executable harness.
- The design references current executable demo, QSE, qsl-server, and
  qsl-attachments evidence, but it does not implement identifier rotation,
  padding defaults, or public-copy changes.
- Metadata phase-2 completion remains `NOT_READY`; anonymity, metadata-free
  messaging, untraceability, external review completion, production readiness,
  and public internet service readiness also remain `NOT_READY`.

## What Changed After NA-0291

- NA-0291 adds an executable harness and vector file for deterministic
  identifier / opaque-handle policy fixtures and padding-default policy
  fixtures.
- The harness emits design-only markers for rotation and padding defaults. It
  does not implement runtime identifier rotation, runtime default padding, or
  public-copy changes.
- Metadata phase-2 completion remains `NOT_READY`; anonymity, metadata-free
  messaging, untraceability, external review completion, production readiness,
  and public internet service readiness also remain `NOT_READY`.

## What Changed After NA-0292

- NA-0292 adds docs/governance design for metadata phase-2 sanitized-error
  expansion and retention/purge metadata policy.
- The design maps selected executable demo, metadata smoke, NA-0291,
  qsl-server, and qsl-attachments evidence while keeping service-local proof
  separate from qsl-protocol metadata phase-2 completion.
- Sanitized-error expansion and retention/purge policy are not implemented by
  NA-0292. Metadata phase-2 completion remains `NOT_READY`; anonymity,
  metadata-free messaging, untraceability, external review completion,
  production readiness, and public internet service readiness also remain
  `NOT_READY`.

## What Changed After NA-0293

- NA-0293 adds an executable harness and vector file for deterministic
  sanitized-error and retention/purge metadata policy fixtures.
- The harness emits markers only after bounded policy fixtures, NA-0291
  identifier/padding proof, metadata conformance smoke, demo smoke, and
  artifact leak/panic scans pass.
- The harness does not implement runtime sanitized-error normalization,
  production retention or deletion behavior, public-copy changes, anonymity,
  metadata-free messaging, untraceability, external review completion,
  production readiness, or public internet service readiness.
- Metadata phase-2 completion remains `NOT_READY`.

## What Changed After NA-0294

- NA-0294 refreshes public evidence navigation in [README.md](../../README.md),
  [START_HERE.md](../../START_HERE.md), and [docs/public/INDEX.md](INDEX.md)
  so readers can move from the public hook to evidence receipts and visible
  gaps faster.
- The refresh applies the NA-0290A public attention strategy without changing
  protocol, crypto, runtime, service, website, workflow, Cargo, dependency,
  branch-protection, or public-safety configuration.
- The update does not change any `NOT_READY` gate. Production readiness, public
  internet service readiness, external review completion, anonymity,
  metadata-free messaging, untraceability, runtime identifier rotation/default
  padding, broader runtime sanitized-error coverage, production
  retention/deletion behavior, and website implementation remain open.

## What Changed After NA-0539

- NA-0539 syncs selected repository public docs to current bounded evidence from
  D419, D439, D441, D446, D449, and D450 inheritance.
- The sync links direct qsc client-to-client E2EE evidence, same-host qsc tests,
  retained-qsc staging/restaging, SSH reverse-forward marker/ACK proof,
  Build-to-Inspiron qsc E2EE success, selected replay/corrupt negatives,
  selected wrong-peer and stale/replaced-peer negatives, repeated-run
  cleanup/freshness, public-safety/advisories gates, quinn-proto
  RUSTSEC-2026-0185 remediation, bounded formal/model checks, corpus validators,
  and secret-material scans.
- This remains engineering evidence only: no public readiness, no production
  readiness, no public internet readiness, no external review completion, no
  crypto completeness, no identity completeness, no trust completeness, no
  replay-proof status, no downgrade-proof status, no secret-material
  completeness, no side-channel freedom, no vulnerability freedom, no bug
  freedom, and no perfect crypto.
- qsl-server and qsl-attachments remain deferred from this repository public
  evidence sync.

## Bounded qsc Evidence And Gaps

| Evidence category | Current bounded evidence | Residual gap |
| --- | --- | --- |
| Same-host qsc tests | Current validation and prior qsc test lanes keep same-host client-to-client behavior in the evidence set. | Same-host tests do not prove public internet, service, or production operation. |
| Direct remote qsc E2EE | D446 records repeated remote qsc E2EE success using synthetic data under controlled lab conditions. | Synthetic controlled-lab proof is not crypto completeness or production service proof. |
| Retained qsc staging/restaging | D446 records retained qsc freshness and no stale state reuse across repeated runs. | This does not prove every future staging or release path. |
| Reverse-forward marker/ACK | D439 records port 39176 marker traversal and ACK proof for the diagnostic path. | Marker/ACK proof is transport evidence, not qsl-server or qsl-attachments integration. |
| Build-to-Inspiron qsc E2EE | D441 and D446 inheritance keep the Build-to-Inspiron remote qsc E2EE proof in the public evidence map. | The proof remains bounded to controlled synthetic data and selected hosts. |
| Wrong-peer and stale/replaced-peer negatives | D441 and D446 record selected fail-closed negatives and selected-state no-mutation checks. | Selected negatives are not universal identity or trust completeness. |
| Replay and corrupt-delivery negatives | D419 records selected replay/corrupt delivery negative boundaries. | Selected negatives are not a replay-proof or downgrade-proof claim. |
| Public-safety and advisories | NA-0541 startup proof records green public-safety/advisories gates on `origin/main` `9e7e389b6c42` after PR #1354. | CI gates are required integrity checks, not external review completion. |
| quinn-proto RUSTSEC-2026-0185 remediation | Startup proof confirms root and nested qsc fuzz lockfiles retain `quinn-proto 0.11.15`. | Advisory posture is time-sensitive and must stay gate-backed. |
| Formal/model checks | Formal/model checks remain bounded evidence for modeled slices. | They are not full cryptographic secrecy, side-channel, or implementation-completeness proofs. |
| Corpus validators and secret scans | Validation includes corpus/vector and private-material/secret-output scans where applicable. | Scans reduce exposure risk; they do not prove secret-material completeness. |
| qsl-server and qsl-attachments | Deferred from NA-0539 public evidence sync and NA-0541 Progress publication; existing service evidence remains separate production-gate material. | No production relay, attachment service, or public internet readiness is claimed. |

## Metadata / Privacy Readiness Map

## What Changed After NA-0541

- NA-0541 creates the public [Progress index](PROGRESS.md) and the first dated
  [June 25, 2026 Progress entry](progress/2026-06-25.md).
- The entry summarizes merged June 25 workday evidence, accepted decisions,
  site-wide public accuracy corrections, and the publication-time handoff.
- The site-wide public accuracy sweep refreshes stale current-main,
  public-safety, advisory, dependency-health, and recent-PR references in this
  map and the [external review package](EXTERNAL_REVIEW_PACKAGE.md).
- The update does not change any `NOT_READY` release gate: no public readiness,
  no production readiness, no public internet readiness, no external review
  completion, no crypto completeness, no identity completeness, no trust
  completeness, no replay-proof status, no downgrade-proof status, no
  secret-material completeness, no side-channel freedom, no vulnerability
  freedom, no bug freedom, and no perfect crypto.

| Topic | Current status | Boundary |
| --- | --- | --- |
| Loopback-only default | PROVEN for demo profile | Covered by metadata/demo smoke. |
| Required relay access control | PROVEN for demo profile | Covered by metadata/demo smoke. |
| Padding support | PARTIAL | Supported as optional profile; [NA-0291](../governance/evidence/NA-0291_metadata_phase2_identifier_padding_harness.md) proves deterministic policy fixtures for a named default profile, but runtime defaults are not implemented. |
| Identifier rotation | PARTIAL for policy harness; runtime NOT_READY | [NA-0291](../governance/evidence/NA-0291_metadata_phase2_identifier_padding_harness.md) proves deterministic opaque-handle policy fixtures and boundary distinction, but runtime rotation is not implemented. |
| Retention/purge policy | PARTIAL for policy harness; production behavior NOT_READY | [NA-0292](../governance/evidence/NA-0292_metadata_phase2_sanitized_errors_retention_design.md) designs qsl-protocol metadata phase-2 retention/purge proof, and [NA-0293](../governance/evidence/NA-0293_metadata_phase2_sanitized_errors_retention_harness.md) proves bounded policy fixtures. Service-local retention evidence and NA-0293 fixtures do not prove production retention/deletion behavior. |
| Error normalization | PARTIAL for selected smoke and policy harness; broader runtime coverage NOT_READY | Current smoke covers selected sanitized errors, [NA-0292](../governance/evidence/NA-0292_metadata_phase2_sanitized_errors_retention_design.md) designs broader policy, and [NA-0293](../governance/evidence/NA-0293_metadata_phase2_sanitized_errors_retention_harness.md) proves bounded sanitized-error policy fixtures. Broader runtime normalization remains open. |
| Anonymity | NOT_READY | Explicit non-goal. |

## Formal Verification Readiness Map

| Modeled slice | Status | Limitation |
| --- | --- | --- |
| SCKA bounded model | PARTIAL | State-machine invariant model, not cryptographic secrecy proof. |
| Suite-2 negotiation downgrade/no-mutation model | PARTIAL | Abstracts authenticated capability evidence and transcript binding. |
| Full protocol proof | NOT_READY | No full AEAD/KDF/authentication/secrecy model is claimed. |

## External Review Readiness Map

| Need | Current status | Next work |
| --- | --- | --- |
| Self-contained reviewer package | DOCS_ONLY by NA-0289 refresh | Refreshed with NA-0287 service-boundary evidence and NA-0288 metadata gap classifications; reviewer acceptance and findings remain separate. |
| Reproducible command list | PROVEN for current local commands where referenced | Commands have current local proof where listed, but reviewer handoff must still record exact reviewed commit and environment. |
| Known limitation disclosure | PROVEN by NA-0250/NA-0288 docs | Keep limitations visible in public and reviewer copy. |
| External cryptographic review findings | NOT_READY | Record reviewer findings in a later evidence lane; package existence is not review completion. |

## Do Not Claim Yet

- Production-ready QSL protocol.
- Production-ready Triple Ratchet.
- Proven true Triple Ratchet.
- Quantum-proof communications.
- Anonymous messaging.
- Metadata-free or metadata-eliminated messaging.
- Production KT deployment readiness.
- Live qshield KT evidence ingestion.
- Production attachment readiness.
- Cross-host/private-network attachment proof.
- Production-ready desktop release.
- qsl-server production relay readiness.
- qsl-attachments production hardening.
- public internet service readiness.
- production backup/restore readiness.
- Metadata phase-2 completion.
- External cryptographic review completion.
- External review package refresh as external review completion.
