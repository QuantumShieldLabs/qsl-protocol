//! QuantumShield reference implementation skeleton.
//!
//! Governing specs:
//! - QSP v4.3.1 (protocol_version 0x0403, suite_id 0x0001)
//! - QSE v1.8.1 (env_version 0x0100)
//!
//! This crate focuses on correctness, canonical parsing, bounded work, and test-vector stability.

pub mod codec;
pub mod crypto;
pub mod kt;
pub mod qse;
pub mod qsp;
pub mod suite2;

pub use qsp::{HandshakeInit, HandshakeResp, ProtocolMessage, SessionState, SessionRole};
pub use qse::{Envelope, EnvelopeProfile};
