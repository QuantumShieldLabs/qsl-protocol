# NA-0683 — AS-BUILT: the naming sweep (Server → Relay), qsl-desktop (D618)

Goals: G4 (primary), supports G1

**Result class:** `RELAY_NAMING_PASS`
**Directive:** `QSL-DIR-2026-07-29-618_relay_naming_sweep.md`, sha256
`48d77b123718275306708e5fdaf7ff4fa0489331980ee325ee3d342ff9d8b390`, 445 lines
(amended in place with all six flags ruled; the drafted 319-line `4a4007fe…a7b76dfa` is
superseded and marked, never rewritten).
**Repo:** `qsl-desktop` — PR **#20** merged `d3d46c8e1e71ff951b85bf8ebfbff37f65aa5cef`,
lane commit `9bb598b7`, decision **D-0021** — plus this spine closeout.
**Testplan (the instrument):** `tests/NA-0683_relay_naming_testplan.md`.

---

## 0. What this lane was

The operator ruled on 2026-07-27 that **the user-facing term is "Relay", never "Server"**.
The rationale is the load-bearing part: *relay* **teaches the security model** — a dumb pipe
forwarding opaque bytes, not a trust-holding service — it matches the protocol docs and the
invite system, and it suits a privacy-conscious audience.

**That first clause is why the sweep stops where it stops.** The word is doing security-model
work **on surfaces a user reads**. It is doing no work at all inside `pane-server`,
`relay_server_info` or `GET /v1/server-info`, where renaming would cost compatibility and buy
the user nothing.

⚠ **The census found the ruling already ~90% shipped.** "Relay address", "Relay name", "Open
relay", "Not a QSL relay", "This relay requires an access token" and `Relay: {url}` were
already in the product — **in the same pane** as "No server configured" and "Couldn't reach
the server". This lane finished a word the product had already chosen.

---

## 1. The census, and the four premises it corrected

Sweep: `git grep -n -I -i -e 'server' -- .` over **all 49 tracked files** at `399f45e8`;
**196 matching LINES = 207 OCCURRENCES**; the one binary file (`icons/icon.png`) excluded by
`-I` and checked separately. Classified by an instrument that **asserts its classes sum to
the raw hit count**, so a lost hit crashes rather than miscounts.

```
USER-FACING 21 · TEST-NEEDLE 10 · DESIGN-SPEC 19 · IDENTIFIER 61 · COMMENT 42 · LEAVE-FILE 54
```

**C1 — the lane intent's "qsl-desktop has NO public-safety CI scan" is wrong, in a way that
helped.** The scan exists (`ci.yml`'s `public-safety` job, whole tree + added lines, plus an
opt-in pre-commit call site). What is true is that it **cannot block**: the repo requires
exactly one status context, `rust`. The lane therefore had a real instrument instead of an
eyeball pass — and it used it (see §5).

**C2 — the config-key stop condition could not fire.** The persisted key is **`relay_url`**,
not `server_url` (`settings.rs:37`, under `deny_unknown_fields`). Read, not assumed.

**C3 — the "update the tests that assert the old strings" work set was EMPTY.** Ten test
literals contain "server"; **not one asserts a string this lane changes** — six assert that
*retired* slice-A copy stays absent, four are assertion messages naming the pane. Verified by
grepping every changed literal against `src-tauri/tests/`, not by reading the class.

**C4 — one user-facing string was not in the UI at all.** `commands.rs:314` sets
`app_info().slice`, which `main.js:594` renders in the About pane. Found by reading the
writer *and* the reader; a sweep confined to `ui/` would have missed it.

---

## 2. What shipped

**16 lines, 8 files, one word each:** the settings nav item and the pane heading, the
main-window status line, the two `Settings › Server` strings, the `Server version` result
row, the `Couldn't reach the server` banner, the cert-not-trusted sentence, the CA-unreadable
sentence, `app_info().slice`, two README lines, and the rendered text of both reference
mockups.

**F1 (ruled FIX):** 13 live normative lines across `DESIGN_SPEC.md`, Appendix D and
Appendix F — *including the two that are the literal strings the UI renders* — each edited
file carrying **exactly one** dated revision line and **no other added prose**. Every
`⛔ SUPERSEDED` block and dated note untouched, per Appendix F's own mark-don't-rewrite rule.

**F4 (ruled (b)):** `src-tauri/tests/relay_naming.rs`, 5 tests, in the existing
`claim_discipline_five_surfaces_swept` idiom.

### 2.1 What deliberately did not change — and must not later

