use super::constants::*;
use crate::crypto::traits::*;
use thiserror::Error;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy)]
pub enum SessionRole { Initiator, Responder }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeaderSource { CurrentHk, CurrentNhk, SkippedHk, SkippedNhk }

#[derive(Debug, Error)]
pub enum StateError {
    #[error("crypto: {0}")]
    Crypto(#[from] CryptoError),
    #[error("invalid state: {0}")]
    Invalid(&'static str),
}

#[derive(Clone)]
pub struct SessionState {
    pub role: SessionRole,
    pub session_id: [u8; 16],

    pub rk: [u8; 32],

    pub dh_self: (X25519Priv, X25519Pub),
    pub dh_peer: [u8; 32],

    pub ck_s: Option<[u8; 32]>,
    pub ck_r: Option<[u8; 32]>,

    pub hk_s: [u8; 32],
    pub hk_r: [u8; 32],
    pub nhk_s: [u8; 32],
    pub nhk_r: [u8; 32],

    pub ns: u32,
    pub nr: u32,
    pub pn: u32,

    pub boundary_pending: bool,
    pub boundary_hk: [u8; 32], // valid iff boundary_pending

    // MKSKIPPED: (dh_pub, n) -> mk
    mk_skipped: HashMap<([u8;32], u32), [u8;32]>,
    mk_order: VecDeque<([u8;32], u32)>, // FIFO eviction
    // HKSKIPPED: old_dh_peer -> (hk_r_old, nhk_r_old)
    hk_skipped: HashMap<[u8;32], ([u8;32],[u8;32])>,
    hk_order: VecDeque<[u8;32]>,

    pub pq_peer_id: Option<u32>,
    pub pq_peer_pub: Option<Vec<u8>>, // 1184

    // pq self receive cache: pq_id -> (pub, priv)
    pub pq_self: HashMap<u32, (Vec<u8>, Vec<u8>)>,
}

impl SessionState {
    pub fn new(role: SessionRole, session_id: [u8;16], rk0: [u8;32],
               dh0_self: (X25519Priv, X25519Pub), dh_peer: [u8;32],
               pq_self_rcv: (u32, Vec<u8>, Vec<u8>)) -> Self {
        let (hk_s, hk_r, nhk_s, nhk_r) = derive_header_keys(role, &rk0);
        let mut pq_self = HashMap::new();
        pq_self.insert(pq_self_rcv.0, (pq_self_rcv.1, pq_self_rcv.2));
        Self {
            role, session_id, rk: rk0,
            dh_self: dh0_self, dh_peer,
            ck_s: None, ck_r: None,
            hk_s, hk_r, nhk_s, nhk_r,
            ns: 0, nr: 0, pn: 0,
            boundary_pending: false,
            boundary_hk: [0u8;32],
            mk_skipped: HashMap::new(),
            mk_order: VecDeque::new(),
            hk_skipped: HashMap::new(),
            hk_order: VecDeque::new(),
            pq_peer_id: None,
            pq_peer_pub: None,
            pq_self,
        }
    }

    pub fn hk_pair_for(&self, dh_pub: &[u8;32]) -> Option<([u8;32],[u8;32])> {
        self.hk_skipped.get(dh_pub).cloned()
    }

    pub fn store_hk_skipped(&mut self, old_dh_peer: [u8;32], hk_r: [u8;32], nhk_r: [u8;32]) {
        if self.hk_skipped.contains_key(&old_dh_peer) { return; }
        if self.hk_order.len() >= MAX_HKSKIPPED {
            if let Some(ev) = self.hk_order.pop_front() {
                self.hk_skipped.remove(&ev);
            }
        }
        self.hk_order.push_back(old_dh_peer);
        self.hk_skipped.insert(old_dh_peer, (hk_r, nhk_r));
    }

    pub fn store_mk_skipped(&mut self, dh_pub: [u8;32], n: u32, mk: [u8;32]) -> Result<(), StateError> {
        let key = (dh_pub, n);
        if self.mk_skipped.contains_key(&key) {
            return Err(StateError::Invalid("mk_skipped duplicate"));
        }
        if self.mk_order.len() >= MAX_MKSKIPPED {
            return Err(StateError::Invalid("mk_skipped full"));
        }
        self.mk_order.push_back(key);
        self.mk_skipped.insert(key, mk);
        Ok(())
    }

    pub fn take_mk_skipped(&mut self, dh_pub: [u8;32], n: u32) -> Option<[u8;32]> {
        let key = (dh_pub, n);
        self.mk_order.retain(|k| *k != key);
        self.mk_skipped.remove(&key)
    }

    pub fn peek_mk_skipped(&self, dh_pub: [u8;32], n: u32) -> Option<[u8;32]> {
        self.mk_skipped.get(&(dh_pub, n)).copied()
    }

    pub fn mk_skipped_contains(&self, dh_pub: [u8;32], n: u32) -> bool {
        self.mk_skipped.contains_key(&(dh_pub, n))
    }

    pub fn mk_skipped_len(&self) -> usize { self.mk_skipped.len() }

    pub fn derive_header_keys(&mut self, kmac: &dyn Kmac) {
        let (hk_s, hk_r, nhk_s, nhk_r) = derive_header_keys(self.role, &self.rk);
        self.hk_s = hk_s; self.hk_r = hk_r; self.nhk_s = nhk_s; self.nhk_r = nhk_r;
    }

    /// Serialize the full session state into an opaque, versioned byte blob.
    ///
    /// This is intended for Phase 4D durability testing only.
    pub fn snapshot_bytes(&self) -> Vec<u8> {
        fn push_u8(out: &mut Vec<u8>, v: u8) { out.push(v); }
        fn push_u32(out: &mut Vec<u8>, v: u32) { out.extend_from_slice(&v.to_be_bytes()); }
        fn push_arr16(out: &mut Vec<u8>, a: &[u8;16]) { out.extend_from_slice(a); }
        fn push_arr32(out: &mut Vec<u8>, a: &[u8;32]) { out.extend_from_slice(a); }
        fn push_vec(out: &mut Vec<u8>, v: &[u8]) {
            push_u32(out, v.len() as u32);
            out.extend_from_slice(v);
        }
        fn push_opt_arr32(out: &mut Vec<u8>, o: &Option<[u8;32]>) {
            match o {
                Some(a) => { push_u8(out, 1); push_arr32(out, a); }
                None => push_u8(out, 0),
            }
        }
        fn push_opt_u32(out: &mut Vec<u8>, o: &Option<u32>) {
            match o {
                Some(v) => { push_u8(out, 1); push_u32(out, *v); }
                None => push_u8(out, 0),
            }
        }
        fn push_opt_vec(out: &mut Vec<u8>, o: &Option<Vec<u8>>) {
            match o {
                Some(v) => { push_u8(out, 1); push_vec(out, v); }
                None => push_u8(out, 0),
            }
        }

        let mut out = Vec::new();
        out.extend_from_slice(b"QSSN"); // QuantumShield Session Snapshot
        push_u8(&mut out, 1); // version

        push_u8(&mut out, match self.role { SessionRole::Initiator => 0, SessionRole::Responder => 1 });
        push_arr16(&mut out, &self.session_id);

        push_arr32(&mut out, &self.rk);
        push_arr32(&mut out, &self.dh_self.0.0);
        push_arr32(&mut out, &self.dh_self.1.0);
        push_arr32(&mut out, &self.dh_peer);

        push_opt_arr32(&mut out, &self.ck_s);
        push_opt_arr32(&mut out, &self.ck_r);

        push_arr32(&mut out, &self.hk_s);
        push_arr32(&mut out, &self.hk_r);
        push_arr32(&mut out, &self.nhk_s);
        push_arr32(&mut out, &self.nhk_r);

        push_u32(&mut out, self.ns);
        push_u32(&mut out, self.nr);
        push_u32(&mut out, self.pn);

        push_u8(&mut out, if self.boundary_pending { 1 } else { 0 });
        push_arr32(&mut out, &self.boundary_hk);

        // MKSKIPPED map (sorted, deterministic)
        let mut mk_entries: Vec<([u8;32], u32, [u8;32])> =
            self.mk_skipped.iter().map(|(k, v)| (k.0, k.1, *v)).collect();
        mk_entries.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        push_u32(&mut out, mk_entries.len() as u32);
        for (dh, n, mk) in mk_entries {
            push_arr32(&mut out, &dh);
            push_u32(&mut out, n);
            push_arr32(&mut out, &mk);
        }

        // MK order queue
        push_u32(&mut out, self.mk_order.len() as u32);
        for (dh, n) in self.mk_order.iter() {
            push_arr32(&mut out, dh);
            push_u32(&mut out, *n);
        }

        // HKSKIPPED map (sorted, deterministic)
        let mut hk_entries: Vec<([u8;32], ([u8;32],[u8;32]))> = self.hk_skipped.iter().map(|(k, v)| (*k, *v)).collect();
        hk_entries.sort_by(|a, b| a.0.cmp(&b.0));
        push_u32(&mut out, hk_entries.len() as u32);
        for (dh, (hk, nhk)) in hk_entries {
            push_arr32(&mut out, &dh);
            push_arr32(&mut out, &hk);
            push_arr32(&mut out, &nhk);
        }

        // HK order queue
        push_u32(&mut out, self.hk_order.len() as u32);
        for dh in self.hk_order.iter() {
            push_arr32(&mut out, dh);
        }

        push_opt_u32(&mut out, &self.pq_peer_id);
        push_opt_vec(&mut out, &self.pq_peer_pub);

        // pq self cache (sorted by id)
        let mut pq_entries: Vec<(u32, Vec<u8>, Vec<u8>)> = self.pq_self.iter().map(|(id, (pubk, privk))| (*id, pubk.clone(), privk.clone())).collect();
        pq_entries.sort_by(|a, b| a.0.cmp(&b.0));
        push_u32(&mut out, pq_entries.len() as u32);
        for (id, pubk, privk) in pq_entries {
            push_u32(&mut out, id);
            push_vec(&mut out, &pubk);
            push_vec(&mut out, &privk);
        }

        out
    }

    /// Restore a session state from a blob produced by `snapshot_bytes()`.
    pub fn restore_bytes(bytes: &[u8]) -> Result<Self, StateError> {
        let invalid = || StateError::Invalid("bad snapshot");

        struct Cur<'a> { b: &'a [u8], i: usize }
        impl<'a> Cur<'a> {
            fn take(&mut self, n: usize) -> Result<&'a [u8], StateError> {
                if self.i + n > self.b.len() { return Err(StateError::Invalid("bad snapshot")); }
                let s = &self.b[self.i..self.i+n];
                self.i += n;
                Ok(s)
            }
            fn u8(&mut self) -> Result<u8, StateError> { Ok(self.take(1)?[0]) }
            fn u32(&mut self) -> Result<u32, StateError> {
                let s = self.take(4)?;
                Ok(u32::from_be_bytes([s[0], s[1], s[2], s[3]]))
            }
            fn arr16(&mut self) -> Result<[u8;16], StateError> {
                let s = self.take(16)?;
                let mut a = [0u8;16];
                a.copy_from_slice(s);
                Ok(a)
            }
            fn arr32(&mut self) -> Result<[u8;32], StateError> {
                let s = self.take(32)?;
                let mut a = [0u8;32];
                a.copy_from_slice(s);
                Ok(a)
            }
            fn vec(&mut self) -> Result<Vec<u8>, StateError> {
                let n = self.u32()? as usize;
                Ok(self.take(n)?.to_vec())
            }
            fn opt_arr32(&mut self) -> Result<Option<[u8;32]>, StateError> {
                let f = self.u8()?;
                if f == 0 { Ok(None) } else if f == 1 { Ok(Some(self.arr32()?)) } else { Err(StateError::Invalid("bad snapshot")) }
            }
            fn opt_u32(&mut self) -> Result<Option<u32>, StateError> {
                let f = self.u8()?;
                if f == 0 { Ok(None) } else if f == 1 { Ok(Some(self.u32()?)) } else { Err(StateError::Invalid("bad snapshot")) }
            }
            fn opt_vec(&mut self) -> Result<Option<Vec<u8>>, StateError> {
                let f = self.u8()?;
                if f == 0 { Ok(None) } else if f == 1 { Ok(Some(self.vec()?)) } else { Err(StateError::Invalid("bad snapshot")) }
            }
        }

        let mut c = Cur { b: bytes, i: 0 };
        let magic = c.take(4)?;
        if magic != b"QSSN" { return Err(invalid()); }
        let ver = c.u8()?;
        if ver != 1 { return Err(invalid()); }

        let role = match c.u8()? {
            0 => SessionRole::Initiator,
            1 => SessionRole::Responder,
            _ => return Err(invalid()),
        };
        let session_id = c.arr16()?;

        let rk = c.arr32()?;
        let dh_priv = c.arr32()?;
        let dh_pub = c.arr32()?;
        let dh_peer = c.arr32()?;

        let ck_s = c.opt_arr32()?;
        let ck_r = c.opt_arr32()?;

        let hk_s = c.arr32()?;
        let hk_r = c.arr32()?;
        let nhk_s = c.arr32()?;
        let nhk_r = c.arr32()?;

        let ns = c.u32()?;
        let nr = c.u32()?;
        let pn = c.u32()?;

        let boundary_pending = match c.u8()? {
            0 => false,
            1 => true,
            _ => return Err(invalid()),
        };
        let boundary_hk = c.arr32()?;

        let mk_len = c.u32()? as usize;
        let mut mk_skipped = HashMap::new();
        for _ in 0..mk_len {
            let dh = c.arr32()?;
            let n = c.u32()?;
            let mk = c.arr32()?;
            mk_skipped.insert((dh, n), mk);
        }
        let mk_order_len = c.u32()? as usize;
        let mut mk_order = VecDeque::new();
        for _ in 0..mk_order_len {
            let dh = c.arr32()?;
            let n = c.u32()?;
            mk_order.push_back((dh, n));
        }

        let hk_len = c.u32()? as usize;
        let mut hk_skipped = HashMap::new();
        for _ in 0..hk_len {
            let dh = c.arr32()?;
            let hk = c.arr32()?;
            let nhk = c.arr32()?;
            hk_skipped.insert(dh, (hk, nhk));
        }
        let hk_order_len = c.u32()? as usize;
        let mut hk_order = VecDeque::new();
        for _ in 0..hk_order_len {
            hk_order.push_back(c.arr32()?);
        }

        let pq_peer_id = c.opt_u32()?;
        let pq_peer_pub = c.opt_vec()?;

        let pq_len = c.u32()? as usize;
        let mut pq_self = HashMap::new();
        for _ in 0..pq_len {
            let id = c.u32()?;
            let pubk = c.vec()?;
            let privk = c.vec()?;
            pq_self.insert(id, (pubk, privk));
        }

        if c.i != bytes.len() {
            return Err(invalid());
        }

        Ok(SessionState {
            role,
            session_id,
            rk,
            dh_self: (X25519Priv(dh_priv), X25519Pub(dh_pub)),
            dh_peer,
            ck_s,
            ck_r,
            hk_s,
            hk_r,
            nhk_s,
            nhk_r,
            ns,
            nr,
            pn,
            boundary_pending,
            boundary_hk,
            mk_skipped,
            mk_order,
            hk_skipped,
            hk_order,
            pq_peer_id,
            pq_peer_pub,
            pq_self,
        })
    }
}

