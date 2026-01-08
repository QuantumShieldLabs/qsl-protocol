use std::collections::HashMap;
use std::io::{self, BufRead, Write};

use base64::Engine;
use clap::Parser;
use rand_chacha::ChaCha20Rng;
use rand_core::{RngCore, SeedableRng};
use serde::{Deserialize, Serialize};
use sha3::{Digest as Sha3Digest, Sha3_256};
use sha2::{Digest as Sha2Digest, Sha512};

use ed25519_dalek::Signer as _;

use ml_dsa::{MlDsa65, Signature as MlDsaSig, SigningKey as MlDsaSk, VerifyingKey as MlDsaVk};
use ml_dsa::KeyGen as _;
use ml_dsa::signature::{Keypair as _, Signer as _, Verifier as _};

use ml_kem::{B32, EncapsulateDeterministic, Encoded, EncodedSizeUser, KemCore, MlKem768};
use ml_kem::kem::{DecapsulationKey as MlKemDk, EncapsulationKey as MlKemEk, Decapsulate as _};

use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
use quantumshield_refimpl::crypto::traits::{
    CryptoError, PqKem768, PqSigMldsa65, Rng12, SigEd25519, X25519Dh, X25519Priv, X25519Pub,
};
use quantumshield_refimpl::kt::{KtError, KtVerifier};
use quantumshield_refimpl::qsp::{
    initiator_build, initiator_finalize, responder_process, ratchet_decrypt, ratchet_encrypt,
    HandshakeDeps, HandshakeInit, HandshakeResp, InitiatorState, PrekeyBundle, ProtocolMessage,
    SessionState, SZ_ED25519_SIG, SZ_MLDSA65_SIG, SZ_MLKEM768_PUB,
};

// ---------------------------
// CLI
// ---------------------------

#[derive(Parser, Debug)]
#[command(name = "refimpl_actor", version)]
struct Cli {
    /// Actor name (e.g., impl_a / impl_b)
    #[arg(long)]
    name: String,

    /// CI mode: deterministic defaults
    #[arg(long)]
    ci: bool,
}

// ---------------------------
// Actor contract types (JSONL)
// ---------------------------

#[derive(Debug, Deserialize)]
struct Req {
    id: String,
    op: String,
    #[serde(default)]
    params: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct RespOk {
    id: String,
    ok: bool,
    result: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct RespErr {
    id: String,
    ok: bool,
    error: ErrObj,
}

#[derive(Debug, Serialize)]
struct ErrObj {
    code: String,
    message: String,
}

// ---------------------------
// Errors
// ---------------------------

#[derive(thiserror::Error, Debug)]
enum ActorError {
    #[error("invalid request: {0}")]
    Invalid(String),
    #[error("unsupported operation: {0}")]
    Unsupported(String),
    #[error("crypto error: {0}")]
    Crypto(String),
    #[error("internal error: {0}")]
    Internal(String),
}

impl ActorError {
    fn code(&self) -> &'static str {
        match self {
            ActorError::Invalid(_) => "INVALID",
            ActorError::Unsupported(_) => "UNSUPPORTED",
            ActorError::Crypto(_) => "CRYPTO",
            ActorError::Internal(_) => "INTERNAL",
        }
    }
}

// ---------------------------
// Deterministic seeding + encoding helpers
// ---------------------------

fn sha3_256(data: &[u8]) -> [u8; 32] {
    let mut h = Sha3_256::new();
    h.update(data);
    let out = h.finalize();
    let mut b = [0u8; 32];
    b.copy_from_slice(&out);
    b
}

fn derive_seed32(label: &str, name: &str, seed: &str) -> [u8; 32] {
    // Domain-separated, deterministic across runs.
    let mut v = Vec::new();
    v.extend_from_slice(b"QSHIELD_ACTOR_SEED\0");
    v.extend_from_slice(label.as_bytes());
    v.push(0);
    v.extend_from_slice(name.as_bytes());
    v.push(0);
    v.extend_from_slice(seed.as_bytes());
    sha3_256(&v)
}

fn b64u_encode(bytes: &[u8]) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

fn b64u_decode(s: &str) -> Result<Vec<u8>, ActorError> {
    base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(s)
        .map_err(|e| ActorError::Invalid(format!("bad base64url: {e}")))
}

fn session_id_to_string(sid: &[u8; 16]) -> String {
    b64u_encode(sid)
}

fn session_id_from_string(s: &str) -> Result<[u8; 16], ActorError> {
    let b = b64u_decode(s)?;
    if b.len() != 16 {
        return Err(ActorError::Invalid("session_id must decode to 16 bytes".into()));
    }
    let mut sid = [0u8; 16];
    sid.copy_from_slice(&b);
    Ok(sid)
}

// ---------------------------
// Crypto adapters for the refimpl skeleton
// ---------------------------

/// Deterministic X25519 provider.
struct DhDet {
    rng: std::sync::Mutex<ChaCha20Rng>,
}

impl DhDet {
    fn new(seed32: [u8; 32]) -> Self {
        Self { rng: std::sync::Mutex::new(ChaCha20Rng::from_seed(seed32)) }
    }
}

impl X25519Dh for DhDet {
    fn keypair(&self) -> (X25519Priv, X25519Pub) {
        let mut sk = [0u8; 32];
        let mut rng = self.rng.lock().expect("rng mutex");
        rng.fill_bytes(&mut sk);
        let secret = x25519_dalek::StaticSecret::from(sk);
        let pubk = x25519_dalek::PublicKey::from(&secret);
        (X25519Priv(secret.to_bytes()), X25519Pub(pubk.to_bytes()))
    }

