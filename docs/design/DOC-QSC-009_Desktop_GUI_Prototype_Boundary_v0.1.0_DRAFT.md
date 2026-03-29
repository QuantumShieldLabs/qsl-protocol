Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-03-29

# DOC-QSC-009 — Desktop GUI Prototype Boundary (Linux/macOS) v0.1.0 DRAFT

Purpose:
- execute `NA-0215A` against the current qsc / qsl-attachments / qsl-server posture;
- freeze the smallest truthful Linux/macOS desktop GUI prototype boundary without implementing the GUI;
- define the exact `qsc` command/output subset, bundled-sidecar contract, packaging claims, and secret boundary the prototype may rely on; and
- decide whether bounded GUI implementation is now the next blocker or whether a smaller boundary-finalization or broader hardening lane still outranks it.

Non-goals:
- no qsc runtime changes;
- no qsl-attachments changes;
- no qsl-server changes;
- no website, `.github`, or workflow changes;
- no protocol, relay, attachment-service, auth, or cryptographic semantic changes;
- no scraping of `qsc tui` or simulated keystrokes against the TUI; and
- no GUI implementation in this item.

## 1. Authoritative inputs reviewed

This boundary decision is grounded by the current merged state of:
- qsl-protocol governance:
  - `NEXT_ACTIONS.md`
  - `TRACEABILITY.md`
  - `DECISIONS.md`
- current desktop-GUI and qsc product-surface artifacts:
  - `docs/design/DOC-QSC-008_Desktop_GUI_Architecture_Decision_v0.1.0_DRAFT.md`
  - `docs/design/DOC-QSC-007_qsc_TUI_UX_Error_State_and_Packaging_Audit_v0.1.0_DRAFT.md`
  - `docs/qsc/QSC_TUI_SPEC.md`
  - `docs/qsc/QSC_TUI_INVARIANTS.md`
  - `qsl/qsl-client/qsc/README.md`
  - `qsl/qsl-client/qsc/LOCAL_TWO_CLIENT_RUNBOOK.md`
- current qsc command/output/tests proving the sidecar-facing surface:
  - `qsl/qsl-client/qsc/src/cmd/mod.rs`
  - `qsl/qsl-client/qsc/tests/cli.rs`
  - `qsl/qsl-client/qsc/tests/diagnostics.rs`
  - `qsl/qsl-client/qsc/tests/vault.rs`
  - `qsl/qsl-client/qsc/tests/unlock_gate.rs`
  - `qsl/qsl-client/qsc/tests/two_client_local_runbook_na0182.rs`
  - `qsl/qsl-client/qsc/tests/timeline_store.rs`
  - `qsl/qsl-client/qsc/tests/message_state_model.rs`
  - `qsl/qsl-client/qsc/tests/identity_secret_at_rest.rs`
  - `qsl/qsl-client/qsc/tests/session_state_at_rest.rs`
- current metadata / relay / attachment boundaries the GUI must preserve:
  - `docs/design/DOC-G5-004_Metadata_Leakage_Surface_Review_and_Logging_Contract_v0.1.0_DRAFT.md`
  - `docs/design/DOC-ATT-009_Post_W0_Activation_and_Legacy_Mode_Retirement_Decision_v0.1.0_DRAFT.md`
  - qsl-attachments `README.md`
  - qsl-server `docs/server/DOC-SRV-005_Route_Token_API_Shape_Review_v1.0.0_DRAFT.md`
- current primary-source platform/package facts reviewed on 2026-03-29:
  - Tauri prerequisites
  - Tauri sidecar / external-binary docs
  - Tauri capability / ACL reference
  - Tauri macOS app-bundle docs
  - Tauri AppImage docs
  - Apple Developer ID / notarization docs for outside-App-Store distribution

## 2. Desktop GUI prototype-boundary inventory

