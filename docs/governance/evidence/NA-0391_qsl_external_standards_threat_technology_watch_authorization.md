Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-30

# NA-0391 QSL External Standards / Threat / Technology Watch Authorization

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0391 authorizes a future bounded, read-only, source-cited external
standards / threat / technology watch process. It does not perform the first
recurring watch sweep, does not create a durable watch report store, does not
change runtime, protocol, crypto, workflow, dependency, public docs, website,
backup scripts, qsl-server, qsl-attachments, qshield runtime, or response
archives, and does not expand public, readiness, privacy, or external-review
claims.

Targeted source discovery succeeded. The discovered source set is sufficient to
define a future watch process and to select the next lane:

`NA-0392 -- QSL External Standards / Threat / Technology Watch First Source-Cited Sweep`

The future first sweep should produce source-cited governance evidence plus
temporary proof output only. Durable watch-report storage, background
automation, schedulers, or local Codex ops watch directories remain future
gated and require separate backup-impact review.

Authorization decision:

`EXTERNAL_STANDARDS_THREAT_TECH_WATCH_FIRST_SWEEP_AUTHORIZATION_READY`

## Live NA-0391 scope

Live `NEXT_ACTIONS.md` recorded:

- READY_COUNT `1`.
- READY `NA-0391 -- QSL External Standards / Threat / Technology Watch Authorization Plan`.
- NA-0390 DONE.
- D-0762 exists once.
- D-0763 exists once.
- D-0764 absent at startup.
- public-safety required and green on `origin/main` `e1a2e3b781d9`.

Allowed Packet P scope for NA-0391:

