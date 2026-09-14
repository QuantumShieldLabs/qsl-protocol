//! Directional first-release core; exact identifiers reserved in DOC-CAN-003.
//! The authoritative vault controller commits before ciphertext release.
use quantumshield_refimpl::crypto::stdcrypto::{runtime_pq_kem_keypair, StdCrypto};
use quantumshield_refimpl::crypto::traits::{
    Aead, Hash, Kmac, PqKem768, X25519Dh, X25519Priv, X25519Pub,
};
use quantumshield_refimpl::suite2::state::Suite2SessionState;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use zeroize::Zeroize;

pub(crate) type Key = [u8; 32];
pub(crate) type R<T> = Result<T, &'static str>;
pub(crate) const PROFILE: &[u8] = b"NA0780-DIR-EPOCH-CORE-01";
pub(crate) const MAX_SKIP: usize = 16;
pub(crate) const MAX_EPOCHS: usize = 2;
pub(crate) const MAX_TARGETS: usize = 1;
pub(crate) const MAX_WIRE: usize = 65536;
// Opt-in, bounded, thread-local laboratory observations. Never a runtime nonce
// allocator or permission to release speculative ciphertext. Only hashes are kept.
#[cfg(feature = "na0780-test-hooks")]
pub(crate) mod seal_observer {
    use super::{h, lp, Key};
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    const CAP: usize = 4096;
    #[derive(Default)]
    struct Observer { records: Vec<(Key, Key, Key, bool)>, overflow: bool, active: bool, nested: bool }
    thread_local! { static OBSERVER: RefCell<Option<Observer>> = const { RefCell::new(None) }; }
    pub(crate) fn reset() { OBSERVER.with(|o| *o.borrow_mut()=Some(Observer::default())); }
    pub(crate) fn disable() { OBSERVER.with(|o| *o.borrow_mut()=None); }
    pub(crate) fn record(key:&Key,nonce:&[u8;12],ad:&[u8],pt:&[u8],ct:&[u8]) {
        OBSERVER.with(|o| { let mut o=o.borrow_mut(); let Some(o)=o.as_mut() else{return};
            if o.records.len()==CAP {o.overflow=true;return;}
            let mut pair=key.to_vec();pair.extend(nonce);
            let mut input=lp(ad);input.extend(lp(pt));
            o.records.push((h(&pair),h(&input),h(ct),false));
            use zeroize::Zeroize; pair.zeroize();input.zeroize();
        });
    }
    pub(crate) struct Attempt { start: Option<usize> }
    pub(crate) fn begin()->Attempt {
        let start=OBSERVER.with(|o| {let mut o=o.borrow_mut();o.as_mut().map(|o| {
            if o.active {o.nested=true;} o.active=true;o.records.len()
        })});Attempt{start}
    }
    impl Attempt {
        // Called only after the authoritative vault save succeeds. This tracks
        // bytes eligible for release, conservatively including unreleased commits.
        pub(crate) fn committed(&self) {if let Some(start)=self.start {OBSERVER.with(|o| {
            if let Some(o)=o.borrow_mut().as_mut() {for r in &mut o.records[start..] {r.3=true;}}
        });}}
    }
    impl Drop for Attempt {fn drop(&mut self) {if self.start.is_some() {OBSERVER.with(|o| {
        if let Some(o)=o.borrow_mut().as_mut() {o.active=false;}
    });}}}
    // [all seals, committed seals, conflicting internal pairs, conflicting
    // committed pairs, overflow/nesting]. Same inputs and output are exact repeats.
    pub(crate) fn stats()->[usize;5] {OBSERVER.with(|o| {
        let o=o.borrow();let Some(o)=o.as_ref() else{return [0;5]};
        let mut all=BTreeMap::new();let mut committed=BTreeMap::new();let mut s=[o.records.len(),0,0,0,usize::from(o.overflow||o.nested)];
        for &(pair,input,ct,done) in &o.records {
            if all.insert(pair,(input,ct)).is_some_and(|old|old!=(input,ct)) {s[2]+=1;}
            if done {s[1]+=1;if committed.insert(pair,(input,ct)).is_some_and(|old|old!=(input,ct)) {s[3]+=1;}}
        } s
    })}
}

