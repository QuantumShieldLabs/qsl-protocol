Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-07

# DOC-QSC-011 — qsc Modularization and File-Size Reduction Plan v0.1.0 DRAFT

Purpose:
- execute `NA-0224` as a planning-refresh, docs/governance-only lane against refreshed merged `main`;
- replace the stale pre-`NA-0217A` concentration picture with current merged metrics;
- freeze the live qsc seam ownership map after the completed `NA-0217A` through `NA-0217J` extraction wave plus the later handshake audit/remediation/adversarial batch; and
- determine truthfully whether another bounded extraction lane still dominates maintainability and audit-radius risk.

Non-goals:
- no qsc, qsc-desktop, qsl-server, or qsl-attachments runtime code change in this item;
- no `.github`, website/public-runtime, `Cargo.toml`, or `Cargo.lock` change;
- no protocol, wire, crypto, auth, persistence, or state-machine semantic change;
- no `NEXT_ACTIONS.md` closeout or successor promotion; and
- no attempt to restart the completed `NA-0217A` through `NA-0217J` wave as if those seams were still pending.

Result:
- the original `NA-0217` plan baseline is now stale: the extraction wave it proposed is already merged, and `qsl/qsl-client/qsc/src/main.rs` is no longer the dominant concentration point;
- current concentration has shifted to `qsl/qsl-client/qsc/src/tui/controller.rs` (`9,417 / 25,025` LOC, `37.63%`), while `main.rs` is now `2,933 / 25,025` LOC (`11.72%`);
- further extraction is still justified, but it is now a TUI-internal decomposition problem rather than another `main.rs`-centric foundation wave; and
- the next truthful bounded lane is a `tui/**` controller state / command-flow decomposition that preserves the current CLI/TUI, sidecar, marker, route-token/header, and honest-delivery contracts.

## 1. Authoritative Inputs Reviewed

Authority ordering used for this refresh:
1. refreshed qbuild checkout proven against configured remotes and `origin/main`;
2. live `origin/main` plus the local bare mirror `main`;
3. governance spine in order:
   - `START_HERE.md`
   - `GOALS.md`
   - `AGENTS.md`
   - `PROJECT_CHARTER.md`
   - `NEXT_ACTIONS.md`
   - `DECISIONS.md`
   - `TRACEABILITY.md`;
4. current `docs/design/**` on merged main; and
5. current merged `qsl/qsl-client/qsc/src/**` inventory and metrics.

Refreshed qbuild proof used for this refresh:
- `qsl-protocol` checkout `HEAD`, `mirror/main`, bare mirror `main`, and `origin/main` all resolve to `13a07831d71c`.
- `qsl-server` checkout `HEAD`, `mirror/main`, bare mirror `main`, and `origin/main` all resolve to `0826ffa4d6f3`.
- `qsl-attachments` checkout `HEAD`, `mirror/main`, bare mirror `main`, and `origin/main` all resolve to `e94107ac094d`.
- `qsl-protocol` queue truth remains `READY_COUNT=1`, with `NA-0224 — qsc Modularization / File-Size Reduction Plan Refresh` as the sole live `READY` item.
- `qsl-server` remains at `READY_COUNT=0`.
- `qsl-attachments` remains at `READY_COUNT=0`.

Status-ledger hygiene note:
- `STATUS.md` remains non-authoritative and stale.
- Its last checked-in update still reports `NA-0177` as `READY`.
- All queue truth in this refresh comes from refreshed `NEXT_ACTIONS.md`.

Current live-code evidence reviewed:
- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/src/output/mod.rs`
- `qsl/qsl-client/qsc/src/fs_store/mod.rs`
- `qsl/qsl-client/qsc/src/protocol_state/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/src/contacts/mod.rs`
- `qsl/qsl-client/qsc/src/timeline/mod.rs`
- `qsl/qsl-client/qsc/src/transport/mod.rs`
- `qsl/qsl-client/qsc/src/attachments/mod.rs`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/tui/mod.rs`
- `qsl/qsl-client/qsc/src/tui/controller.rs`
- `qsl/qsl-client/qsc/src/tui/render.rs`
- `qsl/qsl-client/qsc/src/tui/script.rs`
- `qsl/qsl-client/qsc-desktop/README.md`
- `qsl/qsl-client/qsc-desktop/src-tauri/src/qsc.rs`