`data-pane="server"`, `#pane-server`, `.server-form`, `.srv-sect`, `serverBusy`,
`setServerBusy`, `clearServerResults`, `serverDirty`, `onServerChanged`,
`commitServerSettings`, `refreshServerState`, `refreshServerPane`, `renderServerOutcome`,
`renderServerError`, `ServerInfoDocDto`, `RelayServerInfoOutcome`, `relay_server_info`,
`GET /v1/server-info`, and the test filename `server_pane.rs`.

Two lines were **ruled left, visibly**: `AppendixD:60` (F2 — a one-word substitution would
make the sentence **false**, because the clause it carries was retired when slice B shipped)
and `Cargo.toml:6` (F5 — stale rather than mis-named).

---

## 3. ⚠ THE 14th F1 LINE COULD NOT BE MADE, AND THE REASON IS NOT ABOUT NAMING

F1 ruled 14 lines. **13 landed.** `docs/DESIGN_SPEC_AppendixF.md:239` carries an
operator-infrastructure literal **already present on `main`**:

```
infra-literal-scan: FAILED (staged; 11 files, 244 lines examined)
docs/DESIGN_SPEC_AppendixF.md:0: [added-line:host_retired_rig]
```

**Tier-1 (`--mode tree`) does not flag it** — that class applies to **added lines only** — so
the tree reads clean until a lane *touches* the line. The one-word `server version` →
`relay version` edit re-adds it, and the pre-commit gate refused the commit.

**The edit was reverted rather than worked around.** Redacting a hostname inside a naming PR
would have pre-empted the sanitization micro-lane already approved for exactly this class.

**Director ruling (Option 1):** 13/14 stands; **`AppendixF:239` transfers WHOLE to the
sanitization micro-lane**, which makes **both** edits to that one line — hostname →
placeholder **and** the one-word relay fix. Recorded in **D-1320's map** as F1's deferred
14th line.

---

## 4. The gate, and the arithmetic that had to be rewritten three times

The gate is `naming_sweep.py`: it classifies **every occurrence**, prints **what it
examined**, and carries a `RULED-LEAVE` bucket that **prints every operator-ruled exception**
— an exception you cannot see is not an exception.

**It was run RED first, at base: `GATE FAIL: 21 user-facing "server" occurrence(s) remain`.**
A gate that has never returned positive proves nothing.

**The expectation was written before every run — and restated in full three times, because
the tree kept changing under it:**

| restatement | cause | predicted total |
|---|---|---|
| 1 | the F4 guard **must spell the strings it forbids** — measured in a sandbox copy, it adds exactly 24 occurrences | 197 |
| 2 | the guard's own fix (§6) added one more comment occurrence | 222 |
| 3 | the 14th F1 line could not be made (§3) | **223** |

Plus *d* = **24**, the occurrences in the new D-0021 entry, counted from its written text
before the run — because the lane's own paperwork is inside the thing the gate measures.

**Final run against the committed tree, matching exactly:**

| class | base | final |
|---|---|---|
| total | 207 | **223** |
| `USER-FACING` | 21 | **0** ← the gate |
| `RULED-LEAVE` | 0 | **1** (`Cargo.toml:6`, printed) |
| `TEST-NEEDLE` | 10 | **24** |
| `DESIGN-SPEC` | 19 | **6** |
| `IDENTIFIER` | 61 | **62** |
| `COMMENT` | 42 | **52** |
| `LEAVE-FILE` | 54 | **78** |

---

## 5. Checks and controls

| check | result |
|---|---|
| PHASE 0 base reproduction | `fmt` 0 · **97 passed / 0 failed / 1 ignored** · `clippy` 0 · `clean (tree; 48 files, 16974 lines examined)` — reproduced to the figure **before any edit** |
| PHASE 0 anchors | **37 line anchors re-verified, zero drift** |
| suite (post-fix) | **11 targets · 102 passed / 0 failed / 1 ignored** — 97 + N, **N = 5 stated before the run** |
| `fmt` / `clippy --all-targets -D warnings` | clean |
| `infra-literal-scan` | `clean (tree; 49 files, 17186 lines examined)` · `clean (diff; 11 files, 243 lines examined)` |
| CI on #20 | `rust` **pass** (the one required context), `public-safety` **pass**, `advisories` **pass** — **read from the job logs**: CI's suite is 102/0/1 and the scanner's examined-line counts are **identical to local** |

**F4's binding red-capability control, both halves:**

```
reintroduce "No server configured." into ui/index.html
  -> cargo test --test relay_naming   EXIT 101
     FAILED: the_pane_is_named_relay_everywhere_it_is_shown    (positive pin)
     FAILED: the_retired_server_wording_stays_gone             (negative pin)
restore -> cmp byte-identical, git status clean
  -> cargo test --test relay_naming   EXIT 0, 5 passed
```

