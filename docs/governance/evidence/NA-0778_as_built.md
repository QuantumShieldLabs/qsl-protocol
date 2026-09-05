# NA-0778 -- AS BUILT (BORN AT THE PROMOTION): THE INVITATIONS LANE

Goals: G4 (primary), drives G1-G3 delivery

Lane `NA-0778` (the operator's "Lane C") . `D-1421` . class: **`%s`** (declared at the close by `RULING_NA0778_016` R104; sec 8).

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

## 8. THE CLOSE (2026-09-05): THE CLASS, THE MERGES, THE COMMITS

Class: **`INVITATIONS_PAGE_FLOWN_SHOW_ONCE_HELD_PASS`** -- declared by `RULING_NA0778_016` R104, with its sentence: the invitation flow completed both ways WITH THE INVITER AWAKE (a design fact; the visibility it owes is `ENG-0296`). Desktop PR #54 merged as `9ff557ba01ea36252692c17d0a738b7ce04220a4` (parents `0b87209b` + `10764117`, `merged_at` `2026-09-05T03:43:33Z`); the nine commits b67b95db, 4c7b0c76, a364ba0f, c455ed2e, 14079140, d3a02986, 826f3519, c5ef502e, 10764117; product bytes on main equal `c5ef502e`. Desktop PR #55 (the harness rule) merged as `b4ec469339fb02d80adde277a909e25dd50cfcae` (`merged_at` `2026-09-05T04:10:17Z`). Protocol main `f32a4c20fc186fda0829d475097712cf63ed41e8` (the promotion, PR #1816) unmoved by this lane until this records PR.

## 9. THE RULINGS -- NAMED, NOT COUNTED (each sha-verified BEFORE reading; banked 444 under `state/operator/NA-0778/`)

| document | bytes | sha256 |
|---|---:|---|
| `RULING_NA0778_001_20260903.md` (R1-R10, the formalization) | 13721 | `0e04e22135c682f105dbe18203b93d5bbe3bda2560f682a555e3d3ac862459d1` |
| `RULING_NA0778_002_20260903.md` (the promotion) | 6937 | `a2d1b87f0d24daae352fe2705ab91ccd527ac0493f42cb181b2fc20dc555f7d1` |
| `RULING_NA0778_003_20260903.md` (R12-R20, the measurement stop) | 6298 | `cf52aeb4e95717a4895553a0502d14a250e4ad6ad14b0931717354eb31b4c056` |
| `RULING_NA0778_004_20260903.md` (R21-R28, the build order) | 9502 | `c8ba9539558516c3a355531ab01d65f3981fb209b2aef11ac1f3bd469173e573` |
| `RULING_NA0778_005_20260904.md` (R29-R36, the SR-15 read drawn; HOLD) | 9725 | `9b32d3080a29b3e19f72f5c97f45be8018b58d34cef8836158dbb947948ec174` |
| `RULING_NA0778_006_20260904.md` (R37-R46, the read's cures) | 10173 | `5269d741c493e7f328becce542ed1e53b489cf752c354d402e8a482a5da13529` |
| `RULING_NA0778_007_20260904.md` (R47-R51, the data commit; the flight) | 6267 | `9b67a6c8e01742dd1ffa69891e3f80db234820cd16dd4965fcb02e8bf409b99b` |
| `RULING_NA0778_008_20260904.md` (R52-R57, the operator's first flight rulings) | 9910 | `af54e5be94181c88767dff91d1012cc87d8f58db094bd24ab34dda8abdd78109` |
| `RULING_NA0778_009_20260904.md` (R58-R64, the audits' grammar) | 7094 | `05f8eb888806fdf878ac0675f5610665023b7c48da68f722a6eb7fa59539a590` |
| `RULING_NA0778_010_20260904.md` (R65-R72, the reproduction confirmed) | 7281 | `db0b57c90039e97fe40c483098cb3d55e937edb07f458e269da02c942884d03d` |
| `RULING_NA0778_011_20260904.md` (R73-R78, the second read; the sixth commit) | 6512 | `006e1a9e11ba4bb510cd236a16fce69b71f66f274a8873eb7b3f4f4126404d8c` |
| `RULING_NA0778_012_20260904.md` (R79-R83, the third read reconciled) | 4406 | `7397fe13c04135441d9190594fb44e013d6eadedf7b6844e7608b124001b6ee9` |
| `RULING_NA0778_013_20260905.md` (R84-R90, the last read; the seventh commit) | 7124 | `ea17d5836b3c298fb8a28de17c0e336f5b42fa8c3bdd16e079977d51ce59e665` |
| `RULING_NA0778_014_20260905.md` (R91-R93, the final flight) | 5940 | `9acf74cd67169b8f259d0acd289060d7ee9cce64829fe4e033e9d9f06a19a3e7` |
| `RULING_NA0778_015_20260905.md` (R94-R103, the merge and the close order) | 7499 | `3ccc2bdf410978878b1846f131909e802eec71f76786108eac0484439710b715` |
| `RULING_NA0778_016_20260905.md` (R104-R111, the close-out ruled; the class declared) | 6182 | `2763afe2f781fd214e914a53ddd510a8a374b644ef552bc0136a60ebec510ef8` |

## 10. THE STOPS

| document | bytes | sha256 |
|---|---:|---|
| `STOP_NA0778_001_20260903T200355Z.md` (the formalization) | 145983 | `5a9b7ebc6b6ae4fce588bcc960269d3e0a662567dfd02b570a052bf1ed40e908` |
| `STOP_NA0778_002_20260903T205107Z.md` (the promotion PR) | 132686 | `8af91f451d31743b7abe7be5931fcd7e39cbf8f035d8df7a94382375f790b8ae` |
| `STOP_NA0778_003_20260903T221155Z.md` (the L3/L5 measurements) | 104254 | `178fa7b021f947c0c512599b2a317f55e95005be6c6ae4fb42188b1168a75476` |
| `STOP_NA0778_004_20260904T014054Z.md` (the build) | 149155 | `1e9f5ea1600ddb02d77615eecb328a81b7481d3fd68d7d55df2db53b90fd5987` |
| `STOP_NA0778_004HOLD_20260904T143026Z.md` (the HOLD for the read) | 30180 | `eda4fac84cd2a2c63479614118186b2e5a587e3f0f184903b303731a61c69dbc` |
| `STOP_NA0778_004a_20260904T153648Z.md` (the second commit) | 206080 | `7b7c947a0ad0c35892cc7910917963a5ee162410687634831112417e1d52d780` |
| `STOP_NA0778_004b_20260904T162010Z.md` (the third commit (data)) | 78589 | `8d46135abda5a2f0182594dde951bb1da80c26e6f88a76eaca782dd8f5aade2a` |
| `STOP_NA0778_004c_20260904T205744Z.md` (the flight incident and the fourth commit) | 168962 | `fa6e8b22b2476b87fc61dfcf5f802aedc10a87437349f8f3b3655ba63ae18560` |
| `STOP_NA0778_004d_20260904T214403Z.md` (the fifth commit (the grammar)) | 82991 | `14c3fe7419e2ed7f9fe018c85c6fff7155cd16a004016c898a129a6948218fbd` |
| `STOP_NA0778_004e_20260904T221402Z.md` (the reproduction against the real relay) | 65386 | `886d5d64aa60f406acd9b623eda1f3876eb285c21b87a75d67b42386a66fcdf4` |
| `STOP_NA0778_004eHOLD_20260904T230457Z.md` (the HOLD for the delta read) | 89221 | `729c8f7b51592bb865f17e71aff0c370b44b1e4de09e73b53d41f8856c04ef45` |
| `STOP_NA0778_004f_20260904T235001Z.md` (the sixth commit (the guards)) | 73005 | `5d698a850e40e7eeb679d940fd5452760316fad324f8bb1f43e8a5db41502ef9` |
| `STOP_NA0778_004g_20260905T003251Z.md` (the seventh commit (two lines); the final script) | 47523 | `feef88b6883391378021d0f9c9e57c429261f39fd6c1077699edc8c0bf5e6827` |
| `STOP_NA0778_004h_20260905T030532Z.md` (the eighth commit (the nudge removed)) | 39455 | `edb8b9cfd4b4a94bd0933e134ca4d9dc6c0996346b51f02293a6352ee1f1426c` |
| `STOP_NA0778_004i_20260905T033941Z.md` (the ninth commit (data; the CI race)) | 46687 | `f6b17932d864e2dd26e0b3f1aa3ad16d2887655100d4aec7e4fc12a651bb0312` |
| `STOP_NA0778_005_20260905T040142Z.md` (the close-out) | 56305 | `19afb0aaa9bd0c7a19980464f917cc44452eabd590c2763f1495e67ea71499dc` |

## 11. THE COLD READS (four reads by three fresh instances) AND THE OPERATOR'S BANKS

| document | bytes | sha256 |
|---|---:|---|
| `FINDINGS_SR15_NA0778_read_20260904T144353Z.md` (the first read, at b67b95db) | 33557 | `7ac52a21f803412121cbde99804a3e67ac1b619835d12788f245794d72a459bf` |
| `FINDINGS_SR15_NA0778_reread_20260904T154727Z.md` (the same instance's re-read of the delta to 4c7b0c76) | 23661 | `b5ab951aeaa93ed55a0a18eccf1d3705e04bbf282208fd860404dc061815b279` |
| `FINDINGS_SR15_NA0778_read2_20260904T225717Z.md` (the second reader, the delta to 14079140) | 40326 | `9a034f5ba6950caf88116275f455d06cdfd663ce98665cd9d9d01a0365d6eaa6` |
| `FINDINGS_NA0778_R70_delta_read_20260904T230048Z.md` (the R70 reader, fresh, the delta to 14079140) | 48663 | `63ae6b6989e8b3d7324b8f958cb144a726816a9a2aed962206c2db4592ce76f1` |
| `FINDINGS_SR15_NA0778_read2_last_20260905T000820Z.md` (the second reader's last read, the delta to d3a02986) | 34662 | `417357f8ca8a4aa2acd339bbe1862f0490d688a0f5b591df889c80f03d0fbfa1` |
| `RBANK_flight_rulings_NA0778_20260904.md` (the operator's first flight rulings) | 11677 | `7a7d781003b85f0a16e040d34a07dccff276802ba6904ae7f684ce2bf5278461` |
| `RBANK_final_flight_rulings_NA0778_20260905.md` (the operator's final flight report, verbatim, one redaction) | 8579 | `16b4f4554e930df3df483b5f7e8b311a9948e87e709c58bd5da1f7a1b899289b` |
| `RBANK_debug_log_first_and_no_presence_20260904.md` (the debug log first; no presence signal, ever) | 3871 | `785e9e2fc5e5c80bcdb639cc72c75ae4cb9ff4881e33ecef4ab57643dd9828d7` |
| `RBANK_debug_log_extensive_audience_20260905.md` (the debug log extensive; three audiences) | 3087 | `2f6356bc7d1aed839275cc359204c09183c18f9e2ab181b202e6e5b69215d0f5` |
| `TRIAGE_AND_PLAN_audits_2026-09-04.md` (the audits' triage and plan, program context) | 18838 | `e05f9401d1272782bed6bd7c1c3b1e06dadc7c8d6c7026bb2c943818b429db25` |

## 12. THE FILINGS LANDED AT THE CLOSE (ids derived at the edit; the operator ratifies the severities by merging)

`ENG-0287` the Received source (P3) . `ENG-0288` the invitation -> contact link (P3) . `ENG-0289` the f_d race, cured (P3) . `ENG-0290` no clear verb (P4) . `ENG-0291` the focus/overlay family (P3) . `ENG-0292` the redeem window hides an outcome (P3) . `ENG-0293` the lock order (P3) . `ENG-0294` the silent accept (P2) . `ENG-0295` the grammar's engine half, with the audit's D-7 (P2) . `ENG-0296` paused inviter / stranded invitation / wedged relay, no presence signal (P2) . `ENG-0297` the relay client's timeout (P3) . `ENG-0298` the cap purge candidate (P4) . `ENG-0299` the write-back class (P3) . `WF-0105` fly the first head first (workflow) . datapoints under `WF-0090` (E-3) and `WF-0103` (the f_k race; duty 11 of the harness at `b4ec469339fb02d80adde277a909e25dd50cfcae`). R92's design ruling and R-8's words recorded beside the contact-management item in `D-1421`'s close block; R93 benign by design, recorded; the unreachable list view a hygiene note.

## 13. THE ROWS

`docs/ops/PREDICTION_LEDGER.md` rows 427-445 (the promotion) and 446-484 (the close): S-1..S-29 against the seat, D-1, D-2, D-4..D-13 against the Director, the readers' HITs, the operator's five flight rulings.

## 14. WHAT IS NOT CLAIMED AT THE CLOSE

The earlier legal-name stall is explained by mechanism (the paused inviter) and corroborated by the second flight, NOT reproduced on the inviter's other machine; the relay-side cause is not supported and not excluded for that hour. The two measurements of R94 are by reading and by the reproduction's beats, not by the operator's profile. Records only in this PR; nothing merged by the seat; the record push is the last act, after this PR's merge and the housekeeping.
