Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-31

# NA-0396 QSL Dependency / Advisory Watch Trigger Policy Plan

Goals: G1, G2, G3, G4, G5

Directive: QSL-DIR-2026-05-31-216

## Executive Summary

NA-0396 defines how QSL treats dependency and upstream security-advisory
signals from RustSec, GitHub Security Advisories, NVD/CVE, CISA KEV,
`cargo audit`, and official upstream security or release-note locations.

This lane is policy and queue-trigger planning only. It does not remediate
dependencies, update `Cargo.toml`, update `Cargo.lock`, mutate workflows, alter
runtime/protocol/crypto behavior, mutate sibling repositories, create a durable
advisory report outside this governance evidence, or expand public claims.

Current local posture at startup:

- `cargo audit --deny warnings` completed successfully for the current lockfile.
- `cargo tree -i rustls-webpki --locked` reported `rustls-webpki v0.103.13`
  through `rustls v0.23.36`, `reqwest`, `qsc`, `qsl-tui`, and `qshield-cli`.
- qsl-server PR #56 remains merged at `d40e6003fdf0` and was inspected
  read-only.
- qsl-attachments PR #37 remains merged at `96b9352bd63` and was inspected
  read-only.

Selected successor:

`NA-0397 -- QSL Code / Crypto Research Watch and Audit Follow-Up Plan`

No active dependency/advisory blocker is selected by NA-0396 because source
verification succeeded, local `cargo audit` remained green, and the live
`rustls-webpki` dependency is on the previously required safe line. This does
not prove the project is bug-free, vulnerability-free, or perfectly secure.

## Live NA-0396 Scope

Live `NEXT_ACTIONS.md` shows:

- Status: READY.
- Goals: G1, G2, G3, G4, G5.
- Objective: create a qsl-protocol governance plan defining how RustSec, GHSA,
  NVD/CVE, CISA KEV, `cargo audit`, and upstream project advisories should
  trigger queue candidates without automatic READY promotion, dependency
  changes, workflow changes, or runtime changes.

Allowed mutation for this lane is limited to:

- `docs/governance/evidence/NA-0396_qsl_dependency_advisory_watch_trigger_policy_plan.md`
- `tests/NA-0396_qsl_dependency_advisory_watch_trigger_policy_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden scope includes runtime code, qsc/qsp/qsl implementation paths,
qshield runtime, Cargo files, workflows, public docs, website, README,
START_HERE, qsl-server, qsl-attachments, qsc-desktop, backup scripts/timers,
fstab/services, local qstart/qresume tools, response archive mutation, secret
handling, and off-host setup.

Acceptance criteria:

1. READY_COUNT remains 1.
2. READY remains NA-0396 during this policy PR.
3. NA-0395 is DONE.
4. D-0772 and D-0773 exist once.
5. D-0774 is added once.
6. Official advisory sources are cited and mapped.
7. Trigger policy and queue-action rules are recorded.
8. Sibling-repo boundaries are recorded.
9. Selected successor is exact.
10. Required CI is green before merge.

Stop conditions include source-verification failure that prevents useful policy,
dependency or Cargo mutation, workflow mutation, runtime/protocol/crypto
mutation, sibling-repo mutation, secret handling, public-claim expansion,
multiple READY items, or any attempt to treat advisory policy as remediation.

## Inherited NA-0395 Rationale

NA-0395 selected NA-0396 because the PQC standards and IETF/CFRG RFC/draft
boundary maps were complete enough to move to the next external-watch group:
dependency and advisory trigger discipline.

NA-0395 established:

- TLS 1.3, HPKE, and MLS are not claimed as implemented QSL compliance.
- Active TLS/HPKE/CFRG/MLS PQ and hybrid drafts remain draft-watch inputs, not
  final-standard proof.
- qsc/Suite-2 has implementation and evidence but remains evidence-incomplete
  for public or external-review claims.
- Cargo audit and `rustls-webpki v0.103.13` were green and therefore supported
  a policy lane rather than an emergency dependency-remediation lane.

The future Project Goal / Operating Principles canon request remains a future
governance candidate only. It must not override the live NA-0396 scope or the
selected NA-0397 successor.

## Authoritative Advisory Source Verification

Source verification used targeted official source locations only. Advisory
source discovery is not external review and is not a public claim basis.

| Source | Authority | URL | Access date | Source priority tier | Classification | QSL relevance | Trigger implication |
|---|---|---|---|---|---|---|---|
| RustSec Advisory Database site | RustSec project / Rust Secure Code WG ecosystem | https://rustsec.org/ | 2026-05-31 | 1 | OFFICIAL_ADVISORY_FEED | Rust crate advisory front door and human-readable RustSec advisory source. | Primary Rust crate advisory source for future QSL advisory checks. |
| RustSec advisory-db repository | RustSec project | https://github.com/RustSec/advisory-db | 2026-05-31 | 1 | OFFICIAL_ADVISORY_DATABASE | Canonical RustSec advisory records for crates published through crates.io. | Cross-reference RUSTSEC IDs, affected package, patched range, informational type, withdrawn status, and aliases. |
| cargo-audit | RustSec project | https://github.com/RustSec/rustsec/tree/main/cargo-audit | 2026-05-31 | 2 | OFFICIAL_TOOLING_SOURCE | Audits `Cargo.lock` against RustSec advisory data. | Local red output is evidence requiring source-cited triage; green output is necessary health evidence with caveats. |
| GitHub Advisory Database | GitHub | https://github.com/advisories | 2026-05-31 | 3 | OFFICIAL_ADVISORY_DATABASE | CVE and GitHub-originated advisory database, including Rust ecosystem filters. | Cross-reference GHSA IDs, CVE aliases, severity, affected packages, withdrawn status, and API metadata. |
| GitHub global advisories REST API docs | GitHub Docs | https://docs.github.com/en/rest/security-advisories/global-advisories | 2026-05-31 | 3 | OFFICIAL_TOOLING_SOURCE | Documents fields such as GHSA ID, CVE ID, severity, vulnerable ranges, CVSS, CVSS severities, EPSS, and withdrawal metadata. | Use as structured correlation source, not as automatic local impact proof. |
| NVD vulnerabilities | NIST National Vulnerability Database | https://nvd.nist.gov/vuln | 2026-05-31 | 4 | OFFICIAL_VULNERABILITY_CATALOG | Official NVD CVE search and vulnerability metadata context. | CVE/NVD entries support severity and version-range context but must be tied back to Cargo.lock and reachability. |
| CVE Program | CVE Program / MITRE-sponsored program | https://www.cve.org/ | 2026-05-31 | 4 | OFFICIAL_VULNERABILITY_CATALOG | CVE identifiers provide common vulnerability names. | CVE identity alone is insufficient for local impact; use with NVD, GHSA, RustSec, and upstream notes. |
| CISA KEV catalog page | CISA | https://www.cisa.gov/known-exploited-vulnerabilities-catalog | 2026-05-31 | 5 | OFFICIAL_VULNERABILITY_CATALOG | Official catalog of known exploited vulnerabilities. | KEV relevance increases urgency and can justify blocker-candidate triage after local impact review. |
| CISA KEV JSON feed | CISA | https://www.cisa.gov/sites/default/files/feeds/known_exploited_vulnerabilities.json | 2026-05-31 | 5 | OFFICIAL_VULNERABILITY_CATALOG | Machine-readable KEV feed; read-only host fetch returned catalog version `2026.05.29`. | Use to verify CVE presence in KEV without treating KEV presence as automatic QSL reachability. |
| FIRST CVSS | FIRST | https://www.first.org/cvss/ | 2026-05-31 | supporting | OFFICIAL_TOOLING_SOURCE | Severity scoring reference commonly used by NVD/GHSA. | CVSS informs prioritization, not QSL-specific exploitability by itself. |
| FIRST EPSS | FIRST | https://www.first.org/epss/ | 2026-05-31 | supporting | OFFICIAL_TOOLING_SOURCE | Exploit-probability signal for CVEs where present. | EPSS can influence urgency, but local dependency and reachability review remains mandatory. |
| rustls security advisories | rustls project on GitHub | https://github.com/rustls/rustls/security/advisories | 2026-05-31 | 6 | OFFICIAL_PROJECT_SECURITY_NOTES | Official rustls maintainer advisory location. | Directly relevant when `rustls` versions in Cargo.lock are affected. |
| rustls releases | rustls project on GitHub | https://github.com/rustls/rustls/releases | 2026-05-31 | 6 | UPSTREAM_RELEASE_NOTES | Official rustls release notes. | Supports fix-version and behavior-change context; not a substitute for advisory triage. |
| rustls-webpki security advisories | rustls/webpki project on GitHub | https://github.com/rustls/webpki/security/advisories | 2026-05-31 | 6 | OFFICIAL_PROJECT_SECURITY_NOTES | Official webpki advisory location. | Directly relevant because QSL currently depends on `rustls-webpki v0.103.13`. |
| rustls-webpki releases | rustls/webpki project on GitHub | https://github.com/rustls/webpki/releases | 2026-05-31 | 6 | UPSTREAM_RELEASE_NOTES | Release 0.103.13 references fixes for GHSA-82j2-j2ch-gfr8 and URI name-constraint issues. | Confirms current lockfile is on the required patched line; still requires future monitoring. |
| RustCrypto organization | RustCrypto on GitHub | https://github.com/RustCrypto | 2026-05-31 | 6 | OFFICIAL_PROJECT_SECURITY_NOTES | Upstream home for many Rust crypto crates. | No single org-wide advisory feed was verified; use per-repository advisories/releases and RustSec/GHSA. |
| RustCrypto utils advisories | RustCrypto/utils on GitHub | https://github.com/RustCrypto/utils/security/advisories | 2026-05-31 | 6 | OFFICIAL_PROJECT_SECURITY_NOTES | Example per-repository RustCrypto advisory source. | Future findings affecting RustCrypto utility crates require per-crate impact review. |
| RustCrypto AEADs advisories | RustCrypto/AEADs on GitHub | https://github.com/RustCrypto/AEADs/security/advisories | 2026-05-31 | 6 | OFFICIAL_PROJECT_SECURITY_NOTES | Example per-repository AEAD advisory source. | Crypto misuse or plaintext-exposure advisories deserve security-boundary review. |
| OpenSSL vulnerabilities | OpenSSL Library project | https://openssl-library.org/news/vulnerabilities/ | 2026-05-31 | 6 | OFFICIAL_PROJECT_SECURITY_NOTES | Official OpenSSL vulnerability list and severity context. | Relevant only if a QSL dependency or environment uses OpenSSL; QSL must not infer impact without evidence. |
| OpenSSH security | OpenSSH project | https://www.openssh.org/security.html | 2026-05-31 | 6 | OFFICIAL_PROJECT_SECURITY_NOTES | Official OpenSSH security page. | Relevant to backup/ops/transport watch, not current qsl-protocol dependency impact unless used in scope. |
| OpenSSH release notes | OpenSSH project | https://www.openssh.org/releasenotes.html | 2026-05-31 | 6 | UPSTREAM_RELEASE_NOTES | Official release notes. | Release-note security changes can create future ops or dependency-watch candidates. |
| GnuPG download/release page | GnuPG project | https://gnupg.org/download/index.html | 2026-05-31 | 6 | UPSTREAM_RELEASE_NOTES | Official GnuPG release and lifecycle source. | Relevant to backup/key custody reference lanes, not current QSL dependency impact without direct use. |
| GnuPG announce archive | GnuPG project | https://lists.gnupg.org/pipermail/gnupg-announce/ | 2026-05-31 | 6 | OFFICIAL_PROJECT_SECURITY_NOTES | Official announcement archive for releases and security advisories. | Use for future key-custody/ops source verification when GnuPG is in scope. |
| liboqs advisories | Open Quantum Safe / liboqs | https://github.com/open-quantum-safe/liboqs/security/advisories | 2026-05-31 | 6 | OFFICIAL_PROJECT_SECURITY_NOTES | Official liboqs advisory location. | Relevant to PQC watch and any future liboqs use; no current QSL dependency impact is claimed here. |
| liboqs releases | Open Quantum Safe / liboqs | https://github.com/open-quantum-safe/liboqs/releases | 2026-05-31 | 6 | UPSTREAM_RELEASE_NOTES | Official liboqs release notes. | Release notes identify security issues and prototype limitations; future PQC watch input only. |
| Cargo yank semantics | Rust Cargo Book | https://doc.rust-lang.org/cargo/reference/publishing.html#cargo-yank | 2026-05-31 | supporting | OFFICIAL_TOOLING_SOURCE | Defines yanked-crate behavior for lockfiles and new resolution. | Yanked crate status requires impact review; it is not automatic exploit evidence. |
| cargo-yank command | Rust Cargo Book | https://doc.rust-lang.org/cargo/commands/cargo-yank.html | 2026-05-31 | supporting | OFFICIAL_TOOLING_SOURCE | Documents that yanking does not delete data and affects future resolution. | Yanked direct/runtime dependency may be a NEXT_ACTION_CANDIDATE or blocker candidate only after context review. |
| RustSec informational types | RustSec crate docs | https://docs.rs/rustsec/latest/rustsec/advisory/enum.Informational.html | 2026-05-31 | supporting | OFFICIAL_TOOLING_SOURCE | Defines notice, unmaintained, unsound, and other informational categories. | Informational advisories require policy handling distinct from vulnerability advisories. |

Citation gaps and uncertainty:

- GitHub security-advisory pages can display partial data without JavaScript;
  REST API fields should be used for future structured evidence.
- RustCrypto was verified as an official GitHub organization with
  per-repository advisory pages; no single official org-wide advisory feed was
  verified.
- CISA catalog verification through the web module was incomplete for the page
  body, but read-only host fetch of the official page and JSON feed succeeded.
- Source conflicts must be recorded rather than over-resolved. RustSec, GHSA,
  NVD, CISA KEV, and upstream notes can legitimately differ in timing,
  severity, affected range, or withdrawal state.

## Current Local Advisory / Dependency Posture

`cargo audit --deny warnings`:

- Completed successfully.
- Loaded RustSec advisory data and scanned the current `Cargo.lock`.
- Reported no blocking advisory finding for the current lockfile.

`cargo tree -i rustls-webpki --locked`:

- Reports `rustls-webpki v0.103.13`.
- Shows the dependency path through `rustls v0.23.36`, `hyper-rustls`,
  `reqwest`, `qsc`, `qsl-tui`, and `qshield-cli`.

`rustls-webpki` current status:

- Upstream `rustls/webpki` advisory and release pages identify recent
  advisories and fixes.
- The current lockfile is on `0.103.13`, the release line cited by NA-0395 as
  the expected safe version.
- This is advisory-health evidence only. It does not prove absence of all bugs
  or future advisories.

Current classification:

- Direct advisory status: `NO_ACTIVE_CARGO_AUDIT_FINDING_AT_STARTUP`.
- Transitive advisory status: `NO_ACTIVE_CARGO_AUDIT_FINDING_AT_STARTUP`.
- Yanked/unmaintained/notice status: no blocker selected by NA-0396 because
  local audit is green; future findings require source-cited impact review.
- qsl-server/qsl-attachments boundary: read-only evidence only; no sibling repo
  mutation occurred.

## RustSec and Advisory-DB Policy

RustSec is the primary Rust crate advisory source for QSL.

Required future evidence for a RustSec finding:

- RUSTSEC ID.
- advisory-db source URL or rustsec.org advisory URL.
- affected crate.
- affected version range.
- patched/unaffected range when present.
- informational type if present: notice, unmaintained, unsound, or other.
- withdrawn status if present.
- aliases such as GHSA or CVE.
- `Cargo.lock` version and direct/transitive/dev dependency position.
- local reachability and security-boundary review.

RustSec policy:

- CRITICAL/HIGH direct runtime/security dependency: `BLOCKER_CANDIDATE` after
  impact review; can become immediate STOP if the live directive would otherwise
  continue lower-priority work with an unresolved active security blocker.
- Medium/low direct dependency: `NEXT_ACTION_CANDIDATE` or
  `BACKLOG_CANDIDATE` after reachability review.
- Transitive-only dependency: `NEXT_ACTION_CANDIDATE` only after dependency
  tree and reachability review; otherwise `WATCH_ONLY` or `BACKLOG_CANDIDATE`.
- Dev dependency: require build/test/release-path context before selecting any
  blocker candidate.
- Informational unmaintained/unsound/notice: do not treat as panic evidence;
  map to `BACKLOG_CANDIDATE`, `NEXT_ACTION_CANDIDATE`, or
  `BLOCKER_CANDIDATE` based on safe-code unsoundness, crypto/security boundary,
  and local use.
- Withdrawn advisory: `NO_ACTION` or `CLAIM_BOUNDARY_ONLY` unless another
  official source still supports impact.

## GitHub Security Advisories Policy

GHSA is a cross-ecosystem advisory source and a useful RustSec/NVD correlation
source.

Required future evidence:

- GHSA ID.
- advisory type: reviewed, unreviewed, malware, or withdrawn.
- CVE aliases.
- ecosystem and package name.
- vulnerable version range and first patched version.
- severity, CVSS, CVSS v3/v4 when present.
- EPSS when present through official GitHub API data.
- source-code location and upstream repository advisory URL when present.
- RustSec alias or advisory-db linkage if present.

GHSA policy:

- GitHub-reviewed GHSA affecting an active direct runtime/security dependency
  with HIGH/CRITICAL severity: `BLOCKER_CANDIDATE` after local impact review.
- Unreviewed GHSA or NVD-imported item: `NEXT_ACTION_CANDIDATE` until
  corroborated by RustSec, NVD, or upstream.
- Withdrawn GHSA: no blocker; record as `WITHDRAWN` and preserve claim
  boundary only if a public statement might otherwise overstate health.
- Malware advisory: treat as `IMMEDIATE_STOP_CANDIDATE` only if the affected
  package/version is active in QSL build/runtime paths; otherwise create a
  source-cited candidate for impact review.

## NVD / CVE Policy

NVD/CVE provides vulnerability identifiers and metadata, not local impact by
itself.

Required future evidence:

- CVE ID.
- NVD detail URL.
- vulnerability status.
- CVSS vector and score if present.
- CPE/package mapping caveats.
- references to upstream, RustSec, GHSA, or vendor advisory sources.
- KEV status when applicable.
- local crate/package version and dependency position.

CVSS/EPSS caveats:

- CVSS expresses generic severity characteristics; it does not prove QSL
  reachability, exploitability, or exposure.
- EPSS estimates exploitation likelihood for a CVE when present; it does not
  replace local impact review.
- NVD CPE mappings can be broad or delayed for Rust crates. QSL should prefer a
  RustSec/GHSA/upstream package mapping when available and record conflicts.

NVD/CVE policy:

- CVE with CISA KEV presence and active direct dependency impact:
  `BLOCKER_CANDIDATE` after local impact review.
- CVE with HIGH/CRITICAL severity but uncertain Rust package mapping:
  `NEXT_ACTION_CANDIDATE` for mapping and reachability review.
- CVE without matching package/version in Cargo.lock: `NO_ACTION` or
  `CLAIM_BOUNDARY_ONLY`, with evidence of non-applicability.

## CISA KEV Policy

CISA KEV identifies vulnerabilities known to be exploited in the wild.

KEV policy:

- KEV presence increases urgency and should override ordinary backlog ordering
  when the affected package/version is active in QSL or a sibling repo.
- KEV does not automatically mean QSL is affected.
- KEV affecting an active direct runtime/security dependency:
  `BLOCKER_CANDIDATE` after source-cited local impact review.
- KEV affecting a transitive dependency: impact review must determine
  reachability and whether the vulnerable feature/path is used.
- KEV affecting qsl-server or qsl-attachments: propose a sibling-specific queue
  candidate; do not mutate sibling repos from qsl-protocol.

## cargo audit Linkage Policy

`cargo audit` is the local RustSec-linked lockfile scanner for qsl-protocol.

`cargo audit --deny warnings` red policy:

- Record exact command, exit status, advisory IDs, affected crates, versions,
  dependency tree, and suggested patched ranges.
- Cross-reference each advisory with RustSec advisory-db and, when available,
  GHSA/NVD/CVE/upstream sources.
- Classify each finding through the trigger matrix before any queue change.
- Do not run `cargo update` or dependency mutation unless a future directive
  explicitly authorizes exact files and remediation scope.

`cargo audit --deny warnings` green policy:

- Record as necessary dependency/advisory health evidence.
- Do not present it as bug-free, vulnerability-free, complete advisory
  resolution, dependency risk-free proof, external review, or perfect security.
- Still inspect high-signal upstream security notes when the directive scope
  asks for source verification.

False positive policy:

- False-positive/non-applicable classification requires source-cited evidence,
  local Cargo.lock/version evidence, and a reachability or feature-path reason.
- False-positive findings must not create blocker items.
- If uncertainty remains around security-boundary reachability, classify as
  `EVIDENCE_INCOMPLETE` and create a bounded impact-review candidate.

## Upstream Official Security / Release Notes Policy

Upstream security pages and release notes are required for security-sensitive
crypto/network components when relevant.

Policy:

- Treat upstream project advisories as first-party context for affected ranges,
  patched versions, severity rationale, and mitigation notes.
- Cross-reference upstream notes to RustSec/GHSA/NVD when package advisories
  exist.
- If upstream notes conflict with advisory databases, record the conflict and
  select the more conservative queue action until resolved.
- Release notes without explicit advisory status are `WATCH_ONLY` or
  `BACKLOG_CANDIDATE` unless they mention security, panic, constant-time,
  certificate validation, authentication, key schedule, RNG, side channel,
  memory safety, or exploitability in a dependency QSL uses.

Upstream examples verified in NA-0396:

- rustls/rustls: security advisories and releases.
- rustls/webpki: security advisories and releases, including 0.103.13.
- RustCrypto: official organization plus per-repository advisory pages.
- OpenSSL: official vulnerabilities page.
- OpenSSH: official security and release-note pages.
- GnuPG: official release page and announce archive.
- liboqs: official GitHub advisories and release notes.

## Trigger Policy Design

| Trigger | Source requirements | Severity | Confidence | Local impact review | Queue action | Stop condition | Public-claim implication |
|---|---|---|---|---|---|---|---|
| OFFICIAL_CRITICAL_DIRECT_DEPENDENCY_ADVISORY | RustSec/GHSA/NVD/upstream source plus Cargo.lock direct dependency proof | CRITICAL | High if source and lockfile match | Required before remediation | BLOCKER_CANDIDATE | Yes if continuing would ignore active security blocker | Must state advisory under review; no risk-free claim |
| OFFICIAL_HIGH_DIRECT_DEPENDENCY_ADVISORY | RustSec/GHSA/NVD/upstream source plus Cargo.lock direct dependency proof | HIGH | High if source and lockfile match | Required | BLOCKER_CANDIDATE or NEXT_ACTION_CANDIDATE | Yes if runtime/security boundary is active | Must caveat advisory posture |
| OFFICIAL_KEV_RELEVANT_ADVISORY | CISA KEV CVE plus source-cited package/version mapping | CRITICAL or HIGH by urgency | High for exploitation status, variable for QSL impact | Required | BLOCKER_CANDIDATE | Yes if active direct dependency impact is likely | No public assurance claim |
| TRANSITIVE_CRITICAL_OR_HIGH_ADVISORY | Advisory source plus `cargo tree` path | CRITICAL/HIGH | Medium until reachability is known | Required | NEXT_ACTION_CANDIDATE or BLOCKER_CANDIDATE | Only after active reachable path is likely | Explain transitive uncertainty |
| DEV_DEPENDENCY_ADVISORY | Advisory source plus dev/build/test dependency proof | CRITICAL/HIGH/MEDIUM | Medium | Required for release/build exposure | BACKLOG_CANDIDATE or NEXT_ACTION_CANDIDATE | Only if build/release integrity is affected | Do not imply runtime impact without proof |
| UNSOUND_ADVISORY | RustSec informational unsound or upstream soundness note | UNSOUND | Medium to high | Required for safe-code UB and security boundary | NEXT_ACTION_CANDIDATE or BLOCKER_CANDIDATE | Yes if active safe-code UB crosses crypto/security boundary | Caveat safe-code soundness |
| UNMAINTAINED_ADVISORY | RustSec informational unmaintained or upstream abandonment source | UNMAINTAINED | Medium | Required for role and alternatives | BACKLOG_CANDIDATE or NEXT_ACTION_CANDIDATE | No by default | Do not imply exploitability |
| YANKED_CRATE | Cargo/crates.io yanked status plus Cargo.lock proof | YANKED | Medium | Required for direct/transitive role | BACKLOG_CANDIDATE or NEXT_ACTION_CANDIDATE | No by default | Explain yanking is resolution/maintenance signal, not exploit proof |
| WITHDRAWN_ADVISORY | RustSec/GHSA withdrawn metadata | WITHDRAWN | High if official withdrawn status | Confirm no conflicting source | NO_ACTION or CLAIM_BOUNDARY_ONLY | No | Do not cite as active blocker |
| FALSE_POSITIVE_OR_NOT_APPLICABLE | Source-cited mismatch or reachability proof | FALSE_POSITIVE/NOT_APPLICABLE | High only with evidence | Required | NO_ACTION or CLAIM_BOUNDARY_ONLY | No | Record why not applicable |
| UPSTREAM_SECURITY_RELEASE | Official upstream release/security note | CRITICAL/HIGH/MEDIUM/LOW/NOTICE | Medium until package mapping is known | Required | WATCH_ONLY, BACKLOG_CANDIDATE, or NEXT_ACTION_CANDIDATE | Only if active dependency impact likely | Release note is not QSL remediation |
| UPSTREAM_PROTOCOL_SECURITY_NOTE | Official protocol/project note | NOTICE/MEDIUM/HIGH | Medium | Required for QSL protocol impact | WATCH_ONLY or NEXT_ACTION_CANDIDATE | Only if QSL semantics affected and scope authorized | Do not claim compliance or external review |
| ADVISORY_SOURCE_UNAVAILABLE | Source fetch or official source verification blocked | EVIDENCE_INCOMPLETE | Low | Required before proceeding | BLOCKER_CANDIDATE if needed source blocks policy | Yes if policy cannot be completed | Record source gap |
| CARGO_AUDIT_RED | Local `cargo audit --deny warnings` non-zero with advisories | CRITICAL/HIGH/MEDIUM/LOW/INFO | High for local lockfile finding | Required | NEXT_ACTION_CANDIDATE or BLOCKER_CANDIDATE | Yes for active direct HIGH/CRITICAL until triaged | No health overclaim |
| CARGO_AUDIT_GREEN_WITH_CAVEAT | Local `cargo audit --deny warnings` success | INFO | High for current RustSec lockfile scan | Not an impact review | CLAIM_BOUNDARY_ONLY or NO_ACTION | No | Necessary evidence, not proof of all safety |

## Queue Action Policy

Action definitions:

- `IMMEDIATE_STOP_REQUIRED`: stop the current directive because continuing
  would violate scope, security, truthfulness, or required evidence.
- `BLOCKER_CANDIDATE`: propose the next READY successor only after local impact
  review proves urgency; never create multiple READY items.
- `NEXT_ACTION_CANDIDATE`: future candidate appropriate for normal queue order.
- `BACKLOG_CANDIDATE`: documented future work without immediate READY claim.
- `WATCH_ONLY`: monitor source movement; no queue mutation by itself.
- `CLAIM_BOUNDARY_ONLY`: update or preserve caveats without claiming current
  remediation.
- `NO_ACTION`: no queue candidate because source is withdrawn, false positive,
  not applicable, or outside current scope.

Rules:

- No advisory finding automatically promotes READY.
- Exactly one READY item must exist.
- Active HIGH/CRITICAL direct dependency advisories may become blocker
  candidates only after local impact review.
- CISA KEV relevance increases urgency but does not skip local impact review.
- Transitive-only findings require dependency-path and reachability context.
- Dev-dependency findings require build, test, packaging, or release exposure
  context.
- Yanked and unmaintained findings are maintenance and resolution signals, not
  automatic exploit evidence.
- Withdrawn and false-positive findings must not create blocker items.
- Green `cargo audit` output is advisory health evidence with caveats.

## Advisory Impact Review Template

Future advisory triage should record:

- Advisory source and ID.
- Source title, publisher, URL, access date, source tier, and classification.
- Affected package or crate.
- Direct, transitive, or dev dependency position.
- Version in `Cargo.lock`.
- Affected version range and first patched/unaffected version.
- RustSec/GHSA/NVD/CVE/CISA/upstream cross-references.
- Yanked, informational, withdrawn, or false-positive status if applicable.
- Reachable code path: yes/no/unknown.
- Runtime exposure: yes/no/unknown.
- Cryptographic/security boundary: yes/no/unknown.
- qsl-server affected: yes/no/unknown.
- qsl-attachments affected: yes/no/unknown.
- Exploitability/KEV status.
- CVSS/EPSS metadata if present, with caveat.
- Mitigation options.
- Required future scope and exact allowed paths.
- Public-claim implication.
- Rollback, backup, deploy, and operator-impact notes.
- Proposed NEXT_ACTIONS item.
- Stop/no-stop decision and rationale.

## Sibling Repo Advisory Boundary

qsl-protocol may inspect qsl-server and qsl-attachments read-only when a
directive authorizes cross-repo evidence, but it must not mutate sibling repos
without a separate exact directive.

Sibling policy:

- qsl-protocol `cargo audit` covers qsl-protocol only.
- qsl-server advisory health must be proven in qsl-server.
- qsl-attachments advisory health must be proven in qsl-attachments.
- Service-local harness evidence remains service-local prerequisite evidence,
  not production, public-internet, or external-review proof.
- Advisory findings affecting qsl-server or qsl-attachments may propose a
  sibling-specific queue candidate, with repo, PR/state, affected package, and
  CI evidence.
- qsl-protocol must not silently claim sibling advisory health based only on
  qsl-protocol dependency checks.

## Public Claim / Paper Implications

Advisory policy is not dependency remediation.

Required claim boundaries:

- Advisory-source discovery is not external review.
- Green `cargo audit` is not proof of bug-free or perfect-security status.
- No production-ready, public-internet-ready, metadata-free, anonymity,
  untraceable, vulnerability-free, or perfect-crypto claim follows from this
  plan.
- Public technical paper work remains future-gated.
- If a future public paper references advisory posture, it must explain source
  coverage, local lockfile date, sibling-repo boundaries, and the caveat that
  advisory tools cannot prove absence of all defects.

## Future Queue Candidates

Future candidate lanes carried forward:

1. Dependency/advisory critical blocker lane, if an active blocker appears.
2. Routine advisory watch first sweep lane.
3. Code / Crypto Research Watch and Audit Follow-Up Plan.
4. Metadata Privacy / Secure Messaging Claim Boundary Plan.
5. Backup / Restore / Key Custody External Guidance Mapping Plan.
6. External Review / Disclosure / Public Claim Readiness Plan.
7. Project Goal / Operating Principles Canon Authorization Plan.
8. Director State Index Authorization Plan.
9. Public Technical Position Paper Evidence Prerequisite Plan.

## Selected Successor

Selected successor:

`NA-0397 -- QSL Code / Crypto Research Watch and Audit Follow-Up Plan`

Rationale:

- NA-0394 mapped PQC standards and migration boundaries.
- NA-0395 mapped IETF/CFRG RFC and draft boundaries.
- NA-0396 now maps dependency/advisory trigger policy.
- No active advisory blocker or source-verification blocker was selected.
- The next external-watch group should map code/crypto research and audit
  follow-up themes into future queue candidates without runtime, dependency,
  workflow, or public-claim changes.

Rejected alternatives:

- `NA-0397 -- QSL Dependency / Advisory Critical Finding Blocker Resolution`:
  rejected because no active blocker was found by current `cargo audit` and
  source verification.
- `NA-0397 -- QSL Dependency / Advisory Source Verification Blocker
  Resolution`: rejected because official source verification was sufficient to
  complete policy.

## Future Path / Scope Bundle

Future NA-0397 normal-successor allowed paths:

- `docs/governance/evidence/NA-0397_qsl_code_crypto_research_watch_audit_follow_up_plan.md`
- `tests/NA-0397_qsl_code_crypto_research_watch_audit_follow_up_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future forbidden scope:

