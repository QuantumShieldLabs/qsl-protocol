Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-10

# NA-0231 ML-DSA-65 Timing Oracle Resolution Evidence

Goals: G4, G5

## Scope

- Repo: `qsl-protocol`
- Lane: `NA-0231`
- Posture: implementation/evidence already merged; this closeout is governance-only and introduces no runtime changes
- Implementation PR: #683
- Implementation branch head before merge: `4282a0391cd4`
- Implementation merge SHA: `8db0e709a37c`
- Implementation mergedAt: `2026-04-10T02:14:13Z`

## Authority Proof

- before this governance-only closeout branch mutated docs, refreshed `qsl-protocol` `main`, `mirror/main`, and `origin/main` all resolved to `8db0e709a37c`
- refreshed merged main contains `DECISIONS.md` `D-0398`, the `TRACEABILITY.md` `NA-0231 implementation/evidence` entry, `.cargo/audit.toml` with the tooling-only advisory suppression narrative, the merged handshake tamper tests in `qsl/qsl-client/qsc/tests/handshake_mvp.rs`, and the canonical intake artifact's stale-on-main update
- refreshed live queue still showed `READY_COUNT=1` with `NA-0231` as the sole live `READY` item before this closeout
- refreshed sibling-repo proof stayed read-only and truthful: `qsl-server READY=0`, `qsl-attachments READY=0`
- PR `#683` is already `MERGED`; refreshed merged `main` is authoritative for queue truth, and this closeout lane does not mutate that PR

## Implementation Evidence Source On Main

- `DECISIONS.md` `D-0398`
- `TRACEABILITY.md` `NA-0231 implementation/evidence`
- `.cargo/audit.toml`
- `qsl/qsl-client/qsc/tests/handshake_mvp.rs`
- `docs/audit/DOC-AUD-003_Security_Audit_Packet_Intake_and_Remediation_Plan_v0.1.0_DRAFT.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0231_rolling_journal_entry_testplan.md`

## Exact Implementation Summary

- refreshed current-main proof showed the shipped/runtime `qsc` path already resolves to patched `ml-dsa 0.1.0-rc.7`
- the RustSec/GitHub advisory pair for the ML-DSA issue scopes the patched range as `>= 0.1.0-rc.3`, and the current production/shared verify path is not on an affected implementation version
- the remaining `RUSTSEC-2025-0144` ignore in `.cargo/audit.toml` is tooling-only for the `refimpl_actor` direct `ml-dsa 0.0.4` lock entry
- direct B1/A2 signature-tamper regressions were added in `qsl/qsl-client/qsc/tests/handshake_mvp.rs`
- no production runtime crypto path required a new patch in this lane

## Acceptance-Proof Surface

- the production/shared verify path no longer carries an unresolved ML-DSA timing-oracle claim
- stale-on-main resolution is captured truthfully in `DECISIONS.md`, `TRACEABILITY.md`, `.cargo/audit.toml`, and the canonical intake artifact
- representative handshake and cross-seam canaries remained green in the implementation/evidence lane
- no unrelated protocol, service, wire, qsc-desktop, qsl-server, qsl-attachments, `.github`, website, `Cargo.toml`, or `Cargo.lock` drift occurred

## Closeout Note

- `NA-0231` is now closed truthfully because its merged implementation/evidence state is durable on refreshed `main`
- this closeout PR is governance-only and introduces no runtime changes
- refreshed `DOC-AUD-003` identifies `F02` / `QSC_HANDSHAKE_SEED` as the next Tier 0 blocker after stale `F01`, while KT remains prerequisite-blocked; therefore the truthful direct successor is `NA-0232 — QSC_HANDSHAKE_SEED Deterministic RNG Path Resolution`
