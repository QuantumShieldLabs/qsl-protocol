Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-01

# NA-0398 QSL Metadata Privacy / Secure Messaging Claim Boundary Plan

Goals: G1, G2, G3, G4, G5

Directive: QSL-DIR-2026-06-01-218

## Executive Summary

NA-0398 maps current QSL evidence and public-claim boundaries against secure
messaging and metadata privacy source categories. It is a governance and
claim-boundary plan only.

This lane does not implement metadata/privacy features, change runtime code,
change qshield runtime, change cryptography, change dependencies, change
workflows, mutate qsl-server, mutate qsl-attachments, mutate qsc-desktop,
mutate website/public docs, change backup scripts/timers/fstab, handle
secrets, or strengthen public claims.

The conservative outcome is:

- QSL may continue to treat content confidentiality as a protocol and
  implementation goal with existing QSL-specific evidence, but it must not
  infer metadata privacy from content encryption.
- Sender identity, recipient identity, contact graph, timing, traffic shape,
  attachment size, IP/network metadata, delivery/receipt/presence, and
  storage/logging/retention are distinct axes with mixed evidence classes.
- qshield metadata-runtime evidence is bounded demo/harness evidence.
- qsl-server PR #56 and qsl-attachments PR #37 remain service-local evidence,
  not production, public-internet, or external-review proof.
- Metadata-free, anonymity, untraceability, hidden timing, hidden traffic
  shape, hidden attachment size, sender-hidden, recipient-hidden, social
  graph-hidden, and complete privacy remain not claimed.

Selected successor:

`NA-0399 -- QSL Backup / Restore / Key Custody External Guidance Mapping Plan`

Rationale: after metadata/privacy claim boundaries, the next external-watch
group is backup, restore, and key custody guidance. That topic remains a major
evidence and public-claim prerequisite and is still operationally blocked by
off-host/operator-input gaps.

## Live NA-0398 Scope

Live `NEXT_ACTIONS.md` shows:

- Status: READY.
- Goals: G1, G2, G3, G4, G5.
- Objective: create a qsl-protocol governance plan that maps secure messaging
  and metadata privacy sources into explicit claim boundaries for
  metadata-free behavior, anonymity, untraceability, timing, traffic-shape,
  cover-traffic, batching, padding, and public technical-paper readiness,
  without runtime, dependency, workflow, or public-claim changes.

Allowed qsl-protocol mutation for this lane is limited to:

- `docs/governance/evidence/NA-0398_qsl_metadata_privacy_secure_messaging_claim_boundary_plan.md`
- `tests/NA-0398_qsl_metadata_privacy_secure_messaging_claim_boundary_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Allowed source verification:

- Targeted read-only verification of official secure messaging protocol
  sources, RFCs, metadata/privacy research categories, and NA-0392-named
  secure messaging / metadata privacy references.

Allowed read-only repo scan scope:

- Bounded `rg` searches for metadata, privacy, relay, padding, cover traffic,
  batching, timing, traffic shape, attachment size, identifiers, errors,
  retention, qshield, qsl-server, and qsl-attachments boundary evidence.
- Existing validation checks and harnesses used only as evidence boundaries.

Forbidden scope:

- `.github/**`, workflows, `Cargo.toml`, `Cargo.lock`, dependency updates,
  qsc/qsp/qsl runtime implementation, qshield runtime, qsl-server,
  qsl-attachments, qsc-desktop, website, docs/public, README, START_HERE,
  backup scripts/timers/fstab/services, response archive/history mutation,
  qstart/qresume tooling, helper script mutation, off-host setup, secret
  handling, and any public-claim expansion.

Acceptance criteria:

1. READY_COUNT remains 1.
2. READY remains NA-0398 until closeout.
3. NA-0397 is DONE.
4. D-0776 and D-0777 exist once.
5. D-0778 is added once by this plan.
6. Secure messaging and metadata/privacy sources are cited and classified.
7. Metadata axes are mapped with conservative evidence classes.
8. Claim boundaries are recorded.
9. Future candidates are recorded.
10. Exact NA-0399 successor is selected.
11. Required validation and CI are green before merge.

Stop conditions include source verification unavailable, code/crypto/runtime
mutation, dependency or Cargo mutation, workflow mutation, sibling-repo
mutation, public-doc/website mutation, secret handling, multiple READY items,
or treating this plan as implementation, production proof, external review, or
a public privacy claim.

The future Project Goal / Operating Principles canon request remains a future
governance candidate only. It must not override NA-0398 or the selected
NA-0399 successor.

## Inherited NA-0397 Rationale

NA-0397 selected NA-0398 after completing the code/crypto research-watch and
audit-follow-up plan. That prior lane established:

- qsl-protocol PR #1057 merged at `38b4da16362e`.
- qsl-protocol PR #1058 merged at `9b6d1c14f938`.
- READY_COUNT 1 and READY NA-0398.
- NA-0397 DONE.
- D-0776 and D-0777 present once.
- D-0778 absent before NA-0398.
- cargo audit green and `rustls-webpki v0.103.13`.
- qsl-server PR #56 remains bounded harness evidence only.
- qsl-attachments PR #37 remains service-local prerequisite evidence only.
- Public technical paper work remains future-gated.

Inherited successor rationale:

- NA-0392 identified secure messaging and metadata privacy as a claim-boundary
  area.
- NA-0393 triaged metadata privacy as important but not an immediate public
  blocker while public claims remain unchanged.
- NA-0395 preserved RFC 9420 / MLS as context only and made no MLS claim.
- NA-0397 kept code/crypto audit findings future-scoped and selected
  metadata/privacy claim boundaries as the next evidence lane.

## Authoritative Secure Messaging / Metadata Privacy Source Verification

Access date for all web sources in this section: 2026-06-01 UTC.

This is targeted source verification, not a literature review, implementation
authorization, external review, or public-claim basis.

| Source | Authority / publisher | URL | Source tier | Classification | Relevance to QSL | Claim-boundary implication |
|---|---|---|---|---|---|---|
| The X3DH Key Agreement Protocol | Signal | https://signal.org/docs/specifications/x3dh/ | Tier 1 official protocol specification | OFFICIAL_PROTOCOL_SPEC | Baseline asynchronous prekey secure messaging context for identity keys, signed prekeys, one-time prekeys, and initial shared secret derivation. | Context only; QSL does not claim Signal compatibility or X3DH implementation unless a future lane proves exact evidence. |
| The Double Ratchet Algorithm | Signal | https://signal.org/docs/specifications/doubleratchet/ | Tier 1 official protocol specification | OFFICIAL_PROTOCOL_SPEC | Baseline two-party ratchet context for chain keys, message keys, skipped keys, and post-compromise recovery terminology. | Context only; QSL may compare concepts but must not claim Signal Double Ratchet implementation or external review from source discovery. |
| The PQXDH Key Agreement Protocol | Signal | https://signal.org/docs/specifications/pqxdh/ | Tier 1 official protocol specification | OFFICIAL_PROTOCOL_SPEC | Post-quantum initial key agreement context involving X25519, ML-KEM, prekeys, identity binding, and post-quantum transition concerns. | Context only; QSL Suite-2/PQC evidence is QSL-specific and not PQXDH compliance proof. |
| Technology preview: Sealed Sender for Signal | Signal | https://signal.org/blog/sealed-sender/ | Tier 1 official technical source | OFFICIAL_PROTOCOL_SPEC | Official sealed-sender source for sender-certificate and server-visibility claim boundaries. | Useful for sender-metadata caveats; QSL does not implement sealed sender and must not claim sender-hidden behavior. |
| RFC 9420: The Messaging Layer Security (MLS) Protocol | RFC Editor / IETF | https://www.rfc-editor.org/rfc/rfc9420 | Tier 1 RFC | RFC | Secure messaging group protocol context and ciphersuite/credential vocabulary. | QSL does not claim MLS implementation or MLS compliance. |
| RFC 6973: Privacy Considerations for Internet Protocols | RFC Editor / IETF | https://www.rfc-editor.org/rfc/rfc6973 | Tier 1 RFC | RFC | Privacy-threat vocabulary for correlation, identification, secondary use, disclosure, exclusion, traffic analysis, and protocol privacy reviews. | Supports axis-by-axis privacy mapping; does not prove QSL privacy properties. |
| XRD: Scalable Messaging System with Cryptographic Privacy | USENIX Symposium on Networked Systems Design and Implementation | https://www.usenix.org/conference/nsdi20/presentation/kales | Tier 3 peer-reviewed systems research | PEER_REVIEWED_RESEARCH | NA-0392 named XRD as metadata-private messaging watch context. | Research context only; QSL does not implement XRD or inherit its privacy properties. |
| Loopix: a low-latency anonymous communication system | USENIX Security | https://www.usenix.org/conference/usenixsecurity17/technical-sessions/presentation/piotrowska | Tier 3 peer-reviewed research | PEER_REVIEWED_RESEARCH | Mixnet/cover-traffic/timing research context for traffic-analysis resistance costs and assumptions. | Cover traffic and timing resistance require explicit architecture, cost, abuse, and service evidence; not claimed by QSL. |
| Vuvuzela: Scalable Private Messaging Resistant to Traffic Analysis | USENIX / OSDI | https://www.usenix.org/conference/osdi15/technical-sessions/presentation/van-den-hooff | Tier 3 peer-reviewed systems research | PEER_REVIEWED_RESEARCH | Metadata-private messaging context for batching/noise/server-side architecture tradeoffs. | QSL does not implement Vuvuzela-style architecture; batching/noise cannot be implied. |
| Riposte: An Anonymous Messaging System | IEEE Symposium on Security and Privacy | https://ieeexplore.ieee.org/document/7163040 | Tier 3 peer-reviewed research | PEER_REVIEWED_RESEARCH | Anonymous messaging research context for write-private messaging and system assumptions. | Research context only; QSL does not claim anonymous messaging. |
| Padding Ain't Enough: Assessing the Privacy Guarantees of Encrypted DNS | USENIX Security | https://www.usenix.org/conference/usenixsecurity19/presentation/siby | Tier 3 peer-reviewed research | PEER_REVIEWED_RESEARCH | Evidence that padding alone may not remove traffic-analysis leakage. | QSL padding/bucket evidence must be caveated and must not become a metadata-free or traffic-shape-hidden claim. |

Citation gaps and uncertainty:

- Signal sources are official for Signal protocols, not QSL behavior.
- RFC 9420 is secure messaging context, not an MLS implementation claim.
- RFC 6973 provides privacy review vocabulary, not a checklist pass.
- Research systems often rely on different trust, latency, cover-traffic,
  batching, server, and abuse assumptions than QSL currently proves.
- XRD appears in NA-0392 as a named metadata-private messaging source. NA-0398
  verifies a high-quality source category for it but does not conduct a full
  XRD design review.

## Prior Metadata Privacy / Secure Messaging Evidence Intake

| Evidence source | Evidence present | QSL boundary |
|---|---|---|
| NA-0392 external watch | Secure messaging and metadata privacy source category recorded; Signal PQXDH, Double Ratchet, sealed sender, XRD, and related privacy research were identified as watch context. | Source discovery only; no QSL implementation or public privacy claim. |
| NA-0393 triage | Metadata privacy finding classified as claim-boundary / backlog candidate; not selected as immediate blocker while public claims remain unchanged. | Public claims remain constrained; future lane required before paper/website expansion. |
| NA-0395 RFC/draft boundary | RFC 9420 / MLS mapped as context; QSL MLS implementation and MLS compliance remain not claimed. | Secure messaging context does not equal MLS or draft implementation proof. |
| NA-0397 code/crypto plan | Code/crypto audit candidates carried forward; side-channel/external-review gaps remain future-scoped. | Metadata/privacy plan must not overstate code/crypto assurance. |
| qshield metadata-runtime harnesses | Existing bounded qshield demo evidence for identifier/default padding, sanitized errors/retention, timing/traffic measurement, retry cadence, jitter, batching, cover traffic prototype, padding bucket expansion, and attachment size class. | Harness/demo evidence only; not qsl-server/qsl-attachments production proof and not metadata-free proof. |
| qsl-server PR #56 | Merged at `d40e6003fdf0`; bounded end-to-end integration harness evidence. | Service-local / harness evidence only; relay visibility and production/public-internet behavior remain unproven. |
| qsl-attachments PR #37 | Merged at `96b9352bd63`; service-local production size-class harness evidence. | Service-local prerequisite evidence only; not hidden-size, production, or public-internet proof. |
| qshield demo boundary | qshield remains non-production/demo bounded evidence where cited by prior plans. | Demo evidence is not production proof and not external review. |
| Backup/local ops evidence | `/backup/qsl` mounted and scheduled as same-host continuity. | No backup-plan update required for this governance lane; same-host continuity is not complete disaster recovery. |

What may be claimed internally:

- QSL has governance and harness evidence identifying metadata/privacy axes,
  demo-only mitigations, and service-boundary prerequisites.
- Some qshield demo behaviors have executable evidence under bounded profiles.
- qsl-server/qsl-attachments have service-local harness evidence.

What cannot be claimed publicly:

- Metadata-free behavior.
- Anonymity or untraceability.
- Sender, recipient, social graph, timing, traffic-shape, attachment-size, or
  network metadata elimination.
- Production/public-internet readiness.
- External-review completion.
- Service-production proof from qshield demo or service-local harnesses.

## Read-Only Metadata / Privacy Surface Inventory

Corrected scan roots:

- `docs`
- `apps/qshield-cli`
- `qsl/qsl-client/qsc`
- `tests`
- `scripts`
- `formal`
- `tools`
- `inputs`

Scan results:

- Broad metadata/privacy term set: 22,502 matches.
- High-risk claim phrase set: 915 matches.
- Tracked files in corrected scan roots: 1,372.

Selected per-term counts:

| Term | Count |
|---|---:|
| metadata | 3499 |
| privacy | 187 |
| anonymous | 30 |
| anonymity | 469 |
| untraceable | 298 |
| traffic | 710 |
| timing | 767 |
| padding | 1069 |
| bucket | 770 |
| cover | 3205 |
| batching | 199 |
| jitter | 376 |
| relay | 4570 |
| sealed | 4 |
| sender | 211 |
| recipient | 48 |
| delivery | 290 |
| receipt | 553 |
| presence | 52 |
| attachment | 4497 |
| size | 1951 |
| storage | 210 |
| retention | 639 |
| log | 1748 |
| sanitized | 275 |
| IP | 8055 |
| network | 206 |
| queue | 2289 |
| retry | 672 |
| shape | 934 |
| rate | 1503 |

Inventory interpretation:

- Hits are planning evidence only, not defects.
- Many hits are governance/testplan prohibitions or bounded harness labels.
- qshield demo code/tests show bounded local profiles for padding, jitter,
  batching, cover traffic prototype, and attachment size classes.
- qsc runbooks and docs show relay, delivery, receipt, attachment, and
  operator-surface evidence, but do not prove production metadata hiding.
- Existing governance repeatedly prohibits metadata-free, anonymity,
  untraceability, production, and public-internet claims.

Recovered scan note:

- An initial scan used historical top-level names (`qsc`, `qsp`,
  `qshield-cli`) that are not current top-level paths and emitted `rg` path
  warnings. It was classified as a recoverable command-shape discovery issue.
  Corrective action: rerun over the corrected current roots above. Final
  result: corrected counts are recorded in this section.

## Metadata Axis Matrix

| Axis | Source basis | QSL current evidence | Evidence class | Confidence | Claim allowed? | Claim forbidden? | Missing evidence | Future lane | Priority |
|---|---|---|---|---|---|---|---|---|---|
| Content confidentiality | Signal specs, RFC 9420 context, QSL Suite-2/QSP evidence | QSL-specific protocol docs, qsc/refimpl/vectors/formal slices, qshield demo harnesses | IMPLEMENTED_BUT_EVIDENCE_INCOMPLETE | Medium | Bounded internal statement that QSL has content-encryption evidence for scoped surfaces | Any inference that content encryption removes metadata | External review, full implementation audit, production/service evidence | Code/crypto audit and external review readiness | High |
| Sender identity metadata | Sealed Sender, RFC 6973, qshield/qsl-server evidence | No sealed-sender implementation; relay/service evidence remains bounded | CLAIM_BOUNDARY_REQUIRED / NOT_CLAIMED | High | Sender identity metadata remains a reviewed axis | sender-hidden or anonymous sender claim | Relay visibility map, auth/logging review, production service evidence | qsl-server relay visibility audit | High |
| Recipient identity metadata | Signal/X3DH/PQXDH context, RFC 6973 | Recipient routing/mailbox/service visibility not proven hidden | CLAIM_BOUNDARY_REQUIRED / NOT_CLAIMED | High | Recipient metadata is not claimed hidden | recipient-hidden claim | Routing/token/logging/queue proof; production service audit | qsl-server relay visibility audit | High |
| Social/contact graph metadata | RFC 6973, secure messaging research | qsc contact/device/trust evidence exists; no graph-hiding architecture | NOT_CLAIMED / CLAIM_BOUNDARY_REQUIRED | High | Contact graph risk can be documented | social-graph-hidden or relationship privacy claim | Contact-discovery, relay-correlation, address-book/privacy architecture evidence | Metadata privacy evidence gap scanner | Medium |
| Timing metadata | RFC 6973, Loopix, Vuvuzela, prior qshield timing harnesses | qshield timing measurement, retry, jitter, batching harnesses are demo-only/bounded | HARNESS_ONLY / DEMO_ONLY | Medium | Bounded demo timing measurement/partial mitigation evidence | hidden timing claim | Production relay timing, client scheduling, queueing, retry, clock, and adversarial measurement | Timing/traffic-shape evidence lane | High |
| Traffic shape / volume | Traffic-analysis research, qshield measurement and batching evidence | Message count, queue shape, volume, and traffic bursts remain observable unless future evidence proves a bounded mitigation | HARNESS_ONLY / CLAIM_BOUNDARY_REQUIRED | Medium | Bounded demo traffic-shape observations and caveats | hides traffic shape / hides all metadata | Production traffic analysis, cover/batching cost model, service operations | Traffic-shape evidence lane | High |
| Attachment size metadata | qsl-attachments PR #37, qshield attachment-size class harness | qshield demo classes and qsl-attachments service-local size-class evidence | SERVICE_LOCAL_ONLY / HARNESS_ONLY | Medium | Service-local/demo size-class evidence with caveats | hidden attachment size claim | Production qsl-attachments padding/size-bucket, retention, quota, abuse, backup evidence | qsl-attachments size/padding/retention evidence | High |
| Network/IP metadata | RFC 6973, relay/service architecture | No QSL transport evidence showing IP/network metadata hiding | NOT_CLAIMED | High | QSL can state network/IP metadata requires separate transport assumptions | Tor/VPN/non-QSL transport protection implied by QSL | Transport privacy architecture, proxy/Tor/VPN assumptions, service logs | Network/transport metadata boundary lane | Medium |
| Delivery/read/receipt/presence metadata | qsc delivery/receipt surfaces; RFC 6973 | qsc delivery/receipt evidence exists; no proof these signals are hidden | IMPLEMENTED_BUT_EVIDENCE_INCOMPLETE / CLAIM_BOUNDARY_REQUIRED | Medium | Honest delivery/read/receipt boundary statements | presence-hidden or receipt-hidden claim | Receipt policy audit, production relay logs, correlation review | Delivery/receipt privacy evidence lane | Medium |
| Storage/logging/retention metadata | qshield sanitized-error/retention harnesses, qsl-server/qsl-attachments evidence | Sanitized errors/retention purge harnesses exist for qshield; service-local evidence exists | HARNESS_ONLY / SERVICE_LOCAL_ONLY | Medium | Bounded sanitized-error/retention evidence | no metadata stored/logged, production-retention-complete | Production logging, retention, redaction, backup/log evidence across services | Service logging/retention audit | High |
| Cover traffic / batching / padding / jitter | Loopix, Vuvuzela, Padding Ain't Enough, qshield harnesses | qshield demo retry/jitter/batching/cover/padding evidence exists under bounded labels | DEMO_ONLY / HARNESS_ONLY | Medium | Bounded demo profile evidence only | cover-traffic protection, padding hides all metadata, hidden traffic shape | Abuse/cost, production service, network adversary, quotas, operations evidence | Cover/batching/padding/jitter evidence lane | High |
| Metadata privacy vs anonymity/untraceability distinction | RFC 6973 and research category boundaries | Governance prohibits conflation; no QSL anonymity architecture | NOT_CLAIMED / CLAIM_BOUNDARY_REQUIRED | High | Metadata privacy is an evidence-mapping concern | anonymity, anonymous messaging, untraceable, complete privacy | Formal threat model, system architecture, external review, service evidence | Public claim readiness / claim scanner | High |

## Content Confidentiality Boundary

QSL evidence may support bounded internal statements that scoped QSL surfaces
have content-confidentiality design and regression evidence. Relevant evidence
includes Suite-2/QSP canonical docs, qsc/refimpl/vector coverage, formal/model
slices, and qshield demo harnesses.

That evidence does not support:

- Metadata privacy claims.
- Sender/recipient/contact graph privacy claims.
- Timing or traffic-shape hiding.
- Attachment-size hiding.
- Network/IP hiding.
- Production/public-internet readiness.
- External-review completion.

Future evidence needed:

- Code/crypto audit of content-encryption surfaces.
- Service production boundary evidence.
- External review package and disclosure readiness.
- Claim scanner proof before public material expands.

## Sender / Recipient / Social Graph Metadata Boundary

Signal Sealed Sender is useful context because it separates message content
protection from server-visible sender metadata. QSL has no sealed-sender
implementation evidence and must not imply one.

Current QSL caveats:

- qsl-server can remain a transport/relay visibility point unless future
  evidence proves otherwise.
- Recipient routing/mailbox/service metadata can remain visible to relay or
  service components unless future evidence proves otherwise.
- qsc contacts/devices/trust surfaces do not prove social/contact graph hiding.
- qshield demo evidence is not production relay evidence.

Future evidence needed:

- qsl-server relay visibility audit.
- Sender/recipient routing and token metadata map.
- Contact graph and identifier-linkability evidence.
- Production logging/retention proof.
- External review before any public privacy claim expands.

## Timing / Traffic Shape / Volume Boundary

Timing, queueing, retry cadence, message count, traffic bursts, and volume are
separate metadata axes. They are not removed by content encryption.

Current QSL evidence:

- qshield has bounded demo measurement, retry cadence, jitter, batching, and
  cover-traffic prototype evidence.
- Prior evidence states timing metadata and traffic shape remain observable
  unless exact future evidence proves a bounded mitigation.
- qsl-server and qsl-attachments production timing remain unproven.

Claim boundary:

- Allowed: bounded demo timing/traffic evidence with exact caveats.
- Forbidden: timing-hidden, hides traffic shape, metadata-free, complete
  privacy, or production/public-internet timing mitigation.

Future evidence needed:

- Production relay timing and queueing audit.
- Cross-service traffic-shape measurement.
- Abuse/cost model for batching, jitter, and cover traffic.
- Claim scanner and external review before public statements.

## Attachment Size / Padding / Bucketing Boundary

Current QSL evidence:

- qshield has demo padding bucket and attachment-size class harnesses.
- qsl-attachments PR #37 remains service-local prerequisite evidence.
- qsl-protocol governance records that attachment-size class handling is not
  proof that attachment size is hidden.

Claim boundary:

- Allowed: bounded demo/service-local size-class evidence with caveats.
- Forbidden: attachment-size-hidden, hidden-size, padding hides all metadata,
  or qsl-attachments production proof unless future exact evidence exists.

Future evidence needed:

- qsl-attachments production size/padding/retention evidence.
- qsl-server integration visibility map.
- Quota, abuse, cost, backup, and restore evidence.
- Public-claim review.

## Network / IP / Transport Metadata Boundary

QSL does not automatically hide network/IP metadata.

Current boundary:

- No Tor, VPN, proxy, mixnet, or transport-anonymity dependency may be implied
  as a QSL property without exact future evidence.
- Server/relay visibility remains a caveat.
- Client network-layer metadata remains outside current proof.

Future evidence needed:

- Explicit transport privacy architecture if QSL chooses one.
- Threat model for client IP, relay IP, proxy/Tor/VPN assumptions, and logs.
- Service operational logging and retention evidence.

## Storage / Logging / Retention Metadata Boundary

Current QSL evidence:

- qshield sanitized-error and retention/purge harnesses provide bounded demo
  evidence.
- qsl-server/qsl-attachments evidence is service-local.
- Local ops and backup logs exist, but are operational evidence, not proof
  that service metadata is absent.

Claim boundary:

- Allowed: bounded sanitized-error and retention evidence by exact lane.
- Forbidden: no metadata stored, no metadata logged, retention complete,
  external-review complete, or production logging proof.

Future evidence needed:

- qsl-server logging/retention audit.
- qsl-attachments logging/retention/cleanup audit.
- Backup/log inclusion and redaction review.
- Operational retention and incident-response policy.

## Cover Traffic / Batching / Padding / Jitter Boundary

Research sources show that cover traffic, batching, padding, and timing
defenses have architecture, cost, abuse, latency, and deployment assumptions.
QSL current evidence is not broad enough to claim these as production privacy
properties.

Current QSL evidence:

- qshield demo retry cadence normalization.
- qshield demo bounded jitter.
- qshield demo batching.
- qshield demo cover-traffic prototype.
- qshield demo padding bucket expansion.

Claim boundary:

- Allowed: exact bounded qshield demo evidence, with the demo/non-production
  label and residual leakage caveats.
- Forbidden: cover traffic protection, hidden timing, hidden traffic shape,
  padding hides all metadata, metadata-free behavior, or production readiness.

Future evidence needed:

- Production service architecture for any cover/batching/padding/jitter.
- Cost and abuse analysis.
- Queue fairness/starvation analysis.
- Measurement harnesses across client, relay, attachment, and network layers.
- External review.

## Claim Language Policy

Allowed language:

- "Metadata privacy is a design concern under active evidence mapping."
- "Specific metadata axes remain not claimed or evidence-incomplete."
- "Service-local harness evidence is bounded and not production proof."
- "qshield demo evidence is non-production and cannot be generalized to
  qsl-server/qsl-attachments."
- "Content confidentiality evidence does not eliminate metadata."

Forbidden language unless exact future evidence exists:

- metadata-free.
- anonymous.
- anonymity.
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
- padding hides all metadata.
- no metadata stored.
- no metadata logged.

When the forbidden words appear in governance evidence, they must be negated,
classified as prohibited wording, or stated as a future/unproven boundary.

## Future Queue Candidates

| Candidate | Source/evidence basis | Why next / why not next | Likely allowed scope | Likely forbidden scope | Public-claim implication |
|---|---|---|---|---|---|
| QSL Backup / Restore / Key Custody External Guidance Mapping Plan | NA-0392 backup/key watch; local backup status; off-host/operator-input blockers | Selected next because it is the next external-watch group and a major public-claim prerequisite | Governance evidence, testplan, decisions, traceability, journal; targeted backup/key source verification if authorized | Real backup/restore/key/off-host operations; scripts/timers/fstab; secret handling | Keeps same-host continuity, off-host backup, restore, and key custody claims bounded |
| QSL External Review / Disclosure / Public Claim Readiness Plan | NA-0392 disclosure/public-claim sources; metadata/code/crypto gaps | Important, but should follow backup/key guidance and stronger evidence maps | Governance readiness criteria | Public paper, website claims, runtime/service changes | Prevents external-review-complete overclaim |
| Metadata Privacy Evidence Gap / Claim Scanner Plan | NA-0398 axis matrix and high-risk phrase scan | Useful after this claim-boundary plan, but not the next external-watch group | Governance/scanner plan and allowed helper scope if authorized | Runtime, claims expansion, website | Prevents unsupported privacy language |
| qsl-server Metadata Logging / Relay Visibility Audit Plan | qsl-server PR #56 boundary; sender/recipient/network axes | Needed before relay metadata claims | qsl-server read-only/governance or future exact service scope | qsl-server implementation unless authorized | Clarifies server-visible metadata |
| qsl-attachments Size / Padding / Retention Evidence Plan | qsl-attachments PR #37; attachment size axis | Needed before attachment-size claims | Governance or future exact service scope | qsl-attachments mutation unless authorized | Prevents hidden-size overclaim |
| qshield Demo / Production Claim Boundary Plan | qshield demo harnesses | Useful before public demo expansion | Governance/demo boundary evidence | qshield runtime unless authorized | Keeps demo proof separate from production proof |
| Public Technical Position Paper Evidence Prerequisite Plan | NA-0392 through NA-0398 source/evidence maps | Still future-gated; not next until backup, service, review, and claim evidence improve | Governance prerequisite checklist | Drafting/publishing public paper | Avoids public overclaiming |
| Project Goal / Operating Principles Canon Authorization Plan | Operator carry-forward request | Valuable governance lane, but should not preempt active external-watch sequence | Governance canon authorization | Queue override without directive | Clarifies operating principles without affecting claims |
| Director State Index Authorization Plan | Local-ops carry-forward | Useful after workflow-support maturity; not next | Local governance/index authorization | Unapproved history/index writes | Reduces handoff friction |

## Selected Successor

Selected:

`NA-0399 -- QSL Backup / Restore / Key Custody External Guidance Mapping Plan`

Rationale:

- No immediate metadata/privacy public-claim blocker was found because
  unsupported claims remain prohibited and no public docs/website were changed.
- Source verification succeeded with primary official/RFC/research sources.
- The next external-watch group after metadata/privacy is backup, restore, and
  key custody guidance.
- Backup/restore/key custody remains a major evidence gap for future public
  technical claims.
- Off-host target/host-identity and real key/restore evidence remain blocked
  pending deliberate no-secret operator input and future exact authorization.

Rejected alternatives:

- `NA-0399 -- QSL Metadata Privacy / Public Claim Critical Boundary Resolution`
  was rejected because NA-0398 found no immediate unsupported public-claim
  expansion requiring a blocker successor.
- `NA-0399 -- QSL Metadata Privacy / Secure Messaging Source Verification Blocker Resolution`
  was rejected because targeted source verification completed with caveats.
- Implementing metadata/privacy runtime features now was rejected as out of
  scope.
- Updating public docs or website now was rejected as out of scope.

## Future Path / Scope Bundle

Future NA-0399 allowed paths if the selected normal successor is executed:

- `docs/governance/evidence/NA-0399_qsl_backup_restore_key_custody_external_guidance_mapping_plan.md`
- `tests/NA-0399_qsl_backup_restore_key_custody_external_guidance_mapping_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future forbidden scope:

- dependency changes.
- `Cargo.toml` / `Cargo.lock` changes.
- runtime code.
- crypto implementation.
- qsc/qsp/qsl implementation.
- qshield runtime.
- qsl-server.
- qsl-attachments.
- workflows.
- public docs/website.
- backup scripts/timers/fstab.
- response archives.
- real backup/restore/key/off-host operations.
- external claims.

Future NA-0399 may use targeted web verification only for backup, restore, and
key-custody sources if live NA-0399 scope authorizes it.

## Public Claim / External Review / Website Boundary

This plan is not implementation.

This source discovery is not external review.

This plan does not update website or public docs.

This plan does not support:

- metadata-free behavior.
- anonymity.
- untraceability.
- hidden timing.
- hidden traffic shape.
- hidden attachment size.
- production readiness.
- public-internet readiness.
- bug-free behavior.
- perfect crypto.
- external-review completion.

## Future Validation / Marker Plan

Future NA-0399 markers if the normal successor is executed:

- `NA0399_BACKUP_RESTORE_KEY_GUIDANCE_MAPPING_PLAN_OK`
- `NA0399_RESTIC_SOURCE_REFERENCE_OK`
- `NA0399_BORG_SOURCE_REFERENCE_OK`
- `NA0399_RCLONE_SOURCE_REFERENCE_OK`
- `NA0399_AGE_SOURCE_REFERENCE_OK`
- `NA0399_GNUPG_OPENSSH_SOURCE_REFERENCE_OK`
- `NA0399_SAME_HOST_CONTINUITY_CAVEAT_OK`
- `NA0399_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK`
- `NA0399_NO_OFF_HOST_BACKUP_COMPLETE_CLAIM_OK`
- `NA0399_NO_RESTORE_PROVEN_CLAIM_OK`
- `NA0399_NO_KEY_CUSTODY_IMPLEMENTED_CLAIM_OK`
- `NA0399_NO_RUNTIME_CHANGE_OK`
- `NA0399_NO_CRYPTO_IMPLEMENTATION_CHANGE_OK`
- `NA0399_NO_DEPENDENCY_CHANGE_OK`
- `NA0399_NO_WORKFLOW_CHANGE_OK`
- `NA0399_NO_SECRET_MATERIAL_OK`

## Future Project Goal / Operating Principles Canon Carry-Forward Note

The operator-requested future Project Goal / Operating Principles canon lane is
carried forward as a future governance candidate only:

`QSL Project Goal and Operating Principles Canon Authorization Plan`

It should record QSL's north star, security-before-speed discipline, evidence
over vibes, code and crypto excellence, no public overclaiming, one-READY
queue discipline, routine audits, external awareness without hype, public
technical paper timing, safer future directives, and Director/Codex/human role
boundaries. It is not selected over NA-0399.

## Rejected Alternatives

- Implementing sealed sender, mixnets, cover traffic, batching, padding,
  jitter, or anonymity features now.
- Changing qshield runtime now.
- Changing qsl-server/qsl-attachments now.
- Changing dependencies or Cargo files now.
- Changing workflows now.
- Updating public docs or website now.
- Starting public technical paper work now.
- Treating Signal, RFC, or research source discovery as QSL external review.
- Treating qshield demo evidence as production proof.
- Treating qsl-server/qsl-attachments service-local evidence as
  public-internet proof.

## Backup-Plan Impact Statement

No backup-plan update is required for NA-0398 because durable changes are
limited to tracked qsl-protocol governance, testplan, decision, traceability,
and rolling-journal files under `/srv/qbuild/work`.

No real backup, restore, key custody, key recovery, off-host target, host
identity, fstab, timer, service, backup script, credential, secret, remote
connection, or restore path is changed by this lane.

Future durable metadata/privacy reports, recurring source-watch report stores,
local history/index outputs, backup/key/restore evidence stores outside the
repository, or actual backup/restore/key/off-host operations require separate
backup-impact review.

Same-host continuity remains same-host continuity only. It must not be called
complete disaster recovery.

## Next Recommendation

Merge NA-0398 after validation and required CI pass. If post-merge
public-safety is green, close out NA-0398 and restore the exact selected
successor:

`NA-0399 -- QSL Backup / Restore / Key Custody External Guidance Mapping Plan`

Do not implement NA-0399 during NA-0398.

## Source List

| Title | Authority / publisher | URL | Access date | Source tier | Classification | Relevance |
|---|---|---|---|---|---|---|
| The X3DH Key Agreement Protocol | Signal | https://signal.org/docs/specifications/x3dh/ | 2026-06-01 UTC | Tier 1 | OFFICIAL_PROTOCOL_SPEC | Secure messaging initial key agreement context. |
| The Double Ratchet Algorithm | Signal | https://signal.org/docs/specifications/doubleratchet/ | 2026-06-01 UTC | Tier 1 | OFFICIAL_PROTOCOL_SPEC | Secure messaging ratchet context. |
| The PQXDH Key Agreement Protocol | Signal | https://signal.org/docs/specifications/pqxdh/ | 2026-06-01 UTC | Tier 1 | OFFICIAL_PROTOCOL_SPEC | PQ secure messaging key agreement context. |
| Technology preview: Sealed Sender for Signal | Signal | https://signal.org/blog/sealed-sender/ | 2026-06-01 UTC | Tier 1 | OFFICIAL_PROTOCOL_SPEC | Sender metadata/server visibility context. |
| RFC 9420: The Messaging Layer Security (MLS) Protocol | RFC Editor / IETF | https://www.rfc-editor.org/rfc/rfc9420 | 2026-06-01 UTC | Tier 1 | RFC | Secure group messaging context only. |
| RFC 6973: Privacy Considerations for Internet Protocols | RFC Editor / IETF | https://www.rfc-editor.org/rfc/rfc6973 | 2026-06-01 UTC | Tier 1 | RFC | Privacy axis/threat vocabulary. |
| XRD: Scalable Messaging System with Cryptographic Privacy | USENIX NSDI | https://www.usenix.org/conference/nsdi20/presentation/kales | 2026-06-01 UTC | Tier 3 | PEER_REVIEWED_RESEARCH | Metadata-private messaging context. |
| Loopix: a low-latency anonymous communication system | USENIX Security | https://www.usenix.org/conference/usenixsecurity17/technical-sessions/presentation/piotrowska | 2026-06-01 UTC | Tier 3 | PEER_REVIEWED_RESEARCH | Mixnet/cover-traffic/timing context. |
| Vuvuzela: Scalable Private Messaging Resistant to Traffic Analysis | USENIX OSDI | https://www.usenix.org/conference/osdi15/technical-sessions/presentation/van-den-hooff | 2026-06-01 UTC | Tier 3 | PEER_REVIEWED_RESEARCH | Metadata-private messaging and batching/noise context. |
| Riposte: An Anonymous Messaging System | IEEE Symposium on Security and Privacy | https://ieeexplore.ieee.org/document/7163040 | 2026-06-01 UTC | Tier 3 | PEER_REVIEWED_RESEARCH | Anonymous messaging research context, not QSL claim basis. |
| Padding Ain't Enough: Assessing the Privacy Guarantees of Encrypted DNS | USENIX Security | https://www.usenix.org/conference/usenixsecurity19/presentation/siby | 2026-06-01 UTC | Tier 3 | PEER_REVIEWED_RESEARCH | Padding/traffic-analysis caution. |
