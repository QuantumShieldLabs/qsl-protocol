//! Suite-2 session snapshot/restore (durability test support).

use crate::suite2::ratchet::{MkSkippedEntry, Suite2RecvWireState, Suite2SendState};
use std::collections::BTreeSet;

const MAX_TARGETS_RESTORE: usize = 10_000; // DOC-SCL-001 rsf.inbox_max_items cap (deployment profile default/upper bound).
const MAX_MKSKIPPED_RESTORE: usize = 1000; // Align with suite2/ratchet.rs MAX_MKSKIPPED.

#[derive(Debug)]
pub enum Suite2StateError {
    Invalid(&'static str),
}

impl std::fmt::Display for Suite2StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suite2StateError::Invalid(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for Suite2StateError {}

#[derive(Clone)]
pub struct Suite2SessionState {
    pub send: Suite2SendState,
    pub recv: Suite2RecvWireState,
}

impl Suite2SessionState {
    pub fn snapshot_bytes(&self) -> Vec<u8> {
        fn push_u8(out: &mut Vec<u8>, v: u8) { out.push(v); }
        fn push_u16(out: &mut Vec<u8>, v: u16) { out.extend_from_slice(&v.to_be_bytes()); }
        fn push_u32(out: &mut Vec<u8>, v: u32) { out.extend_from_slice(&v.to_be_bytes()); }
        fn push_arr16(out: &mut Vec<u8>, a: &[u8; 16]) { out.extend_from_slice(a); }
        fn push_arr32(out: &mut Vec<u8>, a: &[u8; 32]) { out.extend_from_slice(a); }
        fn push_set(out: &mut Vec<u8>, s: &BTreeSet<u32>) {
            push_u32(out, s.len() as u32);
            for v in s.iter() {
                push_u32(out, *v);
            }
        }

        let mut out = Vec::new();
        out.extend_from_slice(b"QS2S");
        push_u8(&mut out, 1); // version

        // Send state
        push_arr16(&mut out, &self.send.session_id);
        push_u16(&mut out, self.send.protocol_version);
        push_u16(&mut out, self.send.suite_id);
        push_arr32(&mut out, &self.send.dh_pub);
        push_arr32(&mut out, &self.send.hk_s);
        push_arr32(&mut out, &self.send.ck_ec);
        push_arr32(&mut out, &self.send.ck_pq);
        push_u32(&mut out, self.send.ns);
        push_u32(&mut out, self.send.pn);

        // Recv state
        push_arr16(&mut out, &self.recv.session_id);
        push_u16(&mut out, self.recv.protocol_version);
        push_u16(&mut out, self.recv.suite_id);
        push_arr32(&mut out, &self.recv.dh_pub);
        push_arr32(&mut out, &self.recv.hk_r);
        push_arr32(&mut out, &self.recv.rk);
        push_arr32(&mut out, &self.recv.ck_ec);
        push_arr32(&mut out, &self.recv.ck_pq_send);
        push_arr32(&mut out, &self.recv.ck_pq_recv);
        push_u32(&mut out, self.recv.nr);
        push_u8(&mut out, if self.recv.role_is_a { 1 } else { 0 });
        push_u32(&mut out, self.recv.peer_max_adv_id_seen);
        push_set(&mut out, &self.recv.known_targets);
        push_set(&mut out, &self.recv.consumed_targets);
        push_set(&mut out, &self.recv.tombstoned_targets);

        push_u32(&mut out, self.recv.mkskipped.len() as u32);
        for entry in self.recv.mkskipped.iter() {
            push_arr32(&mut out, &entry.dh_pub);
            push_u32(&mut out, entry.n);
            push_arr32(&mut out, &entry.mk);
        }

        out
    }

    pub fn restore_bytes(bytes: &[u8]) -> Result<Self, Suite2StateError> {
        let invalid = || Suite2StateError::Invalid("bad suite2 snapshot");

        struct Cur<'a> { b: &'a [u8], i: usize }
        impl<'a> Cur<'a> {
            fn take(&mut self, n: usize) -> Result<&'a [u8], Suite2StateError> {
                if self.i + n > self.b.len() { return Err(Suite2StateError::Invalid("bad suite2 snapshot")); }
                let s = &self.b[self.i..self.i + n];
                self.i += n;
                Ok(s)
            }
            fn u8(&mut self) -> Result<u8, Suite2StateError> { Ok(self.take(1)?[0]) }
            fn u16(&mut self) -> Result<u16, Suite2StateError> {
                let s = self.take(2)?;
                Ok(u16::from_be_bytes([s[0], s[1]]))
            }
            fn u32(&mut self) -> Result<u32, Suite2StateError> {
                let s = self.take(4)?;
                Ok(u32::from_be_bytes([s[0], s[1], s[2], s[3]]))
            }
            fn arr16(&mut self) -> Result<[u8; 16], Suite2StateError> {
                let s = self.take(16)?;
                let mut a = [0u8; 16];
                a.copy_from_slice(s);
                Ok(a)
            }
            fn arr32(&mut self) -> Result<[u8; 32], Suite2StateError> {
                let s = self.take(32)?;
                let mut a = [0u8; 32];
                a.copy_from_slice(s);
                Ok(a)
            }
        }

        let mut c = Cur { b: bytes, i: 0 };
        let remaining = |c: &Cur| -> usize { c.b.len().saturating_sub(c.i) };
        let magic = c.take(4)?;
        if magic != b"QS2S" { return Err(invalid()); }
        let ver = c.u8()?;
        if ver != 1 { return Err(invalid()); }

        let send = Suite2SendState {
            session_id: c.arr16()?,
            protocol_version: c.u16()?,
            suite_id: c.u16()?,
            dh_pub: c.arr32()?,
            hk_s: c.arr32()?,
            ck_ec: c.arr32()?,
            ck_pq: c.arr32()?,
            ns: c.u32()?,
            pn: c.u32()?,
        };

        let mut recv = Suite2RecvWireState {
            session_id: c.arr16()?,
            protocol_version: c.u16()?,
            suite_id: c.u16()?,
            dh_pub: c.arr32()?,
            hk_r: c.arr32()?,
            rk: c.arr32()?,
            ck_ec: c.arr32()?,
            ck_pq_send: c.arr32()?,
            ck_pq_recv: c.arr32()?,
            nr: c.u32()?,
            role_is_a: false,
            peer_max_adv_id_seen: 0,
            known_targets: BTreeSet::new(),
            consumed_targets: BTreeSet::new(),
            tombstoned_targets: BTreeSet::new(),
            mkskipped: Vec::new(),
        };

        let role_flag = c.u8()?;
        recv.role_is_a = match role_flag {
            0 => false,
            1 => true,
            _ => return Err(invalid()),
        };
        recv.peer_max_adv_id_seen = c.u32()?;

        let known_len = c.u32()? as usize;
        if known_len > MAX_TARGETS_RESTORE {
            return Err(invalid());
        }
        let known_bytes = known_len.checked_mul(4).ok_or_else(invalid)?;
        if known_bytes > remaining(&c) {
            return Err(invalid());
        }
        for _ in 0..known_len {
            recv.known_targets.insert(c.u32()?);
        }
        let consumed_len = c.u32()? as usize;
        if consumed_len > MAX_TARGETS_RESTORE {
            return Err(invalid());
        }
        let consumed_bytes = consumed_len.checked_mul(4).ok_or_else(invalid)?;
        if consumed_bytes > remaining(&c) {
            return Err(invalid());
        }
        for _ in 0..consumed_len {
            recv.consumed_targets.insert(c.u32()?);
        }
        let tomb_len = c.u32()? as usize;
        if tomb_len > MAX_TARGETS_RESTORE {
            return Err(invalid());
        }
        let tomb_bytes = tomb_len.checked_mul(4).ok_or_else(invalid)?;
        if tomb_bytes > remaining(&c) {
            return Err(invalid());
        }
        for _ in 0..tomb_len {
            recv.tombstoned_targets.insert(c.u32()?);
        }

        let mk_len = c.u32()? as usize;
        if mk_len > MAX_MKSKIPPED_RESTORE {
            return Err(invalid());
        }
        let mk_bytes = mk_len.checked_mul(68).ok_or_else(invalid)?;
        if mk_bytes > remaining(&c) {
            return Err(invalid());
        }
        for _ in 0..mk_len {
            let dh_pub = c.arr32()?;
            let n = c.u32()?;
            let mk = c.arr32()?;
            recv.mkskipped.push(MkSkippedEntry { dh_pub, n, mk });
        }

        if c.i != bytes.len() {
            return Err(invalid());
        }

        Ok(Suite2SessionState { send, recv })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn sample_state() -> Suite2SessionState {
        let send = Suite2SendState {
            session_id: [0x11; 16],
            protocol_version: 5,
            suite_id: 2,
            dh_pub: [0x22; 32],
            hk_s: [0x33; 32],
            ck_ec: [0x44; 32],
            ck_pq: [0x55; 32],
            ns: 0,
            pn: 0,
        };
        let recv = Suite2RecvWireState {
            session_id: [0x11; 16],
            protocol_version: 5,
            suite_id: 2,
            dh_pub: [0x22; 32],
            hk_r: [0x33; 32],
            rk: [0x44; 32],
            ck_ec: [0x55; 32],
            ck_pq_send: [0x66; 32],
            ck_pq_recv: [0x77; 32],
            nr: 0,
            role_is_a: true,
            peer_max_adv_id_seen: 0,
            known_targets: BTreeSet::new(),
            consumed_targets: BTreeSet::new(),
            tombstoned_targets: BTreeSet::new(),
            mkskipped: Vec::new(),
        };
        Suite2SessionState { send, recv }
    }

    fn length_offsets(bytes: &[u8]) -> (usize, usize, usize, usize) {
        struct Cur<'a> { b: &'a [u8], i: usize }
        impl<'a> Cur<'a> {
            fn take(&mut self, n: usize) -> Result<&'a [u8], Suite2StateError> {
                if self.i + n > self.b.len() { return Err(Suite2StateError::Invalid("bad suite2 snapshot")); }
                let s = &self.b[self.i..self.i + n];
                self.i += n;
                Ok(s)
            }
            fn u8(&mut self) -> Result<u8, Suite2StateError> { Ok(self.take(1)?[0]) }
            fn u16(&mut self) -> Result<u16, Suite2StateError> {
                let s = self.take(2)?;
                Ok(u16::from_be_bytes([s[0], s[1]]))
            }
            fn u32(&mut self) -> Result<u32, Suite2StateError> {
                let s = self.take(4)?;
                Ok(u32::from_be_bytes([s[0], s[1], s[2], s[3]]))
            }
            fn arr16(&mut self) -> Result<[u8; 16], Suite2StateError> {
                let s = self.take(16)?;
                let mut a = [0u8; 16];
                a.copy_from_slice(s);
                Ok(a)
            }
            fn arr32(&mut self) -> Result<[u8; 32], Suite2StateError> {
                let s = self.take(32)?;
                let mut a = [0u8; 32];
                a.copy_from_slice(s);
                Ok(a)
            }
        }

