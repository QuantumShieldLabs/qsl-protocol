Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-03-29

# DOC-QSC-010 — Desktop GUI Prototype Active-Ops Boundary v0.1.0 DRAFT

Purpose:
- execute `NA-0215BC` against the merged desktop GUI prototype;
- freeze what "active ops" means for the bounded Linux/macOS shell;
- decide whether keychain-backed active operations stay deferred;
- define the protocol-readiness and unlock ingress requirements before GUI send/receive; and
- determine whether the truthful successor is validation/cleanup or one smaller direct implementation follow-on.

Non-goals:
- no qsc, qsl-server, or qsl-attachments semantic change;
- no new GUI runtime feature work;
- no handshake/session-establish UI;
- no second persistent secret store; and
- no widening into attachments, transcript history, multiprofile, or shell passthrough.

## 1. Authoritative inputs reviewed

This boundary finalization is grounded by the merged state of:
- governance and prior boundary items:
  - `NEXT_ACTIONS.md`
  - `TRACEABILITY.md`
  - `DECISIONS.md`
  - `docs/design/DOC-QSC-008_Desktop_GUI_Architecture_Decision_v0.1.0_DRAFT.md`
  - `docs/design/DOC-QSC-009_Desktop_GUI_Prototype_Boundary_v0.1.0_DRAFT.md`
- current desktop-shell operator docs:
  - `qsl/qsl-client/qsc-desktop/README.md`
- current desktop-shell implementation truth:
  - `qsl/qsl-client/qsc-desktop/src/main.js`
  - `qsl/qsl-client/qsc-desktop/src-tauri/src/main.rs`
  - `qsl/qsl-client/qsc-desktop/src-tauri/src/model.rs`
  - `qsl/qsl-client/qsc-desktop/src-tauri/src/qsc.rs`
- deterministic proof already merged for the allowed shell contract:
  - `qsl/qsl-client/qsc/tests/desktop_gui_contract_na0215b.rs`
  - `qsl/qsl-client/qsc/tests/qsp_protocol_gate.rs`

## 2. Current boundary issue set

| Issue | Classification | Why it stays within `NA-0215BC` scope | Resolution |
| --- | --- | --- | --- |
| "Active ops" is still implied across README, tests, and runtime behavior instead of frozen in one decision-grade place. | boundary-finalization blocker | The merged runtime already behaves consistently; the missing work is to state the boundary explicitly without changing semantics. | Freeze the active-ops set and readiness rules here. |
| `DOC-QSC-009` still allows "passphrase-backed or keychain-backed local use" language for profile bootstrap. | docs/boundary mismatch | Current merged prototype and README already behave as passphrase-centered for active flows, so the mismatch is documentary rather than an implementation gap. | Align the older prototype-boundary wording to the merged truth. |
| Protocol-activation readiness is proven in tests/runtime but not yet stated as one explicit GUI rule set. | docs/boundary mismatch | Current `qsc` handshake status markers and desktop bridge already expose the truth; no new protocol behavior is needed. | Freeze exact send/receive readiness requirements and out-of-band activation wording here. |
| No remaining runtime/UI mismatch was found in the merged prototype for the active-ops boundary. | runtime/UI mismatch | The current GUI already surfaces `keychain_deferred`, `protocol_inactive`, and out-of-scope messaging truthfully. | No runtime patch required in this lane. |

## 3. Frozen active-ops decision set

### 3.1 What counts as "active ops" in the prototype

For this bounded prototype, "active ops" means GUI-triggered sidecar actions that either:
- mutate local `qsc` state under the current profile; or
- produce peer-visible relay traffic or delivery-state changes.

In-scope active ops are:
- passphrase profile initialization via `qsc vault init --non-interactive --passphrase-stdin`;
- passphrase unlock via `qsc vault unlock --non-interactive --passphrase-env <ENV>`;
- `qsc identity rotate --confirm` during local bootstrap;
- `qsc relay inbox-set --token <token>`;
- `qsc contacts add ...`;
- `qsc contacts device trust ...`;
- `qsc send ...`; and
- `qsc receive ...`.

Read-only posture inspection is not an active op. The GUI may still use:
- `qsc status`;
- `qsc doctor --check-only`;
- `qsc vault status`;
- `qsc identity show`;
- `qsc contacts list`;
- `qsc contacts device list --label <label>`;
- `qsc handshake status --peer <label>`; and
- `qsc timeline list --peer <label> --limit <n>`.

### 3.2 Keychain-backed active operations

Keychain-backed active operations stay deferred for this prototype.

Meaning:
- the GUI may surface that a vault exists and that `key_source=keychain`;
- the GUI may show the stable prototype posture `keychain_deferred`;
- but the GUI must not claim that keychain-backed unlock, send, receive, relay mutation, contact mutation, or device-trust mutation is supported in this prototype build.

Reason:
- the merged desktop shell only exposes the passphrase-backed active-operation ingress truthfully;
- inventing keychain activation semantics in the GUI would require sidecar/runtime behavior that is not frozen in the current bounded shell contract; and
- deferring keychain-backed active ops preserves the "no second persistent secret store" and "no keychain-state guesswork" constraints.

