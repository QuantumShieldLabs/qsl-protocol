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
pub mod refimpl_error;
pub mod suite2;

pub use qse::{Envelope, EnvelopeProfile};
pub use qsp::{HandshakeInit, HandshakeResp, ProtocolMessage, SessionRole, SessionState};
pub use refimpl_error::RefimplError;