**Two independent tests caught it, not one.**

---

## 6. ⚠ THE GUARD FAILED ON ITS FIRST RUN AGAINST THE CORRECT TREE

`relay_naming.rs` asserted `!cmds.contains("server connectivity")`. That phrase **also lives
in `commands.rs:319`'s section comment** — internal prose the ruling deliberately leaves
alone. The needle was testing the **mechanism** (the word appears somewhere in this file)
instead of the **property** (the rendered string says it). Tightened to the literal
`slice: "B (server connectivity:`, with the failure recorded in the file beside the fix.

**Files contain prose ABOUT strings as well as strings.** A negative needle scoped to a whole
file is almost always broader than the property it defends. Had it shipped as drafted, the
next person to edit that comment would have hit a naming guard failing for a non-naming
reason.

---

## 7. Operator live acceptance — **PASS 5/5**

Flown 2026-07-29 on the real rig, on a **throwaway profile** via the sanctioned
`QSLD_DATA_DIR` override (`paths.rs:20`); the operator's real profile was never opened.

| # | expectation (written first) | observed |
|---|---|---|
| 1 | main-window status line reads "No relay configured." | ✅ |
| 2 | settings rail item reads **Relay** | ✅ |
| 3 | pane heading reads **Relay** | ✅ |
| 4 | steady-state line: "No relay configured — add one in Settings › Relay." | ✅ |
| 5 | unroutable address + Test connection → **"Couldn't reach the relay"** | ✅ verbatim |

**Not claimed:** the `Relay version` row renders only on a **successful** probe and the rig is
retired, so it was **not verifiable live**. It is covered by `relay_naming.rs` and both
mockups, and is recorded as unverifiable rather than counted as a pass.

⚠ **THE FLIGHT BINARY HAD TO BE REBUILT, AND THIS IS NOW A STANDING RULE.**
`tauri.conf.json` sets `frontendDist: "../ui"`, so the frontend is compiled **into** the
binary. The cached binary from the test runs had been built **during the red-capability
control**, while `index.html` temporarily said "No server configured." again. **Flying it
would have shown the operator the wrong app and produced a false RED.** Operator acceptance
now always runs against a **rebuild from the committed tree with `git status` empty at build
time**, recorded in the flight note.

---

## 8. Cross-repo enumeration (read-only; no file outside `qsl-desktop` was touched)

`qsl-protocol`, `qsl-server` and `qsl-attachments` were read **straight out of the bare
mirrors**; the org `.github` repo was cloned read-only into a scratch directory.

| repo | raw hits | user-facing candidates |
|---|---|---|
| `qsl-protocol` | 13 708 | **3** |
| `qsl-server` | 541 | **1** |
| `qsl-attachments` | 66 | **0** |
| org `.github` | 4 | **1** |

The raw counts are not the finding — they are reported so it is clear the sweep ran over
everything and the small answer is a **result**, not a truncation. Dispositions live in
**D-1320's map**.

⚠ **`"server": "qsl-server"` in the relay's `/v1/server-info` body is a WIRE FIELD and a
NAMED BOUNDARY — never touched by any naming work, ever.** The `relay_server_info=` CLI
markers and the endpoint path are equally off-limits.

---

## 9. Observations carried out of the lane

- **OBS-8** → ENG-0089. An added-line-only literal class means **the tree is only as clean as
  its last edit**: any lane touching a legacy line inherits a gate failure it did not create.
  Operator's question for the CI/tooling lane: **promote `host_retired_rig` to Tier-1
  tree-wide once known instances are zeroed.**
- **OBS-9** → §6, disposition approved as recorded.
- **OBS-10** → routed to the **Slice 4 design session** as motivating evidence for
  `DESIGN_status_bar_v1` — **not a new design item**. `#status-line` is a `<footer>` inside
  `#scr-main`, so the app's only persistent "what am I pointed at" indicator **disappears
  exactly while the user is in Settings › Relay changing the thing it reports**. The operator
  raised it unprompted, twice, which is itself the evidence that it reads as missing rather
  than as scoped.
- **OBS-1 / F5** → ENG-0088. The claim-discipline guard family covers neither Cargo metadata
  nor module docs; `Cargo.toml:6` and `lib.rs:1` still say "serverless skeleton".
- **OBS-2** → the Slice 4 spec refresh: `AppendixF:79` names reference markup
  `02-settings-server-pane.html` that is **not tracked in the repo**.
