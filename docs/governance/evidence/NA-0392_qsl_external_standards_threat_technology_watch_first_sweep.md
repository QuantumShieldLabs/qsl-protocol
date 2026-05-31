Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-30

# NA-0392 QSL External Standards / Threat / Technology Watch First Source-Cited Sweep

Goals: G1, G2, G3, G4, G5

Directive: QSL-DIR-2026-05-31-211

Host clock note: the director-declared directive date is 2026-05-31. The local host clock used for file metadata during execution reported 2026-05-30 America/Chicago and 2026-05-31 UTC. Source access dates below use 2026-05-31 UTC.

## Executive Summary

NA-0392 completed the first bounded, source-cited external standards / threat / technology watch sweep authorized by NA-0391. The sweep covered PQC standards and migration, IETF/CFRG protocol evolution, Rust/advisory/dependency health, code and crypto research venues, secure messaging and metadata privacy, backup/restore/key custody, external review/public-claim/disclosure references, and bounded adjacent public narrative context.

No official CRITICAL or HIGH immediate blocker was identified for active qsl-protocol code or dependencies during this first bounded sweep. The result is not an external review, not production readiness evidence, not a public-internet readiness claim, not metadata-free proof, not anonymity proof, not untraceability proof, not bug-free proof, not perfect-crypto proof, not disaster recovery completion, and not off-host backup completion.

Selected successor:

`NA-0393 -- QSL External Standards / Threat Watch Findings Triage and Queue Candidate Plan`

## Live NA-0392 Scope

Live `NEXT_ACTIONS.md` listed NA-0392 as READY with objective to perform a first bounded, read-only, source-cited external standards / threat / technology watch sweep using the NA-0391 taxonomy, citation policy, source tiers, stability classifications, and public-claim boundaries.

Allowed qsl-protocol mutation in this packet is limited to governance evidence, a testplan, `DECISIONS.md`, `TRACEABILITY.md`, and the rolling operations journal. No runtime, service, protocol, crypto, dependency, workflow, public docs, website, backup script/timer/fstab, qsl-server, qsl-attachments, qshield runtime, or response archive mutation is authorized.

Acceptance criteria:

- READY count remains exactly one until optional closeout.
- NA-0392 remains READY until closeout.
- NA-0391 remains DONE.
- D-0764 and D-0765 remain present once.
- D-0766 is added once by this sweep.
- Source citations include URL, publisher/authority, source tier, access date, stability classification, category, and relevance.
- Drafts, preprints, and vendor claims are not treated as final standards.
- Findings do not automatically promote any READY successor.

## Inherited NA-0391 Authorization

NA-0391 authorized this first source-cited sweep using:

- Tier 1: final standards, RFCs, official guidance, official advisories.
- Tier 2: official project security/release notes, RustSec, GitHub advisories, NVD/CVE.
- Tier 3: peer-reviewed research and high-quality conference material.
- Tier 4: preprints, drafts, and working-group drafts.
- Tier 5: vendor blogs, competitor claims, news, and marketing.

NA-0391 also required public-claim boundaries:

- Source discovery must not be presented as external review.
- Source discovery must not be presented as production readiness.
- Drafts and preprints must not be presented as final standards.
- Vendor and competitor claims must not be treated as primary technical evidence.
- Public paper drafting remains future-gated.

## Source-Cited Sweep Method

Categories:

- PQC standards and migration.
- IETF/CFRG/protocol standards and drafts.
- Rust/advisory/dependency health.
- Code/crypto research watch.
- Secure messaging and metadata privacy.
- Backup/restore/key custody.
- External review/public claim/disclosure.
- Bounded adjacent/competitor/public narrative context.

Stability labels:

- `FINAL_STANDARD`
- `OFFICIAL_GUIDANCE`
- `RFC`
- `INTERNET_DRAFT`
- `ADVISORY_FEED`
- `PEER_REVIEWED_RESEARCH`
- `PREPRINT`
- `PROJECT_RELEASE_NOTES`
- `VENDOR_CLAIM_LOW_CONFIDENCE`

Relevance scoring:

- `DIRECT`: source directly affects QSL protocol/security claims, implementation standards, dependency/advisory posture, backup/restore/key custody, or external-review claims.
- `SUPPORTING`: source informs a future lane or claim boundary.
- `CONTEXT`: source gives watch context but does not justify implementation or public claims.

