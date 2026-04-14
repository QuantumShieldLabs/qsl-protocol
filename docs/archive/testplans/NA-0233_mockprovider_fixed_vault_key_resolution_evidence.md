Goals: G4, G5

Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-13
Replaces: n/a
Superseded-By: n/a

# NA-0233 MockProvider Fixed Vault Key Resolution Evidence

## Summary

`NA-0233` resolved the staged MockProvider fixed/default vault-key issue as a live current-main runtime issue.

Implementation PR: #688, `NA-0233 MockProvider Fixed Vault Key Resolution`
Implementation merge SHA: `c6c5f44e32b5`
Implementation mergedAt: `2026-04-13T01:06:44Z`
Implementation head SHA: `fa7318d5b2c8`

## Implementation Evidence Surfaces On Main

- `DECISIONS.md` `D-0406` records that refreshed current-main verification found the shipped/shared `qsc` path still accepted production `mock` key-source selection, still derived the fixed MockProvider key for legacy tag `4`, and still reached auto-unlock from bootstrap and TUI flows before the implementation lane retired that path.
- `TRACEABILITY.md` contains the `NA-0233 implementation/evidence` entry pointing to the merged runtime, regression, audit, decision, traceability, journal, and companion testplan surfaces changed by PR #688.
- `qsl/qsl-client/qsc/src/vault/mod.rs` now rejects production `mock` key-source selection, retires legacy tag `4` fixed-key derivation with truthful fail-closed `vault_mock_provider_retired` / `mock_retired` handling, and prevents the shipped/shared vault runtime from reopening the retired fixed-key path.
- `qsl/qsl-client/qsc/src/main.rs` and `qsl/qsl-client/qsc/src/tui/controller/commands/dispatch.rs` no longer reach MockProvider-based auto-unlock from shipped/shared bootstrap or TUI flows.
- Directly affected `qsl/qsl-client/qsc/tests/**` consumers on main now use explicit passphrase-backed helpers or derived envelope keys instead of relying on a production-reachable fixed/default MockProvider key path.
- `docs/audit/DOC-AUD-003_Security_Audit_Packet_Intake_and_Remediation_Plan_v0.1.0_DRAFT.md` marks `F03` resolved and keeps `F04` as the next still-live Tier 0 issue.

## Exact Implementation Outcome

- Before PR #688, the production/shared path permitted fixed/default MockProvider vault-key behavior through `mock` key-source acceptance, fixed-key derivation, and auto-unlock reachability.
- PR #688 retired that fixed/default key path from shipped/shared runtime rather than leaving a production fallback or compatibility flag in place.
- Existing `key_source=4` envelopes are now handled fail-closed with truthful retirement diagnostics instead of silently unlocking.
- Direct regression coverage and directly affected consumer updates are now on main.
- No unrelated runtime surfaces changed: PR #688 stayed within the bounded vault/bootstrap/TUI seam, directly affected tests, and required governance/evidence companions.

## Acceptance Proof Surface

- The production/shared path no longer carries an unresolved fixed/default MockProvider vault-key claim.
- Representative handshake and cross-seam canaries remained green in the implementation/evidence lane.
- No unrelated protocol, service, wire, auth, state-machine, qsc-desktop, qsl-server, qsl-attachments, `.github`, website/public-runtime, `Cargo.toml`, or `Cargo.lock` surface changed.
- The staged 8-file audit packet remains present and unchanged on main.

## Closeout Scope

This closeout PR is governance-only. It records durable archive evidence, marks `NA-0233` DONE, and promotes the next canonical Tier 0 successor without reopening runtime implementation.