Representative regression proofs reviewed:
- `qsl/qsl-client/qsc/tests/tui_contract_na0217j.rs`
- `qsl/qsl-client/qsc/tests/tui_charter.rs`
- `qsl/qsl-client/qsc/tests/tui_product_polish_na0214a.rs`
- `qsl/qsl-client/qsc/tests/tui_fixed_polling.rs`
- `qsl/qsl-client/qsc/tests/tui_lock_unlock_polish.rs`
- `qsl/qsl-client/qsc/tests/tui_locked_cmd_init_ux.rs`
- `qsl/qsl-client/qsc/tests/tui_main_scroll_focus.rs`
- `qsl/qsl-client/qsc/tests/tui_messages_ux_mvp.rs`
- `qsl/qsl-client/qsc/tests/tui_contacts_option1.rs`
- `qsl/qsl-client/qsc/tests/tui_relay_drop_reorder.rs`
- `qsl/qsl-client/qsc/tests/desktop_gui_contract_na0215b.rs`
- `qsl/qsl-client/qsc/tests/route_header_migration_docs_na0195a.rs`
- `qsl/qsl-client/qsc/tests/message_state_model.rs`
- `qsl/qsl-client/qsc/tests/timeline_delivery_contract_na0217f.rs`
- `qsl/qsl-client/qsc/tests/transport_contract_na0217g.rs`
- `qsl/qsl-client/qsc/tests/attachments_contract_na0217h.rs`
- `qsl/qsl-client/qsc/tests/handshake_contract_na0217i.rs`

## 2. Baseline Drift From The Original NA-0217 Plan

The original `NA-0217` version of this document captured truthful live state on `2026-03-31`, but it is no longer current merged-main truth:

- it measured `main.rs` at `21,627 / 24,790` LOC (`87.24%`) and named `NA-0217A — qsc Marker / Output Foundation Extraction` as the direct successor;
- merged main now already carries `output`, `fs_store`, `protocol_state`, `identity`, `contacts`, `timeline`, `transport`, `attachments`, `handshake`, and `tui/**` ownership from `NA-0217A` through `NA-0217J`; and
- the remaining maintainability concentration is no longer the original `main.rs` monolith that drove the first wave.

This refresh therefore keeps the authoritative plan path but replaces the stale concentration premise, stale seam order, and stale "next successor" wording with current merged-main truth.

## 3. Current Concentration Summary

### A. Live qsc/src file concentration

Current live `qsl/qsl-client/qsc/src/**` total:
- `25,025` LOC across all checked-in source files under `qsc/src`

| File | LOC | Share of `qsc/src` | Current role |
| --- | ---: | ---: | --- |
| `tui/controller.rs` | 9,417 | 37.63% | Dominant TUI controller, state, and command-flow owner |
| `main.rs` | 2,933 | 11.72% | CLI bootstrap plus remaining relay/meta/util and receipt/wire helpers |
| `attachments/mod.rs` | 2,176 | 8.70% | Attachment / file-transfer pipeline |
| `transport/mod.rs` | 1,741 | 6.96% | Relay send/receive and outbox transport helpers |
| `contacts/mod.rs` | 1,713 | 6.85% | Contacts, trust, and routing ownership |
| `handshake/mod.rs` | 1,324 | 5.29% | Handshake execution and pending-state ownership |
| `vault/mod.rs` | 1,138 | 4.55% | Vault envelope/runtime helpers |
| `cmd/mod.rs` | 776 | 3.10% | CLI surface/types |
| `timeline/mod.rs` | 712 | 2.85% | Timeline and delivery-state ownership |
| `protocol_state/mod.rs` | 483 | 1.93% | Protocol activation/status and session-at-rest ownership |

Immediate reading:
- the original `main.rs` concentration blocker is gone;
- the largest remaining single-file concentration is now `tui/controller.rs`; and
- the next maintainability problem is no longer "continue extracting subsystems from `main.rs`" but "finish decomposing the oversized consumer/controller shell that now sits in `tui/**`."

### B. Current subsystem ownership map

