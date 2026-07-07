Goals: G5 (primary), supports G1, G2, G3, G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-07
Supersedes: (none)
Extends: DOC-G5-001, DOC-G5-002, DOC-G5-004, docs/audit/METADATA_MITIGATIONS_ROADMAP_NA-0137.md

# DOC-G5-005 — Hostile-Analyst Metadata Minimization and Implementation-Attack Hardening Plan v0.1.0 DRAFT

## 0. Purpose, scope, and non-goals

Purpose: turn the NA-0608 LAN hostile-analyst findings into a bounded, prioritized
hardening plan that scopes concrete future implementation lanes and sets external/
formal-review readiness. This document is a PLAN only; it changes no protocol, wire,
crypto, or state-machine behavior. Every concrete change it names returns to
governance as its own lane.

Scope: the qsc client, the qsl-server transport-only relay, and the qsl-attachments
opaque-object service, under the hostile-analyst model of NA-0608 (an attacker who
reads the protocol/code, operates or observes relay/storage metadata, and sends
malformed/adversarial traffic).

Non-goals (aligned with DOC-G5-001): anonymity networks, mixnets, PIR, global cover
traffic, and any "metadata eliminated" / "traceability eliminated" claim. This plan
minimizes and documents residual metadata; it does not claim to remove it.

Relationship to the existing G5 corpus: DOC-G5-001 (threat model), DOC-G5-002
(leakage inventory), and the NA-0137 mitigations roadmap primarily cover the
**message plane**, where qsc already implements optional padding/bucketing
(`pad_bucket` / `bucket_max` / `meta_pad_config_from_args` in
`qsl/qsl-client/qsc/src/transport/mod.rs`). This plan extends that work to the
**attachment plane** and to the **implementation-attack surface**, which NA-0608
exercised for the first time and which the earlier corpus did not cover.

## 1. Inputs

- NA-0608 evidence (`docs/governance/evidence/NA-0608_lan_qsl_attachments_hostile_analyst_metadata_fail_closed_harness.md`):
  fail-closed negatives all rejected with zero plaintext; opaque ciphertext-only
  storage confirmed; seed fallback not used; metadata matrix classified
  `LAN_ATTACHMENT_METADATA_CLASSIFIED_ACCEPTABLE_FOR_NEXT_STEP` with size/count/
  timing as EXPOSED residual metadata.
- NA-0609B audit (D-1213) and NA-0609C/NA-0609D outcomes: handshake seam verified
  sound; ENG-0003 (non-constant-time MAC comparison) fixed; ENG-0004 confirmed a
  false positive (durability sound on Unix).
- Improvement ledger (DOC-OPS-007): ENG-0001 (self-label footgun) and ENG-0002
  (attachment single-send-per-session) open.

## 2. Traffic-analysis metadata (message plane vs attachment plane)

Class-only. Distinguish what is already mitigable from what is residual.

| dimension | message plane | attachment plane (NA-0608) |
| --- | --- | --- |
| plaintext / keys / capabilities | PROTECTED | PROTECTED (canary test: 0 leaks) |
| content size | mitigable via padding/bucketing (implemented) | EXPOSED as ciphertext-object size (residual) |
| object / part count | n/a | EXPOSED (residual) |
| timing / cadence | mitigable via fixed-interval polling (NA-0137) | EXPOSED (upload/fetch timing; residual) |
| route / mailbox | pseudonymous route token (hashed in markers) | pseudonymous route token |
| sender/receiver identity | PROTECTED (relay sees tokens, not identities) | PROTECTED |

Conclusion: the residual attacker-observable metadata on the attachment plane is
**ciphertext-object size, object/part count, and upload/fetch timing**. These are
the focus of the mitigation items below.

## 3. Implementation-attack surface

- Constant-time comparisons: the handshake keyed-MAC comparisons were hardened to
  constant time in NA-0609C (ENG-0003 done). A follow-on sweep should enumerate any
  remaining tag/MAC/secret comparisons outside the handshake seam (e.g. attachment
  capability/token checks, vault unlock) and confirm or convert them.
- Error/retry normalization: reject codes and retry/backoff behavior can be a
  distinguishing side channel. Scope a review of whether distinct reject classes,
  timings, or retry patterns are externally distinguishable in a way that reveals
  which check failed.
- Non-content leakage: markers/log lines must remain class-only (DOC-G5-004 logging
  contract); a periodic re-scan is a low-cost control.

## 4. Relay and qsl-attachments compromise models

