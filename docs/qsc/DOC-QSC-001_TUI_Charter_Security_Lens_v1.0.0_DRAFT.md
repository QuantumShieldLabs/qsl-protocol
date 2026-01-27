# DOC-QSC-001 — TUI Charter: Security Lens (v1.0.0 DRAFT)

## Purpose
The qsc TUI is a **Security Lens** into protocol truth. It exposes state and events safely and deterministically without mutating state unless the user issues explicit commands.

## Non-negotiables (Must Never)
- **No implicit sends.** All sends must be explicit and user-confirmed.
- **No automatic retries.** Failures must be shown and must not retry silently.
- **No background recovery.** Recovery must be explicit and observable.
- **No auto key/identity changes** without an explicit command + deterministic marker.
- **No secret material in display, markers, or logs.** Redaction is mandatory.
- **No state mutation without emitting deterministic markers/events.**

## Allowed interactions
- `/send` — explicit send; shows prepare → send → commit lifecycle markers
- `/status` — safe state summary (redacted)
- `/envelope` — show envelope planning (bucket/tick) and ACK camouflage markers
- `/export` — redacted diagnostics export only

## Threat model notes
- Shoulder-surfing / screen recording: UI must redact secrets and minimize exposure.
- Log scraping: logs/markers must never contain secret material.
- Filesystem hostility: refuse unsafe parents/symlinks/perms (fail-closed).

## Enforcement
- Tests MUST assert charter rules (no implicit sends, deterministic markers, no secret leakage, no mutation on reject, fail-closed filesystem).
- CI MUST remain green with these checks in place.
