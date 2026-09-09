QSL PROGRAM RECOMMENDATIONS -- EIGHT ITEMS BEYOND THE FINDINGS -- 2026-09-04
============================================================================
Prepared for the Director by the external audit seat (Claude). Companion to:
  AUDIT_qsl-protocol_security_2026-09-03.md            sha256 e4d91c08...8da3
  AUDIT_qsl-server_desktop_interaction_2026-09-03.md   sha256 ea256c99...ae03
NOT a ruling and NOT repo truth. These are recommendations the operator may
promote, decline, or overrule; per SR-14 they exist as a file because chat is
never canonical. Finding ids (F-nn, S-nn, G-nn) refer to the two audits.

WHY THIS FILE EXISTS
  The two audit reports carry finding-level fixes and a suggested lane shape.
  Fragments of what follows appear there (F-03's harness gate; report 1 s7.3
  "cargo audit still owed"; report 1 s8 lane ordering; S-01's note tying the
  capability split to WF-0086; G-02's pre-merge checklist). The eight items
  AS A PROGRAM-LEVEL SET were stated only in chat. This file is that set.

CONVENTIONS
  Owner chairs: OPERATOR decides / DIRECTOR drafts / SEAT executes.
  Size: XS (one edit), S (one lane), M (one lane with SR-15), L (an epic).
  Every item names its evidence in the audits and its acceptance signal.


R-1  A TWO-PARTY INTERLEAVING SIMULATOR AS A CI GATE
----------------------------------------------------
  Evidence: F-01, F-02, F-03. All three live in the one region the existing
  test corpus cannot see -- two peers acting concurrently, frames in flight,
  loss, reordering. The corpus is sequential and single-writer; 187 days of
  green never exercised a crossed boundary. This is the highest-leverage
  engineering change available to the program.
  What: a property-based test over the refimpl state machine. proptest is
  already a qsc dev-dependency (qsc/Cargo.toml :42). Model: two
  Suite2SessionState values plus one FIFO queue per direction; operations
  drawn at random from {send, send_boundary, send_pq_advertise,
  send_pq_reseed, deliver_next, drop_next, deliver_out_of_order(k)}.
  Invariants after quiescence: every non-dropped frame was delivered; no
  REJECT_* on honest traffic; both peers reach `established`; the DH pubs
  and roots on both sides agree. proptest shrinking yields the minimal
  counterexample, which is the bug report.
  Seed: AUDIT_harness_exp.rs (sha 67313de6...cb3c) already drives the
  unmodified ratchet with mock primitives; with the pinned toolchain use
  StdCrypto instead.
  Acceptance (red first, per house rule): on current main it MUST fail on
  the F-01, F-02 and F-03 shapes before it is trusted; after R-2 it passes.
  Owner: DIRECTOR drafts the brief; SEAT builds; OPERATOR promotes.  Size: M.
  When: now, in parallel with the Invitations lane.

R-2  LAND THE RATCHET FIXES BEFORE THE MESSAGING SLICE
------------------------------------------------------
  Evidence: F-01 (crossing DH boundaries wedge both directions), F-02 (PQ
  reseed loses in-flight reverse traffic and desynchronises), F-03
  (out-of-order reach collapses to one frame at nr >= 95).
  Why before, not after: the roadmap places messaging last, which also
  places the first two-human concurrent use last. When it happens it will
  present as a relay or delivery bug -- the exact misattribution that cost
  the previous five causes -- and the only recovery today is a re-handshake
  that replaces the session (see R-4).
  What: report 1 s8 Lane A (F-01 + F-02, one design brief, SR-15 by
  construction, DOC-CAN-003 s8.5.2/s8.5.3 amendment) and Lane B (F-03,
  order-only change in recv_nonboundary_ooo, experiment A as its gate).
  Acceptance: R-1's simulator green; experiment-A table reports max
  received gap >= 90 for every nr; experiments B and C deliver.
  Owner: OPERATOR promotes; DIRECTOR drafts; SEAT executes.  Size: M + S.
  When: alongside Invitations, before onboarding closes.

R-3  DECIDE THE RELAY CAPABILITY SPLIT BEFORE THE INVITATIONS DESIGN IS BANKED
------------------------------------------------------------------------------
  Evidence: S-01 (address = read/ack capability; every contact can drain
  and observe your inbox).
  Why now: the invite envelope and the B1 wrapper carry the route token
  (invite/mod.rs :451-460, :747-769). If the split lands later the invite
  format changes twice, and WF-0086's remote round-trip gate would be
  built against the wrong contract.
  The decision, with three doors:
    (a) split now: read_secret in the vault, deposit_token derived by one
        hash, pull/ack present the preimage in a new header (report 2 s1);
    (b) split later: version the envelope now so the second change is a
        field, not a format;
    (c) accept the risk: record in the claims document (R-5) that any
        contact can delete or delay your inbox.
  Recommendation: (a). It needs no registration step and leaves push
  untouched.
  Acceptance: a contact holding only the deposit token gets 403 on pull
  and ack; the owner succeeds; the negative control runs against loopback
  AND the AWS relay.
  Owner: OPERATOR decides (XS); implementation is one cross-repo lane (M)
  touching relay, qsc transport/contacts/invite, and the desktop pin.
  When: the decision before the Invitations kickoff consumes the envelope
  format; the implementation inside or immediately after that lane.

R-4  AN EXPLICIT SESSION-RESET AND RECOVERY DESIGN
--------------------------------------------------
  Evidence: F-01/F-02 (desync with no recovery path); ENG-0142 (a
  re-handshake makes undelivered traffic permanently undecryptable);
  report 2 S-04 (unbounded redelivery). Even after R-2, a crash between
  session store and relay ack will corrupt state occasionally; the
  program's own persistence documents say so.
  What: an authenticated RESET control frame, sent under the current
  session keys when they still work and as a fresh A1 when they do not;
  the rule "drain and decrypt the inbox BEFORE replacing the session";
  a receiver-side guard that refuses to replace a session while leased
  frames remain unacked; and a visible UI state ("conversation reset by
  <peer>") because silence is not success. Home: the diagnostics lane.
  Acceptance: R-1's simulator extended with a `reset` operation shows no
  acked-but-undecrypted frame is ever dropped.
  Owner: OPERATOR blesses the design (banked verbatim per SR-14);
  DIRECTOR drafts.  Size: M design + M implementation.
  When: before the messaging slice.

R-5  EXTERNAL CRYPTOGRAPHIC REVIEW, AND THE HONEST CLAIMS DOCUMENT FIRST
------------------------------------------------------------------------
  Evidence: F-04 (handshake forward secrecy rests on X25519 alone until
  the first reseed), F-05 (authentication is PQ-only), F-08 (control frames
  are size-distinguishable), S-01, S-08; and the gap between
  specs/00_security_objectives.md (QSP v4.3.2, Suite-1B, Key Transparency
  required) and what ships (Suite-2, pinned fingerprints, no KT).
  What, in order:
    1. A two-page "what we ship" statement: hybrid confidentiality;
       PQ-only authentication; the forward-secrecy boundary; what the relay
       sees; what a contact can do (S-01 until R-3); padding floors;
       retention; the standing "no security-assurance claims" line. This
       is also the substrate of the no-warranty page the legal track owes.
    2. An external review of DOC-CAN-003/004 with a named question list:
       the header-nonce-from-counter design (it forces trial decryption
       and bounds out-of-order recovery); epoch-transition semantics; each
       deviation from the published SPQR / Triple Ratchet design the code
       already cites; the KMAC hybrid combiner; the invite capability
       model; the vault KDF parameters (F-09).
  Acceptance: the claims document is linked from every README; the review
  report is banked with a sha and its findings become ledger entries.
  Owner: OPERATOR (engagement and budget); DIRECTOR drafts the claims
  document and the question list.  Size: S (document) + external.
  When: the document now; the review before any public release.

R-6  THE RIGOUR-TO-PRODUCT RATIO
--------------------------------
  Evidence: the succession packet's own words ("this tenure shipped zero
  product; do not let the rigour become the work"); DECISIONS.md at
  ~45,000 lines; an open count no single needle reproduces (124 to 157
  depending on instrument); SR-17 remanded so every lane defaults to full
  ceremony.
  Three changes, each small:
    1. Un-remand SR-17 with a concrete tier table. Proposed: T0 records-
       only (one PR, no cold read); T1 code outside lock/crypto, <= 5
       files, targeted tests only (no cold read); T2 lock/crypto, > 5
       files, or a retired guard (full ceremony, SR-15). The interim rule
       "any surprise auto-upgrades the lane" is preserved verbatim.
       Because this governs the Director's own discretion, SR-15 applies
       to the rule itself.
    2. Compaction: a generated CURRENT.md (STATE line, open P1/P2 list,
       owed items) regenerated at every lane close, and DECISIONS.md
       archived by quarter so the live file is readable, not searchable.
       Make the open count derivable by ONE declared convention: the LAST
       `- Status:` bullet in an entry is its state (the convention the
       ENG-0250/0251/0252 closures already used), and a CI needle counts
       exactly that.
    3. A budget rule: for every records-only lane, one product lane
       before the next records-only lane.
  Acceptance: a lane of size XS (F-03) reaches main in one PR and one
  ruling; the open count is a CI output, not a debate.
  Owner: OPERATOR rules; DIRECTOR drafts.  Size: S.
  When: next records act.

R-7  THE CHEAP HYGIENE BUNDLE (one sweep lane, all XS)
-----------------------------------------------------
  - cargo audit as a scheduled GitHub Actions job on all four repos
    (report 1 s7.3: still owed; Actions is free for public repos);
    cargo deny optional.
  - cfg-gate every environment seam that touches keys or the clock
    (F-06: QSC_ALLOW_SEED_FALLBACK / QSC_UNSAFE_TEST_SEED_FALLBACK /
    QSC_QSP_SEED / QSC_UNSAFE_TEST_CLOCK_UNIX_S; G-01: QSLD_INJECT_MARKER),
    using the existing cfg(qsc_rng_failure_test_seam) pattern.
  - fail closed on a missing or short RELAY_TOKEN for any non-loopback
    bind (S-03, one `if` in qsl-server main.rs).
  - bound head-of-queue redelivery (S-04: a deliveries column, re-enqueue
    at the tail after 3 expirations, drop after 6 with a log event).
  - desktop qsc pin: bump as a pre-merge checklist item for every
    qsl-protocol PR that touches qsc, and stamp the pinned rev into
    app_info (G-02).
  - run WF-0086's remote round-trip gate; every delivery claim is loopback
    at n = 2 today.
  - redirect(Policy::none()) on the shared HTTP client (F-16); remove
    `encode gzip` from the Caddy example (S-07); ReadWritePaths without
    the binary directory (S-07).
  - the two-line canonical-encoding check on ML-KEM public keys (F-10) and
    hedged ML-DSA signing with a context string (F-11).
  Acceptance: each item carries its own one-line negative control (a
  release binary that ignores the seam variables; a relay that refuses to
  start; a poisoned frame that reaches the tail).
  Owner: SEAT executes under one DIRECTOR brief; OPERATOR merges.
  Size: S total.  When: any idle slot; none of it blocks anything.

R-8  TREAT INVITE CODES AS PASSWORDS IN THE UX
----------------------------------------------
  Evidence: the invite code carries the capability (invite/mod.rs codec);
  the relay's redeem is single-use (store.rs invite_redeem); the inviter
  pins whatever identity arrives in the QSLH-1 envelope. Whoever holds the
  code BECOMES the contact. The compensating control is the ratified
  two-tier fingerprint verification, and it is currently optional-looking.
  What: UI copy at creation ("anyone with this code becomes this contact
  until you verify them"); a shorter default expiry (the relay's default
  ceiling is 72 h, qsl-server lib.rs :97; the OPERATOR chooses the client
  default -- 24 h is a defensible floor); unverified contacts visibly
  marked in the contacts rail; a verify nudge on first message; a
  prominent revoke; and the code never rendered in a log or marker.
  Acceptance: the mockups 16/17 carry the verified/unverified state; the
  invitations lane's acceptance flight includes "redeem with a stolen
  code" and the resulting contact is shown unverified.
  Owner: OPERATOR (defaults and copy); fits the Invitations lane.  Size: S.
  When: inside the Invitations lane.


SUGGESTED ORDER (the operator blesses the queue)
------------------------------------------------
  NOW, before the Invitations kickoff consumes the envelope format:
      R-3 (decision), R-1, R-2 in parallel with Invitations, R-8 inside it.
  BEFORE THE MESSAGING SLICE:
      R-4, R-5 item 1 (the claims document), R-7.
  BEFORE ANY PUBLIC RELEASE:
      R-5 item 2 (external review), R-6, the legal track already owed.

  If only three are taken: R-1, R-2, R-3. Everything else compounds on
  those.

END OF RECOMMENDATIONS -- if this line is missing, the copy is truncated.
