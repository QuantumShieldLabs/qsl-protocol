# NA-0134 Metadata Leakage Audit + Mitigation Matrix

## Scope and Method
This is a docs-only metadata/privacy audit artifact for NA-0134. No code changes are included.

Method:
- Enumerate observable metadata classes.
- Distinguish unavoidable vs mitigable vs unknown leakage.
- Tie claims to implementation/test evidence in this repo.
- Propose follow-on NAs only (no fixes in this report).

## A) System Model for Metadata

### Actors and channels
- Client endpoint(s): `qsc` process and local OS/terminal environment.
- Relay/service endpoint: relay is explicitly untrusted for confidentiality/integrity.
- Network observer classes:
  - passive network observer,
  - active network attacker,
  - malicious relay.

Primary channel:
- client <-> relay (`relay_inbox_push`, `relay_inbox_pull`) in `qsl/qsl-client/qsc/src/main.rs:9966`, `qsl/qsl-client/qsc/src/main.rs:9990`.

### What metadata includes in this audit
- Timing and cadence:
  - send timing, receive timing, poll cadence, batch cadence.
- Size and volume:
  - payload length, bucket size class, chunk size/count, message frequency.
- Identifiers and correlation surfaces:
  - channel labels, file ids, receipt request/ack patterns, repeated polling patterns.
- Error/abort side channels:
  - reject timings and failure class observability.

### Assumptions (explicit)
- Endpoint IP and relay endpoint are observable at network level and are not hidden by this protocol alone.
- Full timing obfuscation without significant UX/bandwidth cost is out-of-scope for current posture.
- Local OS/terminal side channels are partially outside protocol control.
- Deterministic markers are valuable for auditability but can expose structured behavior classes if exposed to an adversary with log access.

## B) Leakage Matrix (Core Deliverable)

| Leakage category | Leakage source | What observer learns | Status | Evidence | Mitigation pointer |
|---|---|---|---|---|---|
| Connection metadata (IP, endpoint) | TCP/HTTP connection to relay | Who talks to which relay and when | Unavoidable | Relay transport and HTTP functions (`qsl/qsl-client/qsc/src/main.rs:9946`, `qsl/qsl-client/qsc/src/main.rs:9966`, `qsl/qsl-client/qsc/src/main.rs:9990`) | M-7, M-8 |
| Traffic timing (message send/recv) | Explicit send and polling operations | Message timing and burst patterns | Mitigable | Poll config + tick/batch markers (`qsl/qsl-client/qsc/src/main.rs:6526`, `qsl/qsl-client/qsc/src/main.rs:9461`); deterministic poll tests (`qsl/qsl-client/qsc/tests/meta_phase2.rs:159`, `qsl/qsl-client/qsc/tests/meta_min.rs:103`) | M-1, M-2, M-6 |
| Traffic volume (bytes per interval) | Payload sizes and per-batch volume | Relative content size and activity level | Mitigable | Metadata padding labels (`QSC.META.PAD`) in pack path (`qsl/qsl-client/qsc/src/main.rs:7164`, `qsl/qsl-client/qsc/src/main.rs:7177`); padding tests (`qsl/qsl-client/qsc/tests/meta_min.rs:156`) | M-3, M-4 |
| Polling cadence / batching | Configured interval/ticks and batch max | Idle/active cycle and pull frequency | Mitigable | Poll controls and bounds (`qsl/qsl-client/qsc/src/main.rs:150`, `qsl/qsl-client/qsc/src/main.rs:6537`); conformance tests (`qsl/qsl-client/qsc/tests/meta_min.rs:60`, `qsl/qsl-client/qsc/tests/meta_phase2.rs:86`) | M-1, M-2 |
| Delivered receipt behavior | Explicit receipt request/emit flow | Message-read/delivery timing signal and roundtrip coupling | Mitigable | Receipt request/send/recv markers (`qsl/qsl-client/qsc/src/main.rs:10126`, `qsl/qsl-client/qsc/src/main.rs:9705`, `qsl/qsl-client/qsc/src/main.rs:9626`); tests (`qsl/qsl-client/qsc/tests/receipts_delivered.rs:132`, `qsl/qsl-client/qsc/tests/receipts_delivered.rs:168`) | M-5 |
| File transfer chunking | Deterministic chunk loop and manifest completion | File size class, chunk cadence, transfer completion timing | Mitigable | Chunk sizing/counting and markers (`qsl/qsl-client/qsc/src/main.rs:6769`, `qsl/qsl-client/qsc/src/main.rs:6790`, `qsl/qsl-client/qsc/src/main.rs:6837`, `qsl/qsl-client/qsc/src/main.rs:6873`); tests (`qsl/qsl-client/qsc/tests/file_transfer_mvp.rs:164`) | M-4, M-6 |
| Error/abort pattern leakage | Distinct reject paths and timing | Which failure class happened; possible policy inference | Mitigable | Reject markers and codes (`qsl/qsl-client/qsc/src/main.rs:9714`, `qsl/qsl-client/qsc/src/main.rs:9983`, `qsl/qsl-client/qsc/src/main.rs:10018`); reject/no-mutation tests (`qsl/qsl-client/qsc/tests/message_state_model.rs:199`, `qsl/qsl-client/qsc/tests/file_transfer_mvp.rs:251`) | M-9 |
| Contact discovery leakage | Contact and peer labels used in commands/channels | Whether a user is configured to communicate with specific labels | Unknown | Channel and peer usage exists (`qsl/qsl-client/qsc/src/main.rs:8451`, `qsl/qsl-client/qsc/src/main.rs:2141`); tests cover identity binding but not network-level discovery resistance (`qsl/qsl-client/qsc/tests/identity_binding.rs:40`) | M-8, M-10 |
| Relay-backed UI timing correlation | Inbound message behavior under focused/unfocused states | Correlation between network events and UI counters/append timing | Mitigable | NA-0127 workflow + ignored tests (`.github/workflows/relay-ui-integration.yml:1`, `qsl/qsl-client/qsc/tests/relay_ui_integration.rs:1`) | M-1, M-2 |