| Subsystem | Current owner(s) | LOC | Current responsibilities | Representative coupled proofs |
| --- | --- | ---: | --- | --- |
| Output | `qsl/qsl-client/qsc/src/output/mod.rs` | 318 | marker format/routing, redaction, log-safe output, terminal sanitization | `output_marker_contract_na0217a.rs`; `desktop_gui_contract_na0215b.rs`; `route_header_migration_docs_na0195a.rs` |
| Filesystem / config / locking | `qsl/qsl-client/qsc/src/fs_store/mod.rs` | 374 | config path selection, parent-safety, secure perms, atomic writes, lock helpers | `fs_store_contract_na0217b.rs`; `identity_secret_at_rest.rs`; `session_state_at_rest.rs` |
| Protocol status / session-at-rest | `qsl/qsl-client/qsc/src/protocol_state/mod.rs` | 483 | `ACTIVE` / `INACTIVE` truth, qsp status tuple, encrypted session load/store/migration | `protocol_state_contract_na0217c.rs`; `qsp_protocol_gate.rs`; `session_state_at_rest.rs` |
| Identity | `qsl/qsl-client/qsc/src/identity/mod.rs` | 409 | identity public/secret records, fingerprint/pin helpers, legacy migration | `identity_foundation_contract_na0217d.rs`; `identity_secret_at_rest.rs`; `handshake_security_closure.rs` |
| Contacts / trust / routing | `qsl/qsl-client/qsc/src/contacts/mod.rs` | 1,713 | contact/device stores, trust remediation, route-token normalization, routing-target resolution | `relay_auth_header.rs`; `message_state_model.rs`; `desktop_gui_contract_na0215b.rs` |
| Timeline / delivery state | `qsl/qsl-client/qsc/src/timeline/mod.rs` | 712 | timeline persistence, delivery semantics, confirmation apply helpers | `timeline_delivery_contract_na0217f.rs`; `timeline_store.rs`; `message_state_model.rs` |
| Relay transport | `qsl/qsl-client/qsc/src/transport/mod.rs` | 1,741 | send/receive execution, outbox replay, local relay HTTP parsing, retry policy | `transport_contract_na0217g.rs`; `relay_auth_header.rs`; `route_header_migration_docs_na0195a.rs` |
| Attachments | `qsl/qsl-client/qsc/src/attachments/mod.rs` | 2,176 | attachment journal/staging, descriptor/file-confirm parsing, service upload/commit helpers | `attachments_contract_na0217h.rs`; `attachment_streaming_na0197c.rs`; `message_state_model.rs` |
| Handshake | `qsl/qsl-client/qsc/src/handshake/mod.rs` | 1,324 | handshake encode/decode, pending-state transitions, init/poll execution, transcript checks | `handshake_contract_na0217i.rs`; `handshake_security_closure.rs`; `qsp_protocol_gate.rs` |
| TUI | `qsl/qsl-client/qsc/src/tui/mod.rs`; `qsl/qsl-client/qsc/src/tui/controller.rs`; `qsl/qsl-client/qsc/src/tui/render.rs`; `qsl/qsl-client/qsc/src/tui/script.rs` | 9,839 | headless scripting, interactive loop, key/focus handling, locked/unlocked command dispatch, render/layout, TUI state | `tui_contract_na0217j.rs`; `tui_charter.rs`; `tui_product_polish_na0214a.rs`; `tui_fixed_polling.rs`; `desktop_gui_contract_na0215b.rs` |
| Remaining ownership in `main.rs` | `qsl/qsl-client/qsc/src/main.rs` | 2,933 | CLI bootstrap, relay/meta subcommands, doctor export, receipt policy, QSP pack/unpack, vault security counters, misc utilities | `cli.rs`; `diagnostics.rs`; `receive_e2e.rs`; `relay_auth_header.rs` |

## 4. Current Dominant Blocker Assessment

### C. Why the blocker shifted from `main.rs` to `tui/controller.rs`

The remaining high-concentration file is not a protocol-core seam. It is a controller shell that still mixes several large responsibilities:

| `tui/controller.rs` subcluster | Approx. line span | Approx. LOC | Why it still matters |
| --- | --- | ---: | --- |
| unlocked command dispatch | `1392-4059` | 2,668 | slash-command handling and user-visible side effects still live in one block |
| `impl TuiState` | `4862-8574` | 3,713 | view-state orchestration and panel behavior still concentrate here |
| render/main-panel helpers | `4394-4861`, `8721-9397` | 1,145 | inspector rendering remains coupled to controller state |
| startup, headless, key, and lock flow | `143-1391` | 1,249 | startup preflight, headless parity, locked/unlocked shell behavior, and key handling still share one file |
| message send/receive mediation | `4060-4393` | 334 | honest-delivery and readiness-sensitive TUI flows still sit in the controller |

Current decision:
- another bounded extraction lane is still justified;
- the dominant blocker is maintainability concentration inside `tui/controller.rs`, not inside `main.rs`; and
- the refreshed plan should name one bounded TUI-internal lane instead of pretending the completed `NA-0217` foundation/subsystem order is still future work.

## 5. Next Bounded Extraction Lane

Title:
- `qsc TUI Controller State / Command-Flow Decomposition`

Why this lane is the next truthful move:
- it targets the largest remaining single-file concentration on merged `main`;
- it decomposes controller-only responsibilities without re-opening already-extracted subsystem seams such as `contacts`, `transport`, `attachments`, or `handshake`; and
- it keeps the next move bounded to the consumer shell that now fans into the extracted modules, which is a smaller and truer follow-on than inventing another cross-module foundation wave.

Likely future scope for that lane:
- `qsl/qsl-client/qsc/src/tui/controller.rs`
- new `qsl/qsl-client/qsc/src/tui/state.rs`
- new `qsl/qsl-client/qsc/src/tui/commands.rs`
- `qsl/qsl-client/qsc/src/tui/mod.rs`
- `qsl/qsl-client/qsc/tests/**` only for narrow regression alignment if direct module visibility changes are needed

