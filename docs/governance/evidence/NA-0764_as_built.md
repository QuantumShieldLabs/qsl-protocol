# NA-0764 — AS BUILT — Lane C: Contacts, auto-connect, and the notice footer

Spine decision **D-1405**; desktop decision **D-0041**. Bases re-derived bare and unpiped at the
NAMED github remotes: protocol `b0d1ac7eb00b4dc84f3f80ece36f3e3664fb7bfc`, desktop
`e48dfed019e3210164a236985edf81cad4175540`, qsc pin `0b9d6967948c2fcf799cb817aeee55d5095835aa`.

⚠ **THIS FILE RECORDS WHAT WAS BUILT AND WHAT IT DOES NOT PROVE.** The reasoning lives in
`DECISIONS.md` D-1405; it is not repeated here.

## WHAT SHIPPED

**qsl-protocol** — `display_name` on `ContactRecord` (`#[serde(default)]`, beside the alias KEY,
never re-keying it) with a `pub(super)` setter using this module's own `Result<bool, ErrorCode>`
idiom; `ContactSummary` widened by `display_name` and `device_count` (a PROJECTION of
`devices.len()`, never the array); `facade::contact_set_display_name`, mapping the engine's
`false` onto the existing `FacadeError::NotFound` — **zero new enum variants**.

**qsl-desktop** — the Contacts pane (a PEER of Chats, reached from the rail), the six-state
mapping in ruled precedence (blocked ≻ CHANGED ≻ verify-badge ≻ Active), the state-dependent
detail with the **ratified 30-digit tier-1 code in 6×5 groups**, the auto-connect scan class on
the existing tick, the notice footer with **reason-first precedence**, and the in-memory
verify-first badge that clears on open.

## INSTRUMENTS

    qsl-protocol  na0764_m3_empty_slot_accept              2 tests   REAL qsl-server in-process
                  na0764_contact_surface_widening          5 tests
    qsl-desktop   na0764_contacts_surface                  6 tests   structural
                  f_n_contacts_autoconnect                78 steps   gui-driver
    re-aimed      design_polish stub seal (2 -> 0), f_m rows (3), f_h precondition, f_k note,
                  server_pane justification

Inventory 183 → 190, delta exactly **−1 / +8**; the −1 is a deliberate, reviewed RENAME, and the
checker was observed failing on it BY NAME before being re-pinned.

## RED ARMS — EVERY ARM RUN, OBSERVED SET BESIDE THE PREDICTION

    ARM                                      PREDICTED           OBSERVED                VERDICT
    drop #[serde(default)]                    {legacy blob}       {} NOTHING RED          CLAIM FALSE
    skip_serializing_if (the real arm)        {legacy blob}       {legacy blob}           EXACT
    setter re-keys the store                  {rename}            {rename}                EXACT
    device_count hard-coded                   {projection}        {projection}            EXACT
    add invite_id to the summary              {allowlist}         {allowlist, projection} WIDER
    the pane ships (stub seal)                {stub seal}         {stub seal}             EXACT

⚠ The `#[serde(default)]` arm was CLAIMED and did not hold: serde already defaults a missing
`Option<T>` to `None`. Corrected in the open by its own author. ⛳ The `invite_id` arm reddened
WIDER than predicted, and that is defence in depth — the allowlist catches the DECLARATION, the
projection test catches the RENDERING — recorded as a delta rather than smoothed away.

## WHAT THIS DOES NOT PROVE

* **HARNESS GREEN IS NOT A FIELD CLAIM.** This repo has **no fixture relay** (`ENG-0226`, open),
  so **no desktop scenario completes a handshake**. `f_n` drives every DECISION the front end
  makes, with `invoke` stubbed and the recorded calls as the instrument.
* The relay-facing half is measured **only** in qsl-protocol, against the real `qsl-server` on
  loopback — and that says nothing about NAT, partitions, or two physical machines.
* **The loop end to end is proven only by the operator's two-machine flight** (`E1` as rewritten
  by `R7`), plus the badge seen and one revoked-invite negative.
* `R3`'s Devices line and the `ContactDto` widening are **NOT in the desktop PR at STOP 2**: they
  cannot compile until the qsc pin moves, and ride the named pending edit per the P1
  choreography.
* The unlabelled-invite gap is **skipped and tallied, not solved**; its disposition is the
  operator's.