Notes:
- Status categories are from this audit’s evidence scope only.
- "Unknown" means insufficient direct evidence in current repo artifacts.

## C) Mitigation Options with Cost Matrix

| ID | Mitigation technique | Security benefit | Bandwidth cost | Latency cost | Client CPU/battery | Complexity/risk | Recommended stance |
|---|---|---|---|---|---|---|---|
| M-1 | Optional fixed-interval polling mode | Reduces cadence variability leaks | Low-Medium | Medium (waiting window) | Low | Medium | Optional |
| M-2 | Batching windows with jitter bounds | Smears exact event timing correlation | Low | Medium | Low | Medium | Optional |
| M-3 | Message padding to coarse bucket classes | Reduces fine-grained size leakage | Medium | Low | Low | Medium | Optional |
| M-4 | File chunk size normalization policy | Reduces chunk-count/size fingerprinting | Medium-High | Low-Medium | Low | Medium | Optional |
| M-5 | Receipt delay/randomization within bounded SLA | Reduces immediate delivery timing linkage | Low | Medium | Low | Medium | Optional |
| M-6 | Cover pull ticks during low activity | Reduces idle-vs-active cadence disclosure | Medium | Low | Medium | High | Optional (advanced) |
| M-7 | Multi-relay strategy / relay rotation | Reduces single-endpoint correlation over long windows | Medium | Low | Low | High | Optional (advanced) |
| M-8 | Network-layer anonymity transport (VPN/Tor/mix) guidance | Reduces endpoint-level metadata visibility | Externalized | Medium-High | Medium | High | Default guidance (docs), not protocol default |
| M-9 | Error-shape normalization | Reduces failure-class inference | None | Low | Low | Medium | Default (where safe) |
| M-10 | Contact-discovery privacy mode design | Reduces peer-label discovery exposure | Low-Medium | Medium | Low | High | Optional (future NA) |

## D) Current Posture vs Best-in-Class Metadata Resistance

### What we have today
- Deterministic, bounded behavior with explicit controls and markers for polling, receipts, and file transfer progression.
- Optional padding/bucket mechanics are present in metadata and envelope paths (`qsl/qsl-client/qsc/src/main.rs:7147`, `qsl/qsl-client/qsc/src/main.rs:7164`).
- Relay-backed UI integration lane exists for real-IO behavior validation (`.github/workflows/relay-ui-integration.yml:1`).

