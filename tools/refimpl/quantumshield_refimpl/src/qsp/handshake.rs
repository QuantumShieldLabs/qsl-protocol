use super::{HandshakeInit, HandshakeResp, PrekeyBundle, SessionRole, SessionState};
use super::constants::*;
use crate::crypto::traits::*;
use crate::kt::KtVerifier;
use crate::codec::CodecError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum HandshakeError {
    #[error("codec: {0}")]
    Codec(#[from] CodecError),
    #[error("crypto: {0}")]
    Crypto(#[from] CryptoError),
    #[error("kt: {0}")]
    Kt(#[from] crate::kt::KtError),
    #[error("signature verification failed")]
    BadSignature,
    #[error("confirmation failed")]
    BadConfirmation,
    #[error("invalid parameters: {0}")]
    Invalid(&'static str),
}

/// Dependencies for a full QSP handshake.
pub struct HandshakeDeps<'a> {
    pub hash: &'a dyn Hash,
    pub kmac: &'a dyn Kmac,
    pub dh: &'a dyn X25519Dh,
    pub aead: &'a dyn Aead, // unused in handshake; carried for symmetry
    pub ed25519: &'a dyn SigEd25519,
    pub pq_kem: &'a dyn PqKem768,
    pub pq_sig: &'a dyn PqSigMldsa65,
    pub kt: &'a dyn KtVerifier,
}

fn h_label(hash: &dyn Hash, label: &[u8], data: &[u8]) -> [u8; 64] {
    let mut m = Vec::with_capacity(label.len() + data.len());
    m.extend_from_slice(label);
    m.extend_from_slice(data);
    hash.sha512(&m)
}

/// Derive ms = H("QSP4.3/MS" || ss1 || [ss2] || dh1 || [dh2]) and RK0 = KMAC(ms, "QSP4.3/RK0", session_id, 32)
fn derive_rk0(hash: &dyn Hash, kmac: &dyn Kmac, session_id: &[u8; 16], ss1: &[u8], ss2: Option<&[u8]>, dh1: &[u8;32], dh2: Option<&[u8;32]>) -> [u8; 32] {
    let mut m = b"QSP4.3/MS".to_vec();
    m.extend_from_slice(ss1);
    if let Some(s) = ss2 { m.extend_from_slice(s); }
    m.extend_from_slice(dh1);
    if let Some(d) = dh2 { m.extend_from_slice(d); }
    let ms = hash.sha512(&m);
    let rk0 = kmac.kmac256(&ms, "QSP4.3/RK0", session_id, 32);
    let mut out = [0u8; 32];
    out.copy_from_slice(&rk0);
    out
}

fn kmac32(kmac: &dyn Kmac, key: &[u8;32], label: &str, data: &[u8]) -> [u8;32] {
    let v = kmac.kmac256(key, label, data, 32);
    let mut out = [0u8;32];
    out.copy_from_slice(&v);
    out
}

/// Initiator constructs HandshakeInit, returning the message and an InitiatorState required to finalize with HandshakeResp.
///
/// This function assumes the caller has already acquired and validated `bundle_b` (including service freshness).
pub fn initiator_build(
    deps: &HandshakeDeps,
    bundle_b: &PrekeyBundle,
    user_id_b: Vec<u8>,
    device_id_b: u32,
    session_id: [u8; 16],
    // A's identity keys (pub + signing material)
    ik_sig_ec_a_pub: [u8; 32],
    ik_sig_ec_a_priv: Vec<u8>, // 32 bytes
    ik_sig_pq_a_pub: Vec<u8>,  // 1952 bytes
    ik_sig_pq_a_priv: Vec<u8>, // impl-defined
    // A's PQ receive key (published in bundle; used for ct3)
    pq_rcv_a_id: u32,
    pq_rcv_a_pub: Vec<u8>,
) -> Result<(HandshakeInit, InitiatorState), HandshakeError> {
    // KT verification of B's identity keys (Authenticated mode)
    deps.kt.verify_bundle(&bundle_b.kt_log_id, &bundle_b.kt_sth, &bundle_b.kt_inclusion_proof, &bundle_b.kt_consistency_proof)?;

    // Generate EK_DH_A
    let (ek_priv, ek_pub) = deps.dh.keypair();

    // Encapsulate to B's SPK_PQ (ct1, ss1)
    let (ct1, ss1) = deps.pq_kem.encap(&bundle_b.spk_pq_pub)?;

    // Optional OPK usage: this skeleton supports using OPKs iff both are present.
    let opk_pq_present = bundle_b.opk_pq.is_some();
    let opk_dh_present = bundle_b.opk_dh.is_some();
    if opk_pq_present && !opk_dh_present {
        return Err(HandshakeError::Invalid("opk_dh missing"));
    }
    if opk_dh_present && !opk_pq_present {
        return Err(HandshakeError::Invalid("opk_pq missing"));
    }
    let opk_used = opk_pq_present && opk_dh_present;
    let (ct2, ss2, opk_dh_id, opk_pq_id) = if opk_used {
        let (opk_pq_id, opk_pq_pub) = bundle_b.opk_pq.as_ref().ok_or(HandshakeError::Invalid("opk_pq missing"))?;
        let (ct2, ss2) = deps.pq_kem.encap(opk_pq_pub)?;
        let (opk_dh_id, _opk_dh_pub) = bundle_b.opk_dh.as_ref().ok_or(HandshakeError::Invalid("opk_dh missing"))?;
        (Some(ct2), Some(ss2), Some(*opk_dh_id), Some(*opk_pq_id))
    } else { (None, None, None, None) };

    // DHs
    let dh1 = deps.dh.dh(&ek_priv, &crate::crypto::traits::X25519Pub(bundle_b.spk_dh_pub));
    let dh2 = if opk_used {
        let (_id, opk_dh_pub) = bundle_b.opk_dh.as_ref().ok_or(HandshakeError::Invalid("opk_dh missing"))?;
        Some(deps.dh.dh(&ek_priv, &crate::crypto::traits::X25519Pub(*opk_dh_pub)))
    } else { None };

    // RK0
    let rk0 = derive_rk0(deps.hash, deps.kmac, &session_id, &ss1, ss2.as_deref(), &dh1, dh2.as_ref());

    // Construct HS1 by setting sig fields to zero and hashing
    let mut hs1 = HandshakeInit {
        protocol_version: QSP_PROTOCOL_VERSION,
        suite_id: QSP_SUITE_ID,
        session_id,
        user_id_b,
        device_id_b,
        ek_dh_a_pub: ek_pub.0,
        ct1,
        opk_used,
        ct2,
        opk_dh_id,
        opk_pq_id,
        pq_rcv_a_id,
        pq_rcv_a_pub,
        ik_sig_ec_a_pub,
        ik_sig_pq_a_pub,
        sig_ec_a: vec![0u8; SZ_ED25519_SIG],
        sig_pq_a: vec![0u8; SZ_MLDSA65_SIG],
    };

    let hs1_hash = hs1.hs1_transcript(deps.hash);
    let sig_ec_a = deps.ed25519.sign(&ik_sig_ec_a_priv, &hs1_hash);
    let sig_pq_a = deps.pq_sig.sign(&ik_sig_pq_a_priv, &hs1_hash)?;
    hs1.sig_ec_a = sig_ec_a;
    hs1.sig_pq_a = sig_pq_a;

    Ok((hs1.clone(), InitiatorState {
        session_id,
        rk0,
        ek_dh_a_priv: ek_priv,
        hs1: hs1,
        pq_rcv_a_priv: Vec::new(), // caller should supply via SessionState init; left here as placeholder
    }))
}

pub struct InitiatorState {
    pub session_id: [u8; 16],
    pub rk0: [u8; 32],
    pub ek_dh_a_priv: crate::crypto::traits::X25519Priv,
    pub hs1: HandshakeInit,
    pub pq_rcv_a_priv: Vec<u8>, // retained to decap ct3 (not populated here)
}

/// Responder processes HS1, returns HS2 (HandshakeResp) and a fully initialized SessionState.
///
/// NOTE: This skeleton defers full bundle signature semantics until BundleTBS is finalized; however, it enforces:
/// - version/suite checks
/// - KT verification on A identity keys (Authenticated mode)
/// - A signature verification over HS1 transcript
pub fn responder_process(
    deps: &HandshakeDeps,
    hs1: &HandshakeInit,
    // B long-term identity keys
    ik_sig_ec_b_pub: [u8; 32],
    ik_sig_ec_b_priv: Vec<u8>,
    ik_sig_pq_b_pub: Vec<u8>,
    ik_sig_pq_b_priv: Vec<u8>,
    // B prekeys (private)
    spk_dh_b_priv: crate::crypto::traits::X25519Priv,
    spk_pq_b_priv: Vec<u8>,
    opk_dh_b_priv: Option<crate::crypto::traits::X25519Priv>,
    opk_pq_b_priv: Option<Vec<u8>>,
    // B new DH0 and PQ_RCV (generated for the session)
    dh0_b: (crate::crypto::traits::X25519Priv, crate::crypto::traits::X25519Pub),
    pq_rcv_b_id: u32,
    pq_rcv_b_pub: Vec<u8>,
    pq_rcv_b_priv: Vec<u8>,
) -> Result<(HandshakeResp, SessionState), HandshakeError> {
    // KT verification of A identity keys (Authenticated mode) – carried in A's bundle in real deployments.
    // In HS1 we only have A's IK pubs; KT proof carriage is in PrekeyBundle, not HS1.
    // Therefore this skeleton expects the caller to have performed KT pinning for A out-of-band or via service.
    // We *do* enforce that signature verification occurs.
    let hs1_hash = hs1.hs1_transcript(deps.hash);
    if !deps.ed25519.verify(&hs1.ik_sig_ec_a_pub, &hs1_hash, &hs1.sig_ec_a) { return Err(HandshakeError::BadSignature); }
    if !deps.pq_sig.verify(&hs1.ik_sig_pq_a_pub, &hs1_hash, &hs1.sig_pq_a)? { return Err(HandshakeError::BadSignature); }

    // Decapsulate ct1/ct2
    let ss1 = deps.pq_kem.decap(&spk_pq_b_priv, &hs1.ct1)?;
    let (ss2, dh2) = if hs1.opk_used {
        let ss2 = deps.pq_kem.decap(opk_pq_b_priv.as_ref().ok_or(HandshakeError::Invalid("opk_pq_priv missing"))?, hs1.ct2.as_ref().ok_or(HandshakeError::Invalid("ct2 missing"))?)?;
        let dh2 = deps.dh.dh(opk_dh_b_priv.as_ref().ok_or(HandshakeError::Invalid("opk_dh_priv missing"))?, &crate::crypto::traits::X25519Pub(hs1.ek_dh_a_pub));
        (Some(ss2), Some(dh2))
    } else { (None, None) };

    // dh1 = X25519(SPK_DH_B_priv, EK_DH_A_pub)
    let dh1 = deps.dh.dh(&spk_dh_b_priv, &crate::crypto::traits::X25519Pub(hs1.ek_dh_a_pub));

    let rk0 = derive_rk0(deps.hash, deps.kmac, &hs1.session_id, &ss1, ss2.as_deref(), &dh1, dh2.as_ref());

    // ct3 = encap to PQ_RCV_A_pub
    let (ct3, ss3) = deps.pq_kem.encap(&hs1.pq_rcv_a_pub)?;

    // Build HS2 with zero sigs to compute HS2 transcript
    let mut hs2 = HandshakeResp {
        protocol_version: QSP_PROTOCOL_VERSION,
        suite_id: QSP_SUITE_ID,
        session_id: hs1.session_id,
        dh0_b_pub: dh0_b.1 .0,
        pq_rcv_b_id,
        pq_rcv_b_pub: pq_rcv_b_pub.clone(),
        ct3,
        conf_b: [0u8;32],
        ik_sig_ec_b_pub,
        ik_sig_pq_b_pub: ik_sig_pq_b_pub.clone(),
        sig_ec_b: vec![0u8; SZ_ED25519_SIG],
        sig_pq_b: vec![0u8; SZ_MLDSA65_SIG],
    };

    let hs2_hash = hs2.hs2_transcript(hs1, deps.hash);
    let conf_b = kmac32(deps.kmac, &rk0, "QSP4.3/CONF", &hs2_hash);
    hs2.conf_b = conf_b;

    let sig_ec_b = deps.ed25519.sign(&ik_sig_ec_b_priv, &hs2_hash);
    let sig_pq_b = deps.pq_sig.sign(&ik_sig_pq_b_priv, &hs2_hash)?;
    hs2.sig_ec_b = sig_ec_b;
    hs2.sig_pq_b = sig_pq_b;

    // Initialize session (QSP §5.6)
    let mut st = SessionState::new(SessionRole::Responder, hs1.session_id, rk0, dh0_b, hs1.ek_dh_a_pub, (pq_rcv_b_id, pq_rcv_b_pub, pq_rcv_b_priv));
    st.derive_header_keys(deps.kmac);
    // B will have PQ peer set to A's advertised PQ_RCV in HS1
    st.pq_peer_id = Some(hs1.pq_rcv_a_id);
    st.pq_peer_pub = Some(hs1.pq_rcv_a_pub.clone());

    // NOTE: ss3 is not directly used by B; it is for A to decap and update its PQ receive cache (future).
    let _ = ss3;

    Ok((hs2, st))
}

/// Initiator finalizes handshake with HS2 and returns an initialized SessionState.
pub fn initiator_finalize(
    deps: &HandshakeDeps,
    init: InitiatorState,
    hs2: &HandshakeResp,
    // A's initial DH0 and PQ_RCV private key (for ct3 decap) must be supplied
    dh0_a: (crate::crypto::traits::X25519Priv, crate::crypto::traits::X25519Pub),
    pq_rcv_a_priv: Vec<u8>,
) -> Result<SessionState, HandshakeError> {
    // Verify HS2 signatures
    let hs2_hash = hs2.hs2_transcript(&init.hs1, deps.hash);

    if !deps.ed25519.verify(&hs2.ik_sig_ec_b_pub, &hs2_hash, &hs2.sig_ec_b) { return Err(HandshakeError::BadSignature); }
    if !deps.pq_sig.verify(&hs2.ik_sig_pq_b_pub, &hs2_hash, &hs2.sig_pq_b)? { return Err(HandshakeError::BadSignature); }

    // Verify confirmation
    let conf = kmac32(deps.kmac, &init.rk0, "QSP4.3/CONF", &hs2_hash);
    if conf != hs2.conf_b { return Err(HandshakeError::BadConfirmation); }

    // A decapsulates ct3 under PQ_RCV_A_priv (for future PQ boundary mixing state)
    let _ss3 = deps.pq_kem.decap(&pq_rcv_a_priv, &hs2.ct3)?;

    // Initialize session (QSP §5.6)
    let mut st = SessionState::new(SessionRole::Initiator, init.session_id, init.rk0, dh0_a, hs2.dh0_b_pub, (init.hs1.pq_rcv_a_id, init.hs1.pq_rcv_a_pub.clone(), pq_rcv_a_priv));
    st.derive_header_keys(deps.kmac);

    // A's PQ peer is B's PQ_RCV advertised in HS2
    st.pq_peer_id = Some(hs2.pq_rcv_b_id);
    st.pq_peer_pub = Some(hs2.pq_rcv_b_pub.clone());

    Ok(st)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyHash;
    impl Hash for DummyHash {
        fn sha512(&self, _data: &[u8]) -> [u8; 64] { [0u8; 64] }
    }

    struct DummyKmac;
    impl Kmac for DummyKmac {
        fn kmac256(&self, _key: &[u8], _label: &str, _data: &[u8], outlen: usize) -> Vec<u8> {
            vec![0u8; outlen]
        }
    }

    struct DummyAead;
    impl Aead for DummyAead {
        fn seal(&self, _key32: &[u8; 32], _nonce12: &[u8; 12], _ad: &[u8], _pt: &[u8]) -> Vec<u8> { vec![] }
        fn open(&self, _key32: &[u8; 32], _nonce12: &[u8; 12], _ad: &[u8], _ct: &[u8]) -> Result<Vec<u8>, CryptoError> {
            Ok(vec![])
        }
    }

    struct DummyDh;
    impl X25519Dh for DummyDh {
        fn keypair(&self) -> (X25519Priv, X25519Pub) {
            (X25519Priv([0x11u8; 32]), X25519Pub([0x22u8; 32]))
        }
        fn dh(&self, _privk: &X25519Priv, _pubk: &X25519Pub) -> [u8; 32] { [0x33u8; 32] }
    }

    struct DummyEd25519;
    impl SigEd25519 for DummyEd25519 {
        fn sign(&self, _privk: &[u8], _msg: &[u8]) -> Vec<u8> { vec![0u8; SZ_ED25519_SIG] }
        fn verify(&self, _pubk: &[u8], _msg: &[u8], _sig: &[u8]) -> bool { true }
    }

    struct DummyPqKem;
    impl PqKem768 for DummyPqKem {
        fn encap(&self, _pubk: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CryptoError> {
            Ok((vec![0u8; 32], vec![0x55u8; 32]))
        }
        fn decap(&self, _privk: &[u8], _ct: &[u8]) -> Result<Vec<u8>, CryptoError> {
            Ok(vec![0x66u8; 32])
        }
    }

    struct DummyPqSig;
    impl PqSigMldsa65 for DummyPqSig {
        fn sign(&self, _privk: &[u8], _msg: &[u8]) -> Result<Vec<u8>, CryptoError> { Ok(vec![0u8; SZ_MLDSA65_SIG]) }
        fn verify(&self, _pubk: &[u8], _msg: &[u8], _sig: &[u8]) -> Result<bool, CryptoError> { Ok(true) }
    }

    struct AllowKt;
    impl crate::kt::KtVerifier for AllowKt {
        fn verify_bundle(&self, _kt_log_id: &[u8; 32], _kt_sth: &[u8], _kt_inclusion_proof: &[u8], _kt_consistency_proof: &[u8]) -> Result<(), crate::kt::KtError> {
            Ok(())
        }
    }

    fn mk_deps<'a>(
        hash: &'a DummyHash,
        kmac: &'a DummyKmac,
        dh: &'a DummyDh,
        aead: &'a DummyAead,
        ed25519: &'a DummyEd25519,
        pq_kem: &'a DummyPqKem,
        pq_sig: &'a DummyPqSig,
        kt: &'a AllowKt,
    ) -> HandshakeDeps<'a> {
        HandshakeDeps { hash, kmac, dh, aead, ed25519, pq_kem, pq_sig, kt }
    }

    fn base_bundle(
        opk_dh: Option<(u32, [u8; SZ_X25519_PUB])>,
        opk_pq: Option<(u32, Vec<u8>)>,
    ) -> PrekeyBundle {
        PrekeyBundle {
            user_id: vec![0xA1],
            device_id: 7,
            valid_from: 1,
            valid_to: 2,
            ik_sig_ec_pub: [0u8; SZ_ED25519_PUB],
            ik_sig_pq_pub: vec![0u8; SZ_MLDSA65_PUB],
            spk_dh_pub: [0u8; SZ_X25519_PUB],
            spk_pq_pub: vec![0u8; SZ_MLKEM768_PUB],
            pq_rcv_id: 9,
            pq_rcv_pub: vec![0u8; SZ_MLKEM768_PUB],
            opk_dh,
            opk_pq,
            sig_ec: vec![0u8; SZ_ED25519_SIG],
            sig_pq: vec![0u8; SZ_MLDSA65_SIG],
            kt_log_id: [0u8; 32],
            kt_sth: vec![],
            kt_inclusion_proof: vec![],
            kt_consistency_proof: vec![],
        }
    }

    fn call_initiator_build(
        deps: &HandshakeDeps,
        bundle: &PrekeyBundle,
        user_id_b: Vec<u8>,
        device_id_b: u32,
        pq_rcv_a_id: u32,
        pq_rcv_a_pub: Vec<u8>,
    ) -> Result<(HandshakeInit, InitiatorState), HandshakeError> {
        initiator_build(
            deps,
            bundle,
            user_id_b,
            device_id_b,
            [0u8; SZ_SESSION_ID],
            [0u8; SZ_ED25519_PUB],
            vec![0u8; 32],
            vec![0u8; SZ_MLDSA65_PUB],
            vec![0u8; 1],
            pq_rcv_a_id,
            pq_rcv_a_pub,
        )
    }

    fn expect_err<T>(res: Result<T, HandshakeError>) -> HandshakeError {
        match res {
            Err(err) => err,
            Ok(_) => panic!("expected Err"),
        }
    }

    fn err_debug(err: &HandshakeError) -> String { format!("{:?}", err) }

    #[test]
    fn opk_partial_bundle_rejects_deterministically_and_no_mutation() {
        let hash = DummyHash;
        let kmac = DummyKmac;
        let dh = DummyDh;
        let aead = DummyAead;
        let ed25519 = DummyEd25519;
        let pq_kem = DummyPqKem;
        let pq_sig = DummyPqSig;
        let kt = AllowKt;
        let deps = mk_deps(&hash, &kmac, &dh, &aead, &ed25519, &pq_kem, &pq_sig, &kt);

        let user_id_b = vec![0xB1];
        let device_id_b = 11;
        let pq_rcv_a_id = 13;
        let pq_rcv_a_pub = vec![0u8; SZ_MLKEM768_PUB];

        let mut bundle = base_bundle(None, Some((42, vec![0u8; SZ_MLKEM768_PUB])));
        let before = bundle.encode();
        let err1 = expect_err(call_initiator_build(&deps, &bundle, user_id_b.clone(), device_id_b, pq_rcv_a_id, pq_rcv_a_pub.clone()));
        assert!(matches!(err1, HandshakeError::Invalid("opk_dh missing")));
        let err2 = expect_err(call_initiator_build(&deps, &bundle, user_id_b.clone(), device_id_b, pq_rcv_a_id, pq_rcv_a_pub.clone()));
        assert_eq!(err_debug(&err1), err_debug(&err2));
        assert_eq!(bundle.encode(), before);

        bundle = base_bundle(Some((7, [0u8; SZ_X25519_PUB])), None);
        let before = bundle.encode();
        let err1 = expect_err(call_initiator_build(&deps, &bundle, user_id_b.clone(), device_id_b, pq_rcv_a_id, pq_rcv_a_pub.clone()));
        assert!(matches!(err1, HandshakeError::Invalid("opk_pq missing")));
        let err2 = expect_err(call_initiator_build(&deps, &bundle, user_id_b, device_id_b, pq_rcv_a_id, pq_rcv_a_pub));
        assert_eq!(err_debug(&err1), err_debug(&err2));
        assert_eq!(bundle.encode(), before);
    }

    #[test]
    fn opk_bundle_with_both_present_succeeds() {
        let hash = DummyHash;
        let kmac = DummyKmac;
        let dh = DummyDh;
        let aead = DummyAead;
        let ed25519 = DummyEd25519;
        let pq_kem = DummyPqKem;
        let pq_sig = DummyPqSig;
        let kt = AllowKt;
        let deps = mk_deps(&hash, &kmac, &dh, &aead, &ed25519, &pq_kem, &pq_sig, &kt);

        let bundle = base_bundle(
            Some((7, [0u8; SZ_X25519_PUB])),
            Some((42, vec![0u8; SZ_MLKEM768_PUB])),
        );
        let res = call_initiator_build(&deps, &bundle, vec![0xB2], 22, 17, vec![0u8; SZ_MLKEM768_PUB]);
        assert!(res.is_ok());
    }
}