    fn dh(&self, privk: &X25519Priv, pubk: &X25519Pub) -> [u8; 32] {
        let sk = x25519_dalek::StaticSecret::from(privk.0);
        let pk = x25519_dalek::PublicKey::from(pubk.0);
        sk.diffie_hellman(&pk).to_bytes()
    }
}

/// Deterministic 12-byte nonce source.
struct Rng12Det {
    rng: ChaCha20Rng,
}

impl Rng12Det {
    fn new(seed32: [u8; 32]) -> Self {
        Self { rng: ChaCha20Rng::from_seed(seed32) }
    }
}

impl Rng12 for Rng12Det {
    fn random_nonce12(&mut self) -> [u8; 12] {
        let mut b = [0u8; 12];
        self.rng.fill_bytes(&mut b);
        b
    }
}

/// ML-KEM-768 adapter.
///
/// Uses deterministic encapsulation (m derived from a local counter) to ensure CI stability.
struct MlKemDet {
    ctr: std::sync::Mutex<u64>,
}

impl MlKemDet {
    fn new() -> Self {
        Self { ctr: std::sync::Mutex::new(0) }
    }

    fn next_m(&self, domain: &[u8]) -> B32 {
        let mut c = self.ctr.lock().expect("ctr mutex");
        let n = *c;
        *c = n.wrapping_add(1);

        let mut v = Vec::with_capacity(domain.len() + 8);
        v.extend_from_slice(domain);
        v.extend_from_slice(&n.to_be_bytes());
        B32::from(sha3_256(&v))
    }

    fn dk_from_bytes(privk: &[u8]) -> Result<MlKemDk<ml_kem::MlKem768Params>, CryptoError> {
        let enc = Encoded::<MlKemDk<ml_kem::MlKem768Params>>::try_from(privk)
            .map_err(|_| CryptoError::InvalidKey)?;
        Ok(MlKemDk::from_bytes(&enc))
    }

    fn ek_from_bytes(pubk: &[u8]) -> Result<MlKemEk<ml_kem::MlKem768Params>, CryptoError> {
        let enc = Encoded::<MlKemEk<ml_kem::MlKem768Params>>::try_from(pubk)
            .map_err(|_| CryptoError::InvalidKey)?;
        Ok(MlKemEk::from_bytes(&enc))
    }
}

impl PqKem768 for MlKemDet {
    fn encap(&self, pubk: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CryptoError> {
        let ek = Self::ek_from_bytes(pubk)?;
        let m = self.next_m(b"QSHIELD_MLKEM_M");
        let (ct, ss) = ek.encapsulate_deterministic(&m).map_err(|_| CryptoError::InvalidKey)?;
        let ct_bytes: &[u8] = ct.as_ref();
        let ss_bytes: &[u8] = ss.as_slice();
        Ok((ct_bytes.to_vec(), ss_bytes.to_vec()))
    }

    fn decap(&self, privk: &[u8], ct: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let dk = Self::dk_from_bytes(privk)?;
        let ct_enc = ml_kem::Ciphertext::<MlKem768>::try_from(ct).map_err(|_| CryptoError::AuthFail)?;
        let ss = dk.decapsulate(&ct_enc).map_err(|_| CryptoError::AuthFail)?;
        Ok(ss.as_slice().to_vec())
    }
}

/// ML-DSA-65 adapter.
struct MlDsaDet;

impl PqSigMldsa65 for MlDsaDet {
    fn sign(&self, privk: &[u8], msg: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let enc = ml_dsa::EncodedSigningKey::<MlDsa65>::try_from(privk).map_err(|_| CryptoError::InvalidKey)?;
        let sk = MlDsaSk::<MlDsa65>::decode(&enc);
        let sig = sk.sign(msg);
        Ok(sig.encode().as_slice().to_vec())
    }

