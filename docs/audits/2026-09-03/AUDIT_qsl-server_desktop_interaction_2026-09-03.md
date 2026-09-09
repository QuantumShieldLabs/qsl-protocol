QSL-SERVER + QSL-DESKTOP SECURITY AUDIT -- WITH PROTOCOL INTERACTION -- 2026-09-03
==================================================================================
Prepared for the Director by an independent reviewer (Claude, external audit seat).
Companion to AUDIT_qsl-protocol_security_2026-09-03.md (sha256 e4d91c08...8da3).
NOT a ruling. NOT repo truth. Every line number is against the uploaded archives
and MUST be re-derived at the edit; neither archive carries git metadata, so no
commit sha is claimed.

Bases:
  qsl-server-main.zip   sha256 5149521e5e3d86b280772da3ba834faced741b3d1631d8410b56f8ec5107dea0
  qsl-desktop-main.zip  sha256 aef9f742512558a83006ef8a302cc9ff39fb217575f03d88f8ebc87d408e6b0d
  qsl-protocol-main.zip sha256 768ebe74fca6bea62859a347835830d7e12ff38ee8ff3ba82778ffe5bdcd5e72

PROVENANCE TAGS
  [R]  READ, with file:line.   [X] EXECUTED (none in this report: no toolchain
  that builds axum/rusqlite/tauri was available).   [P] NOT VERIFIED.

CLAIM BOUNDARY
  - The relay was read in full (src/lib.rs 1980 lines, src/store.rs 850 lines,
    src/main.rs 555 lines, packaging/). Its tests were not run.
  - The desktop was read for its trust boundaries: tauri.conf.json, the one
    capability file, src/lib.rs (all 677 lines), the IPC command surface in
    src/commands.rs (registration list and the destructive/config commands),
    paths.rs, settings.rs, markers.rs, and the webview code (ui/main.js scanned
    for injection sinks). The 3,715-line UI was not read line by line.
  - Interaction claims about qsc are readings of the qsl-protocol archive and
    are cited into it.


0. TRIAGE TABLE
---------------
ID    SEV     WHERE          TITLE
S-01  HIGH    relay+client   The route token is address AND read/ack capability:
                             every contact can drain and observe your inbox
S-02  MEDIUM  relay          One bearer for all users; global caps exhaustible by
                             any co-tenant; routes are created by push
S-03  MEDIUM  relay          Auth is optional: RELAY_TOKEN unset = open relay
S-04  MEDIUM  relay+client   Lease-without-ack poison (ENG-0142) has no
                             server-side bound: unbounded head-of-queue redelivery
S-05  LOW     relay          Client-controlled x-msg-id is trusted, echoed and logged
S-06  LOW     relay          No server-side shape validation of route tokens,
                             invite ids or cap hashes
S-07  LOW     packaging      Caddy `encode gzip` on the API; unit can write its
                             own binary directory
S-08  LOW     relay          Metadata retention: per-message info logs and 7-day
                             ciphertext retention; invite bundles stored in clear
G-01  LOW     desktop        Runtime env test seam (QSLD_INJECT_MARKER) and
                             QSLD_DATA_DIR redirection in the shipped binary
G-02  LOW     desktop        qsc git-rev pin lags qsl-protocol main
I-01  INFO    interaction    Quantification of protocol F-12 (unbounded pull body)
I-02  INFO    interaction    Token validity band: client 22..=128 charset vs relay
                             "any non-empty string"

Confirmed sound (Section 4) covers the desktop's CSP, capabilities, IPC
hygiene, XSS sinks, file modes, erase guards, and the relay's constant-time
compares, at-rest token hashing, fsync policy, transaction scoping, SQL
parameterisation and systemd hardening.


