Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-03-29

# DOC-QSC-008 — Desktop GUI Architecture Decision (Linux/macOS) v0.1.0 DRAFT

Purpose:
- execute `NA-0215` against the current qsc / qsl-attachments / qsl-server posture;
- decide whether a desktop GUI is warranted now for Linux and macOS; and
- freeze the narrowest architecture boundary that keeps any future GUI shell from changing protocol, relay, attachment-service, or secret-hygiene semantics.

Non-goals:
- no qsc runtime changes;
- no qsl-attachments changes;
- no qsl-server changes;
- no website, `.github`, or workflow changes;
- no protocol, relay, attachment-service, or cryptographic semantic changes; and
- no GUI implementation in this item.

## 1. Authoritative inputs reviewed

This decision is grounded by the current merged state of:
- qsl-protocol governance:
  - `NEXT_ACTIONS.md`
  - `TRACEABILITY.md`
  - `DECISIONS.md`
- current qsc product contracts and validated operator baseline:
  - `docs/design/DOC-QSC-007_qsc_TUI_UX_Error_State_and_Packaging_Audit_v0.1.0_DRAFT.md`
  - `docs/qsc/QSC_TUI_SPEC.md`
  - `docs/qsc/DOC-QSC-006_Remote_Relay_Testing_Contract_v1.0.0_DRAFT.md`
  - `qsl/qsl-client/qsc/README.md`
  - `qsl/qsl-client/qsc/LOCAL_TWO_CLIENT_RUNBOOK.md`
- qsc implementation and packaging anchors:
  - `qsl/qsl-client/qsc/Cargo.toml`
  - `qsl/qsl-client/qsc/src/main.rs`
  - `qsl/qsl-client/qsc/src/cmd/mod.rs`
  - `qsl/qsl-client/qsc/src/store/mod.rs`
  - `qsl/qsl-client/qsc/src/relay/mod.rs`
  - `qsl/qsl-client/qsc/src/vault/mod.rs`
  - `.github/workflows/macos-build.yml`
  - `.github/workflows/release-auth.yml`
- current metadata / attachment / relay boundaries that the GUI must preserve:
  - `docs/design/DOC-G5-004_Metadata_Leakage_Surface_Review_and_Logging_Contract_v0.1.0_DRAFT.md`
  - `docs/design/DOC-ATT-009_Post_W0_Activation_and_Legacy_Mode_Retirement_Decision_v0.1.0_DRAFT.md`
  - qsl-attachments `NEXT_ACTIONS.md`
  - qsl-server `NEXT_ACTIONS.md`
- current primary-source framework/platform facts reviewed on 2026-03-29:
  - Tauri v2 prerequisites and distribution docs
  - Tauri capability / ACL reference
  - Electron packaging tutorial
  - Electron security tutorial

## 2. Desktop GUI architecture inventory