QSL impact scoring:

- `BLOCKER`: immediate official CRITICAL/HIGH issue affecting active code/dependency or required governance evidence.
- `TRIAGE_REQUIRED`: credible finding should become an explicit queue-candidate review item.
- `CLAIM_BOUNDARY`: finding constrains public/readiness/privacy wording.
- `WATCH`: monitor for standardization, advisory, or research movement.
- `NO_ACTION_NOW`: no immediate governance action beyond citation.

Queue-candidate rules:

- No source finding auto-promotes READY.
- Official CRITICAL/HIGH advisory affecting active qsl-protocol code or dependencies can justify a blocker successor.
- Standards updates can justify migration/readiness planning lanes.
- Drafts, preprints, and vendor claims can justify watch or triage, not final implementation/readiness claims.

Storage and backup policy:

- This file is durable qsl-protocol governance evidence.
- Optional temporary proof may live under `/srv/qbuild/tmp/NA0392_external_watch_*`.
- No durable external-watch report is created outside authorized governance evidence.
- No backup-plan update is required for this governance-only evidence packet.
- Future durable watch report stores require separate backup-impact review.

## PQC Standards / Migration Sweep

Key finding: NIST FIPS 203, FIPS 204, and FIPS 205 are final PQC standards for ML-KEM, ML-DSA, and SLH-DSA, respectively. These final standards provide high-trust references for future QSL PQC planning, but they do not prove QSL implementation compliance or deployment readiness.

Migration guidance exists from NIST/NCCoE, NCSC, CISA/NCCoE, and NSA CNSA 2.0. These sources support migration planning, inventory, prioritization, and claim discipline. They do not authorize QSL to claim standards compliance or production readiness without exact implementation, test, validation, and review evidence.

HQC status: NIST selected HQC as a future backup KEM algorithm and identifies FIPS 206 as in development. HQC is therefore a watch item, not a final QSL implementation claim.

Queue candidates:

- PQC evidence triage: map QSL current crypto claims against FIPS 203/204/205 and explicit non-claims.
- PQC migration planning: inventory where final standards, drafts, and hybrid guidance may affect future QSL design.
- HQC watch: monitor FIPS 206 progress without treating it as final until NIST finalizes it.

Public-claim caution: QSL must not claim PQC standards compliance, quantum-proof behavior, implementation readiness, or migration completion from this source sweep.

## IETF / CFRG / Protocol Standards Sweep

RFC 8446, RFC 9180, and RFC 9420 are stable RFC references for TLS 1.3, HPKE, and MLS. They are stronger evidence than Internet-Drafts and should be preferred when anchoring future protocol discussions.

Relevant PQ/hybrid IETF and CFRG work remains in Internet-Draft form, including TLS hybrid design, HPKE post-quantum KEMs, CFRG hybrid KEMs, and MLS post-quantum ciphersuites. These drafts are useful awareness inputs and possible future design inputs, but they must not be treated as final standards.

Queue candidates:

- Protocol standards watch: track whether current IETF/CFRG PQ/hybrid drafts advance to RFC or materially change.
- Claim-boundary audit: separate stable RFC references from experimental or draft-only PQ/hybrid protocol statements.

Public-claim caution: QSL should base public implementation claims only on stable standards or explicit experimental caveats supported by exact evidence.

## Rust / Advisory / Dependency Health Sweep

RustSec, RustSec advisory DB, GitHub Security Advisories, NVD/CVE, CISA KEV, and upstream crypto/security project security pages are relevant to ongoing dependency/advisory health. Local `cargo audit --deny warnings` remained green during NA-0392, and `cargo tree -i rustls-webpki --locked` reported `rustls-webpki v0.103.13`.

This sweep did not change dependencies, Cargo files, or lockfiles. Future watch should cross-reference:

- local `cargo audit --deny warnings`;
- dependency tree evidence for security-sensitive crates;
- RustSec advisory identifiers;
- GitHub advisory identifiers;
- NVD/CVE records;
- CISA KEV entries when applicable;
- upstream official security/release notes for crypto/security projects.

Advisory trigger policy:

- Official CRITICAL/HIGH advisory affecting active qsl-protocol code or dependencies should trigger immediate blocker analysis.
- Lower-severity or non-applicable advisory items should become explicit triage candidates, not automatic implementation changes.
- Any dependency update remains a separate authorized implementation lane.

## Code / Crypto Research Watch Sweep

IACR ePrint, Real World Crypto, USENIX Security, IEEE S&P, ACM CCS, NDSS, and selected arXiv preprints provide research watch coverage. NA-0392 used these as category anchors, not a full literature review.

Research watch categories for future triage:

- PQC migration/hybrid composition pitfalls.
- Protocol proof gaps and downgrade/fallback risks.
- Implementation side-channel risks.
- Secure messaging metadata privacy and traffic-analysis risks.
- Backup/key custody threat modeling.
- Formal methods and cryptographic implementation review practices.

Peer-reviewed venue material should be distinguished from preprints. Preprints and working drafts can motivate watch or future audit questions, but they do not justify public readiness claims or immediate security-semantics changes without separate review.

## Secure Messaging / Metadata Privacy Watch Sweep

Signal PQXDH, Signal Double Ratchet, and Signal sealed sender provide high-signal secure messaging references. XRD and related metadata-private messaging research provide watch context for privacy and traffic-analysis tradeoffs.

QSL implications:

- Do not claim metadata-free behavior.
- Do not claim anonymity.
- Do not claim untraceability.
- Treat traffic shape, timing, sender metadata, batching, padding, cover traffic, and delivery architecture as separate evidence lanes.
- Secure messaging protocol references do not prove QSL implementation security or external review completion.

Queue candidates:

- Metadata privacy claim-boundary review.
- Secure messaging research watch triage.
- Future evidence lane for traffic-analysis assumptions if QSL public claims approach that topic.

## Backup / Restore / Key Custody Watch Sweep

restic, Borg, rclone, age, GnuPG, and OpenSSH official/security sources remain relevant external references for backup, encryption, restore, key custody, and operational security watch. NA-0392 did not alter backup scripts, timers, fstab entries, service units, keys, passphrases, restore paths, recovery envelopes, remote destinations, or monitoring configuration.

QSL implications:

- Same-host continuity is not disaster recovery.
- Off-host encrypted backup remains incomplete unless exact future evidence proves it.
- Restore proof remains incomplete unless future exact restore-drill evidence exists.
- Key custody/recovery implementation remains future-gated.

Queue candidates:

- Off-host backup/key/restore lane prerequisite review.
- Backup source-list and durable external-watch report coverage review if future reports move outside tracked governance evidence.

## External Review / Public Claim / Disclosure Watch Sweep

CISA coordinated vulnerability disclosure guidance, OpenSSF vulnerability disclosure resources, and OWASP ASVS are credible references for future vulnerability disclosure, assurance, and public-claim readiness planning.

QSL implications:

- External-review-complete is not claimed.
- Source discovery is not external review.
- Public technical paper work remains future-gated until standards watch, protocol standards watch, code/crypto audit status, public-claim boundary audit, external-review readiness, service production boundaries, and backup/restore/key/off-host status are refreshed.
- Website and public docs are not changed by NA-0392.

Queue candidates:

- Public claim readiness checklist.
- Vulnerability disclosure readiness lane.
- External review readiness criteria lane.

## Adjacent / Competitor / Public Narrative Context

NA-0392 treated adjacent public narrative context as bounded and low-confidence unless backed by primary technical sources. Vendor or competitor claims were not used as technical evidence and did not drive implementation decisions.

Claim patterns to avoid:

- Treating marketing language as protocol evidence.
- Treating source discovery as third-party review.
- Treating a standards watch as implementation compliance.
- Treating local harness or service-local evidence as public-internet proof.

## Findings Matrix and Queue Candidate Analysis