        let mut c = Cur { b: bytes, i: 0 };
        let _ = c.take(4).expect("magic");
        let _ = c.u8().expect("version");
        let _ = c.arr16().expect("session_id");
        let _ = c.u16().expect("protocol_version");
        let _ = c.u16().expect("suite_id");
        let _ = c.arr32().expect("dh_pub");
        let _ = c.arr32().expect("hk_s");
        let _ = c.arr32().expect("ck_ec");
        let _ = c.arr32().expect("ck_pq");
        let _ = c.u32().expect("ns");
        let _ = c.u32().expect("pn");

        let _ = c.arr16().expect("session_id");
        let _ = c.u16().expect("protocol_version");
        let _ = c.u16().expect("suite_id");
        let _ = c.arr32().expect("dh_pub");
        let _ = c.arr32().expect("hk_r");
        let _ = c.arr32().expect("rk");
        let _ = c.arr32().expect("ck_ec");
        let _ = c.arr32().expect("ck_pq_send");
        let _ = c.arr32().expect("ck_pq_recv");
        let _ = c.u32().expect("nr");
        let _ = c.u8().expect("role");
        let _ = c.u32().expect("peer_max_adv_id_seen");

        let known = c.i;
        let _ = c.u32().expect("known_len");
        let consumed = c.i;
        let _ = c.u32().expect("consumed_len");
        let tomb = c.i;
        let _ = c.u32().expect("tomb_len");
        let mk = c.i;
        (known, consumed, tomb, mk)
    }

    fn read_u32_be(bytes: &[u8], off: usize) -> u32 {
        u32::from_be_bytes([bytes[off], bytes[off + 1], bytes[off + 2], bytes[off + 3]])
    }

    #[test]
    fn restore_bytes_rejects_oversize_lengths_deterministically() {
        let st0 = sample_state();
        let st0_pre = st0.snapshot_bytes();

        let mut bytes = sample_state().snapshot_bytes();
        let (known_off, _consumed_off, _tomb_off, mk_off) = length_offsets(&bytes);
        let oversize = (MAX_TARGETS_RESTORE as u32).saturating_add(1).to_be_bytes();
        bytes[known_off..known_off + 4].copy_from_slice(&oversize);
        let oversize_mk = (MAX_MKSKIPPED_RESTORE as u32).saturating_add(1).to_be_bytes();
        bytes[mk_off..mk_off + 4].copy_from_slice(&oversize_mk);
        assert_eq!(read_u32_be(&bytes, known_off), MAX_TARGETS_RESTORE as u32 + 1);
        assert_eq!(read_u32_be(&bytes, mk_off), MAX_MKSKIPPED_RESTORE as u32 + 1);

        let err1 = Suite2SessionState::restore_bytes(&bytes).err().expect("expected err");
        let err2 = Suite2SessionState::restore_bytes(&bytes).err().expect("expected err");
        assert_eq!(format!("{:?}", err1), format!("{:?}", err2));

        let st0_post = st0.snapshot_bytes();
        assert_eq!(st0_pre, st0_post);
    }

    #[test]
    fn restore_bytes_rejects_truncated_buffers_deterministically() {
        let st0 = sample_state();
        let st0_pre = st0.snapshot_bytes();

        let mut bytes = sample_state().snapshot_bytes();
        bytes.truncate(bytes.len().saturating_sub(1));

        let err1 = Suite2SessionState::restore_bytes(&bytes).err().expect("expected err");
        let err2 = Suite2SessionState::restore_bytes(&bytes).err().expect("expected err");
        assert_eq!(format!("{:?}", err1), format!("{:?}", err2));

        let st0_post = st0.snapshot_bytes();
        assert_eq!(st0_pre, st0_post);
    }
}