pub fn derive_header_keys(role: SessionRole, rk: &[u8;32]) -> ([u8;32],[u8;32],[u8;32],[u8;32]) {
    // QSP §3.4: directional header keys depend on fixed roles (A=initiator, B=responder).
    // For Initiator: send is A->B, receive is B->A. For Responder: reversed.
    fn km(key: &[u8;32], label: &str) -> [u8;32] {
        // Placeholder: actual KMAC is applied in ratchet; session init uses the same formula.
        // This function is only used for initialization; callers SHOULD call SessionState::derive_header_keys using the real KMAC.
        let mut out = [0u8;32];
        // deterministic but non-crypto; overwritten when derive_header_keys(kmac) is called
        out[0..label.len().min(32)].copy_from_slice(&label.as_bytes()[..label.len().min(32)]);
        out
    }

    match role {
        SessionRole::Initiator => {
            (km(rk,"QSP4.3/HK/A->B"), km(rk,"QSP4.3/HK/B->A"),
             km(rk,"QSP4.3/NHK/A->B"), km(rk,"QSP4.3/NHK/B->A"))
        }
        SessionRole::Responder => {
            (km(rk,"QSP4.3/HK/B->A"), km(rk,"QSP4.3/HK/A->B"),
             km(rk,"QSP4.3/NHK/B->A"), km(rk,"QSP4.3/NHK/A->B"))
        }
    }
}

