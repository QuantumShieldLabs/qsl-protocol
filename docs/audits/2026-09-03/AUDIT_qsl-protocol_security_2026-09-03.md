QSL-PROTOCOL SECURITY AUDIT -- CODE AND CRYPTO -- 2026-09-03
============================================================
Prepared for the Director by an independent reviewer (Claude, external audit seat).
NOT a ruling. NOT repo truth. Nothing here governs until the Director verifies it
against the live tree and the operator promotes a lane.

Base: the uploaded archive qsl-protocol-main.zip
      sha256 768ebe74fca6bea62859a347835830d7e12ff38ee8ff3ba82778ffe5bdcd5e72
      (2550 entries). The archive carries no git metadata, so NO commit sha is
      claimed. Every line number below is against the files in that archive and
      MUST be re-derived at the edit.

PROVENANCE TAGS (per WF-0087: no figure without its instrument)
  [X]  EXECUTED. Measured by running the unmodified refimpl source in a harness
       (Section 6). The harness copy of suite2/ratchet.rs is byte-identical to the
       archive's: sha256 d4c12526e754272e432a983e44e29708d1617e78c9199c7865dd3a2939dde524.
  [R]  READ. A claim about source text, with file:line. Not executed.
  [D]  DEPENDENCY SOURCE READ. The pinned crate's source was fetched from
       static.crates.io and read.
  [P]  NOT VERIFIED. Stated as a boundary, not a finding.

CLAIM BOUNDARY, STATED FIRST
  - No Rust 1.95 toolchain was available. The harness compiled the refimpl
    suite2 module with rustc 1.75 and DEPENDENCY-FREE MOCK primitives that keep
    the algebra the state machine relies on (commutative DH; an AEAD that fails
    iff key, nonce, AD or ciphertext differ; deterministic PRF-shaped Hash/KMAC).
    [X] results therefore measure the STATE MACHINE, not cryptographic strength.
  - The [X] results are refimpl-level. Whether qsc drives the refimpl into each
    measured state is a [R] claim about qsc's trigger policy (F-01) or about
    ordinary in-flight traffic (F-02, F-03).
  - Not audited: qsl-server (separate repo), kt/ (unused by the product),
    qshield-cli (labelled demo-only), the refimpl qsp/ v4.3 handshake (labelled
    non-production, ENG-0019), Ed25519 call sites (none found in qsc), the GUI.
  - cargo audit was not run; the RustSec advisory database was fetched and the
    locked versions of the security-relevant crates were checked by hand (7.3).


0. TRIAGE TABLE
---------------
ID    SEV     CLASS                    TITLE                                          EVIDENCE
F-01  HIGH    availability/design      Crossing DH boundaries wedge the session       [X]+[R]
F-02  HIGH    availability/design      PQ reseed loses in-flight reverse traffic      [X]+[R]
F-03  HIGH    availability/impl        OOO recovery starves at nr >= 95               [X]+[R]
F-04  MEDIUM  crypto/design            Handshake FS vs quantum rests on X25519 only   [R]
F-05  MEDIUM  crypto/design (note)     Authentication is PQ-only, not hybrid          [R]
F-06  MEDIUM  impl/hardening           Unsafe test seams live in the shipped binary   [R]
F-07  MEDIUM  protocol-agility         Shipped invite flow hardcodes LegacyCompat     [R]
F-08  MEDIUM  metadata                 SCKA control frames are size-distinguishable   [X]+[R]
F-09  MEDIUM  vault/hardening          Argon2id at the floor; header params ignored   [R]
F-10  LOW     crypto/conformance       ml-kem 0.2.1 has no FIPS 203 7.2 ek check      [D]
F-11  LOW     crypto/hardening         ML-DSA signing uses the deterministic variant  [D]+[R]
F-12  LOW     impl/hardening           Unbounded relay response body on pull          [R]
F-13  LOW     impl/fail-open shapes    seal() returns empty Vec; u16 length casts     [R]
F-14  LOW     hygiene                  Debug on secret-bearing structs; no zeroize    [R]
F-15  LOW     impl/hardening           One key + random 96-bit nonces for all blobs   [R]
F-16  LOW     transport/hardening      Redirects followed by the shared HTTP client   [R]
F-17  LOW     supply chain             refimpl_actor pins ml-dsa 0.0.4 (pre-final)    [R]+[D]
F-18  LOW     ingress                  Vault passphrase env ingress still compiled    [R]

