Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-30

# NA-0391 QSL External Standards / Threat / Technology Watch Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the NA-0391 authorization evidence for a future read-only,
source-cited external standards / threat / technology watch process. The lane
must define source taxonomy, source quality tiers, cadence, report/storage
policy, queue insertion rules, claim boundaries, and the selected NA-0392
successor without performing the first watch sweep and without changing
runtime, workflows, dependencies, public docs, backup scripts, qsl-server, or
qsl-attachments.

## Protected invariants

- READY_COUNT remains `1`.
- READY remains `NA-0391` until optional closeout.
- NA-0390 is DONE.
- D-0762 exists once.
- D-0763 exists once.
- D-0764 exists once after NA-0391 authorization.
- D-0765 remains absent until optional closeout.
- public-safety remains required and green.
- No runtime, service, protocol, crypto, auth, state-machine, qsc/qsp/qsl,
  qshield runtime, workflow, dependency, Cargo, qsl-server, qsl-attachments,
  qsc-desktop, website, docs/public, README, START_HERE, backup script, timer,
  fstab, source-list, system service, local tool, or response archive path
  changes.
- No secret handling, off-host setup, target setup, restore operation, backup
  mutation, public technical paper work, or public/readiness/privacy claim
  expansion.

## Allowed scope

- `docs/governance/evidence/NA-0391_qsl_external_standards_threat_technology_watch_authorization.md`
- `tests/NA-0391_qsl_external_standards_threat_technology_watch_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

Forbidden paths include `.github/**`, workflows, `Cargo.toml`, `Cargo.lock`,
runtime/service/protocol/crypto/auth/state-machine paths, qsc/qsp/qsl runtime
paths, qshield runtime, qsl-server, qsl-attachments, qsc-desktop, website,
docs/public, README, START_HERE, backup scripts/timers/fstab/source
lists/system services, qstart/qresume tooling, local history roots, durable
watch reports, response/request/directive/journal archive mutation, and
`/home/victor/work/qsl/codex/**` except the required final D210 response file.

## NA-0390 inheritance requirements

Evidence must record:

- PR #1043 merged the routine audit cadence helper.
- PR #1044 closed out NA-0390 and restored NA-0391.
- NA-0390 fixture matrix passed `42/42`.
- NA-0390 temp-output digest was recorded.
- NA-0390 represented external standards / threat / technology watch only as
  future-gated.
- NA-0390 did not perform an external watch, browse the web, create a durable
  audit report, create a scheduler/workflow, mutate runtime/dependencies, or
  expand public claims.

## Web/source discovery requirements

Targeted source discovery must cover at least:

- NIST/CSRC PQC standards and migration sources.
- NCSC or equivalent official PQC migration guidance.
- IETF/CFRG/RFC/Internet-Draft sources for TLS, HPKE, MLS, PQ hybrids, or
  protocol evolution.
- RustSec and GitHub Security Advisories.
- CVE/NVD or official vulnerability feeds.
- RustCrypto, OpenSSL, BoringSSL, OQS/liboqs, or related official project
  sources where relevant.
- IACR ePrint, Real World Crypto, USENIX Security, IEEE S&P, ACM CCS, NDSS, or
  equivalent research venues.
- Secure messaging and metadata privacy sources.
- Backup/restore/key custody tool sources such as restic, Borg, rclone, age,
  GnuPG, or OpenSSH where relevant.
- Public-claim, external-review, and vulnerability disclosure guidance.

If web discovery is unavailable, evidence must record
`WEB_SOURCE_DISCOVERY_BLOCKED` and select the blocker successor.

## Source inventory requirements

Each source inventory row must include:

- source name;
- publisher/authority;
- URL;
- access date;
- source tier;
- watch relevance;
- stability classification;
- cadence recommendation;
- claim-boundary implication.

Allowed stability labels:

- `FINAL_STANDARD`
- `OFFICIAL_GUIDANCE`
- `RFC`
- `INTERNET_DRAFT`
- `ADVISORY_FEED`
- `PEER_REVIEWED_RESEARCH`
- `PREPRINT`
- `PROJECT_RELEASE_NOTES`
- `VENDOR_CLAIM_LOW_CONFIDENCE`

## Source quality requirements

Evidence must define:

- Tier 1: final standards, RFCs, official guidance, official advisories.
- Tier 2: official project security/release notes, RustSec/GitHub advisories,
  NVD/CVE.
- Tier 3: peer-reviewed research and high-quality conference material.
- Tier 4: preprints and working-group drafts.
- Tier 5: vendor blogs, competitor claims, news, and marketing.

Evidence must state which tiers may justify blocker candidates, which tiers
should produce BACKLOG candidates only, how conflicts are handled, how drafts
are handled, and how vendor/competitor claims are handled.

## Watch domain requirements

Evidence must define watch domains for:

1. PQC standards and migration.
2. IETF/CFRG protocol evolution.
3. Rust/advisory/dependency health.
4. Secure messaging and metadata privacy.
5. Backup/restore/key custody.
6. External review and public claims.
7. Adjacent/competitor project context.
8. High-impact vulnerabilities and breaking research.

Each domain must specify scope, example sources, cadence, triggers, expected
output, queue insertion rules, public-claim implications, and stop conditions.

## Citation requirements

- Every current external claim must cite a source row or be clearly marked as
  an internal policy decision.
- Drafts, preprints, and vendor claims must be labeled with their stability
  class.
- Long quoted passages are forbidden.
- Source conflicts and citation gaps must be recorded rather than hidden.

## Queue insertion requirements

- External findings must not automatically create READY items.
- Findings may propose BACKLOG candidates or blocker candidates.
- Exactly one READY remains enforced.
- Official high/critical advisories that directly affect QSL may justify a
  blocker candidate.
- Draft/preprint/vendor findings must not auto-promote.

## Public technical paper boundary requirements

Evidence must keep the public technical paper future-gated and require, before
that paper starts:

- latest source-cited standards watch;
- current code/crypto audit status;
- current public-claim boundary audit;
- current external-review readiness assessment;
- current service, backup, restore, and key evidence status.

## Source conflict handling requirements

Evidence must require both conflicting sources to be recorded with tier, date,
stability, and affected domain. It must prefer final standards/RFCs/official
advisories over drafts, research, and vendor claims without making in-lane
implementation changes.

## Draft/preprint caveat requirements

Evidence must require Internet-Drafts and preprints to be rechecked by exact
version before implementation or public-claim use. They may shape BACKLOG
candidates but must not be treated as final standards.

## Report storage/backup requirements

Evidence must compare report storage options and require the first sweep to use
tracked qsl-protocol governance evidence plus optional temp proof. Durable
local watch report storage must remain forbidden unless separately authorized
with backup-impact review.

## First-lane authorization requirements

If targeted source discovery succeeds and report/citation/storage policy is
sufficient, selected decision marker must be:

`EXTERNAL_STANDARDS_THREAT_TECH_WATCH_FIRST_SWEEP_AUTHORIZATION_READY`

If discovery or report policy is insufficient, selected marker must be:

`EXTERNAL_STANDARDS_WATCH_BLOCKED_PENDING_SOURCE_OR_REPORT_POLICY`

NA-0391 must not implement NA-0392.

## Fail-closed requirements

Stop if:

- source discovery fails and cannot support authorization;
- unsupported current claims are introduced;
- drafts/preprints/vendor claims are treated as final standards;
- a full external watch sweep is performed;
- durable external-watch report storage is created;
- runtime/protocol/crypto/dependency/workflow/public-doc/backup/qsl-server or
  qsl-attachments changes are attempted;
- public/readiness/privacy/external-review claims are expanded.

## Public-claim boundary requirements

Evidence must state that source discovery and standards watch are not:

- implementation;
- external review;
- production readiness;
- public-internet readiness;
- metadata-free proof;
- anonymity proof;
- untraceability proof;
- complete disaster recovery;
- bug-free proof;
- perfect-crypto proof.

## Successor selection requirements

Expected successor if authorization succeeds:

`NA-0392 -- QSL External Standards / Threat / Technology Watch First Source-Cited Sweep`

Expected successor if blocked:

`NA-0392 -- QSL External Standards Watch Report Storage / Citation Policy Blocker Resolution`

Evidence must select exactly one.

## Required local checks

- `git diff --check`
- `python3 scripts/ci/qsl_routine_audit_cadence.py --help`
- representative local-ops helper fixture checks
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- qsc `send_commit`
- formal model checks
- qshield-cli test/build if feasible
- queue/decision helper proof
- scope guard over exact allowed paths
- markdown link check
- leak scan
- overclaim scan
- classifier/goal-lint/PR-body checks where applicable

## CI expectations

Required GitHub checks, including public-safety, must pass normally before
merge. No admin bypass, direct push, squash, rebase, force-push, amend after PR
creation, or branch deletion command is allowed.

## Successor handoff

If the authorization PR merges and post-merge public-safety remains green,
optional closeout may restore:

`NA-0392 -- QSL External Standards / Threat / Technology Watch First Source-Cited Sweep`

The closeout must not implement NA-0392 and must preserve one-READY queue
discipline.