| Item | Classification | Evidence | Boundary consequence |
| --- | --- | --- | --- |
| Smallest honest feature slice for a first prototype | needs boundary decision | `DOC-QSC-008`; `QSC_TUI_SPEC`; `LOCAL_TWO_CLIENT_RUNBOOK.md`; `timeline_store.rs`; `message_state_model.rs`; `two_client_local_runbook_na0182.rs` | The prototype must be narrower than the full GUI architecture allowance so it does not invent a transcript API, attachment UI, or multiprofile model that current `qsc` does not expose cleanly. |
| Exact `qsc` command/output subset the prototype may rely on | needs boundary decision | `src/cmd/mod.rs`; `cli.rs`; `diagnostics.rs`; `vault.rs`; `unlock_gate.rs`; `timeline_store.rs`; `two_client_local_runbook_na0182.rs` | The prototype must rely only on commands and marker/text shapes already present in `main`, and should prefer exit-status-plus-refresh over parsing ad hoc human wording. |
| Exact bundled-sidecar contract | needs boundary decision | `DOC-QSC-008`; `src/cmd/mod.rs`; `diagnostics.rs`; `vault.rs`; current Tauri sidecar/capability docs | The GUI must bundle one target-specific `qsc` binary and invoke only an explicit allowlist through a small Rust/Tauri bridge, never through arbitrary shell execution. |
| Honest Linux/macOS packaging and distribution claims | needs boundary decision | `qsl/qsl-client/qsc/README.md`; `.github/workflows/macos-build.yml`; `.github/workflows/release-auth.yml`; current Tauri bundle docs; Apple signing/notarization docs | The first prototype can promise bounded developer-run packaging, but not App Store, auto-update, Windows, browser, or broad distro-matrix support. |
| Security / secret-hygiene treatment for GUI-visible secrets, logs, metadata, and support artifacts | already clear | `DOC-G5-004`; `identity_secret_at_rest.rs`; `session_state_at_rest.rs`; `diagnostics.rs`; qsl-attachments `README.md`; qsl-server `DOC-SRV-005` | The GUI must not create a second persistent secret store, emit token-bearing URLs, or widen passive logging/screenshot leakage. |
| What remains in `qsc` versus what the GUI may surface | already clear, with one prototype narrowing decision | `DOC-QSC-008`; `QSC_TUI_SPEC`; `QSC_TUI_INVARIANTS`; `src/cmd/mod.rs` | `qsc` remains the owner of vault, identity, contacts, routing, send/receive, timeline truth, and local encrypted state. The GUI may own only presentation, ephemeral compose state, and native desktop affordances. |
| Whether broader hardening should outrank direct prototype implementation after this boundary is frozen | needs boundary decision | current qsc audit/test surface plus sibling repo `READY=0` proofs | Current evidence does not show a missing adversarial-program definition that blocks a shell-only GUI prototype more directly than the implementation work itself. |

## 3. GUI prototype-boundary option set

| Option | Summary | Fit with roadmap / TUI maturity | Fit with qsl-attachments / qsl-server boundaries | Packaging / secret-hygiene implication | Result |
| --- | --- | --- | --- | --- | --- |
| `GP0` | bounded GUI prototype implementation is next | Strong. The qsc product-polish and validation wave is complete, the local qbuild-first runbook is explicit, and the needed command/marker surface already exists in `main`. | Strong. A message-first shell can stay inside current qsc ownership while leaving qsl-server transport-only and qsl-attachments opaque ciphertext-only. | Strong if the prototype is narrowed to a sidecar-only, message-first slice with explicit packaging and no second secret store. | chosen |
| `GP1` | one smaller GUI-boundary/finalization gap still blocks implementation | Weak. The only real ambiguities come from over-broad prototype assumptions (attachments UI, historical transcript rendering, multiprofile state), and those disappear once the prototype is narrowed to the existing qsc surface. | No additional sibling-repo contract work is required. | Would add another docs-only loop without reducing current implementation risk more than this boundary document already does. | rejected |
| `GP2` | broader hardening should outrank GUI implementation after this boundary | Weak. The current repo already has deterministic tests for lock gating, secret-safe markers, at-rest protection, route-header discipline, local runbook flows, and delivery-state truth. | No relay or attachment-service blocker newly outranks the GUI shell after this boundary is frozen. | Adversarial validation remains valuable, but current evidence does not show it as the more immediate blocker than implementing the already-frozen shell slice. | rejected |

Why `GP0` wins:
- the remaining risk is not whether a GUI is warranted, but whether the prototype is forced to parse or own surfaces that current `qsc` does not expose cleanly;
- this document can freeze a smaller, message-first prototype that avoids those non-frozen surfaces without inventing new runtime behavior; and
- after that narrowing, the next blocker becomes implementation of the shell, not more policy or queue work.

## 4. Accumulated evidence review

### 4.1 Why the prototype can be frozen now

