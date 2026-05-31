Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-30

# NA-0392 QSL External Standards / Threat / Technology Watch First Source-Cited Sweep Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0392 performs the first bounded, source-cited external standards / threat / technology watch sweep authorized by NA-0391, records durable governance evidence, preserves all runtime/security/public-claim boundaries, and selects an exact NA-0393 successor without implementing NA-0393.

## Protected Invariants

- Exactly one READY item remains present before closeout.
- NA-0392 remains READY until optional closeout.
- NA-0391 remains DONE.
- D-0764 and D-0765 remain present exactly once.
- D-0766 is absent at start and present exactly once after the NA-0392 evidence patch.
- No source finding auto-promotes READY.
- No public/readiness/privacy claim is strengthened.
- Source discovery is not external review.
- Source discovery is not production readiness.

## Allowed Scope

Allowed qsl-protocol paths:

- `docs/governance/evidence/NA-0392_qsl_external_standards_threat_technology_watch_first_sweep.md`
- `tests/NA-0392_qsl_external_standards_threat_technology_watch_first_sweep_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Optional temporary proof may remain under `/srv/qbuild/tmp/NA0392_external_watch_*`.

## Forbidden Scope

NA-0392 must not mutate:

- runtime, service, protocol, crypto, qsc, qsp, qsl, qshield runtime, qsl-server, qsl-attachments, qsc-desktop, website, public docs, README, START_HERE, Cargo files, dependency files, workflows, branch protection, public-safety configuration, backup scripts, timers, fstab, services, keys, passphrases, remote/off-host targets, response archives except the final D211 response file, local history roots, qstart/qresume tools, or durable external-watch reports outside authorized governance evidence.

## NA-0391 Inheritance Requirements

Verify the evidence records:

- inherited source tiers;
- inherited watch domains;
- inherited citation policy;
- inherited queue insertion rules;
- inherited public-claim boundaries;
- NA-0391 D-0764 authorization;
- NA-0391 D-0765 closeout and NA-0392 restoration.

## Web / Source Discovery Requirements

The sweep must use targeted read-only web/source discovery for current-source verification and citations. It must not download or execute code, interact with external services beyond read-only source pages, or search unrelated topics.

If web/source discovery is unavailable enough to prevent a useful source-cited sweep, record `WEB_SOURCE_DISCOVERY_BLOCKED` and select the storage/citation/policy blocker successor.

## Citation Requirements

Each source category must record:

- source title;
- publisher or authority;
- URL;
- access date;
- watch category;
- source tier;
- stability classification;
- relevance to QSL;
- citation gaps or uncertainty where applicable.

Long source passages must not be copied. Evidence should paraphrase source relevance.

## Source Tier Requirements

Verify the report distinguishes:

- Tier 1 final standards, RFCs, official guidance, and official advisories.
- Tier 2 project security/release notes and advisory feeds.
- Tier 3 peer-reviewed research and high-quality conference material.
- Tier 4 preprints and Internet-Drafts.
- Tier 5 vendor, competitor, news, or marketing claims.

## Source Stability Classification Requirements

Verify use of:

- `FINAL_STANDARD`
- `OFFICIAL_GUIDANCE`
- `RFC`
- `INTERNET_DRAFT`
- `ADVISORY_FEED`
- `PEER_REVIEWED_RESEARCH`
- `PREPRINT`
- `PROJECT_RELEASE_NOTES`
- `VENDOR_CLAIM_LOW_CONFIDENCE`

Drafts, preprints, and vendor claims must not be treated as final.

## PQC Standards Requirements

Verify coverage of:

- NIST FIPS 203 / ML-KEM.
- NIST FIPS 204 / ML-DSA.
- NIST FIPS 205 / SLH-DSA.
- NIST/NCCoE migration guidance.
- NCSC migration guidance/timeline.
- CISA quantum readiness or CISA/NCCoE fact sheet.
- NSA CNSA 2.0.
- HQC or backup-algorithm status if official sources confirm relevance.

Expected invariant: final standards do not prove QSL implementation compliance or deployment readiness.

## IETF / CFRG Requirements

Verify coverage of:

- RFC 8446 TLS 1.3.
- RFC 9180 HPKE.
- RFC 9420 MLS.
- relevant IETF/CFRG PQ hybrid drafts.

Expected invariant: RFCs are stable references; Internet-Drafts are awareness inputs unless finalized.

## Rust / Advisory / Dependency Requirements

Verify coverage of:

- RustSec.
- RustSec advisory DB.
- GitHub Security Advisories.
- NVD/CVE.
- CISA KEV.
- relevant upstream crypto/security project security/release notes.
- local `cargo audit --deny warnings`.
- local `cargo tree -i rustls-webpki --locked`.

Expected invariant: no dependency update or Cargo mutation occurs in NA-0392.

## Code / Crypto Research Requirements

Verify coverage of:

- IACR ePrint.
- Real World Crypto.
- USENIX Security.
- IEEE S&P.
- ACM CCS.
- NDSS.
- selected preprint sources only as preprints.

Expected invariant: this is a research watch category, not a complete literature review and not external review proof.

## Secure Messaging / Metadata Privacy Requirements

Verify coverage of:

- secure messaging protocol docs/specs where relevant;
- metadata privacy, traffic analysis, timing, cover traffic, batching, padding, or comparable topics;
- Signal PQXDH, Double Ratchet, sealed sender, or equivalent sources if relevant;
- XRD or comparable research if relevant.

Expected invariant: metadata-free behavior, anonymity, and untraceability remain not claimed.

## Backup / Restore / Key Custody Requirements

Verify coverage of:

- restic;
- Borg;
- rclone;
- age;
- GnuPG/OpenSSH official security/release sources where relevant;
- credible backup/restore/key custody references.

Expected invariant: same-host continuity is not disaster recovery; off-host backup complete, restore proven, and key custody implementation remain not claimed.

## Public-Claim / External-Review Requirements

Verify coverage of:

- CISA coordinated vulnerability disclosure guidance.
- OpenSSF guidance/tools where relevant.
- OWASP ASVS or comparable assurance reference where relevant.

Expected invariant: external-review-complete and production-readiness claims remain not made; no website or public docs are changed.

## Findings Matrix Requirements

Verify the matrix records:

- finding ID;
- domain;
- source tier and stability;
- severity;
- QSL relevance;
- affected lane;
- current QSL evidence status;
- public-claim implication;
- recommended action;
- proposed future queue candidate;
- blocker yes/no;
- rationale.

## Queue Candidate Requirements

Verify no finding auto-promotes READY. Future queue candidates must be explicit recommendations only.

Blocker successor is selected only if an official high-impact advisory or standards change creates an immediate blocker. Storage/citation blocker successor is selected only if safe source-cited evidence cannot be completed.

## Public Technical Paper Boundary Requirements

Verify public technical paper implications are recorded only as future prerequisites. NA-0392 must not draft a public paper or create public docs.

## Report Storage / Backup Requirements

Verify:

- evidence document is durable qsl-protocol governance evidence;
- optional proof remains under `/srv/qbuild/tmp`;
- no durable external-watch report exists outside authorized governance evidence;
- no backup-plan update is required for governance-only evidence;
- future durable external-watch report stores require separate backup-impact review.

## Fail-Closed Requirements

Stop if:

- source discovery cannot support a useful source-cited sweep;
- live NA-0392 scope conflicts with the directive;
- forbidden paths are touched;
- unsupported external claims are introduced;
- drafts, preprints, or vendor claims are treated as final;
- source discovery is presented as external review or production readiness;
- any validation failure would require out-of-scope edits;
- more than one READY item exists.

## Public-Claim Boundary Requirements

High-risk phrases are allowed only when negated, prohibited, caveated, future-bound, or exact bounded source-watch wording. Unsafe matches must be zero.

## Successor Selection Requirements

Expected normal successor:

`NA-0393 -- QSL External Standards / Threat Watch Findings Triage and Queue Candidate Plan`

Alternative blocker successor:

`NA-0393 -- QSL External Standards / Threat Watch Critical Finding Blocker Resolution`

Alternative storage/citation blocker successor:

`NA-0393 -- QSL External Standards Watch Report Storage / Citation Policy Blocker Resolution`

NA-0393 must not be implemented by this testplan.

## Required Local Checks

Run or record:

- `git status --porcelain=v1 --branch`
- `git diff --name-only origin/main...HEAD`
- `git diff --stat origin/main...HEAD`
- `git diff --check`
- helper `--help` checks for local ops scripts;
- representative fixture tests for routine audit cadence, response history catalog, response writer, bounded polling, and directive manifest validation;
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- metadata runtime JSON parse checks;
- metadata runtime no-secret harnesses if directly runnable;
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- qshield-cli test/build if feasible;
- queue and decisions proof;
- scope guard with exact allowed paths;
- link-check;
- leak-scan;
- classifier proof;
- goal-lint or PR-body goal proof.

## CI Expectations

Required qsl-protocol CI must pass normally before merge. `public-safety` must remain required and green before merge and after merge.

No admin bypass, direct push, squash, rebase, force-push, amend-after-PR, branch deletion command, or delete-branch flag is authorized.

## Successor Handoff

If Packet R merges and public-safety is green, optional closeout may mark NA-0392 DONE and restore the exact selected NA-0393 successor as READY. Closeout must not implement NA-0393.
