Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-03-31

# DOC-QSC-011 — qsc Modularization and File-Size Reduction Plan v0.1.0 DRAFT

Purpose:
- execute `NA-0217` as a planning-only, qbuild-first lane against refreshed live repo truth;
- quantify the current maintainability and auditability concentration in `qsl/qsl-client/qsc/src/main.rs`;
- freeze the modularization invariants and extraction order needed to reduce audit radius without semantic drift; and
- define the smallest truthful direct implementation successor implied by the live code and regression surface.

Non-goals:
- no `qsl/qsl-client/qsc` runtime code move in this item;
- no `qsc-desktop`, `qsl-server`, or `qsl-attachments` write in this item;
- no `.github`, `Cargo.toml`, or `Cargo.lock` change;
- no protocol, wire, crypto, auth, persistence, or state-machine semantic change; and
- no AWS-dependent implementation or validation work.

Result:
- `main.rs` concentration is still the next load-bearing blocker after `NA-0216AA`;
- the truthful extraction shape is one narrow foundation lane followed by several bounded subsystem lanes, not one large refactor PR; and
- the smallest truthful direct successor is `NA-0217A — qsc Marker / Output Foundation Extraction`.

## 1. Authoritative inputs reviewed

Authority ordering used for this plan:
1. refreshed qbuild checkout proven against `origin/main`;
2. live `origin/main` / bare mirror `main`;
3. governance spine in order:
   - `START_HERE.md`
   - `GOALS.md`
   - `AGENTS.md`
   - `PROJECT_CHARTER.md`
   - `NEXT_ACTIONS.md`
   - `DECISIONS.md`
   - `TRACEABILITY.md`;
4. strategic-only historical inputs when locally present; and
5. non-authoritative historical bundles only when live canon already made the same point.

Refreshed qbuild proof used for this plan:
- `qsl-protocol` checkout `HEAD`, `mirror/main`, bare mirror `main`, and `origin/main` all resolved to `b3b6d527ca02`.
- `qsl-server` checkout `HEAD`, `mirror/main`, bare mirror `main`, and `origin/main` all resolved to `0826ffa4d6f3`.
- `qsl-attachments` checkout `HEAD`, `mirror/main`, bare mirror `main`, and `origin/main` all resolved to `e94107ac094d`.
- `qsl-protocol` live queue truth remained `READY_COUNT=1`, with `NA-0217` as the sole live `READY` item.
- `qsl-server` remained at `READY_COUNT=0`.
- `qsl-attachments` remained at `READY_COUNT=0`.
- `gh pr list --state open` for `QuantumShieldLabs/qsl-protocol` returned no active PRs, so no in-scope `NA-0217` conflict was live.

Status-ledger hygiene note:
- `STATUS.md` is present but non-authoritative and stale.
- Its last update still reports `NA-0177` as `READY`.
- The live queue truth for this plan comes only from refreshed `NEXT_ACTIONS.md`.

Strategic-only historical input note:
- No dedicated transition audit / roadmap pack was present as a cohesive local attachment for this lane.
- Historical local files existed and were inventoried only as non-authoritative background:
  - `docs/audit/METADATA_MITIGATIONS_ROADMAP_NA-0137.md`
  - `docs/audit/ONGOING_PQ_RATCHET_ROADMAP_NA-0135.md`
  - `tests/NA-0199_legacy_transition_validation.md`
  - `inputs/phase2/QuantumShield_Phase2_CANONICAL_FROZEN_QSP4.3.2_QSE1.8.2.zip`
  - `inputs/phase3/QuantumShield_Phase3_SUPPORTING_COMPLETE_P3-02_to_P3-30.zip`
- No decision in this document depends on those historical inputs outranking live repo truth.

