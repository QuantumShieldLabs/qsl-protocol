C03 -- RELAY AUTHORITY AND RECOVERY -- FINAL (ACCEPTED WITH NAMED FIXES: CONTRACT_ACCEPTED_WITH_NAMED_FIXES)

==============================================================================================================
FIXES (draft -> FINAL), keyed to RULING_NA0783_C03_ACCEPT_2026-09-24 (sha256
bbeb679c87b74978e87a6fb95c034555dbca62eb4ed0516d0da181121496a3fe)
==============================================================================================================
Status: C03 ACCEPTED (contract) WITH NAMED FIXES; result class CONTRACT_ACCEPTED_WITH_NAMED_FIXES. The Director's
ruling, on the SR-15 read (SR15_C03_FINDINGS.md sha256
6f6d8d5712a594605627e9b28af54ff95ba9bad8e31f1f1b1ce16af06aab055a, fable/xhigh, recommendation ACCEPT WITH NAMED FIXES
F1-F8, no BLOCKER; X1 and X2 MAJOR), mandates exactly the fixes F1-F8 and the rulings E1-E5 below; nothing else in the
draft changes. The body below is the draft (C03_DRAFT.md sha256
11889b6fb99ca251310ffec0f4ebd6eb298e17e3afd73f2eae8df33291928c5f) with these applied. The word PROPOSED in the body is
retained verbatim and now reads: the accepted contract value, NOT YET ALLOCATED; identifiers become ALLOCATED only when
the DOC-CAN-003 sec 12 rows for C03 merge, and the label spellings stay PROPOSED / OPEN even then (C3-O1). C3-O1, C3-O2,
C3-O7, C3-O13 and C3-O14 stay the Director's, OPEN. F01 stays ACTIVE (C04, C05, C06 boundary next). Every fix was
verified at the source before it was written (S = qsl-server main 5ea0f925; M = qsl-protocol main a0c0c491; P28 =
#1828 e29a07df).
| Key | Fix | Where | Source verified |
|---|---|---|---|
| F1 (X1) | EP1 open draws a GLOBAL open bucket BEFORE any lookup (429 ERR_RATE_LIMITED); a never-pulled mailbox idles out at the pull lease ceiling, 3600 s; MAILBOX_IDLE_SECS applies only to a mailbox pulled at least once; MAX_V2_MAILBOXES gets its own PROPOSED ceiling; T6 "Open" order rows; T7 states that v1's equivalent surface is not carried into v2; vectors V47-V52 | T3 EP1; T6 Open (EP1); T7 mailboxes_v2 row and the v1 note; T11 C3-O3; T10 | S lib.rs:83-91 (the cap bounds STORAGE, the bucket bounds DENIAL; D614 F6), :354-356 (global bucket), :679-698 (drawn before any parse or storage; ERR_RATE_LIMITED :696), :99, :101 (defaults 32, 1/s); store.rs:8 MAX_PULL_LEASE_SECS_CEILING 3600; lib.rs:116 MAX_ROUTE_COUNT_CEILING 256; v1 surface store.rs:672-693 |
| F2 (X2) | failed-expired applies only to an attempt with NO committed selection or session; on 410/404 for a KNOWN delivery the redeemer first drains its own mailbox and binds any B1 before marking failed-expired; an advanced attempt is never reset or zeroized by the horizon; vector CL9 | T6 client horizons; T10 CL9 | P28 handshake/mod.rs:630-631 ("never overwrite a same-SID session: it may already have advanced via send"), :632-639; C02 T4 X-d, T2 step 3 |
| F3 (X3) | a kind invite_slot mailbox row and its messages are owned by the SLOT sweep and exempt from MAILBOX_IDLE_SECS; the messages_v2 -> mailboxes_v2 FK/orphan rule stated; one TIME vector (V53) | T7 (new row; slot mailbox messages row; FK rule) | S store.rs:212 (foreign_keys ON), :228 (messages REFERENCES routes ON DELETE CASCADE), :339-345 (slots swept on their own clock) |
| F4 (X4) | every missing vector listed, each with delta symbol and base: EP1 exists / KIND_CONFLICT / MAILBOX_CAP / open bucket / idle removal; EP5 CAP_FULL / RATE_LIMITED / KIND_CONFLICT / 413; EP6 wrong cap on a fresh and on a claimed slot, REVOKED for a new claimant, RESULT_QUOTA; EP8 wrong S, 404; EP9 404 and the idempotent second revoke; EP2 LOOKUP_RATE; relay RESTART between claim and exact retry and between delivery and exact retry | T10 "Added by the acceptance" (V47-V66) | v1 precedents S tests/na0678_invite_slots.rs:254, :605, :641, :680; tests/na0678_invite_durability.rs |
| F5 (X5) | T2 v1 capability-header cell corrected: the relay accepts any trimmed non-empty token; the 22..128 [A-Za-z0-9_-] grammar is the client's | T2 table | S lib.rs:997-1009 resolve_route_token; M adversarial/route.rs:21-37 |
| F6 (E3) | C02 AMENDMENT (appended to the C02 contract as AM-1): T2 step 2b route_cap = exactly hex32(D), 64 lowercase hex, refused handshake_envelope_noncanonical; C02 T7 M12 re-expected; C3-O8 closed | T9 OC1; T11 C3-O8; ESCALATIONS E3 | C02 at M T1b row 10, T2 step 2b, T7 M12 |
| F7 | T8 M8: marker read and newer-store refusal decided BEFORE any DDL; DDL and marker advance in one transaction | T8 M8 | S store.rs:214-250 (DDL first today), :251-268 (marker read), :271-273 (refusal), :278-280 (advance) |
| F8 | T4 Compares: P and Q_C listed with the secret-derived values; E stated as a content commitment, not a secret | T4 Compares | T4 digests (P and Q_C carry K / K_slot); S lib.rs:984-995 ct_eq_secret |
| E1 | 65536 is the rig's configured max_body_bytes, not a source fact (source default and ceiling 1048576); the contract needs >= 12288 | T9 OC4; ESCALATIONS E1 | S lib.rs:114, :125, :148; M docs/ops/RIG_PROVISION_RUNBOOK.md:65, :136, :247 |
| E2 | CONFIRMED and stronger: on v1 /v1/push SlotReject::Expired is UNREACHABLE (sweep and admission use one captured now), so an expired invite slot admits ticketless pushes as an ordinary route and DOC-SRV-007:93's promised 410 does not occur; exposure LOW, latent v1 contract defect MAJOR. Filed as ENG-0356 (qsl-protocol docs/ops/IMPROVEMENT_LEDGER.md) with the named one-line repair and the missing push-after-expiry test (repair NOT done here); C02's F2 source note corrected (C02 AMENDMENTS AM-2; its safety claim stands); v2 closure N5/V46 accepted | ESCALATIONS E2 | S lib.rs:1053 (one now), :1058 route_status, store.rs:343-345 (sweep), :633-635 (the unreachable arm), :621-629 and :653-693 (the None arm); main.rs:15, :293-297 |
| E3 | = F6 | as F6 | as F6 |
| E4 | the draft's slot reading CONFIRMED: slot read only with R_slot; the invitation grants redemption plus ONE ticket-gated deposit addressed by L; a push by D_slot runs the same slot_admit; no C02 payload change; V24 pins it | ESCALATIONS E4 | C02 T1a (no slot deposit-key field) |
| E5 | a DRAFT v2 server contract document in qsl-server docs/server, DOC-SRV-008 (number derived), states the four deltas against DOC-SRV-007 by section; DOC-SRV-007 gains one pointer line "v1 only; v2 in DOC-SRV-008" | ESCALATIONS E5 | S docs/server holds DOC-SRV-001..007; DOC-SRV-007:20-23, :42-48, :61, :81-102, :104-112 |
Not taken, named: the SR-15 NOTEs outside F1-F8 (length extension Q2(b), the index-lookup timing Q2(f)(ii), the result
quota Q5(b), the per-K lookup bucket Q5(d), the shared op_id namespace Q2(h), the M1 "read-before-write" wording Q1 #26
beyond M8) -- the ruling names none of them. Also changed, as consequences of acceptance only: the title line, this
FIXES section and the closing END line.

PLAN: QSL-solution-plan rev3 d9016e53d2ab46c32b7a7cb060ea70dc79421617e9b054848b464ba7a5518275 (amended A4), card F01,
sub-assignment C03. Lane NA-0783 (D-1425, D-1426, D-1427). Drafted 2026-09-24 by the executor seat (opus/high).
Status of every row: PROPOSED; ACCEPTED only by the Director's ruling after the SR-15 read (CRYPTO-TOUCHING: mandatory,
fable/xhigh). Scope: THE PLAN row C03 -- "Exact read/deposit derivation and namespaces including invite slots;
endpoint/request/response schema; request identity and secret handling; transactional first claim;
retry/conflict/expiry/revoke order; fixed recovery horizon; result quotas and cleanup." Not in scope: C04 (client
ownership/accounting), C05 (dispatcher, ACK dispositions, budgets, DTO names), C06, C07; each dependency is an OPEN
cell in T11.
Fits C01 (docs/ops/contracts/C01_versions_and_boundaries.md at main, with AMENDMENTS AM-1..AM-3): refusal before
effects (every refusal in T6 precedes its effect; a known-operation replay returns saved bytes and makes no new
effect); O8 (every client secret below lives in the successor vault in the successor directory; C03 adds no plaintext
file); identifiers (new labels, headers and codes are PROPOSED spellings, allocated in the DOC-CAN-003 sec 12 table by
the commit PR, as C01 and C02 did). CENSUS V11 is C03's and is disposed in T4.
Fits C02 (docs/ops/contracts/C02_invitation_authentication.md at main): the bearer secret never reaches the relay and
redeem_cap/cap_hash keep their accepted shape (C02 T1a, E7); relay_ep equality (F1) is unchanged; known-operation
resume (F2: R2b, L0) uses the horizon fixed here; C02's OPEN cells naming C03 are answered in T9.
Sources measured (read-only; the drafting mission's MEASURE.md, sha256 08da36d4..., five tables, zero unclassified):
S = qsl-server main 5ea0f925 (src/<file>:<line>); M = qsl-protocol main a0c0c491 (#1839 merge; client
qsl/qsl-client/qsc/src/<file>:<line>); P28 = #1828 e29a07df. "MEASURE n.m" cites a row of that file.
Notation: H(x) = SHA-256(x); "++" byte concatenation; labels are ASCII bytes; hex16 = 32 lowercase hex characters
encoding 16 bytes; hex32 = 64 lowercase hex characters encoding 32 bytes; b64 = base64url without padding, canonical
trailing bits; integers in JSON are unsigned decimal. CT = CRYPTO-TOUCHING (SR-15 sensitive review required).

==============================================================================================================
0. WHAT CHANGES, IN ONE TABLE
==============================================================================================================
| Surface | Today (S, M) | Successor v2 (PROPOSED) | Section |
|---|---|---|---|
| Mailbox authority | one route token hashes to one key for push, pull AND ack (MEASURE 1.4, 1.5); peers hold the owner's pull token (1.8, 1.9) | recipient secret R reads; D = H(label ++ R) deposits; the relay derives D from R; only D is ever given to a peer | T1 |
| Invite slot authority | invite_id (inside every code) is the slot's route token: any code holder can pull and ACK the slot (1.6, 1.7, 4.10) | the slot is a v2 mailbox with the inviter's own R_slot; the invitation grants redemption and ONE ticket-gated deposit, never read | T1 |
| Namespaces | one key space: unlabelled SHA-256 hex of a string; no namespace column (3.6) | separate v2 tables keyed by labelled hashes of decoded bytes; v1 headers refused on v2 | T2 |
| Endpoints | /v1/*: legacy delete-on-pull by default (1.18), JSON number-array bodies (1.21), silent expiry clamp (2.3) | /v2/*: lease-only, b64 items, strict schemas, explicit service capability, bounded client parsing and deadlines | T3 |
| Request identity | none at the relay; duplicates enqueue (5.5); ticket and bundle one-shot (2.7-2.10) | op_id + request digest on every v2 write; exact retry returns saved bytes; changed data conflicts | T4, T5 |
| Redemption | consume-and-erase; relay-minted ticket burned on first push; lost responses strand the redeemer (2.7-2.10) | transactional first claim records request digest, recovery-secret hash, client-minted ticket hash and the replayable result; A1 delivery records its exact-envelope digest and receipt | T5 |
| Expiry | push path sweeps an expired slot and then admits the push UNGATED (2.12, source reading, NOT RUN) | fresh expiry refuses new claimants only; recovery_until fixed at first claim; a slot never degrades into an ungated mailbox | T6, T7 |
| Storage schema | SCHEMA_VERSION 2, downgrade refusal exists (3.3-3.5) | SCHEMA_VERSION 3 adds the v2 tables; an old binary refuses the new store; no v1 data is migrated into v2 | T8 |

==============================================================================================================
T1. CAPABILITY CONSTRUCTION -- CT (every row with a label is a hash-domain choice; SR-15 sensitive review named)
==============================================================================================================
| # | Object | Construction | Held by | Presented on | Relay stores |
|---|---|---|---|---|---|
| 1 | R, mailbox read secret (CT) | 32 bytes from the OS CSPRNG, minted by the owner per mailbox; never derived from any v1 token (T2 N4) | the owner's vault only | open, pull, ack (X-QSL-Read-Cap: hex32(R)); for a slot also create and revoke | nothing; derives D, then K, per request |
| 2 | D, deposit capability (CT) | D = H("QSL.relay.deposit.v1" ++ R), 32 bytes | the owner; peers ONLY through the signed QSLH v2 route_cap field (C02 T1b #10) as hex32(D) | push (X-QSL-Deposit-Cap: hex32(D)) | nothing; derives K |
| 3 | K, storage key (CT) | K = H("QSL.relay.key.v1" ++ D) | the relay (primary key) | never | K, 32-byte BLOB (a stolen store yields no D and no R: the at-rest property of today's route_key_for, S lib.rs:950-952, kept) |
| 4 | log id | hex of the first 8 bytes of H("QSL.relay.log.v1" ++ K); replaces the FNV-1a of the raw token string (S lib.rs:940-948) | relay logs | never | not stored |
| 5 | Invite slot | a v2 mailbox of kind invite_slot with its own R_slot, D_slot, K_slot (rows 1-3), minted by the inviter at create. D_slot is never shared: the invitation carries no read or deposit key for the slot | inviter (R_slot) | create, pull, ack, revoke (R_slot) | K_slot and L (row 6) |
| 6 | L, slot locator (CT) | L = H("QSL.relay.slot-locator.v1" ++ invite_id), invite_id = C02 T1a #4 (16 bytes) | every code holder (invite_id is in the code) | redeem, deliver, settle, revoke (as hex16(invite_id)) | L, 32-byte BLOB; invite_id itself is never stored |
| 7 | redeem_cap | C02 T1a, UNCHANGED: KMAC<16>(bearer_secret, "QSL.invite.redeem-cap.v1", invite_id), presented as wire_id (hex16); the create call uploads cap_hash_hex(wire_id(redeem_cap)) (M invite/mod.rs:213) | code holders | redeem | cap_hash as uploaded (C02 E7) |
| 8 | S, recovery secret (CT) | 32 bytes CSPRNG, minted in the redeem operation's local commit (T5 step L1) | the redeemer only | redeem (every request, including the first), settle | H_S = H("QSL.relay.recovery.v1" ++ S) |
| 9 | T, initial-push ticket (CT) | 32 bytes CSPRNG, CLIENT-minted in the same local commit (THE PLAN sec 2); the relay no longer mints a ticket | the redeemer only | deliver (X-QSL-Invite-Ticket: hex32(T)) | H_T = H("QSL.relay.ticket.v1" ++ T), registered by the first claim |
| 10 | op_id | 16 bytes. A redemption: op_id = C02 attempt_id (T1b #6), one identity, not two. A push: 16 bytes CSPRNG per queued item, fixed when the item is committed (C04/C05 own the queue row) | the client | every v2 write (X-QSL-Operation-Id or the JSON field) | op_id (not secret) |
Label set (CT): QSL.relay.deposit.v1, QSL.relay.key.v1, QSL.relay.log.v1, QSL.relay.slot-locator.v1, QSL.relay.recovery.v1,
QSL.relay.ticket.v1, QSL.relay.request.redeem.v1, QSL.relay.request.push.v1, QSL.relay.request.create.v1. PREFIX-FREE
(no label is a prefix of another) and each is followed by fixed-length input, so no two derivations can share a
pre-image. Disjoint from C02's QSL.invite.* / QSL.handshake.* labels. Spellings PROPOSED (C3-O1).
What each party can do (the property the split exists for):
| Holder | Push/deposit | Read (pull) | ACK/delete | Revoke | Redeem |
|---|---|---|---|---|---|
| owner (R) | yes (derives D) | yes | yes | yes (slot) | -- |
| peer (D) | yes | NO: H preimage | NO | NO | -- |
| code holder (invite code) | ONE A1 under its own winning claim (T5) | NO (the slot is read with R_slot) | NO | NO (R_slot) | first claim wins |
| stolen relay store (K, L, hashes) | NO | NO | NO | NO | NO (cap_hash, H_S, H_T are hashes) |
| relay operator | sees R, D, S, T, redeem_cap transiently on requests; can censor, drop, delay or read ciphertext | -- | -- | -- | -- |
The last row is THE PLAN's own boundary: "This restricts peer capability, not a malicious relay operator, who can still
censor." Nothing here protects against the relay operator beyond today's TLS and end-to-end encryption.

==============================================================================================================
T2. NAMESPACES -- v2 UNREACHABLE BY ANY v1 TOKEN REPRESENTATION
==============================================================================================================
| Dimension | v1 (today; UNCHANGED for old clients) | v2 (PROPOSED) |
|---|---|---|
| Key | TEXT: lowercase hex SHA-256 of the trimmed header STRING, no label (S lib.rs:950-959, :1002) | BLOB 32: K (T1 row 3) over DECODED bytes, labelled |
| Tables | routes, messages, invites (S store.rs:214-250) | mailboxes_v2 (K, kind mailbox or invite_slot, created_at, touched_at), messages_v2, slots_v2 (L, K_slot, cap_hash, result bytes, expiry, state), claims_v2, push_receipts_v2 |
| Capability header | X-QSL-Route-Token: the relay accepts any trimmed, non-empty token (S lib.rs:997-1009); the 22..128 [A-Za-z0-9_-] grammar is the CLIENT's (M adversarial/route.rs:21-37) (F5) | X-QSL-Deposit-Cap / X-QSL-Read-Cap: EXACTLY hex32, no trim, decoded before hashing |
| Slot addressing | invite_id string = route token = slot key | L (T1 row 6) for claims and deliveries; K_slot for the owner |
Rules:
 N1 No v1 handler reads or writes a v2 table; no v2 handler reads or writes a v1 table (F08 source-shape test, XV7).
 N2 A v2 request carrying X-QSL-Route-Token is refused (ERR_V2_V1_TOKEN_PRESENT) before any lookup; a v2 request
    without its v2 capability header is refused (ERR_V2_CAP_MISSING).
 N3 v2 keys are labelled hashes of decoded bytes in BLOB columns of v2 tables; v1 keys are unlabelled hashes of
    strings in TEXT columns of v1 tables. Adding a hash domain alone is NOT relied on (THE PLAN sec 3): N1 separates
    storage, N3 separates derivation, N2 separates representation.
 N4 The successor never reuses a v1 token's bytes as R or D: R is fresh CSPRNG output; tui.relay.inbox_token is
    RETIRED on the successor path (T4, CENSUS V11).
 N5 A slot never degrades into an ungated mailbox: a mailbox's kind is fixed at creation; every deposit into a
    kind invite_slot row, by /v2/invite/deliver (L) or by /v2/push (K_slot), runs ONE admission function, slot_admit,
    with one set of codes; slot and claim rows are removed only by the T7 sweep, after which the key is unknown (404),
    never an ordinary mailbox. Contrast, measured by source reading: v1 admits an ungated push after the sweep deletes
    an expired slot row (MEASURE 2.12, NOT RUN).
Cross-version addressing tests (named; specified in T10): XV1-XV8.

==============================================================================================================
T3. ENDPOINT / REQUEST / RESPONSE SCHEMA (v2, lease-only)
==============================================================================================================
Common rules: every v2 request passes the relay-wide bearer gate unchanged (auth_ok, S lib.rs:961-982; 401
ERR_UNAUTHORIZED) -- an ADMISSION credential, not a capability. The body length is checked against the endpoint's cap
BEFORE any parse (413 ERR_TOO_LARGE). JSON bodies: exact field set (unknown, missing or duplicate field -> 400
ERR_V2_BAD_REQUEST), "v" = 2, hex fields exact length and lowercase. x-msg-id is not a v2 header (present -> 400).
Every 200 body is JSON with "v":2; error bodies are the plain code, as today. Codes marked NEW are PROPOSED spellings.
| # | Endpoint | Request (headers; body) | Body cap | 200 response | Refusals (besides 401/413/400 above) |
|---|---|---|---|---|---|
| EP1 | POST /v2/mailbox/open | Read-Cap; empty body | 0 B | {"v","state":"created"/"exists"} | 429 ERR_RATE_LIMITED (the GLOBAL open bucket, drawn BEFORE any lookup, F1; T6 Open); 409 ERR_V2_KIND_CONFLICT (K is a slot) NEW; 429 ERR_V2_MAILBOX_CAP NEW |
| EP2 | POST /v2/push | Deposit-Cap, Operation-Id (hex16); body = opaque frame | max_body_bytes (configured; >= 12288 required, T8 M9) | {"v","id":hex16} (relay-minted msg id) | 404 ERR_V2_MAILBOX_UNKNOWN NEW; kind invite_slot -> slot_admit (T6: the 12288 B slot cap, then D2-D7); 409 ERR_V2_OPERATION_CONFLICT NEW; 429 ERR_OVERLOADED / ERR_RATE_LIMITED / ERR_V2_LOOKUP_RATE NEW; 400 ERR_EMPTY_BODY |
| EP3 | GET /v2/pull?max=N | Read-Cap; N in 1..=V2_PULL_MAX_ITEMS (PROPOSED 32, <= max_queue_depth); no other query parameter | -- | {"v","lease_secs","items":[{"id":hex16,"data":b64}]} (lease-only: every returned row is leased) or 204 | 404 ERR_V2_MAILBOX_UNKNOWN; 400 ERR_BAD_MAX |
| EP4 | POST /v2/pull/ack | Read-Cap; {"v","ids":[hex16...]} 1..=4096 unique ids (MAX_ACK_IDS, S store.rs:21) | 163840 B | {"v","acked":n}; deletes LEASED copies only (as S store.rs:810-817) | 404 ERR_V2_MAILBOX_UNKNOWN; 400 ERR_BAD_ACK_IDS |
| EP5 | POST /v2/invite/create | Read-Cap (R_slot); {"v","invite_id":hex16,"cap_hash":hex32,"expiry":u64,"bundle":b64,"invite_sig":b64} | 2*ceil(4*max_invite_bundle_bytes/3)+512 | {"v","expiry","recovery_secs"} -- no revoke_token (revoke is authorized by R_slot, T4) | 400 ERR_V2_EXPIRY_RANGE NEW (no clamp: expiry must be in (now, now+max_expiry]); 409 ERR_INVITE_DUPLICATE; 409 ERR_V2_KIND_CONFLICT; 429 ERR_RATE_LIMITED; 429 ERR_INVITE_CAP_FULL |
| EP6 | POST /v2/invite/redeem | {"v","invite_id":hex16,"redeem_cap":hex16,"op_id":hex16,"recovery_secret":hex32,"ticket_hash":hex32} (T is never sent here) | 512 B | the RESULT: {"v","op_id","bundle":b64,"invite_sig":b64,"recovery_until":u64}; stored and replayed byte-identical | 404 ERR_INVITE_NOT_FOUND; 403 ERR_INVITE_CAP_INVALID; 409 ERR_INVITE_ALREADY_USED / ERR_V2_OPERATION_CONFLICT / ERR_V2_RESULT_SETTLED NEW; 410 ERR_INVITE_REVOKED / ERR_INVITE_EXPIRED / ERR_V2_RECOVERY_EXPIRED NEW; 429 ERR_V2_RESULT_QUOTA NEW / ERR_V2_LOOKUP_RATE |
| EP7 | POST /v2/invite/deliver | X-QSL-Invite-Locator (hex16 invite_id), X-QSL-Invite-Ticket (hex32 T), Operation-Id; body = the exact A1 envelope (C02 T1b) | 12288 B (= C02 ENV_MAX) | the RECEIPT: {"v","id":hex16,"envelope_digest":hex32,"accepted_at":u64}; stored and replayed byte-identical | 404 ERR_INVITE_NOT_FOUND; 403 ERR_INVITE_TICKET_INVALID; 409 ERR_V2_DELIVERY_CONFLICT NEW; 410 ERR_INVITE_REVOKED / ERR_V2_RECOVERY_EXPIRED; 429 ERR_OVERLOADED / ERR_RATE_LIMITED / ERR_V2_LOOKUP_RATE |
| EP8 | POST /v2/invite/settle | {"v","invite_id","op_id","recovery_secret"} -- sent only after the client durably saved the result AND holds the delivery receipt (THE PLAN: "The client acknowledges the result only after saving it durably") | 256 B | {"v","settled":true}; erases the retained result bytes early; idempotent | 404; 403 ERR_V2_RECOVERY_INVALID NEW; 409 ERR_V2_NOT_DELIVERED NEW |
| EP9 | POST /v2/invite/revoke | Read-Cap (R_slot); {"v","invite_id"} | 128 B | {"v","revoked":true,"delivered":bool} (whether an A1 was accepted before the revoke); idempotent | 404; 403 ERR_INVITE_REVOKE_INVALID |
| EP10 | GET /v1/server-info (existing, additive per DOC-SRV-006 rule 1) | unchanged | -- | api gains "relay_v2"; new object "v2": {max_body_bytes, pull_max_items, lease_secs, recovery_secs, max_ack_ids, push_receipts_per_mailbox, max_mailboxes} | unchanged (S lib.rs:895-938) |
Client obligations (the successor client; qsl-protocol transport):
 C1 EXPLICIT SERVICE CAPABILITY CHECK before the first v2 call to a relay endpoint in a process: server-info must list
    "relay_v2" and carry every "v2" field, and v2.recovery_secs must EQUAL the client's H_REC constant; else
    relay_v2_unsupported (NEW client code) and NO request of any kind is made to /v1/ mailbox or invite paths.
 C2 NO FALLBACK: a 404 from a /v2/ path whose body is not an ERR_V2_* / ERR_INVITE_* code -> relay_v2_unsupported;
    never "legacy complete" (today's v1 ACK maps 404 to LegacyComplete, M transport/mod.rs:3647 -- not carried).
 C3 BOUNDED RESPONSE PARSING: read at most CAP+1 body bytes BEFORE parsing; CAP = pull: N_requested *
    (ceil(4*v2.max_body_bytes/3) + 64) + 128; redeem: 2*ceil(4*max_invite_bundle_bytes/3) + 512; every other
    endpoint: 1024. Over CAP -> relay_v2_response_too_large; strict JSON (unknown field, items > N_requested, an item's
    decoded data > v2.max_body_bytes, a non-hex16 or repeated id) -> relay_v2_response_malformed (both NEW). Today's
    pull parses with no cap (M transport/mod.rs:3584-3591; triage F-12).
 C4 DEADLINES: connect 10 s (the probe's precedent, M transport/mod.rs:2551-2567); whole request 30 s (push, pull,
    invite calls) and 15 s (ack); values PROPOSED (C3-O5). Today the shared client sets none (M transport/mod.rs:
    2183-2199). A timeout is a transport failure: the operation stays committed and is retried with the SAME op_id
    and the SAME bytes.
 C5 PULL BUDGET: N_requested is chosen so that CAP stays within the per-pull byte budget; the budget value is C05's
    (finite request/byte budgets; C3-O5).

==============================================================================================================
T4. REQUEST IDENTITY AND SECRET HANDLING
==============================================================================================================
| Item | Rule |
|---|---|
| op_id | T1 row 10; committed durably BEFORE the first network call of its operation (I03); immutable across retries (I04); a retry re-sends identical bytes |
| Redeem request digest Q_R (CT) | H("QSL.relay.request.redeem.v1" ++ invite_id(16) ++ op_id(16) ++ redeem_cap(16) ++ H_S(32) ++ H_T(32)), computed by the relay from the DECODED fields, never over raw JSON text |
| Delivery digest E | H(exact envelope bytes): the same value as C02 T3's envelope_digest, so the relay receipt and the inviter's HsReceipt name the same bytes |
| Push digest P (CT) | H("QSL.relay.request.push.v1" ++ K ++ op_id ++ H(body)) |
| Create digest Q_C (CT) | H("QSL.relay.request.create.v1" ++ L ++ K_slot ++ H(cap_hash as uploaded) ++ H(bundle) ++ H(invite_sig) ++ u64be(expiry)) |
| What the relay stores | hashes only: K, L, cap_hash (as uploaded), H_S, H_T, Q_R, E, P, Q_C; msg ids; timestamps; the PUBLIC result bytes (bundle, invite_sig) until settle or recovery_until (T7); the saved receipt bytes. NEVER stored: R, D, S, T, redeem_cap, bearer_secret, invite_id |
| Compares | every comparison of a secret-derived value (cap_hash, H_S, H_T, Q_R, P, Q_C, K on revoke; P and Q_C carry K / K_slot, F8) is a constant-time compare of 32-byte digests (the existing ct_eq_secret, S lib.rs:984-995) INSIDE the store transaction; rows are found by index on K or L (hashes of secrets), the precedent of today's route_key_for. E = H(exact envelope bytes) is a CONTENT COMMITMENT, not a secret (the envelope is delivered to the inviter), and is compared as such: equality of the presented body's digest with claim.E (F8) |
| Logging | log id (T1 row 4) and outcome only; never R, D, S, T, redeem_cap, invite_id, op_id or body bytes (today's rule "Never log payload", S lib.rs:1160) |
| Client storage | R per mailbox; S and T per redeem operation; all in the successor vault (O8; key spellings C01 O5, C3-O12). R zeroized when its mailbox is retired; S and T zeroized in the commit that makes the operation terminal (applied, failed-expired, revoked), the C02 T3 (F5c) pattern |
| Where each secret may travel | R: only to its own mailbox's canonical relay_ep. D: to peers only inside the signed QSLH v2 route_cap, and to its relay on push. S, T, redeem_cap: only to the invite's relay_ep. bearer_secret: never to any relay (C02) |
| Relay bearer credential, per endpoint (C02 OC15) | the client keeps a map canonical relay_ep (C02 grammar G1-G7) -> credential, and attaches a credential ONLY to a request whose canonical endpoint equals its key; the environment credential binds to the configured relay endpoint only; no entry -> no Authorization header (the relay answers 401 -> relay_unauthorized for that endpoint). Today one value is attached to every push, pull, ack, invite POST and probe (MEASURE 4.1-4.6) |
| C01 CENSUS V11 (C03's), disposed by name | tui.relay.token -> an entry of the per-endpoint map (SUCC-NEW); tui.relay.token_file -> per-endpoint (SUCC-NEW); tui.relay.inbox_token -> RETIRED on the successor path (replaced by per-mailbox R; never reused, T2 N4); tui.relay.ca_file -> KEEP (TLS trust anchor, unchanged). Spellings: C01 O5 |

==============================================================================================================
T5. TRANSACTIONAL FIRST CLAIM AND RECOVERY
==============================================================================================================
Client side (the redeemer; C02 R2b-R6 name the surrounding steps):
| Step | Durable act (one commit each) | Retry rule |
|---|---|---|
| L1 | BEFORE any network (C02 R4): the operation record {op_id = attempt_id, the invite payload, local identity, profile A02, S, T, H_S, H_T, created_at} | a crash here leaves a known operation; the redeem request is rebuilt from these exact fields |
| L2 | redeem; on 200, the RESULT bytes and recovery_until saved durably BEFORE verification (C02 R4 "cache"; the cached bytes are UNVERIFIED and pin nothing) | lost response -> exact retry (same op_id, S, H_T) returns the saved result |
| L3 | C02 R5 verify, then C02 R6: outgoing candidate with the exact A1 envelope, committed BEFORE deliver | the envelope bytes are fixed here; every deliver retry sends them |
| L4 | deliver; on 200 the RECEIPT saved durably | lost response -> exact retry returns the saved receipt, even after the inviter ACKed |
| L5 | settle (optional, shortens the privacy window, T7); then S and T zeroized when the operation is terminal | idempotent |
Relay side: the FIRST CLAIM is ONE store transaction (the same single-connection discipline that makes today's CAS exact,
S store.rs:458-465). It inserts claims_v2 {L, op_id, Q_R, H_S, H_T, claimed_at, recovery_until = claimed_at + H_REC,
result_bytes = the exact 200 body, settled = false, E = NULL, receipt = NULL} and sets the slot Claimed; commit; then
respond. A crash before the commit leaves no claim; a crash after it is recovered by the exact retry.
| Case | Rule | Result |
|---|---|---|
| Exact authenticated retry of the claim | same op_id AND Q_R equal (constant time; Q_R binds H_S, so it proves S) | 200 with result_bytes byte-identical; no new effect, no second ticket |
| Changed data under the same op_id | op_id equal, Q_R differs | 409 ERR_V2_OPERATION_CONFLICT; nothing changes |
| Another claimant | op_id differs | 409 ERR_INVITE_ALREADY_USED (today's spelling and meaning) |
| Initial A1 delivery | ONE transaction: insert the envelope into messages_v2 under K_slot with a relay msg id AND set claim.E = H(body), claim.receipt = the exact 200 body | 200 receipt |
| Exact delivery retry | ticket authenticates (H_T), claim.E set and equal to H(body) | 200 saved receipt; NO second enqueue, even when the message was ACKed and deleted (the receipt lives on the claim, not on the message) |
| Changed delivery | claim.E set, H(body) differs | 409 ERR_V2_DELIVERY_CONFLICT |
The ticket is therefore valid for exactly ONE envelope digest per claim, and replaying proof of an accepted effect never
delivers it again (THE PLAN sec 2).

==============================================================================================================
T6. ORDER TABLES -- AUTHENTICATE, THEN LOOK UP A KNOWN OPERATION, THEN (NEW WORK ONLY) EXPIRY, CAPACITY AND RATE
==============================================================================================================
recovery_until is set ONCE, at the first claim, on the relay's clock, and is never extended by a retry, a delivery,
a settle or a revoke. Fresh invite expiry refuses NEW claimants only. H_REC = 259200 s PROPOSED (C3-O2).
Redeem (EP6):
| Step | Check | Refusal | Effect before it? |
|---|---|---|---|
| R0 | body <= 512 B, before parse; bearer; strict schema | 413; 401; 400 | none |
| R1 | slot by L | 404 ERR_INVITE_NOT_FOUND | none |
| R2 | AUTHENTICATE: cap_hash vs sha256_hex(redeem_cap wire string), constant time (C02 T1a shape) | 403 ERR_INVITE_CAP_INVALID | none |
| R3 | KNOWN OPERATION (a claim exists): (a) op_id equal, Q_R equal -> per-L lookup bucket (else 429 ERR_V2_LOOKUP_RATE); if the slot is Revoked and claim.E is NULL -> 410 ERR_INVITE_REVOKED; if settled -> 409 ERR_V2_RESULT_SETTLED; if now >= recovery_until -> 410 ERR_V2_RECOVERY_EXPIRED; else 200 result_bytes. (b) op_id equal, Q_R differs -> 409 ERR_V2_OPERATION_CONFLICT. (c) op_id differs -> 409 ERR_INVITE_ALREADY_USED | as stated | none: no create bucket, slot cap, result quota or fresh expiry is consulted before R3 |
| R4 | NEW CLAIMANT only: Revoked -> 410 ERR_INVITE_REVOKED; now >= expiry -> 410 ERR_INVITE_EXPIRED; retained-result quota (T7) -> 429 ERR_V2_RESULT_QUOTA | as stated | none |
| R5 | first claim commit (T5) | 500 ERR_STORE on a store failure (nothing committed) | the claim |
Order change named: today not-found -> revoked -> expired -> already-used -> cap-invalid (S store.rs:467-527). v2
authenticates (R2) before it reports any state beyond existence, so that the known-operation lookup (R3) can precede
revoke/expiry for the operation's owner.
Deliver (EP7) and slot_admit (EP2 on a kind invite_slot row applies D0's 12288 B cap, then D2-D7, with K_slot's slot):
| Step | Check | Refusal | Effect before it? |
|---|---|---|---|
| D0 | body 1..=12288 before parse; headers exact; bearer | 413 / 400; 401 | none |
| D1 | slot by L (deliver) or by K_slot (push) | 404 | none |
| D2 | a claim exists and its op_id equals Operation-Id | 403 ERR_INVITE_TICKET_INVALID | none |
| D3 | AUTHENTICATE: H("QSL.relay.ticket.v1" ++ T) vs claim.H_T, constant time; a missing ticket header is a mismatch | 403 ERR_INVITE_TICKET_INVALID | none |
| D4 | KNOWN DELIVERY (claim.E set): per-L lookup bucket (429 ERR_V2_LOOKUP_RATE); E == H(body) -> now < recovery_until ? 200 saved receipt : 410 ERR_V2_RECOVERY_EXPIRED; E differs -> 409 ERR_V2_DELIVERY_CONFLICT | as stated | none: replaying an accepted effect consumes no slot depth, no push bucket, and ignores a later revoke |
| D5 | NEW DELIVERY only: Revoked -> 410 ERR_INVITE_REVOKED (revoke before first acceptance prevents the effect); now >= recovery_until -> 410 ERR_V2_RECOVERY_EXPIRED; fresh invite expiry is NOT applied (an admitted claim may deliver until its recovery_until) | as stated | none |
| D6 | NEW DELIVERY only: slot depth >= max_queue_depth -> 429 ERR_OVERLOADED; per-slot push bucket -> 429 ERR_RATE_LIMITED | as stated | none |
| D7 | commit (T5 initial A1 delivery) | 500 ERR_STORE | the delivery |
Push to a kind mailbox (EP2):
| Step | Check | Refusal |
|---|---|---|
| P0 | body 1..=max_body_bytes before parse; headers exact (N2); bearer | 413 / 400; 401 |
| P1 | K from D; mailbox exists | 404 ERR_V2_MAILBOX_UNKNOWN (no row is created by a push; today any push creates a route, MEASURE 1.14) |
| P2 | kind invite_slot -> slot_admit (the 12288 B slot cap, then D2-D7) | 413; as D2-D7 |
| P3 | KNOWN (receipt for K, op_id exists): per-K lookup bucket; P equal -> 200 saved {"id"}; P differs -> 409 ERR_V2_OPERATION_CONFLICT | as stated (no depth, no push bucket consumed) |
| P4 | NEW only: depth >= max_queue_depth -> 429 ERR_OVERLOADED; per-K push bucket -> 429 ERR_RATE_LIMITED | as stated |
| P5 | commit: message + receipt (ring eviction, T7) | 500 ERR_STORE |
Pull / ack (EP3, EP4): bounds; N2; bearer; K from R; mailbox exists else 404; pull leases (no delete-on-pull arm exists
in v2); ack deletes leased copies of THIS K only. No known-operation step (both are idempotent in effect).
Open (EP1) (F1):
| Step | Check | Refusal | Effect before it? |
|---|---|---|---|
| O0 | body empty (0 B) before parse; headers exact (N2); bearer | 413 / 400; 401 | none |
| O1 | the GLOBAL open bucket, drawn BEFORE any lookup, by every open (an "exists" answer included) -- the D614 F6 idiom: the cap bounds storage, the bucket bounds denial (S lib.rs:83-91, :679-698); values PROPOSED (C3-O3) | 429 ERR_RATE_LIMITED | none |
| O2 | K from R; K is a kind invite_slot row | 409 ERR_V2_KIND_CONFLICT | none |
| O3 | K is a kind mailbox row | 200 "exists" | none |
| O4 | NEW only: mailbox count >= MAX_V2_MAILBOXES | 429 ERR_V2_MAILBOX_CAP | none |
| O5 | commit: mailboxes_v2 row (kind mailbox, never pulled) | 500 ERR_STORE | the row |
Create (EP5): bounds; bearer; schema; KNOWN: L exists -> K_slot and Q_C equal (constant time) ? 200 saved response :
409 ERR_INVITE_DUPLICATE (checked BEFORE the create bucket and the slot cap, so an exact retry after a lost response
consumes neither); NEW: expiry range -> 400 ERR_V2_EXPIRY_RANGE; create bucket -> 429 ERR_RATE_LIMITED; slot cap ->
429 ERR_INVITE_CAP_FULL; K_slot already a kind mailbox -> 409 ERR_V2_KIND_CONFLICT; commit mailbox row + slot row.
Revoke (EP9): bounds; bearer; slot by L (404); derive K from the presented R and compare with slot.K_slot in constant
time (403 ERR_INVITE_REVOKE_INVALID) BEFORE any state is reported (today's rule, S store.rs:574-578); set Revoked;
idempotent; report delivered = (claim.E is set). An accepted A1 stays in the slot mailbox for the owner to pull and
dispose (disposition C05, C3-O6); revoke never deletes committed work.
Settle (EP8): bounds; bearer; slot by L; claim with op_id; H_S constant time (403 ERR_V2_RECOVERY_INVALID); claim.E
NULL -> 409 ERR_V2_NOT_DELIVERED; erase result_bytes, settled = true; idempotent.
Client horizons (C02 L0, R2b): the REDEEMER resumes a known operation until the first of (a) a relay answer 410
ERR_V2_RECOVERY_EXPIRED or 404 for it, or (b) by its own clock, the relay's recovery_until + CLOCK_SLACK when the result
was received, else its own L1 time + H_REC + CLOCK_SLACK (the relay's clock is not the client's; CLOCK_SLACK PROPOSED
3600 s); it then marks the attempt failed-expired (C02 T5) and zeroizes it (C02 T3) -- SUBJECT TO F2: the failed-expired transition applies ONLY to an attempt with NO committed selection or session (C02 T4 X-d; P28 handshake/mod.rs:630-631); on 410 ERR_V2_RECOVERY_EXPIRED or 404 for a KNOWN delivery (the operation holds its committed A1 envelope, T5 L3) the redeemer FIRST drains its own mailbox and binds any B1 to the attempt (C02 T2 step 3) before marking it failed-expired; an ADVANCED attempt (selection or session committed) is never reset or zeroized by the horizon (vector CL9). The INVITER pulls its slot and processes an admitted A1 until
invite expiry + H_REC, an upper bound on every claim's recovery_until (a claim can only be made before expiry).

==============================================================================================================
T7. QUOTAS, CLEANUP AND THE PRIVACY TRADE-OFF
==============================================================================================================
| Retained item | Count bound | Byte bound | Cleanup deadline |
|---|---|---|---|
| slots_v2 rows | <= MAX_INVITE_SLOTS (S lib.rs:92-93: 256 default, 4096 ceiling), counted separately from v1 | row ~ 200 B | the slot with its claim tombstone at max(expiry, recovery_until); at expiry when never claimed (today's tombstone rule, S store.rs:339-345) |
| claim result bytes (bundle, invite_sig) | <= 1 per slot | <= 2*max_invite_bundle_bytes + 256 per claim: 8.1 MiB at defaults (256 x (32 KiB + 256 B)); 513 MiB at the ceilings (4096 x (128 KiB + 256 B)) -- stated, and the global ERR_V2_RESULT_QUOTA exists to hold it | erased at the earlier of settle and recovery_until |
| claim tombstone (op_id, Q_R, H_S, H_T, E) | <= 1 per slot | ~ 200 B | with the slot, at max(expiry, recovery_until) |
| delivery receipt | <= 1 per claim | <= 256 B | recovery_until (after it a retry is 410; the tombstone keeps E) |
| slot mailbox messages | <= max_queue_depth | the envelope, <= 12288 B each | owned by the SLOT sweep with their invite_slot mailbox row (F3): removed by ACK or with that row, not earlier by the retention TTL (S store.rs:5-6) |
| mailboxes_v2 rows of kind mailbox | <= MAX_V2_MAILBOXES, its OWN ceiling (F1; PROPOSED default 4096, ceiling 65536; not MAX_ROUTE_COUNT's 256, S lib.rs:116), counted separately from v1 routes | ~ 100 B | NEVER PULLED: removed when empty and not opened for 3600 s (the pull lease ceiling, S store.rs:8) (F1); PULLED AT LEAST ONCE: removed when empty and not opened, pulled or acked for MAILBOX_IDLE_SECS (PROPOSED = retention TTL; F1: it applies only to a mailbox pulled at least once); either way a later push is 404 and the owner re-opens idempotently before its next pull |
| mailboxes_v2 rows of kind invite_slot (F3) | <= 1 per slots_v2 row | ~ 100 B | owned by the SLOT sweep: removed with its slots_v2 row (at max(expiry, recovery_until); at expiry when never claimed); EXEMPT from MAILBOX_IDLE_SECS and from the never-pulled horizon |
| push receipts | <= PUSH_RECEIPTS_PER_MAILBOX (PROPOSED 1024) per mailbox, oldest evicted first | ~ 100 B each (~ 100 KiB per mailbox) | the retention TTL. CONSEQUENCE, stated: an exact push retry older than the ring is treated as NEW (one duplicate item); receiver-side duplicate handling is C05's (C3-O4) |
| lookup buckets | one per L or K, in memory (as today's push buckets, S lib.rs:342-344) | -- | reset on restart |
v1's equivalent surface is NOT carried into v2 (F1): on v1 any bearer holder's push to an unknown token creates a route (MEASURE 1.14; S store.rs:672-693), so the 256-route cap can be held for the retention TTL by one message per route; on v2 a push creates no row (T6 P1), and an open draws the global open bucket before any lookup (T6 O1).
FK / orphan rule (F3): messages_v2 references mailboxes_v2 by K with ON DELETE CASCADE under foreign_keys ON (the v1 rule for messages -> routes, S store.rs:212, :228). A message is inserted only in a transaction that finds its parent mailboxes_v2 row (none found -> 404, never an orphan insert and never a 500); removing a mailbox row removes its messages in the same transaction. A kind invite_slot row cannot be idle-removed under a live slots_v2 row, so EP7 D7 always finds its parent.
Result-lookup rate policy: every known-operation answer (R3a, D4, P3, create KNOWN) draws from its own per-L/per-K
lookup bucket (PROPOSED burst 16, refill 1 per 4 s; C3-O3) and from nothing else: a replayed saved result consumes no
mailbox slot, no push or create bucket and no quota. New work never draws from the lookup bucket.
Sweep: all v2 deadlines run in the existing lazy-plus-periodic sweep, in its own transaction (S store.rs:292-358
pattern). Relay configuration must satisfy retention_ttl >= H_REC for relay_v2 (T8 M9), so a delivered A1 outlives
its operation's recovery horizon unless it is ACKed.
PRIVACY TRADE-OFF (THE PLAN sec 2, stated plainly): today the relay erases the inviter's bundle and invite signature in
the winning redeem transaction (S store.rs:530-537), so after redemption a stolen store or a curious operator holds
only a tombstone. v2 KEEPS that public identity bundle (the inviter's ML-KEM and ML-DSA public keys and the invite
signature, linked to the slot locator L) until the redeemer settles or recovery_until passes -- up to H_REC (72 h
PROPOSED) after the claim instead of zero. The keys are public, but their presence ties an identity to an invitation
event on the relay for that window. settle (EP8) shortens it to the recovery round trip; recovery_until bounds it.

==============================================================================================================
T8. SERVER SCHEMA MIGRATION AND DOWNGRADE REFUSAL (no silent fallback, D02)
==============================================================================================================
| Case | Behaviour | Code / evidence |
|---|---|---|
| M1 new binary (SCHEMA_VERSION 3) opens a version-2 store | CREATE the v2 tables IF NOT EXISTS and advance the marker to 3, using the existing read-before-write marker machinery (S store.rs:189-199, :251-280); v1 rows untouched and still served by v1 handlers | -- |
| M2 version-2 binary (today's S) opens a version-3 store | refuses to open: stored > SCHEMA_VERSION -> ERR_STORE_VERSION; the relay does not start | EXISTING guard S store.rs:269-273, pinned by S:tests/na0678_schema_version.rs:113-129 |
| M3 old relay receives a /v2/ request | route not registered -> 404 (the router lists /v1/ routes only, S lib.rs:549-558); its server-info lacks "relay_v2", so the successor client refuses first (T3 C1) and never treats that 404 as delivered (C2) | relay_v2_unsupported (client) |
| M4 new relay receives v1 requests from old clients | v1 handlers unchanged; they cannot address v2 rows (T2 N1-N3) | -- |
| M5 successor client + a v1-only relay | refuse; no v1 fallback of any kind (I01, D02) | relay_v2_unsupported |
| M6 v1 routes or slots on a new relay | NOT migrated into v2 (no reinterpretation): they live out their expiry/retention under v1 rules | -- |
| M7 operator rolls a relay binary back after v2 data exists | M2 applies: the old binary refuses the store; restoring the new binary is the only path; the old code never reads v2 rows | ERR_STORE_VERSION |
| M8 migration crash and order (F7) | on the v2 path the marker is READ and the newer-store refusal DECIDED BEFORE ANY DDL; the DDL and the marker advance then commit in ONE transaction. Today the whole CREATE TABLE IF NOT EXISTS batch (S store.rs:214-250) runs BEFORE the marker read (:251-268) and the newer-store refusal (:271-273), and the DDL and the marker upsert (:278-280) are two units; a crash between them is benign only because the DDL is IF NOT EXISTS and the upsert is idempotent; v2 makes the decision first and the change one unit | -- |
| M9 relay_v2 configuration | startup refuses when max_body_bytes < 12288 (C02 B1 envelope <= 11729 B travels by /v2/push) or retention_ttl < H_REC; no partial enablement | ERR_INVALID_CONFIG_V2_MAX_BODY_BYTES, ERR_INVALID_CONFIG_V2_RETENTION (NEW, in the existing ERR_INVALID_CONFIG_* idiom, S lib.rs:280-285) |

==============================================================================================================
T9. C02 OPEN CELLS NAMING C03 -- CLOSED HERE OR KEPT OPEN
==============================================================================================================
| C02 cell | Answer here (PROPOSED) | State |
|---|---|---|
| OC1 route representation, relay_ep form | field 10 route_cap = hex32(D), exactly 64 lowercase hex characters (inside C02's 22..128 [A-Za-z0-9_-]); never R; field 9 relay_ep = the invitation's canonical relay_ep (C02 F1, unchanged) | CLOSED; the C02 step 2b tightening to exactly hex32 is C02 AMENDMENTS AM-1 (F6), C3-O8 closed |
| OC2 attempt_id vs operation identity, recovery secret, ticket | op_id = attempt_id; S and T minted in the same L1 commit; H_S and H_T registered by the first claim (T5) | CLOSED |
| OC3 recovery_until, retention and cleanup | recovery_until = first-claim time + H_REC, set once (T6); relay retention and cleanup T7; client horizons T6 (last paragraph); client-side receipt and exact-reply retention until recovery_until then zeroize (C02 T3); client byte/count quotas stay C04 (C02 OC5) | CLOSED, value OPEN (C3-O2) |
| OC4 frame ceiling >= 11729 (B1) and >= 9668 (A1); v2 slot namespace; cap_hash shape | A1 deliver cap 12288 fixed (EP7); B1 travels by /v2/push to the redeemer's mailbox, so relay_v2 requires max_body_bytes >= 12288 (T8 M9; the deployed rig's CONFIGURED value 65536 satisfies it -- a deployment setting, not a source fact: the source default and ceiling are 1048576, S lib.rs:114, :125, :148; E1); namespace T2; cap_hash shape unchanged (C02 E7 confirmed at S lib.rs:800-808, store.rs:393-396) | CLOSED |
| OC15 cross-relay replies; per-endpoint credential scoping | the precondition is SPECIFIED (T4 per-endpoint map); lifting C02 F1 is a separate decision not proposed here | PRECONDITION CLOSED; decision OPEN (C3-O7) |
| C02 T2 R4 / R6, T3 "the ticket (C03)", T5 failed-expired | T5 L1-L5; T6 client horizons | CLOSED |
| C02 T6 "pull-body cap (F-12) is transport, C03/C05" | transport part: T3 C3; the budget value: C05 (C3-O5) | CLOSED in part |
| C02 T7 EX1 fixture horizon; RP8v "NOT_RUN until C03" | fixture horizon = H_REC (or the test seam of C3-O11); after the horizon the relay answers 410 / 404 and the client disposes with no durable row | specifiable now |
| OC5 (C04), OC6 and OC14 (C05), OC7-OC13 | not C03's | unchanged |

==============================================================================================================
T10. VECTORS SPECIFIED (NOT GENERATED; NOTHING RUN)
==============================================================================================================
Fixtures (F08/F09 record them): R_A, R_B, R_slot, S, T, op ids from recorded seeds; an invite per C02 T7 fixtures;
the relay store in memory; TIME vectors need a relay clock seam that does not ship (C3-O11, I13). Every refusal vector
also asserts: no row inserted or changed in any v1 or v2 table (compare a store dump), no lease changed, no bucket
drawn except where stated.
SR-19 delta symbols (names PROPOSED) and BASE: qsl-server symbols that do not exist yet -- DK v2::derive_key, CP
v2::parse_cap, MO v2::mailbox_open, VP v2::push, VL v2::pull, VA v2::ack, SA v2::slot_admit, VC v2::invite_create, VV
v2::invite_revoke (F08); VR v2::invite_redeem, VD v2::invite_deliver, VS v2::invite_settle, SW v2::sweep (F09) -- base
= the F08 or F09 commit that introduces the symbol with no refusal it does not already make, so the vector compiles and
runs RED there and GREEN on the change. EXISTING qsl-server symbols: SO store::Store::open and SI server_info, base S
main 5ea0f925. Client (qsl-protocol): CV transport::relay_v2_service_check, CB transport::relay_v2_read_bounded, CE
transport::relay_credential_for (new; base = the introducing F08-client/F10 commit); the v1 contrast symbols
relay_inbox_ack_inner and relay_auth_token exist at M a0c0c491.
Mailbox capability
| Id | Input | Expected | Delta |
|---|---|---|---|
| V01 | open(R_A); push(D_A, op1, x); pull(R_A); ack | 200 created; 200 id; item x leased; acked 1 | MO, VP, VL, VA |
| V02 | pull with Read-Cap = hex32(D_A) (deposit-only cannot read) | 404 ERR_V2_MAILBOX_UNKNOWN; K_A leases unchanged | DK, VL |
| V03 | ack with Read-Cap = hex32(D_A) | 404; the leased item remains | DK, VA |
| V04 | push with Deposit-Cap = hex32(R_A) | 404; no row created | DK, VP |
| V05 | push to an unopened D | 404; mailbox count unchanged | VP |
| V06 | cap: upper-case hex; 63 and 65 characters; leading or trailing space; b64 of the same 32 bytes; empty | 400 ERR_V2_CAP_MALFORMED (NEW), no effect | CP |
| V07 | X-QSL-Route-Token alone on each v2 endpoint; with the v2 header too | 400 ERR_V2_CAP_MISSING; 400 ERR_V2_V1_TOKEN_PRESENT | CP |
| V08 | push body max_body_bytes+1; empty body | 413 before any parse; 400 ERR_EMPTY_BODY | VP |
| V09 | push, response dropped after commit, exact retry | 200 same id; depth unchanged | VP |
| V10 | same op_id, different body | 409 ERR_V2_OPERATION_CONFLICT; depth unchanged | VP |
| V11 | exact retry of an accepted push with the mailbox at max_queue_depth; a NEW push at that depth | 200 saved; 429 ERR_OVERLOADED | VP |
| V12 | exact retry of an accepted push with the push bucket empty; a NEW push | 200 saved; 429 ERR_RATE_LIMITED | VP |
| V13 | pull with ?ack=lease or any parameter other than max; a second pull inside the lease; after the lease (TIME) | 400; 204; redelivered | VL |
| V14 | pull max 0; max V2_PULL_MAX_ITEMS+1 | 400 ERR_BAD_MAX (both) | VL |
| V15 | ack ids: none; 4097; a non-hex16 id; a repeated id | 400 ERR_BAD_ACK_IDS | VA |
| V16 | ack an unleased id | 200 acked 0; the item remains | VA |
| V17 | x-msg-id header on push | 400 ERR_V2_BAD_REQUEST | VP |
Invite slot and recovery
| Id | Input | Expected | Delta |
|---|---|---|---|
| V20 | create(R_slot); redeem(op, S, H_T); deliver(T, op, A1); pull(R_slot); ack; settle | 200 each; A1 bytes pulled equal those delivered; result bytes erased after settle | VC, VR, VD, VL, VA, VS |
| V21 | a code holder (invite_id, bearer_secret, no R_slot) pulls or acks the slot with any cap derivable from the code | 404 or 400; the slot's items untouched (the invitation grants no read) | VL, VA |
| V22 | revoke with a wrong R | 403 ERR_INVITE_REVOKE_INVALID; state unchanged | VV |
| V23 | deliver with no ticket; a wrong ticket; another claim's ticket; a right ticket with another op_id | 403 ERR_INVITE_TICKET_INVALID; no enqueue | SA |
| V24 | /v2/push to K_slot with D_slot: without a ticket; with the valid T and the same envelope as deliver | 403 ERR_INVITE_TICKET_INVALID; identical saved receipt as deliver (one canonical slot authorization) | SA |
| V25 | redeem, response dropped after commit, exact retry | 200 result byte-identical; one claim; H_T unchanged | VR |
| V26 | same op_id, different H_T; same op_id, different S | 409 ERR_V2_OPERATION_CONFLICT (both) | VR |
| V27 | another op_id with a valid redeem_cap | 409 ERR_INVITE_ALREADY_USED | VR |
| V28 | 16 concurrent first claims with distinct op ids | exactly one 200; fifteen 409 ERR_INVITE_ALREADY_USED | VR |
| V29 | exact redeem retry after the invite expiry, before recovery_until (TIME) | 200 saved result (fresh expiry not applied to a known operation) | VR |
| V30 | a NEW claimant after expiry (TIME) | 410 ERR_INVITE_EXPIRED | VR |
| V31 | exact redeem retry with the slot cap full, the create bucket empty and the result quota full | 200 saved result | VR |
| V32 | deliver, response dropped after commit, exact retry | 200 same receipt; depth unchanged | VD |
| V33 | exact deliver retry AFTER the inviter pulled and ACKed the A1 | 200 same receipt; no re-enqueue (depth 0) | VD |
| V34 | a different envelope under the same claim and ticket | 409 ERR_V2_DELIVERY_CONFLICT; no enqueue | VD |
| V35 | first delivery after the invite expiry, before recovery_until (TIME) | 200 accepted | VD |
| V36 | new or retried delivery at or after recovery_until (TIME) | 410 ERR_V2_RECOVERY_EXPIRED | VD |
| V37 | revoke BEFORE the first delivery, then deliver | revoke 200 delivered=false; deliver 410 ERR_INVITE_REVOKED; no enqueue | VV, VD |
| V38 | revoke AFTER the delivery, then an exact deliver retry and an exact redeem retry | revoke 200 delivered=true; deliver 200 saved receipt, no second enqueue; redeem 200 saved result | VV, VD, VR |
| V39 | exact deliver retry with the slot at max_queue_depth or its bucket empty | 200 saved receipt | VD |
| V40 | exact retries until the lookup bucket is empty | 429 ERR_V2_LOOKUP_RATE; no state change; the push and create buckets untouched | VR, VD |
| V41 | deliver body 12289 B | 413 before any parse | VD |
| V42 | create, response dropped, exact retry; the same invite_id with another bundle | 200 same, one slot; 409 ERR_INVITE_DUPLICATE | VC |
| V43 | create with expiry > now + max_expiry; expiry <= now | 400 ERR_V2_EXPIRY_RANGE (both; no clamp) | VC |
| V44 | settle before delivery; after delivery; then an exact redeem retry | 409 ERR_V2_NOT_DELIVERED; 200; 409 ERR_V2_RESULT_SETTLED | VS, VR |
| V45 | sweep at recovery_until; at max(expiry, recovery_until); then redeem (TIME) | result bytes erased; slot and claim rows gone; 404 | SW |
| V46 | after expiry and after the sweep: /v2/push to K_slot and deliver with no claim (TIME) | refused (404 / 403); never admitted ungated (contrast MEASURE 2.12) | SA, SW |
Added by the acceptance (F1, F3, F4); each row's delta symbol and base per the BASE rule above (F08 symbols: the
introducing F08 commit; F09 symbols: the introducing F09 commit; SO: S main 5ea0f925, existing)
| Id | Input | Expected | Delta | Base |
|---|---|---|---|---|
| V47 | open(R_A) twice | 200 created; 200 exists; one row | MO | F08 |
| V48 | open(R_slot) of a live slot's K | 409 ERR_V2_KIND_CONFLICT; no row | MO | F08 |
| V49 | open of a NEW R with mailboxes at MAX_V2_MAILBOXES; open of an EXISTING R at the cap | 429 ERR_V2_MAILBOX_CAP, no row; 200 exists | MO | F08 |
| V50 | opens until the global open bucket is empty; then an open of a NEW R and of an EXISTING R | 429 ERR_RATE_LIMITED (both: the bucket is drawn before any lookup); no row, no lookup | MO | F08 |
| V51 | open(R_A), never pulled, empty; 3600 s pass (TIME); push(D_A); open(R_A) | row removed; push 404 ERR_V2_MAILBOX_UNKNOWN; re-open 200 created | MO, SW | F08 (MO), F09 (SW) |
| V52 | open(R_A); pull once (204); empty; 3600 s pass; then MAILBOX_IDLE_SECS pass (TIME) | row kept at 3600 s; removed after MAILBOX_IDLE_SECS; push 404 | MO, SW | F08 (MO), F09 (SW) |
| V53 | (F3) create(R_slot) with a long expiry; the slot mailbox never pulled; 3600 s and MAILBOX_IDLE_SECS pass, before expiry (TIME); then redeem, deliver, pull(R_slot) | the invite_slot mailbox row is kept (exempt); deliver 200; the A1 is pulled; no orphan message, no 500 | SW, VD | F09 |
| V54 | create with slots at MAX_INVITE_SLOTS | 429 ERR_INVITE_CAP_FULL; no slot and no mailbox row; nothing evicted | VC | F08 |
| V55 | create with the create bucket empty | 429 ERR_RATE_LIMITED; no row | VC | F08 |
| V56 | create with an R_slot whose K is a kind mailbox | 409 ERR_V2_KIND_CONFLICT; no row | VC | F08 |
| V57 | create body one byte over the EP5 cap | 413 before any parse | VC | F08 |
| V58 | redeem with a wrong redeem_cap of the right length on a FRESH slot; on a CLAIMED slot | 403 ERR_INVITE_CAP_INVALID (both: R2 authenticates before any state beyond existence is reported); no row changed | VR | F09 |
| V59 | revoke(R_slot); then a NEW claimant redeems | 410 ERR_INVITE_REVOKED; no claim | VV, VR | F08 (VV), F09 (VR) |
| V60 | a NEW claimant with the retained-result quota full | 429 ERR_V2_RESULT_QUOTA; no claim | VR | F09 |
| V61 | settle with a wrong S | 403 ERR_V2_RECOVERY_INVALID; result bytes kept, settled false | VS | F09 |
| V62 | settle for an unknown invite_id; for a known slot with an unknown op_id | 404 (both); no change | VS | F09 |
| V63 | revoke an unknown invite_id; revoke(R_slot) twice | 404; 200 twice, the second changes nothing | VV | F08 |
| V64 | exact push retries until the per-K lookup bucket is empty | 429 ERR_V2_LOOKUP_RATE; no state change; the push bucket untouched | VP | F08 |
| V65 | first claim committed; relay RESTART (store reopened; in-memory buckets reset); exact redeem retry | 200 result byte-identical; one claim; recovery_until unchanged | VR, SO | F09 (VR); SO existing |
| V66 | delivery committed; relay RESTART; exact deliver retry, before and after the inviter ACKed | 200 saved receipt byte-identical; no second enqueue | VD, SO | F09 (VD); SO existing |
Cross-version addressing (T2)
| Id | Input | Expected | Delta |
|---|---|---|---|
| XV1 | v1 push with X-QSL-Route-Token = hex32(D_A) | lands in v1 routes; v2 pull(R_A) -> 204; v2 tables unchanged | DK (v1 handlers unchanged) |
| XV2 | v1 pull and v1 ack with X-QSL-Route-Token = hex32(R_A), hex32(D_A), hex32(K_A) | 204 / acked 0; v2 leases unchanged | DK |
| XV3 | v1 redeem and v1 push with invite_id = a v2 slot's invite_id | v1 404 / an ordinary v1 route; the v2 claim, ticket and messages unchanged | DK |
| XV4 | a v1-grammar token (22..128 of [A-Za-z0-9_-], not hex32) as a v2 capability | 400 ERR_V2_CAP_MALFORMED | CP |
| XV5 | a v1 token that is 64 lowercase hex, used as a v2 Read-Cap | K = derivation of its bytes: 404 unless opened as v2; never a v1 message | DK |
| XV6 | the same 32 bytes used as a v1 token string and as a v2 R | the two mailboxes' contents stay disjoint | DK |
| XV7 | source-shape test: no v1 handler names a v2 table, no v2 handler a v1 table | pass | (structural; F08) |
| XV8 | V06 representations cannot reach K | as V06 | CP |
Migration (T8)
| Id | Input | Expected | Delta / base |
|---|---|---|---|
| MG1 | the version-3 binary opens a version-2 store holding v1 rows | v2 tables exist; marker 3; v1 row count and bytes unchanged | SO; base S main 5ea0f925 (RED there: no v2 tables, marker 2) |
| MG2 | a version-2 binary opens a version-3 store | ERR_STORE_VERSION | PIN of existing behaviour (S store.rs:271-273), green at S main |
| MG3 | server-info | api contains "relay_v2"; the v2 object has every field | SI; base S main (RED) |
| MG4 | max_body_bytes 12287; retention_ttl < H_REC | startup refusal ERR_INVALID_CONFIG_V2_MAX_BODY_BYTES / _RETENTION | config symbol (new; F08) |
Client (qsl-protocol)
| Id | Input | Expected | Delta |
|---|---|---|---|
| CL1 | server-info without "relay_v2" | relay_v2_unsupported before any v2 call; the mock relay records no /v1/ mailbox or invite request | CV |
| CL2 | v2.recovery_secs != the client constant | relay_v2_unsupported | CV |
| CL3 | 404 with a non-v2 body on /v2/pull/ack | relay_v2_unsupported; never LegacyComplete (v1 contrast: M transport/mod.rs:3647) | CV |
| CL4 | pull response CAP+1 bytes; items > N_requested; an item longer than v2.max_body_bytes; an unknown field | relay_v2_response_too_large before parse; relay_v2_response_malformed (x3) | CB |
| CL5 | the relay accepts and never answers | transport failure at the deadline; the operation stays committed; the retry bytes are identical | CB |
| CL6 | two relays configured; requests to relay B | never carry relay A's credential (the mock relays record Authorization); RED at M: relay_auth_token attaches one value everywhere (M transport/mod.rs:2059-2067) | CE |
| CL7 | request capture across a whole invite flow | R only to its relay_ep; D only in the signed route_cap and on push; S, T, redeem_cap only to the invite's relay_ep; bearer_secret never | CE |
| CL8 | redeem response lost; restart; retry | identical op_id, S and H_T bytes; the result saved before C02 R5 verification | client redeem caller (F10) |
| CL9 | (F2) deliver 200 lost after the relay's commit; the inviter answers and its B1 lands in the redeemer's mailbox; the redeemer resumes after recovery_until and retries deliver FIRST (TIME) | deliver 410 ERR_V2_RECOVERY_EXPIRED; the redeemer drains its mailbox, binds the B1 (C02 T2 step 3) and the session completes; the attempt is NOT marked failed-expired; its secrets are zeroized only after selection (C02 T3) | client redeem caller (F10) |

==============================================================================================================
T11. OPEN CELLS (each with the cards it blocks). CT = CRYPTO-TOUCHING
==============================================================================================================
| Id | Question | Depends on | Blocks |
|---|---|---|---|
| C3-O1 (CT) | Label spellings and the prefix-free set (T1); allocation in the DOC-CAN-003 sec 12 table by the commit PR | SR-15, Director | F08 |
| C3-O2 | H_REC value: 259200 s PROPOSED (the tree's existing invite-lifetime default, S lib.rs:96-97); a protocol constant checked by the client (T3 C1), not operator configuration | Director | F09, F10 (C02 OC3) |
| C3-O3 | Lookup-bucket values, PUSH_RECEIPTS_PER_MAILBOX, MAILBOX_IDLE_SECS, MAX_V2_MAILBOXES (its own ceiling, F1: PROPOSED 4096 default, 65536 ceiling), V2_PULL_MAX_ITEMS, CLOCK_SLACK, the global open bucket (F1: PROPOSED burst 32, refill 1 per s, the invite-create defaults S lib.rs:99, :101) | F08/F09 measurement | F08, F09 |
| C3-O4 | Receiver tolerance of a duplicate item after push-receipt eviction (T7) | C05 | F11 |
| C3-O5 | Per-pull byte budget and client deadline values (T3 C4, C5) | C05 finite budgets | F11 |
| C3-O6 | ACK dispositions of v2 items, including a slot A1 the inviter refuses or that is left after a revoke | C05 (C02 OC6) | F11 |
| C3-O7 | Cross-relay replies: lift C02 F1 now that per-endpoint scoping (T4) exists? | Director | F10 |
| C3-O8 | CLOSED by F6: C02 T2 step 2b route_cap = exactly hex32(D), refused handshake_envelope_noncanonical (C02 AMENDMENTS AM-1) | Director (C02 amendment): RULED | F10 |
| C3-O9 | Retirement of the v1 relay endpoints (deployment decision) | operator | F18 |
| C3-O10 | Registration of the NEW relay codes (a new qsl-server contract document) and the NEW client codes (DOC-SCL-002) | implementing PRs | F08-F10, F12 |
| C3-O11 | A relay clock seam for TIME vectors that does not ship (I13) | F09 design | F09 |
| C3-O12 | Vault key spellings for R, S, T and the per-endpoint credential map | C01 O5 | F08 (client), F10 |
| C3-O13 | One mailbox per (identity, relay) shared by all peers (today's shape, PROPOSED) vs one per peer: per-peer isolates depth denial and allows per-peer revocation of D, at the dispatcher's cost | C05 | F08, F11 |
| C3-O14 | Whether E5's changes to the server's own invite-slot contract need a server-side ruling beyond THE PLAN | Director | F08, F09 |
CRYPTO-TOUCHING ROWS: T1 rows 1-3, 6, 8, 9 and the label set; T4 digests Q_R, P, Q_C; C3-O1. Required: SR-15 independent
sensitive design review (fable/xhigh) of this contract, and the same at the implementing PRs (F08 "independent security
review", F09 "independent sensitive review").
SR-18 CENSUS LINE: F08/F09 (relay) and F10 (client) remap observables and each owes the census at drafting: the
server-info api list; the relay code strings (ERR_V2_* NEW; ERR_INVITE_* reused with v2 meanings); the client's ACK 404
mapping (LegacyComplete, not on the v2 path); tui.relay.inbox_token (RETIRED on the successor path); the relay
credential lookup (relay_auth_token); relay_seen_ids_* naming (C01 CENSUS D20, class O6). v1 relay observables and
their pins (for example S:tests/na0678_invite_slots.rs:461-468 "pull is deliberately ungated") are UNCHANGED.

==============================================================================================================
ESCALATIONS RAISED WITH THIS DRAFT (named, NOT resolved by choice; the same list heads the drafting mission's REPORT.md, sha256 e72b36f2...)
==============================================================================================================
| Id | Conflict | Between | Draft's position (a proposal only); RULING (RULING_NA0783_C03_ACCEPT) |
|---|---|---|---|
| E1 | The directive states "max_body_bytes 65536" as a relay fact. Measured: the qsl-server SOURCE default and ceiling are 1048576 (S lib.rs:114, :125, :148); 65536 is the deployed rig's CONFIGURED value (qsl-protocol docs/ops/RIG_PROVISION_RUNBOOK.md:65, :136, :247) | directive / tree | the contract uses the configured value and requires >= 12288 for relay_v2 (T8 M9); both values satisfy it. RULED: 65536 is the rig's configured max_body_bytes, not a source fact (source default and ceiling 1048576); the contract needs >= 12288 |
| E2 | C02 (accepted) F2 source note and T2 L0 say the relay "checks expiry on ITS clock at push and burns the ticket (store.rs:633-635)". By source reading, /v1/push sweeps first (S lib.rs:1055-1062 -> store.rs:360-366 -> :343-345), deleting the expired slot row, and enqueue then admits the push as an ORDINARY UNGATED route (store.rs:621-629, None arm). NOT RUN; no test covers it. C02's own steps 4-5 (bearer tag, signature) still refuse such a frame before effects, so C02's safety claim stands; its relay premise does not, and today's v1 relay admits ticketless pushes into an expired slot | tree / C02 | v2 closes it (T2 N5, V46); the v1 behaviour is outside C03's scope and is reported as a finding. RULED: CONFIRMED and stronger -- SlotReject::Expired is UNREACHABLE on /v1/push (sweep and admission use one captured now), so DOC-SRV-007:93's 410 does not occur; exposure LOW, latent v1 contract defect MAJOR; filed as ENG-0356 with the named one-line repair and the missing push-after-expiry test (repair NOT done here); C02's F2 source note corrected (C02 AMENDMENTS AM-2; its safety claim stands); v2 closure N5/V46 accepted |
| E3 | C03 needs route_cap = exactly hex32(D); C02 T2 step 2b admits any 22..128 of [A-Za-z0-9_-]. Without an amendment a peer can sign a route_cap the v2 relay refuses, discovered only at push time, after occupancy | C02 / C03 | proposed C02 amendment (C3-O8); not applied here. RULED (= F6): C02 AMENDMENTS AM-1, route_cap = exactly hex32(D), refused handshake_envelope_noncanonical; C02 T7 M12 re-expected; C3-O8 closed |
| E4 | THE PLAN sec 3 "Push uses D; leased pull/ACK use R" and "Apply this split to invitation slots too" vs C02 T1a (accepted), whose payload has no field for a slot deposit key. C03 reads the split for slots as: the slot is read with R_slot only; the invitation grants redemption and ONE ticket-gated deposit addressed by the locator L; a push by D_slot runs the same slot_admit. Deposit into a slot is therefore not a presentation of D | THE PLAN / C02 | the reading above; it is the Director's to confirm. RULED: CONFIRMED; no C02 payload change; V24 pins it |
| E5 | qsl-server's own contract DOC-SRV-007 (tombstoning "normative": bundle cleared at consumption; relay-minted ticket; relay-minted revoke_token; expiry clamped, S DOC-SRV-007:36-112) vs THE PLAN sec 2 (bundle kept through recovery; client-minted ticket) and this draft (revoke by R_slot, expiry refused not clamped). v1 is unchanged; v2 needs its own server contract document | tree / THE PLAN / C03 | v2-only changes, listed; C3-O10, C3-O14. RULED: a DRAFT v2 server contract document, qsl-server docs/server/DOC-SRV-008, states the four deltas against DOC-SRV-007 by section; DOC-SRV-007 gains one pointer line; C3-O14 stays OPEN |

END OF C03 FINAL

==============================================================================================================
AMENDMENTS (appended by D-1429; the text above is C03 FINAL as merged by D-1428 and is NOT rewritten)
==============================================================================================================
Ruled by RULING_NA0783_F01_audit_2026-09-24 (sha256 642a9ff82d42936e16c93136fd230dcc11327a7d3d7fc3c0e65c179f4e09e61d) on the combined
adversarial audit of C01, C02 and C03 as a set, AUDIT_F01_C01_C03_FINDINGS (sha256
512d42751d7a7dc3b26fa68fb100a7bf5c812f646e44fad47adc80bb142001c2; fable/xhigh; 26 findings, 21 NEW; no BLOCKER; one MAJOR,
X1 = S3.1). Result class for the audit: F01_COMBINED_AUDIT_ONE_MAJOR_AMENDED. Each amendment names its finding and the
cell it amends; where an amendment and the text above disagree, the amendment governs. PROPOSED keeps the meaning the
FIXES section gives it. Citations: M = qsl-protocol main d5de33d7 (client qsl/qsl-client/qsc/src/<file>:<line>); P28 =
#1828 e29a07df; S = qsl-server main 5ea0f925 (its src/ is byte-identical at 0c04fa47). The companion amendments are C02
AMENDMENTS AM-3..AM-9 and C01 AMENDMENT AM-4.
EDITED IN PLACE BEFORE MERGE: every row whose Id cites an FX id was edited by RULING_NA0783_F01_amend_read_2026-09-24
(sha256 28a6caa1a0fc8e87aee0197b4b8b6c4bfd95e73415f9336e4152e01aeaed1492) on the SR-15 delta read SR15_F01_AMEND_FINDINGS (sha256
74a37b30537cf249076673bee1fec69b026d4b34acd2bdcbc286de24e9f299d0); AM-16 is appended by that ruling (FX4).
| Id | Amends | Amendment |
|---|---|---|
| AM-1 (X1 / S3.1 edit (1)) | T3 EP3, the 200 response | For a mailbox of kind invite_slot every returned item carries one more field, the recovery_until of the slot's single claim (T5; set once, T6): {"id":hex16,"data":b64,"recovery_until":u64}. Items of a kind mailbox are unchanged ({"id","data"}). The field belongs to the exact field set of a slot pull (C3 strict JSON). It is the only route by which the INVITER learns recovery_until: EP6 returns it to the redeemer only, C02 T3 adds no timestamp to any envelope, and the client's HandshakePending carries none (M handshake/mod.rs:161-190) |
| AM-2 (X1 / S3.1 edit (2); FX2, FX4, FX8) | T6 "Client horizons", the INVITER (its last sentence, "processes an admitted A1 until invite expiry + H_REC", stays a true upper bound on every recovery_until but is no longer the inviter's horizon) | The INVITER's horizon is the claim's recovery_until, read from the pulled slot item (AM-1), on the inviter's own clock, CLAMPED (FX4): RU_I = min(item.recovery_until, invite expiry + H_REC); a slot item whose recovery_until exceeds invite expiry + H_REC is refused as relay_v2_response_malformed (T3 C3) and disposed (disposition C05, C3-O6). (a) An A1 is answered (C02 T2 step 10) only while now < RU_I; otherwise no candidate, contact row or B1 is created and the A1 is disposed (disposition C05, C3-O6). (b) A responder candidate for which no A2 has been selected is failed-expired at RU_I + 2 x CLOCK_SLACK (FX2; this value supersedes the ruled "recovery_until + CLOCK_SLACK"), strictly after the redeemer's last binding instant for the one recovery_until an honest relay serves both sides (T6 (b) with AM-16: at most recovery_until + CLOCK_SLACK on its clock); BEFORE marking it failed-expired the inviter pulls its inbox and binds any A2 present (C02 T2 A2-1..A2-6), the mirror of F2 (FX2). In the same commit as the failure the invite record becomes terminal (Expired; a record already Redeemed stays Redeemed) (InviteState, M invite/mod.rs:601-607; FX8) and the candidate's secrets are zeroized (C02 T3, F5c). (c) An ADVANCED candidate (an A2 selection written OR a session committed -- two commits, P28 handshake/mod.rs:829-838 then :636, and the horizon never runs between them; FX8) is never reset or zeroized by this horizon (as F2 for the redeemer; P28 handshake/mod.rs:630-631). (d) A candidate failed-expired here releases its first-connection generation as C02 AMENDMENTS AM-3 states. (e) CLOCK_SLACK applies on the failing side only; the answer window (a) is strict on purpose (FX8). (f) A 404 on a slot pull means the slot was swept (T7): the invite record becomes terminal (Expired; a record already Redeemed stays Redeemed) and nothing is re-opened (FX8). CLAIM BOUNDARY (FX2): an A2 that reaches the inviter's inbox only after its final drain leaves the redeemer with a session the inviter never applied -- the last-message problem, stated, not closed; the margin between the redeemer's last binding instant and the inviter's final drain is CLOCK_SLACK less the two clocks' disagreement and the A2's delivery time. Without (a)-(d), measured at P28: a first-connection record has no expiry and no terminal state short of applied (handshake/mod.rs:418-425), no entry is ever removed (entries are only put, :579-597), and a new record for the pair is refused while a pending exists (:535-541, handshake_lifecycle_conflict; reached from the receive path :963 and the outgoing path :2439), so a stranded candidate blocked the pair forever |
| AM-3 (X1 / S3.1 edit (3); FX7) | T7 row "slot mailbox messages", cleanup deadline (F3); T10 (new V68) | A slot mailbox message is removed by ACK or, at the latest, by the sweep at its claim's recovery_until -- no longer kept with the invite_slot row to max(expiry, recovery_until) -- so the relay stops serving an A1 whose claim's horizon has passed. The claim tombstone keeps E: an exact deliver retry after recovery_until still answers 410 ERR_V2_RECOVERY_EXPIRED and a changed envelope 409 ERR_V2_DELIVERY_CONFLICT (T6 D4); nothing is re-enqueued. The slots_v2 row, its kind invite_slot mailbox row and the tombstone keep their deadlines, and F3's exemption from the retention TTL stands. V68 (TIME) (FX7): redeem; deliver; the sweep at the claim's recovery_until: the slot message is gone (pull(R_slot) 204), and the slots_v2 row, its kind invite_slot mailbox row and the tombstone remain; an exact deliver retry -> 410 ERR_V2_RECOVERY_EXPIRED; a changed envelope -> 409 ERR_V2_DELIVERY_CONFLICT; at max(expiry, recovery_until) -> 404 (V45). Delta SW, VD; base F09 |
| AM-4 (X1 / S3.1 edit (5); S7.1) | T9 OC3 | The answer "client-side receipt and exact-reply retention until recovery_until then zeroize (C02 T3)" is replaced: each side's retention ends at the horizon T6 fixes for it -- the redeemer's (T6, F2) and the inviter's (AM-2), recovery_until reaching the inviter on the slot pull (AM-1). recovery_until is thereby observable by BOTH sides, which the replaced answer assumed and the text above did not provide. State: CLOSED by AM-1 and AM-2; value OPEN (C3-O2) |
| AM-5 (X1 / S3.1 edit (6); FX2, FX6, FX8) | T10 Client vectors (new CL10, CL11, CL12, CL13) | CL10 (TIME): redeem and deliver; the inviter stays offline past recovery_until, then pulls. Expected: the relay serves no A1 (removed at recovery_until, AM-3): pull(R_slot) 204 before the slot sweep, 404 after it (at max(expiry, recovery_until), T7), and on the 404 the invite record becomes terminal (AM-2 (f)) (FX8); a copy leased before recovery_until and processed after it by the inviter's clock (fixture) is disposed (AM-2 (a)); either way no candidate, no contact row, no B1. Delta: client accept caller (F10). CL11 (TIME): the inviter commits a responder candidate and pushes its B1; no A2 arrives; recovery_until + 2 x CLOCK_SLACK passes on the inviter's clock (FX2). Expected: the inviter first drains its inbox (no A2); then in one commit the candidate is failed-expired, its secrets zeroized and the invite Expired; then a new invite between the same pair completes (a new generation, C02 AMENDMENTS AM-3, vector X6). Delta: client accept caller (F10); C02's LP, LS. CL12 (TIME) (FX2): the redeemer binds the B1 at its last binding instant (recovery_until + CLOCK_SLACK by its clock), commits its session and pushes the A2; the inviter's clock runs ahead of the redeemer's by less than CLOCK_SLACK (fixture) and no ordinary pass pulls the A2 before the inviter's horizon. Expected: at recovery_until + 2 x CLOCK_SLACK by the inviter's clock the horizon pass drains first, binds the A2 (C02 A2-1..A2-6) and applies the session; nothing is failed-expired, the invite is not Expired, one session on both sides. Arm: the relay withholds the A2 until after that drain (fixture) -> the candidate is failed-expired and the late A2 is refused at C02 A2-2 (the stated last-message residual, AM-2). Delta: client accept caller (F10). CL13 (TIME) (FX6): I_A holds a live responder candidate for I_B (generation g1; its B1 lost); I_B's g1 attempt fails at its own horizon and releases; I_B redeems a NEW invite from I_A and the new A1 reaches I_A before the g1 candidate's horizon. Expected: C02 T2 step 10 handshake_lifecycle_occupied, disposition DEFER (retained un-ACKed, no write; C02 AMENDMENTS AM-3); at the g1 horizon the candidate is failed-expired and the record released; the next pass processes the retained A1: a new generation, a responder candidate and a B1; the session completes and the new invite is not burned. Delta: client accept caller (F10); C02's LP, LS |
| AM-6 (S1.1) | T3 C1; T10 CL2 | C1 also requires v2.max_body_bytes >= 12288 (= C02 ENV_MAX: the B1 envelope, <= 11729 B, travels by /v2/push), v2.pull_max_items >= 1 and v2.lease_secs >= 1; otherwise relay_v2_unsupported, before any v2 call. The client no longer relies on the relay's own startup guard (T8 M9) for the floor: without it, a relay advertising a smaller max_body_bytes refuses the committed B1 with 413 ERR_TOO_LARGE (the S lib.rs:1041-1043 shape) only AFTER the inviter's step-10 occupancy. server-info sits behind the bearer gate (S lib.rs:895-905 answers 401 with a body that carries no "api"), so a 401 on server-info is relay_unauthorized for that endpoint (T4), not relay_v2_unsupported. CL2 gains the arm "v2.max_body_bytes = 12287 -> relay_v2_unsupported, before any v2 call" |
| AM-7 (S2.2; FX9) | T4 row "Relay bearer credential, per endpoint"; T10 CL6 | The map key is the CANONICAL relay_ep (C02 T1a grammar G1-G7) of the configured endpoint, CANONICALIZED ONCE AT LOAD (FX9; this replaces the row's former refuse-at-load arm): a trailing '/' (G6), an upper-case scheme or host (G1, G2) and an explicit default port (G4) are normalized, so the key can never differ from the canonical relay_ep a request is matched against (today the base is user-typed and normalized per call, M transport/mod.rs:2553-2555). relay_endpoint_noncanonical (NEW client code; registration C3-O10) is kept ONLY for a configured value that cannot be canonicalized under G1-G7 (for example userinfo, a query or a fragment, G3; percent-encoding or a dot segment, G5; "http" to a non-loopback host, G1): it refuses at load, before any request. CL6 asserts both arms: "a configured endpoint spelled non-canonically (a trailing '/', an upper-case scheme or host, an explicit default port) -> loaded under its canonical key; requests to that relay carry its credential and no other" and "a configured value that cannot be canonicalized under G1-G7 -> relay_endpoint_noncanonical at load; no request is made" |
| AM-8 (S3.2 (and S4.2); FX2, FX5) | T5 client table (new step L2b); T3 client obligations (new C6); T10 (new V67) | L2b (narrowed by FX5): the redeemer opens its reply mailbox (EP1, idempotent) in the pass that commits the operation (L1), again before the first pull after any gap of 3600 s or more without a successful pull, and whenever a pull or push naming that mailbox is answered 404 ERR_V2_MAILBOX_UNKNOWN; a successful pull keeps the row alive (T7); so the mailbox its signed route_cap names exists while the attempt is live (a never-pulled mailbox is removed after 3600 s, T7 F1). C6: a committed handshake reply (B1 or A2) whose push is answered 404 ERR_V2_MAILBOX_UNKNOWN or 429 is retried with the same op_id and the same bytes on each later pass until the INVITER's candidate deadline, recovery_until + 2 x CLOCK_SLACK (AM-2), each side by its own clock: the inviter for its B1, the redeemer for its A2, from its own recovery_until (AM-16) (FX2); the disposition vocabulary is C05's. V67 (TIME): open(R_X); deliver the A1; 3600 s pass with R_X never pulled (row removed); the inviter's B1 push -> 404 ERR_V2_MAILBOX_UNKNOWN; R_X re-opens (L2b); the B1 push retried with the same op_id and bytes -> 200. Delta MO, VP, SW; base F08 (MO, VP), F09 (SW) |
| AM-9 (S5.1, S2.4) | T1 holder table, last row (relay operator) | The relay operator also READS EVERY ENVELOPE FIELD: the A1 and B1 envelopes are signed, not encrypted (C02 T1b), so it sees both identities' public keys (inner kem_pk and sig_pk), both commitments, invite_id, attempt_id, relay_ep and both deposit capabilities (route_cap = hex32(D), hence K); with request timing and source addresses it learns the CONTACT GRAPH (every later push carries the recipient's D). It can INJECT frames into any mailbox whose D it has seen: the receiver refuses them (C02 T2 steps 1-6), but each costs the reader a pull and a parse (bounded by T3 C3 and C05's budgets). Only the inner handshake secrets and message bodies are protected. v1 exposes the same set today (the QSLH-1 envelope carries the bundle, route token and A1 in clear, M invite/mod.rs:474-546); v2 does not worsen it |
| AM-10 (S3.4) | T4 row "Client storage", the terminal list | The terminal list "(applied, failed-expired, revoked)" gains "superseded" (C02 T4 X-c). Because settle presents S (EP8) and C02 AMENDMENTS AM-7 has the crossing loser settle its superseded claim on the next dispatcher pass, a superseded operation's T is zeroized in the commit that marks it superseded and its S in the commit that records the settle's answer, or at its horizon (T6) if no answer comes |
| AM-11 (S2.3) | T1 row 10 (op_id) | Note: "op_id = attempt_id, one identity" is ASSERTED by the redeemer; neither the relay (the EP7 body is opaque to it) nor the inviter (it never sees op_id) can verify it. A redeemer that used two values harms only itself (C02 RP2 on retry; the B1 binding at C02 T2 step 3). The cross-party binding that holds is E = H(exact envelope) (T4, T5) |
| AM-12 (S4.1) | T5 client table (a new INVITER line I0, before L1) | I0 (the inviter; today's order, M invite/mod.rs:597-599, :925, :944): the invite record in state Creating, with R_slot, is committed BEFORE EP5; it becomes Active after the 200; a crash between the two is recovered by an exact create retry, which returns the saved response (T6 Create, the KNOWN arm, Q_C) |
| AM-13 (S7.2) | T7 "Sweep" paragraph, the rationale of retention_ttl >= H_REC | The rule (T8 M9) stands; its rationale is reworded. Slot mailbox messages are owned by the slot sweep (F3; AM-3), not by the retention TTL, so the rule serves ORDINARY mailboxes: a B1 or A2 pushed at time t >= claimed_at survives unACKed to t + retention_ttl >= claimed_at + H_REC = recovery_until, so a reply pushed after the claim outlives the operation's recovery_until |
| AM-14 (S5.3, S5.4; FX5) | T11 C3-O3; T7 slots_v2 row | C3-O3 gains two sizing inequalities for F08/F09's measured values ("the cap bounds storage, the bucket bounds denial" holds only when they hold). (i) Mailboxes: the kind-mailbox rows a bearer holder can keep alive at steady state number at most open burst + open refill x 3600 s (the never-pulled horizon), so MAX_V2_MAILBOXES > open burst + open refill x 3600 + the expected population; at the PROPOSED values 32 + 3600 = 3632 of 4096, leaving 464. (i) also counts the LEGITIMATE draws on the same global bucket (FX5): L2b opens at most once per live redeem operation per 3600 s (AM-8), plus once after each 404, so the open refill x 3600 s must also carry the expected live redeem operations. (ii) Slots: a self-claimed slot holds a MAX_INVITE_SLOTS entry (shared by every bearer holder, S lib.rs:92-93) to max(expiry, recovery_until) <= expiry + H_REC, so MAX_INVITE_SLOTS > create burst + create refill x (max invite expiry + H_REC) + the expected population; at the source defaults (burst 32, refill 1 per s, S lib.rs:99-101; 259200 s + 259200 s) the right side is 518432, above the 4096 ceiling, so on v2, as on v1 over its 259200 s (259232 > 256), the create bucket alone does not bound slot denial -- a sizing fact for F08/F09 (the relay serves a closed user group today). T7 slots_v2 row: the hold window of a claimed slot is up to expiry + H_REC (about 144 h at the PROPOSED defaults) against v1's expiry (72 h default; DOC-SRV-007 "Tombstoning": slots persist until expiry): DOUBLED |
| AM-15 (S3.5) | T6 "Client horizons" (the redeemer): one claim-boundary sentence | A vault restored to a state before L3 cannot recover its in-flight redemption (the relay remembers the first seal: 409 ERR_V2_DELIVERY_CONFLICT or ERR_INVITE_ALREADY_USED, T5; I04 forbids a second) and surfaces it as failed-expired, not "connecting"; F02's starting input counts in-flight invitation operations in the covered restore domain (THE PLAN amendment A4) |
| AM-16 (SR-15 delta read N4; FX4) | T6 "Client horizons", the REDEEMER's (b): a new appended item; the text of (b) is not rewritten | recovery_until as EP6 returns it is relay-supplied and unauthenticated, so the redeemer's horizon (b) is CLAMPED: min(recovery_until, its own L1 time + H_REC) + CLOCK_SLACK, by its own clock (the no-result arm, L1 time + H_REC + CLOCK_SLACK, is unchanged). Without the clamp a relay that returns a far-future recovery_until holds the redeemer's attempt, and so the pair's generation, for as long as it likes; a past value only ends the attempt early (equivalent to dropping) |

END OF C03 AMENDMENTS

==============================================================================================================
AMENDMENTS AM-17 ONWARD (appended by D-1431; the text above, C03 FINAL and AM-1..AM-16, is NOT rewritten)
==============================================================================================================
Ruled by RULING_NA0783_C04_ACCEPT_2026-09-24 (sha256 c99b476ad0d3236528dcd5438bc5207cbc46061a6368e8d295d305e67307842e) at the C04
acceptance (docs/ops/contracts/C04_ownership_and_accounting.md): AM-17 is the C04 draft's E1 with the SR-15 read's X7;
AM-18 is its E7; AM-19..AM-24 are the owed items R-1..R-8 of SR15_F01_FIXUP_FINDINGS (sha256
474de737e18c24bdee0e6986b735d67788e4960065ce0e5a0aca427ecd05e5c8, section 8), owed to this ruling by
RULING_NA0783_F01_PR1841_2026-09-24 (sha256 41ec1f8da9b63c238ed2cab4f88c010e40929470ffc876a388d8f306e2a8a658) R2 (R-1 takes the
VALUE; R-2 through R-8 as worded). Each amendment names its source and the cell it amends; where an amendment and the
text above disagree, the amendment governs. PROPOSED keeps the meaning the FIXES section gives it. Citations: C = #1831
ffc8fc52 (client qsl/qsl-client/qsc/src/<file>:<line>); M = qsl-protocol main 1fbaa814; P28 = #1828 e29a07df. The
companion C02 amendments are C02 AMENDMENTS AM-10..AM-15.
| Id | Amends | Amendment |
|---|---|---|
| AM-17 (C04 E1 + X7) | T1 row 10 (op_id), "A push: 16 bytes CSPRNG per queued item, fixed when the item is committed (C04/C05 own the queue row)" | The op_id of EVERY v2 push is 16 bytes CSPRNG minted in the durable commit that fixes the pushed bytes and stored with them. Its holders are THREE: the Flight (the PREPARED commit; application and maintenance flights), the Disposition (the receive commit; receipts) and the invitation candidate (the C02 T2 R6 / step 10 commit; the handshake replies B1 and A2, which C6 retries "with the same op_id and the same bytes", AM-8). The queue row carries a copy for application sends. Receipts and controls have no queue row (C04 T4 Q09), so "per queued item" no longer defines the holder. A redemption's op_id = attempt_id is unchanged (T1 row 10; AM-11). Byte effect: + 16 B raw per Flight and Disposition, inside C04 S04/S05 (F06) |
| AM-18 (C04 E7) | T3 C1 (with AM-6); T10 CL2 | AM-6's floor v2.max_body_bytes >= 12288 is raised to v2.max_body_bytes >= MAX_WIRE = 65536 (the largest NDE1 wire, C core:18; C04 S01), so a relay that passes C1 can take every wire the client may seal; otherwise relay_v2_unsupported, before any v2 call. The pull_max_items and lease_secs floors of AM-6 are unchanged. C04 T4 Q11 is KEPT as the seal-time guard against a relay reconfigured after C1 ran, and a 413 after sealing leaves the row PREPARED (C04 Q01). CL2's AM-6 arm reads "v2.max_body_bytes = 65535 -> relay_v2_unsupported, before any v2 call". The rig's configured 65536 meets the floor (C03 E1); the qsl-server source default 1048576 exceeds it. C04 C4-O17 closes. DOC-CAN-003 row C03-04-AM2 |
| AM-19 (R-1; N-A) | AM-2 (b); AM-2 CLAIM BOUNDARY; AM-5 CL11, CL12; AM-8 C6 | AM-2 (b): a responder candidate for which no A2 has been selected is failed-expired at RU_I + 3 x CLOCK_SLACK (this value supersedes FX2's RU_I + 2 x CLOCK_SLACK), which holds for any two clocks each within CLOCK_SLACK of the relay's; the rest of (b) is unchanged. The figures follow: AM-5 CL11 "recovery_until + 2 x CLOCK_SLACK passes" and CL12 "at recovery_until + 2 x CLOCK_SLACK by the inviter's clock" read 3 x CLOCK_SLACK; AM-8 C6's "the INVITER's candidate deadline, recovery_until + 2 x CLOCK_SLACK (AM-2)" reads 3 x CLOCK_SLACK; C02 AM-3's two figures are C02 AMENDMENTS AM-11. AM-2 CLAIM BOUNDARY's margin, "CLOCK_SLACK less the two clocks' disagreement and the A2's delivery time", reads "2 x CLOCK_SLACK less the two clocks' disagreement and the A2's delivery time" (the same arithmetic at the new value) |
| AM-20 (R-2; N-B) | AM-2 CLAIM BOUNDARY (one sentence added) | The same state is reached by an A2 whose C6 retries exhaust at the deadline, by a relay serving the two sides different recovery_until values, and by clocks disagreeing by 2 x CLOCK_SLACK or more (the read's "CLOCK_SLACK or more" at the FX2 value; the figure follows AM-19); all inside the relay's stated power to drop or delay. With C02 AM-4 (FX3) no invitation in either direction rebuilds the pair until OC7. The step-10 mirror of FX3 is C02 AMENDMENTS AM-12 |
| AM-21 (R-3; N-C) | AM-7 (one sentence); T10 CL6 | Two configured values that canonicalize to one key within one source refuse at load (relay_endpoint_duplicate, NEW; or the existing config-conflict code, C3-O10); across sources the key holds the credential of today's precedence -- environment, then the vault secret, then the token file (M transport/mod.rs:2059-2067). CL6 gains the arm "two spellings of one relay configured -> refused at load; no request is made" |
| AM-22 (R-4; N-D) | AM-7; AM-2 | AM-7: exactly these three normalizations; any other deviation from G1-G7 refuses. AM-2: after the clamp RU_I = min(item.recovery_until, invite expiry + H_REC), "(the refusal is the clamp; an accepted item's recovery_until is RU_I)" |
| AM-23 (R-5; N-E) | AM-5 CL13 (one clause) | The retained A1 outlives the g1 horizon at the relay because its own claim's recovery_until exceeds RU_1 + 3 x CLOCK_SLACK (H_REC > 2 x CLOCK_SLACK). The read wrote "RU_1 + 2 x CLOCK_SLACK (H_REC > CLOCK_SLACK)" at the FX2 value; the figures follow AM-19 (its section 4 bound: the new claim's recovery_until is at least RU_1 + CLOCK_SLACK + H_REC). True at the PROPOSED values (259200 > 7200). The C02 part of R-5 is C02 AMENDMENTS AM-13 |
| AM-24 (R-7; N-G) | AM-8 C6 | "from its own recovery_until (AM-16)" reads "from its clamped recovery_until (AM-16)" |

END OF C03 AMENDMENTS AM-17 ONWARD

==============================================================================================================
AMENDMENTS AM-25 ONWARD (appended by D-1433; the text above, C03 FINAL and AM-1..AM-24, is NOT rewritten)
==============================================================================================================
Ruled by RULING_NA0783_C05_ACCEPT_2026-09-24 (sha256 c135b67538405af40cab8410c3970b2bbf97d7e8b7dd6757dcb2aaa916ae02ea) at the C05
acceptance (docs/ops/contracts/C05_dispatcher_and_gui.md): AM-25 is the C03 C1 amendment the ruling's F1 includes (the
SR-15 C05 read's X1 and fix F1 (a), SR15_C05_FINDINGS sha256
cc5c69d77ee0be110126cca1d77ae69197494f403690f5fb1f035ef218bd5174); AM-26 is the C05 draft's E9, ruled "by F1". Each
amendment names its source and the cell it amends; where an amendment and the text above disagree, the amendment
governs. Citations: S = qsl-server 5ea0f925 (the revision the read cites); M = qsl-protocol main 4e9dfd0e.
| Id | Amends | Amendment |
|---|---|---|
| AM-25 (C05 F1; SR-15 C05 X1) | T3 C1 (with AM-6 and AM-18); T10 CL2 | C1 also requires v2.lease_secs >= T_PASS + the ack deadline + LEASE_MARGIN (C05 T3 BU12 (a): T_PASS is the dispatcher's per-pass soft wall, C05 BU4; the ack deadline is 15 s, C4; LEASE_MARGIN is a stated margin; T_PASS and LEASE_MARGIN are HYPOTHESIS values, C05 C5-O1); otherwise relay_v2_unsupported, before any v2 call. This floor supersedes AM-6's v2.lease_secs >= 1. Why: EP4 deletes LEASED copies only and V16 pins "ack an unleased id -> 200 acked 0; the item remains", so an ACK flushed after the item's lease expired is lost and the item returns every lease. The client obligations that go with it are C05's: the ACK set of a mailbox is flushed right after that mailbox's items are processed, and per-mailbox processing is bounded to v2.lease_secs / 2, items past it left un-ACKed (HOLD) (C05 D-R5, BU12 (b)). Stated arithmetic: at C05's PROPOSED T_PASS 60 s the floor exceeds 75 s, above the v1 relay's shipped default pull lease of 60 s (S src/store.rs:7 PULL_LEASE_SECS_DEFAULT; ceiling 3600, :8); the v2 relay's lease default (DOC-SRV-008, F08) or F11's T_PASS must meet it. CL2 gains the arm "v2.lease_secs below the floor -> relay_v2_unsupported, before any v2 call"; C05 T8 V1125 is the dispatcher's vector. DOC-CAN-003 row C03-04-AM3 |
| AM-26 (C05 E9, by F1) | T3 C1 (with AM-6 and AM-18); T10 CL2 | C1 also requires v2.max_body_bytes <= 1048576, the relay's own source ceiling (S src/lib.rs:113 MAX_BODY_BYTES_CEILING = 1024 * 1024; E1), so the pull CAP of C3, which scales with v2.max_body_bytes, has a client-side bound; otherwise relay_v2_unsupported, before any v2 call. AM-18's floor v2.max_body_bytes >= 65536 is unchanged. CL2 gains the arm "v2.max_body_bytes = 1048577 -> relay_v2_unsupported, before any v2 call". C05 C5-O14 closes. DOC-CAN-003 row C03-04-AM3 |

END OF C03 AMENDMENTS AM-25 ONWARD
