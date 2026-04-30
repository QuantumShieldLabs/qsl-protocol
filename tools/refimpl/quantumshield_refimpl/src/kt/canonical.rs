use super::{KtError, KtVerification, KtVerifier};
use crate::crypto::traits::{PqSigMldsa65, SigEd25519};
use crate::qsp::{HandshakeInit, PrekeyBundle};

use sha2::{Digest, Sha256, Sha512};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AcceptedSth {
    pub tree_size: u64,
    pub root_hash: [u8; 32],
    pub timestamp_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KtTimeSource {
    System,
    Fixed(u64),
}

impl KtTimeSource {
    fn now_ms(self) -> u64 {
        match self {
            Self::System => SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0),
            Self::Fixed(now_ms) => now_ms,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KtPinnedLog {
    pub log_id: [u8; 32],
    pub verifying_key: [u8; 32],
    pub proof_cache_ttl_seconds: u64,
}

#[derive(Debug)]
pub struct CanonicalKtVerifier {
    pinned_logs: HashMap<[u8; 32], KtPinnedLog>,
    allow_disabled_nonproduction: bool,
    time_source: KtTimeSource,
    accepted: Mutex<HashMap<[u8; 32], AcceptedSth>>,
}

#[derive(Debug, Clone, Copy)]
struct SthBlob {
    version: u8,
    log_id: [u8; 32],
    tree_size: u64,
    timestamp_ms: u64,
    root_hash: [u8; 32],
    signature: [u8; 64],
}

#[derive(Debug, Clone)]
struct InclusionProof {
    leaf_index: u64,
    tree_size: u64,
    siblings: Vec<[u8; 32]>,
}

#[derive(Debug, Clone)]
struct ConsistencyProof {
    from_tree_size: u64,
    to_tree_size: u64,
    nodes: Vec<[u8; 32]>,
}

#[derive(Debug, Clone, Copy)]
struct BundleEvaluation {
    status: KtVerification,
    state_update: Option<([u8; 32], AcceptedSth)>,
}

struct BlobReader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> BlobReader<'a> {
    fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    fn take(&mut self, n: usize, detail: &'static str) -> Result<&'a [u8], KtError> {
        if self.buf.len().saturating_sub(self.pos) < n {
            return Err(KtError::kt_fail(detail));
        }
        let out = &self.buf[self.pos..self.pos + n];
        self.pos += n;
        Ok(out)
    }

    fn read_u8(&mut self, detail: &'static str) -> Result<u8, KtError> {
        Ok(self.take(1, detail)?[0])
    }

    fn read_u16(&mut self, detail: &'static str) -> Result<u16, KtError> {
        let bytes = self.take(2, detail)?;
        Ok(u16::from_be_bytes([bytes[0], bytes[1]]))
    }

    fn read_u64(&mut self, detail: &'static str) -> Result<u64, KtError> {
        let bytes = self.take(8, detail)?;
        Ok(u64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }

    fn read_exact<const N: usize>(&mut self, detail: &'static str) -> Result<[u8; N], KtError> {
        let bytes = self.take(N, detail)?;
        let mut out = [0u8; N];
        out.copy_from_slice(bytes);
        Ok(out)
    }

    fn finish(&self, detail: &'static str) -> Result<(), KtError> {
        if self.pos == self.buf.len() {
            Ok(())
        } else {
            Err(KtError::kt_fail(detail))
        }
    }
}

impl CanonicalKtVerifier {
    pub fn new<I>(
        pinned_logs: I,
        time_source: KtTimeSource,
        allow_disabled_nonproduction: bool,
    ) -> Self
    where
        I: IntoIterator<Item = KtPinnedLog>,
    {
        let mut logs = HashMap::new();
        for log in pinned_logs {
            logs.insert(log.log_id, log);
        }
        Self {
            pinned_logs: logs,
            allow_disabled_nonproduction,
            time_source,
            accepted: Mutex::new(HashMap::new()),
        }
    }

    pub fn disabled_nonproduction() -> Self {
        Self::new([], KtTimeSource::System, true)
    }

    fn evaluate_bundle(
        &self,
        bundle: &PrekeyBundle,
        ed25519: &dyn SigEd25519,
        pq_sig: &dyn PqSigMldsa65,
    ) -> Result<BundleEvaluation, KtError> {
        verify_bundle_signatures(bundle, ed25519, pq_sig)?;

        if bundle.kt_disabled_shape() {
            if self.allow_disabled_nonproduction {
                return Ok(BundleEvaluation {
                    status: KtVerification::DisabledNonProduction,
                    state_update: None,
                });
            }
            return Err(KtError::kt_fail("disabled_shape_rejected"));
        }

        let pinned = self
            .pinned_logs
            .get(&bundle.kt_log_id)
            .ok_or_else(|| KtError::kt_fail("unpinned_log_id"))?;
        let sth = parse_sth(&bundle.kt_sth)?;
        if sth.log_id != bundle.kt_log_id {
            return Err(KtError::kt_fail("sth_log_id_mismatch"));
        }

        verify_sth_signature(ed25519, pinned, &sth)?;
        verify_freshness(
            self.time_source.now_ms(),
            pinned.proof_cache_ttl_seconds,
            &sth,
        )?;
        verify_inclusion(
            bundle,
            &sth,
            &parse_inclusion_proof(&bundle.kt_inclusion_proof)?,
        )?;

        let state_update = self.evaluate_consistency(bundle, &sth)?;
        Ok(BundleEvaluation {
            status: KtVerification::Verified,
            state_update,
        })
    }

    fn evaluate_consistency(
        &self,
        bundle: &PrekeyBundle,
        sth: &SthBlob,
    ) -> Result<Option<([u8; 32], AcceptedSth)>, KtError> {
        let previous = self
            .accepted
            .lock()
            .map_err(|_| KtError::kt_fail("kt_state_poisoned"))?
            .get(&bundle.kt_log_id)
            .copied();

        let next = AcceptedSth {
            tree_size: sth.tree_size,
            root_hash: sth.root_hash,
            timestamp_ms: sth.timestamp_ms,
        };

        match previous {
            None => {
                if !bundle.kt_consistency_proof.is_empty() {
                    let proof = parse_consistency_proof(&bundle.kt_consistency_proof)?;
                    if proof.to_tree_size != sth.tree_size {
                        return Err(KtError::kt_fail("consistency_to_tree_size_mismatch"));
                    }
                }
                Ok(Some((bundle.kt_log_id, next)))
            }
            Some(prev) => {
                if sth.tree_size < prev.tree_size {
                    return Err(KtError::kt_fail("tree_size_regressed"));
                }

                if sth.tree_size == prev.tree_size {
                    if !bundle.kt_consistency_proof.is_empty() {
                        return Err(KtError::kt_fail("same_tree_requires_empty_consistency"));
                    }
                    if sth.root_hash != prev.root_hash {
                        return Err(KtError::kt_fail("same_tree_root_mismatch"));
                    }
                    if sth.timestamp_ms < prev.timestamp_ms {
                        return Err(KtError::kt_fail("same_tree_timestamp_regressed"));
                    }
                    return Ok(Some((bundle.kt_log_id, next)));
                }

                if bundle.kt_consistency_proof.is_empty() {
                    return Err(KtError::kt_fail("missing_consistency_proof"));
                }

                let proof = parse_consistency_proof(&bundle.kt_consistency_proof)?;
                if proof.from_tree_size != prev.tree_size {
                    return Err(KtError::kt_fail("consistency_from_tree_size_mismatch"));
                }
                if proof.to_tree_size != sth.tree_size {
                    return Err(KtError::kt_fail("consistency_to_tree_size_mismatch"));
                }
                if !verify_consistency_proof(
                    prev.tree_size,
                    sth.tree_size,
                    &prev.root_hash,
                    &sth.root_hash,
                    &proof.nodes,
                ) {
                    return Err(KtError::kt_fail("consistency_proof_invalid"));
                }
                Ok(Some((bundle.kt_log_id, next)))
            }
        }
    }

    fn commit(&self, candidate: Option<([u8; 32], AcceptedSth)>) -> Result<(), KtError> {
        if let Some((log_id, state)) = candidate {
            self.accepted
                .lock()
                .map_err(|_| KtError::kt_fail("kt_state_poisoned"))?
                .insert(log_id, state);
        }
        Ok(())
    }

    #[cfg(test)]
    fn accepted_state(&self, log_id: &[u8; 32]) -> Option<AcceptedSth> {
        self.accepted
            .lock()
            .ok()
            .and_then(|m| m.get(log_id).copied())
    }
}

impl KtVerifier for CanonicalKtVerifier {
    fn verify_bundle(
        &self,
        bundle: &PrekeyBundle,
        ed25519: &dyn SigEd25519,
        pq_sig: &dyn PqSigMldsa65,
    ) -> Result<KtVerification, KtError> {
        let evaluation = self.evaluate_bundle(bundle, ed25519, pq_sig)?;
        self.commit(evaluation.state_update)?;
        Ok(evaluation.status)
    }

    fn verify_responder_binding(
        &self,
        hs1: &HandshakeInit,
        initiator_bundle: Option<&PrekeyBundle>,
        ed25519: &dyn SigEd25519,
        pq_sig: &dyn PqSigMldsa65,
    ) -> Result<KtVerification, KtError> {
        let bundle =
            initiator_bundle.ok_or_else(|| KtError::kt_fail("missing_initiator_bundle"))?;
        let evaluation = self.evaluate_bundle(bundle, ed25519, pq_sig)?;
        if bundle.ik_sig_ec_pub != hs1.ik_sig_ec_a_pub {
            return Err(KtError::kt_fail("hs1_ik_sig_ec_mismatch"));
        }
        if bundle.ik_sig_pq_pub != hs1.ik_sig_pq_a_pub {
            return Err(KtError::kt_fail("hs1_ik_sig_pq_mismatch"));
        }
        if bundle.pq_rcv_id != hs1.pq_rcv_a_id {
            return Err(KtError::kt_fail("hs1_pq_rcv_id_mismatch"));
        }
        if bundle.pq_rcv_pub != hs1.pq_rcv_a_pub {
            return Err(KtError::kt_fail("hs1_pq_rcv_pub_mismatch"));
        }
        self.commit(evaluation.state_update)?;
        Ok(evaluation.status)
    }
}

fn verify_bundle_signatures(
    bundle: &PrekeyBundle,
    ed25519: &dyn SigEd25519,
    pq_sig: &dyn PqSigMldsa65,
) -> Result<(), KtError> {
    let digest = sha512(&bundle.bundle_tbs());
    if !ed25519.verify(&bundle.ik_sig_ec_pub, &digest, &bundle.sig_ec) {
        return Err(KtError::bundle_sig("sig_ec_verify_failed"));
    }
    let pq_ok = pq_sig
        .verify(&bundle.ik_sig_pq_pub, &digest, &bundle.sig_pq)
        .map_err(|_| KtError::bundle_sig("sig_pq_verify_error"))?;
    if !pq_ok {
        return Err(KtError::bundle_sig("sig_pq_verify_failed"));
    }
    Ok(())
}

fn verify_sth_signature(
    ed25519: &dyn SigEd25519,
    pinned: &KtPinnedLog,
    sth: &SthBlob,
) -> Result<(), KtError> {
    let mut msg = Vec::with_capacity(1 + 32 + 8 + 8 + 32 + 13);
    msg.extend_from_slice(b"QSL-KT/STH/v1");
    msg.push(sth.version);
    msg.extend_from_slice(&sth.log_id);
    msg.extend_from_slice(&sth.tree_size.to_be_bytes());
    msg.extend_from_slice(&sth.timestamp_ms.to_be_bytes());
    msg.extend_from_slice(&sth.root_hash);
    let digest = sha512(&msg);
    if ed25519.verify(&pinned.verifying_key, &digest, &sth.signature) {
        Ok(())
    } else {
        Err(KtError::kt_fail("sth_signature_verify_failed"))
    }
}

fn verify_freshness(now_ms: u64, ttl_seconds: u64, sth: &SthBlob) -> Result<(), KtError> {
    let max_age_ms = ttl_seconds.saturating_mul(1000);
    let age = now_ms.saturating_sub(sth.timestamp_ms);
    if age > max_age_ms {
        Err(KtError::kt_fail("sth_stale"))
    } else {
        Ok(())
    }
}

fn verify_inclusion(
    bundle: &PrekeyBundle,
    sth: &SthBlob,
    proof: &InclusionProof,
) -> Result<(), KtError> {
    if proof.leaf_index >= proof.tree_size {
        return Err(KtError::kt_fail("leaf_index_out_of_range"));
    }
    if proof.tree_size != sth.tree_size {
        return Err(KtError::kt_fail("inclusion_tree_size_mismatch"));
    }
    let leaf_hash = sha256_prefixed(0x00, &bundle.bundle_leaf_data());
    let root = compute_inclusion_root(leaf_hash, proof.leaf_index, &proof.siblings);
    if root == sth.root_hash {
        Ok(())
    } else {
        Err(KtError::kt_fail("inclusion_root_mismatch"))
    }
}

fn parse_sth(buf: &[u8]) -> Result<SthBlob, KtError> {
    let mut r = BlobReader::new(buf);
    let version = r.read_u8("sth_truncated")?;
    if version != 0x01 {
        return Err(KtError::kt_fail("sth_version"));
    }
    let log_id = r.read_exact::<32>("sth_truncated")?;
    let tree_size = r.read_u64("sth_truncated")?;
    let timestamp_ms = r.read_u64("sth_truncated")?;
    let root_hash = r.read_exact::<32>("sth_truncated")?;
    let signature = r.read_exact::<64>("sth_truncated")?;
    r.finish("sth_trailing_bytes")?;
    Ok(SthBlob {
        version,
        log_id,
        tree_size,
        timestamp_ms,
        root_hash,
        signature,
    })
}

fn parse_inclusion_proof(buf: &[u8]) -> Result<InclusionProof, KtError> {
    let mut r = BlobReader::new(buf);
    let version = r.read_u8("inclusion_truncated")?;
    if version != 0x01 {
        return Err(KtError::kt_fail("inclusion_version"));
    }
    let leaf_index = r.read_u64("inclusion_truncated")?;
    let tree_size = r.read_u64("inclusion_truncated")?;
    let sibling_count = r.read_u16("inclusion_truncated")?;
    let mut siblings = Vec::with_capacity(sibling_count as usize);
    for _ in 0..sibling_count {
        siblings.push(r.read_exact::<32>("inclusion_truncated")?);
    }
    r.finish("inclusion_trailing_bytes")?;
    Ok(InclusionProof {
        leaf_index,
        tree_size,
        siblings,
    })
}

fn parse_consistency_proof(buf: &[u8]) -> Result<ConsistencyProof, KtError> {
    let mut r = BlobReader::new(buf);
    let version = r.read_u8("consistency_truncated")?;
    if version != 0x01 {
        return Err(KtError::kt_fail("consistency_version"));
    }
    let from_tree_size = r.read_u64("consistency_truncated")?;
    if from_tree_size == 0 {
        return Err(KtError::kt_fail("consistency_from_tree_size_zero"));
    }
    let to_tree_size = r.read_u64("consistency_truncated")?;
    if from_tree_size > to_tree_size {
        return Err(KtError::kt_fail("consistency_size_regression"));
    }
    let node_count = r.read_u16("consistency_truncated")?;
    let mut nodes = Vec::with_capacity(node_count as usize);
    for _ in 0..node_count {
        nodes.push(r.read_exact::<32>("consistency_truncated")?);
    }
    r.finish("consistency_trailing_bytes")?;
    Ok(ConsistencyProof {
        from_tree_size,
        to_tree_size,
        nodes,
    })
}

fn compute_inclusion_root(
    mut current: [u8; 32],
    mut leaf_index: u64,
    siblings: &[[u8; 32]],
) -> [u8; 32] {
    for sibling in siblings {
        current = if (leaf_index & 1) == 1 {
            node_hash(sibling, &current)
        } else {
            node_hash(&current, sibling)
        };
        leaf_index >>= 1;
    }
    current
}

fn verify_consistency_proof(
    old_size: u64,
    new_size: u64,
    old_root: &[u8; 32],
    new_root: &[u8; 32],
    proof: &[[u8; 32]],
) -> bool {
    if old_size == 0 || old_size > new_size {
        return false;
    }
    if old_size == new_size {
        return proof.is_empty() && old_root == new_root;
    }
    if proof.is_empty() {
        return false;
    }

    let mut fn_idx = old_size - 1;
    let mut sn_idx = new_size - 1;
    while (fn_idx & 1) == 1 {
        fn_idx >>= 1;
        sn_idx >>= 1;
    }

    let mut fr = proof[0];
    let mut sr = proof[0];

    for node in &proof[1..] {
        if sn_idx == 0 {
            return false;
        }
        if (fn_idx & 1) == 1 || fn_idx == sn_idx {
            fr = node_hash(node, &fr);
            sr = node_hash(node, &sr);
            while fn_idx != 0 && (fn_idx & 1) == 0 {
                fn_idx >>= 1;
                sn_idx >>= 1;
            }
        } else {
            sr = node_hash(&sr, node);
        }
        fn_idx >>= 1;
        sn_idx >>= 1;
    }

    &fr == old_root && &sr == new_root
}

fn node_hash(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut data = Vec::with_capacity(65);
    data.push(0x01);
    data.extend_from_slice(left);
    data.extend_from_slice(right);
    sha256(&data)
}

fn sha256_prefixed(prefix: u8, data: &[u8]) -> [u8; 32] {
    let mut buf = Vec::with_capacity(1 + data.len());
    buf.push(prefix);
    buf.extend_from_slice(data);
    sha256(&buf)
}

fn sha256(data: &[u8]) -> [u8; 32] {
    let digest = Sha256::digest(data);
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest);
    out
}

fn sha512(data: &[u8]) -> [u8; 64] {
    let digest = Sha512::digest(data);
    let mut out = [0u8; 64];
    out.copy_from_slice(&digest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::stdcrypto::StdEd25519;
    use crate::crypto::traits::{CryptoError, PqSigMldsa65};
    use crate::qsp::{
        HandshakeInit, PrekeyBundle, SZ_ED25519_SIG, SZ_MLDSA65_PUB, SZ_MLDSA65_SIG,
        SZ_MLKEM768_PUB, SZ_X25519_PUB,
    };
    use ed25519_dalek::{Signer as _, SigningKey};
    use rand_core::{OsRng, RngCore};
    use std::sync::Arc;

    #[derive(Clone)]
    struct TestPqSig;

    impl TestPqSig {
        fn signature(pubk: &[u8], msg: &[u8]) -> Vec<u8> {
            let mut out = Vec::with_capacity(SZ_MLDSA65_SIG);
            let mut counter = 0u32;
            while out.len() < SZ_MLDSA65_SIG {
                let mut chunk = Vec::with_capacity(pubk.len() + msg.len() + 4);
                chunk.extend_from_slice(pubk);
                chunk.extend_from_slice(msg);
                chunk.extend_from_slice(&counter.to_be_bytes());
                out.extend_from_slice(&sha512(&chunk));
                counter = counter.wrapping_add(1);
            }
            out.truncate(SZ_MLDSA65_SIG);
            out
        }
    }

    impl PqSigMldsa65 for TestPqSig {
        fn sign(&self, privk: &[u8], msg: &[u8]) -> Result<Vec<u8>, CryptoError> {
            Ok(Self::signature(privk, msg))
        }

        fn verify(&self, pubk: &[u8], msg: &[u8], sig: &[u8]) -> Result<bool, CryptoError> {
            Ok(sig == Self::signature(pubk, msg))
        }
    }

    #[derive(Clone)]
    struct SigningFixture {
        bundle_seed: [u8; 32],
        log_seed: [u8; 32],
        pq_sig: Arc<TestPqSig>,
    }

    impl SigningFixture {
        fn new() -> Self {
            let mut bundle_seed = [0u8; 32];
            let mut log_seed = [0u8; 32];
            OsRng.fill_bytes(&mut bundle_seed);
            OsRng.fill_bytes(&mut log_seed);
            Self {
                bundle_seed,
                log_seed,
                pq_sig: Arc::new(TestPqSig),
            }
        }

        fn bundle_vk(&self) -> [u8; 32] {
            SigningKey::from_bytes(&self.bundle_seed)
                .verifying_key()
                .to_bytes()
        }

        fn log_vk(&self) -> [u8; 32] {
            SigningKey::from_bytes(&self.log_seed)
                .verifying_key()
                .to_bytes()
        }

        fn sign_bundle(&self, bundle: &mut PrekeyBundle) {
            let digest = sha512(&bundle.bundle_tbs());
            let bundle_sk = SigningKey::from_bytes(&self.bundle_seed);
            bundle.sig_ec = bundle_sk.sign(&digest).to_bytes().to_vec();
            bundle.sig_pq = TestPqSig::signature(&bundle.ik_sig_pq_pub, &digest);
        }

        fn build_bundle(
            &self,
            tree_size: u64,
            timestamp_ms: u64,
            siblings: Vec<[u8; 32]>,
            consistency_nodes: Vec<[u8; 32]>,
            consistency_from: Option<u64>,
        ) -> PrekeyBundle {
            let mut bundle = PrekeyBundle {
                user_id: b"alice".to_vec(),
                device_id: 7,
                valid_from: 1,
                valid_to: 9,
                ik_sig_ec_pub: self.bundle_vk(),
                ik_sig_pq_pub: vec![0x42; SZ_MLDSA65_PUB],
                spk_dh_pub: [0x11; SZ_X25519_PUB],
                spk_pq_pub: vec![0x22; SZ_MLKEM768_PUB],
                pq_rcv_id: 17,
                pq_rcv_pub: vec![0x33; SZ_MLKEM768_PUB],
                opk_dh: None,
                opk_pq: None,
                sig_ec: vec![0u8; SZ_ED25519_SIG],
                sig_pq: vec![0u8; SZ_MLDSA65_SIG],
                kt_log_id: [0xAB; 32],
                kt_sth: Vec::new(),
                kt_inclusion_proof: Vec::new(),
                kt_consistency_proof: Vec::new(),
            };

            let leaf_hash = sha256_prefixed(0x00, &bundle.bundle_leaf_data());
            let root_hash = compute_inclusion_root(leaf_hash, 0, &siblings);
            bundle.kt_inclusion_proof = build_inclusion_proof(0, tree_size, &siblings);
            bundle.kt_sth = build_sth(
                self.log_seed,
                bundle.kt_log_id,
                tree_size,
                timestamp_ms,
                root_hash,
            );
            bundle.kt_consistency_proof = if let Some(from) = consistency_from {
                build_consistency_proof(from, tree_size, &consistency_nodes)
            } else {
                Vec::new()
            };
            self.sign_bundle(&mut bundle);
            bundle
        }
    }

    fn build_sth(
        seed: [u8; 32],
        log_id: [u8; 32],
        tree_size: u64,
        timestamp_ms: u64,
        root_hash: [u8; 32],
    ) -> Vec<u8> {
        let mut msg = Vec::new();
        msg.extend_from_slice(b"QSL-KT/STH/v1");
        msg.push(0x01);
        msg.extend_from_slice(&log_id);
        msg.extend_from_slice(&tree_size.to_be_bytes());
        msg.extend_from_slice(&timestamp_ms.to_be_bytes());
        msg.extend_from_slice(&root_hash);
        let digest = sha512(&msg);
        let sig = SigningKey::from_bytes(&seed).sign(&digest).to_bytes();

        let mut out = Vec::new();
        out.push(0x01);
        out.extend_from_slice(&log_id);
        out.extend_from_slice(&tree_size.to_be_bytes());
        out.extend_from_slice(&timestamp_ms.to_be_bytes());
        out.extend_from_slice(&root_hash);
        out.extend_from_slice(&sig);
        out
    }

    fn build_inclusion_proof(leaf_index: u64, tree_size: u64, siblings: &[[u8; 32]]) -> Vec<u8> {
        let mut out = Vec::new();
        out.push(0x01);
        out.extend_from_slice(&leaf_index.to_be_bytes());
        out.extend_from_slice(&tree_size.to_be_bytes());
        out.extend_from_slice(&(siblings.len() as u16).to_be_bytes());
        for sibling in siblings {
            out.extend_from_slice(sibling);
        }
        out
    }

    fn build_consistency_proof(
        from_tree_size: u64,
        to_tree_size: u64,
        nodes: &[[u8; 32]],
    ) -> Vec<u8> {
        let mut out = Vec::new();
        out.push(0x01);
        out.extend_from_slice(&from_tree_size.to_be_bytes());
        out.extend_from_slice(&to_tree_size.to_be_bytes());
        out.extend_from_slice(&(nodes.len() as u16).to_be_bytes());
        for node in nodes {
            out.extend_from_slice(node);
        }
        out
    }

    fn verifier(
        fixture: &SigningFixture,
        now_ms: u64,
        allow_disabled_nonproduction: bool,
    ) -> CanonicalKtVerifier {
        CanonicalKtVerifier::new(
            [KtPinnedLog {
                log_id: [0xAB; 32],
                verifying_key: fixture.log_vk(),
                proof_cache_ttl_seconds: 300,
            }],
            KtTimeSource::Fixed(now_ms),
            allow_disabled_nonproduction,
        )
    }

    fn disabled_bundle(fixture: &SigningFixture) -> PrekeyBundle {
        let mut bundle = PrekeyBundle {
            user_id: b"alice".to_vec(),
            device_id: 7,
            valid_from: 1,
            valid_to: 9,
            ik_sig_ec_pub: fixture.bundle_vk(),
            ik_sig_pq_pub: vec![0x42; SZ_MLDSA65_PUB],
            spk_dh_pub: [0x11; SZ_X25519_PUB],
            spk_pq_pub: vec![0x22; SZ_MLKEM768_PUB],
            pq_rcv_id: 17,
            pq_rcv_pub: vec![0x33; SZ_MLKEM768_PUB],
            opk_dh: None,
            opk_pq: None,
            sig_ec: vec![0u8; SZ_ED25519_SIG],
            sig_pq: vec![0u8; SZ_MLDSA65_SIG],
            kt_log_id: [0u8; 32],
            kt_sth: Vec::new(),
            kt_inclusion_proof: Vec::new(),
            kt_consistency_proof: Vec::new(),
        };
        fixture.sign_bundle(&mut bundle);
        bundle
    }

    fn hs1_from_bundle(bundle: &PrekeyBundle) -> HandshakeInit {
        HandshakeInit {
            protocol_version: 0x0403,
            suite_id: 0x0001,
            session_id: [0x55; 16],
            user_id_b: b"bob".to_vec(),
            device_id_b: 9,
            ek_dh_a_pub: [0x44; SZ_X25519_PUB],
            ct1: vec![0x66; 1088],
            opk_used: false,
            ct2: None,
            opk_dh_id: None,
            opk_pq_id: None,
            pq_rcv_a_id: bundle.pq_rcv_id,
            pq_rcv_a_pub: bundle.pq_rcv_pub.clone(),
            ik_sig_ec_a_pub: bundle.ik_sig_ec_pub,
            ik_sig_pq_a_pub: bundle.ik_sig_pq_pub.clone(),
            sig_ec_a: vec![0x88; 64],
            sig_pq_a: vec![0x99; SZ_MLDSA65_SIG],
        }
    }

    #[test]
    fn disabled_shape_requires_explicit_nonproduction_mode() {
        let fixture = SigningFixture::new();
        let bundle = disabled_bundle(&fixture);
        let ed25519 = StdEd25519;

        let err = verifier(&fixture, 1_000, false)
            .verify_bundle(&bundle, &ed25519, fixture.pq_sig.as_ref())
            .unwrap_err();
        assert!(matches!(err, KtError::VerifyFailed { .. }));

        let status = verifier(&fixture, 1_000, true)
            .verify_bundle(&bundle, &ed25519, fixture.pq_sig.as_ref())
            .unwrap();
        assert_eq!(status, KtVerification::DisabledNonProduction);
    }

    #[test]
    fn verify_bundle_rejects_missing_sth_for_enabled_kt() {
        let fixture = SigningFixture::new();
        let ed25519 = StdEd25519;
        let mut bundle = fixture.build_bundle(1, 1_000, vec![], vec![], None);
        bundle.kt_sth.clear();
        fixture.sign_bundle(&mut bundle);

        let err = verifier(&fixture, 1_001, false)
            .verify_bundle(&bundle, &ed25519, fixture.pq_sig.as_ref())
            .unwrap_err();
        assert!(matches!(err, KtError::VerifyFailed { .. }));
    }

    #[test]
    fn verify_bundle_rejects_stale_sth() {
        let fixture = SigningFixture::new();
        let ed25519 = StdEd25519;
        let bundle = fixture.build_bundle(1, 1_000, vec![], vec![], None);

        let err = verifier(&fixture, 1_000 + 301_000, false)
            .verify_bundle(&bundle, &ed25519, fixture.pq_sig.as_ref())
            .unwrap_err();
        assert!(matches!(err, KtError::VerifyFailed { .. }));
    }

    #[test]
    fn verify_bundle_rejects_malformed_sth_with_valid_bundle_signature() {
        let fixture = SigningFixture::new();
        let ed25519 = StdEd25519;
        let mut bundle = fixture.build_bundle(1, 1_000, vec![], vec![], None);
        bundle.kt_sth = vec![0x01];
        fixture.sign_bundle(&mut bundle);

        let err = verifier(&fixture, 1_001, false)
            .verify_bundle(&bundle, &ed25519, fixture.pq_sig.as_ref())
            .unwrap_err();
        assert!(matches!(err, KtError::VerifyFailed { .. }));
    }

    #[test]
    fn verify_bundle_rejects_unsigned_bundle_material_before_kt_state_update() {
        let fixture = SigningFixture::new();
        let ed25519 = StdEd25519;
        let mut bundle = fixture.build_bundle(1, 1_000, vec![], vec![], None);
        bundle.sig_ec.fill(0);

        let err = verifier(&fixture, 1_001, false)
            .verify_bundle(&bundle, &ed25519, fixture.pq_sig.as_ref())
            .unwrap_err();
        assert!(matches!(err, KtError::BundleSigFail { .. }));
    }

    #[test]
    fn verify_bundle_rejects_wrong_or_unpinned_log_id() {
        let fixture = SigningFixture::new();
        let ed25519 = StdEd25519;
        let mut bundle = fixture.build_bundle(1, 1_000, vec![], vec![], None);
        bundle.kt_log_id = [0xCD; 32];
        fixture.sign_bundle(&mut bundle);

        let err = verifier(&fixture, 1_001, false)
            .verify_bundle(&bundle, &ed25519, fixture.pq_sig.as_ref())
            .unwrap_err();
        assert!(matches!(err, KtError::VerifyFailed { .. }));
    }

    #[test]
    fn verify_bundle_rejects_bad_sth_signature_with_valid_bundle_signature() {
        let fixture = SigningFixture::new();
        let ed25519 = StdEd25519;
        let mut bundle = fixture.build_bundle(1, 1_000, vec![], vec![], None);
        let leaf_hash = sha256_prefixed(0x00, &bundle.bundle_leaf_data());
        bundle.kt_sth = build_sth([0x55; 32], bundle.kt_log_id, 1, 1_000, leaf_hash);
        fixture.sign_bundle(&mut bundle);

        let err = verifier(&fixture, 1_001, false)
            .verify_bundle(&bundle, &ed25519, fixture.pq_sig.as_ref())
            .unwrap_err();
        assert!(matches!(err, KtError::VerifyFailed { .. }));
    }

    #[test]
    fn verify_bundle_requires_consistency_proof_when_tree_advances() {
        let fixture = SigningFixture::new();
        let ed25519 = StdEd25519;
        let verifier = verifier(&fixture, 5_000, false);

        let first = fixture.build_bundle(1, 4_000, vec![], vec![], None);
        verifier
            .verify_bundle(&first, &ed25519, fixture.pq_sig.as_ref())
            .unwrap();

        let sibling = sha256(b"leaf-1");
        let mut advanced = fixture.build_bundle(
            2,
            4_100,
            vec![sibling],
            vec![sha256_prefixed(0x00, &first.bundle_leaf_data()), sibling],
            Some(1),
        );
        advanced.kt_consistency_proof.clear();
        fixture.sign_bundle(&mut advanced);

        let err = verifier
            .verify_bundle(&advanced, &ed25519, fixture.pq_sig.as_ref())
            .unwrap_err();
        assert!(matches!(err, KtError::VerifyFailed { .. }));
        let state = verifier.accepted_state(&first.kt_log_id).unwrap();
        assert_eq!(state.tree_size, 1);
    }

    #[test]
    fn verify_bundle_rejects_inclusion_root_mismatch() {
        let fixture = SigningFixture::new();
        let ed25519 = StdEd25519;
        let mut bundle = fixture.build_bundle(1, 4_000, vec![], vec![], None);
        bundle.kt_inclusion_proof = build_inclusion_proof(0, 1, &[[0x55; 32]]);
        fixture.sign_bundle(&mut bundle);

        let err = verifier(&fixture, 4_050, false)
            .verify_bundle(&bundle, &ed25519, fixture.pq_sig.as_ref())
            .unwrap_err();
        assert!(matches!(err, KtError::VerifyFailed { .. }));
    }

    #[test]
    fn responder_binding_rejects_missing_or_mismatched_bundle() {
        let fixture = SigningFixture::new();
        let ed25519 = StdEd25519;
        let bundle = fixture.build_bundle(1, 1_000, vec![], vec![], None);
        let verifier = verifier(&fixture, 1_010, false);
        let hs1 = hs1_from_bundle(&bundle);

        let err = verifier
            .verify_responder_binding(&hs1, None, &ed25519, fixture.pq_sig.as_ref())
            .unwrap_err();
        assert!(matches!(err, KtError::VerifyFailed { .. }));

        let mut mismatched = bundle.clone();
        mismatched.pq_rcv_id ^= 1;
        fixture.sign_bundle(&mut mismatched);
        let err = verifier
            .verify_responder_binding(&hs1, Some(&mismatched), &ed25519, fixture.pq_sig.as_ref())
            .unwrap_err();
        assert!(matches!(err, KtError::VerifyFailed { .. }));

        let status = verifier
            .verify_responder_binding(&hs1, Some(&bundle), &ed25519, fixture.pq_sig.as_ref())
            .unwrap();
        assert_eq!(status, KtVerification::Verified);
    }
}