1. S-01  HIGH  The route token is the address and the read capability
---------------------------------------------------------------------
  Where [R]  qsl-server-main/src/lib.rs
    resolve_route_token   :997-1009   header X-QSL-Route-Token -> any non-empty
                                       string
    push_message          :1034-1051  channel -> route_key_for(channel) (:950-959,
                                       SHA-256 of the header string)
    pull_message          :1179-1200  SAME header, SAME route_key_for, then
                                       store.pull(&key, max, now, mode)
    ack_messages          :1236-1254  SAME header, SAME key, store.ack(...)
    store.rs pull         :703-790    leases or deletes by route_key only
    store.rs ack          :800-840    DELETE ... WHERE route_key = ?1 AND
                                       leased_until IS NOT NULL AND msg_id IN (..)
  Nothing distinguishes the mailbox OWNER from a SENDER. The only credential
  a pull needs beyond the shared bearer (S-02) is the route token, and the
  route token is exactly what every sender must hold to deliver to you.
  How peers obtain it, by design [R, qsl-protocol]: each account mints one
  route token at vault creation; the invite envelope carries the inviter's
  token to the redeemer (invite/mod.rs :451-460 HandshakeEnvelope.route_token)
  and the redeemer's comes back in the B1 wrapper (:747-769
  encode_envelope_resp); each lands in the other's contact record; `receive`
  pulls the account's own token (transport/mod.rs :257-263, the self-inbox).
  So: every contact you have ever accepted holds your inbox's read capability.

  What a malicious or compromised contact can do, with no cryptographic
  break and no relay-operator access:
    - pull your inbox with ack=lease and POST the ids to /v1/pull/ack:
      every frame from every other contact is deleted before you see it.
      Silent, repeatable, indistinguishable at your end from "nobody wrote".
    - pull without acking: every frame is delayed by one lease (60 s default,
      store.rs :7) per pass, indefinitely.
    - observe: item count, sizes, and timing of everyone messaging you (the
      contents stay encrypted; the padding floor is 1024 B so sizes are
      coarse). Combined with S-08 this is the MCS adversary's view handed to
      any contact.
  The same holds for invite slots: invite_id is the slot's route token and it
  is inside the invite code, so a redeemer can pull the slot -- harmless
  today because the slot only ever holds the redeemer's own envelope.

  Fix (design; touches relay, client transport, contacts and the invite
  envelope; the Invitations lane is the natural carrier)
    Split the capability. The client mints a 256-bit inbox_read_secret and
    DERIVES the shareable address from it:
        deposit_token = base64url( SHA-256( "QSL.deposit.v1" || read_secret ) )
    Peers receive and store only deposit_token (it is what the invite
    envelope and contact records carry; its shape is the current token
    band, so contacts/relay code that already handles tokens is unchanged).
    Push keeps X-QSL-Route-Token = deposit_token (relay unchanged for push).
    Pull and ack present the read secret in a NEW header
    (X-QSL-Inbox-Secret); the relay derives deposit_token from it with the
    same hash and continues with route_key_for(deposit_token) as today. No
    registration step, no new table, no owner binding race: possession of
    the preimage is the proof. A pull that presents a bare deposit token
    must be refused (ERR_READ_SECRET_REQUIRED) after a transition window.
    Client: the read secret joins the vault beside the token it replaces;
    `--mailbox` (the CLI lab affordance) takes the read secret. Invite
    slots: the inviter mints the slot from a slot read secret the same way,
    and the invite code carries only the derived id.
    Negative control: a test in which a contact holding the deposit token
    attempts pull and ack must receive 403, while the owner succeeds.
  Ledger shape: ENG, P1, "mailbox read capability = address"; record the
  dependency with WF-0086's remote round-trip gate so the gate exercises
  the split.