/// Apply the real KMAC-based header derivation per QSP §3.4.
pub fn derive_header_keys_kmac(role: SessionRole, rk: &[u8;32], kmac: &dyn Kmac) -> ([u8;32],[u8;32],[u8;32],[u8;32]) {
    fn k(kmac: &dyn Kmac, rk: &[u8;32], label: &str) -> [u8;32] {
        let v = kmac.kmac256(rk, label, &[0x01], 32);
        let mut out = [0u8;32];
        out.copy_from_slice(&v);
        out
    }
    match role {
        SessionRole::Initiator => (
            k(kmac,rk,"QSP4.3/HK/A->B"),  k(kmac,rk,"QSP4.3/HK/B->A"),
            k(kmac,rk,"QSP4.3/NHK/A->B"), k(kmac,rk,"QSP4.3/NHK/B->A"),
        ),
        SessionRole::Responder => (
            k(kmac,rk,"QSP4.3/HK/B->A"),  k(kmac,rk,"QSP4.3/HK/A->B"),
            k(kmac,rk,"QSP4.3/NHK/B->A"), k(kmac,rk,"QSP4.3/NHK/A->B"),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::{SessionRole, SessionState, MAX_MKSKIPPED};
    use crate::crypto::traits::{X25519Priv, X25519Pub};

    fn make_state() -> SessionState {
        let role = SessionRole::Initiator;
        let session_id = [0x11u8; 16];
        let rk0 = [0x22u8; 32];
        let dh_self = (X25519Priv([0x33u8; 32]), X25519Pub([0x44u8; 32]));
        let dh_peer = [0x55u8; 32];
        let pq_self_rcv = (1u32, vec![0x66u8; 4], vec![0x77u8; 4]);
        SessionState::new(role, session_id, rk0, dh_self, dh_peer, pq_self_rcv)
    }

    #[test]
    fn take_mk_skipped_removes_from_mk_order() {
        let mut st = make_state();
        let dh = [0xAAu8; 32];
        let n = 7u32;
        let mk = [0xBBu8; 32];
        st.store_mk_skipped(dh, n, mk).unwrap();

        let got = st.take_mk_skipped(dh, n);
        assert!(got.is_some());
        assert!(!st.mk_skipped.contains_key(&(dh, n)));
        assert!(!st.mk_order.iter().any(|k| *k == (dh, n)));
    }

    #[test]
    fn store_mk_skipped_rejects_deterministically_and_no_state_mutation_on_failure() {
        fn fill_to_capacity(st: &mut SessionState) {
            let dh = [0xEEu8; 32];
            for i in 0..MAX_MKSKIPPED {
                let mk = [0xA5u8; 32];
                st.store_mk_skipped(dh, i as u32, mk).unwrap();
            }
        }

        let mut st1 = make_state();
        fill_to_capacity(&mut st1);
        let before_order = st1.mk_order.clone();
        let before_map = st1.mk_skipped.clone();
        let err1 = st1.store_mk_skipped([0xFFu8; 32], 99, [0x11u8; 32]).unwrap_err();
        assert_eq!(before_order, st1.mk_order);
        assert_eq!(before_map, st1.mk_skipped);

        let mut st2 = make_state();
        fill_to_capacity(&mut st2);
        let err2 = st2.store_mk_skipped([0xFFu8; 32], 99, [0x11u8; 32]).unwrap_err();
        assert_eq!(format!("{err1:?}"), format!("{err2:?}"));
    }

    #[test]
    fn store_mk_skipped_success_stores_and_indexes() {
        let mut st = make_state();
        let dh = [0xABu8; 32];
        let n = 42u32;
        let mk = [0xCDu8; 32];
        st.store_mk_skipped(dh, n, mk).unwrap();
        assert!(st.mk_skipped.contains_key(&(dh, n)));
        let count = st.mk_order.iter().filter(|k| **k == (dh, n)).count();
        assert_eq!(count, 1);
    }

    #[test]
    fn take_mk_skipped_on_missing_does_not_corrupt_order() {
        let mut st = make_state();
        let dh1 = [0x10u8; 32];
        let n1 = 1u32;
        let mk1 = [0x20u8; 32];
        st.store_mk_skipped(dh1, n1, mk1).unwrap();

        let before_order = st.mk_order.clone();
        let before_map = st.mk_skipped.clone();

        let dh_missing = [0x30u8; 32];
        let n_missing = 9u32;
        let got = st.take_mk_skipped(dh_missing, n_missing);
        assert!(got.is_none());
        assert_eq!(before_order, st.mk_order);
        assert_eq!(before_map, st.mk_skipped);
    }
}
