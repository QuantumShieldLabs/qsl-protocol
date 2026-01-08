# QuantumShield Security Objectives & Threat Model
**Document:** Security Objectives & Threat Model (SOTM)

**Status:** Normative (governance)  
**Version:** 1.0.0 (DRAFT)  
**Date:** 2025-12-22  

**Applies to:**
- **QSP v4.3.2** (Wire `protocol_version` **0x0403**)  
- **Primary suite:** **Suite-1B** (`suite_id` **0x0002**)  
- **Legacy compatibility:** Suite-1 (`suite_id` **0x0001**) is permitted only where explicitly required, and MUST NOT be the default.

**Related canonical documents:**
- QuantumShield Protocol Specification (QSP) v4.3.2
- QuantumShield Envelope Specification (QSE) v1.8.2

---

## 0. Purpose and authority
This document defines **what security and privacy properties QuantumShield MUST achieve**, the **adversary model** those properties are measured against, and the **evidence standard** required to claim conformance.

It exists to prevent a common failure mode in security projects: a well-written crypto core that cannot support the intended privacy claims, and/or a feature-rich protocol that cannot be validated.

### 0.1 Precedence
In case of conflict:
1) **This document** governs *what is being claimed and what must be proven by tests*.
2) **QSP** governs cryptographic mechanisms and message semantics.
3) **QSE** governs transport wrapping and deployment-controlled metadata minimization.
4) Guidelines/whitepapers are informational.

### 0.2 Interpretation
The keywords **MUST**, **MUST NOT**, **SHOULD**, **SHOULD NOT**, **MAY** are to be interpreted as in RFC 2119.

---

## 1. Scope
### 1.1 In-scope
- **Pairwise (1:1) end-to-end encrypted messaging** between devices.
- A protocol that remains secure under **asynchronous**, **lossy**, and **out-of-order** delivery.
- **Hybrid post-quantum** design goals as defined in §3.2.
- **Key Transparency (KT)** verification as a required authentication primitive (per QSP v4.3.x).
- **Metadata minimization at the protocol and envelope layer**, with explicit limits (§4).

### 1.2 Out-of-scope (explicit non-goals)
These items are intentionally not claimed by QSP/QSE v4.3.2/1.8.2:
- Group messaging semantics and group key management.
- Resistance to a **global passive adversary** performing full traffic analysis across the entire Internet.
- Endpoint OS compromise, keyloggers, or malicious client runtimes beyond the guarantees implied by FS/PCS.
- Perfect deniability properties (unless explicitly specified and tested; not currently a goal).

---

## 2. Definitions
### 2.1 Security properties
- **Confidentiality:** An adversary cannot learn message plaintext from observing protocol traffic.
- **Integrity:** An adversary cannot modify traffic to cause undetected changes in plaintext or protocol state.
- **Authentication:** Parties can cryptographically bind a session to the intended identity keys, as defined by KT policy.
- **Forward Secrecy (FS):** Compromise of long-term identity keys does not reveal past session message keys.
- **Post-Compromise Security (PCS):** After a compromise ends, the protocol can recover confidentiality for future messages, assuming the attacker no longer controls the endpoint.

### 2.2 Adversary classes
- **Passive network adversary (PNA):** Observes all protocol bytes but cannot modify or inject.
- **Active network adversary (ANA):** Can inject, modify, drop, delay, reorder, and replay.
- **Malicious or compromised service (MCS):** Controls routing, storage, and envelope metadata; may withhold, reorder, or selectively deliver.
- **Endpoint compromise (EC):** Attacker obtains device secrets for a period of time.

### 2.3 Metadata
- **Content metadata:** protocol-visible fields that do not contain message plaintext (e.g., counters, flags).
- **Transport metadata:** outer routing fields and timing/size patterns (e.g., route tokens, timestamp buckets, message sizes, send times).

---