Current repo evidence already exposes a bounded sidecar-facing surface:
- `status` and `doctor --check-only` emit deterministic markers, including a test-backed `jsonl` schema for status markers in `diagnostics.rs`.
- vault init/unlock/status success and reject paths are already explicit and secret-safe in `vault.rs` and `unlock_gate.rs`.
- `identity show`, `contacts list`, and `contacts device list` already expose deterministic plain-text fields used by current tests (`identity_fp=...`, `label=...`, `device=...`).
- `send`, `receive`, and `timeline list` already produce delivery-state and timeline markers strong enough to support a message-first shell without reading qsc store files directly.
- current metadata and relay/attachment docs already freeze the secret boundary the GUI must preserve: header-only route-token carriage, secret-free canonical URLs, and no passive leakage of `resume_token`, `fetch_capability`, or `enc_ctx_b64u`.

The meaningful rough edge is that current `qsc` does not expose a separately frozen transcript-rendering API or attachment-centric GUI API. That is not a blocker once the prototype is narrowed to:
- session-scoped compose/send/receive flows for messages,
- timeline-state inspection rather than a full historical plaintext transcript viewer, and
- no attachment/file UI in the first prototype.

### 4.2 Why broader hardening does not outrank the prototype now

The repo already proves the main fail-closed and secret-hygiene properties the GUI shell must preserve:
- locked-path rejects and no-mutation behavior (`unlock_gate.rs`);
- no plaintext identity/session material at rest (`identity_secret_at_rest.rs`, `session_state_at_rest.rs`);
- secret-safe diagnostics and marker schema (`diagnostics.rs`);
- route-header and local-runbook truthfulness (`two_client_local_runbook_na0182.rs`, `DOC-G5-004`, qsl-server `DOC-SRV-005`);
- honest delivery-state transitions (`message_state_model.rs`, `two_client_local_runbook_na0182.rs`);
- sibling repos already at `READY=0`, so no relay or attachment contract blocker currently outranks the prototype shell boundary.

An adversarial validation / fuzz / chaos program remains a sensible future lane, but current evidence does not show it as the immediate blocker for a desktop shell that intentionally reuses existing qsc-owned behavior rather than changing protocol, relay, or attachment semantics.

## 5. Decision

Chosen result:
- `GP0`

Closeout path implied by this decision:
- `BB1`

Exact reason:
- the current repo state is already explicit enough to freeze a minimal, truthful prototype without semantic invention; and
- once the prototype is narrowed to a message-first shell over existing `qsc` commands and markers, the next blocker is implementation of that shell rather than another decision or hardening-definition lane.

Truthful successor lane:
- `NA-0215B — Desktop GUI Prototype Implementation`

## 6. Frozen desktop GUI prototype boundary

### 6.1 Minimal feature slice

The first prototype is a message-first desktop shell for one local qsc profile on Linux/macOS.

In scope:
1. profile readiness gate:
   - detect missing / present / locked posture via `qsc status`, `qsc doctor --check-only`, and vault markers;
2. explicit profile bootstrap for passphrase-backed or keychain-backed local use:
   - `vault init`
   - `identity rotate --confirm`
   - `identity show`
3. relay/account setup:
   - relay base URL entry
   - self inbox token set/replace
4. contact bootstrap:
   - contacts list
   - add contact by fingerprint + route token
   - list devices for a contact
   - trust one device explicitly
5. message session actions:
   - compose and send one-to-one messages through existing `qsc send`
   - poll / receive through existing `qsc receive`
   - display honest delivery state (`accepted_by_relay`, `peer_confirmed`) and timeline state for the current peer
6. truthful status/help panes:
   - show fail-closed errors and locked/profile posture without exposing secrets

Out of scope for the first prototype:
- file / attachment send or receive UI;
- attachment-service configuration UI or any qsl-attachments management surface;
- full historical plaintext transcript rendering from qsc-owned storage;
- multiprofile, multidrive, or account-switching behavior;
- local relay hosting as a primary product surface (it may remain a qbuild/test harness aid outside the GUI);
- TUI embedding, TUI scraping, or command-bar emulation;
- arbitrary shell execution, plugins, remote-hosted frontend content, background auto-update, or Windows/mobile/browser support.

Why this is the smallest honest slice:
- it proves the desktop-shell boundary over real qsc-owned behavior;
- it avoids inventing a transcript or attachment UI contract that the current repo has not frozen separately; and
- it keeps the first implementation inside the already-tested qbuild/local messaging baseline.

### 6.2 Exact `qsc` command and output subset

The prototype may invoke only the following `qsc` surfaces.

Read / posture commands:
- `qsc status`
  - parse only `QSC_MARK/1 event=status ...` and companion marker lines already used in `cli.rs` / `diagnostics.rs`;