| Finding ID | Domain | Source tier / stability | Severity | QSL relevance | Affected lane | Current QSL evidence status | Public-claim implication | Recommended action | Proposed future candidate | Blocker? | Rationale |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-0392-01 | PQC final standards | Tier 1 / `FINAL_STANDARD` | CLAIM_BOUNDARY | DIRECT | PQC standards | FIPS 203/204/205 final sources cited; no QSL compliance proof | Do not claim PQC standards compliance | Triage exact claims and evidence gaps | PQC standards evidence triage | No | Final standards exist, but implementation compliance is unproven here. |
| F-0392-02 | PQC migration | Tier 1 / `OFFICIAL_GUIDANCE` | BACKLOG_CANDIDATE | DIRECT | Migration planning | Official migration guidance cited | Do not claim migration completion | Create migration planning candidate | PQC migration readiness plan | No | Guidance supports planning, not readiness claims. |
| F-0392-03 | HQC backup KEM | Tier 1 / `OFFICIAL_GUIDANCE` | WATCH | SUPPORTING | PQC watch | NIST selection and FIPS 206 development status cited | Do not treat HQC as final QSL standard | Monitor FIPS 206 progress | HQC/FIPS 206 watch item | No | Official source indicates future standardization path. |
| F-0392-04 | IETF/CFRG PQ/hybrid drafts | Tier 4 / `INTERNET_DRAFT` | WATCH | SUPPORTING | Protocol standards | Drafts identified | Drafts are not final standards | Track draft maturity and changes | Protocol PQ/hybrid draft watch | No | Useful awareness input, not production claim basis. |
| F-0392-05 | Rust/dependency advisory health | Tier 2 / `ADVISORY_FEED` | INFO | DIRECT | Dependency health | Local cargo audit green; advisory sources mapped | No dependency safety overclaim | Keep cargo audit and advisory cross-reference | Advisory-trigger policy lane | No | No active CRITICAL/HIGH blocker found in this sweep. |
| F-0392-06 | Upstream crypto/security projects | Tier 2 / `PROJECT_RELEASE_NOTES` | WATCH | SUPPORTING | Dependency/project watch | Security pages identified | Do not imply QSL inherits all upstream assurance | Maintain watch mapping | Upstream security notes watch | No | Official project notes are useful but need applicability review. |
| F-0392-07 | Research venues | Tier 3/4 / mixed | BACKLOG_CANDIDATE | SUPPORTING | Code/crypto audit | Research categories identified | Do not claim external review | Define triage filters | Research watch triage | No | Research watch is broad and needs candidate selection. |
| F-0392-08 | Metadata privacy | Tier 1/3 / mixed | CLAIM_BOUNDARY | DIRECT | Public claims / messaging | Secure messaging references cited | Metadata-free, anonymity, and untraceability remain NOT CLAIMED | Future metadata privacy claim review | Metadata privacy evidence lane | No | Sources show complexity; QSL evidence is not sufficient for those claims. |
| F-0392-09 | Backup/restore/key custody | Tier 2 / `PROJECT_RELEASE_NOTES` and official docs | CLAIM_BOUNDARY | DIRECT | Local ops / backup | Same-host continuity caveat remains | Disaster recovery/off-host/restore/key custody remain NOT CLAIMED | Keep backup lanes gated | Backup/key/restore prerequisite review | No | External sources inform planning; no backup mutation occurred. |
| F-0392-10 | Disclosure/external review | Tier 1/2 / `OFFICIAL_GUIDANCE` | BACKLOG_CANDIDATE | DIRECT | External review / public claim | Guidance cited; no external review performed | External-review-complete remains NOT CLAIMED | Build future readiness criteria | CVD/public-claim readiness lane | No | Guidance supports future process, not current review completion. |

## Report / Storage / Backup Impact

This evidence document is durable qsl-protocol governance evidence. Temporary proof may remain under `/srv/qbuild/tmp/NA0392_external_watch_*`. No durable external-watch report was created outside authorized governance evidence.

Backup impact classification: no NA-0392 backup-plan update is required because durable changes are limited to tracked qsl-protocol governance/evidence/testplan/traceability/journal files and optional proof remains temporary under `/srv/qbuild/tmp`. Future recurring reports or durable external-watch stores outside the repository require separate backup-impact review.

Same-host continuity caveat remains active. Local backup status must not be presented as complete disaster recovery.

## Public Technical Paper Implications

Future public technical paper work should be supported by refreshed:

- PQC standards watch.
- Protocol standards watch.
- Code/crypto audit status.
- Public-claim boundary audit.
- External-review readiness status.
- Service production boundary status.
- Backup/restore/key/off-host status.