    fn verify(&self, pubk: &[u8], msg: &[u8], sig: &[u8]) -> Result<bool, CryptoError> {
        let enc = ml_dsa::EncodedVerifyingKey::<MlDsa65>::try_from(pubk).map_err(|_| CryptoError::InvalidKey)?;
        let vk = MlDsaVk::<MlDsa65>::decode(&enc);
        let sig = MlDsaSig::<MlDsa65>::try_from(sig).map_err(|_| CryptoError::AuthFail)?;
        Ok(vk.verify(msg, &sig).is_ok())
    }
}

/// Ed25519 adapter where the 32-byte private key is treated as an Ed25519 seed.
struct Ed25519Det;

impl SigEd25519 for Ed25519Det {
    fn sign(&self, privk: &[u8], msg: &[u8]) -> Vec<u8> {
        if privk.len() != 32 {
            return vec![]; // fail-closed by producing an unusable signature
        }
        let mut seed = [0u8; 32];
        seed.copy_from_slice(privk);
        let sk = ed25519_dalek::SigningKey::from_bytes(&seed);
        sk.sign(msg).to_bytes().to_vec()
    }

    fn verify(&self, pubk: &[u8], msg: &[u8], sig: &[u8]) -> bool {
        let pubk: [u8; 32] = match pubk.try_into() {
            Ok(x) => x,
            Err(_) => return false,
        };
        let vk = match ed25519_dalek::VerifyingKey::from_bytes(&pubk) {
            Ok(v) => v,
            Err(_) => return false,
        };
        let sig = match ed25519_dalek::Signature::from_slice(sig) {
            Ok(s) => s,
            Err(_) => return false,
        };
        vk.verify_strict(msg, &sig).is_ok()
    }
}

/// KT verifier for harness execution.
///
/// This implementation only accepts the "KT disabled" shape:
/// - kt_log_id == all-zero
/// - kt_sth / proofs are empty
///
/// If any KT material is present, it fail-closes with NotImplemented.
struct KtAllowEmptyOnly;

impl KtVerifier for KtAllowEmptyOnly {
    fn verify_bundle(
        &self,
        kt_log_id: &[u8; 32],
        kt_sth: &[u8],
        kt_inclusion_proof: &[u8],
        kt_consistency_proof: &[u8],
    ) -> Result<(), KtError> {
        let all_zero = kt_log_id.iter().all(|&b| b == 0);
        if all_zero && kt_sth.is_empty() && kt_inclusion_proof.is_empty() && kt_consistency_proof.is_empty() {
            Ok(())
        } else {
            Err(KtError::NotImplemented)
        }
    }
}

// ---------------------------
// Deterministic identity + prekey derivation
// ---------------------------

#[derive(Clone)]
struct StaticKeys {
    // Identity keys
    ik_sig_ec_seed: [u8; 32],
    ik_sig_ec_pub: [u8; 32],
    ik_sig_pq_priv: Vec<u8>,
    ik_sig_pq_pub: Vec<u8>,

