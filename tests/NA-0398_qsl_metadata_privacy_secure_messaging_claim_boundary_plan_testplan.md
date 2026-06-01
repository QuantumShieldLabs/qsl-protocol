Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-01

# NA-0398 QSL Metadata Privacy / Secure Messaging Claim Boundary Plan Testplan

Goals: G1, G2, G3, G4, G5

Directive: QSL-DIR-2026-06-01-218

## Objective

Validate that NA-0398 records a qsl-protocol secure messaging and metadata
privacy claim-boundary plan without implementing privacy features, changing
runtime behavior, changing cryptography, changing dependencies, mutating
workflows, mutating sibling repositories, updating public docs/website,
changing backup configuration, handling secrets, or expanding public claims.

## Protected Invariants

- READY_COUNT remains exactly one.
- READY remains NA-0398 until closeout.
- NA-0397 is DONE.
- D-0776 exists once.
- D-0777 exists once.
- D-0778 exists once after this PR.
- D-0779 is absent before closeout.
- No runtime, service, protocol, crypto, dependency, Cargo, workflow,
  public-doc, website, backup-script, qsl-server, qsl-attachments, qshield
  runtime, qstart/qresume, response archive, local tool, or secret-bearing
  path is changed.
- Source discovery is not external review.
- Planning is not implementation.
- Service-local evidence is not production proof.
- qshield demo evidence is not production proof.
- No metadata-free, anonymity, untraceable, hidden timing, hidden traffic
  shape, hidden attachment size, sender-hidden, recipient-hidden, social
  graph-hidden, complete privacy, production-ready, public-internet-ready,
  bug-free, perfect-crypto, or external-review-complete claim is introduced.

## Allowed Scope