NA-0392 does not draft a public paper, does not create public docs, and does not claim readiness.

## Selected Successor

Selected:

`NA-0393 -- QSL External Standards / Threat Watch Findings Triage and Queue Candidate Plan`

Rationale: source discovery succeeded, report/citation/storage policy did not block evidence, and no official CRITICAL/HIGH immediate blocker affecting active qsl-protocol code or dependencies was identified in this first bounded sweep. The next work should triage findings into explicit queue candidates, claim-boundary updates, evidence gaps, and future directive recommendations.

Rejected alternatives:

- `NA-0393 -- QSL External Standards / Threat Watch Critical Finding Blocker Resolution`: rejected because no immediate official CRITICAL/HIGH active-code/dependency blocker was found.
- `NA-0393 -- QSL External Standards Watch Report Storage / Citation Policy Blocker Resolution`: rejected because storage and citation policy allowed safe governance evidence.

## Public-Claim Boundary

NA-0392 does not claim:

- production readiness is not claimed;
- public-internet readiness is not claimed;
- external-review completion is not claimed;
- metadata-free behavior is not claimed;
- anonymity is not claimed;
- untraceability is not claimed;
- bug-free status is not claimed;
- perfect crypto is not claimed;
- disaster recovery completion is not claimed;
- off-host backup completion is not claimed;
- restore proof completion is not claimed;
- key custody implementation is not claimed.

## Source List

Access date for all rows: 2026-05-31 UTC.

