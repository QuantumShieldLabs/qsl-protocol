//! Suite-2 (QSP v5.0) implementation surface.

pub mod binding;
pub mod establish;
pub mod parse;
pub mod ratchet;
pub mod scka;
pub mod state;
pub mod types;

use crate::crypto::traits::{Aead, Hash, Kmac};
use crate::RefimplError;

pub fn decode_suite2_ratchet_message_canon(
    buf: &[u8],
) -> Result<parse::Suite2ParsedRatchetMsg, RefimplError> {
    parse::decode_suite2_ratchet_message(buf).map_err(RefimplError::from)
}

pub fn decode_suite2_wire_canon(
    buf: &[u8],
) -> Result<(u16, u16, u8, parse::Suite2ParsedRatchetMsg), RefimplError> {
    parse::decode_suite2_wire(buf).map_err(RefimplError::from)
}

pub fn send_wire_canon(
    hash: &dyn Hash,
    kmac: &dyn Kmac,
    aead: &dyn Aead,
    st: ratchet::Suite2SendState,
    flags: u16,
    plaintext: &[u8],
) -> Result<ratchet::SendWireOutcome, RefimplError> {
    ratchet::send_wire(hash, kmac, aead, st, flags, plaintext).map_err(RefimplError::from)
}

pub fn recv_wire_canon(
    hash: &dyn Hash,
    kmac: &dyn Kmac,
    aead: &dyn Aead,
    st: ratchet::Suite2RecvWireState,
    wire: &[u8],
    pq_epoch_ss: Option<&[u8]>,
    peer_adv_id: Option<u32>,
) -> Result<ratchet::RecvWireOutcome, RefimplError> {
    ratchet::recv_wire(hash, kmac, aead, st, wire, pq_epoch_ss, peer_adv_id)
        .map_err(RefimplError::from)
}
