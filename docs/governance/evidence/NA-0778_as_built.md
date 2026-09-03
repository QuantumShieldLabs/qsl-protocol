# NA-0778 -- AS BUILT (BORN AT THE PROMOTION): THE INVITATIONS LANE

Goals: G4 (primary), drives G1-G3 delivery

Lane `NA-0778` (the operator's "Lane C") . `D-1421` . class: OWED (the Director declares it at close; none is invented here).

**EVERY DIGEST IN THIS FILE WAS COMPUTED FROM THE FILE AT GENERATION TIME, NOT TRANSCRIBED.** Where a figure is carried from another record it says so and names the record.

This file is BORN at the promotion PR and grows at the close. It is added with `git add -f` because `.gitignore:65` `**/evidence/` would otherwise silently drop it (`WF-0087`'s write half), and the staging is confirmed with `git diff --cached --name-only`.

## 1. THE GOVERNING TEXTS, BY SHA

| document | bytes | sha256 |
|---|---:|---|
| `KICKOFF_invitations_lane_20260903.md` (the Director's kickoff; sha-verified BEFORE reading) | 25965 | `8b0f1eb52899fa88167032a8e24acba642766ddf85609f0130355d7d52f65fdd` |
| `DIRECTOR_SEATING_20260903.md` v2 (the 09-03 chair's seating record) | 12870 | `77c4c94cbe83911fa83eb9e67e2f9e82dfb1cf03eb6c1a5b6e229365d798d1b5` |
| `DIRECTOR_SEATING_20260902.md` (the 09-02 chair's seating record) | 13260 | `e91817f65cafe1007141e2aaa9695973e587d1ab5843ea6d8998c831c930e02a` |
| `RBANK_operator_decisions_seating_20260902.md` (R1-R6, the operator's blessed order) | 4782 | `101bebc6982c2e6b37b971d5df47aa8ce8b6aff22fb2d5ad84579b842d055576` |
| `RBANK_record_push_cadence_20260903.md` (C1-C5, the operator's "bless") | 4061 | `9747b9febbe21c47a0e3e2731fc655774f2e1d8c3996523f9b73c7d5c1746e49` |
| `STOP_NA0778_001_20260903T200355Z.md` (the formalization; nine carries) | 145983 | `5a9b7ebc6b6ae4fce588bcc960269d3e0a662567dfd02b570a052bf1ed40e908` |
| `RULING_NA0778_001_20260903.md` (R1-R10) | 13721 | `0e04e22135c682f105dbe18203b93d5bbe3bda2560f682a555e3d3ac862459d1` |
| `RULING_NA0777_016_PR_20260903.md` (banked by this lane under NA-0777/, kickoff B(a)) | 3640 | `441456bb07706ebe55efc17d6f694e08b110c64518dd112b491015487c9d3b1e` |
| `RULING_D1-3_freeze_20260903.md` (banked by this lane, kickoff B(b)) | 3776 | `edd7e9d0e1ce53a3fd3c10eec0db00b6b81eafecddb4213f220acd8a0c30b882` |
| `RULING_NA0776_021_closeout_20260902.md` (the class this PR transcribes) | 2918 | `2208a1f0817064240d82121f7c7b365ed5e22282f5b76bc32dad020ea8b3257b` |
| `CLOSEOUT_NA0776_20260902.md` (sec 4(ii), corrected by the ruling above) | 13790 | `f306b2ded4a0016781a3c01f2d36f44033df45a9890e294edac203d59ff0d593` |

All banked 444 under `state/operator/` or `state/operator/NA-0778/` in the new build tree, the record of authority since NA-0777's S4.

## 2. THE P1 BOARD, MEASURED WITH A NEEDLE AS WIDE AS THE CLAIM (`RULING_NA0778_001` R4)

Instrument: every `### ` entry of `docs/ops/IMPROVEMENT_LEDGER.md` at `2c3c39b4` whose `- Severity:` bullet (any of the forms `- Severity:`, `- **Severity:**`, `- **Severity**:`) contains the token `P1`. Result: **19 hits** -- equal to the Director's own wide run. Every hit is classified below; ZERO unclassified is the gate. The four ruled classes are CLOSED / RE-GRADED / OPEN / UNDECLARED; the needle's width requires a fifth, MENTION, for a `P2` bullet whose argument names `P1` ("Against P1: ...") -- an entry that was never P1 is not forced into a P1 bucket.

| # | entry | Severity bullet says | Status / closure, as the entry reads at `2c3c39b4` | class |
|---|---|---|---|---|
| 1 | `ENG-0012` | P1 | `Status: CLOSED (the P1 resolved at NA-0624, D-1243)` | CLOSED |
| 2 | `ENG-0173` | P1 | `Status: CLOSED 2026-08-11 -- fixed, instrumented RED-first` (NA-0711, D-1348) | CLOSED |
| 3 | `ENG-0174` | "re-argued down from P1 -- the loss is temporary and self-healing" | `Status: open (corrected)` at the re-argued (unstated) grade | RE-GRADED |
| 4 | `ENG-0176` | P1 as filed | `Status: CLOSED as corrected` (NA-0711) | CLOSED |
| 5 | `ENG-0038` | P1 | `Status: REMEDIATED at NA-0633 (D-1257, directive D570) -- construction C1`; the assurance depth is tracked as `ENG-0172` (P2, open) | CLOSED (remediated) |
| 6 | `ENG-0080` | P2 as filed; "would be P1 if it shipped" | `Status: open -- FILING ONLY` | MENTION |
| 7 | `ENG-0095` | P1 | `Status: FIXED at NA-0688 (D-1327, directive D622 C0-A)` | CLOSED (fixed) |
| 8 | `ENG-0134` | P1 (transcribed from its own heading) | `Status: no status was declared when this entry was filed on 2026-08-09, and none has been declared since ... a status invented to make a count work is the defect this repair exists to fix. The entry's disposition needs its own act.` A candidate attribution (NA-0735) refuted by measurement 2026-08-15 | UNDECLARED |
| 9 | `ENG-0142` | P1 (transcribed); "Severity, RE-GRADED 2026-08-16 to P2 by the OPERATOR ... scoped" to the non-adversarial trigger | The non-adversarial trigger CLOSED by lane 1 + N-PRIME (NA-0741); the Legacy portion CLOSED by NA-0770 (`D-1411`); the REMAINDER (message-class and unknown-class frames from any route-token holder; `relay_inbox_parse_failed`; the three post-unpack content aborts) "REMAINS OPEN ... Severity is UNCHANGED at P1 here", under the OPERATOR'S ACCEPTED-RISK RULING of 2026-08-26 ("accept for now", NA-0763 `D-1404`) | OPEN (P1 remainder; accepted risk) |
| 10 | `ENG-0172` | P2 ("assurance depth on a P1 authentication property") | `Status: open -- FILING ONLY` | MENTION |
| 11 | `ENG-0250` | P1 | `Status: open` at `2c3c39b4`; CLOSED by THIS PR (R4), residuals named | CLOSED (this PR) |
| 12 | `ENG-0251` | P1 | `Status: open` at `2c3c39b4`; CLOSED by THIS PR (R4), residuals named | CLOSED (this PR) |
| 13 | `ENG-0252` | P1, OPERATOR-assigned (`D-1411`) | `Status: open` at `2c3c39b4`; CLOSED by THIS PR (R4), residuals named | CLOSED (this PR) |
| 14 | `ENG-0269` | P2 argued ("Against P1: it requires a failure inside a narrow window") | `Status: open` (NA-0773; repaired by NA-0775 `D-1418`, whose entry says so) | MENTION |
| 15 | `ENG-0272` | P2 argued ("Against P1: nothing is destroyed and no secret is exposed") | NO `- Status:` bullet in the entry (observation; NA-0774 repaired it) | MENTION |
| 16 | `ENG-0281` | P2 argued ("Against P1: it needs a corrupt stored suite-context block") | `Status: closed -- filed and repaired 2026-09-01 by NA-0775` | MENTION |
| 17 | `ENG-0282` | P2 argued ("Against P1: per-peer, never global") | `Status: open` (NA-0775) | MENTION |
| 18 | `ENG-0283` | P2 argued ("Against P1: no data is lost and no key is exposed") | `Status: open` (NA-0775) | MENTION |
| 19 | `ENG-0284` | P2 argued ("Against P1: no data is lost and the handshake itself is fine") | `Status: closed -- introduced, found, and repaired within NA-0775` | MENTION |

TALLY after this PR: CLOSED 8 (five before this PR; three by it) . RE-GRADED 2 (`ENG-0174`, `ENG-0142`'s non-adversarial trigger) . OPEN 1 (`ENG-0142`'s remainder, P1, operator-accepted risk) . UNDECLARED 1 (`ENG-0134`) . MENTION 7. Unclassified: 0.

**"NO OPEN P1" IS NOT CLAIMABLE.** Two entries stand in the way and are named: `ENG-0142`'s remainder is OPEN at P1 by the entry's own words under an operator accepted-risk ruling; `ENG-0134` is a P1 whose status was never declared and whose disposition "needs its own act". This lane does not take either act and claims nothing about them.

## 3. THE CLOSURES THIS PR CARRIES, AND THE PROOFS THEY REST ON (a READ at `2c3c39b4`, not a run)

| entry | the proof | measured by this seat |
|---|---|---|
| `ENG-0250` | NA-0768 (`D-1409`): #1808 merged 2026-08-30T17:17:41Z as `63ece4fe`; desktop #47 `ac03da05`; NA-0768's block `Status: DONE`, class `INVITER_FINISH_HANDS_OFF_FLOWN_PASS`, the fourth flight X0-X4 HIT operator-attested | `handshake/mod.rs:1702` `ProvidedSpeculative`; `invite/mod.rs:1573` the offer; `:1661` `invite_finish_hs_unconsumed`; three `na0768_*` test files present |
| `ENG-0251` | the same repair on the entry's second surface (the finish-scan's treatment of the frames it declines); X1 HIT | as above; the lease-law re-derivation (R1(b)) NOT found at n=2 needles (`lease law`, `lawful tick`, `tick period`) -- appended OWED beside NA-0768's block |
| `ENG-0252` | NA-0771 (`D-1412`): #1805 merged 2026-08-29T22:02:15Z as `25cdb923`, class `HANDSHAKE_PENDING_INVARIANT_PASS`; tightened by NA-0775 (`D-1418`, `RULING_NA0775_008` sec 2) | `hs_pending_clear` call sites at `2c3c39b4`: THREE (`:1841`, `:1858`, `:2408`), each after a stored session; per-merge counts 25cdb923 4, 63ece4fe 4, 8b0a3efc 3, 244ea846 3, 2c3c39b4 3; the guard `na0771_g_clear_sites_are_three_and_named` at `tests/na0771_eng0252_arms.rs:824`; registered in `QSC_SHARD_MANIFEST.txt` and `QSC_SHARD_MANIFEST_MACOS.txt` |
| `ENG-0280` | desktop #51 merged 2026-09-02T02:50:11Z, merge `83019356` (base `b6d9237a`, head `e591cce4`) | `.github/workflows/ci.yml` at `2411bf9f` sha256 `8d89373e6de596c200233448e9ada23676af42bc084f273f06702f81e9e62ecc`, 161 lines |

## 4. D1 CLOSED -- THE OPERATOR RECORD'S OFF-HOST COPY, BY SHA

`QuantumShieldLabs/qsl-record` (private): `87e91e3060a71c338040b904d4ff68070a72c575` (D1, 28 exclusions), `d6f3b5ac02180c14103a778a3a3e7a7272d00899` (D1-2/D1-3, 39 declared exclusions, archives barred); the hook at qsl-ops `b577fb1e7f31260019e8cc97c46bf545f3f865f0`, `hooks/qsl-record-pre-push` sha256 `c4a1aa58253e5fe0ea995a73e84e1e51f1c0aceedf0f7fd08d08341183dad8c3`, measured identical in the clone, the apparatus and the remote by this seat (`STOP_NA0778_001` sec 9.2). The six D1 artifacts by sha are in `D-1421` DV-7; the rows are `docs/ops/PREDICTION_LEDGER.md` 440-443.

## 5. THE RECOVERED AS-BUILT, AND THE WRITE-HALF OF `WF-0087` AS AN INSTANCE (`RULING_NA0778_001` R6)

`docs/governance/evidence/NA-0771_as_built.md` -- cited by `D-1412` AB-5, never committed on any ref (`git log --all` over the path: 0 commits at `2c3c39b4`), found ONLY in the frozen old work tree at the NA-0771 seat's checkout, where `git check-ignore --no-index -v` names `.gitignore:65` `**/evidence/`. Carried by this PR BYTE-IDENTICAL: sha256 `5d31f6a683e3dc538500f11c858457eb0fd6fade4316e5458be142e996b5f832`, 6521 B, 110 lines, mtime 2026-08-29; `git add -f`; content untouched -- it is the NA-0771 seat's artifact. A correcting line is appended at the end of `D-1412`'s block.

NA-0768 and NA-0772 have no as-built in the tree, on any ref, or anywhere on the box (census of the old work tree: exactly one uncommitted `NA-07xx_as_built.md`, NA-0771's), and no D-record cites one (`as_built` mentions in `D-1409`: 0; in `D-1415`: 0). Nothing dangles; their evidence is their stop chain, on the box and in qsl-record. Recorded here as an instance under `WF-0087`'s write half -- the same `.gitignore:65` rule that defeats `git add` -- and NOT as an owed act.

## 6. THE FIRST ACTS, MEASURED (STOP 001 secs 1-4, carried by sha above)

The three Director artifacts sha-verified BEFORE reading, all HIT; banked 444, cmp-identical, the seal proven on both arms. `RULING_NA0777_016` and `RULING_D1-3` found already handed and banked at their packet shas. `lanes/NA-0777` already closed by the cutover seat; `lanes/SR15-NA0777-read` and `lanes/SR15-NA0777-S5-read` closed by this seat after their one file each matched the banked copies by cmp (`9d44a872...b51aeb`, `10b12155...7f54ca`). The two 09-01 banks at their full shas. The stale protocol mirror refreshed. The lane id derived on declaring forms across all three input sets with both controls and the sentinel classified. The hook proven on both arms in delta mode; its new-ref refusal measured and filed (`WF-0104`).

## 7. WHAT IS NOT CLAIMED

No product byte, no test run, no build, no relay contacted, no GUI driven. The three closures are the Director's rulings on source measurements and operator-attested flights recorded in `NEXT_ACTIONS.md`; NA-0768's and NA-0771's arms are green on the authority of their merged PRs' checks. The pin's proof is the operator's run. The P1 board is exactly section 2's table. This file grows at the close with the retrospective, the flight script's outcome, the result class the Director declares, and the record-push proof.
