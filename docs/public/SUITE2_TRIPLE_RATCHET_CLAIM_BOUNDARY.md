Goals: G1, G2, G3, G4, G5

Status: Authoritative
Owner: QSL governance
Last-Updated: 2026-05-04
Replaces: n/a
Superseded-By: n/a

# Suite-2 Triple-Ratchet Claim Boundary

## Executive Summary

Safe short wording:

Suite-2 is a research-stage Triple-Ratchet-style hybrid messaging design.

This document defines the public claim boundary for Suite-2 / Triple-Ratchet-style wording. It maps current QSL repository evidence to safe public wording and keeps unsupported claims out of public copy until release-readiness gates prove them.

The repo currently supports a research-stage architecture claim: Suite-2 combines a classical EC message-key chain with a per-message PQ chain, sparse SCKA reseeds, hybrid message-key derivation, fail-closed downgrade checks, KT verifier evidence, no-state-mutation reject coverage, metadata conformance negatives, and bounded demo / GUI readiness evidence.

The repo does not support production-readiness, anonymity, metadata-elimination, "quantum-proof", deployment-ready, or "proven true Triple Ratchet" claims.

## Unsafe Wording Examples

Do not use these phrases in public copy:

- "production-ready Triple Ratchet"
- "proven true Triple Ratchet"
- "quantum-proof"
- "metadata-free"
- "anonymity"
- "production deployment ready"

These phrases either outrun current repo evidence, imply external review or production assurance that is not present, or confuse metadata minimization with metadata elimination.

## External Definition Summary

External references are used only to define terms. They are not proof of QSL implementation status.

- Signal Double Ratchet specification: the classical Double Ratchet combines an elliptic-curve ratchet with symmetric KDF chains to derive message keys and recover after compromise under classical assumptions.
- Signal Sparse Post-Quantum Ratchet / Triple Ratchet description: the post-quantum ratchet uses a Sparse Continuous Key Agreement (SCKA) to produce ordered epoch secrets, and the Triple Ratchet combines an EC Double Ratchet and Sparse Post-Quantum Ratchet in parallel before deriving the final encryption key.
- Signal ML-KEM Braid specification: ML-KEM Braid is an SCKA using ML-KEM, with ordered epochs and send/receive state-machine transitions designed for bandwidth-limited post-quantum continuous key agreement.
- NIST FIPS 203: ML-KEM is the standardized module-lattice key-encapsulation mechanism with parameter sets ML-KEM-512, ML-KEM-768, and ML-KEM-1024.

QSL may use "Triple-Ratchet-style" because the repo evidence maps to a hybrid EC-plus-PQ ratcheting architecture. QSL must not claim equivalence to Signal's production protocol, Signal's implementation, or Signal's formal/security proofs.

## QSL Evidence Map