- `qsc doctor --check-only`
  - parse only `QSC_MARK/1 event=doctor ...`;
- `qsc vault status`
  - parse only `QSC_MARK/1 event=vault_status ...`;
- `qsc identity show`
  - parse only `identity_fp=...` from stdout;
- `qsc contacts list`
  - parse only `label=...`, `state=...`, `blocked=...`, and `device_count=...` fields already exercised in current tests;
- `qsc contacts device list --label <label>`
  - parse only `device=...` rows;
- `qsc timeline list --peer <label> --limit <n>`
  - parse only `event=timeline_list ...` and `event=timeline_item ...` marker rows with `id=`, `dir=`, `kind=`, `ts=`, and `state=`.

Mutating / action commands:
- `qsc vault init --non-interactive --passphrase-stdin` or the existing qsc-supported noninteractive key-source path;
- `qsc vault unlock --passphrase-env <ENV>`;
- `qsc identity rotate --confirm`;
- `qsc relay inbox-set --token <token>`;
- `qsc contacts add --label <label> --fp <fingerprint> --route-token <token>`;
- `qsc contacts device trust --label <label> --device <device_id> --confirm`;
- `qsc --unlock-passphrase-env <ENV> send --transport relay --relay <url> --to <label> --file <path> --receipt delivered`;
- `qsc --unlock-passphrase-env <ENV> receive --transport relay --relay <url> --mailbox <token> --from <label> --max <n> --out <dir> --emit-receipts delivered --receipt-mode immediate`.

Parsing rule:
- for mutating commands other than send/receive, the GUI must rely on process exit status plus a follow-up read command instead of scraping free-form success text;
- for send/receive, the GUI may rely only on:
  - `QSC_DELIVERY state=accepted_by_relay ...`
  - `QSC_DELIVERY state=peer_confirmed ...`
  - `QSC_MARK/1 event=error code=...`
- the GUI must treat any unknown or extra stdout/stderr as non-contract output and must not infer new semantics from it.

Explicitly forbidden dependencies:
- `qsc tui`
- store-file parsing by the GUI
- attachment/file commands in the first prototype
- undocumented shell wrappers around `qsc`

### 6.3 Bundled-sidecar contract

The prototype must bundle exactly one target-specific `qsc` executable as a sidecar binary for the host platform/arch.

Required contract:
1. the frontend/webview may not spawn processes directly;
2. only a small Rust/Tauri bridge may invoke the sidecar;
3. the bridge may execute only the allowlisted commands in section 6.2 with explicit argument construction;
4. no shell interpolation, pass-through command mode, or arbitrary process execution is allowed;
5. the bridge must serialize qsc mutations per profile so the GUI does not race qsc's locked state or journal updates; and
6. the GUI must refresh state by re-running allowlisted read commands, not by reading qsc private files directly.

Secret-handling contract:
- the GUI may keep a passphrase only in memory for the active app session;
- it must never persist that passphrase in local storage, webview storage, logs, crash reports, screenshots, or copied diagnostics;
- it must never pass secrets on the command line;
- if the implementation uses qsc's existing env-based unlock path for a child process, that env must be scoped to the child invocation only and omitted from all diagnostics.

### 6.4 Linux/macOS packaging claims

The first prototype may truthfully claim only:
- Linux:
  - one developer-run Tauri desktop build that bundles the GUI plus the target-matched `qsc` sidecar;
  - one AppImage distributable path for the validated Linux lane;
  - no promise of universal distro compatibility beyond the documented Tauri/system prerequisites on the build and test hosts;
- macOS:
  - one developer-run `.app` bundle built on macOS with bundled `qsc`;
  - local execution from the bundle as the first validated macOS path;
  - no App Store submission, auto-update, or broad signing/notarization promise in the first prototype.

Out of scope packaging claims:
- Windows, iOS, Android, browser, or remote web deployment;
- macOS App Store delivery;
- automatic updates;
- universal distro-matrix guarantees across Linux families; and
- any claim that packaging changes move protocol, secret handling, or relay/attachment semantics out of `qsc`.

Inference from current platform docs:
- current Tauri docs support bundling external binaries and producing macOS app bundles / Linux AppImage artifacts;
- current Apple distribution docs imply that outside-App-Store macOS distribution eventually needs Developer ID signing and notarization, but that is a later packaging-hardening concern rather than a blocker for the first prototype boundary.

### 6.5 Security and ownership boundary

