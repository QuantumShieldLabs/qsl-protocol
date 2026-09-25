C07 -- ROLLBACK AND DURABILITY -- FINAL (every row ACCEPTED; CONTRACT_ACCEPTED_WITH_NAMED_FIXES)

PUBLICATION NOTE: this file is C07_FINAL.md of the operator record (sha256
ea0d7f5e3b2bf2a876cec57a1e00f3deec114217aee023e3dc5bd4080ed58920) sanitized for this public repository: operator-record
and lane paths are replaced by the cited artifact's name and sha256 (or a bare evidence file name), and hardware model
strings are withheld. No rule, value, marker, status or measurement differs from the record copy.

==============================================================================================================
FIXES / HISTORY  [final: d]
==============================================================================================================
RESULT CLASS: CONTRACT_ACCEPTED_WITH_NAMED_FIXES. Every row of this contract is ACCEPTED by the Director's
RULING_NA0784_phase4_and_C07_ACCEPT_2026-09-25.md (sha256 f3c85a67d2c2bc91b64de4b1d9bdb2ffb0c0c2c211f4254093e75a36bdd821e5;
"RACC" below). No third SR-15 read (Director discretion, stated in RACC): every edit since rev2 is Phase 4 measurement or
text RACC names. CONSEQUENCE (RACC; RULING_NA0784_C07_rev2 V4): no machine is QUALIFIED today (T2 PL8); the TPM profile is
offered to nobody until a machine passes the C7-O7 disk-image restore gate and PL8.
| Step | Artifact (sha256) | Effect |
|---|---|---|
| 1 | C07_DRAFT.md 14a1ae4ba207fe48f9d30178561b88674e384f5c0a8be3cccb202602847fcfaa (phase 3) | first draft; every row PROPOSED |
| 2 | SR15_C07_FINDINGS.md c7f9dbe1edefce643ebbe8f17042f414867e23a4dec6fe8ed300d67b78ae5296 (fresh fable/xhigh read) | READY WITH NAMED FIXES; no BLOCKER |
| 3 | RULING_SR15_C07_draft_2026-09-25.md 8d42252411ff53cb8e192c68b832d57008e6c1fd135d12e22ba1df2eea7d391d (RSR15 R1-R8) | R2 owner authorization (b)-REFUSE; R3 QUALIFIED (PL8) and the C7-O7 gate; R4 minors m1-m8 and notes (a)-(h); R5 QQ2-QQ8 |
| 4 | RBANK_F02_operator_decisions_DA_DD_2026-09-25.md 1cb96aa6370a713a7ee7da848b82b4ca424cd3804d8e9da94580c878f14c6819 | D-A ACCEPT, D-B OPTIN, D-C TSS, D-D1 NONE, D-D2 LATER, D-D3 SPARE; the Director's E5 condition on OPTIN is carried in T9 and T2 PL7 |
| 5 | C07_DRAFT_rev2.md 49abca551208b6200ba96e9bcc663f8f958fff4018ce0d0e1877a0ce5dab3de6 (DELTA.md cef30d86a71fe7ec3dd47b9fc5a6f1ad2461725dd4ea6684bc91e5fc13a278ea) | rev2 per RSR15 R2-R5 |
| 6 | RULING_NA0784_C07_rev2_2026-09-25.md 016bd55b1b4c371032d2a872b03c9ebe3dcb7dd116f4fbab93b4a4f38bba9c53 (V1-V6) | rev2 = the draft of record; V2 seat values accepted; V3 items carried to this FINAL |
| 7 | Phase 4 (P4): QUALIFICATION_PHASE4.md 21d0398b194f83d20638f1b7584804cfeb0feb52e9a33db7d38982efbd846b3f, REPORT.md f534a89887ded07ddfc6777002cf14bfe21bf5512af46969482faf1a4f3fa686 | NO_DA re-qualification: Q2, Q3.5, Q4.4 GREEN on the ruled property; cleanup GREEN |
| 8 | RACC f3c85a67d2c2bc91b64de4b1d9bdb2ffb0c0c2c211f4254093e75a36bdd821e5 | E1 accepted (ER11 and QQ3 re-spelled); E2 adopted (the anchor_auth_failed mapping); E3 noted (tool session contexts carry the auth; NP1 already forbids them in the product); E4 recorded NOT MEASURED; C07 ACCEPTED WITH NAMED FIXES |
NAMED FIXES APPLIED IN THIS FINAL (each changed row carries [final: <item>]; C07_DRAFT_rev2 is otherwise unchanged):
(a) Phase 4 measured values: NV1 (TPMA_NV 0x02040044 / 0x22040044, TO-BE-MEASURED -> MEASURED), NV4 (the NO_DA-SET
    comparison value), QQ8 (the double guard measured on the NO_DA-SET template); source key P4 added.
(b) ER11 and QQ3 re-spelled with RACC's text: "TPM_RC_BAD_AUTH (0x0A2 + session bits, measured 0x9A2) with no DA-counter
    increment; 0x98E only for a NO_DA-CLEAR index"; E2's mapping in ER11 (anchor_auth_failed accepts BAD_AUTH, 0x0A2 base,
    any session index, as well as AUTH_FAIL); E4's boundary in QQ3 (a NO_DA index in lockout: NOT MEASURED).
(c) RULING_NA0784_C07_rev2 V3 items:
    - the SR-15 reader's citation nits (SR15 H1: T0.2 "86-90" -> 85-89; T0.3 "223-254" -> -253) were MEASURED at M before
      applying and are NOT APPLIED: at M the #[derive] line of VaultPayload is vault/mod.rs:86 and its closing brace :90,
      and write_atomic runs from its fn line fs_store/mod.rs:223 to its closing brace :254; the cited ranges are exact.
    - "224 NV acts by the seats": the figure does not occur in this contract (it is on the decision sheet, D-B). The
      precision, recorded here: the 224 (Phase 1: 108, Phase 2: 116) are NV acts BY THE SEATS; the TPM's own two
      DA-counter writes in Phase 1 are not in that count (SR15 H1).
    - the passphrase-change note: T8.2 W1.
(d) status PROPOSED -> ACCEPTED on every row and header (a token change, not tagged row by row); the contract's own status
    sentences brought current (title, REV2 NOTE, STATUS, H3, T6 heading, T10, T11 preamble, END line); the operator
    decisions of step 4 recorded in the cells they decide (PL1, PL2, T6.3, T10 C7-O1..C7-O3, C7-O7); the Phase 4
    pre-commit condition marked done where it was stated as pending (PL1, T10 C7-O4); this section.

REV2 NOTE [rev2: R1]: this file revises C07_DRAFT.md (sha256 14a1ae4ba207fe48f9d30178561b88674e384f5c0a8be3cccb202602847fcfaa,
which stays sealed) per the Director's RULING_SR15_C07_draft_2026-09-25.md (sha256
8d42252411ff53cb8e192c68b832d57008e6c1fd135d12e22ba1df2eea7d391d) items R2-R5, applying the SR-15 reader's fixes in
SR15_C07_FINDINGS.md (sha256 c7f9dbe1edefce643ebbe8f17042f414867e23a4dec6fe8ed300d67b78ae5296). Every changed or added
row carries a tag [rev2: <ruling item>]. Drafted 2026-09-25 by the NA0784-C07-rev2 seat (opus/high). At rev2 nothing was
accepted; the Director verified the delta (DELTA.md, sha256 cef30d86a71fe7ec3dd47b9fc5a6f1ad2461725dd4ea6684bc91e5fc13a278ea) (R7) and accepted rev2 as the draft of record
(RULING_NA0784_C07_rev2 V5) [final: d].

==============================================================================================================
STATUS
==============================================================================================================
PLAN: QSL-solution-plan rev3 d9016e53d2ab46c32b7a7cb060ea70dc79421617e9b054848b464ba7a5518275 (amended A4, A5), card F02,
THE PLAN row C07 ("Covered restore domain and platform support; independent anchor provider or recorded unresolved
blocker; trust/offline implications; commit/fencing/recovery contract; process-crash versus power-loss claims").
Lane NA-0784, phase 3 of 3 (PART C of the F02 directive draft). Drafted 2026-09-25 by the phase-3 seat (opus/high).
Status of EVERY row: ACCEPTED by the Director's ruling (RACC, FIXES / HISTORY above) after the SR-15 read (a FRESH
fable/xhigh seat, SEAT POLICY v4) [final: d]. Nothing here allocates an identifier: allocation happens only
when the DOC-CAN-003 rows of T7 merge (C01 O1). Three rows needed the OPERATOR (Q6 AMBER, Q7 RED, QQ1 access model):
they are T10 cells C7-O1..C7-O3, DECIDED on the decision sheet DECISIONS_FOR_OPERATOR.md (operator record; answered per RBANK 1cb96aa6).
This contract is F02's investigation output. It is NEVER acceptance of implemented rollback protection (THE PLAN F02);
release stays BLOCKED until F18 proves the implementation.

Scope: THE PLAN row C07 and AMENDMENT A4. Not in scope: C01-C06 values (cited, not changed), the peer-witness strict mode
(X D, not proposed for v1: X D :153, X F :179), any implementation.