2. S-02  MEDIUM  One bearer for every user; caps are global and creation is implicit
-----------------------------------------------------------------------------------
  Where [R]  lib.rs auth_ok :961-982 (one RELAY_TOKEN); push_message
  :1065-1078 and store.rs enqueue :654-690 (a push CREATES a route when none
  exists, capped by max_route_count); constants :115-117
  MAX_QUEUE_DEPTH_CEILING 257, MAX_ROUTE_COUNT_CEILING 256; invite slots
  MAX_INVITE_SLOTS_DEFAULT 256 (:92); push rate buckets are PER ROUTE
  (:1080-1095), not per sender.
  Consequence
    Any holder of the shared bearer can, cheaply: push one byte to 256
    random routes and block every NEW route for the retention window (7
    days, store.rs :5) -- ERR_ROUTE_CAP for everyone; fill a victim's queue
    to max_queue_depth so legitimate senders get ERR_OVERLOADED; spend the
    victim's per-route push rate bucket; create invite slots to the global
    cap. None of this is attributable: the relay has no notion of WHO.
    The MAX_ROUTE_COUNT_CEILING of 256 is also a hard product ceiling on
    concurrent conversations per relay.
  Fix
    Per-user bearer credentials issued by the relay operator (one per
    account; the invite code already carries relay_ep, add nothing to it --
    the redeemer must already have their own credential for that relay).
    Quotas then key on the credential: route creation, invite creation,
    push rate. Route creation should become an owner act (S-01's read-secret
    header on a `PUT /v1/inbox` or, cheaper, "a route exists only after its
    owner has pulled it once"), so a stranger's push cannot mint a route.
    Raise or remove the 256 ceilings with the per-user quotas in place.


3. S-03  MEDIUM  Auth is optional: an unset RELAY_TOKEN is an open relay
------------------------------------------------------------------------
  Where [R]  lib.rs :361-365, :375, :401 (RELAY_TOKEN read as Option; empty
  filtered to None); auth_ok :962-963 (None => true); main.rs :251 (a
  warning, not a refusal); packaging/runbook_ubuntu.md :36 ("set a strong
  token"); docs/server/DOC-SRV-001 :70 ("relay auth is disabled;
  deployments must rely on compensating controls").
  Consequence
    A deployment that forgets the variable (or an env file that fails to
    load: EnvironmentFile= in the unit is not `-` prefixed, so a missing
    file stops the unit, but an EMPTY variable does not) serves push, pull
    and ack to the whole internet behind Caddy's valid TLS.
  Fix
    Fail closed: refuse to start when the bind address is not loopback and
    RELAY_TOKEN is unset or shorter than 32 bytes; keep the open mode only
    for 127.0.0.1 (the in-process test relay). One `if` in main.rs.


4. S-04  MEDIUM  Head-of-queue redelivery is unbounded (the server half of ENG-0142)
----------------------------------------------------------------------------------
  Where [R]  store.rs pull :721-724 (`WHERE ... leased_until IS NULL OR
  leased_until <= ?2 ORDER BY seq`): an item whose lease expired is
  redelivered FIRST on every pull, forever; no delivery counter exists
  (git grep redeliver|attempt|poison|park in src: 0 hits). PULL_LEASE_SECS
  default 60 (:7), ceiling 3600 (:8).
  Client half [R, qsl-protocol]: relay_inbox_pull leases and never acks
  (three callers per the ENG-0142 amendment); `receive` aborts the whole
  pull on one undecodable frame (transport/mod.rs :1249). The operator
  accepted the remainder on 2026-08-26 ("accept for now", DECISIONS.md).
  Why the server should still change
    The accepted risk assumes the client fix lands; a server-side bound
    costs one column and turns "permanent wedge" into "bounded delay" for
    every future client bug of this class, and for a hostile sender who
    deliberately pushes an undecodable frame to the head of your queue.
  Fix
    Add `deliveries INTEGER NOT NULL DEFAULT 0` to messages; increment on
    every lease; after K expirations (K = 3) re-enqueue the item at the TAIL
    (new seq) with a log event, and after K*2 drop it with
    event=poison_dropped. The client's ENG-0142 repair remains the primary
    fix; this is the relay refusing to be the amplifier.


5. LOW FINDINGS (relay and packaging)
-------------------------------------
  S-05  x-msg-id is client-controlled, unbounded, echoed and logged  [R]
    lib.rs :1045-1050 takes any non-empty header value as the message id;
    store.rs :686-691 inserts it without uniqueness; :1161-1166 logs it in
    the push line; pull returns it to the recipient (:1220-1223) and ack
    keys on it (:815-816). qsc never sends it (0 hits in qsc/src). A sender
    can forge log records (`push ... id=event=retention_expired ...` in a
    line the tests grep by prefix) and can collide an id with one the
    recipient is about to ack, so the ack deletes two rows. Fix: ignore the
    header, or require a UUID shape and reject otherwise.
  S-06  No shape validation on the relay side  [R]
    resolve_route_token :997-1009 accepts any non-empty string up to the
    HTTP header limit; invite_create :702-704 checks only non-empty for
    invite_id and cap_hash (the client's own comment, invite/mod.rs
    :25-28, records that the relay "stores whatever the client uploaded
    without validating its shape"). Mirror the client's band (22..=128,
    [A-Za-z0-9_-]) for route tokens and invite ids, and 64 lowercase hex
    for cap_hash; reject with ERR_BAD_ROUTE_TOKEN. Bounds header abuse and
    makes an encoding drift fail loud at the relay instead of as a "wrong
    capability".
  S-07  Packaging  [R]
    packaging/caddy/Caddyfile.example :2 `encode gzip` on an API whose
    bodies are random ciphertext (nothing to gain) and whose invite-redeem
    response carries a secret ticket beside caller-influenced data
    (compression-oracle shape). Remove `encode`. packaging/systemd unit:
    ReadWritePaths=/opt/qsl-server makes the binary directory writable by
    the service user; move state to StateDirectory only and leave /opt
    read-only. Everything else in the unit is good (NoNewPrivileges,
    ProtectSystem=strict, empty capability set, syscall filter).
  S-08  Metadata retention  [R]
    lib.rs :1161-1166, :1214-1219, :1263 log one info line per push, pull
    item and ack with a stable pseudonym (FNV-1a of the route token,
    :940-948), the id and the byte length; journald keeps them by its own
    policy. Undelivered ciphertext is retained 7 days (store.rs :5). Invite
    bundles (the inviter's identity keys) and expiry are stored in clear
    (:235-247) and returned to any redeemer. All by design and consistent
    with DOC-G5-004's "metadata only" posture; the gap is a stated
    retention bound for the logs (DOC-SRV-001 should name one, e.g. 24 h)
    and a note that FNV-1a is a linkable pseudonym, not a redaction.


6. DESKTOP FINDINGS
-------------------
  G-01  LOW  Runtime environment seams in the shipped binary  [R]
    src-tauri/src/lib.rs :561 gw.markers.inject_from_env() reads
    QSLD_INJECT_MARKER (markers.rs :143-149) into the notice buffer; paths.rs
    :19-22 honours QSLD_DATA_DIR for the whole data directory. Same class as
    protocol F-06, lower impact (a fake notice; a relocated store that the
    S0/S1 believed-state check :428-433 already watches). Put both behind
    the test cfg the protocol audit recommends.
  G-02  LOW  qsc git-rev pin lags main  [R]
    src-tauri/Cargo.toml :22 pins qsc at 63ece4fe. The protocol repo's own
    records cite later merges (DECISIONS.md: NA-0775 / D-1418 tightening
    read at 2c3c39b4). Whatever landed after the pin -- and every fix from
    these two audits -- is not in the GUI until the pin moves. Make the
    bump part of the pre-merge checklist for any qsl-protocol PR that
    touches qsc, and stamp the pinned rev into app_info so acceptance flights
    can read it.


7. INTERACTION NOTES
--------------------
  I-01  Protocol F-12 quantified: a pull may return max_queue_depth items
    (ceiling 257) of max_body_bytes each (ceiling 1 MiB): up to ~257 MiB
    per response at ceiling config, ~16 MiB at the AWS relay's 64 KiB; the
    client reads it with resp.json() and no cap (transport/mod.rs :3586).
    The relay also serialises `data: Vec<u8>` as a JSON array of numbers
    (lib.rs :517-520), 2.5-4x the byte size on the wire; base64 would
    shrink both the response and the client's parse cost.
  I-02  Token band: the client refuses tokens outside 22..=128
    [A-Za-z0-9_-] (adversarial/route.rs :21-28); the relay accepts anything
    non-empty. Nothing breaks today; S-06 aligns them.
  I-03  The lease default the packet marked UNKNOWN for the AWS box is 60 s
    in code (store.rs :7) and overridable by PULL_LEASE_SECS (main.rs :50,
    :104); the running value is still an operator measurement, not a code
    fact.


8. CONFIRMED SOUND (no action)
------------------------------
  Relay: bearer and capability compares are constant-time over a fixed
  digest (lib.rs :984-995); route tokens, revoke tokens and tickets are
  stored only as SHA-256 (:950-959, :729-730, :794-795); all SQL is
  parameterised; pull, ack, enqueue and invite consume run inside one
  transaction with synchronous=FULL and WAL (store.rs :207-213); the
  schema version fails closed against a newer store (:269-273); invite
  redeem is single-use and the ticket gates the slot push inside the same
  transaction (:1105-1122); invite create is rate-limited before parsing
  (:679-698); body size is checked before any store work (:1041-1043);
  bind defaults to 127.0.0.1 (main.rs :150-153); no request-tracing layer
  is installed, so Authorization and route headers are never logged
  (lib.rs :549-559); the unauthenticated server-info probe is a fixed
  two-key document (:895-938); the systemd unit is well hardened.
  Desktop: qsc is linked in-process (Cargo.toml :22), so no argv/env
  passphrase ingress; CSP is default-src 'self' with no script exceptions
  (tauri.conf.json); the single capability is core:default for the main
  window; every dynamic string reaches the DOM via textContent -- the only
  innerHTML writes are static icon constants or "" (ui/main.js :189, :206,
  :1233 ...); the relay URL is normalised through qsc's endpoint policy
  (commands.rs :699-706); unlock runs through the guarded, rate-limited
  path (:214); settings are 0600 and re-tightened at launch (settings.rs
  :123, :137-141; lib.rs :464); the data dir and qsc dir are 0700 (:468-473);
  erase refuses the CLI config dir by canonical path (:481-490) and removes
  legacy webview residue without following symlinks (:374-387); the
  notice surface exposes classifications, never marker text (:526-537).


9. WHAT WAS NOT DONE
--------------------
  - No relay test was executed; no HTTP traffic was generated.
  - ui/main.js was scanned for sinks, not read for logic.
  - The GUI's invite/contact flows were not exercised.
  - Caddy's runtime configuration on the AWS box is [P]; only the example
    file was read.

END OF AUDIT REPORT -- if this line is missing, the copy is truncated.
