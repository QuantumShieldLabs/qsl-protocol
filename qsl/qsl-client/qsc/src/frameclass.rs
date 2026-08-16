//! NA-0741 (D-1376) LANE 1 — **THE RECEIVE-SIDE FRAME CLASSIFIER.**
//!
//! ⚠⚠ **THIS IS A MATCH, NOT AN IDENTIFICATION.** `FrameClass` names *the discriminator these
//! leading bytes MATCH*, never *what this frame is*. Nothing here — and no marker fed from here —
//! may claim the classifier identifies a frame. The distinction is not pedantry: the tree's own
//! junk fixture `vec![1,2,3,4,5,6,7]` (`tests/timeline_store.rs`) matched the InviteResp
//! discriminator `01 02` exactly while being nobody's invite reply, which is why that fixture was
//! re-aimed by this lane and why the rule is fixed here at the definition rather than left to the
//! call site.
//!
//! ⚠ **EVERY CONSTANT IS REACHED BY REFERENCE. NO MAGIC LITERAL LIVES IN THIS MODULE**, so a
//! discriminator that ever moves moves here too, and a copied byte pair cannot silently drift out
//! of agreement with the encoder that writes it.
//!
//! ⚠ **`Unknown` IS DELIBERATELY NOT KNOWN-FOREIGN** (the ruled option, N-PRIME). An Unknown-class
//! frame routes to unpack exactly as it does today, which is what preserves the six committed
//! assertions over Unknown-class junk fixtures and the NA-0187 contact-request onboarding surface.
//! An attacker chooses their own leading bytes, so skipping Unknown would have bought no
//! adversarial ground; what it would have cost is measured, and the trade is recorded in D-1376.
//!
//! ⚠ **TWO BYTES IS THE FLOOR.** All three envelope classes share `bytes[0] == 0x01`, so a
//! one-byte prefix cannot separate a user's message from an invite frame.

/// The frame classes the receive loop can tell apart from a frame's leading bytes alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FrameClass {
    Handshake,
    Message,
    InviteInit,
    InviteResp,
    Unknown,
}

impl FrameClass {
    /// The marker's `class=` value. A CLOSED vocabulary: these five strings are the whole
    /// range, so a marker consumer can match exhaustively.
    pub(crate) fn name(self) -> &'static str {
        match self {
            FrameClass::Handshake => "handshake",
            FrameClass::Message => "message",
            FrameClass::InviteInit => "invite_init",
            FrameClass::InviteResp => "invite_resp",
            FrameClass::Unknown => "unknown",
        }
    }

    /// ⚠ **THE RULED SKIP PREDICATE.** True for exactly the three classes this client can
    /// recognise as *another protocol's frame* — a handshake or invite frame that some other
    /// consumer on this device is entitled to collect. `Message` is ours and `Unknown` is
    /// deliberately excluded (see the module note).
    pub(crate) fn is_known_foreign(self) -> bool {
        matches!(
            self,
            FrameClass::Handshake | FrameClass::InviteInit | FrameClass::InviteResp
        )
    }
}

/// Pure, total, allocation-free, I/O-free. Every input maps to exactly one class, and any input
/// shorter than two bytes is `Unknown`.
pub(crate) fn classify(bytes: &[u8]) -> FrameClass {
    // ⚠ Rule order is stated for clarity and is NOT load-bearing: `51 48` ("QH") cannot collide
    // with any `01 xx` pair, so no reordering of these arms changes any answer.
    if bytes.len() >= 4 && bytes[0..4] == *crate::handshake::HS_MAGIC {
        return FrameClass::Handshake;
    }
    if bytes.len() >= 2 {
        // `QSE_ENV_VERSION_V1` is a u16 and the wire is big-endian (`Writer::write_u16` is
        // `to_be_bytes`), so `0x0100` is the byte pair `01 00`. Derived, never transcribed.
        if bytes[0..2] == crate::QSE_ENV_VERSION_V1.to_be_bytes() {
            return FrameClass::Message;
        }
        let pair = (bytes[0], bytes[1]);
        if pair == (crate::invite::ENVELOPE_VER, crate::invite::ENVELOPE_TYPE_INIT) {
            return FrameClass::InviteInit;
        }
        if pair == (crate::invite::ENVELOPE_VER, crate::invite::ENVELOPE_TYPE_RESP) {
            return FrameClass::InviteResp;
        }
    }
    FrameClass::Unknown
}
