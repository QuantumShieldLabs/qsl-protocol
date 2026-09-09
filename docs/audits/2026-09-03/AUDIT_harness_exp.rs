// Experiments against the UNMODIFIED suite2 ratchet source (sha-verified copy).
// Primitives are dependency-free MOCKS with the algebraic properties the ratchet relies on:
//   Hash/Kmac  : deterministic PRF-like mixers (distinct inputs -> distinct outputs w.h.p.)
//   Aead       : ct = pt || tag, tag = mix(key, nonce, ad, pt); open fails iff any input differs
//   X25519Dh   : toy commutative DH (g^x mod p) so dh(a, B) == dh(b, A)
// These do not measure cryptographic strength; they measure the STATE MACHINE.
use harness::crypto::traits::*;
use harness::suite2::establish::init_from_base_handshake;
use harness::suite2::ratchet::*;
use harness::suite2::state::Suite2SessionState;

fn mix64(mut h: u64, x: u64) -> u64 {
    h ^= x;
    h = h.wrapping_mul(0x9E3779B97F4A7C15);
    h ^= h >> 29;
    h = h.wrapping_mul(0xBF58476D1CE4E5B9);
    h ^= h >> 32;
    h
}
fn mix_bytes(seed: u64, parts: &[&[u8]]) -> u64 {
    let mut h = mix64(0x243F6A8885A308D3, seed);
    for p in parts {
        h = mix64(h, p.len() as u64);
        for c in p.chunks(8) {
            let mut w = [0u8; 8];
            w[..c.len()].copy_from_slice(c);
            h = mix64(h, u64::from_le_bytes(w));
        }
        h = mix64(h, 0xA5A5A5A5);
    }
    h
}
fn expand(seed: u64, parts: &[&[u8]], out: &mut [u8]) {
    let base = mix_bytes(seed, parts);
    for (i, chunk) in out.chunks_mut(8).enumerate() {
        let w = mix64(base, i as u64 + 1).to_le_bytes();
        chunk.copy_from_slice(&w[..chunk.len()]);
    }
}

struct Mock;
impl Hash for Mock {
    fn sha512(&self, data: &[u8]) -> [u8; 64] {
        let mut o = [0u8; 64];
        expand(1, &[data], &mut o);
        o
    }
}
impl Kmac for Mock {
    fn kmac256(&self, key: &[u8], label: &str, data: &[u8], outlen: usize) -> Vec<u8> {
        let mut o = vec![0u8; outlen];
        expand(2, &[key, label.as_bytes(), data], &mut o);
        o
    }
}
impl Aead for Mock {
    fn seal(&self, key32: &[u8; 32], nonce12: &[u8; 12], ad: &[u8], pt: &[u8]) -> Vec<u8> {
        let mut tag = [0u8; 16];
        expand(3, &[key32, nonce12, ad, pt], &mut tag);
        let mut ct = pt.to_vec();
        ct.extend_from_slice(&tag);
        ct
    }
    fn open(&self, key32: &[u8; 32], nonce12: &[u8; 12], ad: &[u8], ct: &[u8]) -> Result<Vec<u8>, CryptoError> {
        if ct.len() < 16 {
            return Err(CryptoError::AuthFail);
        }
        let (pt, tag) = ct.split_at(ct.len() - 16);
        let mut exp = [0u8; 16];
        expand(3, &[key32, nonce12, ad, pt], &mut exp);
        if exp[..] == tag[..] { Ok(pt.to_vec()) } else { Err(CryptoError::AuthFail) }
    }
}
// Toy commutative DH: p = 2^61-1 (prime), g = 3; priv = random mod (p-1), pub = g^priv.
const P: u128 = 2305843009213693951; // 2^61 - 1
fn powmod(mut b: u128, mut e: u128) -> u128 {
    let mut r: u128 = 1;
    b %= P;
    while e > 0 {
        if e & 1 == 1 { r = r * b % P; }
        b = b * b % P;
        e >>= 1;
    }
    r
}
fn to32(v: u128) -> [u8; 32] {
    let mut o = [0u8; 32];
    o[16..].copy_from_slice(&v.to_be_bytes());
    o
}
fn from32(v: &[u8; 32]) -> u128 {
    let mut w = [0u8; 16];
    w.copy_from_slice(&v[16..]);
    u128::from_be_bytes(w)
}
struct ToyDh(std::cell::Cell<u64>);
impl X25519Dh for ToyDh {
    fn keypair(&self) -> (X25519Priv, X25519Pub) {
        let s = self.0.get();
        self.0.set(mix64(s, 77));
        let sk = (mix64(s, 0x1234) as u128) % (P - 1) + 1;
        (X25519Priv(to32(sk)), X25519Pub(to32(powmod(3, sk))))
    }
    fn dh(&self, privk: &X25519Priv, pubk: &X25519Pub) -> [u8; 32] {
        to32(powmod(from32(&pubk.0), from32(&privk.0)))
    }
}

