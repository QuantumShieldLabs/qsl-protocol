# NA-0686 — AS BUILT: CI/tooling & test-instrument audit

**Lane:** NA-0686. **Ruling:** D-1325. **Base:** `qsl-protocol` `d2bf480e`,
`qsl-desktop` `f91fc75`, `qsl-server` `131d63f`, `qsl-attachments` `a71d348`.
**Goals:** G4 (primary), G1, G5.

This lane changed **no product behaviour**. It repaired the instruments that
measure the product, and one diagnostic surface. Every migration and every new
guard carries a recorded red/green control pair.

---

## 1. What this lane is really about

Seven findings, one thesis: **an instrument that is not itself instrumented gets
believed instead of checked.**

- A test that learns an identifier by scraping a log is measuring redaction
  policy, not the product (ENG-0087).
- A guard whose greenness depends on an unrelated constant's value is measuring
  that constant (ENG-0084).
- A test that asserts a marker was emitted is measuring the emit site, not the
  state (ENG-0085).
- A scan that reports "clean" over zero files is measuring nothing (ENG-0089).
- A suite that prints "ok" without saying what ran cannot notice a test leaving
  (ENG-0075).

## 2. ⚠ The three places measurement contradicted the instruction

Recorded first, because they are the lane's most useful output. In each case the
directive was reasonable, the code disagreed, and the code was right.

### 2.1 The ENG-0084 fix as specified was a NO-OP

The instruction was field-name-keyed redaction for the `msg_id` field. Measured:
**all eight marker fields literally named `msg_id` already carry the literal
string `"<redacted>"`.** The field carrying real message ids is keyed `id`. A rule
on the name `msg_id` would have redacted the sentinel and left the finding's own
site untouched — a green control proving nothing.

Ruled to **Option C**: the emitting helper stops accepting an identifier at all.
Zero redactor edits. A parameter that does not exist cannot be passed raw.

### 2.2 The ENG-0087 remedy DOES NOT TRANSFER to instance #4

The instruction was to migrate instance #4 to `first_party_sent_msg_id`, the
remedy proven twice in NA-0682. Measured **RED**: `event=error code=state_unknown`.

`receipt-apply --msg-id` keys on the **timeline entry id**, minted as
`forced_id.unwrap_or_else(|| "{dir}-{ts}")`; the send path's only `forced_id` is
`receipt_msg_id`, populated **only when a receipt was requested**. These two tests
request none, so their entry id is the short `out-<ts>` form — **not** the queue
record's 128-bit `msg_id`, and the queue record carries no timeline id to read
instead. The two identifiers coincide only under the exact condition instance #4
lacks.

So instance #4 took ENG-0087's **second** clause instead of its first: the scrape
remains and is made LOUD. The coupling is not removed; it is made to fail at the
defect rather than three steps downstream.

### 2.3 The ENG-0082 fix did NOT require rewriting a guard

The finding recorded that splitting 401/403 *"requires rewriting `NA_0663`'s
assertion"*, and rejected the option on that basis. The reasoning was right; the
premise was false. **`NA_0663` contains no 403 case at all.** Splitting only the
403 arm left every assertion true and byte-identical, and **NA_0663 passing
untouched (11 passed, exit 0) is the measurement that proves the split kept 401
intact.**

> A filed reason to AVOID an edit is itself a claim, and it can be measured.

## 3. The census reconciled exactly — and the reconciliation corrected the record

| class | qsl-protocol | qsl-server | qsl-attachments | qsl-desktop | total |
|---|---|---|---|---|---|
| retired-rig host token | **771** (79 files) | 0 | 0 | 0 | **771** |
| public dynamic-DNS domain | 9 (5 files) | 8 (3) | 8 (3) | 0 | **25** |
| CGNAT `100.64/10` | **23** (8 files) | 0 | 0 | 0 | **23** |
| tracked paths carrying the token | **10** | 0 | 0 | 0 | **10** |

⚠ **771 + 25 = 796, byte-identical to NA-0684's recorded final-gate total.** The
ledger's "796-occurrence record class" is **two needles summed**, not the host
token alone; a first pass measuring only the host token read 771 and looked like
drift. Nothing had drifted.

⚠ **NA-0685 added ZERO new occurrences of either token**, though the tree grew by
12 files and 3 508 lines between the two measurements. The predecessor's
paperwork was clean, and the arithmetic is the proof.

## 4. What changed, with its control

