# NA-0755 — AS BUILT — INVITE LANE A: THE CREATE FLOW

**Lane** NA-0755 · **Decision** D-1397 (this repo) / D-0036 (qsl-desktop) · **Ruled at** `R380`
**Bases** qsl-desktop `142c1eb62892949aef3fd34dc538782490702ba7` · qsl-protocol `d0db5a1755654a8b835c989398937fd5edb04f33`
Both re-derived bare and unpiped at the NAMED github remotes (WF-0088); open-PR sets measured **0 / 0**, each proven NON-VACUOUS by a positive control that returned rows.

## 1. WHAT SHIPPED

The GUI's invite CREATE flow — mockup-14 states 1-2 as a modal over the main screen, reachable
from the un-stubbed welcome button AND from a new "+" in the Chats list header. Create, Copy,
New code, Revoke. Redeeming is Lane B; the open-invites list and the approval gate are Lane C.

**Three files of product code, and none of them Rust:** `ui/index.html`, `ui/main.js`,
`ui/style.css`. Plus one harness scenario, one three-line driver wrapper, twelve source seals
and the inventory pin.

## 2. THE ENUMERATION SHRANK — ITEM 4 STRUCK

The brief's item 4 authorized desktop wrappers for the invite verbs *"ONLY IF S2(b) measures a
needed invite wrapper missing"*. Measured at base:

- `generate_handler` census: **40** `commands::` entries — **exactly** the brief's expected figure.
- All six facade invite verbs already registered by NA-0751: `invite_list`, `invite_create`,
  `invite_redeem`, `invite_accept`, `invite_finish`, `invite_revoke`. (`invite_list_at` is
  deliberately absent — a clock-injection seam.)

⇒ `R380` §1 STRUCK item 4. **No `.rs` product file is touched at all.**

## 3. THE CLIPBOARD, MEASURED IN THE REAL WEBVIEW

Instrument: the harness's own `exec` op (WebDriver `execute/sync`) driving the shipped base
binary from a **scratchpad-only** scenario — no repo byte touched. Platform: this build box,
Linux/X11, `webkit2gtk-driver 2.52.3`, UA `AppleWebKit/605.1.15 Version/60.5`.

| probe | measured |
|---|---|
| `location.origin` | `tauri://localhost` |
| `window.isSecureContext` | **true** |
| `typeof navigator.clipboard.writeText` | **function** |
| `writeText()` under a real WebDriver click | ⛳ **promise RESOLVED** |
| `navigator.clipboard.readText()` | ❌ **REJECTED** — `NotAllowedError` |
| `document.execCommand('copy')` **with a valid selection** | ⚠ **false** — the legacy route is dead here |
| selection machinery | ✅ works (`"QSLI-1-SELECTABLE"`, rangeCount 1) |

⇒ Copy uses `navigator.clipboard.writeText` at capability `core:default`. **No plugin, no
`Cargo.toml` byte, no capability edit.** The bank's select-the-code fallback never engaged
(its precondition measured FALSE) and the clipboard candidate is **NOT filed**.

⚠ **SEAL CONSTRAINT, recorded in the seal's own doc:** because `readText` is denied, an
automated check may assert **that `writeText` resolved** — never that the clipboard holds the
code. The paste is the operator's acceptance card.

## 4. THE Z2 PREMISE MEASURED FALSE

`invite_post` routes every send failure through `relay_send_outcome_from_parts`
(`transport/mod.rs:2228`), which returns **the caller's own fallback** unless the error is a
certificate refusal. The caller's fallback is `INVITE_CREATE_FAILED` → `RelayRejected` → wire
`relay_rejected`.

**Driven live** against `https://relay.invalid.test`: `{"code":"relay_rejected","detail":null}`
in **4112 ms**.

