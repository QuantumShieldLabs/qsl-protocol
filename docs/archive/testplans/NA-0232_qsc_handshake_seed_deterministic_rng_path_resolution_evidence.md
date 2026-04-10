Goals: G4, G5

Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-10
Replaces: n/a
Superseded-By: n/a

# NA-0232 QSC_HANDSHAKE_SEED Deterministic RNG Path Resolution Evidence

## Summary

`NA-0232` resolved the staged `QSC_HANDSHAKE_SEED` deterministic-RNG path as a live current-main runtime issue.

Implementation PR: #685, `Resolve NA-0232 handshake seed path`
Implementation merge SHA: `24d7a5a5d93e`
Implementation mergedAt: `2026-04-10T12:11:44Z`
Implementation head SHA: `ce78c092a937`

## Implementation Evidence Surfaces On Main

- `DECISIONS.md` `D-0400` records that refreshed current-main verification found the shipped/shared `qsc` handshake path still reached `QSC_HANDSHAKE_SEED` through `perform_handshake_init_with_route()` -> `hs_session_id()` -> `hs_rand_bytes()`.
- `TRACEABILITY.md` contains the `NA-0232 implementation/evidence` entry pointing to the runtime, regression, audit, decision, traceability, journal, and companion testplan surfaces changed by PR #685.
- `qsl/qsl-client/qsc/src/handshake/mod.rs` removes the environment-read branch from `hs_rand_bytes()` so shipped/shared handshake runtime fills from `OsRng`.
- `qsl/qsl-client/qsc/tests/handshake_mvp.rs` adds `handshake_seed_env_does_not_steer_session_id`, which sets `QSC_HANDSHAKE_SEED` for repeated production handshake init attempts and proves the session IDs are not reproducible.
- `docs/audit/DOC-AUD-003_Security_Audit_Packet_Intake_and_Remediation_Plan_v0.1.0_DRAFT.md` marks `F02` resolved and keeps `F03` plus `F04` as the remaining still-live Tier 0 items.

## Exact Implementation Outcome

- Before PR #685, the production/shared path permitted deterministic handshake randomness for handshake session IDs when `QSC_HANDSHAKE_SEED` was present.
- PR #685 removed that environment-read path from shipped/shared runtime rather than retaining a production feature flag or fallback.
- Direct regression coverage proves repeated `QSC_HANDSHAKE_SEED` settings no longer reproduce handshake session IDs.
- No unrelated runtime surfaces changed: PR #685 only touched the bounded handshake runtime, one handshake regression file, and governance/evidence companions.

## Acceptance Proof Surface

- The production/shared handshake path no longer carries an unresolved deterministic-RNG claim.
- Representative handshake and cross-seam canaries remained green in the implementation/evidence lane.
- No unrelated protocol, service, wire, auth, state-machine, qsc-desktop, qsl-server, qsl-attachments, `.github`, website/public-runtime, `Cargo.toml`, or `Cargo.lock` surface changed.
- The staged 8-file audit packet remains present and unchanged on main.

## Closeout Scope

This closeout PR is governance-only. It records durable archive evidence, marks `NA-0232` DONE, and promotes the next canonical Tier 0 successor without reopening runtime implementation.