    // Signed prekeys
    spk_dh_priv: X25519Priv,
    spk_dh_pub: [u8; 32],
    spk_pq_priv: Vec<u8>,
    spk_pq_pub: Vec<u8>,
}

fn gen_static_keys(name: &str, seed: &str) -> Result<StaticKeys, ActorError> {
    // Ed25519 seed
    let ik_seed = derive_seed32("IK_EC", name, seed);
    let ik_sk = ed25519_dalek::SigningKey::from_bytes(&ik_seed);
    let ik_pk = ik_sk.verifying_key().to_bytes();

    // ML-DSA-65 deterministic keygen (seeded RNG)
    let pq_rng_seed = derive_seed32("IK_PQ", name, seed);
    let mut pq_rng = ChaCha20Rng::from_seed(pq_rng_seed);
    let kp = MlDsa65::key_gen(&mut pq_rng);
    let ik_pq_priv = kp.signing_key().encode().as_slice().to_vec();
    let ik_pq_pub = kp.verifying_key().encode().as_slice().to_vec();

    // X25519 signed prekey
    let spk_dh_seed = derive_seed32("SPK_DH", name, seed);
    let spk_dh_sk = x25519_dalek::StaticSecret::from(spk_dh_seed);
    let spk_dh_pk = x25519_dalek::PublicKey::from(&spk_dh_sk);

    // ML-KEM-768 deterministic keygen (d,z)
    let d = B32::from(derive_seed32("SPK_PQ_D", name, seed));
    let z = B32::from(derive_seed32("SPK_PQ_Z", name, seed));
    let (dk, ek) = MlKem768::generate_deterministic(&d, &z);

    Ok(StaticKeys {
        ik_sig_ec_seed: ik_seed,
        ik_sig_ec_pub: ik_pk,
        ik_sig_pq_priv: ik_pq_priv,
        ik_sig_pq_pub: ik_pq_pub,
        spk_dh_priv: X25519Priv(spk_dh_sk.to_bytes()),
        spk_dh_pub: spk_dh_pk.to_bytes(),
        spk_pq_priv: dk.as_bytes().as_slice().to_vec(),
        spk_pq_pub: ek.as_bytes().as_slice().to_vec(),
    })
}


const BUNDLE_TBS_LABEL: &[u8] = b"QSP4.3/BUNDLE";

fn push_u16_be(out: &mut Vec<u8>, x: u16) {
    out.extend_from_slice(&x.to_be_bytes());
}

fn push_u32_be(out: &mut Vec<u8>, x: u32) {
    out.extend_from_slice(&x.to_be_bytes());
}

/// Compute BundleTBS per QSP §4.3.2:
/// BundleTBS = SHA-512("QSP4.3/BUNDLE" || suite_id || canonical bundle fields up to OPK fields).
fn bundle_tbs_hash(bundle: &PrekeyBundle) -> [u8; 64] {
    let mut preimage: Vec<u8> = Vec::with_capacity(2048);

    // suite_id
    push_u16_be(&mut preimage, bundle.suite_id);

    // user_id
    let user_id = &bundle.user_id;
    push_u16_be(&mut preimage, u16::try_from(user_id.len()).unwrap_or(u16::MAX));
    preimage.extend_from_slice(user_id);

    // device_id, validity
    push_u32_be(&mut preimage, bundle.device_id);
    push_u32_be(&mut preimage, bundle.valid_from);
    push_u32_be(&mut preimage, bundle.valid_to);

    // IK signature public keys
    preimage.extend_from_slice(&bundle.ik_sig_ec_pub);
    push_u16_be(
        &mut preimage,
        u16::try_from(bundle.ik_sig_pq_pub.len()).unwrap_or(u16::MAX),
    );
    preimage.extend_from_slice(&bundle.ik_sig_pq_pub);

    // SPK public keys
    preimage.extend_from_slice(&bundle.spk_dh_pub);
    push_u16_be(
        &mut preimage,
        u16::try_from(bundle.spk_pq_pub.len()).unwrap_or(u16::MAX),
    );
    preimage.extend_from_slice(&bundle.spk_pq_pub);

    // PQ receiver
    push_u32_be(&mut preimage, bundle.pq_rcv_id);
    push_u16_be(
        &mut preimage,
        u16::try_from(bundle.pq_rcv_pub.len()).unwrap_or(u16::MAX),
    );
    preimage.extend_from_slice(&bundle.pq_rcv_pub);

    // OPK (optional)
    match &bundle.opk_dh_pub {
        None => preimage.push(0u8),
        Some(pk) => {
            preimage.push(1u8);
            preimage.extend_from_slice(pk);
        }
    }

    // OPK PQ optional
    let opk_pq_present = bundle.opk_pq_id.is_some() && bundle.opk_pq_pub.is_some();
    if !opk_pq_present {
        preimage.push(0u8);
    } else {
        preimage.push(1u8);
        push_u32_be(&mut preimage, bundle.opk_pq_id.unwrap());
        let pk = bundle.opk_pq_pub.as_ref().unwrap();
        push_u16_be(&mut preimage, u16::try_from(pk.len()).unwrap_or(u16::MAX));
        preimage.extend_from_slice(pk);
    }

    let mut h = Sha512::new();
    h.update(BUNDLE_TBS_LABEL);
    h.update(&preimage);
    let out = h.finalize();

    let mut ret = [0u8; 64];
    ret.copy_from_slice(&out[..]);
    ret
}

fn build_prekey_bundle_for(peer: &StaticKeys, peer_name: &str, peer_device_id: u32) -> PrekeyBundle {
    let user_id = peer_name.as_bytes().to_vec();

    // Build bundle with placeholder bundle signatures; we compute sigs over BundleTBS (excluding sig fields).
    let mut bundle = PrekeyBundle {
        protocol_version: QSP_PROTOCOL_VERSION,
        suite_id: peer.suite_id,

        user_id,
        device_id: peer_device_id,
        valid_from: 1,
        valid_to: 2,

        ik_sig_ec_pub: peer.ik_sig_ec_pub,
        ik_sig_pq_pub: peer.ik_sig_pq_pub.clone(),

        spk_dh_pub: peer.spk_dh_pub,
        spk_pq_pub: peer.spk_pq_pub.clone(),

        pq_rcv_id: peer.pq_rcv_id,
        pq_rcv_pub: peer.pq_rcv_pub.clone(),

        opk_dh_pub: None,
        opk_pq_id: None,
        opk_pq_pub: None,

        // Signed-prekey signatures (by IK) are included and may be bound into handshake transcripts.
        spk_dh_sig_ec: peer.spk_dh_sig_ec.clone(),
        spk_dh_sig_pq: peer.spk_dh_sig_pq.clone(),
        spk_pq_sig_ec: peer.spk_pq_sig_ec.clone(),
        spk_pq_sig_pq: peer.spk_pq_sig_pq.clone(),

        // Bundle signatures (computed below)
        sig_ec: Vec::new(),
        sig_pq: Vec::new(),

        kt_sth: None,
        kt_audit_path: None,
    };

    let tbs = bundle_tbs_hash(&bundle);

    // Bundle EC signature (Ed25519 over BundleTBS hash)
    let ed = Ed25519Det {};
    bundle.sig_ec = ed.sign(&peer.ik_sig_ec_seed, &tbs);

    // Bundle PQ signature (ML-DSA-65 over BundleTBS hash)
    let pq = MlDsaDet {};
    bundle.sig_pq = pq.sign(&peer.ik_sig_pq_priv, &tbs);

    bundle
}


fn derive_peer_name(self_name: &str) -> String {
    // Harness wires impl_a <-> impl_b.
    if self_name.ends_with("_a") {
        format!("{}_b", self_name.trim_end_matches("_a"))
    } else if self_name.ends_with("_b") {
        format!("{}_a", self_name.trim_end_matches("_b"))
    } else {
        // Fallback: deterministic but explicit.
        format!("{self_name}_peer")
    }
}

// ---------------------------
// Actor runtime state
// ---------------------------

struct PendingInit {
    init: InitiatorState,
    dh0_a: (X25519Priv, X25519Pub),
    pq_rcv_a_priv: Vec<u8>,
}

struct SessionEntry {
    st: SessionState,
    rng12: Rng12Det,
}

struct Actor {
    name: String,
    ci: bool,
    seed: String,