| # | change | control (red → green) |
|---|---|---|
| 1 | ENG-0087 #3 migrated first-party; helper can no longer return an id | property byte-identical; test now proves a *correctly addressed* forgery is refused |
| 2 | ENG-0087 #4 hardened under rule 2 (scrape stays, sentinel refused) | first-party substitution measured RED (`state_unknown`); hardened form green ×1 |
| 3 | sentinel fail-fast rule, one field-agnostic implementation | disable the rule → its guard RED; restored byte-identical (`cmp`) |
| 4 | ENG-0082: 403 splits at **all THREE** layers — `relay_forbidden` / `access_forbidden` / `access_refused` | collapse any layer → guard RED naming it; restored (`cmp`); **NA_0663 untouched, 13 passed**; the two 401-driven consumers pass unmodified |
| 5 | ENG-0084: helper stops accepting an id (+ clause (b)) | id narrowed test-only: fix in place → C17 guard GREEN; old form → **RED**; both restored (`cmp`) |
| 6 | ENG-0085: timeline read-back added | expected state flipped → fails against the real stored row; restored (`cmp`) |
| 7 | ENG-0089 (i) Tier-1 promotion + per-path baseline | synthetic added occurrence → RED naming the class; removed → green |
| 8 | ENG-0089 (ii) CGNAT Tier-2b | synthetic added line → RED; removed → green |
| 9 | ENG-0089 (iii) public-DDNS Tier-2b | synthetic added line → RED; removed → green |
| 10 | ENG-0089 vacuous-pass guard (13-check self-test) | pins `NOTHING EXAMINED`/exit 2, and the deliberate `staged`-mode asymmetry |
| 11 | ENG-0088: needles extended to Cargo metadata + module docs | reintroduce the retired phrase in either surface → RED; both restored (`cmp`) |
| 12 | ENG-0075: `-q` removed; test inventory pinned by NAME | delete a test file → check fails naming 5 missing tests; restored (`cmp`) |
| 13 | Phase 6: one shared-fixture test split into three | 3 consecutive green runs (13 tests each) |
| 14 | ENG-0090: three one-word naming edits | measured; docs/public remainder was ZERO, not ~10 |

## 4a. Instance #4: accepted as LOUD, with the other half booked to the flip

The operator accepted the interim disposition — **loud beats silent** — while
noting the flip-hazard rationale is only half-discharged. **ENG-0086 therefore
carries a binding annotation:** the flip commit migrates #4 *in the same commit*
as the default flip — the fixture requests an explicit receipt
(mechanism-by-explicit-flag, the intended Option D shape), which makes the
timeline id equal the queue `msg_id`, and first-party acquisition then applies per
the proven remedy under the binding condition in full.

⚠ **Until then the loud sentinel IS the guard, and its red is by design:** if the
default flips without the migration, #4's sentinel check fires red. **That red is
the tripwire working, not a surprise** — which is the whole point of converting a
silent coupling into a loud one.

## 5. The allowlist is a BUDGET, not an exemption

The Tier-1 promotion meets its 771-occurrence history as a **per-path expected
count**. A file may keep what it has and may lose it; it may not gain one.

⚠ **A path-only allowlist would have let a new occurrence into an already-listed
file silently — which is precisely the hole ENG-0089 was filed about.** With
counts, Option B stops being a habit and becomes a tree invariant.

⚠ **The intended bite, so nobody mistakes it for a bug:** a future lane that
writes the retired name into `DECISIONS.md`, the journal, or any other record
**will go red**, and must take the placeholder as part of that edit. This
document was written under that constraint.

⚠ **Keys are salted digests of `<repo>:<path>` because TEN OF THE ALLOWLISTED
PATHS CARRY THE TOKEN IN THE PATH ITSELF.** A plaintext list would have
republished, inside the gate file, exactly what two sanitization lanes removed —
and the Tier-1 scan would then have hit its own allowlist. The scan prints the
real paths at run time instead: an exception you cannot see is not an exception.

## 6. Measurements

| measurement | result |
|---|---|
| desktop suite (base = final; 11 binaries) | **102 passed / 0 failed / 1 ignored**, exit 0 — matched the written point prediction exactly |
| desktop test inventory | **103 pinned** (102 + 1 ignored) |
| `--mode tree`, all four repos | clean; 2307 / 49 / 80 / 41 files examined |
| scanner self-test, all four repos | 13 checks, 0 failed |
| gate file identity across repos | md5 identical (`dbb9bb33…`) |
| `NA_0663_relay_tls_trust` | 11 → 13 tests after the Phase-6 split; 3 consecutive green runs |