- `docs/governance/evidence/NA-0398_qsl_metadata_privacy_secure_messaging_claim_boundary_plan.md`
- `tests/NA-0398_qsl_metadata_privacy_secure_messaging_claim_boundary_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

Forbidden changes include `.github/**`, workflows, `Cargo.toml`, `Cargo.lock`,
runtime/protocol/crypto implementation paths, qsc/qsp/qsl implementation,
qshield runtime, qsl-server, qsl-attachments, qsc-desktop, website, docs/public,
README, START_HERE, backup scripts/timers/fstab/services, durable
metadata/privacy reports outside governance evidence, response archives,
request/directive/history roots, qstart/qresume tooling, helper script
mutations, off-host setup, and secret handling.

## NA-0397 Inheritance Requirements

Verify the evidence records:

- PR #1057 merge `38b4da16362e`.
- PR #1058 merge `9b6d1c14f938`.
- origin/main handoff SHA `9b6d1c14f93888d1cd9184da1036541299b47f4f`.
- READY_COUNT 1 and READY NA-0398.
- NA-0397 DONE.
- D-0776 and D-0777 presence.
- D-0778 absence before NA-0398.
- cargo audit green and `rustls-webpki v0.103.13`.
- code/crypto research-watch findings remain future audit candidates.
- future Project Goal / Operating Principles canon lane is carry-forward only.

## Official Source Verification Requirements

Verify official source categories are cited for:

- Signal X3DH.
- Signal Double Ratchet.
- Signal PQXDH.
- Signal Sealed Sender source if available.
- RFC 9420 / MLS as context only.
- RFC 6973 Privacy Considerations for Internet Protocols.
- Traffic-analysis and metadata-privacy research category.
- Cover traffic, batching, padding, and timing research category.
- XRD or NA-0392-named metadata privacy source when present.

## Source Citation Requirements

Each citation must include:

- source title.
- publisher/authority.
- URL.
- access date.
- source tier.
- source classification.
- relevance to QSL.
- claim-boundary implication.

Protocol specs, RFCs, peer-reviewed research, research workshops, preprints,
and vendor/competitor claims must be distinguished. Vendor or public narrative
claims must not be primary evidence.

## Prior Evidence Intake Requirements

Verify the evidence records:

- NA-0392 secure messaging / metadata privacy watch evidence.
- NA-0393 triage status.
- NA-0395 RFC/MLS boundary status.
- NA-0397 code/crypto planning dependency.
- qsl-server PR #56 boundary.
- qsl-attachments PR #37 boundary.
- qshield demo boundary.
- metadata runtime harnesses and governance evidence.
- what can be claimed internally.
- what cannot be claimed publicly.
- which items are service-local, demo-only, harness-only, or governance-only.

## Read-Only Scan Requirements

Verify read-only scans cover current paths for:

- metadata.
- privacy.
- anonymous / anonymity / untraceable.
- traffic / timing / traffic shape.
- padding / bucket / cover traffic / batching / jitter.
- relay.
- sealed / sender / recipient.
- delivery / receipt / presence.
- attachment / size.
- storage / retention / log / sanitized.
- IP / network.
- social graph / contact graph.
- queue / retry / rate.

Search hits must be planning evidence only and must not be treated as proof of
a bug, exploit, privacy property, or implementation.

## Metadata Axis Matrix Requirements

Verify the matrix covers:

1. Content confidentiality.
2. Sender identity metadata.
3. Recipient identity metadata.
4. Social/contact graph metadata.
5. Timing metadata.
6. Traffic shape / volume.
7. Attachment size metadata.
8. Network/IP metadata.
9. Delivery/read/receipt/presence metadata.
10. Storage/logging/retention metadata.
11. Cover traffic / batching / padding / jitter.
12. Metadata privacy versus anonymity/untraceability distinction.

Each row must include source basis, QSL current evidence, evidence class,
confidence, allowed claim, forbidden claim, missing evidence, future lane, and
priority.

## Content Confidentiality Boundary Requirements

Verify the evidence:

- separates content confidentiality from metadata privacy.
- records QSL-specific content confidentiality evidence only as bounded
  internal evidence.
- states that content encryption does not imply metadata hiding.
- records future audit/external-review evidence needs.

## Sender / Recipient / Social Graph Boundary Requirements

Verify the evidence:

- records relay/server visibility caveats.
- records qsl-server service-local boundary.
- records qshield demo boundary.
- records no anonymity or untraceable claim.
- records future qsl-server relay/logging/identifier evidence needs.

## Timing / Traffic / Volume Boundary Requirements

Verify the evidence:

- records send/receive timing, retry cadence, queueing, message count, and
  volume as separate metadata axes.
- records qshield timing/traffic evidence as bounded demo/harness evidence.
- records qsl-server/qsl-attachments production timing as unproven.
- forbids hidden-timing and hidden-traffic-shape claims.

## Attachment-Size / Padding Boundary Requirements

Verify the evidence:

- records qsl-attachments service-local evidence.
- records qshield demo padding and attachment-size class evidence.
- states no hidden attachment-size claim unless exact future evidence exists.
- records future qsl-attachments evidence needs.

## Network / IP / Transport Boundary Requirements

Verify the evidence:

- states QSL does not automatically hide IP/network metadata.
- does not imply Tor, VPN, proxy, mixnet, or transport privacy protection.
- records server/relay visibility caveats.
- records future transport/privacy evidence needs.

## Storage / Logging / Retention Boundary Requirements

Verify the evidence:

- records sanitized-error and retention/purge evidence if present.
- records qsl-server and qsl-attachments service boundaries.
- records local-ops logs as operational evidence only.
- forbids no-metadata-stored and no-metadata-logged claims.

## Cover Traffic / Batching / Padding / Jitter Boundary Requirements

Verify the evidence:

- distinguishes modeled/planned/demo/harness evidence from implementation.
- records no production cover-traffic claim.
- records no broad batching/padding/jitter claim.
- records cost, abuse, latency, and service-production evidence gaps.

## Claim Language Policy Requirements

Verify the evidence defines allowed and forbidden language.

Allowed examples must stay caveated:

- metadata privacy is under active evidence mapping.
- specific metadata axes remain not claimed or evidence-incomplete.
- service-local harness evidence is bounded and not production proof.

Forbidden language must be negated, prohibited, or future/unproven:

- metadata-free.
- anonymous / anonymity.
- untraceable.
- hides all metadata.
- hides timing.
- hides traffic shape.
- hides attachment size.
- sender-hidden.
- recipient-hidden.
- social-graph-hidden.
- production-ready.
- public-internet-ready.
- externally reviewed.
- complete privacy.

## Future Queue Candidate Requirements

Verify the evidence records future candidates for:

1. Backup / Restore / Key Custody External Guidance Mapping Plan.
2. External Review / Disclosure / Public Claim Readiness Plan.
3. Metadata Privacy Evidence Gap / Claim Scanner Plan.
4. qsl-server Metadata Logging / Relay Visibility Audit Plan.
5. qsl-attachments Size / Padding / Retention Evidence Plan.
6. qshield Demo / Production Claim Boundary Plan.
7. Public Technical Position Paper Evidence Prerequisite Plan.
8. Project Goal / Operating Principles Canon Authorization Plan.
9. Director State Index Authorization Plan.

For each, the evidence must include source/evidence basis, why next or not
next, likely allowed scope, likely forbidden scope, and public-claim
implication.

## No Implementation Requirements

Verify no file changes occur outside the allowed governance/testplan paths.
The lane must not:

- implement metadata/privacy features.
- change qshield runtime.
- change qsl-server.
- change qsl-attachments.
- change docs/public or website.
- change code, crypto, dependencies, Cargo files, or workflows.
- create durable metadata/privacy reports outside governance evidence.

## Public Claim Boundary Requirements

Verify the evidence:

- states metadata/privacy planning is not implementation.
- states source discovery is not external review.
- forbids metadata-free, anonymity, untraceable, hidden timing, hidden traffic
  shape, hidden attachment size, production, public-internet, bug-free,
  perfect-crypto, and external-review-complete claims.
- records no website/public docs update.

## Public Paper Boundary Requirements

Verify public technical position paper work remains future-gated until
metadata/privacy, code/crypto, dependency/advisory, PQC/IETF, backup/key,
service-boundary, external-review, and public-claim evidence are stronger.

## Successor Selection Requirements

Verify the exact selected successor is:

`NA-0399 -- QSL Backup / Restore / Key Custody External Guidance Mapping Plan`

Unless a blocker is found, reject:

- `NA-0399 -- QSL Metadata Privacy / Public Claim Critical Boundary Resolution`
- `NA-0399 -- QSL Metadata Privacy / Secure Messaging Source Verification Blocker Resolution`

NA-0399 must not be implemented by NA-0398.

## Future Project Goal Canon Carry-Forward Requirements

Verify the future Project Goal / Operating Principles canon lane is carried
forward only as a future governance candidate and does not override NA-0398 or
the selected NA-0399 successor.

## Backup-Impact Requirements

Verify:

- no backup-plan update is required if changed paths are limited to
  qsl-protocol governance/testplan files.
- same-host continuity is not described as complete disaster recovery.
- future durable metadata/privacy reports or backup/key/restore outputs require
  separate backup-impact review.
- no backup scripts, timers, fstab, services, keys, passphrases, restore paths,
  remote destinations, or monitoring configs are mutated.

## Required Local Checks

Recommended checks:

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard ...`
- `python3 scripts/ci/qsl_evidence_helper.py link-check --root .`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- helper help/fixture checks for local-ops scripts.
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- targeted metadata-runtime JSON/harness checks as feasible.
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- qshield-cli build/test as feasible.
- goal-lint or PR body preflight with standalone `Goals: G1, G2, G3, G4, G5`.

## CI Expectations

The PR must pass required checks normally, including public-safety and
goal-lint. No admin bypass, squash, rebase, direct push, force-push,
amend-after-PR, branch-deletion command, or branch-protection mutation is
allowed.

## Successor Handoff

After this plan merges and post-merge public-safety is green, optional closeout
may mark NA-0398 DONE and restore exactly one READY successor:

`NA-0399 -- QSL Backup / Restore / Key Custody External Guidance Mapping Plan`

The closeout must not implement NA-0399.
