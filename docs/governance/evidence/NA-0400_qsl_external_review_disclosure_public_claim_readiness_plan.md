Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-01

# NA-0400 QSL External Review / Disclosure / Public Claim Readiness Plan

Goals: G1, G2, G3, G4, G5

Directive: QSL-DIR-2026-06-01-220

## Executive Summary

NA-0400 maps QSL's current evidence boundaries against external review,
coordinated vulnerability disclosure, public security policy, public claim,
and public technical paper prerequisites. It is a governance readiness map
only.

This lane does not perform an external review, create a disclosure program,
publish a security policy, update public docs or website content, contact
reviewers, start a public technical paper, change code, change cryptography,
change dependencies, mutate workflows, mutate qsl-server or qsl-attachments,
mutate qshield runtime, handle secrets, run backups, run restores, or expand
public claims.

Conservative outcome:

- QSL has a partial reviewer-orientation package and substantial governance,
  harness, formal/model, advisory, metadata/privacy, and backup/key boundary
  evidence.
- QSL does not have a complete future external-review package because code /
  crypto audit, accepted reviewer scope, finding disposition, service
  production evidence, disclosure operating procedure, and backup/key/restore
  evidence remain incomplete.
- QSL has an existing `SECURITY.md` and GitHub private vulnerability reporting
  is enabled, but coordinated disclosure readiness remains partial because the
  end-to-end handling workflow, triage SLAs, embargo, advisory process,
  credits, remediation release process, security.txt, and website/public policy
  decision are not completed here.
- QSL public claims must remain bounded: no production, public-internet,
  external-review-complete, metadata-free, anonymity, untraceable, complete
  disaster-recovery, off-host-backup-complete, restore-proven, key-custody,
  bug-free, perfect-crypto, certification, or compliance claim is supported.
- Public technical paper work remains future-gated.

Selected successor:

`NA-0401 -- QSL Project Goal and Operating Principles Canon Authorization Plan`

Rationale: source verification succeeded, no critical public-claim or
disclosure blocker was found, and QSL has now mapped the major prerequisite
evidence groups from NA-0392 through NA-0400. Before any public technical paper
or public-facing claim work, the project should canonize the internal north
star and operating principles that enforce security-before-speed, evidence over
vibes, and no public overclaiming.

## Live NA-0400 Scope

Live `NEXT_ACTIONS.md` records NA-0400 as READY with objective to create a
qsl-protocol governance plan that maps external review, coordinated disclosure,
and public-claim readiness prerequisites into current evidence boundaries
without public-doc, website, production, public-internet, external-review
complete, or public technical paper claims.

Allowed mutation for this lane is limited to:

- `docs/governance/evidence/NA-0400_qsl_external_review_disclosure_public_claim_readiness_plan.md`
- `tests/NA-0400_qsl_external_review_disclosure_public_claim_readiness_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Allowed source verification:

- official coordinated vulnerability disclosure guidance;
- official vulnerability handling guidance;
- official security policy / vulnerability reporting standards;
- official secure development and assurance guidance;
- selected official supply-chain assurance frameworks used as context only.

Allowed read-only scans:

- qsl-protocol governance evidence, README, START_HERE, docs/public, scripts,
  tests, inputs, decisions, traceability, and queue files;
- qsl-server and qsl-attachments PR/source/CI metadata only;
- public-claim term searches;
- existing validation checks already used by prior lanes.

Forbidden scope:

- runtime, service, qsc/qsp/qsl, protocol, crypto, key schedule, qshield
  runtime, qsl-server, qsl-attachments, qsc-desktop, website, docs/public,
  README, START_HERE, workflow, Cargo, dependency, branch-protection,
  public-safety, backup script/timer/fstab/source-list, response archive
  history, local qstart/qresume tooling, security.txt, new disclosure policy,
  public technical paper, external-review package outside this governance
  evidence, reviewer contact, secret handling, real backup, real restore, or
  off-host target setup mutation.

Acceptance criteria:

1. READY_COUNT remains 1.
2. READY remains NA-0400 until closeout.
3. NA-0399 is DONE.
4. D-0780 and D-0781 exist once.
5. D-0782 is added once by this plan.
6. External review, disclosure, public security policy, public claim, and
   public technical paper readiness axes are mapped.
7. Official sources are cited.
8. Claim boundaries and not-ready states are explicit.
9. Exactly one NA-0401 successor is selected.
10. Required validation and CI remain green before merge.

Stop conditions include source verification failure preventing useful policy,
unsupported public claims, code/crypto/dependency/workflow/public-doc/website
mutation, sibling-repo mutation, security policy creation, reviewer contact,
secret handling, multiple READY items, or treating this plan as external
review completion or public claim authorization.

## Inherited NA-0399 Rationale

NA-0399 selected NA-0400 after completing backup / restore / key custody
external guidance mapping. It established:

- qsl-protocol PR #1061 merged at `43c90b60b34c`.
- qsl-protocol PR #1062 merged at `655c89f34975`.
- READY_COUNT 1 and READY NA-0400.
- NA-0399 DONE.
- D-0780 and D-0781 present once.
- D-0782 absent before NA-0400.
- same-host local continuity evidence exists but is not disaster recovery.
- off-host encrypted backup, real restore, real key custody, and real key
  recovery remain incomplete or blocked.
- qsl-server PR #56 remains bounded harness evidence only.
- qsl-attachments PR #37 remains service-local prerequisite evidence only.
- public technical paper work remains future-gated.

Inherited successor rationale:

- NA-0392 identified disclosure, external review, and public claims as a watch
  category.
- NA-0393 kept that category as a future candidate after foundational mapping.
- NA-0394 through NA-0399 mapped PQC standards, RFC/draft boundaries,
  dependency/advisory posture, code/crypto audit planning, metadata/privacy
  claim boundaries, and backup/restore/key custody boundaries.
- The next truthful step is to consolidate readiness and claim boundaries
  before any public paper, website, public security policy implementation, or
  external-review package authorization.

## Authoritative External Review / Disclosure / Public Claim Readiness Source Verification

Access date for all web sources in this section: 2026-06-01.

This is targeted source verification only. It is not external review,
coordinated disclosure implementation, public policy publication, security
assessment, certification, or public-claim authorization.

| Source | Authority / publisher | URL | Source tier | Classification | Relevance to QSL | Readiness / claim-boundary implication |
|---|---|---|---|---|---|---|
| Coordinated Vulnerability Disclosure Program | CISA | https://www.cisa.gov/resources-tools/programs/coordinated-vulnerability-disclosure-program | Tier 1 official guidance | OFFICIAL_DISCLOSURE_GUIDANCE | CVD process, VINCE intake, coordination, mitigation, and public disclosure timing context. | QSL needs a documented handling workflow before claiming disclosure readiness. |
| CERT Guide to Coordinated Vulnerability Disclosure | CERT/CC, Software Engineering Institute | https://certcc.github.io/CERT-Guide-to-CVD | Tier 1 official guidance | OFFICIAL_DISCLOSURE_GUIDANCE | Roles, coordination, operation, and CVD capability guidance. | Useful operating model; source discovery is not a QSL CVD process. |
| CERT Vulnerability Disclosure Guidance | CERT/CC | https://www.kb.cert.org/vuls/guidance/ | Tier 1 official guidance | OFFICIAL_DISCLOSURE_GUIDANCE | Reporter guidance and coordination context. | QSL needs reporter instructions, triage, communications, and advisory workflow. |
| ISO/IEC 29147:2018 Vulnerability disclosure | ISO | https://www.iso.org/standard/72311.html | Tier 1 official standard page | OFFICIAL_DISCLOSURE_GUIDANCE | External-facing vulnerability disclosure standard reference. | Supports need for published policy and intake boundary; QSL does not claim ISO conformance. |
| ISO/IEC 30111:2019 Vulnerability handling processes | ISO | https://www.iso.org/standard/69725.html | Tier 1 official standard page | OFFICIAL_VULNERABILITY_HANDLING_GUIDANCE | Internal handling and remediation process reference. | QSL needs handling records, verification, remediation, release, and closure process evidence. |
| ISO/IEC TR 5895:2022 Multi-party coordinated vulnerability disclosure and handling | ISO | https://www.iso.org/standard/81807.html | Tier 1 official technical report page | OFFICIAL_VULNERABILITY_HANDLING_GUIDANCE | Multi-party CVD life-cycle context. | Relevant if QSL issues affect dependencies, sibling repos, or downstream users. |
| PSIRT Services Framework v1.1 | FIRST | https://www.first.org/standards/frameworks/psirts/psirt_services_framework_v1.1 | Tier 1 official framework | OFFICIAL_VULNERABILITY_HANDLING_GUIDANCE | Product security incident response services and vulnerability handling functions. | QSL can use as a future process checklist; it is not current PSIRT capability proof. |
| RFC 9116: A File Format to Aid in Security Vulnerability Disclosure | RFC Editor / IETF | https://www.rfc-editor.org/rfc/rfc9116 | Tier 1 RFC | OFFICIAL_SECURITY_POLICY_STANDARD | security.txt format and well-known contact mechanism. | QSL currently has no security.txt; public website policy decision remains future-gated. |
| Configuring private vulnerability reporting for a repository | GitHub Docs | https://docs.github.com/en/code-security/security-advisories/working-with-repository-security-advisories/configuring-private-vulnerability-reporting-for-a-repository | Tier 1 official platform docs | OFFICIAL_SECURITY_POLICY_STANDARD | GitHub private vulnerability reporting for public repositories. | qsl-protocol PVR is enabled, but workflow/policy evidence remains partial. |
| Secure Software Development Framework SP 800-218 | NIST CSRC | https://csrc.nist.gov/pubs/sp/800/218/final | Tier 1 official guidance | OFFICIAL_SECURE_DEVELOPMENT_GUIDANCE | Secure software development practices and response-to-vulnerabilities vocabulary. | QSL has strong CI/governance posture but no SSDF conformance claim. |
| NIST Cybersecurity Framework 2.0 | NIST | https://www.nist.gov/cyberframework | Tier 1 official framework | OFFICIAL_ASSURANCE_FRAMEWORK | Risk-management framework context. | Useful future governance mapping; not a QSL maturity or compliance claim. |
| OWASP Application Security Verification Standard | OWASP Foundation | https://owasp.org/www-project-application-security-verification-standard/ | Tier 2 official open framework | OFFICIAL_ASSURANCE_FRAMEWORK | Security verification requirements framework for applications/APIs. | Useful reviewer checklist context; not proof QSL meets any ASVS level. |
| OpenSSF Scorecard | OpenSSF | https://openssf.org/scorecard/ | Tier 2 official open framework | OFFICIAL_ASSURANCE_FRAMEWORK | Automated open-source security posture metrics. | Can inform future supply-chain hygiene checks; scorecard output is not external review. |
| SLSA framework | SLSA project / OpenSSF ecosystem | https://slsa.dev/ | Tier 2 official open framework | OFFICIAL_ASSURANCE_FRAMEWORK | Supply-chain integrity framework from source to build artifacts. | Useful future build/release integrity context; no SLSA level is claimed. |
| OpenSSF Best Practices Badge | OpenSSF | https://openssf.org/projects/best-practices-badge/ | Tier 2 official open framework | OFFICIAL_ASSURANCE_FRAMEWORK | Voluntary open-source best-practices self-certification criteria. | Possible future hygiene lane; no badge or self-certification is claimed. |

Citation gaps and uncertainty:

- ISO pages provide public summaries; full standards text is not reproduced in
  this evidence.
- GitHub repository API confirmed private vulnerability reporting is enabled
  for qsl-protocol, but this plan does not change repository settings.
- Frameworks such as OWASP ASVS, OpenSSF Scorecard, SLSA, and NIST CSF are
  assurance/readiness context only. They are not external review outcomes or
  public certification proof.

## Cumulative Evidence Intake

| Evidence source | What exists | What remains partial or absent | Claim boundary |
|---|---|---|---|
| NA-0392 external watch | First source-cited standards/threat/technology sweep. | Not a full external review or literature review. | Source discovery is not external review. |
| NA-0393 triage | Ten NA-0392 findings triaged into candidate groups. | No automatic READY promotion beyond selected successors. | Triage is not implementation. |
| NA-0394 PQC mapping | NIST PQC standards and migration guidance mapped; ML-KEM/ML-DSA classified as implemented but evidence-incomplete. | No FIPS validation, certification, external review, or conformance matrix. | No FIPS/compliance/certification claim. |
| NA-0395 RFC/draft boundary | TLS, HPKE, MLS RFCs and PQ/hybrid drafts mapped. | QSL does not implement or claim TLS, HPKE, MLS, or active draft compliance. | Drafts are watch inputs only. |
| NA-0396 advisory policy | RustSec, GHSA, NVD/CVE, CISA KEV, upstream source policy recorded; cargo audit green. | Green audit is point-in-time, not vulnerability-free proof. | No bug-free or advisory-free claim. |
| NA-0397 code/crypto audit planning | Research venues, tool guidance, NA-0380 audit themes, future audit groups recorded. | No completed code/crypto audit or external reviewer findings. | Audit planning is not audit completion. |
| NA-0398 metadata/privacy map | Secure messaging/privacy sources and claim axes mapped. | No metadata-free, anonymity, untraceable, hidden timing/traffic/size proof. | Strong privacy claims remain forbidden. |
| NA-0399 backup/key map | Same-host continuity, no-secret harnesses, and official backup/key/DR sources mapped. | Off-host backup, real restore, key custody, and key recovery incomplete or blocked. | Same-host continuity is not disaster recovery. |
| qsl-server PR #56 | Merged at `d40e6003fdf0`; bounded harness evidence. | No public-internet or production operations evidence. | Service-local proof only. |
| qsl-attachments PR #37 | Merged at `96b9352bd63`; service-local prerequisite evidence. | No production attachment service, public-internet, or operational proof. | Service-local proof only. |
| qshield demo | Demo/harness evidence for selected paths. | No production proof or live service proof. | Demo evidence is not production proof. |
| Local ops helpers | qstart/qresume guard, bounded polling, directive manifest validator, response writer, catalog, routine cadence helper exist. | Validation profile, durable external reports, and full source/claim scanner remain future candidates. | Local ops support is not release readiness. |

## Read-Only Public Surface / Claim Inventory

Read-only scan roots included `README.md`, `START_HERE.md`, `docs`, `tests`,
`scripts`, `inputs`, `DECISIONS.md`, `TRACEABILITY.md`, and
`NEXT_ACTIONS.md`.

Results:

- Broad public-claim/security term scan: 9,275 matches.
- Representative hits are mostly explicit non-claims, claim-boundary warnings,
  public evidence navigation, and governance/testplan language.
- Existing `SECURITY.md` is present at repo root.
- No `security.txt` file was found in a bounded find scan.
- `docs/public/EXTERNAL_REVIEW_PACKAGE.md` exists and states that it is
  reviewer-orientation material, not external review completion.
- `README.md`, `START_HERE.md`, and `docs/public/INDEX.md` contain visible
  non-production and not-ready boundaries.
- GitHub API reported private vulnerability reporting enabled for
  qsl-protocol.

Planning implication:

- The current public surface already contains useful non-claim language, but it
  is not a completed public security policy rollout.
- `SECURITY.md` is minimal and references GitHub private reporting; future CVD
  work should decide whether to expand it, add security.txt, publish website
  contact details, and define handling operations.
- NA-0400 does not edit public docs or security policy files.

## External Review Readiness Map

| Area | Current evidence | Missing package/evidence | Classification | Claim boundary |
|---|---|---|---|---|
| Scope statement | Existing external review package and many governance evidence docs. | Current NA-0392 through NA-0400 consolidation and reviewer instructions are not packaged as a separate authorized bundle. | PARTIAL_REVIEW_PACKAGE | Reviewer orientation exists; package completion is not claimed. |
| Architecture / threat model | Canonical docs, privacy docs, service-boundary docs, traceability. | Reviewer-facing current architecture summary and exact reviewed commit scope. | PARTIAL_REVIEW_PACKAGE | Not external-review acceptance. |
| Crypto/protocol evidence | Suite-2 vectors, qsc/refimpl evidence, formal models, PQC/RFC maps. | Code/crypto audit, conformance matrix, side-channel review, provider maturity review. | EVIDENCE_INCOMPLETE | No external-review-complete or crypto-audited claim. |
| Metadata/privacy evidence | NA-0398 axis map and qshield harnesses. | Production service metadata behavior, traffic analysis assumptions, public claim audit. | BLOCKED_PENDING_METADATA_PRIVACY_BOUNDARY | No metadata-free/anonymity/untraceable claim. |
| Backup/restore/key evidence | NA-0399 source map, same-host local continuity, no-secret harnesses. | Off-host encrypted backup, real restore, key custody/recovery proof. | BLOCKED_PENDING_BACKUP_RESTORE_KEY_EVIDENCE | No DR/off-host/restore/key-custody claim. |
| Service evidence | qsl-server PR #56 and qsl-attachments PR #37 merged. | Public-internet deployment, production operations, monitoring, runbooks, external review. | BLOCKED_PENDING_SERVICE_PRODUCTION_EVIDENCE | Service-local proof only. |
| Reviewer workflow | Existing reproducible commands and package rows. | No accepted reviewer scope, no findings/dispositions/residual risk log. | EVIDENCE_INCOMPLETE | Not externally reviewed. |

Overall classification: `PARTIAL_REVIEW_PACKAGE` and `EVIDENCE_INCOMPLETE`.
NA-0400 is not external review and does not authorize an external-review
complete claim.

## Coordinated Vulnerability Disclosure Readiness Map

| Axis | Current QSL state | Missing items | Classification | Claim boundary |
|---|---|---|---|---|
| Disclosure policy | `SECURITY.md` exists with GitHub private reporting guidance. | Full CVD policy, safe-harbor decision, scope, response expectations, public website decision. | PARTIAL_REVIEW_PACKAGE | Minimal policy exists; full CVD program not claimed. |
| Security contact | GitHub private vulnerability reporting enabled. | Stable public contact/security.txt/contact redundancy and maintainer response ownership. | PARTIAL_REVIEW_PACKAGE | Contact channel is not full process readiness. |
| security.txt | No file found. | RFC 9116 decision, website placement, contact, expires, encryption, acknowledgments if chosen. | BLOCKED_PENDING_WEBSITE_POLICY_DECISION | No security.txt claim. |
| Handling workflow | NA-0396 advisory policy exists. | Intake triage, severity, embargo, reporter communication, remediation, advisory publication, credits, closure. | GOVERNANCE_PLANNED | Advisory watch policy is not vulnerability handling implementation. |
| GitHub security advisories | Platform private reporting enabled. | Maintainer playbook for draft advisories, CVE/CNA path, release notes, downstream notification. | GOVERNANCE_PLANNED | Platform feature is not complete PSIRT process. |
| Multi-party coordination | Source guidance cited. | Sibling repo / dependency / downstream coordination procedure. | EVIDENCE_INCOMPLETE | No multi-party CVD readiness claim. |

Overall classification: `GOVERNANCE_PLANNED` / `EVIDENCE_INCOMPLETE`.

## Public Security Policy Readiness Map

| Axis | Status | Missing evidence | Classification | Claim boundary |
|---|---|---|---|---|
| `SECURITY.md` | Present and minimal. | Expanded policy, scope, response expectations, contact durability, advisory procedure. | PARTIAL_REVIEW_PACKAGE | Do not claim complete security policy. |
| `security.txt` | Absent. | RFC 9116 policy and website/public-doc decision. | BLOCKED_PENDING_WEBSITE_POLICY_DECISION | No security.txt availability claim. |
| Website/public docs | Existing public docs are read-only in NA-0400. | No authorized website update or public security policy rollout. | CLAIM_BOUNDARY_REQUIRED | NA-0400 does not publish. |
| Public contact | GitHub PVR enabled. | Alternative contact, encrypted contact, published policy fields. | EVIDENCE_INCOMPLETE | Contact readiness remains partial. |

## Public Claim Readiness Map

| Claim area | Allowed now? | Required future evidence | Classification / forbidden wording |
|---|---|---|---|
| Production readiness | No. | Production deployment plan, operations, monitoring, service proof, backup/key/restore, external review, public claim audit. | NOT_READY; forbidden: production-ready / ready for production. |
| Public-internet readiness | No. | Public deployment architecture, threat model, abuse/rate/DoS handling, monitoring, incident response, external review. | NOT_READY. |
| External review complete | No. | Accepted review scope, reviewed commit, findings, dispositions, residual-risk signoff. | NOT_CLAIMED. |
| PQC / FIPS / certification | No. | Standards conformance matrix, validation/certification evidence if claimed, audit. | CLAIM_BOUNDARY_REQUIRED; no FIPS compliance/certification claim. |
| TLS / HPKE / MLS compliance | No. | Dedicated implementation/conformance evidence. | NOT_CLAIMED. |
| Metadata-free / anonymity / untraceable | No. | Architecture, threat model, traffic analysis, service, external review, public claim audit. | BLOCKED_PENDING_METADATA_PRIVACY_BOUNDARY. |
| Timing / traffic shape / attachment-size hiding | No. | Production service behavior, measurements, cost/abuse model, external review. | EVIDENCE_INCOMPLETE. |
| Off-host backup complete | No. | Off-host target, encrypted repository, key custody, monitoring, restore proof. | BLOCKED_PENDING_BACKUP_RESTORE_KEY_EVIDENCE. |
| Disaster recovery complete | No. | Off-host backup, key recovery, real restore, runbook, metrics, exercises. | NOT_READY. |
| Restore proven | No. | Real isolated restore with verification and cleanup evidence. | NOT_READY. |
| Key custody / key recovery implemented | No. | Real key custody and recovery controls under explicit no-secret-safe process. | BLOCKED_PENDING_OPERATOR_INPUT. |
| Bug-free / perfect crypto / vulnerability-free | No. | Not a valid claim class. | Forbidden. |
| qshield demo proof | Only as demo/harness evidence. | Production service proof if stronger claims are desired. | DEMO_ONLY; not production proof. |

## Public Technical Paper Prerequisite Map

Before any public technical paper, QSL should have:

- Project Goal / Operating Principles canon.
- Current external standards watch and citation policy.
- PQC standards and migration evidence map.
- IETF/RFC/draft boundary map.
- Advisory trigger policy.
- Code/crypto audit status and future audit candidate resolution.
- Metadata/privacy claim boundary.
- Backup/restore/key custody status.
- Service production/public-internet boundary status.
- External-review package authorization.
- Public claim audit and non-claim language policy.
- Website/public docs readiness decision.
- Evidence package and citation policy.
- Explicit non-claim language policy for all strong assertions.

Current classification: `EVIDENCE_INCOMPLETE`; public paper remains
future-gated.

## Service Production / Demo Boundary Map

| Surface | Evidence | Classification | Claim boundary |
|---|---|---|---|
| qsl-server PR #56 | Merged at `d40e6003fdf0`; CI success; bounded end-to-end integration harness. | PARTIAL_REVIEW_PACKAGE / SERVICE_LOCAL_ONLY | Not production, not public internet, not external review. |
| qsl-attachments PR #37 | Merged at `96b9352bd63`; CI success; service-local production size-class harness. | PARTIAL_REVIEW_PACKAGE / SERVICE_LOCAL_ONLY | Not production attachment service proof. |
| qshield demo | Local demo and harness evidence. | DEMO_ONLY / HARNESS_ONLY | Not production proof. |
| qshield-cli | Build/test and harness evidence in qsl-protocol. | HARNESS_ONLY | No public service claim. |

Overall classification: `BLOCKED_PENDING_SERVICE_PRODUCTION_EVIDENCE`.

## Secure Development / Assurance Posture Map

Existing posture:

- public-safety is required by branch protection and green on current
  origin/main.
- force pushes and deletions are disabled; admins enforcement is enabled.
- `cargo audit --deny warnings` is green at NA-0400 startup.
- `rustls-webpki v0.103.13` is present.
- formal model checks exist for bounded SCKA / negotiation / handshake slices.
- qsc send_commit and qshield-cli checks are part of established validation.
- local-ops helpers exist for bounded CI polling, manifests, response writing,
  response history, and routine cadence.

Gaps:

- no completed code/crypto external audit;
- no complete side-channel review;
- no complete fuzz/property/differential coverage claim;
- no SSDF, ASVS, SLSA, OpenSSF Scorecard, or badge claim;
- no production incident-response proof.

Classification: `PARTIAL_REVIEW_PACKAGE` / `EVIDENCE_INCOMPLETE`.

## Vulnerability / Advisory Response Posture Map

Existing posture:

- NA-0396 defines source-cited advisory trigger policy for RustSec, GHSA,
  NVD/CVE, CISA KEV, upstream project notes, cargo audit, and local impact
  review.
- `cargo audit --deny warnings` is green at startup.
- GitHub private vulnerability reporting is enabled for qsl-protocol.
- `SECURITY.md` provides minimal reporting guidance.

Gaps:

- no complete CVD handling playbook;
- no triage SLA or responder rotation;
- no embargo and advisory publication procedure;
- no sibling-repo coordination procedure for qsl-server/qsl-attachments;
- no security.txt or public website policy decision;
- no public incident response/advisory archive process.

Classification: `GOVERNANCE_PLANNED` / `EVIDENCE_INCOMPLETE`.

## Evidence Package Requirements for Future External Review

A future external-review package should include:

- scope statement and reviewed commit(s);
- architecture summary and data-flow map;
- threat model;
- crypto/protocol documents and claim boundaries;
- formal model docs and exact model limitations;
- vectors, test vectors, and harness instructions;
- dependency/advisory status and applicability review;
- code/crypto audit results or current audit candidate state;
- metadata/privacy claim boundary;
- backup/restore/key custody boundary;
- service production/public-internet boundary;
- build/test instructions and expected markers;
- no-secret handling rules;
- public claim boundaries and prohibited claims;
- known gaps and residual risk;
- reviewer instructions and communication rules;
- disclosure policy and contact/handling procedure;
- explicit statement that no secrets, passphrases, credentials, private keys,
  recovery envelopes, live target identities, or sensitive host material are to
  be included.

NA-0400 does not create those package files.

## Public Claim Language Policy

Allowed language when evidence is precise:

- evidence-incomplete;
- governance planned;
- harness evidence;
- bounded formal model evidence;
- same-host continuity;
- service-local evidence;
- demo-only evidence;
- no-secret harness;
- future-gated;
- source-cited mapping;
- partial readiness;
- not claimed;
- claim boundary required.

Forbidden unless exact future evidence exists:

- production-ready;
- public internet ready;
- externally reviewed;
- external review complete;
- metadata-free;
- anonymous;
- untraceable;
- hides all metadata;
- hides timing;
- hides traffic shape;
- hides attachment size;
- off-host backup complete;
- disaster recovery complete;
- restore proven;
- key custody implemented;
- key recovery implemented;
- FIPS compliant, certified, or validated;
- TLS compliant;
- HPKE compliant;
- MLS compliant;
- bug-free;
- perfect crypto;
- vulnerability-free.

## Future Queue Candidates

| Candidate | Evidence basis | Why next / why not next | Likely allowed scope | Likely forbidden scope | Public-claim implication |
|---|---|---|---|---|---|
| QSL Project Goal and Operating Principles Canon Authorization Plan | NA-0392 through NA-0400 carry-forward and operator request. | Selected next because no blocker was found and internal principles should precede public paper/claims. | Internal governance canon, testplan, decisions, traceability, journal. | Runtime, public docs, website, claims, secrets. | Reinforces no-overclaim posture. |
| External Review Package Scope / Evidence Bundle Authorization Plan | NA-0400 partial review package map. | Not next; principles canon should land before package authorization. | Future governance package scope. | Reviewer contact, secrets, public claims. | Enables later package only. |
| Coordinated Vulnerability Disclosure / Security Policy Authorization Plan | SECURITY.md/PVR present, security.txt absent. | Important but not next absent critical blocker. | SECURITY/security.txt/public policy authorization if explicit. | Public launch without process. | Would improve reporting posture. |
| Public Claim / Website Boundary Audit Plan | Public docs contain non-claims and older external-review package. | Future before website updates. | Public surface audit. | Website edits unless exact scope. | Prevents overclaiming. |
| Public Technical Position Paper Evidence Prerequisite Plan | Paper remains future-gated. | Not next because prerequisites and principles canon remain. | Evidence prerequisite map. | Paper draft. | Keeps paper bounded. |
| Code / Crypto Audit Candidate Implementation Authorization Plan | NA-0397 audit themes. | Future; no immediate critical blocker. | Exact audit/fix scope. | Unscoped crypto changes. | Required before strong security claims. |
| Service Production Evidence Gap Plan | qsl-server/qsl-attachments service-local evidence. | Future; not next. | Service evidence mapping. | Sibling repo mutation unless authorized. | Needed before service claims. |
| Backup / Restore / Key Custody Critical Gap Plan | NA-0399 incomplete backup/key evidence. | Future; operator input remains blocker. | No-secret planning or explicit implementation if authorized. | Real secrets or targets without approval. | Needed before DR claims. |
| Director State Index Authorization Plan | Local-ops/history residual. | Useful later, not claim-critical now. | Governance/local ops index. | Public/runtime changes. | Improves continuity. |
| D132 Cleanup Authorization Plan | Preservation bundle remains. | Later after explicit authorization. | Cleanup only if exact. | Destructive cleanup without direction. | No public claim effect. |

## Selected Successor

Selected:

`NA-0401 -- QSL Project Goal and Operating Principles Canon Authorization Plan`

Rationale:

- Source verification succeeded.
- No external-review/public-claim critical blocker was found.
- No disclosure source-verification blocker was found.
- Current evidence supports returning to internal governance canonization before
  public technical paper, public claim, website, or external review package
  work.
- The selected successor matches the expected normal carry-forward.

Rejected alternatives:

- `NA-0401 -- QSL External Review / Public Claim Critical Boundary Resolution`
  because no critical blocker was found.
- `NA-0401 -- QSL External Review / Disclosure Source Verification Blocker Resolution`
  because source verification succeeded with caveats.
- Starting public technical paper now.
- Writing public docs now.
- Creating security.txt now.
- Changing website now.
- Contacting external reviewers now.
- Changing runtime/security code now.
- Making production/public-internet/external-review claims now.

## Future Path / Scope Bundle

Future NA-0401 normal successor allowed paths:

- `docs/governance/PROJECT_GOAL_AND_OPERATING_PRINCIPLES.md`
- `tests/NA-0401_project_goal_operating_principles_canon_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future forbidden unless explicitly authorized:

- runtime code;
- crypto implementation;
- qsc/qsp/qsl implementation;
- qshield runtime;
- qsl-server;
- qsl-attachments;
- workflows;
- dependencies;
- `Cargo.toml` / `Cargo.lock`;
- public docs / website;
- `README.md` / `START_HERE.md`;
- backup scripts / timers / fstab;
- response archives;
- public technical paper;
- public claims;
- secrets, credentials, private keys, passphrases, recovery envelopes, or live
  target setup.

## Public Claim / External Review / Website Boundary

NA-0400 is not external review.

NA-0400 is not disclosure policy implementation.

NA-0400 is not public technical paper work.

NA-0400 is not public docs or website work.

NA-0400 authorizes no production, public-internet, external-review-complete,
metadata/privacy strong, backup/restore/key completion, bug-free, or
perfect-crypto claim.

## Future Validation / Marker Plan

If the selected normal successor is implemented later, expected markers include:

- `NA0401_PROJECT_GOAL_CANON_AUTHORIZATION_OK`
- `NA0401_SECURITY_BEFORE_SPEED_OK`
- `NA0401_EVIDENCE_OVER_VIBES_OK`
- `NA0401_CODE_CRYPTO_EXCELLENCE_GOAL_OK`
- `NA0401_NO_PUBLIC_OVERCLAIMING_OK`
- `NA0401_ONE_READY_QUEUE_DISCIPLINE_OK`
- `NA0401_ROUTINE_AUDIT_RHYTHM_OK`
- `NA0401_EXTERNAL_AWARENESS_WITHOUT_HYPE_OK`
- `NA0401_PUBLIC_PAPER_TIMING_BOUNDARY_OK`
- `NA0401_DIRECTOR_CODEX_HUMAN_ROLE_BOUNDARY_OK`
- `NA0401_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0401_NO_RUNTIME_CHANGE_OK`
- `NA0401_NO_CRYPTO_IMPLEMENTATION_CHANGE_OK`
- `NA0401_NO_DEPENDENCY_CHANGE_OK`
- `NA0401_NO_WORKFLOW_CHANGE_OK`
- `NA0401_NO_SECRET_MATERIAL_OK`