- `docs/governance/evidence/NA-0391_qsl_external_standards_threat_technology_watch_authorization.md`;
- `tests/NA-0391_qsl_external_standards_threat_technology_watch_authorization_testplan.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Forbidden scope includes runtime, protocol, crypto, qsc/qsp/qsl
implementation, qshield runtime, workflows, dependencies, Cargo files,
qsl-server, qsl-attachments, qsc-desktop, website/public docs, README,
START_HERE, backup scripts/timers/fstab/services, durable external-watch
reports, durable audit reports, response/request/directive/journal archive
mutation, qstart/qresume tooling, public-safety tooling, and any secret or
off-host target handling.

## Inherited NA-0390 routine audit result

NA-0390 implemented `scripts/ci/qsl_routine_audit_cadence.py` as a bounded
temp-output helper with fixtures under
`inputs/local_ops/routine_audit_cadence_fixtures/`.

Inherited proof:

- qsl-protocol PR #1043 merge: `e1e71d8c1883`.
- qsl-protocol PR #1044 closeout merge: `e1a2e3b781d9`.
- Fixture matrix passed `42/42`.
- Temp proof root remained present during NA-0391 startup:
  `/srv/qbuild/tmp/NA0390_routine_audit_cadence_20260530T193850-0500/`.
- Combined temp-output digest remained:
  `aa6527ec11c995fe82a0a99f44011a2112563b55440c87785969ab3086c98b44`.
- External standards / threat / technology watch was represented only as a
  future-gated profile.
- NA-0390 performed no external watch, no web browsing, no scheduler, no
  workflow, no durable report, no READY mutation, no response archive mutation,
  no runtime or dependency change, and no public-claim expansion.

The inherited result supports NA-0391 because it proves the routine audit
cadence can recommend an external watch as a future candidate without
disrupting one-READY queue discipline or creating durable report storage.

## Web/source discovery method and limits

The Codex Web Module was used only for targeted, read-only source discovery and
citation collection. The discovery objective was process design, not a full
recurring external watch report.

Discovery limits:

- no code was downloaded;
- no implementation behavior was changed from external sources;
- no long copyrighted passages were copied;
- source claims were paraphrased;
- official standards and official project/security sources were preferred over
  blogs or vendor claims;
- drafts, preprints, and vendor/competitor sources are explicitly lower
  confidence than final standards, RFCs, official guidance, and official
  advisories.

Access date for the source inventory: 2026-05-31 UTC.

Citation gaps and uncertainty:

- Some project ecosystems do not expose one canonical advisory feed; future
  sweeps must combine project release notes, GitHub Security Advisories,
  RustSec, NVD/CVE, and upstream security pages.
- Internet-Drafts remain unstable until published as RFCs or otherwise adopted
  by the relevant standards body.
- Research venues and preprint servers are useful for risk discovery but do not
  by themselves justify protocol or public-claim changes.
- Vendor or competitor claims may reveal positioning patterns, but must never
  be primary implementation evidence.

## Source inventory by category

| Category | Source | Publisher / authority | URL | Tier | Stability | Cadence | Watch relevance | Claim-boundary implication |
|---|---|---|---|---|---|---|---|---|
| PQC standards | FIPS 203, Module-Lattice-Based Key-Encapsulation Mechanism Standard | NIST CSRC | https://csrc.nist.gov/pubs/fips/203/final | Tier 1 | FINAL_STANDARD | Monthly, plus errata trigger | ML-KEM baseline and errata watch | Final standard may support internal planning, not a no-risk claim |
| PQC standards | FIPS 204, Module-Lattice-Based Digital Signature Standard | NIST CSRC | https://csrc.nist.gov/pubs/fips/204/final | Tier 1 | FINAL_STANDARD | Monthly, plus errata trigger | ML-DSA baseline and errata watch | Final standard may support internal planning, not public readiness |
| PQC standards | FIPS 205, Stateless Hash-Based Digital Signature Standard | NIST CSRC | https://csrc.nist.gov/pubs/fips/205/final | Tier 1 | FINAL_STANDARD | Monthly, plus errata trigger | SLH-DSA baseline and backup signature watch | Final standard must not be described as perfect crypto |
| PQC migration | Migration to Post-Quantum Cryptography | NIST NCCoE | https://www.nccoe.nist.gov/applied-cryptography/migration-to-pqc | Tier 1 | OFFICIAL_GUIDANCE | Monthly | Migration inventory, roadmaps, and partner guidance | Supports backlog planning only |
| PQC migration | Timelines for migration to post-quantum cryptography | UK NCSC | https://www.ncsc.gov.uk/guidance/pqc-migration-timelines | Tier 1 | OFFICIAL_GUIDANCE | Monthly | External timeline and adoption trigger source | Do not convert timelines into QSL readiness claims |
| PQC migration | Quantum-Readiness: Migration to Post-Quantum Cryptography | CISA / NSA / NIST | https://www.cisa.gov/resources-tools/resources/quantum-readiness-migration-post-quantum-cryptography | Tier 1 | OFFICIAL_GUIDANCE | Monthly | Roadmap, inventory, vendor engagement | Supports blocker candidates only when QSL scope is directly affected |
| PQC migration | CNSA Suite 2.0 algorithm requirements | NSA | https://www.nsa.gov/Press-Room/Press-Releases-Statements/Press-Release-View/Article/3148990/nsa-releases-future-quantum-resistant-qr-algorithm-requirements-for-national-se/index.html | Tier 1 | OFFICIAL_GUIDANCE | Monthly | National-security-system algorithm transition context | Context only unless QSL claims NSS alignment, which is not claimed |
| IETF/RFC | RFC 8446, TLS 1.3 | RFC Editor / IETF | https://www.rfc-editor.org/rfc/rfc8446 | Tier 1 | RFC | Quarterly, plus errata trigger | TLS baseline and hybrid transition reference | RFC status does not imply QSL runtime implementation |
| IETF/RFC | RFC 9180, Hybrid Public Key Encryption | RFC Editor / IETF | https://www.rfc-editor.org/info/rfc9180 | Tier 1 | RFC | Quarterly, plus errata trigger | HPKE baseline and PQ HPKE draft dependency | RFC status supports design comparison only |
| IETF/RFC | RFC 9420, Messaging Layer Security Protocol | RFC Editor / IETF | https://www.rfc-editor.org/rfc/rfc9420 | Tier 1 | RFC | Quarterly, plus errata trigger | MLS baseline for secure group messaging | Does not prove QSL MLS compatibility |
| IETF draft | Hybrid key exchange in TLS 1.3 | IETF TLS WG Datatracker | https://datatracker.ietf.org/doc/draft-ietf-tls-hybrid-design/ | Tier 4 | INTERNET_DRAFT | Biweekly while active | Hybrid TLS design signals | Drafts may create BACKLOG candidates only |
| IETF draft | Post-Quantum and Post-Quantum/Traditional Hybrid Algorithms for HPKE | IETF HPKE WG Datatracker | https://datatracker.ietf.org/doc/draft-ietf-hpke-pq/ | Tier 4 | INTERNET_DRAFT | Biweekly while active | PQ and hybrid HPKE identifiers | Drafts must not be cited as final standards |
| IETF draft | Hybrid PQ/T Key Encapsulation Mechanisms | IRTF CFRG Datatracker | https://datatracker.ietf.org/doc/draft-irtf-cfrg-hybrid-kems/ | Tier 4 | INTERNET_DRAFT | Biweekly while active | Hybrid KEM combiner guidance | Draft guidance requires caveat and later recheck |
| IETF draft | ML-KEM and Hybrid Cipher Suites for MLS | IETF MLS WG Datatracker | https://datatracker.ietf.org/doc/draft-ietf-mls-pq-ciphersuites/ | Tier 4 | INTERNET_DRAFT | Biweekly while active | PQ MLS direction and cipher suite watch | Drafts cannot justify public claims |
| Rust advisories | RustSec Advisory Database | Rust Secure Code WG / RustSec | https://rustsec.org/ | Tier 2 | ADVISORY_FEED | Weekly, plus dependency-change trigger | Cargo advisory and `cargo audit` input | Advisory hits may justify blocker candidates |
| Rust advisories | RustSec advisory-db | RustSec GitHub | https://github.com/rustsec/advisory-db | Tier 2 | ADVISORY_FEED | Weekly | Source database for Rust crate advisories | Use for reproducible advisory evidence |
| Advisory ecosystem | GitHub Advisory Database | GitHub | https://github.com/advisories | Tier 2 | ADVISORY_FEED | Weekly, plus Dependabot/security alert trigger | OSS advisory cross-check | GHSA without upstream confirmation needs source triangulation |
| Advisory ecosystem | National Vulnerability Database and CVE APIs | NIST NVD | https://nvd.nist.gov/developers/vulnerabilities | Tier 2 | ADVISORY_FEED | Weekly, plus CVE trigger | CVE metadata and severity context | NVD lag means not the only source |
| Threat advisories | Known Exploited Vulnerabilities Catalog | CISA | https://www.cisa.gov/known-exploited-vulnerabilities-catalog | Tier 1 | ADVISORY_FEED | Weekly, plus high-impact trigger | Exploited vulnerability prioritization | Directly relevant critical items can justify blocker candidates |
| Crypto project | OpenSSL Release and Advisory Timeline | OpenSSL Library | https://openssl-library.org/news/timeline/index.html | Tier 2 | PROJECT_RELEASE_NOTES | Weekly | TLS/crypto dependency security watch | Upstream advisories outrank news reports |
| Crypto project | OpenSSH Security | OpenSSH | https://www.openssh.org/security.html | Tier 2 | PROJECT_RELEASE_NOTES | Monthly, plus CVE trigger | SSH backup/restore transport watch | Applies only to tool/ops exposure, not QSL protocol claims |
| Crypto project | GnuPG Security | GnuPG Project | https://www.gnupg.org/documentation/security.html | Tier 2 | PROJECT_RELEASE_NOTES | Monthly, plus advisory trigger | GPG/key custody ecosystem watch | Project security notes need distro/package context |
| Crypto project | liboqs Security | Open Quantum Safe | https://openquantumsafe.org/liboqs/security.html | Tier 2 | PROJECT_RELEASE_NOTES | Monthly, plus release trigger | PQ prototype library security posture | Prototype/project notes are not QSL implementation evidence |
| Crypto project | liboqs releases | Open Quantum Safe GitHub | https://github.com/open-quantum-safe/liboqs/releases | Tier 2 | PROJECT_RELEASE_NOTES | Monthly, plus release trigger | PQ algorithm availability and removals | Release notes may create watch findings only |
| Crypto project | BoringSSL advisory archive | BoringSSL / Google | https://boringssl.googlesource.com/boringssl.git/+/HEAD/docs/advisories | Tier 2 | PROJECT_RELEASE_NOTES | Monthly | TLS library advisory context | Applies only if QSL depends on affected components |
| Crypto project | RustCrypto organization | RustCrypto GitHub | https://github.com/RustCrypto | Tier 2 | PROJECT_RELEASE_NOTES | Monthly | Crypto crate upstream release and maintenance context | Use with RustSec/GHSA for security decisions |
| Research venue | Cryptology ePrint Archive | IACR | https://www.iacr.org/eprint/ | Tier 4 | PREPRINT | Monthly | Early cryptanalysis and protocol research | Preprint only until peer-reviewed or confirmed |
| Research venue | Real World Crypto 2026 | IACR RWC | https://rwc.iacr.org/2026/ | Tier 3 | PEER_REVIEWED_RESEARCH | Annual, plus program updates | Applied crypto practice watch | Conference talks require source-specific citation |
| Research venue | USENIX Security 2026 | USENIX | https://www.usenix.org/conference/usenixsecurity26 | Tier 3 | PEER_REVIEWED_RESEARCH | Annual, plus accepted-paper updates | Security/privacy systems research | Research evidence can motivate audit lanes |
| Research venue | IEEE Symposium on Security and Privacy 2026 | IEEE S&P | https://www.ieee-security.org/TC/SP2026/ | Tier 3 | PEER_REVIEWED_RESEARCH | Annual, plus program updates | Security/privacy research watch | Research findings require QSL relevance analysis |
| Research venue | ACM CCS 2025 / 2026 | ACM SIGSAC | https://www.sigsac.org/ccs/CCS2025/overview/ | Tier 3 | PEER_REVIEWED_RESEARCH | Annual | Security/privacy research watch | Research findings do not auto-change public claims |
| Research venue | NDSS Symposium 2026 Program | Internet Society NDSS | https://www.ndss-symposium.org/ndss-program/symposium-2026/ | Tier 3 | PEER_REVIEWED_RESEARCH | Annual, plus program updates | Network/security/privacy research | High-impact results require separate audit lane |
| Secure messaging | PQXDH Key Agreement Protocol | Signal | https://signal.org/docs/specifications/pqxdh/ | Tier 2 | PROJECT_RELEASE_NOTES | Quarterly, plus spec update trigger | Secure messaging PQ handshake context | Official protocol docs are not QSL external review |
| Secure messaging | Double Ratchet Algorithm specification | Signal | https://signal.org/docs/specifications/doubleratchet/ | Tier 2 | PROJECT_RELEASE_NOTES | Quarterly, plus spec update trigger | Ratchet and sparse PQ ratchet context | Compare patterns without claiming compatibility |
| Metadata privacy | Sealed sender technology preview | Signal | https://signal.org/blog/sealed-sender/ | Tier 5 | VENDOR_CLAIM_LOW_CONFIDENCE | Quarterly | Metadata-minimization claim-boundary example | Vendor blog cannot be primary evidence |
| Metadata privacy | XRD: Scalable Messaging System with Cryptographic Privacy | USENIX / NSDI | https://www.usenix.org/conference/nsdi20/presentation/kwon | Tier 3 | PEER_REVIEWED_RESEARCH | Annual | Metadata-private messaging design research | Research informs risk, not implementation claims |
| Backup/restore | restic project and security contact | restic | https://restic.net/ | Tier 2 | PROJECT_RELEASE_NOTES | Monthly, plus release trigger | Backup tool security and disclosure source | Tool evidence does not complete disaster recovery |
| Backup/restore | restic 0.18.0 release notes | restic | https://restic.net/blog/2025-03-27/restic-0.18.0-released/ | Tier 2 | PROJECT_RELEASE_NOTES | Release trigger | Backup release/security watch | Release changes require local impact review |
| Backup/restore | Borg security documentation | BorgBackup | https://borgbackup.readthedocs.io/en/stable/internals/security.html | Tier 2 | PROJECT_RELEASE_NOTES | Quarterly | Backup threat model and restore/key caveats | Tool security docs must not be overclaimed |
| Backup/restore | rclone crypt documentation | rclone | https://rclone.org/crypt/ | Tier 2 | PROJECT_RELEASE_NOTES | Monthly, plus release/advisory trigger | Remote encryption and key rotation caveats | Do not claim off-host backup complete |
| Backup/restore | rclone security policy | rclone GitHub | https://github.com/rclone/rclone/security | Tier 2 | PROJECT_RELEASE_NOTES | Monthly, plus GHSA trigger | Tool vulnerability disclosure source | Pair with GHSA/CVE for severity |
| Backup/restore | age repository and specification links | age project | https://age-encryption.org/ | Tier 2 | PROJECT_RELEASE_NOTES | Monthly, plus release trigger | File encryption and key custody watch | Tool support is not QSL key recovery proof |
| Disclosure/review | CISA Coordinated Vulnerability Disclosure Program | CISA | https://www.cisa.gov/resources-tools/programs/coordinated-vulnerability-disclosure-program | Tier 1 | OFFICIAL_GUIDANCE | Quarterly | Vulnerability intake and disclosure process | Needed before public vulnerability process claims |
| Disclosure/review | OpenSSF Best Practices Badge | OpenSSF | https://openssf.org/projects/best-practices-badge/ | Tier 2 | OFFICIAL_GUIDANCE | Quarterly | OSS project hygiene and policy checks | Badge-style evidence is not external review complete |
| Disclosure/review | OWASP ASVS | OWASP Foundation | https://owasp.org/www-project-application-security-verification-standard/ | Tier 2 | OFFICIAL_GUIDANCE | Quarterly | Security verification requirement taxonomy | Web/app standard only where applicable |

## Source quality / trust model

Tier 1: final standards, RFCs, official government guidance, and official
advisory sources.

- May justify a future blocker candidate when the finding is directly relevant,
  current, and materially affects QSL security posture.
- May justify updating audit prerequisites, but not automatic READY promotion.
- Examples: NIST FIPS, RFC Editor pages, NCSC guidance, CISA KEV, CISA/NSA/NIST
  guidance.

Tier 2: official project security pages, release notes, RustSec, GitHub
Security Advisories, NVD/CVE, and upstream package feeds.

- May justify blocker candidates for affected QSL dependencies, tools, or
  directly used components.
- Requires local impact review before any dependency or implementation change.
- If NVD/CVE and upstream project data conflict, record both and prefer the
  upstream maintainer for affected-version/fix details while using NVD/CVE for
  standardized tracking.

Tier 3: peer-reviewed research, accepted conference papers, and high-quality
conference material.

- May justify BACKLOG audit candidates, threat-model updates, or research
  follow-up.
- May justify blocker candidates only if independently corroborated, directly
  exploitable against QSL assumptions, and not dependent on speculative or
  unpublished artifacts.

Tier 4: Internet-Drafts, IRTF drafts, working-group drafts, and preprints.

- May justify BACKLOG candidates and monitoring.
- Must be labeled `INTERNET_DRAFT` or `PREPRINT`.
- Must not be cited as final standards.
- Must be rechecked before any implementation lane or public claim.

Tier 5: vendor blogs, competitor claims, media, marketing, and adjacent project
positioning.

- Context only.
- May identify claims to avoid or questions to investigate.
- Must never be primary implementation evidence or public-claim evidence.

Conflicting guidance policy:

- Record both sources with tier, date, stability class, and affected domain.
- Prefer final standards/RFCs/official advisories over drafts, research, and
  vendor claims.
- Do not resolve conflicts by implementation changes inside a watch lane.
- If conflict affects a security decision, propose a blocker/audit candidate
  that asks for explicit Director authorization.

Draft versus final standard policy:

- RFCs and final standards are stable references for current requirements.
- Internet-Drafts are volatile signals and must be revalidated by exact version.
- Drafts can shape future watch questions but cannot establish QSL compliance,
  readiness, or implementation correctness.

Vendor/competitor claim policy:

- Track only for public narrative risk and adjacent-project awareness.
- Record evidence offered by the source, not marketing conclusions.
- Do not adopt vendor language such as production, public-internet, metadata,
  anonymity, untraceability, or perfect-crypto claims.

## Watch domains and cadence design

### 1. PQC standards and migration

Scope: NIST FIPS, errata, NIST/NCCoE migration guidance, NCSC timelines,
CISA/NSA/NIST guidance, CNSA context, validation/certification notes, backup
algorithm selection, and hybrid migration guidance.

Refresh cadence: monthly; immediately on NIST FIPS errata, new final standards,
new migration guidance, or high-severity official advisory.

Expected output: source inventory delta, QSL relevance, migration risk
classification, candidate audit/backlog item, and public-claim caveat.

Queue rule: official guidance can propose a blocker candidate only when it
directly affects QSL cryptographic assumptions or public claims.

Stop condition: any finding requiring protocol, wire, crypto, auth, state
machine, dependency, or public-claim changes must stop at recommendation unless
future live scope explicitly authorizes exact files.

### 2. IETF/CFRG protocol evolution

Scope: TLS, HPKE, MLS, hybrid KEMs, ML-KEM identifiers, PQ HPKE, PQ MLS,
Internet-Drafts, RFC errata, and CFRG recommendations.

Refresh cadence: monthly for RFCs and quarterly for errata; biweekly for active
drafts during relevant working-group movement.

Expected output: final/RFC versus draft split, version numbers, stability class,
and affected QSL design questions.

Queue rule: final RFCs can propose audit candidates; active drafts usually
produce BACKLOG watch candidates only.

Stop condition: draft-derived implementation changes are forbidden in watch
lanes.

### 3. Rust/advisory/dependency health

Scope: RustSec, GitHub Security Advisories, NVD/CVE, CISA KEV, cargo audit
inputs, duplicate dependency-family watch, crypto crate upstream release notes,
OpenSSL/BoringSSL/RustCrypto/OQS/liboqs where relevant.

Refresh cadence: weekly; immediately on cargo audit failure, new high/critical
advisory, dependency PR, or CISA KEV hit affecting used components.

Expected output: affected package/component, version range, fixed version,
local usage, exploit status, and proposed lane.

Queue rule: affected direct dependencies with official high/critical advisories
may justify blocker candidates; unaffected ecosystem news remains BACKLOG.

Stop condition: dependency updates require separate implementation scope.

### 4. Secure messaging and metadata privacy

Scope: official protocol docs first, peer-reviewed research second, and vendor
claims only as low-confidence context. Topics include metadata minimization,
traffic analysis, timing, cover traffic, batching, padding, secure messaging
protocols, and claims.

Refresh cadence: quarterly; immediately before public technical paper or
website/public-claim changes.

Expected output: finding classification, QSL relevance to G5, evidence tier,
and public-claim caveat.

Queue rule: research creates threat-model/audit candidates, not automatic READY
items.

Stop condition: no metadata-free, anonymity, untraceability, or hides-all-
metadata claim may be introduced.

### 5. Backup/restore/key custody

Scope: restic, Borg, rclone, age, GnuPG, OpenSSH, restore-drill guidance, key
custody/recovery practice, backup monitoring, and operator runbook practices.

Refresh cadence: quarterly; immediately on security advisory, backup design
change, off-host target work, real restore drill, or key custody lane.

Expected output: tool source, release/advisory delta, relevance to local
continuity, backup-impact classification, and future prerequisite.

Queue rule: official tool advisories affecting local backup/restore pathways
can propose blocker candidates.

Stop condition: local same-host continuity must not be described as complete
disaster recovery, and off-host backup completion must not be claimed without
exact evidence.

### 6. External review and public claims

Scope: CVD, vulnerability disclosure, external review prerequisites, public
claim risk patterns, OpenSSF/OWASP-style verification guidance, and claim
taxonomy.

Refresh cadence: quarterly; immediately before public technical paper,
website/public-doc update, release-readiness, or external-review package
refresh.

Expected output: claim-boundary delta, evidence gap, required review/audit
candidate, and prohibited wording notes.

Queue rule: this domain can propose blocker candidates before public-facing
claims.

Stop condition: source discovery, internal audit, or standards watch must not
be represented as external-review complete.

### 7. Adjacent/competitor project context

Scope: what similar projects claim, what evidence they provide, and what claim
patterns QSL should avoid.

Refresh cadence: quarterly or before public narrative work.

Expected output: low-confidence context notes and claim-boundary warnings.

Queue rule: BACKLOG context only; never primary evidence.

Stop condition: marketing/vendor claims must not drive implementation or public
claims.

### 8. High-impact vulnerabilities and breaking research

Scope: official critical/high advisories, active exploitation, relevant
cryptanalysis, severe protocol attacks, severe dependency issues, and major
ecosystem breakage.

Refresh cadence: event-triggered plus weekly advisory check.

Expected output: severity, source tier, affected QSL component, local exposure,
and recommended blocker/audit candidate.

Queue rule: directly relevant official high/critical advisory can justify a
blocker candidate, but one-READY discipline remains enforced.

Stop condition: if root cause is unclear enough that continuing would risk
untruthful evidence or behavior drift, stop and request direction.

## Report format / storage / backup design

Future watch report fields:

- source inventory table;
- change summary;
- risk classification;
- QSL relevance;
- affected components;
- recommended NEXT_ACTIONS candidates;
- public-claim implications;
- external-review implications;
- evidence gaps;
- citations and links;
- access date;
- source stability class;
- conflict notes.

Storage options considered:

1. `/srv/qbuild/tmp` temp report.
   - Preferred for machine-readable proof during the first sweep.
   - No durable backup-plan update needed when temp-only.
2. qsl-protocol tracked governance evidence summary.
   - Preferred durable summary for NA-0392 if scoped.
   - Keeps reviewable citations in PR history.
3. `/home/victor/work/qsl/codex/ops/watch`.
   - Rejected for NA-0392 unless separately authorized.
   - Would require backup-impact review and archive mutation controls.
4. Final response only.
   - Useful as supporting evidence but not sufficient as the only future watch
     artifact.
5. No durable report.
   - Acceptable only for authorization lanes; insufficient for a first
     source-cited sweep.

Decision: NA-0392 should use tracked qsl-protocol evidence plus optional temp
proof under `/srv/qbuild/tmp/NA0392_external_watch_*`. Durable local watch
directories and background reports remain forbidden unless future live scope
explicitly authorizes them and records backup impact.

Backup-plan impact: no backup-plan update is required for NA-0391 because only
qsl-protocol governance/testplan/traceability/journal files change. Future
durable watch reports outside qsl-protocol require separate backup-impact
review.

## Queue insertion and claim boundary policy

- External watch findings do not automatically create READY items.
- Findings may propose BACKLOG candidates or blocker candidates.
- Exactly one READY remains enforced at all queue transitions.
- CRITICAL/HIGH official advisories can justify a blocker candidate when
  directly relevant to QSL.
- Draft, preprint, vendor, or competitor findings usually produce BACKLOG
  candidates only.
- Public-claim changes require a separate audit and evidence lane.
- Public technical paper work requires a fresh source-cited watch summary.
- Source discovery does not equal external review.
- A standards watch does not equal production readiness, public-internet
  readiness, complete disaster recovery, metadata-free behavior, anonymity,
  untraceability, bug-free behavior, or perfect crypto.

## External watch / public technical paper interaction

The public technical paper remains future-gated.

Before a public technical paper starts, QSL should require:

- latest source-cited external standards / threat / technology watch summary;
- current code/crypto audit status;
- current public-claim boundary audit;
- current external-review readiness assessment;
- current service evidence status;
- current backup/restore/key custody status;
- explicit classification of each claim as proven, tested, modeled, planned,
  not proven, or not claimed.

The paper must distinguish:

- proven;
- tested;
- modeled;
- planned;
- not proven;
- not claimed.

## Future watch implementation options

| Option | Value | Risk | Web dependency | Backup impact | Workflow impact | Dependency impact | Public-claim risk | Testability | Authority | Recommended status |
|---|---|---|---|---|---|---|---|---|---|---|
| First source-cited watch sweep as governance/evidence report | Delivers immediate source-cited baseline | Manual and citation-heavy | Required read-only web | Low if tracked evidence plus temp output only | None | None | Manageable with claim scan | High via source table and scope guard | Strong | Preferred for NA-0392 |
| Standalone qsl-protocol helper for watch schema validation | Improves repeatability | Adds code surface | None after sources gathered | Low | None | Low | High | Medium | Future candidate after first sweep |
| Extend routine audit cadence helper | Integrates with existing cadence | Blurs audit versus watch boundary | None after sources gathered | Low | None | Medium | Medium | Medium | Defer |
| Docs-only source inventory | Simple and reviewable | Can become stale | Required at update time | Low | None | Medium | Medium | Medium | Accept as part of first sweep, not alone |
| Durable ops watch directory | Durable local continuity | Backup/archive policy needed | Required | Medium | None | Medium | Medium | Medium | Blocked unless separately authorized |
| GitHub workflow/scheduled watch | Automation | High scope and web/dependency risk | Required | Medium | High | Medium | Medium | Low for first lane | Reject for first implementation |
| Manual-only status quo | No new surface | Easy to forget and uncited | Ad hoc | None | None | Medium | Low | Low | Reject after authorization |

## First-lane authorization decision

Targeted source discovery succeeded and identified sufficient primary and
supporting source locations to design the future watch process. Report storage
can be handled by qsl-protocol tracked governance evidence plus optional temp
proof, without creating a durable local watch report store.

Decision:

`EXTERNAL_STANDARDS_THREAT_TECH_WATCH_FIRST_SWEEP_AUTHORIZATION_READY`

NA-0392 should perform the first bounded, read-only, source-cited external
standards / threat / technology watch sweep. NA-0392 must not mutate runtime,
protocol, crypto, workflows, dependencies, public docs, website, backup
scripts, qsl-server, qsl-attachments, qshield runtime, or response archives
unless future live scope explicitly authorizes exact paths.

## Future allowed/forbidden path bundle

Future NA-0392 allowed paths if this successor is restored:

- `docs/governance/evidence/NA-0392_qsl_external_standards_threat_technology_watch_first_sweep.md`;
- `tests/NA-0392_qsl_external_standards_threat_technology_watch_first_sweep_testplan.md`;
- optional temp proof under `/srv/qbuild/tmp/NA0392_external_watch_*`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Future durable source inventory or local watch report store:

- forbidden unless future live scope explicitly authorizes exact paths and
  backup-impact handling.

Future forbidden paths unless separately authorized:

- `.github/**`;
- scripts/helpers;
- Cargo files and dependencies;
- runtime, service, protocol, crypto, auth, state-machine, qsc/qsp/qsl, qshield
  runtime, qsl-server, qsl-attachments, qsc-desktop, website, docs/public,
  README, START_HERE;
- backup scripts/timers/fstab/services/source lists/remote targets;
- durable local watch reports under `/home/victor/work/qsl/codex/**`;
- response archive mutation;
- directive/request/journal archive mutation;
- `/srv/qbuild/tools/**`.

## Governance/security/fail-closed requirements

- Source-cited evidence is required.
- No uncited current-technology claims.
- No vendor-hype claims.
- Drafts and preprints must be clearly labeled.
- Final standards and RFCs must be distinguished from drafts.
- No code mutation inside watch authorization/sweep lanes unless future live
  scope explicitly authorizes exact files.
- No dependency mutation inside watch authorization/sweep lanes unless future
  live scope explicitly authorizes exact files.
- No workflow or scheduler changes.
- No public-claim changes.
- No background watch.
- No durable report unless authorized.
- Queue insertion is proposal-only.
- No automatic READY promotion.
- No secret material.
- No archive mutation.
- No public technical paper in NA-0391.

## Public-claim/external-review/website boundary

- External watch authorization is not implementation.
- Targeted source discovery is not external review.
- A standards watch is not production readiness.
- A standards watch is not public-internet readiness.
- A standards watch is not metadata-free proof.
- A standards watch is not anonymity or untraceability proof.
- A standards watch is not complete disaster recovery.
- A standards watch is not bug-free or perfect-crypto proof.
- No website/public docs update is authorized.
- Public technical paper work remains future-gated.

## Future validation/marker/verification plan

Future NA-0392 should prove these markers:

- `NA0392_EXTERNAL_WATCH_AUTHORIZATION_OK`
- `NA0392_SOURCE_INVENTORY_OK`
- `NA0392_PRIMARY_SOURCES_OK`
- `NA0392_STANDARDS_SOURCE_TIERING_OK`
- `NA0392_PQC_STANDARDS_WATCH_OK`
- `NA0392_IETF_CFRG_PROTOCOL_WATCH_OK`
- `NA0392_RUST_ADVISORY_WATCH_OK`
- `NA0392_CODE_CRYPTO_RESEARCH_WATCH_OK`
- `NA0392_METADATA_PRIVACY_RESEARCH_WATCH_OK`
- `NA0392_BACKUP_RESTORE_WATCH_OK`
- `NA0392_PUBLIC_CLAIM_BOUNDARY_WATCH_OK`
- `NA0392_SOURCE_CITATION_OK`
- `NA0392_DRAFT_PREPRINT_CAVEAT_OK`
- `NA0392_NO_RUNTIME_CHANGE_OK`
- `NA0392_NO_DEPENDENCY_CHANGE_OK`
- `NA0392_NO_WORKFLOW_CHANGE_OK`
- `NA0392_NO_SECRET_MATERIAL_OK`
- `NA0392_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0392_NO_PUBLIC_INTERNET_READY_CLAIM_OK`
- `NA0392_NO_METADATA_FREE_CLAIM_OK`
- `NA0392_NO_ANONYMITY_CLAIM_OK`
- `NA0392_NO_UNTRACEABLE_CLAIM_OK`
- `NA0392_NO_EXTERNAL_REVIEW_COMPLETE_CLAIM_OK`

Future validation bundle:

- queue/decision helper proof;
- source table completeness check;
- citation fields present for every source;
- source stability labels present for every source;
- no uncited current claims;
- scope guard;
- overclaim scan;
- link check;
- leak scan;
- cargo audit and rustls-webpki proof;
- qsc send_commit;
- formal model checks;
- PR body goal-lint and public-safety.

## Selected successor

Selected successor:

`NA-0392 -- QSL External Standards / Threat / Technology Watch First Source-Cited Sweep`

Rationale:

- Source discovery succeeded.
- The source taxonomy has enough Tier 1 and Tier 2 sources to support a first
  sweep.
- Storage policy can remain tracked governance evidence plus optional temp
  proof.
- Durable report storage is not required before the first sweep.
- Backup impact is understood and bounded.
- Public-claim boundaries and queue insertion rules are explicit.

## Rejected alternatives

- Performing a full watch sweep in NA-0391: rejected because NA-0391 is an
  authorization/planning lane.
- Changing runtime, protocol, crypto, or dependencies now: rejected as outside
  scope.
- Changing workflows or adding a scheduler now: rejected as outside scope and
  higher operational risk.
- Writing a durable local watch report now: rejected because it requires future
  backup-impact authorization.
- Starting the public technical paper now: rejected because source-cited watch,
  code/crypto audit, external-review readiness, service evidence, and
  backup/restore/key evidence remain prerequisites.
- Selecting a report-storage blocker successor: rejected because first-sweep
  evidence can use tracked governance files plus optional temp proof without a
  durable local report store.

## Backup-plan impact statement

No backup-plan update is required for NA-0391. Durable changes are limited to
qsl-protocol governance/evidence/testplan/traceability/journal files. No
backup script, timer, fstab, service, source list, key, passphrase, off-host
target, restore path, recovery envelope, or monitoring configuration changes
are authorized or made.

Future durable external-watch report storage under local Codex ops paths, or
any backup source-list change to include such storage, requires separate
backup-impact review. Same-host local continuity remains same-host continuity
only, not complete disaster recovery.

## Next recommendation

Merge NA-0391 authorization evidence, then optionally close out NA-0391 and
restore:

`NA-0392 -- QSL External Standards / Threat / Technology Watch First Source-Cited Sweep`

NA-0392 should run the first bounded, read-only, source-cited sweep using the
taxonomy, cadence, citation policy, queue rules, claim boundaries, and allowed
path bundle defined here.
