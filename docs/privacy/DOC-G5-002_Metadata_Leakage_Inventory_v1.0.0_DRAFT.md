Goals: G5

# DOC-G5-002 — Metadata Leakage Inventory (v1.0.0 DRAFT)

Status: DRAFT
Scope: Current demo relay/CLI surface (post NA-0016 PR1 hardening). This is a snapshot of what is observable today.

## 1. Surface inventory (demo relay/CLI)
Endpoints:
- GET /health (public)
- GET /bundle/<id> (token required)
- POST /register (token required)
- POST /consume (token required)
- POST /establish_record (token required)
- POST /send (token required)
- POST /poll (token required)

Auth:
- Bearer token required for bundle/register/consume/establish_record/send/poll.
- Token provided via QSHIELD_RELAY_TOKEN or config (no logging; demo-only).

Binding:
- Loopback-only by default; non-loopback requires explicit unsafe flags (demo-only).
- Bundles include a self-declared `bundle.id` that must match the requested peer id; establish rejects missing/mismatched binding.

Bounds:
- Request body size capped (64 KiB).
- Queue caps enforced (per-recipient + global).
- Per-token send quota enforced (429 with retry_after_ms).
- Rate limits enforced for /register and /poll (429 with retry_after_ms).
- Relay identifiers must be 1-64 chars of [a-z0-9_-]; invalid ids reject with 400.
- Duplicate /register attempts reject with 409.

## 2. Leakage table

| Field / Observable | Where it appears | Who can observe | Persisted? | Default mitigation (PR1) | Remaining risk | Planned mitigation (NA-0016)
|---|---|---|---|---|---|---|
| Peer id (stable) | /register, /send, /poll payloads | Relay, network observer | In memory (relay) and local store | Token required; loopback-only default | Stable identifier linkability | Identifier rotation / opaque handles
| Session id | Local store, actor ops | Endpoint, compromised host | On disk (store) | Store dir 0700, files 0600 by default | Local compromise reveals ids | Store encryption / access controls
| Store file permissions | Local filesystem (config/state) | Local users on host | On disk | Store dir 0700; files 0600 | Mis-set umask/permissions | Enforce ownership checks / encryption
| Relay bind address | CLI config, relay logs | Endpoint, local operator | On disk (config) | Loopback-only default | Misconfiguration risk | Safer defaults + warnings (done)
| Relay token presence | HTTP headers (not value) | Relay, network observer | No | Token required for mutating endpoints | Token value exposure if logged | Explicit no-log guidance
| Message size | /send payload, queue storage | Relay, network observer | In memory | Optional bucket padding | Bucket fingerprinting | Padding/bucketing profile tuning
| Padding bucket | /send payload, queue storage | Relay, network observer | In memory | Bucket list is explicit when enabled | Bucket category leakage | Profile refinement or rotation
| Timing (poll cadence) | /poll frequency | Relay, network observer | No | None | Timing analysis | Optional batching/jitter knobs
| Queue length / overflow | HTTP 429/413 responses | Client, relay | No | Queue caps enforced | Observable backpressure | Error normalization policy
| Request body size limit | HTTP 413 | Client, relay | No | Enforced in relay | Observable via errors | Error normalization policy
| Local actor path | Env var QSHIELD_ACTOR | Endpoint | On disk/env | None | Misconfiguration/privilege | Documented constraints

## 3. Storage artifacts
- Store config: relay_url, relay_token (if set); default perms 0600.
- Store state: ids, session ids, public keys (demo-only); default perms 0600.
- Store rotation: `qshield rotate` overwrites + deletes config/state (best effort) and requires re-init.
- Relay state: bundles and queued messages (in-memory only; bundles removed on consumption).
- Relay state: establish replay fingerprints (SHA-256 of peer_id, bundle_id, session_id_hex, dh_init, pq_init_ss).

## 4. Notes
- This inventory is intentionally honest and non-exhaustive; it is a baseline for NA-0016 improvements.
- Future steps must update this table when mitigations change observable fields.