| Claim area | Current QSL evidence | Classification | Boundary |
| --- | --- | --- | --- |
| EC/classical message-key chain | [GOALS.md](../../GOALS.md) G1 defines `ec_mk` from the classical Double Ratchet chain and [TRACEABILITY.md](../../TRACEABILITY.md) maps G1 to Suite-2 ratchet code, vectors, and `KDF_HYBRID` composition. | SUPPORTED | Architecture and conformance evidence only; not production release proof. |
| PQ per-message chain | [GOALS.md](../../GOALS.md) G1 requires `pq_mk` from a per-direction PQ chain that advances every message. [TRACEABILITY.md](../../TRACEABILITY.md) maps this to Suite-2 vectors including KDF, PQ reseed, boundary, OOO/replay, E2E receive, crash/restart, and interop categories. | SUPPORTED | Supports per-message hybrid design wording, not "quantum-proof". |
| SCKA sparse reseed / epoch evidence | D-0445 and [NA-0240 evidence](../governance/evidence/NA-0240_scka_persistence_monotonicity_audit.md) record executable SCKA persistence, monotonicity, rollback, tombstone, one-time consumption, and no-mutation coverage. | SUPPORTED | SCKA evidence is bounded to QSL's current model/refimpl paths. |
| KDF_HYBRID / per-message hybrid key evidence | [GOALS.md](../../GOALS.md) G1 requires `mk = KDF_HYBRID(ec_mk, pq_mk)`. [TRACEABILITY.md](../../TRACEABILITY.md) maps G1 to Suite-2 KDF and hybrid message-key vectors. | SUPPORTED | Supports "hybrid per-message key derivation" wording. |
| Downgrade resistance evidence | D-0447 and [NA-0241 evidence](../governance/evidence/NA-0241_demo_downgrade_no_mutation_audit.md) add downgrade/transcript reject and no-mutation proof; [CONFORMANCE_VECTOR_PRIORITIZATION.md](../conformance/CONFORMANCE_VECTOR_PRIORITIZATION.md) keeps downgrade vectors as a priority class. | SUPPORTED | Fail-closed downgrade evidence exists, but further formal/model expansion remains a release gap. |
| KT evidence | D-0440 records PR #708 fail-closed KT verifier implementation. D-0449 and [NA-0242 evidence](../governance/evidence/NA-0242_kt_consistency_no_mutation_audit.md) add accepted-state no-mutation proof for rejected KT advancement and related rejects. | SUPPORTED | Supports "KT verifier evidence exists"; demo KT-negative readiness is not yet claimed. |
| No-state-mutation evidence | D-0445, D-0447, D-0449, D-0452, and evidence reports for NA-0240 through NA-0243 cover SCKA, downgrade/capability, KT, skipped-key, and receive/decrypt reject no-mutation proofs. | SUPPORTED | Coverage is strong for named paths, not universal proof over all future code. |
| Metadata conformance evidence | [DOC-G5-001](../privacy/DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md), [DOC-G5-003](../privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md), D-0454, and [NA-0244 evidence](../governance/evidence/NA-0244_metadata_conformance_negative_expansion_audit.md) define residual metadata leakage and executable negative checks. | SUPPORTED with disclaimer | Supports metadata minimization language only; no anonymity or metadata-free claim. |
| Demo acceptance evidence | [DEMO_ACCEPTANCE_CRITERIA.md](../demo/DEMO_ACCEPTANCE_CRITERIA.md), D-0458, and [NA-0246 evidence](../governance/evidence/NA-0246_one_command_demo_acceptance_audit.md) prove one-command loopback positive flow plus bounded negative rejects. | PARTIALLY_SUPPORTED | Demo is non-production and does not prove KT-negative, attachment, or deployment readiness. |
| Desktop GUI readiness evidence | D-0460 and [NA-0247 evidence](../governance/evidence/NA-0247_desktop_gui_public_demo_readiness_audit.md) validate existing desktop contract tests, protocol-inactive gate proof, frontend build, sidecar prep, and host-limited package notes. | PARTIALLY_SUPPORTED | Guided prototype readiness only; no production GUI or native package proof on this host. |

## Current Claim Classification

SUPPORTED:

- Triple-Ratchet-style Suite-2 research architecture.
- Always-hybrid per-message key design as defined in G1.
- SCKA sparse reseed / epoch evidence for current QSL model and refimpl paths.
- Fail-closed downgrade resistance evidence for the covered vector/test paths.
- KT verifier evidence and selected KT no-mutation rejects.
- No-state-mutation evidence for the named SCKA, downgrade, KT, skipped-key, and receive/decrypt paths.
- Metadata minimization work with explicit residual leakage.

PARTIALLY_SUPPORTED:

- Production-stability readiness.
- External-review readiness.
- Public demo readiness.
- Desktop GUI guided-demo readiness.

These areas have meaningful evidence but still require release-gate completion, broader reproducibility, external review packaging, and documented host/platform proof.

UNSUPPORTED:

- Production-ready claims.
- Proven true Triple Ratchet claims.
- "Quantum-proof" claims.
- Metadata-free, anonymity, untraceable, or full traffic-analysis-resistance claims.
- Deployment-ready or production deployment ready claims.
- Claims that QSL implements or inherits Signal's production protocol or Signal's formal proofs.