### 3.3 Protocol-activation readiness before GUI send/receive

The GUI may inspect protocol posture, but it may not activate protocol state itself.

Required readiness:
- send is allowed only when:
  - a contact is selected;
  - the current profile is passphrase-backed for active use;
  - the current session is unlocked; and
  - `qsc handshake status --peer <label>` proves `send_ready=yes`;
- receive is allowed only when:
  - a contact is selected;
  - the current profile is passphrase-backed for active use;
  - the current session is unlocked; and
  - `qsc handshake status --peer <label>` reports `status=established` or `status=established_recv_only`.

Fail-closed consequences:
- `status=no_session` or `protocol_inactive reason=no_session|missing_seed` means the peer is not activated yet;
- `status=established_recv_only` or `protocol_inactive reason=chainkey_unset` means receive may be truthful while send remains blocked;
- `protocol_inactive reason=session_invalid` means the stored state must be re-established outside the GUI; and
- `protocol_inactive reason=vault_secret_missing` means the local profile must be unlocked again before readiness can be restored.

Handshake activation remains out of scope:
- the GUI may call `handshake status` for inspection only;
- the GUI must not add `handshake init` or `handshake poll` surfaces in this prototype; and
- the operator must run those flows in `qsc` outside the GUI when protocol state is not ready.

### 3.4 Unlock and init ingress constraints

The only truthful active-operation ingress for this prototype is passphrase-centered:
- passphrase init through stdin;
- passphrase unlock through a child-scoped environment variable;
- passphrase retained only in backend memory for the current app session; and
- no passphrase on the command line, in frontend storage, in passive logs, or in copied diagnostics.

The prototype must continue to preserve:
- the qsc-centered sidecar-shell model;
- memory-only and child-scoped passphrase handling;
- no second persistent secret store; and
- no movement of client-core logic into frontend JavaScript.

### 3.5 Operator-visible messaging requirements

The GUI must keep these truths explicit:
- for keychain-backed vault posture:
  - "Keychain-backed active operations remain deferred in this prototype."
- for protocol activation missing before send/receive:
  - instruct the operator to run `qsc handshake init/poll` outside the GUI before retrying;
- for receive-only posture:
  - explain that receive can be ready while send remains blocked until out-of-band activation completes;
- for unsupported scope:
  - keep attachments UI, transcript-history rendering, multiprofile behavior, and handshake/session-establish UI marked out of scope.

The GUI must not:
- auto-fall back;
- simulate handshake/session behavior;
- infer broader readiness from a keychain vault alone; or
- present inactive peers as send-ready.

### 3.6 Deterministic proof set

This boundary is already supported by merged deterministic proof:
- `qsl/qsl-client/qsc/tests/desktop_gui_contract_na0215b.rs`
  - passphrase-backed profile bootstrap is deterministic;
  - pre-activation send/receive fail closed with `protocol_inactive`;
  - post-activation handshake status becomes peer-specific and truthful;
  - delivery and timeline state remain qsc-owned;
- `qsl/qsl-client/qsc/tests/qsp_protocol_gate.rs`
  - send/receive refuse deterministically when protocol state is inactive;
- `qsl/qsl-client/qsc-desktop/src-tauri/src/qsc.rs`
  - protocol-summary parsing and `protocol_inactive` detail mapping remain stable for the GUI bridge;
- `qsl/qsl-client/qsc-desktop/README.md`
  - operator-facing packaging and limitation wording now matches the frozen boundary.

## 4. Successor decision

Result:
- `BF1`

Why:
- the current merged desktop prototype already behaves inside the frozen active-ops boundary;
- the only load-bearing gap found in this lane was documentary drift, not a smaller remaining implementation defect; and
- after freezing this boundary, the next truthful blocker is merged-lane validation/cleanup rather than direct implementation finalization.

Truthful successor:
- `NA-0215BA — Desktop GUI Prototype Validation + Cleanup`

## 5. References

- `NEXT_ACTIONS.md`
- `TRACEABILITY.md`
- `DECISIONS.md`
- `docs/design/DOC-QSC-008_Desktop_GUI_Architecture_Decision_v0.1.0_DRAFT.md`
- `docs/design/DOC-QSC-009_Desktop_GUI_Prototype_Boundary_v0.1.0_DRAFT.md`
- `qsl/qsl-client/qsc-desktop/README.md`
- `qsl/qsl-client/qsc-desktop/src/main.js`
- `qsl/qsl-client/qsc-desktop/src-tauri/src/main.rs`
- `qsl/qsl-client/qsc-desktop/src-tauri/src/model.rs`
- `qsl/qsl-client/qsc-desktop/src-tauri/src/qsc.rs`
- `qsl/qsl-client/qsc/tests/desktop_gui_contract_na0215b.rs`
- `qsl/qsl-client/qsc/tests/qsp_protocol_gate.rs`