⇒ Unreachable and refused are **indistinguishable**. The relay pane's "Couldn't reach the
relay" copy cannot be reused honestly. `R380` §2 ruled option (A) — one sentence naming BOTH
provenances. Option (B), a composed diagnosis via `relay_test`, was **offered and refused**,
with the hazard recorded: that command now PERSISTS on green (NA-0754), so any future
composition must use the pure `relay_probe`.

## 5. THE COPY: SIX DIVERGENCES BETWEEN THE BRIEF AND THE MOCKUP

Authority: `docs/mockups/mockup-14-invite-create.html` at `142c1eb6`, blob
`338042f03120e28f7c9ca5273867cd15903ae324`. Extracted from **those bytes**, per the brief's own
closing NOTE, and diffed mechanically against the brief's S6.

| Δ | brief S6 | mockup (ruled) | disposition |
|---|---|---|---|
| 1 | "share it **only with the person you mean**" | "share it **through a channel you trust**" | mockup |
| 2 | *(absent)* | the "Who is this invite for?" field + hint | **DEFERRED to Lane C** |
| 3 | "**confirm the connection before anything else happens**" | "**verify fingerprints before the conversation is trusted**" | mockup |
| 4 | *(absent)* | the meta line (Note chip · expiry · acceptance) | **two thirds ship**; Note chip waits with Δ2 |
| 5 | `.muted` + "**Treat the code like a house key**…" | `.warn` + "**Share it through a channel you trust — a text, a call, or in person.**…" | mockup; class → `.callout` |
| 6 | inline `style="margin-top:16px"` | no inline style | mockup |

⚠ **Δ2 is homeless twice over, measured from two directions:** nothing stores a per-invite note
(`AppSettings` = `{autolock_minutes, self_alias, relay_url}`; `InviteRecord` has no such field
and `qsc` is frozen), **and** the only free-text parameter on the create path is `self_label`,
which is the SENDER's identity — driven live with `selfLabel:'Dana'` it returns
`identity_self_ambiguous`, i.e. it **fails closed**. Wiring the mockup's field to it would break
every create.

⚠ The mockup's `.warn` was **not** adopted: `style.css:277` records the ruling that renamed it to
`.callout` because *"a class named `warn` rendering in accent is a lie the next reader has to
decode"*.

## 6. STRUCTURE, NOT DISCIPLINE

**The one-time boundary.** The overlay is deliberately NOT a `SCREENS` member (it must float
over the main screen without becoming a navigation destination), so the screen loop cannot hide
it. `show()` closes it — one line beside `clearCeremonyState()` — covering all **eight** call
sites including the autolock at `main.js:232`. Without it, an autolock with the modal open would
leave a live one-time code rendered over the unlock screen.

**Revoke exists only by composition.** `invite_create` returns the **code**; `invite_revoke`
needs the **invite_id**; no command returns the id of the invite just minted. The id comes from
an `invite_list` snapshot diff across the mint — NA-0751's own recorded pattern. That same call
returns the **real** expiry, which is why the meta line reports what the invite CARRIES rather
than the 72 h requested: `resolve_expiry` clamps to the relay's advertised ceiling and subtracts
a 300 s skew margin, and a clamp is a NORMAL outcome.

**The code box wraps.** An invite code is `QSLI-1-` + base64url(76 + len(relay_ep)) =
**133–154 characters**, about twice the mockup's 73-char placeholder. `.verify-code` is
`nowrap` + `overflow: hidden` with `fitCode()` — the pair whose own comment records NA-0753's
silent clip. `.code-box` follows `.fingerprint` instead and re-enables the selection that
`body { user-select: none }` otherwise denies.

## 7. MEASUREMENTS

