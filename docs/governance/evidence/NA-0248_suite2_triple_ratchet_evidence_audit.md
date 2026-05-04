Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-04
Replaces: n/a
Superseded-By: n/a

# NA-0248 Suite-2 Triple-Ratchet Evidence Audit

Directive: QSL-DIR-2026-05-04-028 / NA-0248

## Objective

Audit Suite-2 / Triple-Ratchet-style public wording against external terminology and current QSL repository evidence. This audit produces a claim boundary; it does not change protocol, runtime, crypto, demo, service, website, CI, branch-protection, public-safety, or Cargo behavior.

## Exact Commands And Tools Used

Hard-start and authority proof:

```bash
pwd
git status --porcelain=v1 --branch
git branch --show-current
git rev-parse HEAD
git diff --name-only
git ls-files --others --exclude-standard
git fetch --all --prune
git rev-parse origin/main
gh pr view 743 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 742 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 741 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 740 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 739 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 738 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 737 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 736 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 735 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 734 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 733 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 732 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 731 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 729 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 722 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 708 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh api /repos/QuantumShieldLabs/qsl-protocol/branches/main/protection/required_status_checks
gh api /repos/QuantumShieldLabs/qsl-protocol/branches/main/protection
gh api "/repos/QuantumShieldLabs/qsl-protocol/commits/${origin_main_sha}/check-runs?per_page=100"
```

