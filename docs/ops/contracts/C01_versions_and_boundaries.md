C01 -- VERSIONS AND BOUNDARIES -- FINAL (ACCEPTED WITH NAMED FIXES: CONTRACT_ACCEPTED_WITH_NAMED_FIXES)

==============================================================================================================
FIXES (revision 2 -> FINAL), keyed to RULING_NA0783_C01_ACCEPT_2026-09-23 (sha256
5748f61f8f1525a3577a7a1186ae4e48f5f0acda7bccaf2b8505c15e4d40d421)
==============================================================================================================
Status: C01 ACCEPTED (contract) WITH NAMED FIXES; result class CONTRACT_ACCEPTED_WITH_NAMED_FIXES. The Director's
ACCEPT ruling, on the SR-15 delta re-read (SR15_C01_REV2_FINDINGS.md sha256
185ae10fb57cd5c6905ab5fcfde826b0a73d2aba203e1febbe3d047ab6cc5235, recommendation ACCEPT WITH NAMED FIXES, no BLOCKER),
mandates exactly the fixes below; nothing else in revision 2 changes. The body below is revision 2 (sha256
06ba4863f6fe954545a435a1f61a3669f5d24bc433f53419e96569ac2d5d6c01) with these fixes applied. The word PROPOSED in the
body is retained verbatim and now reads: the accepted contract value, NOT YET ALLOCATED. Identifiers become
ALLOCATED only when the DOC-CAN-003 table (APPENDIX A) merges. O2, O5, O6, O9-O13 stay OPEN; O7 is a PRECONDITION.
F01 stays ACTIVE. Every fix was verified at the source before it was written (file:line measured at C unless
stated); where the ruling's or SR-15's line numbers differ from the source, the source is cited.
| Key | Fix | Where | Source verified |
|---|---|---|---|
| F-Y1 | KDF value m=262144 KiB, t=3, p=1 (was t=4) in all five cells, citing RULING round2 E5+E6 (median 505.5 ms, max 506.0 ms; tolerates a machine up to 1.98x slower; chosen over t=4 because the operator's laptop is unmeasured). CONDITIONAL on a held derived key (F05). All three Argon2 call sites named: init, unlock, provider | CHANGELOG O4; T1 row 14; T5; T6 O4; APPENDIX A A12 | O4_KDF_MEASUREMENT.md:44 (row 262144/3/1), :67 (1.98x); vault/mod.rs init vault_init_core :678-685 -> derive_key call :701; unlock derive_runtime_key :1075, :1092-1094; provider derive_key :1630-1642 (hash_password_into :1640) |
| F-Y2 | O13 added to T6. Row 15 / A13 field list RESERVED, NOT FROZEN until F02 settles the C07 fields; no successor vault written outside tests before F02. Reserve-only C07 rows (class C07; no bytes, no names allocated) for the prepared-successor slot, the freshness checkpoint location and the per-lineage lock. protection_mode reserved beside `mode` (separate discriminators). The init selector's second axis reserved | T1 rows 15, 16, 18; CENSUS classes, D30-D32; T6 O13; APPENDIX A A13, A14, A16, A24 | EXTERNAL_C07 design (sha256 425ff881691d461172bd2a05e9c55c699534d13b7dafcf2a94e8f9610356dbcc) sec A2 table :33 (vault fields), :34 (prepared successor slot), :35 (freshness checkpoint under the XDG state home), :37 (one lineage lock); AMENDMENT A4 "Impact"; RULING round2 O13 |
| F-D5 | O8 L4: the store.meta read-back runs BEFORE protection_state_load / ensure_store_layout and before lock creation (the lock file the one permitted entry), or ensure_store_layout is read-verify-only on the successor path. L5 worded "writes no data file" and narrowed to the QSLD_DATA_DIR reach. F13 note: the successor desktop sets its own location variable, pinned by a test | O8 L3, L4, L5 | protection.rs:150 (protection_state_load first), :158 (passphrase test after), :467-474 (ensure_store_layout :469, lock_store_shared :474); fs_store:213-221 (store.meta written if absent, :219), .qsc.lock created :392-400 and :418-426, probe.tmp :432; vault/mod.rs:802-807; D:lib.rs:452; D:paths.rs:20-24, :38-40 |
| F-D6 | D7/D8 protection-file format line allocated as APPENDIX A row A23 (allocated line; unknown format -> refuse). A02 charset excludes '=' (whitespace is already outside 0x21-0x7E). O12 gains the fourth disagreement (C DOC-CAN-003 :669 transaction prefix vs code) | CENSUS D7; APPENDIX A A02, A23; T6 O12 | protection.rs:476-503 (readers), :521-533 (writer: key=value lines, no format line); C DOC-CAN-003 :669 "na0780_directional_transaction/{peer}" vs protocol_state:1404 "na0780_directional_transaction_v2/" |
| F-D3b | Row 2 names hs_root_combine as a DIRECT KDF input: a row-2 change re-keys every session | T1 row 2 | handshake:802-807 hs_append_key_context, :818 HS_ROOT_COMBINE_KEY, :820-838 hs_root_combine (the ruling's :811-837 spans its doc comment from :809), :836 block appended, :851 hs_pq_init_ss; directional_core:82-89 k() |
| m1 | Doctor export: OUTPUT row | CENSUS D27 | lib.rs:1691-1700, :539-550; main.rs:135 |
| m2 | Feature-gated test blocker directory: TEST row | CENSUS D28 | protocol_state:1339-1343 |
| m3 | OS keychain service: out of successor scope until decided, class O6 | CENSUS D29; T6 O6 list | vault/mod.rs:79-80, :1285-1288; qsc Cargo.toml:33, :36; D src-tauri/Cargo.toml:23 (no features) |
| m4 | .qsc.lock second literal named | CENSUS D4 | model/mod.rs:264, :275 (inside the #[cfg(test)] module :225-453); lib.rs:38 |
Also changed, as consequences of acceptance only: the title line and the closing END line.

PLAN: QSL-solution-plan rev3 d9016e53d2ab46c32b7a7cb060ea70dc79421617e9b054848b464ba7a5518275, card F01,
sub-assignment C01, revision 2. Lane NA-0783 (D-1425). Drafted 2026-09-23 by the executor seat (opus/high).
Supersedes round 1 (C01_DRAFT.md sha256 8410604b424f223578aa9772f62089cf06a9cd3b71af12473406ab7a3c077e41), which
the Director's ruling RULING_NA0783_C01_round1_2026-09-23 did NOT ACCEPT (R1). This revision folds R2 and the
resolutions O1, O3, O4 (method), O8, E4 and O7 of that ruling. Status of every row: PROPOSED. ACCEPTED only by the
Director's ruling after the delta-only SR-15 re-read.
Scope unchanged: THE PLAN row C01. Not in scope: C02 handshake/invite bytes (K-07), C03 relay, C04 reservation
dimensions, C05 dispatcher/ACK rules, C06 file bytes, C07 rollback.

Sources measured (read-only, bare mirrors): qsl-protocol main M = 3748dbfe2db8d15b8081fe0cee96af9e676bac7a;
candidate #1831 C = ffc8fc529374e62b00ae9018726ab313efe0f036 (not a descendant of M; merge-base b9e7307c);
qsl-desktop main D = 15818498cdd0536f08a1b9909291ee5153168cfe, which pins qsc at PIN =
08c0e327d21cabb100c1c43cfcd1bb1619fee149 (D src-tauri/Cargo.toml:23; PIN is an ancestor of both M and C).
File:line is qsl/qsl-client/qsc/src/<file>:<line> at C unless a revision or path is given. "T:" marks
qsl/qsl-client/qsc/tests/. "D:" marks qsl-desktop src-tauri/src/. Every citation in this revision was re-read at
the source by this seat; where the SR-15 findings and the source disagree, the source is used and the
disagreement is listed in CHANGELOG entry S1-S3.

==============================================================================================================
CHANGELOG (round 1 -> revision 2), keyed to the ruling and the SR-15 ids
==============================================================================================================
| Key | Change in revision 2 | Where |
|---|---|---|
| X1 / Q2c | Row 2 marked CRYPTO-TOUCHING: the profile value is a KDF input of the receipt key (delivery:40-42) and an AD input of every receipt (delivery:60-63), besides the handshake transcript. SR-15 sensitive review named as required for any row-2 change. Dimension list and cell header made consistent | T1 row 2 |
| X2a | T2 A-C rewritten for the desktop AS SHIPPED: D pins PIN, where Current=QSCV02 and KnownOld={QSCV01} only; a C-era QSCV03 vault is UNKNOWN there, is not gated (D:commands.rs:323), reaches the guard and is counted | T2 A, B, C |
| X2b / O7 | O7 is a PRECONDITION of any desktop acceptance of T2 A-C, not a mitigation. Refusals after the magic (KDF header, payload version/profile) still count on a successor-pinned desktop | T2 A-C; T3; T6 O7; checks |
| X2c | Named: every refusal through the guard writes the counter file (protection.rs:178-180) and arms the delay schedule (:151-157, :203-206), even when wipe is not armed | T2 row V7 |
| X3 | Hostile wire rows added: strip, duplicate, non-critical, reorder, over-long block, truncated handshake, truncated NDE1, truncated NDI2, altered B1/confirm block | T2 rows W1-W9 |
| X4 / Q2e | Row 13 rationale extended: the magic AND the KDF fields are AEAD-bound (vault/mod.rs:1162-1169). New-magic-over-old-bytes row added, with the successor refinement (an old KDF header refuses before Argon2) | T1 row 13; T2 row V1 |
| X5 | Row 21 REPLACED by the local-store CENSUS (measured, enumerate-and-classify, zero unclassified) | T1 row 21; CENSUS |
| X6 / #22 | Check "D (no effects)" re-targeted from handshake:1541 (initiator) to the responder: hs_decode_init :2512 precedes hs_pending_store :2671; Err arm :2721 -> continue :2768 | checks |
| Q2a | Row 12 names the domain labels C02 owns: invite:66 DS_COMMIT, :68 DS_SIG, handshake:1504-1506 "QSC.HS.SID", and (NEW) handshake:818 "QSC.HS.ROOT.COMBINE.v1" | T1 row 12 |
| Q2b | Row 10 names the retired legacy payload versions: adversarial/payload.rs:3 FILE_XFER_VERSION, :4 ATTACHMENT_DESCRIPTOR_VERSION, :45 CTRL_VERSION_MAX; lib.rs:86 duplicate descriptor constant | T1 row 10 |
| Q2d | Row 20 decides: the per-peer read-to-refuse at protocol_state:1367 is REPLACED by the directory-level foreign-store refusal (O8 rule L4). A legacy session file cannot exist in a successor directory, so establishment never reads the legacy store; F03 must know this | T1 row 20; O8 |
| Q2f | QSCV04 with an old KDF header -> vault_parse_failed before Argon2 (today's shape, vault/mod.rs:1062-1063): row added; distinct code routed to O9 | T2 row V2; T6 O9 |
| Q2g | Mixed-profile rows added: inside one vault (aggregate :1968-1977) and on disk (old per-contact directories); both closed by O8 location separation | T2 rows V3, V4 |
| Q2h | Desktop destroy and create doors added as rows (NOTE class; not wipe paths) | T2 rows V5, V6 |
| Q2i | T2 D/E states the two mechanisms: PARSE on A1 (hs_parse_parameter_block :278-379), byte EQUALITY on B1/confirm (hs_contexts_match :1308-1315 at :2014, :2297). C02 keeps both | T2 D, E, W9 |
| Q3 dup | Row 15: duplicate keys refused at EVERY map level. Measured: the top-level `secrets` map ALREADY refuses them at C (vault/mod.rs:94, :98-120); nested maps do not (see S1) | T1 row 15 |
| Q3 queue | Row 19 decides: record AAD label, store-key secret name and directory name all change (msgqueue_v2 family); the seen-ids AAD is test-only at C and is RETIRED (see S2) | T1 row 19; CENSUS |
| Q4 | Identity's lazy migration (identity:348, called :459) is an explicit O6 decision under D02. NEW: a second live lazy migration exists (handshake pending file -> vault, handshake:1250-1270) and a legacy re-interpreting reader (identity:484-489); both joined to O6 | T6 O6; CENSUS |
| #28 #36 #40 #42 | Stale citations corrected: typed_body_decode :378-417; QSP session AAD :144-145 (path :140); directional_file_shape store:311-377; M qsp_unpack :2260 with FLAG_* imported (M lib.rs:24), not defined | T1, T3, T4 |
| O1 | RESOLVED (ruling): allocation by a new canonical table in DOC-CAN-003, recorded by the D-record of the PR that adds it. Draft rows in APPENDIX A. Values PROPOSED | sec 0; APPENDIX A |
| O3 | RESOLVED (ruling): storage-only is a MODE of the one profile. Carried by a payload field `mode`; no second profile string | T1 row 16; APPENDIX A A14 |
| O4 | RESOLVED IN METHOD (ruling); MEASURED here (O4_KDF_MEASUREMENT.md). VALUE per RULING round2 E5+E6: m=262144 KiB, t=3, p=1, CONDITIONAL on N1 (a held derived key, F05); all three Argon2 call sites (init, unlock, provider) | T1 row 14; T5 |
| O8 | RESOLVED (ruling): distinct data/config location. Rule L1-L6 specified, with old-build and new-build behaviour | O8 RULE; T2 C, C2 |
| E4 | L5 S5 migration NOT carried (ruling) | T5 |
| O7 | Stated as a PRECONDITION with its RED-at-main regression named | T6 O7; checks |
| S1 | SR-15 Q3 DISAGREED IN PART: "duplicate keys inside `secrets` are last-wins" is false at C (detector at vault/mod.rs:94-120), true at M (M vault/mod.rs:86-90) and true of nested maps at C | T1 row 15 |
| S2 | SR-15 X5 DISAGREED IN PART: the seen-ids AAD "qsc.msgqueue.seen.v1\|" (msgqueue:727) is #[cfg(test)] at C (msgqueue:709-790); production only at M (M msgqueue:670-684) | CENSUS D18 |
| S3 | SR-15 X5 DISAGREED: "outbox v1" is not row 19 double-listed. outbox.json is a distinct PLAINTEXT store (lib.rs:39; store/mod.rs:10-23) carrying the user-typed `to`, read and written inside relay_send_with_payload (transport:2887, :2942-2960, :3093-3113) | CENSUS D10; T5 |
| N1 | NEW: every free-function vault access re-derives Argon2 with the retained passphrase (secret_get :326-333 -> :1040-1052 -> derive_runtime_key :1075-1094); per-derivation time is not per-action time | T1 row 14; T5; T6 O4c |
| N5 | NEW: handshake:93 hard-codes the parameter length 25; the successor value length must be computed | T1 row 3 |

==============================================================================================================
0. THE REGISTRY QUESTION -- RESOLVED IN DIRECTION (O1)
==============================================================================================================
| Dimension | Registry that allocates it today | After O1 (PROPOSED) |
|---|---|---|
| Suite tuple (protocol_version, suite_id) | DOC-CAN-003 sec 1.1 (M :48-56; namespace rule :55-56) | unchanged; APPENDIX A row A01 cites it |
| Profile / handshake parameter ids / wire magics / body kinds | NONE at M. C carries an appended "NA0780 first-release directional profile reservation" (C DOC-CAN-003 :653-685) that says of itself "not an external registry allocation" (:659) and disagrees with C's code (:663 "-01", :665 NDI1, :670 schema 3; code -03 / NDI2 / version 4) | the new canonical table (APPENDIX A), which SUPERSEDES the C appended reservation section; that section's rows become RETIRED rows of the table (O12) |
| Local schema versions (vault, queue, stores) | NONE (constants in code only) | APPENDIX A rows A11-A20 |
| File / attachment kinds | DOC-CAN-005 sec 2.2 covers part_size_class and retention_class only | APPENDIX A row A22 (reservations only; C06 allocates bytes) |
| Reason / error codes | DOC-SCL-002 sec 5 | unchanged; new codes named here are PROPOSED spellings, registered through DOC-SCL-002 by the implementing PR (O9) |
The table is committed by the PR that adds it; that PR's D-record records the allocation (ruling O1). Until then
every value in APPENDIX A is PROPOSED and allocates nothing.

==============================================================================================================
T1. IDENTIFIERS
==============================================================================================================
| # | Dimension / identifier | Existing at M | Existing at C | Proposed successor value (PROPOSED) | Registry row | Owning module (proposed) |
|---|---|---|---|---|---|---|
| 1 | Suite tuple | (0x0500,0x0002) Suite-2, handshake:33-35; legacy (0x0403,0x0001) :37, :39 | same lines | KEEP (0x0500,0x0002). Legacy tuple stays REJECT_QSC_HS_DOWNGRADE | A01 | handshake HS_SUITE_CONTEXT_BLOCK; refimpl suite2/types.rs:3-4 |
| 2 | Successor profile id -- CRYPTO-TOUCHING. ONE value used in: handshake parameter 0x7f80 value (enters the transcript MAC/hash, T:handshake unit :3222-3230); SESSION ROOT KDF, DIRECTLY: hs_root_combine (handshake:820-838) appends the whole parameter block (hs_append_key_context :802-807, call :836) into the KMAC data keyed by "QSC.HS.ROOT.COMBINE.v1" (:818), e.g. hs_pq_init_ss :851, and core k() (core:82-89) keys every session, epoch and receipt key from that root and sid, so a row-2 change RE-KEYS EVERY SESSION; RECEIPT KEY KDF binding `lp(profile) \|\| ectx(epoch,direction,dh)` -> k(sid,root,"RECEIPT_KEY",binding) (delivery:40-42); RECEIPT AD `lp(profile) \|\| lp(core PROFILE) \|\| prefix(slot)` (delivery:60-63); vault payload `protocol`; Transaction.version; QueuedIntent.profile; packed queue marker; store.meta marker (A18) | ABSENT | "NA0780-DIR-INTEGRATION-03" delivery:9; -01/-02 refused (vault/mod.rs:2216, delivery:1385) | ONE FRESH ASCII value, distinct from -01/-02/-03 and from NA0780-OWNER-FREE-01; exact string OPEN (O2); length 1..50 bytes (HS_PARAM_BLOCK_MAX 64 minus the 9-byte suite block minus the 5-byte parameter header, handshake:28, :33, :93). Changing it changes every session root (hs_root_combine) and every receipt key and receipt AD by construction: that is the intended authenticated separation, and it makes this row a CRYPTO-INPUT change. REQUIRED REVIEW: SR-15 independent sensitive design review of the row-2 change (F01 acceptance line "independent sensitive design review where required"), and the same at the implementing PR. Reason not to reuse -03: K-08 changes the vault KDF and the per-contact layout; reuse would make -03 development state read as successor state (I01 reinterpretation) | A02 | one const, today directional_delivery INTEGRATION_PROFILE |
| 3 | Handshake parameter id for the profile | ABSENT (M treats it as unknown critical, M handshake:332-343) | 0x7f80, flag 1, inline literal `[0x7f,0x80,1,0,25]` handshake:93; parse :336-338 | KEEP id 0x7f80, flag critical; named const. Value = row 2. The length byte pair is COMPUTED from row 2 (today the literal 25 at :93 equals len("NA0780-DIR-INTEGRATION-03"); copying it is a defect) | A03 | handshake |
| 4 | QHSM handshake version | HS_VERSION_LEGACY 1, V2 2, handshake:23-24 | v1 -> REJECT_QSC_HS_INTEGRATION_REQUIRED (:476); v2 | KEEP v2; v1 refused. Whether C02's A1 change needs a bump is C02's (O11); it rides row 2 | A04 | handshake |
| 5 | Core KDF context / KMAC prefix | ABSENT | "NA0780-DIR-EPOCH-CORE-01" core:14; "NA0780.DE1/" :86 | KEEP BYTE-FOR-BYTE (renaming is a crypto change, not a version act). Separation from -0n sessions: sid and root come from a handshake whose transcript binds row 2, AND row 2 enters the receipt KDF/AD directly (row 2) | A05, A06 | directional_core |
| 6 | Directional epoch frame | ABSENT | "NDE1", kinds 0/1, core:220, :313-320 | KEEP NDE1; kind > 1 refused TYPE | A07 | directional_core Wire |
| 7 | Core typed kind | ABSENT | 0 body / 1 reserved / 2 target adv., core:105-112 (`p[0] > 2 -> TYPED` :111) | KEEP; kind 1 reserved (delivery refuses INTEGRATION_KIND, delivery:791-793) | A08 | directional_core |
| 8 | Exact-wire receipt | ABSENT | "NDR1", 113 bytes, delivery:10-11, :52 | KEEP | A09 | directional_delivery ReceiptContext |
| 9 | Inner typed body | ABSENT | "NDI2" delivery:390 (typed_body_decode :378-417); NDI1 refused | KEEP NDI2. Kinds: 0 application; 1-4 RESERVED file kinds (T4); 5 maintenance; 6-255 -> INTEGRATION_KIND (:345) | A10 | directional_delivery |
| 10 | Legacy message wire and legacy payload versions | QSE env 0x0100 M lib.rs:53; CTRL_VERSION 2 M lib.rs:51; qsp_unpack M lib.rs:2260 (FLAG_* imported M lib.rs:24, used :2275-2277); FILE_XFER_VERSION 1, ATTACHMENT_DESCRIPTOR_VERSION 1 (adversarial/payload.rs:3-4; M lib.rs:94), CTRL_VERSION_MAX 2 (payload.rs:45) | receive path removed; 01 00 frames -> MAGIC non-admission; payload.rs:3-4, :45 and lib.rs:86 still present | RETIRED in the successor: no reader, no fallback (I01, D01). The legacy payload versions retire with it (T4 rule 4) | A21 | -- |
| 11 | Frame classifier | QHSM / 01 00 / 01 01 / 01 02 / Unknown, frameclass.rs:64-81 | identical | KEEP as a match, not an identification (frameclass.rs:1-9) | n/a | frameclass |
| 12 | Invite wire and handshake/invite domain labels | invite:71-85 | same; labels invite:66 DS_COMMIT "QSL.invite.identity-commitment.v1", :68 DS_SIG "QSL.invite.payload.v1" (uses facade:1012-1013); "QSC.HS.SID" handshake:1504-1506; "QSC.HS.ROOT.COMBINE.v1" handshake:818 | DEFERRED TO C02, NAMED: C02 owns the invite wire (QSLI-1-, INVITE_VER, BUNDLE_VER, ENVELOPE_VER, TAG_*) AND the four domain labels listed. C01 froze none of them. Any C02 change is allocated in APPENDIX A and rides row 2 | A04 note | invite; handshake |
| 13 | Vault envelope magic | "QSCV02"; KnownOld = QSCV01 (vault_format.rs:1, :18) | "QSCV03"; KnownOld = QSCV01, QSCV02 | "QSCV04". Collision probe re-run: 0 hits at M, 0 in D, 1 at C (T:vault.rs:512, a negative probe SR-18 must move). KnownOld = QSCV01, QSCV02, QSCV03. Reasons: (a) the magic is the only gate every build and the desktop pre-flight see; (b) the magic AND the KDF fields are AEAD-bound: envelope_header_bytes writes the BUILD's VAULT_MAGIC (vault/mod.rs:1162-1163) and the three KDF words (:1167-1169) into the AD, so QSCV04 ciphertexts are separated from every older envelope with no further act; (c) consequence: a file whose magic is rewritten cannot be refused by name after the AEAD (T2 V1) | A11 | vault_format classify_vault_magic (proposed move out of `adversarial`) |
| 14 | Vault KDF parameters (header, AEAD-authenticated) | m=19456 KiB, t=2, p=1 exact-match, vault/mod.rs:45-47, M :960 | same, :1062 (refusal vault_parse_failed :1063) | MEASURED (O4_KDF_MEASUREMENT.md, argon2 0.5.3 as locked by M and D); VALUE per RULING round2 E5+E6: PROPOSED Argon2id v0x13, m=262144 KiB, t=3, p=1, 32-byte output. Measured max 506.0 ms (median 505.5) on the measurement box against the 1000 ms budget: tolerates a machine up to 1.98x slower (chosen over t=4, max 640.1 ms, because the operator's laptop is unmeasured; the laptop is measured before F13 acceptance); m = 0.80 percent of that box's RAM against the 25 percent cap. THREE Argon2 call sites carry the value: init (vault_init_core :678-685 -> derive_key :701), unlock (derive_runtime_key :1075, :1092-1094) and the provider (derive_key :1630-1642, hash_password_into :1640); F03/F05 pin all three. CONDITION (N1): qsc re-derives Argon2 on every free-function vault access (secret_get :326-333 -> load_vault_runtime :1040-1052 -> derive_runtime_key :1075-1094); session paths reuse the key (:1940-1945). At about 506 ms per derivation that cost multiplies per user action, so the value is proposed ONLY together with a held derived key (F05, ROADMAP item 5 "key rather than retained passphrase"); without it the proposal falls to the strongest row whose per-ACTION time fits (O4c). Reader keeps exact-match against the successor constants, checked BEFORE any Argon2 run (today :1050 precedes :1052) | A12 | vault |
| 15 | Vault payload schema | version 1 written, never checked (M vault/mod.rs:86-96); no unknown-field or duplicate detector | PAYLOAD_VERSION 4 (:50), deny_unknown_fields (:90), exact (:1963-1965); top-level `secrets` duplicate keys REFUSED (unique_secret_map :94, :98-120) | version 5 under QSCV04, exact-match, deny_unknown_fields, fields {version, protocol = row 2, mode = row 16, secrets}. FIELD LIST RESERVED, NOT FROZEN (O13): the C07 lineage fields (vault_id, protection_mode, generation, predecessor anchor, checkpoint MAC key, TPM enrollment record) are RESERVED and enter version 5 only when F02 settles them; no successor vault is written outside tests before F02, so no second schema change is forced (A4 Impact). Duplicate keys refused at EVERY map level: KEEP C's top-level detector; ADD detection to the nested JSON maps carried as secret values, which are derived BTreeMaps with no detector today (CapacityOwner peers/entries protocol_state:1417-1418; core :132, :189, :191, :194; delivery :252-257) -- F04, with a test per map (serde_json behaviour on those not executed here) | A13 | vault; protocol_state; directional_* |
| 16 | Vault storage mode (O3 RESOLVED: a MODE of row 2) | -- | a SECOND value of the same discriminator: `protocol` = "NA0780-OWNER-FREE-01" (:49, :126-129), own branch (:1974-1978), own selector (:628) | NO second profile value. Payload field `mode`, exact values "messaging" or "storage-only"; `protocol` is row 2 in BOTH modes. Refusals: unknown/absent mode -> vault_mode_unsupported (PROPOSED code) at decode, before any write; a storage-only vault holding the owner key or any peer key -> directional_owner_binding (today's code, :1975-1976); a messaging vault without the owner key -> directional_reserve_missing (today :1958). Mode is fixed at init; no in-place change (a change is a new vault). Mode never appears on the wire; a storage-only vault refuses session establishment (PROPOSED code directional_mode_storage_only) before any handshake write. RESERVED beside `mode` (O13, A4): the field name protection_mode, a SEPARATE discriminator for the C07 protection profile; `mode` and protection_mode never share a discriminator | A14 | vault |
| 17 | Vault namespaces | -- | owner "na0780_directional_owner_v1", peer prefix "na0780_directional_transaction_v2/" + alias (protocol_state:1401-1406; key build :1253-1256) | Fresh pair bound to row 15; peer suffix = opaque contact id (label split, T5), never the typed label. Exact strings OPEN with C04 (O5) | A15 | protocol_state layout |
| 18 | Init selector | -- | --protocol directional-v1 / owner-free-v1 (vault/mod.rs:626-629); fresh config required (:803-807) | `--mode messaging` or `--mode storage-only`; absent/unknown -> refuse before any write; no spelling of the profile on the CLI. Fresh successor directory required (as :803-807). The selector's SECOND AXIS (the C07 protection profile selected at fresh-identity creation, A4) is RESERVED; no spelling allocated (O13) | A16 | vault_init |
| 19 | Queue record family | RECORD_VERSION 1 never checked; AAD "qsc.msgqueue.v1\|" ; dir msgqueue_v1/hex(sha512(label)[..8]) (M msgqueue) | same (msgqueue:36, :44, :360-364, :387); read_record checks msg_id/seq only (:445-467); packed marker {"schema":1,"protocol":"directional-v1"} (msgqueue:2129-2146) | ALL CHANGE (Q3 decided): directory "msgqueue_v2"; per-contact subdirectory keyed by the contact id (row 17), not a label hash; record version 2 CHECKED on read (after the AEAD: defence in depth, the record is under a vault-held key); record AAD label "qsc.msgqueue.v2\\|<contact id>\|<msg_id>\|<seq>"; store-key secret "msgqueue_store_key_v2"; packed marker carries row 2 (no "directional-v1" spelling). The seen-ids store is not carried (test-only at C, CENSUS D18). Binary file queue row: RESERVED to C06. The AAD label is a local-store AEAD input: named for the SR-15 re-read | A17 | msgqueue |
| 20 | Legacy QSP session store | QSSV01 v1; AAD "QSC.QSP.SESSION.V1:<label>" (protocol_state:144-145; store:35-37); QTRG / QS2S v3 (:236-241, :651, :664-668) | same; directional_establish READS it to refuse (:1367 directional_migration_refused) | RETIRED: never read, never written, preserved untouched IN THE OLD DIRECTORY. Q2d decided: the per-peer read-to-refuse at :1367 is replaced by the directory-level refusal O8 L4 (a qsp_sessions/ entry in a successor directory = successor_dir_foreign at open). Consequence F03 must pin: establishment never reads the legacy store | A21 | protocol_state |
| 21 | Other local stores | see CENSUS | see CENSUS | REPLACED BY THE CENSUS below (X5). Rule unchanged: every reader kept on the successor path checks an exact version and refuses with a distinct code; no default-to-empty; no lazy migration | A13, A17, A18 | each store |
| 22 | File / attachment kinds | see T4 | see T4 | see T4 | A22 | T4 |

==============================================================================================================
CENSUS -- LOCAL STORES (replaces round-1 row 21). MEASURED at C; M delta checked.
==============================================================================================================
Method: (1) every path join in qsc src at C (`.join(` sites, 91 lines inspected); (2) every named file/dir/secret
constant (const &str / &[u8] literals matching file|dir|name|store|meta|lock|queue|json|txt|key|secret|version|
magic|aad); (3) every vault secret access (secret_get/secret_set/secrets.insert/get call sites) with dynamic key
names resolved; (4) M vs C constant-set delta: the only store-relevant constant at M absent at C is VAULT_MAGIC
"QSCV02". Test-only temp paths under the OS temp dir (tests' own fixtures) are excluded by rule and listed as
class TEST. Classes: SUCC (on the successor path, kept; exact reader required), SUCC-NEW (replaced by a successor
identifier), RETIRE (legacy; never read or written by the successor; preserved untouched in the OLD directory),
O6 (disposition OPEN, proposal given), C02/C03/C06 (owned by that contract), C07 (RESERVED for C07: no bytes, no
names allocated, O13), TRANSIENT, TEST, OUTPUT (user output
outside the store), DESKTOP (app-owned, outside qsc). UNCLASSIFIED: 0.

On-disk entries (relative to the qsc config directory unless stated)
| Id | Entry | Where (C) | Version / reader today | Label in name or plaintext? | Class | Successor disposition (PROPOSED) |
|---|---|---|---|---|---|---|
| D1 | vault.qsv | vault/mod.rs:1714 | QSCV03 magic + payload v4, exact | no | SUCC-NEW | QSCV04 + payload v5 (rows 13-16) |
| D2 | <file>.tmp.<pid> atomic temps | fs_store:223-239 | n/a | no | TRANSIENT | keep |
| D3 | vault.qsv.tombstone.<pid> | protection.rs:563 | n/a (wipe path) | no | TRANSIENT | keep |
| D4 | .qsc.lock | lib.rs:38; fs_store:384, :414; a second literal at model/mod.rs:264, :275 (inside the #[cfg(test)] module :225-453) | n/a | no | SUCC | keep; SR-18 moves both spellings if the name ever changes |
| D5 | store.meta "store_version=1 ..." | lib.rs:37, :53; fs_store:213-221 | written if absent; content NEVER read | no | SUCC-NEW | successor writes store_version=2 and profile=<row 2> and READS it at open (O8 L4) |
| D6 | config.txt (policy_profile; ack_mode tombstone) | lib.rs:36, :46; :434-560, :939, :2132; fs_store:40-45 | key=value, tombstone detection | no | O6 | proposed SUCC with an exact key set; unknown key -> refuse |
| D7 | vault_security.txt (attempt_limit) | store/mod.rs:75; protection.rs:467-510, :512-530 | unversioned text; unreadable -> vault_attempt_limit_io (:150) | no | SUCC | proposed: add a format line (APPENDIX A A23); unknown format -> refuse (fail closed, no count). Carries the ARMED LIMIT (O7) |
| D8 | vault_unlock_failures.txt | store/mod.rs:76; same readers/writer | unversioned text | no | SUCC | as D7 |
| D9 | send.state | lib.rs:44; transport:93, :3281, :3340 | unversioned | no | O6 | proposed RETIRE with the legacy send commit (F04 traces reach) |
| D10 | outbox.json (OutboxRecord) | lib.rs:39; store/mod.rs:10-23; transport:69-96, :123-126, :2942-2960, :3093-3122 | version 1 written (:3094), NEVER checked; six fields serde(default) | YES: `to` and `channel` in PLAINTEXT JSON | O6 | proposed RETIRE in favour of the vault-committed queue intent (D03); if kept: contact id instead of label, exact version, no serde(default). Reach from a directional APPLICATION send not traced (F04) |
| D11 | qsp_status.json | protocol_state:32, :51-55 | unversioned plaintext status | no | RETIRE | with row 20 (F04 confirms no successor reader) |
| D12 | qsp_sessions/<peer>.qsv (QSSV01) | protocol_state:141, :144-145; store:33-37 | QSSV01 v1; AAD binds label | YES (filename, AAD) | RETIRE | row 20 |
| D13 | qsp_sessions/<peer>.bin (+ tombstone QSC_SESSION_MIGRATED_V1) | protocol_state:137; store:34 | legacy | YES (filename) | RETIRE | row 20 |
| D14 | qsp_sessions/<peer>.scka.json | protocol_state:544 | legacy | YES (filename) | RETIRE | row 20 |
| D15 | identities/self_<self_label>.json | identity:34, :117-121 | public record; legacy record LAZILY MIGRATED (:348, called :459) and a legacy record reinterpreted with empty sig_pk (:484-489) | YES (filename; own label) | O6 | proposed SUCC-NEW keyed by an opaque local id; BOTH legacy readers removed from the successor path (D02) |
| D16 | handshake_pending_<self>_<peer>.json (legacy) | handshake:1178-1179; read and MIGRATED into the vault then deleted :1250-1270; removed on clear :1294-1297 | legacy JSON | YES (filename) | O6 | proposed RETIRE: the successor never reads it (a lazy migration, D02); its presence in a successor directory = successor_dir_foreign |
| D17 | msgqueue_v1/<hex16(sha512(label))>/<seq>_<msg_id>.rec | msgqueue:36, :360-377, :540, :643 | RECORD_VERSION 1 never checked (:445-467) | hashed label in dir name | SUCC-NEW | row 19 |
| D18 | msgqueue_v1/<ck>/seen_inbound.dedup, AAD "qsc.msgqueue.seen.v1\|" | C msgqueue:709-790 ALL #[cfg(test)]; production at M (M msgqueue:670, :680, :684) | M: v1 | hashed label | TEST (C) / RETIRE (M) | not carried |
| D19 | quarantine_v1/<record> | quarantine:66, :255, :271, :434-449 | RECORD_VERSION 1 (:75) written, never read back | no (checked: record_name from time and id) | O6 | proposed SUCC with exact version, or RETIRE if F04 finds no successor writer |
| D20 | relay_seen_ids_v1_<hex16(sha512(route token))>.json | dedup:17, :74 | wrong version or parse error -> RESET to empty (:69-87) | no (token hashed) | O6 | proposed exact refuse; the availability trade-off stated at dedup:69-72 is the O6 question |
| D21 | attachments/{direction}/<id>.cipher staging | lib.rs:98; attachments:62-84, :680 | n/a | no | C06 | RESERVED; not written while file send is refused (T2 I) |
| D22 | probe.tmp.<pid> | fs_store:432 | n/a | no | TRANSIENT | keep |
| D23 | qsc__<account> keychain seam file | vault/mod.rs:1450-1453 cfg(all(feature="keychain", qsc_keychain_test_seam)) | test seam | no | TEST | not carried |
| D24 | recv_<hex(h(sid,peer,id))>.bin in a caller-chosen output dir | delivery:908-914 | n/a | no (hashed) | OUTPUT | unchanged; not a store |
| D25 | attachment output + its .tmp | attachments:1333-1335 | n/a | user-chosen name | OUTPUT | gated with C06 |
| D26 | desktop settings.json, webview/ (app data dir, outside qsc) | D:paths.rs:50-52, :66-68 | app-owned | not measured | DESKTOP | F13; O8 L3 leaves them in the app data dir |
| D27 | doctor export <path> + <name>.tmp.<pid> at a caller-chosen path | lib.rs:1691-1700 write_doctor_export; main.rs:135 (doctor --export) | DoctorReport JSON (:539-550): booleans, a static config_dir tag, `redacted` | no | OUTPUT | unchanged; not a store (as D24, D25) |
| D28 | vault.qsv.tmp.<pid> DIRECTORY (test save-fault blocker) | protocol_state:1339-1343 under #[cfg(feature = "na0780-test-hooks")] | n/a | no | TEST | not carried (non-shipping) |
| D29 | OS keychain entries: service "qsc", account "vault-<hex salt>" (outside the config directory) | vault/mod.rs:79-80, :1285-1288; feature "keychain" (qsc Cargo.toml:33 default = [], :36); the desktop enables no qsc feature (D src-tauri/Cargo.toml:23) | per-salt entries | no | O6 | OUT OF SUCCESSOR SCOPE until decided; the shared service name "qsc" is not reached by O8's directory rule |
| D30 | prepared-successor vault slot (C07 A2, A5) | does not exist | -- | -- | C07 | RESERVED: no name, no bytes allocated (O13) |
| D31 | freshness checkpoint location (outside the store directory; C07 A2) | does not exist | -- | -- | C07 | RESERVED: no path, name or format allocated; not governed by O8 L1-L6 (O13) |
| D32 | per-lineage lock (C07 A2) | does not exist | -- | -- | C07 | RESERVED: no name allocated; distinct from D4 (O13) |
Vault secret keys (inside the encrypted payload)
| Id | Key | Where (C) | Version / reader today | Label in key? | Class | Successor disposition (PROPOSED) |
|---|---|---|---|---|---|---|
| V1 | na0780_directional_owner_v1 | protocol_state:1403 | exact, aggregate-checked | no | SUCC-NEW | row 17 (O5) |
| V2 | na0780_directional_transaction_v2/<alias> | protocol_state:1404, :1253-1256 | exact | YES | SUCC-NEW | contact-id suffix (row 17) |
| V3 | msgqueue_store_key_v1 | msgqueue:39, :319-342 | 32-byte hex | no | SUCC-NEW | msgqueue_store_key_v2 (row 19) |
| V4 | quarantine_store_key_v1 | quarantine:70, :222-239 | hex | no | O6 | with D19 |
| V5 | owed_receipts_v1 | owed_receipts:46, :73-93 | parse error -> RESET to empty (:85); STORE_VERSION never compared | no | O6 | proposed exact refuse, leave bytes |
| V6 | qsp_session_store_key_v1 | store:37; protocol_state:174-212 | legacy | no | RETIRE | row 20 |
| V7 | contacts.json | store:38; contacts:368-423, :857 | not measured | holds labels (content) | O6 / O5 | the label-to-contact-id resolver's home (O5); exact version |
| V8 | timeline.json | store:39; timeline:375-394; vault/mod.rs:2124-2128 | touched by the directional commit | holds labels (content) | O6 | SUCC with exact version |
| V9 | tui.receipt.mode, tui.receipt.batch_window_ms, tui.receipt.jitter_ms, tui.file_confirm.mode | store:40-43; preflight lib.rs:2219-2227 | unsupported values refused at preflight | no | O6 | keep the preflight refusal |
| V10 | tui.trust.mode | store:44; contacts:1589 | not measured | no | O6 | -- |
| V11 | tui.relay.token, tui.relay.token_file, tui.relay.inbox_token, tui.relay.ca_file | store:54-61; vault/mod.rs:48, :736-739; main.rs:541-595; transport:1258-1452 | plain values | no | C03 | C03 owns |
| V12 | invite.created, invite.redeemed | store:66-67; invite:684-706 | not measured | not measured | C02 | C02 owns |
| V13 | invite.ownership | invite:1042; vault/mod.rs:543, :556-559 | exact | not measured | C02 | C02 owns |
| V14 | outbox.next_state.v1 | store:68; transport:128-152 | Suite2 snapshot hex | no | O6 | with D9/D10 |
| V15 | contact_requests.json | store:69; contacts:411-423 | not measured | holds labels (content) | C02 | C02 owns |
| V16 | attachments.json | store:70; attachments:8-20 | journal | no | C06 | RESERVED |
| V17 | identity.kem_sk.<self_label>, identity.sig_sk.<self_label> | identity:206, :210, :216-300 | hex | YES (own label) | O6 / O5 | opaque local id suffix |
| V18 | handshake.pending.<self_label>.<peer> | handshake:1182-1184, :1233-1292 | JSON, cleared = "" | YES | SUCC-NEW / C02 | contact-id suffix (T5); content C02 |
| V19 | generic caller-named secrets | vault/mod.rs:326-349, :389-406, :480-495 (guard_directional_generic_write :492) | none | caller-chosen | SUCC | keep; the directional prefix stays guarded |

==============================================================================================================
O8 RULE -- SUCCESSOR DATA/CONFIG LOCATION (RESOLVED by the ruling; the RULE is fixed here, spellings in APPENDIX A)
==============================================================================================================
Measured inputs: qsc resolves its directory as QSC_CONFIG_DIR, else XDG_CONFIG_HOME/qsc, else <HOME>/.config/qsc
(fs_store:10-30). The desktop resolves <app data dir>/qsc with app data dir = QSLD_DATA_DIR, else
XDG_DATA_HOME/<app id>, else the XDG default data home under <HOME> ("dot-local/share") /<app id>, and sets QSC_CONFIG_DIR once at bootstrap
(D:paths.rs:1-4, :8, :19-44). An old build entering a directory writes store.meta if absent (fs_store:213-221) and
refuses init over an existing vault (M vault/mod.rs:706 vault_exists).
 L1 LEAF. The successor's store directory leaf is LEAF = "qsc-" + TAG, where TAG is one lowercase ASCII token
    [a-z0-9]{1,16} allocated with row 2 (O2) in APPENDIX A row A19. LEAF is never "qsc".
 L2 CLI RESOLUTION. SUCCESSOR_OVERRIDE (A20), else XDG_CONFIG_HOME/LEAF, else <HOME>/.config/LEAF. The successor
    NEVER resolves a location from QSC_CONFIG_DIR. If QSC_CONFIG_DIR is set and SUCCESSOR_OVERRIDE is not, it emits
    one marker (legacy_config_override_ignored, PROPOSED) and continues at its own location: announced, not silent.
 L3 DESKTOP. <app data dir>/LEAF (sibling of today's <app data dir>/qsc); the successor desktop sets
    SUCCESSOR_OVERRIDE, not QSC_CONFIG_DIR. settings.json and webview/ stay where they are (CENSUS D26).
    F13 NOTE: the successor desktop sets its own location variable (SUCCESSOR_OVERRIDE), pinned by a test at F13;
    without it the successor desktop and the successor CLI would share XDG_CONFIG_HOME/LEAF.
 L4 NEW BUILD FINDS OLD FILES. (a) In its own directory: at open, before the attempt guard, before any KDF and
    before any write, the successor reads store.meta and requires store_version=2 and profile=<row 2>; a missing,
    older or foreign store.meta, a KnownOld vault magic, or any RETIRE-class entry of the CENSUS (qsp_sessions/,
    msgqueue_v1/, handshake_pending_*.json) -> refuse successor_dir_foreign (PROPOSED code); nothing counted,
    delayed, written, migrated or deleted. PLACEMENT: the store.meta read-back runs BEFORE protection_state_load /
    ensure_store_layout and BEFORE lock creation, the lock file being the one permitted entry (as L6); or
    ensure_store_layout is read-verify-only on the successor path. Today the guard calls protection_state_load
    first (protection.rs:150), which writes store.meta if absent (ensure_store_layout :469 -> fs_store:219) and
    creates .qsc.lock (lock_store_shared :474 -> fs_store:418-426) before the passphrase test (:158); kept, that
    shape would write a successor store.meta and then pass its own check on it.
    (b) The old default directory: never opened, never written, never
    migrated, never deleted. A read-only existence test for a one-line notice is permitted (as the desktop already
    does for the CLI directory, D:paths.rs:70-73, "never written").
 L5 OLD BUILD FINDS NEW FILES. By default it never does (it resolves only qsc/ locations). Through an explicit
    QSC_CONFIG_DIR pointing at a successor directory (this reaches the OLD CLI only): it writes no data file
    (store.meta exists, so it writes none; it still creates .qsc.lock and a transient, removed probe.tmp.<pid>,
    fs_store:384, :414, :432); the vault magic QSCV04 is
    Unknown -> vault_parse_failed (M/PIN vault_format.rs:46); init refuses vault_exists (M :706). On the CLI nothing
    is counted (unlock_guarded has no CLI caller). On an OLD DESKTOP the guard counts and, if armed, can wipe
    (T2 C). The old desktop sets QSC_CONFIG_DIR itself (D:lib.rs:452), so an exported QSC_CONFIG_DIR never reaches
    it: it reaches a successor directory ONLY through QSLD_DATA_DIR plus a directory literally named "qsc" holding
    successor files (rename, copy or symlink; D:paths.rs:20-24, :38-40).
    This cannot be fixed in shipped builds; L1-L3 make it require an explicit operator override. Named in
    the claim boundary, not hidden.
 L6 INIT. The successor initialises only in an empty successor directory (the rule C already has at vault/mod.rs:
    803-807, directional_fresh_vault_required), writing store.meta (L4) and the vault in that order.

==============================================================================================================
T2. COMPATIBILITY / REFUSAL MATRIX  (behaviour = PROPOSED; "today" = measured; PIN = the desktop's qsc)
==============================================================================================================
| Case | Proposed behaviour and distinct error | Refuses before effects? | D02 check |
|---|---|---|---|
| A. QSCV01/QSCV02 vault x successor build | vault_version_unsupported at the magic (KnownOld arm) | CLI: YES (before KDF, writes, counter). Desktop AS SHIPPED (PIN: Current=QSCV02, KnownOld={QSCV01}): QSCV01 gated at D:commands.rs:323, never reaches the guard; QSCV02 is Current there, only a wrong passphrase counts. A successor desktop: gated (KnownOld) -- AND O7 must hold, because the gate covers only what the magic carries | Holds on the CLI; desktop acceptance PRECONDITIONED on O7 |
| B. QSCV03 vault (C era) x successor build | KnownOld -> vault_version_unsupported at the magic | Successor CLI and successor desktop: YES (gated). TODAY, desktop as shipped: QSCV03 is UNKNOWN at PIN (vault_format.rs:18 KnownOld=QSCV01 only) -> not gated -> unlock_guarded (D:commands.rs:328) -> vault_parse_failed (PIN :46) -> COUNTED (protection.rs:178-180) -> WIPED at an armed limit (:181-183) with the correct passphrase. (Round 1's "magic Current, refused after KDF+decrypt" described a C-pinned desktop that does not exist) | Successor: holds. Today's shipped desktop: FAILS when armed |
| C. Successor vault x old build (M CLI, desktop 15818498/PIN, C) | Unknown magic -> vault_parse_failed (M/PIN vault_format.rs:46): refused, not distinct from corruption, not fixable in shipped builds | CLI: YES (no CLI caller of unlock_guarded; vault unlock uses unlock_with_passphrase). Old desktop: Unknown is not KnownOld (D:commands.rs:40-41) -> guard -> COUNTED -> WIPE if armed. Reachable only if the old build is pointed at the successor directory (O8 L5) | Holds for reads; residual wipe risk only through an explicit override (O8 L5) |
| C2. Successor build x a directory holding old files | successor_dir_foreign at open (O8 L4) | YES: before the guard, KDF and any write | Holds (no migration, no reset, nothing deleted) |
| D. Peer: successor <-> M | M receiving successor A1: REJECT_QSC_HS_UNKNOWN_CRITICAL (M handshake:332-343). Successor receiving M's A1 (no 0x7f80): REJECT_QSC_HS_INTEGRATION_REQUIRED (:347); QHSM v1: same (:476). M's QSE 01 00 frames: MAGIC non-admission. Two mechanisms: PARSE on A1 (:278-379); byte EQUALITY of the stored block on B1/confirm (hs_contexts_match :1308-1315 at :2014, :2297; admission off :650-651 -> :490-498). C02 keeps both | YES: decode precedes hs_pending_store on the responder (:2512 < :2671); refused A1 -> continue (:2721, :2768). ACK: Relay source acks nothing on a refusal; Provided source acks Consumed/AlreadyComplete (:2749-2754); disposition -> C05 | Holds |
| E. Peer: successor <-> C (-0n) | REJECT_QSC_HS_INTEGRATION_PROFILE both ways (value mismatch :337); on B1/confirm by equality (as D) | YES (pure decode) | Holds |
| F. Unknown wire kind | NDE1 kind > 1 -> TYPE (core:319-320); core typed > 2 -> TYPED (:111); NDI2 kind 6..255 -> INTEGRATION_KIND (delivery:345); wrong NDE1 magic -> MAGIC (core:314); wrong NDI magic -> INTEGRATION_PROFILE today (delivery:390; distinct code OPEN, O9) | YES: staged clone, no save (protocol_state:1323 < :1327), no receipt, relay item not ACKed and retained (transport:495, :520-522) | Holds; retained redelivery is I07 (C05/F11) |
| G. Unknown local schema version | Vault payload != 5 -> vault_version_unsupported; mode unknown -> vault_mode_unsupported; Transaction.version other -> TRANSACTION_PROFILE (delivery:471); foreign-profile intent -> APPLICATION_ID_CONFLICT today (delivery:329; distinct code OPEN, O9); queue record != 2 -> PROPOSED exact refuse; store.meta -> successor_dir_foreign; owed_receipts/dedup: O6 | Vault/Transaction/intent: YES. owed_receipts: NO today (parse error -> empty, next save overwrites, owed_receipts:85, :158-159). dedup: NO today (reset, dedup:69-87). Disposition.response_pending defaults TRUE (delivery:223, :229-230) (F04) | owed_receipts, dedup: silent resets today (FAIL if kept on the successor path; O6) |
| H. Reserved file kind (NDI2 1-4) received | INTEGRATION_FILE_GATED, refused on the KIND BYTE before any payload parse (today validate_typed_payload :415 runs directional_file_shape store:311-377 first, so a malformed kind-2 body yields INTEGRATION_FILE_SHAPE before the gate at delivery:420) | YES: no durable write, no receipt, no ACK | Holds; no placeholder format |
| I. Reserved file kind: local send / options | file send -> directional_attachments_unsupported before unlock, read or network (attachments:1622-1623 -> lib.rs:2215-2216); receive file options refused at entry (lib.rs:2190-2193) | YES | Holds |
| W1. A1 with 0x7f80 stripped | REJECT_QSC_HS_INTEGRATION_REQUIRED (:347) | YES (pure) | Holds |
| W2. 0x7f80 duplicated | REJECT_QSC_HS_DUPLICATE_PARAMETER (:308-311) | YES | Holds |
| W3. 0x7f80 sent non-critical, or with another value | REJECT_QSC_HS_INTEGRATION_PROFILE (:336-337) | YES | Holds |
| W4. Parameters reordered | REJECT_QSC_HS_NONCANONICAL_ORDER (:312-313) | YES | Holds |
| W5. Parameter block > 64 bytes or a length field past the end | REJECT_QSC_HS_MALFORMED_LENGTH (:287-288, :298-299, :317-318, :482-483, :486-487) | YES | Holds |
| W6. Handshake frame shorter than 7 bytes | handshake_len (:465-466) | YES | Holds |
| W7. Truncated NDE1 | WIRE_BOUND / PARSE (core Wire::parse :313-320) | YES (staged, no save) | Holds |
| W8. Truncated or over-long NDI2 | INTEGRATION_LENGTH (delivery:382 Reader::take; :388 > 60000) | YES | Holds |
| W9. B1/confirm whose parameter block differs from the pending block | refused by equality (hs_contexts_match :1308-1315 at :2014, :2297) | YES | Holds |
| V1. Magic rewritten to the reader's magic over foreign bytes (X4) | Successor: if the KDF header is old -> vault_parse_failed at the exact KDF check, BEFORE Argon2 (A12). If the KDF fields are ALSO rewritten -> Argon2 runs, AEAD fails (AD binds magic and KDF words, vault/mod.rs:1162-1169) -> "vault_locked" (:1142), indistinguishable from a wrong passphrase | CLI: refused, nothing written. Guard: the second shape COUNTS and cannot be told apart after the AEAD. Named, not fixable by a code | Holds for data (no reinterpretation); the counted case is inherent |
| V2. QSCV04 magic, old KDF header | vault_parse_failed before Argon2 (today's shape :1062-1063); distinct code OPEN (O9) | CLI YES. Guard: must not count (O7) | Holds once O7 holds |
| V3. Mixed profile inside one vault | directional_schema_incompatible (:1968-1972); storage-only holding owner/peer keys -> directional_owner_binding (:1974-1977) | YES (decode) | Holds |
| V4. Old per-contact directories on disk beside a successor store | cannot occur in a successor directory (O8 L4: successor_dir_foreign). Today, same directory: a fresh vault mints a new store key, so a leftover record fails its AEAD and load_contact propagates the error (msgqueue:487): loud, not a version code | YES under O8 | Holds under O8 |
| V5. Desktop destroy door | D:commands.rs:483 gates KnownOld only; for Unknown magic, destroy_with_passphrase refuses via the parser (PIN protection.rs:303) before the passphrase test and without counting. NOTE: not a wipe path | YES | Holds |
| V6. Desktop create door | reachable only from launch state S0 (no vault file) per D:commands.rs:226-233; its unlock_guarded (:235) sees only the vault just created. NOTE | n/a | Holds while the precondition holds |
| V7. Any refusal through the guard, wipe NOT armed (X2c) | TODAY: counter written (protection.rs:178-180) and the delay schedule applies to the next attempt (:151-157, :203-206). PROPOSED: version/format refusals write no counter and impose no delay (O7) | TODAY NO (an effect on a refusal, I03) | FAILS today; holds once O7 holds |
| V8. Short or corrupt file | vault_parse_failed (vault_format.rs:36-39, :46) | CLI YES; guard: must not count (O7) | Holds once O7 holds |

==============================================================================================================
T3. OWNING MODULES (identifier and its refusal)
==============================================================================================================
| Identifier family | At M | At C | Proposed successor owner |
|---|---|---|---|
| Suite tuple | handshake hs_parse_parameter_block; refimpl suite2 parse.rs:150-154 | handshake :278 | unchanged |
| Profile id + refusals | none | delivery INTEGRATION_PROFILE (:9), consumed by handshake :93/:337, vault :127/:1955/:1980, Transaction::decode :464-475, QueuedIntent::decode :326-334, receipt KDF/AD :40-42/:60-63 | ONE const; each consumer refuses with its own distinct code; row-2 change is CRYPTO-TOUCHING |
| Frame classification | frameclass classify | same | unchanged |
| Wire parse / body kinds | M lib.rs qsp_unpack :2260 (+ refimpl) | core Wire::parse :313; delivery typed_body_decode :378-417, body_decode :418-421; store directional_file_shape :311-377 | core + delivery; file-kind gate on the kind byte in body decode (no shape parse first) |
| Receive disposition of a refusal | transport receive loop (skip marker) | transport :488-525; protocol_state expected_non_admission :1272-1302 | unchanged here; ACK rules are C05 |
| Vault magic | adversarial/vault_format.rs classify_vault_magic :15-23 | same | same function, moved out of `adversarial` -- PROPOSED |
| Vault attempt guard | vault/protection.rs unlock_guarded_at :145-207 | byte-identical | MUST distinguish version/format refusals from passphrase failure (O7 PRECONDITION); desktop pre-flight becomes defence in depth |
| Vault payload / namespaces / mode | vault decrypt_payload | vault decrypt_payload :1114-1146 -> check_directional_aggregate :1961-1982 (called INSIDE decrypt_payload at :1144); protocol_state approved_directional_layout :1401-1406 | unchanged owners; `mode` checked in check_directional_aggregate's successor |
| Store directory and foreign-store refusal | fs_store config_dir :10-30, ensure_store_layout :213-221 | same | fs_store: successor resolver (O8 L2) and store.meta read-back (O8 L4) |
| Queue record | msgqueue read_record | msgqueue read_record :445-467; directional_packed_decode :2139 | msgqueue read_record gains the exact version check |
| File send refusal | attachments file_send_execute (proceeds) | attachments :1622 -> lib.rs :2215 | unchanged |

==============================================================================================================
T4. RESERVED UNSUPPORTED FILE KINDS (C06 boundary)
==============================================================================================================
| Reserved value | Meaning (C) | Status in the successor until C06 is ACCEPTED |
|---|---|---|
| NDI2 body kind 1 | attachment descriptor | RESERVED, refused INTEGRATION_FILE_GATED |
| NDI2 body kind 2 | file chunk | RESERVED, refused |
| NDI2 body kind 3 | file manifest | RESERVED, refused |
| NDI2 body kind 4 | file confirmation (DirectionalConfirmation{handle,content_len}) | RESERVED, refused |
| Core typed kind 1 | reserved (delivery refuses INTEGRATION_KIND) | RESERVED, refused |
| Binary file queue row tag (THE PLAN sec 6) | does not exist | RESERVED NAME ONLY; no tag byte allocated |
| NIF payload format / version ("NIF1") | does not exist (0 hits in qsc at M and C) | NOT ALLOCATED; C06 allocates |
| attachments/ staging and attachments.json | CENSUS D21, V16 | RESERVED to C06; not written while file send is refused |
Rules: (1) no placeholder format: the successor defines NO payload syntax for kinds 1-4; the refusal is on the kind
byte, so the legacy JSON file shape (store:311-377) is not pinned as an accepted syntax. (2) Refusal precedes
durable effects, receipts and ACK (T2 H). (3) The mapping of THE PLAN's three kinds (manifest, chunk, completion)
onto 1-4, and whether kind 1 (QATT descriptor) survives, is C06's (O10). (4) Legacy JSON kinds file_chunk /
file_manifest / file_confirmed / attachment_descriptor v1, their versions (adversarial/payload.rs:3-4, :7) and
PendingReceipt::FileComplete (M) are RETIRED with row 10; DOC-CAN-005/006/007 are unchanged by C01.

==============================================================================================================
T5. K-08 ABSORPTION
==============================================================================================================
| Item | What C01 fixes (PROPOSED) | What stays OPEN |
|---|---|---|
| KDF raise (ROADMAP item 5 "versioned KDF changes") | Rides QSCV04 (row 13) and row 2; no later format-version lane. Parameters in the AEAD-authenticated header, exact-match, checked before any Argon2 run (I06: an attacker cannot make the reader allocate more than the successor constant). MEASURED; VALUE m=262144 KiB, t=3, p=1 per RULING round2 E5+E6 (row 14; O4_KDF_MEASUREMENT.md), at all three Argon2 call sites (init, unlock, provider). Old parameters are never accepted under QSCV04 | O4c: the per-ACTION budget. The value is conditional on a held derived key (F05) because of N1; the measurement is one box (see its claim boundary). "Key rather than retained passphrase" is F05 |
| Label split (NA-0778 L5 census F1-F6 filename sites, N1-N9 non-filename sites, 0 wire; session-blob AAD binds the label, N5) | The successor persists NO user-typed name in a filename, directory name, vault key suffix, AAD, OR PLAINTEXT FILE CONTENT outside the vault: every per-contact artifact is keyed by an opaque contact id (rows 17, 19; CENSUS V2, V17, V18, D15). Named additionally by this revision: handshake.pending.<self>.<peer> (V18); the legacy handshake_pending_<self>_<peer>.json (D16, RETIRE); outbox.json `to`/`channel` (D10); identities/self_<label>.json and identity.*_sk.<label> (D15, V17). E4 (ruling): the L5 spec's S5 migration is NOT carried; the fresh profile and the distinct location (O8) make it unnecessary and D02 forbids it | Contact-id format (L5 S1 proposes 16 CSPRNG bytes as 32 hex) and the CLI name-to-id resolver (L5 S4) (O5); facade/DTO carriage is C05. The 32-hex id passes channel_label_ok (lib.rs:1460-1465) and directional_single_channel (:2093-2098) |

==============================================================================================================
T6. OPEN CELLS (each blocks the named cards; all decisions above are PROPOSED)
==============================================================================================================
| Id | State | Open question | Blocks |
|---|---|---|---|
| O1 | RESOLVED (ruling) | Allocation by the new DOC-CAN-003 table (APPENDIX A), recorded by the adding PR's D-record. Rows PROPOSED until that PR | -- (the table's own acceptance blocks F03) |
| O2 | OPEN | Exact row-2 string, and TAG (O8 L1) which rides it | F03 (successor-vault and real-pair helpers), C02 vectors |
| O3 | RESOLVED (ruling) | Storage-only = `mode` field (row 16, A14) | -- |
| O4 | RESOLVED IN METHOD (ruling); MEASURED; VALUE per RULING round2 E5+E6 | m=262144 KiB t=3 p=1 (row 14), at all three Argon2 call sites (init, unlock, provider); the operator's laptop is measured before F13 acceptance. O4c OPEN: the per-action unlock budget and the target machine class (K-06); the value is conditional on F05's held key | F03 successor-vault fixtures, F04, F05 |
| O5 | OPEN | Contact-id format, resolver, namespace strings (rows 17, 19; CENSUS V2, V7, V17, V18) | F03, F04, C04 owner schema |
| O6 | OPEN | Which stores stay on the successor path (CENSUS class O6: D6, D9, D10, D15, D16, D19, D20, D29, V4, V5, V7-V10, V14). MUST DECIDE EXPLICITLY UNDER D02 (Q4): identity's lazy migration (identity:348, :459) and legacy reinterpreting reader (:484-489); the handshake-pending file migration (handshake:1250-1270). Proposal: all three removed from the successor path | F04 |
| O7 | PRECONDITION (ruling) | The engine attempt guard must not COUNT, DELAY or WIPE on any version or format refusal (vault_version_unsupported, vault_parse_failed incl. KDF-header mismatch, vault_mode_unsupported, directional_* schema refusals, successor_dir_foreign). Only a passphrase-authentication failure counts; V1's post-AEAD case is inherently counted. RED-at-main regression named in the checks (row "O7") | F03 (regression), F13 (desktop acceptance of T2 A-C), F18 |
| O8 | RESOLVED (ruling) | Rule L1-L6 above; spellings TAG and SUCCESSOR_OVERRIDE ride O2 (A19, A20) | -- |
| O9 | OPEN | Distinct error codes where today's are misleading: NDI magic -> INTEGRATION_PROFILE; foreign-profile intent -> APPLICATION_ID_CONFLICT; QSCV04 with an old KDF header -> vault_parse_failed (V2); old build -> vault_parse_failed is unfixable. New PROPOSED codes in this revision (vault_mode_unsupported, directional_mode_storage_only, successor_dir_foreign, legacy_config_override_ignored) are registered through DOC-SCL-002 by the implementing PR | F03 (assertions), F12 |
| O10 | OPEN | File-kind mapping; whether kind 1 survives | F14 (C06) |
| O11 | OPEN | Whether C02's A1 change bumps QHSM v2 (row 4) | F10 (C02) |
| O12 | OPEN | Reconciling C's own records before F03 cites them (C DOC-CAN-003 :653-685 says -01/NDI1/schema 3 and, at :669, transaction key na0780_directional_transaction/{peer}; code says -03/NDI2/version 4 and na0780_directional_transaction_v2/ (protocol_state:1404); README carries three generations). Under O1 the appended section becomes RETIRED rows of APPENDIX A | F03 |
| O13 | OPEN (RULING round2; AMENDMENT A4) | C07 LINEAGE FIELDS: payload version 5's field list (row 15, A13) is NOT frozen until F02 settles the C07 vault fields (vault_id, protection_mode, generation, predecessor anchor, checkpoint MAC key, TPM enrollment record). C01 reserves the dimension: protection_mode beside `mode` as a separate discriminator (row 16, A14); the init selector's second axis (row 18, A16); reserve-only C07 rows for the prepared-successor slot, the freshness checkpoint location and the per-lineage lock (CENSUS D30-D32, A24). No successor vault is written outside tests before F02 settles them | F03 production successor-vault writes (not F03 test fixtures), F04, F05 |

==============================================================================================================
DISCRIMINATING CHECKS (named, not written, not run). Delta symbol per SR-19; reachability by source reading only.
==============================================================================================================
| Cell | Test (existing file:line, or F03-TO-WRITE) | Delta symbol | Reachability (source read) |
|---|---|---|---|
| A (CLI) | C T:na0694_vault_envelope_aad.rs:281 qscv01_vault_refused_with_distinct_error (:295, :305) | classify_vault_magic KnownOld arm, vault_format.rs:18 | CLI unlock -> parse_envelope (vault:1056) -> parse_vault_envelope (:43-47) |
| A/B (strings) | F03-TO-WRITE: QSCV02 and QSCV03 refused with vault_version_unsupported (T:vault.rs:512 checks failure only) | same arm, extended to QSCV03 | same path |
| O7 (guard; RED at main) | F03-TO-WRITE: with attempt_limit armed at 1, unlock_guarded on (i) a KnownOld-magic vault, (ii) an Unknown-magic vault, (iii) the build's magic with a mismatched KDF header, (iv) a payload-version refusal -> vault_unlock_failures.txt bytes unchanged, no Delayed outcome on the next call, vault bytes unchanged. RED at M 3748dbfe and at C by source reading (protection.rs:158 collapses every Err; :178-183 count and wipe) | unlock_guarded_at failure branch, protection.rs:158/:178 | unlock_guarded :138 -> unlock_guarded_at :145 -> authenticate_with_passphrase (vault:261) -> open_session :444 -> decrypt_payload :1114 -> check_directional_aggregate :1144. Test must set QSC_CONFIG_DIR (or the successor override) and arm explicitly |
| B | C T:na0780_directional_integration.rs:2594 r02_real_layout_refusals [feature na0780-test-hooks] (a helper; its #[test] caller not traced) | check_directional_aggregate :1964, :1981 | open_session_with_passphrase :459 -> decrypt_payload -> :1144 |
| B (unit) | C src vault/mod.rs:2193 r02_fresh_discriminator_and_owner_are_one_payload (:2212-2219) | same | direct |
| C | F03-TO-WRITE: old-binary fixture (pinned by sha) opens a successor vault -> refusal, bytes unchanged | successor VAULT_MAGIC | M classify_vault_magic Unknown arm (:46) |
| C2 (O8 L4) | F03-TO-WRITE: successor open over a directory with store_version=1 / QSCV03 / qsp_sessions/ -> successor_dir_foreign; no counter, no write (directory listing and bytes unchanged) | the store.meta read-back (new) | open path before the guard |
| D (M side) | M/C T:na_0313_handshake_suite_id_parameter_block.rs:727 in the harness (:576) | M handshake unknown_critical arm :332-343 | harness -> hs_parse_parameter_block |
| D/E (successor side) | C src handshake/mod.rs:3191 directional_exact_profile_and_suite_are_mandatory (:3195-3218) | :336-338, :347 | direct |
| D (no effects) | F03-TO-WRITE: a refused A1 leaves no handshake.pending secret and, under the relay source, the item un-ACKed | responder ordering hs_decode_init :2512 before hs_pending_store :2671; Err arm :2721 -> :2768 | receive -> hs_decode_init -> hs_parse_parameter_block |
| W1-W5 | EXISTING at C: handshake unit :3196 (strip), :3199-3206 (value, non-critical), :3210-3211 (duplicate), :3215-3217 (reorder), :3218 (truncated block; asserts is_err only -> F03 pins the exact code) | hs_parse_parameter_block arms :287-375 | direct |
| W6 | F03-TO-WRITE: 6-byte frame -> handshake_len | :465-466 | hs_decode_header |
| W7/W8 | F03-TO-WRITE: truncated NDE1 and NDI2 -> exact codes, durable state unchanged | core :313-320; delivery :382, :388 | transport :488-525 -> directional_receive_update |
| W9 | F03-TO-WRITE: B1 whose block differs by one byte -> refused, no session stored | hs_contexts_match :1308-1315 | :2014 (resp), :2297 (confirm) |
| F | C T:na0780_directional_integration.rs:1841-1886, :1998-2039 (state unchanged :2011, no output :2012, not ACKed :2036-2039), run by :2092-2094 [feature] | expected_non_admission protocol_state:1272-1302; `_ => INTEGRATION_KIND` delivery:345 | transport :488-525 -> directional_receive_update |
| G (vault) | as B | as B | as B |
| G (mode) | F03-TO-WRITE: payload with unknown/absent `mode` -> vault_mode_unsupported before any write | successor aggregate check | decrypt_payload -> aggregate |
| G (queue record) | F03/F04-TO-WRITE: record with v != 2 refused (RED today: v never checked) | msgqueue read_record (:445), the check to be added | send/receive -> load_contact :470 -> read_record :487 |
| G (owed_receipts) | F04-TO-WRITE: unparseable owed_receipts refuses and leaves bytes (RED today) | owed_receipts load, :85 | receive -> owed_receipts::load |
| G (nested duplicates) | F04-TO-WRITE: a duplicated key inside CapacityOwner.peers (and each nested map in row 15) refused | new detectors | decrypt_payload -> directional_owner :1952-1960 |
| H | C src store/mod.rs:427 file_execution_stays_gated_after_codec_validation (delivery test_file_body :1066-1072); C T:na0780_directional_integration.rs:2003-2039 | body_decode kind arm, delivery:420 | receive_inner (delivery:722, call :795) -> body_decode |
| H (kind-first) | F03-TO-WRITE: malformed kind-2 payload yields INTEGRATION_FILE_GATED, not INTEGRATION_FILE_SHAPE | ordering validate_typed_payload (:415 -> :342) vs body_decode (:420) | same |
| I | C src lib.rs:2286 receive_unsupported_options_refuse_at_entry_before_vault_and_output_effects; F03-TO-WRITE for `file send` | attachments:1622-1623 early return | main.rs:472, :484 -> file_send_execute |
| V1/V2/V8 | F03-TO-WRITE: rewritten magic with old KDF header -> vault_parse_failed before Argon2 (PERF_KDF_CALLS vault/mod.rs:1033 unchanged); with rewritten KDF words -> vault_locked | parse_envelope :1062; envelope_header_bytes :1162-1169 | unlock -> parse_envelope -> derive -> decrypt |
Not established here: any test's current pass/fail (nothing built or run from qsc). Suspected stale tests at C (not
run), unchanged from round 1: T:na0780_directional_integration.rs:1276-1281, T:a2_signature_provider_rng_failure.rs:268,
T:b1_signature_provider_rng_failure.rs:241, T:kem_provider_rng_failure.rs:275 (QSCV02), T:na_0313...:677,
T:qsp_qse_onwire.rs:305, T:handshake_mvp.rs:1402, T:na0741/na0742 skip markers. SR-18 applies at F03: every pin of
QSCV03, -03, "directional-v1", payload version 4, msgqueue_v1, "qsc.msgqueue.v1|", msgqueue_store_key_v1 and the
parameter length literal 25 moves with rows 2, 3, 13, 15, 19.

==============================================================================================================
APPENDIX A -- PROPOSED DOC-CAN-003 CANONICAL TABLE (TEXT ONLY; NOT COMMITTED; EVERY VALUE PROPOSED)
==============================================================================================================
Placement: a new section "12. Successor identifier allocation (normative table)" before "End of DOC-CAN-003".
It supersedes C's appended "NA-0780 first-release directional profile reservation" (C :653-685), whose rows appear
below as RETIRED. Allocation takes effect only when the PR that adds this section merges; that PR's D-record
records the allocation (ruling O1). Goals line and governance updates per DOC-CAN-003 sec 11 (M :641-647).

| Row | Namespace | Exact allocation | Refusal when absent / different | Status |
|---|---|---|---|---|
| A01 | Suite tuple | (protocol_version 0x0500, suite_id 0x0002), sec 1.1 | REJECT_QSC_HS_SUITE_UNSUPPORTED / _DOWNGRADE (sec 1.2) | EXISTING, cited |
| A02 | Successor profile | ONE ASCII string, bytes 0x21-0x7E except 0x3D '=' (no whitespace: 0x20 and control bytes lie outside the range), length 1..50; not equal to NA0780-DIR-INTEGRATION-01, -02, -03 or NA0780-OWNER-FREE-01. VALUE: OPEN (O2). Consumers: A03 value; A13 `protocol`; delivery Transaction.version; QueuedIntent.profile; A17 packed marker; A18 marker; receipt-key KDF binding and receipt AD (CRYPTO INPUT) | per consumer: REJECT_QSC_HS_INTEGRATION_PROFILE; vault_version_unsupported; TRANSACTION_PROFILE; intent code (O9); successor_dir_foreign | PROPOSED; value OPEN |
| A03 | Handshake critical parameter | id 0x7f80, flag 0x01, value length = len(A02), value = A02; exactly once; in canonical order with 0x0001 | missing -> REJECT_QSC_HS_INTEGRATION_REQUIRED; value/flag -> REJECT_QSC_HS_INTEGRATION_PROFILE; repeated -> REJECT_QSC_HS_DUPLICATE_PARAMETER; order -> REJECT_QSC_HS_NONCANONICAL_ORDER; length -> REJECT_QSC_HS_MALFORMED_LENGTH | PROPOSED |
| A04 | QHSM version | 2; version 1 refused. Bump question OPEN (O11, C02). C02 owns invite/handshake labels QSL.invite.payload.v1, QSL.invite.identity-commitment.v1, QSC.HS.SID, QSC.HS.ROOT.COMBINE.v1 | v1 -> REJECT_QSC_HS_INTEGRATION_REQUIRED | PROPOSED |
| A05 | Core KDF profile | NA0780-DIR-EPOCH-CORE-01 (unchanged bytes) | n/a (label) | PROPOSED KEEP |
| A06 | KMAC customization prefix | NA0780.DE1/ (unchanged bytes) | n/a (label) | PROPOSED KEEP |
| A07 | Directional epoch frame | NDE1; kind 0 ordinary, 1 boundary | magic -> MAGIC; kind > 1 -> TYPE | PROPOSED KEEP |
| A08 | Core typed kind | 0 body, 1 reserved, 2 target advertisement | > 2 -> TYPED; 1 -> INTEGRATION_KIND | PROPOSED KEEP |
| A09 | Exact-wire receipt | NDR1, 113 bytes | per receipt parser | PROPOSED KEEP |
| A10 | Inner typed body | NDI2; kind 0 application, 1-4 reserved file kinds, 5 maintenance, 6-255 unassigned | 1-4 -> INTEGRATION_FILE_GATED (on the kind byte); 6-255 -> INTEGRATION_KIND; magic -> code OPEN (O9) | PROPOSED |
| A11 | Vault envelope magic | QSCV04; recognised-old QSCV01, QSCV02, QSCV03 | old -> vault_version_unsupported; other -> vault_parse_failed | PROPOSED |
| A12 | Vault KDF | Argon2id, version 0x13, m = 262144 KiB, t = 3, p = 1, output 32 bytes, salt 16 bytes; exact match before derivation. Conditional on a held derived key (O4c, F05). Value per RULING round2 E5+E6; applies at all three Argon2 call sites (init, unlock, provider) | mismatch -> vault_parse_failed today; distinct code OPEN (O9) | PROPOSED (measured) |
| A13 | Vault payload | version 5; fields {version, protocol, mode, secrets} plus the RESERVED C07 lineage fields (O13); the field list is RESERVED, NOT FROZEN until F02 settles them, and no successor vault is written outside tests before then; protocol = A02; duplicate keys refused at every map level | vault_version_unsupported; parse -> vault_parse_failed | PROPOSED; field list RESERVED (O13) |
| A14 | Vault mode | field `mode`: "messaging" or "storage-only"; fixed at init; local only, never on the wire. RESERVED beside it: field name protection_mode, a separate discriminator for the C07 protection profile (O13) | vault_mode_unsupported; storage-only with owner/peer keys -> directional_owner_binding; storage-only establishing a session -> directional_mode_storage_only | PROPOSED |
| A15 | Vault owner/peer namespaces | owner key and peer prefix strings OPEN (O5); peer suffix = contact id | directional_schema_incompatible | PROPOSED; strings OPEN |
| A16 | Init selector | --mode messaging / --mode storage-only; the second axis (C07 protection profile at fresh-identity creation) RESERVED, no spelling (O13) | absent/unknown -> refuse before any write | PROPOSED |
| A17 | Queue family | directory msgqueue_v2; per-contact subdirectory = contact id; record version 2 (checked); record AAD label qsc.msgqueue.v2; store-key secret msgqueue_store_key_v2; packed marker protocol = A02 | record version -> refuse (code OPEN, O9); AEAD -> msgqueue_record_tampered | PROPOSED |
| A18 | Store marker | store.meta lines store_version=2 and profile=<A02>; read at open | successor_dir_foreign | PROPOSED |
| A19 | Store directory leaf | "qsc-" + TAG; TAG = [a-z0-9]{1,16}, allocated with A02 (O2); never "qsc" | n/a (location) | PROPOSED rule; TAG OPEN |
| A20 | Store location override | environment variable "QSC_" + uppercase(TAG) + "_CONFIG_DIR"; QSC_CONFIG_DIR is not honoured by a successor build | legacy_config_override_ignored (marker, not a refusal) | PROPOSED rule; TAG OPEN |
| A21 | RETIRED (refused, never reused) | profiles NA0780-DIR-INTEGRATION-01, -02, -03, NA0780-OWNER-FREE-01; NDI1; QSCV01, QSCV02, QSCV03; payload versions 1-4; selector and marker spelling directional-v1, owner-free-v1; QSE envelope 0x0100 and the legacy Suite-2 message path; QSSV01 session store; legacy payload versions FILE_XFER_VERSION 1, ATTACHMENT_DESCRIPTOR_VERSION 1, CTRL_VERSION_MAX 2; the C appended reservation rows (C :663-671) | as A02-A13 | PROPOSED |
| A22 | Reserved file formats | NDI2 kinds 1-4 (reserved, refused); binary file queue row tag (name reserved, no byte); NIF format (not allocated); allocation by C06 only | INTEGRATION_FILE_GATED | PROPOSED |
| A23 | Protection-file format line | vault_security.txt and vault_unlock_failures.txt (CENSUS D7, D8): the FIRST line is exactly protection_version=2, split once at the first '='; the key=value lines after it keep today's exact key set (attempt_limit; failed_unlocks, last_failure_unix_s). An absent (today's unversioned) or any other format line -> refuse | refuse, fail closed, nothing counted, delayed or written (O7); today's unreadable shape is vault_attempt_limit_io (protection.rs:150); distinct code OPEN (O9) | PROPOSED |
| A24 | C07 reserved dimensions | RESERVED, no bytes and no names allocated: the prepared-successor vault slot; the freshness checkpoint location (outside the store directory); the per-lineage lock; the C07 lineage fields of A13, protection_mode (A14) and the selector's second axis (A16). Allocation by C07 / F02 only (O13) | n/a | RESERVED |

END OF C01 FINAL

==============================================================================================================
AMENDMENTS (appended by D-1427; the text above is C01 FINAL as merged by D-1426 and is NOT rewritten)
==============================================================================================================
Ruled by RULING_NA0783_C02_ACCEPT_2026-09-23 (sha256 d3f37903f8bacbcb167afe8bd167a74f303a06b45a658023cbe95661f5d5d795),
items E2, E3 and E4 of the C02 acceptance (docs/ops/contracts/C02_invitation_authentication.md). Each amendment names
the cell it amends; where an amendment and the text above disagree, the amendment governs. Citations in this section
are at M = qsl-protocol main 87aec475 unless "C" (#1831 ffc8fc52) is named.
| Id | Amends | Amendment |
|---|---|---|
| AM-1 (C02 E2) | T2 rows D and E: the refusal-code cells | After C02's layout change an old peer refuses EARLIER than rows D/E state; the codes, per old-peer path. An old peer (M, or C) RECEIVING successor frames: on the invite path the QSLH v2 envelope reaches its QSLH-1 decoder, whose first-byte check (0x51 != ENVELOPE_VER 0x01) returns handshake_envelope_version_newer (invite:503-507 for the A1 envelope, :774-777 for the B1 response; the same lines at C); on a bare path a QHSM v3 frame returns handshake_version (handshake:500; C :501) before any parameter-block parse. The successor RECEIVING an old peer: a QSLH-1 envelope -> handshake_envelope_version_retired (C02 T2 step 2a); a bare QHSM v1 or v2 frame -> REJECT_QSC_HS_INTEGRATION_REQUIRED (C02 T1c). No successor frame reaches the parameter parse of M or C, so rows D/E's UNKNOWN_CRITICAL and INTEGRATION_PROFILE cells no longer describe successor traffic. Refusal before effects is unchanged (M: decode_envelope invite:1530 precedes the provisioning :1536); an OLD inviter leaves the undecodable frame un-acked on its slot (ENG-0346's shape, pre-existing) |
| AM-2 (C02 E3) | Row 4, APPENDIX A04 and T6 O11 | O11 RESOLVED: QHSM version 3; versions 1 and 2 are refused on the successor path (REJECT_QSC_HS_INTEGRATION_REQUIRED); any other version -> handshake_version. Reason, measured: hs_decode_header checks the exact frame length (handshake:485-486) BEFORE it parses the parameter block (:488-490), so a v3-layout frame labelled version 2 would be refused by an old peer with the misleading REJECT_QSC_HS_MALFORMED_LENGTH; labelled 3 it is refused with handshake_version (:500). The v3 frame layouts are C02 T1c. DOC-CAN-003 sec 12.3 carries the amendment row A04-AM1 (value 3) |
| AM-3 (C02 E4) | Row 12 and APPENDIX A04: the label list | "QSC.HS.SID" (handshake:1500-1502 via :754-767; C :1504-1506) is reclassified as a TEST-SEAM label: the label of the RNG-failure test seam, ignored outside that seam's cfg; the session id is 16 OsRng bytes. It is NOT a domain separator. Of row 12's four labels C02 owns three as domain labels (DS_COMMIT, DS_SIG, QSC.HS.ROOT.COMBINE.v1) and classifies the fourth as a test-seam label (C02 T1d). No crypto effect |

END OF C01 AMENDMENTS

==============================================================================================================
AMENDMENT AM-4 (appended by D-1429; the text above, C01 FINAL and AM-1..AM-3, is NOT rewritten)
==============================================================================================================
Ruled by RULING_NA0783_F01_audit_2026-09-24 (sha256 642a9ff82d42936e16c93136fd230dcc11327a7d3d7fc3c0e65c179f4e09e61d), finding
S7.4 of AUDIT_F01_C01_C03_FINDINGS (sha256 512d42751d7a7dc3b26fa68fb100a7bf5c812f646e44fad47adc80bb142001c2). Where the amendment
and the text above disagree, the amendment governs.
| Id | Amends | Amendment |
|---|---|---|
| AM-4 (audit S7.4) | DISCRIMINATING CHECKS row "D (no effects)" | After "under the relay source, the item un-ACKed" read the parenthetical "(today's relay source; the successor disposition is C05's)" |

END OF C01 AMENDMENT AM-4

==============================================================================================================
AMENDMENT AM-5 (appended by D-1439; the text above, C01 FINAL and AM-1..AM-4, is NOT rewritten)
==============================================================================================================
Ruled by `RULING_F03_formalization_2026-09-25.md` (`07d8fae5d31da0e47b10445fdc87784efb4cd93946395f9886bc29e99310de75`) on the operator-approved values RBANK_F03_C01_O2_O5_values_2026-09-25
(ed682274...). Where the amendment and the text above disagree, the amendment governs.
| Id | Amends | Amendment |
|---|---|---|
| AM-5 | T6 O2, T6 O5; T1 rows 2, 3, 17; O8 L1/L2; APPENDIX A A02, A03, A15, A18, A19, A20 | O2 and O5 CLOSED: the values are allocated by DOC-CAN-003 sec 12.9 (rows A02-V1, A03-V1, A15-V1, A18-V1, A19-V1, A20-V1, O5-01, A21-AM1; C07-01-V1). Row 2 stays CRYPTO-TOUCHING: the implementing PR of A02-V1 carries an SR-15 independent sensitive review. The CENSUS V7 resolver home and the rest of O6 stay OPEN (F04) |

END OF C01 AMENDMENT AM-5