| measurement | value |
|---|---|
| GUI baseline at base, reproduced to COMPLETION **before any edit** | **10 passed / 0 failed**, **396 steps**, 580.62 s |
| baseline decomposition (from the runner's own terminal rows) | 96+20+28+25+52+21+27+27+60+40 = **396** |
| ⚠ carried figures corrected | `g` = **27**, `h` = **27** (not 26 / 28); the total was unaffected |
| new scenario `na0755_gui_k_invite_create` | **49 steps**, PASS |
| GUI suite after | **11 passed / 0 failed**, **445 steps** |
| full desktop suite (non-ignored) | **146 passed / 0 failed** |
| inventory | **145 → 158**; gate printed 13 ADDED at **rc 0** |
| counterfactual controls on the new seals | **10**, every one RED on breakage |

⛳ **The inventory gate's asymmetry was RE-MEASURED at this base rather than inherited:**
`test_inventory.sh:96-99` prints ADDED and does not exit; only `MISSING` exits 1.

## 8. WHAT THE HARNESS CANNOT REACH

No fixture relay exists in this repo (`ENG-0226`, OPEN), so **no scenario can reach a successful
mint**. The GREEN half — a code actually rendered, copied, re-minted and revoked — is the
operator's acceptance card, recorded [O]. Every offline-reachable arm IS driven: both entries,
the handler-scoped un-stubbing, the no-relay gate, the create arm against an unreachable relay,
close by button and by Escape.

⚠ The create step polls to 20 s and that number is **measured**, not padded: two failed network
calls (the pre-clamp `relay_server_info`, bounded at `RELAY_SERVER_INFO_TIMEOUT_SECS = 10`, then
the create POST) at 4112 ms driven. ⚠ `relay_http_client()` sets **no** `.timeout(...)` and
`invite_post` sets none either, so the create POST is bounded only by reqwest's own default —
a figure stated nowhere in this tree's bytes.

## 9. INSTRUMENT LESSONS EARNED THIS LANE

1. ⚠⚠ **`git checkout -- <file>` reverts to HEAD, not to "before my last edit".** With nothing
   committed, the first counterfactual control **destroyed all three `ui/` files**. Recovered by
   re-applying; the controls were then re-run with `cp`/restore, and the work was committed
   before any further tamper. **A tamper harness must never use the VCS as its undo.**
2. ⚠ **A control proves nothing until you check WHICH occurrence it hit.** One control replaced
   the *first* `class="callout"` in the file — a pre-existing one in the vault wizard — and the
   seal correctly PASSED. Re-aimed at the modal's own bytes, it went RED.
3. ⚠ **Build the needle from the target line's bytes.** One control's needle dropped a leading
   dot (`"status-banner"` vs `.status-banner"`) and silently never ran.
4. ⚠ **`read_text` reads RENDERED text and applies `text-transform`.** `.tag` came back
   `INVITE — STEP 1`. Rendered text pins what the user sees; the mockup-verbatim STRING is
   pinned in `design_polish.rs` instead.
5. ⚠ **Documenting a removal re-plants it** — the `.verify-code` seal fired on the modal's own
   comment explaining why that class is NOT used. The seal was testing a MENTION; rebuilt to
   test the attribute (a USAGE). The comment stayed.
6. ⚠ **`QSLD_GUI_RUN_ROOT` makes re-runs non-hermetic.** The default run root is UTC-stamped per
   run; the env override is not, so re-running one scenario reuses its profile — a leftover vault
   made the app launch to Unlock and the launch liveness check read empty.
7. ⚠ **A prebuilt binary can predate the tree it appears to prove.** The prior seat's binary sat
   on a clean tree at the right commit, and the merge tree DID equal its branch head's tree — but
   the binary was built **16 minutes before** that branch head existed. Rebuilt from this lane's
   own seat.

## 10. CLAIM BOUNDARY

No `qsc`/protocol source byte, no harness engine byte, no mockup byte, no `.github/**`, no
`Cargo.toml`/`Cargo.lock`. **Two measured needs in `qsc` were FILED, never patched** —
`ENG-0228` and `ENG-0229` — per the `ENG-0218` precedent. No test weakened, skipped or deleted.
The clipboard measurement is **one platform**; macOS and Windows are unmeasured. Nothing was
merged by the seat.
