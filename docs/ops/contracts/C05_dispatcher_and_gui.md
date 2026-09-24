C05 -- DISPATCHER AND GUI CONTRACT -- FINAL (ACCEPTED WITH NAMED FIXES, SUBJECT TO THE OPERATOR'S FINAL APPROVAL:
CONTRACT_ACCEPTED_WITH_NAMED_FIXES_OPERATOR_APPROVAL_PENDING)

==============================================================================================================
FIXES (draft -> FINAL), keyed to RULING_NA0783_C05_ACCEPT_2026-09-24 (sha256
c135b67538405af40cab8410c3970b2bbf97d7e8b7dd6757dcb2aaa916ae02ea)
==============================================================================================================
Status: C05 ACCEPTED (contract) WITH NAMED FIXES, SUBJECT TO THE OPERATOR'S FINAL APPROVAL (Q8: the operator approves
how the app looks and functions; the change carrying this contract is not merged until the operator says so); result
class CONTRACT_ACCEPTED_WITH_NAMED_FIXES_OPERATOR_APPROVAL_PENDING. The Director's ruling, on the SR-15 read
(SR15_C05_FINDINGS.md sha256 cc5c69d77ee0be110126cca1d77ae69197494f403690f5fb1f035ef218bd5174, fable/xhigh,
recommendation ACCEPT WITH NAMED FIXES F1-F10; X1 and X2 MAJOR; no BLOCKER), takes F1-F10 as the read words them (F5 its
FIRST option), rules E1-E11 below, and lets the operator's answers Q1-Q8 GOVERN the user-visible rows: they are banked
verbatim in RBANK_C05_operator_answers_2026-09-24.md (sha256
6e5e8bd43ae8f3f83ff207b87a5e660da3930c7fdeabde2fe67f75baa6bd219c) and its addendum
RBANK_C05_operator_answers_A1_2026-09-24.md (sha256
848a1c9ebda18c6beb8a8db83c286825f40687a4049201fa440072886625b256). Q1 is carried by AMENDMENT A5 to
THE PLAN (AMENDMENT_PLAN_A5_status_display_2026-09-24.md, sha256
6d7b9d17ca839436a52343146637e1821f248b9014129c3326721eaab72803c1), so THE PLAN now reads amended A4 and A5.
Nothing else in the draft changes. The body below is the draft (C05_DRAFT.md sha256
237dd64e17a154c85582742a57f50bb356512a6d1a6f053bff8e0c8df7b006d1) with these applied. The word PROPOSED in the body is
retained verbatim and now reads: the accepted contract value, NOT YET ALLOCATED; C05 allocates no wire or local-schema
identifier, and every code or spelling marked NEW stays a PROPOSED spelling registered through DOC-SCL-002 by the
implementing PR (C01 O9). F01 stays ACTIVE (the C06 boundary next, then F01 close). Every fix was verified at the source
before it was written (C = #1831 ffc8fc52; M = qsl-protocol main 4e9dfd0e; D = qsl-desktop main 92cba80a; S =
qsl-server 5ea0f925, the revision the read cites; contracts C01-C04 with their AMENDMENTS at M).
| Key | Fix | Where | Source verified |
|---|---|---|---|
| F1 (X1, MAJOR) | New BU12, the lease relation: (a) a C1-style service check, v2.lease_secs >= T_PASS + the ack deadline + LEASE_MARGIN (a stated margin, value HYPOTHESIS, C5-O1), else relay_v2_unsupported before any v2 call (C03 AMENDMENTS AM-25, beside E9's ceiling AM-26); (b) the ACK set of a mailbox is flushed immediately after that mailbox's items are processed, and per-mailbox processing is bounded to lease_secs / 2, items past it HOLD; (c) the read's vector as V1125. D-R5 and BU2 (4) follow (b). The arithmetic consequence is stated in BU12: at the PROPOSED T_PASS 60 s and ack deadline 15 s the floor exceeds 75 s, above the v1 relay's shipped default pull lease of 60 s | T1 D-R5; T3 BU2, BU12 (new); T8 V1125 (new); T9 C5-O1 | C03 EP4 "deletes LEASED copies only" and V16 "ack an unleased id -> 200 acked 0; the item remains" (M contracts); C03 AM-6 lease floor 1 s; S src/store.rs:7 PULL_LEASE_SECS_DEFAULT = 60 (also at qsl-server main 0c04fa47), :8 ceiling 3600; S src/lib.rs:115 MAX_QUEUE_DEPTH_CEILING 257 |
| F2 (X2, MAJOR) | New T1a: every code of expected_non_admission (C protocol_state:1276-1300), the K04 codes (delivery:862) and Local(_), each with exactly ONE disposition: SKIP_BOUND -> DEFER-P, new reason P6; EPOCH_UNKNOWN with epoch == the highest recv epoch + 1 -> P2, otherwise DISPOSE; EPOCH_CAPACITY -> DEFER-P (new reason P7: resolves when an epoch retires; horizon session close); EVENT_CAPACITY / TRANSACTION_CAPACITY / COMPLETION_CAPACITY -> DEFER-C (C2, C5); RECEIPT_CONTEXT_CAPACITY, SEND_WINDOW, MAINTENANCE_CAPACITY -> P5; every other Apply code -> DISPOSE; Local(_) -> FAULT; the read's vector as V1126. Placed by the seat, named here: (i) the table is keyed by CHANNEL, because TRANSACTION_CAPACITY is DEFER-C as an Apply error and P5 as a K04 deferral (the draft's K5), and SEND_WINDOW and MAINTENANCE_CAPACITY occur only on the K04 channel; (ii) PEER_TARGET_CAPACITY, which the read's X2 lists among the deferrable codes but F2's enumeration does not name, is DEFER-P P5 by D-R4, not "every other Apply code -> DISPOSE"; (iii) NoCandidate (:1274) is mapped by the draft's own K4 rule (P1 when a pending candidate holds the SID, else DISPOSE) | T1 D-R4, K4, T1a (new); T2 G4, P5, P6 and P7 (new); T8 V1126 (new) | C protocol_state/mod.rs:1268-1302 (the one bool; :1297-1298 "Admission backpressure leaves this incoming frame uncommitted"); directional_core.rs:15-16 (MAX_SKIP 16, MAX_EPOCHS 2), :614-615 (PEER_TARGET_CAPACITY, MAX_TARGETS 1), :639, :651-652, :688-689, :725-726; directional_delivery.rs:352, :396 (CLOSURE_CAPACITY is a shape bound), :735, :769, :814, :862 |
| F3 (X3 a) | K10: the class caps 12,288 / 65,536 stay DISPOSE; the "any item <= mbb" arm is removed and an item whose decoded data exceeds mbb makes the WHOLE response RESP-REFUSED per C03 C3 | T1 K10 | C03 T3 C3 (an item's decoded data > v2.max_body_bytes -> relay_v2_response_malformed); C03 AM-18 (mbb >= 65,536) |
| F4 (X3 b) | G1 gains the exception for the one superseded receipt of C02 T2 step 8 / X-b (durable; the crossing loser's replay record); P3's resolution reads "left to the C03 slot sweep (the slot is Redeemed and no longer pulled, C02 X-h, D-R6)" -- the read's FIRST wording (its alternative, one final pull of that slot inside the recovery window, is not taken); T7 X11 reworded; K1 and V1116 follow | T2 G1, P3; T1 K1; T7 X11; T8 V1116 | C02 T2 step 8 (M :228 "the only write is one bounded superseded receipt of an AUTHENTICATED A1"), T3 "Remembered" (:273), RP7 (:271), X-b (:290), X-h (:295) |
| F5 (X3 c) | G4 reads "mark the entries of the matching reason STALE so the next redelivery takes the full path" -- the FIRST option (the ruling); no item bytes are held in memory | T2 G4 | T2 G1 (holds H(item bytes), not the bytes), G2 (retry by redelivery only), C03 T3 (no nack verb) |
| F6 (X3 d) | BU2 phase (2) gains a round-robin cursor over sessions for owed-control pushes, resumed like BU6's | T3 BU2 | T3 BU6 (the application-send cursor) |
| F7 (X3 e) | OPEN_MAX and SETTLE_MAX defined (HYPOTHESIS, C5-O1); settles included in REQ_PASS | T3 BU2; T9 C5-O1 | the draft names OPEN_MAX once (BU2) and SETTLE_MAX nowhere (measured: grep counts 1 and 0) |
| F8 (X4) | LK9 reads "sensitive content gone at LK2 (immediate); key material gone after at most one in-flight request deadline (BU4) plus one commit underway (LK5)"; V1308 asserts both bounds | T6 LK9; T8 V1308 | D src-tauri/src/gateway.rs:121-128 (the gate held across spawn_blocking); BU4 deadlines (connect 10 s, push/pull 30 s) |
| F9 (X4) | BU11's B5/B6 cite reads "main.rs:221 (receive max), :231 (poll_max_per_tick) -> transport/mod.rs:442, :516 (no clamp)"; the five DRIFT rows of the read's section 1 corrected in the same pass (optional in the read; taken): D-R6 :490 -> :491; E8 C:497 -> C:495; decision 8 and E3 "hidden, not cleared" -> D:main.js:114 (show()); BU1 the tick constants at D main.js:2045-2059; T5 the invite latch is D:main.js:2535 inviteInFlight (PIN invite:916-931 is the local commit-before-network insert) | T3 BU11, BU1; T1 D-R6; T4 decision 8; T5 intro; ESCALATIONS E3, E8 | M main.rs:221, :231; M transport/mod.rs:442, :516, :1768 (clamps a different pull); M lib.rs:262-265 = identity_show, M main.rs:262-265 = recv_file_conflict (the MISS); C transport:491, :495; D ui/main.js:114, :2045-2059, :2535 |
| F10 (X4) | Vectors V1119-V1124 (F11) and V1318-V1326 (F13) for the rules the read lists with no vector, each with its delta symbol and base; the discriminating case and expected cells are the seat's wording of each named rule, nothing beyond it | T8 (new rows) | the read's section 6.2 list |
| Q1 | AMENDMENT A5: the engine keeps the four states; the GUI draws Queued/Prepared with no tick, an OUTLINE tick "on its way" for relay accepted (M3), a SOLID tick "delivered" (M4); NEVER a read mark. T4 M-rows and the decision 1 row updated; E1 RESOLVED; C5-O12 closed; V1305 follows | T4 M1-M4, decision 1; ESCALATIONS E1; T9 C5-O12; T8 V1305 | A5 items 1-5; THE PLAN sec 4 table |
| Q2 | option (a): "New message" only while unlocked and the window is not focused; nothing while locked. Option (d) (a read-only has-mail capability) recorded OPEN (C5-O19), not taken | T4 N1, decision 7; ESCALATIONS E2; T9 C5-O9, C5-O19 (new) | C03 T4 (R vault-held); D ui/main.js:2086-2089 (tickGateOpen requires an unlocked screen) |
| Q3 | on lock, wipe the main window of all information at once (not waiting on the engine: LK1-LK2 outside the gate) and show a small unlock window on top of it; the MAJOR stands until F13 builds it | T6 LK2; T4 decision 8; ESCALATIONS E3; T9 C5-O11, packet table; T8 V1308 | D src-tauri/src/lib.rs:254-298, :586-591; ui/main.js:114, :2016-2019 |
| Q4 | a per-message failure mark in AMBER (accent; red stays reserved) with a retry action on M5 and M6; the exact look is the operator's at F13; conversation-level notices remain for causes that are not one message's | T4 M5, M6, decision 1; ESCALATIONS E4; T7 X27; T9 C5-O10 | C04 Q01, Q11, K07 (at M) |
| Q5 | a user setting "keep messages for": 1 week / 1 month / 1 year / Forever, default 1 month; expired history deleted automatically; an entry whose message is undelivered (M1-M3) or closed_undelivered (M6) is never retired by expiry; PLUS a per-peer history share H_P (C04 AMENDMENTS AM-1, value HYPOTHESIS) so one contact cannot fill R07 (retention interplay: C04 AMENDMENTS AM-2); what "delete a conversation" does to still-pending messages is OPEN for F13 (C5-O20) | T6 HI3, HI4; T2 C3; T7 X28, X30; T9 C5-O6, C5-O7, C5-O20 (new), packet table | C04 R07 (M :95, "only a user-visible history retirement"), Q06, Q08 (Q_P < Q_S) |
| Q6 | blocked = full stop: a blocked contact's frames are DISPOSED (ACKed away, not decrypted into history, no receipt, no tick), nothing is sent to them, a "Blocked" notice in the conversation header (new T4 C8). Stated: the blocked peer cannot tell blocked from offline; its re-pushes continue at the BU8 cadence (bounded); a relay-level block needs per-peer mailboxes (C3-O13, OPEN). "Nothing is sent" is carried into BU2 (this side's owed controls and receipts for that session are not pushed) | T1 K11; T3 BU2; T4 C8 (new); T8 V1117; T9 C5-O8 | C03 C3-O13 at M; T3 BU8 |
| Q7 | option (a): keep the typed text, do not queue, say so once | T4 C4, decision 6; ESCALATIONS E5; T8 V1307; T9 C5-O13 | C04 Q03 (refusal before any write) |
| Q8 | the operator gives final approval of how the app looks and functions (C05 now; F13 acceptance later) | ESCALATIONS E6; T9 C5-O17 | ROADMAP item 3 (M ROADMAP.md:16) |
| E1 | RESOLVED by A5 (Q1) | ESCALATIONS E1 | as Q1 |
| E2 | by Q2 | ESCALATIONS E2 | as Q2 |
| E3 | by Q3 (MAJOR stands until F13 builds it) | ESCALATIONS E3 | as Q3 |
| E4 | by Q4 | ESCALATIONS E4 | as Q4 |
| E5 | by Q7 | ESCALATIONS E5 | as Q7 |
| E6 | by Q8 | ESCALATIONS E6 | as Q8 |
| E7 | UV3 ACCEPTED (C02 identity_changed governs) | T6 UV3; ESCALATIONS E7 | C02 T5 at M |
| E8 | ENG-0358 filed (receive-path candidate defects at #1831/P28); repair named for F11 (T1 D-R1, D-R4, K6, K10, T1a; V1101, V1102, V1106, V1109, V1126); NOT repaired here | ESCALATIONS E8 | the read's E8 verdict: C transport:489, :495, :401-407, :520-522, :642; delivery:730, :759-766; protocol_state:1287; P28 handshake:696-701, :2862-2867 |
| E9 | by F1: C03 AMENDMENTS AM-26, v2.max_body_bytes <= 1,048,576, else relay_v2_unsupported before any v2 call; C5-O14 closed | T3 BU3; ESCALATIONS E9; T9 C5-O14 | S src/lib.rs:113 MAX_BODY_BYTES_CEILING = 1024 * 1024 |
| E10 | ENG-0359 filed (lock does not stop the engine at main); repair named for F05/F13 (LK7; V1317); NOT repaired here | ESCALATIONS E10 | the read's E10 verdict: M vault/protection.rs:255-259; msgqueue:295; quarantine:206; vault/mod.rs:275-283 |
| E11 | ENG-0360 filed (truthful-status defects at main); repair named for F13 (T4 M1-M6, C1-C3; V1304, V1306); NOT repaired here | ESCALATIONS E11 | the read's E11 verdict: M transport:1965, :4484-4486, :4687-4691; msgqueue:1104-1107, :1144 onward |
Not taken, named: the SR-15 read's NOTEs outside F1-F10 -- DISPOSITION_CONFLICT's own word (sec 2.1), committed
handshake replies named in BU7's non-backing-off set (sec 3.4), the throughput sentence beside C5-O1 (sec 3.5), the M3
lost-200 and relay_busy surface refinements (sec 4.3 a, b), AI2's 48-byte fit note (sec 5.5) and the Q1-Q8 option notes
of its section 8 (the operator has answered); the ruling names none of them. The per-peer history share H_P, which the
read recommended as a NOTE, IS taken, by the ruling's Q5. Also changed, as consequences of the ruling only: the title
line, this FIXES section, the RULED clauses of ESCALATIONS E1-E11, the ESCALATIONS and OPERATOR QUESTIONS headers, the
ANSWERED line after the questions, the state cells of T7 X27, X28, X30 and T9 C5-O6, C5-O8..C5-O14, C5-O17, two rows of
the packet OPEN_QUESTIONS table, and the closing END line. The C03 and C04 amendments are carried by the AMENDMENTS
appended to those contracts with this one, not by this text.

PLAN: QSL-solution-plan rev3 d9016e53d2ab46c32b7a7cb060ea70dc79421617e9b054848b464ba7a5518275 (amended A4), card F01,
sub-assignment C05. Lane NA-0783 (D-1425..D-1432). Drafted 2026-09-24 by the executor seat (opus/high). Status of every
row: PROPOSED; ACCEPTED only by the Director's ruling after the SR-15 read (fable/xhigh).
Scope, THE PLAN row C05 verbatim: "Per-item dispositions and ACK rules; pending SID/epoch and temporary admission-capacity
deferral; finite request/byte/time/fairness budgets; DTO/status meanings; action-ID scope/retention; text limits,
history policy, unverified-send policy and lock cancellation." It is the contract F11 (the single bounded dispatcher)
and F13 (typed desktop text and verified-contact acceptance) build against. Not in scope: C06 file bytes, C07, visual
design. RULING_PLAN_F00 K-01 maps ROADMAP item 3, the "lean background design" (rungs, transitions, persistence,
retries, idempotency, close/lock/crash survival, the relay lease law, the handshake-poll class and "one fetch per
mailbox per beat"), to C05/F11; this contract carries that subject (T1-T3, T6 LK).
REQUIRED INPUT (Director's ruling on the packet's role): the operator's UI packet QSL_UI_PACKET_20260919 (banked 444 at
the operator record, sha256 301a3620012583c11c3527e62e7e29b791315c916b0d78f1124f2522ccca8626). C05's user-visible
meanings match its DECISIONS.md (fifteen); every place the engine or THE PLAN cannot deliver a decision exactly is
ESCALATED (list at the end), never changed silently. OPEN_QUESTIONS stay open unless named in T9. Mockups are layout
authority only and are not refined here; "which states exist" and "what it says" in a mockup are read, and where a
meaning below touches a drawn state the mockup is cited.
Fits C01 (docs/ops/contracts/C01_versions_and_boundaries.md at main, AM-1..AM-4): C05 allocates NO wire or local-schema
identifier; every spelling marked NEW is PROPOSED and registered through DOC-SCL-002 by the implementing PR (C01 O9);
refusal before effects at every admission point; the contact id is opaque (C01 T5, O5) and aliases are local-only;
every store named here is inside the successor directory (O8). Fits C02 (AM-1..AM-15), C03 (AM-1..AM-24) and C04 as
merged: T7 answers every cell of C01-C04 that names C05 (census: 66 matched lines, zero unclassified).
Prior bindings read and kept (operator-blessed method banks of the operator record): the delivery-ladder design of record v2
(DESIGN_delivery_ladder_metronome_v2_20260825, sha256 aba8e2a5...) P1-P5 and sec 1.1-1.3 (one handler, concurrency
guard with a rerun bit, lease law, backoff-and-report); its AMENDMENT 1 A1-A4 (ONE PULL PER BEAT PER MAILBOX; classes are
dispatch targets, not pullers; no frame fetched and silently declined; independent fetchers refused at design time) and
AMENDMENT 2 A5-A7 (disposal is part of the design; a new consumer joins the dispatch table; declined classes stated);
ORDER_one_facade_two_frontends (P-FACADE: product behaviour enters through the library facade the GUI calls).
Sources measured (read-only, bare mirrors; the drafting mission's measure files, zero unclassified in each):
measure_receive_budgets.md sha256 5dcbcb6da4650cbba92725a3c0a8281b0bbc22ae54ac1eddae5d9d27193aa611 (rows R1-R70, B*),
measure_engine_status.md sha256 bd93a0a56e3dfece3ae83e12de774fa3846b7950e1461d57c03055787c38ec1d,
measure_desktop.md sha256 25f877c94cf2f80d3faa211ac5b8bd887199c3142b7d5020f1251fdcfc75b149. Revisions: M = qsl-protocol
main 4e9dfd0e (#1842 merge); C = #1831 ffc8fc52; P28 = #1828 e29a07df; D = qsl-desktop main 92cba80a, which pins qsc
PIN = 08c0e327. File:line is qsl/qsl-client/qsc/src/<file>:<line> unless prefixed D: (qsl-desktop). Key citations were
re-read by this seat at the source (named "verified" in the drafting mission's REPORT.md, sha256
63af0b5c6da2d8663bdfaa4c9e8adfc8d831b4c7305b6ca47cf98d3ca4ff696a). Nothing was built or run: every "today" is a source
reading.
NOTATION. mbb = the relay's advertised v2.max_body_bytes (C03 EP10); N = items requested per pull; "pass" = one
dispatcher pass (one handler run, THE PLAN sec 5 drive_step); "commit" = the durable authority of the item's class
(C04 "paired commit" for directional state; the C02 step-10 / A2-6 commit for handshakes). Relay verbs are C03's v2
endpoints (EP1-EP10). No nack/release verb exists in v2 (C03 T3): an un-ACKed leased item returns only at lease
expiry (v2.lease_secs).

==============================================================================================================
T1. PER-ITEM DISPOSITIONS AND ACK RULES
==============================================================================================================
Disposition vocabulary (the "disposition vocabulary" C02 T2, C03 AM-8 and C03 C3-O6 leave to C05; spellings PROPOSED):
| Id | Disposition | Relay action | Local effect |
|---|---|---|---|
| ACK | admitted | ACK after the item's commit is durable | the commit; outputs project FROM committed state; an owed reply (NDR1, B1, A2) is a committed obligation pushed independently (D-R1) |
| ACK-DUP | duplicate of committed work | ACK | no new effect; the exact SAVED reply is re-sent where one exists (Disposition receipt; C02 RP1 B1; RP5 A2) |
| DISPOSE | permanently invalid after full dispatch (D-R4) | ACK | nothing durable; a bounded lossy diagnostic only (D-R3) |
| DEFER-P | known pending prerequisite (pending attempt/session, permissible future epoch, protocol wait) | NOT ACKed | no trusted-state commit; an in-memory deferral entry (T2); re-evaluated on redelivery |
| DEFER-C | valid authenticated new work awaiting LOCAL admission capacity | NOT ACKed; NO acceptance NDR1 | no trusted-state commit; in-memory entry (T2); the user-visible notice T4 N-HIST / N-FULL where the cause is user-actionable |
| FAULT | local store / lock / corruption failure | NOT ACKed | reported as a LOCAL fault (never as malformed input or relay offline); the pass stops intake from that store; ACKs already earned are flushed |
| HOLD | fetched but not processed in this pass (time budget ended, or the lock generation changed, T6 LK) | NOT ACKed | nothing; counted and reported (ladder A3) |
| RESP-REFUSED | the whole pull response is refused (over CAP, strict-JSON failure, C03 C3) | nothing ACKed; no item parsed | relay_v2_response_too_large / _malformed; that mailbox backs off (T3 BU7); reported as a relay fault, not "no relay" |
Item kinds x conditions. "C02 n" = C02 T2 step n; "RPn" = C02 T3; "AM-n" = the named contract's amendment.
| Kind (class test, before any parse) | Valid | Duplicate | Deferred prerequisite | Capacity-deferred | Permanently invalid | Local failure |
|---|---|---|---|---|---|---|
| K1 A1 envelope, "QSLH" env_type 1, on an invite slot | C02 1-10; ACK after the step-10 commit (candidate + exact B1 + receipt), NOT after the B1 push | RP1: ACK-DUP, re-send the saved B1 | crossing loser at the lower side (C02 X-b): DEFER-P until W applies, then left to the C03 slot sweep (T2 P3, F4); LIVE prior-generation candidate (C02 AM-3 FX6, handshake_lifecycle_occupied): DEFER-P until that candidate's horizon (C03 AM-2 b, AM-19) | R11 quota full (C02 AM-10, C04 X01): DEFER-C until a record retires (V1002) | C02 1-6, 9 refusals; A1 at or after RU_I (C03 AM-2 a); RP3, RP8; committed session / applied record for that identity (C02 AM-12); invite revoked locally before this A1 was answered (C3-O6, T7): DISPOSE | FAULT |
| K2 B1 envelope, "QSLH" env_type 2, on the inbox | C02 1-10 (selection commit + exact A2); ACK after commit | RP5: ACK-DUP, re-send the saved A2 | none (a B1 exists only after the attempt's R6 commit) | the session reserve (C04 R02) cannot be funded at selection: DEFER-C until capacity frees or the attempt's horizon (then DISPOSE) | C02 1-6, 9; superseded attempt (handshake_crossing_superseded, C02 X-c); no operation (C02 AM-5); after the redeemer's horizon (C03 T6, AM-16) | FAULT |
| K3 A2, bare "QHSM" v3 type 3, on the inbox | C02 A2-1..A2-6; ACK after the session commit | exact duplicate: ACK-DUP, no state | none | session reserve unfundable at A2-6: DEFER-C until capacity frees or the candidate horizon | A2-1..A2-5 refusals; A2 after the candidate failed-expired (A2-2, the stated last-message residual, C03 AM-2) | FAULT |
| K4 NDE1 ordinary frame whose NDI2 kind is 0 (text) | staged receive (C receive_inner, directional_delivery:722-851), paired commit of Disposition + event; ACK after commit; the NDR1 is pushed from the committed Disposition independently | a Disposition still holds the frame (C :752-757): ACK-DUP, re-send its saved receipt | SID of a PENDING responder candidate (A2 not yet applied; THE PLAN sec 3 "unknown SID lookup must cover both committed and pending handshakes"): DEFER-P; epoch = the next epoch whose boundary is not yet admitted, within the core's admissible window (MAX_EPOCHS 2, core:16): DEFER-P; a frame more than MAX_SKIP 16 ahead in the current epoch (SKIP_BOUND): DEFER-P, P6; an epoch boundary awaiting an old epoch's retirement (EPOCH_CAPACITY): DEFER-P, P7 (T1a, F2) | ordinary receive admission full (C04 R05 64 events / 4 MiB), projection credit (R06), history quota (R07), vault aggregate (R01): DEFER-C; the frame's piggybacked closures wait with it (T2 G5) | no committed AND no pending SID; AEAD/MAC failure; below the closed floor or confirmed count (CLOSED_REPLAY: a retired duplicate, C :759-766); NDI2 kinds 1-4 (INTEGRATION_FILE_GATED, C01 T4) and 6-255; epoch outside the admissible window (EPOCH_UNKNOWN for any epoch other than the highest recv epoch + 1, T1a): DISPOSE | FAULT |
| K5 NDE1 control frame (NDI2 kind 5 maintenance; core typed kind 2 advertisement; boundary kind 1) | admitted on control credit (C04 R03), never on ordinary credit; ACK after commit | as K4 | as K4, plus the protocol waits C04 K04 (SEND_WINDOW, RECEIPT_CONTEXT_CAPACITY, TRANSACTION_CAPACITY, MAINTENANCE_CAPACITY): DEFER-P (deferral is not failure) | NEVER capacity-deferred on ordinary credit (C04 K02, K03; I05) | as K4 | FAULT |
| K6 NDR1 receipt, exactly 113 B | exact receipt for an outstanding Flight: commit (Flight retired, completed id; DELIVERED); ACK after commit | NDR1 matching no outstanding Flight (its Flight already retired by an earlier exact receipt, or junk): DISPOSE -- no effect, and never DELIVERED | none | COMPLETION_CAPACITY (64 undrained completions, C :735): DEFER-C until projection drains (C04 Q07 makes it unreachable once rows are reconstructed) | length != 113, MAC failure (wrong peer or wrong message: never DELIVERED, THE PLAN Q1 table): DISPOSE | FAULT |
| K7 retired formats: QSLH-1 (01 01 / 01 02), QSE (01 00), bare QHSM type 1/2 (C02 AM-9), QHSM v1/v2 | -- | -- | -- | -- | always DISPOSE (C02 T1g, 2a, AM-9; C01 T2 D, F) | FAULT |
| K8 unknown class (frameclass Unknown) | -- | -- | -- | -- | DISPOSE | FAULT |
| K9 malformed within a known class | -- | -- | -- | -- | DISPOSE (the class's own code) | FAULT |
| K10 over the class cap: envelope > ENV_MAX 12,288; NDE1 > MAX_WIRE 65,536 (each cap at or below mbb, C03 AM-18) | -- | -- | -- | -- | DISPOSE, decided on the length BEFORE any parse or copy; one over-cap item never aborts the batch (contrast P28 :690-702, :2862-2867, R54). An item whose decoded data exceeds mbb is NOT a per-item case: the WHOLE response is RESP-REFUSED (relay_v2_response_malformed, C03 C3) (F3) | -- |
| K11 user-blocked contact's session (a drawn state: MOCKUP_contact_details D, "Anything they send while blocked is dropped, not delivered later") | -- | -- | -- | -- | DISPOSE after the SID lookup, before any decrypt of the body; whether an NDR1 is still owed is OPERATOR QUESTION Q6. RULED (Q6): blocked = full stop -- the frame is ACKed away, not decrypted into history, no Disposition, no receipt, no tick at the peer; nothing is sent to them (UV2 refuses application sends; BU2 does not push this side's owed controls or receipts for that session); the conversation header shows "Blocked" (T4 C8). Stated: the blocked peer cannot tell blocked from offline; its re-pushes continue at the BU8 cadence (bounded); a relay-level block needs per-peer mailboxes (C3-O13, OPEN) | -- |
Rules.
 D-R1 ACK ORDER (I03, I07). An item is ACKed only (a) after the durable commit that admits it (ACK, ACK-DUP), or (b) after
      full dispatch concludes DISPOSE. Never before the commit; never conditioned on the relay accepting a reply. CHANGE vs
      C: C ACKs an admitted frame only if its reply push succeeded, and leaves both pending on 429 (transport:512-516, R35);
      C05 decouples: the reply is a committed obligation retried by the pass (T3 BU2), the item is ACKed on commit.
 D-R2 NO PREMATURE RECEIPT. An NDR1 (acceptance receipt) is produced only from a committed Disposition (C04 K01); a
      DEFER-C or DEFER-P item produces none.
 D-R3 INVALID TRAFFIC WRITES NO DURABLE HISTORY (I07). DISPOSE records a per-pass in-memory counter per (kind, code) and at
      most one debug-log event per (kind, code) per pass carrying the kind and the code only: no relay id, SID, contact
      id, alias, byte or length (the Diagnostics promise, packet IMPLEMENTER_NOTES sec 6). No quarantine row (M's
      quarantine_then_ack, R21-R28, is not carried on the successor path; C01 O6 owns the store).
 D-R4 FULL DISPATCH before DISPOSE: every check of the item's class ran, and the SID/attempt lookup covered committed AND
      pending records; an item that a live pending record could make valid later is DEFER-P, not DISPOSE.
      The code-level map from the receive path's codes to exactly one disposition is T1a (F2).
 D-R5 ACK FLUSH. The pass collects its ACK set per mailbox and sends it in at most one EP4 per mailbox per pass (<= 4096
      ids, C03 EP4), IMMEDIATELY after that mailbox's items are processed, never at the end of the pass (T3 BU12 (b),
      F1). A failed or lost ACK is not retried in the pass; the redelivered item comes back as ACK-DUP or
      DISPOSE, never re-applied (vector V1106). No 404 is read as success (C03 C2; not carried: M/C LegacyComplete, R3/R4).
 D-R6 ONE CONSUMER (ladder AMENDMENT 1 A1, A2, A4; AMENDMENT 2 A6). The dispatcher owns every pull of every mailbox the
      vault holds R for (the inbox and each live invite slot); kinds are dispatch targets, never pullers. The per-command
      pulls measured today -- receive (transport:392-525, one peer per call, :491), handshake poll (handshake:2835, returns
      after the first consumed frame, R44-R52), invite accept (first item only, R58-R59), invite finish (fan-out, R62-R63)
      -- all route through it (THE PLAN F11 "no per-contact competing pulls"); CLI verbs become thin verbs over the same
      facade step (P-FACADE).
 D-R7 A FETCHED ITEM IS NEVER SILENTLY DECLINED (ladder A3, A5, A7). DEFER-P, DEFER-C and HOLD are counted per pass and
      reported on the Diagnostics surface (counts only); DEFER-C with a user-actionable cause is also said once in the
      conversation (T4). Their lease cost is named: a deferred item is invisible to every later pull for v2.lease_secs.
 D-R8 CLASS BEFORE PARSE. The kind is decided from the leading bytes and the length (frameclass, with the QSLH arm C02
      T1g) BEFORE any parse, decrypt or allocation beyond the item; the per-kind caps (K10) apply first.

T1a. CODE TO DISPOSITION (F2; SR-15 X2). Every code the receive path can return has exactly ONE disposition. Channel A =
the Apply codes that DirectionalUpdateError::expected_non_admission folds into ONE bool at C (protocol_state:1276-1300;
F11 MUST NOT read that bool as "permanently invalid": it holds the deferrable codes too); channel K = the deferral codes
control_before_receive returns as Ok(Some(code)) (C directional_delivery:862, the C04 K04 set). The table is keyed by
channel because TRANSACTION_CAPACITY occurs on both with different dispositions.
| Channel | Code(s) | Disposition | Reason / rule |
|---|---|---|---|
| A | SKIP_BOUND (a frame more than MAX_SKIP 16 ahead inside the CURRENT epoch, core:651-652, :725-726) | DEFER-P | P6 (new): earlier frames of this epoch not yet admitted |
| A | EPOCH_UNKNOWN (core:639) | DEFER-P when the frame's epoch == the highest recv epoch + 1; otherwise DISPOSE | P2 (the split rule) |
| A | EPOCH_CAPACITY (core:688-689: a boundary that cannot be admitted until an old epoch retires) | DEFER-P | P7 (new): resolves when an epoch retires; horizon session close |
| A | EVENT_CAPACITY, TRANSACTION_CAPACITY, COMPLETION_CAPACITY | DEFER-C | C2 (EVENT_CAPACITY, TRANSACTION_CAPACITY), C5 (COMPLETION_CAPACITY) |
| A | RECEIPT_CONTEXT_CAPACITY | DEFER-P | P5 |
| A | PEER_TARGET_CAPACITY (core:614-615: the peer's advertised targets already at MAX_TARGETS 1) | DEFER-P | P5 (a deferrable code, grouped with the admission-backpressure codes at protocol_state:1297-1300; placed by D-R4) |
| A | every other Apply code of protocol_state:1276-1296: PARSE, MAGIC, TYPE, WIRE_BOUND, CT_LENGTH, BODY_LENGTH, TRAILING, HEADER_AUTH, HEADER_BINDING, BODY_AUTH, SESSION_DIRECTION, ORDINARY_SHAPE, EPOCH_BINDING, TERMINAL_BOUND, REPLAY, BOUNDARY_REPLAY, ROOT_PARENT_OWNER, PREVIOUS_EPOCH, PQ_SHAPE, TARGET_SPENT, TARGET_NONMONOTONIC, TARGET_UNKNOWN, TERMINAL_REGRESSION, BOOTSTRAP_TERMINAL, DH_NONCONTRIBUTORY, TYPED, TYPED_LENGTH, ADV_LENGTH, ADV_AUTH, TARGET_EQUIVOCATION, RECEIPT_BINDING, RECEIPT_AUTH, RECEIPT_CONTENT, RECEIPT_NOT_OUTSTANDING, DISPOSITION_CONFLICT, CLOSED_REPLAY, INTEGRATION_LENGTH, INTEGRATION_PROFILE, INTEGRATION_KIND, INTEGRATION_ID, INTEGRATION_PADDING_PROFILE, INTEGRATION_PADDING_SIZE, INTEGRATION_PADDING_NONZERO, INTEGRATION_BODY, INTEGRATION_FILE_SHAPE, INTEGRATION_FILE_REQUEST, INTEGRATION_FILE_GATED, CLOSURE_CAPACITY (a shape bound, delivery:352, :396), CLOSURE_ORDER, CLOSURE_FINAL, CLOSURE_EPOCH, CLOSURE_PREFIX, CLOSURE_TERMINAL, APPLICATION_ID_CONFLICT, timeline_id_conflict | DISPOSE | after full dispatch (D-R4) |
| K | SEND_WINDOW, RECEIPT_CONTEXT_CAPACITY, TRANSACTION_CAPACITY, MAINTENANCE_CAPACITY | DEFER-P | P5 (K5: "deferral is not failure") |
| -- | NoCandidate (protocol_state:1274) | DEFER-P when the SID belongs to a PENDING responder candidate; otherwise DISPOSE | P1 (K4, D-R4) |
| -- | Local(_) | FAULT | never classified by string (protocol_state:1269-1271) |
Vector: V1126 (T8).

==============================================================================================================
T2. PENDING SID/EPOCH AND ADMISSION-CAPACITY DEFERRAL
==============================================================================================================
| # | Rule |
|---|---|
| G1 | WHAT IS RETAINED. Nothing durable, except the one superseded receipt of C02 T2 step 8 / X-b, which is durable and is the crossing loser's replay record (F4) (I07; C02 AM-13 "the deferral may be remembered in memory; no durable write"; THE PLAN sec 3 "leave trusted receive state uncommitted"). The ITEM is retained by the relay (un-ACKed, leased). The dispatcher keeps an in-memory DEFER table per mailbox: {relay item id, H(item bytes), reason (T2 table), first_seen, last_seen, count}, at most DEFER_MAX entries (PROPOSED 256, HYPOTHESIS for F11), least-recently-seen evicted; lost on restart and cleared on lock (rebuilt from redelivery). The table is an optimisation and a report source, never an authority: an evicted entry only costs a re-verification |
| G2 | HOW IT IS RETRIED. By redelivery only: the relay re-offers an un-ACKed item after v2.lease_secs (no nack verb exists). On redelivery the item is re-dispatched in full; if H(item) matches an entry whose reason is unchanged and no resolving event (G4) occurred since last_seen, the dispatcher may skip re-verification and keep it DEFER (fast path; the result must equal the full path's) |
| G3 | FOR HOW LONG, per reason. Each resolves to ACK / ACK-DUP (condition met) or DISPOSE (horizon passed); the relay's retention bounds all of them (C03 T7; AM-13: retention_ttl >= H_REC) |
| G4 | RESOLVING EVENTS (mark the entries of the matching reason STALE so the next redelivery takes the full path; no item bytes are held in memory, F5): an A2 applied or a session committed (P1); a boundary admitted (P2); any frame of that epoch admitted (P6); an epoch retired (P7); a crossing winner applied (P3); a candidate failed-expired or released (P4, C1); a projection drained, a Disposition closed, a history unit retired, a session closed (C2, C3, C4) |
| G5 | OWED CONTROLS KEEP PROGRESSING WHILE NEW WORK IS DEFERRED (THE PLAN F11): (a) this side's owed receipts, maintenance, advertisements and closures are pushed from the vault every pass BEFORE any application send (C04 Q09; T3 BU2 order), never through the queue, never gated by queue fullness or by any DEFER-C; (b) incoming controls (K5) and receipts (K6) are admitted on their own funded credit (C04 R03, K02), never capacity-deferred; (c) a DEFER-C ordinary frame (K4) cannot be partially applied: its piggybacked closure promises wait with it. The peer's standalone closure carrier (maintenance kind 5 body [2], C04 K01) is a control and is admitted. F06 MUST show no schedule turns "our ordinary receive is full because our Dispositions await the peer's closures, and the peer's closures ride ordinary frames we defer" into a cycle (C5-O3, vector V1112; joins C04 K02 / C4-O2) |
| G6 | NO DURABLE REJECTION HISTORY, NO QUARANTINE: a deferral that ends in DISPOSE leaves only D-R3's lossy counter |
| G7 | STATED LIMIT (remote capacity, C4-O9, C3-O13): deferred items keep occupying the recipient mailbox depth and a fetch slot every lease period. With one shared inbox per (identity, relay) (C03 C3-O13 today's shape), D deferred items delay other peers' items by at most ceil(D / (N x passes per lease period)) lease periods, and a sender whose push meets max_queue_depth gets 429 (its Flight stays PREPARED, T3 BU7). Per-peer mailboxes would isolate this (C3-O13 stays OPEN, T9) |
Reasons (the "pending SID/epoch" and "temporary admission-capacity" arms of THE PLAN sec 3):
| Reason | Kind | Retained until (resolved) | Horizon (then DISPOSE) | Source |
|---|---|---|---|---|
| P1 pending responder SID | K4, K5 | the A2 is applied (the session commits) | the candidate is failed-expired, RU_I + 3 x CLOCK_SLACK (C03 AM-2 b, AM-19) | THE PLAN sec 3; F11 "A2/boundary behind more than one batch" |
| P2 future epoch | K4, K5 | the boundary frame is admitted | the epoch leaves the admissible window, or an explicit session close (C04 K07) | core MAX_EPOCHS 2 |
| P3 crossing loser (lower side) | K1 | the winner W applies; the slot is then Redeemed and no longer pulled (C02 X-h, D-R6), so the item is left to the C03 slot sweep, un-ACKed and bounded by the sweep (F4) | the lower side's own attempt horizon | C02 X-b, X-h |
| P4 lifecycle occupied (prior generation live) | K1 | the prior candidate is failed-expired and released (then processed: a new generation) | the new claim's recovery_until (AM-23: it outlives RU_1 + 3 x CLOCK_SLACK) | C02 AM-3 FX6; C03 AM-5 CL13 |
| P5 protocol wait | K5; K4 via T1a | the peer's receipt or closure frees the window / context / slot | explicit session close | C04 K02, K04 |
| P6 earlier frames of this epoch not yet admitted (SKIP_BOUND) | K4, K5 | any frame of that epoch admitted | as P2 | C core:651-652, :725-726 (F2) |
| P7 epoch window full (EPOCH_CAPACITY) | K4, K5 | an epoch retires | explicit session close | C core:688-689 (F2) |
| C1 R11 quota full | K1 | a first-connection record retires | the A1's RU_I | C02 AM-10; C04 V1002 |
| C2 ordinary receive / aggregate full | K4 | a projection drains, a Disposition closes, or the aggregate frees | relay retention; the sender's exact re-push (T3 BU8) re-creates the item later | C04 R01, R05, R06 |
| C3 history quota full (R07, or that peer's share H_P, C04 AMENDMENTS AM-1) | K4 | history retires: expiry under the user's "keep messages for" setting, or a user deletion (T6 HI4; RULED Q5) | as C2 | C04 R07 |
| C4 session reserve unfundable | K2, K3 | capacity frees (a session closes, history retires) | the attempt / candidate horizon | C04 R02, V605 |
| C5 completion backlog | K6 | projection drains | as C2 | C :735; C04 Q07 |

==============================================================================================================
T3. FINITE BUDGETS PER DISPATCHER PASS  (every value HYPOTHESIS for F11 unless marked; formulas PROPOSED)
==============================================================================================================
| # | Budget / rule | Value or formula | Status / basis |
|---|---|---|---|
| BU1 | CONCURRENCY. One pass in flight per process; a trigger during a pass sets a rerun bit, never stacks (ladder sec 1.1). Triggers: unlock, surface open, the beat (the desktop's existing timer, D main.js:2045-2059 (constants), 2072-2191: default 20 s +/- 5 s; private and pull-only 300 s +/- 90 s; doubling to 900 s after consecutive failed scans), a resolving event (T2 G4) | -- | ladder, measured (desktop measure T-rows) |
| BU2 | REQUESTS, in this ORDER per pass: (1) opens owed by C03 AM-8 L2b; (2) pushes of owed controls and receipts for every session (a blocked contact's excepted, K11, Q6) (C04 Q09), served by a round-robin cursor over sessions resumed at the next pass where the last stopped, as BU6's (F6), then committed handshake replies B1/A2 (C03 AM-8 C6), then settles owed by C02 AM-7 / C03 EP8, then application flights round-robin across peers; (3) pulls: the inbox once, then at most SLOT_PULLS invite slots, rotated so every live slot is pulled within ceil(slots / SLOT_PULLS) passes; (4) one ACK per pulled mailbox, sent immediately after that mailbox's items are processed (BU12 (b), F1). Caps: OPEN_MAX opens and SETTLE_MAX settles per pass (F7); PUSH_MAX pushes per pass, PUSH_PEER application pushes per peer per pass; at most ONE pull per mailbox per pass (ladder A1) | SLOT_PULLS 4; PUSH_MAX 32; PUSH_PEER 4; OPEN_MAX and SETTLE_MAX HYPOTHESIS (C5-O1); REQ_PASS <= OPEN_MAX + SETTLE_MAX + PUSH_MAX + 2 x (1 + SLOT_PULLS) (F7) | HYPOTHESIS (F11); order PROPOSED |
| BU3 | BYTES. Pull: N = max(1, floor((B_PULL - 128) / (ceil(4 x mbb / 3) + 64))), N <= v2.pull_max_items; the response is read to at most CAP + 1 = N x (ceil(4 x mbb / 3) + 64) + 128 + 1 bytes before any parse (C03 C3). At mbb = 65,536: per-item 87,446, N = 11, CAP = 962,034 (DERIVED). Push: <= PUSH_MAX x 65,536 = 2,097,152 wire bytes per pass (DERIVED). mbb is bounded above by the relay's own source ceiling 1,048,576 (C03 E1); at that value N = 1 and CAP = 1,398,294 (DERIVED); the client refusing a larger advertised mbb needs a C03 C1 amendment (ESCALATION E9). RULED (E9, by F1): C03 AMENDMENTS AM-26 -- v2.max_body_bytes <= 1,048,576, else relay_v2_unsupported before any v2 call | B_PULL 1,048,576 | formula PROPOSED; B_PULL HYPOTHESIS |
| BU4 | TIME. Per request: connect 10 s; push, pull, invite calls 30 s; ack 15 s (C03 C4 values, kept; C3-O5 CLOSED here with those values). Per pass: T_PASS 60 s soft wall -- no request STARTS after it; items already fetched but unprocessed at T_PASS are HOLD. Between requests and between items the pass checks T_PASS and the lock generation (T6 LK3). TODAY: the shared client sets no deadline (transport M:2183-2200, C:1366-1383, B1) and a stalled relay holds any receive indefinitely | T_PASS 60 s | deadlines PROPOSED in C03; T_PASS HYPOTHESIS |
| BU5 | WORK PER ITEM. Class and cap before parse (D-R8); at most one ML-DSA-65 verification per pulled item (C02 step 5); items per pass <= sum of N over the pulls; no speculative fan-out (TODAY invite finish offers every handshake frame to every session-less contact, invite M:1670-1843, R63, bounded only by the contact count) | ITEMS_PASS = sum N | PROPOSED |
| BU6 | FAIRNESS AND CONTINUATION. Inbox items are processed in the order the relay serves them (one shared inbox, C3-O13). Send side: round-robin cursor over peers with pending application flights, PUSH_PEER each, resumed at the next pass where the last stopped (in memory; a restart restarts the rotation); queue admission share Q_P per peer (C04 Q08; value C4-O1). Slot pulls rotate (BU2). A pass that exhausts a budget sets the rerun bit, so continuation is the next pass, not an unbounded loop | -- | PROPOSED; closes C4-O10 in direction |
| BU7 | 429 AND BACKOFF (ladder sec 1.3, P4 fail loud). A 429 on a pull: that mailbox is skipped for a backoff window (exponential with jitter, base = the beat, ceiling 900 s), items untouched. A 429 on a push (ERR_OVERLOADED: the recipient's mailbox is full; ERR_RATE_LIMITED): the Flight stays PREPARED, retried in a later pass with the same op_id and bytes (I04; C04 Q01); ordinary pushes to that relay back off, owed-control pushes do not skip their turn. After BACKOFF_REPORT consecutive 429s the conversation notice N-BUSY (T4) is said once. A 429 never shows the no-relay notice (it is not unreachability). TODAY: a 429 on pull aborts the command (transport M:3596, B13) and C's replay loop ignores it (C:4156-4160) | BACKOFF_REPORT 3 | HYPOTHESIS |
| BU8 | EXACT RE-PUSH of a relay-accepted Flight with no receipt: once REPUSH_AFTER after its relay acceptance and then every REPUSH_AFTER, same op_id and bytes (the relay answers a known op from its push-receipt ring without re-enqueue, C03 T6 P3, so an earlier re-push is only a lookup). This is how a message whose relay copy expired while the recipient deferred it (T2 C2/C3) is delivered later. TODAY C re-pushes every pending flight on every receive (transport C:545, :4184-4193, B16) | REPUSH_AFTER = H_REC (259,200 s) | HYPOTHESIS (C5-O4) |
| BU9 | NO PREMATURE ACK OR ACCEPTANCE NDR1 (D-R1, D-R2); no budget exhaustion ever converts into an ACK |
| BU10 | THE LEASE LAW, re-derived (ladder sec 1.2 says the floor is re-derived after every fetched item is ack-or-dispose): on the successor every fetched item is ACK, ACK-DUP, DISPOSE, DEFER or HOLD by T1, so the beat may be shorter than v2.lease_secs; only DEFER and HOLD items pay the lease (D-R7) | -- | PROPOSED |
| BU11 | UNBOUNDED TODAY, CLOSED HERE (measured): no request deadline (B1, B27); pull body parsed with no cap and the item count never checked against max (transport M:3584-3590, B3, B4); receive --max and handshake poll --max unbounded (M main.rs:221 (receive max), :231 (poll_max_per_tick) -> transport/mod.rs:442, :516 (no clamp; :1768 clamps a different pull to 64), B5, B6; F9); one undecryptable frame aborts M's receive (M:1303, R29); one oversize or occupied item aborts P28's poll (R54, R55); C re-pushes every pending flight every receive (B16); un-ACKed refusals redelivered every lease until the relay TTL (C R37, R41, R42) | -- | measure_receive_budgets |
| BU12 | THE LEASE RELATION (F1; SR-15 X1). C03 EP4 deletes LEASED copies only and C03 V16 pins "ack an unleased id -> 200 acked 0; the item remains": an ACK that lands after the item's lease expired is lost, and the item is redelivered and re-verified every lease (for a malicious peer's invalid frames, one ML-DSA each every lease), the mailbox never drains toward its depth ceiling (qsl-server MAX_QUEUE_DEPTH_CEILING 257) and honest senders meet 429. Hence: (a) a C1-style service check (C03 AMENDMENTS AM-25): v2.lease_secs >= T_PASS + the ack deadline (BU4, 15 s) + LEASE_MARGIN, else relay_v2_unsupported before any v2 call; (b) the ACK set of a mailbox is flushed (its one EP4, D-R5) immediately after that mailbox's items are processed, and per-mailbox processing is bounded to v2.lease_secs / 2 -- items past it are HOLD; (c) vector V1125. Stated arithmetic: at the PROPOSED T_PASS 60 s the floor exceeds 75 s, above the v1 relay's shipped default pull lease of 60 s (qsl-server 5ea0f925 src/store.rs:7); F08's v2 lease default or F11's T_PASS must meet it (C5-O1) | LEASE_MARGIN: a stated margin, value HYPOTHESIS (C5-O1) | PROPOSED; values HYPOTHESIS |

==============================================================================================================
T4. DTO / STATUS MEANINGS  (I12 "GUI states follow durable facts"; D07 distinct states; DTO spellings PROPOSED)
==============================================================================================================
Facade surface F13 builds (THE PLAN sec 5 "typed engine APIs for submit, paginated conversation read, delivery snapshot
and bounded drive_step"; P-FACADE: the CLI gets thin verbs over the same calls). TODAY the facade has NONE of them at M,
C or PIN (engine measure S-rows; facade/mod.rs identical at M and C). Every DTO below carries the opaque contact id
(C01 T5; closes C01 T5 "facade/DTO carriage is C05") plus the local alias as a display string only; no DTO field ever
carries a network-supplied name (packet IMPLEMENTER_NOTES sec 6).
| Id | DTO / field (PROPOSED spelling) | Meaning | Backing durable fact (the only thing that may set it) | Never set by |
|---|---|---|---|---|
| M1 | MessageState "queued" | immutable local send intent saved; may wait for protocol capacity; the GUI draws NO tick (A5, Q1) | the encrypted queue row's write (the initial queue intent authority, THE PLAN sec 1; C04 Q01 QUEUED) | a probe; the composer |
| M2 | MessageState "prepared" | exact wire and liabilities committed; retries reuse the bytes; the GUI draws NO tick (A5, Q1) | the PREPARED paired commit (Flight in the vault; C04 R04, Q01) | -- |
| M3 | MessageState "relay_accepted" | the relay accepted those exact bytes; no peer receipt yet ("awaiting peer confirmation", THE PLAN sec 4); the GUI draws an OUTLINE tick, "on its way" (A5, Q1; the text "awaiting peer confirmation" is not required on the bubble) | relay 200 for the Flight's op_id, recorded in the SENT commit (C04 Q01). TODAY the marker accepted_by_relay fires on HTTP 200 BEFORE the local commit (transport M:4686-4691, C:3719-3724, S12) and a relay-REFUSED sealed flight is recorded as accepted (C04 E3, ENG-0357) | a 4xx, a 429, a timeout |
| M4 | MessageState "delivered" | an exact authenticated NDR1 for that Flight was durably applied (decision 1: "delivered to the other client"); the GUI draws a SOLID tick, "delivered" (A5, Q1). There is NEVER a read mark: no read receipt exists in the engine or the GUI (A5) | the K6 commit (C :723-750). TODAY at M no path sets queue Delivered (S5); util_receipt_apply sets timeline Delivered from CLI input under QSC_TEST_MODE (lib.rs M:2686-2712, S15) -- a shipping hook (I13) not carried | relay acceptance; a receipt that is not an exact MAC-valid NDR1 for that Flight (K6) |
| M5 | MessageState "not_sent" + reason | pre-seal terminal (C04 Q01: FAILED only from QUEUED): reasons relay_cannot_take (C04 Q11), contact_removed; the GUI draws a per-message failure mark in AMBER (accent; red stays reserved) with a retry action; the exact look is the operator's at F13 (Q4) | the pre-seal FAILED commit | anything after PREPARED |
| M6 | MessageState "closed_undelivered" | the session closed (C04 K07: explicit close, or device revoked after seal, C04 Q01 F2) with this message never receipted; drawn with M5's amber per-message failure mark and retry action (Q4) | the K07 close commit | a timeout; a relay error |
| C1 | ConversationNotice "relay_unreachable" (decision 6) | the relay cannot be reached AND this conversation holds >= 1 outgoing message in M1/M2 | the most recent pass's requests to the configured relay failed at connect / DNS / deadline (transport class), and the queue rows | a 401, a TLS refusal, a 429, a relay_v2_unsupported (each its own notice: C2-C5) |
| C2 | "relay_not_configured" / "relay_credential_rejected" / "relay_not_trusted" / "relay_unsupported" | distinct causes, said once (packet invariant "distinct errors for distinct causes"; the build's four relay failure messages) | no relay configured; 401 (C03 T4 per-endpoint credential); TLS trust failure; relay_v2_unsupported (C03 C1) | each other |
| C3 | "relay_busy" (N-BUSY) | the relay answered 429 BACKOFF_REPORT times in a row | T3 BU7 | unreachability |
| C4 | "queue_full" (N-FULL) | a new message cannot be queued: C04 Q03 msgqueue_queue_full / _bytes_full for this peer or globally; the composer keeps the typed text, nothing is queued, said once (Q7 (a)) | the refusal itself (nothing written) | -- |
| C5 | "history_full" (N-HIST) | a new message cannot be saved, or an arrived message is waiting (T2 C3), because the history quota is full | C04 R07 refusal; a DEFER-C of reason C3 | -- |
| C6 | "session_closed" | messaging with this contact ended (K07); closed_undelivered messages exist | the K07 commit | -- |
| C7 | "identity_changed" | a pin mismatch (C02 T2 step 6, T5): messaging blocked, never silently replaced | the refusal (no write) plus the contact's stored pin | a new first contact |
| C8 | "blocked" (Q6) | the user blocked this contact: shown as a "Blocked" notice in the conversation header; nothing is received into history or sent (K11) | the user's block commit | a relay error; anything the peer sends |
| T1 | ContactTrust "invited" / "connecting" / "unverified" / "verified" / "identity_changed" / "expired" / "revoked" / "blocked" | C02 T5 meanings (and AM-8 revoked; failed-expired spelled "expired"); "blocked" = the drawn blocked state (MOCKUP_contact_details) | invite record; C03 L1 operation commit; the session commit (unverified: possession proven, bearer-trusted, NOT verified, I02); the user's compare-and-mark commit bound to the current fingerprint (verified); the step-6 refusal; the horizon commit; the relay's ERR_INVITE_REVOKED for a known operation; the user's block commit | the "pinned" -> TRUSTED mapping (not carried, C02 E5); a relay probe; "Connected" never means "Verified" (THE PLAN sec 5) |
| L1 | ChatRow {contact_id, alias, last_activity_at, preview, preview_mine, unread, trust} | one row per contact with a conversation; sorted by last_activity_at, newest first (decision 5) | the latest committed timeline entry of that conversation; unread = an incoming entry newer than the stored last_read (a vault field; the packet keeps only column width outside the vault, decision 13) | a count: the DTO carries a BOOLEAN, so no count badge can be drawn (decision 5) |
| L2 | ConversationPage {entries: [{msg_id, direction, text, at, state (M1-M6 for outgoing)}], cursor} | paginated read of the committed history | committed timeline entries (bodies inside the vault, T6 HI1) | -- |
| N1 | Notify {kind: "new_message"} | at least one incoming message was committed since the last notify, from a contact not muted, while the vault is unlocked and the window is not focused; nothing while locked (Q2 (a)) | the K4 commit | sender, alias, text, count, contact id (decision 7) |
| X1 | Lock {generation} | the lock generation (T6 LK) | the desktop process state | -- |
| X2 | HistoryUsage {entries, bytes, quota_entries, quota_bytes} | the visible history quota (THE PLAN sec 4 "make history quota visible") | C04 R07 counters inside the owner | -- |
Mapping to the packet's DECISIONS.md (all fifteen, each classified):
| Decision | C05 rows | Status |
|---|---|---|
| 1 One tick = delivered to the other client; no in-flight or failed mark | RULED (A5, Q4): M1, M2 no tick; M3 an OUTLINE tick ("on its way"); M4 a SOLID tick ("delivered"); never a read mark; M5, M6 an amber per-message failure mark with a retry action | MATCHES for the tick's meaning (M4 is backed by the applied NDR1 only). ESCALATED E1: THE PLAN sec 4 names Queued / Prepared / Sent-awaiting confirmation / Delivered as "State shown to user" and says the UI says "awaiting peer confirmation". ESCALATED E4: M5, M6 (honest failure, C04 Q01, K07) have no drawn surface. RULED: AMENDED at the operator's word -- E1 RESOLVED by AMENDMENT A5 (the four states are engine and DTO meanings; A5 supersedes decision 1's "one tick"); E4 RESOLVED by Q4 (conversation-level notices remain for causes that are not one message's) |
| 2 Message preview ships ON (two lines) | L1 preview | MATCHES (the engine supplies the last committed message text; the UI clamps). Its bound at lock depends on decision 8 (E3) |
| 3 Column share, clamped | -- | NOT C05 (visual) |
| 4 Bubble share | -- | NOT C05 (visual) |
| 5 Unread is a dot, no bold, no count; sort by most recent | L1 unread (boolean), last_activity_at | MATCHES (weight is visual) |
| 6 No relay: queued, said once, amber, composer live | M1, C1 | MATCHES up to the queue admission bound; ESCALATED E5 beyond it (C04 Q03 refuses; the composer cannot "accept and hold" an 65th row). RULED (Q7 (a)): the composer keeps the typed text, nothing is queued, C4 queue_full is said once |
| 7 Notification says "New message" and nothing else | N1 | MATCHES for the content. ESCALATED E2: its premise ("fires when the app is closed or the vault is locked") cannot be delivered: locked means no pull (R is vault-held, C03 T4; ladder "Locked = no ticks"), closed means no process. RULED (Q2 (a)): "New message" only while unlocked and the window is not focused; nothing while locked; option (d), a read-only has-mail capability, recorded OPEN (C5-O19), not taken |
| 8 Auto-lock closes the main window (recorded as existing build behaviour) | X1, T6 LK | ESCALATED E3: MEASURED FALSE at D 92cba80a -- the single "main" window is kept and switched to the unlock surface (D:lib.rs:254-298, :586-591), only AFTER lock_now returns through the one gate held across network work (D:main.js:2016-2018, D:gateway.rs:116-129, D:commands.rs:380-384); contact rows are hidden, not cleared (D:main.js:114, show() toggles the class "hidden"; F9). RULED (Q3): on lock the main window is wiped of all information at once, not waiting on the engine (LK1-LK2 outside the gate), and a small unlock window is shown on top of it; the MAJOR stands until F13 builds it |
| 9 Darker outgoing fill | -- | NOT C05 (visual) |
| 10 Timestamps (today / Yesterday / weekday / date; locale 12/24 h) | L2 at, L1 last_activity_at | MATCHES: every time shown is the LOCAL clock time of a local durable fact (outgoing: the queue commit; incoming: the receive commit), carried as UTC seconds; formatting is the UI's. No sender-claimed time is shown (none is on the wire) |
| 11 Icon letter from the name; layout direction from the app's locale | L1 alias | MATCHES (alias local-only; derivation visual) |
| 12 Keyboard and focus | -- | NOT C05 (visual/interaction) |
| 13 Column width in a plaintext config, not the vault | -- | NOT C05; C05 stores NO DTO state outside the vault (last_read, mute and block live in the vault) |
| 14 What supersedes what | -- | NOT C05 (packet governance) |
| 15 Rows reorder under the cursor | L1 sort key | MATCHES (sort by last_activity_at; the reorder is accepted by the decision) |
Packet-internal note (settled by the packet's own rule, not escalated): the conversation mockup's caption "Two ticks
delivered, one tick sent." (MOCKUP_chat_conversation, section "My messages ...") contradicts decision 1; MANIFEST.md
"DECISIONS.md and this file ... where either contradicts the older documents, they win". Its internal decision numbers
(13, 18) are not DECISIONS.md's (1, 6).

==============================================================================================================
T5. ACTION IDS (IPC idempotency)
==============================================================================================================
TODAY (measured): no command of the 51 registered takes a request id (D:lib.rs:492-555; desktop measure I-rows);
na0700_ipc_replay.rs re-plays argument shapes, not duplicate or conflicting submits; invite_create mints a fresh id per
call, guarded only by UI latches (PIN invite/mod.rs:884-887; the latch is D:main.js:2535 inviteInFlight; PIN :916-931 is the
local commit-before-network insert; F9); rename is last-writer-wins; the engine mints every
msg_id itself (msgqueue M:560, C:579; engine A2), so a caller retry after a lost reply sends twice; `qsc send` can return Ok for
another contact's message (transport M:1965, C:1148, engine S9).
| # | Rule |
|---|---|
| AI1 | SCOPE. action_id = 16 bytes CSPRNG minted by the GUI once per user gesture, for the RECORD-CREATING actions only: submit_text and invite_create. Its scope is (vault, action kind). invite_redeem is keyed by the code's invite_id (C02 R2b known operation; C02 AM-4 conflicts), not by an action id |
| AI2 | BINDING. The engine stores action_id with H(canonical body) in the record it creates, in the same commit: the queue row's intent (submit_text; body = contact_id, text, padding choice) or the invite record (invite_create; body = label, expiry choice), and carries it to the timeline entry |
| AI3 | DUPLICATE, same body: returns the ORIGINAL result (msg_id or invite_id, and its current state); no second record, no second history unit, no second relay request |
| AI4 | CONFLICT, same action_id and a different body: action_id_conflict (NEW), refused before any write |
| AI5 | RETENTION. The engine remembers an action_id as long as its record or that record's timeline entry exists (bounded by the history quota, C04 R07); after retirement it is forgotten. The GUI uses an action_id only for retries of one IPC call within one process and one lock generation: never after a restart, never after it received that call's result, never after a lock (T6 LK). Hence "offline/restart: exactly one visible copy" (F13) holds: a crash after the commit leaves one record and no retry |
| AI6 | RETRY TRIGGER. The GUI retries only when the IPC call itself failed or timed out (no result), with the same action_id and the same body; an engine refusal is a result and is never retried automatically |
| AI7 | REDEEM. A second redeem of one code: the known operation is resumed (C02 R2b) and its state returned; with a DIFFERENT alias -> redeem_alias_conflict (NEW) before any network (today the alias is not compared, PIN invite/mod.rs:1236-1255) |
| AI8 | STATE-IDEMPOTENT ACTIONS need no action_id: rename (to the same name: no-op), verify (compare-and-mark bound to the CURRENT fingerprint; a different fingerprint -> identity_changed), delete, block / unblock, revoke, lock. Repeating one is a no-op returning the current state |
| AI9 | A call carrying a stale lock generation is refused (locked) before any effect (T6 LK6) |

==============================================================================================================
T6. TEXT LIMITS, HISTORY POLICY, UNVERIFIED-SEND POLICY, LOCK CANCELLATION
==============================================================================================================
| # | Rule |
|---|---|
| TX1 | TEXT. A message body is valid UTF-8 of 1..TEXT_MAX bytes, checked by the ENGINE at submit before any write (text_empty, text_too_long, text_invalid_utf8: NEW); the composer mirrors it (send dim until something is typed, packet IMPLEMENTER_NOTES sec 12). TEXT_MAX = 3,913 bytes DERIVED at C's default padding ceiling 4,096 (Padding::resolve, directional_delivery:287-298: 4,096 - 16 - 32 (msg id) - 135); the exact-60,000 bucket carries 59,817 (C04 S15, P4). Raising TEXT_MAX is a padding-and-queue-bytes decision (C04 C4-O1, MAXPACK): value HYPOTHESIS (C5-O5). TODAY no UTF-8 check exists on any send or receive path (engine T8) and M has no client body limit (engine T1) |
| TX2 | RECEIVED TEXT from an authenticated peer that is not valid UTF-8 or exceeds TEXT_MAX is valid PROTOCOL work: it is admitted and receipted (K4) and projected as a placeholder entry ("message could not be shown"), never DISPOSED (disposing would leave the sender's exact-retry obligation live forever) |
| TX3 | ALIAS / LABEL grammar and length stay C01 O5's. Measured drift for that cell: the packet draws a 40-character refusal ("Use 40 characters or fewer.", MOCKUP_contact_details G) while the build's UI caps 32 and the engine none (desktop measure T1-T5; the redeem name has no length cap at all, D:index.html:786) |
| HI1 | WHERE HISTORY LIVES. Conversation history = committed timeline entries WITH their message bodies, inside the successor vault, charged to C04 R07 (H_N entries, H_B bytes) and R01. TODAY the timeline holds metadata only (timeline:14-28, engine H1); bodies live in queue rows kept forever (engine H5) and in PLAINTEXT recv_*.bin files in a caller-chosen directory (engine H6; C01 CENSUS D24 OUTPUT). The GUI reads history only from the vault (L2); recv_*.bin stays a CLI output, never the GUI's history |
| HI2 | QUOTA VISIBLE: X2 HistoryUsage is part of the facade; where the GUI shows it is undrawn (T9) |
| HI3 | TRUTHFUL HISTORY-FULL. A submit when R07 cannot take one more unit -> history_full before any row, directory or temporary (C04 Q03, V708); an arriving K4 when R07 (or that peer's share H_P, C04 AMENDMENTS AM-1) is full -> DEFER-C reason C3 (no receipt; the sender sees no tick) with notice C5 said once. Never "recipient full" on the sender side (THE PLAN sec 4: without an authenticated remote reason the sender's UI says only what it knows) |
| HI4 | RETIREMENT. History is freed ONLY by a user-visible history retirement (C04 R07); a protocol retirement never frees it (THE PLAN sec 4). WHICH retirement exists is OPERATOR QUESTION Q5 (the packet's "deleting or editing a message" is OPEN and undrawn; "Disappearing messages" is a drawn, dimmed row with no decision). Until answered, F07/F13 build the quota and the truthful refusal, not a retirement. RULED (Q5): a user setting "keep messages for": 1 week / 1 month / 1 year / Forever, default 1 month; expired history is deleted automatically (a user-visible history retirement, C04 R07); an entry whose message is still undelivered (M1-M3) or closed_undelivered (M6) is never retired by expiry. PLUS a per-peer history share H_P (C04 AMENDMENTS AM-1, value HYPOTHESIS) so one contact cannot fill the global quota R07; the retention interplay is C04 AMENDMENTS AM-2. What "delete a conversation" does to still-pending messages is OPEN for F13 (the operator; C5-O20) |
| HI5 | ON LOCK, no history plaintext stays in the GUI (LK2); history never leaves the vault except to the unlocked GUI's L2 reads |
| UV1 | UNVERIFIED SEND: ALLOWED to a contact in trust "unverified" (possession proven by the handshake, C02 T5), behind a persistent "Not verified" mark -- the operator ruling recorded at contacts/mod.rs M:868-871 ("Messaging a pending contact is allowed (operator ruling) behind a persistent 'Not verified' badge") and C02 T5. The engine does not refuse and does not warn per message; the DTO carries trust (T1) on every chat row and conversation so the GUI cannot render unverified as verified. TODAY the badge is never drawn (D:main.js:3543 compares only "changed"; desktop D12, D13) |
| UV2 | Sending is REFUSED for trust identity_changed (C02 T5, I12), expired, revoked, blocked, invited and connecting (no session). tui.trust.mode's Strict arm (refuses verified-but-not-trusted sends, contacts M:772-774, engine U4) is a mode the GUI cannot see: its disposition is C01 O6 with P-FACADE; the successor has ONE policy (UV1) |
| UV3 | VERIFY = compare-and-mark bound to the current identity fingerprint (THE PLAN sec 5). A later identity change is identity_changed (C02), NOT a silent lapse to unverified: the contact-details mockup's Director default "the verified mark lapses if the code ever changes" is NOT deliverable as drawn under C02 (NOTE N2 in the drafting mission's REPORT.md, sha256 63af0b5c6da2d8663bdfaa4c9e8adfc8d831b4c7305b6ca47cf98d3ca4ff696a; it is a Director default awaiting the operator's keep-or-strike, not a DECISION). RULED (E7): UV3 ACCEPTED; C02 identity_changed governs |
| LK1 | GENERATION. The desktop process holds a lock generation G. Every lock trigger (auto-lock, File > Lock now, a destroy/erase ceremony) increments G SYNCHRONOUSLY, OUTSIDE and BEFORE the gateway queue: it is not an engine call and never waits behind one |
| LK2 | HIDE FIRST. On the increment the GUI removes all sensitive content (chat rows, previews, conversation, drafts, contact details: removed, not hidden) and shows the unlock surface, BEFORE the engine lock completes (THE PLAN sec 5 "Lock immediately hides sensitive UI"). Whether that surface is a closed main window plus a small unlock window (decision 8) is E3. RULED (Q3): on lock the main window is wiped of all information at once (not waiting on the engine: LK1-LK2 run outside the gate) and a small unlock window is shown on top of it |
| LK3 | ENGINE CHECKS. Every engine call carries G. drive_step and submit check it before each new secret operation (vault read, decrypt, seal), before each network request, and between items; on mismatch they stop and return Cancelled. Items fetched and not yet committed are HOLD (T1): un-ACKed, redelivered after the lease |
| LK4 | IN-FLIGHT NETWORK. A request in flight at the increment is abandoned (its result dropped). This is safe by construction: every network operation is committed first and retried later with the same op_id and the same bytes (I04; C03 C4), and a pull's leased items come back after the lease |
| LK5 | COMMIT UNDERWAY COMPLETES. A paired commit or queue-row write already started runs to completion under its lock (old-or-new complete, I03); none is interrupted mid-write; none is started after the check |
| LK6 | STALE RESULTS NEVER REPAINT. The GUI drops every result tagged with a generation != the current G: no repaint, no follow-up call, no new secret operation (I12). TODAY this guard exists only for invite mint, redeem and contacts refresh (D:main.js:2536-2540, 4111-4115, 3867-3899) and is ABSENT for the relay scan and the Server-pane test (D:main.js:3973-4090, 1913-1987) |
| LK7 | ENGINE LOCK. After the in-flight call yields, the engine lock runs: process passphrase cleared, sessions dropped (zeroize on drop), AND the process-lifetime caches of the queue and quarantine store keys cleared (TODAY they survive lock, msgqueue:295-342, quarantine:206, engine L2) and no engine read path proceeds for a keychain vault while locked (TODAY secret_get checks no flag, vault:275-283, engine L3). The gateway admits the lock ahead of queued non-lock calls; queued calls with a stale G return Cancelled without running |
| LK8 | WHILE LOCKED: no pull, no push, no notification (R, D and every session key are vault-held, C03 T4); the beat is off (ladder sec 2 "Locked = no ticks") |
| LK9 | LOCK LATENCY: sensitive content gone at LK2 (immediate); key material gone after at most one in-flight request deadline (BU4) plus one commit underway (LK5) (F8) -- HYPOTHESIS bound, measured at F13 (V1308) |
| LK10 | DRAFT TEXT in the composer is plaintext outside the vault: discarded at LK2, never persisted outside the vault (PROPOSED; the packet is silent: T9) |

==============================================================================================================
T7. CROSS-CONTRACT CELLS (census of every line of C01-C04 at main naming C05: 66 lines = 7 scope notes, 1 id collision
    (C02 T7 vector "C05"), 58 cell lines -> X01-X33 below; classification file census_C05_classified.txt, 0 unclassified)
==============================================================================================================
| # | Cell (contract:line at main) | Answer here | State |
|---|---|---|---|
| X01 | C01 T2 D (:249) "disposition -> C05" of a refused A1 | K1/K2 DISPOSE after full dispatch (ACK, no durable write); C01's "Relay source acks nothing on a refusal" is today's, replaced (D-R1) | CLOSED |
| X02 | C01 T2 F (:251) "relay item not ACKed and retained ... retained redelivery is I07 (C05/F11)" | unknown / retired / reserved-file / malformed kinds are K7-K11 DISPOSE (ACKed after full dispatch); only DEFER and HOLD are retained | CLOSED (changes the measured shape, C transport:520-522) |
| X03 | C01 T3 (:282) "ACK rules are C05" | T1 | CLOSED |
| X04 | C01 T5 (:316) "facade/DTO carriage is C05" of the contact id | T4 header: every DTO carries the opaque contact id; the alias is a display string | CLOSED (format stays C01 O5) |
| X05 | C01 AM-4 (:433) successor disposition of a refused A1 | as X01; a refused A1 writes no handshake.pending and is ACKed | CLOSED |
| X06 | C02 state names and spellings (:30, :109, :295, :313, :325, :490, :524, :562; OC14, E5, AM-8) | T4 T1 spellings; "pinned" -> TRUSTED not carried | CLOSED (spellings PROPOSED) |
| X07 | C02 T1g (:208) QSLH-1 "permanently invalid" | K7 DISPOSE | CLOSED |
| X08 | C02 T2 Disposition column (:215-216) | T1: "permanently invalid" -> DISPOSE; "duplicate of committed work" -> ACK-DUP; "known pending attempt: defer" -> DEFER-P; step 10 capacity -> DEFER-C | CLOSED |
| X09 | C02 step 10 (:230) "then send; ACK per C05" | D-R1: ACK after the step-10 commit, independent of the B1 push | CLOSED |
| X10 | C02 A2-6 (:256) | K3: ACK after the session commit; duplicate ACK-DUP | CLOSED |
| X11 | C02 RP7 (:271), X-b (:290), X-h (:295) | T2 P3: DEFER-P until W applies; the slot is then Redeemed and no longer pulled (C02 X-h, D-R6), so the item is left to the C03 slot sweep, un-ACKed and bounded by the sweep (F4); the crossing loser's superseded receipt is the one durable record (G1) | CLOSED |
| X12 | C02 T5 (:318) "C05 unverified-send policy" | T6 UV1-UV3 | CLOSED |
| X13 | C02 T6 (:336) pull-body cap "transport, C03/C05" | T3 BU3 (formula and budget) with C03 C3 | CLOSED; value HYPOTHESIS (C5-O1) |
| X14 | C02 T7 (:349) "no ACK beyond its C05 disposition" | every refusal vector asserts the T1 disposition (T8 header) | CLOSED |
| X15 | C02 OC6 (:482) | T1, T2, T4, T6 UV | CLOSED |
| X16 | C02 AM-3 FX6 (:557) DEFER mechanics | T2 P4 | CLOSED |
| X17 | C02 AM-10 (:581) and C04 E6 (:359) DEFER mechanics | T2 C1 | CLOSED |
| X18 | C02 AM-12 (:583) "disposed per C05" | K1 DISPOSE | CLOSED |
| X19 | C03 T1 row 10 (:89), AM-17 (:545), C04 X03 (:240), E1 (:354): the queue row C04/C05 "own" | CLOSED by C03 AM-17 (three op_id holders); C05 adds the action_id and body digest to the row's intent (AI2) | CLOSED |
| X20 | C03 C5 (:165), T9 (:331), C3-O5 (:464) per-pull byte budget and deadline values | BU3 (formula, B_PULL HYPOTHESIS), BU4 (C03 C4 values kept) | CLOSED; values HYPOTHESIS |
| X21 | C03 T6 revoke (:267), C3-O6 (:465), AM-2 (:513) slot-item dispositions | K1: revoked-before-answer DISPOSE; A1 at or after RU_I DISPOSE; the AM-22 clamp is not a refusal | CLOSED |
| X22 | C03 T7 (:288), C3-O4 (:463) receiver tolerance of a duplicate after push-receipt eviction | K4 ACK-DUP while the Disposition lives, DISPOSE once retired (CLOSED_REPLAY); K6 DISPOSE; no effect is duplicated; BU8 relies on it | CLOSED |
| X23 | C03 T9 (:333) OC5/OC6/OC14 "not C03's" | X06, X15 | CLOSED |
| X24 | C03 C3-O13 (:472) one mailbox per (identity, relay) vs per peer | today's shape kept for this draft; the cost stated (T2 G7) | OPEN (C5-O2, Director) |
| X25 | C03 AM-8 C6 (:519) "the disposition vocabulary is C05's" | T1 vocabulary; the 404/429 retries of committed replies are BU2 order and BU7 | CLOSED |
| X26 | C03 AM-9 (:520) injected frames "bounded by T3 C3 and C05's budgets" | T3 BU3-BU5, D-R3 | CLOSED |
| X27 | C04 F2 (:23), Q01 (:171), V709 (:306) device revoked after seal -> K07 "with C05's user meaning" | M6 closed_undelivered + C6 session_closed; its drawn surface is E4 / Q4 | CLOSED IN MEANING; surface RULED (Q4: the amber per-message failure mark with a retry action; the look the operator's at F13) |
| X28 | C04 R07 (:95), C4-O3 (:323) history retirement policy | T6 HI1-HI5; the policy is OPERATOR QUESTION Q5 | RULED (Q5): the "keep messages for" setting and the per-peer share H_P (C04 AMENDMENTS AM-1, AM-2); H_N, H_B, H_P values OPEN (C5-O7); delete-a-conversation OPEN (C5-O20) |
| X29 | C04 Q08 (:178), C4-O10 (:330) per-peer drain and fairness budgets | BU2, BU6 | CLOSED IN DIRECTION; values C5-O1 |
| X30 | C04 X11 (:248) | T1-T4, T6 | CLOSED except X24, X32 (X28 RULED by Q5) |
| X31 | C04 C4-O4 (:324) session close "C05 for its user meaning" | M6, C6; the transition stays F06/F16 | CLOSED IN MEANING |
| X32 | C04 C4-O9 (:329) remote mailbox depth shared by ordinary and control frames | T2 G7, BU7 (a push 429 leaves the Flight PREPARED; controls keep their turn) | OPEN with X24 |
| X33 | C04 E6 (:359) | = X17 | CLOSED |
Cells that implicate C05 without naming it, answered too: C04 Q03 "The refusal is visible to the user" -> C4
queue_full, C5 history_full; C04 Q11 "a visible reason" -> M5 relay_cannot_take; C04 K04 "a remote QueueFull leaves the
committed response pending" -> D-R1 (the item is ACKed; the response stays owed) and BU7.

==============================================================================================================
T8. VECTORS SPECIFIED (NOT GENERATED; NOTHING RUN)
==============================================================================================================
BASE (SR-19(a)): as C04 T8. A symbol existing at C has base = the F03 integration head that carries it (RED there where
the expected behaviour changes); a symbol existing only at M, base M 4e9dfd0e; a desktop symbol, base D 92cba80a; a symbol
that exists nowhere yet, base = the F-card commit introducing it with today's behaviour. Every vector also asserts its T1
disposition for every item it pulls (ACK set compared by id), no durable write on a refusal, and no NDR1 before commit.
Symbols (existing at C unless marked): RPR transport::receive_pull_rounds (:453); FPA transport::flush_pending_acks (:663);
RPI transport::relay_inbox_pull_mode_inner (:2722); DRU protocol_state::directional_receive_update (:1308); ENA
DirectionalUpdateError::expected_non_admission (:1272); RCV Transaction::receive_inner (directional_delivery:722); DRP
transport::directional_replay (:4184); RHC transport::relay_http_client (M :2183); HSP handshake::handshake_poll (M
:2835); LCK vault::protection::lock (:255); RMM RelayMessageSender::commit (:3748); D-GW CoreGateway::call (D
gateway.rs:116); D-LK commands::lock_now (D commands.rs:381); D-RS relayScan (D main.js:4020). NEW: DSP dispatcher
drive_step; DSO dispatcher dispose path; DFT the DEFER table; BUD the pass budgets; SUB facade submit_text; ACT action-id
binding; GEN the desktop lock generation; CNV the chat-list / conversation / delivery DTO builders; NTF the notify builder.
| Id | Card | Rule | Discriminating case | Expected | Delta | Base |
|---|---|---|---|---|---|---|
| V1101 | F11 | T1, D-R6 | ONE inbox pull returning a B1 for a live attempt, an A2 for a live candidate, an NDE1 text, an NDE1 maintenance control, an NDR1 for an outstanding Flight, a QSE 01 00 frame and an Unknown item | each per T1 (ACK, ACK, ACK, ACK, ACK, DISPOSE, DISPOSE); one EP4 carrying all seven ids; no batch abort; the handshake frames are not "known foreign" | DSP, RPR | F03 head (RED: C lets known-foreign fall through un-ACKed, R39, and leaves non-admissions, R37) |
| V1102 | F11 | D-R3, D-R4 | a batch of N invalid items (unknown class, bad-MAC NDE1, retired QSLH-1, 12,289-byte envelope) ahead of one valid text in the next pass | pass 1 ACKs all N (DISPOSE), vault / queue / timeline bytes identical; pass 2 admits the text without waiting a lease | DSO, ENA | F03 head (RED: R37 un-ACKed, lease blocks) |
| V1103 | F11 | K4 P1 | responder candidate pending; 3 x N text frames of the new session arrive before its A2 (the A2 behind more than one batch) | every text DEFER-P: un-ACKed, no NDR1, no state; A2 applied -> the redelivered texts admitted in order, each receipted once; nothing DISPOSED | DFT, DRU | F11 commit introducing DFT |
| V1104 | F11 | K4 C2/C3, BU8 | receiver ordinary admission full (64 unprojected events) -> sender's text; then capacity freed; arm: history quota full; arm (TIME): the relay copy expires while deferred | DEFER-C: un-ACKed, no NDR1, sender shows relay_accepted, no tick; freed -> next redelivery admitted, ACK, NDR1, sender M4 exactly once; history arm: notice history_full once at the receiver, never "recipient full" at the sender; expiry arm: the sender's exact re-push after REPUSH_AFTER is admitted once | DFT, BUD, DRP | F11 commit |
| V1105 | F11 | G5 | receiver ordinary admission full AND its queue P full; the peer sends a standalone maintenance closure and NDR1 receipts; this side owes receipts | the closure and receipts admitted and ACKed (never DEFER-C); this side's owed receipts pushed first in the pass | DSP, DRP | F03 head (with C04 V707) |
| V1106 | F11 | D-R5 | (a) text committed, EP4 lost; (b) NDR1 applied, EP4 lost | after the lease: (a) ACK-DUP, saved receipt re-sent, no second event or timeline entry; (b) DISPOSE (ACKed), no second DELIVERED, no error | DSP, FPA | F03 head (RED for (b): RECEIPT_NOT_OUTSTANDING left un-ACKed every lease, R41) |
| V1107 | F11 | BU7 | pull answered 429; push answered 429 ERR_OVERLOADED three passes running | the mailbox backed off, other mailboxes still pulled, no abort; the Flight stays PREPARED, re-pushed with the same op_id and bytes; notice relay_busy once; never relay_unreachable | BUD | F11 commit (RED at M and C: a pull 429 aborts, B13) |
| V1108 | F11 | D-R1 | an A1 committed at step 10 whose B1 push is answered 429; the same for an NDE1 whose NDR1 push fails | both items ACKed on commit; B1 / NDR1 re-pushed from their committed holders on later passes | DSP | F03 head (RED: C keeps the item un-ACKed until the reply push succeeds, R35) |
| V1109 | F11 | K10, D-R8 | an envelope-class item of 12,289 B among valid items | DISPOSE on length before any parse or copy; the rest processed | DSO | F11 commit (contrast P28 abort, R54) |
| V1110 | F11 | FAULT | vault write fails (fixture) on the second item of five | item 2 FAULT (un-ACKed); items 3-5 HOLD; item 1's ACK flushed; the fault reported as LOCAL, not malformed, not relay | DSP | F11 commit |
| V1111 | F11 | BU3, BU4 | mbb 65,536; a pull response of CAP + 1 bytes; a relay that accepts and never answers | N = 11 requested; RESP-REFUSED before parse, nothing ACKed; the silent relay ends the request at its deadline, the pass continues with other mailboxes | BUD, RHC | F03 head (RED: no deadline, B1; no cap, B3) |
| V1112 | F06/F11 | G5 | two peers; A's ordinary admission full of Dispositions awaiting B's closures; B's closures ride only ordinary frames A defers | B's standalone maintenance closure admitted; A frees capacity; no cycle -- or F06 fails and the design returns | DFT, NXC (C04) | F06 commit |
| V1113 | F11 | BU6, G7 | peer X floods 200 texts; peer Y has 1 control and 1 text queued behind them | Y's control admitted in the first pass that fetches it; Y's text within G7's bound; send side X and Y each <= PUSH_PEER per pass, round-robin | BUD | F11 commit |
| V1114 | F11 | BU2, D-R6 | 9 live invite slots, SLOT_PULLS 4 | every slot pulled within 3 passes; the request log shows at most one pull per mailbox per pass | DSP | F03 head (RED at M: per-command pulls of one mailbox, B26) |
| V1115 | F11 | K1 (C3-O6) | inviter revokes after the relay accepted the A1 (EP9 delivered = true), then pulls | the A1 DISPOSED: no candidate, no contact row, no B1 | DSP | F11 commit |
| V1116 | F11 | K1 P3/P4, AM-2 a | an A1 at RU_I; a crossing loser's A1 at the lower side; an A1 hitting handshake_lifecycle_occupied | DISPOSE; DEFER-P, and once W applies the slot is not pulled again and the item is removed by the C03 slot sweep (F4); DEFER-P then processed at the g1 horizon (C03 AM-5 CL13) | DSP, LS (C02) | F10/F11 commit |
| V1117 | F11 | K11 | a blocked contact's text, receipt and control frames; this side owes that session receipts | each DISPOSED (ACKed) before any body decrypt: no Disposition, no history entry, no NDR1; nothing pushed to that contact; the conversation header notice "blocked" (C8) (Q6) | DSO | F11 commit |
| V1118 | F11 | D-R3 | 100 invalid items | debug log: at most one event per (kind, code) per pass, carrying kind and code only; no id, SID, alias or length | DSO | F11 commit |
| V1119 | F11 | D-R7 | one pass yielding 2 DEFER-P, 1 DEFER-C (history quota full) and 3 HOLD items | the Diagnostics surface shows the counts 2 / 1 / 3 (counts only: no id, SID or alias); the DEFER-C cause said once in its conversation (C5); nothing ACKed for those six | DSP | F11 commit |
| V1120 | F11 | G1 | DEFER_MAX + 1 distinct deferred items | the least-recently-seen entry evicted; on its redelivery the item is re-verified in full and gets the same disposition; no ACK, no durable write either way | DFT | F11 commit |
| V1121 | F11 | G2 | a deferred item redelivered with its reason unchanged and no resolving event; the same run with the fast path disabled (fixture) | identical disposition and identical ACK set on both runs; no durable write | DFT | F11 commit |
| V1122 | F11 | BU1 | five triggers (beat, unlock, resolving event) during one pass | exactly one further pass runs after it; never two passes in flight | DSP | F11 commit |
| V1123 | F11 | BU5 | one handshake item and 10 session-less contacts | exactly one ML-DSA-65 verification for the item; the item offered to no other contact | DSP | F11 commit (contrast the fan-out at M invite:1670-1843, R63) |
| V1124 | F11 | BU10 | a beat shorter than v2.lease_secs; a pass whose items are all ACK / ACK-DUP / DISPOSE; then a pass with DEFER and HOLD items | the next beat's pull is not delayed by any lease after the first pass; after the second only the DEFER and HOLD items are invisible for v2.lease_secs | BUD | F11 commit |
| V1125 | F11 | BU12 | lease_secs 5 s, one pull of 10 items processed in 6 s (fixture: the C1 check bypassed); arm: the C1 check in force | all 10 ACKed (the flush precedes the lease) -- the read's words; with BU12 (b)'s bound, items not processed within lease_secs / 2 are HOLD and every ACK sent lands inside its item's lease (acked == ids sent); arm: refused relay_v2_unsupported before any v2 call | BUD | F11 commit |
| V1126 | F11 | T1a | a frame 17 ahead in the current epoch, then the 16 missing frames | DEFER-P (P6), then all admitted in order, one receipt each; nothing DISPOSED | DFT, DRU | F11 commit |
| V1301 | F13 | AI3 | submit_text(a, c, "hi") twice | one queue row, one timeline entry, the same msg_id | SUB, ACT | F13 commit (RED at M/PIN: a fresh msg_id per call, engine A2) |
| V1302 | F13 | AI4 | submit_text(a, c, "hi") then (a, c, "bye") | action_id_conflict; no write | ACT | F13 commit |
| V1303 | F13 | AI5 | kill the process after the submit commit, before the IPC result; restart | exactly one message in L2; the GUI does not retry | SUB | F13 commit |
| V1304 | F13 | M1-M4 | walk one message through queued, prepared, relay_accepted, delivered; a relay 200 without receipt; an NDR1 valid for another Flight; a relay-refused sealed Flight | each state only with its backing fact; no delivered without the exact NDR1; the refused sealed Flight stays prepared; no accepted_by_relay before the SENT commit | CNV, RMM | F03 head (RED: marker before commit, engine S12; C04 E3 / ENG-0357) |
| V1305 | F13 | decision 1, 10 | the peer offline 2 days (TIME), then online | the outline tick at relay acceptance (M3) and the solid tick only when the NDR1 is applied (M4); never a read mark (A5); the stamp is the local queue time | CNV | F13 commit |
| V1306 | F13 | C1, C2 | relay down, 3 submits; then a 401; then the relay back | 3 queued, notice relay_unreachable once in that conversation and in no conversation without waiting messages; 401 -> relay_credential_rejected (not unreachable); back -> notice gone, relay_accepted then delivered | CNV | F13 commit (RED at M: 403/4xx shown as "will send when reachable", engine S8) |
| V1307 | F13 | C4 | Q_P rows queued for one peer, then a submit | queue_full before any write; notice once; composer text kept (Q7 (a): kept, not queued, said once) | SUB | F13 commit |
| V1308 | F13 | LK1-LK4, LK9 | a pull that never answers; auto-lock fires | sensitive DOM removed at the increment, before lock_now returns (the main window wiped and the small unlock window shown, Q3); the in-flight pull abandoned; a late result dropped (no repaint, no call); key material gone within one request deadline (BU4) plus one commit (LK9, F8); after unlock the items return after the lease and are processed once | GEN, D-GW | D 92cba80a (RED: lock waits behind the gate, desktop L11; rows hidden, not cleared, desktop L3-L4) |
| V1309 | F13 | LK5 | lock during a paired commit (non-shipping pause seam, I13) | the commit completes, old-or-new state; no operation starts after it | GEN, DSP | F13 commit |
| V1310 | F13 | LK6 | a relay scan in flight at lock | no "connected" repaint; no failure-count change | GEN, D-RS | D 92cba80a (RED: no generation on the scan, desktop L8) |
| V1311 | F13 | N1, LK8 | an incoming message while unlocked with the window unfocused; one while locked | one notification "New message", no sender, no text; none while locked | NTF | F13 commit |
| V1312 | F13 | L1 | an incoming entry; open the conversation; restart | unread true, then false, false after restart (last_read in the vault); the DTO has no count field | CNV | F13 commit |
| V1313 | F13 | UV1-UV3 | send to an unverified contact; to an identity_changed contact; verify against a changed fingerprint | allowed with trust unverified on the row; refused before any write; identity_changed | SUB, CNV | F13 commit |
| V1314 | F13 | HI3 | history at quota: a submit; an arriving text | history_full before any row; the arrival DEFER-C with notice once; the sender never sees "recipient full" | SUB, DFT | F13 commit |
| V1315 | F13 | TX1, TX2 | empty; TEXT_MAX + 1; invalid UTF-8; the peer sends invalid UTF-8 (fixture) | text_empty / text_too_long / text_invalid_utf8 before any write; the peer's frame admitted, receipted, shown as a placeholder | SUB | F13 commit (RED: no UTF-8 check, engine T8) |
| V1316 | F13 | AI7 | redeem one code twice with two aliases | the second -> redeem_alias_conflict before any network | SUB | F13 commit (RED at PIN: alias not compared) |
| V1317 | F13 | LK7 | lock, then inspect the queue store-key cache; a keychain vault engine read while locked | cache empty; the read refused | LCK | M 4e9dfd0e (RED: caches survive lock, engine L2; secret_get checks no flag, engine L3) |
| V1318 | F13 | AI6 | a submit_text whose IPC call times out with no result (fixture); a submit_text the engine refuses (queue_full) | the first retried once with the same action_id and body and answered with the original result (AI3); the refusal is a result and is not retried | ACT | F13 commit |
| V1319 | F13 | AI8 | rename to the same name; block twice; unblock twice; verify twice at the same fingerprint; lock twice | each repeat a no-op returning the current state; no second write | ACT | F13 commit |
| V1320 | F13 | AI9 | a submit_text carrying a generation from before a lock and unlock | refused (locked) before any effect; no row, no history unit | GEN, ACT | F13 commit |
| V1321 | F13 | LK8 | lock; ten beat intervals pass; the peer pushes | no pull, no push and no notification while locked (the request log is empty); after unlock the next pass pulls | GEN, NTF | F13 commit |
| V1322 | F13 | LK10 | text typed in the composer; lock; unlock | the composer is empty; the text is in no store outside the vault | GEN | F13 commit |
| V1323 | F13 | HI1 | a message received while a CLI recv_*.bin output directory exists; the file deleted; the conversation opened | L2 shows the message from the vault; the GUI reads no recv_*.bin | CNV | F13 commit |
| V1324 | F13 | M5, M6, C6 | a contact removed with a QUEUED message; a session closed (K07) with a relay_accepted, unreceipted message | not_sent with reason contact_removed; closed_undelivered with the conversation notice session_closed; both carry the amber per-message failure mark with a retry action (Q4) | CNV | F13 commit |
| V1325 | F13 | X2 | three entries committed, then one retired | HistoryUsage entries and bytes equal the C04 R07 counters before and after; the quota fields present | CNV | F13 commit |
| V1326 | F13 | T4 T1 | a contact walked through invited, connecting, expired and revoked; a blocked contact | each trust value present only after its backing commit (invite record; C03 L1 operation commit; horizon commit; ERR_INVITE_REVOKED for a known operation; block commit); never set by a probe | CNV | F13 commit |

==============================================================================================================
T9. OPEN CELLS (each with the cards it blocks) AND THE PACKET'S OPEN_QUESTIONS TOUCHED
==============================================================================================================
| Id | Question | Depends on | Blocks |
|---|---|---|---|
| C5-O1 | Values SLOT_PULLS, PUSH_MAX, PUSH_PEER, OPEN_MAX and SETTLE_MAX (F7), B_PULL, T_PASS, LEASE_MARGIN (F1: with T_PASS it must meet BU12 (a) against the relay's advertised lease), DEFER_MAX, BACKOFF_REPORT, the backoff ceiling | F11 measurement | F11 |
| C5-O2 | C3-O13: one inbox per (identity, relay) vs one per peer (isolation of deferred backlogs and depth denial, per-peer revocation of D for Block) | Director | F08, F11 |
| C5-O3 | Cycle freedom when piggybacked closures ride deferred ordinary frames (G5; joins C4-O2) | F06 | F06, F11 |
| C5-O4 | REPUSH_AFTER, and whether server-info should advertise the relay's retention TTL (a C03 EP10 field: a C03 amendment) | F08/F11 | F11 |
| C5-O5 | TEXT_MAX (3,913 at the default padding ceiling; up to 59,817) with C4-O1 queue bytes | F07/F13 | F13 |
| C5-O6 | History retirement policy (OPERATOR QUESTION Q5). CLOSED: RULED (Q5), T6 HI4 | operator: RULED | F07, F13 |
| C5-O7 | H_N, H_B (C4-O3's values) and the per-peer share H_P (C04 AMENDMENTS AM-1) | F07 | F07, F13 |
| C5-O8 | Block: is an NDR1 still sent for a blocked contact's frames (OPERATOR QUESTION Q6). CLOSED: RULED (Q6), no receipt; T1 K11 | operator: RULED | F11, F13 |
| C5-O9 | Notification scope (E2 / Q2). CLOSED: RULED (Q2 (a)), T4 N1; option (d) is C5-O19 | operator: RULED | F13 |
| C5-O10 | Surfaces for M5, M6, C4, C6 (E4 / Q4). RULED (Q4): the amber per-message failure mark with a retry action for M5, M6; conversation-level notices for C4, C6; the exact look is the operator's at F13 | operator: RULED (the look at F13) | F13 |
| C5-O11 | Decision 8's window behaviour (E3 / Q3). CLOSED: RULED (Q3), T6 LK2 | operator: RULED | F13 |
| C5-O12 | THE PLAN's user-visible delivery states vs decision 1 (E1 / Q1). CLOSED: RULED by AMENDMENT A5 (approved by the operator in substance) | operator: RULED | F13 |
| C5-O13 | The composer at the queue bound (E5 / Q7). CLOSED: RULED (Q7 (a)), T4 C4 | operator: RULED | F13 |
| C5-O14 | A client-side ceiling on mbb (E9; a C03 C1 amendment). CLOSED: RULED (E9, by F1), C03 AMENDMENTS AM-26 | Director: RULED | F08 (client), F11 |
| C5-O15 | GUI placement of X2 HistoryUsage, C3 relay_busy, C4 queue_full; draft discard at lock (LK10); the meaning of the drawn, dimmed "Mute notifications" row (N1 reads it as "no notify for that contact") | packet re-cut / F13 | F13 |
| C5-O16 | Registration of the NEW codes (text_*, action_id_conflict, redeem_alias_conflict, the notice spellings) through DOC-SCL-002 | implementing PRs (C01 O9) | F11, F13 |
| C5-O17 | Whether C05's acceptance needs the operator's approval (ROADMAP item 3; E6 / Q8). CLOSED: RULED (Q8): the operator gives final approval of how the app looks and functions (C05 now; F13 acceptance later) | operator: RULED | F11 |
| C5-O18 | The lock latency bound (LK9) | F13 measurement | F13 |
| C5-O19 | Q2 option (d): a READ-ONLY "has-mail" capability (a fourth v2 object derived from R that can neither pull nor ack; a C03 T1 amendment) so that a locked app could say "New message" -- recorded OPEN, NOT taken | operator / Director | none (F13 builds Q2 (a)) |
| C5-O20 | What "delete a conversation" does to still-pending messages (queue rows and Flights with a live exact-retry obligation, C04 Q06; K07) (Q5) | operator at F13 | F13 |
Packet OPEN_QUESTIONS.md, every entry classified (none resolved here):
| Entry | Touched by C05? | Status |
|---|---|---|
| tight, tint, even, smaller, tall | no (visual) | OPEN, untouched |
| unv (a stronger unverified mark in New Chat) | yes: UV1 puts trust on every row; the mark is visual | OPEN |
| undrawn: attachments | no (C06) | OPEN |
| undrawn: search inside a conversation | no (L2 is pagination, not search) | OPEN |
| undrawn: deleting or editing a message | YES: history retirement (HI4) needs a policy | OPEN; escalated as Q5; Q5 RULED the retention setting; deleting a conversation's pending messages OPEN (C5-O20) |
| undrawn: group conversations | no (C05 is one-to-one throughout) | OPEN |
| undrawn: what the Chats "+" opens | no | OPEN |
| undrawn: the small unlock window | yes: LK2 / E3 | OPEN (layout); Q3 RULED that it is shown on top of the wiped main window |
| decouple (relay page saves only when the test passes) | yes, indirectly: C1-C2 notices are reports; the dispatcher gates on configuration plus an unlocked vault, never on a probe result | OPEN, untouched in substance |
| neutral, sent link, placeholder, when, readable, diagnostics-filter note | no | OPEN |
| intro, match, narrow, bar, bank details, bank contacts, bank v13, undim, undo | no | OPEN |
| owed screenshot of Settings > Security | no | OPEN |
| nobody has drawn: the messaging view | yes: T4 gives it meanings, not drawings | OPEN (drawing) |
| nobody has drawn: Appearance and Notifications | yes: N1 and E2 | OPEN |
| nobody has drawn: fingerprint mockups 07, 07b, 09 stale | no | OPEN |
SR-18 CENSUS LINE: F11 and F13 remap observables and each owes the census at drafting: the receive markers
(recv_frame_skipped, ack_legacy_complete, ack_failed, dedup_store_reset), the ACK precondition (C's reply-success gate),
the delivery marker accepted_by_relay and QSC_DELIVERY states, msgqueue_not_sent codes and the honest_line "will send when
the relay is reachable" (msgqueue M:1098-1109), qsc send's success rule (transport M:1965), the facade ContactState
"Pinned" (facade:516-537), and the desktop's status words (D main.js:758-786, 2279-2306, 3537-3548).

==============================================================================================================
ESCALATIONS RAISED WITH THE DRAFT (named; each RULED by RULING_NA0783_C05_ACCEPT_2026-09-24, clause appended)
==============================================================================================================
| Id | Conflict | Between | Severity | Draft's position (a proposal only) |
|---|---|---|---|---|
| E1 | THE PLAN sec 4 lists Queued / Prepared / Sent-awaiting confirmation / Delivered under "State shown to user" and says "the UI says 'awaiting peer confirmation'"; decision 1 draws ONE mark (delivered) and deliberately no in-flight mark | THE PLAN / packet decision 1 | MEDIUM | engine and DTO keep the four states (D07 holds); the GUI draws per decision 1; whether THE PLAN's "shown to user" is satisfied by the DTO alone is the operator's (Q1). RULED: RESOLVED by AMENDMENT A5 (Q1) -- the engine keeps the four states; the GUI draws Queued / Prepared with no tick, an OUTLINE tick "on its way" (relay accepted), a SOLID tick "delivered"; never a read mark |
| E2 | Decision 7's premise: a notification "fires when the app is closed or the vault is locked". Locked means no pull (R and every session key are vault-held, C03 T4; ladder sec 2 "Locked = no ticks"; I12); closed means no process. The content rule is deliverable, the timing is not | packet decision 7 / C03 T4, THE PLAN I12, ladder | MEDIUM | notifications only while unlocked (window unfocused or minimised); any locked-state arrival signal needs vault-external credential material, the ladder's open M3 question (Q2). RULED (Q2 (a)): "New message" only while unlocked and the window is not focused; nothing while locked; option (d) recorded OPEN (C5-O19), not taken |
| E3 | Decision 8 records "Auto-lock closes the main window and shows a small unlock window" as EXISTING build behaviour. MEASURED FALSE at D 92cba80a: one "main" window, switched to the unlock surface (D:lib.rs:254-298, :586-591) only AFTER lock_now returns through the single gate held across network work (D:main.js:2016-2018; D:gateway.rs:116-129; D:commands.rs:380-384); contact rows hidden, not cleared (D:main.js:114, show() toggles the class "hidden"; F9); no request deadline exists (transport M:2183-2200). So THE PLAN sec 5 "Lock immediately hides sensitive UI" and decision 8's bound on decision 2's exposure do not hold today | packet decision 8 (a premise) / the tree / THE PLAN sec 5, I12 | MAJOR (plaintext previews can stay on screen for as long as a stalled relay call runs) | T6 LK1-LK2: hide first, outside the gate; whether the successor also CLOSES the window (decision 8's form) or clears it in place is the operator's (Q3); either form meets LK2. RULED (Q3): wipe the main window of all information at once (not waiting on the engine: LK1-LK2 outside the gate) and show a small unlock window on top of it; the MAJOR stands until F13 builds it |
| E4 | Honest failure has no drawn surface: C04 Q01/Q11 (pre-seal not_sent with a visible reason), K07 (closed_undelivered), identity_changed and queue_full need a user-visible statement; decision 1 deliberately draws no failed mark | C04, THE PLAN sec 4 (truthful statuses) / decision 1 | MEDIUM | conversation-level notices said once (T4 C4-C7), the decision-6 pattern ("distinct errors for distinct causes, said once rather than per row"); needs the operator's word (Q4). RULED (Q4): a per-message failure mark in AMBER (accent; red stays reserved) with a retry action; the exact look is the operator's at F13; conversation-level notices remain for causes that are not one message's |
| E5 | Decision 6 "the message is accepted and held; the composer stays live" vs C04 Q03's global/per-peer queue admission bound (Q_S, Q_P) and R07's history bound: beyond them the engine must refuse before writing | decision 6 / C04 Q03, R07 | LOW-MEDIUM | the composer keeps the typed text (nothing lost) and the conversation says queue_full / history_full once (Q7). RULED (Q7 (a)): keep the typed text, do not queue, say so once |
| E6 | ROADMAP item 3 ("the lean background design ... The operator approves that design"), mapped to C05/F11 by RULING_PLAN_F00 K-01, vs the F01 acceptance path (the Director's ruling after the SR-15 read) | ROADMAP (subject mapped, not dropped) / THE PLAN F01 | LOW (governance) | the Director decides whether C05's acceptance carries an operator approval (Q8). RULED (Q8): the operator gives final approval of how the app looks and functions (C05 now; F13 acceptance later) |
| E7 | The contact-details mockup's Director default "the verified mark lapses if the code ever changes" (a default awaiting keep-or-strike) presumes a contact continues, unverified, under a changed code; C02 T5 / I12 make a changed identity identity_changed, messaging blocked, never silently replaced | packet default / C02 T5, I12 | LOW | UV3 follows C02; the default, if kept, needs a C02 amendment first. RULED (E7): UV3 ACCEPTED (C02 identity_changed governs) |
| E8 | Candidate receive-path defects measured by source reading (NOT RUN), the class of C04 E3: C leaves EVERY expected non-admission un-ACKed (transport C:495, :520-522; R37), including a redelivered NDR1 after a lost ACK (directional_delivery C:730 RECEIPT_NOT_OUTSTANDING; R41) and a retired duplicate (CLOSED_REPLAY; R42): each is redelivered every lease until the relay TTL and re-processed by every receive; the loaded seen store is never consulted (C:401-407; R40); known-foreign frames fall through with no ACK, marker or count (C:489, :523-525; R39); P28 aborts a whole poll on one oversize item (P28:690-702, :2862-2867; R54) | tree C, P28 / THE PLAN I07, sec 3 | MEDIUM | T1 (D-R1, D-R4, K6, K10) and vectors V1101, V1102, V1106, V1109 fix the successor; an ENG entry is the Director's call; NOT repaired here. RULED (E8): filed as ENG-0358 (the receive-path candidate defects at #1831/P28); repair named for F11 (T1 D-R1, D-R4, K6, K10 and T1a; vectors V1101, V1102, V1106, V1109, V1126); NOT repaired here |
| E9 | C03 C1 bounds mbb below (>= 65,536, AM-18) but not above; the client's pull CAP scales with mbb (C03 C3); the relay's own source ceiling is 1,048,576 (C03 E1) | C03 / T3 BU3 | LOW | a C03 C1 amendment adding mbb <= 1,048,576 (else relay_v2_unsupported) -- C5-O14. RULED (E9, by F1): C03 AMENDMENTS AM-26; C5-O14 closed |
| E10 | Lock does not stop the engine: vault::protection::lock clears only the passphrase and the flag (protection.rs:255-259); the queue and quarantine store keys stay cached for the process lifetime (msgqueue M:295-342, quarantine M:206); secret_get checks no unlocked flag, so a keychain vault stays readable by engine code (vault M:275-283, :973-1000); no cancellation or generation is tied to lock (engine L1-L4, L8) -- live at MAIN | tree M / THE PLAN I12, sec 5, F05 | MEDIUM | LK7 names the successor rule; the repair is F05/F13's; an ENG entry is the Director's call. RULED (E10): filed as ENG-0359 (lock does not stop the engine at main); repair named for F05/F13 (LK7; vector V1317); NOT repaired here |
| E11 | Truthful-status defects at MAIN (source reading): qsc send returns Ok when ANY contact's queued message went out in the drain (transport M:1965; engine S9); the honest line "will send when the relay is reachable" is shown for a reachable relay answering 403 or another 4xx (transport M:4484-4487, msgqueue M:1098-1109); PausedCause::VaultLocked has no producer (S3); accepted_by_relay is emitted on HTTP 200 before the local commit (transport M:4686-4691; S12) | tree M / THE PLAN D07, I12, SR-13 (distinct causes, distinct names) | LOW-MEDIUM | T4 M1-M6, C1-C3 and V1304, V1306 define the successor; not repaired here. RULED (E11): filed as ENG-0360 (truthful-status defects at main); repair named for F13 (T4 M1-M6, C1-C3; vectors V1304, V1306); NOT repaired here |

==============================================================================================================
OPERATOR QUESTIONS (as asked, each with its options; ANSWERED -- the answers after the table govern)
==============================================================================================================
| Id | Question | Options |
|---|---|---|
| Q1 (E1) | THE PLAN shows four delivery states "to the user"; your decision 1 draws one tick. Which governs the GUI? | (a) decision 1 governs the drawing; the four states stay in the engine and its DTO (for a future message-details view); THE PLAN's table read as the engine's meanings -- a clarifying PLAN amendment, your approval; (b) add a single in-flight mark (reverses decision 1's "deliberately absent"); (c) show "awaiting peer confirmation" as text only in a message-details view, nothing on the bubble |
| Q2 (E2) | Notifications cannot fire while the vault is locked or the app is closed. What should "New message" cover? | (a) only while unlocked and the window is not focused; nothing while locked (no secret outside the vault); (b) keep a pull credential outside the vault so a locked app can say "New message" (weakens "secrets live in the vault"; the ladder's M3 question); (c) no notifications until a relay wake rung exists |
| Q3 (E3) | Auto-lock today keeps the one window and swaps it to the unlock screen, after any in-flight relay call finishes. Your decision 8 says it closes the main window. Which do you want built? | (a) close the main window and open a small unlock window, hiding content FIRST, without waiting for the engine (decision 8's form, with LK1-LK2); (b) keep one window, clear (not hide) all content immediately and show the unlock screen; (c) as today (not recommended: plaintext can stay up while a relay call runs) |
| Q4 (E4) | With no failed mark, how should real failures be told? | (a) one amber line in the conversation per cause, said once (not sent: the relay cannot take it; session ended: N messages not delivered; identity changed; queue full), like decision 6; (b) a per-bubble failed mark only for those terminal cases; (c) only in the contact details pane |
| Q5 (C4-O3) | History has a size limit. How is space freed? | (a) the user deletes a conversation (or messages) explicitly, and the app refuses new messages truthfully when full ("History is full") -- needs a drawn delete action (your OPEN "deleting or editing a message"); (b) per-contact "Disappearing messages" (the drawn, dimmed row) as the only automatic retirement, off by default; (c) automatic oldest-first retirement beyond the limit, with a visible notice |
| Q6 (K11) | A blocked contact's messages are "dropped, not delivered later" and they "won't be told". Should their app still receive a delivery receipt? | (a) no receipt: their messages never show a tick (true: not delivered to you; their app keeps retrying until its own limits); (b) send a receipt then drop: their app shows the tick ("reached the client") and keeps working normally, but the tick no longer means you could see it; (c) close the session (as Delete, without removing the contact) |
| Q7 (E5) | When the offline queue is full, should the composer keep accepting? | (a) keep the typed text in the composer, refuse to queue, say "Too many messages waiting" once in the conversation; (b) raise the queue bounds (C04 C4-O1 feasibility first); (c) silently hold in memory (not recommended: lost on lock or crash) |
| Q8 (E6) | ROADMAP item 3 says you approve the background design; THE PLAN maps it to C05. Do you approve C05 yourself? | (a) yes: C05's acceptance carries your approval after the Director's ruling; (b) no: the Director's ruling after SR-15 suffices (the roadmap subject is satisfied by THE PLAN's acceptance path) |
ANSWERED by the operator (RBANK_C05_operator_answers_2026-09-24.md sha256
6e5e8bd43ae8f3f83ff207b87a5e660da3930c7fdeabde2fe67f75baa6bd219c; its addendum A1 sha256
848a1c9ebda18c6beb8a8db83c286825f40687a4049201fa440072886625b256), as the Director's ruling reads them:
Q1 AMENDMENT A5 (outline tick = on its way, solid tick = delivered, never a read mark; Queued / Prepared no tick);
Q2 (a) (option (d) recorded OPEN, C5-O19); Q3 wipe the main window at once and show a small unlock window on top of it;
Q4 an amber per-message failure mark with a retry action (the look at F13); Q5 "keep messages for" 1 week / 1 month /
1 year / Forever, default 1 month, expiry never retires an undelivered (M1-M3) or closed_undelivered (M6) entry, plus
the per-peer share H_P; Q6 blocked = full stop; Q7 (a); Q8 the operator gives final approval of how the app looks and
functions. Where each lands: the FIXES section.

END OF C05 FINAL

==============================================================================================================
AMENDMENTS (appended by D-1434; the text above is C05 FINAL as merged by D-1433 and is NOT rewritten)
==============================================================================================================
Ruled by RULING_NA0783_C05_PR1843_2026-09-24 (sha256 258b63a5bc782edf69f4c7350827b57650cb43b02c404bf2a0c85702bcee7442),
R3, on escalation E3 of the C05 commit: "CONFIRMED as the ruling's intent -- PREPARED application flights queued to a
contact before a block are NOT pushed or re-pushed while blocked (BU2 / BU8 exception). OWED as one clause in the next
amendment (the C06 boundary commit), recorded here so F11 builds it." The operator's final approval (Q8) is banked
verbatim in RBANK_C05_operator_approval_2026-09-24 (sha256
d9b0c3749151f05c1fc751b855b2a2337242017594ea5ebb29059d2d857ce5cb) and recorded by D-1434: C05 is ACCEPTED, no longer
pending that approval. Where the amendment and the text above disagree, the amendment governs.
| Id | Amends | Amendment |
|---|---|---|
| AM-1 (R3; the C05 commit's E3) | T3 BU2 (2) and BU8; T1 K11 | PREPARED application flights queued to a contact before it was blocked are neither pushed (BU2 (2), the application round-robin across peers) nor re-pushed (BU8) while that contact is blocked: the BU2 / BU8 exception, beside BU2 (2)'s existing exception for the blocked contact's owed controls and receipts (K11, Q6). Unchanged: those flights stay PREPARED, committed protocol debt (C04 R04: released only by an authenticated exact receipt or an explicit session close; THE PLAN D08); the block retires, fails and deletes none of them. F11 builds it. V1117's case (owed receipts and controls) holds no PREPARED application flight from before the block; F11 adds that case to V1117 (named here, not written) |

END OF C05 AMENDMENTS
