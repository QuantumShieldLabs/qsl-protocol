use ed25519_dalek::{Signer as _, SigningKey};
use quantumshield_refimpl::crypto::stdcrypto::StdEd25519;
use quantumshield_refimpl::crypto::traits::{CryptoError, PqSigMldsa65};
use quantumshield_refimpl::kt::{
    CanonicalKtVerifier, KtError, KtPinnedLog, KtTimeSource, KtVerifier,
};
use quantumshield_refimpl::qsp::{
    PrekeyBundle, SZ_ED25519_SIG, SZ_MLDSA65_PUB, SZ_MLDSA65_SIG, SZ_MLKEM768_PUB, SZ_X25519_PUB,
};
use serde::Deserialize;
use sha2::{Digest, Sha256, Sha512};

#[derive(Deserialize)]
struct Root {
    vectors: Vec<VectorCase>,
}

#[derive(Deserialize)]
struct VectorCase {
    id: String,
    expect: Expect,
    ext: CaseExt,
}

#[derive(Deserialize)]
struct Expect {
    ok: bool,
    reason_code: Option<String>,
}

#[derive(Deserialize)]
struct CaseExt {
    tree_size: u64,
    timestamp_ms: u64,
    now_ms: u64,
    prime_first_seen: bool,
    mutation: String,
}

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
}

impl SigningFixture {
    fn new() -> Self {
        Self {
            bundle_seed: [0x11; 32],
            log_seed: [0x22; 32],
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
            user_id: b"vector".to_vec(),
            device_id: 3,
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

fn build_consistency_proof(from_tree_size: u64, to_tree_size: u64, nodes: &[[u8; 32]]) -> Vec<u8> {
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

#[test]
fn kt_verifier_vectors() {
    let root: Root = serde_json::from_str(include_str!(
        "../../../../inputs/suite2/vectors/qshield_suite2_kt_verifier_vectors_v1.json"
    ))
    .expect("vector json");
    let fixture = SigningFixture::new();
    let ed25519 = StdEd25519;
    let pq_sig = TestPqSig;
    let base_leaf_hash = sha256_prefixed(
        0x00,
        &fixture
            .build_bundle(1, 1_000, vec![], vec![], None)
            .bundle_leaf_data(),
    );
    let sibling = sha256(b"vector-sibling");

    for case in root.vectors {
        let verifier = CanonicalKtVerifier::new(
            [KtPinnedLog {
                log_id: [0xAB; 32],
                verifying_key: fixture.log_vk(),
                proof_cache_ttl_seconds: 300,
            }],
            KtTimeSource::Fixed(case.ext.now_ms),
            false,
        );

        if case.ext.prime_first_seen {
            let first = fixture.build_bundle(1, 1_000, vec![], vec![], None);
            verifier
                .verify_bundle(&first, &ed25519, &pq_sig)
                .expect("prime first seen");
        }

        let consistency = if case.ext.tree_size == 2 {
            Some((1, vec![base_leaf_hash, sibling]))
        } else {
            None
        };
        let mut bundle = fixture.build_bundle(
            case.ext.tree_size,
            case.ext.timestamp_ms,
            if case.ext.tree_size == 2 {
                vec![sibling]
            } else {
                vec![]
            },
            consistency
                .as_ref()
                .map(|(_, nodes)| nodes.clone())
                .unwrap_or_default(),
            consistency.as_ref().map(|(from, _)| *from),
        );

        match case.ext.mutation.as_str() {
            "clear_sth" => bundle.kt_sth.clear(),
            "bad_inclusion" => {
                bundle.kt_inclusion_proof =
                    build_inclusion_proof(0, case.ext.tree_size, &[[0x55; 32]]);
            }
            "clear_consistency" => bundle.kt_consistency_proof.clear(),
            "none" => {}
            other => panic!("unknown mutation {other}"),
        }
        fixture.sign_bundle(&mut bundle);

        if case.expect.ok {
            verifier
                .verify_bundle(&bundle, &ed25519, &pq_sig)
                .unwrap_or_else(|e| panic!("case {} should pass, got {e:?}", case.id));
        } else {
            match case.expect.reason_code.as_deref() {
                Some("kt_fail") => {
                    let err = verifier
                        .verify_bundle(&bundle, &ed25519, &pq_sig)
                        .unwrap_err();
                    assert!(
                        matches!(err, KtError::VerifyFailed { .. }),
                        "case {} expected kt_fail, got {:?}",
                        case.id,
                        err
                    );
                }
                Some(other) => panic!("unknown expectation {other}"),
                None => panic!("negative case {} missing reason_code", case.id),
            }
        }
    }
}
