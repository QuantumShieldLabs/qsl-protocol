Goals: G5

# DOC-G5-003 — Envelope/Transport Profile (v0.1.0 DRAFT)

Status: DRAFT
Scope: Demo relay/CLI transport posture (non-production). This is a baseline profile, not an anonymity system.

## 1. Profile goals
- Prevent accidental unsafe exposure in demo usage.
- Make metadata leakage explicit and testable.
- Keep protocol-core behavior unchanged.

## 2. Identifier rules
- Stable identifiers (peer ids) are currently used and are observable by the relay.
- No anonymity or rotation is claimed in v0.1.
- Future NA-0016 steps may define rotating/opaque identifiers.
- Relay identifiers must be 1-64 chars of [a-z0-9_-]; invalid ids are rejected with 400.
- Duplicate /register attempts for the same id reject with 409.

## 3. Authentication and authorization
- Relay endpoints that mutate or expose data (/register, /send, /poll, /bundle, /consume, /establish_record) require a bearer token.
- Token must be provided via QSHIELD_RELAY_TOKEN or config; token value must not be logged.

## 4. Binding and exposure rules
- Default bind is loopback-only; non-loopback requires explicit unsafe acknowledgement.
- Public exposure without token auth is forbidden.
- Demo CLI warns on first establish to verify peer identity; `--demo-identity-verified` suppresses the warning.

## 5. Error behavior (baseline)
- Unauthorized requests are rejected with 401/403 (demo default).
- Oversized payloads are rejected with 413.
- Queue overflow is rejected with 429.
- Rate limiting for /register and /poll is rejected with 429 and retry_after_ms.
- Per-token send quotas are rejected with 429 and retry_after_ms.
- Invalid relay id format is rejected with 400.
- Duplicate /register attempts are rejected with 409.
- Error normalization across these cases is not yet implemented; documented as residual leakage.

## 6. Size and timing posture
- Optional size-bucket padding is supported; default is OFF.
- When padding is enabled, each message is padded to the smallest configured bucket size >= ciphertext length.
- The relay transport envelope includes:
  - `msg`: hex payload bytes (padded ciphertext)
  - `pad_len`: number of pad bytes appended (0 when padding disabled)
  - `bucket`: selected bucket size in bytes (optional, for observability/testing)
- No batching/jitter is applied in v0.1; timing remains observable.

## 7. Logging and retention posture
- Relay should not log bearer tokens or plaintext identifiers.
- In-memory storage only for demo relay; no persistence in v0.1.
- Demo CLI stores ids and session identifiers locally; treat as sensitive.
- Demo CLI rotation (`qshield rotate`) overwrites + deletes local config/state as best-effort.

## 8. Conformance invariants (CI-gated)
These invariants are enforced by CI and must remain true:
- Unauthenticated register/send/poll/bundle requests are rejected.
- Loopback-only binding is default; non-loopback requires explicit unsafe acknowledgement.
- Request size limit is enforced (413 on overflow).
- Queue caps are enforced (429 on overflow).
- Per-token quotas are enforced for /send (429 on overflow).
- Store directory defaults to 0700 and config/state files default to 0600 on Unix.
- Store rotation removes config/state artifacts (best effort) and requires re-init.
- First establish emits an identity verification warning unless `--demo-identity-verified` is provided.
- Demo unauthenticated override is explicit and off by default.
- Successful establish consumes peer bundle; reuse rejects deterministically.
- Establish rejects if bundle identity binding is absent or mismatched.
- Establish replays (same fingerprint) are rejected deterministically.
- Register/poll requests above the rate limit return 429 with retry_after_ms.
- Relay identifiers must match the allowed format; invalid ids are rejected.
- Duplicate /register attempts reject deterministically (409).
- When padding is enabled, `msg` length must match a configured bucket and `pad_len` must be consistent.

Enforced by:
- scripts/ci/metadata_conformance_smoke.sh
- .github/workflows/ci.yml (metadata-conformance-smoke job)

## 9. Residual leakage (explicit)
- Timing, size, and stable ids remain observable in v0.1.
- This profile makes no anonymity claims.