fn establish(dh: &ToyDh) -> (Suite2SessionState, Suite2SessionState) {
    let m = Mock;
    let (a_priv, a_pub) = dh.keypair();
    let (b_priv, b_pub) = dh.keypair();
    let dh_init = dh.dh(&a_priv, &b_pub);
    assert_eq!(dh_init, dh.dh(&b_priv, &a_pub));
    let sid = [7u8; 16];
    let pq = [9u8; 32];
    let mut a = init_from_base_handshake(&m, true, 0x0500, 0x0002, &sid, &dh_init, &pq, &a_pub.0, &b_pub.0, true).unwrap();
    let mut b = init_from_base_handshake(&m, false, 0x0500, 0x0002, &sid, &dh_init, &pq, &b_pub.0, &a_pub.0, true).unwrap();
    a.set_dh_self_priv(a_priv.0);
    b.set_dh_self_priv(b_priv.0);
    (a, b)
}

fn send_normal(st: &mut Suite2SessionState, pt: &[u8]) -> Vec<u8> {
    let m = Mock;
    let out = send_wire(&m, &m, &m, st.send.clone(), 0, pt).expect("send_wire");
    st.send = out.state;
    out.wire
}
fn recv_normal(st: &mut Suite2SessionState, wire: &[u8]) -> Result<Vec<u8>, &'static str> {
    let m = Mock;
    let out = recv_wire(&m, &m, &m, st.recv.clone(), &st.rk, wire, None, None)?;
    st.recv = out.state;
    st.rk = out.rk;
    Ok(out.plaintext)
}

