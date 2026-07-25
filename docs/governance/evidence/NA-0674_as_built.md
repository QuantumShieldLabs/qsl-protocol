# NA-0674 as-built — Server pane redesign (D610)

Lane: NA-0674 · Directive: **QSL-DIR-2026-07-25-610 (D610, APPROVED 2026-07-25,
sha256 `6b8e8ac11d9375e53b8362335b812ce68fa4419f9655c16593392bd60a3516ed`, 807
lines)** · Spine decisions **D-1304** (impl-evidence) + **D-1305** (closeout) ·
qsl-desktop decisions **D-0010** (the redesign) + **D-0011** (the flight fixes).

Formalized from the operator-approved lane intent
(`/home/victor/work/LANE_INTENT_server_pane_redesign.md`, sha256
`a3113bae67e4e9e1473c756720753773a3e5ab089075ead85a46a3c30addc42d`, 326 lines),
whose rulings R-A1…R-F4 were carried verbatim into D610.

---

## §1 — What shipped

**The lane REVERSES `[F.1-COMMIT]`**, a design ruling recorded one lane earlier in
qsl-desktop D-0008 and reasoned in D-0009.

**The trap it removes.** The probe reads the bearer token FROM THE VAULT. Under
the split commit model the token committed through its own "Set token" button,
so a user who typed a new token and pressed **Test** — the obvious gesture, and
the one the layout invited — got a result computed against the OLD token. The
pane then reported that result **truthfully**, which is what made it dangerous:
"Token rejected" for a token the user believed they had just replaced is
indistinguishable, on screen, from a genuinely bad token. The model could have
been patched with a warning ("press Set token before testing"). **Removing the
trap beats warning about it** — a warning puts the burden on the user to
remember an ordering the interface itself created.

**"Secrets to the vault, URL to settings" is UNCHANGED.** Only the user-facing
commit surface unified.

| PR | decision | merge | CI |
|---|---|---|---|
| qsl-desktop **#10** | D-0010 | `a836eaf` | `[rust]` green |
| qsl-desktop **#11** | D-0011 | `b4ea47e7` | `[rust]` green |
| spine (this) | D-1304 + D-1305 | — | docs_only |

- ONE unified **Save** commits everything through the EXISTING trios (R-A1)
- **Test saves first** on a dirty pane, then probes the just-saved state (R-A2);
  Save stays independently clickable (R-A3)
- the four per-field **Set/Clear buttons REMOVED**, replaced by per-field
  *"remove it"* prose links; removal is PENDING until the next commit and is
  cancelled by typing (R-B3, R-E3)
- **three sections, exactly two hairlines** at `var(--sp-6)` (R-D1, F2R)
- results **state 8 REMOVED** into a dirty helper (R-F1); **state 14 ADDED**
  (R-F2); state 10's trigger broadened (F1R); states 1-7, 9, 11-13 unchanged in
  trigger and wording (R-F4)
- Appendix F revised **with superseded passages MARKED and left legible**
  (175 → 404 lines)
- frozen needles moved in LOCKSTEP with NEGATIVE pins (C5)
- mockups committed SANITIZED under `docs/mockups/` (F3R)

**Scope held:** UI + tests + docs only. No `src-tauri/src` change, no new backend
command, no qsc API change, **zero dependency motion**, no colour or token
change. `design_round2.rs`, `design_system.rs`, `design_round3.rs` byte-identical
by `cmp`.

---

## §2 — The census: six corrections to the intent's own text

VERIFIED READ-ONLY at drafting against qsl-desktop `ec7e53e` / spine `ac8fe5a4`.
Verified state replaces asserted state.

