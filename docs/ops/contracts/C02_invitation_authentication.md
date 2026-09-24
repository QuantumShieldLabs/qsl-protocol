C02 -- INVITATION AUTHENTICATION -- FINAL (ACCEPTED WITH NAMED FIXES: CONTRACT_ACCEPTED_WITH_NAMED_FIXES)

==============================================================================================================
FIXES (draft -> FINAL), keyed to RULING_NA0783_C02_ACCEPT_2026-09-23 (sha256
d3f37903f8bacbcb167afe8bd167a74f303a06b45a658023cbe95661f5d5d795)
==============================================================================================================
Status: C02 ACCEPTED (contract) WITH NAMED FIXES; result class CONTRACT_ACCEPTED_WITH_NAMED_FIXES. The Director's
ruling, on the SR-15 cold read (SR15_C02_FINDINGS.md sha256
106fb421f745d4360f302e69a24e04b4ad289afee81c6d6a630931eb8058b62d, recommendation ACCEPT WITH NAMED FIXES, no BLOCKER),
mandates exactly the fixes F1-F8 and the rulings E1-E7 below; nothing else in the draft changes. The body below is the
draft (C02_DRAFT.md sha256 fa78ddb5e6eaae674b194f9cffd9633688a96f4afaf885a28cb75e6f2433fe8a) with these applied. The
word PROPOSED in the body is retained verbatim and now reads: the accepted contract value, NOT YET ALLOCATED;
identifiers become ALLOCATED only when the DOC-CAN-003 sec 12 rows for C02 merge. F01 stays ACTIVE. Every fix was
verified at the source before it was written (qsl/qsl-client/qsc/src/<file>:<line> at M = 87aec475 unless stated;
qsl-server at 5ea0f925; P28 = #1828 e29a07df); where SR-15's line numbers differ from the source, the source is cited.
| Key | Fix | Where | Source verified |
|---|---|---|---|
| F1 (X1) | Both envelopes' relay_ep MUST EQUAL the invitation payload's relay_ep, refused at step 3 as handshake_envelope_binding. Cross-relay replies DEFERRED to C03 with per-endpoint credential scoping as the named precondition (OC15). M09 re-expected; new B1 vector B08 | T1b row 9 and the note under T1b; T1e; T2 step 3; T7 M09, B08; T8 OC15 | transport/mod.rs:2059-2067 relay_auth_token (env, then the vault secret, then the token file); attached as "Authorization: Bearer" at :2996-2999 (relay_inbox_push_inner :2966) and :4246-4248 (invite_post; the ENG-0051 comment :4242-4245); qsl-server src/lib.rs:782 auth_ok against the relay-wide st.relay_token; today every invitation push goes to the invite's relay (invite.rs:1299-1310, :1517, :1868; handshake/mod.rs:2693, :2216) |
| F2 (X2) | Expiry refuses NEW claimants only; a known operation is looked up and resumed BEFORE the expiry refusal, until C03's recovery horizon: redeem R2b before R3; the inviter's L0 pulls an Active slot and processes an admitted A1 whatever the local expiry. Both vectors (EX1, EX2) | T2 L0, R2b, R3; T7 EX1, EX2 | THE PLAN sec 2 (docs/ops/PLAN_QSL_successor_rev3.md:352) "resume a known operation before rejecting it merely because the original invitation has now expired"; accept refuses expiry before the pull (invite.rs:1514-1516; pull :1519); redeem :1228; the relay checks expiry on ITS clock at push and burns the ticket (qsl-server store.rs:633-635, :645-650); default TTL 72 h (invite.rs:715) |
| F3 (X3) | B1 verified with the sig_pk of the bundle verified at R5; step 3 binds inner sig_pk == that key; B04 re-expected. The OPTIONAL outer B1 tag is NOT taken | T2 steps 3, 5; T7 B04 | verify_redeemed_bundle returns the authenticated (kem_pk, sig_pk) (invite.rs:566-585); M's B1 path verifies with the wire's resp.sig_pk (handshake/mod.rs:2093-2094) after the decap :2015 and the MAC :2079-2080 |
| F4 (X4) | SR-18 census line; frameclass::classify in the edit set with a QSLH arm and the retired pairs' disposition | new T1g | frameclass.rs:59-81 classify (QHSM by HS_MAGIC; 01 00 Message; 01 01 / 01 02 by reference to invite::ENVELOPE_VER, ENVELOPE_TYPE_INIT, ENVELOPE_TYPE_RESP); "QSLH" 51 53 vs "QHSM" 51 48 differ at byte 1; LegacyCompat invite.rs:1304, :1558, :1739, :1872 |
| F5 (X5 a-d) | (a) R6: the outgoing candidate committed BEFORE the A1 push; (b) the A2 receiver order; (c) zeroization of a failed or horizon-expired attempt; (d) the losing side's invite record terminal state and the deferred frame's disposal | T2 R6 and the A2 table; T3; T4 X-h | A2 at M: sid :2277, suite context :2291, confirm MAC hs_ct_eq_32 :2318, ML-DSA with pending.peer_sig_pk :2327-2336, session store :2394; InviteState {Creating, Active, Redeemed, Expired, Revoked} invite.rs:601-607 |
| F6 (X6) | Every missing vector in FINDINGS Q6 added; each vector's base stated (the BASE rule, per delta symbol) | T7 | FINDINGS Q6; symbols at M: decode_invite_code, verify_redeemed_bundle (invite.rs:566), hs_decode_header, hs_pq_init_ss; at P28 only: hs_lifecycle_plan, hs_lifecycle_select; pin codes contacts/mod.rs:913, :929, handshake/mod.rs:1014, :1091; decap refusals only on key/ciphertext bytes (refimpl stdcrypto.rs:198-206); B1 DH checks handshake/mod.rs:2035-2055, :880-892 |
| F7 (X7) | The contract owns the canonical relay_ep grammar (G1-G7), replacing "equals normalize_relay_endpoint()" | T1a #6 and the grammar under T1a; T1b row 9; T2 step 2b; T7 M10 | adversarial/route.rs:71-74 normalize_relay_endpoint = reqwest::Url to_string with trailing '/' trimmed; :51-69 validate_relay_endpoint_url (https, or http on a loopback host :39-48) |
| F8 (X8 a-e) | (a) C01 CENSUS V13, V15, V18 disposed by name; (b) route.rs:71-74; (c) env_ver older/newer: a stated reason; (d) constant-time bearer-tag compare; (e) "identity generation" defined | T1a Local; T1a #6; T2 step 2a note, step 4; T4 X-d | contacts/mod.rs:410-447 (contact_request_upsert called only from transport/mod.rs:1237); invite.rs:365-371; handshake/mod.rs:773-779 hs_ct_eq_32; P28 handshake/mod.rs:272, :543-553, :583-585 |
| E1 | K-07 is consistent with THE PLAN sec 2 (appended to the existing combiner, covered by the existing transcript MAC, no second key exchange); the D-record carries the D06 mapping | T6 row 1; ESCALATIONS E1 | handshake/mod.rs:808-838 (combiner documented extensible by list append; count byte :830) |
| E2 | C01 AMENDMENT in the commit PR: C01 T2 rows D/E old-peer codes per path | T8 OC13; ESCALATIONS E2 | M and C invite.rs:503-507, :774-777 (first byte != 0x01 -> handshake_envelope_version_newer); handshake/mod.rs:485-486, :500 (C :501) |
| E3 | C01 AMENDMENT: O11 RESOLVED = QHSM version 3 (A04 row edit; the length check precedes the parameter parse) | T1c; T8 OC9; ESCALATIONS E3 | handshake/mod.rs:485-490 |
| E4 | C01 AMENDMENT: "QSC.HS.SID" reclassified as a test-seam label (row 12 / A04) | T1d; ESCALATIONS E4 | handshake/mod.rs:754-767, :1500-1502 |
| E5 | No contact row, pin or route before step 10; the "pinned" -> TRUSTED mapping does not survive into the successor's first-contact state; state names are C05/F13's; the D-record carries the I02 sentence | T5; T8 OC14; ESCALATIONS E5 | invite.rs:1536-1543; contacts/mod.rs:86, :956, :868-871 |
| E6 | Accepted with F3 | ESCALATIONS E6 | as F3 |
| E7 | Accepted; the relay measurement is recorded | T1a; ESCALATIONS E7 | qsl-server 5ea0f925 src/lib.rs:800 (the presented cap hashed with sha256_hex), :806-808 (ct_eq_secret inside the store transaction), :794-795 (ticket); store.rs:393-396, :525-527, :645-650 |
Not taken, named: SR-15 X8(f) (a NOTE outside the ruled X8 a-e; the combiner is unchanged either way) and F3's optional
outer B1 tag (the ruling: NOT taken). Also changed, as consequences of acceptance only: the title line and the closing
END line.

PLAN: QSL-solution-plan rev3 d9016e53d2ab46c32b7a7cb060ea70dc79421617e9b054848b464ba7a5518275 (amended A4), card F01,
sub-assignment C02. Lane NA-0783 (D-1425, D-1426). Drafted 2026-09-23 by the executor seat (opus/high). Status of every
row: PROPOSED; ACCEPTED only by the Director's ruling after the SR-15 cold read (CRYPTO-TOUCHING: mandatory).
Scope: THE PLAN row C02 plus RULING_PLAN_F00 K-07 (ROADMAP item 5 handshake hardening absorbed: ONE successor
handshake wire change). Not in scope: C03 (relay capabilities, recovery horizon), C05 (dispatcher, ACK rules, DTO
names), C06, C07. Every C03/C04/C05 dependency is an OPEN cell in T8.
Fits C01 (C01_versions_and_boundaries.md at main, sha256 2880bc04...): profile = A02 (value OPEN, O2); handshake
critical parameter = A03; QHSM version = A04 (bump question O11 -- T1c proposes an answer); C01 row 12 hands C02 the
invite wire and the labels; C01 O2 and O11 remain the Director's.

Sources measured (read-only; the drafting mission's MEASURE.md, sha256 6987c30e...): qsl-protocol main M = 87aec475 (#1837 merge);
#1831 C = ffc8fc52; #1828 P28 = e29a07df; #1825 P25 = a4b172bc. File:line is qsl/qsl-client/qsc/src/<file>:<line> at M
unless prefixed C:, P28: or P25:. "t:" = tests/na0768_invite_finish_mixed_role.rs at that revision.
Notation: "++" is byte concatenation; integers are unsigned big-endian; "C(x)" is the identity commitment of identity x
= SHA-256(DS_COMMIT ++ canonical_bundle_bytes(kem_pk, sig_pk)) (invite.rs:221-254, :301-309), 32 bytes. KMAC is the
suite's kmac_out::<N>(key, label, data) (handshake.rs:786 idiom). ML-DSA is the suite's PqSigMldsa65 sign/verify
(refimpl stdcrypto :211-235: deterministic, empty context -- unchanged here, see T8 OC10).

==============================================================================================================
0. WHAT CHANGES, IN ONE TABLE
==============================================================================================================
| Surface | Today (M) | Successor (PROPOSED) | Section |
|---|---|---|---|
| Invite code | QSLI-1-, payload v1, `cap` sent to the relay in clear (transport.rs:4306) | QSLI-2-, payload v2: + profile, `cap` replaced by a bearer secret never sent to the relay; relay capability derived from it | T1a |
| Handshake envelope | QSLH-1: unsigned TLV {bundle, route, A1} / {route, B1}, no magic, order not enforced on decode (invite.rs:474-546, :752-808) | QSLH v2: fixed-order, length-exact, ML-DSA-signed envelope binding profile, invite, attempt, both identity commitments, reply route, bearer tag / reply link, exact inner frame | T1b |
| Inner frames | QHSM v2 (v1 on the shipped invite path: LegacyCompat, invite.rs:1304, :1558, :1739, :1872) | QHSM v3: suite block + A03 always; A1 + ephemeral ML-KEM-768 public key; B1 + ciphertext to it; combiner gains ss_eph | T1c, T6 |
| Verification | responder provisions contact and pins from the UNSIGNED envelope before A1 is decoded (invite.rs:1536-1543) | bounds -> canonical -> binding -> bearer -> signature -> pin -> replay -> crossing -> ONLY THEN occupancy | T2 |
| Crossing | none: mutual refusal (ENG-0345); aliases change the outcome (P25 t:1181-1195) | canonical order of C(x); lower commitment's attempt wins; arrival and alias irrelevant | T4 |

==============================================================================================================
T1. SIGNED BYTES
==============================================================================================================
T1a INVITE PAYLOAD v2 (the code the inviter shares; signed by the inviter as today, invite.rs:912-914)
| # | Field | Type / bound | Rule |
|---|---|---|---|
| 1 | ver | u8 = 0x02 | 0x01 -> invite_version_unsupported (NEW code; v1 is retired, not "newer"); other -> invite_version_newer (existing :101) |
| 2 | type | u8 = 0x01 | else invite_type_unknown (existing) |
| 3 | profile_len, profile | u8 1..50, bytes | == A02 byte-exact, else invite_profile_unsupported (NEW); refused BEFORE any network or durable write |
| 4 | invite_id | 16 B | CSPRNG at mint (as :884) |
| 5 | expiry | u64 | as today (:900) |
| 6 | relay_len, relay_ep | u16 1..512, bytes | ASCII; must match the CANONICAL RELAY_EP GRAMMAR below, byte-for-byte (F7: the contract owns it; F8b: normalize_relay_endpoint is adversarial/route.rs:71-74); else invite_malformed |
| 7 | bearer_secret | 16 B | CSPRNG at mint; NEVER sent to the relay; replaces v1 `cap` |
| 8 | commit | 32 B | C(inviter) |
| -- | trailing bytes | none | invite_malformed (as :390) |
CANONICAL RELAY_EP GRAMMAR (F7; owned by this contract, not by a URL library's output): relay_ep = scheme "://" host
[":" port] [path], ASCII only. G1 scheme is lowercase "https", or "http" only when the host is a loopback host (as
today's validate_relay_endpoint_url, adversarial/route.rs:51-69 with :39-48). G2 host is lowercase: a DNS name of
[a-z0-9-] labels joined by '.', an IPv4 dotted quad without leading zeros, or a bracketed IPv6 address in RFC 5952
text form; no IDNA U-label (an ASCII A-label is allowed). G3 no userinfo, query or fragment. G4 a port only when it
is not the scheme's default (443 https, 80 http), decimal without leading zeros. G5 path empty, or "/" followed by
segments of [A-Za-z0-9._~-] joined by "/", no empty, "." or ".." segment, no percent-encoding. G6 no trailing "/".
G7 length 1..512 bytes. Any other byte string -> invite_malformed (payload) / handshake_envelope_noncanonical
(envelope). normalize_relay_endpoint (route.rs:71-74) is a consumer-side check only and must be the identity on every
grammar-valid value (F10 asserts).
Signature: ML-DSA(inviter sig_sk, DS_SIG ++ payload) with DS_SIG = "QSL.invite.payload.v1" KEPT (the payload's own ver
byte separates v1 and v2 messages; renaming a label is a crypto change with no gain). Max payload 77+50+512 = 639 B;
code = "QSLI-2-" ++ base64url-nopad(payload), max 7+852 = 859 chars; canonical base64 trailing bits required.
Relay-facing capability (derived; the relay stores and compares a hash exactly as today, shape unchanged):
  redeem_cap = KMAC<16>(key = bearer_secret, label = "QSL.invite.redeem-cap.v1", data = invite_id);
  the create call uploads cap_hash_hex(wire_id(redeem_cap)) (invite.rs:206-215); redeem presents wire_id(redeem_cap).
  The relay therefore never learns bearer_secret (today it learns `cap`, transport.rs:4306). CRYPTO-TOUCHING.
  Relay measured (E7, qsl-server 5ea0f925): the redeem handler hashes the presented cap string with sha256_hex
  (src/lib.rs:800) and compares it with ct_eq_secret inside the store's consume transaction (:806-808; store.rs:525-527);
  the store keeps cap_hash as uploaded and never holds the secret in plaintext (store.rs:393-396); the ticket is minted
  at redeem (lib.rs:794-795) and burned at the first admitted push (store.rs:645-650). T1a changes nothing the relay
  does; C03 confirms (T8 OC4).
Local: InviteRecord keeps bearer_secret and the minted C(inviter) (C01 CENSUS V12 is C02's); RedemptionRecord as today
plus the operation fields C03 defines (T8 OC2).
C01 CENSUS cells handed to C02, disposed by name (F8a): V13 invite.ownership (invite.rs:1042; vault/mod.rs:543,
:556-559): KEEP as C02's; the successor records each minted invite_id and C(self) there for the own-invite preflight
(R3), exact version. V15 contact_requests.json (contacts/mod.rs:410-447; written only from transport/mod.rs:1237,
outside the invitation path): the successor invitation path writes NO contact request (T5 rule); its label-keyed
content follows C01 O5 and its state names are C05/F13's. V18 handshake.pending.<self_label>.<peer>: its content is
the outgoing candidate of R6 (pending with eph_kem_sk and dh_sk, the exact A1 envelope, attempt_id), zeroized per T3;
the key's suffix is C01 T5's contact id; its shape change is an SR-18 observable (T1g).

T1b HANDSHAKE ENVELOPE "QSLH" v2 (carries every successor A1 and B1; A2 stays bare, T1c)
| # | Field | Type / bound | A1 (env_type 0x01) | B1 (env_type 0x02) |
|---|---|---|---|---|
| 1 | magic | 4 B = "QSLH" (51 53 4C 48). A QSLH-1 envelope has no magic: its first two bytes are 01 01 or 01 02 (invite.rs:486-487, :762-763) and are recognised as RETIRED before the magic check | same | same |
| 2 | env_ver | u8 = 0x02 | any other value -> unsupported | same |
| 3 | env_type | u8 | 0x01 | 0x02 |
| 4 | profile_len, profile | u8 1..50, bytes | == A02 | == A02 |
| 5 | invite_id | 16 B | the redeemed invite | echo of the A1's |
| 6 | attempt_id | 16 B | CSPRNG, fixed at the redeem operation's commit (THE PLAN sec 2); immutable across retries (I04) | echo |
| 7 | sender_commit | 32 B | C(initiator) | C(responder) |
| 8 | recipient_commit | 32 B | C(inviter) = payload.commit | C(initiator) |
| 9 | relay_len, relay_ep | u16 1..512, bytes | MUST EQUAL the invitation payload's relay_ep (T1a #6) byte-for-byte (F1); canonical per the T1a grammar (F7) | MUST EQUAL the invitation payload's relay_ep (F1) |
| 10 | route_len, route_cap | u8 22..128, bytes | initiator's reply DEPOSIT capability, [A-Za-z0-9_-] only, NO trim (today normalize_route_token trims, route.rs:30-37) | responder's |
| 11 | link | 32 B | bearer_tag (below) | in_reply_to = SHA-256(exact A1 envelope bytes) |
| 12 | inner_len, inner | u16, bytes | == exact QHSM v3 A1 length (T1c) | == exact B1 length |
| 13 | sig | 3309 B (= runtime_pq_sig_signature_bytes(), ML-DSA-65; F10 asserts), fixed, no prefix | ML-DSA-65 by the initiator's identity sig key | by the responder's |
| -- | trailing bytes | none | refused | refused |
Field 9 (F1): the reply relay is FIXED to the invitation's relay; any other value is refused at T2 step 3 as
handshake_envelope_binding. Reason: the client attaches ONE relay bearer token to every push whatever the endpoint
(relay_auth_token, transport/mod.rs:2059-2067, attached at :2996-2999 and :4246-4248; ENG-0051), so a code holder
naming its own host in field 9 would receive the inviter's token with the B1 (and the responder's field 9 would steer
the A2). Cross-relay replies are DEFERRED to C03, with per-endpoint scoping of the client's relay credential as the
named precondition (T8 OC15). The field stays in the envelope, bound and explicit.
Signature input: DS_ENV_T ++ envelope[0 .. offset(sig)). DS_ENV_A1 = "QSL.handshake.envelope.A1.v1", DS_ENV_B1 =
"QSL.handshake.envelope.B1.v1" (distinct per type; PROPOSED spellings, allocated later in the DOC-CAN-003 sec 12 table
as C01 did). CRYPTO-TOUCHING.
Bearer tag (A1 only): K_bearer = KMAC<32>(key = bearer_secret, label = "QSL.invite.bearer-key.v1", data = invite_id);
bearer_tag = KMAC<32>(key = K_bearer, label = "QSL.handshake.bearer.v1", data = envelope[0 .. offset(link)) ++
envelope[offset(inner_len) .. offset(sig))). The inviter recomputes it from its stored bearer_secret. It proves the A1
author holds the invite code (a bearer fact, T5), and excludes the relay, which never sees bearer_secret. CRYPTO-TOUCHING.
Sizes (L = len(A02) <= 50, R <= 512, K <= 128; ML-KEM-768 ek 1184 / ct 1088 and ML-DSA-65 pk 1952 / sig 3309, each
asserted against the runtime_pq_* helpers at F10): fixed part 3449 B; A1 envelope = 3449+L+R+K+(5479+L) <= 9668 B;
B1 envelope = 3449+L+R+K+(7540+L) <= 11729 B. ENV_MAX = 12288 B, checked on the received bytes BEFORE any parse or
copy (both below #1828's FIRST_FRAME_BYTES 32 KiB, P28 handshake.rs:206; relay frame ceiling is C03, T8 OC4).

T1c INNER FRAMES, QHSM v3 (K-07; answers C01 O11 as PROPOSED: bump 2 -> 3)
| Frame | Layout after header "QHSM" ++ u16 3 ++ type ++ u16 block_len ++ block | Length (block = 14+L) | Change vs M |
|---|---|---|---|
| A1 (1) | sid 16 ++ kem_pk 1184 (identity) ++ sig_pk 1952 (identity) ++ dh_pub 32 ++ resp_kem_ct 1088 ++ eph_kem_pk 1184 | 5479+L | + eph_kem_pk (fresh ML-KEM-768 per attempt) |
| B1 (2) | sid 16 ++ kem_ct 1088 (to identity kem_pk) ++ eph_kem_ct 1088 ++ mac 32 ++ sig_pk 1952 ++ sig 3309 ++ dh_pub 32 | 7540+L | + eph_kem_ct; also inside b1_no_auth after kem_ct (transcript input, handshake.rs:561-585) |
| A2 (3) | sid 16 ++ mac 32 ++ sig 3309 | 3380+L | header version only |
Header rules: block = A01 suite block (9 B, handshake.rs:33) ++ A03 parameter (5+L) exactly, canonical order, parsed
before any key use (hs_parse_parameter_block :275-370 with C's 0x7f80 arm, C:336-347). v1 and v2 frames ->
REJECT_QSC_HS_INTEGRATION_REQUIRED (C's code for v1, C:476, extended to v2); unknown -> handshake_version (:500).
Key schedule: pq_init_ss = hs_root_combine("QSC.HS.PQ", sid, 0x01, [ss_eph, ss_pq, resp_kem_ss], ctx) (count 3; order
from audit F-04); every other derivation unchanged (T1d). The initiator keeps eph_kem_sk in its outgoing candidate
until B1 is processed, then zeroizes it (P28 pattern :646-670). CRYPTO-TOUCHING.
Why bump (reason, measured): the A1/B1 layouts change; hs_decode_header checks the exact frame length BEFORE parsing the
parameter block (:485-490), so under v2 an old peer refuses REJECT_QSC_HS_MALFORMED_LENGTH, a misleading code; under v3 it
refuses handshake_version (:500). Either way it refuses before effects. Alternative (keep 2): no table change to A04,
misleading old-peer code. RULED (E3): O11 RESOLVED = QHSM version 3; A04 is amended in the commit PR (C01
AMENDMENTS; DOC-CAN-003 sec 12 amendment row).

T1d DOMAIN LABELS ON THIS PATH (enumerate-and-classify; C01 row 12 handed C02 four of them)
| Label | Where (M) | Use | Successor |
|---|---|---|---|
| QSL.invite.identity-commitment.v1 (DS_COMMIT) | invite.rs:66 | SHA-256 commitment | KEEP bytes |
| QSL.invite.payload.v1 (DS_SIG) | :68 | invite signature | KEEP bytes (payload ver byte separates v2) |
| QSC.HS.ROOT.COMBINE.v1 | handshake.rs:817 | KMAC key of the combiner | KEEP bytes; contributions list grows (count byte is in the data, :830) |
| QSC.HS.PQ, QSC.HS.DHINIT | :850, :868 | combiner domains | KEEP; PQ gets 3 contributions |
| QSC.HS.TRANSCRIPT, QSC.HS.TRANSCRIPT.H, QSC.HS.CONFIRM, QSC.HS.A2 | :786, :794, :919, :934 | KMAC | KEEP |
| QSC.HS.SIG.B1, QSC.HS.SIG.A2 | :939, :947 | ML-DSA message prefix | KEEP |
| QSC.HS.SID | :1500-1502 via :754-767 | RNG-failure TEST-SEAM label only; sid is 16 OsRng bytes | NOT a domain separator; RULED (E4): reclassified as a test-seam label in C01 row 12 / A04 (C01 AMENDMENTS) |
| QSC.SIG.B1, QSC.SIG.A2, QSC.KEM.ENCAP | :2180, :2532, :2623 | test-seam labels | not domain separators |
| QSL.invite.redeem-cap.v1, QSL.invite.bearer-key.v1, QSL.handshake.bearer.v1 | NEW | T1a, T1b | PROPOSED |
| QSL.handshake.envelope.A1.v1 / .B1.v1 | NEW | T1b | PROPOSED |
Separation from every older session and receipt is ALSO carried by A02 entering hs_root_combine (C01 row 2).

T1e WHAT THE A1/B1 SIGNATURE BINDS (THE PLAN sec 2 checklist)
| Required binding | Field(s) |
|---|---|
| exact inner handshake bytes | 12 (and the tag covers them too) |
| both identities, full commitments | 7, 8; 7 must equal C(inner keys) (A1) / the code's commit (B1) |
| invitation / operation identity | 5, 6 (+ link: bearer tag on A1, in_reply_to on B1) |
| selected profile (C01 A02) | 4, and A03 inside 12 |
| reply route capability | 9 (fixed to the invitation's relay_ep, F1), 10 (representation is C03's, T8 OC1) |

T1f RETIRED BY C02 (refused, never reused): QSLI-1- codes and payload v1; QSLH-1 envelope (ENVELOPE_VER 0x01, TAG_BUNDLE
0x01, TAG_ROUTE_TOKEN 0x02, TAG_A1 0x03, TAG_B1 0x04, types 0x01/0x02 of that version); QHSM v1 and v2; the bare
(unenveloped) A1/B1 on the successor mailbox (A1Delivery::Direct, handshake.rs:1430-1437, :1564) -- the known-contact
re-handshake is T8 OC7.

T1g SR-18 CENSUS LINE AND THE FRAME CLASSIFIER (F4)
SR-18 applies at F10's drafting: the census of every pin of the observables C02 remaps is owed by F10's directive.
C02 names them: the error-code strings (handshake_envelope_version_newer no longer produced on the successor path;
every code marked NEW in T2 and T7); the QSLI-1- prefix and payload v1; the QSLH-1 first-byte pairs 01 01 / 01 02
(invite::ENVELOPE_VER with ENVELOPE_TYPE_INIT / ENVELOPE_TYPE_RESP); HandshakeSuiteMode::LegacyCompat at invite.rs:1304,
:1558, :1739, :1872; the contact status "pinned" (contacts.rs:86, :956); the HandshakePending shape (eph_kem_sk added,
T1c; C01 CENSUS V18); QHSM v1/v2 frames on the invite path.
frameclass::classify (frameclass.rs:59-81; C01 row 11 "KEEP as a match") is IN F10's edit set. It gains a QSLH arm:
the four bytes "QSLH" (51 53 4C 48), reached by reference to the successor's constant, never a literal; they cannot
collide with HS_MAGIC "QHSM" (51 48 at bytes 0-1) or with any 01 xx pair. The arm names the class that the successor
slot and inbox route to envelope decoding (T2 step 2a); its spelling joins FrameClass's closed vocabulary through the
census. The retired pairs' disposition: the 01 01 / 01 02 arms are KEPT, still reached by reference to the RETIRED
constants (T1f: refused, never reused), so a QSLH-1 frame is still MATCHED -- it never falls to Unknown -- and is
refused at T2 step 2a as handshake_envelope_version_retired, disposition "permanently invalid" (C05, T8 OC6).

==============================================================================================================
T2. VERIFICATION ORDER -- NOTHING OCCUPIES BEFORE STEP 10
==============================================================================================================
Receiver of an A1 (the inviter, pulling its invite slot) and of a B1 (the redeemer, pulling its inbox). Codes marked
NEW are PROPOSED spellings, registered through DOC-SCL-002 by the implementing PR (as C01 O9). "Disposition" names the
class C05 turns into an ACK rule (THE PLAN sec 3 table); C02 does not set ACK rules.
| Step | Check | Refusal code | Refuses before effects? | Disposition (C05) |
|---|---|---|---|---|
| L0 | local preconditions before any pull: vault unlocked; invite record Active (accept :1506-1512); KNOWN OPERATION FIRST (F2): an Active slot is pulled and an A1 the relay admitted before expiry (the relay checks expiry on its own clock at push and burns the ticket, qsl-server store.rs:633-635, :645-650) is processed through steps 1-10 until C03's recovery horizon (T8 OC3), whatever the local invite expiry; the local expiry refuses only after that horizon; alias/label passes the successor rule (K-07 label check; grammar C01 O5) | vault_locked; invite_not_found / invite_already_redeemed / invite_revoked_locally (existing); invite_expired (existing; only after the recovery horizon, F2); contacts_alias_invalid (existing code, MOVED before the pull) | YES (no pull) | n/a |
| 1 | bounds: received length <= ENV_MAX before parse or copy | handshake_envelope_too_large (NEW) | YES | permanently invalid |
| 2a | structure: first bytes 01 01 / 01 02 (QSLH-1) -> retired; then magic; env_ver != 0x02 -> unsupported (one code for older and newer: F8c, reason under this table); env_type matches the mailbox (A1 on a slot, B1 on the inbox); every length in range and exact; no trailing bytes | handshake_envelope_malformed; handshake_envelope_version_retired (NEW); handshake_envelope_version_unsupported (NEW; replaces _version_newer); handshake_envelope_type (NEW) | YES | permanently invalid |
| 2b | canonical form: profile == A02; relay_ep in the canonical grammar (T1a, F7); route_cap charset exact, untrimmed | handshake_envelope_profile (NEW); handshake_envelope_noncanonical (NEW) | YES | permanently invalid |
| 2c | inner QHSM v3 decode, pure (header, A03 block, exact length); A1 dh_pub not all-zero (:2507) | existing REJECT_QSC_HS_* (A03 row) / handshake_version / dh_pub_invalid | YES | permanently invalid |
| 3 | binding: A1: sender_commit == C(inner kem_pk, inner sig_pk); recipient_commit == this invite's minted C; invite_id == this slot; relay_ep == this invite's relay_ep (F1). B1: sender_commit == the code's commit; recipient_commit == C(self); invite_id and attempt_id == our operation; in_reply_to == SHA-256(our A1 envelope); inner sid == our outgoing sid; relay_ep == the redeemed code's relay_ep (F1); inner sig_pk == the sig_pk of the bundle verified at R5 (F3) | handshake_envelope_binding (NEW) | YES | permanently invalid |
| 4 | A1 bearer tag recomputed from the stored bearer_secret and compared in constant time (F8d; the hs_ct_eq_32 idiom, handshake/mod.rs:773-779) | invite_bearer_invalid (NEW) | YES | permanently invalid |
| 5 | ML-DSA-65 verify of field 13: A1 with the inner sig_pk (tied to sender_commit at step 3); B1 with the sig_pk of the bundle verified at R5 (F3), never a key taken from the wire | handshake_envelope_signature_invalid (NEW) | YES | permanently invalid |
| 6 | identity pin. A1: if this identity or this alias is already pinned, it must match (never silently replaced, I12). B1: the pin written at redeem is REQUIRED and must match fp AND sig_fp (as :2112-2127; the responder-side sig pin, OPTIONAL at M :2525, becomes REQUIRED when a pin exists) | peer_mismatch; contacts_identity_changed; responder_sig_mismatch; responder_sig_unpinned (existing) | YES | permanently invalid (security marker) |
| 7 | replay / idempotence lookup (T3) | handshake_replay_conflict (NEW); REJECT_QSC_HS_REPLAY (existing :1326) | YES (an exact duplicate re-sends saved bytes; no new durable state) | duplicate of committed work / invalid |
| 8 | crossing decision (T4) | none (a loser A1 is RETAINED, not refused) | no refusal; the only write is one bounded superseded receipt of an AUTHENTICATED A1 (T4 X-b) | known pending attempt: defer |
| 9 | compute, pure: encapsulations, decapsulations, transcript, B1 or A2 bytes; B1 path verifies inner MAC and inner ML-DSA (:2079-2096) | existing resp_kem_decap_failed, pq_decap_failed, REJECT_QSC_HS_TRANSCRIPT_CONTEXT / bad_transcript, sig_invalid | YES | permanently invalid |
| 10 | OCCUPANCY, one durable commit: A1 -> responder candidate (pending + exact reply + receipt) and contact row with the SIGNED route, status per T5; B1 -> selection (P28 hs_lifecycle_select :808-844) | store codes | -- | then send; ACK per C05 |
Step 2a code (F8c, the stated reason for one code): under the QSLH magic no env_ver other than 0x02 was ever issued
(QSLH-1 had no magic and keeps its own older-format code, handshake_envelope_version_retired), so 0x00, 0x01 and
0x03-0xFF all mean "a version this client does not know", with one user action; the tree's older/newer distinction
(invite.rs:365-371) is kept where an older format really exists (QSLH-1 above; R1 below).
Order note: 3 and 4 sit between canonical form and signature: 3 is pure consistency of already-parsed fields; 4 is the
cheap symmetric check before the expensive one (the idiom B1 already uses, invite.rs:552-565). Named as a refinement of
the directive's order in the drafting mission's REPORT.md (sha256 d91b5b20...). Redeem side (initiator, before any A1 exists):
| Step | Check | Code | Before effects? |
|---|---|---|---|
| R0 | raw pasted input <= 2048 B BEFORE trim or base64 (K-07 decoder cap, triage D-15; today none, invite.rs:435-449) | invite_code_too_long (NEW) | YES |
| R1 | prefix: QSLI-2- ok; QSLI-1- -> retired; other QSLI- -> newer; else malformed; trimmed code <= 859 | invite_version_unsupported (NEW) / invite_version_newer / invite_malformed | YES |
| R2 | payload v2 strict decode (T1a) incl. profile and canonical relay_ep | invite_malformed / invite_type_unknown / invite_profile_unsupported (NEW) | YES |
| R2b | KNOWN OPERATION FIRST (F2): a committed redeem operation for this invite_id is looked up and, if present, resumed at R4 with its exact saved bytes (THE PLAN sec 2) until C03's recovery horizon, whatever the local expiry; R3's expiry check is not reached for it | none (resume); after the horizon: failed-expired (T5) | YES (a lookup) |
| R3 | NEW claimant only (no operation found at R2b): local expiry; own-invite preflight KEPT (invite.rs:1149-1196, THE PLAN sec 2); alias rule (K-07) | invite_expired; invite_self; contacts_alias_invalid | YES |
| R4 | operation commit, relay redeem, cache (C03 recovery: THE PLAN sec 2; the cached bytes are UNVERIFIED and pin nothing) | C03 | the operation record is the declared authority (I03) |
| R5 | commitment THEN invite signature (verify_redeemed_bundle :566-585) BEFORE any contact, pin or pending write | invite_commitment_mismatch / invite_signature_invalid | YES |
| R6 | (F5a) outgoing candidate committed durably BEFORE the A1 push: pending with eph_kem_sk and dh_sk, the exact A1 envelope, attempt_id; receipt and ticket handling per C03 (T8 OC2) | store codes | the operation's own intent (I03/I04); nothing peer-supplied is pinned; a retry re-sends these exact bytes |
A2 receiver (F5b; the responder, a bare QHSM v3 A2 on its inbox; order KEPT from M, handshake/mod.rs:2275-2394):
| Step | Check | Code | Before effects? |
|---|---|---|---|
| A2-1 | bounds and QHSM v3 decode (header, A03 block, exact length) | handshake_version / REJECT_QSC_HS_* | YES |
| A2-2 | sid == the responder candidate's sid (:2277) | session_id_mismatch | YES |
| A2-3 | suite context equal to the candidate's (hs_contexts_match, :2291) | REJECT_QSC_HS_CONTEXT_MISMATCH | YES |
| A2-4 | confirm MAC, constant time (hs_ct_eq_32, :2318) | REJECT_QSC_HS_TRANSCRIPT_CONTEXT / bad_confirm | YES |
| A2-5 | ML-DSA with the candidate's PINNED peer_sig_pk (:2327-2336), never a wire key | sig_invalid | YES |
| A2-6 | selection and session commit, one durable commit (P28 hs_lifecycle_select :808-844; the session store at M :2394); an exact-duplicate A2 adds no state | store codes | -- then ACK per C05 (T8 OC6) |

==============================================================================================================
T3. REPLAY RULES
==============================================================================================================
Receipt (reused from P28 HsReceipt :227-233): (mailbox, envelope_digest = SHA-256(exact envelope), frame_digest =
SHA-256(inner), route). Recorded ONLY at step 10 (authenticated); an unauthenticated frame records nothing (I07).
| # | Input | Rule | Result |
|---|---|---|---|
| RP1 | A1 whose receipt equals the responder candidate's | exact duplicate | re-send the exact saved B1 bytes (P28 :966-971, :910-919); no new state |
| RP2 | authenticated A1, same invite_id and same identity, different attempt_id or different bytes | the redeem operation is immutable (I04); a second attempt is a conflict | handshake_replay_conflict |
| RP3 | authenticated A1 from a different identity for an invite already Redeemed or holding a responder candidate | single use (client-side, as :1508-1512) | invite_already_redeemed |
| RP4 | A1 whose inner sid equals a committed session's sid for that peer | existing guard | REJECT_QSC_HS_REPLAY |
| RP5 | B1 whose receipt equals the selection's | exact duplicate | re-send the exact saved A2 (P28 :1013-1017) |
| RP6 | QSLH-1 / QSLI-1- / QHSM v1-v2 input | retired formats | T2 step 2a / R1 / 2c codes |
| RP7 | crossing loser A1 redelivered after the winner is applied | superseded | dispose (C05) |
| RP8 | any A1/B1 matching no live attempt after the recovery horizon | nothing remembered | permanently invalid; no durable row |
Remembered: per invite, at most one responder candidate receipt + its exact B1, and at most one superseded (crossing)
receipt; per redeem operation, its exact A1 envelope and the ticket (C03). After the attempt applies, only public replay
data (receipt digests, exact reply bytes) is kept; secrets are zeroized (P28 :646-670). A FAILED attempt, and one
whose recovery horizon passes (RP8; T5 failed-expired), zeroizes its secrets too (eph_kem_sk, dh_sk, resp_kem_ss and
any key derived from them held in the candidate) in the same commit that marks it terminal (F5c).
For how long: until the operation's fixed recovery_until (THE PLAN sec 2) -- value and cleanup are C03's (T8 OC3);
count/byte quotas are C04's (T8 OC5; P28's 64 x 256 KiB, :203-205, is a candidate). No timestamp is added to any
envelope (the only relay-visible timestamp stays the invite expiry, invite.rs:50-55).

==============================================================================================================
T4. DETERMINISTIC CROSSING ORDER
==============================================================================================================
Crossing: identities X and Y each hold an outgoing attempt to the other (each redeemed the other's invite), or one side
holds an outgoing attempt to an identity from which it receives an authenticated A1 (T2 steps 1-6 passed).
| # | Rule |
|---|---|
| X-a | Order key: the full identity commitments C(X), C(Y) (32 B each, the values signed in fields 7-8), compared as unsigned byte strings, lexicographically. The winner W is the attempt INITIATED by the side with the lower commitment. Never aliases, never arrival order, never timestamps. Equal commitments = self: refused earlier (invite_self) |
| X-b | Lower side L: keeps its outgoing W; an authenticated A1 from the higher side is recorded as a single superseded receipt (after authentication, unlike P28 :975-980), answered with nothing, disposition "defer" until W is applied, then dispose (C05) |
| X-c | Higher side H: answers L's authenticated A1 as responder (step 10) and marks its own outgoing attempt superseded in the same commit; a B1 for H's superseded attempt -> handshake_crossing_superseded (NEW) before effects |
| X-d | One outgoing and one authenticated responder candidate per identity generation (THE PLAN sec 2; P28 generation guard :578-598, :820). Winner selection and the exact reply obligation commit before projections; recovery never resets an advanced same-SID session (P28 :630-640). Identity generation (F8e): the first-connection lifecycle record of one (self identity, peer identity) pair, identified by a 16-byte random generation value minted when that record is created (P28 handshake/mod.rs:272, :543-553); a record of one generation never replaces another (:583-585, handshake_lifecycle_conflict) |
| X-e | Both arrival orders yield the same W and the same session sid (the sid of L's A1). A2 delayed behind other traffic does not change W |
| X-f | Two local aliases for one identity: the second is refused handshake_lifecycle_conflict (P28 :507-513), so an alias cannot create a second session (today it does, P25 t:1186-1189) |
| X-h | (F5d) When W applies, L's invite record (the invite H redeemed, whose attempt lost) moves to the existing terminal state Redeemed (InviteState, invite.rs:601-607) in the same commit, so L0 stops pulling that slot (invite_already_redeemed); the deferred frame behind the superseded receipt is disposed (RP7; ACK disposition C05, T8 OC6). H's own outgoing attempt is already superseded (X-c). State spellings are C05/F13's |
| X-g | Counterfeit-first then real. (i) Counterfeit = a real A1 with its unauthenticated sid changed (P28 t:1576): step 4 or 5 refuses it before any write; the real A1 then proceeds -- P28 t:1555-1613 flips from "blocked, handshake_lifecycle_occupied" to completion. (ii) Foreign identity, valid own signature, into a side that already pins the peer: step 6 peer_mismatch. (iii) Foreign identity at a first-contact slot: step 4 invite_bearer_invalid unless the author holds the invite code (T5) |
Key choice named: P28 orders by the qsl-fp-v1 identity fingerprint as lowercase hex strings (:972). C02 proposes C(x),
the commitment that is signed in the envelope and in the invite; same rule (lower keeps its outgoing), different key.

==============================================================================================================
T5. FIRST-CONTACT TRUST
==============================================================================================================
| Evidence | Proves | Does NOT prove |
|---|---|---|
| Invite code (bearer) | whoever holds it may connect to the identity whose commitment it carries | who holds it |
| R5 commitment + invite signature | the relay-served bundle is the minter's committed identity | that the minter is a particular person |
| A1 envelope signature (step 5) | the A1 author holds the signing key of the identity it presents | that this identity is the person invited |
| A1 bearer tag (step 4) | the A1 author (or someone it shared the code with) holds the invite code; not the relay | uniqueness of the holder |
| B1 envelope + inner MAC + inner ML-DSA | the responder holds the identity KEM and signing keys the code committed to | a person's identity |
| A2 | the initiator holds its identity KEM and signing keys | a person's identity |
Explicit trust steps that remain: sharing the bearer invitation, and later human verification (fingerprint comparison,
the existing "verified" status, contacts.rs:1042). Possession is never verification (I02).
User-visible states (PROPOSED meanings; DTO spellings are C05's):
| State | Entered when | Messaging |
|---|---|---|
| invited | code minted, no peer | none |
| connecting | redeem operation committed / A1 answered; no session | none |
| connected-unverified | session committed after T2 (possession proven, bearer-trusted) | allowed behind "Not verified" (operator ruling cited at contacts.rs:868-871; C05 unverified-send policy) |
| verified | the user completed fingerprint comparison | full |
| identity-changed | a pin mismatch (step 6) | blocked; security marker; never silently replaced |
| failed-expired | the recovery horizon passed (C03) | none; a new invite is required |
Rule: no contact row, pin or route is written before step 10. Today's accept path writes status "pinned" before any
check (invite.rs:1536-1543), and "pinned" maps to device state TRUSTED (contacts.rs:86, :956): not carried (E5).
RULED (E5): no contact row, pin or route before step 10; the "pinned" -> TRUSTED mapping does not survive into the
successor's first-contact state; the state names above are meanings only, their spellings are C05/F13's. I02:
possession is not verification.

==============================================================================================================
T6. K-07 ABSORPTION (ROADMAP.md:18 at M: "handshake hardening (ephemeral ML-KEM, required suite, engine-side label check and decoder cap)")
==============================================================================================================
| Item | Source | Measured at M | Lands in | Status |
|---|---|---|---|---|
| Ephemeral ML-KEM in A1 | audit F-04 (2026-09-03, :262-289) | ABSENT: A1 kem_pk is the identity key (handshake.rs:1493-1512); responder encapsulates to it (:2536) | T1c A1/B1 layouts, combiner [ss_eph, ss_pq, resp_kem_ss], zeroize at B1; T2 2c exact lengths | PROPOSED; CRYPTO-TOUCHING; E1 RULED: consistent with THE PLAN sec 2 (appended to the existing combiner, covered by the existing transcript MAC, no second key exchange); the D-record carries the D06 mapping (PQ forward secrecy of the initial root against identity-KEM compromise) |
| SuiteRequired | audit F-07 (:333-353) | ABSENT on the invite path (LegacyCompat x4 -> LegacyV1, handshake.rs:199-204); PRESENT BY CONSTRUCTION at C (C:203-205, :476) | T1c header rules; T1f retires v1/v2; LegacyCompat removed from the successor invite path | PROPOSED |
| Engine-side label check | triage D-7 (desktop audit item 7) | PARTIAL: channel_label_ok only inside contacts_provision_from_invite (contacts.rs:884-886), after network effects; create side trims + non-empty only (invite.rs:856-859) | T2 L0 and R3: before any network or durable effect on create (recipient label), redeem and accept; the label never reaches the wire (C01 T5 label split) | PROPOSED; grammar OPEN with C01 O5 |
| Decoder cap | triage D-15 (desktop audit item 15) | ABSENT (invite.rs:435-449) | T2 R0/R1 (2048 raw, 859 trimmed); T2 step 1 ENV_MAX 12288 for envelopes | PROPOSED; pull-body cap (F-12) is transport, C03/C05 |

==============================================================================================================
T7. VECTORS SPECIFIED (NOT GENERATED; byte values NOT_RUN until F10)
==============================================================================================================
Fixtures (F10 fixes and records them): identities I_A, I_B from recorded seeds with C(I_A) < C(I_B); a third I_X;
profile = A02 (bytes wait on C01 O2); invite_id, attempt_id, bearer_secret, sid, X25519 and ML-KEM ephemerals from
recorded seeds; relay_ep and route_cap are fixture constants satisfying T1b rows 9-10. "Delta symbol" (SR-19) names the
symbol in F10's proposed edit set whose change flips the test (names are proposals, THE PLAN): DIC =
invite::decode_invite_code; DHE = invite::decode_handshake_envelope (new); VHE = invite::verify_handshake_envelope
(new); BT = invite::bearer_tag (new); RC = invite::redeem_cap (new); HDH = handshake::hs_decode_header; PQ =
handshake::hs_pq_init_ss; CW = handshake::hs_crossing_winner (new); LP = handshake::hs_lifecycle_plan; LS =
handshake::hs_lifecycle_select. Every refusal vector also asserts: no durable write (vault bytes, contacts, invite and
redemption records unchanged), no send, no ACK beyond its C05 disposition.
BASE (F6; SR-19(a) "compiles and runs at base"): each vector's base is the base of its delta symbol(s). DIC, HDH, PQ
and VRB (= invite::verify_redeemed_bundle, invite.rs:566) exist at M (qsl-protocol main 87aec475): base M, RED there
wherever the expected behaviour changes. LP and LS exist only at P28 (#1828 e29a07df), not at M: base = the F10
commit that brings them onto its branch, named in F10's directive. DHE, VHE, BT, RC, CW exist nowhere yet: base = an
F10 commit that introduces each symbol with today's behaviour (no refusal it does not already make), so the vector
compiles and runs RED there and GREEN on the change. "<X> caller" names the entry point (redeem, accept) that calls X;
its base is X's. A vector naming several symbols takes the latest of their bases.
Positive
| Id | Inputs | Expected | Delta |
|---|---|---|---|
| P1 | I_A mints; code v2 encode then decode | round-trip equal; redeem_cap, K_bearer, bearer_tag reproducible | DIC, RC, BT |
| P2 | I_B redeems P1, sends A1 envelope | inviter: steps 1-10 pass; responder candidate + contact (connected pending) committed; B1 envelope sent | VHE, LP |
| P3 | B1 of P2 at I_B | selection committed; A2 sent; pq_init_ss includes ss_eph | VHE, PQ, LS |
| P4 | A2 of P3 at I_A | session committed both sides; same sid; state connected-unverified | LS |
Mutations of the A1 envelope of P2 ("ns" = altered, not re-signed and tag not recomputed; "rs" = re-signed by the
sender with a valid tag). Expected code per T2.
| Id | Mutation | Expected (ns) | Expected (rs) | Delta |
|---|---|---|---|---|
| M01 | magic byte flip | handshake_envelope_malformed | same | DHE |
| M02 | env_ver 0x01 / 0x03 (magic kept) | _version_unsupported (both) | same | DHE |
| M03 | env_type 0x02 on the slot | handshake_envelope_type | same | DHE |
| M04 | profile = NA0780-DIR-INTEGRATION-03; len 0; len 51 | handshake_envelope_profile; malformed; malformed | same | DHE |
| M05 | invite_id altered | handshake_envelope_binding | same | VHE |
| M06 | attempt_id altered | invite_bearer_invalid | accepted as a new attempt only if no candidate exists; else handshake_replay_conflict (RP2) | BT, LP |
| M07 | sender_commit altered | handshake_envelope_binding | same | VHE |
| M08 | recipient_commit altered | handshake_envelope_binding | same | VHE |
| M09 | relay_ep altered (canonical, != the invitation's relay_ep) | invite_bearer_invalid | handshake_envelope_binding (F1: re-expected; cross-relay replies deferred to C03) | BT, VHE |
| M10 | relay_ep outside the canonical grammar (F7; e.g. trailing slash; upper-case scheme) | invite_bearer_invalid | handshake_envelope_noncanonical | DHE |
| M11 | route_cap altered | invite_bearer_invalid | accepted (sender-chosen) | BT |
| M12 | route_cap with a leading space; a '.'; 21 and 129 bytes | invite_bearer_invalid | noncanonical; noncanonical; malformed; malformed | DHE |
| M13 | link (bearer_tag) bit flip | invite_bearer_invalid | n/a | BT |
| M14 | inner sid altered | invite_bearer_invalid | accepted (a sender may pick its sid) | BT |
| M15 | inner kem_pk or sig_pk altered | handshake_envelope_binding | same (sender_commit no longer matches) | VHE |
| M16 | inner dh_pub all-zero | invite_bearer_invalid | dh_pub_invalid | DHE |
| M17 | inner eph_kem_pk altered | invite_bearer_invalid | accepted; T8 OC11 if the key is non-canonical | BT |
| M18 | inner A03 value = another profile | invite_bearer_invalid | REJECT_QSC_HS_INTEGRATION_PROFILE | HDH |
| M19 | inner header version 2 / 1 | invite_bearer_invalid | REJECT_QSC_HS_INTEGRATION_REQUIRED | HDH |
| M20 | signature bit flip | handshake_envelope_signature_invalid | n/a | VHE |
| M21 | wrong domain: signed under DS_ENV_B1; under DS_SIG; with no prefix | handshake_envelope_signature_invalid (x3) | n/a | VHE |
| M22 | signed by I_X's key (valid ML-DSA, other key) | handshake_envelope_signature_invalid | n/a | VHE |
| M23 | sender_commit and recipient_commit swapped | handshake_envelope_binding | same | VHE |
| M24 | truncated at every field boundary and to 0 bytes | handshake_envelope_malformed | same | DHE |
| M25 | one trailing byte; total ENV_MAX+1 | malformed; handshake_envelope_too_large (no parse) | same | DHE |
| M26 | each length prefix +1 and -1 (profile_len, relay_len, route_len, inner_len) | handshake_envelope_malformed | same | DHE |
| M27 | (F6) env_type 0x03 (unknown) | handshake_envelope_type | same | DHE |
| M28 | (F6) relay_len 0; relay_len 513 | handshake_envelope_malformed (both) | same | DHE |
| M29 | (F6) inner resp_kem_ct altered | invite_bearer_invalid | accepted at the envelope (the sender's own choice); the responder's decapsulated secret then differs, and the initiator's B1 check fails REJECT_QSC_HS_TRANSCRIPT_CONTEXT / bad_transcript | BT, PQ |
| M30 | (F6) A1 from I_X at a slot whose alias is already pinned to I_A | n/a (needs a valid tag) | contacts_identity_changed (step 6; contacts/mod.rs:913, :929) | VHE |
| M31 | (F6) A1 from I_A after I_A's identity pin was replaced by I_X's in the fixture | n/a | peer_mismatch (step 6; handshake/mod.rs:1014) | VHE |
| M32 | (F6) inviter alias fails the successor label rule at L0 | contacts_alias_invalid, before the pull | same | LP caller (accept, L0) |
B1 envelope of P3 (at I_B)
| Id | Mutation | Expected | Delta |
|---|---|---|---|
| B01 | in_reply_to altered | handshake_envelope_binding | VHE |
| B02 | attempt_id or invite_id altered | handshake_envelope_binding | VHE |
| B03 | sender_commit != the code's commit | handshake_envelope_binding | VHE |
| B04 | inner sig_pk and signature from I_X, sender_commit claimed correct | handshake_envelope_binding (F3: re-expected; step 3 binds inner sig_pk to the key verified at R5) | VHE |
| B05 | inner eph_kem_ct altered, re-signed | REJECT_QSC_HS_TRANSCRIPT_CONTEXT / bad_transcript (:2079-2090) | PQ |
| B06 | signature bit flip | handshake_envelope_signature_invalid | VHE |
| B07 | env_type 0x01 on the inbox | handshake_envelope_type | DHE |
| B08 | (F1) relay_ep != the invitation payload's relay_ep (canonical), re-signed | handshake_envelope_binding | VHE |
| B09 | (F6) recipient_commit altered (ns and rs) | handshake_envelope_binding | VHE |
| B10 | (F6) profile = another value; len 0; len 51 | handshake_envelope_profile; malformed; malformed | DHE |
| B11 | (F6) route_cap altered | ns: handshake_envelope_signature_invalid; rs: accepted (sender-chosen) | VHE |
| B12 | (F6) relay_ep outside the canonical grammar (F7) | handshake_envelope_noncanonical | DHE |
| B13 | (F6) inner sid != our outgoing sid, re-signed | handshake_envelope_binding | VHE |
| B14 | (F6) inner kem_ct altered | ns: handshake_envelope_signature_invalid; rs: REJECT_QSC_HS_TRANSCRIPT_CONTEXT / bad_transcript | VHE, PQ |
| B15 | (F6) inner mac altered | ns: handshake_envelope_signature_invalid; rs: REJECT_QSC_HS_TRANSCRIPT_CONTEXT / bad_transcript | VHE, PQ |
| B16 | (F6) inner dh_pub all-zero, re-signed; inner dh_pub another low-order point, re-signed | dh_pub_invalid (handshake/mod.rs:2035-2037); dh_noncontributory (:2049-2054, :889-890) | PQ |
| B17 | (F6) inner ML-DSA signature bit flip, re-signed outer | sig_invalid (:2093-2095) | PQ |
| B18 | (F6) truncated at every field boundary; each length prefix +1 and -1; one trailing byte; ENV_MAX+1 | handshake_envelope_malformed; ENV_MAX+1: handshake_envelope_too_large (no parse) | DHE |
| B19 | (F6) the responder sig pin written at redeem is absent (fixture) | responder_sig_unpinned (step 6; handshake/mod.rs:1091) | VHE |
| B20 | (F6) the responder identity pin written at redeem replaced by I_X's (fixture) | peer_mismatch (step 6) | VHE |
Local key material (F6)
| Id | Input | Expected | Delta |
|---|---|---|---|
| D1 | a decapsulation key the suite refuses (fixture: the responder's identity kem_sk; the initiator's candidate kem_sk or eph_kem_sk made unreadable). The envelope's exact lengths leave no wire path to a decapsulation refusal as read (stdcrypto.rs:198-206 refuses only on key or ciphertext bytes) | resp_kem_decap_failed (responder); pq_decap_failed (initiator); before any write | PQ |
Invite code
| Id | Input | Expected | Delta |
|---|---|---|---|
| C01 | 2049-byte paste | invite_code_too_long, before base64 | DIC |
| C02 | QSLI-1- code minted by M | invite_version_unsupported, before network | DIC |
| C03 | payload profile = -03 | invite_profile_unsupported, before network | DIC |
| C04 | relay_ep not normalized | invite_malformed | DIC |
| C05 | payload trailing byte; each field truncated | invite_malformed | DIC |
| C06 | non-canonical base64 trailing bits | invite_malformed | DIC |
| C07 | own code (minted id, commitment) | invite_self, before any durable write (kept) | DIC caller |
| C08 | relay learns only redeem_cap: redeem request body carries wire_id(redeem_cap), never bearer_secret | assert on the request bytes | RC |
| C09 | (F6) QSLI-3- prefix | invite_version_newer, before network | DIC |
| C10 | (F6) payload type 0x02 | invite_type_unknown, before network | DIC |
| C11 | (F6) NEW claimant (no committed operation) after the local expiry | invite_expired at R3, before network | DIC caller (redeem) |
| C12 | (F6) relay serves a substitute bundle | invite_commitment_mismatch at R5, before any contact, pin or pending write | VRB |
| C13 | (F6) a signed code byte altered (e.g. expiry), bundle genuine | invite_signature_invalid at R5, before any contact, pin or pending write | VRB |
| C14 | (F6) alias fails the successor label rule at redeem | contacts_alias_invalid at R3, before network | DIC caller (redeem) |
Expiry and resume (F2)
| Id | Input | Expected | Delta |
|---|---|---|---|
| EX1 | committed redeem operation retried after the invite's expiry, before recovery_until (fixture horizon; the value is C03's, T8 OC3) | R2b finds it; resumed at R4 with the exact saved bytes; no invite_expired | DIC caller (redeem) |
| EX2 | the inviter pulls after its local expiry; an A1 the relay admitted before expiry is in the slot | steps 1-10 proceed; responder candidate committed; B1 sent | LP caller (accept, L0) |
Replay (T3)
| Id | Input | Expected | Delta |
|---|---|---|---|
| RP1v | P2 envelope again after step 10 | exact saved B1 re-sent; no new state | LP |
| RP2v | P2 re-built with a new attempt_id by I_B | handshake_replay_conflict | LP |
| RP3v | valid A1 from I_X to the same, redeemed invite | invite_already_redeemed | LP |
| RP4v | A1 reusing a committed session's sid | REJECT_QSC_HS_REPLAY | LP |
| RP5v | P3 B1 again after A2 | exact saved A2 re-sent | LP |
| RP6v | QSLH-1 envelope (first bytes 01 01) and QSLH-1 response (01 02) | handshake_envelope_version_retired | DHE |
| RP8v | P2 after the recovery horizon | dispose; no durable row | NOT_RUN until C03 (T8 OC3) |
Counterfeit and crossing (T4)
| Id | Scenario | Expected | Delta |
|---|---|---|---|
| CF1 | crossing; counterfeit = I_A's real A1 with sid changed, no key; then the real A1 | counterfeit invite_bearer_invalid (or _signature_invalid with the code but no key), no write; real completes (flips P28 t:1555-1613) | VHE, LP |
| CF2 | first contact; I_X-signed A1 without the code | invite_bearer_invalid; the real A1 then completes | BT |
| CF3 | first contact; I_X-signed A1 WITH the code, before the real one | accepted as first contact (bearer limit, T5); the real A1 -> invite_already_redeemed | LP |
| CF4 | crossing; I_X-signed A1 into I_B, which pins I_A | peer_mismatch | VHE |
| X1 | both redeem; I_A's A1 reaches I_B first, then I_B's A1 reaches I_A | winner = I_A's attempt; I_A defers I_B's A1; one session, sid of I_A's A1 | CW |
| X2 | same, reverse arrival | identical winner and sid as X1 | CW |
| X3 | X1 with A2 delayed behind two batches of other traffic | same winner; I_B's superseded attempt never selected; a B1 for it -> handshake_crossing_superseded | CW, LS |
| X4 | I_B holds two aliases for I_A | second alias refused handshake_lifecycle_conflict; one session | LP |
| X5 | lost HTTP response after step 10, retry | exact saved bytes; winner unchanged | LS |

==============================================================================================================
T8. OPEN CELLS (each with the cards it blocks). CT = CRYPTO-TOUCHING (SR-15 sensitive review required)
==============================================================================================================
| Id | Question | Depends on | Blocks |
|---|---|---|---|
| OC1 | Representation of the reply route capability in fields 9-10 (deposit capability D vs today's route token) and the canonical relay_ep form | C03 | F08, F10 |
| OC2 | attempt_id's relation to C03's operation/request identity, recovery secret and initial-push ticket | C03 | F09, F10 |
| OC3 | recovery_until horizon, retention and cleanup of receipts and exact replies (T3) | C03 | F09, F10 |
| OC4 | relay frame ceiling >= 11729 B for B1 envelopes and >= 9668 B for A1; v2 invite-slot namespace; the relay stores cap_hash of redeem_cap unchanged in shape | C03 | F08 |
| OC5 | count and byte quotas of first-connection records (P28 64 x 256 KiB candidate) | C04 | F10 |
| OC6 | ACK disposition per T2 refusal class; defer/dispose of a crossing loser; DTO state names; unverified-send policy | C05 | F11, F13 |
| OC7 | Known-contact re-handshake (no invitation): proposal = the same envelope with its own env_type values, no invite/bearer fields, pins REQUIRED both sides; until decided the successor accepts no bare A1/B1 | this contract | F10 (non-invite path), F11 |
| OC8 | Byte values of every vector | C01 O2 (A02 value) | F10 |
| OC9 | QHSM v3 (T1c) and the A04 table amendment (CT: frame version enters the transcript) -- RESOLVED (E3): version 3; A04 amended in the commit PR | Director, C01 O11 | F10 |
| OC10 | ML-DSA hedged signing and a context string (audit F-11); NOT part of K-07; unchanged here (CT) | Director | F10 if adopted |
| OC11 | ML-KEM encapsulation-key canonical check on eph_kem_pk and identity keys (audit F-10) (CT) | Director | F10 |
| OC12 | Registration of the NEW codes through DOC-SCL-002 (as C01 O9) | implementing PR | F10, F12 |
| OC13 | Refusal-code cells of C01 T2 rows D/E after the layout change (E2) -- RESOLVED (E2): C01 AMENDMENT in the commit PR, one code per old-peer path | Director | F10 assertions |
| OC14 | Contact status vocabulary: "pinned" -> TRUSTED today (E5) -- RULED in part (E5): that mapping does not survive into the successor's first-contact state; the spellings stay C05/F13's | C05 | F13 |
| OC15 | (F1) Cross-relay replies (a reply relay other than the invitation's): precondition = per-endpoint scoping of the client's relay credential (today one token for every push, transport/mod.rs:2059-2067, :2996-2999, :4246-4248; ENG-0051) | C03 | F08, F10 |
CRYPTO-TOUCHING ROWS: T1a (redeem_cap derivation), T1b (envelope signature, DS_ENV_*, bearer key and tag), T1c (v3
layouts, combiner, eph key lifetime), T1d (label set), T6 row 1, T8 OC9-OC11. Required: SR-15 independent sensitive design
review (fable/xhigh) of this contract, and the same at the implementing PR (F10 "independent handshake/security review
before downstream reliance").

==============================================================================================================
REUSE FROM #1828 / #1825 (selective; nothing imported wholesale)
==============================================================================================================
| Reused behaviour | Where | Used in |
|---|---|---|
| Receipt of the exact envelope and frame | P28 handshake.rs:227-233, :690-710 | T3 |
| Exact saved replies re-sent on duplicates | P28 :910-919, :966-971, :1013-1017 | T3 RP1, RP5 |
| Generation guard; one outgoing + one responder candidate | P28 :578-598, :808-844 | T4 X-d |
| Selection before projections; never overwrite an advanced same-SID session | P28 :600-675 | T4 X-d |
| Identity-ordered crossing (lower keeps its outgoing) | P28 :972 | T4 X-a (key changed to C(x)) |
| Alias ambiguity refused | P28 :507-513 | T4 X-f |
| Zeroize after apply | P28 :646-670 | T1c, T3 |
| Route kept out of the contact until authentication | P28 invite.rs:1587-1588, :1934-1935 | T2 step 10 (now the route is signed) |
| Crossed-invitation characterisations | P25 t:1181-1195 | T7 X1-X2 (desired progress) |
| NOT reused: responder reservation on a pin-matched, unauthenticated A1 | P28 :3709-3726 | replaced by T2 |
| NOT reused: deferred receipt written before authentication | P28 :975-980 | replaced by T4 X-b |
| NOT reused: "blocked" as the counterfeit-first expectation | P28 t:1555-1613 | T7 CF1 inverts it |

==============================================================================================================
ESCALATIONS RAISED WITH THIS DRAFT (named, NOT resolved by choice; the same list heads the drafting mission's REPORT.md)
==============================================================================================================
| Id | Conflict | Between | Draft's position (a proposal only); RULING (RULING_NA0783_C02_ACCEPT) |
|---|---|---|---|
| E1 | THE PLAN sec 2 "Preserve the inner handshake's transcript/crypto rather than inventing another key exchange" vs K-07 "ephemeral ML-KEM in A1", which changes the inner key schedule and the A1/B1 layouts | THE PLAN / RULING_PLAN_F00 K-07 | drafted as K-07 rules it (T1c, T6), read as a contribution appended to the existing combiner, not a new exchange; the reading is the Director's. RULED: consistent with THE PLAN sec 2; D06 mapping in the D-record |
| E2 | C01 T2 rows D/E (accepted) name the codes an old peer returns (UNKNOWN_CRITICAL at M, INTEGRATION_PROFILE at C). After the layout change an old peer refuses earlier: its QSLH-1 decoder sees 0x51 (handshake_envelope_version_newer, invite.rs:504-507) on the invite path; on a bare path the frame-length check (REJECT_QSC_HS_MALFORMED_LENGTH, handshake.rs:485-486) under v2 or the version check (handshake_version, :500) under v3 | C01 / K-07 | refusal-before-effects still holds; the code cells need a C01 amendment (T8 OC13). RULED: C01 AMENDMENT in this PR, codes per old-peer path |
| E3 | DOC-CAN-003 sec 12 row A04 is ALLOCATED as QHSM version 2 at main; T1c proposes 3 | C01 table / this draft | O11 was left to C02; any bump is a table amendment in the commit PR (T8 OC9). RULED: O11 RESOLVED = version 3 (A04 row edit) |
| E4 | C01 row 12 and A04 list "QSC.HS.SID" as a domain label C02 owns; measured, it is only the RNG-failure test-seam label (handshake.rs:754-767, :1500-1502) | C01 / tree | classification correction only (T1d); no crypto effect. RULED: reclassified as a test-seam label (C01 AMENDMENT) |
| E5 | The accept path writes contact status "pinned" before any authentication (invite.rs:1536-1543) and "pinned" maps to device state TRUSTED (contacts.rs:86, :956), while the tree's own doc calls it PENDING (contacts.rs:868-871) and THE PLAN I02 says possession is not verification | tree / THE PLAN I02 | T5 states; vocabulary decision at C05/F13 (T8 OC14). RULED: see T5 (E5) |
| E6 | The directive's T2 order is bounds -> canonical -> signature -> pin -> occupancy; the draft adds pure binding and a bearer MAC before the signature, and replay, crossing and pure compute after the pin | directive / this draft | occupancy stays last; THE PLAN's own list (lengths, canonical form, signature, pin before the slot) is met. RULED: accepted with F3 |
| E7 | The bearer secret replaces v1 `cap` and derives the relay-facing capability (T1a): a change to the content (not the shape) of what the relay compares, which borders C03's capability scope | C02 / C03 | proposed here because C02 owns the invite wire (C01 row 12); C03 confirms (T8 OC4). RULED: accepted; relay measured at qsl-server 5ea0f925 (T1a) |

END OF C02 FINAL
