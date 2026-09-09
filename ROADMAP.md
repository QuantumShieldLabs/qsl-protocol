Goals: G1, G2, G3, G4, G5
Status: Authoritative
Owner: QSL governance
Last-Updated: 2026-09-09

# QuantumShield Roadmap

QuantumShield remains a research-stage protocol and demo system. Production readiness is not established. The September audits contain unresolved findings, including state-machine results obtained with mock primitives; the confirming StdCrypto rerun remains owed. This update follows the operator-blessed ROADMAP_of_record_20260905.md. NEXT_ACTIONS.md governs executable queue authority; this document promotes no lane.

## Order of work

1. Finish the debug-log lane's records. Its implementation is merged and final-build acceptance is satisfied; the records PR and post-merge closeout gates remain pending. Audit publication and the source documentation deferral are explicitly tracked.
2. Ratchet work and the relay capability split are the next parallel subjects in the roadmap of record, with a ceiling of two implementation seats. Ratchet work begins with experiments A–D under StdCrypto, then a two-party interleaving simulator demonstrated red on the baseline and green after approved repairs. The relay split separates the vault-held read secret from the derived deposit address; pull/ack present the preimage. The split precedes invitations involving outside users.
3. Write the lean background design from actual code: rungs, transitions, persistence, retries, idempotency, close/lock/crash survival, reset/recovery and the relay lease law. The operator approves that design; it must establish the handshake-poll class and one fetch per mailbox per beat.
4. Deliver messaging on the fixed ratchet and split relay. The receive loop joins the shared dispatcher, the interleaving simulator and remote round-trip gate pass, and the approved claims document is linked.
5. Once messaging works between the operator's two machines, undertake vault hardening (key rather than retained passphrase, zeroization, no Debug on secrets, then versioned KDF changes), handshake hardening (ephemeral ML-KEM, required suite, engine-side label check and decoder cap), environment seams and the label split before onboarding. Wire changes precede outside users.
6. Then onboarding, contact management, diagnostics polish and metadata work; shaped-push rungs follow an operator-captured real wire baseline.

The operator has approved the exact **NA-0780 — Invitation reliability** successor block covering own-invitation rejection, simultaneous invitations, refused-frame disposal, necessary instrumentation and stuck-contact recovery. Its placement immediately after NA-0779 closeout is approved in RBANK_NA0780_invitation_reliability_approved_20260909.md. The original roadmap bank is preserved; this approval changes the successor placement prospectively. READY is not advanced and no implementation or protocol repair design is authorized.

## Evidence and review

The six audit files and reproduction instructions are placed as reviewed publication drafts under docs/audits/2026-09-03. Publication is pending this records PR landing. Reports are historical; full-source findings and mock outcomes must not be presented as current acceptance. The claims document is conditional on the Director supplying an approved draft; no new claims are inferred here.

The operator's review plan is public source and reproducible evidence for critique, replacing the earlier funded outside-review plan. This is a review invitation, not evidence of completed independent cryptographic review. Simulator negative controls and fresh cold reads for changes to keys, sessions or redaction remain required. SECURITY.md retains the private reporting path.

Operator work outside this queue retains its own authority and gates. The recorded relay lease measurement is from a service environment file, not a new live-process observation. Retirement of old infrastructure remains gated by the recorded trial and date conditions.

## Invariants

Keep fail-closed behavior, claim boundaries, required checks and the sole READY item. Governance supports executable behavior, discriminating tests, vectors and measured acceptance; publication or a merge alone does not establish those outcomes.
