# NA-0756 — AS BUILT — INVITE LANE B: THE REDEEM FLOW

Spine decision **D-1398** · desktop decision **D-0037** · ruled at **`R387`**
Bases: qsl-desktop `5eb64c2e371906af1533804a542bdc119661d960` · qsl-protocol `d3fefd12afc3b7c2e783aeee9c32d3c7c5eddf86`

## 1. What shipped

The app's second contact-making act. A user pastes an invite code, names the contact, and
Connects in ONE gesture; sees an honest "Request sent"; and the handshake completes on two
bounded triggers — at vault unlock, and when the add-contact surface opens. No timer, no
poll, no background loop, and none touched: the only standing interval in the app remains the
idle autolock, which a seal COUNTS rather than eyeballs.

**ZERO `.rs` product bytes in either repo.** `invite_redeem` / `invite_accept` /
`invite_finish` were all registered at NA-0751/0755. Four measured needs were FILED, not
patched.

## 2. The measurements

| instrument | baseline | after |
|---|---|---|
| qsl-desktop `cargo test` | 159 passed / 0 failed / 12 ignored | **165 / 0 / 13** |
| qsl-desktop `cargo fmt --check` | clean | clean |
| qsl-desktop `clippy --all-targets -D warnings` | clean | clean |
| qsl-desktop infra literal scan (tree) | clean | clean, 85 files / 29995 lines |
| qsl-desktop test inventory | 171 pinned | **178**, growth only, zero removed |
| qsl-desktop GUI driver | 11 scenarios | **12**, all green |

The `generate_handler` census re-measured **41** (`commands::`-only, NA-0755's instrument) and
**42** (all entries, incl. the unprefixed `ui_surface_changed`). The brief expected **40**,
which reproduces EXACTLY at the parent of the commit that added `invite_clear` — an INSTRUMENT
and a BASE difference, not a defect, and reconciled rather than argued.

## 3. The seals, and every control shown RED

Six source-side seals in `design_polish.rs`, plus the 84-step `f_l_invite_redeem` scenario in
the real webview. Each control's tamper was asserted to match **exactly one** occurrence
before being applied — the NA-0755 lesson that a control hitting the wrong occurrence proves
nothing — and each landed on a **different** assertion line, which is what proves they are
aimed at their own arm's bytes rather than all tripping one shared check.

| control | tamper | result |
|---|---|---|
| C1 | mutate ONE byte of the operator's blessed callout | RED |
| C2 | introduce a Retry into the security-failure state | RED |
| C3 | widen the name predicate to admit a space | RED |
| C4 | remove the residual's `redeem` verb arm | RED |
| C5 | swap the pending predicate's equality for a substring test | RED |
| C6 | put a literal hex into the new warning selector | RED |

Preserved 444 under `/srv/qbuild/operator/NA-0756/redruns/`. Restore verified by
`git status --porcelain` returning **0 lines** against the committed bytes — never against a
memory of them, and never by using the VCS as the tamper harness's undo.

## 4. What is driven, and what is not

**DRIVEN, offline and for real:** the chooser entry and its finish trigger; every copy pin as
RENDERED text; the shared-width geometry across states; the admissibility gate INCLUDING the
"Ben Smith" control that performs the operator-class defect exactly; and a **live**
`Connect` → `invite_redeem` → `malformed` round trip. `malformed` is decided at
`invite/mod.rs:435-442`, BEFORE any socket opens, so it needs no relay — and it is the
likeliest real failure in the flow, because it is what a truncated paste produces.

**DRIVEN in-process at the facade** (`na0756_two_party_invite_roundtrip.rs`, against
qsl-server's real router with no mocks): X1–X4 end to end; **both security tells firing** for
the first time in this suite's history; and the **not-yet** outcome, also a first.

**NOT DRIVEN, stated so silence is not read as coverage:** a successful redeem from the GUI,
the "Request sent" state, every relay-reported arm, and a finish that completes — all need a
relay the desktop harness does not have (`ENG-0226`, open). The GREEN half is the operator's
acceptance card, recorded [O].

⚠ **THE SECURITY TELLS ARE A SPLIT CLAIM, DELIBERATELY.** The desktop scenario drives the
STATE'S RENDERING — its copy, its warning accent proven distinct from danger, the ABSENCE of
any Retry. The ENGINE path fires in the protocol-side facade drive. Neither half is dressed up
as the other.

## 5. What went wrong, kept for the next lane

1. **The 444 trap fired on my own tamper control.** `cp` carried the source's read-only mode,
   the appending tamper was refused, and `cmp` returned 0 having proved nothing. Reported, then
   redone with the arm made writable.
2. **STOP 001 was true in every line and still failed its job** — it omitted the shipped copy
   vocabulary the ruling needed. Re-assembled, not supplemented.
3. **A negative control REFUSED a prediction and that is the control working.** `ENG-0232`
   measured PRESENT; the space runs to 0234, and an adopted id would have collided.
4. **Running found a consumer the sweep did not.** The screen-transition seal went RED because
   its slice was a fixed 1400-byte window, not because its property broke.
5. **The new scenario was red on its first run** — one pin used a curly apostrophe where the JS
   ships ASCII. The red isolated the needle exactly; everything else had already driven.
6. **`gh pr list`'s last column is `createdAt`, not the merge time.** STOP 002 §1 carried the
   wrong timestamps; merged-ness and the bases were independently correct. Use the TYPED field.
7. **I nearly reported a lane-blocker that was not one** — no facade verb sets the relay inbox
   token, but `vault/mod.rs:575-579` means every vault is BORN with one. Kept measuring instead
   of designing.

## 6. Filings

`ENG-0235` the comment-blind scanner (discharging NA-0755's owed filing — and its **fourth**
occurrence was found in this very lane, in the red-when-green direction) · `ENG-0236` the alias
reaching its gate after the burn · `ENG-0237` `RelayCaFile`'s doc falsified against the pinned
bytes · `ENG-0238` a reachable `Other` payload breaching its own documented shape seal.
`ENG-0239` an in-process `invite_finish` failing its session store where a SUBPROCESS one
succeeds on the same config dir, same relay and same vault — driven both ways with the process
shape isolated as the only variable — with the typed `ErrorCode` discarded by a `map_err` at
`handshake/mod.rs:1929`/`:2156`, which is why the cause could not be isolated further.

All five ids DERIVED at the edit with both controls. None patched — `qsc` is frozen.

## 7. ⚠⚠ THE OPEN QUESTION THIS LANE HANDS FORWARD

**`ENG-0239` may bound this lane's headline, and no harness here can settle it.** If the
desktop's in-process `invite_finish` fails the same way, the GUI's finish trigger cannot
complete a handshake, and what shipped is "the request is sent" rather than "the connection
completes automatically" — which is a sentence the UI currently makes to the user. The
desktop unlocks in-process exactly as the failing test does, but against a REAL vault whose
store key can be written, so it may well succeed. Neither harness can decide it: the
desktop's cannot reach a relay (`ENG-0226`), and this one has no real vault.

**The operator's acceptance flight is the instrument that answers it.** That is why the card
is the gate and not a formality.