| Item | Classification | Evidence | Decision consequence |
| --- | --- | --- | --- |
| Whether a desktop GUI is warranted now | needs decision | `NA-0214A` and `NA-0214C` closeout evidence; `qsl/qsl-client/qsc/README.md`; `LOCAL_TWO_CLIENT_RUNBOOK.md`; `.github/workflows/macos-build.yml`; `.github/workflows/release-auth.yml` | The TUI/product-quality wave is complete enough that the next blocker is no longer more direct TUI polish. The remaining question is whether a GUI shell is now the truthful next client-surface investment. |
| qsc / qsl-attachments / qsl-server role split | already clear | `DOC-G5-004`; `DOC-ATT-009`; qsl-attachments `READY=0`; qsl-server `NA-0011` done / `READY=0`; qsc README/runbook | Any GUI must remain a client shell only. qsc stays the owner of protocol/session/vault/attachment behavior; qsl-server stays transport-only; qsl-attachments stays opaque ciphertext-only. |
| Tauri-first versus alternatives | needs decision | qsc is a Rust binary crate with `ratatui`/`crossterm`; `qsl/qsl-client/qsc/src/main.rs` is the dominant product/runtime surface; Tauri official docs support sidecar-friendly desktop bundling and IPC capabilities; Electron official docs package app code with the Electron runtime and require broader security hardening | The decision must choose whether the smallest honest GUI lane is a Rust-adjacent shell over existing qsc behavior or a heavier JS/Chromium-centered app stack. |
| Linux/macOS packaging and distribution plausibility | already clear | `.github/workflows/macos-build.yml`; `.github/workflows/release-auth.yml`; Tauri official docs for Linux prerequisites and macOS app bundles; Electron official docs for packaging/signing | Packaging exists as a bounded product concern, not a blocking architecture unknown. The repo already proves cross-platform qsc builds, and official desktop-framework docs show a plausible Linux/macOS bundling path. |
| Secrets, logs, metadata, and support-artifact treatment in a GUI shell | already clear | `DOC-G5-004`; `LOCAL_TWO_CLIENT_RUNBOOK.md`; qsc marker/output tests cited in `DOC-QSC-007` | The GUI must preserve existing secret-safe output rules: no capability-like secrets in URLs, passive logs, screenshots, or support artifacts; no shadow persistence of vault/session/attachment secrets outside qsc-owned state. |
| What stays in qsc versus what a GUI wrapper may surface | needs decision | `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/src/cmd/mod.rs`; `qsl/qsl-client/qsc/src/store/mod.rs`; `qsl/qsl-client/qsc/src/vault/mod.rs`; validated runbook flows | The architecture must freeze whether a GUI may reimplement client logic or must stay a shell over qsc-owned truth sources. |

## 3. Architecture option set

| Option | Summary | Fit with roadmap / TUI maturity | Fit with qsl-attachments / qsl-server boundaries | Packaging / security implication | Result |
| --- | --- | --- | --- | --- | --- |
| `GUI0` | bounded prototype-boundary work is next | Strong. The qbuild-first local baseline is already explicit, TUI product polish is complete, macOS CI builds are already enforced, and release artifacts already include `qsc` binaries | Strong. A GUI shell can be frozen as client-only while preserving transport-only relay and opaque ciphertext-only attachment service roles | Strong if the GUI remains a shell over qsc-owned behavior. Tauri-sidecar packaging and capability scoping align with that boundary on Linux/macOS | chosen |
| `GUI1` | one smaller architecture/finalization gap still blocks a prototype boundary | Weak. The only meaningful unresolved questions are exactly the prototype-boundary questions rather than a separate missing architecture proof item | No additional cross-repo policy gap is visible | Would create an extra docs-only loop without reducing current risk | rejected |
| `GUI2` | continued TUI-only support is the truthful next posture for now | Weak. The repo no longer shows a direct TUI product-quality blocker; cross-platform build and packaging evidence are already good enough to evaluate a GUI shell honestly | Boundaries stay safe, but this would defer the next client-surface lane without a newly proven blocker | Would preserve today’s state, but only by ignoring the now-explicit GUI-shell question instead of answering it | rejected |

Why `GUI0` wins:
- the current repo state already separates stable protocol/relay/attachment behavior from operator-surface evolution;
- the main architectural risk is not whether a GUI should exist, but how to keep it from becoming a second client-core implementation;
- that risk can be bounded now by freezing a shell-only architecture; and
- the remaining open work naturally belongs in a prototype-boundary lane, not another architecture-finalization lane.

## 4. Accumulated evidence review

### 4.1 Why a GUI is warranted now

The current qsc lane is no longer blocked on basic product safety or operator truthfulness:
- `NA-0214A` and `NA-0214C` already completed the TUI polish and validation/cleanup wave;
- `qsl/qsl-client/qsc/README.md` and `LOCAL_TWO_CLIENT_RUNBOOK.md` now define a truthful qbuild-first, AWS-free front door;
- `.github/workflows/macos-build.yml` already proves an enforced macOS build/test lane for qsc; and
- `.github/workflows/release-auth.yml` already produces release artifacts for `qsc`.

That does not mean the TUI is obsolete. It means the next client-surface question can now be answered without inventing new relay, attachment, or protocol semantics first.

### 4.2 Why the GUI must stay a shell over qsc-owned truth

