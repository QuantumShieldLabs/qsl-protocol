Goals: G5

# DOC-G5-001 — Metadata Threat Model (v1.0.0 DRAFT)

Status: DRAFT
Scope: Suite-2 demo transport and relay (non-production). This document defines the metadata threat model for NA-0016.
Non-goals: anonymity networks, mixnets, PIR, global cover traffic, or “metadata eliminated” claims.

## 1. Metadata categories
- Identifiers: stable ids, session ids, relay token presence (not the token value).
- Contact graph: which parties communicate with whom.
- Timing: message send/poll cadence, relay queueing time.
- Size: message and bundle sizes, queue depth signals.
- IP/location: client IP as seen by relay or network observers.
- Server-side linkability: relay-visible ids and request correlation.
- Retention/logging: what is stored on disk or in memory and for how long.

## 2. Adversaries
- Relay operator: can observe request metadata and stored records.
- Network observer: can observe traffic patterns and sizes.
- Active probing client: can call relay endpoints to infer state.
- Compromised endpoint: may reveal local store/config and session identifiers.

## 3. Assumptions
- Demo relay is local-only by default; public exposure is explicitly gated.
- Suite-2 protocol core remains unchanged; this is transport-layer behavior only.
- Actors are untrusted inputs; fail-closed behavior is required on malformed data.

## 4. Non-goals (explicit)
- This is not an anonymity system and does not hide IP-level metadata.
- No claims of perfect unlinkability or traffic analysis resistance.
- No claims about third-party relay trustworthiness.

## 5. Success metrics (what is minimized vs exposed)
Minimized (baseline):
- Accidental exposure: default loopback binding and token-authenticated endpoints.
- Unauthorized API access: reject unauthenticated register/send/poll/bundle requests.
- Unbounded resource growth: enforce request size limits and queue caps.

Exposed (residual):
- Timing of requests and polling cadence.
- Message size (prior to any padding profile).
- Stable ids used by demo CLI unless explicitly rotated (not yet implemented).

## 6. Residual leakage (honest claims)
- Relay operator can observe ids and message sizes by default.
- Network observer can infer timing and size characteristics.
- Local store artifacts can reveal peer ids and session ids if the endpoint is compromised.

## 7. Out-of-scope mitigations (future NA-0016 steps)
- Padding/bucket profiles and batching/jitter policies.
- Identifier rotation or short-lived capabilities.
- Retention/purge policies beyond in-memory demo default.