- **C1 — ENG-0072's stated premise is FALSE.** The ledger recorded it as an
  asymmetry ("the spine seat is set to GH007, the desktop seat is NOT — that
  asymmetry IS the finding"). Measured: `new_checkout.sh` was the only checkout
  creator and set NO identity for ANY repo; no `/etc/gitconfig`, no `includeIf`.
  The seats that read GH007 were exactly the ones an executor had fixed by hand.
  A UNIVERSAL tooling gap masked by executor discipline. See §4.
- **C2 — "validate everything first" is not fully possible.** No validate-only
  command exists. `relay_ca_file_set` validates BY writing. ⚠ **C2 also asserted
  the URL could be validated without writing — that was wrong too**, discovered
  at implementation: `relay_config_set` runs `normalize_relay_endpoint` AND
  writes in one call. See §5 (the R-B1 amendment).
- **C3 — the mockups draw a CA path the app cannot know.**
  `relay_ca_file_show()` returns `{configured, path_hash}`; the path is never
  returned. The input renders EMPTY; the status line carries the state.
- **C4 — R-D3's summary wording governs**, not the mockups' "(optional)", which
  predates the relabel.
- **C5 — G3 named the wrong pin file.** `design_round3.rs` has ZERO server-pane
  coupling. The Server pane's frozen needles live in
  `src-tauri/tests/server_pane.rs`, whose lines 25-28 pinned the four buttons
  this lane removes — it would have FAILED the instant the redesign landed.
- **C6 — R-E6's "may" promoted to "does":** a Test that commits must say so.

**Three operator-ruled flags (2026-07-25), each folded into the rule it
modifies:** **F1R** *remove it* also clears the results block, broadening state
10's trigger to "any change to what the app will use" · **F2R** hairline padding
`--sp-6` (32px); the mockup's 30px sat exactly between `--sp-x28` and `--sp-6`
and no new token was added · **F3R** the mockups commit SANITIZED — `qsl-desktop`
is PUBLIC and the captured markup used a private host as its illustrative
example.

---

## §3 — Result classes

| class | result |
|---|---|
| `SEAT_IDENTITY_FIX_PASS` | ✅ PASS |
| `SERVER_PANE_REDESIGN_PANE_PASS` | ✅ PASS |
| **`SERVER_PANE_REDESIGN_PASS`** | ✅ **PASS** |

---

## §4 — PHASE 1: ENG-0072 closed, proved by positive control

Two touch points, because there are two ways a seat comes to exist:
`new_checkout.sh` (creation, covering direct invocations too) and `qwork.sh`
(reuse — the seats already on disk, which `new_checkout.sh` never revisits).
Fixing only creation would have left most existing seats wrong. Tools commit
`4235786d` (`/srv/qbuild/tools`, a local repo, no remote, no PR).

**A negative result is only evidence if the instrument could have returned
positive** (the NA-0668 standing method), so it was made to return positive
first. Evidence: `/srv/qbuild/evidence/NA-0674/eng0072_positive_control.txt`.

| | reading |
|---|---|
| BEFORE — both seats `qwork NA-0674` had just created | `(unset)` → inheriting `tebbens@proton.me`. **Both** — including the SPINE seat the filing claimed was correct |
| AFTER — a freshly created checkout | GH007 |
| AFTER — the same two existing seats, re-seated (`created_or_existing=existing`) | GH007 |

Every commit in this lane took its identity from tooling rather than from
someone remembering. `~/.gitconfig` was deliberately NOT touched — that identity
serves repositories outside this project.

---

## §5 — The R-B1 amendment (Director ruling, 2026-07-25)

C2's premise was false for the URL as well as the CA: the app registers nine
relay commands and **none is validate-only**. That put two rulings in direct
conflict — **R-B1** (vault writes first, `settings.json` last) versus **R-B2** (a
malformed address blocks the ENTIRE commit with NOTHING persisted, on Save AND
on Test). Honouring R-B1's order lands the vault writes before the bad address
is rejected; honouring R-B2 forces the address to commit first.

> **RULED: R-B2's guarantee GOVERNS. R-B1's vault-first ordering is AMENDED to
> address-first.** Rationale as given: *an absolute stated guarantee — nothing
> persists on validation failure — outranks unexplained write ordering; and
> partial-commit-on-vault-failure is acceptable because state 14 reports it
> honestly and a re-test heals it.*

**The accepted cost, stated rather than hidden:** if a vault write fails, the
address has already been saved. State 14 names the failed part, the remainder is
abandoned, the probe does not run, and the pane re-reads live state so it never
describes state a partial commit already changed. **The healing path** is a
re-test: fix the failing field, press Test again, and the commit completes from
where it stopped — demonstrated live (§6, shots 11-11-16 → 11-17-20).

**D610's C2 text was NOT rewritten.** The directive is sha-pinned in the spine
queue block, so amending it in place would break that pin and quietly rewrite an
approved document — the same mark-don't-rewrite discipline this lane applies to
Appendix F. C2 stands as approved and is superseded in qsl-desktop D-0010 and
here.

---

## §6 — THE LIVE ACCEPTANCE FLIGHT — FLOWN AND PASSED

Operator-flown 2026-07-25 against the tserver rig (the operator's LAN host —
its address is in the operator-side runsheet and is deliberately NOT restated in
this public repository; qsl-server behind a user Caddy `tls internal`) **over real TLS. No mocks (§7.4).**
The build host cannot drive the GUI (xdotool absent) — the executor enumerated
the shots, the operator flew them.

**25 screenshots** in `/srv/qbuild/evidence/NA-0674/flight/`; the full per-state
record with verdicts and timestamps is
`/srv/qbuild/evidence/NA-0674/flight_results.md`. Build provenance (the flown
binary proven tree-identical to merged main) is `flight_build_provenance.txt`.

### Coverage — the full Appendix F.2 enumeration

| state | verdict | evidence (CDT) |
|---|---|---|
| 1 Reachable / Bearer | ✅ | 10-26-33, 10-39-32 |
| 2 Reachable / Open | ✅ | 12-08-33 |
| 3 Certificate not trusted | ✅ | 12-02-54 |
| 4 Couldn't reach the server | ✅ | 11-57-32 |
| 5 Token rejected | ✅ | 10-34-25 |
| 6 This relay requires an access token | ✅ | 10-49-08 |
| 7 Not a QSL relay | ✅ | 12-04-16 |
| 9 Idle / never-tested | ✅ | every pane open |
| 10 Cleared by a change | ✅ | 10-31-29 (field edit) · 10-45-10 (*remove it* — F1R) |
| 11 Bad address — inline, no card | ✅ | 11-58-34 |
| 12 CA file unreadable | ✅ | 10-59-47 |
| 13 Couldn't start the connection test | ⚠ **NOT EXERCISED** | no reachable trigger found |
| 14 Couldn't save settings | ✅ (fault-injected) | 11-11-16 |
| 8 | correctly ABSENT (R-F1) | — |

**All SEVEN probe outcomes exercised.**

### Rulings confirmed live

- **R-A2 (the reason the lane exists):** a wrong token typed then tested returned
  **"Token rejected"**. The pre-lane build would have probed the STORED token and
  said "Connected". *(10-31-29 → 10-34-25.)*
- **R-E1:** **8 placeholder dots for a 64-character token** — length not leaked.
  `relay_token_show` is a bare bool; the app does not know the length and does
  not appear to. *(10-39-32.)*
- **C6:** *"Settings saved."* present on the committed Test, ABSENT on the clean
  one — real feedback, not decoration. *(10-34-25 vs 10-39-32.)*
- **R-B2:** proven three ways in one frame — inline error, no results card, and
  `settings.json` unchanged **on disk**. *(11-58-34.)*
- **R2b under compound failure:** with a dead port AND an unreadable CA, the pane
  reported the **CA** problem, not "couldn't reach" and not "certificate not
  trusted" — the client build fails before any request is formed. *(10-59-47.)*
- Also confirmed: R-A3, R-B3, R-B5, R-B6, R-C1, R-E2/E3/E4/E5, R-D1..D4, R-F4.

### State 14 required DELIBERATE FAULT INJECTION

With the commit path's guards **no user input can fail a commit step** — empty
token and empty CA are skipped rather than submitted, and a bad address routes
to inline state 11. State 14 is reachable only via a vault or disk fault.
Recorded in full at `/srv/qbuild/evidence/NA-0674/fault_injection_state14.txt`,
including a first attempt that FAILED and why:

> `chmod 0555` on the vault directory did not hold — **qsc HARDENS its own config
> directory to 0700 immediately before every vault write** (`vault/mod.rs:852`).
> Recorded as a POSITIVE observation about the crate. The fault that worked was a
> *directory* at the temp path, which `fs::remove_file` cannot clear.

The state-14 card was corroborated **on disk**: `settings.json` held the new
address while `vault.qsv`'s mtime predated the attempt — "Everything before it
was saved, and nothing after it was" was true in both directions.

### Verification method

**All 25 shots were read by the executor and compared to the EXACT shipped
copy** (verification pass, 2026-07-25). Nothing in §6 rests on eyeballing or on
live verbal confirmation alone. ⚠ That pass CORRECTED one entry: state 12's
evidence was first recorded as 10-56-13, which in fact shows a clean post-Save
pane with no results card; the real frame is 10-59-47.

---

## §7 — ⚠ THE FLIGHT FOUND THREE DEFECTS THAT CI COULD NOT

All three in this lane's own D-0010 code. All three fixed in **D-0011 / PR #11**.
**None was caught by the 70 passing tests.**

| | defect | severity | why the suite missed it |
|---|---|---|---|
| a | the dirty helper claimed "not saved" about settings that WERE saved | P2 | invisible unless the typed address differs from its normalized form (`https://192` → `https://0.0.0.192`, WHATWG IPv4 shorthand) |
| b | a stale **"Testing…"** persisted under the inline address error — the pane asserting a test was running when none had been attempted | P2 | needs a slow probe still holding the SERIAL GATE when the next action starts: real network latency plus an impatient human |
| c | state 14 opened with a raw `vault_write_failed` | P3 | only reachable under fault injection |

**(b) was a reasoning error, not a typo, and it is the one that generalises.**
C2(b) requires re-reading live state after a **PARTIAL** commit, because
something landed. R-B2 guarantees a validation failure persists **NOTHING** — so
that branch has nothing to re-read. Applying the obligation to a branch it does
not cover is what put a **gated** call (`relay_token_show` /
`relay_ca_file_show`, both on the process-wide serial blocking gate) in front of
the panel clear.

**No design ruling changed.** Appendix F already specified the correct behaviour
in all three cases; these were deviations from it, so the spec needed no
revision.

**Both (a) and (b) were RE-FLOWN and confirmed fixed** (11-57-15, 11-58-34). The
(b) frame also finally flew check 8 properly — the earlier attempt had used a
valid address.

**The three regression pins were verified as a POSITIVE CONTROL:** run against
the merged, defective `main`, **exactly the three fail and the other ten pass**;
against the fix, all thirteen pass. A pin that also passes on the buggy code
documents nothing. Tests 70 → 73.

**This is the lane's most transferable result.** CI green was never the
acceptance, and this flight is the evidence for that claim rather than a
restatement of the principle.

---

## §8 — Filings and records

- **ENG-0072 CLOSED** — with its stated premise **MARKED as superseded by
  measurement, not silently rewritten** (§2 C1, §4).
- **ENG-0074 FILED** — the ENG-0072 fix is now load-bearing and nothing observes
  it. Implementation deferred by Director ruling to review Lane C or the next
  spine tooling touch.
- **ENG-0073** — discharged by construction: the two identically-labelled "Clear"
  buttons it describes no longer exist.
- **OBS-I → review Lane C** (Director): desktop CI runs clippy without
  `--all-targets`, hiding 5 pre-existing errors. Folded into Lane C's scope
  alongside the safety scan and cargo-audit; **the 5 errors are to be fixed
  there**.
- **OBS-J → ENG-0064 reproduced again** on this lane's two-repo seat
  (`CARGO_TARGET_DIR` leaking between repos). Candidate for the same tooling
  touch that implements ENG-0074.
- **OBS-M → lane-3 intent**: state 13 has no reachable trigger. Correct
  defensive code, but if a future lane cannot construct one either it may be
  dead code rather than a state — worth a look when messaging builds the client
  on more paths.
- **Rig reachability correction ACCEPTED** (Director): the build host **CAN**
  reach the rig's LAN address; the "AP isolation" warning is stale and the
  standing notes will be corrected at next refresh.

---

## §9 — What was NOT done / remains owed

- **State 13 NOT EXERCISED** (§6).
- **ENG-0051** — messaging push/pull/ack: untouched, the later lane.
- **ENG-0058** — the Logs pane; the rail toggle (round-4b); onboarding; contacts.
- The intent's §5 lane-3 notes (L3-1…L3-4) are carried into the messaging lane's
  intent, NOT executed here.
- The `settings.rs` `deny_unknown_fields` downgrade property remains knowingly
  untouched (pre-existing class, D609 R6).

No public / production / crypto-complete / bug-free claim is made.
