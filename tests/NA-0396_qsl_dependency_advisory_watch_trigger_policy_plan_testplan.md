Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-31

# NA-0396 QSL Dependency / Advisory Watch Trigger Policy Plan Testplan

Goals: G1, G2, G3, G4, G5

Directive: QSL-DIR-2026-05-31-216

## Objective

Validate that NA-0396 records a qsl-protocol dependency/advisory watch trigger
policy for RustSec, GitHub Security Advisories, NVD/CVE, CISA KEV,
`cargo audit`, and upstream official security/release notes without changing
dependencies, workflows, runtime behavior, sibling repos, public docs, backup
configuration, response archives, or public claims.

## Protected Invariants

- READY_COUNT remains exactly one.
- READY remains NA-0396 until closeout.
- NA-0395 is DONE.
- D-0772 exists once.
- D-0773 exists once.
- D-0774 exists once after this PR.
- D-0775 is absent before closeout.
- No runtime, service, protocol, crypto, dependency, workflow, public docs,
  website, backup, response archive, qsl-server, qsl-attachments, qshield
  runtime, qstart/qresume, or secret-bearing path is changed.
- No advisory finding automatically promotes READY.
- No production, public-internet, metadata-free, anonymity, untraceable,
  vulnerability-free, bug-free, perfect-crypto, compliance, certification, or
  external-review-complete claim is introduced.

## Allowed Scope

