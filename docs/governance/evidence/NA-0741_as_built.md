# NA-0741 — AS BUILT

**Lane:** NA-0741 · **Decision:** D-1376 · **Rulings:** R340 (interim), R341 (final)
**Base:** main `4c59ffdadef38d9a7f058b1b39387bd9a7298e44`
**This PR is RECORDS ONLY. Zero product source bytes. The repair is promoted, not implemented.**

## 1. WHAT WAS PROMOTED

The final directive `DIRECTIVE_NA0741_FINAL_NPRIME.md` (597 lines, sha256
`a8f25983a80d1bae8df78244772e5e50ff95abe9939beab087bb182c5c062d28`, mode 444), which a **fresh
seat** will execute under the SR-15 ceremony. It specifies **OPTION N-PRIME**: classify before
unpack, skip **exactly** Handshake / InviteInit / InviteResp, route Unknown-class frames to unpack
exactly as today, leave `transport/mod.rs:1186-1250` byte-unchanged, and gate the skip to
`AckMode::Lease` only.

## 2. INPUTS, ALL SEALED 444 AND BANKED BEFORE ANYTHING CONSUMED THEM

| artifact | sha256 | lines |
|---|---|---|
| formalization brief (verbatim) | `f643939273339ec6d982c9c751827f4729c8d44fe8f9a1d10d7df70ca9f0a8b4` | 153 |
| operator-blessed design block (extracted from the brief's own bytes) | `316b09acedee3221a7c429898ef84d4833b3ad7edaf40bd06898d7ec5b306e6b` | 56 |
| interim ruling (R340) | `4ade0ca7966984c0a8bf4d76ed388a59d7cc15dbe4d803c8c3a249c2897f60d1` | 65 |
| **SR-15 adversarial cold read — NOT this seat's work** | `98e266562d2797c06e1bfb1337a3501a0072f77fccdcbbf1ce31914ac42f95b0` | 842 |
| final ruling (R341) | `b5284c1d1da20836544d84251b4be942cc04e9b77582c2ec854d611ce92972ef` | 113 |
| Director's verification of the final directive | `f2877a61f0d3328356d9ff3dd553825f3dfc12d001cf30474a99289b0070cae9` | 22 |
| **final directive (the deliverable)** | `a8f25983a80d1bae8df78244772e5e50ff95abe9939beab087bb182c5c062d28` | 597 |

The design block was **extracted from the banked brief's own bytes by delimiter**, never retyped,
and diffed back **identical (rc 0)** against a **negative control returning rc 1**, non-vacuity
asserted. Every ruled text folded into the final directive was likewise **extracted from its
source's own bytes**: **6/6 positive** (region occurs exactly once in the finished document) and
**6/6 negative** (last-character-mutated copy occurs zero times).

## 3. THE MEASUREMENTS THIS LANE OWNS

- **Base UNMOVED**, bare and unpiped, against the **NAMED** `github` remote, re-verified at the
  edit. **Open-PR set MEASURED EMPTY** with a positive control returning merged rows.
- **Ids on DECLARING forms** via `git grep` (never a `.gitignore`-honouring recursive search —
  WF-0087 part 2, which is blind to 514 tracked files in this repo), `D-` across **all four**
  record forms, **every** id space swept: `NA-0741` / `D-1376` each **0 declaring, 0 mentions**,
  against three positive controls at `declaring=1`.
- **The ruling's premise, verified from the tree's own fixture bytes rather than adopted:** six of
  the seven breaking fixtures are Unknown class (`00 00`, `6e 6f`, `6e 6f`, `6e 6f`, `6e 61`,
  `7b 22`); the seventh, `timeline_store.rs:154`, is `01 02` — InviteResp. Controls confirmed the
  classifier returns Message / InviteInit / InviteResp / Handshake for the four real frame shapes
  and Unknown for 0- and 1-byte inputs, so it discriminates in both directions.
- **The per-item loop** spans `transport/mod.rs:521-1252` with **35 early exits** (7 `return Err`,
  28 `?`). Three of the seven abort on **frame content** after a successful unpack (`:642`,
  `:682`, `:720`) and ride the ENG-0142 closure amendment.
- **`qsp_unpack`'s Err channel carries three populations**, which is why the design block's third
  arm was ruled void: ten frame-content codes, plus `qsp_session_store_failed` (a store WRITE),
  `qsp_no_session` and `qsp_channel_invalid`, with `session_rollback_detected` riding the same
  failure.

## 4. THE SEAT'S OWN ERRORS, RECORDED RATHER THAN TIDIED

1. **The §2(d) census was narrower than its claim** — it counted rc idioms while the claim covered
   marker consumers. Caught by the SR-15 read; landed as SR-16 row 44.
2. **The first rc census missed 15 of 28 test functions** (helper-wrapped receives), and the miss
   was found **by reading a file, not by the instrument**. Widened before the stop.
3. **The R-id sweep took `max()` over MENTIONS** and would have taken `R341` instead of `R340` —
   the instrument committing the very hazard it was written to detect.
4. **Four line-wrap / model-built needle misses** across the lane (a banked clause, an extraction
   anchor, a status-line needle carrying a leading space that was an artifact of the tool output
   it was read from). Each time the **needle** was wrong and the **tree** was right.
5. **The assembler's anti-placeholder assert fired on its own proof table**, which had reproduced
   the placeholder literal. The assert was right; the table was wrong.

## 5. WHAT IS EXPLICITLY NOT DONE

ENG-0142 **not closed, not repaired** · ENG-0196 **untouched (lane 2)** · ENG-0191 (a)–(e)
**unruled** · ENG-0194 **not repaired** · ENG-0193 **not built** · WF-0086 and WF-0087 gates
**not built** · **#1745 — an ISSUE, not a PR — stays OPEN** · no prior findings swept · no test
weakened, skipped or deleted · no standing rule minted · no fenced ruling or sealed artifact
edited · **zero product source bytes.**