==============================================================================================================
SOURCES AND CITATION KEY (every source sha256-verified by the phase-3 seat before reading; recorded in its evidence)
==============================================================================================================
X <sec> :<line>  EXTERNAL_C07_rollback_design_2026-09-23.md 425ff881691d461172bd2a05e9c55c699534d13b7dafcf2a94e8f9610356dbcc
                 (third-party design; F02's STARTING INPUT, not an accepted contract -- A4 Impact).
A4               AMENDMENT_PLAN_A4_D08_C07_2026-09-23.md 0d8b8233b9f885d28a5610f9dc2dfec46b4dc3718f0a5a0ef4593ce61ffa98a3.
DOM              DOMAINS.md (F02 formalization) abe70f75dafa54160aefaac94851c4daa5bb8349e9868a1a7a46775074549ed9.
QP :<line>       QUALIFICATION_PLAN.md (F02 formalization) a0d8a5ec1d94c946cb212098704c65c7ed19bd02fd67da11fe3cb3c7dc17210b.
R<n>             RULING_F02_formalization_2026-09-24.md 7d268aaee5798dd5e5301ca8f25648e734b5ce333003cd4ab8cfe3660f74ac14.
B4/B6            RBANK_F02_operator_B4_B6_2026-09-24.md 3f8079c89df0d49c4aac61ad40a600718bbd615cdb2e7c4f60eb71e0916e9e03.
S4-DEC           RBANK_F02_phase1_decision_2026-09-25.md 48e863c861dfd8e40b7ade014ee8448fdd6fda89df73f67aa84f58e4d7a5bfcf.
ENC              RBANK_F04_encoding_schema_version_2026-09-24.md fcb7bbd5c438e913c8608ed8f6e960ddffca9bfed910dccf84862007f5a88fe4.
P1 <Qn|En>       Phase 1: QUALIFICATION_PHASE1.md 065267c5f99bc80510d3b959e0418ed38234346d6f015362d705790e33c100de
                 and REPORT.md 93e1b1d934336e2e53aa2e1bd974bcf9fb52f90a0d22d936829ed2e4496b5eb0 (escalations E1-E6).
P2 <row|En>      Phase 2: QUALIFICATION_PHASE2.md d487055393d97100ad549bf8e314be82281a23f7c9bb7cf5c257b305a152e000
                 and REPORT.md 2224b933bfcb76504f6e84a4e20bea5e8b0a0785f384faa615751059f299bd86. Part B row ids (G, T1, K-*,
                 I1, I2, F1, F2, R-CA/CC/CS, D1, D1b, D2, T-END) are the sealed EXPECTATIONS2.md rows
                 (5252a6b2aa2577cccb97980df2cb3f602ea1344a03f8e1d1ff5f11a41180b8c7), results RESULTS.tsv of the prototype run (56/56 PASS).
P4 <Qn|En>       Phase 4 (NO_DA re-qualification): QUALIFICATION_PHASE4.md
                 21d0398b194f83d20638f1b7584804cfeb0feb52e9a33db7d38982efbd846b3f and REPORT.md
                 f534a89887ded07ddfc6777002cf14bfe21bf5512af46969482faf1a4f3fa686 (escalations E1-E4) [final: a].
LAP              the F02 inputs of the operator record: laptop getcap f126f2a336d08845d6046c24445b1211e041f387cd4872d4662cb3ee7e61e0ee,
                 laptop L1-L5 dbc04160cd2ddec2980c4568cb5a2e283774271ee344bdaea0ea8bb5cf6ff6da (READ-ONLY data point, R3).
M <file>:<line>  qsl-protocol main 0159d423f81dccfb9af27d360cc6f9734e382f21 (bare ls-remote 2026-09-25T04:01Z; mirror fetched),
                 path qsl/qsl-client/qsc/src/<file>. Writer census: writer_census_main.txt (phase 3); C07 absence probe with
                 positive control: main_c07_absence.txt (phase 3).
C01/C04/...      docs/ops/contracts/C0n_*.md at M (C01 FINAL + AM-1..AM-4; C04 FINAL; C05 FINAL; C06 boundary).
CAN12            docs/canonical/DOC-CAN-003_QSP_Suite-2_True_Triple_Ratchet_v5.0.0_DRAFT.md sec 12 at M (rows A13, A14, A16,
                 A23, A24; subsections 12.1-12.7).
ENG-0365/0366    docs/ops/IMPROVEMENT_LEDGER.md at M.
TCG P1 <sec>     TCG TPM 2.0 Library Part 1 Architecture v185 (2026/03/12), fetched this phase from the URL X Evidence :185 names,
                 TCG_TPM2_Part1_v185.pdf sha256 c2604f79a833b7bfa557c0ab2f880458e6af0993826ed48f22c41c34a6c0daad.
                 The box's TPM reports spec revision 1.38 (P1 Q0); section text is quoted from v185 and its applicability to a
                 rev-1.38 part is the spec's own backward-compatibility claim, NOT measured here.
TCG P2 <item>    TCG TPM 2.0 Library Part 2 Structures v185, fetched by the SR-15 reader (public, read-only), copied to
                 TCG_TPM2_Part2.pdf (rev2 evidence) sha256 67a6786816d6ef36952e910750652c0fb4668926369be81954dcf0f63c012b77;
                 table TPMA_NV bit 25 TPMA_NV_NO_DA quoted at P2.txt:10190-10195 (the text extract of that PDF) [rev2: R5 QQ3].
SR15 <sec|m#>    SR15_C07_FINDINGS.md c7f9dbe1edefce643ebbe8f17042f414867e23a4dec6fe8ed300d67b78ae5296 (the reader's findings, not
                 rulings); RSR15 R<n>: RULING_SR15_C07_draft_2026-09-25.md 8d42252411ff53cb8e192c68b832d57008e6c1fd135d12e22ba1df2eea7d391d
                 (the Director's ruling on them) [rev2: R1].
BC               bytecount.py (phase 3) b6da4732d4fdbad5ef60bce46e377dde6c5e84f024d701f2925f79c4e5a7a241 and its output
                 bytecount_output.txt 035975fcc7dfcad23e58a1998d4b69b94b9371ecb6bc142c9bc0316b33501ec3 (arithmetic only).
[U]              a claim the external design ITSELF labels unverified (U1-U8, DOM "CLAIMS THE DESIGN LABELS UNVERIFIED"),
                 kept [U] unless a Phase 1/2 row settles it (then the row is cited and the mark says SETTLED BY).
NOT RUN          no measurement exists; the row states what would establish it.
Measured facts come from ONE box (the project's build box (Intel PTT, spec rev 1.38): "INTC", fw 0x01930001,
Ubuntu 24.04.5, ext4). Every "measured" below means measured THERE, by the named row, and nowhere else.

==============================================================================================================
T0. WHAT EXISTS TODAY AT MAIN (measured, not design)
==============================================================================================================
| # | Fact | Evidence |
|---|---|---|
| T0.1 | No C07 artifact exists in the engine: 0 hits for QSLFRESH, QSL-C07-VAULT, NV_Extend, nv_extend, protection_mode, vault_id, freshness, XDG_STATE_HOME (1 hit for "predecessor", an unrelated comment M quarantine/mod.rs:76); positive control VaultPayload = 5 hits | main_c07_absence.txt (phase 3) |
| T0.2 | VaultPayload { version: u8, secrets: BTreeMap<String,String> } -- no lineage, generation, anchor or profile field; serialized compact by serde_json::to_vec | M vault/mod.rs:86-90, :298, :353, :647 |
| T0.3 | One write primitive: tmp + write_all + sync_all + rename + fsync_dir_best_effort; the directory-flush result is DISCARDED (`let _ = File::open(dir).and_then(\|d\| d.sync_all())`). Vault init has its own copy with the same discarded flush | M fs_store/mod.rs:223-254, :252, :492-493; M vault/mod.rs:745-777, :771; ENG-0365 |
| T0.4 | Authoritative state is written by write_atomic in 9 source files outside the vault file (the full census is T8.2) | writer_census_main.txt (phase 3); ENG-0366 |
| T0.5 | => TODAY NO DOMAIN OF T1 IS DETECTED at main: a restored older vault file opens normally. Every T1 claim below is an ACCEPTED contract for F03-F05, evidenced only by the toy (P2 Part B) and the TPM measurements (P1/P2) | T0.1-T0.4; DOM row 0 |

==============================================================================================================
T1. COVERED RESTORE DOMAINS PER PROFILE  (item 1)
==============================================================================================================
Meaning of DETECTED (X B :106): stale continuation is detected and refused before protected effects; the restore itself
is not prevented. TPM-profile rows assume enrollment from genesis, a trusted current client/OS, confidential vault/NV
secrets and a QUALIFIED anchor (X B :106; QUALIFIED is defined by T2 PL8 [rev2: R3(1)]). "Toy" = the P2 Part B prototype: random opaque blobs, HMAC tag in place of the
AEAD, NV auth held in a file OUTSIDE the blob, no QSL code path (P2 REPORT sec 8). A toy PASS establishes that the A4
order and the A5 table, as implemented in the toy, behave as sealed; it establishes nothing about production.
| Row | Domain | LOCAL-CHECKPOINT profile (ACCEPTED claim) | Evidence (local) | TPM profile (ACCEPTED claim) | Evidence (TPM) |
|---|---|---|---|---|---|
| D1 | VAULT-ONLY ROLLBACK: the vault (and/or prepared slot) replaced by an older authentic copy; the checkpoint survives | DETECTED / PREVENTED: FREEZE freshness_generation_regression (older generation) or committed_state_missing; checkpoint binds generation AND digest d | X A1 :23, A2 :39, :43-48, B :110. MEASURED ON THE TOY: P2 B3 D1 LOCAL FREEZE "authenticated generation regression" | DETECTED / PREVENTED: FREEZE committed_state_missing (authenticated TPM head matches neither blob) | X B :110. MEASURED ON THE TOY with the real TPM: P2 B3 D1 TPM FREEZE "matches neither" |
| D1b | SAME-GENERATION ALTERNATE: an authentic prepared-but-uncommitted blob at the committed generation substituted | DETECTED: FREEZE freshness_digest_conflict ("generation equality alone never suffices") | X A2 :48. MEASURED ON THE TOY: P2 B3 D1b LOCAL FREEZE "conflicting state digest (same generation, different blob)" | DETECTED: FREEZE committed_state_missing | MEASURED ON THE TOY: P2 B3 D1b TPM FREEZE |
| D2 | WHOLE-MACHINE RESTORE, SAME MACHINE: vault + checkpoint + config restored together; OS/app trusted, secrets confidential | NOT COVERED. The profile label is DISCLOSURE, not detection | X "Decision and scope" :7, B :111-112; A4 Counterexample. MEASURED ON THE TOY (the discriminating negative): P2 B3 D2 LOCAL OPEN(7), NOT detected -- the expected result | DETECTED / PREVENTED while the independent TPM state survives: the TPM head is past the image's anchor | X A1 :19, B :111-112. MEASURED ON THE TOY: P2 B3 D2 TPM FREEZE "matches neither" (TPM advanced 2 generations past the image). The toy restored the lineage's FILES only; a real disk-image restore was NOT RUN: it is the C7-O7 release gate, and the claim rests on the anchor's NV medium lying outside the disk restore domain (T2 PL7, PL8) [rev2: R3(2), R3(4)] |
| D2x | OLD IMAGE MOVED TO ANOTHER MACHINE | NOT COVERED if its old checkpoint is imported too | X B :119 | PREVENTED BY REFUSAL: enrollment unavailable (different storage-primary Name / index absent) | X B :119. Partial measurement: P1 Q3.4 negative control (a primary Name mismatch refuses the session before any NV command). Cross-machine: NOT RUN |
| D2t | TPM CLEARED / REPLACED / INDEX DELETED OR RECREATED WITHOUT ITS SECRET | (not an anchor in this profile) | X B :118 | PREVENTED BY REFUSAL (not classified as proven rollback): anchor_missing / anchor_identity_changed / anchor_auth_failed; never auto re-enrolled | X A3 :66, B :118. MEASURED: P1 Q4 (recreated index: original secret refused 0x98E; attacker reproduces value AND Name, only the secret distinguishes); P2 T-END (index undefined -> FREEZE "hardware anchor missing", not re-created). TPM CLEAR: NOT RUN (Q11 declined, B6) |
| D3 | PROCESS CRASH at any transaction boundary (kernel and page cache survive) | RECOVERED by the A5 table (T4.3); no effect released before its commit | X A4 :76-81, A5 :89-98. MEASURED ON THE TOY: P2 B2 LOCAL 13/13 (G, T1, T1b, K-B2, K-M2, K-A2, K-M3, K-A34, K-M5, K-A5, K-A6, F1, F2) | RECOVERED by the A5 table against the authenticated TPM head; an interrupted extend is an indeterminate commit resolved by authenticated read, never re-extended | MEASURED ON THE TOY: P2 B2 TPM 17/17 incl. K-X4 (kill between NV_Extend and readback: PROMOTE, exactly 1 extend), I1, I2 |
| D4 | POWER LOSS (unflushed data lost; the TPM's own NV write may be interrupted) | Exact-blob recovery ONLY IF the anchor commit and the matching blob both survive AND flushes are honored; otherwise fail-closed, no availability guarantee | X A5 :100, B :120-121. NOT RUN (Q10 declined, B6). BLOCKED on main by ENG-0365 (T8.1) | Fail-closed refusal after anchor loss; automatic recovery NOT COVERED; a hardware head that REVERTS to an older valid head would break protection | X A5 :100, B :121; TCG P1 34.7.1 ("A TPM is not required to maintain the integrity of the data in an NV Index if a power loss interrupts the write ... the TPM should indicate that the Index no longer exists"). NOT RUN (Q10 declined, B6) |
| D5 | PRE-UNLOCK STATE (unlock-failure counter and attempt limit) restored to an older copy | ACCEPTED OUT OF ROLLBACK SCOPE, DISCLOSED (T8.3) | R5; DOM last section; M vault/protection.rs:512-534 | same | same |
| D6 | RELAY-CONTROLLED / OFFLINE / COMPROMISED EXECUTION | as X B :113-117 verbatim; no new claim | X B :113-117. Relay replay completeness [U] (U5) | as X B :113-117 | X B :113-117 |
Carried [U] marks (DOM U1-U8): U1 end-to-end implementation [U]. U2 vault schema allocation [U] -> ACCEPTED here (T6), SETTLED
only when T7 merges. U3 whole-path nonce freshness [U] (measured today: OsRng 96-bit nonce per write, M vault/mod.rs:69-73,
:303, :358, :500, :631; the bound over a vault lifetime is NOT analysed). U4 writer inventory/coverage [U] -> ENUMERATED
at main in T8.2 (the enumeration is measured; COVERAGE stays [U] until F03-F05 implement it). U5 replay-check
completeness [U]. U6 TPM provider/lifecycle/recovery UX/integration [U] (the provider is ACCEPTED in T3; lifecycle
partially SETTLED BY P1 Q2-Q4 and P2 Q5; UX and integration [U]). U7 hardware anchor capability: SETTLED BY P1/P2 ON THE
BOX for authenticated reads (P1 Q3), provisioning permissions (P1 Q2), delete/recreate refusal (P1 Q4), persistence across
one warm reboot and one systemd power-off (P2 Q5), rate/backpressure (0 NV_RATE in 216 successful extends: P1 103 incl. the Q6 loop of 100, P2 113
incl. the in-process 100), throughput (P1 Q6, P2 in-process); NOT settled: endurance (P1 Q7 RED), TPM clear (NOT RUN), power cut
(NOT RUN), any other machine. U8 fresh-identity recovery integration and retention/replacement UI [U].

==============================================================================================================
T2. PLATFORM SUPPORT  (item 2)
==============================================================================================================
| Row | Platform | ACCEPTED status | Evidence |
|---|---|---|---|
| PL1 | The build box: Intel PTT ("INTC", Intel firmware TPM inferred from CRB + 8th-gen CPU; spec rev 1.38, fw 0x01930001), /dev/tpmrm0, Ubuntu 24.04.5, bare metal, ext4 | QUALIFIED FOR CORRECTNESS: Q0-Q5 GREEN, Q8 recorded, Q9 GREEN. TPM profile OFFERABLE here only after the operator disposes of Q6 AMBER and Q7 RED (C7-O1, C7-O2) and the access model (C7-O3) -- DISPOSED: ACCEPT / OPTIN / TSS (T10) [final: d]. NOT YET on the PL8 list: its NV medium is UNMEASURED (PL7); the C7-O7 disk-image restore test is the discriminating measurement, and the Phase 4 NO_DA re-qualification (T11 QQ3) ran GREEN before the commit PR (P4 Q2, Q3.5, Q4.4) [rev2: R3(1), R3(2), R5 QQ3] [final: d] | P1 Q0-Q4, Q8; P2 Q5, Q9, STEP 6; S4-DEC; P4 |
| PL2 | The operator's laptop (model withheld): Ubuntu 26.04.1, bare metal, tpm_crb, TPM 2.0 spec rev 1.16 (2016), manufacturer INTC, fw 0x12F000C, 96 commands, TPM2_CC_NV_Extend 0x136 and NV_Read 0x14e present, NV_DefineSpace 0x12a present, NV_COUNTERS_MAX 16, ownerAuthSet 0, lockoutAuthSet 1, MAX_AUTH_FAIL 31, sha256 PCR bank not allocated (irrelevant to NV Extend) | READ-ONLY DATA POINT, NOT QUALIFIED. No TPM command was run on it by any F02 seat. Before it (or any machine) is claimed, it passes the same Q0-Q9 set; laptop qualification (L6) is the operator's optional call (R3) -- DECIDED: LATER, before release, the same tests as the box (RBANK D-D2) [final: d] | LAP (getcap lines 7-9 revision, 14-16 manufacturer, 31 fw, 51 NV_COUNTERS_MAX, 99-104 ownerAuthSet/lockoutAuthSet/inLockout, 127 MAX_AUTH_FAIL, 244-246 NV_DefineSpace, 332-334 NV_Extend, 563-565 NV_Read); R RULING_F02 "Director reading" |
| PL3 | Any other Linux machine with TPM 2.0 | UNQUALIFIED until its (manufacturer, firmware version, spec revision) tuple is on the PL8 list (the qualification is the gate, not the device node: A4 "Measured fact"); an unlisted tuple refuses enrollment with anchor_unqualified (ER21) [rev2: R3(1)] | A4; QP :70-204; SR15 M2 |
| PL4 | vTPM / software TPM whose state can be snapshotted with the disk | EXCLUDED from the TPM profile (may test transitions; establishes no hardware rollback resistance) | X A3 :62, E step 3 :167 |
| PL5 | Non-Linux | OUT OF SCOPE (the design is Linux desktop only) | X header :3 |
| PL6 | Filesystems | The box: vault dir and default checkpoint home on ONE ext4 filesystem (so the checkpoint separates only the vault-file-only domain). Flush honesty of the device: NOT RUN. A device or filesystem that does not honor flushes BLOCKS the durability claim | DOM row 0 (S1_14); X A2 :35, A4 :85 |
| PL7 | The external-NV caveat for firmware TPMs | A firmware TPM keeps NV outside the TPM core; the spec requires such NV be "encrypted, integrity checked, and rollback protected" (TCG P1 34.7.2.1). Whether Intel PTT meets that is [UNVERIFIED]: no Intel document found (P1 Q7 sources) and not testable without destructive acts. WHERE the fTPM's NV lives (platform flash is believed; no F02 row measured it) is UNMEASURED; the D2 TPM claim needs that medium to lie outside the disk restore domain, and T9's exclusion "simultaneous rollback of every trusted anchor" is what covers a flash-level restore [rev2: R3(4)] | TCG P1 34.7.2.1; P1 Q7; SR15 M2(ii) |
| PL8 | QUALIFIED (definition) | A machine is QUALIFIED for the TPM profile iff its TPM's (manufacturer, firmware version, spec revision) tuple is on a PROJECT-MAINTAINED LIST of hardware classes that passed Q0-Q9 AND whose NV medium is documented or measured outside the disk restore domain. At enrollment the provider reads the tuple (TPM2_GetCapability fixed properties, the data P1 Q0 read) and runs NV4's readpublic self-check; an unlisted tuple refuses enrollment with anchor_unqualified (ER21) before any write. No tuple is on the list today: the only candidate, the box (PL1), has its NV medium unmeasured (PL7) [rev2: R3(1)] | RSR15 R3(1); SR15 M2 fix (1); P1 Q0 (fixed properties read on the box); QP :70-204 |

==============================================================================================================
T3. THE PROVIDER  (item 3)
==============================================================================================================
T3.1 NV TEMPLATE
| Row | Rule (ACCEPTED) | Evidence |
|---|---|---|
| NV1 | One NV index per vault lineage; type TPM_NT_EXTEND; nameAlg SHA-256; dataSize 32; attributes EXACTLY AUTHWRITE, NT_EXTEND, AUTHREAD, NO_DA (NO_DA SET per T11 QQ3); TPMA_NV values MEASURED in Phase 4: 0x02040044 as defined, 0x22040044 once WRITTEN (equal to the expected values) [final: a]; the index Name changes accordingly; authPolicy empty. NOT set: OWNERREAD, OWNERWRITE, PPREAD, PPWRITE, POLICYREAD, POLICYWRITE, ORDERLY, CLEAR_STCLEAR, READ_STCLEAR, WRITEDEFINE, WRITEALL, PLATFORMCREATE, WRITE_STCLEAR, GLOBALLOCK, READLOCKED, WRITELOCKED [rev2: R5 QQ3] | X A3 :62. MEASURED WITH THE NO_DA-SET PRODUCTION TEMPLATE: P4 Q2 readpublic size 32, sha256, TPMA_NV 0x02040044 exactly (AUTHWRITE, NT_EXTEND, AUTHREAD, NO_DA; WRITTEN clear), 0x22040044 after one extend; XOR with the NO_DA-CLEAR test template = 0x02000000 (bit 25 only) in both states; Phase 4 ran before the commit PR (RSR15 R5, R7) [final: a]. Earlier, NO_DA-CLEAR test template: P1 Q2 readpublic TPMA_NV 0x00040044 exactly (q2_attr_decode.txt); P1 Q5_BEFORE 0x20040044 after WRITTEN [rev2: R5 QQ3] |
| NV2 | Owner bypass refused by the template (no owner read/write): an owner-authorized read or extend returns TPM_RC_NV_AUTHORIZATION | MEASURED: P1 Q2 owner read and owner extend both 0x149 |
| NV3 | Auth value: 32 random bytes (OS CSPRNG), generated at genesis, stored ONLY inside the vault's enrollment record (T6), never on disk elsewhere, never printed | X A2 :33, A3 :62. P2 D4: the toy held it OUTSIDE the blob (a named toy deviation, not the contract) |
| NV4 | Provisioning validates the ENTIRE public area after define (readpublic) against NV1 -- the NO_DA-SET template -- before the genesis commit; any difference -> refuse enrollment (anchor_template_mismatch) and undefine the just-created index [rev2: R5 QQ3] | X A3 :62 "Provisioning must validate the entire template". MEASURED on the NO_DA-SET template: P4 Q2 readpublic after define == 0x02040044, with the Name computed offline from the template (000bcf79...b15c for the test handle 0x01000F02; the Name is handle-specific, the attributes are not) [final: a]; the shape was first measured by P1 Q2 (NO_DA-CLEAR test template) [rev2: R5 QQ3] |
| NV5 | The extend rule: new = SHA-256(old \|\| data), the index starting at 32 zero bytes; the value is uninitialized (TPM_RC_NV_UNINITIALIZED) before the first extend | X A2 :50. MEASURED: P1 Q3.2 read-before-extend 0x14A; P1 Q3.3 extend d1 -> A1 == sealed (formula verified on hardware); P1 Q6 100/100 readbacks == software chain; P2 in-process final == Python chain == tpm2-tools read |
| NV6 | ORDERLY (hybrid) is FORBIDDEN for the anchor: for a non-counter hybrid index the RAM copy reaches NV only on Shutdown(STATE), "need not be copied ... on Shutdown(CLEAR)", and on TPM Reset WRITTEN is re-initialized CLEAR -- i.e. a power cut, or a restart that the TPM sees as a TPM Reset (an ordinary Linux reboot is expected to be one -- inference, NOT measured), can return the index to an OLDER value or to uninitialized, which breaks rollback protection (X A5 :100) | TCG P1 34.2.1 ("data may be lost if the TPM does not shut down in an orderly fashion"), 34.2.4.2 items 1-3; X A3 :62 |
T3.2 HANDLE POLICY
| Row | Rule (ACCEPTED) | Evidence |
|---|---|---|
| H1 | Production indices live in the OWNER range 0x01800000-0x01BFFFFF (TCG Registry of Reserved TPM 2.0 Handles v1.1 rev 1.00 sec 2.2.2 Table 2). The TPM-manufacturer range 0x01000000-0x013FFFFF (where the F02 test index 0x01000F02 lived) is never used in production | P1 E3; P2 E8; P1 Q7 sources TCG_Registry_Handles_v1p1.* ("The TPM does not enforce the conventional usage") |
| H2 | The handle is drawn at enrollment uniformly at random from the owner range, EXCLUDING any handle already present (TPM2_GetCapability handles-nv-index, which is PAGED: the provider follows moreData and enumerates the range FULLY before drawing), then defined; a define that returns NV_DEFINED retries with a new draw, at most 8 tries in total, then refuses anchor_capacity_exhausted. Never touch an index this client did not create and does not hold in an enrollment record [rev2: R4 note (b)] | P1 Q8: owner range already holds 3 indices of other software on the box (0x1800100, 0x1880001, 0x1880011); QP :30-31 (never touch an index not created) |
| H3 | The handle is recorded in the enrollment record (inside nv_public, T6 F6) and never derived from vault_id (a derived handle would let two lineages collide deterministically and would publish a lineage correlator in TPM metadata) | design choice of this draft; ACCEPTED by the ruling (was: SR-15 to confirm, QQ4) [final: d] |
| H4 | The handle is NOT an identity binding: the Name AND the secret-authenticated read are (T3.4). A value+Name match without the secret is forgeable | MEASURED: P1 Q4.5 attacker with HIS secret reproduces value A1 AND Name N1 |
T3.3 ENROLLMENT RECORD (bytes in T6 row F6)
| Row | Rule (ACCEPTED) | Evidence |
|---|---|---|
| E1 | Created ONLY at genesis of a fresh TPM-profile identity, inside the genesis vault B0, which is written (flushed file + directory) to the prepared slot BEFORE the define (genesis order: T11 QQ8); never at unlock, import or recovery; a missing index is never re-provisioned [rev2: R4 m4] | X A3 :56-58, :60; MEASURED: P2 T-END (after undefine: FREEZE "hardware anchor missing", index not re-created) |
| E2 | Contents: nv_public (the TPM2B_NV_PUBLIC as defined, 16 bytes: handle, nameAlg, attributes with WRITTEN clear, empty policy, dataSize); nv_auth (32 bytes); primary_template (a spelling naming the pinned storage-primary template); primary_name (the 34-byte Name of that primary). The expected Name of the index in every state is DERIVED from nv_public (WRITTEN clear before the genesis extend, set after); it is not stored twice | X A3 :64 lists "NV public template, index locator, expected initialized state, and a pinned storage-primary public Name". DEVIATION NAMED FOR SR-15 (found SAFE, SR15 H3(a)): the locator is nv_public's nvIndex and the expected initialized state is derived from the RECOVERY OUTCOME, not from the generation and not a separate field: read the head; UNINITIALIZED with a generation-0 prepared B0 and no current = incomplete genesis (T11 QQ8 path); UNINITIALIZED with any committed current (generation 0 included) = anchor_uninitialized (ER9); a value = compare with each retained blob's expected anchor (T4.3). The expected Name uses WRITTEN SET for every committed state and WRITTEN CLEAR only for the pre-genesis reconfirm [rev2: R4 m1]. MEASURED: the Name depends on the WRITTEN bit (P1 Q4.1 "N1 (written SET)" vs Q4.3 "N2 (written CLEAR)"); prim.name = 34 bytes (P1) |
| E3 | The pinned primary is a TRANSIENT storage primary re-created under the owner hierarchy from the pinned template for each session setup and flushed after use; a persistent handle of other software (e.g. 0x81000001) is never relied on. OWNER AUTHORIZATION: TPM2_CreatePrimary under the owner hierarchy is authorized by the owner password, so TPM enrollment is REFUSED when ownerAuthSet = 1 (marker tpm_owner_auth_set, ER22) and every session setup on a supported machine uses the empty owner authorization -- no password is ever asked for, stored or set by QSL (T11 QQ2). An owner password set LATER on an enrolled machine makes session setup fail: a FREEZE of class anchor_unavailable with the distinct marker tpm_owner_auth_set (ER22), named in the recovery text (T4.3). UPGRADE PATH ON THE RECORD (not adopted): PERSIST -- a QSL-owned persistent salt key made at enrollment (TPM2_EvictControl), after which no hierarchy authorization is needed at session setup; it would add a persistent slot, a TPM command to Q1's list, a T6 F6 member and a re-qualification, and is taken up only if owner-password machines must be supported [rev2: R2] | MEASURED: P1 Q3.4 (transient primary, Name 000b110d...); P1 sec 7 persistent handles 0x81000001 0x81000002 0x81010001 == Q0 (untouched); P2 in-process client reproduced the pinned Name with template variant 0. Owner authorization: measured ownerAuthSet = 0 on the box (P1 Q0) and the laptop (LAP line 99); the ownerAuthSet = 1 path (refusal, later-set freeze) is NOT RUN; the owner-auth requirement of CreatePrimary(owner) is SR15 M1's reading, not measured here [rev2: R2] |
| E4 | TPM clear / owner-seed change changes the primary Name -> anchor_identity_changed (refusal, never re-enrollment) | X A3 :66, TCG Part 3 sec 24.6 (cited by X Evidence :186; NOT fetched this phase). NOT RUN on hardware (Q11 declined). Only evidence: P1 Q3.4 negative control (a Name mismatch refuses) |
T3.4 AUTHENTICATED-SESSION RULE
| Row | Rule (ACCEPTED) | Evidence |
|---|---|---|
| S1 | EVERY command that uses the NV auth value (NV_Read and NV_Extend) runs under an HMAC session SALTED to the transient primary whose Name was compared to primary_name BEFORE the session is started; a mismatch refuses before any NV command (anchor_identity_changed) | X A3 :64. MEASURED: P1 Q3.4 salted+bound session, session attrs continuesession/decrypt/encrypt, authenticated read == A1; NEGATIVE CONTROL: flipped Name byte -> "Expected name does not match", no session created (Q3_4g_negctl_startauthsession_badname.txt) |
| S2 | The response HMAC is verified by the TSS on every response; a read whose response authentication fails is anchor_transport_failed, never a value | X A3 :64 "validate response authentication". MEASURED: P1 Q3.4 EXIT 0 means the TSS check passed; a forged-response test was NOT RUN |
| S3 | NEVER a password session for the NV auth value (it sends the auth value in the clear to the TPM and returns an unauthenticated value an interposer could forge); NEVER a check of the numeric handle alone; NEVER a raw saved read | X A3 :64; QP :41-43. P1 D4 used password auth for its extends (qualification only); P2 extends (13) and the in-process arm (100) used salted HMAC sessions -- the production form |
| S4 | Session lifetime: at most one session per unlocked lineage, HELD in process memory, flushed on lock, error or exit; re-established after any TPM error (T11 QQ7: HELD). Properties of the held session: (1) nonceCaller is fresh on every command, so a replayed earlier response fails the response HMAC -- the freshness X A3 :64 wants, per command, not per session; (2) the response HMAC is verified on every response (S2); (3) parameter encryption is on for NV_Read; (4) after ANY TPM error the session is dropped and re-established, with the transient primary's Name compared to primary_name BEFORE the new session (S1); (5) session contexts live only in the kernel resource manager, never on disk (NP1). What it gives up versus a fresh session per transaction: only the per-transaction re-check of the primary Name; a TPM clear invalidates loaded sessions, so the next command errors and (4) re-checks it [rev2: R5 QQ7] | P2 in-process arm: ONE held session, setup 155.151 ms, then 100 pairs; tool form: a fresh session per call (P2 toy, 92 sessions). Property (3): P1 Q3.4 session attrs decrypt|encrypt. Properties (1), (4), (5) are the reader's statement of TSS / resource-manager behaviour (SR15 H3(c)), NOT measured by F02 [rev2: R5 QQ7] |
T3.5 NO PERSISTED SESSION CONTEXT; NO TOOL SUBPROCESS IN PRODUCTION
| Row | Rule (ACCEPTED) | Evidence |
|---|---|---|
| NP1 | Never persist a TPM session context (no ContextSave of a session or of the salt primary to disk, no temp file). Measured: tpm2-tools 5.6 writes the authValue passed as session:<ctx>+file:<auth> INTO the saved session context, and created those files 0660 under umask 077 | MEASURED: P1 E4 (secret_scan: 3 hits, all .ctx); P2 E4 (repeat: q5_hmac.ctx, q5_prim.ctx 0660) |
| NP2 | The production provider is an IN-PROCESS client (libtss2-esys class); it never spawns tpm2-tools (NP1, and latency: tool-process p99 216.439 ms vs in-process p99 97.841 ms) | P1 Q6; P2 in-process row (inproc_stats.txt) |
| NP3 | The provider never logs, prints or hexdumps nv_auth, a session key or a context; logs carry only RCs and marker codes | QP :25-27; P1/P2 secret scans with positive control (P2 s6_scan_positive_control.txt) |
T3.6 ERROR TAXONOMY (X C :127, A3 :64) -> ACCEPTED MARKER SPELLINGS (C01 O9: registered through DOC-SCL-002 by the implementing PR)
| Row | Condition | ACCEPTED marker | Measured TPM result / toy evidence |
|---|---|---|---|
| ER1 | vault authentication failure (passphrase / AEAD) | vault_locked (EXISTING, unchanged; M vault code) -- never reported as a freshness failure (X A4 :70) | -- |
| ER2 | local checkpoint missing | freshness_checkpoint_missing | toy P2 R-CA LOCAL FREEZE "local checkpoint missing" (not rebuilt) |
| ER3 | local checkpoint corrupt (MAC failure, bad length, trailing bytes) | freshness_checkpoint_corrupt | toy P2 R-CC LOCAL FREEZE "checkpoint MAC failure" |
| ER4 | checkpoint unsupported version or profile byte | freshness_checkpoint_unsupported | not exercised (NOT RUN) |
| ER5 | authenticated generation regression | freshness_generation_regression | toy P2 D1 LOCAL |
| ER6 | conflicting state digest (same generation, different blob) | freshness_digest_conflict | toy P2 D1b LOCAL |
| ER7 | committed state unavailable (authority matches neither retained blob) | committed_state_missing | toy P2 R-CS LOCAL, D1/D1b/D2 TPM "matches neither" |
| ER8 | hardware anchor missing (index absent) | anchor_missing | toy P2 T-END (tool nvreadpublic of the absent index EXIT 1; the toy classifies by RC text, the toy's toy.py:184/:213; the exact RC was not captured in the transcript row) |
| ER9 | hardware anchor uninitialized (index present, never extended) for any committed blob (generation 0 included; the expected state is derived from the recovery outcome, T3.3 E2) [rev2: R4 m1, note (e)] | anchor_uninitialized | MEASURED RC: P1 Q3.2 0x14A TPM_RC_NV_UNINITIALIZED |
| ER10 | hardware identity changed (primary Name, or index public area/Name, differs from the enrollment record) | anchor_identity_changed | MEASURED: P1 Q3.4 negctl "Expected name does not match"; P1 Q4.3 N2 != N1 on recreate |
| ER11 | hardware authorization failure | anchor_auth_failed | With the production NO_DA-SET template (T11 QQ3): TPM_RC_BAD_AUTH (0x0A2 + session bits, measured 0x9A2) with no DA-counter increment; 0x98E only for a NO_DA-CLEAR index. MEASURED: P4 Q3.5 (two wrong-auth reads) and Q4.4 (the original secret against a recreated index) 0x9A2 each, LOCKOUT_COUNTER 0 -> 0 (TCG P1 v185 16.6.8: a DA-exempt entity returns TPM_RC_BAD_AUTH, no counter increment). NO_DA-CLEAR test template: P1 Q3.5 and Q4.4 0x98E TPM_RC_AUTH_FAIL (DA counter +1 each). MAPPING (RACC E2): anchor_auth_failed is returned for TPM_RC_BAD_AUTH (0x0A2 base, any session index) as well as for TPM_RC_AUTH_FAIL; a provider that maps only 0x98E would misclassify every wrong authorization on the production index [final: b] |
| ER12 | hardware dictionary-attack lockout | anchor_lockout | not exercised (inLockout 0 throughout: P1 Q0, P2 Q9) |
| ER13 | hardware rate-limited (TPM_RC_NV_RATE / RETRY / YIELDED) | anchor_rate_limited | 0 occurrences in 216 extends (P1 103, P2 113) -- NOT exercised |
| ER14 | hardware unavailable / transport failure / response authentication failure (device absent, permission denied, TPM_RC_NV_UNAVAILABLE, TSS response-HMAC failure) | anchor_unavailable | not exercised (condition list: X A3 :64 "unavailable ... transport failure", X C :127) |
| ER15 | owner bypass or template mismatch at provisioning | anchor_template_mismatch | MEASURED: P1 Q2 owner read/extend 0x149 TPM_RC_NV_AUTHORIZATION (the template's correct refusal) |
| ER16 | indeterminate commit (TPM command timeout / lost response on NV_Extend) | commit_indeterminate (transient: resolved per T4.4; surfaced only if the resolving read fails after its bounded retries; the prepared slot is RETAINED for T4.3 at the next unlock) [rev2: R4 m2] | toy P2 I1 (resolved COMMITTED), I2 (resolved NOT committed, 0 extends), K-X4 |
| ER17 | storage durability failure (a write or flush result not OK) | storage_durability_failed | toy P2 F1 (prepared-file fsync failure: aborted, 0 extends), F2 (directory fsync failure after promotion: stopped before step 6, then recovered) |
| ER18 | generation exhausted (g + 1 would exceed u64) | freshness_generation_exhausted | arithmetic only (BC: u64 max 20 digits) |
| ER19 | unknown protection_mode value in the vault | vault_protection_mode_unsupported | not exercised (rule: X A3 :56; the pattern of C01 row 16 vault_mode_unsupported) |
| ER20 | anchor capacity exhausted at enrollment (TPM_RC_NV_SPACE / no free handle) | anchor_capacity_exhausted | not exercised (QQ4; design choice of this draft, T3.2 H2) |
| ER21 | TPM not qualified: its (manufacturer, firmware version, spec revision) tuple is not on the T2 PL8 list, or NV4's readpublic self-check fails, at enrollment [rev2: R3(1)] | anchor_unqualified | not exercised (rule: RSR15 R3(1); SR15 M2 fix (1)) [rev2: R3(1)] |
| ER22 | TPM owner authorization set (ownerAuthSet = 1): at enrollment -> enrollment REFUSED; on an enrolled machine at session setup -> FREEZE (class anchor_unavailable, this distinct marker) [rev2: R2] | tpm_owner_auth_set | not exercised (both measured machines ownerAuthSet = 0: P1 Q0, LAP line 99) [rev2: R2] |
Every marker: "rollback" is used in user text ONLY where evidence supports it (X C :127); crash loss and malicious deletion
may be indistinguishable. Every marker is a refusal BEFORE effects; NO freshness or anchor marker (ER2-ER22) increments the
unlock-failure counter (C01 O7); ER1 vault_locked, the passphrase failure, is outside this sentence (SR15 H9(a)) [rev2: R4 note (a)].

==============================================================================================================
T4. COMMIT / FENCING / RECOVERY CONTRACT  (item 4)  -- each rule with the Part B row(s) that exercised it
==============================================================================================================
T4.1 COMMIT ORDER (X A4 :74-81; A4 amendment text)
| Step | Rule (ACCEPTED, verbatim in substance from X A4) | Toy row(s) that exercised it (P2; LOCAL / TPM) |
|---|---|---|
| C1 | Preserve the current committed vault; produce immutable encrypted successor B with generation g+1 and predecessor A; fresh per-write AEAD nonce/key material ([U] U3, X A2 :52) (X A4 :76) | G, T1, T1b (both profiles); K-B2 (kill before step 2: OPEN(n), no effect) |
| C2 | Write B to the prepared slot: temp file, write, flush the FILE (result checked), atomic rename, flush the DIRECTORY (result checked). Never advance the authority if preparation is not durably complete | K-M2 (temp written+flushed, not renamed: OPEN(n)); K-A2 (prepared durable: OPEN(n), prepared discarded); F1 (injected fsync failure on the prepared file: aborted, authority unchanged, 0 extends in TPM) |
| C3 | LOCAL: authenticated next checkpoint (QSLFRESH v1) to a temp file, flush, atomic replace of head, flush the checkpoint directory = the LOCAL COMMIT POINT | K-M3 (checkpoint temp not renamed: OPEN(n)); K-A34 LOCAL (PROMOTE(n+1), effect once) |
| C4 | TPM: reconfirm the authenticated old anchor; NV_Extend(d) ONCE; the hardware advance is the commit point; the authenticated readback of A_next confirms it (X A4 :80); then replace and flush the local checkpoint (convenience, never a substitute for a TPM read) [rev2: R4 m8] | K-B4 (after reconfirm, before extend: OPEN(n), 0 extends); K-X4 (between extend and readback: PROMOTE(n+1), stale checkpoint rebuilt, exactly 1 extend); K-R4 (after readback, before checkpoint: as K-X4); K-A34 TPM |
| C5 | Atomically promote the exact prepared vault to current; flush the vault directory; no new transaction until this succeeds or recovery resolves it | K-M5 (renamed, dir not flushed: OPEN(n+1), effect once); K-A5 (OPEN(n+1), outbox resumed); F2 (injected dir-fsync failure after promotion: stopped before step 6, recovered) |
| C6 | Only now release effects (outgoing ciphertext, delivered plaintext, destructive relay ACK, success reports); retries resend the EXACT committed bytes with the same operation identity | K-A6 (OPEN(n+1), effect once, no re-release); invariants (i)-(v) held after every row (P2 "Invariants"); independent check: TPM head 5456ce65... == chain recomputed from the effects ledger alone (verify_independent.txt of the prototype run) |
| C7 | Genesis: generation 0, predecessor 32 zero bytes, committed through C1-C6 before the identity, invitations or keys are used; TPM genesis reconfirm expects NV_UNINITIALIZED; TPM genesis order (B0 written and flushed BEFORE the define) per T11 QQ8 [rev2: R4 m4] | X A2 :50. P2 G (both profiles); P2 sec 5 "the genesis reconfirm read of the fresh index returned 0x14A (NV_UNINITIALIZED), as designed" |
| C8 | Digest and anchor: d = SHA-256("QSL-C07-VAULT-v1" \|\| V \|\| B); A_next = SHA-256(A \|\| d) = the TPM extend rule (NV5) | X A2 :45-46. MEASURED: P2 verify_independent.txt; P1 Q3.3 |
T4.2 FENCING (serialization)
| Row | Rule (ACCEPTED) | Evidence |
|---|---|---|
| FN1 | One trusted broker per lineage: every same-user open of a lineage takes ONE exclusive lineage lock (D32, T6) irrespective of vault pathname, acquired after quarantined decrypt and before reading the authority; every transaction C1-C6 runs under it. The lock file is NEVER unlinked; after flock the holder verifies fstat(fd).st_ino == stat(path).st_ino and retries otherwise; the lock is held for the unlocked lifetime [rev2: R4 m3] | X A2 :37, A4 :70, :83; SR15 m3 (an unlinked-and-recreated lock file lets a second broker lock a different inode) [rev2: R4 m3] |
| FN2 | A head change between verification and readback fails the transaction; no effect released. The TPM is not a compare-and-swap API: correctness depends on FN1; an unauthorized conflicting advance causes a PERMANENT FREEZE OF THE LINEAGE (the TPM head then equals neither blob's expected anchor; recovery is a fresh identity), not a transient denial of service -- never acceptance of another branch [rev2: R4 m3] | X A4 :83. Not exercised by a two-process race on the toy (NOT RUN; named check for F18) |
| FN3 | Batching: permitted ONLY if every included effect waits for the shared durable commit; periodic anchoring after effects is FORBIDDEN; default one transaction per authoritative mutation "until batching is separately reviewed" | X A5 :102; S4-DEC (no seat may batch) |
T4.3 RECOVERY TABLE (X A5 :89-96)
| Observed authority after restart | Action (ACCEPTED) | Toy row(s) |
|---|---|---|
| exactly matches the authenticated CURRENT blob | open it; discard/quarantine an uncommitted preparation | K-A2, K-M3, K-B4 (discarded_prepared) |
| exactly matches the authenticated PREPARED blob | promote that exact blob durably, then resume its committed outbox; the admitted inbound message and its owed receipt are INSIDE B, and recovery re-issues them from committed state (so a crash between C4 and C6 is not a lost delivery) [rev2: R4 m5] | K-A34, K-X4, K-R4 (PROMOTE, effect once) |
| matches neither retained blob | FREEZE committed_state_missing (never "largest generation", never one-step-behind) | R-CS LOCAL; D1, D1b, D2 TPM |
| local checkpoint absent/corrupt, LOCAL profile | FREEZE freshness_checkpoint_missing / _corrupt; never rebuilt from the supplied vault | R-CA LOCAL (still absent after), R-CC LOCAL |
| local checkpoint absent/stale, TPM profile, blob exactly matches the authenticated TPM head | rebuild the convenience checkpoint durably from that blob; the TPM is never reset | R-CA, R-CC, R-CS TPM (rebuilt byte-identical to the true checkpoint) |
| TPM missing, reset, uninitialized, identity changed, unauthorized, unavailable or unreadable, or an owner password set after enrollment | FREEZE with the corresponding ER marker; an owner password set after enrollment freezes at session setup with tpm_owner_auth_set (ER22), and the recovery text names that cause [rev2: R2]; no local fallback, no auto-provisioning | T-END (anchor missing, not re-created). Identity/auth rows: P1 Q3.4 negctl, Q4.4 (TPM level; not driven through the toy's unlock). Owner-password path: NOT RUN [rev2: R2] |
| any restart, before the rows above are applied | a prepared-slot temporary of a DEAD pid (vault.qsv.prepared.tmp.<pid>) is removed under the lineage lock; a prepared slot retained by a commit_indeterminate freeze is classified by this table first (T4.4 IC1) [rev2: R4 m6, m2] | not exercised by the toy; C04 R10 (a crashed process's temporary is never removed today) [rev2: R4 m6] |
KDF cost at recovery: a prepared blob written under its own salt would cost a second Argon2id (m = 262144 KiB, t = 3) at
every recovery that must open it; the answer is C01's held derived key (C01 row 14 / O4 / N1, APPENDIX A A12; F05), which
this table references for opening a retained prepared blob [rev2: R4 (Argon2 note, SR15 H6 b)].
T4.4 INDETERMINATE COMMIT (X A5 :98)
| Row | Rule (ACCEPTED) | Evidence |
|---|---|---|
| IC1 | A TPM command timeout or lost response on NV_Extend is an INDETERMINATE commit, not a failure. The invariant is NEVER A SECOND EXTEND (IC2); the resolving READ is idempotent and may be retried, at most 3 attempts in total. Resolve it, under the lineage lock, by that authenticated read: old head -> not committed (abort, discard the preparation); expected A_next -> committed (continue at C4's checkpoint write); anything else, or no read succeeding within the 3 attempts -> FREEZE commit_indeterminate with the prepared slot RETAINED, so that T4.3 classifies it at the next unlock (B4's removal applies only after that classification). ORDERING PREMISE: the resolving read is issued on the SAME device connection after the timed-out extend, so it cannot overtake it (kernel / resource-manager serialization); a tool subprocess would not give that (NP2). A read that times out at the next unlock is ER14 and freezes again [rev2: R4 m2] | toy I1 (extend issued, timeout asserted: read == A_next -> COMMITTED, effect once); I2 (no extend issued: read == A_n -> NOT committed, aborted, 0 extends). Retention: the toy's stop path exits without unlinking the prepared slot (the toy's toy.py:410, read by the rev2 seat; SR15 m2). The retry bound (3) is ACCEPTED, not measured; the ordering premise is not measured [rev2: R4 m2] |
| IC2 | NEVER a second extend for the same d (it would advance twice) | toy K-X4 "exactly 1 extend in the row, head not advanced twice" |
| IC3 | A disk flush error likewise requires reconciliation (the A5 table) before any further effect | toy F2 |
| IC4 | Boundary: no REAL TPM timeout occurred in any F02 run; the rows are simulations (a kill, or a timeout the toy asserts to itself) | P2 REPORT sec 8 |

==============================================================================================================
T5. PROCESS-CRASH VERSUS POWER-LOSS CLAIMS  (item 5)
==============================================================================================================
| Row | Claim (ACCEPTED wording) | Status | Evidence |
|---|---|---|---|
| CP1 | Process crash at any C1-C6 boundary: recovery to exactly the committed state; no effect released twice or before its commit | EVIDENCED ON THE TOY ONLY (both profiles); production: [U] U1 | P2 B2 LOCAL 13/13, TPM 17/17 |
| CP2 | NV anchor persistence across a warm reboot and an orderly power-off | MEASURED on the box (once after both events) | P2 Q5 GREEN (reboot.target 03:12:07Z; poweroff.target 03:13:57Z; 4 min 53 s off; authenticated read == A2+loop[100]). Boundary: journal cannot show the wall switch-off |
| CP3 | Power cut during a vault/checkpoint write | NOT RUN (Q10 declined, B6). No claim. Blocked at main by ENG-0365 (unchecked directory flush) | B6; ENG-0365 |
| CP4 | Power cut during NV_Extend | NOT RUN. Spec: the index may be lost (TCG P1 34.7.1) -> fail-closed refusal expected (anchor_missing), automatic recovery NOT COVERED | TCG P1 34.7.1; X A5 :100, B :121 |
| CP5 | Reversion of an acknowledged TPM head to an OLDER valid head after a power event | NOT RUN. Would BREAK rollback protection; the TPM claim requires compliant non-orderly persistence (NV6 forbids ORDERLY). For a firmware TPM this also rests on TCG P1 34.7.2.1 external-NV rollback protection [UNVERIFIED for Intel PTT] | X A5 :100; T2 PL7 |
| CP6 | Devices or filesystems that do not honor flushes | BLOCK the durability claim (X A4 :85). NOT measured on any device | X A4 :85 |
| CP7 | No zero-loss claim after power loss exists anywhere in this contract | -- | X E :161, E step 6 :170 |

==============================================================================================================
T6. C01 O13 CLOSURE (ACCEPTED)  (item 6)  -- names, types, bytes
==============================================================================================================
Encoding rules applied: canonical padded standard base64 for every persisted byte field, strict decode (a non-canonical
encoding is refused) (ENC decision 1, C4-O7 = BASE64); explicit schema version, unknown version refused with its own error
(ENC decision 2); deny_unknown_fields kept; no serde(default) (C01 row 21 "no default-to-empty"). Layer = C04 L2 (compact
serde_json; VaultPayload is serialized by serde_json::to_vec, M vault/mod.rs:298/:353/:647). Byte counts: BC.
T6.1 VAULTPAYLOAD VERSION 5 C07 MEMBERS (freezes the A13 field list together with C01's {version, protocol, mode, secrets})
| Row | Field | Type and JSON form (ACCEPTED) | Rule | L2 bytes of the member (BC) |
|---|---|---|---|---|
| F1 | vault_id | 32 bytes, CSPRNG at genesis; base64 string (44 chars) | immutable for the lineage; never reused (X A2 :29) | 57 |
| F2 | protection_mode | string, exactly "local-checkpoint" or "tpm"; SEPARATE discriminator from `mode` (C01 row 16 / A14) | immutable from genesis; no in-place change or downgrade (X A3 :56); unknown -> vault_protection_mode_unsupported (ER19) | 36 ("local-checkpoint") / 23 ("tpm") |
| F3 | generation | u64 as a JSON number, 0 at genesis, +1 per commit, canonical (no leading zeros, no sign, no exponent) | X A2 :33 "unsigned 64-bit generation"; X C :127 "generation exhausted"; g+1 > u64 max -> freshness_generation_exhausted (ER18) | 14 (1 digit) .. 33 (20 digits) |
| F4 | predecessor_anchor | 32 bytes; base64 string | 32 zero bytes at genesis; = the anchor A this blob extends from (X A2 :48) | 67 |
| F5 | checkpoint_mac_key | 32 bytes, CSPRNG at genesis; base64 string | keys the QSLFRESH HMAC-SHA-256; present in BOTH profiles (the TPM profile keeps a convenience checkpoint, X A1 :23) | 67 |
| F6 | tpm_enrollment | JSON null in "local-checkpoint"; in "tpm" an object {"nv_public": base64 of the 16-byte TPM2B_NV_PUBLIC as defined (24 chars), "nv_auth": base64 of 32 bytes, "primary_template": string (ACCEPTED "qsl-srk-ecc-p256-v1", QQ6), "primary_name": base64 of the 34-byte Name} with deny_unknown_fields | null iff protection_mode = "local-checkpoint"; object iff "tpm"; any other combination -> vault_parse_failed (distinct code per C01 O9) | 21 (null) / 221 (object of 204) |
| F-sum | the six members plus 6 separating commas | -- | BC (bytecount_output.txt) | LOCAL 268 (generation 1 digit) .. 287 (20 digits); TPM 455 .. 474 |
T6.2 C04 C4-O12 -- BYTES OF THE C07 LINEAGE FIELDS INSIDE R01 (ACCEPTED closure)
| Row | Statement | Evidence |
|---|---|---|
| B1 | The C07 members are ACTUAL L2 bytes of VaultPayload under R01 (actual + promises + H_W <= B_V), charged in every commit; they pass 1:1 into L2 (base64 contains neither '"' nor '\') | C04 R01, NOTATION L2; C04 X07 |
| B2 | Per vault: LOCAL <= 287 bytes, TPM <= 474 bytes, the generation counted at its 20-digit maximum (as C04 R02 funds u64 scalars at 20 digits) so no commit can fail INV by a digit rollover | BC; C04 R02 "20-digit funding" |
| B3 | Against B_V 16,777,216 that is about 0.003 percent (TPM, 474 / 16,777,216); the per-session reserve L06 (1,699,966) and the L29 ceiling are unchanged in form; F06's proof adds the constant 474 (or 287) to the fixed part of INV | C04 T6 L01, L06, L29 |
| B4 | NOT in R01: the QSLFRESH checkpoint (147 bytes, outside the vault) and its temp (147) -- disk only (C4-O8); the prepared slot: one more full encrypted vault on disk. ACCEPTED disk rule: a stale prepared slot is removed (after T4.3 classifies it uncommitted -- a slot retained by a commit_indeterminate freeze is removed ONLY after that classification [rev2: R4 m2]) BEFORE a new preparation is written, and a prepared-slot temporary of a dead pid is removed at recovery (T4.3) [rev2: R4 m6], so the L4 peak stays 2 x (B_V + 69) as C04 R10 states today (current + prepared-temp, then prepared + current) | C04 R10, C4-O8; X A2 :34 |
T6.3 THE A16 SELECTOR'S SECOND AXIS (ACCEPTED spelling)
`--protection local-checkpoint` or `--protection tpm`, REQUIRED at fresh-identity creation beside `--mode`; absent or
unknown -> refuse before any write (explicit selection, A4 text "selection at fresh-identity creation"); `--protection tpm`
on a machine whose TPM tuple is not on the T2 PL8 list -> refuse anchor_unqualified (ER21) before any write [rev2: R3(1)];
`--protection tpm` with ownerAuthSet = 1 -> refuse tpm_owner_auth_set (ER22) before any write [rev2: R2]; `--mode storage-only`
with either value is permitted (a storage-only vault still has a lineage); with `--protection tpm` this anchors EVERY
secret write (one NV_Extend per authoritative mutation, FN3), and that is intended [rev2: R4 note (d)]. No in-place change
later (X A3 :56). The desktop
(F13) presents the same choice with the A4 disclosure; which option (if any) is pre-selected is D-D1 of the decision sheet --
DECIDED: NONE, the user must pick a protection after a one-screen explanation (RBANK D-D1) [final: d].
T6.4 D30-D32 NAMES (ACCEPTED; C01 CENSUS reserved them)
| Row | Entry | ACCEPTED name / location | Rule |
|---|---|---|---|
| D30 | prepared-successor vault slot | `vault.qsv.prepared` in the successor store directory (same filesystem as the vault: rename must be atomic); its temp `vault.qsv.prepared.tmp.<pid>` (C01 D2 pattern) | at most one; mode 0600; never opened as current except by T4.3 row 2 |
| D31 | freshness checkpoint | `<STATE>/qsl/freshness/<64 lowercase hex of vault_id>/head`, STATE = $XDG_STATE_HOME if set and absolute, else <HOME>/.local/state; directory 0700, file 0600; temp `head.tmp.<pid>`; test override variable `QSC_<TAG>_STATE_DIR` (rides C01 O2 like A20) | QSLFRESH v1 (T7 row C07-01); not governed by C01 O8 L1-L6 (C01 D31); on the box it shares the vault's filesystem (T2 PL6) |
| D32 | per-lineage lock | `<STATE>/qsl/freshness/<hex vault_id>/lock` (exclusive flock), distinct from .qsc.lock (C01 D4) | keyed by vault_id, so every pathname/copy of one lineage meets the same lock (FN1) |
T6.5 QSLFRESH v1 LAYOUT (X A2 :39, measured on the toy)
magic "QSLFRESH" @0+8 | version u16 BE = 1 @8+2 | profile u8 (1 local, 2 TPM) @10+1 | vault_id @11+32 | generation u64 BE
@43+8 | digest d @51+32 | resulting anchor A @83+32 | HMAC-SHA-256(checkpoint_mac_key, bytes 0..114) @115+32. Total
147 bytes; no trailing bytes; reject unsupported version/profile (ER4) and a profile byte that disagrees with the
vault's protection_mode. The plaintext reveals lineage continuity and write count to a local disk observer (X A2 :39).
MEASURED ON THE TOY: P2 B1 head 147 bytes, fields verified independently (verify_independent.txt of the prototype run). BC offsets.

==============================================================================================================
T7. QSLFRESH v1 AND THE C07 IDENTIFIERS -- REGISTRATION ROUTE  (item 7)
==============================================================================================================
Route (the existing format governance, as C01 O1 and CAN12 sec 12 define it; nothing is allocated by this draft):
(1) SR-15 read of this draft; (2) the Director's ruling fixes the values; (3) ONE qsl-protocol PR appends
"### 12.8 C07 allocations (rollback and durability)" to DOC-CAN-003 sec 12 (12.1-12.7 are not edited; rows appended only,
as 12.3-12.7 state for their predecessors) with the rows below, and its D-record records the allocation; (4) the marker
spellings of T3.6 are registered in DOC-SCL-002 by the IMPLEMENTING PR (C01 O9), not by the allocation PR; (5) until
(3) merges no successor vault is written outside tests (C01 O13, A13) and no QSLFRESH file is written outside tests.
| Row (ACCEPTED) | Namespace | Allocation | Amends |
|---|---|---|---|
| C07-01 | Freshness checkpoint | QSLFRESH v1, 147 bytes, layout T6.5; location D31 | A24 (checkpoint location) |
| C07-02 | Vault payload C07 members | F1-F6 of T6.1; the A13 field list becomes FROZEN as {version, protocol, mode, protection_mode, vault_id, generation, predecessor_anchor, checkpoint_mac_key, tpm_enrollment, secrets} | A13 (FIELD LIST RESERVED -> ALLOCATED) |
| C07-03 | Protection profile discriminator | protection_mode values "local-checkpoint", "tpm" | A14 (protection_mode RESERVED -> ALLOCATED) |
| C07-04 | Init selector second axis | --protection local-checkpoint / --protection tpm | A16 |
| C07-05 | Reserved locations | D30 vault.qsv.prepared; D32 lineage lock | A24 |
| C07-06 | Vault digest domain label (CRYPTO INPUT) | ASCII "QSL-C07-VAULT-v1" in d = SHA-256(label \|\| V \|\| B) | new; SR-15 names it as a crypto input |
| C07-07 | TPM NV usage profile | NV1 template, H1 owner range 0x01800000-0x01BFFFFF, E2 enrollment fields, primary template spelling (QQ6) | new |
Governance note: the vault envelope magic stays QSCV04 and payload version 5 (C01 rows 13, 15): no new version is minted
because no successor vault has been written outside tests (C01 O13); this is the single schema change A4 Impact demanded.

==============================================================================================================
T8. ENG-0365 / ENG-0366 GATES FOR F03-F05 AND THE PRE-UNLOCK DOMAIN  (item 8)
==============================================================================================================
T8.1 ENG-0365 GATE (F05 owner, R4)
G-365: every write of the successor transaction path (C2, C3, C4's checkpoint, C5) uses a CHECKED file flush AND a CHECKED
directory flush; a failure is storage_durability_failed (ER17) and enters T4.3 reconciliation before any effect. No
power-loss or durability claim (T5 CP3, CP6) may be made on fs_store::write_atomic or the init copy as they are at M
(fs_store/mod.rs:252, :492-493; vault/mod.rs:771). Evidence that the rule is implementable: the toy's checked flushes and F1/F2.
T8.2 ENG-0366 GATE -- EVERY AUTHORITATIVE FILE AT MAIN AND ITS ACCEPTED DISPOSITION
Dispositions: INSIDE (moves into, or stays in, the anchored vault); PROJECTION (reconstructible from the vault; a rolled-back
copy can only cause re-derivation, never acceptance of stale authority); VAULT-AUTH (stays outside, but the vault commits what
it must contain, and reopen refuses a mismatch); RETIRE (never read or written by the successor, C01 CENSUS); OUT (outside
rollback scope, disclosed); TRANSIENT / OUTPUT / C06 (not authoritative state). Rows keyed to C01 CENSUS ids where they exist.
| # | File (at M) | Writer (M file:line) | C01 id / class | ACCEPTED C07 disposition |
|---|---|---|---|---|
| W1 | vault.qsv | vault/mod.rs:327, :380, :535; init :745-777 | D1 SUCC-NEW | INSIDE: the anchored object itself (C1-C6). A passphrase change is an anchored vault replacement by this rule: it commits through C1-C6 like every other vault write and is no escape route (X E step 4 :168 "No passphrase-change or legacy writer escape route"; SR15 H7) [final: c] |
| W2 | qsp_sessions/<peer>.qsv session blobs + legacy tombstone | protocol_state/mod.rs:984, :987 | D12/D13 RETIRE | RETIRE; successor session/ratchet state is INSIDE the vault (the candidate's commit_directional_pair shape, ENG-0366) |
| W3 | qsp_sessions/<peer>.scka.json SCKA monotonic state | protocol_state/mod.rs:639 | D14 RETIRE | RETIRE; successor equivalent INSIDE (ratchet/skipped-key state is named by X A4 :72) |
| W4 | qsp_status.json | protocol_state/mod.rs:59 (result discarded) | D11 RETIRE | RETIRE; any successor status display is a PROJECTION |
| W5 | msgqueue_v1/<ck>/<seq>_<msg_id>.rec (successor msgqueue_v2, A17) | msgqueue/mod.rs:532 | D17 SUCC-NEW | VAULT-AUTH for QUEUED rows (a resurrected cancelled row could be sent): the paired commit that enqueues or retires a row records the per-contact live row set (msg_id, seq, row hash) in the owner; reopen refuses an extra, missing or altered row. PREPARED and later states are INSIDE (the Flight, C04). The mechanism is F04's; until it exists no rollback claim covers queued-not-prepared rows |
| W6 | msgqueue_v1/<ck>/seen_inbound.dedup (replay acceptance, production at M) | msgqueue/mod.rs:765 | D18 RETIRE (M) | RETIRE; successor replay acceptance is INSIDE (X A4 :72 "replay acceptance"); completeness [U] U5 |
| W7 | send.state | transport/mod.rs:111, :4095, :4751 | D9 O6 (proposed RETIRE) | RETIRE; if C01 O6 keeps it: INSIDE (it is send-commit state) |
| W8 | outbox.json (plaintext labels) | transport/mod.rs:3927 | D10 O6 (proposed RETIRE) | RETIRE in favour of the vault-committed intent (INSIDE); if kept: INSIDE |
| W9 | relay_seen_ids_v1_<hash>.json | dedup/mod.rs:136 | D20 O6 | PROJECTION (a cache of relay item ids): the authoritative replay decision is the vault's (W6); a rolled-back cache may only cause re-evaluation. Conditional on U5 |
| W10 | identities/self_<label>.json (public identity record) | identity/mod.rs:344, :397 (the second is the lazy migration C01 O6 removes) | D15 O6 | PROJECTION: re-derived from the vault's identity secret keys at unlock; a mismatching file is refused, never trusted |
| W11 | quarantine_v1/<record> | quarantine/mod.rs:489 | D19 O6 | VAULT-AUTH if kept (the vault records the live quarantine set), else RETIRE (C01 O6 / F04 decide) |
| W12 | config.txt (policy_profile; ack_mode tombstone) | fs_store/mod.rs:210 (write_config_key) | D6 O6 | Security-relevant keys (policy_profile and any key that weakens a check) move INSIDE the vault; display-only keys stay OUT (disclosed). F04 classifies each key |
| W13 | store.meta | fs_store/mod.rs:219 | D5 SUCC-NEW | OUT: a static marker read at open (C01 O8 L4); a restored copy is byte-identical |
| W14 | vault_security.txt, vault_unlock_failures.txt | vault/protection.rs:532-533 | D7/D8 SUCC (A23) | OUT, the D5 pre-unlock domain: T8.3 |
| W15 | received payload files recv_<n>.bin in a caller-chosen dir | transport/mod.rs:1094 | D24-like OUTPUT | OUTPUT = an EFFECT: written only at C6, after the commit that accepts the message |
| W16 | doctor export | lib.rs:2891-2900 | D27 OUTPUT | OUTPUT (not state) |
| W17 | QSC_LOG_PATH marker log (append) | output/mod.rs:391 | not in C01 CENSUS (env-gated debug log; NAMED for C01's census) | OUTPUT (not state) |
| W18 | attachments/ staging and output | attachments/mod.rs:705, :1338, :1440-1503 | D21/D25 C06 | C06 (RESERVED; not written while file send is refused) |
| W19 | .qsc.lock, model lock, probe.tmp, *.tmp.<pid>, vault tombstone | fs_store:393, :419, :436; model/mod.rs:146; protection.rs:564 | D2-D4, D22 TRANSIENT | TRANSIENT |
| W20 | keychain seam file (test) and OS keychain entries | vault/mod.rs:1368; D29 | D23 TEST / D29 O6 | TEST / out of successor scope (C01 D29) |
| W21 | desktop settings.json, webview/ (qsl-desktop) | D:paths.rs (C01 D26) | D26 DESKTOP | F13 classifies; any security-relevant desktop setting moves INSIDE or is disclosed OUT |
| W22 | NEW: vault.qsv.prepared, freshness head, lineage lock | T6.4 | D30-D32 | C07's own (INSIDE the transaction) |
| W23 | READER, not a writer: legacy handshake_pending_<self>_<peer>.json, read and lazily MIGRATED INTO the vault, then deleted (a restored old file would re-inject old pending-handshake authority as a fresh, anchored mutation the anchor cannot see) | handshake/mod.rs:1249-1273 (read, vault::secret_set, remove_file :1268); removed on clear :1296 | D16 O6 (proposed RETIRE) | RETIRE per C01 O6's proposal: the successor never reads it; its presence in a successor directory = successor_dir_foreign (C01 O8 L4) [rev2: R4 m6] |
Census method and its limit: `write_atomic(` call sites plus fs::write / File::create / OpenOptions / rename / remove_file in
qsl/qsl-client/qsc/src at M (writer_census_main.txt of phase 3, 79 lines, test-module hits included and classified TEST by
C01). The #1831 candidate adds a tenth write_atomic file (directional_delivery, ENG-0366); F03 re-runs this census on
whatever tree it integrates. GATE G-366 (ACCEPTED): F03-F05 may claim rollback coverage only for files this table places
INSIDE or VAULT-AUTH (with the mechanism implemented), and must re-run the census with zero unplaced writers AND zero
unplaced readers that mutate vault state from a disk file (W23 handshake/mod.rs:1249-1273; identity's legacy reinterpreting
reader identity/mod.rs:484-489, C01 O6) [rev2: R4 m6].
T8.3 THE PRE-UNLOCK DOMAIN (R5 / QQ5) -- ACCEPTED TREATMENT (Director: OUT OF ROLLBACK SCOPE, DISCLOSED -- accepted, with
the wipe-after-N sentence, RSR15 R5 QQ5) [rev2: R5 QQ5]
Facts: the unlock-failure counter and attempt limit must be read BEFORE the vault is decrypted, so they cannot live in it
(M vault/protection.rs:512-534; DOM last section). Restoring older copies resets the escalating delay and the wipe budget.
ACCEPTED: D5 is OUT OF ROLLBACK SCOPE, DISCLOSED. Reason: the attempt limit only rate-limits guesses made THROUGH THIS APP.
Anyone who can restore files on the disk can also copy the vault file and guess offline at the KDF's cost, where no counter
applies at all; rollback-protecting the counter would therefore not bound guessing by the attacker D5 describes. The
protection against guessing is the passphrase and the KDF (C01 row 14 / A12: Argon2id m=262144 KiB, t=3, p=1). Kept rules:
the A23 format line and fail-closed refusal of an unknown format; C01 O7 (no count on format/version refusals). Disclosure
text (ACCEPTED): "The unlock attempt limit protects against guessing through this app. It does not stop someone who has a
copy of your files from guessing offline, and the wipe after N failed attempts, if you turned it on, cannot protect a
copy of your files either; a strong passphrase does." (The wipe is an opt-in setting: M vault/protection.rs:13.)
[rev2: R4 (wipe-after-N note, SR15 H7), R5 QQ5]
Rejected alternative (named): a second TPM NV counter for the unlock counter -- adds NV writes (Q7) and a pre-unlock TPM dependency, and still does not bound offline guessing.

==============================================================================================================
T9. RELEASE-CLAIM TEXT  (item 9)  -- X E :159 or narrower
==============================================================================================================
ACCEPTED text = X E :159 VERBATIM, plus ONE narrowing sentence (inserted after its second sentence) and nothing wider:
> QSL detects restored or conflicting vault files when its separate checkpoint survives. For identities created with
> qualified TPM protection, QSL also refuses stale state of that enrolled identity after disk rollback. TPM protection is
> offered only on machines that have passed QSL's TPM qualification. It refuses to resume when the required anchor or
> matching committed state cannot be verified. Without an independent anchor, restoring the vault and its checkpoint
> together may go undetected. Rollback protection does not cover compromised client execution, extracted vault secrets,
> simultaneous rollback of every trusted anchor, or substitution of a different historical identity without an
> independently retained identity binding. Recovery may require a new identity and contact verification.
The third sentence ("TPM protection is offered only on machines that have passed QSL's TPM qualification") and the word
"qualified" in the second stand ONLY by reference to T2 PL8: a machine whose (manufacturer, firmware version, spec
revision) tuple is on the project-maintained list (Q0-Q9 passed AND NV medium documented or measured outside the disk
restore domain), checked at enrollment (ER21); the disk-rollback claim additionally rests on the C7-O7 disk-image restore
release gate [rev2: R3(3)].
Never claimed (X E :161): universal Linux whole-backup protection; that peers always detect rollback; that the relay proves
freshness; that a TPM makes old backup secrets unusable; that every power interruption permits automatic recovery; a
post-quantum attestation claim. ALSO never claimed on today's evidence: any power-loss durability (T5 CP3-CP6 NOT RUN);
any statement about TPM wear or lifetime (Q7 RED, no vendor figure). The pre-unlock disclosure (T8.3) ships with it.
The text is usable only after F18 proves implementation and qualification (X E :157, THE PLAN F18).

==============================================================================================================
T10. OPEN CELLS  (each blocks the named cards; every row above is ACCEPTED by the ruling, RACC)
==============================================================================================================
| Id | Owner | Open question | Blocks |
|---|---|---|---|
| C7-O1 | OPERATOR (A4; S4-DEC) | Q6 AMBER: accept per-change anchoring latency or require GREEN (decision sheet D-A). DECIDED: ACCEPT -- AMBER accepted for the in-process client only; the app never anchors via command-line tools; "catching up" progress is shown when a backlog is processed (RBANK D-A) [final: d] | TPM profile offer (F13/F18); not the local profile |
| C7-O2 | OPERATOR (A4; S4-DEC) | Q7 RED: wear with no vendor figure (decision sheet D-B). DECIDED: OPTIN -- the TPM profile only as the user's choice at identity creation; one anchor per change as designed; plain disclosure that the chip's write limit is unpublished; the app's own TPM-write count shown in settings; Intel asked for the figure in parallel (sealed formula re-run if answered) (RBANK D-B); the Director's E5 condition is carried in T9 and PL7 [final: d] | TPM profile offer (F13/F18) |
| C7-O3 | OPERATOR (R5: QQ1 is product-visible) | Production TPM access model (decision sheet D-C). DECIDED: TSS -- v1 access via group tss, with an explanation shown at install, while the TPM profile is opt-in; HELPER revisited if it ever becomes the default (RBANK D-C) [final: d] | TPM profile install/enrollment UX (F13), provider implementation |
| C7-O4 | Director after SR-15 (R5) | RULED by RSR15 R5: QQ2 per R2 (refuse ownerAuthSet = 1); QQ3 NO_DA SET; QQ4 accepted with note (c); QQ6 accepted; QQ7 HELD; QQ8 per m4 (T11). DONE under this cell: the Phase 4 NO_DA re-qualification (Q2 define + readpublic, Q3.5, Q4.4 with NO_DA SET; its own directive; tss membership was an operator act) ran GREEN on the ruled property BEFORE the commit PR (RSR15 R5 QQ3, R7; P4) [rev2: R5] [final: d] | provider implementation (F05), T7 row C07-07, the C07 commit PR |
| C7-O5 | Director | D5 pre-unlock OUT-of-scope proposal (T8.3): ACCEPTED by RSR15 R5 QQ5, with the wipe-after-N sentence [rev2: R5 QQ5] | F05, F13 disclosure text |
| C7-O6 | F04 (with C01 O6) | mechanisms for W5 and W11 (VAULT-AUTH); classification of W12 keys | F04, F05 rollback coverage claims |
| C7-O7 | F18 | NOT RUN rows: power cut (CP3-CP5; DECIDED: SPARE -- before release, at F18, on a sacrificial machine, NEVER on the build box, RBANK D-D3 [final: d]), TPM clear (E4), cross-machine (D2x), two-process race (FN2), forged TPM response (S2), real TPM timeout (IC4), any second machine (PL2, PL3). RELEASE GATE: the real disk-image restore test -- image the disk, advance the TPM lineage two generations, restore the image, expect FREEZE committed_state_missing (non-destructive to the TPM; the discriminating measurement for the D2 TPM claim and PL8's NV-medium condition) -- run later on a machine the operator names, NOT in this lane [rev2: R3(2)] | F18 release claim |
C01 O13 and C04 C4-O12 are CLOSED by T6 (ruled by RACC; effective when the PR carrying T7's sec 12.8 merges) [final: d]. THE PLAN C07 row: a feasible
C07 is ACCEPTED for the local-checkpoint profile on every supported Linux machine and for the TPM profile on machines QUALIFIED
per T2 PL8 (none yet: the box's NV medium is unmeasured, PL1/PL7) [rev2: R3(1)], the latter subject to C7-O1..C7-O3; the negative-finding alternative (DIRECTIVE_DRAFT sec 4 (ii)) applies to the TPM
profile if the operator declines D-A or D-B -- the operator did not (D-A ACCEPT, D-B OPTIN) [final: d].

==============================================================================================================
T11. OPEN QUESTIONS FOR THE DIRECTOR (R5: QQ2-QQ4 ruled after SR-15; QQ5 is T8.3; QQ6-QQ8 new) -- each with its ACCEPTED answer
(rev2: all of QQ2-QQ8 are now RULED by RSR15 R5 and the row text carries the ruling; FINAL: the rows are ACCEPTED contract
text, RACC) [rev2: R5] [final: d]
==============================================================================================================
| Id | Question | ACCEPTED ANSWER | Basis |
|---|---|---|---|
| QQ2 | If an install has an owner password set, who holds it at provisioning? | RULED (RSR15 R2, (b)-REFUSE, subject to operator override): "QSL never sets, stores or asks for the owner password; a TPM with ownerAuthSet = 1 is unsupported for the TPM profile." Enrollment is REFUSED when ownerAuthSet = 1 (tpm_owner_auth_set, ER22); every session setup on a supported machine then needs no password (E3); an owner password set later on an enrolled machine freezes at session setup (ER22, T4.3). PERSIST (a QSL-owned persistent salt key) stays on the record as the upgrade path (E3) [rev2: R2] | measured ownerAuthSet 0 on the box (P1 Q0) and the laptop (LAP line 99); the define needs owner authorization (P1 Q2); the per-session-setup need is SR15 M1 [rev2: R2] |
| QQ3 | TPMA_NV_NO_DA clear or set? | RULED (RSR15 R5 QQ3): SET on the production index (NV1). The auth value is 256 random bits, so dictionary-attack protection adds nothing for it (TCG P1 22.2.6 names "high-entropy authorization values" as the case needing none), while CLEAR lets any process that reaches the TPM raise the machine-wide DA counter by wrong-auth attempts on our index (measured: +1 per failure) toward lockout, which then blocks everything DA-protected. The former [UNVERIFIED] (NV-index behaviour in lockout) is SETTLED BY TCG P2 table TPMA_NV bit 25: "SET (1): Authorization failures of the Index do not affect the DA logic and authorization of the Index is not blocked when the TPM is in Lockout mode." COST: the NV1 attribute values changed (MEASURED in Phase 4: 0x02040044 as defined / 0x22040044 once written) [final: a], NV4's comparison and QQ8's guard use the NO_DA-SET template, the index Name changes; the re-qualification (Q2 define + readpublic, Q3.5, Q4.4 with NO_DA SET) ran as Phase 4 with its own directive BEFORE the commit PR: GREEN on the ruled property [rev2: R5 QQ3] [final: a]. A wrong authorization on the NO_DA-SET index returns TPM_RC_BAD_AUTH (0x0A2 + session bits, measured 0x9A2) with no DA-counter increment; 0x98E only for a NO_DA-CLEAR index (ER11, with the E2 mapping) [final: b]. CLAIM BOUNDARY: the behaviour of a NO_DA index while the TPM is IN LOCKOUT is NOT MEASURED (P4 E4: measuring it needs lockout, which was not authorized); for that case this contract rests on the TCG P2 bit 25 text alone [final: b] | TCG P1 16.8.1 ("an NV Index receives DA protection unless the TPMA_NV_NO_DA attribute of the Index is SET"), 22.2.6; TCG P2 TPMA_NV bit 25 (P2.txt:10190-10195 (the text extract of that PDF)); P1 Q3.5, Q4.4 (0x98E, counter 0->1->2); P1 Q0 MAX_AUTH_FAIL 31, LOCKOUT_INTERVAL 600 s, LOCKOUT_RECOVERY 86400 s [rev2: R5 QQ3]; P4 Q2, Q3.5, Q4.4 (0x9A2, counter 0->0->0), P4 E1, E4; TCG P1 v185 16.6.8 [final: a, b] |
| QQ4 | Per-machine lineage limit and index exhaustion | A client-side limit of N_LIN TPM lineages per machine user (HYPOTHESIS value 4; F05 measures free NV space, which F02 did not); beyond it, or on TPM_RC_NV_SPACE / no free handle, enrollment refuses anchor_capacity_exhausted before any write, with NO fallback to the local profile for that identity (the user may choose a local-checkpoint identity explicitly). An index is undefined only by an explicit user deletion of its identity (after that identity is retired), never by another lineage. A FROZEN lineage's index stays deletable by that explicit identity-deletion path (its handle is in the decryptable enrollment record), else NV space leaks by one index per recovery. ACCEPTED with this note (RSR15 R5 QQ4) [rev2: R5 QQ4, R4 note (c)] | P1 Q8 (owner range: 3 foreign indices; NV_COUNTERS 3, AVAIL 4; NV_INDEX_MAX 0x800 is a per-index size limit, not free space); X A2 :36 one index per vault |
| QQ6 | Which storage-primary template is pinned? | The ECC P-256 / SHA-256 storage template the F02 runs used (tpm2_createprimary -C o -g sha256 -G ecc256; Name 000b110d... on the box; the in-process client reproduced that Name with "template variant 0"). The implementing PR writes the template bytes down and the spelling "qsl-srk-ecc-p256-v1" names exactly those bytes. Whether it equals a TCG-published SRK template is [UNVERIFIED] (the bytes were not recorded in F02 evidence). ACCEPTED (RSR15 R5 QQ6); the implementing PR records the template bytes [rev2: R5 QQ6] | P1 Q3.4; P2 in-process row; T3.3 E3 |
| QQ7 | Held session or a fresh session per transaction? | Held: one salted HMAC session per unlocked lineage in process memory (nonces roll on every command), flushed on lock/error/exit. A fresh session per transaction would add the measured ~155 ms setup to every anchored change (median ~39 -> ~194 ms), i.e. Q6 would move toward RED. RULED (RSR15 R5 QQ7): HELD, with the reader's properties (1)-(5) written into T3.4 S4 [rev2: R5 QQ7] | P2 in-process (setup 155.151 ms, pair median 38.992 ms); X A3 :64 |
| QQ8 | NEW: an interrupted GENESIS can orphan an NV index. (The phase-3 premise "the define must precede the genesis vault, because nv_public carries the handle" -- phase3 REPORT E3 -- is recorded as UNTRUE AS STATED: the client draws the handle and nv_auth before both, SR15 m4 / H9(h).) [rev2: R4 m4, note (h)] | RULED (RSR15 R4 m4, R5 QQ8): the AUTHENTICATED genesis vault B0 is the record; the plain "enrollment pending" marker is DROPPED. Genesis order: draw handle + nv_auth -> build nv_public (the NO_DA-SET NV1 template) -> write B0 (holding nv_public and nv_auth) to the prepared slot, flushed FILE + DIRECTORY -> define -> NV4 readpublic check -> reconfirm UNINITIALIZED -> extend d0 -> readback -> checkpoint -> promote. On restart, "prepared B0 at generation 0 with no current" is the incomplete-genesis signal; its decrypted nv_public names the ONLY handle the cleanup may touch, and the cleanup undefines it ONLY IF its public area equals the NO_DA-SET NV1 template exactly AND it is uninitialized (both guards); otherwise it leaves the index and reports. Never touches any other index. An attacker without the passphrase cannot forge B0; a restored B0 of a committed lineage fails the double guard (WRITTEN SET changes the attributes). The cleanup undefine is owner-authorized: under QQ2 (ownerAuthSet = 0 on every supported machine) it needs no password [rev2: R4 m4, R5 QQ3, R2] | X A2 :50 ("Incomplete genesis never becomes a usable identity"); H2 (never touch an index not ours); SR15 m4 (double-guard basis: P1 Q4.1 / Q4.3 WRITTEN changes the Name); MEASURED on the NO_DA-SET template: P4 Q2 unwritten 0x02040044 / Name 000bcf79...b15c vs written 0x22040044 / Name 000beacd...3b2f -- WRITTEN changes the attributes and the Name [final: a]; not exercised by the toy (its index was pre-defined in SETUP, P2 sec 5): F05 must exercise genesis interruption [rev2: R4 m4] |

END OF C07 FINAL