Current qsc evidence shows a strong product surface but not a clean reusable library boundary:
- `qsl/qsl-client/qsc/src/main.rs` is the dominant runtime/product surface at `21,787` lines;
- command parsing lives in `qsl/qsl-client/qsc/src/cmd/mod.rs`, while protocol/session/store/vault behavior is still tightly anchored to the main binary crate; and
- the current stable machine-readable surface is deterministic marker output plus existing CLI subcommands, not a separately frozen GUI API crate.

The truthful consequence is that a near-term GUI must not reimplement protocol/session/attachment logic in a second codepath or in frontend JavaScript. The safest narrow boundary is a GUI shell that invokes qsc-owned behavior and preserves qsc as the canonical truth source.

### 4.3 Why Tauri-first beats the alternatives for this repo

Current primary-source platform facts materially inform the choice:
- Tauri’s current docs explicitly support Linux/macOS application bundling and per-platform bundle configuration, and Tauri’s capability model exists to isolate window/webview access to IPC and local-system permissions.
- Tauri’s current docs also show Linux development depends on WebKitGTK-class system packages, which is a real but bounded packaging constraint rather than a protocol blocker.
- Electron’s current packaging docs describe distribution by packaging app code with the Electron runtime and then producing OS-specific distributables, with code signing strongly recommended for end-user trust and required for the usual auto-update path.
- Electron’s current security docs keep a longer hardening checklist around `contextIsolation`, sandboxing, Node exposure, permission handling, CSP, and IPC sender validation.

For this repo, that leads to a clear architecture preference:
- qsc is already Rust-heavy and security-sensitive;
- the desired GUI must preserve strict local-permission and secret-hygiene boundaries;
- the smallest honest GUI shell is therefore Rust-adjacent and least powerful by default; and
- Tauri fits that boundary better than Electron.

Electron is not impossible. It is simply a worse fit for the current repo because it adds a heavier runtime/tooling/security surface without solving the core architectural problem that qsc itself is still the only truthful owner of client behavior.

## 5. Decision

Chosen result:
- `GUI0`

Closeout path implied by this decision:
- `BB1`

Exact reason:
- a desktop GUI is warranted now as the next client-shell investment because qsc’s TUI/local-baseline/product-polish work is already complete enough to stop treating the TUI itself as the load-bearing blocker; and
- the architecture boundary is already explicit enough to say that the next blocker is defining a bounded prototype shell, not re-litigating whether a GUI should exist or whether qsc / qsl-attachments / qsl-server roles should change.

Truthful successor lane:
- `NA-0215A — Desktop GUI Prototype Boundary`

## 6. Frozen desktop GUI architecture boundary

### 6.1 Framework direction

The desktop GUI direction is:
1. Tauri-first.
2. Linux and macOS only.
3. Local bundled assets only; no remote-hosted web frontend.

Electron remains a fallback only if a later lane proves a load-bearing Linux/macOS packaging or sidecar constraint that Tauri cannot satisfy without weakening security or boundary rules. No such blocker is proven today.

### 6.2 Component ownership

`qsc` remains the owner of:
- vault init/unlock and secret handling;
- identity, contacts, trust, and device state;
- handshake/session logic;
- send/receive and truthful delivery/file state;
- attachment policy and attachment-service interaction;
- relay configuration and route-token/header behavior;
- local encrypted stores and journals; and
- deterministic markers / operator-truth surfaces.

The GUI wrapper may own:
- native window management;
- forms, lists, routing, and progress presentation;
- local non-secret UI state;
- OS file-picker / notification / menu integration; and
- packaging/update/help surfaces that do not alter qsc truth.

`qsl-server` remains transport-only.

`qsl-attachments` remains opaque ciphertext-only.

### 6.3 Process and IPC boundary

The prototype-direction boundary is:
1. bundle and invoke `qsc` as a target-specific sidecar binary rather than scraping the full-screen TUI or simulating keystrokes against `qsc tui`;
2. keep frontend JavaScript/WebView code away from direct relay, attachment-service, vault, and journal ownership;
3. restrict local-system access to a small Tauri/Rust-side bridge with explicit capability/ACL scope; and
4. treat arbitrary shell execution as forbidden.