    std: StdCrypto,
    dh: DhDet,
    pq_kem: MlKemDet,
    pq_sig: MlDsaDet,
    ed: Ed25519Det,
    kt: KtAllowEmptyOnly,

    sid_rng: ChaCha20Rng,
    static_keys: StaticKeys,

    pending: HashMap<[u8; 16], PendingInit>,
    sessions: HashMap<[u8; 16], SessionEntry>,
}

impl Actor {
    fn new(name: String, ci: bool) -> Result<Self, ActorError> {
        let seed = if ci { "ci-default".to_string() } else { "local-default".to_string() };

        let static_keys = gen_static_keys(&name, &seed)?;

        Ok(Self {
            dh: DhDet::new(derive_seed32("DH", &name, &seed)),
            sid_rng: ChaCha20Rng::from_seed(derive_seed32("SID", &name, &seed)),
            std: StdCrypto,
            pq_kem: MlKemDet::new(),
            pq_sig: MlDsaDet,
            ed: Ed25519Det,
            kt: KtAllowEmptyOnly,
            name,
            ci,
            seed,
            static_keys,
            pending: HashMap::new(),
            sessions: HashMap::new(),
        })
    }

    fn deps(&self) -> HandshakeDeps<'_> {
        HandshakeDeps {
            hash: &self.std,
            kmac: &self.std,
            aead: &self.std,
            dh: &self.dh,
            pq_kem: &self.pq_kem,
            pq_sig: &self.pq_sig,
            ed25519: &self.ed,
            kt: &self.kt,
        }
    }

