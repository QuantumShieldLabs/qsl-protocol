Goals: G4, G5

Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-15
Replaces: n/a
Superseded-By: n/a

# NA-0234 Vault Read-Path KDF Floor / Format Acceptance Resolution Evidence

## Summary

`NA-0234` resolved the staged vault read-path KDF-floor / format-acceptance issue as a live current-main runtime issue.

Implementation PR: #693, `NA-0234: resolve vault read-path KDF acceptance`
Implementation merge SHA: `7c48828fc1ef`
Implementation mergedAt: `2026-04-14T02:18:14Z`
Implementation head SHA: `e748833403c9`

## Implementation Evidence Surfaces On Main

- `DECISIONS.md` `D-0408` records that refreshed current-main verification found the shipped/shared passphrase-vault read path still accepted stored Argon2 profiles below the deployed floor or otherwise outside the one truthful write-time profile before the implementation lane tightened that path.
- `TRACEABILITY.md` contains the `NA-0234 implementation/evidence` entry pointing to the merged runtime, regression, audit, decision, traceability, journal, and companion testplan surfaces changed by PR #693.
- `qsl/qsl-client/qsc/src/vault/mod.rs` now rejects any passphrase vault whose stored KDF profile differs from the exact write-time `19456/2/1` profile and derives the runtime key from those canonical constants only.
- `qsl/qsl-client/qsc/tests/vault.rs` now carries direct regressions proving both below-floor and otherwise non-canonical passphrase profiles fail closed without mutating the vault.
- `docs/audit/DOC-AUD-003_Security_Audit_Packet_Intake_and_Remediation_Plan_v0.1.0_DRAFT.md` marks `F04` resolved on refreshed `main`.

## Exact Implementation Outcome

- Before PR #693, the shipped/shared vault read path accepted sub-floor or otherwise non-canonical passphrase KDF/profile parameters because it derived directly from stored envelope fields.
- PR #693 now enforces the canonical passphrase-vault profile on `main`: passphrase vault reads reject any stored profile other than the exact write-time `19456/2/1` profile while leaving non-passphrase key sources explicit.
- Direct regression coverage and directly affected consumer updates are now on `main`.
- No unrelated runtime surfaces changed: PR #693 stayed within the bounded vault seam, directly affected test surface, and required governance/evidence companions.

## Acceptance Proof Surface

- The shipped/shared vault read path no longer carries an unresolved below-floor / untruthful format-acceptance claim.
- Representative vault and cross-seam canaries remained green in the implementation/evidence lane.
- No unrelated protocol, service, wire, auth, state-machine, qsc-desktop, qsl-server, qsl-attachments, `.github`, website/public-runtime, `Cargo.toml`, or `Cargo.lock` surface changed.
- The staged 8-file audit packet remains present and unchanged on `main`.

## Closeout Scope

This closeout PR is governance-only. It records durable archive evidence, marks `NA-0234` DONE, and promotes the next truthful successor without reopening runtime implementation.