- Dependency changes.
- `Cargo.toml` or `Cargo.lock` changes.
- Runtime code.
- Crypto implementation.
- qsc/qsp/qsl implementation.
- qshield runtime.
- qsl-server.
- qsl-attachments.
- Workflows.
- Public docs/website.
- Backup scripts/timers/fstab/services.
- Response archives.
- External claims.

Future NA-0397 may use targeted web only to cite and verify research or audit
sources if live scope authorizes it.

## Public Claim / External Review / Website Boundary

NA-0396 does not authorize:

- Website or public-doc updates.
- Public technical paper drafting.
- Production or public-internet readiness claims.
- External-review-complete claims.
- Metadata-free, anonymity, or untraceable claims.
- Bug-free, vulnerability-free, or perfect-crypto claims.
- Compliance or certification claims.

## Future Validation / Marker Plan

Future NA-0397 normal-successor markers:

- `NA0397_CODE_CRYPTO_RESEARCH_WATCH_PLAN_OK`
- `NA0397_IACR_EPRINT_REFERENCE_OK`
- `NA0397_REAL_WORLD_CRYPTO_REFERENCE_OK`
- `NA0397_USENIX_SECURITY_REFERENCE_OK`
- `NA0397_IEEE_SP_REFERENCE_OK`
- `NA0397_ACM_CCS_REFERENCE_OK`
- `NA0397_NDSS_REFERENCE_OK`
- `NA0397_PREPRINT_CAVEAT_OK`
- `NA0397_CODE_CRYPTO_AUDIT_FOLLOW_UP_OK`
- `NA0397_NO_RUNTIME_CHANGE_OK`
- `NA0397_NO_CRYPTO_IMPLEMENTATION_CHANGE_OK`
- `NA0397_NO_DEPENDENCY_CHANGE_OK`
- `NA0397_NO_WORKFLOW_CHANGE_OK`
- `NA0397_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0397_NO_PUBLIC_INTERNET_READY_CLAIM_OK`
- `NA0397_NO_BUG_FREE_CLAIM_OK`
- `NA0397_NO_PERFECT_CRYPTO_CLAIM_OK`
- `NA0397_NO_SECRET_MATERIAL_OK`