Recommended order (reviewer's, the operator decides): F-01 and F-02 together
(one design, one SR-15 cold read), then F-03 (one-function change, its own
negative control), then F-06/F-07 (cheap, closes real doors), then F-04, F-08,
F-09, then the LOWs as a single hardening lane.


1. HIGH FINDINGS
----------------

F-01  HIGH  Crossing DH boundaries wedge the session permanently (both directions)

  Where [R]
    tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs
      send_boundary          :1283-1382  commits st.rk = rk1 (:1372) and
                                          st.dh.dhs_priv = new_priv (:1373)
      recv_dh_boundary       :1389-1529  opens the header under ONE key only:
                                          NHK_r derived from the CURRENT root
                                          (:1440-1443, :1460-1468)
    qsl/qsl-client/qsc/src/lib.rs
      qsp_should_ratchet     :1776-1794  ratchet when pending_send_ratchet OR
                                          msgs_since_ratchet >= 4 OR 900 s elapsed
      qsp_unpack             :2307, :2426 "Any received message arms the
                                          reply-driven trigger" -- ANY message,
                                          not a new peer DH key
    qsl/qsl-client/qsc/src/protocol_state/mod.rs :277 (N=4), :279 (T=900 s)
    Your own record of the same mechanism, measured for the ack case:
      lib.rs :1959-1973 ("PERMANENT, BIDIRECTIONAL wedge"). The control-send
      workaround (deferring acks) does not cover USER sends.

  What happens
    Two peers who have each received something (the normal state of a
    conversation) both originate a boundary from the same root before pulling
    the other's. Each side's root moves to a different value; each side's
    receive header key is now derived from a root the other side never
    reached. Every frame in both directions fails header authentication from
    then on. The N=4 / 900 s fallbacks produce the same crossing with no reply
    at all (both silent > 15 min, both send).

  Measured [X] (harness experiment B, mock primitives, refimpl level)
    pre-crossing control: both directions deliver
    A originates boundary; B originates boundary from the same root
    roots differ afterwards: true
    A receives B's boundary:  ok=false  REJECT_S2_HDR_AUTH_FAIL
    B receives A's boundary:  ok=false  REJECT_S2_HDR_AUTH_FAIL
    next normal A->B:         Err(REJECT_S2_HDR_AUTH_FAIL)
    next normal B->A:         Err(REJECT_S2_HDR_AUTH_FAIL)
    control (same boundaries applied sequentially b1, a1, b2): all accepted

  Why it is design, not a typo
    The Double Ratchet avoids this by construction: a party generates a new
    DH key ONLY in response to a NEW key from the peer, which linearises the
    root chain so both sides apply the same DH pairs in the same order. QSL's
    trigger fires on any received message and on timers, so two parties can
    both hold an "un-answered" turn at once. No receive path retains the
    pre-boundary root, so there is no recovery short of a re-handshake --
    which replaces the session (D-record at NA-0738) and, under ENG-0142,
    makes any wedged traffic permanently undecryptable.

  Fix (design change; SR-15 triggers: crypto region)
    1. In qsp_unpack, set pending_send_ratchet = true ONLY in the DH-boundary
       and combined-boundary arms, and only when next_state.dh.dhr differs
       from st.dh.dhr. Remove the unconditional arming at :2307 and :2426.
    2. Make the N and T fallbacks conditional on the same predicate ("the
       peer's current dhr has not yet been answered by one of my boundaries",
       trackable as last_answered_dhr in QspTriggerState). A timer alone must
       never mint a key.
    3. Add the negative control that this program's discipline requires: a
       session-level test in which both sides call send_boundary from the
       same root; the fixed design must either prevent it (trigger) or
       recover from it (receive path). The harness in Section 6 already
       produces the failing shape.
    4. Amend DOC-CAN-003 s8.5.2 to state the linearisation rule explicitly.
    Optional defence in depth: retain the previous root for one epoch and, on
    header failure in recv_dh_boundary, retry under NHK from that root. This
    is more code in a lock/crypto region; the trigger fix alone removes the
    hazard for two honest clients.

  Ledger shape: ENG entry, P1, class "session availability / ratchet
  linearisation". Cross-reference lib.rs :1959-1973 as the prior measured
  instance.


F-02  HIGH  PQ reseed loses in-flight reverse traffic, then desynchronises the
            direction permanently

  Where [R]
    suite2/ratchet.rs
      send_pq_reseed           :1712-1834  commit block :1825-1832 replaces the
                                            SENDER'S receive schedule at once:
                                            st.recv.ck_pq_recv (:1830),
                                            st.recv.hk_r (:1831)
      recv_pq_reseed           :2227-2301  refreshes the receiver's send half
                                            (:2282-2292) -- correct, but see below
      recv_nonboundary_ooo     :317-651    opens headers under st.hk_r ONLY
      recv_boundary_in_order   :757        `let _ = pn;` -- the previous-chain
                                            length is decoded and discarded
      recv_dh_boundary         :1472-1478  reads n only; pn never used

  What happens
    B originates a reseed while a message from A is in flight (sent by A
    before A processed the reseed -- the ordinary case in a two-way
    conversation). B's receive header key is already the post-reseed one, so
    A's in-flight frame fails header auth and is lost. A then processes the
    reseed and switches its send PQ chain to the new seed at ITS ns; B's
    receive PQ chain restarted at B's nr, which is behind by the number of
    in-flight frames. Every later A->B frame fails body auth. Nothing
    recovers this.

  Measured [X] (harness experiment C)
    B receives A's in-flight message after originating the reseed:
                                          Err(REJECT_S2_HDR_AUTH_FAIL)
    A accepts B's reseed:                 ok=true
    A->B after reseed (1):                Err(REJECT_S2_BODY_AUTH_FAIL)
    A->B after reseed (2):                Err(REJECT_S2_BODY_AUTH_FAIL)
    control (no in-flight message):       Ok("post-reseed control")

  Same root cause as F-01: epoch transitions retain no previous-epoch keys,
  and pn -- the field that exists precisely so a receiver can seal the tail
  of the previous chain (Signal's SkipMessageKeys(PN)) -- is never consumed.
  The same-direction reorder across a DH boundary (a frame sent before the
  boundary, delivered after it) is lost by the same mechanism; a FIFO relay
  makes that rarer, not impossible.

  Fix (design change; SR-15 triggers)
    1. Keep one "previous receive epoch" (hk_r, ck_ec, ck_pq_recv, nr, and
       the mkskipped entries keyed by that epoch's dh_pub) after every
       transition the LOCAL side initiates (send_pq_reseed, send_boundary is
       not needed -- it does not touch the receive chain) and after every
       peer transition; on header failure under the current key, try the
       previous epoch's key before rejecting. Bound it by MAX_SKIP.
    2. Consume pn: when a boundary or reseed arrives with pn > nr, stage the
       skipped keys nr..pn-1 of the OLD chain before switching. This is the
       missing step of DOC-CAN-003 s8.5.3 step 7 and s8.5.2.
    3. Alternatively (smaller, sender-side only): on send_pq_reseed do NOT
       switch st.recv.* immediately; hold the post-reseed receive schedule as
       "pending" and adopt it on the first frame that authenticates under it.
       This removes the in-flight loss but not the cross-epoch reorder loss.
    4. Negative control: the harness C flow (one in-flight message, then a
       reseed) must deliver both the in-flight message and the post-reseed
       messages.

  Ledger shape: ENG entry, P1, sibling of F-01; amend DOC-CAN-003 s8.5.3.


F-03  HIGH  Out-of-order recovery starves once nr >= 95: a gap of two lost or
            reordered messages stalls the direction until a boundary

  Where [R]
    suite2/ratchet.rs recv_nonboundary_ooo
      MAX_HEADER_ATTEMPTS = 100 (:10), MAX_SKIP = 1000 (:11)
      probe order: seeds (:431-447) -> mkskipped (:450-456) ->
                   BACKWARD window nr-1 .. nr-1000 (:459-468) ->
                   forward window nr+2 .. (:471-483)
    The header nonce is derived from n (nonce_hdr :269-279), so the receiver
    must guess n by trial decryption; the budget is 100 trials. The backward
    window is tried before the forward window and, for nr >= 95, consumes
    every remaining trial. MAX_SKIP = 1000 is therefore nominal: the
    effective forward reach is min(1000, 96 - min(nr, 94)) - 1 or so, and it
    is 1 for every session that has exchanged more than ~95 messages.

  Measured [X] (harness experiment A; "gap" = frames missing before the one
  delivered; largest gap tried = 12)
    nr=0     max received gap 12    no failure
    nr=10    max received gap 12    no failure
    nr=50    max received gap 12    no failure
    nr=90    max received gap 6     gap 7  -> REJECT_S2_HDR_AUTH_FAIL
    nr=93    max received gap 3     gap 4  -> REJECT_S2_HDR_AUTH_FAIL
    nr=94    max received gap 2     gap 3  -> REJECT_S2_HDR_AUTH_FAIL
    nr=95    max received gap 1     gap 2  -> REJECT_S2_HDR_AUTH_FAIL
    nr=100   max received gap 1     gap 2  -> REJECT_S2_HDR_AUTH_FAIL
    nr=500   max received gap 1     gap 2  -> REJECT_S2_HDR_AUTH_FAIL
    nr=2000  max received gap 1     gap 2  -> REJECT_S2_HDR_AUTH_FAIL

  Impact
    Any two consecutive frames lost (relay retention expiry, a crash between
    ack and decrypt, a quarantined poison frame followed by loss) leaves the
    receiver unable to locate any later header: the direction is dead until
    the peer's next DH boundary. The reject code is HDR_AUTH_FAIL, which
    reads as tampering, so the diagnosis will look like an attack.

  Fix (one function; SR-15 by shape -- it is a lock/crypto region; the change
  is order-only and carries its own negative control)
    1. Probe the forward window BEFORE the backward window, and cap the
       backward probe at a small constant (8 is plenty for replay
       classification; a replay beyond it correctly reads HDR_AUTH_FAIL
       rather than REPLAY).
    2. Keep the seeds. Consider raising MAX_HEADER_ATTEMPTS only if profiling
       says the cost is acceptable; the reorder alone restores reach ~90.
    3. Negative control: the experiment-A table is the control; the fixed
       code must report max received gap >= 90 for every nr in the table.
    Longer term: the spec's choice to derive the header nonce from n is what
    forces guessing. A small AEAD-bound counter hint in the clear (Signal
    encrypts the header under HK with a random nonce carried in the clear)
    would remove the trial loop; that is a wire-format change and a DOC-CAN
    amendment, not a lane.

  Ledger shape: ENG entry, P1 (availability), "OOO probe ordering".


2. MEDIUM FINDINGS
------------------

F-04  MEDIUM  Handshake forward secrecy against a quantum adversary rests on
              X25519 alone; no ephemeral KEM in the handshake

  Where [R]  qsl/qsl-client/qsc/src/handshake/mod.rs
    :1493-1512  A1 carries kem_pk = the initiator's IDENTITY KEM key
                (identity_self_kem_keypair); the pending record stores kem_sk
    :2536       the responder encapsulates to init.kem_pk (identity key)
    :1482       the initiator encapsulates resp_kem_ct to the responder's
                PINNED IDENTITY KEM key
    :839-851    pq_init_ss = combine(ss_pq, resp_kem_ss) -- both secrets are
                decapsulable with a long-term identity key
    establish.rs :64-73  rk = KMAC(KMAC(dh_init, ...), pq_init_ss)
  Consequence
    A harvest-now/decrypt-later adversary who later obtains BOTH identity
    KEM secrets recovers pq_init_ss for every recorded handshake; the only
    remaining protection of the initial root is X25519 (dh_init), which the
    same adversary breaks. This contradicts specs/00_security_objectives.md
    s2.1 (FS: "compromise of long-term identity keys does not reveal past
    session message keys") for the interval before the first SCKA reseed.
    Compromise of ONE identity key is not sufficient (both contributions are
    needed); that is the correct part of the design.
  Fix
    Add an ephemeral ML-KEM-768 keypair to A1 (eph_kem_pk, 1184 bytes); the
    responder encapsulates to it and the combiner at :819-837 takes the extra
    secret as a list append: [ss_eph, ss_identity, resp_kem_ss]. Keep the
    identity-KEM contributions -- they carry the C1 (ENG-0038) authentication
    property. Erase the ephemeral secret at B1 processing. Wire-format change
    to A1 and B1; the transcript MAC covers it automatically. SR-15 triggers.

F-05  MEDIUM  Authentication is post-quantum-only, not hybrid (design note)

  Where [R]  handshake/mod.rs :954-982 (ML-DSA-65 only); no Ed25519 use in
  qsc (git grep StdEd25519|SigEd25519 in qsc/src: 0 hits). Peer
  authentication = possession of the pinned ML-KEM identity key (C1) plus an
  ML-DSA-65 signature on the transcript hash. Confidentiality is hybrid;
  authentication has no classical member. A classical break of either PQ
  scheme (implementation or mathematical) leaves no fallback. Record it as a
  design position in DOC-CAN-003 / the security objectives (s3.2 says
  "hybrid design goals"); adding an Ed25519 identity signature over the same
  message is cheap if the operator wants the claim to be true for
  authentication too.

F-06  MEDIUM  Unsafe test seams are compiled into the shipped binary and gated
              only by environment variables

  Where [R]  qsl/qsl-client/qsc/src/protocol_state/mod.rs
    :1017-1019  allow_unsafe_seed_fallback_for_tests() =
                env QSC_ALLOW_SEED_FALLBACK && env QSC_UNSAFE_TEST_SEED_FALLBACK
    :1040-1102  qsp_session_for_channel: with the gate set and no session on
                disk, EVERY session key (rk, hk, ck_ec, ck_pq, dh priv/pub)
                is derived from env QSC_QSP_SEED, a u64
    :143-156, :168-181  the session-store key falls back to a seed-derived
                key when the vault is missing or locked
    qsl/qsl-client/qsc/src/main.rs :54-57  the same gate marks the process
                "unlocked" with no passphrase
    qsl/qsl-client/qsc/src/clock/mod.rs :49-75  QSC_UNSAFE_TEST_CLOCK_UNIX_S
                moves the one clock that invite expiry, the vault unlock
                delay, and both ratchet cadences read
  Consequence
    Whoever can set three environment variables on the desktop process
    (launcher, .desktop entry, shell rc, a wrapper) makes the client encrypt
    under keys derived from a 64-bit seed and treat the vault as unlocked.
    The named-UNSAFE convention is a good alarm; it is not a boundary.
  Fix
    The repo already has the right pattern: cfg(qsc_rng_failure_test_seam)
    (handshake/mod.rs :726-767). Put the seed fallback and the clock override
    behind a cfg the test build sets (RUSTFLAGS --cfg qsc_unsafe_test_seams);
    subprocess-driven tests keep working because they build the binary with
    the cfg. Add a release-build assertion (a test that the release binary
    ignores the variables) as the negative control.

F-07  MEDIUM  The shipped invite flow hardcodes HandshakeSuiteMode::LegacyCompat

  Where [R]  qsl/qsl-client/qsc/src/invite/mod.rs :1126, :1380, :1561, :1694
    handshake/mod.rs :199-204  LegacyCompat => HsSuiteContext::LegacyV1 (no
                parameter block on the wire); :801-806 hs_append_key_context
                appends NOTHING for LegacyV1, so the suite tuple is absent
                from the key schedule; :205-213 your own note that the
                replay guard "needs an explicit suite context, and the invite
                path hardcodes LegacyCompat".
  Consequence today
    None exploitable: Suite-2 is the only suite, the wire version byte is
    inside the MAC'd A1 bytes, and a v1->v2 flip fails the transcript MAC.
    The downgrade machinery (REJECT_QSC_HS_DOWNGRADE, the explicit-context
    replay guard) is simply inert on the shipped path, and the objectives'
    s3.1(3) downgrade-resistance claim is unexercised.
  Consequence the day a second suite exists
    This default is the downgrade vector.
  Fix
    Flip the four call sites to SuiteRequired (both ends are this client);
    keep LegacyCompat only behind the CLI flag for old fixtures, and record
    a retirement date for it.

F-08  MEDIUM  SCKA control frames are size-distinguishable on the relay

  Where [R]  qsl/qsl-client/qsc/src/lib.rs
    :1833-1863  qsp_wrap_standard_envelope (the ADV carrier) pads only UP TO
                EnvelopeProfile::Standard.min_size_bytes() = 1024
    :2137-2153  the main-message path: same floor; the meta-pad ladder
                (:2154-2192) applies only when a MetaPadConfig is supplied
    refimpl qse/envelope.rs :14-20 (profile floors), envelope.rs BUCKET_SIZES
                (a ladder exists and is not applied here)
  Measured [X] (harness experiment D, 40-byte plaintext; QSE overhead 16 B)
    normal message      suite2 wire  124 B   on the relay 1024 B
    DH boundary         suite2 wire  124 B   on the relay 1024 B
    SCKA ADV            suite2 wire 1344 B   on the relay 1360 B
    PQ reseed (CTXT)    suite2 wire 1216 B   on the relay 1232 B (+ payload)
  Consequence
    A passive observer of the relay (the MCS adversary of the objectives)
    sees every PQ epoch transition and can tell an advertisement from a
    reseed. This is the traffic-signature class under which the
    "channel-established" auto-message was refused (WF-0085), arriving by a
    different door. The DH boundary is correctly indistinguishable.
  Fix
    Pad control carriers and any envelope above the floor to the next rung
    of the ladder (2048 here), i.e. apply bucket_for_len from envelope.rs to
    the encoded length rather than the fixed floor. Cost: ~700 B per control
    frame. Also consider co-scheduling an ADV only inside a pack that would
    already exceed the floor.

F-09  MEDIUM  Vault KDF at the floor, and the authenticated header parameters
              are ignored at unlock

  Where [R]  qsl/qsl-client/qsc/src/vault/mod.rs
    :45-47   KDF_M_KIB = 19456, KDF_T = 2, KDF_P = 1 (Argon2id, 19 MiB)
    :921-925 derive_runtime_key uses the CONSTANTS, not env.kdf_m_kib/kdf_t/
             kdf_p from the parsed (and AAD-authenticated, :997-1005) header
    vault/protection.rs :21-23  "an offline copy of the vault file is
             defended only by passphrase strength + Argon2id"
  Consequence
    19 MiB / t=2 is the OWASP minimum for interactive logins, chosen for a
    file that is the sole defence against offline attack for the stated user
    base. And because the header parameters are decorative, the cost cannot
    be raised per-vault; any change is a format version bump.
  Fix
    Honour the header parameters at unlock (they are already authenticated),
    raise the defaults to at least 64 MiB / t=3 (RFC 9106 second
    recommendation) with a measured unlock time on the target hardware, and
    re-wrap existing vaults on next successful unlock. Keychain-backed vaults
    (key_source 2) are unaffected.


3. LOW FINDINGS
---------------

F-10  LOW  ml-kem 0.2.1 performs no FIPS 203 s7.2 encapsulation-key check  [D]
  Fetched ml-kem-0.2.1 from static.crates.io. EncapsulationKey::from_bytes
  (kem.rs :177-179) -> EncryptionKey::from_bytes (pke.rs :138-145) ->
  decode_u12, which REDUCES coefficients mod q (encode.rs :60) instead of
  rejecting a non-canonical encoding; from_bytes is infallible. The KEM then
  hashes the RE-ENCODED canonical bytes (kem.rs :160), so a non-canonical ek
  yields the same shared secret as its canonical form. In QSL every ek is
  authenticated (pinned fingerprint over the bytes, or ADVAUTH MAC), so this
  is conformance, not exposure.
  Fix: at stdcrypto.rs :32-37, after from_bytes, compare ek.as_bytes() with
  the input and return InvalidKey on mismatch (two lines). Same for
  ml_kem768_dk_from_bytes if untrusted dk bytes are ever accepted.

F-11  LOW  ML-DSA-65 signing uses the deterministic variant  [D]+[R]
  ml-dsa 0.1.0-rc.7 implements FIPS 204 final (README :11). Its Signer::sign
  is the "optional deterministic variant" (lib.rs :240-252, :560); stdcrypto.rs
  :219 calls it with an empty context. FIPS 204 s3.4 makes the hedged variant
  the default; deterministic signing is permitted but raises fault-injection
  exposure of the identity key. Fix: use the RandomizedSigner path
  (sign_with_rng with OsRng); and pass a context string (e.g. b"QSC.HS") so
  the identity key's signatures are domain-separated at the algorithm level
  as well as in the message prefix.

F-12  LOW  Unbounded response body on relay pull  [R]
  transport/mod.rs :3586 resp.json::<InboxPullResp>() reads the whole body
  into memory with no client-side cap; a malicious or compromised relay (an
  in-scope adversary, objectives s2.2 MCS) can exhaust memory. The blocking
  client's default 30 s timeout bounds time, not size. Fix: read with
  Read::take(max * max_frame + slack) and reject oversize as
  relay_inbox_parse_failed.

F-13  LOW  Fail-open shapes in the primitive layer  [R]
  stdcrypto.rs :150  Aead::seal returns an empty Vec on error
  (unwrap_or_default); every caller checks is_empty (ratchet.rs :1047, :1359,
  :1679, :1810), so nothing ships an empty ciphertext today, but the trait
  shape invites a caller that does not. Return Result.
  ratchet.rs :1061-1062, :1271-1272, :1585-1586  header.len() as u16 and
  body_ct.len() as u16 truncate silently above 65535; the parser then rejects
  the frame (parse.rs :151-153), so the failure is loud at the receiver and
  silent at the sender. Bound plaintext at the pack site and fail closed.

F-14  LOW  Debug on secret-bearing structs; no zeroize on session state  [R]
  ratchet.rs :982-987 Suite2DhRatchetState derives Debug (holds dhs_priv);
  protocol_state/mod.rs :313-314 SckaAdvKey derives Debug (holds the ML-KEM
  secret). Suite2SendState / Suite2RecvWireState / MkSkippedEntry hold chain
  and message keys with no ZeroizeOnDrop. Fix: manual Debug impls that
  redact, and Zeroize derives (the crate already depends on zeroize).

F-15  LOW  One session-store key with random 96-bit nonces for every blob write  [R]
  protocol_state/mod.rs :824-846: ChaCha20-Poly1305, one key from the vault
  for all peers, a fresh random 12-byte nonce per write; qsc writes a blob on
  every send and receive. The 2^32-write birthday bound is unreachable for a
  desktop user, so this is hardening: XChaCha20-Poly1305 (24-byte nonce) or a
  per-peer subkey (KMAC(store_key, peer)) removes the bound.

F-16  LOW  The shared HTTP client follows redirects  [R]
  transport/mod.rs :2183-2200 relay_http_client() never sets a redirect
  policy; the relay API has no legitimate redirect. Set
  redirect(Policy::none()). (The earlier draft of this audit claimed https
  was not enforced; that was WRONG -- see Section 5.)

F-17  LOW  refimpl_actor pins ml-dsa 0.0.4  [R]+[D]
  tools/actors/refimpl_actor_rs/Cargo.toml :12. Pre-FIPS-204-final AND in
  the affected range of RUSTSEC-2025-0144 (timing side channel in Decompose;
  patched >= 0.1.0-rc.3). Test actor only; its signatures cannot interoperate
  with the product's rc.7 either. Pin it to the product's version.

F-18  LOW  Vault passphrase environment ingress is still compiled  [R]
  vault/mod.rs :49, :1408-1425 accepts the passphrase from
  QSC_DESKTOP_SESSION_PASSPHRASE; main.rs :38-58 wires it. A process
  environment is readable by every same-user process (/proc/<pid>/environ)
  and is inherited by children. NA-0649 gave the GUI an in-memory path; if
  nothing still spawns qsc with this variable [P: not verifiable from this
  repo], retire the ingress (vault_passphrase_env_retired already exists).


4. CONFIRMED SOUND (no action; recorded so the next reader does not re-audit)
-----------------------------------------------------------------------------
  - Contributory-DH checks on both the handshake (handshake/mod.rs :875-893)
    and the ratchet (ratchet.rs :1310-1312, :1485-1487); ENG-0034 holds.
  - Constant-time MAC compares (handshake/mod.rs :773-779, ratchet.rs
    :117-126).
  - Full-identity fingerprint is length-prefixed and domain-separated
    (identity/mod.rs :163-203, NA-0749); the voice form is ~2^99.7 of
    preimage resistance, on par with Signal's safety numbers.
  - Suite-2 parser is strict and bounded (parse.rs); unknown flags, trailing
    bytes and inconsistent lengths reject.
  - ADV control plane is authenticated (ADVAUTH MAC under the canonical
    root, ratchet.rs :95-114, :2135-2146); replay of a stale ADV fails the
    in-order counter check.
  - Replay handling: n < nr rejects REPLAY; a consumed skipped key is removed
    (ratchet.rs :497-528); a second delivery reads REPLAY.
  - Handshake: transcript MAC keyed on pq_init_ss binds A1 and B1 bytes; the
    suite-context block, when present, is in the key schedule; the
    responder pins the initiator's full identity (KEM + SIG) and the
    initiator requires both the primary and the SIG pin (NA-0634).
  - Pending-handshake secrets and identity secrets live in the vault, not on
    disk (handshake/mod.rs :1276-1284; identity/mod.rs :213-239).
  - Vault: the 53-byte header is AEAD associated data (vault/mod.rs
    :950-975); nonce per write is random; unlock is rate-limited with a
    rollback-proof delay schedule (protection.rs :104-134).
  - Invite: canonical, length-prefixed encodings; commitment over decoded
    bytes; strict trailing-byte rejection; relay endpoint validated (https
    anywhere, http on loopback only) at every relay base resolution
    (adversarial/route.rs :49-69; transport/mod.rs :2975, :3546, :3611,
    :4234; invite/mod.rs :859, :1468).
  - Attachments: fresh 32-byte CEK per attachment, 8-byte prefix + 32-bit
    part counter nonces, full AAD binding of id/lengths/class/index, and a
    domain-separated Merkle tree (attachments/mod.rs :204-270, :735-780).
  - Dependencies (RustSec, checked by hand against Cargo.lock): aes-gcm
    0.10.3, curve25519-dalek 4.1.3, rustls 0.23.36, ring 0.17.14, ml-dsa
    rc.7 are outside every applicable advisory range; ring carries only the
    informational "security maintenance by the rustls team" notice.


5. CORRECTIONS TO THIS AUDIT'S OWN FIRST PASS
---------------------------------------------
  - WITHDRAWN: "relay client does not enforce https." The policy exists and
    is enforced at every relay base-URL resolution (Section 4, invite item).
    The residual is F-16 (redirect policy). This was an instance of asserting
    from one file (relay_http_client) what is decided in another
    (normalize_relay_endpoint) -- the narrow-instrument shape.
  - The first pass's HIGH-1/2/3 were READINGS; they are now EXECUTED (Section
    6). The thresholds stated in the first pass ("nr >= ~95") reproduced
    exactly.


6. EVIDENCE: THE HARNESS AND ITS RESULTS
----------------------------------------
  Files delivered beside this report:
    AUDIT_harness_exp.rs        the experiment binary
                                sha256 67313de62dbd826333fbb810007a4701b3d717791bd41880321ef9c02bc4cb3c
    AUDIT_harness_results.txt   its complete output
                                sha256 d17ac9198e2467faadcf6e75dab7b2e5f966f0d6c4b771f178aaba874cc34b60
  How it was built (so a seat can reproduce on the real toolchain in one
  sitting):
    - copy tools/refimpl/quantumshield_refimpl/src/suite2/*.rs and
      crypto/traits.rs UNMODIFIED into a fresh crate (verify the ratchet.rs
      sha above); provide crypto/mod.rs (pub mod traits;), a minimal
      RefimplError with From<&'static str>, and lib.rs exposing crypto,
      refimpl_error, suite2. The only edit made here was to traits.rs to
      remove the thiserror/zeroize derives (rustc 1.75 cannot build the
      pinned zeroize/base64ct editions); with the pinned toolchain that edit
      is unnecessary -- use the real crate as a path dependency and the real
      StdCrypto for Hash/Kmac/Aead/X25519Dh.
    - run: cargo run --release --bin exp
  What the mocks are and are not: they preserve exactly the properties the
  ratchet depends on (key-nonce-AD-binding of AEAD, commutativity of DH,
  determinism of KDFs). They prove the state-machine outcomes; they prove
  nothing about the primitives. Re-running experiments A-D with StdCrypto
  is the confirming step and should take a seat under an hour.


7. WHAT WAS NOT DONE
--------------------
  7.1 Not read: qsl-server; kt/canonical.rs (1070 lines, no product caller);
      qshield-cli (demo-only per its README); the refimpl qsp/ handshake and
      ratchet (v4.3, labelled non-production); the Tauri GUI (other repo).
  7.2 Not executed: anything in qsc (no toolchain); the [X] tag applies to
      refimpl-level experiments only.
  7.3 Dependency check was manual against the RustSec database as fetched on
      2026-09-03 for: ml-kem, ml-dsa, argon2, chacha20poly1305, aes-gcm,
      x25519-dalek, ed25519-dalek, curve25519-dalek, tiny-keccak, reqwest,
      rustls, ring, zeroize, hybrid-array. cargo audit on the real lockfile
      is still owed.
  7.4 Not assessed: side channels beyond the two constant-time compares
      read; the KMAC implementation (tiny-keccak) was not reviewed.


8. SUGGESTED LANE SHAPE (recommendation only)
---------------------------------------------
  Lane A  F-01 + F-02, one design brief, SR-15 cold read by construction,
          harness experiments B and C promoted to session-level negative
          controls in the refimpl test suite BEFORE the fix lands (red first).
  Lane B  F-03, order-only change in one function, experiment A as its gate.
  Lane C  F-06 + F-07, cfg-gating and the SuiteRequired flip, small.
  Lane D  F-04 (A1/B1 wire change) with a DOC-CAN-003 amendment; F-05 as a
          recorded design position in the same records PR.
  Lane E  F-08 + F-09 + the LOWs as one hardening sweep.

END OF AUDIT REPORT -- if this line is missing, the copy is truncated.