## 7. Observations carried out of the lane

**OBS-H — the Phase-3a control was SUBSTITUTED, and the substitution is the
evidence.** The ruled control (*re-accept the id, pass it raw, the C17 guard goes
red*) **cannot fire**: a raw 32-hex id is redacted anyway by the shape rule, which
is precisely ENG-0084's content — the old code was defused by WIDTH, not by
correctness. A control that cannot fire proves nothing, and reporting it green
would have been this lane's own failure class. The substituted control varies the
property the coupling depends on and is strictly stronger: it shows the
DEPENDENCE removed rather than one instance patched. **Operator-ratified.**

**OBS-A — `emit_message_state_transition` is shared with the attachment path.**
Despite the name, `timeline_append_entry_for_target` and
`timeline_transition_entry_state` both serve file transfers, so clause (b)'s
redaction is a (diagnostic-only) change to the attachment surface too. Not the
`attachments/mod.rs` `file_id` population the ruling excluded — but the overlap is
real and is reported rather than absorbed. ⚠ **Operator-ruled ACCEPTED, no
revert** — strictly-more-redaction is the house direction (name the field, never
the value), and the consumer count was measured at zero. The
`attachments/mod.rs` `file_id` sites remain untouched and out of scope; **the
future attachments-diagnostics lane meets this as a known fact, not a
discovery.**

**OBS-B — a third 401/403 collapse site survived, and was RULED IN mid-lane.**
`relay_push_diagnostic_class_for_status` mapped both to `bearer_auth_failed`.
⚠ **Operator grounds, and the generalisable rule: ENG-0082 could not close with
one collapse standing — the ledger's closure claim would have been false.** A
finding that closes while its own defect survives one layer out is a false
closure. Fixed in-lane: **403 → `access_refused`**.

⚠ **It was not merely imprecise, it was WRONG.** The function's neighbours name
WHICH CREDENTIAL failed; a 403 is the case where the bearer was **accepted**.
Calling that a bearer failure **sends an operator to re-check a token that was
never the problem** — a diagnostic that misdirects is worse than one that
under-informs. Three layers now carry three vocabularies, each from its own
neighbours: `relay_forbidden` / `access_forbidden` / `access_refused`. 401 is
untouched at all three, and the three 401-driven consumer suites
(`relay_push_diagnostics` 3, `secret_material_diagnostic_boundary` 4,
`NA_0663_relay_tls_trust` 13) all pass unmodified.

**OBS-C — the satellites conflate two causes at the branch-protection layer.** In
`qsl-desktop`, `qsl-server` and `qsl-attachments` an infrastructure literal and a
leaked private key **both fail as `public-safety`**. Same defect class as
ENG-0082, one layer out. Recommendation and safe sequencing are in the Phase-4c
parity audit; **no protection was changed — settings are operator-only.**

**OBS-D — the workflow comments in the three satellites are STALE.** They say the
scan is "advisory until branch protection changes"; `public-safety` is in fact a
required context in all three, so the scan blocks today.

**OBS-E — the guard caught this lane writing the defect back in.** The first draft
of the `lib.rs` note QUOTED the retired claim, and the newly extended
claim-discipline needle failed on it. Direct evidence the needle works on live
content rather than only on the case it was written for.

**OBS-F — ~60 further marker scrapes share the id class's latent coupling.**
Enumerated in the ENG-0087 annex by field (`identity_fp=`, `device=`, `state=`,
`invite=`, `send_seq=`, `max=`). None crosses the redactor today; every one is
safe by a fact about VALUE WIDTH rather than about the code. The remedy costs one
call each and was written field-agnostic for exactly that reason.

**OBS-G — a Director artifact repeated the parenthetical-estimate failure.**
D-1320's map said "~10 `docs/public` prose lines"; it measured **2**, both LEAVE.
NA-0685 recorded the same mechanism against a predecessor's "about 14" that
measured 16. **The map was also line-number-keyed and had already drifted**
(`cmd/mod.rs:601` → `:641`) — in an artifact later lanes were told to cite rather
than re-derive, while this project's own instrument already carries the
counter-rule: *key on content needles, never on a line number.* The map is
annotated in place with content needles.
