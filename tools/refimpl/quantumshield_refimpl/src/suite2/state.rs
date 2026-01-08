//! Suite-2 session snapshot/restore (durability test support).

use crate::suite2::ratchet::{MkSkippedEntry, Suite2RecvWireState, Suite2SendState};
use std::collections::BTreeSet;

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
        for _ in 0..known_len {
            recv.known_targets.insert(c.u32()?);
        }
        let consumed_len = c.u32()? as usize;
        for _ in 0..consumed_len {
            recv.consumed_targets.insert(c.u32()?);
        }
        let tomb_len = c.u32()? as usize;
        for _ in 0..tomb_len {
            recv.tombstoned_targets.insert(c.u32()?);
        }

        let mk_len = c.u32()? as usize;
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
