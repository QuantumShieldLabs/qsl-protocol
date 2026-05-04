Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-04
Replaces: n/a
Superseded-By: n/a

# Release Readiness Evidence Map

## Summary

This map classifies current QSL release evidence by goal. It is intentionally conservative: evidence can be strong for a bounded property while overall release readiness remains partial or not ready.

Status meanings:

- `PROVEN`: current repo evidence proves the stated bounded property and a reviewer can reproduce it.
- `PARTIAL`: meaningful evidence exists, but release readiness still depends on open gates or broader reproduction.
- `NOT_READY`: current evidence does not yet support the release claim.

## G1 Through G5 Status Matrix

| Goal | Status | Evidence | Commands | Gaps | Next action |
| --- | --- | --- | --- | --- | --- |
| G1 - Always-hybrid per-message keys | PARTIAL | [GOALS.md](../../GOALS.md), [TRACEABILITY.md](../../TRACEABILITY.md), Suite-2 vectors, D-0462 claim-boundary mapping. | Existing Suite-2 CI/vector jobs; local NA-0250 proof included `cargo audit`, `send_commit`, formal, demo, metadata. | External release review and cross-host conformance reproduction remain open. | Keep public wording at research-stage Suite-2 / Triple-Ratchet-style until all G1-G5 release gates are green and reviewed. |
| G2 - Explicit SCKA with epoch monotonicity and persistence safety | PARTIAL | D-0445, [NA-0240 evidence](../governance/evidence/NA-0240_scka_persistence_monotonicity_audit.md), [formal README](../../formal/README.md). | `python3 formal/run_model_checks.py`; Suite-2 SCKA vector runners in CI. | Current evidence is bounded to model/refimpl surfaces and does not prove every future SCKA implementation path. | Extend reproducible vector map and keep no-mutation proofs tied to durable snapshots. |
| G3 - Fail-closed downgrade resistance | PARTIAL | D-0447, D-0464, [NA-0241 evidence](../governance/evidence/NA-0241_demo_downgrade_no_mutation_audit.md), [NA-0249 evidence](../governance/evidence/NA-0249_formal_downgrade_no_mutation_audit.md). | `python3 formal/run_model_checks.py`; demo smoke; Suite-2 downgrade/transcript vectors in CI. | Formal model abstracts authenticated transcript details; public demo downgrade surface is bounded. | Add more stateful reject no-mutation vectors where implementation surfaces expose safe harnesses. |
| G4 - Verification as a release gate | PARTIAL | Formal model checks, goal-lint, protected required checks, recent evidence audits, testplans. | `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`; `python3 formal/run_model_checks.py`; required CI contexts. | External review is not complete; cross-host reproduction remains uneven. | Package reviewer commands and capture external findings as separate evidence. |
| G5 - Metadata minimization lane | PARTIAL | [DOC-G5-001](../privacy/DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md), [DOC-G5-003](../privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md), D-0454, [NA-0244 evidence](../governance/evidence/NA-0244_metadata_conformance_negative_expansion_audit.md). | `scripts/ci/metadata_conformance_smoke.sh`; `scripts/ci/demo_cli_smoke.sh`. | Stable ids, timing, size, relay-visible metadata, and IP-level metadata remain observable. | Define phase-2 identifier rotation, padding defaults, retention/purge, and error-normalization expansion. |

## Release-Readiness Gate Checklist