pub(crate) fn lp(x: &[u8]) -> Vec<u8> {
    let mut b = (x.len() as u32).to_be_bytes().to_vec();
    b.extend(x);
    b
}
pub(crate) fn h(x: &[u8]) -> Key {
    StdCrypto.sha512(x)[..32].try_into().unwrap()
}
pub(crate) fn context(sid: &[u8; 16]) -> Vec<u8> {
    let mut b = lp(PROFILE);
    b.extend(sid);
    b
}
pub(crate) fn k(sid: &[u8; 16], key: &Key, label: &str, x: &[u8]) -> Key {
    let mut c = context(sid);
    c.extend(x);
    StdCrypto
        .kmac256(key, &format!("NA0780.DE1/{label}"), &c, 32)
        .try_into()
        .unwrap()
}
pub(crate) fn ectx(id: u64, dir: u8, dh: &Key) -> Vec<u8> {
    let mut b = id.to_be_bytes().to_vec();
    b.push(dir);
    b.extend(dh);
    b
}
pub(crate) fn seal(key: &Key, nonce: &[u8; 12], ad: &[u8], pt: &[u8]) -> R<Vec<u8>> {
    let out = StdCrypto.seal(key, nonce, ad, pt);
    if out.len() != pt.len() + 16 {
        return Err("AEAD_SEAL");
    }
    #[cfg(feature = "na0780-test-hooks")]
    seal_observer::record(key, nonce, ad, pt, &out);
    Ok(out)
}
pub(crate) fn typed(kind: u8, body: &[u8]) -> Vec<u8> {
    let mut p = vec![kind];
    p.extend(lp(body));
    p
}
pub(crate) fn open_typed(p: &[u8]) -> R<(u8, Vec<u8>)> {
    if p.len() < 5 || p[0] > 2 {
        return Err("TYPED");
    }
    let n = u32::from_be_bytes(p[1..5].try_into().unwrap()) as usize;
    if n != p.len() - 5 {
        return Err("TYPED_LENGTH");
    }
    Ok((p[0], p[5..].to_vec()))
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct Epoch {
    pub(crate) id: u64,
    pub(crate) dir: u8,
    pub(crate) dh: Key,
    pub(crate) ec: Key,
    pub(crate) pq: Key,
    pub(crate) hk: Key,
    pub(crate) adv: Key,
    pub(crate) next: u32,
    pub(crate) terminal: Option<u32>,
    pub(crate) skipped: BTreeMap<u32, Key>,
}
impl Epoch {
    pub(crate) fn keys(sid: &[u8; 16], id: u64, dir: u8, dh: Key, root: &Key, ec: Key, pq: Key) -> Self {
        let e = ectx(id, dir, &dh);
        Self {
            id,
            dir,
            dh,
            ec,
            pq,
            hk: k(sid, root, "HK", &e),
            adv: k(sid, root, "ADV_KEY", &e),
            next: 0,
            terminal: None,
            skipped: BTreeMap::new(),
        }
    }
    pub(crate) fn step(&mut self, sid: &[u8; 16]) -> R<Key> {
        let n = self.next;
        let next = n.checked_add(1).ok_or("COUNTER_OVERFLOW")?;
        let mut z = ectx(self.id, self.dir, &self.dh);
        z.extend(n.to_be_bytes());
        let em = k(sid, &self.ec, "EC_MK", &z);
        let pm = k(sid, &self.pq, "PQ_MK", &z);
        let mut hybrid = z.clone();
        hybrid.extend(pm);
        let mk = k(sid, &em, "HYBRID", &hybrid);
        self.ec = k(sid, &self.ec, "EC_STEP", &z);
        self.pq = k(sid, &self.pq, "PQ_STEP", &z);
        self.next = next;
        Ok(mk)
    }
}
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct LocalTarget {
    pub(crate) pk: Vec<u8>,
    pub(crate) sk: Vec<u8>,
}
impl Drop for LocalTarget {
    fn drop(&mut self) {
        self.sk.zeroize();
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct Core {
    pub(crate) sid: [u8; 16],
    pub(crate) role: u8,
    pub(crate) root: Key,
    pub(crate) seq: u64,
    pub(crate) digest: Key,
    pub(crate) owner: u8,
    pub(crate) own_priv: Key,
    pub(crate) own_pub: Key,
    pub(crate) peer_pub: Key,
    pub(crate) send: Option<Epoch>,
    pub(crate) recv: BTreeMap<u64, Epoch>,
    pub(crate) active_recv: Option<u64>,
    pub(crate) local: BTreeMap<u32, LocalTarget>,
    pub(crate) local_next: u32,
    pub(crate) local_consumed_prefix: u32,
    pub(crate) peer: BTreeMap<u32, Vec<u8>>,
    pub(crate) peer_max: u32,
    pub(crate) peer_selected_prefix: u32,
    pub(crate) last_in: Option<Key>,
    pub(crate) last_out: Vec<u8>,
}

#[derive(Clone)]
pub(crate) struct Wire {
    pub(crate) kind: u8,
    pub(crate) sid: [u8; 16],
    pub(crate) dir: u8,
    pub(crate) epoch: u64,
    pub(crate) n: u32,
    pub(crate) dh: Key,
    pub(crate) parent: u64,
    pub(crate) parent_digest: Key,
    pub(crate) previous: u64,
    pub(crate) terminal: u32,
    pub(crate) target: u32,
    pub(crate) ct: Vec<u8>,
    pub(crate) hc: Vec<u8>,
    pub(crate) bc: Vec<u8>,
}
impl Wire {
    pub(crate) fn prefix(&self, body_len: usize) -> Vec<u8> {
        let mut b = b"NDE1".to_vec();
        b.push(self.kind);
        b.extend(self.sid);
        b.push(self.dir);
        b.extend(self.epoch.to_be_bytes());
        b.extend(self.n.to_be_bytes());
        b.extend(self.dh);
        b.extend(self.parent.to_be_bytes());
        b.extend(self.parent_digest);
        b.extend(self.previous.to_be_bytes());
        b.extend(self.terminal.to_be_bytes());
        b.extend(self.target.to_be_bytes());
        b.extend(lp(&self.ct));
        b.extend((body_len as u32).to_be_bytes());
        b
    }
    pub(crate) fn ad_nonce(&self, label: &[u8], nonce_label: &[u8], body_len: usize) -> (Vec<u8>, [u8; 12]) {
        let p = self.prefix(body_len);
        let mut ad = context(&self.sid);
        ad.extend(label);
        ad.extend(&p);
        let mut nd = context(&self.sid);
        nd.extend(nonce_label);
        nd.extend(p);
        (ad, StdCrypto.sha512(&nd)[..12].try_into().unwrap())
    }
    pub(crate) fn encrypt(mut self, hk: &Key, mk: &Key, p: &[u8]) -> R<Vec<u8>> {
        let mut hp = self.n.to_be_bytes().to_vec();
        hp.extend(self.terminal.to_be_bytes());
        let len = p.len() + 16;
        if self.prefix(len).len() + 24 + len > MAX_WIRE {
            return Err("WIRE_BOUND");
        }
        let (ha, hn) = self.ad_nonce(b"HDR", b"NH", len);
        let (ba, bn) = self.ad_nonce(b"BODY", b"NB", len);
        self.hc = seal(hk, &hn, &ha, &hp)?;
        self.bc = seal(mk, &bn, &ba, p)?;
        let wire = self.encode();
        if wire.len() > MAX_WIRE {
            return Err("WIRE_BOUND");
        }
        Ok(wire)
    }
    pub(crate) fn header(&self, hk: &Key) -> R<()> {
        let (ad, no) = self.ad_nonce(b"HDR", b"NH", self.bc.len());
        let p = StdCrypto
            .open(hk, &no, &ad, &self.hc)
            .map_err(|_| "HEADER_AUTH")?;
        let mut expected = self.n.to_be_bytes().to_vec();
        expected.extend(self.terminal.to_be_bytes());
        if p != expected {
            return Err("HEADER_BINDING");
        }
        Ok(())
    }
    pub(crate) fn body(&self, mk: &Key) -> R<Vec<u8>> {
        let (ad, no) = self.ad_nonce(b"BODY", b"NB", self.bc.len());
        StdCrypto
            .open(mk, &no, &ad, &self.bc)
            .map_err(|_| "BODY_AUTH")
    }
    pub(crate) fn encode(&self) -> Vec<u8> {
        let mut b = self.prefix(self.bc.len());
        b.extend(&self.hc);
        b.extend(&self.bc);
        b
    }
    pub(crate) fn parse(raw: &[u8]) -> R<Self> {
        struct Cur<'a> {
            b: &'a [u8],
            i: usize,
        }
        impl<'a> Cur<'a> {
            fn take(&mut self, n: usize) -> R<&'a [u8]> {
                let end = self.i.checked_add(n).ok_or("PARSE")?;
                let x = self.b.get(self.i..end).ok_or("PARSE")?;
                self.i = end;
                Ok(x)
            }
            fn a<const N: usize>(&mut self) -> R<[u8; N]> {
                Ok(self.take(N)?.try_into().unwrap())
            }
            fn u32(&mut self) -> R<u32> {
                Ok(u32::from_be_bytes(self.a()?))
            }
            fn u64(&mut self) -> R<u64> {
                Ok(u64::from_be_bytes(self.a()?))
            }
        }
        if raw.len() > MAX_WIRE {
            return Err("WIRE_BOUND");
        }
        let mut c = Cur { b: raw, i: 0 };
        if c.take(4)? != b"NDE1" {
            return Err("MAGIC");
        }
        let kind = c.a::<1>()?[0];
        let sid = c.a()?;
        let dir = c.a::<1>()?[0];
        if kind > 1 || dir > 1 {
            return Err("TYPE");
        }
        let epoch = c.u64()?;
        let n = c.u32()?;
        let dh = c.a()?;
        let parent = c.u64()?;
        let parent_digest = c.a()?;
        let previous = c.u64()?;
        let terminal = c.u32()?;
        let target = c.u32()?;
        let ctlen = c.u32()? as usize;
        if ctlen != 0 && ctlen != 1088 {
            return Err("CT_LENGTH");
        }
        let ct = c.take(ctlen)?.to_vec();
        let bl = c.u32()? as usize;
        if bl < 16 {
            return Err("BODY_LENGTH");
        }
        let hc = c.take(24)?.to_vec();
        let bc = c.take(bl)?.to_vec();
        if c.i != raw.len() {
            return Err("TRAILING");
        }
        Ok(Self {
            kind,
            sid,
            dir,
            epoch,
            n,
            dh,
            parent,
            parent_digest,
            previous,
            terminal,
            target,
            ct,
            hc,
            bc,
        })
    }
}

pub(crate) fn schedule(
    s: &Core,
    g: u64,
    d: u8,
    dh: &Key,
    other: &Key,
    dhss: &Key,
    pq_contribution: (u32, &[u8], &[u8]),
) -> R<(Key, Epoch)> {
    let (target, ct, ss) = pq_contribution;
    if *dhss == [0; 32] {
        return Err("DH_NONCONTRIBUTORY");
    }
    let mut x = g.to_be_bytes().to_vec();
    x.extend(s.digest);
    x.push(d);
    x.extend(dh);
    x.extend(other);
    x.extend(dhss);
    let rd = k(&s.sid, &s.root, "RK_DH", &x);
    let e = ectx(g, d, dh);
    let ec = k(&s.sid, &rd, "EC0", &e);
    let (root, pq) = if target == 0 {
        if !ct.is_empty() || !ss.is_empty() {
            return Err("PQ_SHAPE");
        }
        (rd, k(&s.sid, &rd, "PQ0", &e))
    } else {
        if ct.len() != 1088 || ss.len() != 32 {
            return Err("PQ_SHAPE");
        }
        let mut p = e.clone();
        p.extend(target.to_be_bytes());
        p.extend(h(ct));
        p.extend(ss);
        (k(&s.sid, &rd, "RK_PQ", &p), k(&s.sid, &rd, "PQ_SEED", &p))
    };
    Ok((root, Epoch::keys(&s.sid, g, d, *dh, &root, ec, pq)))
}
impl Core {
    pub(crate) fn authenticated(s: &Suite2SessionState) -> Self {
        // Called only at the verified fresh candidate-profile establishment commit.
        let role = if s.recv.role_is_a { 0 } else { 1 };
        let sid = s.send.session_id;
        let root = k(&sid, &s.rk, "ROOT0", &[]);
        let ap = if role == 0 { s.dh.dhs_pub } else { s.dh.dhr };
        let bp = if role == 1 { s.dh.dhs_pub } else { s.dh.dhr };
        let mut initial = context(&sid);
        initial.extend(ap);
        initial.extend(bp);
        let digest = h(&initial);
        let e = ectx(0, 0, &ap);
        let aep = Epoch::keys(
            &sid,
            0,
            0,
            ap,
            &root,
            k(&sid, &root, "EC0", &e),
            k(&sid, &root, "PQ0", &e),
        );
        let mut recv = BTreeMap::new();
        if role == 1 {
            recv.insert(0, aep.clone());
        }
        Self {
            sid,
            role,
            root,
            seq: 0,
            digest,
            owner: 1,
            own_priv: s.dh.dhs_priv,
            own_pub: s.dh.dhs_pub,
            peer_pub: s.dh.dhr,
            send: if role == 0 { Some(aep) } else { None },
            recv,
            active_recv: if role == 1 { Some(0) } else { None },
            local: BTreeMap::new(),
            local_next: 0,
            local_consumed_prefix: 0,
            peer: BTreeMap::new(),
            peer_max: 0,
            peer_selected_prefix: 0,
            last_in: None,
            last_out: Vec::new(),
        }
    }


    pub(crate) fn ordinary(&mut self, kind: u8, body: &[u8]) -> R<Vec<u8>> {
        let mut e = self.send.clone().ok_or("SEND_UNSET")?;
        let n = e.next;
        let w = Wire {
            kind: 0,
            sid: self.sid,
            dir: self.role,
            epoch: e.id,
            n,
            dh: e.dh,
            parent: 0,
            parent_digest: [0; 32],
            previous: 0,
            terminal: 0,
            target: 0,
            ct: vec![],
            hc: vec![],
            bc: vec![],
        };
        let mk = e.step(&self.sid)?;
        let wire = w.encrypt(&e.hk, &mk, &typed(kind, body))?;
        self.send = Some(e);
        Ok(wire)
    }
    pub(crate) fn advertise(&mut self) -> R<(u32, Vec<u8>)> {
        if self.local.len() >= MAX_TARGETS {
            return Err("TARGET_CAPACITY");
        }
        let id = self.local_next.checked_add(1).ok_or("TARGET_OVERFLOW")?;
        let (pk, sk) = runtime_pq_kem_keypair();
        let e = self.send.as_ref().ok_or("SEND_UNSET")?;
        let mut macdata = ectx(e.id, e.dir, &e.dh);
        macdata.extend(id.to_be_bytes());
        macdata.extend(lp(&pk));
        let mac = k(&self.sid, &e.adv, "ADV_AUTH", &macdata);
        let mut body = id.to_be_bytes().to_vec();
        body.extend(lp(&pk));
        body.extend(mac);
        let mut next = self.clone();
        let wire = next.ordinary(2, &body)?;
        next.local.insert(id, LocalTarget { pk, sk });
        next.local_next = id;
        *self = next;
        Ok((id, wire))
    }
    // Internal negative-frame builder can name an arbitrary target/public key. It still
    // requires genuine ownership and a fresh DH key; normal boundary() validates selection.
    pub(crate) fn craft_boundary(&self, target: Option<(u32, Vec<u8>)>, body: &[u8]) -> R<(Self, Vec<u8>)> {
        if self.owner != self.role {
            return Err("NOT_OWNER");
        }
        let g = self.seq.checked_add(1).ok_or("ROOT_OVERFLOW")?;
        let (privk, pubk) = StdCrypto.keypair();
        let dhss = StdCrypto.dh(&privk, &X25519Pub(self.peer_pub));
        let (tid, ct, mut ss) = if let Some((id, pk)) = target {
            let (ct, ss) = StdCrypto.encap(&pk).map_err(|_| "ENCAP")?;
            (id, ct, ss)
        } else {
            (0, vec![], vec![])
        };
        let (root, mut e) = schedule(
            self,
            g,
            self.role,
            &pubk.0,
            &self.peer_pub,
            &dhss,
            (tid, &ct, &ss),
        )?;
        ss.zeroize();
        let mut nh = self.seq.to_be_bytes().to_vec();
        nh.push(self.role);
        let hk = k(&self.sid, &self.root, "NHK", &nh);
        let w = Wire {
            kind: 1,
            sid: self.sid,
            dir: self.role,
            epoch: g,
            n: 0,
            dh: pubk.0,
            parent: self.seq,
            parent_digest: self.digest,
            previous: self.send.as_ref().map(|e| e.id).unwrap_or(u64::MAX),
            terminal: self.send.as_ref().map(|e| e.next).unwrap_or(0),
            target: tid,
            ct,
            hc: vec![],
            bc: vec![],
        };
        let mk = e.step(&self.sid)?;
        let wire = w.encrypt(&hk, &mk, &typed(0, body))?;
        let mut next = self.clone();
        next.seq = g;
        next.root = root;
        next.digest = h(&wire);
        next.owner = 1 - self.role;
        next.own_priv = privk.0;
        next.own_pub = pubk.0;
        next.send = Some(e);
        next.last_out = wire.clone();
        if tid != 0 {
            next.peer_selected_prefix = tid;
            next.peer.remove(&tid);
        }
        Ok((next, wire))
    }
    pub(crate) fn boundary(&mut self, target: Option<u32>, body: &[u8]) -> R<Vec<u8>> {
        if self.owner != self.role {
            return Err("NOT_OWNER");
        }
        let t = if let Some(id) = target {
            if id <= self.peer_selected_prefix {
                return Err("TARGET_SPENT");
            }
            if Some(id) != self.peer_selected_prefix.checked_add(1) {
                return Err("TARGET_NONMONOTONIC");
            }
            Some((id, self.peer.get(&id).ok_or("PEER_TARGET_UNKNOWN")?.clone()))
        } else {
            None
        };
        let (next, wire) = self.craft_boundary(t, body)?;
        *self = next;
        Ok(wire)
    }
    pub(crate) fn skip_count(&self) -> usize {
        self.recv.values().map(|e| e.skipped.len()).sum()
    }
    pub(crate) fn dispatch(&mut self, e: &Epoch, pt: &[u8]) -> R<(u8, Vec<u8>)> {
        let (kind, body) = open_typed(pt)?;
        if kind == 2 {
            if body.len() != 4 + 4 + 1184 + 32 {
                return Err("ADV_LENGTH");
            }
            let id = u32::from_be_bytes(body[..4].try_into().unwrap());
            if id == 0 || u32::from_be_bytes(body[4..8].try_into().unwrap()) != 1184 {
                return Err("ADV_LENGTH");
            }
            let pk = &body[8..1192];
            let mut data = ectx(e.id, e.dir, &e.dh);
            data.extend(id.to_be_bytes());
            data.extend(lp(pk));
            let expected = k(&self.sid, &e.adv, "ADV_AUTH", &data);
            let diff = expected
                .iter()
                .zip(&body[1192..])
                .fold(0u8, |v, (a, b)| v | (*a ^ *b));
            if diff != 0 {
                return Err("ADV_AUTH");
            }
            if id <= self.peer_selected_prefix {
                return Ok((kind, body));
            }
            if Some(id) != self.peer_selected_prefix.checked_add(1) {
                return Err("TARGET_NONMONOTONIC");
            }
            if let Some(prior) = self.peer.get(&id) {
                if prior != pk {
                    return Err("TARGET_EQUIVOCATION");
                }
            } else {
                if self.peer.len() >= MAX_TARGETS {
                    return Err("PEER_TARGET_CAPACITY");
                }
                self.peer.insert(id, pk.to_vec());
                self.peer_max = id;
            }
        }
        Ok((kind, body))
    }
    pub(crate) fn receive(&mut self, raw: &[u8]) -> R<(u8, Vec<u8>)> {
        let w = Wire::parse(raw)?;
        if w.sid != self.sid || w.dir != 1 - self.role {
            return Err("SESSION_DIRECTION");
        }
        let mut next = self.clone();
        let result = if w.kind == 0 {
            if w.parent != 0
                || w.parent_digest != [0; 32]
                || w.previous != 0
                || w.terminal != 0
                || w.target != 0
                || !w.ct.is_empty()
            {
                return Err("ORDINARY_SHAPE");
            }
            let mut e = next.recv.get(&w.epoch).ok_or("EPOCH_UNKNOWN")?.clone();
            if w.dh != e.dh || w.dir != e.dir {
                return Err("EPOCH_BINDING");
            }
            w.header(&e.hk)?;
            if e.terminal.is_some_and(|t| w.n >= t) {
                return Err("TERMINAL_BOUND");
            }
            let mk = if w.n < e.next {
                *e.skipped.get(&w.n).ok_or("REPLAY")?
            } else {
                let gap = (w.n - e.next) as usize;
                if next.skip_count() + gap > MAX_SKIP {
                    return Err("SKIP_BOUND");
                }
                while e.next < w.n {
                    let n = e.next;
                    let mk = e.step(&next.sid)?;
                    e.skipped.insert(n, mk);
                }
                e.step(&next.sid)?
            };
            let pt = w.body(&mk)?;
            let result = next.dispatch(&e, &pt)?;
            e.skipped.remove(&w.n);
            if e.terminal.is_some() && e.skipped.is_empty() {
                next.recv.remove(&e.id);
            } else {
                next.recv.insert(e.id, e);
            }
            result
        } else {
            if self.last_in == Some(h(raw)) {
                return Err("BOUNDARY_REPLAY");
            }
            if self.owner != w.dir
                || w.parent != self.seq
                || w.epoch != self.seq.checked_add(1).ok_or("ROOT_OVERFLOW")?
                || w.parent_digest != self.digest
                || w.n != 0
            {
                return Err("ROOT_PARENT_OWNER");
            }
            if w.previous != self.active_recv.unwrap_or(u64::MAX) {
                return Err("PREVIOUS_EPOCH");
            }
            let mut nh = self.seq.to_be_bytes().to_vec();
            nh.push(w.dir);
            w.header(&k(&self.sid, &self.root, "NHK", &nh))?;
            if self.recv.len() >= MAX_EPOCHS {
                return Err("EPOCH_CAPACITY");
            }
            let dhss = StdCrypto.dh(&X25519Priv(self.own_priv), &X25519Pub(w.dh));
            let mut ss = if w.target == 0 {
                if !w.ct.is_empty() {
                    return Err("PQ_SHAPE");
                }
                vec![]
            } else {
                if w.target <= self.local_consumed_prefix {
                    return Err("TARGET_SPENT");
                }
                if Some(w.target) != self.local_consumed_prefix.checked_add(1) {
                    return Err("TARGET_NONMONOTONIC");
                }
                let t = self.local.get(&w.target).ok_or("TARGET_UNKNOWN")?;
                StdCrypto.decap(&t.sk, &w.ct).map_err(|_| "DECAP")?
            };
            let (root, mut e) = schedule(
                self,
                w.epoch,
                w.dir,
                &w.dh,
                &self.own_pub,
                &dhss,
                (w.target, &w.ct, &ss),
            )?;
            ss.zeroize();
            let mk = e.step(&self.sid)?;
            let pt = w.body(&mk)?;
            let result = next.dispatch(&e, &pt)?;
            if let Some(id) = self.active_recv {
                let mut old = next.recv.get(&id).ok_or("PREVIOUS_EPOCH")?.clone();
                if w.terminal < old.next {
                    return Err("TERMINAL_REGRESSION");
                }
                if next.skip_count() + (w.terminal - old.next) as usize > MAX_SKIP {
                    return Err("SKIP_BOUND");
                }
                while old.next < w.terminal {
                    let n = old.next;
                    let mk = old.step(&next.sid)?;
                    old.skipped.insert(n, mk);
                }
                old.terminal = Some(w.terminal);
                old.ec.zeroize();
                old.pq.zeroize();
                if old.skipped.is_empty() {
                    next.recv.remove(&id);
                } else {
                    next.recv.insert(id, old);
                }
            } else if w.terminal != 0 {
                return Err("BOOTSTRAP_TERMINAL");
            }
            next.recv.insert(e.id, e);
            next.active_recv = Some(w.epoch);
            next.root = root;
            next.seq = w.epoch;
            next.digest = h(raw);
            next.owner = self.role;
            next.peer_pub = w.dh;
            next.last_in = Some(h(raw));
            if w.target != 0 {
                next.local_consumed_prefix = w.target;
                let mut target = next.local.remove(&w.target).unwrap();
                target.sk.zeroize();
                assert!(target.sk.iter().all(|b| *b == 0));
            }
            result
        };
        if next.recv.len() > MAX_EPOCHS || next.skip_count() > MAX_SKIP {
            return Err("INTERNAL_BOUND");
        }
        *self = next;
        Ok(result)
    }
}
