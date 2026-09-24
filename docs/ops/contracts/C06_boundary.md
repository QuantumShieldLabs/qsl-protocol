C06 -- FILES -- BOUNDARY ONLY (PLAN F01). C06 DETAIL IS OPEN UNTIL F14.

PLAN: QSL-solution-plan rev3 d9016e53d2ab46c32b7a7cb060ea70dc79421617e9b054848b464ba7a5518275 (amended A4, A5), card F01,
final sub-assignment. Lane NA-0783; recorded by D-1434. Goals: G4.
Status of every row: PROPOSED (the F01 boundary text for the Director's review; it decides no C06 detail). This is the
ONLY file-contract text F01 writes. It allocates NO identifier, value, byte or format: every file kind, row and format
named below stays exactly as C01 reserved it (DOC-CAN-003 sec 12.1 row A22), and every dimension it names is a C04 row
(sec 12.6). No production placeholder format exists or is created here.
Sources (read-only, fetched bare mirrors): C = qsl-protocol #1831 head ffc8fc52 (the candidate); M = qsl-protocol main
d38b5195 (#1843's merge); D = qsl-desktop main 92cba80a; contracts C01-C05 with their AMENDMENTS at M. File:line is
qsl/qsl-client/qsc/src/<file>:<line> at C unless prefixed M: or D:. Quotations of THE PLAN are verbatim except that its
non-ASCII characters are rendered in ASCII (em dash as --, en dash as -, curly quotes as straight quotes).

==============================================================================================================
B1. WHAT C06 OWNS, WHAT IT DOES NOT, AND THAT ITS DETAIL IS OPEN
==============================================================================================================
C06 OWNS -- THE PLAN, "Contract decisions", row C06, verbatim (docs/ops/PLAN_QSL_successor_rev3.md:51):
  | C06 -- files | Exact binary fields, endian/length rules, NIF/file-row versions and AAD; semantic FileIntent versus
  final manifest; FileJob transitions; nonce ownership; full bulk-spool reservation versus two in-flight slots; durable
  publication intent, ownership and no-replace rules; cancellation/deadline/drain; combined resource target. | F14
  before file serialization, FileJob persistence or enablement |
WHAT F01 FIXES INSTEAD, AND SO C06 DOES NOT OWN -- THE PLAN :54, verbatim: "F01 fixes profile identity, extension
  boundaries, ownership interfaces and reservation dimensions. File kinds remain reserved and unsupported. F14
  settles/reviews the remaining exact file bytes and proves their bounds before any file-format implementation. If an
  unresolved file choice would change an earlier persisted interface, settle that choice earlier; do not invent a
  temporary format or silently allocate a second successor profile." THE PLAN sec 1 (:330): "Finalize exact file bytes
  and persisted FileJob details in C06/F14 before those formats are written or enabled; settle earlier any field that
  affects an earlier persisted interface." F01 (:105, :107): "Define C06 ownership/interface boundary now; reserve its
  unsupported formats." ... "C06 detail and C07 may remain explicitly open. No production "placeholder" format."
| Not C06's | Owner (ACCEPTED text at M) |
|---|---|
| Profile identity; NDE1 / NDI2 / NDR1 framing; the kind bytes and their reservation | C01 rows 7-10, 22, T4; DOC-CAN-003 A07-A10, A22 |
| Reservation DIMENSIONS and their durable owners, incl. R12 (file incoming) and R13 (file outgoing) | C04 T1 (values OPEN, C4-O13) |
| The disposition vocabulary and ACK rules (DISPOSE, DEFER-C, ...) | C05 T1, T1a. C06 detail adds file-kind ROWS in that vocabulary (B6 C6-O10); it does not change it |
| Relay objects, capabilities and namespaces | C03 (C06 adds no relay object; file frames ride the directional channel inside the existing kinds, THE PLAN sec 6) |
| Rollback and durability claims | C07 (A4) |
| Core cipher / KDF / nonce and the receipt format | preserved unchanged (THE PLAN sec 6: "preserve core cipher/KDF/nonce and receipt formats") |
| The attachment service (qsl-attachments; DOC-CAN-006/007) | not a fallback (D05; THE PLAN sec 6 "Do not silently route it to the attachment service"); outside C06 |
C06 DETAIL = every cell of the verbatim row above. Each is OPEN (B6) until F14's design-only subassignment finalizes it
and its review accepts it (THE PLAN F14: "The first, design-only subassignment may start with C06 OPEN and must
finalize/review it. Implementation subassignments require C06 ACCEPTED").

==============================================================================================================
B2. RESERVED UNSUPPORTED FORMATS (C01 T4 / A22), EACH REFUSED BEFORE EFFECTS TODAY
==============================================================================================================
"Today" = measured at C, the only revision carrying the successor path. At M the live file path is the LEGACY one
(M attachments/mod.rs:1797 file_send_execute has no directional preflight; C01 T3), which C01 row 10 / A21 RETIRES.
| # | Reserved value | C01 row | Refusal today (C) | Before effects? | Successor rule |
|---|---|---|---|---|---|
| U1 | NDI2 body kind 1 (C: attachment descriptor) | T4; row 9; A10, A22 | INTEGRATION_FILE_GATED (body_decode directional_delivery:420); a malformed body is refused EARLIER by the legacy shape parse (validate_typed_payload :342 -> store:311 directional_file_shape) with INTEGRATION_FILE_SHAPE / _REQUEST | YES: staged clone, no durable write, no receipt (C01 T2 H). ACK: C leaves it un-ACKed and retained (transport:495, :520-522) | refused on the KIND BYTE before any payload parse (C01 T2 H, T3; F03 writes it); DISPOSE, ACKed after full dispatch (C05 T1a, X02) |
| U2 | NDI2 body kind 2 (C: file chunk) | as U1 | as U1 | as U1 | as U1 |
| U3 | NDI2 body kind 3 (C: file manifest) | as U1 | as U1 | as U1 | as U1 |
| U4 | NDI2 body kind 4 (C: file confirmation) | as U1 | as U1 | as U1 | as U1 |
| U5 | Core typed kind 1 | T4; row 7; A08 | INTEGRATION_KIND (directional_delivery:791-793) | YES (as U1) | unchanged; DISPOSE (C05 T1a) |
| U6 | Binary file queue row tag (THE PLAN sec 6) | T4; row 19; A17, A22 | no byte exists, so there is nothing to refuse; the msgqueue_v2 record version check (A17, F04) refuses any record that is not version 2 | n/a | name reserved, no byte (C06 allocates; B4 ES5) |
| U7 | NIF payload format / version ("NIF1") | T4; A22 | not allocated (0 hits in qsc at M and C, C01 T4) | n/a | C06 allocates (C6-O1, C6-O2) |
| U8 | attachments/{direction}/<id>.cipher staging; attachments.json | T4; CENSUS D21, V16 | never written: `file send` refuses directional_attachments_unsupported at entry (attachments:1622-1623 -> lib.rs:2215), before unlock, file read, staging or network | YES | RESERVED to C06 |
| U9 | Attachment output and its .tmp | CENSUS D25 (OUTPUT, "gated with C06") | never written (the U8 refusal) | YES | gated with C06 |
| U10 | Receive file options (attachment service, max file size / chunks, file-confirm mode) | T2 I | directional_attachments_unsupported at entry (lib.rs:2190-2193) | YES | unchanged |
| U11 | Legacy JSON kinds file_chunk / file_manifest / file_confirmed / attachment_descriptor v1; FILE_XFER_VERSION 1; ATTACHMENT_DESCRIPTOR_VERSION 1; PendingReceipt::FileComplete (M) | T4 rule 4; row 10; A21 | RETIRED on the successor path (no reader, no fallback) | -- | never reused |
NO PRODUCTION PLACEHOLDER FORMAT: the successor defines NO payload syntax for kinds 1-4, NO file queue row, NO NIF
bytes, NO FileJob / StagePreparing / publication-intent schema and NO spool format. The candidate's legacy JSON file shape
(store:311) is not an accepted syntax (C01 T4 rule 1). Nothing in this boundary is a format.

==============================================================================================================
B3. OWNERSHIP AND INTERFACE BOUNDARY
==============================================================================================================
Owners (the durable authority is THE PLAN sec 1's table; module names are proposals, THE PLAN "New module/API names are
proposals"):
| Artifact | Durable owner | Module (C today / PROPOSED) | Reservation (C04 T1) |
|---|---|---|---|
| FileJob, incoming and outgoing (identity, key reference, progress bitmap, in-flight slots, credits, horizon) | the paired capacity owner + peer transaction (THE PLAN sec 1: "Ratchet, exact sealed flights, received dispositions, receipts, liabilities, FileJobs") | protocol_state CapacityOwner (C protocol_state:1413-1419), committed only through the paired commit; schema C06/F14 | R12, R13 |
| StagePreparing ticket (sender, before bulk source I/O) | the same owner | as FileJob | R13 (charge point: the StagePreparing commit) |
| Encrypted snapshot / bulk spool | its authoritative FileJob (THE PLAN sec 1: "Bulk bytes referenced and authenticated by its authoritative FileJob") | a spool module (PROPOSED; THE PLAN F15 "encrypted spool/snapshot module"), inside the successor store directory (C01 O8; C04 X08) | R12, R13 at L4 (disk) |
| Semantic FileIntent (immutable send intent) | the queue row before preparation (THE PLAN sec 1), as the separately tagged binary file row | msgqueue (tag RESERVED, U6) | R08/R09 or a reserved file pool (B4 ES5) |
| Sealed file frames (manifest, chunks, completion) | Flight in the peer Transaction (sealed once; THE PLAN sec 6 sender order) | directional_delivery prepare, via the pure seal-selection interface (PROPOSED, THE PLAN F14) | R04 or R13 slots (B4 ES4) |
| NIF codec for kinds 1-4 | -- | directional_delivery body decode (C01 T3: "file-kind gate on the kind byte in body decode") | -- |
| Publication intent and publication (no-replace) | the owner (committed BEFORE assembly; THE PLAN sec 6 receiver order) | the F16 publication step | R12 (publication temporary, L4) |
| File-kind item dispositions | the dispatcher (C05) | dispatcher (F11); file rows C06 detail (C6-O10) | -- |
| The legacy attachments module | none on the successor path: its JSON path RETIRES (C01 T4 rule 4) | attachments keeps only the entry refusal (U8) | -- |
Interfaces an earlier card (F03-F13) must NOT assume (PROPOSED):
| # | Must not assume | Card | Why / where |
|---|---|---|---|
| NI1 | that NDI2 kinds 1-4 have any payload syntax; the gate reads the kind byte first | F03 | C01 T2 H (kind-first); U1-U4 |
| NI2 | that the owner record, the Transaction (with Flight, Disposition) or the msgqueue_v2 record can grow later without a declared schema step | F04 | B4 ES1, ES2, ES5 (measured: no schema version in any of them) |
| NI3 | that the vault and queue byte-field encoding can be fixed without file flights | F04, F07 | B4 ES3 (C4-O7) |
| NI4 | that R04 / R05 carry only text and controls, or that the per-session bound excludes file flights and the liabilities a cancelled job transfers to the session; the session-close transition (C4-O4) must fail outstanding file work honestly | F06 | B4 ES4; THE PLAN sec 6 |
| NI5 | that every msgqueue_v2 row is a text row, or that Q_S / Q_B are the only queue pools | F04, F07 | B4 ES5; THE PLAN sec 4: "Ordinary queue limits also cannot consume the reserved queue/output slots needed for already owed controls or file completion" |
| NI6 | that every history entry is a text entry | F07, F13 | B4 ES6 |
| NI7 | that INTEGRATION_FILE_GATED is retained or deferred (it is DISPOSE while gated), or that every admitted frame yields an event body (file chunk bytes go to the spool, never to R05 events) | F11 | C05 T1a, X02; THE PLAN sec 6 receiver order |
| NI8 | any environment variable, cfg or cargo feature that enables file execution in a shipping build | F12 | B5; I13 |
| NI9 | that Delivered is the last state of every item (file publication confirmation is a separate state, D07), or any attachment affordance in the GUI | F13 | C05 packet "undrawn: attachments" OPEN; A5 unchanged for text |
| NI10 | any route of file content to the attachment service, a reduced size, or a reduced concurrency | all | D05 |
| NI11 | a relay capacity promise for file frames; the peer's mailbox depth is shared (C4-O9) and v2.max_body_bytes >= 65,536 already admits a full NDE1 (C03 AM-18) | F08-F11 | I05 ("Local funding never implies remote admission") |
C04 reservation dimensions files will use (cited; no new values): R12 and R13 (dimensions only; values C4-O13); R01 (the
FileJob / StagePreparing / publication-intent record bytes, L2, inside B_V); R02-R05 (file flights, dispositions and
transferred liabilities; B4 ES4); R06 (the projection credit of a file message's owned operation); R07 with H_P (one
history unit per file message; C04 AM-1); R08 / R09 (B4 ES5); R10 (the replacement temporaries of every commit); the
disk budget C4-O8 (spool, publication temporary, received outputs), joint with the combined resource target (C6-O9).
Values: C04 T6 L26, HYPOTHESIS.

==============================================================================================================
B4. EARLY-SETTLEMENT LIST (THE PLAN: "settle earlier any field that affects an earlier persisted interface")
==============================================================================================================
Measured at C (step1 source verification): CapacityOwner {generation, peers, entries} is deny_unknown_fields with NO
schema version (protocol_state:1413-1419), as are PeerReserve, OwnerEntry and SessionControlReserve (:1422-1468);
Transaction is deny_unknown_fields and its ONLY version field is the profile string (directional_delivery:240-264; a
different value is TRANSACTION_PROFILE, :470); Flight (:211-222) carries the rule "new persisted field, never
default/backfill from current state" (:219); Disposition (:226-231); TimelineEntry has a `kind` string and defaulted
fields, not deny_unknown_fields (timeline:13-29). C01 row 21: no default-to-empty. THE PLAN :54: no second successor
profile. Therefore a file field added LATE to any of these records either breaks reopening older state or needs a
profile change -- both forbidden. The following are EARLY; each row decides no C06 value and no byte.
| # | Field or choice | Earlier persisted interface it would change | What must be settled early (PROPOSED) | Card that must settle it |
|---|---|---|---|---|
| ES1 | How the owner record admits file state (FileJob, StagePreparing ticket, publication intent, per-job key reference, the persisted horizon) | CapacityOwner (the owner secret in the successor vault, A13) | the owner record's growth rule, fixed before any successor vault is written outside tests (C01 O13): an explicit owner-schema version, refused when unknown (recommended: it defines no file field), OR the file dimensions present from the first write. The FileJob bytes stay C06/F14 | F04 (strict owner decode); serves the same need as C07's A13 field freeze at F02 |
| ES2 | How the Transaction admits file flights and file-kind dispositions (a file Flight's job reference, so a receipt advances the job and a cancelled job's liabilities can transfer; a file-kind Disposition holds a receipt, never an event body) | Transaction / Flight / Disposition (delivery:211-264) | a Transaction schema version independent of the profile string, OR the file fields fixed before the strict decoder is frozen | F04 (strict Transaction decode), with F05 (writers) |
| ES3 | The byte-field encoding with file flights counted (a 16,384-byte all-255 chunk is 65,537 B as a JSON array, THE PLAN sec 6; one maximum wire is <= 262,145 B at L2, C04 L29) | the vault and queue byte fields (C4-O7) | C4-O7 decided with the file load in the budget: <= 2 in-flight chunk flights per job (HYPOTHESIS), one incoming and one outgoing job, at least two sessions (D05) | F04 (vault), F07 (queue row, MAXPACK) |
| ES4 | Whether file flights draw on the session's R04 / R05 or on R13's own slots, and how a cancelled job's committed flights transfer to the session (THE PLAN sec 6: "explicit transfer of remaining protocol liabilities to the session owner") | the per-session reserve R02 and its proof (C4-O2) | F06's session bound states its file-flight term, or F06 records that R13 keeps their credit until retirement so the session bound needs none; values C06 | F06 (with C4-O4 session close, F06 / F16) |
| ES5 | The queue family's room for the binary file row: where its tag sits so the msgqueue_v2 reader refuses it distinctly and never parses it as text; whether file rows count in the pool P or in a reserved file pool (THE PLAN sec 6: "two reusable in-flight protocol/queue slots per job") | msgqueue_v2 (A17: record version 2 checked at reopen) and C04 Q03 admission | the tag position and its refusal (reader); the reserved file pool as a DIMENSION outside ordinary Q_S / Q_B (value C06) | F04 (reader), F07 (admission) |
| ES6 | A history entry for a file (its own states, D07) | the successor TimelineEntry schema | a closed kind set with only text allocated, unknown kinds refused, fixed by the first card that persists a successor history entry | F07 (R07, V708) or, at the latest, F13 (C05 HI1) |
NOT EARLY (each with its reason): the NDI2 kind bytes 1-4 and core typed kind 1 (already reserved, A08, A10, A22; refused
on the kind byte, so nothing earlier parses a file body); the NIF bytes, version, endianness, metadata cap and padding
(inside a kind 1-4 body no earlier card parses); NDR1, core cipher, KDF and nonce (preserved, THE PLAN sec 6); the binary
file row's bytes and AAD label (a new record under ES5's tag; the text row's A17 AAD is not touched and a distinct label
separates them); the spool / snapshot format and nonce ownership (new artifacts no earlier card reads; their names are
RESERVED, C01 D21 / V16, and the store-open rule C01 O8 L4 is a deny-list of RETIRE-class entries, so adding them later
changes no earlier check); FileJob transitions, publication, no-replace, cancellation and drain rules (inside ES1's
record); the file values (inside the ES1-ES5 dimensions; C04 L26 HYPOTHESIS); the relay (no file object; C03
unchanged); the facade DTO (not persisted; a file item is added at F17 without changing text meanings).

==============================================================================================================
B5. FEATURE GATE: FILE EXECUTION OFF THROUGH F14
==============================================================================================================
THE PLAN, verbatim: "File execution stays off through F14." (:93); F14: "File execution remains disabled."; Q5:
"Implement and accept text before enabling file execution."
OFF MEANS, ALL AT ONCE (measured at C): (a) receive -- NDI2 kinds 1-4 refused INTEGRATION_FILE_GATED, no durable write,
no receipt (U1-U4), disposition DISPOSE (C05 T1a); core typed kind 1 INTEGRATION_KIND (U5); (b) send -- `file send`
refused directional_attachments_unsupported at entry (U8); (c) receive options refused at entry (U10); (d) stores -- no
attachments/ staging, attachments.json or attachment output is written, and no FileJob, spool or file row exists (U6,
U8, U9); (e) GUI -- no file affordance (C05 packet "undrawn: attachments" OPEN; D has none: its only attachment mention
is the relay server-info field attachments_service_url, displayed as a diagnostic, D:src-tauri/src/commands.rs:682,
:813).
ENFORCEMENT: the gate is UNCONDITIONAL CODE, not a switch. No cargo feature, cfg or environment variable enables file
execution: qsc's features are default = [], na0780-test-hooks ("never enable in distributed builds") and keychain at C,
default = [] and keychain at M; none is file-related. Lifting the gate is a code change made only by a card whose
prerequisite C06 is ACCEPTED (THE PLAN row C06: "before ... enablement"); fixture-only lifting is test-only and absent
from shipping builds (I13, F12).
TESTS (SPECIFIED, not written, nothing run; ids local to this boundary):
| Id | Card | Case | Expected |
|---|---|---|---|
| GT1 | F03 | each of NDI2 kinds 1-4 with a malformed body, the legacy JSON fixture and random bytes | INTEGRATION_FILE_GATED for all, never INTEGRATION_FILE_SHAPE (kind-first, C01 check "H (kind-first)", F03-TO-WRITE); no durable write, no receipt. Existing at C: store:427 file_execution_stays_gated_after_codec_validation (legacy shape only) |
| GT2 | F03 | `file send` on an unlocked successor store; the receive file options | directional_attachments_unsupported before unlock, read or network; no attachments/ entry (C01 check I, F03-TO-WRITE); options: existing C lib.rs:2286 |
| GT3 | F11 | a pulled batch: text, an NDE1 carrying NDI2 kind 3, text | the kind-3 item DISPOSE, ACKed after full dispatch; no Disposition, no event; both texts committed (C05 T1a, X02) |
| GT4 | F12 | the shipping-profile build of the pinned revision (no test feature) | GT1 and GT2 hold; no feature in the resolved graph enables file execution (I13) |
| GT5 | F13 | the desktop at the F13 qsc pin | no file action is offered and no engine file call exists |
WHEN IT LIFTS: never through F14 (THE PLAN). F15 and F16 build behind the gate with test-only enablement; enabling file
execution for real use belongs to F17's acceptance of the complete inline workflow. The subassignment that removes the
gate from shipping code is OPEN (C6-O12).

==============================================================================================================
B6. OPEN CELLS FOR F14-F17 (each with the cards it blocks). Every cell is OPEN; C06 detail closes them at F14.
==============================================================================================================
| Id | Open cell | Blocks |
|---|---|---|
| C6-O1 | Exact binary fields and endian / length rules of the NIF payload (manifest, chunk, completion); the metadata cap and padding policy from actual serializer maxima; the mapping of THE PLAN's three kinds onto NDI2 1-4 and whether kind 1 survives (C01 O10) | F14, F15, F16 |
| C6-O2 | NIF version; the binary file row's version, tag byte and AAD (format / type / record binding), within ES5's position | F14, F15 |
| C6-O3 | Semantic FileIntent versus the final self-referencing manifest (the reference constructed once in paired preparation; its exact hash retained) | F14, F15 |
| C6-O4 | FileJob and StagePreparing schema and transitions (Ready, Cancelled, Expired, PUBLISHED, READY completion), within ES1 | F14, F15, F16 |
| C6-O5 | Nonce ownership and the key reference of the snapshot / spool (unique nonces; a restarted uncommitted snapshot uses a new job / key) | F14, F15 |
| C6-O6 | Full bulk-spool reservation versus two in-flight slots: the equivalent-bound / progress argument that slot reuse keeps every needed chunk byte | F14, F15, F16 |
| C6-O7 | Durable publication intent, ownership, no-replace and collision rules (an unrelated destination, identical content included, is never overwritten or adopted); restart adoption; the user's destination authorization, separate from wire metadata | F16, F17 |
| C6-O8 | Cancellation, deadline and drain: the discard / receipt drain allowance, late authenticated chunks disposed with ordinary NDR1, sender cancellation's retirement credit, no new outbound file until old file flights retire (with ES4, C4-O4) | F16 (F06 through ES4) |
| C6-O9 | The combined resource target: both jobs, at least two sessions, unrelated text, output temporary, queue replacement, retained controls and cleanup, jointly with the disk budget C4-O8 | F14 (proof), F07 (C4-O8), F17 |
| C6-O10 | File-kind rows in C05's vocabulary: manifest admission DEFER-C when incoming capacity lacks (before any ACK or NDR1); chunk and completion frames never create jobs; late valid chunks DISPOSE durably with ordinary NDR1; exact old manifest replay by its disposition or closed replay floor | F14 (design), F11 (table extension), F16 |
| C6-O11 | The file DTO and GUI: the "File confirmed" state (D07), transfer statuses, the destination choice, the undrawn attachments UI; the operator's acceptance | F17 |
| C6-O12 | Which subassignment removes the gate from shipping code (B5) | F15-F17 |
| C6-O13 | The legacy attachment canon DOC-CAN-005 (descriptor and control plane), DOC-CAN-006 and DOC-CAN-007 (the QATT service; unchanged by C01): its disposition for the successor | F14 |
| C6-O14 | The early-settlement rows ES1-ES6 (B4), each OPEN until its named card settles it | F04, F05, F06, F07, F13 |
THE PLAN's file values ("Numerical requirements versus hypotheses"; C04 T6 L26), each carried as stated, none frozen:
| Value | Status | Blocks |
|---|---|---|
| 16,384-byte chunks | HYPOTHESIS ("Selected file design, to be validated in C06/F14") | F14 |
| at most 256 chunks | HYPOTHESIS (as above) | F14 |
| two reusable in-flight protocol / queue slots per job | HYPOTHESIS (as above; C6-O6) | F14; F06 and F07 through ES4 / ES5 |
| 60,000-byte complete file row | HYPOTHESIS ("Candidate limits to freeze with exact accounting in C04/C06") | F14; F07 through ES5 |
| 24-hour file horizon, from each endpoint's first durable admission, persisted once, never extended | OPEN ("Proposed development policy, OPEN until C06 accepts it") | F14, F16 |
| inline 1..4,194,304 bytes; at least two active sessions; one incoming plus one outgoing transfer | REQUIRED acceptance workload, not a hypothesis ("Do not reduce it to obtain green tests") | F17 |

END OF C06 BOUNDARY