`qsc` remains the owner of:
- vault lifecycle and encrypted local state;
- identity, contact, device-trust, relay token, and timeline truth;
- send/receive behavior and honest delivery state; and
- all route-token/header, attachment-policy, and fail-closed reject semantics.

The GUI may own only:
- windows, forms, view routing, list selection, and compose-buffer state;
- bounded in-memory passphrase handling for explicit unlock/init flows;
- ephemeral receive output directories used for the current app session; and
- desktop affordances such as menus and notifications, provided they remain secret-safe.

The GUI must not:
- create a second persistent secret store;
- persist plaintext transcripts or attachment metadata outside qsc-owned state;
- emit route tokens, `resume_token`, `fetch_capability`, `enc_ctx_b64u`, bearer tokens, or passphrases in URLs, passive logs, crash reports, screenshots, or support bundles;
- weaken header-only route-token carriage;
- claim stronger message/file state than qsc has actually reported; or
- move relay, attachment-service, protocol, or vault semantics into frontend JavaScript.

### 6.6 Why `NA-0215B` is now the truthful next lane

After this boundary is frozen:
- the remaining work is implementing the sidecar shell itself;
- no smaller boundary question remains load-bearing once the prototype is narrowed to the surfaces above; and
- broader fuzz/chaos-program definition does not outrank this shell-only prototype because current repo evidence already makes the shell boundary decision-grade without first inventing a new validation program.

## 7. Queue implication

The next truthful queue item after this boundary decision is:
- `NA-0215B — Desktop GUI Prototype Implementation`

That lane should:
1. implement exactly the message-first, sidecar-only prototype boundary frozen here;
2. preserve qsc as the canonical owner of state and truth;
3. add deterministic proof for the allowed command/output subset the GUI consumes; and
4. keep Linux/macOS packaging claims limited to the bundle paths frozen here.

It should not reopen:
- whether a GUI is warranted;
- whether file/attachment UI belongs in the first prototype;
- whether protocol/session logic moves into frontend code; or
- whether qsl-server / qsl-attachments roles change.

## References

Repo evidence:
- `docs/design/DOC-QSC-008_Desktop_GUI_Architecture_Decision_v0.1.0_DRAFT.md`
- `docs/design/DOC-QSC-007_qsc_TUI_UX_Error_State_and_Packaging_Audit_v0.1.0_DRAFT.md`
- `docs/qsc/QSC_TUI_SPEC.md`
- `docs/qsc/QSC_TUI_INVARIANTS.md`
- `qsl/qsl-client/qsc/README.md`
- `qsl/qsl-client/qsc/LOCAL_TWO_CLIENT_RUNBOOK.md`
- `qsl/qsl-client/qsc/src/cmd/mod.rs`
- `qsl/qsl-client/qsc/tests/cli.rs`
- `qsl/qsl-client/qsc/tests/diagnostics.rs`
- `qsl/qsl-client/qsc/tests/vault.rs`
- `qsl/qsl-client/qsc/tests/unlock_gate.rs`
- `qsl/qsl-client/qsc/tests/two_client_local_runbook_na0182.rs`
- `qsl/qsl-client/qsc/tests/timeline_store.rs`
- `qsl/qsl-client/qsc/tests/message_state_model.rs`
- `qsl/qsl-client/qsc/tests/identity_secret_at_rest.rs`
- `qsl/qsl-client/qsc/tests/session_state_at_rest.rs`
- `docs/design/DOC-G5-004_Metadata_Leakage_Surface_Review_and_Logging_Contract_v0.1.0_DRAFT.md`
- `docs/design/DOC-ATT-009_Post_W0_Activation_and_Legacy_Mode_Retirement_Decision_v0.1.0_DRAFT.md`
- qsl-attachments `README.md`
- qsl-server `docs/server/DOC-SRV-005_Route_Token_API_Shape_Review_v1.0.0_DRAFT.md`

Primary-source platform/package docs reviewed 2026-03-29:
- Tauri prerequisites: https://v2.tauri.app/start/prerequisites/
- Tauri sidecar docs: https://v2.tauri.app/develop/sidecar/
- Tauri capability / ACL reference: https://v2.tauri.app/reference/acl/capability/
- Tauri macOS application bundle docs: https://v2.tauri.app/distribute/macos-application-bundle/
- Tauri AppImage docs: https://v2.tauri.app/distribute/appimage/
- Apple Developer ID / notarization overview: https://developer.apple.com/developer-id/