- Relay (qsl-server, transport-only): a hostile relay operator observes route-token
  presence, message/object sizes and counts, and timing. It does not see plaintext,
  keys, capabilities, or identities. Residual: traffic-analysis metadata (section 2).
- qsl-attachments (opaque object store): a hostile storage operator observes
  ciphertext-object sizes, part counts, and access timing, and can withhold/delete/
  corrupt objects. NA-0608 showed all such tampering is caught fail-closed
  (REJECT_QATTSVC_*, ciphertext precheck, locator-unknown). Residual: the same
  size/count/timing metadata.

## 5. Malformed envelope/descriptor/object test-expansion scope

NA-0608 exercised the LAN-safe negatives but recorded several as "if supported /
not separately exercised" — notably the corrupted-descriptor case, which is covered
by envelope authentication rather than a dedicated negative. Scope explicit,
deterministic negative tests for malformed envelope, attachment descriptor, and
attachment object inputs, asserting fail-closed reject and no-mutation-on-reject.
This is the **highest-priority bounded item** (test-only, low risk, closes a real
coverage gap) and is the proposed NA-0610 successor.

## 6. Padding/bucketing feasibility for the attachment plane

The message plane already supports size padding/bucketing. For the attachment plane,
assess: (a) padding ciphertext objects to size classes (cost: storage/bandwidth
overhead; benefit: collapses the size channel); (b) fixed part counts / part-size
bucketing (collapses the count channel); (c) timing normalization for upload/fetch.
This requires a dedicated feasibility+design lane (cost/benefit matrix per DOC-G5-004
style) before any behavior change, because it touches the attachment wire/service
contract.

## 7. Error/retry normalization

Scope a review + design lane: normalize externally observable reject/retry behavior
so that distinct internal failure causes are not distinguishable by timing or shape
beyond what the deterministic reject taxonomy already exposes. Keep fail-closed
semantics; this is a distinguishability-reduction, not a semantic change.

## 8. External/formal-review readiness

Before external or formal review can be claimed, the following must hold: the
handshake/identity seam audit is complete (NA-0609B, done) with remediations closed
or ranked; the metadata residuals are documented with explicit mitigation options
and costs (this plan); the malformed-input negative coverage is expanded (NA-0610);
and the claim boundaries in DECISIONS remain honest (no security-complete/
metadata-free claims). This plan does not itself constitute or schedule external
review; it defines the preconditions.

## 9. Prioritized hardening backlog (each returns as its own lane)

> **SUPERSEDED (2026-07-07, D-1231).** This ranked table is retained for history only; its
> ranks 1–6 are all complete (NA-0610…NA-0617). It is **no longer the live roadmap.** The
> single authoritative prioritized backlog is now `docs/ops/IMPROVEMENT_LEDGER.md` (ENG-####
> / WF-#### entries, sorted by severity), surfaced via the LIVE QUEUE / ON DECK header in
> `NEXT_ACTIONS.md`. Do not add new items here; file them in the ledger.

| rank | item | plane / area | severity | recommended lane shape |
| --- | --- | --- | --- | --- |
| 1 | Malformed envelope/descriptor/object negative-test expansion | test coverage | P2 | source/test (NA-0610) |
| 2 | Constant-time comparison sweep beyond the handshake seam | impl-attack | P3 | read-only audit, then fix |
| 3 | Error/retry normalization review + design | impl-attack | P3 | audit/docs, then fix |
| 4 | Attachment-plane size/count/timing mitigation feasibility + design | metadata | P3 | audit/docs, then implementation |
| 5 | ENG-0001 self-label footgun (fail loud on unknown self-label) | identity UX | P3 | small source fix |
| 6 | ENG-0002 attachment single-send-per-session: document/clarify | attachment | P3 | docs, or small fix |

Ranking rationale: item 1 is bounded, test-only, and closes a concrete NA-0608
coverage gap; items 2-3 are low-risk implementation-attack reviews; item 4 is the
highest-value metadata mitigation but touches the attachment contract and needs
feasibility first; items 5-6 are small, already-scoped ledger items.

## 10. Claim boundary

This is a plan, not an implementation and not an external or formal review. It makes
no public-readiness, production-readiness, security-completion, crypto-complete,
attachment-complete, metadata-free, anonymity, untraceability, side-channel-free, or
bug-free claim. Residual metadata is documented and ranked, not eliminated. No
endpoint, port, token, capability, key, seed, plaintext, ciphertext body, or raw
private material appears in this document.
