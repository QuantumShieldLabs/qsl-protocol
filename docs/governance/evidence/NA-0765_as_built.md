# NA-0765 — AS BUILT — The Lane C acceptance repairs

Spine decision **D-1406**; desktop decision **D-0042**. Bases re-derived bare and unpiped at the
NAMED github remotes: protocol `b3ae24b430e516d9b43bb654905c7992beb59ce7`, desktop
`5cee1e16f2551035e3c741b715f847c82288cebf`. The qsc pin is **UNCHANGED** at
`b3ae24b430e516d9b43bb654905c7992beb59ce7` — measured current before any edit, and this lane
needed nothing from it.

⚠ **THIS FILE RECORDS WHAT WAS BUILT, WHAT WAS MEASURED, AND WHAT IT DOES NOT PROVE.** The
reasoning lives in `DECISIONS.md` D-1406 and desktop `DECISIONS.md` D-0042; it is not repeated.

⛳ **THE SEATS HAVE NO `origin`.** Both were cloned and had `origin` REMOVED, leaving only a
remote literally named `github`. The stale-mirror trap cannot fire in this lane by construction
rather than by discipline.

## WHAT SHIPPED

**qsl-protocol** — RECORDS ONLY. Zero product bytes; no path under any goal-lint CORE pattern
is touched, so the docs-only shape is admissible on its own terms.

**qsl-desktop** — `ui/index.html`, `ui/main.js`, `ui/style.css`, plus **two lines of Rust
surface**: one thin command forwarder in the shipped `contact_list` idiom
(`src-tauri/src/commands.rs`) and one `generate_handler!` registration
(`src-tauri/src/lib.rs`). Registered commands **42 → 43**.

  A1  the main rail's Chats button gains `btn-rail-chats-m` and a listener; a `railSelect()`
      helper moves `.active` and is called from BOTH pane functions, so the settings rail gets
      it for free and the shipped listener line a design seal pins byte-exact stays untouched.
  A2  `.content-pane:not(.welcome)` takes `padding: var(--sp-x20) var(--sp-5)` and `overflow:
      auto` — shipped tokens, and scoped so the welcome pane keeps centring in an unpadded box.
  A3  RENAME ships, in the shipped `.pane-form` / `.pane-sect` / `.field-label` / `.ctlrow`
      idiom. BLOCK AND UNBLOCK DO NOT — see the refusal below.
  B1  the Chats "+" and its listener retire together.
  B2  ONE new CSS class is minted, `.contact-code-card`, and it is the only one.
  B3  nothing-selected renders the Welcome panel; ONE welcome element is reused.
  B4  X + Back on the code-entry view and on the invite-creation view. Escape and the scrim
      already worked and are untouched.
  C1  one word for one thing: the retired term occurs **ZERO** times under `ui/`.

## THE REFUSAL, AND WHY IT IS THE HONEST ANSWER

The blessed layout draws Block and an Unblock captioned *"Unblocking restores the connection you
already had."* Measured at this pin, and independently confirmed by the Director from his own
chair: `contacts_block`/`contacts_unblock` exist at `contacts/mod.rs:1541`/`:1551`, are
symmetric and non-destructive, and are **NOT IN THE FACADE** — **0 hits each**, with
`contact_set_display_name` returning **1** as the positive control on the same needle. The one
verb the desktop CAN reach, `contact_request_block`, is one-way and writes
`primary.state = "REVOKED"`, which nothing exposed restores. ⇒ the blessed sentence would be
FALSE, so no blocking control ships in any state, and the ABSENCE is asserted structurally so a
later lane cannot add one silently. `ENG-0248` files the gap.

## WHAT WAS MEASURED ALREADY-BUILT AND NOT RE-BUILT

  B5   `#redeem-code` was ALREADY the app's ONLY `<textarea>`, `min-height: 5.5rem` — **88px of
       box against the blessed layout's 86px**. The ordered repair described a mockup-VERSION
       delta (v1 → v2), not an app defect. No product change; no token bump invented.
  I3   The alias-invariant structural census AND its can-fail proof were already in main
       (`na0764_contacts_surface.rs`), so the ruling's "committed and red-capable BEFORE Rename
       is wired" held by construction. What this lane owed instead was proof the census guards
       the NEW call — control C-7 below.