- `docs/governance/evidence/NA-0396_qsl_dependency_advisory_watch_trigger_policy_plan.md`
- `tests/NA-0396_qsl_dependency_advisory_watch_trigger_policy_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

Forbidden changes include `.github/**`, workflows, `Cargo.toml`, `Cargo.lock`,
runtime/protocol/crypto implementation paths, qsc/qsp/qsl implementation,
qshield runtime, qsl-server, qsl-attachments, qsc-desktop, website, public docs,
README, START_HERE, backup scripts/timers/fstab/services, durable advisory
reports outside governance evidence, response archives, request/directive
history, qstart/qresume tooling, and helper script mutations.

## NA-0395 Inheritance Requirements

Verify the evidence:

- Records NA-0395 PR #1053 and PR #1054 context by dependency.
- Preserves that NA-0395 selected NA-0396 as the dependency/advisory watch
  trigger policy successor.
- Records that TLS, HPKE, MLS, and active PQ/hybrid drafts remain claim-boundary
  inputs and not QSL compliance proof.
- Carries forward the future Project Goal / Operating Principles canon lane as
  a future candidate only.

## Official Source Verification Requirements

Verify official sources are cited for:

- RustSec official site.
- RustSec advisory-db.
- cargo-audit / RustSec linkage.
- GitHub Advisory Database.
- GitHub global advisories API fields.
- NVD/CVE.
- CISA KEV page and JSON feed.
- rustls/rustls advisories and releases.
- rustls/webpki advisories and releases.
- RustCrypto organization and per-repository advisories.
- OpenSSL vulnerabilities.
- OpenSSH security and release notes.
- GnuPG releases and announce archive.
- liboqs advisories and releases.
- Cargo yank semantics.
- RustSec informational advisory categories.

## Source Citation Requirements

Each source citation must include:

- source title.
- publisher/authority.
- URL.
- access date.
- source priority tier or supporting status.
- stability/source classification.
- one-paragraph QSL relevance or concise table equivalent.
- trigger implication.

## Local Advisory Posture Requirements

Verify the evidence records:

- `cargo audit --deny warnings` success.
- `cargo tree -i rustls-webpki --locked` result.
- `rustls-webpki v0.103.13`.
- current direct advisory status.
- current transitive advisory status.
- qsl-server/qsl-attachments read-only boundary.
- caveat that green audit output is necessary evidence, not bug-free or
  perfect-security proof.

## cargo audit Linkage Requirements

Verify policy requires future red output to be correlated with:

- RustSec ID and advisory source.
- affected crate and Cargo.lock version.
- dependency tree position.
- GHSA/NVD/CVE/upstream aliases when present.
- direct/transitive/dev dependency classification.
- reachability and security-boundary impact review.
- no dependency mutation unless separately authorized.

## RustSec Policy Requirements

Verify policy covers:

- RUSTSEC IDs.
- advisory-db and rustsec.org source locations.
- affected and patched version ranges.
- withdrawn status.
- aliases.
- informational categories: notice, unmaintained, unsound, other.
- direct, transitive, and dev-dependency handling.

## GHSA Policy Requirements

Verify policy covers:

- GHSA ID.
- reviewed, unreviewed, malware, and withdrawn status.
- CVE aliases.
- ecosystem/package mapping.
- vulnerable version range and patched version.
- severity, CVSS, CVSS v3/v4, EPSS when present.
- no automatic local impact from GHSA metadata alone.

## NVD/CVE Policy Requirements

Verify policy covers:

- CVE identity.
- NVD status and metadata.
- CVSS/EPSS prioritization caveats.
- CPE/package mapping caveats.
- requirement to cross-reference RustSec/GHSA/upstream sources.

## CISA KEV Policy Requirements

Verify policy covers:

- KEV exploited-in-the-wild implication.
- CVE/package/version correlation.
- blocker-candidate rules for active direct dependency impact.
- transitive and sibling-repo caveats.
- no automatic QSL affected claim from KEV presence alone.

## Upstream Security Notes Requirements

Verify policy covers:

- rustls/rustls.
- rustls/webpki.
- RustCrypto per-repository advisories/releases.
- OpenSSL.
- OpenSSH.
- GnuPG.
- liboqs/Open Quantum Safe.
- source conflict recording.
- release-note versus advisory distinction.

## Trigger Classification Requirements

Verify every required trigger class appears:

- OFFICIAL_CRITICAL_DIRECT_DEPENDENCY_ADVISORY.
- OFFICIAL_HIGH_DIRECT_DEPENDENCY_ADVISORY.
- OFFICIAL_KEV_RELEVANT_ADVISORY.
- TRANSITIVE_CRITICAL_OR_HIGH_ADVISORY.
- DEV_DEPENDENCY_ADVISORY.
- UNSOUND_ADVISORY.
- UNMAINTAINED_ADVISORY.
- YANKED_CRATE.
- WITHDRAWN_ADVISORY.
- FALSE_POSITIVE_OR_NOT_APPLICABLE.
- UPSTREAM_SECURITY_RELEASE.
- UPSTREAM_PROTOCOL_SECURITY_NOTE.
- ADVISORY_SOURCE_UNAVAILABLE.
- CARGO_AUDIT_RED.
- CARGO_AUDIT_GREEN_WITH_CAVEAT.

Each trigger must define source requirements, severity, confidence, local impact
review, queue action, stop condition, and public-claim implication.

## Queue Action Requirements

Verify action definitions appear:

- IMMEDIATE_STOP_REQUIRED.
- BLOCKER_CANDIDATE.
- NEXT_ACTION_CANDIDATE.
- BACKLOG_CANDIDATE.
- WATCH_ONLY.
- CLAIM_BOUNDARY_ONLY.
- NO_ACTION.

Verify rules state:

- no automatic READY promotion.
- one READY must remain.
- active CRITICAL/HIGH direct advisory may become blocker candidate after impact
  review.
- CISA KEV increases urgency but does not skip impact review.
- transitive-only findings require reachability context.
- dev-dependency findings require build/test/release context.
- yanked/unmaintained does not automatically mean exploitability.
- withdrawn/false-positive findings must not create blockers.

## Impact Review Template Requirements

Verify the template includes:

- advisory source and ID.
- affected package/crate.
- direct/transitive/dev dependency.
- version in Cargo.lock.
- affected version range.
- reachable code path.
- runtime exposure.
- cryptographic/security boundary.
- qsl-server/qsl-attachments affected status.
- exploitability/KEV status.
- mitigation options.
- required future scope.
- public-claim implication.
- rollback/backup/deploy impact.
- proposed NEXT_ACTIONS item.
- stop/no-stop decision.

## Sibling Repo Boundary Requirements

Verify policy states:

- qsl-protocol may inspect qsl-server/qsl-attachments read-only when authorized.
- no sibling mutation occurs without separate exact directive.
- qsl-protocol cargo audit does not prove sibling advisory health.
- qsl-server and qsl-attachments advisory findings may propose sibling-specific
  queue candidates.
- service-local evidence remains not production or external-review proof.

## No Automatic READY Promotion Requirements

Verify advisory findings become explicit candidates only after policy review and
queue discipline. There must never be more than one READY item.

## Claim Boundary Requirements

Verify the evidence rejects:

- advisory policy as remediation.
- audit green as bug-free proof.
- advisory discovery as external review.
- vulnerability-free, perfect-security, production, public-internet, metadata,
  anonymity, untraceable, compliance, or certification overclaims.

## Public Paper Boundary Requirements

Verify public technical paper remains future-gated and must include advisory
health caveats if it later references dependency posture.

## Successor Selection Requirements

Expected normal successor:

`NA-0397 -- QSL Code / Crypto Research Watch and Audit Follow-Up Plan`

Verify blocker alternatives are rejected unless an active advisory blocker or
source-verification blocker is found:

- `NA-0397 -- QSL Dependency / Advisory Critical Finding Blocker Resolution`
- `NA-0397 -- QSL Dependency / Advisory Source Verification Blocker Resolution`

## Future Project Goal Canon Carry-Forward Requirements

Verify the future Project Goal / Operating Principles canon lane is mentioned as
a future governance candidate only and is not promoted over NA-0397.

## Backup-Impact Requirements

Verify the backup-impact statement records:

- only tracked qsl-protocol governance/testplan/decision/traceability/journal
  paths change.
- no backup scripts/timers/fstab/services/keys/targets/restore paths change.
- no backup-plan update is required for NA-0396.
- future durable advisory reports or recurring report stores require separate
  backup-impact review.
- same-host continuity is not complete disaster recovery.

## Required Local Checks

Run or record equivalent validation:

- `git status --porcelain=v1 --branch`
- `git diff --name-only origin/main...HEAD`
- `git diff --stat origin/main...HEAD`
- `git diff --check`
- `python3 scripts/ci/qsl_routine_audit_cadence.py --help`
- representative routine audit cadence fixture tests.
- `python3 scripts/ci/qsl_response_history_catalog.py --help`
- representative response history catalog fixture tests.
- `python3 scripts/ci/qsl_codex_response_writer.py --help`
- representative response writer fixture tests.
- `python3 scripts/ci/qsl_bounded_check_poll.py --help`
- representative bounded poll fixture tests.
- `python3 scripts/ci/qsl_directive_manifest_validate.py --help`
- representative manifest validator fixture tests.
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- metadata runtime JSON parse checks if directly runnable.
- metadata runtime no-secret harnesses if directly runnable.
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- qshield-cli test/build if feasible.
- queue/decisions helper checks.
- exact allowed-path scope guard.
- link-check.
- leak-scan.
- classifier proof.
- goal-lint.

## CI Expectations

- public-safety remains required.
- Required PR checks must pass before merge.
- Post-merge public-safety must pass.
- No admin bypass, direct push, squash, rebase, force-push, amend, or
  branch-deletion command is allowed.

## Successor Handoff

Packet Q must leave READY as NA-0396. Packet R closeout, if executed in a
separate PR after Packet Q merges and post-merge public-safety is green, may
mark NA-0396 DONE and restore exactly:

`NA-0397 -- QSL Code / Crypto Research Watch and Audit Follow-Up Plan`

NA-0397 must not be implemented by NA-0396 closeout.
