Goals: G1, G4, G5

Status: Supporting
Owner: QSL documentation
Last-Updated: 2026-11-12
Replaces: n/a
Superseded-By: n/a

# QuantumShield Chat: vault progress and the QSL/KP direction

**Maintainer update · November 12, 2026**

We’re building a messenger that brings together hybrid post-quantum encryption, a locally controlled vault and planned Katzenpost (KP) transport. The aim is to protect message content while addressing the separate privacy problem of communication patterns.

QSL remains **research-stage, not independently audited and not production-ready**. Ordinary desktop messaging is not yet available. Do not use it for safety-critical communications.

## Read this update in three parts

| Category | What it means here |
|---|---|
| Published source | The public repositories contain protocol drafts, conformance inputs, a Rust client core, a desktop prototype and supporting services. Their default branches and dated evidence define what an outside reviewer can reproduce. |
| Development progress | The focused results below are maintainer-reported work in development candidates. This update does not claim that every change or its full evidence package is already published. |
| Intended experience | The v6 interface mockups and QSL/KP direction describe what we are working toward. They do not establish implemented features or adopted protocol changes. |

## What has moved forward

Focused application checks in development candidates covered:

- **Encrypted private storage:** sending and reopening state with the intended encrypted storage path.
- **Tamper refusal:** rejecting modified encrypted state while preserving the relevant state rather than silently accepting it.
- **Sensitive-output redaction:** checking success and refusal output for protected material.
- **Guarded unreadable-vault deletion:** checking the core path for an exact confirmation phrase, a waiting period, cancellation and deletion of the intended file.

These are bounded engineering results. They do not establish a complete vault, all GUI behavior, platform-wide keychain qualification, a new public release or an independent security audit. Source composition and the remaining validation still matter.

## What comes next

1. **Finish the vault baseline.** Compose accepted fixes and complete the required persistence, refusal, recovery, regression and platform coverage, with independent review. Validate the complete application path, not just isolated helpers.
2. **Prove the real QSL/KP composition.** After the vault and necessary application prerequisites, connect two QSL instances through real KP. Exercise encrypted exchange, authenticated confirmation, restart, retry and transport failure; measure the operating costs and state the remaining privacy limits.
3. **Complete the desktop experience.** Bring the validated paths into ordinary contact setup, verified identity handling, messaging and recovery. The current public prototype and the interface mockup are different artifacts.
4. **Qualify a release.** Required security analysis, independent review, platform testing, packaging and release gates remain. There is no announced production release date.

This is a public summary of direction, not a replacement for the implementation queue or permission to skip a gate. Published historical results keep their original revisions and scope.

## How QSL and Katzenpost fit together

The intended layering is:

```text
Sender device              Katzenpost transport               Recipient device
QSL encrypts content  →  encrypted routing + mixing  →  QSL authenticates/decrypts
```

**QSL protects the conversation at the endpoints.** Suite-2 is a research-stage Triple-Ratchet-style hybrid messaging design. Classical and post-quantum secret material contributes to message keys; the precise construction and assumptions are in the [canonical drafts](../canonical/) and [claim boundary](SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md).

**KP addresses a different problem during transport.** Its mix network uses encrypted routing, delays and cover traffic to make communication patterns harder to correlate. Those mechanisms have bandwidth and latency costs, and their guarantees depend on the threat model and deployment. See [Katzenpost’s documentation](https://katzenpost.network/docs/) and [mix-network specification](https://katzenpost.network/docs/specs/mixnet/).

KP is the intended transport direction and lead integration candidate. The real composition remains to be validated. QSL does not currently establish anonymity, resistance to a global traffic observer or a proven end-to-end composition with KP. A design intention is not a completed integration. Katzenpost is independent; this update implies no partnership or endorsement.

Transport acceptance, recipient-device confirmation and a person reading a message are different events. The planned interface must keep those distinctions clear. Confirmation traffic also needs privacy analysis; an encrypted receipt is not a promise to hide activity from every contact.

## The latest interface preview

The [v6 preview on our website](https://quantumshieldlabs.org/preview) shows a conversation and the planned network settings. All people, conversations and connection states are synthetic. The packet performs no cryptography, identity verification, messaging or real KP connection. Groups, file sharing, device management and recovery controls shown in the design are proposals subject to implementation and review.

We welcome feedback about clarity, accessibility and how security decisions are explained. For reproducible implementation reports, include the affected source revision and use disposable data. For sensitive findings, follow [SECURITY.md](../../SECURITY.md) rather than posting exploit details publicly.

## Inspect the project

- [Public evidence guide](INDEX.md) — scope and limits of published evidence.
- [Release-readiness map](RELEASE_READINESS_EVIDENCE_MAP.md) — dated evidence and outstanding gates.
- [Desktop repository](https://github.com/QuantumShieldLabs/qsl-desktop) — current public feature limits.
- [Relay repository](https://github.com/QuantumShieldLabs/qsl-server) — existing mailbox/invitation service.
- [Encrypted-object service](https://github.com/QuantumShieldLabs/qsl-attachments) — separate attachment storage contract.

The existing services remain documented on their own terms. This update does not announce their retirement or a completed migration to KP.