## THE INSTRUMENTS, AND EVERY RED ARM PROVEN

Exact red sets were committed in writing BEFORE any edit
(`PHASE0_NA0765_redsets_20260827T053830Z.md`, sha256
`f920a77de783b70589245afde0ed9bdc396a8be9cf72d9eb7e879cef9ed3a8ea`, 205 l, 444). **All eight
controls fired, each at the row Phase 0 named.** Every ui/ control ran the REBUILD BRACKET
(edit → build → run → restore → build), because `frontendDist` embeds the ui assets at build
time and a perturbation without the bracket is a silent no-op.

  C-1  delete `railSelect("btn-rail-chats-m")`  → f_n: expected `true,false`, measured
       `false,true` — the highlight failed to move.
  C-2  delete the `btn-rail-chats-m` listener   → f_n: expected `hidden`, measured `shown`.
  C-3  remove the Connection section            → f_n: `count #contact-detail-body
       .contact-detail-kv` expected 1, measured **0** — the BY-ELEMENT row, which is the
       instrument whose absence let the gap ship.
  C-4  collapse the two welcome wordings        → f_n: expected `Add a contact`, measured
       `Add your first contact`.
  C-5  delete the `btn-redeem-back` listener    → f_l: `#choose-view.className` expected `""`,
       measured `"hidden"`.
  C-6  re-introduce the retired term once       → `na0765_the_verification_code_naming_is_singular`
       FAILED, exact red set of one.
  C-7  swap the LABEL into the KEY's argument position in the NEW Rename call →
       `display_name_never_reaches_a_command` FAILED. ⇒ the inherited census is LOAD-BEARING for
       code that did not exist when it was written.
  C-8  re-add a `#btn-invite-open` element      →
       `na0765_the_chats_plus_is_retired_and_the_flow_still_has_two_entries` FAILED.

Every restore was cmp-identical to the commit; the working tree was CLEAN after all eight.

## SUITES

  desktop unit, at the base   **175 passed / 0 failed**, 19 result lines, exit 0
  desktop unit, at the head   **181 passed / 0 failed**, exit 0  (+6: this lane's six seals)
  desktop gui-driver          **14 / 14 passed**, 0 failed, 176.45 s, exit 0
  `cargo fmt --all --check`   exit 0
  test inventory              **190 → 196**, +6 and **ZERO removals** — growth only
  registered commands         **42 → 43**
  product-source files        **5**, which does not exceed five ⇒ **SR-15 does not trigger**

⛳ The f_n run was checked for vacuity rather than trusted: **91 step rows, 90 PASS / 0 FAIL**,
and the harness's own P9 liveness proof fired BOTH arms in that same run (an absent selector
yielded `no such element`; a deliberately-wrong expected text miscompared).

## RE-AIMS — SEVEN, EACH AN EXACT EQUALITY, EACH WITH ITS OWN CAN-FAIL PROOF

`f_k` count `#btn-invite-open` 1 → **0** (exact, per the Z6 precedent) · `f_k` two clicks
retargeted to the carrier that replaced it · `f_l` one click likewise · `design_polish.rs:2559`
the name caption, re-aimed AND strengthened from a bare phrase to its whole element ·
`design_polish.rs:2563` the standing hint · `f_l` `#redeem-name-caption` · `f_l`
`#redeem-name-hint`, **twice**. ⚠ **Three more than STOP 001 predicted**, found by re-censusing
the ACTUAL change list rather than the stop's summary of it. ⛳ And one predicted re-aim was NOT
needed: putting the highlight move inside the pane functions left `design_polish.rs:1963`
byte-identical.

## WHAT THIS DOES NOT PROVE

**HARNESS GREEN IS NOT A FIELD CLAIM.** The desktop still has no fixture relay (`ENG-0226`,
open), so no scenario completes a handshake. Acceptance is the operator's flight: the rail round
trip, a contact detail matching the blessed layout, a rename that survives a restart, the
code-entry close, and one regression pass on the auto-connect loop.

**NA-0764 FLIPS DONE WITH NO CLASS.** No close-out is banked for it and neither the brief nor
the build ruling declares one, so none was invented — the `NA-0759` precedent by name. The class
remains OWED, and `D-1405`'s `**Class:**` field is amended in place to say exactly that.
