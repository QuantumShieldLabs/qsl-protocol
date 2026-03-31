Status: Archive
Owner: QSL governance
Last-Updated: 2026-03-31

# NA-0217 qsc Modularization / File-Size Reduction Planning Evidence

Goals: G4, G5

## Scope

- Repo: `qsl-protocol`
- Lane: `NA-0217`
- Posture: planning-only, qbuild-first, docs/governance only
- Base commit: `b3b6d527ca02`

## Authority Proof

- `qsl-protocol` checkout `HEAD`, `mirror/main`, bare mirror `main`, and `origin/main` all resolved to `b3b6d527ca02`
- `qsl-server` checkout `HEAD`, `mirror/main`, bare mirror `main`, and `origin/main` all resolved to `0826ffa4d6f3`
- `qsl-attachments` checkout `HEAD`, `mirror/main`, bare mirror `main`, and `origin/main` all resolved to `e94107ac094d`
- `qsl-protocol` live `READY` count remained `1`, with `NA-0217` as the sole live `READY` item
- `qsl-server` remained at `READY=0`
- `qsl-attachments` remained at `READY=0`
- `qsl-protocol` had no open PR already acting on `NA-0217`
- `STATUS.md` still points to `NA-0177`; recorded as non-authoritative hygiene debt only

## Strategic-Input Note

- No dedicated transition audit / roadmap pack was locally available as a cohesive attachment for this lane.
- Historical local files existed and were inventoried only as non-authoritative background:
  - `docs/audit/METADATA_MITIGATIONS_ROADMAP_NA-0137.md`
  - `docs/audit/ONGOING_PQ_RATCHET_ROADMAP_NA-0135.md`
  - `tests/NA-0199_legacy_transition_validation.md`
  - frozen phase 2 / phase 3 zip bundles
- No planning conclusion here depends on those historical files outranking live repo truth.

## Concentration Proof

- total `qsl/qsl-client/qsc/src` LOC: `24,790`
- `qsl/qsl-client/qsc/src/main.rs` LOC: `21,627`
- `main.rs` share: `87.24%`
- largest extracted modules after `main.rs`:
  - `vault/mod.rs` -> `1,138`
  - `cmd/mod.rs` -> `776`
  - `store/mod.rs` -> `240`
  - `adversarial/route.rs` -> `196`
  - `envelope.rs` -> `193`

Approximate live `main.rs` cluster inventory used for the plan:
- CLI bootstrap and dispatch -> `679` LOC
- TUI shell and rendering -> `9,650` LOC
- status / session-state / delivery metadata -> `1,992` LOC
- attachment / file-transfer / receipt pipeline -> `2,455` LOC
- QSP pack/unpack / handshake / identity -> `1,143` LOC
- contacts / trust / routing / timeline -> `2,695` LOC
- send / receive / relay transport -> `1,991` LOC
- output / store-safety / utility foundations -> `1,022` LOC

## Fragile-Zone Proof

Representative live proofs explicitly reviewed before shaping the seam order:
- marker formatting / deterministic diagnostics:
  - `qsl/qsl-client/qsc/tests/tui_charter.rs`
  - `qsl/qsl-client/qsc/tests/tui_product_polish_na0214a.rs`
  - `qsl/qsl-client/qsc/tests/route_header_migration_docs_na0195a.rs`
  - `qsl/qsl-client/qsc/tests/remote_soak_diag_mapping_na0168.rs`
- TUI rendering and headless scripting:
  - `qsl/qsl-client/qsc/tests/tui_charter.rs`
  - `qsl/qsl-client/qsc/tests/tui_product_polish_na0214a.rs`
- desktop sidecar contract:
  - `qsl/qsl-client/qsc-desktop/README.md`
  - `qsl/qsl-client/qsc-desktop/src-tauri/src/qsc.rs`
  - `qsl/qsl-client/qsc/tests/desktop_gui_contract_na0215b.rs`
- attachment send/receive path:
  - `qsl/qsl-client/qsc/tests/attachment_streaming_na0197c.rs`
- relay/header migration:
  - `qsl/qsl-client/qsc/tests/relay_auth_header.rs`
  - `qsl/qsl-client/qsc/tests/route_header_migration_docs_na0195a.rs`
- handshake/session activation:
  - `qsl/qsl-client/qsc/tests/qsp_protocol_gate.rs`
  - `qsl/qsl-client/qsc/tests/handshake_security_closure.rs`
- vault / identity / session-at-rest:
  - `qsl/qsl-client/qsc/tests/identity_secret_at_rest.rs`
  - `qsl/qsl-client/qsc/tests/session_state_at_rest.rs`
- operator/AWS assumptions, read-only only:
  - `qsl/qsl-client/qsc/REMOTE_TWO_CLIENT_AWS_RUNBOOK.md`
  - `qsl/qsl-client/qsc/REMOTE_SOAK_PLAYBOOK.md`

## Successor Proof

Chosen direct successor:
- `NA-0217A — qsc Marker / Output Foundation Extraction`

Why this is the smallest truthful follow-on:
- it removes one coherent, contiguous responsibility cluster from `main.rs`;
- it has high fan-out but lower semantic risk than session, handshake, transport, attachment, or TUI-first moves;
- the current regression surface already freezes marker truth, redaction, and sidecar expectations tightly; and
- it creates the foundation other subsystem lanes can reuse without widening scope or changing protocol/client semantics.

Primary regression set for that successor:
- `qsl/qsl-client/qsc/tests/tui_charter.rs`
- `qsl/qsl-client/qsc/tests/tui_product_polish_na0214a.rs`
- `qsl/qsl-client/qsc/tests/desktop_gui_contract_na0215b.rs`
- `qsl/qsl-client/qsc/tests/route_header_migration_docs_na0195a.rs`
- `qsl/qsl-client/qsc/tests/remote_soak_diag_mapping_na0168.rs`