    fn reset(&mut self, seed_opt: Option<String>) -> Result<(), ActorError> {
        if let Some(s) = seed_opt {
            self.seed = s;
        } else {
            self.seed = if self.ci { "ci-default".to_string() } else { "local-default".to_string() };
        }

        self.static_keys = gen_static_keys(&self.name, &self.seed)?;
        self.dh = DhDet::new(derive_seed32("DH", &self.name, &self.seed));
        self.pq_kem = MlKemDet::new();
        self.sid_rng = ChaCha20Rng::from_seed(derive_seed32("SID", &self.name, &self.seed));

        self.pending.clear();
        self.sessions.clear();
        Ok(())
    }

    fn new_session_id(&mut self) -> [u8; 16] {
        let mut sid = [0u8; 16];
        self.sid_rng.fill_bytes(&mut sid);
        sid
    }

    fn derive_session_dh0(&self, session_id: &[u8; 16]) -> (X25519Priv, X25519Pub) {
        let mut v = Vec::new();
        v.extend_from_slice(b"DH0");
        v.extend_from_slice(session_id);
        v.extend_from_slice(self.name.as_bytes());
        v.extend_from_slice(self.seed.as_bytes());
        let sk32 = sha3_256(&v);
        let sk = x25519_dalek::StaticSecret::from(sk32);
        let pk = x25519_dalek::PublicKey::from(&sk);
        (X25519Priv(sk.to_bytes()), X25519Pub(pk.to_bytes()))
    }

    fn derive_session_pq_rcv(&self, session_id: &[u8; 16]) -> (u32, Vec<u8>, Vec<u8>) {
        // Deterministic keygen inputs (d,z) derived from session id.
        let mut d_in = Vec::new();
        d_in.extend_from_slice(b"PQ_RCV_D");
        d_in.extend_from_slice(session_id);
        d_in.extend_from_slice(self.name.as_bytes());
        d_in.extend_from_slice(self.seed.as_bytes());
        let d = B32::from(sha3_256(&d_in));

        let mut z_in = Vec::new();
        z_in.extend_from_slice(b"PQ_RCV_Z");
        z_in.extend_from_slice(session_id);
        z_in.extend_from_slice(self.name.as_bytes());
        z_in.extend_from_slice(self.seed.as_bytes());
        let z = B32::from(sha3_256(&z_in));

        let (dk, ek) = MlKem768::generate_deterministic(&d, &z);
        let id = 1; // deterministic placeholder
        (id, ek.as_bytes().as_slice().to_vec(), dk.as_bytes().as_slice().to_vec())
    }

    fn rng12_seed_for_session(&self, session_id: &[u8; 16]) -> [u8; 32] {
        let sid = session_id_to_string(session_id);
        derive_seed32("NONCE12", &format!("{}:{sid}", self.name), &self.seed)
    }

    fn handle_capabilities(&self) -> serde_json::Value {
        serde_json::json!({
            "actor": "refimpl",
            "name": self.name,
            "mode": if self.ci { "ci" } else { "local" },
            "suites": ["Suite-1", "Suite-1B"],
            "ops": [
                "capabilities",
                "ping",
                "reset",
                "handshake_init",
                "handshake_respond",
                "handshake_finish",
                "encrypt",
                "decrypt"
            ]
        })
    }

    fn handle_handshake_init(&mut self, suite: &str) -> Result<serde_json::Value, ActorError> {
        if suite != "Suite-1" && suite != "Suite-1B" {
            return Err(ActorError::Unsupported(format!("unsupported suite: {suite}")));
        }

        let peer_name = derive_peer_name(&self.name);
        let peer_keys = gen_static_keys(&peer_name, &self.seed)?;
        let bundle_b = build_prekey_bundle_for(&peer_keys, &peer_name, 1);

        let session_id = self.new_session_id();

        // Session-specific A keys: DH0_A and PQ_RCV_A
        let dh0_a = self.derive_session_dh0(&session_id);
        let (pq_rcv_a_id, pq_rcv_a_pub, pq_rcv_a_priv) = self.derive_session_pq_rcv(&session_id);

        let deps = self.deps();
        let (hs1, init) = initiator_build(
            &deps,
            &bundle_b,
            // A identity
            self.name.as_bytes().to_vec(),
            1,
            session_id,
            // IK(A)
            self.static_keys.ik_sig_ec_pub,
            self.static_keys.ik_sig_ec_seed.to_vec(),
            self.static_keys.ik_sig_pq_pub.clone(),
            self.static_keys.ik_sig_pq_priv.clone(),
            // PQ receive for A
            pq_rcv_a_id,
            pq_rcv_a_pub.clone(),
        )
        .map_err(|e| ActorError::Crypto(format!("handshake_init failed: {e}")))?;

        self.pending.insert(session_id, PendingInit { init, dh0_a, pq_rcv_a_priv });

        let msg1 = hs1.encode();
        Ok(serde_json::json!({
            "session_id": session_id_to_string(&session_id),
            "msg1_b64": b64u_encode(&msg1)
        }))
    }