## Future Project Goal / Operating Principles Canon Carry-Forward Note

The requested `QSL Project Goal and Operating Principles Canon Authorization
Plan` remains useful and should be kept as a future governance candidate. It
should not override NA-0397 because the live sequence now favors code/crypto
research watch and audit follow-up after PQC standards, IETF/CFRG boundaries,
and dependency/advisory trigger policy.

## Rejected Alternatives

- Changing dependencies now.
- Running `cargo update` now.
- Running `cargo audit fix`.
- Changing workflows now.
- Mutating advisory/public-safety/helper scripts now.
- Writing public docs now.
- Starting the public technical paper now.
- Treating RustSec/GHSA/NVD/KEV discovery as external review.
- Treating green `cargo audit` output as bug-free or perfect-security proof.
- Automatically promoting more than one READY item based on advisory discovery.

## Backup-Plan Impact Statement

NA-0396 changes only tracked qsl-protocol governance evidence, testplan,
decision, traceability, and rolling-journal files. It changes no backup script,
timer, fstab, systemd unit, key, passphrase, restore path, recovery envelope,
remote destination, source list, or monitoring config.

No backup-plan update is required for NA-0396. Future durable advisory reports,
durable recurring watch outputs, dependency remediation artifacts, backup
source-list changes, real restore drills, key custody material, or off-host
target setup require separate backup-impact review.

The local `/backup/qsl` status remains same-host continuity only and must not be
presented as complete disaster recovery.

## Next Recommendation

Merge NA-0396 governance policy if validation and CI remain green. Then close
out NA-0396 and restore:

`NA-0397 -- QSL Code / Crypto Research Watch and Audit Follow-Up Plan`

Do not implement NA-0397 in NA-0396.

## Source List

The source list is the table in `Authoritative Advisory Source Verification`.
All sources were accessed on 2026-05-31. Stability/source classifications used:

- OFFICIAL_ADVISORY_FEED
- OFFICIAL_ADVISORY_DATABASE
- OFFICIAL_TOOLING_SOURCE
- OFFICIAL_PROJECT_SECURITY_NOTES
- OFFICIAL_VULNERABILITY_CATALOG
- UPSTREAM_RELEASE_NOTES
- EVIDENCE_INCOMPLETE