### What best-in-class metadata resistance would require
- Constant-rate or cover-traffic-driven send/receive schedules.
- Stronger size normalization across more traffic classes.
- Potential anonymity-network integration (or equivalent relay architecture changes).
- Much higher operational cost (bandwidth, latency, complexity, and reliability tradeoffs).

Conclusion:
- Current posture is auditable and bounded, but not "metadata-hard" in the mixnet/constant-rate sense.

## E) Findings + Follow-on NA Recommendations

### P0-1: Endpoint and timing correlation remain broadly observable
- Severity: P0 (privacy impact)
- Evidence:
  - Direct relay network operations (`qsl/qsl-client/qsc/src/main.rs:9966`, `qsl/qsl-client/qsc/src/main.rs:9990`).
  - Poll/timing controls expose cadence classes (`qsl/qsl-client/qsc/src/main.rs:6526`, `qsl/qsl-client/qsc/src/main.rs:9461`).
- Follow-on NA:
  - Title: "Optional fixed-interval polling + bounded jitter mode"
  - Acceptance:
    - deterministic config with explicit bounds,
    - tests proving cadence normalization,
    - no mutation on invalid config.

### P1-1: Receipt timing creates correlation side-channel
- Severity: P1
- Evidence:
  - Explicit delivered receipt request/send/recv flow (`qsl/qsl-client/qsc/src/main.rs:10126`, `qsl/qsl-client/qsc/src/main.rs:9705`, `qsl/qsl-client/qsc/src/main.rs:9626`).
  - Receipt behavior tests (`qsl/qsl-client/qsc/tests/receipts_delivered.rs:168`).
- Follow-on NA:
  - Title: "Receipt timing policy (delay/randomization within bounds)"
  - Acceptance:
    - configurable bounded delay policy,
    - deterministic markers for policy mode,
    - tests proving no over-claim and bounded behavior.

### P1-2: File transfer chunk metadata remains fingerprintable
- Severity: P1
- Evidence:
  - Chunk count and chunk size flows (`qsl/qsl-client/qsc/src/main.rs:6769`, `qsl/qsl-client/qsc/src/main.rs:6790`).
  - File-transfer marker and state tests (`qsl/qsl-client/qsc/tests/file_transfer_mvp.rs:164`).
- Follow-on NA:
  - Title: "File chunk size normalization and transfer pacing options"
  - Acceptance:
    - policy-controlled normalization profile,
    - deterministic reject rules for unsupported profiles,
    - integration tests for truthful state preservation.

### P2-1: Contact-discovery metadata posture is under-specified
- Severity: P2
- Evidence:
  - Peer labels/channels are present in handshake/channel logic (`qsl/qsl-client/qsc/src/main.rs:8451`).
  - No dedicated metadata-minimization spec/test lane for discovery resistance found in current artifacts.
- Follow-on NA:
  - Title: "Contact/discovery metadata minimization model"
  - Acceptance:
    - explicit threat model updates,
    - leakage classification for discovery events,
    - test plan for observable surfaces.

### P2-2: Error-shape normalization is incomplete
- Severity: P2
- Evidence:
  - Distinct error classes for relay/push/pull and receipts are visible (`qsl/qsl-client/qsc/src/main.rs:9983`, `qsl/qsl-client/qsc/src/main.rs:10018`, `qsl/qsl-client/qsc/src/main.rs:9714`).
- Follow-on NA:
  - Title: "Metadata-safe error taxonomy normalization"
  - Acceptance:
    - normalized externally observable error classes,
    - internal diagnostics preserved without widening external leakage.

## Unknowns and Limits
- This report does not claim hidden endpoint metadata against a global passive adversary.
- Some leakage classes are marked Unknown due to missing direct evidence artifacts in current repository state.
- No code fixes are included by design for NA-0134.

## Reproduction Commands
- `rg -n "META\.PAD|receipt_|file_xfer_|meta_poll|relay_inbox_" qsl/qsl-client/qsc/src/main.rs`
- `rg -n "meta_phase2|meta_min|receipts_delivered|file_transfer_mvp|relay_ui_integration" qsl/qsl-client/qsc/tests`
- `rg -n "relay-ui-integration|workflow_dispatch|schedule" .github/workflows/relay-ui-integration.yml`