## 3. Security objectives (MUST)
### 3.1 Core cryptographic objectives
QuantumShield MUST provide the following properties for Suite-1B (`suite_id=0x0002`) when both parties follow the protocol:

1) **End-to-end confidentiality and integrity** for all application plaintexts carried in QSP messages.
2) **Mutual authentication** anchored in identity keys whose bindings are validated via **Key Transparency** per QSP.
3) **Downgrade resistance:** an adversary MUST NOT be able to force parties to negotiate a weaker `suite_id` or different protocol version without detection.
4) **Replay resistance at the protocol layer:** replays MUST NOT result in acceptance of a duplicate message as new application data.
5) **Robustness under reordering:** out-of-order delivery MUST be handled with bounded memory/time, and MUST NOT create state desynchronization that breaks security.
6) **Fail-closed parsing and state transitions:** malformed inputs MUST be rejected without heuristic recovery.

### 3.2 Post-quantum objective (hybrid security goal)
QSP v4.3.2 uses **hybrid** constructions for key establishment and authentication.

**Required security claim (hybrid goal):** Session keys MUST remain computationally indistinguishable from random to a network adversary if **at least one** of the following remains secure:
- the classical component(s) (e.g., X25519), **or**
- the post-quantum component(s) (e.g., ML-KEM / ML-DSA).

**Non-claim (important):** QuantumShield does not claim security if **all** hybrid components are broken.

### 3.3 Forward secrecy and post-compromise security
The protocol MUST achieve:
- **FS:** compromise of long-term identity keys MUST NOT enable decryption of prior message plaintexts.
- **PCS:** after endpoint compromise ends, subsequent protocol evolution MUST restore confidentiality for future messages.

**PCS boundary condition:** PCS is measured under the assumption that the attacker no longer controls the endpoint and cannot continuously read fresh secrets.

### 3.4 Key Transparency requirements (normative)
QuantumShield’s authentication model depends on Key Transparency.

Implementations claiming conformance to QSP v4.3.2 MUST:
- Perform KT verification for identity bindings required by QSP.
- Enforce rollback/freshness policies at least as strict as QSP’s minimum requirements.
- Treat KT validation failures as **fatal** for authenticated sessions.

### 3.5 Side-channel and implementation security requirements
Conforming implementations MUST:
- Use a cryptographically secure RNG for all key generation and nonces.
- Compare tags/MACs/signatures in constant time.
- Enforce strict length checks and canonical parsing rules.
- Avoid secret-dependent branching and memory access patterns in cryptographic primitives to the extent feasible (especially in signature/KEM operations and key schedule logic).

---

## 4. Privacy and metadata minimization objectives
QuantumShield’s privacy goal is **minimization and controlled leakage**, not absolute invisibility.

### 4.1 Protocol-layer privacy (QSP)
QSP MUST:
- Protect message **plaintext** against PNA/ANA.
- Provide **header confidentiality** for message counters (e.g., N/PN) as defined by QSP.
- Ensure that control-plane fields that could influence security (e.g., PQ-related fields in Suite-1B) are **cryptographically bound** to the session context (Suite-1B’s binding objective).

QSP is permitted to leak (and therefore does not claim to hide):
- `protocol_version`, `suite_id`.
- `session_id` (treated as protocol metadata).
- Message timing at the transport layer.

### 4.2 Envelope-layer privacy (QSE)
QSE MUST:
- Carry exactly **one complete QSP message per envelope**.
- Support deployment-controlled **padding** to reduce size-based traffic analysis.
- Enforce DoS bounds and canonical parsing (reject unknown flags, unknown versions, length overruns).

QSE is permitted to leak (deployment-dependent):
- Coarse timestamp bucket (if used).
- Route token length (unless padding policy covers it).

### 4.3 Deployment-layer privacy (explicitly a separate layer)
The following are **deployment responsibilities**, not guaranteed solely by QSP/QSE:
- Sender/receiver IP address concealment.
- Global timing correlation resistance.
- Mixnet/relay/onion routing integration.