Implication:
- the GUI may consume existing deterministic markers and bounded machine-readable qsc outputs;
- it may not create a parallel protocol/session implementation in JavaScript; and
- any future move from sidecar orchestration to in-process Rust library integration would require a later explicit lane because current qsc is not yet a clean reusable library.

### 6.4 Secret-hygiene and metadata boundary

The GUI must preserve `DOC-G5-004` exactly:
- no route tokens, bearer tokens, `resume_token`, `fetch_capability`, `enc_ctx_b64u`, or vault passphrases in URLs, passive logs, screenshots, copied debug artifacts, or support bundles;
- no shadow persistence of qsc-owned secret-bearing state in frontend stores, browser-like caches, or webview local storage;
- no weakening of header-only route-token carriage;
- no weakening of retired-mode or fail-closed attachment behavior; and
- no dishonest delivery or file-state presentation.

GUI-native diagnostics, crash reports, and screenshots must be secret-safe by default and at least as redacted as current qsc markers/runbooks.

### 6.5 Packaging boundary for Linux and macOS

The current architecture decision freezes these boundaries:
- Linux and macOS are in scope; Windows, mobile, and browser delivery are out of scope.
- A future prototype may promise only bounded developer-run packaging plus one honest distributable path per platform.
- macOS App Store distribution, Windows support, auto-update infrastructure, and broad distro-matrix guarantees are out of the first prototype boundary unless a later lane explicitly promotes them.
- Linux framework/runtime dependencies and macOS signing/notarization remain packaging concerns, not reasons to move protocol or secret handling out of qsc.

### 6.6 GUI functional scope boundary

A bounded GUI prototype may surface:
- unlock / init / identity onboarding;
- contact list and trust/device status;
- conversation list and send/receive flows;
- file-send / file-receive status;
- relay / account settings; and
- truthful help/status surfaces.

It must not:
- embed a terminal emulator as the core product surface;
- change protocol, relay, attachment-service, or auth semantics;
- bypass qsc’s vault/store/marker truth sources; or
- introduce a second persistent secret store.

## 7. Queue implication

The next truthful queue item after this decision is:
- `NA-0215A — Desktop GUI Prototype Boundary`

That lane should define:
1. the minimal GUI feature set that proves the shell boundary honestly;
2. what `qsc` commands/outputs are in scope for the prototype;
3. what Linux/macOS packaging claims are in and out of scope for the prototype; and
4. the concrete security boundary between the GUI shell, its Rust/Tauri bridge, and bundled `qsc`.

It should not reopen:
- whether a GUI is warranted at all;
- whether qsl-server or qsl-attachments roles change; or
- whether protocol/session logic belongs in the webview/frontend.

## References

Repo evidence:
- `docs/design/DOC-QSC-007_qsc_TUI_UX_Error_State_and_Packaging_Audit_v0.1.0_DRAFT.md`
- `docs/qsc/QSC_TUI_SPEC.md`
- `docs/design/DOC-G5-004_Metadata_Leakage_Surface_Review_and_Logging_Contract_v0.1.0_DRAFT.md`
- `docs/design/DOC-ATT-009_Post_W0_Activation_and_Legacy_Mode_Retirement_Decision_v0.1.0_DRAFT.md`
- `qsl/qsl-client/qsc/README.md`
- `qsl/qsl-client/qsc/LOCAL_TWO_CLIENT_RUNBOOK.md`
- `qsl/qsl-client/qsc/Cargo.toml`
- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/src/cmd/mod.rs`
- `.github/workflows/macos-build.yml`
- `.github/workflows/release-auth.yml`
- qsl-attachments `NEXT_ACTIONS.md`
- qsl-server `NEXT_ACTIONS.md`

Primary-source framework/platform docs reviewed 2026-03-29:
- Tauri prerequisites: https://v2.tauri.app/start/prerequisites/
- Tauri capability / ACL reference: https://v2.tauri.app/reference/acl/capability/
- Tauri macOS bundle docs: https://v2.tauri.app/distribute/macos-application-bundle/
- Electron packaging tutorial: https://www.electronjs.org/docs/latest/tutorial/packaging-your-application
- Electron security tutorial: https://www.electronjs.org/docs/latest/tutorial/security