// ---------------- Experiment A: out-of-order reach vs nr ----------------
fn experiment_a() {
    println!("=== A. Forward out-of-order reach of recv_nonboundary_ooo (MAX_HEADER_ATTEMPTS=100, MAX_SKIP=1000) ===");
    println!("For each starting nr: advance in order to nr, then send k more messages and deliver ONLY the last one (gap = k-1).");
    println!("Reported: largest gap that is still received (max gap tried = 12), and the reject code at the first failing gap.");
    let dh = ToyDh(std::cell::Cell::new(1));
    for &target_nr in &[0u32, 10, 50, 90, 93, 94, 95, 100, 500, 2000] {
        let mut max_ok: i64 = -1;
        let mut first_fail: Option<(u32, &'static str)> = None;
        for gap in 0u32..=12 {
            let (mut a, mut b) = establish(&dh);
            // B's send chain is zero until its first boundary; A->B direction is seeded, so measure A->B.
            for i in 0..target_nr {
                let w = send_normal(&mut a, format!("m{i}").as_bytes());
                recv_normal(&mut b, &w).expect("in-order recv");
            }
            assert_eq!(b.recv.nr, target_nr);
            let mut last = Vec::new();
            for _ in 0..=gap {
                last = send_normal(&mut a, b"x");
            }
            match recv_normal(&mut b, &last) {
                Ok(_) => { max_ok = gap as i64; }
                Err(code) => { if first_fail.is_none() { first_fail = Some((gap, code)); } }
            }
        }
        println!("  nr={:<5} max received gap = {:<3} first failing gap = {:?}", target_nr, max_ok, first_fail);
    }
}

// ---------------- Experiment B: crossing DH boundaries ----------------
fn experiment_b() {
    println!("=== B. Crossing DH boundaries: both peers originate a boundary from the same root before pulling ===");
    let dh = ToyDh(std::cell::Cell::new(5));
    let m = Mock;
    let (mut a, mut b) = establish(&dh);
    // Warm the session: A sends, B receives; B's first send is a boundary (its chain is unseeded), A receives it.
    let w = send_normal(&mut a, b"hello");
    recv_normal(&mut b, &w).unwrap();
    let ob = send_boundary(&m, &m, &m, &dh, b.clone(), b"hi back").expect("B first boundary");
    b = ob.state;
    let ra = recv_dh_boundary(&m, &m, &m, &dh, a.clone(), &ob.wire);
    assert!(ra.ok, "A must accept B's first boundary: {:?}", ra.reason);
    a = ra.state;
    // Sanity: a normal message each way works now.
    let w = send_normal(&mut a, b"ok?"); recv_normal(&mut b, &w).unwrap();
    let w = send_normal(&mut b, b"ok!"); recv_normal(&mut a, &w).unwrap();
    println!("  pre-crossing: both directions deliver (control)");
    // THE CROSSING: both have received something, so (in qsc) both have pending_send_ratchet=true.
    assert_eq!(a.rk, b.rk);
    let oa = send_boundary(&m, &m, &m, &dh, a.clone(), b"A crosses").expect("A boundary");
    let ob2 = send_boundary(&m, &m, &m, &dh, b.clone(), b"B crosses").expect("B boundary");
    let a2 = oa.state; let b2 = ob2.state;
    println!("  roots after crossing differ: {}", a2.rk != b2.rk);
    let r1 = recv_dh_boundary(&m, &m, &m, &dh, a2.clone(), &ob2.wire);
    let r2 = recv_dh_boundary(&m, &m, &m, &dh, b2.clone(), &oa.wire);
    println!("  A receives B's boundary: ok={} reason={:?}", r1.ok, r1.reason);
    println!("  B receives A's boundary: ok={} reason={:?}", r2.ok, r2.reason);
    // Aftermath: normal sends in both directions.
    let mut a3 = if r1.ok { r1.state } else { a2 };
    let mut b3 = if r2.ok { r2.state } else { b2 };
    let w = send_normal(&mut a3, b"after A->B");
    let e1 = recv_normal(&mut b3, &w);
    let w = send_normal(&mut b3, b"after B->A");
    let e2 = recv_normal(&mut a3, &w);
    println!("  subsequent normal A->B: {:?}", e1.map(|p| String::from_utf8_lossy(&p).to_string()));
    println!("  subsequent normal B->A: {:?}", e2.map(|p| String::from_utf8_lossy(&p).to_string()));
    // Control: the same boundaries applied SEQUENTIALLY (each delivered before the next originates) work.
    let (mut ca, mut cb) = establish(&dh);
    let w = send_normal(&mut ca, b"hello"); recv_normal(&mut cb, &w).unwrap();
    let o = send_boundary(&m, &m, &m, &dh, cb.clone(), b"b1").unwrap(); cb = o.state;
    let r = recv_dh_boundary(&m, &m, &m, &dh, ca.clone(), &o.wire); assert!(r.ok); ca = r.state;
    let o = send_boundary(&m, &m, &m, &dh, ca.clone(), b"a1").unwrap(); ca = o.state;
    let r = recv_dh_boundary(&m, &m, &m, &dh, cb.clone(), &o.wire); assert!(r.ok); cb = r.state;
    let o = send_boundary(&m, &m, &m, &dh, cb.clone(), b"b2").unwrap(); cb = o.state;
    let r = recv_dh_boundary(&m, &m, &m, &dh, ca.clone(), &o.wire);
    println!("  control (sequential boundaries b1,a1,b2): last accepted ok={} reason={:?}", r.ok, r.reason);
}

// ---------------- Experiment C: PQ reseed with in-flight reverse traffic ----------------
fn experiment_c() {
    println!("=== C. PQ reseed while a reverse-direction message is in flight ===");
    let dh = ToyDh(std::cell::Cell::new(9));
    let m = Mock;
    let adv_pub = vec![0x42u8; 1184];
    let ss = [0x33u8; 32];
    let ct = vec![0x55u8; 1088];

    let (mut a, mut b) = establish(&dh);
    let w = send_normal(&mut a, b"hello"); recv_normal(&mut b, &w).unwrap();
    let o = send_boundary(&m, &m, &m, &dh, b.clone(), b"b1").unwrap(); b = o.state;
    let r = recv_dh_boundary(&m, &m, &m, &dh, a.clone(), &o.wire); assert!(r.ok); a = r.state;
    // A advertises an ML-KEM key (mocked bytes), B tracks it.
    let o = send_pq_advertise(&m, &m, &m, a.clone(), 1, &adv_pub, &[]).expect("adv");
    a = o.state;
    let r = recv_pq_adv_session(&m, &m, &m, b.clone(), &o.wire, 0);
    assert!(r.ok, "B tracks A's ADV: {:?}", r.reason);
    b = r.state;
    // In-flight: A sends a normal message that B has NOT yet pulled.
    let inflight = send_normal(&mut a, b"in flight");
    // B reseeds targeting A's advertisement (mock KEM: both sides agree on ss).
    let o = send_pq_reseed(&m, &m, &m, b.clone(), 1, &ct, &ss, b"reseed").expect("reseed");
    b = o.state;
    // Now B pulls the in-flight message.
    let e = recv_normal(&mut b, &inflight);
    println!("  B receives A's in-flight message after originating the reseed: {:?}", e);
    // A processes the reseed, then sends again.
    let r = recv_pq_reseed(&m, &m, &m, &dh, a.clone(), &o.wire, &ss, 1);
    println!("  A accepts B's reseed: ok={} reason={:?}", r.ok, r.reason);
    a = r.state;
    let w = send_normal(&mut a, b"post-reseed 1");
    let e1 = recv_normal(&mut b, &w);
    let w = send_normal(&mut a, b"post-reseed 2");
    let e2 = recv_normal(&mut b, &w);
    println!("  A->B after reseed (1): {:?}", e1.map(|p| String::from_utf8_lossy(&p).to_string()));
    println!("  A->B after reseed (2): {:?}", e2.map(|p| String::from_utf8_lossy(&p).to_string()));

    // Control: identical flow with NO in-flight message.
    let (mut a, mut b) = establish(&dh);
    let w = send_normal(&mut a, b"hello"); recv_normal(&mut b, &w).unwrap();
    let o = send_boundary(&m, &m, &m, &dh, b.clone(), b"b1").unwrap(); b = o.state;
    let r = recv_dh_boundary(&m, &m, &m, &dh, a.clone(), &o.wire); assert!(r.ok); a = r.state;
    let o = send_pq_advertise(&m, &m, &m, a.clone(), 1, &adv_pub, &[]).unwrap(); a = o.state;
    let r = recv_pq_adv_session(&m, &m, &m, b.clone(), &o.wire, 0); assert!(r.ok); b = r.state;
    let o = send_pq_reseed(&m, &m, &m, b.clone(), 1, &ct, &ss, b"reseed").unwrap(); b = o.state;
    let r = recv_pq_reseed(&m, &m, &m, &dh, a.clone(), &o.wire, &ss, 1); assert!(r.ok); a = r.state;
    let w = send_normal(&mut a, b"post-reseed control");
    let e = recv_normal(&mut b, &w);
    println!("  control (no in-flight message): A->B after reseed: {:?}", e.map(|p| String::from_utf8_lossy(&p).to_string()));
}


// ---------------- Experiment D: on-wire sizes of frame classes (metadata) ----------------
// QSE envelope overhead as encoded by qsc (env_version 2 + flags 2 + route_token varbytes 2+0 +
// timestamp_bucket 4 + pad_len 2 + payload_len 4) = 16 bytes; qsc pads the whole envelope UP TO
// 1024 bytes only when it is shorter than 1024 (lib.rs qsp_pack / qsp_wrap_standard_envelope).
fn experiment_d() {
    println!("=== D. Suite-2 wire sizes per frame class (payload = 40-byte plaintext), QSE overhead = 16, pad floor = 1024 ===");
    let dh = ToyDh(std::cell::Cell::new(21));
    let m = Mock;
    let pt = [0x61u8; 40];
    let (mut a, mut b) = establish(&dh);
    let w_norm = send_normal(&mut a, &pt);
    recv_normal(&mut b, &w_norm).unwrap();
    let ob = send_boundary(&m, &m, &m, &dh, b.clone(), &pt).unwrap(); b = ob.state;
    let r = recv_dh_boundary(&m, &m, &m, &dh, a.clone(), &ob.wire); assert!(r.ok); a = r.state;
    let oadv = send_pq_advertise(&m, &m, &m, a.clone(), 1, &vec![0x42u8; 1184], &pt).unwrap(); a = oadv.state;
    let r = recv_pq_adv_session(&m, &m, &m, b.clone(), &oadv.wire, 0); assert!(r.ok); b = r.state;
    let ors = send_pq_reseed(&m, &m, &m, b.clone(), 1, &vec![0x55u8; 1088], &[0x33u8; 32], &pt).unwrap();
    let _ = ors.state;
    let env = |wire_len: usize| -> usize { let e = wire_len + 16; if e < 1024 { 1024 } else { e } };
    for (name, w) in [("normal message", w_norm.len()), ("DH boundary", ob.wire.len()), ("SCKA ADV", oadv.wire.len()), ("PQ reseed (CTXT)", ors.wire.len())] {
        println!("  {:<18} suite2 wire = {:>5} B   on the relay (QSE+pad) = {:>5} B", name, w, env(w));
    }
}

fn main() {
    experiment_a();
    experiment_b();
    experiment_c();
    experiment_d();
}