## Future Project Goal / Operating Principles Canon Successor Rationale

The prior lanes have created enough supporting maps that QSL now needs a
single internal governance artifact for:

- north star and project goals;
- security before speed;
- evidence over vibes;
- code and crypto excellence;
- no public overclaiming;
- one-READY queue discipline;
- routine audits as operating rhythm;
- external awareness without hype;
- public technical paper timing;
- shorter and safer future directives;
- Director / Codex / human role boundaries.

This canon should remain internal governance unless a future directive
explicitly authorizes a public-doc/website path.

## Rejected Alternatives

- Treat source verification as external review.
- Treat the existing external review package as completed external review.
- Treat `SECURITY.md` and GitHub PVR as full coordinated disclosure readiness.
- Treat service-local qsl-server/qsl-attachments proof as production proof.
- Treat qshield demo proof as production proof.
- Start public technical paper drafting.
- Update README, START_HERE, docs/public, website, or public security policy.
- Add security.txt without an exact future policy lane.
- Select a blocker successor despite no blocker evidence.

## Backup-Plan Impact Statement

NA-0400 changes only qsl-protocol governance/testplan/traceability/journal
paths. No backup script, timer, fstab, source list, off-host target, real
backup, real restore, key, credential, passphrase, private key, or recovery
envelope path is changed.

