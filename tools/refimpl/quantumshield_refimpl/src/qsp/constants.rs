//! Constants from QSP v4.3.1.

pub const QSP_PROTOCOL_VERSION: u16 = 0x0403;
pub const QSP_SUITE_ID: u16 = 0x0001;

// Fixed sizes (QSP §1.2)
pub const SZ_SESSION_ID: usize = 16;
pub const SZ_X25519_PUB: usize = 32;
pub const SZ_ED25519_PUB: usize = 32;
pub const SZ_ED25519_SIG: usize = 64;

pub const SZ_MLKEM768_PUB: usize = 1184;
pub const SZ_MLKEM768_CT: usize = 1088;

pub const SZ_MLDSA65_PUB: usize = 1952;
pub const SZ_MLDSA65_SIG: usize = 3309;

pub const SZ_NONCE: usize = 12;

// Messaging flags (QSP §6.3)
pub const FLAG_PQ_ADV: u16 = 0x0001;
pub const FLAG_PQ_CTXT: u16 = 0x0002;
pub const FLAG_BOUNDARY: u16 = 0x0004;

// Bounds (QSP §8.3)
pub const MAX_SKIP: u32 = 1000;
pub const MAX_MKSKIPPED: usize = 2000;
pub const MAX_HEADER_ATTEMPTS: usize = 100;
pub const MAX_HKSKIPPED: usize = 4;
pub const MAX_MKSKIPPED_SCAN: usize = 50;