| Source title | Publisher / authority | URL | Watch category | Tier | Stability | Relevance |
|---|---|---|---|---|---|---|
| FIPS 203 Module-Lattice-Based Key-Encapsulation Mechanism Standard | NIST CSRC | https://csrc.nist.gov/pubs/fips/203/final | PQC standards | Tier 1 | `FINAL_STANDARD` | Final ML-KEM reference; does not prove QSL compliance. |
| FIPS 204 Module-Lattice-Based Digital Signature Standard | NIST CSRC | https://csrc.nist.gov/pubs/fips/204/final | PQC standards | Tier 1 | `FINAL_STANDARD` | Final ML-DSA reference; future claim anchor only with exact evidence. |
| FIPS 205 Stateless Hash-Based Digital Signature Standard | NIST CSRC | https://csrc.nist.gov/pubs/fips/205/final | PQC standards | Tier 1 | `FINAL_STANDARD` | Final SLH-DSA reference; future claim anchor only with exact evidence. |
| Migration to Post-Quantum Cryptography | NIST NCCoE | https://www.nccoe.nist.gov/applied-cryptography/migration-to-pqc | PQC migration | Tier 1 | `OFFICIAL_GUIDANCE` | Supports inventory and migration planning. |
| PQC Migration Timelines | UK NCSC | https://www.ncsc.gov.uk/guidance/pqc-migration-timelines | PQC migration | Tier 1 | `OFFICIAL_GUIDANCE` | Timeline guidance for migration planning and claim discipline. |
| Quantum Readiness: Migration to Post-Quantum Cryptography Fact Sheet | CISA, NSA, and NIST/NCCoE | https://www.nccoe.nist.gov/publications/fact-sheet/quantum-readiness-migration-post-quantum-cryptography-fact-sheet | PQC migration | Tier 1 | `OFFICIAL_GUIDANCE` | Official migration readiness context. |
| CNSA Suite 2.0 Algorithm Requirements | NSA | https://www.nsa.gov/serve-from-netstorage/Press-Room/Press-Releases-Statements/Press-Release-View/Article/3148990/nsa-releases-future-quantum-resistant-qr-algorithm-requirements-for-national-se/index.html | PQC migration | Tier 1 | `OFFICIAL_GUIDANCE` | Government algorithm-transition reference. |
| NIST Selects HQC as Fifth Algorithm for Post-Quantum Encryption | NIST | https://www.nist.gov/news-events/news/2025/03/nist-selects-hqc-fifth-algorithm-post-quantum-encryption | PQC standards watch | Tier 1 | `OFFICIAL_GUIDANCE` | HQC backup-KEM watch item; not final QSL implementation evidence. |
| Post-Quantum Cryptography Standardization | NIST CSRC | https://csrc.nist.gov/projects/post-quantum-cryptography/post-quantum-cryptography-standardization | PQC standards watch | Tier 1 | `OFFICIAL_GUIDANCE` | Tracks FIPS 203/204/205 and in-development items. |
| NIST IR 8610 status report | NIST | https://nvlpubs.nist.gov/nistpubs/ir/2026/NIST.IR.8610.pdf | PQC standards watch | Tier 1 | `OFFICIAL_GUIDANCE` | Official status context for additional signature standardization. |
| RFC 8446 TLS 1.3 | IETF RFC Editor | https://www.rfc-editor.org/rfc/rfc8446 | Protocol standards | Tier 1 | `RFC` | Stable TLS 1.3 reference. |
| RFC 9180 HPKE | IETF RFC Editor | https://www.rfc-editor.org/info/rfc9180 | Protocol standards | Tier 1 | `RFC` | Stable HPKE reference. |
| RFC 9420 MLS | IETF RFC Editor | https://www.rfc-editor.org/rfc/rfc9420 | Protocol standards | Tier 1 | `RFC` | Stable MLS reference. |
| TLS Hybrid Key Exchange Design | IETF Datatracker | https://datatracker.ietf.org/doc/draft-ietf-tls-hybrid-design/ | Protocol drafts | Tier 4 | `INTERNET_DRAFT` | Draft awareness input; not a final standard. |
| Post-Quantum KEMs for HPKE | IETF Datatracker | https://datatracker.ietf.org/doc/draft-ietf-hpke-pq/ | Protocol drafts | Tier 4 | `INTERNET_DRAFT` | Draft awareness input for HPKE PQ work. |
| Hybrid Public-Key Encryption | IRTF CFRG Datatracker | https://datatracker.ietf.org/doc/draft-irtf-cfrg-hybrid-kems/ | Protocol drafts | Tier 4 | `INTERNET_DRAFT` | Draft hybrid-KEM awareness input. |
| Post-Quantum Ciphersuites for MLS | IETF Datatracker | https://datatracker.ietf.org/doc/draft-ietf-mls-pq-ciphersuites/ | Protocol drafts | Tier 4 | `INTERNET_DRAFT` | Draft MLS PQ awareness input. |
| RustSec | RustSec project | https://rustsec.org/ | Advisory ecosystem | Tier 2 | `ADVISORY_FEED` | Rust advisory source for future dependency checks. |
| RustSec Advisory Database | RustSec project | https://github.com/RustSec/advisory-db | Advisory ecosystem | Tier 2 | `ADVISORY_FEED` | Advisory records used by cargo-audit ecosystem. |
| GitHub Advisory Database | GitHub | https://github.com/advisories | Advisory ecosystem | Tier 2 | `ADVISORY_FEED` | Cross-check source for dependency advisories. |
| NVD Vulnerability APIs | NIST NVD | https://nvd.nist.gov/developers/vulnerabilities | Advisory ecosystem | Tier 2 | `ADVISORY_FEED` | CVE/NVD source for advisory cross-reference. |
| Known Exploited Vulnerabilities Catalog | CISA | https://www.cisa.gov/known-exploited-vulnerabilities-catalog | Advisory ecosystem | Tier 1 | `ADVISORY_FEED` | Official exploited-vulnerability source. |
| OpenSSL News and Advisories Timeline | OpenSSL project | https://openssl-library.org/news/timeline/index.html | Upstream security notes | Tier 2 | `PROJECT_RELEASE_NOTES` | Official OpenSSL release/security watch source. |
| OpenSSH Security | OpenSSH project | https://www.openssh.org/security.html | Upstream security notes | Tier 2 | `PROJECT_RELEASE_NOTES` | Official OpenSSH security watch source. |
| GnuPG Security | GnuPG project | https://www.gnupg.org/documentation/security.html | Upstream security notes | Tier 2 | `PROJECT_RELEASE_NOTES` | Official GnuPG security watch source. |
| liboqs Security | Open Quantum Safe project | https://openquantumsafe.org/liboqs/security.html | Upstream security notes | Tier 2 | `PROJECT_RELEASE_NOTES` | OQS/liboqs security watch source. |
| IACR ePrint Archive | IACR | https://www.iacr.org/eprint/ | Crypto research | Tier 4 | `PREPRINT` | Preprint watch source; not final evidence. |
| Real World Crypto 2026 | IACR | https://rwc.iacr.org/2026/ | Crypto research | Tier 3 | `PEER_REVIEWED_RESEARCH` | High-signal applied-crypto venue watch. |
| USENIX Security 2026 | USENIX | https://www.usenix.org/conference/usenixsecurity26 | Security research | Tier 3 | `PEER_REVIEWED_RESEARCH` | Peer-reviewed security venue watch. |
| IEEE Symposium on Security and Privacy 2026 | IEEE Computer Society | https://www.ieee-security.org/TC/SP2026/ | Security research | Tier 3 | `PEER_REVIEWED_RESEARCH` | Peer-reviewed security venue watch. |
| ACM CCS 2025 | ACM SIGSAC | https://www.sigsac.org/ccs/CCS2025/overview/ | Security research | Tier 3 | `PEER_REVIEWED_RESEARCH` | Peer-reviewed security venue watch. |
| NDSS Symposium 2026 | Internet Society / NDSS | https://www.ndss-symposium.org/ndss-program/symposium-2026/ | Security research | Tier 3 | `PEER_REVIEWED_RESEARCH` | Peer-reviewed security venue watch. |
| PQXDH Specification | Signal | https://signal.org/docs/specifications/pqxdh/ | Secure messaging | Tier 2 | `PROJECT_RELEASE_NOTES` | Secure messaging PQ reference; not QSL implementation proof. |
| Double Ratchet Specification | Signal | https://signal.org/docs/specifications/doubleratchet/ | Secure messaging | Tier 2 | `PROJECT_RELEASE_NOTES` | Secure messaging protocol reference. |
| Sealed Sender | Signal | https://signal.org/blog/sealed-sender/ | Metadata privacy | Tier 5 | `VENDOR_CLAIM_LOW_CONFIDENCE` | Useful context only; not primary proof. |
| XRD: Scalable Messaging System with Cryptographic Privacy | USENIX NSDI | https://www.usenix.org/conference/nsdi20/presentation/kwon | Metadata privacy research | Tier 3 | `PEER_REVIEWED_RESEARCH` | Metadata-private messaging research context. |
| restic | restic project | https://restic.net/ | Backup/restore | Tier 2 | `PROJECT_RELEASE_NOTES` | Backup tool context for future planning. |
| Borg Security Internals | BorgBackup project | https://borgbackup.readthedocs.io/en/stable/internals/security.html | Backup/restore | Tier 2 | `PROJECT_RELEASE_NOTES` | Backup security model context. |
| rclone Security | rclone project | https://github.com/rclone/rclone/security | Backup/restore | Tier 2 | `PROJECT_RELEASE_NOTES` | Upstream project security watch source. |
| age | age project | https://age-encryption.org/ | Key custody/encryption | Tier 2 | `PROJECT_RELEASE_NOTES` | Encryption tool context for future backup/key planning. |
| Coordinated Vulnerability Disclosure Program | CISA | https://www.cisa.gov/resources-tools/programs/coordinated-vulnerability-disclosure-program | Disclosure/public claims | Tier 1 | `OFFICIAL_GUIDANCE` | Future vulnerability disclosure readiness reference. |
| OpenSSF Vulnerability Disclosures Working Group | OpenSSF | https://openssf.org/groups/vulnerability-disclosures/ | Disclosure/public claims | Tier 2 | `OFFICIAL_GUIDANCE` | Disclosure process reference. |
| OWASP Application Security Verification Standard | OWASP | https://owasp.org/www-project-application-security-verification-standard/ | Assurance/public claims | Tier 2 | `OFFICIAL_GUIDANCE` | Assurance checklist context; not QSL review proof. |

## Citation Gaps and Uncertainty

- NA-0392 is a first bounded sweep, not a complete literature review.
- Some research venues were used as watch anchors rather than paper-by-paper findings.
- Vendor and project statements are low-confidence unless backed by primary standards, advisories, RFCs, or peer-reviewed research.
- Internet-Drafts may change or expire.
- Advisory applicability requires separate dependency-specific triage.