Backup-plan update required: no.

Future durable external-review bundles, public-claim reports, public technical
paper artifacts, or security/disclosure policy artifacts outside ordinary
tracked governance paths require separate backup-impact review.

Same-host continuity remains local continuity only and must never be described
as complete disaster recovery.

## Next Recommendation

Close NA-0400 after validation and restore:

`NA-0401 -- QSL Project Goal and Operating Principles Canon Authorization Plan`

Do not implement NA-0401 in NA-0400.

## Source List

| Source | Authority | URL | Access date | Tier | Classification | Relevance |
|---|---|---|---|---|---|---|
| Coordinated Vulnerability Disclosure Program | CISA | https://www.cisa.gov/resources-tools/programs/coordinated-vulnerability-disclosure-program | 2026-06-01 | 1 | OFFICIAL_DISCLOSURE_GUIDANCE | CVD coordination and disclosure timing. |
| CERT Guide to Coordinated Vulnerability Disclosure | CERT/CC SEI | https://certcc.github.io/CERT-Guide-to-CVD | 2026-06-01 | 1 | OFFICIAL_DISCLOSURE_GUIDANCE | CVD roles and operations. |
| CERT Vulnerability Disclosure Guidance | CERT/CC | https://www.kb.cert.org/vuls/guidance/ | 2026-06-01 | 1 | OFFICIAL_DISCLOSURE_GUIDANCE | Reporter and coordinator guidance. |
| ISO/IEC 29147:2018 | ISO | https://www.iso.org/standard/72311.html | 2026-06-01 | 1 | OFFICIAL_DISCLOSURE_GUIDANCE | Vulnerability disclosure standard page. |
| ISO/IEC 30111:2019 | ISO | https://www.iso.org/standard/69725.html | 2026-06-01 | 1 | OFFICIAL_VULNERABILITY_HANDLING_GUIDANCE | Vulnerability handling standard page. |
| ISO/IEC TR 5895:2022 | ISO | https://www.iso.org/standard/81807.html | 2026-06-01 | 1 | OFFICIAL_VULNERABILITY_HANDLING_GUIDANCE | Multi-party CVD handling context. |
| PSIRT Services Framework v1.1 | FIRST | https://www.first.org/standards/frameworks/psirts/psirt_services_framework_v1.1 | 2026-06-01 | 1 | OFFICIAL_VULNERABILITY_HANDLING_GUIDANCE | Product security response framework. |
| RFC 9116 | RFC Editor / IETF | https://www.rfc-editor.org/rfc/rfc9116 | 2026-06-01 | 1 | OFFICIAL_SECURITY_POLICY_STANDARD | security.txt standard. |
| GitHub private vulnerability reporting docs | GitHub | https://docs.github.com/en/code-security/security-advisories/working-with-repository-security-advisories/configuring-private-vulnerability-reporting-for-a-repository | 2026-06-01 | 1 | OFFICIAL_SECURITY_POLICY_STANDARD | Repository private reporting channel. |
| NIST SP 800-218 SSDF | NIST CSRC | https://csrc.nist.gov/pubs/sp/800/218/final | 2026-06-01 | 1 | OFFICIAL_SECURE_DEVELOPMENT_GUIDANCE | Secure development / vulnerability response context. |
| NIST CSF 2.0 | NIST | https://www.nist.gov/cyberframework | 2026-06-01 | 1 | OFFICIAL_ASSURANCE_FRAMEWORK | Risk-management context. |
| OWASP ASVS | OWASP Foundation | https://owasp.org/www-project-application-security-verification-standard/ | 2026-06-01 | 2 | OFFICIAL_ASSURANCE_FRAMEWORK | Security verification checklist context. |
| OpenSSF Scorecard | OpenSSF | https://openssf.org/scorecard/ | 2026-06-01 | 2 | OFFICIAL_ASSURANCE_FRAMEWORK | Supply-chain/security posture metrics context. |
| SLSA | SLSA project | https://slsa.dev/ | 2026-06-01 | 2 | OFFICIAL_ASSURANCE_FRAMEWORK | Supply-chain integrity framework context. |
| OpenSSF Best Practices Badge | OpenSSF | https://openssf.org/projects/best-practices-badge/ | 2026-06-01 | 2 | OFFICIAL_ASSURANCE_FRAMEWORK | Voluntary best-practices self-certification context. |
| NA-0392 through NA-0399 evidence | QSL local evidence | `docs/governance/evidence/NA-0392_*` through `NA-0399_*` | 2026-06-01 | local | LOCAL_QSL_EVIDENCE | Cumulative prerequisite evidence. |
| qsl-server PR #56 | QSL sibling repo | https://github.com/QuantumShieldLabs/qsl-server/pull/56 | 2026-06-01 | local | LOCAL_QSL_EVIDENCE | Service-local harness boundary. |
| qsl-attachments PR #37 | QSL sibling repo | https://github.com/QuantumShieldLabs/qsl-attachments/pull/37 | 2026-06-01 | local | LOCAL_QSL_EVIDENCE | Service-local prerequisite boundary. |