    fn handle_handshake_respond(&mut self, msg1_b64: &str) -> Result<serde_json::Value, ActorError> {
        let msg1 = b64u_decode(msg1_b64)?;
        let hs1 = HandshakeInit::decode(&msg1).map_err(|e| ActorError::Invalid(format!("bad HS1: {e}")))?;

        // Session-specific B keys: DH0_B and PQ_RCV_B
        let dh0_b = self.derive_session_dh0(&hs1.session_id);
        let (pq_rcv_b_id, pq_rcv_b_pub, pq_rcv_b_priv) = self.derive_session_pq_rcv(&hs1.session_id);

        let deps = self.deps();
        let (hs2, st) = responder_process(
            &deps,
            &hs1,
            // IK(B)
            self.static_keys.ik_sig_ec_pub,
            self.static_keys.ik_sig_ec_seed.to_vec(),
            self.static_keys.ik_sig_pq_pub.clone(),
            self.static_keys.ik_sig_pq_priv.clone(),
            // SPK(B)
            self.static_keys.spk_dh_priv.clone(),
            self.static_keys.spk_pq_priv.clone(),
            // OPK not used in this skeleton
            None,
            None,
            // DH0_B + PQ_RCV_B
            dh0_b,
            pq_rcv_b_id,
            pq_rcv_b_pub,
            pq_rcv_b_priv,
        )
        .map_err(|e| ActorError::Crypto(format!("handshake_respond failed: {e}")))?;

        let rng12 = Rng12Det::new(self.rng12_seed_for_session(&hs1.session_id));
        self.sessions.insert(hs1.session_id, SessionEntry { st, rng12 });

        let msg2 = hs2.encode();
        Ok(serde_json::json!({ "msg2_b64": b64u_encode(&msg2) }))
    }

    fn handle_handshake_finish(&mut self, session_id_s: &str, msg2_b64: &str) -> Result<serde_json::Value, ActorError> {
        let session_id = session_id_from_string(session_id_s)?;
        let pending = self
            .pending
            .remove(&session_id)
            .ok_or_else(|| ActorError::Invalid("unknown session_id".into()))?;

        let msg2 = b64u_decode(msg2_b64)?;
        let hs2 = HandshakeResp::decode(&msg2).map_err(|e| ActorError::Invalid(format!("bad HS2: {e}")))?;

        let deps = self.deps();
        let st = initiator_finalize(&deps, pending.init, &hs2, pending.dh0_a, pending.pq_rcv_a_priv)
            .map_err(|e| ActorError::Crypto(format!("handshake_finish failed: {e}")))?;

        let rng12 = Rng12Det::new(self.rng12_seed_for_session(&session_id));
        self.sessions.insert(session_id, SessionEntry { st, rng12 });

        Ok(serde_json::json!({ "session_id": session_id_to_string(&session_id) }))
    }

    fn handle_encrypt(&mut self, session_id_s: &str, pt_b64: &str) -> Result<serde_json::Value, ActorError> {
        let session_id = session_id_from_string(session_id_s)?;
        let pt = b64u_decode(pt_b64)?;
        let entry = self
            .sessions
            .get_mut(&session_id)
            .ok_or_else(|| ActorError::Invalid("unknown session_id".into()))?;

        let msg = ratchet_encrypt(
            &mut entry.st,
            &self.std,
            &self.std,
            &self.std,
            &self.dh,
            &self.pq_kem,
            &mut entry.rng12,
            &pt,
            false,
            false,
        )
        .map_err(|e| ActorError::Crypto(format!("encrypt failed: {e}")))?;

        Ok(serde_json::json!({ "ciphertext_b64": b64u_encode(&msg.encode()) }))
    }

