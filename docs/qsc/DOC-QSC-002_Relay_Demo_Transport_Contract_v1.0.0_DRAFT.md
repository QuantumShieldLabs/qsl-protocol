# DOC-QSC-002 — Relay Demo Transport Contract (Security Lens)

## Purpose
Provide a relay-backed demo transport that emulates real-world conditions (latency/jitter/drop/reorder) while preserving the qsc Security Lens charter: explicit-only behavior, deterministic observability, and no mutation on failure.

## Threat Model
- Relay is hostile/unreliable: can delay, drop, reorder, duplicate, or observe metadata.
- Relay must never access keys or secret content.
- Relay behavior is reproducible when seeded.

## Strict Metadata Policy
- Relay sees only what is required for routing.
- No keys, passphrases, or raw secret material are exposed.
- Logs must be minimal and redacted; markers must be deterministic.

## Deterministic Fault Injection
- All fault injection knobs are seedable:
  - fixed latency
  - jitter window
  - drop percentage
  - duplicate percentage
  - reorder window
- Given the same seed and input, relay behavior is reproducible.

## Explicit Command Model
- No implicit sends, retries, or recovery.
- Every transport action is initiated by an explicit command.
- All lifecycle transitions emit deterministic QSC_MARK lines.

## Security Notes
- TUI must never display secrets; only safe metadata and counters.
- Relay events must be observable but not leak content.
- No persistent mutation on transport failure (prepare→attempt→commit enforced).