## Public Wording Rules

Safe wording:

- "Suite-2 is a research-stage Triple-Ratchet-style hybrid messaging design."
- "QSL has evidence-backed Suite-2 ratchet, SCKA, downgrade, KT, no-mutation, metadata, demo, and GUI readiness artifacts."
- "Suite-2 combines classical and PQ message-key material in a hybrid per-message design."
- "Current evidence is research/demo oriented and release-gated."

Requires disclaimer:

- "post-quantum" when used for Suite-2 must be scoped to algorithm/design intent and current evidence, not absolute security.
- "Triple Ratchet" must be paired with "style", "research-stage", or "inspired architecture" unless release evidence later authorizes stronger wording.
- "metadata minimization" must state residual timing, size, stable-id, relay-observer, and IP-level metadata remain observable in the current demo profile.
- "demo ready" must state non-production, loopback/default boundaries, and unsupported KT-negative/attachment/native-package gaps where relevant.

Prohibited wording:

- production-ready Triple Ratchet
- proven true Triple Ratchet
- quantum-proof
- metadata-free
- anonymity
- production deployment ready
- anonymous messaging
- untraceable communications
- metadata eliminated
- Signal-equivalent security proof

## Release-Readiness Gaps

- Formal verification expansion: model downgrade resistance and no-state-mutation reject invariants beyond current SCKA coverage.
- External review package: prepare a compact, self-contained reviewer bundle with scope, specs, vectors, evidence, and limitations.
- Conformance harness reproducibility: make vector execution reproducible across local Linux, CI Linux, and macOS where relevant.
- Demo KT-negative readiness: add only when the public demo surface carries truthful KT evidence.
- Attachment demo readiness: prove descriptor validation, fetch/decrypt, integrity checks, and negative rejects before claiming attachment demo support.
- Metadata phase-2 roadmap: define future identifier rotation, padding/batching/jitter, retention, and error-normalization work without implying anonymity.

## Website / Public-Copy Implications

Website and public copy must use this document with [WEBSITE_CLAIM_MATRIX.md](WEBSITE_CLAIM_MATRIX.md) and [WEBSITE_UPDATE_PLAN.md](WEBSITE_UPDATE_PLAN.md).

Before adding Suite-2 / Triple-Ratchet wording to a public page:

1. Use the safe short wording exactly or a narrower variant.
2. Link or reference repo evidence for the specific claim.
3. Include a research-stage / non-production disclaimer.
4. Keep live product/service claims separate from qsl-protocol evidence.
5. Avoid production, anonymity, metadata-free, quantum-proof, or proven-Triple-Ratchet wording.

## References

Repo references:

- [GOALS.md](../../GOALS.md)
- [ROADMAP.md](../../ROADMAP.md)
- [CONFORMANCE_VECTOR_PRIORITIZATION.md](../conformance/CONFORMANCE_VECTOR_PRIORITIZATION.md)
- [DEMO_ACCEPTANCE_CRITERIA.md](../demo/DEMO_ACCEPTANCE_CRITERIA.md)
- [WEBSITE_CLAIM_MATRIX.md](WEBSITE_CLAIM_MATRIX.md)
- [WEBSITE_UPDATE_PLAN.md](WEBSITE_UPDATE_PLAN.md)
- [DOC-G5-001 Metadata Threat Model](../privacy/DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md)
- [DOC-G5-003 Envelope/Transport Profile](../privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md)
- [TRACEABILITY.md](../../TRACEABILITY.md)
- [DECISIONS.md](../../DECISIONS.md)
- [NA-0248 evidence audit](../governance/evidence/NA-0248_suite2_triple_ratchet_evidence_audit.md)

External references:

- Signal, The Double Ratchet Algorithm: <https://signal.org/docs/specifications/doubleratchet/>
- Signal, The ML-KEM Braid Protocol: <https://signal.org/docs/specifications/mlkembraid/>
- NIST FIPS 203, Module-Lattice-Based Key-Encapsulation Mechanism Standard: <https://csrc.nist.gov/pubs/fips/203/final>