Current live-code evidence reviewed:
- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/src/cmd/mod.rs`
- `qsl/qsl-client/qsc/src/model/mod.rs`
- `qsl/qsl-client/qsc/src/relay/mod.rs`
- `qsl/qsl-client/qsc/src/store/mod.rs`
- `qsl/qsl-client/qsc/src/tui/mod.rs`
- `qsl/qsl-client/qsc/src/vault/mod.rs`
- `qsl/qsl-client/qsc/src/envelope.rs`
- `qsl/qsl-client/qsc-desktop/README.md`
- `qsl/qsl-client/qsc-desktop/src-tauri/src/main.rs`
- `qsl/qsl-client/qsc-desktop/src-tauri/src/qsc.rs`

Representative regression proofs reviewed:
- `qsl/qsl-client/qsc/tests/tui_charter.rs`
- `qsl/qsl-client/qsc/tests/tui_product_polish_na0214a.rs`
- `qsl/qsl-client/qsc/tests/desktop_gui_contract_na0215b.rs`
- `qsl/qsl-client/qsc/tests/attachment_streaming_na0197c.rs`
- `qsl/qsl-client/qsc/tests/route_header_migration_docs_na0195a.rs`
- `qsl/qsl-client/qsc/tests/qsp_protocol_gate.rs`
- `qsl/qsl-client/qsc/tests/handshake_security_closure.rs`
- `qsl/qsl-client/qsc/tests/identity_secret_at_rest.rs`
- `qsl/qsl-client/qsc/tests/session_state_at_rest.rs`
- `qsl/qsl-client/qsc/tests/message_state_model.rs`
- `qsl/qsl-client/qsc/tests/timeline_store.rs`
- `qsl/qsl-client/qsc/tests/relay_auth_header.rs`
- `qsl/qsl-client/qsc/tests/remote_soak_diag_mapping_na0168.rs`

## 2. Exact problem statement

The direct blocker after `NA-0216AA` is no longer missing adversarial-program definition or cleanup. The live blocker is that `qsl/qsl-client/qsc/src/main.rs` still carries most runtime ownership for:
- CLI bootstrap and command dispatch;
- headless and interactive TUI behavior;
- marker formatting, redaction, and deterministic diagnostics;
- encrypted session-at-rest handling and protocol activation;
- attachment and file-transfer logic;
- handshake and identity logic;
- contacts, trust, routing, and timeline state; and
- relay send/receive plus local relay helpers.

That concentration raises two decision-grade problems:
1. review and audit radius remain larger than the current merged test surface can explain quickly; and
2. future extractions risk semantic drift unless the order is constrained by the fragile surfaces already frozen in tests and docs.

`NA-0158` already performed a no-behavior-change split into `cmd`, `model`, `relay`, `store`, `tui`, and `vault`, but live repo proof shows those modules still mostly hold type definitions, constants, or isolated helpers rather than the dominant behavioral ownership. The problem is therefore not "whether modularization is needed"; it is how to continue it truthfully without widening into a giant refactor or semantic lane.

## 3. Concentration Summary

### A. qsc source concentration summary

Current live `qsl/qsl-client/qsc/src` total:
- `24,790` Rust LOC

| File | LOC | Share of `qsc/src` | Current role |
| --- | ---: | ---: | --- |
| `main.rs` | 21,627 | 87.24% | Dominant runtime owner |
| `vault/mod.rs` | 1,138 | 4.59% | Vault envelope/runtime |
| `cmd/mod.rs` | 776 | 3.13% | CLI surface/types |
| `store/mod.rs` | 240 | 0.97% | Store records/constants |
| `adversarial/route.rs` | 196 | 0.79% | Parser helper |
| `envelope.rs` | 193 | 0.78% | Envelope helper |
| `adversarial/payload.rs` | 183 | 0.74% | Payload parser helper |
| `tui/mod.rs` | 160 | 0.65% | TUI enums/constants |
| `relay/mod.rs` | 115 | 0.46% | Relay types/helpers |
| `model/mod.rs` | 86 | 0.35% | Error/lock primitives |
| `adversarial/vault_format.rs` | 71 | 0.29% | Vault parser helper |

Implication:
- the problem is still overwhelmingly concentrated in `main.rs`;
- the largest extracted modules are useful, but they do not yet own the high-risk behavior clusters; and
- future work should move cohesive behavior out of `main.rs`, not just add more types around it.

### B. main.rs responsibility-cluster inventory

| Cluster | Current line span | Approx. LOC | What still lives there |
| --- | --- | ---: | --- |
| CLI bootstrap and command dispatch | `1-679` | 679 | startup unlock path, command routing, doctor/config/util entrypoints |
| TUI shell and rendering | `680-10329` | 9,650 | relay probe, headless script engine, key handling, command parsing, draw/render, TUI state |
| Status, session-state, and delivery metadata | `10330-12321` | 1,992 | route-token hashing, protocol status, session-at-rest load/store/migrate, receipt policy, message-state markers |
| Attachment, file-transfer, and receipt pipeline | `12322-14776` | 2,455 | attachment journal, staging, service upload/commit, file-transfer manifests, receipt batching |
| QSP pack/unpack, handshake, and identity | `14777-15919` | 1,143 | pack/unpack, handshake wire/messages, identity secret/public records, pin helpers |
| Contacts, trust, routing, and timeline | `15920-18614` | 2,695 | contact/device stores, trust remediation, primary routing, timeline persistence and state transitions |
| Send/receive and relay transport | `18615-20605` | 1,991 | send/receive execution, outbox replay, relay inbox HTTP client/server, transport policy |
| Output, store-safety, and utility foundations | `20606-21627` | 1,022 | marker formatting/routing, redaction/logging, atomic writes, locking, perms/symlink checks, terminal sanitizer |

Immediate reading:
- the TUI is still the largest single resident cluster, but it is downstream of many other shared behaviors;
- several medium-sized clusters own higher semantic risk than their LOC would suggest; and
- the smallest truthful first move is therefore not "largest cluster first", but "lowest-semantic-risk high-fan-out foundation first".

## 4. Fragile-Zone Regression Map

### C. Fragile-zone regression inventory

| Fragile zone | Representative live proof | Why it is fragile | Seam-order consequence |
| --- | --- | --- | --- |
| Marker formatting and deterministic diagnostics | `tui_charter.rs`, `tui_product_polish_na0214a.rs`, `route_header_migration_docs_na0195a.rs`, `remote_soak_diag_mapping_na0168.rs` | event names, redaction, and deterministic marker text are treated as product contract and evidence contract at the same time | move marker/output logic before higher-semantic subsystem logic so later extractions reuse one stable contract |
| TUI rendering and scripted TUI tests | `tui_charter.rs`, `tui_product_polish_na0214a.rs`, `tui_fixed_polling.rs`, `tui_relay_drop_reorder.rs` | headless scripting, render markers, timing tokens, and layout/state behavior are tightly coupled | postpone major TUI decomposition until shared business logic is extracted first |
| qsc-desktop sidecar behavior/contract | `qsl/qsl-client/qsc-desktop/README.md`, `src-tauri/src/qsc.rs`, `desktop_gui_contract_na0215b.rs` | desktop relies on stable markers, passphrase-env handoff, and truthful protocol readiness parsing | preserve CLI/marker contract and sidecar unlock semantics across all early seams |
| Attachment send/receive path | `attachment_streaming_na0197c.rs`, `message_state_model.rs`, `adversarial_properties.rs` | file/attachment path crosses qsc state, timeline, relay, and qsl-attachments contract boundaries | keep attachment/file work later than the first foundations and later than session/routing foundations |
| Relay/header migration behavior | `relay_auth_header.rs`, `route_header_migration_docs_na0195a.rs` | canonical header carriage and secret-safe output are frozen already; drift is security-significant | do not extract relay transport before marker/output and routing helpers are stable |
| Handshake/session activation logic | `qsp_protocol_gate.rs`, `handshake_security_closure.rs`, `session_state_at_rest.rs` | protocol active/inactive truth, transcript rejection, and encrypted-session migration are fail-closed semantics | session-status and identity foundations should come before any broader send/receive or TUI move |
| Vault / identity / session-at-rest coverage | `identity_secret_at_rest.rs`, `session_state_at_rest.rs`, `vault_attempt_limit.rs` | migrations, vault availability, and no-plaintext-at-rest guarantees are easy to regress with structural churn | preserve vault and at-rest helpers as explicit invariants when extracting any identity/session code |
| Operator flows with AWS/runbook assumptions | `REMOTE_TWO_CLIENT_AWS_RUNBOOK.md`, `REMOTE_SOAK_PLAYBOOK.md`, `tui_product_polish_na0214a.rs`, `remote_soak_diag_mapping_na0168.rs` | docs/tests freeze qbuild-first as the front door and classify AWS as compatibility-only | no early seam may weaken qbuild-first marker truth or re-normalize AWS-dependent assumptions |

## 5. Prioritized Seam Inventory

### D. prioritized seam inventory

| Order | Candidate seam | Proposed module boundary | Likely functions / types to move first | Type | Coupling / blast-radius risk | Invariant risk | Critical regression tests that must remain green | Why earlier or later | Immediate audit-radius reduction |
| ---: | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | Marker / output foundation | `qsl/qsl-client/qsc/src/output/mod.rs` | `emit_marker`, `format_marker_line`, `set_marker_routing`, `marker_format`, `redact_*`, `should_redact_value`, `log_marker`, `qsc_sanitize_terminal_text`, `qsc_mark` | Foundation seam | Medium fan-out, low semantic ownership | High marker-truth risk, low protocol risk | `tui_charter.rs`, `tui_product_polish_na0214a.rs`, `desktop_gui_contract_na0215b.rs`, `route_header_migration_docs_na0195a.rs`, `remote_soak_diag_mapping_na0168.rs` | Earliest: smallest coherent high-fan-out seam and the test surface already freezes it tightly | Removes cross-cutting output truth from the monolith so later diffs stop interleaving logic changes with marker noise |
| 2 | Filesystem / config / locking foundation | `qsl/qsl-client/qsc/src/fs_store/mod.rs` | `config_dir`, `ensure_dir_secure`, `write_atomic`, `lock_store_*`, `enforce_safe_parents`, perms helpers, `probe_dir_writable` | Foundation seam | Medium | High fail-closed store-safety risk | `desktop_gui_contract_na0215b.rs`, `identity_secret_at_rest.rs`, `session_state_at_rest.rs`, `timeline_store.rs` | Early, but after output: many later seams depend on safe IO, yet it has slightly more persistence-risk than output | Separates generic storage-safety rules from feature logic and clarifies which later moves are structural vs semantic |
| 3 | Protocol status + session-at-rest foundation | `qsl/qsl-client/qsc/src/protocol_state/mod.rs` | `qsp_status_tuple`, `qsp_session_*`, `protocol_active_or_reason_*`, status record helpers | Foundation seam | Medium | High activation / migration risk | `qsp_protocol_gate.rs`, `session_state_at_rest.rs`, `handshake_security_closure.rs` | Earlier than transport and TUI because it underpins both | Pulls encrypted-session ownership and ACTIVE/INACTIVE truth into one bounded unit |
| 4 | Identity record + pin helpers | `qsl/qsl-client/qsc/src/identity/mod.rs` | identity public/secret record helpers, fingerprint helpers, pin reads, legacy migration helpers | Foundation seam | Medium | High at-rest / pinning risk | `identity_secret_at_rest.rs`, `handshake_security_closure.rs`, `desktop_gui_contract_na0215b.rs` | Earlier than handshake execution because identity storage is the lower-semantic slice | Removes mixed identity persistence logic from the protocol/contacts clusters |
| 5 | Contacts / trust / routing | `qsl/qsl-client/qsc/src/contacts/mod.rs` | contact stores, device normalization, trust remediation, routing-target resolution | Subsystem seam | Medium | High send-gating and routing risk | `relay_auth_header.rs`, `message_state_model.rs`, `desktop_gui_contract_na0215b.rs` | After identity/session foundations because it depends on both | Shrinks a broad operator-state cluster and makes routing logic reviewable on its own |
| 6 | Timeline / delivery state | `qsl/qsl-client/qsc/src/timeline/mod.rs` | timeline store/load/save, state-transition helpers, emitters, confirmation apply helpers | Subsystem seam | Medium | High honest-delivery risk | `timeline_store.rs`, `message_state_model.rs`, `attachment_streaming_na0197c.rs` | After contacts/routing because delivery state stores device-target decisions | Separates honest-delivery ownership from transport and attachments |
| 7 | Relay transport send/receive | `qsl/qsl-client/qsc/src/transport/mod.rs` | `relay_inbox_push`, `relay_inbox_pull`, auth-token resolution, local relay HTTP parsing, send/receive execution wrappers | Subsystem seam | High | High route-token/header and retry/outbox risk | `relay_auth_header.rs`, `route_header_migration_docs_na0195a.rs`, `remote_soak_diag_mapping_na0168.rs`, `ratchet_durability_na0155.rs` | Later than foundations because it depends on output, session, routing, and timeline helpers | Collapses transport-specific review to one area instead of forcing reviewers through the whole monolith |
| 8 | Attachment / file-transfer pipeline | `qsl/qsl-client/qsc/src/attachments/mod.rs` and/or `file_xfer/mod.rs` | attachment journal/staging/service API, file manifests/chunks, confirmation handling, receipt linkage | Subsystem seam | High | High attachment-path / delivery-state risk | `attachment_streaming_na0197c.rs`, `message_state_model.rs`, `adversarial_properties.rs` | Later because it crosses transport, timeline, and qsl-attachments contract boundaries | Removes one of the last multi-thousand-line semantic clusters once the foundations beneath it are isolated |
| 9 | Handshake execution | `qsl/qsl-client/qsc/src/handshake/mod.rs` | handshake encode/decode, pending state, init/poll execution, transcript checks | Subsystem seam | High | Very high protocol / identity-binding risk | `handshake_security_closure.rs`, `qsp_protocol_gate.rs` | Later than identity/session foundation because execution code is higher-semantic-risk | Makes protocol-establish review distinct from unrelated client-shell logic |
| 10 | TUI decomposition | `qsl/qsl-client/qsc/src/tui/` subordinate slices such as `render`, `script`, `controller` | draw helpers, headless script parser, key handlers, state transitions | Subsystem seam | High | High UI-contract risk | `tui_charter.rs`, `tui_product_polish_na0214a.rs`, `tui_fixed_polling.rs`, `tui_relay_drop_reorder.rs` | Deliberately late: the TUI currently consumes most shared logic, so early extraction would mostly move coupling rather than reduce it | Once shared logic is elsewhere, the TUI can be split into true UI slices instead of a second monolith |

## 6. Extraction Order / PR Slicing Plan

Recommended order:
1. direct successor: marker / output foundation extraction;
2. follow-on foundation: filesystem / config / locking helpers;
3. follow-on foundation: protocol status + session-at-rest helpers;
4. follow-on foundation: identity record + pin helpers;
5. first subsystem wave: contacts / trust / routing;
6. second subsystem wave: timeline / delivery state;
7. third subsystem wave: relay transport send/receive helpers;
8. fourth subsystem wave: attachment / file-transfer pipeline;
9. fifth subsystem wave: handshake execution; and
10. late consumer wave: TUI controller/headless and render/layout decomposition after the shared logic is already gone from `main.rs`.

Why this order is truthful:
- it starts with the smallest low-semantic-risk high-fan-out seam;
- it then isolates the shared fail-closed foundations before touching broader feature clusters; and
- it intentionally postpones the TUI and attachment-heavy clusters until their underlying business logic is already module-owned elsewhere.

What this plan explicitly rejects:
- no single "split `main.rs`" mega-PR;
- no first move that touches protocol or attachment semantics just because those regions are large; and
- no TUI-first split that would merely relocate the current coupling instead of reducing it.

## 7. Invariant + Regression-Test Matrix Per Seam

| Seam | Must preserve | Minimum regression set |
| --- | --- | --- |
| Marker / output foundation | event names, redaction rules, plain/jsonl format, desktop sidecar marker parsing, no secret-like output | `tui_charter.rs`, `tui_product_polish_na0214a.rs`, `desktop_gui_contract_na0215b.rs`, `remote_soak_diag_mapping_na0168.rs` |
| Filesystem / config / locking foundation | symlink-safe paths, 0700/0600 enforcement, atomic writes, lock behavior, qbuild/local operator front door | `desktop_gui_contract_na0215b.rs`, `identity_secret_at_rest.rs`, `session_state_at_rest.rs`, `timeline_store.rs` |
| Protocol status + session-at-rest | `ACTIVE/INACTIVE` truth, encrypted-session migration, no plaintext state, no seed-fallback normalization | `qsp_protocol_gate.rs`, `session_state_at_rest.rs` |
| Identity helpers | vault-backed secret storage, legacy import no-mutation on failure, pin/fingerprint stability | `identity_secret_at_rest.rs`, `handshake_security_closure.rs` |
| Contacts / trust / routing | primary-device routing, trust remediation, blocked-peer behavior, route-token normalization | `relay_auth_header.rs`, `message_state_model.rs`, `desktop_gui_contract_na0215b.rs` |
| Timeline / delivery | `accepted_by_relay != peer_confirmed`, no mutation on reject, device-target confirmation gating | `timeline_store.rs`, `message_state_model.rs`, `attachment_streaming_na0197c.rs` |
| Relay transport send/receive | header-carried route tokens, outbox replay semantics, bounded receive behavior, AWS-safe diagnostics | `relay_auth_header.rs`, `route_header_migration_docs_na0195a.rs`, `remote_soak_diag_mapping_na0168.rs` |
| Attachment / file-transfer | post-`w0` receive/send posture, qsl-attachments contract, honest file/attachment confirmation state | `attachment_streaming_na0197c.rs`, `message_state_model.rs` |
| Handshake execution | transcript binding, pinned-identity mismatch reject, no session mutation on tamper | `handshake_security_closure.rs`, `qsp_protocol_gate.rs` |
| TUI decomposition | scripted determinism, stable render markers, relay-test behavior, no implicit actions | `tui_charter.rs`, `tui_product_polish_na0214a.rs`, `tui_fixed_polling.rs`, `tui_relay_drop_reorder.rs` |

## 8. Explicit Non-Regression Rules For Future Modularization

Future modularization lanes must preserve all of the following unless a later explicit NA changes them:
- one `qsc` binary and the current CLI contract;
- current marker names, marker shapes, marker routing, and redaction behavior;
- current qsc-desktop sidecar contract, including child-scoped passphrase handling and peer-specific readiness parsing;
- current route-token/header discipline and the rule that canonical URLs stay secret-free;
- current honest-delivery semantics, especially `accepted_by_relay` versus `peer_confirmed`;
- current `protocol_inactive` / `ACTIVE` / `INACTIVE` fail-closed posture;
- current vault, identity, contacts, timeline, and session-at-rest file/secret formats unless the lane explicitly owns migrations;
- current qbuild/local-first operator posture, with AWS/remote artifacts remaining compatibility-only evidence where already documented that way;
- qsl-server transport-only posture; and
- qsl-attachments opaque ciphertext-only posture.

## 9. First Implementation Lane

Title:
- `NA-0217A — qsc Marker / Output Foundation Extraction`

Scope:
- `qsl/qsl-client/qsc/src/main.rs`
- new `qsl/qsl-client/qsc/src/output/**`
- `qsl/qsl-client/qsc/src/lib.rs` only if a narrow re-export is needed
- `qsl/qsl-client/qsc/tests/**` only for mechanical coverage alignment if the extracted module needs direct test visibility

Forbidden scope:
- no CLI flag changes
- no event-name or marker-shape changes
- no protocol, wire, relay, attachment, vault, identity, or TUI behavior redesign
- no `qsc-desktop` runtime change
- no docs/runbook truth rewrite beyond what a mechanical module path move requires

What is being protected:
- deterministic marker output for CLI, TUI, and diagnostics;
- redaction and log-safety rules;
- desktop sidecar parsing expectations;
- qbuild/AWS-safe operator evidence wording; and
- later modularization lanes' ability to share one stable output contract instead of re-implementing it.

Invariant(s):
- `QSC_MARK/1` output remains byte-for-byte compatible for the same inputs when `QSC_MARK_FORMAT=plain`.
- JSONL marker shape remains unchanged when `QSC_MARK_FORMAT=jsonl`.
- Secret-like values remain redacted under the same keys and heuristics.
- `MarkerRouting::Stdout` versus in-app queue behavior remains unchanged.
- No existing command, exit code, or protocol state transition changes as a side effect of the extraction.

Deliverables:
1. move marker formatting, marker routing, redaction, log-writing, and terminal-sanitizer helpers into a dedicated module;
2. keep call sites behavior-identical, even if imports become cleaner;
3. add or retain the smallest test visibility surface needed so regression tests still target the same product behavior; and
4. document the extracted module as a foundation seam, not a product-surface change.

Acceptance criteria:
1. no behavior drift in marker or redaction output;
2. the representative marker-sensitive suites remain green:
   - `tui_charter`
   - `tui_product_polish_na0214a`
   - `desktop_gui_contract_na0215b`
   - `route_header_migration_docs_na0195a`
   - `remote_soak_diag_mapping_na0168`;
3. `qsl/qsl-client/qsc/src/main.rs` loses the marker/output foundation cluster in one coherent move; and
4. no route-token/header, sidecar, or protocol-activation semantic diff is introduced.

Why this is smaller and truer than the obvious alternatives:
- it removes a coherent responsibility cluster that is already bounded in one contiguous region of `main.rs`;
- it is smaller than a TUI-first move and far lower-semantic-risk than starting with session, handshake, or attachment logic;
- it reduces audit radius immediately because nearly every later seam depends on these helpers; and
- the current regression surface already treats marker/redaction behavior as contract, so the lane can prove "no drift" without inventing new semantics.

## 10. Rejected Alternatives

1. Start with the TUI split.
   - Rejected because the TUI still spans `9,650` lines and currently mixes headless scripting, rendering, relay probes, command handling, and view-state consumers of many other subsystems. As a first move it would relocate coupling rather than reduce it.

2. Start with the attachment / file-transfer pipeline.
   - Rejected because that region is large but semantically hotter: it crosses send/receive truth, timeline state, post-`w0` migration posture, and the qsl-attachments contract. It is not the smallest truthful first cut.

3. Start with handshake or session-state execution.
   - Rejected because those seams are smaller than the TUI but still carry higher semantic risk than marker/output extraction. They should follow the lower-risk foundations that make later diffs clearer and safer.

4. Attempt one giant multi-module refactor.
   - Rejected because it would create the widest review surface, the weakest no-drift proof, and the least honest queue discipline.

## 11. References

- `START_HERE.md`
- `GOALS.md`
- `AGENTS.md`
- `PROJECT_CHARTER.md`
- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/src/cmd/mod.rs`
- `qsl/qsl-client/qsc/src/model/mod.rs`
- `qsl/qsl-client/qsc/src/relay/mod.rs`
- `qsl/qsl-client/qsc/src/store/mod.rs`
- `qsl/qsl-client/qsc/src/tui/mod.rs`
- `qsl/qsl-client/qsc/src/vault/mod.rs`
- `qsl/qsl-client/qsc/src/envelope.rs`
- `qsl/qsl-client/qsc-desktop/README.md`
- `qsl/qsl-client/qsc-desktop/src-tauri/src/main.rs`
- `qsl/qsl-client/qsc-desktop/src-tauri/src/qsc.rs`
- `qsl/qsl-client/qsc/tests/tui_charter.rs`
- `qsl/qsl-client/qsc/tests/tui_product_polish_na0214a.rs`
- `qsl/qsl-client/qsc/tests/desktop_gui_contract_na0215b.rs`
- `qsl/qsl-client/qsc/tests/attachment_streaming_na0197c.rs`
- `qsl/qsl-client/qsc/tests/route_header_migration_docs_na0195a.rs`
- `qsl/qsl-client/qsc/tests/qsp_protocol_gate.rs`
- `qsl/qsl-client/qsc/tests/handshake_security_closure.rs`
- `qsl/qsl-client/qsc/tests/identity_secret_at_rest.rs`
- `qsl/qsl-client/qsc/tests/session_state_at_rest.rs`
- `qsl/qsl-client/qsc/tests/message_state_model.rs`
- `qsl/qsl-client/qsc/tests/timeline_store.rs`
- `qsl/qsl-client/qsc/tests/relay_auth_header.rs`
- `qsl/qsl-client/qsc/tests/remote_soak_diag_mapping_na0168.rs`
