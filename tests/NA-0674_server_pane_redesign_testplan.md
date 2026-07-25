# NA-0674 Test Plan — Server pane redesign (D610)

## Status
- **Automated coverage (§A): DONE** — merged and CI-green (qsl-desktop #10, #11).
- **Live GUI acceptance (§B): FLOWN AND PASSED** (operator-flown, 2026-07-25).
  All seven probe outcomes plus states 9, 10, 11, 12 and 14 exercised against the
  tserver rig over real TLS — no mocks. ⚠ **State 13 NOT EXERCISED** (no reachable
  trigger). ⚠ **The flight found THREE defects** the automated suite did not; all
  three were fixed in D-0011 / PR #11 and the two user-visible ones were
  re-flown. Per-state results, verdicts and screenshot timestamps are in
  `docs/governance/evidence/NA-0674_as_built.md §6-§7` and
  `/srv/qbuild/evidence/NA-0674/flight_results.md` (25 PNGs in
  `/srv/qbuild/evidence/NA-0674/flight/`). ⟹ `SERVER_PANE_REDESIGN_PASS` ASSERTED.
  §B below is retained as the instrument that was flown.

## §A — Automated coverage (in-suite, socket-free; qsl-desktop `cargo test`)

`src-tauri/tests/server_pane.rs` — **the Server pane's frozen-needle home**
(D610 C5: the lane intent's G3 named `design_round3.rs`, which has no server-pane
coupling at all). 4 tests → **13**:

- pane presence + the state-communication surfaces that replaced the per-field
  buttons (`relay-token-help`, `relay-ca-status`, `relay-dirty`)
- ⚠ **NEGATIVE PIN** — the four removed controls (`btn-relay-token-set/-clear`,
  `btn-relay-ca-set/-clear`) and state 8's note are asserted ABSENT, so the
  removal cannot silently regress. A pin file that merely LOSES assertions
  documents nothing.
- three sections / exactly two hairlines, via the adjacent-sibling rule so the
  count follows the structure; `--sp-6` padding (F2R); no `--sp-x30` token
- removal is a per-field prose link, distinctly identified, cancelled by typing
- **Test-saves-first**: the commit precedes the probe; a failed commit
  short-circuits; state 14 renders and says the probe did not run
- the fixed commit ORDER (address → token → CA) and blank-token-means-keep
- no secret outside the qsc trios; the fixed 8-dot placeholder
- **three D-0011 REGRESSION pins** — `renderDirty()` after the R-B5 echo; the
  inline branch awaits nothing before clearing; commit-failure prose never opens
  with a raw error code

Unchanged and still green: the status-banner reuse / no-invented-colour-class
guard, the no-bypass guard (R8), the five-surface claim-discipline sweep,
`settings_key_allowlist`, `slice_a_rules`, `slice_a_flows`, and the byte-frozen
`design_round2.rs` / `design_system.rs` / `design_round3.rs`.

**Totals: 70 passed / 0 failed / 1 ignored at D-0010; 73 / 0 / 1 at D-0011.**

⚠ **The pins were verified as a POSITIVE CONTROL.** Run against the merged,
defective `main`, exactly the three new pins FAIL and the other ten pass; against
the fix all thirteen pass. A pin that also passes on the buggy code proves
nothing — the same standing method used for ENG-0072 in §C.

⚠ **What §A cannot reach, by construction.** Two of the three flight defects are
invisible to a socket-free structural test: (a) requires the typed address to
differ from its normalized form; (b) requires a slow probe still holding the
serial blocking gate when the next action starts. **This is why §B exists.**

## §B — Live GUI acceptance — the operator flies it, against tserver over real TLS

**Rig:** the tserver LAN host — qsl-server on `127.0.0.1:8080` behind a user
Caddy `tls internal`, unprivileged. (The address lives in the operator-side
runsheet and is deliberately NOT restated here: this repository is PUBLIC, and a
testplan is not a reason to publish a private host.) Resting state **BEARER**; the executor
flips `RELAY_TOKEN` and restarts to reach open. **The expected auth mode is
STATED PER CHECK.** No mocks (§7.4): a mock passes a layout check and hides a
wrong-error-mapping bug.

⚠ The build host cannot DRIVE the GUI (xdotool absent) — the executor enumerates
the shots, the operator takes them. ⚠ Launch with `GDK_BACKEND=x11` and a
dedicated `QSLD_DATA_DIR` (the default profile's `autolock_minutes: 1` would lock
the operator out mid-walkthrough, and Settings is unlock-gated).

Preconditions: the vault is unlocked; the app is at Settings › Server.

| # | Check | Rig auth mode | Expected |
|---|---|---|---|
| 1 | Setup: address + CA + token, **Test** | bearer | **Connected** · "Token required — accepted. Certificate trusted." · doc rows show the REAL `ServerInfoDoc`; **no "Relay name" row** when the relay's name is empty |
| 2 | Edit a field | bearer | results panel CLEARS (state 10); dirty helper "Settings changed — not saved." appears, **accent never red**; the token helper line does NOT change |
| 3 | ⭐ **Type a WRONG token → Test** | bearer | **"Token rejected"** + "…Settings saved." — the commit happened, THEN the probe. **The pre-lane build would have said "Connected"** against the stored token. This check is the lane's entire reason to exist |
| 4 | Correct token → **Save** (not Test) | bearer | commits, dirty helper clears, **NO results card** (R-A3) |
| 5 | **Test** with nothing changed | bearer | **Connected**, and **NO trailing "Settings saved."** — a clean pane commits nothing and must not claim otherwise (the 3-vs-5 pair proves C6) |
| 6 | Click ***remove it*** on the token | bearer | results panel CLEARS (**F1R**); dots disappear; helper → "Token will be removed when you save or test."; dirty helper appears |
| 7 | Type one character | bearer | helper REVERTS with the link back; dots return (R-E3) |
| 8 | Clear the field, ***remove it***, **Test** | bearer | **"This relay requires an access token"** + "…Settings saved."; token helper swaps to the no-token line with no dots and no link — the removal really committed. This is the `06e2` no-token layout state |
| 9 | Unreadable CA path → **Test** | bearer | **"Certificate authority file couldn't be read"** — ⚠ CONFIRM it is NOT "Certificate not trusted" and NOT "Couldn't reach the server", even with a dead port (R2b; the client build fails before any request) |
| 10 | ⚠ **State 14** — requires FAULT INJECTION | bearer | **"Couldn't save settings"**, naming the failed part, "…no connection test was run"; the address IS saved and the vault is NOT — the amended ordering's cost, visible. See §C |
| 11 | Fix the failing field → **Test** | bearer | **Connected** — the healing path: the commit completes from where it stopped |
| 12 | Address → a shorthand IP (`https://192:8443`) → **Test** | bearer | normalizes to `https://0.0.0.192:8443` and echoes it back (R-B5); **"Couldn't reach the server"**; ⚠ **the dirty helper must be GONE** (D-0011 defect (a) regression) |
| 13 | While that probe is still in flight, type `not a url` → **Test** | bearer | INLINE error under the field, **NO results card**, ⚠ **no stale "Testing…" surviving** (D-0011 defect (b) regression); confirm `settings.json` is UNCHANGED on disk (R-B2) |
| 14 | Remove the CA → **Test** | bearer | **"Certificate not trusted"** — the R2b contrast with check 9 |
| 15 | Address → `https://example.com` → **Test** | n/a | **"Not a QSL relay"** — something answered, its cert IS trusted, and a token WAS sent, yet the app claims neither connection nor auth problem (the FLAG-2 boundary) |
| 16 | Restore address + CA, ***remove it*** on the token, **Test** | **open** (flip) | **Connected** · "Open relay — anyone who can reach this address can use it…" · Access row reads **Open — no token needed** |
| 17 | In-flight, during any Test/Save | any | both buttons DISABLED; results area shows a neutral **"Testing…"** / **"Saving…"** (R-C1) |
| 18 | Throughout | any | three sections, exactly two hairlines, no Set/Clear buttons anywhere, R-D3 summary wording, CA status shows the HASH never the path, the token placeholder is **always 8 dots** regardless of the real token |

**Recording:** append each check's outcome to
`docs/governance/evidence/NA-0674_as_built.md §6`. ⚠ Compare to the EXACT shipped
copy, not by eye; the executor reads the screenshots back and states which were
verified how.

## §C — Fault injection (required for check 10)

State 14 is **not reachable by user input**: empty token and empty CA are skipped
rather than submitted, and a bad address routes to inline state 11.

⚠ **A directory-permission fault does NOT work** — qsc hardens its own config
directory to `0700` immediately before every vault write (`vault/mod.rs:852`),
so `chmod 0555` is self-healed and the save succeeds. *(Recorded as a positive
observation: a vault directory loosened by any means is re-tightened at the next
secret write.)*

**The fault that works:** create a **directory** at the vault's temp path
(`<data>/qsc/vault.qsv.tmp`). `write_vault_atomic` does
`let _ = fs::remove_file(&tmp)` — which cannot remove a directory, and whose
error is discarded — so the subsequent `create_new` fails with
`Err("vault_write_failed")`. Non-destructive: the write aborts BEFORE any
rename, so `vault.qsv` is untouched. Revert with `rmdir`.

Make BOTH the address and the token dirty, then **Save**: the address write
succeeds, the vault write fails, and the cost of the amended ordering becomes
visible. Corroborate on disk — `settings.json` updated, `vault.qsv` mtime
unchanged. Record the injection and its revert in the evidence directory.