    fn handle_decrypt(&mut self, session_id_s: &str, ct_b64: &str) -> Result<serde_json::Value, ActorError> {
        let session_id = session_id_from_string(session_id_s)?;
        let ct = b64u_decode(ct_b64)?;
        let entry = self
            .sessions
            .get_mut(&session_id)
            .ok_or_else(|| ActorError::Invalid("unknown session_id".into()))?;

        let msg = ProtocolMessage::decode(&ct).map_err(|e| ActorError::Invalid(format!("bad protocol message: {e}")))?;
        let pt = ratchet_decrypt(&mut entry.st, &self.std, &self.std, &self.std, &self.dh, &self.pq_kem, &msg)
            .map_err(|e| ActorError::Crypto(format!("decrypt failed: {e}")))?;

        Ok(serde_json::json!({ "plaintext_b64": b64u_encode(&pt) }))
    }

    fn dispatch(&mut self, req: Req) -> Result<serde_json::Value, ActorError> {
        match req.op.as_str() {
            "capabilities" => Ok(self.handle_capabilities()),
            "ping" => Ok(serde_json::json!({ "name": self.name, "ci": self.ci })),
            "reset" => {
                let seed = req.params.get("seed").and_then(|v| v.as_str()).map(|s| s.to_string());
                self.reset(seed)?;
                Ok(serde_json::json!({ "reset": true }))
            }
            "handshake_init" => {
                let suite = req
                    .params
                    .get("suite")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ActorError::Invalid("missing params.suite".into()))?;
                self.handle_handshake_init(suite)
            }
            "handshake_respond" => {
                let msg1 = req
                    .params
                    .get("msg1_b64")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ActorError::Invalid("missing params.msg1_b64".into()))?;
                self.handle_handshake_respond(msg1)
            }
            "handshake_finish" => {
                let msg2 = req
                    .params
                    .get("msg2_b64")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ActorError::Invalid("missing params.msg2_b64".into()))?;

                // The 4B harness does not pass session_id to handshake_finish; it expects
                // the actor to track the pending initiator state.
                //
                // If the caller provides session_id, use it. Otherwise, if there is exactly
                // one pending initiator session, infer it deterministically. Fail-closed if
                // ambiguous.
                let sid_s: String = if let Some(s) = req.params.get("session_id").and_then(|v| v.as_str()) {
                    s.to_string()
                } else if self.pending.len() == 1 {
                    let only = self.pending.keys().next().unwrap();
                    session_id_to_string(only)
                } else {
                    return Err(ActorError::Invalid("missing params.session_id".into()));
                };

                self.handle_handshake_finish(&sid_s, msg2)
            }
            "encrypt" => {
                let sid = req
                    .params
                    .get("session_id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ActorError::Invalid("missing params.session_id".into()))?;
                let pt = req
                    .params
                    .get("plaintext_b64")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ActorError::Invalid("missing params.plaintext_b64".into()))?;
                self.handle_encrypt(sid, pt)
            }
            "decrypt" => {
                let sid = req
                    .params
                    .get("session_id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ActorError::Invalid("missing params.session_id".into()))?;
                let ct = req
                    .params
                    .get("ciphertext_b64")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ActorError::Invalid("missing params.ciphertext_b64".into()))?;
                self.handle_decrypt(sid, ct)
            }
            other => Err(ActorError::Unsupported(other.to_string())),
        }
    }
}

// ---------------------------
// main
// ---------------------------

fn main() {
    let cli = Cli::parse();

    let mut actor = match Actor::new(cli.name, cli.ci) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("failed to start actor: {e}");
            std::process::exit(2);
        }
    };

    let stdin = io::stdin();
    let mut stdout = io::BufWriter::new(io::stdout());

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        if line.trim().is_empty() {
            continue;
        }

        let parsed: Result<Req, _> = serde_json::from_str(&line);
        let req = match parsed {
            Ok(r) => r,
            Err(e) => {
                // If we can't parse the request, we can't extract an id. Emit a generic error.
                let out = serde_json::json!({
                    "id": "__parse__",
                    "ok": false,
                    "error": {"code": "INVALID", "message": format!("bad json: {e}")}
                });
                let _ = writeln!(stdout, "{}", out.to_string());
                let _ = stdout.flush();
                continue;
            }
        };

        let id = req.id.clone();
        let resp = match actor.dispatch(req) {
            Ok(result) => serde_json::to_string(&RespOk { id, ok: true, result }).unwrap(),
            Err(err) => {
                let out = RespErr {
                    id,
                    ok: false,
                    error: ErrObj {
                        code: err.code().to_string(),
                        message: err.to_string(),
                    },
                };
                serde_json::to_string(&out).unwrap()
            }
        };

        let _ = writeln!(stdout, "{}", resp);
        let _ = stdout.flush();
    }
}