Main health and parser proof:

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 <canonical queue parser>
python3 <canonical decision parser>
git show origin/main:NEXT_ACTIONS.md
```

Repo evidence discovery:

```bash
sed -n '1,240p' ROADMAP.md
sed -n '1,260p' docs/conformance/CONFORMANCE_VECTOR_PRIORITIZATION.md
sed -n '1,260p' docs/demo/DEMO_ACCEPTANCE_CRITERIA.md
sed -n '1,260p' docs/public/WEBSITE_CLAIM_MATRIX.md
sed -n '1,300p' docs/public/WEBSITE_UPDATE_PLAN.md
sed -n '1,280p' docs/privacy/DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md
sed -n '1,280p' docs/privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md
rg -n "D-0440|D-0441|D-0442|D-0443|D-0444|D-0445|D-0446|D-0447|D-0448|D-0449|D-0450|D-0451|D-0452|D-0453|D-0454|D-0455|D-0456|D-0457|D-0458|D-0459|D-0460|D-0461" DECISIONS.md
rg -n "PR #708|#708|PR #727|#727|PR #729|#729|PR #731|#731|PR #734|#734|PR #736|#736|PR #740|#740|PR #742|#742" TRACEABILITY.md
sed -n '6370,7035p' DECISIONS.md
gh pr view 708 --json number,title,state,mergedAt,mergeCommit,headRefOid,body,url
gh pr view 727 --json number,title,state,mergedAt,mergeCommit,headRefOid,body,url
gh pr view 729 --json number,title,state,mergedAt,mergeCommit,headRefOid,body,url
gh pr view 731 --json number,title,state,mergedAt,mergeCommit,headRefOid,body,url
gh pr view 734 --json number,title,state,mergedAt,mergeCommit,headRefOid,body,url
gh pr view 736 --json number,title,state,mergedAt,mergeCommit,headRefOid,body,url
gh pr view 740 --json number,title,state,mergedAt,mergeCommit,headRefOid,body,url
gh pr view 742 --json number,title,state,mergedAt,mergeCommit,headRefOid,body,url
```

External terminology lookup used the Codex Web Module for the allowed authoritative sources listed below.

## External Pages Checked

Retrieval timestamp for the external checks: 2026-05-04T11:52:53Z.

| Source | URL | Use in this audit |
| --- | --- | --- |
| Signal Double Ratchet / Sparse Post-Quantum Ratchet / Triple Ratchet specification | <https://signal.org/docs/specifications/doubleratchet/> | Defines high-level Double Ratchet, SCKA-backed Sparse Post-Quantum Ratchet, and Triple Ratchet concepts. |
| Signal ML-KEM Braid specification | <https://signal.org/docs/specifications/mlkembraid/> | Defines ML-KEM Braid as an SCKA protocol using ML-KEM, with ordered epochs and send/receive state transitions. |
| NIST FIPS 203 | <https://csrc.nist.gov/pubs/fips/203/final> | Defines ML-KEM terminology and parameter-set context. |

External text is paraphrased. External sources are not used as proof of QSL implementation status.

## Repo Evidence Consulted

- [GOALS.md](../../../GOALS.md)
- [ROADMAP.md](../../../ROADMAP.md)
- [docs/conformance/CONFORMANCE_VECTOR_PRIORITIZATION.md](../../conformance/CONFORMANCE_VECTOR_PRIORITIZATION.md)
- [docs/demo/DEMO_ACCEPTANCE_CRITERIA.md](../../demo/DEMO_ACCEPTANCE_CRITERIA.md)
- [docs/public/WEBSITE_CLAIM_MATRIX.md](../../public/WEBSITE_CLAIM_MATRIX.md)
- [docs/public/WEBSITE_UPDATE_PLAN.md](../../public/WEBSITE_UPDATE_PLAN.md)
- [docs/privacy/DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md](../../privacy/DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md)
- [docs/privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md](../../privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md)
- [TRACEABILITY.md](../../../TRACEABILITY.md), including entries for PR #708, #727, #729, #731, #734, #736, #740, and #742.
- [DECISIONS.md](../../../DECISIONS.md), entries D-0440 through D-0461.
- Recent PR summaries for #708, #727, #729, #731, #734, #736, #740, and #742.

## Claim Matrix Summary

| Claim | Evidence result | Classification |
| --- | --- | --- |
| Suite-2 is a research-stage Triple-Ratchet-style hybrid messaging design. | G1-G5 goals, traceability, recent hardening lanes, and public claim docs support this with release disclaimers. | SUPPORTED |
| Suite-2 uses always-hybrid per-message key derivation. | G1 requires EC and PQ message-key material combined by `KDF_HYBRID`; traceability maps vectors and refimpl paths. | SUPPORTED |
| SCKA sparse reseed / epoch behavior has persistence and monotonicity evidence. | D-0445 and NA-0240 evidence add SCKA restart, rollback, tombstone, one-time consumption, and model checks. | SUPPORTED |
| Downgrade resistance and no-mutation reject behavior have evidence. | D-0447, D-0449, D-0452 and NA-0241 through NA-0243 evidence cover downgrade/capability, KT, skipped-key, and receive/decrypt rejects. | SUPPORTED for covered paths |
| Metadata minimization is implemented as bounded demo/profile checks. | DOC-G5-001, DOC-G5-003, D-0454, and NA-0244 evidence support residual-leakage-aware metadata wording. | SUPPORTED with disclaimer |
| Public demo acceptance is ready. | D-0458 and NA-0246 evidence support a loopback one-command demo runner with bounded negatives. | PARTIALLY_SUPPORTED |
| Desktop GUI guided public demo readiness is ready. | D-0460 and NA-0247 evidence support contract/frontend/sidecar readiness; full native package proof is host-limited. | PARTIALLY_SUPPORTED |
| Production-ready Triple Ratchet. | ROADMAP and GOALS release gates explicitly say research-stage / non-production. | UNSUPPORTED |
| Proven true Triple Ratchet. | QSL has local evidence, not external production proof or Signal-equivalent formal proof. | UNSUPPORTED |
| Quantum-proof. | Absolute quantum-proof wording is not justified by FIPS 203 terminology or repo evidence. | UNSUPPORTED |
| Metadata-free / anonymity. | G5 docs explicitly retain residual timing, size, stable-id, relay-observer, and IP-level metadata. | UNSUPPORTED |
| Production deployment ready. | Demo, GUI, formal verification, review package, and conformance reproducibility gaps remain. | UNSUPPORTED |

## Safe Wording Examples

- Suite-2 is a research-stage Triple-Ratchet-style hybrid messaging design.
- QSL maintains evidence-backed Suite-2 ratchet, SCKA, downgrade, KT, no-mutation, metadata, demo, and GUI readiness artifacts.
- Suite-2 combines classical and PQ message-key material in a hybrid per-message design.
- Current Suite-2 evidence is research/demo oriented and release-gated.

## Unsafe Wording Examples

- production-ready Triple Ratchet
- proven true Triple Ratchet
- quantum-proof
- metadata-free
- anonymity
- production deployment ready

## Release-Readiness Gap List

- Formal verification expansion for downgrade resistance and no-state-mutation reject invariants.
- External review package with compact specs, vectors, test commands, and known limitations.
- Conformance harness reproducibility across local Linux, CI Linux, and macOS where applicable.
- Demo KT-negative readiness once the demo surface carries real KT evidence.
- Attachment demo readiness with happy-path and reject-path proof.
- Metadata phase-2 roadmap for identifier rotation, padding/batching/jitter, retention, and error-normalization work.

## No Implementation Change Statement

NA-0248 is a docs/governance claim-boundary lane only. It makes no protocol, runtime, crypto, demo, service, website implementation, public-safety helper/configuration, branch-protection, workflow, script, qsc, qsl, qsl-client, apps, tools, inputs, qsc-desktop, qsl-server, qsl-attachments, Cargo.toml, or Cargo.lock changes.

## Uncertainty Notes

- Signal external sources define terminology and external design concepts; they do not certify QSL behavior.
- QSL evidence is strong for named paths and recent PRs, but not a universal proof over every future reject path.
- ML-KEM terminology should remain tied to FIPS 203 and should not be converted into "quantum-proof" marketing wording.
- Demo KT-negative readiness, attachment demo readiness, native desktop package proof on this host, and external review readiness remain open.
- Website implementation is not performed in this lane; future website copy must consume this claim boundary explicitly.