Protected invariants:
- one `qsc` binary and the current CLI/TUI contract;
- current qsc-desktop sidecar contract, including marker parsing and passphrase-scoped sidecar expectations;
- current command names, marker shapes, command-result routing, and honest-delivery semantics;
- current headless script tokens, fixed polling cadence, lock/unlock behavior, focus/inspector behavior, and deterministic render markers;
- current route-token/header discipline and secret-free canonical URLs;
- qsl-server remains transport-only; and
- qsl-attachments remains opaque ciphertext-only.

Minimum regression surfaces that must remain green:
- `qsl/qsl-client/qsc/tests/tui_contract_na0217j.rs`
- `qsl/qsl-client/qsc/tests/tui_charter.rs`
- `qsl/qsl-client/qsc/tests/tui_product_polish_na0214a.rs`
- `qsl/qsl-client/qsc/tests/tui_fixed_polling.rs`
- `qsl/qsl-client/qsc/tests/tui_lock_unlock_polish.rs`
- `qsl/qsl-client/qsc/tests/tui_locked_cmd_init_ux.rs`
- `qsl/qsl-client/qsc/tests/tui_main_scroll_focus.rs`
- `qsl/qsl-client/qsc/tests/tui_messages_ux_mvp.rs`
- `qsl/qsl-client/qsc/tests/tui_contacts_option1.rs`
- `qsl/qsl-client/qsc/tests/tui_relay_drop_reorder.rs`
- `qsl/qsl-client/qsc/tests/desktop_gui_contract_na0215b.rs`

Explicit no-drift concerns:
- do not move business logic back out of already-extracted subsystem owners;
- do not change marker names, `QSC_MARK/1` shapes, or headless script syntax while decomposing the controller;
- do not alter command semantics, `protocol_inactive` gating, or honest-delivery state transitions as a side effect of TUI-only structure work; and
- do not fork a second interpretation of sidecar-facing readiness or marker truth inside new TUI files.

## 6. Alternatives Rejected By This Refresh

1. Reaffirm `main.rs` as the dominant blocker and keep the old successor wording.
   - Rejected because that is no longer truthful against refreshed merged main.

2. Declare that no further extraction lane is justified.
   - Rejected because `tui/controller.rs` still holds `37.63%` of `qsc/src` and remains the largest remaining mixed-responsibility file by a wide margin.

3. Resume another cross-module foundation order after `NA-0217J`.
   - Rejected because those seams are already extracted and merged; the live problem is now TUI-controller concentration, not missing ownership transfer for `output`, `fs_store`, `protocol_state`, `identity`, `contacts`, `timeline`, `transport`, `attachments`, or `handshake`.

4. Open a broad "split all of `tui/controller.rs` at once" mega-refactor.
   - Rejected because it would widen the review surface and weaken the no-drift proof compared with a bounded command/state-focused lane.

## 7. References

- `START_HERE.md`
- `GOALS.md`
- `AGENTS.md`
- `PROJECT_CHARTER.md`
- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/src/output/mod.rs`
- `qsl/qsl-client/qsc/src/fs_store/mod.rs`
- `qsl/qsl-client/qsc/src/protocol_state/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/src/contacts/mod.rs`
- `qsl/qsl-client/qsc/src/timeline/mod.rs`
- `qsl/qsl-client/qsc/src/transport/mod.rs`
- `qsl/qsl-client/qsc/src/attachments/mod.rs`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/tui/mod.rs`
- `qsl/qsl-client/qsc/src/tui/controller.rs`
- `qsl/qsl-client/qsc/src/tui/render.rs`
- `qsl/qsl-client/qsc/src/tui/script.rs`
- `qsl/qsl-client/qsc/tests/tui_contract_na0217j.rs`
- `qsl/qsl-client/qsc/tests/tui_charter.rs`
- `qsl/qsl-client/qsc/tests/tui_product_polish_na0214a.rs`
- `qsl/qsl-client/qsc/tests/tui_fixed_polling.rs`
- `qsl/qsl-client/qsc/tests/tui_lock_unlock_polish.rs`
- `qsl/qsl-client/qsc/tests/tui_locked_cmd_init_ux.rs`
- `qsl/qsl-client/qsc/tests/tui_main_scroll_focus.rs`
- `qsl/qsl-client/qsc/tests/tui_messages_ux_mvp.rs`
- `qsl/qsl-client/qsc/tests/tui_contacts_option1.rs`
- `qsl/qsl-client/qsc/tests/tui_relay_drop_reorder.rs`
- `qsl/qsl-client/qsc/tests/desktop_gui_contract_na0215b.rs`
- `tests/NA-0224_qsc_modularization_plan_refresh_testplan.md`