| Gate | Current status | Evidence / note |
| --- | --- | --- |
| Dependency/advisory scan clean | PROVEN for current lockfile | `cargo audit --deny warnings` passed locally on 2026-05-04. |
| Required `public-safety` present | PROVEN | Branch protection required contexts include `public-safety`. |
| Latest main `public-safety` green | PROVEN at NA-0250 start | `origin/main` `3408b306666` public-safety completed successfully. |
| G1-G5 evidence mapped | PARTIAL | This document and [external review package](EXTERNAL_REVIEW_PACKAGE.md). |
| External cryptographic review complete | NOT_READY | No external review completion is recorded. |
| Production relay / service hardening complete | NOT_READY | qsl-server production hardening remains open and out of scope here. |
| Attachment demo readiness complete | NOT_READY | Attachment demo proof remains open. |
| KT-negative demo acceptance complete | NOT_READY | Demo surface does not yet carry truthful KT-negative proof. |
| Native desktop package proof complete | NOT_READY | Prior qbuild host lacked native packaging prerequisite proof. |
| Public website evidence-boundary implemented | NOT_READY | Website audit and plan exist; implementation handoff is a recommended successor. |
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

## Claim Boundary Map

| Claim | Status | Safe boundary |
| --- | --- | --- |
| Research-stage Suite-2 / Triple-Ratchet-style design | PROVEN | Safe with non-production and release-gated wording. |
| Always-hybrid per-message design | PARTIAL | Safe as current architecture/evidence claim; not production assurance. |
| Fail-closed downgrade resistance | PARTIAL | Safe for covered vectors/model/tests. |
| Metadata minimization | PARTIAL | Safe only with residual-leakage disclaimer. |
| One-command demo readiness | PARTIAL | Safe only as local loopback non-production demo acceptance. |
| Desktop GUI guided demo readiness | PARTIAL | Safe only as bounded prototype readiness. |
| Production-ready protocol | NOT_READY | Do not claim. |
| Proven true Triple Ratchet | NOT_READY | Do not claim. |
| Anonymity or metadata-free messaging | NOT_READY | Do not claim. |

## Demo / GUI / Website Readiness Map

| Surface | Current status | Evidence | Gap |
| --- | --- | --- | --- |
| qshield demo | PARTIAL | [NA-0246 evidence](../governance/evidence/NA-0246_one_command_demo_acceptance_audit.md). | KT-negative and attachment demo proof remain open. |
| qsc desktop GUI | PARTIAL | [NA-0247 evidence](../governance/evidence/NA-0247_desktop_gui_public_demo_readiness_audit.md). | Native package proof on fully provisioned host remains open. |
| Public website | NOT_READY for implementation | [WEBSITE_CLAIM_MATRIX.md](WEBSITE_CLAIM_MATRIX.md), [WEBSITE_UPDATE_PLAN.md](WEBSITE_UPDATE_PLAN.md), and [Suite-2 claim boundary](SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md). | Implementation handoff and external website repo changes remain future work. |

## Metadata / Privacy Readiness Map

| Topic | Current status | Boundary |
| --- | --- | --- |
| Loopback-only default | PROVEN for demo profile | Covered by metadata/demo smoke. |
| Required relay access control | PROVEN for demo profile | Covered by metadata/demo smoke. |
| Padding support | PARTIAL | Supported as optional profile; defaults and policy remain open. |
| Identifier rotation | NOT_READY | Future work. |
| Retention/purge policy | NOT_READY | Future work. |
| Error normalization | PARTIAL | Current smoke covers selected sanitized errors; broader normalization remains open. |
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
| Self-contained reviewer package | PROVEN by NA-0250 docs | Send package for external review. |
| Reproducible command list | PROVEN for current local commands | Add clean-host setup notes if reviewers hit environment gaps. |
| Known limitation disclosure | PROVEN by NA-0250 docs | Keep limitations visible in public and reviewer copy. |
| External cryptographic review findings | NOT_READY | Record reviewer findings in a later evidence lane. |

## Do Not Claim Yet

- Production-ready QSL protocol.
- Production-ready Triple Ratchet.
- Proven true Triple Ratchet.
- Quantum-proof communications.
- Anonymous messaging.
- Metadata-free or metadata-eliminated messaging.
- KT-negative public demo readiness.
- Attachment demo readiness.
- Native desktop package readiness across fully provisioned hosts.
- qsl-server production relay readiness.
- qsl-attachments production hardening.
