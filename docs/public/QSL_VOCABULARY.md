Goals: G1, G4, G5

Status: Supporting
Owner: QSL documentation
Last-Updated: 2026-10-22
Replaces: n/a
Superseded-By: n/a

# QSL design vocabulary

These names help readers navigate QSL's design work. They are presentation labels, not protocol identifiers, new cryptographic primitives or evidence that a feature has shipped. Published specifications, source revisions and qualification records retain their own authority and status.

| Name | Meaning | Boundary |
|---|---|---|
| **Contact Capsule** | Invitation data used to begin contact setup. | Possession does not establish a verified identity. Authentication, peer verification, expiry, revocation and replay handling remain separate requirements. The label does not select a new invitation format. |
| **Key Continuity** | Message-key evolution and incorporation of fresh key-agreement material. | The label does not promise post-compromise recovery or forward secrecy under every compromise. Exact schedules, authentication, erasure and assumptions remain in the specifications and evidence. |
| **Commit Path** | The requirement to make authorizing state durable before external output and to retry the exact committed operation. | Relay acceptance, endpoint admission, protocol retirement and a person reading are distinct states. The label does not certify power-loss durability or completed desktop integration. |
| **Quiet Transit** | Proposed research into reducing metadata and timing exposure. | This is not a deployed privacy profile or an established anonymity guarantee. Padding, scheduling, cover traffic, route separation and storage privacy require explicit decisions, implementation, measurement and independent review. |

## Technical names and research lineage

Use the QSL names to introduce concepts, then give the precise technical definition. Retain standardized primitive names, existing wire and contract identifiers, formal document titles and historical evidence. A new public label does not amend an accepted contract or adopt a proposed design.

The [canonical drafts](../canonical/) use established cryptographic concepts. In particular, Sparse Continuous Key Agreement (SCKA) was introduced by [Auerbach et al., USENIX Security 2025](https://www.usenix.org/conference/usenixsecurity25/presentation/auerbach). Key Continuity is a broader QSL presentation label; it does not rename that research notion or claim its invention. Other research dependencies should be attributed where used.

Read the [claim boundary](SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md) and [evidence guide](INDEX.md) before interpreting any design name as a security property. QSL remains research-stage, not independently audited and not production-ready.
