[README.md](https://github.com/user-attachments/files/32136231/README_qsl-protocol.md)

# QSL Protocol

**The specification, conformance vectors, and reference implementation for QSL — a
post-quantum-first secure messaging protocol.**

Encryption that is unbreakable today is not automatically unbreakable tomorrow. An adversary
with enough storage can record traffic now and decrypt it years later, once a quantum computer
can break the classical key exchange that protected it. This is called **harvest now, decrypt
later**, and it is why messages that must stay private for a decade need post-quantum
protection today.

This repository is where that protocol is designed in the open — written down first, then
implemented against vectors, with the gaps recorded alongside the proofs.

> [!WARNING]
> **Research-stage. Not independently audited. Not production-ready.**
> Do not use QSL to protect anyone whose safety depends on it.
> See [Status](#status-what-is-and-is-not-established).

---

## The design in one page

QSL **Suite-2** (`protocol_version = 0x0500`, `suite_id = 0x0002`) is a **True Triple
Ratchet**. Three ratchets run in composition so that *every* message key is hybrid — derived
from a classical secret and a post-quantum secret together.

| Ratchet | Construction |
|---|---|
| Classical DH | X25519, advancing the root key at epoch boundaries |
| Classical symmetric | `CK_ec -> ec_mk`, per message |
| Post-quantum symmetric | `CK_pq -> pq_mk`, per message |
| **Hybrid combiner** | `mk = KMAC32(ec_mk, "QSP5.0/HYBRID", pq_mk \|\| 0x01)` |

Breaking the classical half is not enough. Breaking the post-quantum half is not enough.

**Primitives.** ML-KEM-768 (KEM) · ML-DSA-65 (signatures) · X25519 · AES-256-GCM (12-byte
nonce, 16-byte tag) · SHA-512 · KMAC-256 for every key derivation, with domain-separated
labels throughout.

### The interesting problem: SCKA

Running an ML-KEM encapsulation per message is expensive in bandwidth. Skipping post-quantum
material between key exchanges means most messages are not actually hybrid. Neither answer is
good.

QSL's answer is **SCKA — Sparse Continuous Key Agreement** ([DOC-CAN-004](docs/canonical)).
The post-quantum chain is seeded at establishment and *reseeded* by sparse ML-KEM events
carried in message prefix fields. The PQ ratchet keeps advancing continuously while KEM
traffic stays sparse, so per-message hybrid security does not cost per-message KEM bandwidth.
Reseed events are monotonic, fail-closed, and crash-safe.

This is the part of the design most worth attacking. If you find a flaw in it, that is the
most valuable contribution this project can receive.

### Design commitments

- **Fail-closed everywhere.** Unknown version or suite is rejected with no silent fallback.
  Negotiation is downgrade-resistant and transcript-bound.
- **Transactional commit.** Ratchet state is persisted only when the header *and* the body
  decrypt and every bound check passes. A rejected message mutates nothing.
- **Contributory DH.** All-zero X25519 outputs are rejected (RFC 7748 §6.1), on the sending
  side before the root absorbs them as well as on receive.
- **Explicit reject codes.** Failures map to stable, documented reason codes rather than a
  generic error, so conformance vectors can pin exact behaviour.
- **Bounded everything.** Skipped keys, header attempts, and out-of-order windows all carry
  declared limits. An unbounded work request from the wire is a denial-of-service primitive.
- **`VERIFIED` is not `TRUSTED`.** The trust model is per-device and fail-closed. Delivery
  semantics are explicit too: `accepted_by_relay` is a different fact from `peer_confirmed`.

---

## What is in this repository

```
docs/canonical/      normative specifications (DOC-CAN-003 ratchet, DOC-CAN-004 SCKA, ...)
docs/public/         evidence index, external review package, release-readiness map
inputs/suite2/       conformance vectors
tools/refimpl/       the reference implementation
qsl/qsl-client/qsc/  the client core library and its command-line harness
formal/              bounded model checks
tests/               conformance and negative-path suites
```

**Reading order for reviewers:**

1. [`docs/canonical/`](docs/canonical) — **DOC-CAN-003** (the Suite-2 ratchet) then
   **DOC-CAN-004** (SCKA). These are the normative documents; everything else implements them.
2. [`inputs/suite2/`](inputs/suite2) — the conformance vectors.
3. [`tools/refimpl/`](tools/refimpl) — the reference implementation. The desktop client links
   this crate in-process; it is the shipping cryptography, not a parallel demo.
4. [`docs/public/INDEX.md`](docs/public/INDEX.md) — the evidence index, which links each claim
   to the proof or the gap behind it.

**On `qsc`, the command-line client:** it is a laboratory instrument, not the product. It
exposes affordances the shipped application deliberately does not, so that protocol behaviour
can be driven directly in tests. Do not read its interface as a user-facing design.

---

## Conformance

Fourteen Suite-2 vector categories are defined, covering key derivation, establishment,
negotiation and downgrade resistance, parsing, out-of-order delivery and replay, epoch
boundaries, PQ reseed, crash recovery, transcript binding, message-key handling, end-to-end
receive, and cross-implementation interop.

**Coverage is real and it is not complete.** We would rather you knew that than found out
later. If you can name a protocol behaviour that no category can reach, that is a finding in
itself and we want it filed.

---

## Status: what is and is not established

**Established:**

- Deterministic conformance vectors across the fourteen Suite-2 categories, with the reference
  implementation checked against them.
- Bounded formal and model checks over negotiation, transcript binding, and SCKA.
- Selected fail-closed negative paths: wrong peer, stale or replaced peer, replay, corrupt
  delivery.
- End-to-end delivery over a real relay, both directions, with byte-identical payloads.
- An internal adversarial review found a handshake authentication flaw, which was fixed, had
  its whole class retired, and is now covered by a bounded model check. That review is
  published rather than quietly folded in.

**Not established, and we will not imply otherwise:**

- **No external security audit has been performed.** Our own reasoning is not a substitute.
- Not production-ready. Not a public internet service readiness claim.
- The canonical specifications are marked **DRAFT** and they mean it.
- Not an anonymity system, and not metadata-free. A relay operator can observe traffic
  patterns — which mailboxes are active, how much, how large, and when. Message *content* is
  end-to-end encrypted and a relay cannot read it. Metadata resistance is open work.
- No claim of side-channel freedom, vulnerability freedom, or bug freedom.
- Open implementation defects exist and are tracked in the open. A protocol can be correctly
  specified and still have bugs that lose messages. We find them, file them, and fix them in
  public — check the repository's issues and [`TRACEABILITY.md`](TRACEABILITY.md) for current
  state.

---

## How to help

The most valuable contributions, in order:

1. **Attack the specifications.** Tell us where the normative text is ambiguous, circular, or
   silent on a case that occurs in practice. A flaw in DOC-CAN-003 is worth more than a flaw
   in one implementation of it.
2. **Build an independent implementation.** Cross-implementation interop is a defined
   conformance category, and a second implementation is the strongest test a spec can get.
3. **Name a conformance gap.** If two categories each look sound but their union misses a real
   behaviour, that gap is exactly where defects survive.
4. **Propose negative tests.** Most of what matters here is what the protocol *refuses* to do.

See [`CONTRIBUTING.md`](CONTRIBUTING.md) for workflow. Follow goals and accepted decisions
through [`TRACEABILITY.md`](TRACEABILITY.md), [`DECISIONS.md`](DECISIONS.md), and
[`NEXT_ACTIONS.md`](NEXT_ACTIONS.md).

---

## Security reporting

Please do **not** file security-sensitive reports in public issues.

Use GitHub private vulnerability reporting on this repository, or follow
[`SECURITY.md`](SECURITY.md). If private reporting is unavailable, open a minimal public issue
with **no exploit details**, stating that you can share specifics privately.

---

## Related repositories

- [**qsl-desktop**](https://github.com/QuantumShieldLabs/qsl-desktop) — the desktop client
- [**qsl-server**](https://github.com/QuantumShieldLabs/qsl-server) — the relay
- [**qsl-attachments**](https://github.com/QuantumShieldLabs/qsl-attachments) — encrypted
  attachment plane

## License

`AGPL-3.0-only` — see [`LICENSE`](LICENSE). Any future commercial services or support
offerings are separate from this repository and do not replace the AGPL terms on the source
published here.