QuantumShield MAY define optional deployment profiles to address these, but must not claim them as baseline properties unless formally specified and tested.

---

## 5. Threat model (what we defend against)
### 5.1 Network adversaries
**PNA (passive):** The protocol MUST maintain confidentiality and integrity of message content.

**ANA (active):** The protocol MUST:
- Detect forgeries and modifications.
- Resist downgrade attempts.
- Reject malformed/corrupt structures deterministically.
- Bound work (avoid attacker-controlled unbounded parsing or key-search).

### 5.2 Malicious or compromised service
QuantumShield assumes services may:
- Observe envelopes, route tokens, coarse timestamps, and traffic patterns.
- Drop, delay, reorder, replay, or selectively deliver messages.

The protocol MUST remain secure (content confidentiality/integrity/authentication) even if the service is malicious, provided endpoint keys are not compromised.

### 5.3 Endpoint compromise
QuantumShield assumes endpoint compromise can occur.

The protocol MUST:
- Limit retrospective exposure via FS.
- Enable recovery via PCS after the compromise ends.

The protocol does not claim to protect messages while the attacker actively controls the endpoint.

---

## 6. Evidence and conformance standard (this is the rebalance)
Security is only as real as the evidence that continuously verifies it.

### 6.1 Evidence hierarchy
A feature is “real” only when it is backed by:
1) **Normative requirement** in QSP/QSE/SOTM.
2) **Executable test coverage** (vectors and/or property tests).
3) **Interop coverage** (at least two independent implementations or a refimpl + harness acting as the second party).
4) **Negative testing** proving fail-closed behavior.

### 6.2 Canonical test artifact formats (Phase 4)
All conformance artifacts MUST be represented in Phase 4 canonical formats:
- **Vector Set:** `QSHIELD-P4-VECTOR-SET-1`
- **Interop Set:** `QSHIELD-P4-INTEROP-SET-1`

Adapters MAY be used to transform Phase 3 artifacts into canonical Phase 4 format, but the CI pipeline MUST execute only canonical outputs.

### 6.3 Required test categories (minimum bar)
CI MUST include, at minimum:
- **Parsing rejection tests:** truncation, length overruns, trailing bytes, unknown versions/flags, invalid sizes.
- **Downgrade tests:** suite mismatch rejection, protocol version mismatch rejection.
- **Handshake authenticity tests:** KT failures, signature failures, transcript binding failures.
- **State machine tests:** out-of-order handling bounds, skipped-key management bounds, replay handling.
- **PCS/FS regression tests:** controlled compromise scenarios at the harness level (where feasible).
- **QSE DoS bounds tests:** MAX_* enforcement and strict parsing.

### 6.4 Fail-closed CI rule
CI MUST fail if:
- Any required artifact is missing.
- Any vector schema validation fails.
- Any vector executes with an unexpected outcome.
- Negative vectors are parsed but not executed.
- Interop cases cannot be executed end-to-end.

---

## 7. Change control and complexity budget
### 7.1 Primary suite policy
Suite-1B (`suite_id=0x0002`) is the **primary** target for conformance and review.

Suite-1 (`suite_id=0x0001`) may be supported only for legacy reasons and MUST:
- be explicitly marked “legacy,”
- never be the default,
- never receive new features.

### 7.2 Complexity discipline
New protocol mechanisms MUST NOT be introduced unless they:
- directly satisfy an objective in this document,
- are specified normatively,
- have vectors + negative vectors,
- have interop coverage.

---

## 8. Immediate project milestones (practical)
This document is the starting line. The next milestones are:

1) **Land this document** (SOTM) under `specs/` and link it from QSP/QSE.
2) **Land Phase 4 canonical schemas** and adapters for Phase 3 artifacts.
3) **Wire negative vectors into execution** so CI becomes a “truth machine.”
4) Freeze protocol feature work until CI is green and stable.

